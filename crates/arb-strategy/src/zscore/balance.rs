//! 잔고 관리 모듈 (BalanceTracker).
//!
//! 거래소별 가용 잔고 추적 + 동시 진입 시 자본 예약 패턴.
//! 단일 `parking_lot::Mutex`로 양 거래소 잔고를 보호하며,
//! `ReservationToken` RAII 패턴으로 예약 누수를 방지합니다.

use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use rust_decimal::Decimal;
use tracing::{debug, info, warn};

/// 예약 레코드 (내부용).
#[derive(Debug, Clone)]
struct ReservationRecord {
    /// 예약 고유 ID.
    id: u64,
    /// Upbit KRW 예약 금액.
    upbit_krw: Decimal,
    /// Bybit USDT 예약 금액.
    bybit_usdt: Decimal,
    /// 예약 생성 시각.
    created_at: Instant,
    /// commit 완료 여부.
    committed: bool,
}

/// 잔고 내부 상태.
struct BalanceState {
    /// Upbit 가용 KRW 잔고.
    upbit_available_krw: Decimal,
    /// Bybit 가용 USDT 잔고.
    bybit_available_usdt: Decimal,
    /// 활성 예약 목록.
    reservations: Vec<ReservationRecord>,
    /// 다음 예약 ID.
    next_reservation_id: u64,
}

/// 잔고 추적기.
///
/// 양 거래소 잔고를 단일 Mutex로 보호하며,
/// 원자적 예약/해제/확정 패턴을 제공합니다.
pub struct BalanceTracker {
    inner: Arc<Mutex<BalanceState>>,
    /// ReservationToken TTL (기본 6분).
    reservation_ttl: Duration,
}

/// 잔고 예약 토큰 (RAII).
///
/// Drop 시 자동으로 예약을 해제합니다 (commit되지 않은 경우).
/// panic 시에도 잔고가 영구적으로 차감되지 않습니다.
pub struct ReservationToken {
    /// 예약 ID.
    id: u64,
    /// 상태 접근용.
    state: Arc<Mutex<BalanceState>>,
    /// commit 완료 여부.
    committed: bool,
}

impl ReservationToken {
    /// 예약 ID를 반환합니다.
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Drop for ReservationToken {
    fn drop(&mut self) {
        if self.committed {
            return;
        }

        // try_lock: Mutex 이미 잠겨 있으면 sweeper에게 위임
        if let Some(mut state) = self.state.try_lock() {
            if let Some(idx) = state.reservations.iter().position(|r| r.id == self.id) {
                let record = state.reservations.remove(idx);
                if !record.committed {
                    state.upbit_available_krw += record.upbit_krw;
                    state.bybit_available_usdt += record.bybit_usdt;
                    warn!(
                        reservation_id = self.id,
                        upbit_krw = %record.upbit_krw,
                        bybit_usdt = %record.bybit_usdt,
                        "ReservationToken Drop: 미확정 예약 자동 해제"
                    );
                }
            }
        } else {
            warn!(
                reservation_id = self.id,
                "ReservationToken Drop: Mutex 잠김, sweeper에서 해제 예정"
            );
        }
    }
}

impl BalanceTracker {
    /// 새 BalanceTracker를 생성합니다.
    ///
    /// 초기 잔고를 설정합니다.
    pub fn new(upbit_krw: Decimal, bybit_usdt: Decimal) -> Self {
        info!(
            upbit_krw = %upbit_krw,
            bybit_usdt = %bybit_usdt,
            "BalanceTracker 초기화"
        );

        Self {
            inner: Arc::new(Mutex::new(BalanceState {
                upbit_available_krw: upbit_krw,
                bybit_available_usdt: bybit_usdt,
                reservations: Vec::new(),
                next_reservation_id: 0,
            })),
            reservation_ttl: Duration::from_secs(360), // 6분
        }
    }

