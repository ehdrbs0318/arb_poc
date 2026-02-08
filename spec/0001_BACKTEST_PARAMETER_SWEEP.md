# 0001_BACKTEST_PARAMETER_SWEEP

## 사용자의 요청

기존 Z-Score 백테스트(`zscore_backtest`)를 활용하여 **실제 거래소 데이터로 시뮬레이션**을 돌리되:

1. **차익거래 기회가 몇 번 있었는지** 확인
2. **각 거래소에 1,000 USDT**씩 있을 때 (Upbit은 1,000 USDT 상당의 KRW) 수익이 얼마인지 확인
3. **entry_z 임계값을 여러 값으로 비교**하여 최적 파라미터를 탐색

### 배경

라이브 테스트(2026-02-08)에서 Z-Score 범위가 **-2.43 ~ +1.74**로 관측되었으며, 기본 entry_z=2.0에서는 **시그널 0건**이었다. entry_z를 낮추면 거래 기회가 늘어나지만 수익성이 떨어질 수 있으므로, 여러 임계값으로 비교 시뮬레이션이 필요하다.

### 확정 요구사항

| 항목 | 설정 |
|------|------|
| 기반 | 기존 `BacktestSimulator<U, B>` + `ZScoreConfig` 재활용 |
| 자본 | **2,000 USDT** (total_capital_usdt), position_ratio=**0.5** → 각 거래소 1,000 USDT |
| entry_z 비교 대상 | **[1.0, 1.25, 1.5, 1.75, 2.0, 2.5]** (TOML 설정으로 변경 가능) |
| exit_z | 기본 0.5 (entry_z와 함께 sweep 가능하도록 설계) |
| 코인 | 설정 파일(`strategy.toml`)로 자유 선택 |
| 백테스트 기간 | 설정 파일로 자유 설정 (기본값 유지: 8640분 = 6일) |
| 출력 | 콘솔 비교 테이블 + 개별 CSV + 요약 CSV |
| 신규 코드 위치 | `examples/zscore_parameter_sweep.rs` |
| 라이브러리 변경 | `arb-strategy`에 sweep 전용 모듈 추가 (`zscore/sweep.rs`, `output/console.rs` 확장) |

### 자본 배분 상세

```
total_capital_usdt = 2000
position_ratio = 0.5

single_leg_size = 2000 × 0.5 = 1,000 USDT

∴ Upbit 현물: 1,000 USDT 상당 KRW로 코인 매수
   Bybit 선물: 1,000 USDT short (1배 레버리지)
   → 양쪽 자본 합 2,000 USDT 전부 사용
```

> **NOTE**: position_ratio=0.5이면 코인당 포지션 1개가 전체 자본을 사용한다. 다중 코인 동시 진입 시 자본 부족이 발생할 수 있으므로 `max_concurrent_positions = 1`을 권장한다.

---

## 구현 플랜

### Phase 1: Sweep 설정 구조체 (`zscore/sweep.rs`)

#### 1-1. SweepConfig

```rust
/// 파라미터 sweep 설정.
#[derive(Clone, Debug)]
pub struct SweepConfig {
    /// 기본 전략 설정 (sweep 대상 필드는 오버라이드됨).
    pub base_config: ZScoreConfig,
    /// entry_z_threshold sweep 대상 값 목록.
    pub entry_z_values: Vec<f64>,
    /// exit_z_threshold sweep 대상 값 목록 (비어있으면 base_config 값 사용).
    pub exit_z_values: Vec<f64>,
    /// entry_z × exit_z 최대 조합 수 (기본 50). 초과 시 에러 반환.
    pub max_combinations: usize,
}
```

> **가드레일**: `entry_z_values.len() × exit_z_values.len()` 조합이 `max_combinations`을 초과하면 sweep 시작 전에 에러를 반환한다. 의도치 않은 조합 폭발을 방지한다.

**TOML 설정 확장 (`strategy.toml`):**

