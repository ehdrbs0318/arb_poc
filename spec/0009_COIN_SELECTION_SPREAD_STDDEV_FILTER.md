# 0009_COIN_SELECTION_SPREAD_STDDEV_FILTER

## 사용자의 요청

코인 선택 기준에 스프레드 stddev 필터를 추가해줘.
현재 코인 선택이 거래량/변동성 기준으로만 이루어져서, stddev가 지나치게 큰 코인이 선택됨.
stddev가 크면 z_score = (spread - mean) / stddev에서 z가 entry_z(2.0)에 도달하기 사실상 불가능.
예: 이전 세션에서 진입 성공한 AXS(stddev=0.188)는 0.38%p 편차로 z=2.0 도달, 현재 VANA(stddev=4.2)는 8.4%p 편차가 필요.

## 배경

### 현재 코인 선택 알고리즘 (`coin_selector.rs`)

1. 양쪽 거래소 전종목 Ticker 조회
2. 교집합 추출 (공통 코인)
3. 스테이블코인/블랙리스트 제외
4. 24h 거래대금 하위 50% 제거
5. 1h 캔들로 거래량 필터링 (`min_volume_1h_usdt`)
6. **변동성(24h price range) 내림차순 정렬 → 상위 N개 반환**

### 문제

- 6번의 "변동성"은 `(high - low) / low × 100`인 **가격 변동성**이지, **스프레드 변동성**이 아님
- 가격 변동성이 높은 코인은 양 거래소 간 **스프레드 자체의 변동 폭(stddev)**도 크기 때문에 z-score가 안정적으로 높아지지 않음
- z = 2.0 도달에 필요한 spread 편차 = `entry_z × stddev` → stddev가 작을수록 진입 기회 증가

### 목표

코인 선택 시 **워밍업 후 계산된 spread stddev**가 일정 범위를 벗어나는 코인을 필터링하고,
실시간으로 stddev 변화를 감지하여 regime change 시 코인을 교체.

## 구현 플랜

### Phase 1: config에 `max_spread_stddev` 설정 추가

**파일**: `crates/arb-strategy/src/zscore/config.rs`

- `ZScoreConfig`에 `max_spread_stddev: f64` 필드 추가
  - 기본값: `0.5` (stddev > 0.5이면 z=2.0 도달에 1%p 이상 편차 필요)
  - `0.0`이면 필터 비활성화
- `RawZScoreConfig`에 `max_spread_stddev: Option<f64>` 추가
- `validate()`에서 `max_spread_stddev < 0.0` 검증
- `validate()`에서 `!auto_select && max_spread_stddev > 0.0`이면 `warn!("max_spread_stddev는 auto_select=true일 때만 적용됩니다")` 경고
- `strategy.example.toml`에 예시 추가

### Phase 2: 초기 코인 선택 후 워밍업-기반 stddev 필터링

**파일**: `crates/arb-strategy/src/zscore/monitor.rs`

현재 흐름:
```
1. CoinSelector::select(max_coins) → 후보 코인
2. InstrumentCache 초기화 (fetch_instruments)
3. warmup() → SpreadCalculator에 캔들 로드 → stddev 계산됨
4. warmup record 생성
5. OrderBookCache 프리페치
6. WebSocket 연결 → 실시간 모니터링
```

변경 후 흐름:
```
1. CoinSelector::select(max_coins × 2) → 확대 후보
2. 코인별 개별 warmup (실패 코인 스킵) → SpreadCalculator에 캔들 로드 → stddev 계산됨
3. ★ filter_coins_by_stddev() → stddev 필터링 + max_coins개 선택
4. 유효 코인 0개 → StrategyError 반환
5. 제거된 코인 spread_calc.remove_coin() 정리
6. InstrumentCache 초기화 (필터 후 코인만)       ← 이동
7. warmup record 생성 (필터 후 코인만)            ← 이동
8. OrderBookCache 프리페치 (필터 후 코인만)       ← 이동
9. WebSocket 연결 → 실시간 모니터링
```

핵심 변경:
- `CoinSelector::select()` 호출 시 `max_coins * 2`로 확대하여 더 많은 후보 확보
- **확대 warmup을 코인별 `warmup_single_coin_standalone` 개별 호출로 변경**, 실패 시 warn + `remove_coin` + continue (기존 `warmup()`의 `?` 전체 중단 방지)
- `warmup()` 이후 `spread_calc.cached_stats(coin)`에서 stddev 조회
- `max_spread_stddev > 0.0`이면 stddev 초과 코인 제거
- 남은 코인을 stddev 오름차순 정렬하여 상위 `max_coins`개 선택
- **Fallback**: 필터 후 0개이면 stddev 오름차순 `max_coins`개 강제 선택 + `warn!` 로그
- **cached_stats()가 None인 코인** (워밍업 데이터 부족): 제거 (slot 낭비 방지)
- 제거된 코인은 `spread_calc.remove_coin(coin)`으로 정리
- **유효 코인 0개** (fallback 후에도 0개): `StrategyError::Config` 반환
- **InstrumentCache/OB 프리페치/warmup record 생성을 필터 이후로 이동** → REST 호출 절약
- **`auto_select=false` 경로는 기존 `warmup()` 유지**: 수동 지정 코인은 사용자 의도이므로 실패 시 즉시 에러 반환 (`?` 전파). 확대 warmup은 `auto_select=true` 경로에서만 적용.

> **초기화 시간 증가**: `max_coins * 2`로 확대하면 워밍업 REST 호출이 약 2배 증가합니다.
> 코인당 Upbit 캔들(~8페이지) + Bybit 캔들(~2페이지) = ~10 REST 호출.
> `max_coins=5` 기준 10개 코인 = ~100 REST 호출, Upbit rate limiter(8 req/sec) 기준 추가 ~10초 소요.

#### 확대 warmup 패턴

기존 `warmup()`은 코인 목록 전체를 받아 첫 실패에서 `?`로 중단합니다. 확대 선택에서는 코인별 개별 호출로 변경합니다.

```rust
// ★ SpreadCalculator를 빈 상태로 생성 (warmup_single_coin_standalone이 add_coin 호출)
let mut spread_calc_local = SpreadCalculator::new(&[], self.config.window_size);

// CoinSelector가 중복 반환할 수 있으므로 dedup 처리
let expanded_coins: Vec<String> = {
    let mut coins = expanded_coins;
    coins.sort();
    coins.dedup();
    coins
};

// 확대 후보에 대해 코인별 개별 warmup (실패 시 스킵)
for coin in &expanded_coins {
    if let Err(e) = Self::warmup_single_coin_standalone(
        self.upbit.as_ref(), self.bybit.as_ref(),
        &self.config, &self.forex_cache, coin, &mut spread_calc_local,
    ).await {
        warn!(coin = coin.as_str(), error = %e, "워밍업 실패, 해당 코인 스킵");
        spread_calc_local.remove_coin(coin);
    }
}
```