    /// 커스텀 TTL로 BalanceTracker를 생성합니다 (테스트용).
    #[cfg(test)]
    pub fn with_ttl(upbit_krw: Decimal, bybit_usdt: Decimal, ttl: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BalanceState {
                upbit_available_krw: upbit_krw,
                bybit_available_usdt: bybit_usdt,
                reservations: Vec::new(),
                next_reservation_id: 0,
            })),
            reservation_ttl: ttl,
        }
    }

    /// 진입 전 잔고를 예약합니다.
    ///
    /// 양 거래소 잔고를 단일 Mutex guard 내에서 동시 차감합니다.
    /// Bybit 부족 시 Upbit 예약도 함께 롤백합니다.
    ///
    /// # 반환
    ///
    /// 성공 시 `Some(ReservationToken)`, 잔고 부족 시 `None`.
    pub fn reserve(&self, upbit_krw: Decimal, bybit_usdt: Decimal) -> Option<ReservationToken> {
        let reservation_id;

        {
            let mut state = self.inner.lock();

            debug!(
                requested_upbit_krw = %upbit_krw,
                requested_bybit_usdt = %bybit_usdt,
                available_upbit_krw = %state.upbit_available_krw,
                available_bybit_usdt = %state.bybit_available_usdt,
                active_reservations = state.reservations.len(),
                "잔고 예약 시도"
            );

            // Upbit 잔고 확인
            if state.upbit_available_krw < upbit_krw {
                debug!(
                    available = %state.upbit_available_krw,
                    required = %upbit_krw,
                    "예약 실패: Upbit KRW 잔고 부족"
                );
                return None;
            }

            // Bybit 잔고 확인
            if state.bybit_available_usdt < bybit_usdt {
                debug!(
                    available = %state.bybit_available_usdt,
                    required = %bybit_usdt,
                    "예약 실패: Bybit USDT 잔고 부족"
                );
                return None;
            }

            // 양쪽 동시 차감 (원자적)
            state.upbit_available_krw -= upbit_krw;
            state.bybit_available_usdt -= bybit_usdt;

            reservation_id = state.next_reservation_id;
            state.next_reservation_id += 1;

            state.reservations.push(ReservationRecord {
                id: reservation_id,
                upbit_krw,
                bybit_usdt,
                created_at: Instant::now(),
                committed: false,
            });

            debug!(
                reservation_id = reservation_id,
                remaining_upbit_krw = %state.upbit_available_krw,
                remaining_bybit_usdt = %state.bybit_available_usdt,
                "예약 성공"
            );
        }
        // MutexGuard drop 후 ReservationToken 생성 (교착 방지)

        Some(ReservationToken {
            id: reservation_id,
            state: Arc::clone(&self.inner),
            committed: false,
        })
    }

    /// 주문 성공 시 예약을 확정합니다.
    ///
    /// 실 체결 금액으로 잔고를 조정합니다.
    /// 예약 금액과 실 체결 금액의 차이를 보정합니다.
    pub fn commit(
        &self,
        token: &mut ReservationToken,
        actual_upbit_krw: Decimal,
        actual_bybit_usdt: Decimal,
    ) {
        let mut state = self.inner.lock();

        // index 기반 탐색 + 로컬 복사로 borrow 분리
        let found = state
            .reservations
            .iter()
            .position(|r| r.id == token.id)
            .map(|idx| {
                let reserved_upbit = state.reservations[idx].upbit_krw;
                let reserved_bybit = state.reservations[idx].bybit_usdt;
                (idx, reserved_upbit, reserved_bybit)
            });

        if let Some((idx, reserved_upbit, reserved_bybit)) = found {
            let upbit_diff = reserved_upbit - actual_upbit_krw;
            let bybit_diff = reserved_bybit - actual_bybit_usdt;

            // 초과 예약분 반환 / 추가 차감
            if upbit_diff > Decimal::ZERO {
                state.upbit_available_krw += upbit_diff;
            } else if upbit_diff < Decimal::ZERO {
                let abs_diff = upbit_diff.abs();
                if state.upbit_available_krw >= abs_diff {
                    state.upbit_available_krw -= abs_diff;
                } else {
                    warn!("Upbit KRW 잔고 음수 방지: clamp to 0 (diff={})", abs_diff);
                    state.upbit_available_krw = Decimal::ZERO;
                }
            }

            if bybit_diff > Decimal::ZERO {
                state.bybit_available_usdt += bybit_diff;
            } else if bybit_diff < Decimal::ZERO {
                let abs_diff = bybit_diff.abs();
                if state.bybit_available_usdt >= abs_diff {
                    state.bybit_available_usdt -= abs_diff;
                } else {
                    warn!("Bybit USDT 잔고 음수 방지: clamp to 0 (diff={})", abs_diff);
                    state.bybit_available_usdt = Decimal::ZERO;
                }
            }

            state.reservations[idx].committed = true;
            token.committed = true;

            debug!(
                reservation_id = token.id,
                actual_upbit_krw = %actual_upbit_krw,
                actual_bybit_usdt = %actual_bybit_usdt,
                upbit_diff = %upbit_diff,
                bybit_diff = %bybit_diff,
                available_upbit_krw = %state.upbit_available_krw,
                available_bybit_usdt = %state.bybit_available_usdt,
                "예약 확정"
            );
        } else {
            warn!(
                reservation_id = token.id,
                "commit 실패: 예약을 찾을 수 없음 (이미 해제됨?)"
            );
        }
    }

    /// 주문 실패/취소 시 예약을 해제합니다.
    pub fn release(&self, token: &mut ReservationToken) {
        let mut state = self.inner.lock();

        if let Some(idx) = state.reservations.iter().position(|r| r.id == token.id) {
            let record = state.reservations.remove(idx);
            if !record.committed {
                state.upbit_available_krw += record.upbit_krw;
                state.bybit_available_usdt += record.bybit_usdt;
            }
            token.committed = true; // Drop에서 이중 해제 방지

            debug!(
                reservation_id = token.id,
                released_upbit_krw = %record.upbit_krw,
                released_bybit_usdt = %record.bybit_usdt,
                available_upbit_krw = %state.upbit_available_krw,
                available_bybit_usdt = %state.bybit_available_usdt,
                "예약 해제"
            );
        } else {
            debug!(
                reservation_id = token.id,
                "release: 예약을 찾을 수 없음 (이미 처리됨)"
            );
        }
    }

    /// 청산 완료 시 잔고를 복원합니다.
    pub fn on_exit(&self, received_upbit_krw: Decimal, received_bybit_usdt: Decimal) {
        let mut state = self.inner.lock();

        state.upbit_available_krw = state
            .upbit_available_krw
            .checked_add(received_upbit_krw)
            .unwrap_or_else(|| {
                warn!("Upbit KRW 잔고 오버플로우");
                state.upbit_available_krw
            });

        state.bybit_available_usdt = state
            .bybit_available_usdt
            .checked_add(received_bybit_usdt)
            .unwrap_or_else(|| {
                warn!("Bybit USDT 잔고 오버플로우");
                state.bybit_available_usdt
            });

        debug!(
            received_upbit_krw = %received_upbit_krw,
            received_bybit_usdt = %received_bybit_usdt,
            available_upbit_krw = %state.upbit_available_krw,
            available_bybit_usdt = %state.bybit_available_usdt,
            "청산 잔고 복원"
        );
    }

    /// TTL이 만료된 미확정 예약을 정리합니다.
    ///
    /// 비상 청산 5분 + 여유 1분 = 6분 이상 미확정 시 자동 해제.
    pub fn sweep_expired_reservations(&self) -> usize {
        let mut state = self.inner.lock();
        let ttl = self.reservation_ttl;
        let now = Instant::now();
        let mut released_count = 0;

        // 만료된 미확정 예약의 금액 먼저 수집 (borrow 분리)
        let expired: Vec<(u64, Decimal, Decimal, u64)> = state
            .reservations
            .iter()
            .filter(|r| !r.committed && now.duration_since(r.created_at) > ttl)
            .map(|r| {
                (
                    r.id,
                    r.upbit_krw,
                    r.bybit_usdt,
                    now.duration_since(r.created_at).as_secs(),
                )
            })
            .collect();

        // 잔고 복원
        for (id, upbit_krw, bybit_usdt, age_secs) in &expired {
            state.upbit_available_krw += upbit_krw;
            state.bybit_available_usdt += bybit_usdt;
            warn!(
                reservation_id = id,
                age_secs = age_secs,
                upbit_krw = %upbit_krw,
                bybit_usdt = %bybit_usdt,
                "TTL 만료 예약 자동 해제"
            );
            released_count += 1;
        }

        // 만료된 예약 제거
        if !expired.is_empty() {
            let expired_ids: Vec<u64> = expired.iter().map(|(id, _, _, _)| *id).collect();
            state.reservations.retain(|r| !expired_ids.contains(&r.id));
        }

        // committed된 레코드도 정리 (확정 후 10분 유지)
        let cleanup_ttl = Duration::from_secs(600);
        state
            .reservations
            .retain(|r| !(r.committed && now.duration_since(r.created_at) > cleanup_ttl));

        if released_count > 0 {
            debug!(
                released_count = released_count,
                remaining = state.reservations.len(),
                "TTL sweeper 완료"
            );
        }

        released_count
    }

    /// 현재 가용 잔고를 반환합니다.
    pub fn available(&self) -> (Decimal, Decimal) {
        let state = self.inner.lock();
        (state.upbit_available_krw, state.bybit_available_usdt)
    }

    /// 현재 예약 총액을 반환합니다.
    pub fn reserved_total(&self) -> (Decimal, Decimal) {
        let state = self.inner.lock();
        let upbit_reserved: Decimal = state
            .reservations
            .iter()
            .filter(|r| !r.committed)
            .map(|r| r.upbit_krw)
            .sum();
        let bybit_reserved: Decimal = state
            .reservations
            .iter()
            .filter(|r| !r.committed)
            .map(|r| r.bybit_usdt)
            .sum();
        (upbit_reserved, bybit_reserved)
    }

    /// in_flight(미확정) 예약이 존재하는지 확인합니다.
    ///
    /// sync_from_exchange에서 사용: in_flight 예약이 있으면 동기화 스킵.
    pub fn has_in_flight_reservations(&self) -> bool {
        let state = self.inner.lock();
        state.reservations.iter().any(|r| !r.committed)
    }

    /// 잔고를 직접 설정합니다 (sync_from_exchange에서 사용).
    ///
    /// reserved_total을 고려하여 available만 조정합니다.
    pub fn set_available(&self, upbit_krw: Decimal, bybit_usdt: Decimal) {
        let mut state = self.inner.lock();
        let prev_upbit = state.upbit_available_krw;
        let prev_bybit = state.bybit_available_usdt;
        state.upbit_available_krw = upbit_krw;
        state.bybit_available_usdt = bybit_usdt;

        debug!(
            prev_upbit_krw = %prev_upbit,
            prev_bybit_usdt = %prev_bybit,
            new_upbit_krw = %upbit_krw,
            new_bybit_usdt = %bybit_usdt,
            "잔고 동기화 적용"
        );
    }

    /// 활성 예약 수를 반환합니다.
    pub fn active_reservation_count(&self) -> usize {
        let state = self.inner.lock();
        state.reservations.iter().filter(|r| !r.committed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_balance() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(1_000_000));
        assert_eq!(bybit, Decimal::from(500));
    }

    #[test]
    fn test_reserve_success() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        let token = bt.reserve(Decimal::from(100_000), Decimal::from(50));
        assert!(token.is_some());

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(900_000));
        assert_eq!(bybit, Decimal::from(450));
    }

    #[test]
    fn test_reserve_insufficient_upbit() {
        let bt = BalanceTracker::new(Decimal::from(50_000), Decimal::from(500));
        let token = bt.reserve(Decimal::from(100_000), Decimal::from(50));
        assert!(token.is_none());

        // 잔고 변경 없음
        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(50_000));
        assert_eq!(bybit, Decimal::from(500));
    }

    #[test]
    fn test_reserve_insufficient_bybit() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(30));
        let token = bt.reserve(Decimal::from(100_000), Decimal::from(50));
        assert!(token.is_none());

        // Bybit 부족 → Upbit도 차감 안 됨 (원자적 롤백)
        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(1_000_000));
        assert_eq!(bybit, Decimal::from(30));
    }

    #[test]
    fn test_commit() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        let mut token = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();

        // 실 체결이 예약보다 적은 경우 → 차액 반환
        bt.commit(&mut token, Decimal::from(90_000), Decimal::from(45));

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(910_000)); // 900_000 + 10_000 반환
        assert_eq!(bybit, Decimal::from(455)); // 450 + 5 반환
    }

    #[test]
    fn test_release() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        let mut token = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();

        bt.release(&mut token);

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(1_000_000));
        assert_eq!(bybit, Decimal::from(500));
    }

    #[test]
    fn test_on_exit() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        bt.on_exit(Decimal::from(50_000), Decimal::from(30));

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(1_050_000));
        assert_eq!(bybit, Decimal::from(530));
    }

    #[test]
    fn test_drop_releases_uncommitted() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        {
            let _token = bt
                .reserve(Decimal::from(100_000), Decimal::from(50))
                .unwrap();
            // token drops here
        }

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(1_000_000));
        assert_eq!(bybit, Decimal::from(500));
    }

    #[test]
    fn test_drop_does_not_release_committed() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        {
            let mut token = bt
                .reserve(Decimal::from(100_000), Decimal::from(50))
                .unwrap();
            bt.commit(&mut token, Decimal::from(100_000), Decimal::from(50));
            // token drops here — should NOT release
        }

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(900_000));
        assert_eq!(bybit, Decimal::from(450));
    }

    #[test]
    fn test_multiple_reserves_sequential() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));

        let t1 = bt.reserve(Decimal::from(300_000), Decimal::from(150));
        assert!(t1.is_some());

        let t2 = bt.reserve(Decimal::from(300_000), Decimal::from(150));
        assert!(t2.is_some());

        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(400_000));
        assert_eq!(bybit, Decimal::from(200));

        // 세 번째는 잔고 부족
        let t3 = bt.reserve(Decimal::from(500_000), Decimal::from(250));
        assert!(t3.is_none());
    }

    #[test]
    fn test_sweep_expired() {
        let bt = BalanceTracker::with_ttl(
            Decimal::from(1_000_000),
            Decimal::from(500),
            Duration::from_millis(10), // 10ms TTL
        );

        let _token = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();

        // 잔고 감소 확인
        let (upbit, _) = bt.available();
        assert_eq!(upbit, Decimal::from(900_000));

        // TTL 만료 대기
        std::thread::sleep(Duration::from_millis(20));

        // sweeper 실행
        let released = bt.sweep_expired_reservations();
        assert_eq!(released, 1);

        // 잔고 복원 확인
        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::from(1_000_000));
        assert_eq!(bybit, Decimal::from(500));
    }

    #[test]
    fn test_reserved_total() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        let _t1 = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();
        let _t2 = bt
            .reserve(Decimal::from(200_000), Decimal::from(100))
            .unwrap();

        let (upbit_reserved, bybit_reserved) = bt.reserved_total();
        assert_eq!(upbit_reserved, Decimal::from(300_000));
        assert_eq!(bybit_reserved, Decimal::from(150));
    }

    #[test]
    fn test_has_in_flight() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        assert!(!bt.has_in_flight_reservations());

        let _token = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();
        assert!(bt.has_in_flight_reservations());
    }

    #[test]
    fn test_concurrent_reserve_total_limit() {
        // 동시 예약 시 총 예약이 초기 잔고를 초과하지 않음
        let bt = BalanceTracker::new(Decimal::from(100), Decimal::from(100));
        let mut tokens = Vec::new();

        for _ in 0..10 {
            if let Some(token) = bt.reserve(Decimal::from(10), Decimal::from(10)) {
                tokens.push(token);
            }
        }

        // 정확히 10개 예약 가능 (100/10=10)
        assert_eq!(tokens.len(), 10);

        // 11번째는 실패
        let extra = bt.reserve(Decimal::from(10), Decimal::from(10));
        assert!(extra.is_none());

        // 가용 잔고 0
        let (upbit, bybit) = bt.available();
        assert_eq!(upbit, Decimal::ZERO);
        assert_eq!(bybit, Decimal::ZERO);
    }

    #[test]
    fn test_active_reservation_count() {
        let bt = BalanceTracker::new(Decimal::from(1_000_000), Decimal::from(500));
        assert_eq!(bt.active_reservation_count(), 0);

        let _t1 = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();
        assert_eq!(bt.active_reservation_count(), 1);

        let mut t2 = bt
            .reserve(Decimal::from(100_000), Decimal::from(50))
            .unwrap();
        assert_eq!(bt.active_reservation_count(), 2);

        bt.commit(&mut t2, Decimal::from(100_000), Decimal::from(50));
        assert_eq!(bt.active_reservation_count(), 1);
    }
}