```toml
[zscore]
coins = ["BTC"]
window_size = 1440
total_capital_usdt = 2000.0
position_ratio = 0.5
max_concurrent_positions = 1
backtest_period_minutes = 8640
output_dir = "./output/"

# 파라미터 sweep 설정 (zscore_parameter_sweep 예제 전용)
[sweep]
entry_z_values = [1.0, 1.25, 1.5, 1.75, 2.0, 2.5]
# exit_z_values = [0.3, 0.5, 0.7]   # 생략 시 기본 exit_z 사용
# max_combinations = 50              # 조합 수 가드레일 (기본 50)
```

#### 1-1a. RawSweepConfig (TOML 파싱 중간 구조체)

기존 `RawZScoreConfig` 패턴을 따라 TOML `[sweep]` 섹션을 파싱한다.

```rust
/// TOML [sweep] 섹션 파싱용 중간 구조체.
#[derive(Debug, Deserialize)]
pub struct RawSweepConfig {
    /// entry_z sweep 값 목록 (필수).
    pub entry_z_values: Vec<f64>,
    /// exit_z sweep 값 목록 (선택, 생략 시 base_config.exit_z 사용).
    pub exit_z_values: Option<Vec<f64>>,
    /// 최대 조합 수 (선택, 기본 50).
    pub max_combinations: Option<usize>,
}

impl RawSweepConfig {
    /// RawSweepConfig + ZScoreConfig → SweepConfig 변환.
    pub fn into_sweep_config(self, base_config: ZScoreConfig) -> SweepConfig {
        let exit_z_values = self.exit_z_values
            .unwrap_or_else(|| vec![base_config.exit_z_threshold.to_f64()]);
        SweepConfig {
            base_config,
            entry_z_values: self.entry_z_values,
            exit_z_values,
            max_combinations: self.max_combinations.unwrap_or(50),
        }
    }
}
```

#### 1-2. SweepResult

```rust
/// 단일 파라미터 조합의 백테스트 결과 요약.
#[derive(Clone, Debug)]
pub struct SweepResultRow {
    pub entry_z: f64,
    pub exit_z: f64,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub liquidated_trades: usize,
    pub win_rate: f64,
    pub total_pnl: Decimal,
    pub total_fees: Decimal,
    pub net_pnl: Decimal,
    pub max_drawdown: Decimal,
    pub avg_holding_minutes: f64,
    /// 실현 수익률 (%) = net_pnl / total_capital × 100 (청산된 거래만)
    pub realized_roi_pct: f64,
    /// 총 수익률 (%) = (net_pnl + unrealized_pnl) / total_capital × 100 (미청산 포함)
    pub total_roi_pct: f64,
    /// 미청산 포지션 수
    pub open_position_count: usize,
    /// 미청산 포지션 Unrealized PnL
    pub unrealized_pnl: Decimal,
    /// Profit Factor = 총 이익 / 총 손실 (손실 0이면 f64::INFINITY)
    pub profit_factor: f64,
    /// 수익 / 최대 낙폭 비율 (max_drawdown 0이면 f64::INFINITY)
    pub return_max_dd_ratio: f64,
}

/// 전체 sweep 결과.
pub struct SweepResult {
    pub rows: Vec<SweepResultRow>,
    pub coin: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_capital_usdt: Decimal,
}
```

### Phase 2: 데이터 수집 최적화

현재 `BacktestSimulator::run()`은 매 실행마다 REST API로 캔들 데이터를 수집한다. 파라미터 sweep에서 entry_z만 바꾸고 **동일한 캔들 데이터**를 재사용해야 한다.

#### 2-1. 데이터 캐시 분리

`BacktestSimulator`의 데이터 수집 로직을 **별도 함수로 분리**하여, 한 번 수집한 데이터를 여러 시뮬레이션에서 재활용한다.

```rust
/// 캔들 데이터 캐시 — 한 번 수집, 여러 번 시뮬레이션.
pub struct CandleDataCache {
    /// 코인별 Upbit 캔들 (KRW-{COIN}).
    pub upbit_coin_candles: HashMap<String, Vec<Candle>>,
    /// Upbit USDT/KRW 캔들 (KRW-USDT).
    pub upbit_usdt_candles: Vec<Candle>,
    /// 코인별 Bybit 캔들 ({COIN}USDT).
    pub bybit_candles: HashMap<String, Vec<Candle>>,
}
```

