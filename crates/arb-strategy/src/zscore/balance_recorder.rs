//! 잔고 스냅샷 비동기 파이프라인.
//!
//! 전략 이벤트 루프와 완전히 분리된 background task에서
//! 거래소 실잔고를 REST API로 조회하여 DB에 기록합니다.
//!
//! # 아키텍처
//!
//! ```text
//! 전략 이벤트 루프 -> mpsc(32) -> BalanceRecorderTask -> DbWriter -> DB
//! ```
//!
//! - 전략 측에서 `try_send`로 non-blocking 전송 (블로킹 절대 금지)
//! - background task에서 거래소 REST 잔고 조회 + 환율 환산 + Row 조립
//! - 기존 `DbWriter` 채널을 통해 INSERT 위임

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use arb_db::balance_snapshots::BalanceSnapshotRow;
use arb_db::writer::{DbWriteRequest, DbWriter};
use arb_exchange::adapter::ExchangeAdapter;
use arb_exchange::error::ExchangeError;
use arb_exchange::types::Balance;
use arb_forex::{ForexCache, UsdtKrwCache};
use chrono::Utc;
use rust_decimal::Decimal;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};

/// 잔고 스냅샷 요청 메시지.
#[derive(Debug)]
pub enum SnapshotMsg {
    /// 정기 기록 (타이머 트리거).
    Periodic,
    /// 포지션 진입 직후.
    PositionEntry {
        /// DB에 저장된 포지션 ID.
        position_id: i64,
    },
    /// 포지션 청산 직후.
    PositionExit {
        /// DB에 저장된 포지션 ID.
        position_id: i64,
    },
    /// 종료 요청.
    Shutdown,
}

/// 잔고 스냅샷 전송 핸들 (Clone 가능).
///
/// `try_send`로 전략 이벤트 루프를 절대 블로킹하지 않습니다.
/// `position_id <= 0`이면 전송을 스킵합니다 (DB INSERT 실패한 포지션).
#[derive(Clone, Debug)]
pub struct BalanceSnapshotSender {
    tx: mpsc::Sender<SnapshotMsg>,
}

impl BalanceSnapshotSender {
    /// 포지션 진입 시 스냅샷 요청 (non-blocking).
    ///
    /// `position_id <= 0`이면 전송하지 않습니다.
    /// `try_send` 실패 시 warn 로그만 남기고 드롭합니다.
    ///
    /// 반환값: `true`이면 메시지가 드롭됨 (호출자가 카운터 증가 필요).
    pub fn on_position_entry(&self, position_id: i64) -> bool {
        if position_id <= 0 {
            debug!(
                position_id = position_id,
                "잔고 스냅샷 전송 스킵: position_id <= 0"
            );
            return false;
        }
        if let Err(e) = self.tx.try_send(SnapshotMsg::PositionEntry { position_id }) {
            warn!(
                position_id = position_id,
                error = %e,
                "잔고 스냅샷 PositionEntry 전송 실패 (채널 가득 참)"
            );
            return true;
        }
        false
    }

    /// 포지션 청산 시 스냅샷 요청 (non-blocking).
    ///
    /// `position_id <= 0`이면 전송하지 않습니다.
    /// `try_send` 실패 시 warn 로그만 남기고 드롭합니다.
    ///
    /// 반환값: `true`이면 메시지가 드롭됨 (호출자가 카운터 증가 필요).
    pub fn on_position_exit(&self, position_id: i64) -> bool {
        if position_id <= 0 {
            debug!(
                position_id = position_id,
                "잔고 스냅샷 전송 스킵: position_id <= 0"
            );
            return false;
        }
        if let Err(e) = self.tx.try_send(SnapshotMsg::PositionExit { position_id }) {
            warn!(
                position_id = position_id,
                error = %e,
                "잔고 스냅샷 PositionExit 전송 실패 (채널 가득 참)"
            );
            return true;
        }
        false
    }

    /// 종료 요청 (30초 타임아웃).
    ///
    /// `send()` 실패 또는 타임아웃 시 warn 로그를 남깁니다.
    pub async fn shutdown(&self) {
        match tokio::time::timeout(Duration::from_secs(30), self.tx.send(SnapshotMsg::Shutdown))
            .await
        {
            Ok(Ok(())) => {
                debug!("BalanceRecorderTask Shutdown 메시지 전송 완료");
            }
            Ok(Err(e)) => {
                warn!(error = %e, "BalanceRecorderTask Shutdown 전송 실패 (수신자 drop)");
            }
            Err(_) => {
                warn!("BalanceRecorderTask Shutdown 전송 타임아웃 (30초)");
            }
        }
    }
}

/// 코인 현재가 캐시 (잔고 기록 전용, 1초 TTL).
///
/// 전략의 rate limit에 영향을 주지 않기 위해 1초 TTL 캐시를 유지합니다.
/// 캐시 히트 시 REST 호출을 생략하여 rate limit을 소비하지 않습니다.
struct TickerCache {
    /// coin -> (price_krw, updated_at).
    prices: HashMap<String, (Decimal, Instant)>,
    /// 캐시 TTL (기본 1초).
    ttl: Duration,
}

impl TickerCache {
    /// 1초 TTL의 새 TickerCache를 생성합니다.
    fn new() -> Self {
        Self {
            prices: HashMap::new(),
            ttl: Duration::from_secs(1),
        }
    }

    /// 지정된 TTL로 새 TickerCache를 생성합니다 (테스트용).
    #[cfg(test)]
    fn with_ttl(ttl: Duration) -> Self {
        Self {
            prices: HashMap::new(),
            ttl,
        }
    }