#### 순수 함수 추출 (테스트 용이성)

`SpreadCalculator` 직접 의존을 피하고, 호출 측에서 통계를 미리 수집하여 전달합니다.

```rust
/// 워밍업 후 stddev 기준으로 코인을 필터링합니다.
///
/// # 인자
/// * `coin_stats` - 코인별 (mean, stddev). None이면 워밍업 실패로 제거.
/// * `max_spread_stddev` - stddev 상한 (0.0이면 호출 측에서 스킵하지만, 방어적으로 0.0 전달 시 필터링 없이 stddev 오름차순 max_coins개 반환)
/// * `max_coins` - 최대 코인 수
///
/// # 반환값
/// (유지 코인, 제거 코인)
///
/// # Fallback
/// 필터 후 0개이면 stddev 오름차순으로 max_coins개를 강제 선택합니다.
///
/// # 정렬
/// stddev 오름차순. f64 비교는 `partial_cmp(...).unwrap_or(Ordering::Equal)` 사용.
/// cached_stats=None 코인은 사전 제거되므로 NaN 없음.
///
/// # 방어: max_spread_stddev == 0.0
/// 필터링 없이 stddev 오름차순 max_coins개만 반환 (early return).
fn filter_coins_by_stddev(
    coin_stats: &[(String, Option<(f64, f64)>)],
    max_spread_stddev: f64,
    max_coins: usize,
) -> (Vec<String>, Vec<String>)
```

호출 측 패턴:
```rust
let coin_stats: Vec<(String, Option<(f64, f64)>)> = expanded_coins
    .iter()
    .map(|c| (c.clone(), spread_calc_local.cached_stats(c)))
    .collect();

let (kept, removed) = filter_coins_by_stddev(
    &coin_stats, config.max_spread_stddev, config.max_coins,
);

// 유효 코인 0개 체크 (모든 coin_stats의 Option이 None = 전원 워밍업 실패)
if kept.is_empty() {
    let all_none = coin_stats.iter().all(|(_, s)| s.is_none());
    let msg = if all_none {
        "모든 후보 코인의 워밍업이 실패했습니다 (REST API 장애 가능)"
    } else {
        "워밍업은 성공했지만 모든 코인의 spread stddev가 임계값을 초과합니다"
    };
    return Err(StrategyError::Config(msg.to_string()));
}
```

### Phase 3: 재선택 시에도 stddev 필터 적용

**파일**: `crates/arb-strategy/src/zscore/monitor.rs`

`spawn_reselection()` 내부:

**CoinSelector 확대 선택**:
```rust
// 기존: selector.select(config.max_coins, ...)
// 변경: 2배 확대하여 stddev 필터 + pruning 여유분 확보
let new_candidates = selector.select(config.max_coins * 2, ...).await?;
```

**기존 코인 stddev 체크**:
- `diff_coins()` 호출 **이후**에 spread_calc read → 값 복사 → drop → position_mgr lock (lock order 준수)
- stddev 초과 코인은 별도 `stddev_snapshot` 벡터에 수집 후 `diff.to_remove` / `to_keep_with_position`에 분배
- 교집합 코인(새 후보에도 있지만 stddev 초과)도 포착

```rust
let mut diff = {
    let pm = position_mgr.lock().await;
    diff_coins(&current_coins_snapshot, &new_candidates, &pm)
};

// spread_calc read → 값 복사 → drop (lock order 준수)
if config.max_spread_stddev > 0.0 {
    let stddev_snapshot: Vec<(String, f64)> = {
        let sc = spread_calc.read().await;
        current_coins_snapshot.iter()
            .filter_map(|coin| {
                sc.cached_stats(coin)
                    .filter(|(_, s)| *s > config.max_spread_stddev)
                    .map(|(_, s)| (coin.clone(), s))
            })
            .collect()
    };  // sc drop

    if !stddev_snapshot.is_empty() {
        let pm = position_mgr.lock().await;
        let mut added_count = 0u64;
        for (coin, _stddev) in &stddev_snapshot {
            if pm.has_position(coin) {
                if !diff.to_keep_with_position.contains(coin) {
                    diff.to_keep_with_position.push(coin.clone());
                    added_count += 1;
                }
            } else if !diff.to_remove.contains(coin) {
                diff.to_remove.push(coin.clone());
                added_count += 1;
            }
        }
        drop(pm);
        // "stddev 초과로 실제 분류 변경된 횟수"만 카운트
        counters.lock().unwrap().coin_rejected_spread_stddev_count += added_count;
    }

    // ★ 교집합 코인이 stddev 초과로 to_remove에 추가된 경우,
    // diff.to_add에 같은 코인이 있으면 제거 (제거 후 재추가 방지)
    diff.to_add.retain(|coin| !diff.to_remove.contains(coin));
}
```

**새로 추가되는 코인 stddev 체크** (deadlock 방지: read → drop → write):

```rust
if config.max_spread_stddev > 0.0 {
    let exceeds = {
        let sc = spread_calc.read().await;
        sc.cached_stats(&coin)
            .map(|(_, stddev)| stddev > config.max_spread_stddev)
            .unwrap_or(false)
    };  // sc drop
    if exceeds {
        warn!(coin = coin.as_str(), max = config.max_spread_stddev,
            "재선택 코인 stddev 초과, 건너뜀");
        spread_calc.write().await.remove_coin(&coin);
        counters.lock().unwrap().coin_rejected_spread_stddev_count += 1;
        continue;
    }
}
```

**재선택 후 코인 수 > max_coins pruning**:

워밍업 + stddev 필터 후 `spread_calc.active_coins()` 수가 `max_coins`를 초과할 수 있습니다.
`ReselectionResult`를 전송하기 직전에 초과 코인을 pruning합니다.

