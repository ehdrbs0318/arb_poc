//! 리스크 관리 모듈.
//!
//! Kill switch, 일일/rolling 손실 한도, HWM 드로다운,
//! 단건 주문 크기 상한 등을 관리합니다.
//! 실시간 의사결정은 메모리(AtomicBool + parking_lot::Mutex)로 수행하며,
//! DB에는 비동기로 기록합니다.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tracing::{debug, error, info, warn};

/// 리스크 관리 설정.
#[derive(Debug, Clone)]
pub struct RiskConfig {
    /// 일일 최대 손실 비율 (%).
    pub max_daily_loss_pct: f64,
    /// 일일 최대 손실 절대값 (USDT).
    pub max_daily_loss_usdt: Decimal,
    /// HWM 대비 최대 드로다운 비율 (%).
    pub max_drawdown_pct: f64,
    /// HWM 대비 최대 드로다운 절대값 (USDT).
    pub max_drawdown_usdt: Decimal,
    /// 단건 최대 손실 비율 (%).
    pub max_single_loss_pct: f64,
    /// 단건 최대 손실 절대값 (USDT).
    pub max_single_loss_usdt: Decimal,
    /// 단일 주문 크기 상한 (USDT).
    pub max_order_size_usdt: Decimal,
    /// 최대 동시 포지션 수.
    pub max_concurrent_positions: usize,
    /// Rolling 24h 누적 손실 상한 (USDT).
    pub max_rolling_24h_loss_usdt: Decimal,
    /// HWM 드로다운 측정 윈도우 (일수, 기본값 7).
    pub hwm_window_days: u32,
    /// 전체 미실현 손실 한도 (자본 대비 %, 초과 시 kill switch).
    pub max_unrealized_loss_pct: f64,
    /// 총 자본금 (USDT).
    pub total_capital_usdt: Decimal,
    /// Kill switch 활성화 여부.
    pub kill_switch_enabled: bool,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_daily_loss_pct: 10.0,
            max_daily_loss_usdt: Decimal::from(50),
            max_drawdown_pct: 5.0,
            max_drawdown_usdt: Decimal::from(25),
            max_single_loss_pct: 3.0,
            max_single_loss_usdt: Decimal::from(15),
            max_order_size_usdt: Decimal::from(2000),
            max_concurrent_positions: 5,
            max_rolling_24h_loss_usdt: Decimal::from(80),
            hwm_window_days: 7,
            max_unrealized_loss_pct: 7.0,
            total_capital_usdt: Decimal::from(300),
            kill_switch_enabled: true,
        }
    }
}

/// 미실현 PnL 스냅샷 (check_unrealized_exposure용).
#[derive(Debug, Clone)]
pub struct UnrealizedPnlSnapshot {
    /// 코인 심볼.
    pub coin: String,
    /// 포지션 크기 (USDT).
    pub position_size_usdt: Decimal,
    /// 진입 시 스프레드 (%).
    pub entry_spread_pct: f64,
    /// 현재 스프레드 (%).
    pub current_spread_pct: f64,
    /// 추정 청산 수수료 (USDT).
    pub estimated_exit_fees: Decimal,
    /// 진입 시 USD/KRW 환율.
    pub entry_usd_krw: f64,
    /// 현재 USD/KRW 환율.
    pub current_usd_krw: f64,
}

/// kill switch 발동 사유.
#[derive(Debug, Clone)]
pub enum KillSwitchReason {
    /// 일일 손실 한도 초과.
    DailyLossExceeded { daily_pnl: Decimal, limit: Decimal },
    /// Rolling 24h 손실 한도 초과.
    Rolling24hLossExceeded {
        rolling_loss: Decimal,
        limit: Decimal,
    },
    /// HWM 드로다운 한도 초과.
    DrawdownExceeded { drawdown: Decimal, limit: Decimal },
    /// 단건 손실 한도 초과.
    SingleLossExceeded { loss: Decimal, limit: Decimal },
    /// 미실현 손실 한도 초과.
    UnrealizedLossExceeded {
        unrealized_loss_pct: f64,
        limit_pct: f64,
    },
    /// 수동 발동.
    Manual { reason: String },
    /// DB 연결 장애.
    DbConnectionFailure { consecutive_failures: u32 },
    /// WebSocket 재연결 실패.
    WebSocketReconnectFailure { exchange: String },
}

