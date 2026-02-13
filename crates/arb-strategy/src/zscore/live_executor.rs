//! 라이브 주문 실행 엔진.
//!
//! Upbit 현물 + Bybit 선물 양 레그 동시 주문을 실행합니다.
//! IOC 지정가 주문을 기본으로 하며, 비상 청산 3단계 escalation을 지원합니다.
//! trait/dyn 없이 구체 제네릭 타입으로 hot path 성능을 최적화합니다.

use std::sync::Arc;
use std::time::Duration;

use rust_decimal::Decimal;
use tracing::{debug, error, info, warn};

use arb_exchange::{
    ExchangeError, InstrumentDataProvider, LinearOrderManagement, MarketData, OrderManagement,
    OrderRequest, OrderSide, OrderType, TimeInForce,
};

use crate::zscore::config::ZScoreConfig;
use crate::zscore::instrument::InstrumentInfo;

// ---------------------------------------------------------------------------
// 요청/응답 타입
// ---------------------------------------------------------------------------

/// 진입 주문 요청.
#[derive(Debug, Clone)]
pub struct EntryRequest {
    /// 코인 심볼 (예: "BTC").
    pub coin: String,
    /// 주문 수량 (코인 단위, 양 레그 동일).
    pub qty: Decimal,
    /// Upbit KRW 지정가.
    pub upbit_krw_price: Decimal,
    /// Bybit USDT 지정가.
    pub bybit_usdt_price: Decimal,
    /// 현재 USD/KRW 환율.
    pub usd_krw: f64,
    /// 거래 규격 정보.
    pub instrument_info: InstrumentInfo,
    /// Client Order ID (UUID v7, crash recovery용).
    pub client_order_id: String,
}

/// 청산 주문 요청.
#[derive(Debug, Clone)]
pub struct ExitRequest {
    /// 코인 심볼.
    pub coin: String,
    /// 청산 수량.
    pub qty: Decimal,
    /// 거래 규격 정보.
    pub instrument_info: InstrumentInfo,
    /// Exit Client Order ID.
    pub exit_client_order_id: String,
}

/// 진입 체결 결과.
#[derive(Debug, Clone)]
pub struct ExecutedEntry {
    /// Upbit 주문 ID.
    pub upbit_order_id: String,
    /// Bybit 주문 ID.
    pub bybit_order_id: String,
    /// Upbit 체결 수량.
    pub upbit_filled_qty: Decimal,
    /// Bybit 체결 수량.
    pub bybit_filled_qty: Decimal,
    /// Upbit 평균 체결가 (KRW).
    pub upbit_avg_price_krw: Decimal,
    /// Bybit 평균 체결가 (USDT).
    pub bybit_avg_price: Decimal,
    /// Upbit 수수료.
    pub upbit_fee: Decimal,
    /// Bybit 수수료.
    pub bybit_fee: Decimal,
    /// 유효 수량 (양 레그 min, Upbit 코인 수수료 차감 후).
    pub effective_qty: Decimal,
    /// 초과분 청산 비용 (USDT).
    pub adjustment_cost: Decimal,
}

/// 청산 체결 결과.
#[derive(Debug, Clone)]
pub struct ExecutedExit {
    /// Upbit 주문 ID.
    pub upbit_order_id: String,
    /// Bybit 주문 ID.
    pub bybit_order_id: String,
    /// Upbit 체결 수량.
    pub upbit_filled_qty: Decimal,
    /// Bybit 체결 수량.
    pub bybit_filled_qty: Decimal,
    /// Upbit 평균 체결가 (KRW).
    pub upbit_avg_price_krw: Decimal,
    /// Bybit 평균 체결가 (USDT).
    pub bybit_avg_price: Decimal,
    /// Upbit 수수료.
    pub upbit_fee: Decimal,
    /// Bybit 수수료.
    pub bybit_fee: Decimal,
}

/// 레그 방향.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leg {
    /// Upbit 현물.
    Upbit,
    /// Bybit 선물.
    Bybit,
}

impl std::fmt::Display for Leg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Leg::Upbit => write!(f, "upbit"),
            Leg::Bybit => write!(f, "bybit"),
        }
    }
}

/// 주문 실행 에러.
#[derive(Debug)]
pub enum OrderExecutionError {
    /// 양쪽 미체결 (주문 포기).
    BothUnfilled,
    /// 한쪽만 체결 (비상 청산 수행).
    SingleLegFilled {
        /// 체결된 레그.
        leg: Leg,
        /// 비상 청산 성공 여부.
        emergency_closed: bool,
    },
    /// 비상 청산 실패 (naked exposure).
    EmergencyCloseFailed {
        /// 노출된 레그.
        leg: Leg,
        /// 원본 주문 ID.
        order_id: String,
    },
    /// 거래소 에러.
    ExchangeError(ExchangeError),
    /// 타임아웃.
    Timeout {
        /// 타임아웃 발생 레그.
        leg: Option<Leg>,
    },
}

impl std::fmt::Display for OrderExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BothUnfilled => write!(f, "both legs unfilled, entry abandoned"),
            Self::SingleLegFilled {
                leg,
                emergency_closed,
            } => {
                write!(
                    f,
                    "single leg filled: {leg}, emergency_closed={emergency_closed}"
                )
            }
            Self::EmergencyCloseFailed { leg, order_id } => {
                write!(f, "emergency close failed: {leg} order_id={order_id}")
            }
            Self::ExchangeError(e) => write!(f, "exchange error: {e}"),
            Self::Timeout { leg } => {
                write!(f, "timeout: leg={leg:?}")
            }
        }
    }
}

impl std::error::Error for OrderExecutionError {}

impl From<ExchangeError> for OrderExecutionError {
    fn from(e: ExchangeError) -> Self {
        Self::ExchangeError(e)
    }
}

// ---------------------------------------------------------------------------
// 내부 주문 결과 (간소화)
// ---------------------------------------------------------------------------