```rust
// 최종 코인 수집 전 초과 코인 pruning
let new_coins: Vec<String> = {
    let sc = spread_calc.read().await;
    let mut active: Vec<(String, f64)> = sc.active_coins().iter()
        .filter_map(|coin| {
            sc.cached_stats(coin).map(|(_, stddev)| (coin.to_string(), stddev))
        })
        .collect();
    // stddev 오름차순 정렬
    active.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    active.into_iter().map(|(coin, _)| coin).collect()
};
// sc drop

if new_coins.len() > config.max_coins {
    let excess: Vec<String> = new_coins[config.max_coins..].to_vec();

    // ★ write lock을 한 번에 획득하여 일괄 제거 (lock 경합 최소화)
    {
        let mut sc = spread_calc.write().await;
        for coin in &excess {
            sc.remove_coin(coin);
        }
    }
    {
        let mut data = ob_cache.data.write().await;
        for coin in &excess {
            data.remove_coin(coin);
        }
    }

    // WebSocket 구독 해제 + removed_coins 등록
    for coin in &excess {
        let upbit_market = format!("KRW-{coin}");
        let bybit_market = format!("{coin}USDT");
        upbit.unsubscribe_markets(&[&upbit_market]).await.ok();
        bybit.unsubscribe_markets(&[&bybit_market]).await.ok();
        removed_coins.push(coin.clone());
    }
}

let final_coins: Vec<String> = {
    let sc = spread_calc.read().await;
    sc.active_coins().iter().map(|s| s.to_string()).collect()
};
```

### Phase 4: 런타임 stddev 모니터링 (regime change 감지)

**파일**: `crates/arb-strategy/src/zscore/monitor.rs`, `crates/arb-strategy/src/zscore/spread.rs`

**목적**: 선택 시점에는 stddev가 낮았지만 시장 상황 변화로 갑자기 높아진 코인을 감지하여 교체.

#### 4-1. SpreadCalculator에 단기 윈도우 추가

**파일**: `crates/arb-strategy/src/zscore/spread.rs`

24h rolling stddev(window_size=1440)는 시장 급변에 느리게 반응합니다.
최근 60분 데이터로 계산한 **단기 stddev**를 별도 추적하여 regime change 조기 감지에 사용합니다.

```rust
/// 단기 regime change 감지용 윈도우 크기 (60분).
const SHORT_WINDOW_SIZE: usize = 60;
```