impl std::fmt::Display for KillSwitchReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DailyLossExceeded { daily_pnl, limit } => {
                write!(f, "Daily loss exceeded: {} > {} USDT", daily_pnl, limit)
            }
            Self::Rolling24hLossExceeded {
                rolling_loss,
                limit,
            } => {
                write!(
                    f,
                    "Rolling 24h loss exceeded: {} > {} USDT",
                    rolling_loss, limit
                )
            }
            Self::DrawdownExceeded { drawdown, limit } => {
                write!(f, "Drawdown exceeded: {} > {} USDT", drawdown, limit)
            }
            Self::SingleLossExceeded { loss, limit } => {
                write!(f, "Single trade loss exceeded: {} > {} USDT", loss, limit)
            }
            Self::UnrealizedLossExceeded {
                unrealized_loss_pct,
                limit_pct,
            } => {
                write!(
                    f,
                    "Unrealized loss exceeded: {:.2}% > {:.2}%",
                    unrealized_loss_pct, limit_pct
                )
            }
            Self::Manual { reason } => write!(f, "Manual: {}", reason),
            Self::DbConnectionFailure {
                consecutive_failures,
            } => {
                write!(
                    f,
                    "DB connection failure: {} consecutive failures",
                    consecutive_failures
                )
            }
            Self::WebSocketReconnectFailure { exchange } => {
                write!(f, "WebSocket reconnect failure: {}", exchange)
            }
        }
    }
}

/// 리스크 내부 상태 (Mutex로 보호).
struct RiskState {
    /// 당일 실현 PnL 누적.
    daily_realized_pnl: Decimal,
    /// 현재 equity (자본 + 실현 PnL).
    current_equity: Decimal,
    /// 당일 equity 최고점.
    peak_equity: Decimal,
    /// 마지막 일일 리셋 시각.
    last_reset: DateTime<Utc>,
    /// Rolling 24h 실현 손실 기록 (timestamp, 손실 금액).
    /// 손실만 기록 (음수 PnL). 최대 10,000건.
    rolling_24h_losses: VecDeque<(DateTime<Utc>, Decimal)>,
    /// Rolling 7d HWM용 일별 equity 고점 기록.
    hwm_daily_peaks: VecDeque<(DateTime<Utc>, Decimal)>,
    /// 누적 거래 횟수 (cold start 보호용).
    total_trade_count: u64,
    /// 세션 시작 시각 (cold start 보호용).
    session_started_at: DateTime<Utc>,
    /// 양 거래소 연결 상태.
    upbit_connected: bool,
    bybit_connected: bool,
}

/// 리스크 관리자.
///
/// Kill switch + 손실 한도 + 드로다운 관리.
/// `is_killed`는 AtomicBool로 lock 없이 확인 가능합니다.
/// `inner`는 parking_lot::Mutex로 poisoning 없이 안전합니다.
pub struct RiskManager {
    config: RiskConfig,
    inner: Mutex<RiskState>,
    is_killed: AtomicBool,
}

impl RiskManager {
    /// 새 RiskManager를 생성합니다.
    pub fn new(config: RiskConfig) -> Self {
        let now = Utc::now();
        let equity = config.total_capital_usdt;

        debug!(
            total_capital = %equity,
            max_daily_loss_pct = config.max_daily_loss_pct,
            max_daily_loss_usdt = %config.max_daily_loss_usdt,
            max_drawdown_pct = config.max_drawdown_pct,
            max_drawdown_usdt = %config.max_drawdown_usdt,
            max_rolling_24h_loss_usdt = %config.max_rolling_24h_loss_usdt,
            hwm_window_days = config.hwm_window_days,
            kill_switch_enabled = config.kill_switch_enabled,
            "RiskManager 초기화"
        );

        Self {
            config,
            inner: Mutex::new(RiskState {
                daily_realized_pnl: Decimal::ZERO,
                current_equity: equity,
                peak_equity: equity,
                last_reset: now,
                rolling_24h_losses: VecDeque::new(),
                hwm_daily_peaks: VecDeque::new(),
                total_trade_count: 0,
                session_started_at: now,
                upbit_connected: true,
                bybit_connected: true,
            }),
            is_killed: AtomicBool::new(false),
        }
    }