    /// 코인 현재가를 캐시에서 조회하거나 거래소에서 fetch합니다.
    ///
    /// 캐시 히트(TTL 이내) 시 캐시된 가격을 사용합니다.
    /// 캐시 미스 시 `upbit.get_ticker()` 1회 호출로 전체 갱신합니다.
    /// fetch 실패 시 stale 캐시를 사용하고 warn 로그를 남깁니다.
    async fn get_or_fetch(
        &mut self,
        coins: &[String],
        upbit: &dyn ExchangeAdapter,
    ) -> HashMap<String, Decimal> {
        if coins.is_empty() {
            return HashMap::new();
        }

        let now = Instant::now();
        let mut result = HashMap::new();
        let mut need_fetch = Vec::new();

        // 캐시 히트 확인
        for coin in coins {
            if let Some((price, updated_at)) = self.prices.get(coin) {
                if now.duration_since(*updated_at) < self.ttl {
                    result.insert(coin.clone(), *price);
                } else {
                    need_fetch.push(coin.clone());
                }
            } else {
                need_fetch.push(coin.clone());
            }
        }

        // 모든 코인이 캐시 히트이면 바로 반환
        if need_fetch.is_empty() {
            return result;
        }

        // 모든 코인에 대해 한 번에 조회 (캐시 미스가 하나라도 있으면 전체 fetch)
        let markets: Vec<String> = coins.iter().map(|c| format!("KRW-{c}")).collect();
        let market_refs: Vec<&str> = markets.iter().map(|s| s.as_str()).collect();

        match upbit.get_ticker(&market_refs).await {
            Ok(tickers) => {
                let fetch_time = Instant::now();
                for ticker in &tickers {
                    // "KRW-BTC" -> "BTC"
                    if let Some(coin) = ticker.market.strip_prefix("KRW-") {
                        let coin = coin.to_string();
                        self.prices
                            .insert(coin.clone(), (ticker.trade_price, fetch_time));
                        result.insert(coin, ticker.trade_price);
                    }
                }
                debug!(count = tickers.len(), "TickerCache: 코인 현재가 fetch 완료");
            }
            Err(e) => {
                warn!(
                    error = %e,
                    "TickerCache: 코인 현재가 fetch 실패, stale 캐시 사용"
                );
                // fetch 실패 시 stale 캐시에서 가격 사용
                for coin in &need_fetch {
                    if let Some((price, _)) = self.prices.get(coin) {
                        result.insert(coin.clone(), *price);
                    }
                }
            }
        }

        result
    }
}

/// 잔고 스냅샷 background task.
///
/// `mpsc::Receiver<SnapshotMsg>`에서 메시지를 수신하고,
/// 거래소 REST API로 잔고를 조회하여 DB에 기록합니다.
///
/// 단일 consumer task이므로 `&mut self`로 내부 상태에 접근 가능합니다.
pub struct BalanceRecorderTask {
    /// 메시지 수신 채널.
    rx: mpsc::Receiver<SnapshotMsg>,
    /// 현재 세션 ID.
    session_id: i64,
    /// Upbit 거래소 어댑터 (잔고 + 시세 조회).
    upbit: Arc<dyn ExchangeAdapter>,
    /// Bybit 거래소 어댑터 (잔고 조회).
    bybit: Arc<dyn ExchangeAdapter>,
    /// USD/KRW 공시 환율 캐시.
    forex: Arc<ForexCache>,
    /// USDT/KRW 거래소 시세 캐시.
    usdt_cache: Arc<UsdtKrwCache>,
    /// DB writer (기존 채널 재활용).
    db_writer: DbWriter,
    /// 코인 현재가 1초 캐시.
    ticker_cache: TickerCache,
    /// snapshot_group_id 시퀀스 (단일 task이므로 Atomic 불필요).
    next_group_id: u64,
    /// 정기 기록 주기 (초).
    interval_sec: u64,
}

impl BalanceRecorderTask {
    /// BalanceRecorderTask를 생성하고 tokio::spawn으로 시작합니다.
    ///
    /// # 인자
    ///
    /// * `session_id` - 현재 세션 ID
    /// * `upbit` - Upbit 거래소 어댑터
    /// * `bybit` - Bybit 거래소 어댑터
    /// * `forex` - USD/KRW 환율 캐시
    /// * `usdt_cache` - USDT/KRW 시세 캐시
    /// * `db_writer` - DB writer 채널
    /// * `interval_sec` - 정기 기록 주기 (초)
    ///
    /// # 반환값
    ///
    /// `(BalanceSnapshotSender, JoinHandle<()>)` 튜플.
    #[allow(clippy::too_many_arguments)]
    pub fn spawn(
        session_id: i64,
        upbit: Arc<dyn ExchangeAdapter>,
        bybit: Arc<dyn ExchangeAdapter>,
        forex: Arc<ForexCache>,
        usdt_cache: Arc<UsdtKrwCache>,
        db_writer: DbWriter,
        interval_sec: u64,
    ) -> (BalanceSnapshotSender, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel(32);

        let mut task = Self {
            rx,
            session_id,
            upbit,
            bybit,
            forex,
            usdt_cache,
            db_writer,
            ticker_cache: TickerCache::new(),
            next_group_id: 0,
            interval_sec,
        };

        let handle = tokio::spawn(async move {
            task.run().await;
        });

        let sender = BalanceSnapshotSender { tx };
        (sender, handle)
    }

    /// 메인 이벤트 루프.
    ///
    /// `select! biased`로 채널 메시지를 interval보다 우선 처리합니다.
    /// `Shutdown` 수신 또는 채널 닫힘 시 최종 Periodic 스냅샷을 기록하고 종료합니다.
    async fn run(&mut self) {
        info!(
            session_id = self.session_id,
            interval_sec = self.interval_sec,
            "BalanceRecorderTask 시작"
        );

        let mut interval = tokio::time::interval(Duration::from_secs(self.interval_sec));
        // 첫 번째 즉시 tick 소비 (interval 첫 tick은 즉시 발생)
        interval.tick().await;

        loop {
            tokio::select! {
                biased;

                msg = self.rx.recv() => {
                    match msg {
                        Some(SnapshotMsg::Shutdown) | None => {
                            // 종료 전 최종 스냅샷 기록
                            debug!("BalanceRecorderTask: 종료 전 최종 스냅샷 기록");
                            self.handle_snapshot(SnapshotMsg::Periodic).await;
                            break;
                        }
                        Some(msg) => self.handle_snapshot(msg).await,
                    }
                }
                _ = interval.tick() => {
                    self.handle_snapshot(SnapshotMsg::Periodic).await;
                }
            }
        }

        info!("BalanceRecorderTask 종료");
    }