```rust
impl<U: MarketData, B: MarketData> BacktestSimulator<U, B> {
    /// 캔들 데이터만 수집하여 캐시로 반환한다 (API 호출).
    pub async fn fetch_data(&self) -> Result<CandleDataCache, StrategyError>;

    /// 기존 run() — fetch_data() + simulate_with_cache() 조합 (호환성 유지).
    pub async fn run(&self) -> Result<BacktestResult, StrategyError>;
}

/// 캐시된 데이터로 시뮬레이션만 실행한다 (API 호출 없음).
///
/// **독립 함수**로 분리하여 MarketData 제네릭 없이 호출 가능.
/// sweep 엔진에서 config만 바꿔가며 반복 호출할 수 있다.
pub fn simulate_with_cache(
    config: &ZScoreConfig,
    cache: &CandleDataCache,
) -> Result<BacktestResult, StrategyError>;
```

> **설계 근거**: `run_with_cache`를 `BacktestSimulator`의 메서드(`&self`)로 두면 MarketData 제네릭이 불필요하게 전파된다. 시뮬레이션 단계에서는 이미 캐시된 캔들 데이터만 사용하므로, **독립 함수** `simulate_with_cache(config, cache)`로 분리하여 sweep 엔진에서 config만 교체하며 호출할 수 있도록 한다.

> **핵심**: entry_z=1.0부터 2.5까지 6번 실행해도 API 호출은 **1번만** 발생한다.

### Phase 3: Sweep 실행 엔진

#### 3-1. `run_sweep` 함수

```rust
/// 파라미터 sweep을 실행한다.
///
/// 1. 캔들 데이터를 1회 수집 (fetch_data)
/// 2. entry_z × exit_z 조합별로 시뮬레이션 실행 (run_with_cache)
/// 3. 결과를 SweepResult로 집계
pub async fn run_sweep<U: MarketData, B: MarketData>(
    upbit: &U,
    bybit: &B,
    sweep_config: &SweepConfig,
) -> Result<SweepResult, StrategyError>;
```

**실행 흐름:**

```
1. 조합 수 검증: entry_z_values.len() × exit_z_values.len() <= max_combinations
   → 초과 시 StrategyError::InvalidParameter 반환
2. base_config로 BacktestSimulator 생성
3. fetch_data() → CandleDataCache (API 1회)
4. for entry_z in entry_z_values (오름차순 정렬):
     for exit_z in exit_z_values (또는 base_config.exit_z):
       config' = base_config.clone() { entry_z, exit_z }
       result = simulate_with_cache(&config', &cache)
       match result:
         Ok(r)  → rows.push(SweepResultRow::from(r, total_capital))
         Err(e) → warn!("entry_z={entry_z}, exit_z={exit_z} 실패: {e}") → 건너뜀
5. SweepResult 반환 (partial result 허용, 전체 실패 시에만 에러)
```

> **에러 처리 전략**: 개별 파라미터 조합의 시뮬레이션 실패는 경고 로그를 남기고 건너뛴다. 성공한 조합만 결과에 포함하여 **partial result**를 반환한다. 모든 조합이 실패한 경우에만 에러를 반환한다.

> **entry_z < 1.25 경고**: entry_z가 1.25 미만인 값이 포함되면 콘솔에 경고를 출력한다. 낮은 entry_z는 노이즈 거래를 다수 발생시켜 수수료 손실이 수익을 초과할 가능성이 높다.

### Phase 4: 출력 확장

#### 4-1. 콘솔 비교 테이블 (`output/console.rs` 확장)

```rust
/// 파라미터 sweep 결과를 콘솔 테이블로 출력한다.
pub fn print_sweep_summary(result: &SweepResult);
```

**출력 형식 예시:**