    /// Kill switch가 발동되었는지 확인합니다 (lock 불필요).
    pub fn is_killed(&self) -> bool {
        self.is_killed.load(Ordering::Acquire)
    }

    /// 진입이 허용되는지 확인합니다.
    ///
    /// kill switch, 연결 상태를 종합적으로 판단합니다.
    pub fn is_entry_allowed(&self) -> bool {
        if self.is_killed() {
            debug!("진입 차단: kill switch 발동됨");
            return false;
        }

        let state = self.inner.lock();

        if !state.upbit_connected || !state.bybit_connected {
            debug!(
                upbit_connected = state.upbit_connected,
                bybit_connected = state.bybit_connected,
                "진입 차단: 거래소 연결 불안정"
            );
            return false;
        }

        true
    }

    /// 거래 결과를 기록하고 리스크 한도를 체크합니다.
    ///
    /// PnL이 음수이면 손실 한도를 확인하여 kill switch를 발동할 수 있습니다.
    pub fn record_trade(&self, pnl: Decimal) -> Option<KillSwitchReason> {
        if !self.config.kill_switch_enabled {
            debug!(pnl = %pnl, "Kill switch 비활성화, 리스크 체크 스킵");
            return None;
        }

        let now = Utc::now();
        let mut state = self.inner.lock();

        state.total_trade_count += 1;
        state.daily_realized_pnl += pnl;
        state.current_equity += pnl;

        // 당일 equity 최고점 갱신
        if state.current_equity > state.peak_equity {
            state.peak_equity = state.current_equity;
        }

        debug!(
            pnl = %pnl,
            daily_realized_pnl = %state.daily_realized_pnl,
            current_equity = %state.current_equity,
            peak_equity = %state.peak_equity,
            total_trade_count = state.total_trade_count,
            "거래 기록"
        );

        // 손실인 경우 rolling_24h_losses에 기록
        if pnl < Decimal::ZERO {
            state.rolling_24h_losses.push_back((now, pnl));
            // 최대 10,000건 제한
            while state.rolling_24h_losses.len() > 10_000 {
                state.rolling_24h_losses.pop_front();
            }
        }

        // ① 단건 손실 한도
        if pnl < Decimal::ZERO {
            let loss = pnl.abs();
            let pct_limit = self.pct_to_usdt(self.config.max_single_loss_pct);
            let effective_limit = pct_limit.min(self.config.max_single_loss_usdt);

            if loss > effective_limit {
                let reason = KillSwitchReason::SingleLossExceeded {
                    loss,
                    limit: effective_limit,
                };
                warn!(%loss, %effective_limit, "단건 손실 한도 초과 → kill switch");
                drop(state);
                self.trigger_kill_switch_internal(&reason);
                return Some(reason);
            }
        }

        // ② 일일 손실 한도
        if state.daily_realized_pnl < Decimal::ZERO {
            let daily_loss = state.daily_realized_pnl.abs();
            let pct_limit = self.pct_to_usdt(self.config.max_daily_loss_pct);
            let effective_limit = pct_limit.min(self.config.max_daily_loss_usdt);

            if daily_loss > effective_limit {
                let reason = KillSwitchReason::DailyLossExceeded {
                    daily_pnl: state.daily_realized_pnl,
                    limit: effective_limit,
                };
                warn!(daily_loss = %daily_loss, %effective_limit, "일일 손실 한도 초과 → kill switch");
                drop(state);
                self.trigger_kill_switch_internal(&reason);
                return Some(reason);
            }
        }

        // ③ Rolling 24h 누적 손실 한도
        let rolling_loss = self.calc_rolling_24h_loss(&state, now);
        if rolling_loss > self.config.max_rolling_24h_loss_usdt {
            let reason = KillSwitchReason::Rolling24hLossExceeded {
                rolling_loss,
                limit: self.config.max_rolling_24h_loss_usdt,
            };
            warn!(%rolling_loss, limit = %self.config.max_rolling_24h_loss_usdt, "Rolling 24h 손실 한도 초과 → kill switch");
            drop(state);
            self.trigger_kill_switch_internal(&reason);
            return Some(reason);
        }

        // ④ HWM 드로다운 (cold start 보호: 24h/10건 미만 시 비율 kill switch 비활성화)
        let is_cold_start = self.is_cold_start(&state, now);
        if !is_cold_start {
            let hwm = self.calc_hwm(&state, now);
            let drawdown = hwm - state.current_equity;

            if drawdown > Decimal::ZERO {
                let pct_limit = self.pct_to_usdt(self.config.max_drawdown_pct);
                let effective_limit = pct_limit.min(self.config.max_drawdown_usdt);

                debug!(
                    hwm = %hwm,
                    current_equity = %state.current_equity,
                    drawdown = %drawdown,
                    %effective_limit,
                    "드로다운 체크"
                );

                if drawdown > effective_limit {
                    let reason = KillSwitchReason::DrawdownExceeded {
                        drawdown,
                        limit: effective_limit,
                    };
                    warn!(%drawdown, %effective_limit, "HWM 드로다운 한도 초과 → kill switch");
                    drop(state);
                    self.trigger_kill_switch_internal(&reason);
                    return Some(reason);
                }
            }
        } else {
            // cold start 중에는 절대값 한도만 적용
            let hwm = self.calc_hwm(&state, now);
            let drawdown = hwm - state.current_equity;
            if drawdown > Decimal::ZERO && drawdown > self.config.max_drawdown_usdt {
                let reason = KillSwitchReason::DrawdownExceeded {
                    drawdown,
                    limit: self.config.max_drawdown_usdt,
                };
                warn!(%drawdown, limit = %self.config.max_drawdown_usdt, "Cold start 절대값 드로다운 한도 초과 → kill switch");
                drop(state);
                self.trigger_kill_switch_internal(&reason);
                return Some(reason);
            }
        }

        None
    }

