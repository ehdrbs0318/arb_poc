# 0002_SLIPPAGE_PROFITABILITY_GATE

## 사용자의 요청

진입 시그널 평가에서 수수료뿐 아니라 **슬리피지 비용까지 포함**하여 수익성을 판단하도록 수정:

> "진입할때와 청산할 때, z score와 더불어서 차익이 이득이 될 때만 진입하는 로직이 있는데(수수료적으로), 이 로직에 슬리피지도 고려하도록 반영해. 즉 포지션 진입을 한다면 무조건 이득이 나야 해"

### 배경

슬리피지 모델(`slippage.rs`)이 가격 조정을 수행하지만, **진입 수익성 재검증 없이** 바로 포지션을 연다. 결과적으로:

- **슬리피지 OFF**: entry_z=2.5 → +5.95 USDT (PF 5.60)
- **슬리피지 ON**: entry_z=2.5 → **-9.67 USDT** (PF 0.13)

수수료 기준으로는 수익이지만, 슬리피지 포함 시 손실인 거래가 대다수를 차지한다. **"진입하면 무조건 이득"** 보장을 위해 라운드트립 슬리피지 비용을 수익성 게이트에 반영해야 한다.

### 현재 흐름의 문제점

```
signal.rs: expected_profit = (spread - mean) - roundtrip_fee  ← 슬리피지 미포함
  ↓ Signal::Enter 생성
simulator.rs: 슬리피지 적용 → 가격 조정 → 수익성 재검증 없이 진입
```

### 확정 요구사항

| 항목 | 설정 |
|------|------|
| 적용 조건 | `volume_filter_enabled = true`일 때만 |
| 검증 시점 | simulator.rs에서 슬리피지 적용 후, 포지션 오픈 전 |
| 검증 대상 | 진입 슬리피지(실측) + 청산 슬리피지(추정, 동일 볼륨 가정) + 수수료 |
| 결과 | `adjusted_profit > 0`이면 진입, 아니면 `continue`로 건너뜀 |
| 하위 호환 | `volume_filter_enabled = false`일 때 기존 동작 100% 유지 |
| signal.rs 변경 | 없음 (볼륨 데이터에 접근하지 않는 레이어) |
| config 변경 | 없음 (새 설정 필드 불필요) |

---

## 설계

### 접근 방식: simulator.rs에서 슬리피지 적용 후 수익성 재검증

signal.rs는 통계/수수료 레이어로 볼륨 데이터에 접근하지 않으므로 변경하지 않는다. simulator.rs에서 슬리피지를 가격에 적용한 후, 조정된 가격으로 라운드트립 수익성을 재계산한다.

**대안 비교:**

| 방식 | 장점 | 단점 | 판정 |
|------|------|------|------|
| **A. simulator.rs 재검증** | 아키텍처 유지, 실제 가격 사용, 하위 호환 | Signal 생성 후 거부 (미미한 비용) | **채택** |
| B. signal.rs에 볼륨 전달 | 단일 게이트 | 레이어 결합, 라이브(monitor.rs)에 볼륨 없음 | 기각 |
| C. 고정 슬리피지 추정치 | 최소 변경 | 동적 슬리피지와 괴리, 이중 차감 문제 | 기각 |

### 수익성 공식

```
# 1단계: 진입 슬리피지는 이미 가격에 반영됨
adjusted_entry_spread = (adjusted_bybit - adjusted_upbit) / adjusted_upbit × 100

# 2단계: 청산 슬리피지 추정 (진입 시점과 동일 볼륨 가정)
#   청산: Upbit 매도(가격↓) + Bybit 매수(가격↑) → 스프레드 불리하게 변동
#   bps → % 변환: bps / 100
exit_slippage_cost_pct = (upbit_entry_slippage_bps + bybit_entry_slippage_bps) / 100

# 3단계: 라운드트립 수수료
roundtrip_fee_pct = (upbit_fee + bybit_fee) × 2 × 100   # 기본 0.21%

# 4단계: 조정된 기대 수익
adjusted_profit = (adjusted_entry_spread - mean_spread) - roundtrip_fee_pct - exit_slippage_cost_pct

# 게이트
adjusted_profit > 0  →  진입 허용
adjusted_profit ≤ 0  →  진입 거부 + 로그
```

**슬리피지 4이벤트 분석:**

| 이벤트 | 방향 | 가격 영향 | 반영 방식 |
|--------|------|----------|----------|
| 진입 Upbit 매수 | 가격↑ | 불리 | `adjusted_price`에 반영됨 |
| 진입 Bybit short | 가격↓ | 불리 | `adjusted_price`에 반영됨 |
| 청산 Upbit 매도 | 가격↓ | 불리 | `exit_slippage_cost_pct`로 추정 |
| 청산 Bybit close | 가격↑ | 불리 | `exit_slippage_cost_pct`로 추정 |

---

## 구현 플랜

### Phase 1: 수익성 검증 헬퍼 (`slippage.rs`)

