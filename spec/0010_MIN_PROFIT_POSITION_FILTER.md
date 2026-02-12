# 0010_MIN_PROFIT_POSITION_FILTER

## 사용자의 요청

시뮬레이션에서는 수익이지만 실전 마켓 임팩트로 손실 전환되는 **영양가 낮은 거래**를 사전에 걸러내는 필터 추가.

> "시뮬레이션이라서 내 포지션 진입이 실제 차트에 영향을 안주는데, 실제로 진입하면 내 진입이 차트에 영향을 줄 거거든. 그러면 이런 영양가가 낮고 리스크가 큰 거래를 걸러내기 위해선 어떻게 해야할까"

### 배경

16.7시간 라이브 시뮬레이션(255건, +317 USDT) 분석 결과:

- **마켓 임팩트 5bps 적용 시**: 34건이 손실 전환 → PnL +317 → +249 (-21%)
- **마켓 임팩트 10bps 적용 시**: 90건이 손실 전환 → PnL +317 → +180 (-43%)
- 사망 34건의 공통 특징: **avg z_score=1.59**, **avg ROI=0.02%**, **ZRO 85%**

필터 A(`min_roi >= 0.10%` + `min_position >= 100 USDT`) 적용 시:
- 거래 55% 제거(255→115), 시뮬 PnL 93% 유지(+293)
- 10bps 실전에서 +194 USDT (필터 없음 +180 대비 **+8% 개선**)
- 마켓 임팩트가 클수록 필터의 이점이 커짐

### 확정 요구사항

| 항목 | 설정 |
|------|------|
| 필터 1 | `min_expected_roi` — 최소 기대 수익률 (%), 기본값 0.10 |
| 필터 2 | `min_position_usdt` — 최소 포지션 크기 (USDT), 기본값 100.0 |
| 적용 위치 | monitor.rs 진입 검증 ⑥번(post-rounding PnL gate) 이후, 포지션 오픈 전 |
| 구현 방식 | 기존 PnL gate와 별도 단계 + 전용 모니터링 카운터 |
| 청산 영향 | 없음 (진입 필터만) |
| 하위 호환 | `min_expected_roi = 0.0`, `min_position_usdt = 0.0` 이면 기존 동작 100% 유지 |
| TOML 설정 | strategy.toml `[zscore]` 섹션에 Optional 필드 추가 |

---

## 설계

### 진입 검증 흐름 (변경 후)

```
signal::evaluate_entry_signal() → Signal::Enter
  │
  ├─ InstrumentInfo 필수 체크
  ├─ 오더북 safe_volume 계산
  │
  ├─ ① qty 라운딩 (floor to qty_step)
  ├─ ② qty == 0 체크
  ├─ ③ min/max order qty + min_notional 검증
  ├─ ④ Upbit KRW 5100원 하한 검증
  ├─ ⑤ 가격 라운딩 (Upbit ceil, Bybit floor)
  ├─ ⑥ Post-rounding PnL gate (adjusted_profit > 0)      ← 기존
  ├─ ⑦ min_position_usdt 체크                              ← 신규
  ├─ ⑧ min_expected_roi 체크                               ← 신규
  │
  └─ VirtualPosition 생성 + open_position
```

### ⑦ min_position_usdt 체크

```rust
// qty * bybit_price = 실제 포지션 크기 (USDT)
let actual_size_usdt = qty * bybit_entry;  // 라운딩된 가격 기준
if actual_size_usdt < config.min_position_usdt {
    debug!(
        coin = c.as_str(),
        actual_size_usdt = %actual_size_usdt,
        min_position_usdt = %config.min_position_usdt,
        "진입 거부: 최소 포지션 크기 미달"
    );
    Some("min_position")
}
```

### ⑧ min_expected_roi 체크

`adjusted_profit`는 이미 ⑥에서 계산된 값(라운딩 비용 차감 후 기대 수익률 %)을 재활용합니다.

```rust
// adjusted_profit는 ⑥에서 이미 > 0 통과한 값
if adjusted_profit < config.min_expected_roi {
    debug!(
        coin = c.as_str(),
        adjusted_profit = adjusted_profit,
        min_expected_roi = config.min_expected_roi,
        "진입 거부: 최소 기대 수익률 미달"
    );
    Some("min_roi")
}
```

### 모니터링 카운터

`MonitoringCounters`에 2개 필드 추가:

```rust
/// 최소 포지션 크기 미달로 진입 거부된 횟수.
pub entry_rejected_min_position_count: u64,
/// 최소 기대 수익률 미달로 진입 거부된 횟수.
pub entry_rejected_min_roi_count: u64,
```

### Config 필드

`ZScoreConfig`에 2개 필드 추가:

```rust
/// 최소 기대 수익률 (%, 기본값: 0.10).
/// 라운딩 후 adjusted_profit가 이 값 미만이면 진입 거부.
/// 0.0이면 비활성화 (기존 동작과 동일).
pub min_expected_roi: f64,

/// 최소 포지션 크기 (USDT, 기본값: 100.0).
/// qty × bybit_price가 이 값 미만이면 진입 거부.
/// 0.0이면 비활성화.
pub min_position_usdt: Decimal,
```