    /// Kill switch를 강제 발동합니다.
    pub fn trigger_kill_switch(&self, reason: &str) {
        let reason = KillSwitchReason::Manual {
            reason: reason.to_string(),
        };
        self.trigger_kill_switch_internal(&reason);
    }

    /// 거래소 연결 상태를 업데이트합니다.
    pub fn check_connection_health(&self, upbit_ok: bool, bybit_ok: bool) {
        let mut state = self.inner.lock();
        let prev_upbit = state.upbit_connected;
        let prev_bybit = state.bybit_connected;
        state.upbit_connected = upbit_ok;
        state.bybit_connected = bybit_ok;

        if prev_upbit != upbit_ok || prev_bybit != bybit_ok {
            debug!(
                upbit = %upbit_ok,
                bybit = %bybit_ok,
                prev_upbit = %prev_upbit,
                prev_bybit = %prev_bybit,
                "거래소 연결 상태 변경"
            );
        }
    }

    /// 단일 주문 크기가 상한 이내인지 확인합니다.
    pub fn validate_order_size(&self, size_usdt: Decimal) -> bool {
        let valid = size_usdt <= self.config.max_order_size_usdt;
        if !valid {
            warn!(
                size_usdt = %size_usdt,
                max = %self.config.max_order_size_usdt,
                "주문 크기 상한 초과"
            );
        } else {
            debug!(size_usdt = %size_usdt, max = %self.config.max_order_size_usdt, "주문 크기 확인 통과");
        }
        valid
    }