    /// 스냅샷 메시지를 처리합니다.
    ///
    /// 1. snapshot_group_id 채번 + created_at 생성
    /// 2. tokio::join!으로 양 거래소 잔고 동시 조회
    /// 3. 환율 조회 + 환산값 계산
    /// 4. Row 조립 -> db_writer.send()
    async fn handle_snapshot(&mut self, msg: SnapshotMsg) {
        // 1. snapshot_group_id 채번
        self.next_group_id += 1;
        let group_id = self.next_group_id as i64;
        let created_at = Utc::now();

        // 2. record_type, position_id 결정
        let (record_type, position_id) = match &msg {
            SnapshotMsg::Periodic => ("PERIODIC", None),
            SnapshotMsg::PositionEntry { position_id } => ("POS_ENT", Some(*position_id)),
            SnapshotMsg::PositionExit { position_id } => ("POS_EXT", Some(*position_id)),
            SnapshotMsg::Shutdown => ("PERIODIC", None), // Shutdown은 최종 Periodic으로 처리
        };

        debug!(
            group_id = group_id,
            record_type = record_type,
            position_id = ?position_id,
            "잔고 스냅샷 처리 시작"
        );

        // 3. 양 거래소 잔고 동시 조회 (retryable 에러 시 1회 재시도, 500ms 딜레이)
        let (upbit_result, bybit_result) = tokio::join!(
            fetch_balances_with_retry(self.upbit.as_ref(), "Upbit"),
            fetch_balances_with_retry(self.bybit.as_ref(), "Bybit")
        );

        // 4. 환율 조회
        let usd_krw = self.forex.get_cached_rate().unwrap_or_else(|| {
            warn!("USD/KRW 환율 캐시 비어있음, 0.0 사용");
            0.0
        });

        let usdt_krw = self
            .usdt_cache
            .get_usdt_krw()
            .or_else(|| {
                // TTL 만료 시 stale 값 사용
                let stale = self.usdt_cache.get_usdt_krw_with_stale();
                if stale.is_some() {
                    warn!("USDT/KRW 캐시 TTL 만료, stale 값 사용");
                }
                stale
            })
            .unwrap_or_else(|| {
                warn!("USDT/KRW 캐시 비어있음, 0.0 사용");
                0.0
            });

        // 5. Upbit 처리
        match upbit_result {
            Ok(balances) => {
                debug!(
                    count = balances.len(),
                    currencies = ?balances.iter().map(|b| b.currency.as_str()).collect::<Vec<_>>(),
                    "Upbit 잔고 조회 성공"
                );
                match self
                    .build_upbit_row(
                        &balances,
                        group_id,
                        created_at,
                        record_type,
                        position_id,
                        usd_krw,
                        usdt_krw,
                    )
                    .await
                {
                    Some(row) => {
                        debug!(
                            cex = "UPBIT",
                            total = %row.total,
                            "Upbit 잔고 스냅샷 Row 조립 완료"
                        );
                        self.db_writer
                            .send(DbWriteRequest::InsertBalanceSnapshot(row));
                    }
                    None => {
                        warn!("Upbit 잔고 스냅샷 Row 조립 실패 (KRW 항목 없음)");
                    }
                }
            }
            Err(e) => {
                warn!(
                    error = %e,
                    "Upbit 잔고 조회 실패, Upbit 행 스킵 (partial snapshot)"
                );
            }
        }

        // 6. Bybit 처리
        match bybit_result {
            Ok(balances) => {
                debug!(
                    count = balances.len(),
                    currencies = ?balances.iter().map(|b| b.currency.as_str()).collect::<Vec<_>>(),
                    "Bybit 잔고 조회 성공"
                );
                match Self::build_bybit_row(
                    &balances,
                    self.session_id,
                    group_id,
                    created_at,
                    record_type,
                    position_id,
                    usd_krw,
                    usdt_krw,
                ) {
                    Some(row) => {
                        debug!(
                            cex = "BYBIT",
                            total = %row.total,
                            "Bybit 잔고 스냅샷 Row 조립 완료"
                        );
                        self.db_writer
                            .send(DbWriteRequest::InsertBalanceSnapshot(row));
                    }
                    None => {
                        warn!("Bybit 잔고 스냅샷 Row 조립 실패 (USDT 항목 없음)");
                    }
                }
            }
            Err(e) => {
                warn!(
                    error = %e,
                    "Bybit 잔고 조회 실패, Bybit 행 스킵 (partial snapshot)"
                );
            }
        }

        debug!(
            group_id = group_id,
            record_type = record_type,
            "잔고 스냅샷 처리 완료"
        );
    }