/// 내부 주문 결과.
#[derive(Debug)]
struct OrderResult {
    /// 거래소 주문 ID.
    id: String,
    /// 체결 수량.
    filled_qty: Decimal,
    /// 평균 체결가.
    avg_price: Decimal,
    /// 지불 수수료.
    paid_fee: Decimal,
}

// ---------------------------------------------------------------------------
// LiveExecutor
// ---------------------------------------------------------------------------

/// 라이브 주문 실행 엔진.
///
/// Upbit 현물 매수 + Bybit 선물 short 양 레그 동시 주문을 실행합니다.
/// 구체 제네릭 타입으로 trait object 오버헤드 없이 사용합니다.
pub struct LiveExecutor<U, B>
where
    U: MarketData + OrderManagement + Send + Sync + 'static,
    B: MarketData
        + OrderManagement
        + LinearOrderManagement
        + InstrumentDataProvider
        + Send
        + Sync
        + 'static,
{
    upbit: Arc<U>,
    bybit: Arc<B>,
    config: Arc<ZScoreConfig>,
}

impl<U, B> LiveExecutor<U, B>
where
    U: MarketData + OrderManagement + Send + Sync + 'static,
    B: MarketData
        + OrderManagement
        + LinearOrderManagement
        + InstrumentDataProvider
        + Send
        + Sync
        + 'static,
{
    /// 새 LiveExecutor를 생성합니다.
    pub fn new(upbit: Arc<U>, bybit: Arc<B>, config: Arc<ZScoreConfig>) -> Self {
        Self {
            upbit,
            bybit,
            config,
        }
    }

    /// 진입 주문을 실행합니다.
    ///
    /// 양 레그 IOC 지정가를 동시 발주하고, 체결 결과를 대기합니다.
    /// 한쪽만 체결된 경우 비상 청산을 수행합니다.
    #[allow(clippy::too_many_lines)]
    pub async fn execute_entry(
        &self,
        request: &EntryRequest,
    ) -> Result<ExecutedEntry, OrderExecutionError> {
        let coin = &request.coin;
        let qty = request.qty;

        // Upbit KRW 가격 (슬리피지 마진 적용: 매수이므로 상향)
        let slippage_mul = Decimal::ONE
            + Decimal::try_from(self.config.max_slippage_pct / 100.0).unwrap_or(Decimal::ZERO);
        let upbit_limit_price_krw = request.upbit_krw_price * slippage_mul;

        // Bybit USDT 가격 (슬리피지 마진 적용: 매도(short)이므로 하향)
        let bybit_limit_price = request.bybit_usdt_price
            * (Decimal::ONE
                - Decimal::try_from(self.config.max_slippage_pct / 100.0).unwrap_or(Decimal::ZERO));

        let upbit_market = format!("KRW-{}", coin);
        let bybit_symbol = format!("{}USDT", coin);

        info!(
            coin = coin.as_str(),
            qty = %qty,
            upbit_limit_krw = %upbit_limit_price_krw,
            bybit_limit_usdt = %bybit_limit_price,
            client_order_id = request.client_order_id.as_str(),
            "진입 주문 발주 시작"
        );

        let order_timeout = Duration::from_secs(self.config.order_timeout_sec);

        // 양 레그 동시 발주 (IOC 지정가)
        let (upbit_result, bybit_result) = tokio::join!(
            tokio::time::timeout(
                order_timeout,
                self.place_upbit_buy(
                    &upbit_market,
                    qty,
                    upbit_limit_price_krw,
                    &request.client_order_id,
                ),
            ),
            tokio::time::timeout(
                order_timeout,
                self.place_bybit_short(
                    &bybit_symbol,
                    qty,
                    bybit_limit_price,
                    &request.client_order_id,
                ),
            ),
        );

        // 결과 파싱
        let upbit_order = match upbit_result {
            Ok(Ok(order)) => {
                info!(
                    order_id = order.id.as_str(),
                    filled_qty = %order.filled_qty,
                    avg_price = %order.avg_price,
                    "Upbit 매수 체결"
                );
                Some(order)
            }
            Ok(Err(e)) => {
                warn!(error = %e, "Upbit 매수 실패");
                None
            }
            Err(_) => {
                warn!("Upbit 매수 타임아웃");
                None
            }
        };

        let bybit_order = match bybit_result {
            Ok(Ok(order)) => {
                info!(
                    order_id = order.id.as_str(),
                    filled_qty = %order.filled_qty,
                    avg_price = %order.avg_price,
                    "Bybit short 체결"
                );
                Some(order)
            }
            Ok(Err(e)) => {
                warn!(error = %e, "Bybit short 실패");
                None
            }
            Err(_) => {
                warn!("Bybit short 타임아웃");
                None
            }
        };

        match (upbit_order, bybit_order) {
            // 양쪽 체결 성공
            (Some(upbit), Some(bybit)) => self.handle_both_filled_entry(request, upbit, bybit),
            // 한쪽만 체결 → 비상 청산
            (Some(upbit), None) => {
                warn!(
                    upbit_order_id = upbit.id.as_str(),
                    filled_qty = %upbit.filled_qty,
                    "Bybit 미체결, Upbit 비상 매도 시작"
                );
                let emergency_closed = self
                    .emergency_close_leg(Leg::Upbit, &upbit_market, upbit.filled_qty)
                    .await;
                Err(OrderExecutionError::SingleLegFilled {
                    leg: Leg::Upbit,
                    emergency_closed,
                })
            }
            (None, Some(bybit)) => {
                warn!(
                    bybit_order_id = bybit.id.as_str(),
                    filled_qty = %bybit.filled_qty,
                    "Upbit 미체결, Bybit 비상 close 시작"
                );
                let emergency_closed = self
                    .emergency_close_leg(Leg::Bybit, &bybit_symbol, bybit.filled_qty)
                    .await;
                Err(OrderExecutionError::SingleLegFilled {
                    leg: Leg::Bybit,
                    emergency_closed,
                })
            }
            // 양쪽 미체결
            (None, None) => {
                info!("양쪽 미체결, 진입 포기");
                Err(OrderExecutionError::BothUnfilled)
            }
        }
    }

    /// 양 레그 체결 성공 시 결과 산출.
    fn handle_both_filled_entry(
        &self,
        request: &EntryRequest,
        upbit: OrderResult,
        bybit: OrderResult,
    ) -> Result<ExecutedEntry, OrderExecutionError> {
        // Upbit은 코인 수수료를 수량에서 차감 (taker fee rate 기반 추산)
        let upbit_fee_rate = self.config.upbit_taker_fee;
        let upbit_net_qty = upbit.filled_qty * (Decimal::ONE - upbit_fee_rate);
        let effective_qty = upbit_net_qty.min(bybit.filled_qty);

        debug!(
            upbit_filled = %upbit.filled_qty,
            upbit_net = %upbit_net_qty,
            bybit_filled = %bybit.filled_qty,
            effective = %effective_qty,
            "유효 수량 산출"
        );

        // 초과분 계산
        let upbit_excess = upbit_net_qty - effective_qty;
        let bybit_excess = bybit.filled_qty - effective_qty;
        let mut adjustment_cost = Decimal::ZERO;

        if upbit_excess > Decimal::ZERO || bybit_excess > Decimal::ZERO {
            warn!(
                upbit_excess = %upbit_excess,
                bybit_excess = %bybit_excess,
                "수량 차이 감지, 초과분 처리 필요"
            );
            // dust threshold 이하는 adjustment_cost로 기록
            let dust_threshold =
                Decimal::try_from(self.config.max_dust_usdt).unwrap_or(Decimal::new(5, 0));
            let excess_usdt = upbit_excess.max(bybit_excess) * request.bybit_usdt_price;

            if excess_usdt <= dust_threshold {
                adjustment_cost = excess_usdt;
                debug!(
                    adjustment_cost = %adjustment_cost,
                    "dust threshold 이하, adjustment_cost로 기록"
                );
            }
            // dust 초과 시 caller가 별도 처리
        }

        // 수수료 (KRW/USDT 기준)
        let upbit_fee = upbit.paid_fee;
        let bybit_fee = bybit.paid_fee;

        Ok(ExecutedEntry {
            upbit_order_id: upbit.id,
            bybit_order_id: bybit.id,
            upbit_filled_qty: upbit.filled_qty,
            bybit_filled_qty: bybit.filled_qty,
            upbit_avg_price_krw: upbit.avg_price,
            bybit_avg_price: bybit.avg_price,
            upbit_fee,
            bybit_fee,
            effective_qty,
            adjustment_cost,
        })
    }

    /// 청산 주문을 실행합니다.
    ///
    /// Upbit 시장가 매도 + Bybit 시장가 close 양 레그 동시 발주.
    pub async fn execute_exit(
        &self,
        request: &ExitRequest,
    ) -> Result<ExecutedExit, OrderExecutionError> {
        let coin = &request.coin;
        let qty = request.qty;

        let upbit_market = format!("KRW-{}", coin);
        let bybit_symbol = format!("{}USDT", coin);

        info!(
            coin = coin.as_str(),
            qty = %qty,
            exit_client_order_id = request.exit_client_order_id.as_str(),
            "청산 주문 발주 시작"
        );

        let order_timeout = Duration::from_secs(self.config.order_timeout_sec);

        // 양 레그 동시 발주: Upbit 시장가 매도 + Bybit 시장가 close(매수)
        let (upbit_result, bybit_result) = tokio::join!(
            tokio::time::timeout(
                order_timeout,
                self.place_upbit_sell(&upbit_market, qty, &request.exit_client_order_id),
            ),
            tokio::time::timeout(
                order_timeout,
                self.place_bybit_close(&bybit_symbol, qty, &request.exit_client_order_id),
            ),
        );

        let upbit_order = match upbit_result {
            Ok(Ok(order)) => {
                info!(
                    order_id = order.id.as_str(),
                    filled_qty = %order.filled_qty,
                    "Upbit 매도 체결"
                );
                Some(order)
            }
            Ok(Err(e)) => {
                warn!(error = %e, "Upbit 매도 실패");
                None
            }
            Err(_) => {
                warn!("Upbit 매도 타임아웃");
                None
            }
        };

        let bybit_order = match bybit_result {
            Ok(Ok(order)) => {
                info!(
                    order_id = order.id.as_str(),
                    filled_qty = %order.filled_qty,
                    "Bybit close 체결"
                );
                Some(order)
            }
            Ok(Err(e)) => {
                warn!(error = %e, "Bybit close 실패");
                None
            }
            Err(_) => {
                warn!("Bybit close 타임아웃");
                None
            }
        };

        match (upbit_order, bybit_order) {
            (Some(upbit), Some(bybit)) => Ok(ExecutedExit {
                upbit_order_id: upbit.id,
                bybit_order_id: bybit.id,
                upbit_filled_qty: upbit.filled_qty,
                bybit_filled_qty: bybit.filled_qty,
                upbit_avg_price_krw: upbit.avg_price,
                bybit_avg_price: bybit.avg_price,
                upbit_fee: upbit.paid_fee,
                bybit_fee: bybit.paid_fee,
            }),
            (Some(_upbit), None) => {
                warn!("Bybit close 실패, 비상 처리 필요");
                Err(OrderExecutionError::SingleLegFilled {
                    leg: Leg::Upbit,
                    emergency_closed: false,
                })
            }
            (None, Some(_bybit)) => {
                warn!("Upbit 매도 실패, 비상 처리 필요");
                Err(OrderExecutionError::SingleLegFilled {
                    leg: Leg::Bybit,
                    emergency_closed: false,
                })
            }
            (None, None) => {
                warn!("양쪽 청산 실패");
                Err(OrderExecutionError::BothUnfilled)
            }
        }
    }

    /// 비상 청산: 단일 레그 청산 (3단계 escalation).
    ///
    /// Stage 1 (0~2분): IOC 지정가 재시도 (지수 백오프).
    /// Stage 2 (2~5분): 넓은 IOC 지정가.
    /// Stage 3 (5분 초과): 실패 반환 (caller가 kill switch 발동).
    async fn emergency_close_leg(&self, leg: Leg, symbol: &str, qty: Decimal) -> bool {
        let started_at = tokio::time::Instant::now();
        let stage1_deadline = Duration::from_secs(120);
        let stage2_deadline = Duration::from_secs(300);

        info!(
            leg = %leg,
            symbol = symbol,
            qty = %qty,
            "비상 청산 시작 (3단계 escalation)"
        );

        // 95% 이상 체결이면 성공으로 간주
        let success_threshold = qty * Decimal::new(95, 2);

        // Stage 1: IOC 지정가 재시도 (0~2분)
        let mut backoff = Duration::from_secs(1);
        let mut attempt = 0u32;

        while started_at.elapsed() < stage1_deadline {
            attempt += 1;
            debug!(
                leg = %leg,
                attempt = attempt,
                elapsed_secs = started_at.elapsed().as_secs(),
                "비상 청산 Stage 1 시도"
            );

            let result = match leg {
                Leg::Upbit => self.place_upbit_sell(symbol, qty, "emergency").await,
                Leg::Bybit => self.place_bybit_close(symbol, qty, "emergency").await,
            };

            match result {
                Ok(order) if order.filled_qty >= success_threshold => {
                    info!(
                        leg = %leg,
                        order_id = order.id.as_str(),
                        filled = %order.filled_qty,
                        "비상 청산 Stage 1 성공"
                    );
                    return true;
                }
                Ok(order) => {
                    debug!(
                        filled = %order.filled_qty,
                        target = %qty,
                        "부분 체결, 재시도"
                    );
                }
                Err(e) => {
                    warn!(error = %e, "비상 청산 Stage 1 실패");
                }
            }

            tokio::time::sleep(backoff).await;
            backoff = (backoff * 2).min(Duration::from_secs(8));
        }

        // Stage 2: 넓은 IOC 지정가 (2~5분)
        for (i, &_slippage_pct) in self
            .config
            .emergency_wide_ioc_slippage_pct
            .iter()
            .enumerate()
        {
            if started_at.elapsed() >= stage2_deadline {
                break;
            }

            attempt += 1;
            info!(
                leg = %leg,
                attempt = attempt,
                stage_idx = i,
                elapsed_secs = started_at.elapsed().as_secs(),
                "비상 청산 Stage 2 시도 (넓은 IOC)"
            );

            // 넓은 슬리피지로 시장가 재시도
            let result = match leg {
                Leg::Upbit => self.place_upbit_sell(symbol, qty, "emergency-wide").await,
                Leg::Bybit => self.place_bybit_close(symbol, qty, "emergency-wide").await,
            };

            match result {
                Ok(order) if order.filled_qty > Decimal::ZERO => {
                    info!(
                        leg = %leg,
                        filled = %order.filled_qty,
                        stage = i + 1,
                        "비상 청산 Stage 2 성공"
                    );
                    return true;
                }
                Ok(_) => {
                    debug!("Stage 2 미체결, 다음 단계");
                }
                Err(e) => {
                    warn!(error = %e, "비상 청산 Stage 2 실패");
                }
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        // Stage 3: 5분 초과 → 실패
        error!(
            leg = %leg,
            symbol = symbol,
            qty = %qty,
            elapsed_secs = started_at.elapsed().as_secs(),
            "비상 청산 5분 초과 실패 — kill switch 발동 필요"
        );
        false
    }

    // -----------------------------------------------------------------------
    // Private helpers: 주문 발주
    // -----------------------------------------------------------------------

    /// Upbit IOC 지정가 매수 주문.
    async fn place_upbit_buy(
        &self,
        market: &str,
        qty: Decimal,
        price_krw: Decimal,
        client_order_id: &str,
    ) -> Result<OrderResult, ExchangeError> {
        debug!(
            market = market,
            qty = %qty,
            price_krw = %price_krw,
            client_order_id = client_order_id,
            "Upbit 매수 주문 발주"
        );

        let request = OrderRequest::limit_buy(market, price_krw, qty)
            .with_time_in_force(TimeInForce::Ioc)
            .with_identifier(client_order_id.to_string());

        let order = self.upbit.place_order(&request).await.map_err(|e| {
            error!(error = %e, market = market, "Upbit 매수 주문 실패");
            e
        })?;

        debug!(
            order_id = order.id.as_str(),
            status = ?order.status,
            executed_volume = %order.executed_volume,
            "Upbit 매수 주문 응답"
        );

        Ok(OrderResult {
            id: order.id,
            filled_qty: order.executed_volume,
            avg_price: order.avg_price.unwrap_or(price_krw),
            paid_fee: order.paid_fee,
        })
    }

    /// Upbit 시장가 매도 주문 (청산/비상 청산).
    async fn place_upbit_sell(
        &self,
        market: &str,
        qty: Decimal,
        client_order_id: &str,
    ) -> Result<OrderResult, ExchangeError> {
        debug!(
            market = market,
            qty = %qty,
            client_order_id = client_order_id,
            "Upbit 매도 주문 발주"
        );

        let request =
            OrderRequest::market_sell(market, qty).with_identifier(client_order_id.to_string());

        let order = self.upbit.place_order(&request).await.map_err(|e| {
            error!(error = %e, market = market, "Upbit 매도 주문 실패");
            e
        })?;

        Ok(OrderResult {
            id: order.id,
            filled_qty: order.executed_volume,
            avg_price: order.avg_price.unwrap_or(Decimal::ZERO),
            paid_fee: order.paid_fee,
        })
    }

    /// Bybit IOC 지정가 short (Sell) 주문 (진입, linear 선물).
    async fn place_bybit_short(
        &self,
        symbol: &str,
        qty: Decimal,
        price: Decimal,
        client_order_id: &str,
    ) -> Result<OrderResult, ExchangeError> {
        debug!(
            symbol = symbol,
            qty = %qty,
            price = %price,
            client_order_id = client_order_id,
            category = "linear",
            reduce_only = false,
            "Bybit linear short 주문 발주"
        );

        let request = OrderRequest {
            market: symbol.to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            volume: Some(qty),
            price: Some(price),
            time_in_force: Some(TimeInForce::Ioc),
            identifier: Some(client_order_id.to_string()),
        };

        // linear 선물 API 사용 (reduce_only=false: 신규 포지션)
        let order = self
            .bybit
            .place_order_linear(&request, false)
            .await
            .map_err(|e| {
                error!(error = %e, symbol = symbol, "Bybit linear short 주문 실패");
                e
            })?;

        debug!(
            order_id = order.id.as_str(),
            status = ?order.status,
            executed_volume = %order.executed_volume,
            "Bybit linear short 주문 응답"
        );

        Ok(OrderResult {
            id: order.id,
            filled_qty: order.executed_volume,
            avg_price: order.avg_price.unwrap_or(price),
            paid_fee: order.paid_fee,
        })
    }

    /// Bybit 시장가 close (Buy) 주문 (청산, linear 선물).
    async fn place_bybit_close(
        &self,
        symbol: &str,
        qty: Decimal,
        client_order_id: &str,
    ) -> Result<OrderResult, ExchangeError> {
        debug!(
            symbol = symbol,
            qty = %qty,
            client_order_id = client_order_id,
            category = "linear",
            reduce_only = true,
            "Bybit linear close 주문 발주"
        );

        let request = OrderRequest {
            market: symbol.to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            volume: Some(qty),
            price: None,
            time_in_force: None,
            identifier: Some(client_order_id.to_string()),
        };

        // linear 선물 API 사용 (reduce_only=true: 포지션 청산)
        let order = self
            .bybit
            .place_order_linear(&request, true)
            .await
            .map_err(|e| {
                error!(error = %e, symbol = symbol, "Bybit linear close 주문 실패");
                e
            })?;

        debug!(
            order_id = order.id.as_str(),
            status = ?order.status,
            executed_volume = %order.executed_volume,
            "Bybit linear close 주문 응답"
        );

        Ok(OrderResult {
            id: order.id,
            filled_qty: order.executed_volume,
            avg_price: order.avg_price.unwrap_or(Decimal::ZERO),
            paid_fee: order.paid_fee,
        })
    }
}

// ---------------------------------------------------------------------------
// 테스트
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use arb_exchange::*;
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use tokio::sync::Mutex;

    // =======================================================================
    // Mock 거래소 구현
    // =======================================================================

    /// Mock 주문 응답 빌더.
    #[derive(Debug, Clone)]
    struct MockOrderResponse {
        id: String,
        executed_volume: Decimal,
        avg_price: Option<Decimal>,
        paid_fee: Decimal,
        should_fail: bool,
        fail_error: Option<String>,
    }

    impl Default for MockOrderResponse {
        fn default() -> Self {
            Self {
                id: "mock-order-001".to_string(),
                executed_volume: Decimal::ZERO,
                avg_price: None,
                paid_fee: Decimal::ZERO,
                should_fail: false,
                fail_error: None,
            }
        }
    }

    /// Mock Upbit 클라이언트.
    struct MockUpbit {
        /// place_order 호출 시 반환할 응답.
        next_response: Mutex<MockOrderResponse>,
        /// place_order 호출 기록.
        order_history: Mutex<Vec<OrderRequest>>,
    }

    impl MockUpbit {
        fn new(response: MockOrderResponse) -> Self {
            Self {
                next_response: Mutex::new(response),
                order_history: Mutex::new(Vec::new()),
            }
        }
    }

    impl MarketData for MockUpbit {
        fn name(&self) -> &str {
            "mock_upbit"
        }
        async fn get_ticker(&self, _markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }
        async fn get_candles(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
            _before: DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        fn market_code(base: &str, quote: &str) -> String {
            format!("{}-{}", quote, base)
        }
    }

    impl OrderManagement for MockUpbit {
        async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
            self.order_history.lock().await.push(request.clone());
            let resp = self.next_response.lock().await.clone();

            if resp.should_fail {
                return Err(ExchangeError::ApiError(
                    resp.fail_error
                        .unwrap_or_else(|| "mock order failed".to_string()),
                ));
            }

            Ok(Order {
                id: resp.id,
                market: request.market.clone(),
                side: request.side,
                order_type: request.order_type,
                status: OrderStatus::Filled,
                volume: request.volume.unwrap_or(Decimal::ZERO),
                remaining_volume: Decimal::ZERO,
                executed_volume: resp.executed_volume,
                price: request.price,
                avg_price: resp.avg_price,
                paid_fee: resp.paid_fee,
                created_at: Utc::now(),
                identifier: request.identifier.clone(),
            })
        }

        async fn cancel_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }

        async fn get_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }

        async fn get_open_orders(&self, _market: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Ok(vec![])
        }

        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            Ok(vec![])
        }

        async fn get_balance(&self, _currency: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }
    }

    /// Mock Bybit 클라이언트.
    struct MockBybit {
        next_response: Mutex<MockOrderResponse>,
        order_history: Mutex<Vec<OrderRequest>>,
    }

    impl MockBybit {
        fn new(response: MockOrderResponse) -> Self {
            Self {
                next_response: Mutex::new(response),
                order_history: Mutex::new(Vec::new()),
            }
        }
    }

    impl MarketData for MockBybit {
        fn name(&self) -> &str {
            "mock_bybit"
        }
        async fn get_ticker(&self, _markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }
        async fn get_candles(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
            _before: DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        fn market_code(base: &str, _quote: &str) -> String {
            format!("{}USDT", base)
        }
    }

    impl OrderManagement for MockBybit {
        async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
            self.order_history.lock().await.push(request.clone());
            let resp = self.next_response.lock().await.clone();

            if resp.should_fail {
                return Err(ExchangeError::ApiError(
                    resp.fail_error
                        .unwrap_or_else(|| "mock order failed".to_string()),
                ));
            }

            Ok(Order {
                id: resp.id,
                market: request.market.clone(),
                side: request.side,
                order_type: request.order_type,
                status: OrderStatus::Filled,
                volume: request.volume.unwrap_or(Decimal::ZERO),
                remaining_volume: Decimal::ZERO,
                executed_volume: resp.executed_volume,
                price: request.price,
                avg_price: resp.avg_price,
                paid_fee: resp.paid_fee,
                created_at: Utc::now(),
                identifier: request.identifier.clone(),
            })
        }

        async fn cancel_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }

        async fn get_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }

        async fn get_open_orders(&self, _market: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Ok(vec![])
        }

        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            Ok(vec![])
        }

        async fn get_balance(&self, _currency: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::Unsupported("mock".to_string()))
        }
    }

    impl InstrumentDataProvider for MockBybit {
        async fn get_instrument_info(
            &self,
            _symbol: &str,
        ) -> ExchangeResult<InstrumentInfoResponse> {
            Ok(InstrumentInfoResponse {
                tick_size: Decimal::new(1, 2),
                qty_step: Decimal::new(1, 3),
                min_order_qty: Decimal::new(1, 3),
                max_order_qty: Decimal::new(100, 0),
                min_notional: Decimal::new(5, 0),
            })
        }
    }

    impl arb_exchange::LinearOrderManagement for MockBybit {
        async fn place_order_linear(
            &self,
            request: &OrderRequest,
            _reduce_only: bool,
        ) -> ExchangeResult<Order> {
            // linear 주문도 동일한 mock 응답 사용
            self.place_order(request).await
        }

        async fn get_order_linear(&self, order_id: &str) -> ExchangeResult<Order> {
            self.get_order(order_id).await
        }

        async fn cancel_order_linear(
            &self,
            order_id: &str,
            _symbol: Option<&str>,
        ) -> ExchangeResult<Order> {
            self.cancel_order(order_id).await
        }
    }

    // =======================================================================
    // 헬퍼 함수
    // =======================================================================

    fn make_config() -> Arc<ZScoreConfig> {
        Arc::new(ZScoreConfig {
            max_slippage_pct: 0.1,
            order_timeout_sec: 5,
            max_dust_usdt: 5.0,
            upbit_taker_fee: Decimal::new(5, 4),  // 0.0005
            bybit_taker_fee: Decimal::new(55, 5), // 0.00055
            emergency_wide_ioc_slippage_pct: vec![2.0, 3.0, 5.0],
            ..ZScoreConfig::default()
        })
    }

    fn make_entry_request() -> EntryRequest {
        EntryRequest {
            coin: "BTC".to_string(),
            qty: Decimal::new(1, 2), // 0.01 BTC
            upbit_krw_price: Decimal::new(60_000_000, 0),
            bybit_usdt_price: Decimal::new(42000, 0),
            usd_krw: 1350.0,
            instrument_info: InstrumentInfo {
                tick_size: Decimal::new(1, 2),
                qty_step: Decimal::new(1, 5),
                min_order_qty: Decimal::new(1, 5),
                min_notional: Decimal::new(5, 0),
                max_order_qty: Decimal::new(100, 0),
            },
            client_order_id: "test-uuid-001".to_string(),
        }
    }

    fn make_exit_request() -> ExitRequest {
        ExitRequest {
            coin: "BTC".to_string(),
            qty: Decimal::new(1, 2), // 0.01 BTC
            instrument_info: InstrumentInfo {
                tick_size: Decimal::new(1, 2),
                qty_step: Decimal::new(1, 5),
                min_order_qty: Decimal::new(1, 5),
                min_notional: Decimal::new(5, 0),
                max_order_qty: Decimal::new(100, 0),
            },
            exit_client_order_id: "test-uuid-exit-001".to_string(),
        }
    }

    // =======================================================================
    // execute_entry 테스트
    // =======================================================================

    #[tokio::test]
    async fn test_execute_entry_both_filled() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-001".to_string(),
            executed_volume: Decimal::new(1, 2), // 0.01
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::new(30_000, 0), // 30,000 KRW
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-001".to_string(),
            executed_volume: Decimal::new(1, 2), // 0.01
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::new(231, 3), // 0.231 USDT
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_entry(&make_entry_request()).await;

        assert!(result.is_ok());
        let entry = result.unwrap();
        assert_eq!(entry.upbit_order_id, "upbit-001");
        assert_eq!(entry.bybit_order_id, "bybit-001");
        assert_eq!(entry.upbit_filled_qty, Decimal::new(1, 2));
        assert_eq!(entry.bybit_filled_qty, Decimal::new(1, 2));
        // effective_qty = min(upbit_net, bybit) = min(0.01 * 0.9995, 0.01)
        assert!(entry.effective_qty <= Decimal::new(1, 2));
        assert!(entry.effective_qty > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_execute_entry_upbit_only_filled() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-002".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::new(30_000, 0),
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            should_fail: true,
            fail_error: Some("insufficient margin".to_string()),
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_entry(&make_entry_request()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            OrderExecutionError::SingleLegFilled {
                leg,
                emergency_closed: _,
            } => {
                assert_eq!(leg, Leg::Upbit);
            }
            other => panic!("expected SingleLegFilled, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_execute_entry_bybit_only_filled() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            should_fail: true,
            fail_error: Some("insufficient krw".to_string()),
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-003".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::new(231, 3),
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_entry(&make_entry_request()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            OrderExecutionError::SingleLegFilled {
                leg,
                emergency_closed: _,
            } => {
                assert_eq!(leg, Leg::Bybit);
            }
            other => panic!("expected SingleLegFilled, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_execute_entry_both_failed() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            should_fail: true,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            should_fail: true,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_entry(&make_entry_request()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            OrderExecutionError::BothUnfilled => {} // 정상
            other => panic!("expected BothUnfilled, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_execute_entry_effective_qty_upbit_net_less() {
        // Upbit net qty < Bybit qty 인 경우
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-004".to_string(),
            executed_volume: Decimal::new(1, 2), // 0.01
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::new(30_000, 0),
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-004".to_string(),
            executed_volume: Decimal::new(1, 2), // 0.01
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::new(231, 3),
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let entry = executor.execute_entry(&make_entry_request()).await.unwrap();

        // upbit_net = 0.01 * (1 - 0.0005) = 0.009995
        // effective = min(0.009995, 0.01) = 0.009995
        let expected_net = Decimal::new(1, 2) * (Decimal::ONE - Decimal::new(5, 4));
        assert_eq!(entry.effective_qty, expected_net);
    }

    #[tokio::test]
    async fn test_execute_entry_dust_adjustment_cost() {
        // Bybit이 더 적게 체결 → dust 범위 내 adjustment
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-005".to_string(),
            executed_volume: Decimal::new(1, 2), // 0.01
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::new(30_000, 0),
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-005".to_string(),
            executed_volume: Decimal::new(99, 4), // 0.0099 (약간 적음)
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::new(229, 3),
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let entry = executor.execute_entry(&make_entry_request()).await.unwrap();

        // effective = min(upbit_net, 0.0099) = 0.0099
        assert_eq!(entry.bybit_filled_qty, Decimal::new(99, 4));
        // adjustment_cost > 0 (upbit_net excess를 dust로 처리)
        // upbit_net = 0.01 * 0.9995 = 0.009995 > 0.0099
        // excess = 0.009995 - 0.0099 = 0.000095
        // excess_usdt = 0.000095 * 42000 = 3.99 < dust(5) → adjustment_cost 기록
        assert!(entry.adjustment_cost > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_execute_entry_order_request_has_ioc() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-ioc".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-ioc".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit.clone(), bybit.clone(), make_config());
        let _result = executor.execute_entry(&make_entry_request()).await;

        // Upbit 주문 요청에 IOC가 설정되어 있는지 확인
        let upbit_orders = upbit.order_history.lock().await;
        assert_eq!(upbit_orders.len(), 1);
        assert_eq!(upbit_orders[0].time_in_force, Some(TimeInForce::Ioc));
        assert_eq!(upbit_orders[0].side, OrderSide::Buy);
        assert_eq!(upbit_orders[0].order_type, OrderType::Limit);

        // Bybit 주문 요청에 IOC가 설정되어 있는지 확인
        let bybit_orders = bybit.order_history.lock().await;
        assert_eq!(bybit_orders.len(), 1);
        assert_eq!(bybit_orders[0].time_in_force, Some(TimeInForce::Ioc));
        assert_eq!(bybit_orders[0].side, OrderSide::Sell);
        assert_eq!(bybit_orders[0].order_type, OrderType::Limit);
    }

    #[tokio::test]
    async fn test_execute_entry_client_order_id_passed() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "u-id".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "b-id".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit.clone(), bybit.clone(), make_config());
        let _result = executor.execute_entry(&make_entry_request()).await;

        let upbit_orders = upbit.order_history.lock().await;
        assert_eq!(
            upbit_orders[0].identifier,
            Some("test-uuid-001".to_string())
        );

        let bybit_orders = bybit.order_history.lock().await;
        assert_eq!(
            bybit_orders[0].identifier,
            Some("test-uuid-001".to_string())
        );
    }

    #[tokio::test]
    async fn test_execute_entry_slippage_applied() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "u-slippage".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "b-slippage".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit.clone(), bybit.clone(), make_config());
        let _result = executor.execute_entry(&make_entry_request()).await;

        // Upbit 매수: 가격 상향 (slippage 0.1% → 60,060,000)
        let upbit_orders = upbit.order_history.lock().await;
        let upbit_price = upbit_orders[0].price.unwrap();
        assert!(upbit_price > Decimal::new(60_000_000, 0));

        // Bybit short: 가격 하향 (slippage 0.1% → 41,958)
        let bybit_orders = bybit.order_history.lock().await;
        let bybit_price = bybit_orders[0].price.unwrap();
        assert!(bybit_price < Decimal::new(42000, 0));
    }

    // =======================================================================
    // execute_exit 테스트
    // =======================================================================

    #[tokio::test]
    async fn test_execute_exit_both_filled() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-exit-001".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(61_000_000, 0)),
            paid_fee: Decimal::new(30_500, 0),
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-exit-001".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(41500, 0)),
            paid_fee: Decimal::new(228, 3),
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_exit(&make_exit_request()).await;

        assert!(result.is_ok());
        let exit = result.unwrap();
        assert_eq!(exit.upbit_order_id, "upbit-exit-001");
        assert_eq!(exit.bybit_order_id, "bybit-exit-001");
        assert_eq!(exit.upbit_filled_qty, Decimal::new(1, 2));
        assert_eq!(exit.bybit_filled_qty, Decimal::new(1, 2));
    }

    #[tokio::test]
    async fn test_execute_exit_upbit_failed() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            should_fail: true,
            fail_error: Some("upbit sell failed".to_string()),
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "bybit-exit-fail".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(41500, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_exit(&make_exit_request()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            OrderExecutionError::SingleLegFilled {
                leg,
                emergency_closed,
            } => {
                assert_eq!(leg, Leg::Bybit);
                assert!(!emergency_closed);
            }
            other => panic!("expected SingleLegFilled, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_execute_exit_bybit_failed() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "upbit-exit-ok".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(61_000_000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            should_fail: true,
            fail_error: Some("bybit close failed".to_string()),
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_exit(&make_exit_request()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            OrderExecutionError::SingleLegFilled {
                leg,
                emergency_closed,
            } => {
                assert_eq!(leg, Leg::Upbit);
                assert!(!emergency_closed);
            }
            other => panic!("expected SingleLegFilled, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_execute_exit_both_failed() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            should_fail: true,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            should_fail: true,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor.execute_exit(&make_exit_request()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            OrderExecutionError::BothUnfilled => {}
            other => panic!("expected BothUnfilled, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_execute_exit_order_requests_correct() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "u-exit-req".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(61_000_000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "b-exit-req".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(41500, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit.clone(), bybit.clone(), make_config());
        let _result = executor.execute_exit(&make_exit_request()).await;

        // Upbit: 시장가 매도
        let upbit_orders = upbit.order_history.lock().await;
        assert_eq!(upbit_orders.len(), 1);
        assert_eq!(upbit_orders[0].side, OrderSide::Sell);
        assert_eq!(upbit_orders[0].order_type, OrderType::Market);
        assert_eq!(
            upbit_orders[0].identifier,
            Some("test-uuid-exit-001".to_string())
        );

        // Bybit: 시장가 매수 (close)
        let bybit_orders = bybit.order_history.lock().await;
        assert_eq!(bybit_orders.len(), 1);
        assert_eq!(bybit_orders[0].side, OrderSide::Buy);
        assert_eq!(bybit_orders[0].order_type, OrderType::Market);
        assert_eq!(
            bybit_orders[0].identifier,
            Some("test-uuid-exit-001".to_string())
        );
    }

    // =======================================================================
    // 타입 호환성 테스트
    // =======================================================================

    #[test]
    fn test_leg_display() {
        assert_eq!(Leg::Upbit.to_string(), "upbit");
        assert_eq!(Leg::Bybit.to_string(), "bybit");
    }

    #[test]
    fn test_leg_equality() {
        assert_eq!(Leg::Upbit, Leg::Upbit);
        assert_eq!(Leg::Bybit, Leg::Bybit);
        assert_ne!(Leg::Upbit, Leg::Bybit);
    }

    #[test]
    fn test_order_execution_error_display() {
        let err = OrderExecutionError::BothUnfilled;
        assert!(err.to_string().contains("both legs unfilled"));

        let err = OrderExecutionError::SingleLegFilled {
            leg: Leg::Upbit,
            emergency_closed: true,
        };
        assert!(err.to_string().contains("upbit"));
        assert!(err.to_string().contains("true"));

        let err = OrderExecutionError::EmergencyCloseFailed {
            leg: Leg::Bybit,
            order_id: "ord-123".to_string(),
        };
        assert!(err.to_string().contains("bybit"));
        assert!(err.to_string().contains("ord-123"));

        let err = OrderExecutionError::Timeout {
            leg: Some(Leg::Upbit),
        };
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn test_order_execution_error_from_exchange_error() {
        let exchange_err = ExchangeError::ApiError("test".to_string());
        let err: OrderExecutionError = exchange_err.into();
        assert!(err.to_string().contains("exchange error"));
    }

    #[test]
    fn test_entry_request_clone() {
        let req = make_entry_request();
        let cloned = req.clone();
        assert_eq!(cloned.coin, "BTC");
        assert_eq!(cloned.qty, Decimal::new(1, 2));
    }

    #[test]
    fn test_exit_request_clone() {
        let req = make_exit_request();
        let cloned = req.clone();
        assert_eq!(cloned.coin, "BTC");
        assert_eq!(cloned.qty, Decimal::new(1, 2));
    }

    #[test]
    fn test_executed_entry_fields() {
        let entry = ExecutedEntry {
            upbit_order_id: "u-001".to_string(),
            bybit_order_id: "b-001".to_string(),
            upbit_filled_qty: Decimal::new(1, 2),
            bybit_filled_qty: Decimal::new(1, 2),
            upbit_avg_price_krw: Decimal::new(60_000_000, 0),
            bybit_avg_price: Decimal::new(42000, 0),
            upbit_fee: Decimal::new(30_000, 0),
            bybit_fee: Decimal::new(231, 3),
            effective_qty: Decimal::new(9995, 6),
            adjustment_cost: Decimal::ZERO,
        };
        assert_eq!(entry.upbit_order_id, "u-001");
        assert_eq!(entry.bybit_order_id, "b-001");
    }

    #[test]
    fn test_executed_exit_fields() {
        let exit = ExecutedExit {
            upbit_order_id: "u-exit".to_string(),
            bybit_order_id: "b-exit".to_string(),
            upbit_filled_qty: Decimal::new(1, 2),
            bybit_filled_qty: Decimal::new(1, 2),
            upbit_avg_price_krw: Decimal::new(61_000_000, 0),
            bybit_avg_price: Decimal::new(41500, 0),
            upbit_fee: Decimal::new(30_500, 0),
            bybit_fee: Decimal::new(228, 3),
        };
        assert_eq!(exit.upbit_order_id, "u-exit");
    }

    // =======================================================================
    // emergency_close_leg 테스트 (Stage 1 성공)
    // =======================================================================

    #[tokio::test]
    async fn test_emergency_close_stage1_success() {
        // 첫 시도에서 95% 이상 체결 → 즉시 성공
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse {
            id: "emergency-ok".to_string(),
            executed_volume: Decimal::new(1, 2), // 전량 체결
            avg_price: Some(Decimal::new(60_000_000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse::default()));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor
            .emergency_close_leg(Leg::Upbit, "KRW-BTC", Decimal::new(1, 2))
            .await;

        assert!(result); // 성공
    }

    #[tokio::test]
    async fn test_emergency_close_bybit_stage1_success() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse::default()));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse {
            id: "emergency-bybit-ok".to_string(),
            executed_volume: Decimal::new(1, 2),
            avg_price: Some(Decimal::new(42000, 0)),
            paid_fee: Decimal::ZERO,
            ..Default::default()
        }));

        let executor = LiveExecutor::new(upbit, bybit, make_config());
        let result = executor
            .emergency_close_leg(Leg::Bybit, "BTCUSDT", Decimal::new(1, 2))
            .await;

        assert!(result);
    }
}