    /// 전체 미실현 손실을 확인합니다.
    ///
    /// 미실현 손실이 `max_unrealized_loss_pct`를 초과하면
    /// kill switch를 발동합니다.
    pub fn check_unrealized_exposure(
        &self,
        positions: &[UnrealizedPnlSnapshot],
    ) -> Option<KillSwitchReason> {
        if !self.config.kill_switch_enabled || positions.is_empty() {
            return None;
        }

        let total_unrealized: Decimal = positions
            .iter()
            .map(|p| {
                let spread_delta = p.entry_spread_pct - p.current_spread_pct;
                let spread_pnl = p.position_size_usdt
                    * Decimal::try_from(spread_delta).unwrap_or(Decimal::ZERO)
                    / Decimal::from(100);

                let fx_risk = if p.entry_usd_krw > 0.0 {
                    let fx_change = (p.current_usd_krw - p.entry_usd_krw).abs() / p.entry_usd_krw;
                    p.position_size_usdt * Decimal::try_from(fx_change).unwrap_or(Decimal::ZERO)
                } else {
                    Decimal::ZERO
                };

                spread_pnl - p.estimated_exit_fees - fx_risk
            })
            .sum();

        let capital = self.config.total_capital_usdt;
        if capital == Decimal::ZERO {
            return None;
        }

        let unrealized_loss_pct = if total_unrealized < Decimal::ZERO {
            (total_unrealized.abs() * Decimal::from(100) / capital)
                .to_f64()
                .unwrap_or(0.0)
        } else {
            0.0
        };

        debug!(
            total_unrealized = %total_unrealized,
            unrealized_loss_pct = unrealized_loss_pct,
            limit_pct = self.config.max_unrealized_loss_pct,
            position_count = positions.len(),
            "미실현 손실 체크"
        );

        if unrealized_loss_pct > self.config.max_unrealized_loss_pct {
            let reason = KillSwitchReason::UnrealizedLossExceeded {
                unrealized_loss_pct,
                limit_pct: self.config.max_unrealized_loss_pct,
            };
            warn!(
                unrealized_loss_pct = unrealized_loss_pct,
                limit = self.config.max_unrealized_loss_pct,
                "미실현 손실 한도 초과 → kill switch"
            );
            self.trigger_kill_switch_internal(&reason);
            return Some(reason);
        }

        None
    }

    /// 만료된 rolling_24h_losses 엔트리를 정리합니다.
    ///
    /// minute_timer에서 매분 호출합니다.
    pub fn cleanup_expired_losses(&self) {
        let now = Utc::now();
        let cutoff = now - chrono::Duration::hours(24);
        let mut state = self.inner.lock();
        let before = state.rolling_24h_losses.len();

        while let Some(front) = state.rolling_24h_losses.front() {
            if front.0 < cutoff {
                state.rolling_24h_losses.pop_front();
            } else {
                break;
            }
        }

        let removed = before - state.rolling_24h_losses.len();
        if removed > 0 {
            debug!(
                removed = removed,
                remaining = state.rolling_24h_losses.len(),
                "만료된 rolling loss 엔트리 정리"
            );
        }
    }

    /// 일일 리셋을 수행합니다.
    ///
    /// KST 00:00 (UTC 15:00) 기준으로 일일 PnL을 리셋합니다.
    /// 이전일의 peak_equity를 HWM 일별 고점에 기록합니다.
    pub fn try_daily_reset(&self) {
        let now = Utc::now();
        let mut state = self.inner.lock();

        // KST 00:00 = UTC 15:00
        let kst_now = now + chrono::Duration::hours(9);
        let kst_last = state.last_reset + chrono::Duration::hours(9);

        // 날짜가 바뀌었는지 확인 (KST 기준)
        if kst_now.date_naive() <= kst_last.date_naive() {
            return;
        }

        info!(
            prev_daily_pnl = %state.daily_realized_pnl,
            prev_peak_equity = %state.peak_equity,
            current_equity = %state.current_equity,
            "일일 리셋 수행 (KST 00:00)"
        );

        // HWM 일별 고점 기록
        let peak_eq = state.peak_equity;
        state.hwm_daily_peaks.push_back((now, peak_eq));

        // hwm_window_days 초과 엔트리 제거
        let window_days = self.config.hwm_window_days as i64;
        let cutoff = now - chrono::Duration::days(window_days);
        while let Some(front) = state.hwm_daily_peaks.front() {
            if front.0 < cutoff {
                state.hwm_daily_peaks.pop_front();
            } else {
                break;
            }
        }

        // 일일 리셋
        state.daily_realized_pnl = Decimal::ZERO;
        state.peak_equity = state.current_equity;
        state.last_reset = now;

        debug!(
            hwm_entries = state.hwm_daily_peaks.len(),
            new_peak_equity = %state.peak_equity,
            "일일 리셋 완료"
        );
    }

    /// 현재 일일 PnL을 반환합니다.
    pub fn daily_pnl(&self) -> Decimal {
        self.inner.lock().daily_realized_pnl
    }