```
=== Z-Score 파라미터 Sweep 결과 ===

코인: BTC | 기간: 2026-02-02 ~ 2026-02-08 | 자본: 2,000 USDT

 entry_z | exit_z | 거래수 | 승률   | 순 PnL     | 실현ROI | 총ROI  | PF   | Ret/DD | Max DD    | 평균보유
---------|--------|--------|--------|------------|---------|--------|------|--------|-----------|--------
 ⚠ 1.00 |   0.50 |     12 | 58.3%  |  +3.42 USDT|  +0.17% | +0.17% | 1.42 |   1.63 | 2.10 USDT|  45.2분
    1.25 |   0.50 |      8 | 62.5%  |  +4.18 USDT|  +0.21% | +0.21% | 1.68 |   2.32 | 1.80 USDT|  62.3분
    1.50 |   0.50 |      5 | 60.0%  |  +3.95 USDT|  +0.20% | +0.20% | 1.55 |   2.63 | 1.50 USDT|  88.4분
    1.75 |   0.50 |      2 | 100.0% |  +2.80 USDT|  +0.14% | +0.14% |  Inf |    Inf | 0.00 USDT| 120.5분
    2.00 |   0.50 |      0 |   N/A  |  +0.00 USDT|  +0.00% | +0.00% |  N/A |    N/A | 0.00 USDT|   N/A
    2.50 |   0.50 |      0 |   N/A  |  +0.00 USDT|  +0.00% | +0.00% |  N/A |    N/A | 0.00 USDT|   N/A

⚠ entry_z < 1.25: 노이즈 거래 다수 발생 가능, 수수료 손실 주의

★ 최적: entry_z=1.25, exit_z=0.50 (총ROI +0.21%, PF 1.68, 8거래)
```

> **참고**: 위 수치는 예시이며, 실제 결과와 다를 수 있습니다.
>
> **컬럼 설명**:
> - **실현ROI**: 청산된 거래만 기준 수익률
> - **총ROI**: 미청산 포지션 포함 수익률 (실현 + unrealized)
> - **PF**: Profit Factor (총이익 / 총손실, 높을수록 좋음)
> - **Ret/DD**: Return / Max Drawdown 비율 (리스크 대비 수익)

#### 4-2. Sweep 요약 CSV (`output/csv.rs` 확장)

```rust
/// sweep 결과를 CSV로 저장한다.
pub fn write_sweep_csv(
    output_dir: &Path,
    result: &SweepResult,
) -> Result<String, StrategyError>;
```

**파일명**: `sweep_{timestamp}.csv`

CSV 파일은 **메타데이터 헤더**와 **데이터 행**으로 구성된다.

```csv
# coin=BTC,period_start=2026-02-02T00:00:00Z,period_end=2026-02-08T00:00:00Z,total_capital_usdt=2000
entry_z,exit_z,total_trades,winning_trades,losing_trades,liquidated_trades,win_rate,total_pnl,total_fees,net_pnl,max_drawdown,avg_holding_min,realized_roi_pct,total_roi_pct,open_positions,unrealized_pnl,profit_factor,return_max_dd_ratio
1.00,0.50,12,7,5,0,0.583,4.52,1.10,3.42,2.10,45.2,0.17,0.17,0,0.00,1.42,1.63
1.25,0.50,8,5,3,0,0.625,5.28,1.10,4.18,1.80,62.3,0.21,0.21,0,0.00,1.68,2.32
```

### Phase 5: 예제 작성

#### 5-1. `examples/zscore_parameter_sweep.rs`