    /// Upbit 잔고에서 BalanceSnapshotRow를 조립합니다.
    ///
    /// - KRW 항목: available, locked 추출
    /// - 보유 코인: dust 필터(avg_buy_price * (balance+locked) < 1000 KRW) 적용
    /// - coin_value: TickerCache로 현재가 조회 후 qty * price 합산
    /// - total = available + locked + coin_value
    #[allow(clippy::too_many_arguments)]
    async fn build_upbit_row(
        &mut self,
        balances: &[Balance],
        group_id: i64,
        created_at: chrono::DateTime<Utc>,
        record_type: &str,
        position_id: Option<i64>,
        usd_krw: f64,
        usdt_krw: f64,
    ) -> Option<BalanceSnapshotRow> {
        // KRW 항목 찾기
        let krw = balances.iter().find(|b| b.currency == "KRW")?;
        let available = krw.balance;
        let locked = krw.locked;

        // 보유 코인 목록 추출 (dust 필터)
        let threshold_krw = Decimal::new(1000, 0);
        let mut coin_holdings: Vec<(String, Decimal)> = Vec::new();

        for b in balances {
            if b.currency == "KRW" {
                continue;
            }
            let qty = b.balance + b.locked;
            let est_value = b.avg_buy_price * qty;
            if est_value >= threshold_krw {
                coin_holdings.push((b.currency.clone(), qty));
            }
        }

        // coin_value 계산
        let coin_value = if coin_holdings.is_empty() {
            Decimal::ZERO
        } else {
            let coin_names: Vec<String> = coin_holdings.iter().map(|(c, _)| c.clone()).collect();
            let prices = self
                .ticker_cache
                .get_or_fetch(&coin_names, self.upbit.as_ref())
                .await;

            let mut value = Decimal::ZERO;
            for (coin, qty) in &coin_holdings {
                if let Some(price) = prices.get(coin) {
                    value += *qty * *price;
                } else {
                    warn!(coin = %coin, "코인 현재가 조회 실패, coin_value 누락");
                }
            }
            value
        };

        let total = available + locked + coin_value;

        // 환산
        let (total_usd, total_usdt) = convert_krw_to_usd_usdt(total, usd_krw, usdt_krw);

        Some(BalanceSnapshotRow {
            created_at,
            snapshot_group_id: group_id,
            session_id: self.session_id,
            record_type: record_type.to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available,
            locked,
            coin_value,
            total,
            position_id,
            usd_krw,
            usdt_krw,
            total_usd,
            total_usdt,
        })
    }

    /// Bybit 잔고에서 BalanceSnapshotRow를 조립합니다.
    ///
    /// - USDT 항목: equity -> total 직접 사용 (합산하지 않음)
    /// - available = balance (Bybit walletBalance 매핑)
    /// - locked = locked (Option -> ZERO)
    /// - coin_value = unrealised_pnl (Option -> ZERO, 포지션 가치 분해용)
    /// - total = equity (None이면 balance + coin_value fallback + warn)
    #[allow(clippy::too_many_arguments)]
    fn build_bybit_row(
        balances: &[Balance],
        session_id: i64,
        group_id: i64,
        created_at: chrono::DateTime<Utc>,
        record_type: &str,
        position_id: Option<i64>,
        usd_krw: f64,
        usdt_krw: f64,
    ) -> Option<BalanceSnapshotRow> {
        // USDT 항목 찾기
        let usdt = balances.iter().find(|b| b.currency == "USDT")?;

        let available = usdt.balance;
        let locked = usdt.locked;
        let coin_value = usdt.unrealised_pnl.unwrap_or(Decimal::ZERO);

        // total = equity 직접 사용 (합산하지 않음)
        let total = usdt.equity.unwrap_or_else(|| {
            warn!("Bybit equity가 None, balance + coin_value fallback 사용");
            usdt.balance + coin_value
        });

        // USDT = USD 가정: total_usd = total, total_usdt = total
        let total_usd = total.round_dp(8);
        let total_usdt = total.round_dp(8);

        Some(BalanceSnapshotRow {
            created_at,
            snapshot_group_id: group_id,
            session_id,
            record_type: record_type.to_string(),
            cex: "BYBIT".to_string(),
            currency: "USDT".to_string(),
            available,
            locked,
            coin_value,
            total,
            position_id,
            usd_krw,
            usdt_krw,
            total_usd,
            total_usdt,
        })
    }
}

/// 거래소 잔고를 조회하고, retryable 에러 시 1회 재시도합니다 (500ms 딜레이).
///
/// 재시도 대상: `ExchangeError::is_retryable() == true` (네트워크/5xx/rate limit 등).
/// 비-retryable 에러(인증 실패, 잔고 부족 등)는 즉시 반환합니다.
async fn fetch_balances_with_retry(
    adapter: &dyn ExchangeAdapter,
    cex_name: &str,
) -> Result<Vec<Balance>, ExchangeError> {
    match adapter.get_balances().await {
        Ok(balances) => Ok(balances),
        Err(first_err) if first_err.is_retryable() => {
            warn!(
                cex = cex_name,
                error = %first_err,
                "잔고 조회 실패 (retryable), 500ms 후 1회 재시도"
            );
            tokio::time::sleep(Duration::from_millis(500)).await;
            match adapter.get_balances().await {
                Ok(balances) => {
                    debug!(cex = cex_name, "잔고 재시도 성공");
                    Ok(balances)
                }
                Err(retry_err) => {
                    warn!(
                        cex = cex_name,
                        error = %retry_err,
                        "잔고 재시도 실패, 행 스킵"
                    );
                    Err(retry_err)
                }
            }
        }
        Err(err) => {
            // 비-retryable 에러는 즉시 반환 (재시도 무의미)
            Err(err)
        }
    }
}

/// KRW 금액을 USD, USDT로 환산합니다.
///
/// 0 나누기 가드 + f64->Decimal 변환 방어 포함.
/// 변환 실패 시 0으로 기록하고 warn 로그를 남깁니다.
/// 최종 결과는 소수점 8자리로 truncation합니다.
fn convert_krw_to_usd_usdt(total_krw: Decimal, usd_krw: f64, usdt_krw: f64) -> (Decimal, Decimal) {
    let total_usd = convert_krw_to_foreign(total_krw, usd_krw, "USD");
    let total_usdt = convert_krw_to_foreign(total_krw, usdt_krw, "USDT");
    (total_usd, total_usdt)
}