`is_entry_profitable()` 함수를 `slippage.rs`에 추가한다.

```rust
/// 슬리피지 포함 라운드트립 수익성을 검증합니다.
///
/// 진입 슬리피지가 적용된 가격과 추정 청산 슬리피지를 포함하여
/// 기대 수익이 양수인지 판단합니다.
///
/// # 반환
///
/// `(수익성 여부, 조정된 기대 수익률 %)`
pub fn is_entry_profitable(
    adjusted_upbit_price: Decimal,     // 슬리피지 적용 후 Upbit 매수가
    adjusted_bybit_price: Decimal,     // 슬리피지 적용 후 Bybit 매도가
    mean_spread_pct: f64,              // rolling mean spread (%)
    upbit_entry_slippage_bps: f64,     // Upbit 진입 슬리피지 (bps)
    bybit_entry_slippage_bps: f64,     // Bybit 진입 슬리피지 (bps)
    upbit_taker_fee: Decimal,
    bybit_taker_fee: Decimal,
) -> (bool, f64) {
    let adj_upbit = adjusted_upbit_price.to_f64().unwrap_or(0.0);
    let adj_bybit = adjusted_bybit_price.to_f64().unwrap_or(0.0);

    // 슬리피지 적용 후 실제 진입 스프레드
    let adjusted_entry_spread = if adj_upbit > 0.0 {
        (adj_bybit - adj_upbit) / adj_upbit * 100.0
    } else {
        0.0
    };

    // 라운드트립 수수료 (%)
    let fee_pct = roundtrip_fee_pct(upbit_taker_fee, bybit_taker_fee)
        .to_f64().unwrap_or(0.0);

    // 청산 슬리피지 추정 (동일 볼륨 가정, bps → %)
    let exit_slippage_cost_pct =
        (upbit_entry_slippage_bps + bybit_entry_slippage_bps) / 100.0;

    // 조정된 기대 수익
    let adjusted_profit =
        (adjusted_entry_spread - mean_spread_pct) - fee_pct - exit_slippage_cost_pct;

    (adjusted_profit > 0.0, adjusted_profit)
}
```

### Phase 2: simulator.rs Enter 핸들러 수정

기존 슬리피지 적용 블록에서 `slippage_bps`를 캡처하고, 수익성 재검증을 추가한다.

**변경 전 (현재 코드):**
```rust
// 505행: if config.volume_filter_enabled {
//   ...슬리피지 적용...
// 580행: }
// 582행: let liq_price = ...  ← 바로 포지션 생성
```

**변경 후:**
```rust
// 슬리피지 bps 캡처용 변수
let mut upbit_slippage_bps = 0.0_f64;
let mut bybit_slippage_bps = 0.0_f64;

if config.volume_filter_enabled {
    // Upbit 슬리피지 (기존 코드 + bps 캡처)
    if let Some(uv) = upbit_vol {
        match slippage::calculate_slippage(...) {
            Some(r) => {
                upbit_slippage_bps = r.slippage_bps;  // 추가
                upbit_price = r.adjusted_price;
            }
            None => { continue; }
        }
    }

    // Bybit 슬리피지 (기존 코드 + bps 캡처)
    if let Some(bv) = bybit_vol {
        match slippage::calculate_slippage(...) {
            Some(r) => {
                bybit_slippage_bps = r.slippage_bps;  // 추가
                bybit_price_dec = r.adjusted_price;
            }
            None => { continue; }
        }
    }

    // ★ 신규: 슬리피지 포함 수익성 재검증
    let mean_val = spread_calc
        .spread_window(&c)
        .filter(|w| w.is_ready())
        .map(|w| statistics::mean(w.data()))
        .unwrap_or(0.0);

    let (profitable, adj_profit) = slippage::is_entry_profitable(
        upbit_price,
        bybit_price_dec,
        mean_val,
        upbit_slippage_bps,
        bybit_slippage_bps,
        config.upbit_taker_fee,
        config.bybit_taker_fee,
    );

    if !profitable {
        debug!(
            coin = c.as_str(),
            adjusted_profit_pct = adj_profit,
            original_expected_profit = expected_profit_pct,
            upbit_slippage_bps = upbit_slippage_bps,
            bybit_slippage_bps = bybit_slippage_bps,
            "진입 거부: 슬리피지 포함 시 수익성 부족"
        );
        continue;
    }

    debug!(
        coin = c.as_str(),
        adjusted_profit_pct = adj_profit,
        "슬리피지 포함 수익성 확인됨"
    );
}

// (이하 기존 코드: VirtualPosition 생성, open_position)
```

### Phase 3: 테스트

`slippage.rs`에 테스트 4개 추가:

| 테스트 | 시나리오 | 기대 결과 |
|--------|---------|----------|
| `test_is_entry_profitable_no_slippage` | slippage_bps=0, 스프레드 0.3%, mean 0%, fee 0.21% | profitable=true, profit≈0.09% |
| `test_is_entry_profitable_with_moderate_slippage` | slippage_bps=1.5 each, 스프레드 충분 | profitable=true |
| `test_is_entry_profitable_rejected_by_slippage` | slippage_bps=10 each, 스프레드 좁음 | profitable=false |
| `test_is_entry_profitable_borderline` | 수익=0에 가까운 경계값 | profitable=false (0 이하) |
| `test_is_entry_profitable_negative_mean` | mean < 0 (역김프), 스프레드 양수 | profitable=true (더 넓은 마진) |

---

## 파일 변경 목록

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/zscore/slippage.rs` | **수정** | `is_entry_profitable()` 함수 + 테스트 4개 추가 |
| `crates/arb-strategy/src/zscore/simulator.rs` | **수정** | Enter 핸들러에 bps 캡처 + 수익성 재검증 블록 추가 |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `signal.rs` | 통계/수수료 레이어, 볼륨 데이터 접근 없음 |
| `config.rs` | 새 설정 필드 불필요 |
| `monitor.rs` | 라이브 경로는 볼륨 데이터 없음, 향후 과제 |
| `fee.rs` | 기존 `roundtrip_fee_pct()` 그대로 사용 |

---

## 체크리스트

### Phase 1: 수익성 검증 헬퍼
- [x] `slippage.rs`에 `is_entry_profitable()` 함수 추가
- [x] `#[allow(clippy::too_many_arguments)]` 어트리뷰트 추가 (7개 인자)
- [x] `fee.rs`의 `roundtrip_fee_pct()` 임포트 (`use crate::common::fee::roundtrip_fee_pct;`)
- [x] `is_coin_volume` docstring 수정 (Bybit도 코인 수량임을 명시)

### Phase 2: simulator.rs 수정
- [x] bps 캡처 변수를 `if config.volume_filter_enabled` 블록 **안**에 선언 (`unused_mut` 방지)
- [x] 기존 슬리피지 적용 블록에서 bps 캡처 추가
- [x] 수익성 재검증: 기존 `mean_pct` 변수 재사용 (중복 계산 방지)
- [x] 거부 시 `continue` + 디버그 로그

### Phase 3: 테스트
- [x] `test_is_entry_profitable_no_slippage`
- [x] `test_is_entry_profitable_with_moderate_slippage`
- [x] `test_is_entry_profitable_rejected_by_slippage`
- [x] `test_is_entry_profitable_borderline`
- [x] `test_is_entry_profitable_negative_mean`

### Phase 4: 검증
- [x] `cargo test -p arb-strategy` — 113 통과 (기존 108 + 신규 5)
- [x] `cargo clippy -p arb-strategy` — 경고 0
- [x] sweep 비교 실행 (volume_filter OFF vs ON)
- [x] 디버그 로그 확인

---

## 알려진 한계

- **청산 슬리피지 추정**: 진입 시점의 슬리피지를 청산 슬리피지로 가정한다. 실제 청산 시 볼륨이 다를 수 있어 추정과 괴리가 발생할 수 있다.
- **라이브 미적용**: `monitor.rs` 라이브 경로는 캔들 볼륨 데이터가 없어 슬리피지 게이트를 적용하지 않는다. 향후 `MinuteCandleBuilder`에 볼륨 집계를 추가하면 동일 패턴으로 확장 가능하다.
- **보수적 필터링**: 슬리피지를 포함하면 진입 기회가 줄어든다. 이는 의도된 동작이며, 줄어든 거래 중 실제 수익 거래의 비율이 높아져야 한다.
- **filter ratio 모니터링**: 게이트 차단 비율이 90% 이상이면 전략 자체의 수익성을 재검토해야 한다.

---

## 리뷰 이력

### Rev 1 (2026-02-08) — spec-0002-review 팀 리뷰 반영

**Trader 리뷰 반영:**
- [확인] 수익성 공식 수학적 정확성 — bps→% 변환, 이중 차감 없음 확인
- [확인] 4이벤트 슬리피지 방향 모두 올바름
- [확인] 청산 슬리피지 추정은 모델 정밀도에 부합하는 합리적 근사치
- [반영] filter ratio 모니터링 권고 → 알려진 한계에 추가
- [반영] `is_coin_volume` docstring 불일치 수정 → 체크리스트에 추가
- [참고] 6-B 게이트 카운터, 6-C exit_slippage_multiplier → 향후 과제

**Coder 리뷰 반영:**
- [반영] `#[allow(clippy::too_many_arguments)]` 필수 → 체크리스트에 추가
- [반영] `mean_pct` 재사용 (중복 계산 방지) → Phase 2 수정
- [반영] bps 변수를 `volume_filter_enabled` 블록 안으로 이동 (`unused_mut` 방지) → Phase 2 수정
- [반영] 음수 mean 테스트 케이스 추가 → Phase 3에 추가 (5개 테스트)
- [확인] import 경로 `crate::common::fee::roundtrip_fee_pct` 문제없음
- [확인] signal.rs 불변 결정 아키텍처적으로 올바름
- [확인] 기존 108개 테스트 완전 호환