```rust
//! Z-Score 파라미터 sweep 백테스트.
//!
//! entry_z 임계값을 여러 값으로 비교하여 최적 파라미터를 탐색합니다.
//! 캔들 데이터는 1회만 수집하여 모든 파라미터 조합에 재활용합니다.
//!
//! ## 실행 방법
//!
//! ```bash
//! cargo run --example zscore_parameter_sweep
//! STRATEGY_CONFIG=strategy.toml cargo run --example zscore_parameter_sweep
//! ```

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 설정 로드 (strategy.toml)
    // 2. SweepConfig 구성 (TOML [sweep] 섹션 또는 기본값)
    // 3. 거래소 클라이언트 생성
    // 4. run_sweep() 실행
    // 5. 콘솔 비교 테이블 출력
    // 6. sweep 요약 CSV 저장
    // 7. (선택) 최적 파라미터의 개별 거래 상세 출력
}
```

---

## 설정 파일 예시

### `strategy.toml` (1,000 USDT/거래소, BTC 단일 코인, 30일)

```toml
[zscore]
coins = ["BTC"]
window_size = 1440
entry_z_threshold = 2.0      # sweep에서 오버라이드됨
exit_z_threshold = 0.5
total_capital_usdt = 2000.0   # 양 거래소 합산 → 거래소당 1,000 USDT
position_ratio = 0.5          # 전체 자본 사용
max_concurrent_positions = 1
backtest_period_minutes = 41760  # 29일 (원하는 기간 자유 설정)
output_dir = "./output/"

[sweep]
entry_z_values = [1.0, 1.25, 1.5, 1.75, 2.0, 2.5]
# exit_z_values = [0.3, 0.5, 0.7]
```

### `strategy.toml` (다중 코인, 기본 6일)

```toml
[zscore]
coins = ["BTC", "ETH", "XRP"]
window_size = 1440
total_capital_usdt = 2000.0
position_ratio = 0.5
max_concurrent_positions = 1
backtest_period_minutes = 8640

[sweep]
entry_z_values = [1.0, 1.5, 2.0]
```

---

## 파일 변경 목록

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/zscore/sweep.rs` | **신규** | SweepConfig, SweepResultRow, SweepResult, run_sweep() |
| `crates/arb-strategy/src/zscore/simulator.rs` | **수정** | CandleDataCache 구조체, fetch_data(), 독립 함수 simulate_with_cache() 추가 |
| `crates/arb-strategy/src/zscore/mod.rs` | **수정** | `pub mod sweep;` 추가 |
| `crates/arb-strategy/src/zscore/config.rs` | **수정** | SweepConfig TOML 파싱 (`[sweep]` 섹션) |
| `crates/arb-strategy/src/output/console.rs` | **수정** | `print_sweep_summary()` 함수 추가 |
| `crates/arb-strategy/src/output/csv.rs` | **수정** | `write_sweep_csv()` 함수 추가 |
| `examples/zscore_parameter_sweep.rs` | **신규** | 파라미터 sweep 실행 예제 |
| `strategy.example.toml` | **수정** | `[sweep]` 섹션 추가 |

---

## 체크리스트

### Phase 1: Sweep 설정
- [ ] `zscore/sweep.rs` 파일 생성
- [ ] `SweepConfig` 구조체 정의 (`max_combinations` 가드레일 포함)
- [ ] `SweepResultRow` 구조체 정의 (`realized_roi_pct`, `total_roi_pct`, `profit_factor`, `return_max_dd_ratio` 포함)
- [ ] `SweepResult` 구조체 정의
- [ ] `RawSweepConfig` TOML 파싱 중간 구조체 정의 (`config.rs`)
- [ ] `config.rs`에 `[sweep]` TOML 섹션 파싱 추가
- [ ] `zscore/mod.rs`에 `pub mod sweep;` 추가

### Phase 2: 데이터 캐시 분리
- [ ] `CandleDataCache` 구조체 정의 (`simulator.rs`)
- [ ] `BacktestSimulator::fetch_data()` 메서드 추출
- [ ] 독립 함수 `simulate_with_cache(config, cache)` 추가 (MarketData 제네릭 불필요)
- [ ] 기존 `run()` 메서드가 `fetch_data()` + `simulate_with_cache()` 조합으로 동작하도록 리팩터링
- [ ] 기존 테스트 통과 확인

### Phase 3: Sweep 실행 엔진
- [ ] 조합 수 가드레일 검증 (entry_z × exit_z <= max_combinations)
- [ ] `run_sweep()` 함수 구현
- [ ] entry_z × exit_z 조합별 `simulate_with_cache()` 호출
- [ ] 개별 조합 실패 시 경고 로그 + 건너뛰기 (partial result)
- [ ] `SweepResultRow::from(BacktestResult, total_capital)` 변환 구현
- [ ] ROI 이중 계산 (realized_roi_pct + total_roi_pct)
- [ ] 리스크 지표 계산 (profit_factor, return_max_dd_ratio)
- [ ] 최적 파라미터 식별 (total_roi_pct 기준)
- [ ] entry_z < 1.25 경고 출력