/// KRW 금액을 지정 외화로 환산합니다 (내부 헬퍼).
///
/// - rate == 0.0이면 0 반환 + warn
/// - f64->Decimal 변환 실패(NaN/Infinity)이면 0 반환 + warn
/// - 최종 결과 `.round_dp(8)`
fn convert_krw_to_foreign(total_krw: Decimal, rate: f64, currency_name: &str) -> Decimal {
    // 0 나누기 가드
    if rate == 0.0 {
        warn!(
            currency = currency_name,
            rate = rate,
            "exchange rate is zero, conversion result set to 0"
        );
        return Decimal::ZERO;
    }

    // f64 -> Decimal 변환 방어
    let rate_decimal = match Decimal::from_f64_retain(rate) {
        Some(d) => d,
        None => {
            warn!(
                currency = currency_name,
                rate = rate,
                "f64->Decimal conversion failed (NaN/Infinity), conversion result set to 0"
            );
            return Decimal::ZERO;
        }
    };

    // 0 나누기 재확인 (Decimal level)
    if rate_decimal == Decimal::ZERO {
        warn!(
            currency = currency_name,
            "Decimal rate is zero after conversion, result set to 0"
        );
        return Decimal::ZERO;
    }

    (total_krw / rate_decimal).round_dp(8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arb_exchange::error::{ExchangeError, ExchangeResult};
    use arb_exchange::types::{
        Balance, Candle, CandleInterval, Order, OrderBook, OrderRequest, Ticker,
    };
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use std::sync::Mutex;

    // === Mock ExchangeAdapter ===

    /// 테스트용 mock 거래소 어댑터.
    #[derive(Debug)]
    struct MockAdapter {
        name: String,
        balances: Mutex<ExchangeResult<Vec<Balance>>>,
        tickers: Mutex<ExchangeResult<Vec<Ticker>>>,
    }

    #[allow(dead_code)]
    impl MockAdapter {
        fn new_upbit(balances: Vec<Balance>, tickers: Vec<Ticker>) -> Self {
            Self {
                name: "Upbit".to_string(),
                balances: Mutex::new(Ok(balances)),
                tickers: Mutex::new(Ok(tickers)),
            }
        }

        fn new_bybit(balances: Vec<Balance>) -> Self {
            Self {
                name: "Bybit".to_string(),
                balances: Mutex::new(Ok(balances)),
                tickers: Mutex::new(Ok(vec![])),
            }
        }

        fn new_failing(name: &str) -> Self {
            Self {
                name: name.to_string(),
                balances: Mutex::new(Err(ExchangeError::InternalError(
                    "mock network error".to_string(),
                ))),
                tickers: Mutex::new(Err(ExchangeError::InternalError(
                    "mock network error".to_string(),
                ))),
            }
        }
    }

    #[async_trait]
    impl ExchangeAdapter for MockAdapter {
        fn name(&self) -> &str {
            &self.name
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn native_quote_currency(&self) -> &str {
            if self.name == "Upbit" { "KRW" } else { "USDT" }
        }

        async fn get_ticker(&self, _markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            let guard = self.tickers.lock().unwrap();
            match &*guard {
                Ok(v) => Ok(v.clone()),
                Err(e) => Err(ExchangeError::InternalError(e.to_string())),
            }
        }

        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn get_candles(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
            _before: DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn place_order(&self, _request: &OrderRequest) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn cancel_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn get_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn get_open_orders(&self, _market: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }

        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            let guard = self.balances.lock().unwrap();
            match &*guard {
                Ok(v) => Ok(v.clone()),
                Err(e) => Err(ExchangeError::InternalError(e.to_string())),
            }
        }

        async fn get_balance(&self, _currency: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::Unsupported(
                "not implemented for mock".into(),
            ))
        }
    }

    // === 헬퍼 함수 ===

    /// KRW Balance 생성 헬퍼.
    fn make_krw_balance(available: i64, locked: i64) -> Balance {
        Balance {
            currency: "KRW".to_string(),
            balance: Decimal::new(available, 0),
            locked: Decimal::new(locked, 0),
            avg_buy_price: Decimal::ZERO,
            unit_currency: "KRW".to_string(),
            equity: None,
            unrealised_pnl: None,
        }
    }

    /// 코인 Balance 생성 헬퍼.
    fn make_coin_balance(
        currency: &str,
        balance: Decimal,
        locked: Decimal,
        avg_buy_price: Decimal,
    ) -> Balance {
        Balance {
            currency: currency.to_string(),
            balance,
            locked,
            avg_buy_price,
            unit_currency: "KRW".to_string(),
            equity: None,
            unrealised_pnl: None,
        }
    }

    /// USDT Balance 생성 헬퍼 (Bybit).
    fn make_usdt_balance(
        available: Decimal,
        locked: Decimal,
        equity: Option<Decimal>,
        unrealised_pnl: Option<Decimal>,
    ) -> Balance {
        Balance {
            currency: "USDT".to_string(),
            balance: available,
            locked,
            avg_buy_price: Decimal::ZERO,
            unit_currency: "USDT".to_string(),
            equity,
            unrealised_pnl,
        }
    }

    /// Ticker 생성 헬퍼.
    fn make_ticker(market: &str, trade_price: Decimal) -> Ticker {
        use arb_exchange::types::PriceChange;
        Ticker {
            market: market.to_string(),
            trade_price,
            opening_price: trade_price,
            high_price: trade_price,
            low_price: trade_price,
            prev_closing_price: trade_price,
            change: PriceChange::Even,
            change_rate: Decimal::ZERO,
            change_price: Decimal::ZERO,
            acc_trade_volume_24h: Decimal::ZERO,
            acc_trade_price_24h: Decimal::ZERO,
            timestamp: Utc::now(),
        }
    }

    // === 테스트 ===

    #[test]
    fn test_snapshot_msg_variants() {
        // SnapshotMsg 생성/매칭 테스트
        let periodic = SnapshotMsg::Periodic;
        let entry = SnapshotMsg::PositionEntry { position_id: 42 };
        let exit = SnapshotMsg::PositionExit { position_id: 99 };
        let shutdown = SnapshotMsg::Shutdown;

        // 매칭 확인
        assert!(matches!(periodic, SnapshotMsg::Periodic));
        assert!(matches!(
            entry,
            SnapshotMsg::PositionEntry { position_id: 42 }
        ));
        assert!(matches!(
            exit,
            SnapshotMsg::PositionExit { position_id: 99 }
        ));
        assert!(matches!(shutdown, SnapshotMsg::Shutdown));
    }

    #[test]
    fn test_balance_snapshot_sender_skip_invalid_position_id() {
        // position_id <= 0이면 send하지 않아야 함
        let (tx, mut rx) = mpsc::channel(32);
        let sender = BalanceSnapshotSender { tx };

        // position_id = 0 -> 전송하지 않음
        sender.on_position_entry(0);
        sender.on_position_exit(0);

        // position_id = -1 -> 전송하지 않음
        sender.on_position_entry(-1);
        sender.on_position_exit(-1);

        // 채널에 아무것도 없어야 함
        assert!(rx.try_recv().is_err());

        // position_id > 0 -> 전송
        sender.on_position_entry(1);
        sender.on_position_exit(2);

        let msg1 = rx.try_recv().unwrap();
        assert!(matches!(
            msg1,
            SnapshotMsg::PositionEntry { position_id: 1 }
        ));

        let msg2 = rx.try_recv().unwrap();
        assert!(matches!(msg2, SnapshotMsg::PositionExit { position_id: 2 }));
    }

    #[tokio::test]
    async fn test_ticker_cache_ttl() {
        // 캐시 히트/미스 동작 (Instant 기반)
        let mut cache = TickerCache::with_ttl(Duration::from_secs(60));

        // 수동으로 캐시에 가격 주입
        cache.prices.insert(
            "BTC".to_string(),
            (Decimal::new(95_000_000, 0), Instant::now()),
        );

        // 캐시 히트: TTL 이내이므로 fetch하지 않음
        let mock_upbit = MockAdapter::new_upbit(vec![], vec![]);
        let result = cache.get_or_fetch(&["BTC".to_string()], &mock_upbit).await;
        assert_eq!(result.get("BTC"), Some(&Decimal::new(95_000_000, 0)));

        // 캐시 미스: 새 코인은 fetch 필요
        let mock_upbit = MockAdapter::new_upbit(
            vec![],
            vec![make_ticker("KRW-ETH", Decimal::new(4_500_000, 0))],
        );
        let result = cache.get_or_fetch(&["ETH".to_string()], &mock_upbit).await;
        assert_eq!(result.get("ETH"), Some(&Decimal::new(4_500_000, 0)));
    }

    #[tokio::test]
    async fn test_ticker_cache_empty_coins() {
        // 빈 코인 목록이면 빈 결과 반환
        let mut cache = TickerCache::new();
        let mock_upbit = MockAdapter::new_upbit(vec![], vec![]);
        let result = cache.get_or_fetch(&[], &mock_upbit).await;
        assert!(result.is_empty());
    }

    #[test]
    fn test_balance_snapshot_config_default() {
        // BalanceSnapshotConfig 기본값 테스트
        use super::super::config::BalanceSnapshotConfig;
        let config = BalanceSnapshotConfig::default();
        assert_eq!(config.interval_sec, 600);
    }

    #[test]
    fn test_build_upbit_row_dust_filter() {
        // 1000 KRW 미만 코인 필터링 테스트
        // Upbit 잔고: KRW + BTC(고가, 필터 통과) + DOGE(저가*소량 < 1000, 필터됨)
        let balances = vec![
            make_krw_balance(1_000_000, 50_000),
            make_coin_balance(
                "BTC",
                Decimal::new(1, 3), // 0.001 BTC
                Decimal::ZERO,
                Decimal::new(95_000_000, 0), // avg_buy_price = 95,000,000
            ),
            make_coin_balance(
                "DOGE",
                Decimal::new(5, 0), // 5 DOGE
                Decimal::ZERO,
                Decimal::new(100, 0), // avg_buy_price = 100 KRW -> 500 < 1000 -> 필터됨
            ),
        ];

        // dust 필터 검증: BTC는 통과, DOGE는 제외
        let threshold_krw = Decimal::new(1000, 0);
        let mut coin_holdings: Vec<(String, Decimal)> = Vec::new();

        for b in &balances {
            if b.currency == "KRW" {
                continue;
            }
            let qty = b.balance + b.locked;
            let est_value = b.avg_buy_price * qty;
            if est_value >= threshold_krw {
                coin_holdings.push((b.currency.clone(), qty));
            }
        }

        // BTC: 0.001 * 95,000,000 = 95,000 >= 1000 -> 포함
        // DOGE: 5 * 100 = 500 < 1000 -> 제외
        assert_eq!(coin_holdings.len(), 1);
        assert_eq!(coin_holdings[0].0, "BTC");
        assert_eq!(coin_holdings[0].1, Decimal::new(1, 3));
    }

    #[test]
    fn test_build_bybit_row_equity_direct() {
        // equity 직접 사용 확인: available + locked + coin_value != total
        let balances = vec![make_usdt_balance(
            Decimal::new(3000, 0),       // available = 3000
            Decimal::new(500, 0),        // locked = 500
            Some(Decimal::new(5050, 0)), // equity = 5050 (ground truth)
            Some(Decimal::new(-50, 0)),  // unrealised_pnl = -50
        )];

        let row = BalanceRecorderTask::build_bybit_row(
            &balances,
            100, // session_id
            1,   // group_id
            Utc::now(),
            "PERIODIC",
            None,
            1350.0, // usd_krw
            1380.0, // usdt_krw
        );

        let row = row.unwrap();
        assert_eq!(row.cex, "BYBIT");
        assert_eq!(row.currency, "USDT");
        // total = equity 직접 사용 (3000 + 500 + (-50) = 3450 != 5050)
        assert_eq!(row.total, Decimal::new(5050, 0));
        assert_eq!(row.available, Decimal::new(3000, 0));
        assert_eq!(row.locked, Decimal::new(500, 0));
        assert_eq!(row.coin_value, Decimal::new(-50, 0));
        // USDT = USD 가정
        assert_eq!(row.total_usd, Decimal::new(5050, 0).round_dp(8));
        assert_eq!(row.total_usdt, Decimal::new(5050, 0).round_dp(8));
    }

    #[test]
    fn test_bybit_row_equity_none_fallback() {
        // equity가 None이면 balance + coin_value로 fallback
        let balances = vec![make_usdt_balance(
            Decimal::new(3000, 0), // available = 3000
            Decimal::new(500, 0),  // locked = 500
            None,                  // equity = None -> fallback
            None,                  // unrealised_pnl = None -> ZERO
        )];

        let row = BalanceRecorderTask::build_bybit_row(
            &balances,
            100,
            1,
            Utc::now(),
            "PERIODIC",
            None,
            1350.0,
            1380.0,
        );

        let row = row.unwrap();
        // total = balance + coin_value = 3000 + 0 = 3000 (fallback)
        assert_eq!(row.total, Decimal::new(3000, 0));
        assert_eq!(row.coin_value, Decimal::ZERO);
    }

    #[test]
    fn test_conversion_zero_division_guard() {
        // usd_krw = 0일 때 total_usd = 0
        let total_krw = Decimal::new(1_000_000, 0);

        let (total_usd, _) = convert_krw_to_usd_usdt(total_krw, 0.0, 1380.0);
        assert_eq!(total_usd, Decimal::ZERO);

        let (_, total_usdt) = convert_krw_to_usd_usdt(total_krw, 1350.0, 0.0);
        assert_eq!(total_usdt, Decimal::ZERO);

        // 양쪽 다 0
        let (total_usd, total_usdt) = convert_krw_to_usd_usdt(total_krw, 0.0, 0.0);
        assert_eq!(total_usd, Decimal::ZERO);
        assert_eq!(total_usdt, Decimal::ZERO);
    }

    #[test]
    fn test_conversion_f64_to_decimal_nan() {
        // NaN -> 0 fallback
        let total_krw = Decimal::new(1_000_000, 0);
        let result = convert_krw_to_foreign(total_krw, f64::NAN, "USD");
        assert_eq!(result, Decimal::ZERO);

        // Infinity -> 0 fallback
        let result = convert_krw_to_foreign(total_krw, f64::INFINITY, "USD");
        assert_eq!(result, Decimal::ZERO);

        // -Infinity -> 0 fallback
        let result = convert_krw_to_foreign(total_krw, f64::NEG_INFINITY, "USD");
        assert_eq!(result, Decimal::ZERO);
    }

    #[test]
    fn test_conversion_normal_values() {
        // 정상 환산 테스트
        let total_krw = Decimal::new(1_350_000, 0); // 1,350,000 KRW
        let (total_usd, total_usdt) = convert_krw_to_usd_usdt(total_krw, 1350.0, 1380.0);

        // 1,350,000 / 1,350 = 1000.0
        assert_eq!(total_usd, Decimal::new(1000, 0));
        // 1,350,000 / 1,380 = 978.26086957 (소수점 8자리)
        assert!(total_usdt > Decimal::ZERO);
        assert!(total_usdt < Decimal::new(1000, 0));
    }

    #[test]
    fn test_partial_snapshot_bybit_no_usdt() {
        // Bybit 잔고에 USDT 항목이 없으면 None 반환
        let balances = vec![Balance {
            currency: "BTC".to_string(),
            balance: Decimal::new(1, 2),
            locked: Decimal::ZERO,
            avg_buy_price: Decimal::ZERO,
            unit_currency: "USDT".to_string(),
            equity: None,
            unrealised_pnl: None,
        }];

        let row = BalanceRecorderTask::build_bybit_row(
            &balances,
            100,
            1,
            Utc::now(),
            "PERIODIC",
            None,
            1350.0,
            1380.0,
        );

        assert!(row.is_none());
    }

    #[test]
    fn test_bybit_row_position_id() {
        // POS_ENT/POS_EXT에 position_id가 올바르게 설정되는지 확인
        let balances = vec![make_usdt_balance(
            Decimal::new(5000, 0),
            Decimal::ZERO,
            Some(Decimal::new(5000, 0)),
            Some(Decimal::ZERO),
        )];

        let row = BalanceRecorderTask::build_bybit_row(
            &balances,
            100,
            1,
            Utc::now(),
            "POS_ENT",
            Some(42),
            1350.0,
            1380.0,
        );

        let row = row.unwrap();
        assert_eq!(row.record_type, "POS_ENT");
        assert_eq!(row.position_id, Some(42));
    }

    #[test]
    fn test_balance_snapshot_sender_clone() {
        // BalanceSnapshotSender는 Clone 가능해야 함
        let (tx, _rx) = mpsc::channel(32);
        let sender = BalanceSnapshotSender { tx };
        let _cloned = sender.clone();
    }

    #[test]
    fn test_snapshot_msg_debug() {
        // SnapshotMsg의 Debug 출력 확인
        let msg = SnapshotMsg::PositionEntry { position_id: 42 };
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("PositionEntry"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_dust_filter_avg_buy_price_zero() {
        // avg_buy_price = 0인 에어드랍 코인은 항상 필터링
        let balances = vec![
            make_krw_balance(1_000_000, 0),
            make_coin_balance(
                "AIRDROP",
                Decimal::new(100, 0), // 100 코인 보유
                Decimal::ZERO,
                Decimal::ZERO, // avg_buy_price = 0 -> 0 * 100 = 0 < 1000 -> 필터됨
            ),
        ];

        let threshold_krw = Decimal::new(1000, 0);
        let mut coin_holdings: Vec<String> = Vec::new();

        for b in &balances {
            if b.currency == "KRW" {
                continue;
            }
            let qty = b.balance + b.locked;
            let est_value = b.avg_buy_price * qty;
            if est_value >= threshold_krw {
                coin_holdings.push(b.currency.clone());
            }
        }

        assert!(coin_holdings.is_empty());
    }

    #[test]
    fn test_conversion_round_dp_8() {
        // 환산 결과가 소수점 8자리로 truncation되는지 확인
        let total_krw = Decimal::new(1_000_000, 0);
        // 1,000,000 / 1,350 = 740.740740740...
        let result = convert_krw_to_foreign(total_krw, 1350.0, "USD");
        // 소수점 8자리 확인
        assert!(result.scale() <= 8);
    }

    #[test]
    fn test_on_position_entry_returns_false_on_success() {
        // 채널 여유 있으면 false 반환 (드롭 아님)
        let (tx, _rx) = mpsc::channel(32);
        let sender = BalanceSnapshotSender { tx };
        assert!(!sender.on_position_entry(1));
    }

    #[test]
    fn test_on_position_entry_returns_true_on_full_channel() {
        // 채널 가득 차면 true 반환 (드롭됨)
        let (tx, _rx) = mpsc::channel(1);
        let sender = BalanceSnapshotSender { tx };

        // 첫 번째는 성공
        assert!(!sender.on_position_entry(1));
        // 두 번째는 채널 가득 참 -> 드롭
        assert!(sender.on_position_entry(2));
    }

    #[test]
    fn test_on_position_exit_returns_false_on_success() {
        let (tx, _rx) = mpsc::channel(32);
        let sender = BalanceSnapshotSender { tx };
        assert!(!sender.on_position_exit(1));
    }

    #[test]
    fn test_on_position_exit_returns_true_on_full_channel() {
        let (tx, _rx) = mpsc::channel(1);
        let sender = BalanceSnapshotSender { tx };

        assert!(!sender.on_position_exit(1));
        // 두 번째는 채널 가득 참 -> 드롭
        assert!(sender.on_position_exit(2));
    }

    #[test]
    fn test_on_position_entry_skip_returns_false() {
        // position_id <= 0 스킵 시에도 false 반환 (드롭이 아님)
        let (tx, _rx) = mpsc::channel(1);
        let sender = BalanceSnapshotSender { tx };
        assert!(!sender.on_position_entry(0));
        assert!(!sender.on_position_entry(-1));
    }

    // === 재시도 로직 테스트 ===

    /// 호출마다 다른 결과를 반환하는 mock 어댑터 (재시도 테스트용).
    #[derive(Debug)]
    struct RetryMockAdapter {
        name: String,
        /// 호출 순서대로 결과를 pop합니다.
        balance_results: Mutex<Vec<ExchangeResult<Vec<Balance>>>>,
    }

    impl RetryMockAdapter {
        fn new(name: &str, results: Vec<ExchangeResult<Vec<Balance>>>) -> Self {
            // reverse로 저장해서 pop으로 순서대로 꺼냄
            let mut results = results;
            results.reverse();
            Self {
                name: name.to_string(),
                balance_results: Mutex::new(results),
            }
        }
    }

    #[async_trait]
    impl ExchangeAdapter for RetryMockAdapter {
        fn name(&self) -> &str {
            &self.name
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn native_quote_currency(&self) -> &str {
            "USDT"
        }

        async fn get_ticker(&self, _markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }

        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported("mock".into()))
        }

        async fn get_candles(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Err(ExchangeError::Unsupported("mock".into()))
        }

        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
            _before: DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Err(ExchangeError::Unsupported("mock".into()))
        }

        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }

        async fn place_order(&self, _request: &OrderRequest) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }

        async fn cancel_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }

        async fn get_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }

        async fn get_open_orders(&self, _market: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Ok(vec![])
        }

        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            let mut guard = self.balance_results.lock().unwrap();
            guard.pop().unwrap_or(Err(ExchangeError::InternalError(
                "no more mock results".into(),
            )))
        }

        async fn get_balance(&self, _currency: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
    }

    #[tokio::test]
    async fn test_retry_success_after_retryable_error() {
        // 첫 호출: retryable 에러 → 재시도 → 성공
        let usdt_balance = make_usdt_balance(
            Decimal::new(5000, 0),
            Decimal::ZERO,
            Some(Decimal::new(5000, 0)),
            None,
        );
        let adapter = RetryMockAdapter::new(
            "Bybit",
            vec![
                Err(ExchangeError::InternalError("server error".into())),
                Ok(vec![usdt_balance]),
            ],
        );

        let result = fetch_balances_with_retry(&adapter, "Bybit").await;
        assert!(result.is_ok());
        let balances = result.unwrap();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].currency, "USDT");
    }

    #[tokio::test]
    async fn test_retry_both_fail() {
        // 두 번 다 retryable 에러 → 최종 실패
        let adapter = RetryMockAdapter::new(
            "Upbit",
            vec![
                Err(ExchangeError::InternalError("server error 1".into())),
                Err(ExchangeError::InternalError("server error 2".into())),
            ],
        );

        let result = fetch_balances_with_retry(&adapter, "Upbit").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry_non_retryable_no_retry() {
        // 비-retryable 에러 → 즉시 반환 (재시도하지 않음)
        let adapter = RetryMockAdapter::new(
            "Upbit",
            vec![
                Err(ExchangeError::AuthError("invalid key".into())),
                // 이 결과는 도달하면 안 됨
                Ok(vec![]),
            ],
        );

        let result = fetch_balances_with_retry(&adapter, "Upbit").await;
        assert!(result.is_err());
        // AuthError가 그대로 반환
        assert!(matches!(result.unwrap_err(), ExchangeError::AuthError(_)));

        // 재시도하지 않았으므로 두 번째 결과가 남아있어야 함
        let guard = adapter.balance_results.lock().unwrap();
        assert_eq!(guard.len(), 1); // 하나 남아있음
    }

    #[tokio::test]
    async fn test_retry_first_success_no_retry() {
        // 첫 호출 성공 → 재시도 불필요
        let usdt_balance = make_usdt_balance(
            Decimal::new(5000, 0),
            Decimal::ZERO,
            Some(Decimal::new(5000, 0)),
            None,
        );
        let adapter = RetryMockAdapter::new("Bybit", vec![Ok(vec![usdt_balance])]);

        let result = fetch_balances_with_retry(&adapter, "Bybit").await;
        assert!(result.is_ok());
    }
}