SpreadCalculator에 추가:
- 코인별 단기 스프레드 윈도우: `short_spread_windows: HashMap<String, CandleWindow>` (`CandleWindow`는 기존 장기 윈도우에서 사용 중인 동일 타입)
- 코인별 단기 통계: `short_spread_stats: HashMap<String, WelfordStats>` (**Welford's online algorithm** 사용 — 소규모 윈도우에서의 catastrophic cancellation 방지)
  - `add_coin` 시 `SHORT_WINDOW_SIZE` 크기 `CandleWindow` 생성 + `WelfordStats::new()` 생성 (양쪽 동시 초기화)
  - `update` 시 spread 값을 장기/단기 양쪽 윈도우 + stats에 `push_to_window_and_stats` 헬퍼로 push
  - `update`에서 단기 윈도우가 없으면(`short_spread_windows.get_mut(coin)` = None): 에러 반환하지 않고 **skip**. `add_coin`이 양쪽을 동시 초기화하므로 정상 경로에서는 발생하지 않으나, 방어적 처리.
  - `remove_coin` 시 함께 정리

**update 시 장기/단기 push 헬퍼** (push 순서 실수 방지):

```rust
/// 윈도우와 통계에 값을 동시에 push합니다.
/// 순서: window.push(value) → stats 갱신
/// IncrementalStats: push(value, popped) O(1)
/// WelfordStats: pop 발생 시 rebuild(window.data()) O(n)
fn push_to_window_and_stats<S: StatsAccumulator>(
    window: &mut CandleWindow,
    stats: &mut S,
    value: f64,
) {
    let had_pop = window.data().len() >= window.window_size();
    let popped = if had_pop {
        Some(window.data()[0])
    } else {
        None
    };
    // IncrementalStats: push로 O(1) 갱신 (pop 값 차감)
    stats.push(value, popped);
    window.push(value);
    // WelfordStats: pop 발생 시 전체 재계산 (rebuild가 no-op이 아닌 경우)
    if had_pop {
        // VecDeque → slice: window.data().make_contiguous() 또는 Vec 변환
        let data: Vec<f64> = window.data().iter().copied().collect();
        stats.rebuild(&data);
    }
}
```

장기(IncrementalStats)와 단기(WelfordStats) 양쪽에서 동일 헬퍼를 재사용합니다.
`StatsAccumulator` trait은 `push`, `mean`, `stddev`, `rebuild` 메서드를 정의합니다.
`capacity` 파라미터 대신 `window.window_size()`를 직접 사용하여 불일치를 방지합니다.

> **IncrementalStats의 rebuild**: no-op (`fn rebuild(&mut self, _data: &[f64]) {}`)으로 구현.
> IncrementalStats는 `push`에서 pop 값 차감이 O(1)으로 처리되므로 rebuild 불필요.
> WelfordStats만 `rebuild`에서 실제 재계산을 수행합니다.

```rust
/// 단기 윈도우(60분)의 (mean, stddev)를 반환합니다.
/// 윈도우 데이터가 `SHORT_WINDOW_SIZE`(60)개 미만이면 `None` 반환 (`is_ready()` 기준).
///
/// **의도된 동작**: 세션 시작 후 최초 60분간은 항상 `None`을 반환하므로,
/// regime change 감지는 장기 stddev로 fallback됩니다.
/// 이 60분 blind spot은 허용 범위입니다 (장기 stddev가 이미 초기 필터를 통과했으므로).
pub fn cached_short_stats(&self, coin: &str) -> Option<(f64, f64)>
```

#### 4-2. finalize_and_process에 regime change 감지

**구현 위치**: `finalize_and_process()` — **MinuteRecord 생성 이후**에 배치 (early return으로 MinuteRecord 누락 방지)

**감지 기준**: 단기 stddev(`cached_short_stats`) 사용. 단기 윈도우가 아직 충분하지 않으면 장기 stddev로 fallback.

```rust
/// Regime change 감지 배수.
/// max_spread_stddev * 이 값을 초과하면 코인 제거 대상.
/// 1.5x: max_spread_stddev=0.5 기준 stddev > 0.75이면 감지.
/// z=2.0 도달에 1.5%p 편차 필요 → 사실상 진입 불가능 수준.
const REGIME_CHANGE_MULTIPLIER: f64 = 1.5;
```

```rust
/// finalize_and_process의 regime change 감지 결과.
struct RegimeChangeResult {
    /// 포지션 없어서 즉시 제거할 코인.
    immediate_remove: Vec<String>,
    /// 포지션 있어서 dropped_at에 등록할 코인.
    dropped_coins: Vec<String>,
}
```

`finalize_and_process` 반환 타입: `Result<Option<RegimeChangeResult>, StrategyError>`

> **호출 사이트**: `finalize_and_process`는 두 경로에서 호출됩니다:
> 1. `minute_timer.tick()` 분기 → **여기서만** `RegimeChangeResult`를 처리
> 2. `update_candle_and_spread` 내부 (이벤트 수신 경로에서 분 전환 감지 시) → `RegimeChangeResult`를 **무시** (`Ok(_) => {}`)
>
> `update_candle_and_spread` 경로에서 regime change를 무시해도, `minute_timer`가 매분 체크하므로
> 최대 1분 지연으로 감지됩니다. 반환 타입 변경(`Result<Option<RegimeChangeResult>, StrategyError>`)은
> 양쪽 호출 사이트에 모두 반영해야 하지만, 처리 로직은 `minute_timer` 경로에서만 수행합니다.

```rust
// finalize_and_process 내부
// ★ 위치: 코인별 for 루프 종료 후, candle_builder.start_new_minute() 직전
// MinuteRecord 생성 및 session_writer 기록이 모두 완료된 상태
if config.max_spread_stddev > 0.0 {
    let regime_threshold = config.max_spread_stddev * REGIME_CHANGE_MULTIPLIER;

    // spread_calc read → 값 복사 → drop (lock order 준수)
    let stddev_snapshot: Vec<(String, f64)> = {
        let sc = spread_calc.read().await;
        current_coins.iter()
            .filter_map(|coin| {
                // 단기 stddev 우선, 없으면 장기 stddev fallback
                let stddev = sc.cached_short_stats(coin)
                    .or_else(|| sc.cached_stats(coin))
                    .map(|(_, s)| s);
                stddev
                    .filter(|s| *s > regime_threshold)
                    .map(|s| (coin.clone(), s))
            })
            .collect()
    };  // sc drop

    if !stddev_snapshot.is_empty() {
        let mut immediate_remove = Vec::new();
        let mut dropped_coins = Vec::new();

        let pm = position_mgr.lock().await;
        for (coin, stddev) in &stddev_snapshot {
            warn!(
                coin = coin.as_str(),
                stddev = stddev,
                threshold = regime_threshold,
                "regime change 감지: stddev 급등"
            );
            if pm.has_position(coin) {
                // dropped_at 등록, exit_z 청산은 계속 평가됨
                dropped_coins.push(coin.clone());
            } else {
                immediate_remove.push(coin.clone());
            }
        }
        drop(pm);

        counters.lock().unwrap().coin_rejected_spread_stddev_count +=
            stddev_snapshot.len() as u64;

        // ★ return 하지 않고 변수에 저장 (start_new_minute 건너뜀 방지)
        regime_result = Some(RegimeChangeResult {
            immediate_remove,
            dropped_coins,
        });
    }
}

// ★ candle_builder.start_new_minute()가 반드시 실행되도록 regime 결과를 변수에 저장
// return Ok(Some(...))을 사용하면 start_new_minute을 건너뛰어
// 다음 틱에서 is_new_minute이 다시 true → 중복 finalize 호출 버그 발생
candle_builder.start_new_minute(new_minute_ts);
Ok(regime_result)
```

#### 4-3. run() 루프에서 결과 처리

`minute_timer.tick()` 분기의 호출 패턴을 `if let Err` → `if` + `match`로 변경합니다.

run() 루프에 cooldown 변수 추가:
```rust
// run() 루프 시작 부분 (select! 이전)
// regime change 재선택 후 무한 루프 방지용 cooldown (지수적 백오프)
let mut regime_cooldown_until: Option<chrono::DateTime<Utc>> = None;
let mut consecutive_regime_changes: u32 = 0;
const MAX_COOLDOWN_MIN: u64 = 60; // 최대 1시간
```

```rust
_ = minute_timer.tick() => {
    let now = Utc::now();
    if candle_builder.is_new_minute(now) {
        match Self::finalize_and_process(
            &self.config, &mut candle_builder, &spread_calc,
            &position_mgr, &trades, now, &current_coins,
            &self.forex_cache, &session_writer, &mut minute_records,
            &instrument_cache,
        ).await {
            Ok(Some(regime)) => {
                // ★ Cooldown 가드: 재선택 직후에는 regime change를 무시
                // (고변동성 시장에서 무한 재선택 루프 방지)
                let in_cooldown = regime_cooldown_until
                    .map(|until| Utc::now() < until)
                    .unwrap_or(false);
                if in_cooldown {
                    debug!("regime change 감지되었으나 cooldown 중, 무시");
                    counters.lock().unwrap()
                        .regime_change_suppressed_by_cooldown_count += 1;
                } else {
                    counters.lock().unwrap().regime_change_detected_count += 1;
                    // immediate_remove 처리
                    for coin in &regime.immediate_remove {
                        {
                            let mut sc = spread_calc.write().await;
                            sc.remove_coin(coin);
                        }
                        // ob_cache 정리
                        {
                            let mut data = ob_cache.data.write().await;
                            data.remove_coin(coin);
                        }
                        {
                            let mut flags = ob_cache.computing.inner.lock().unwrap();
                            flags.retain(|k, _| k.1 != coin);
                        }
                        // WebSocket 구독 해제
                        let upbit_market = format!("KRW-{coin}");
                        let bybit_market = format!("{coin}USDT");
                        self.upbit.unsubscribe_markets(&[&upbit_market]).await.ok();
                        self.bybit.unsubscribe_markets(&[&bybit_market]).await.ok();
                        current_coins.retain(|c| c != coin);
                        info!(coin = coin.as_str(), "regime change로 코인 즉시 제거");
                    }
                    // dropped_coins: dropped_at에 등록
                    for coin in &regime.dropped_coins {
                        dropped_at.entry(coin.clone()).or_insert(Utc::now());
                    }
                    // 코인 수 부족 시 즉시 재선택 트리거
                    if current_coins.len() < self.config.max_coins
                        && self.config.auto_select && !reselecting
                    {
                        reselecting = true;
                        // ★ 지수적 백오프 Cooldown
                        consecutive_regime_changes += 1;
                        let backoff_min = (self.config.reselect_interval_min as u64)
                            .saturating_mul(
                                1u64 << (consecutive_regime_changes - 1).min(6),
                            )
                            .min(MAX_COOLDOWN_MIN);
                        regime_cooldown_until = Some(Utc::now()
                            + chrono::Duration::minutes(backoff_min as i64));
                        info!(
                            consecutive = consecutive_regime_changes,
                            cooldown_min = backoff_min,
                            "regime change → 즉시 재선택 (지수적 백오프 cooldown)"
                        );
                        Self::spawn_reselection(...);
                    }
                } // else (not in cooldown)
            }
            Ok(None) => {
                // regime change 없이 정상 운영 → consecutive 카운터 리셋
                if consecutive_regime_changes > 0 {
                    let cooldown_expired = regime_cooldown_until
                        .map(|until| Utc::now() >= until)
                        .unwrap_or(true);
                    if cooldown_expired {
                        consecutive_regime_changes = 0;
                        regime_cooldown_until = None;
                    }
                }
            }
            Err(e) => { warn!(error = %e, "finalize_and_process 실패"); }
        }
    }

    // ★ TTL 만료 포지션 체크: is_new_minute 블록 **바깥**에서 매분 실행 (기존 동작 유지)
    // 기존 코드에서 TTL 체크는 is_new_minute과 무관하게 매 분 타이머마다 실행됨.
    // is_new_minute 안으로 옮기면 첫 번째 분(current_minute == None) 동안 스킵됨.
    if let Err(e) = Self::check_ttl_positions(...).await {
        warn!(error = %e, "check_ttl_positions 실패");
    }
}
```

> **Race condition 참고**: regime change로 코인 제거 시, 해당 코인에 대한 `spawned_check_tick_signal` task가
> 실행 중일 수 있습니다. task 완료 시 ob_cache에 다시 insert될 수 있으나, 해당 코인은 `current_coins`에서
> 제거되어 시그널 평가에 사용되지 않으므로 기능적 영향은 없습니다 (메모리만 약간 낭비).
> `ComputingFlags::remove_coin`을 먼저 호출하여 computing flag를 정리합니다.
>
> **Instrument 캐시**: pruning/제거 시 `InstrumentCache`에서는 정리하지 않습니다.
> `HashMap<String, InstrumentInfo>`에 이전 코인 데이터가 남아도 메모리 외 부작용 없음.
>
> **ob_cache 정리**: `ObCacheData`에 `remove_coin(&str)` 메서드 추가 필요 (upbit/bybit 양쪽 캐시에서 해당 코인 제거).
> `ComputingFlags`에도 `remove_coin(&str)` 메서드 추가 (양 exchange의 computing flag 정리).
>
> ```rust
> // ObCacheData
> pub fn remove_coin(&mut self, coin: &str) {
>     self.upbit.remove(coin);
>     self.bybit.remove(coin);
> }
>
> // ComputingFlags — inner: std::sync::Mutex<HashMap<(Exchange, String), bool>>
> pub fn remove_coin(&self, coin: &str) {
>     let mut flags = self.inner.lock().unwrap();
>     flags.retain(|k, _| k.1 != coin);
> }
> ```

### Phase 5: 로깅 및 모니터링

- 초기 선택 시: `info!` 로그로 stddev 필터 결과 (제거된 코인 + 각 stddev)
- 재선택 시: stddev 초과 교체 `info!` 로그
- 런타임 감지: `warn!` 로그 (regime change 감지)
- `MonitoringCounters`에 3개 카운터 추가:
  - `coin_rejected_spread_stddev_count: u64` — stddev 초과로 실제 분류 변경된 횟수
  - `regime_change_detected_count: u64` — regime change 감지 횟수 (cooldown 억제 제외)
  - `regime_change_suppressed_by_cooldown_count: u64` — cooldown에 의해 억제된 횟수
  - 후자 2개는 cooldown 파라미터 튜닝의 근거 데이터로 활용
- `SessionSummary` + `to_text()` 출력에 3개 카운터 모두 반영

### Phase 6: 테스트 + 빌드 검증

- `config.rs`: `max_spread_stddev` 파싱/검증 테스트
- `spread.rs`: `cached_short_stats()` 단기 윈도우 테스트
- `monitor.rs`: `filter_coins_by_stddev()` 순수 함수 테스트
  - 모든 코인 통과 → 전체 유지
  - 일부 초과 → 초과 제거 + stddev 오름차순
  - 전체 초과 → fallback (강제 선택 + 경고)
  - cached_stats() = None → 제거
- `cargo test --workspace` + `cargo clippy` 통과

## 데이터 구조 변경

### ZScoreConfig 추가 필드

```rust
/// 코인 선택 시 최대 스프레드 stddev (기본값: 0.5).
/// 워밍업 후 계산된 stddev가 이 값을 초과하는 코인은 자동 선택에서 제외.
/// 0.0이면 필터 비활성화.
///
/// 이 값은 entry_z_threshold, exit_z_threshold, 수수료율에 의존합니다:
/// - 진입 시 조건부 margin = (entry_z - exit_z) * stddev - total_fee
/// - entry_z=2.0, exit_z=0.5, fee=0.35%p 기준 stddev=0.5이면 margin ≈ 0.40%p
pub max_spread_stddev: f64,
```

### strategy.example.toml 추가

`auto_select` 블록 바로 아래에 배치 (이 필드는 `auto_select=true`일 때만 의미 있으므로):

```toml
# 코인 선택 시 최대 스프레드 stddev (기본값: 0.5)
# stddev가 이 값을 초과하면 z-score 기반 진입이 어려워 자동 제외
# 진입 시 조건부 margin = (entry_z - exit_z) * stddev - total_fee
# entry_z=2.0, exit_z=0.5, fee ~0.35%p 기준:
#   stddev=0.5 → margin ≈ 0.40%p (양호)
#   stddev=1.0 → margin ≈ 1.15%p (도달 확률 낮음)
# 0.0이면 필터 비활성화
max_spread_stddev = 0.5
```

### SpreadCalculator 추가

```rust
/// 단기 regime change 감지용 윈도우 크기 (60분).
const SHORT_WINDOW_SIZE: usize = 60;

// 코인별 단기 스프레드 윈도우 (CandleWindow: 장기 윈도우와 동일 타입)
short_spread_windows: HashMap<String, CandleWindow>,
// 코인별 단기 통계 (Welford's online algorithm — catastrophic cancellation 방지)
short_spread_stats: HashMap<String, WelfordStats>,

/// 단기 윈도우(60분)의 (mean, stddev)를 반환합니다.
pub fn cached_short_stats(&self, coin: &str) -> Option<(f64, f64)>
```

### MonitoringCounters 추가 필드

```rust
/// stddev 필터로 실제 분류 변경된 코인 수 (초기 선택 + 재선택 + 런타임).
pub coin_rejected_spread_stddev_count: u64,
/// regime change 감지 횟수 (cooldown으로 무시된 것 제외).
pub regime_change_detected_count: u64,
/// cooldown에 의해 억제된 regime change 횟수.
pub regime_change_suppressed_by_cooldown_count: u64,
```

### WelfordStats (신규, `spread.rs`)

```rust
/// Welford's online algorithm 기반 rolling statistics.
/// IncrementalStats의 naive two-pass (sum_sq - sum^2) 방식 대비
/// catastrophic cancellation에 면역이며, 소규모 윈도우(60개)에서 특히 안정적.
///
/// StatsAccumulator trait을 구현하여 push_to_window_and_stats 헬퍼와 호환.
struct WelfordStats {
    count: usize,
    mean: f64,
    m2: f64,  // sum of (x - mean)^2
}

impl WelfordStats {
    fn new() -> Self { ... }
    fn push(&mut self, value: f64, popped: Option<f64>) { ... }
    fn mean(&self) -> f64 { self.mean }
    /// Population stddev (분모=n). IncrementalStats와 동일 규약.
    fn stddev(&self) -> f64 {
        if self.count < 2 { 0.0 }
        else { (self.m2 / self.count as f64).sqrt() }  // ★ /count (population), NOT /(count-1) (sample)
    }
    /// 윈도우 전체 데이터로 재계산. pop 발생 시 호출.
    fn rebuild(&mut self, data: &[f64]) {
        self.count = 0;
        self.mean = 0.0;
        self.m2 = 0.0;
        for &v in data {
            self.count += 1;
            let delta = v - self.mean;
            self.mean += delta / self.count as f64;
            let delta2 = v - self.mean;
            self.m2 += delta * delta2;
        }
    }
}
```

> **pop 처리**: Welford's algorithm은 순방향 추가에 최적화되어 있으며, 윈도우에서 pop된 값의 제거가 필요합니다.
> `push_to_window_and_stats` 헬퍼에서 pop 발생 시 `stats.rebuild(window.data())`를 호출하여
> `CandleWindow` 전체 데이터로 재계산합니다 (O(n), n=60). n=60이므로 분당 1회 재계산의 성능 부담은 무시할 수 있습니다.
> `IncrementalStats`의 `rebuild()`는 no-op이므로 장기 윈도우에서는 오버헤드 없음.
> 대안으로 reverse Welford update를 구현할 수 있지만, 복잡도 대비 이점이 적습니다.

### StatsAccumulator trait (신규, `spread.rs`)

```rust
/// IncrementalStats / WelfordStats 공통 인터페이스.
/// push_to_window_and_stats 헬퍼에서 사용.
///
/// **stddev 규약**: 모든 구현체는 **population stddev** (분모=n)를 반환합니다.
/// IncrementalStats와 WelfordStats 사이의 비교 일관성을 위해 필수.
trait StatsAccumulator {
    fn push(&mut self, value: f64, popped: Option<f64>);
    fn mean(&self) -> f64;
    /// Population stddev (분모=n). Sample stddev (분모=n-1)가 아님에 주의.
    fn stddev(&self) -> f64;
    /// 윈도우 전체 데이터로 통계를 재계산합니다.
    /// WelfordStats: pop 발생 시 전체 재계산 (O(n), n=60).
    /// IncrementalStats: no-op (기존 push/pop 방식으로 O(1) 처리).
    fn rebuild(&mut self, data: &[f64]);
}
```

### RegimeChangeResult (신규, `monitor.rs`)

```rust
/// finalize_and_process의 regime change 감지 결과.
/// #[must_use] 미적용 — update_candle_and_spread 경로에서 의도적으로 무시하므로.
struct RegimeChangeResult {
    /// 포지션 없어서 즉시 제거할 코인.
    immediate_remove: Vec<String>,
    /// 포지션 있어서 dropped_at에 등록할 코인.
    dropped_coins: Vec<String>,
}
```

### 상수

```rust
// monitor.rs — regime check 로직에서 사용
/// Regime change 감지 배수.
/// max_spread_stddev * 이 값을 초과하면 코인 제거 대상.
/// 근거: max_spread_stddev=0.5 기준 0.75 → z=2.0 도달에 1.5%p 편차 필요 → 사실상 진입 불가.
const REGIME_CHANGE_MULTIPLIER: f64 = 1.5;

// spread.rs — SpreadCalculator 내부에서만 사용
/// 단기 stddev 윈도우 크기 (분).
const SHORT_WINDOW_SIZE: usize = 60;
```

## 근거: 기본값 `0.5`

| stddev | z=2.0 필요 편차 | 조건부 margin* | 비고 |
|--------|----------------|---------------|------|
| 0.1    | 0.2%p          | -0.20%p       | 수수료 미커버 (손실) |
| 0.2    | 0.4%p          | -0.05%p       | AXS(0.188) 진입 성공, 경계 |
| 0.3    | 0.6%p          | +0.10%p       | BERA(0.255) 진입 성공 |
| **0.5**| **1.0%p**      | **+0.40%p**   | **기본값 (sweet spot)** |
| 1.0    | 2.0%p          | +1.15%p       | 도달 확률 낮음 |
| 2.0    | 4.0%p          | +2.65%p       | 사실상 불가 |

\* 조건부 margin = `(entry_z - exit_z) × stddev - total_fee_pct`.
산식: entry_z=2.0, exit_z=0.5, total_fee_pct = (upbit_fee + bybit_fee) × 2 × 100 = (0.0005 + 0.00055) × 2 × 100 = **0.21%p** (config 기본값 기준).
예: stddev=0.5 → margin = (2.0 - 0.5) × 0.5 - 0.21 = 0.75 - 0.21 = **0.54%p**. 표의 0.40%p는 보수적 추정(수수료 ~0.35%p 가정)임.
이것은 이상적 시나리오의 **하한 추정치**이며, 실제로는:
- z-score가 exit_z를 undershoot하여 더 유리하게 청산될 수 있음 (상한 방향)
- mean이 drift하여 불리하게 될 수 있음 (하한 방향)

**진입이 발생했다는 조건부** profit margin이므로 실제 기대 수익(E[PnL])과는 다릅니다.
실제 E[PnL]은 진입 확률, 스프레드 분포의 비정규성(fat tail), non-stationarity에 의존합니다.

> **고stddev 코인을 제외하는 근거**:
> (a) 고 stddev 코인은 non-stationary하여 z-score의 mean-reversion 가정이 약화됨
> (b) 고 stddev 코인은 슬리피지/실행 비용이 이론치보다 높아 조건부 margin이 추가 감소
> (c) z=2.0 도달 확률은 정규분포 가정 시 약 2.28%이지만, stddev가 클수록 분포 신뢰성이 떨어짐

수수료 합산 ~0.35%p, entry_z=2.0, exit_z=0.5 기준:
- `stddev = 0.5` → 조건부 margin = 1.5 × 0.5 - 0.35 = **0.40%p** (양호)
- `stddev = 1.0` → 조건부 margin = 1.5 × 1.0 - 0.35 = 1.15%p (이론적으로 크지만 도달 확률 낮음)

## 향후 과제 (이 스펙 범위 밖)

- **코인 선택 정렬 기준 개선**: 가격 변동성 대신 예상 수익성 지표 기반 정렬. 별도 스펙으로 분리.
- **Hysteresis 적용**: 진입/제거 기준에 비대칭 임계값을 두어 경계 thrashing 방지. 운영 데이터 축적 후 판단.
- **REGIME_CHANGE_MULTIPLIER config화**: 현재 상수로 하드코딩, 운영 경험 후 config 필드로 전환 검토.
- **Regime change 포지션 별도 청산 로직**: regime change는 mean-reversion 가정 붕괴를 의미하므로, 포지션 보유 리스크가 증가함. stddev 급등 시 z-score 자연 하락(denominator 팽창)으로 false exit signal이 발생할 수 있음. "TTL까지 기다리기" 대신 market exit, tighter exit threshold, 또는 PnL 기반 hard stop 적용 검토.
- **Regime change 감지 기준: ratio 기반 판정**: 현재 `short_stddev > max_spread_stddev * 1.5` 절대 임계값 비교 대신, `short_stddev / long_stddev > 2.0` 같은 비율 기반 판정이 false positive에 더 강건함. 단기 stddev(n=60)의 고유 sampling variance(상대 오차 ~9%)를 고려하면 절대 임계값은 false positive rate가 높을 수 있음.
- **IncrementalStats → Welford 장기 전환**: 이 스펙에서 단기 윈도우에 WelfordStats를 적용. 장기 윈도우(IncrementalStats)는 n=1440으로 안정적이므로 현행 유지, 향후 일관성을 위해 전환 검토.
- **워밍업 vs 런타임 stddev 산출 방식 차이**: 워밍업(REST 분봉 close)과 런타임(WS 틱 기반 분봉)의 spread 계산 방식이 미세하게 달라, 신규 코인이 인위적으로 낮은 stddev를 보일 수 있는 bias 존재. 운영 데이터에서 모니터링 필요.
- **확대 계수 동적 조절**: `max_coins * 2` 하드코딩 대신, 극단 시장(전종목 고변동성)에서 fallback 반복 발동 시 확대 비율 자동 증가, 또는 config `expand_ratio` 필드 검토.
- **Regime change confirmation delay**: 변동성 스파이크는 차익거래의 수익 기회이므로, 단기 stddev 급등 시 과도한 필터링은 수익을 감소시킴. GARCH 클러스터링에 의한 일시적 스파이크(30분 내 정상화)와 구조적 regime change를 구분하기 위해, N분 연속 초과 확인 로직 검토. 현재는 지수적 백오프 cooldown으로 재선택 루프를 방지하지만, 최초 오탐은 방지하지 못함.
- **재선택 워밍업 시 write lock 장기 보유**: `warmup_single_coin_standalone`이 REST 호출을 포함하여 spread_calc write lock을 장시간 보유할 수 있음. 별도 임시 SpreadCalculator에서 워밍업 후 결과 merge 패턴 검토.

## 체크리스트

### Phase 1: config
- [ ] `ZScoreConfig.max_spread_stddev: f64` 필드 추가 (기본값 0.5)
- [ ] `RawZScoreConfig.max_spread_stddev: Option<f64>` 추가
- [ ] `validate()`에서 `max_spread_stddev < 0.0` 검증
- [ ] `validate()`에서 `!auto_select && max_spread_stddev > 0.0` 경고 로그
- [ ] `from_raw()` 변환 로직 추가
- [ ] `strategy.example.toml` 업데이트 (entry_z/fee 의존 관계 주석 포함)
- [ ] config 테스트 추가/수정

### Phase 2: 초기 선택 후 stddev 필터
- [ ] `filter_coins_by_stddev(coin_stats, max_spread_stddev, max_coins)` 순수 함수 추출
- [ ] `max_spread_stddev == 0.0` 방어: 필터링 없이 stddev 오름차순 max_coins개 반환 (early return)
- [ ] f64 정렬: `partial_cmp(...).unwrap_or(Ordering::Equal)` NaN 안전장치
- [ ] `monitor.rs run()`: `CoinSelector::select()` 시 `max_coins * 2` 확대 호출
- [ ] `expanded_coins` 중복 방어: `sort()` + `dedup()` (CoinSelector 중복 반환 대비)
- [ ] 확대 warmup을 `warmup_single_coin_standalone` 코인별 개별 호출 (실패 시 warn + remove + continue)
- [ ] `warmup()` 이후 `filter_coins_by_stddev()` 호출
- [ ] Fallback: 필터 후 0개 → stddev 오름차순 max_coins개 강제 선택 + `warn!`
- [ ] 유효 코인 0개 (fallback 후에도) → `StrategyError::Config` 반환 (에러 메시지 분기: 모두 None=워밍업 실패, 일부 유효=stddev 전초과)
- [ ] `SpreadCalculator::new(&[], window_size)` 빈 상태 생성 (warmup_single_coin_standalone이 add_coin 호출)
- [ ] `auto_select=false` 경로는 기존 `warmup()` 유지 (실패 시 `?` 전파)
- [ ] cached_stats() = None인 코인 제거
- [ ] 제거된 코인 `spread_calc.remove_coin()` 정리
- [ ] InstrumentCache 초기화를 필터 이후로 이동
- [ ] warmup record 생성을 필터 이후로 이동
- [ ] OB 프리페치를 필터 이후로 이동

### Phase 3: 재선택 시 stddev 필터
- [ ] `spawn_reselection()`: CoinSelector 확대 선택 (`max_coins * 2`)
- [ ] 기존 코인 stddev 체크: spread_calc read → 값 복사 → drop → position_mgr lock (lock order 준수)
- [ ] 별도 `stddev_snapshot` 벡터로 수집 후 `diff.to_remove` / `to_keep_with_position`에 분배
- [ ] 교집합 코인(새 후보에도 있지만 stddev 초과)도 포착
- [ ] 교집합 코인이 `to_remove`에 추가된 경우 `to_add`에서 제거 (`to_add.retain(...)` — 제거 후 재추가 방지)
- [ ] 새 코인 워밍업 후 즉시 stddev 체크 → read lock → drop → write lock (deadlock 방지)
- [ ] 워밍업 후 전체 코인 수 > max_coins이면 stddev 오름차순 pruning + ob_cache 일괄 정리 + WS 해제
- [ ] pruning된 초과 코인을 `removed_coins`에 추가 (run() 루프에서 `dropped_at` 정리용)
- [ ] 카운터: 실제 분류 변경된 경우에만 카운트
- [ ] 재선택 diff 로그에 stddev 초과 정보 포함

### Phase 4: 런타임 stddev 모니터링
- [ ] `WelfordStats` 구조체 추가 (`spread.rs`): `rebuild()` 메서드 포함, `stddev()`는 population stddev (`m2/count`)
- [ ] `StatsAccumulator` trait 추가: `push`, `mean`, `stddev`, `rebuild` 메서드
- [ ] `IncrementalStats`에 `StatsAccumulator` 구현: `rebuild`는 no-op
- [ ] `push_to_window_and_stats` 헬퍼: `window.window_size()` 사용 (capacity 파라미터 없음), pop 시 `stats.rebuild(window.data())` 호출
- [ ] SpreadCalculator에 단기 윈도우 추가 (`short_spread_windows: HashMap<String, CandleWindow>`, `short_spread_stats: HashMap<String, WelfordStats>`, `SHORT_WINDOW_SIZE=60`)
- [ ] `add_coin`, `update`, `remove_coin`에서 단기 윈도우 함께 관리 (`update`에서 단기 윈도우 미존재 시 skip)
- [ ] `cached_short_stats()` 메서드 추가
- [ ] `REGIME_CHANGE_MULTIPLIER` 상수 정의 (1.5)
- [ ] `RegimeChangeResult` 구조체 추가
- [ ] `finalize_and_process()` 반환 타입 확장: `Result<Option<RegimeChangeResult>, StrategyError>`
- [ ] regime check 위치: 코인별 for 루프 종료 후, `candle_builder.start_new_minute()` 직전 (MinuteRecord 누락 방지)
- [ ] 감지 기준: `cached_short_stats()` 우선, 없으면 `cached_stats()` fallback
- [ ] Lock order 준수: spread_calc read → 값 복사 → drop → position_mgr lock
- [ ] 포지션 있는 코인: `dropped_coins` (dropped_at 등록, exit_z 청산 계속 평가)
- [ ] 포지션 없는 코인: `immediate_remove`
- [ ] `run()` 루프: `if let Err` → `if` + `match` 패턴으로 변경
- [ ] `ObCacheData::remove_coin()` 메서드 추가 (upbit/bybit 양쪽 캐시에서 코인 제거)
- [ ] `ComputingFlags::remove_coin()` 메서드 추가 (양 exchange flag 정리)
- [ ] `run()` 루프: immediate_remove 시 spread_calc.remove_coin + ob_cache 정리 + WS 해제 + current_coins 갱신
- [ ] `run()` 루프: dropped_coins → dropped_at에 등록
- [ ] 코인 수 부족 + `!reselecting` + `auto_select` 시 즉시 재선택 트리거
- [ ] `regime_cooldown_until` + `consecutive_regime_changes` + `MAX_COOLDOWN_MIN` 변수 추가
- [ ] 지수적 백오프: `reselect_interval_min * 2^(consecutive-1)`, 상한 `MAX_COOLDOWN_MIN`(60분)
- [ ] cooldown 만료 + regime change 없음 시 `consecutive_regime_changes` 리셋
- [ ] `finalize_and_process`에서 `return` 대신 변수 저장 → `start_new_minute` 실행 보장
- [ ] `update_candle_and_spread` 내부의 finalize 호출도 반환 타입 변경 반영 (`RegimeChangeResult` 무시: `Ok(_) => {}`)
- [ ] TTL 체크(`check_ttl_positions`)는 `is_new_minute` 블록 **바깥**에서 매분 실행 (기존 동작 유지)

### Phase 5: 로깅 + MonitoringCounters
- [ ] 초기 선택 stddev 필터 결과 `info!` 로그
- [ ] 재선택 stddev 초과 교체 `info!` 로그
- [ ] 런타임 regime change `warn!` 로그
- [ ] `MonitoringCounters` 3개 카운터 추가: `coin_rejected_spread_stddev_count`, `regime_change_detected_count`, `regime_change_suppressed_by_cooldown_count`
- [ ] `SessionSummary` + `to_text()` 출력에 3개 카운터 모두 반영

### Phase 6: 테스트 + 빌드
- [ ] config 파싱/검증 테스트
- [ ] `cached_short_stats()` 단기 윈도우 테스트
  - [ ] 장기 ready + 단기 not ready (60분 미만) → `None` 반환, 장기 fallback 확인
  - [ ] 단기/장기 stddev가 크게 다른 경우 (regime change 시뮬레이션)
  - [ ] `remove_coin` 후 단기 윈도우도 정리 확인
  - [ ] `add_coin` 후 단기 윈도우 빈 상태 생성 확인
- [ ] `filter_coins_by_stddev()` 단위 테스트 (7개 시나리오)
  - [ ] 모든 코인 통과 → 전체 유지
  - [ ] 일부 초과 → 초과 제거 + stddev 오름차순
  - [ ] 전체 초과 → fallback (강제 선택 + 경고)
  - [ ] cached_stats() = None → 제거
  - [ ] max_coins=1 → 1개만 반환
  - [ ] max_spread_stddev=0.0 → 필터링 없이 stddev 오름차순 max_coins개 반환
  - [ ] 동일 stddev 코인 정렬 안정성
- [ ] `RegimeChangeResult` 통합 테스트: stddev 초과 코인이 있을 때 `Some` 반환 확인
- [ ] `MonitoringCounters` 카운터 정확성 테스트 (3개 모두: 중복 미카운트, cooldown 억제 카운트)
- [ ] `WelfordStats` 단위 테스트: push/pop 정확성, IncrementalStats 대비 결과 비교 (population stddev 일치)
- [ ] `WelfordStats::rebuild()` 테스트: pop 발생 후 rebuild 결과가 처음부터 push한 결과와 일치
- [ ] `WelfordStats.count` == `CandleWindow.data().len()` 동기 보장 테스트
- [ ] `cargo test --workspace` 통과
- [ ] `cargo clippy --workspace` 경고 0
- [ ] `cargo fmt` 통과