### Phase 4: 출력 확장
- [ ] `console::print_sweep_summary()` 구현 (PF, Ret/DD 컬럼 포함)
- [ ] `csv::write_sweep_csv()` 구현 (메타데이터 헤더 + 리스크 지표 컬럼)
- [ ] 기존 console/csv 함수 호환성 유지

### Phase 5: 예제 및 통합
- [ ] `examples/zscore_parameter_sweep.rs` 예제 작성
- [ ] `strategy.example.toml`에 `[sweep]` 섹션 추가
- [ ] `cargo clippy --all-targets --all-features` 경고 0
- [ ] `cargo test --workspace` 전체 통과
- [ ] 실제 데이터로 sweep 실행 및 결과 검증

---

## 알려진 한계

- **데이터 기간 제한**: 거래소 API가 제공하는 과거 데이터에 한계가 있다. Upbit은 약 200일, Bybit은 약 1000일 이내의 1분봉을 제공한다.
- **API 호출 시간**: 30일치 데이터 수집 시 약 2~3분 소요 (페이지네이션 + rate limit). 그러나 sweep 시 API 호출은 1회로 캐싱되므로 추가 시간은 시뮬레이션 연산만큼만 발생한다.
- **다중 코인 = 코인별 독립 sweep**: `coins = ["BTC", "ETH"]` 설정 시 각 코인에 대해 **독립적으로** sweep을 실행한다. 코인 간 포트폴리오 효과(동시 진입, 자본 경합)는 고려하지 않는다. 통합 포트폴리오 분석은 향후 확장 과제로 남긴다.
- **과적합 주의 (CRITICAL)**: 파라미터 최적화는 과거 데이터에 과적합(overfitting)될 **높은 가능성**이 있다. 특히 entry_z를 세밀하게 sweep할수록 과적합 위험이 증가한다. sweep 결과는 **탐색적 분석(exploratory analysis)** 목적으로만 활용해야 하며, walk-forward validation, out-of-sample 검증 없이 sweep 최적값을 실전에 적용하는 것은 **위험하다**. 콘솔 출력에 이 경고를 항상 포함한다.
- **entry_z 하한 주의**: entry_z < 1.25인 값은 시장 노이즈에 의한 허위 시그널이 빈번하여, 수수료 비용이 수익을 초과할 가능성이 높다. 탐색 목적으로 포함하되 결과 해석 시 주의가 필요하다.
- **기존 0000 스펙의 알려진 한계**: 수수료 근사치, USDT/KRW 환율 리스크, 스프레드 발산 리스크 등은 그대로 적용된다.

---

## 리뷰 이력

### Rev 1 (2026-02-08) — arb_poc_team 리뷰 반영

**Trader 리뷰 반영:**
- [반영] ROI 이중 계산: `realized_roi_pct` + `total_roi_pct` (unrealized PnL 포함)
- [반영] 리스크 조정 지표 추가: `profit_factor`, `return_max_dd_ratio`
- [반영] entry_z < 1.25 경고 (노이즈 거래 주의)
- [반영] 과적합 경고 강화 (CRITICAL 등급)
- [반영] 조합 폭발 가드레일: `max_combinations` (기본 50)
- [반영] 다중 코인 = 코인별 독립 sweep 명확화
- [반영] CSV 메타데이터 헤더 추가

**Coder 리뷰 반영:**
- [반영] `run_with_cache` → 독립 함수 `simulate_with_cache(config, cache)` (MarketData 제네릭 불필요)
- [반영] `RawSweepConfig` TOML 파싱 중간 구조체 정의
- [반영] 개별 조합 실패 시 partial result 반환 전략
- [반영] entry_z 오름차순 정렬 보장
- [참고] 메모리 사용량 ~50MB (30일 × 3 시리즈) → 문제 없음 확인