`RawZScoreConfig`에도 대응 필드 추가 (Option<f64>).

---

## 구현 플랜

### Phase 1: Config 추가 (`config.rs`)

1. `ZScoreConfig`에 `min_expected_roi: f64`, `min_position_usdt: Decimal` 추가
2. `Default` impl: `min_expected_roi: 0.10`, `min_position_usdt: Decimal::new(100, 0)`
3. `RawZScoreConfig`에 `min_expected_roi: Option<f64>`, `min_position_usdt: Option<f64>` 추가
4. `RawZScoreConfig::default()`: 둘 다 `None`
5. `From<RawZScoreConfig> for ZScoreConfig`: `unwrap_or(0.10)`, `unwrap_or(100.0)` + Decimal 변환
6. `validate()`: `min_expected_roi >= 0.0`, `min_position_usdt >= 0.0` 검증

### Phase 2: 모니터링 카운터 (`output/summary.rs`)

1. `MonitoringCounters`에 2개 필드 추가
2. `SessionSummary`에 2개 필드 추가
3. `SessionSummary::calculate()`에서 카운터 복사
4. `to_text()`에 2줄 추가: "최소 포지션 미달 진입 거부: N건", "최소 ROI 미달 진입 거부: N건"
5. `to_json()`에 2개 키 추가

### Phase 3: 진입 필터 (`monitor.rs`)

기존 ⑥번 `if adjusted_profit <= 0.0 { Some("rounding_pnl") }` 블록 직후에:

```rust
// ⑥ 통과 후 추가 필터
} else if (qty * bybit_entry) < config.min_position_usdt {
    // ⑦ 최소 포지션 크기 체크
    debug!(..., "진입 거부: 최소 포지션 크기 미달");
    Some("min_position")
} else if adjusted_profit < config.min_expected_roi {
    // ⑧ 최소 기대 수익률 체크
    debug!(..., "진입 거부: 최소 기대 수익률 미달");
    Some("min_roi")
} else {
    // ⑨ VirtualPosition 생성 (기존 ⑦)
    ...
}
```

카운터 분기에 2개 추가:

```rust
"min_position" => {
    counters.lock().unwrap().entry_rejected_min_position_count += 1;
}
"min_roi" => {
    counters.lock().unwrap().entry_rejected_min_roi_count += 1;
}
```

### Phase 4: strategy.toml 업데이트

```toml
[zscore]
# ... 기존 설정 ...

# 실전 마켓 임팩트 필터 (spec/0010)
min_expected_roi = 0.10
min_position_usdt = 100.0
```

### Phase 5: 테스트

1. `config.rs`: 기본값 검증, TOML 파싱 검증, validate 검증
2. `monitor.rs` 또는 통합 테스트: min_position/min_roi 필터 동작 확인 (기존 테스트 패턴 따름)

---

## 파일 변경 목록

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/zscore/config.rs` | 수정 | ZScoreConfig, RawZScoreConfig, Default, From, validate에 필드 추가 |
| `crates/arb-strategy/src/output/summary.rs` | 수정 | MonitoringCounters, SessionSummary에 카운터 2개 + 출력 추가 |
| `crates/arb-strategy/src/zscore/monitor.rs` | 수정 | 진입 검증 ⑦⑧ 추가 + 카운터 분기 2개 추가 |
| `strategy.toml` | 수정 | `min_expected_roi`, `min_position_usdt` 추가 |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `signal.rs` | 통계/수수료 레이어, 필터 로직과 무관 |
| `position.rs` | 포지션 구조체 변경 없음 |
| `orderbook.rs` | 오더북 로직 변경 없음 |
| `pnl.rs` | PnL 계산 변경 없음 |
| `instrument.rs` | 라운딩 로직 변경 없음 |
| `writer.rs` | CSV/JSON 출력 스키마 변경 없음 |

---

## 검증

1. `cargo test -p arb-strategy` — 기존 249 + 신규 테스트 통과
2. `cargo clippy -p arb-strategy` — 경고 0
3. 라이브 테스트: `min_expected_roi = 0.10`, `min_position_usdt = 100.0` 설정 후 실행
   - 카운터 확인: `min_position_count > 0`, `min_roi_count > 0` 확인
   - 기존 대비 거래 수 감소 + 평균 PnL/거래 증가 확인
4. 하위 호환 테스트: `min_expected_roi = 0.0`, `min_position_usdt = 0.0` → 기존과 동일 결과 확인

---

## 알려진 한계

- **마켓 임팩트의 정확한 추정 불가**: 실제 마켓 임팩트는 시점·유동성·호가 깊이에 따라 달라짐. 고정 ROI 필터는 근사치.
- **최적값은 시장 상황에 따라 변동**: 0.10%는 현재 데이터 기준 최적이며, 변동성 증가 시 상향/하향 조정 필요.
- **청산에는 미적용**: 청산은 이미 열린 포지션을 닫는 것이므로 필터 적용 대상 아님.