    /// 현재 equity를 반환합니다.
    pub fn current_equity(&self) -> Decimal {
        self.inner.lock().current_equity
    }

    /// Rolling 24h 누적 손실을 반환합니다.
    pub fn rolling_24h_loss(&self) -> Decimal {
        let now = Utc::now();
        let state = self.inner.lock();
        self.calc_rolling_24h_loss(&state, now)
    }

    /// 현재 거래 횟수를 반환합니다.
    pub fn total_trade_count(&self) -> u64 {
        self.inner.lock().total_trade_count
    }

    // --- 내부 헬퍼 ---

    /// kill switch를 내부적으로 발동합니다.
    fn trigger_kill_switch_internal(&self, reason: &KillSwitchReason) {
        let was_killed = self.is_killed.swap(true, Ordering::Release);
        if !was_killed {
            error!(reason = %reason, "KILL SWITCH 발동");
        } else {
            debug!(reason = %reason, "Kill switch 이미 발동 상태 (중복 트리거)");
        }
    }

    /// 비율을 USDT 금액으로 변환합니다.
    fn pct_to_usdt(&self, pct: f64) -> Decimal {
        Decimal::try_from(pct / 100.0).unwrap_or(Decimal::ZERO) * self.config.total_capital_usdt
    }

    /// Rolling 24h 손실 합계를 계산합니다.
    fn calc_rolling_24h_loss(&self, state: &RiskState, now: DateTime<Utc>) -> Decimal {
        let cutoff = now - chrono::Duration::hours(24);
        state
            .rolling_24h_losses
            .iter()
            .filter(|(ts, _)| *ts >= cutoff)
            .map(|(_, loss)| loss.abs())
            .sum()
    }

    /// HWM (High-Water Mark)를 계산합니다.
    ///
    /// Rolling 7d 내 최고 equity를 반환합니다.
    fn calc_hwm(&self, state: &RiskState, now: DateTime<Utc>) -> Decimal {
        let window_days = self.config.hwm_window_days as i64;
        let cutoff = now - chrono::Duration::days(window_days);

        let historical_max = state
            .hwm_daily_peaks
            .iter()
            .filter(|(ts, _)| *ts >= cutoff)
            .map(|(_, peak)| *peak)
            .max()
            .unwrap_or(self.config.total_capital_usdt);

        // 당일 peak_equity도 포함
        historical_max.max(state.peak_equity)
    }

    /// Cold start 여부를 판단합니다.
    ///
    /// 세션 시작 후 24시간 미만 AND 거래 10건 미만이면 cold start.
    fn is_cold_start(&self, state: &RiskState, now: DateTime<Utc>) -> bool {
        let session_age = now - state.session_started_at;
        let hours_elapsed = session_age.num_hours();
        let is_cold = hours_elapsed < 24 && state.total_trade_count < 10;
        if is_cold {
            debug!(
                hours_elapsed = hours_elapsed,
                trade_count = state.total_trade_count,
                "Cold start 상태 (비율 기반 kill switch 비활성화)"
            );
        }
        is_cold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RiskConfig {
        RiskConfig {
            max_daily_loss_pct: 10.0,
            max_daily_loss_usdt: Decimal::from(50),
            max_drawdown_pct: 5.0,
            max_drawdown_usdt: Decimal::from(25),
            max_single_loss_pct: 3.0,
            max_single_loss_usdt: Decimal::from(15),
            max_order_size_usdt: Decimal::from(2000),
            max_concurrent_positions: 5,
            max_rolling_24h_loss_usdt: Decimal::from(80),
            hwm_window_days: 7,
            max_unrealized_loss_pct: 7.0,
            total_capital_usdt: Decimal::from(300),
            kill_switch_enabled: true,
        }
    }

    #[test]
    fn test_initial_state() {
        let rm = RiskManager::new(test_config());
        assert!(!rm.is_killed());
        assert!(rm.is_entry_allowed());
        assert_eq!(rm.daily_pnl(), Decimal::ZERO);
        assert_eq!(rm.current_equity(), Decimal::from(300));
    }

    #[test]
    fn test_record_profitable_trade() {
        let rm = RiskManager::new(test_config());
        let reason = rm.record_trade(Decimal::from(10));
        assert!(reason.is_none());
        assert!(!rm.is_killed());
        assert_eq!(rm.daily_pnl(), Decimal::from(10));
        assert_eq!(rm.current_equity(), Decimal::from(310));
    }

    #[test]
    fn test_single_loss_kill_switch() {
        let rm = RiskManager::new(test_config());
        // 단건 한도: min(3%*300=9, 15) = 9
        let reason = rm.record_trade(Decimal::from(-10));
        assert!(reason.is_some());
        assert!(rm.is_killed());
        assert!(!rm.is_entry_allowed());
    }

    #[test]
    fn test_daily_loss_kill_switch() {
        let config = RiskConfig {
            max_single_loss_usdt: Decimal::from(100), // 단건 한도 넓게
            max_single_loss_pct: 100.0,
            ..test_config()
        };
        let rm = RiskManager::new(config);
        // 일일 한도: min(10%*300=30, 50) = 30
        rm.record_trade(Decimal::from(-20));
        assert!(!rm.is_killed());

        let reason = rm.record_trade(Decimal::from(-15));
        assert!(reason.is_some());
        assert!(rm.is_killed());
    }

    #[test]
    fn test_rolling_24h_loss_kill_switch() {
        let config = RiskConfig {
            max_single_loss_usdt: Decimal::from(100),
            max_single_loss_pct: 100.0,
            max_daily_loss_usdt: Decimal::from(200),
            max_daily_loss_pct: 100.0,
            max_drawdown_usdt: Decimal::from(200),
            max_drawdown_pct: 100.0,
            max_rolling_24h_loss_usdt: Decimal::from(30),
            ..test_config()
        };
        let rm = RiskManager::new(config);

        rm.record_trade(Decimal::from(-10));
        rm.record_trade(Decimal::from(-10));
        assert!(!rm.is_killed());

        let reason = rm.record_trade(Decimal::from(-15));
        assert!(reason.is_some());
        assert!(rm.is_killed());
    }

    #[test]
    fn test_validate_order_size() {
        let rm = RiskManager::new(test_config());
        assert!(rm.validate_order_size(Decimal::from(500)));
        assert!(rm.validate_order_size(Decimal::from(2000)));
        assert!(!rm.validate_order_size(Decimal::from(2001)));
    }

    #[test]
    fn test_connection_health() {
        let rm = RiskManager::new(test_config());
        assert!(rm.is_entry_allowed());

        rm.check_connection_health(false, true);
        assert!(!rm.is_entry_allowed());

        rm.check_connection_health(true, true);
        assert!(rm.is_entry_allowed());
    }

    #[test]
    fn test_manual_kill_switch() {
        let rm = RiskManager::new(test_config());
        rm.trigger_kill_switch("테스트 발동");
        assert!(rm.is_killed());
        assert!(!rm.is_entry_allowed());
    }

    #[test]
    fn test_kill_switch_disabled() {
        let config = RiskConfig {
            kill_switch_enabled: false,
            ..test_config()
        };
        let rm = RiskManager::new(config);
        let reason = rm.record_trade(Decimal::from(-100));
        assert!(reason.is_none());
        assert!(!rm.is_killed());
    }

    #[test]
    fn test_cleanup_expired_losses() {
        let rm = RiskManager::new(RiskConfig {
            max_single_loss_usdt: Decimal::from(1000),
            max_single_loss_pct: 100.0,
            max_daily_loss_usdt: Decimal::from(1000),
            max_daily_loss_pct: 100.0,
            max_drawdown_usdt: Decimal::from(1000),
            max_drawdown_pct: 100.0,
            max_rolling_24h_loss_usdt: Decimal::from(1000),
            ..test_config()
        });

        // 손실 기록
        rm.record_trade(Decimal::from(-5));
        rm.record_trade(Decimal::from(-3));

        {
            let state = rm.inner.lock();
            assert_eq!(state.rolling_24h_losses.len(), 2);
        }

        // 아직 24h 이내이므로 정리 안 됨
        rm.cleanup_expired_losses();
        {
            let state = rm.inner.lock();
            assert_eq!(state.rolling_24h_losses.len(), 2);
        }
    }

    #[test]
    fn test_cold_start_protection() {
        let rm = RiskManager::new(test_config());

        // cold start 상태 (0 trades, < 24h)
        {
            let state = rm.inner.lock();
            let now = Utc::now();
            assert!(rm.is_cold_start(&state, now));
        }

        // 10건 거래 후 cold start 해제
        let config = RiskConfig {
            max_single_loss_usdt: Decimal::from(1000),
            max_single_loss_pct: 100.0,
            max_daily_loss_usdt: Decimal::from(1000),
            max_daily_loss_pct: 100.0,
            max_drawdown_usdt: Decimal::from(1000),
            max_drawdown_pct: 100.0,
            max_rolling_24h_loss_usdt: Decimal::from(1000),
            ..test_config()
        };
        let rm2 = RiskManager::new(config);
        for _ in 0..10 {
            rm2.record_trade(Decimal::from(1));
        }
        {
            let state = rm2.inner.lock();
            let now = Utc::now();
            assert!(!rm2.is_cold_start(&state, now));
        }
    }

    #[test]
    fn test_unrealized_exposure_normal() {
        let rm = RiskManager::new(test_config());
        let positions = vec![UnrealizedPnlSnapshot {
            coin: "BTC".to_string(),
            position_size_usdt: Decimal::from(100),
            entry_spread_pct: 0.5,
            current_spread_pct: 0.4,
            estimated_exit_fees: Decimal::new(21, 2), // 0.21
            entry_usd_krw: 1400.0,
            current_usd_krw: 1400.0,
        }];
        let reason = rm.check_unrealized_exposure(&positions);
        assert!(reason.is_none());
    }

    #[test]
    fn test_unrealized_exposure_kill_switch() {
        let rm = RiskManager::new(test_config());
        // 스프레드가 entry보다 크게 확대 → 큰 미실현 손실
        // spread_delta = 0.5 - 15.0 = -14.5 → pnl = 200 * (-14.5) / 100 = -29.0
        // unrealized_loss_pct = 29.42 / 300 * 100 ≈ 9.8% > 7.0% limit
        let positions = vec![UnrealizedPnlSnapshot {
            coin: "BTC".to_string(),
            position_size_usdt: Decimal::from(200),
            entry_spread_pct: 0.5,
            current_spread_pct: 15.0, // 크게 확대 (불리)
            estimated_exit_fees: Decimal::new(42, 2),
            entry_usd_krw: 1400.0,
            current_usd_krw: 1400.0,
        }];
        let reason = rm.check_unrealized_exposure(&positions);
        assert!(reason.is_some());
        assert!(rm.is_killed());
    }

    #[test]
    fn test_daily_reset_date_change() {
        let rm = RiskManager::new(RiskConfig {
            max_single_loss_usdt: Decimal::from(1000),
            max_single_loss_pct: 100.0,
            max_daily_loss_usdt: Decimal::from(1000),
            max_daily_loss_pct: 100.0,
            max_drawdown_usdt: Decimal::from(1000),
            max_drawdown_pct: 100.0,
            max_rolling_24h_loss_usdt: Decimal::from(1000),
            ..test_config()
        });

        rm.record_trade(Decimal::from(-20));
        assert_eq!(rm.daily_pnl(), Decimal::from(-20));

        // 어제로 last_reset을 변경하여 리셋 트리거
        {
            let mut state = rm.inner.lock();
            state.last_reset = Utc::now() - chrono::Duration::days(1);
        }

        rm.try_daily_reset();
        assert_eq!(rm.daily_pnl(), Decimal::ZERO);
    }

    #[test]
    fn test_pct_to_usdt() {
        let rm = RiskManager::new(test_config());
        // 10% of 300 = 30
        let result = rm.pct_to_usdt(10.0);
        assert_eq!(result, Decimal::from(30));
    }

    #[test]
    fn test_effective_limit_is_min() {
        // 자본 $300, max_single_loss_pct=3% → $9, max_single_loss_usdt=$15
        // 실효 한도 = min(9, 15) = $9
        let rm = RiskManager::new(test_config());
        // -$8.99 → OK
        let reason = rm.record_trade(Decimal::new(-899, 2));
        assert!(reason.is_none());
    }
}
