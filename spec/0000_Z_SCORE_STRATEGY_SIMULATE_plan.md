# 0000_Z_SCORE_STRATEGY_SIMULATE — 병렬 구현 플랜

## 개요

`spec/0000_Z_SCORE_STRATEGY_SIMULATE.md` 스펙을 Claude Agent Team(`arb_poc_team`)으로 병렬 구현하기 위한 작업 분배 및 실행 계획.

---

## Phase 의존성 그래프

```
Phase 0 arb-exchange (trait 정의)     ← 모든 것의 시작점, blocking
         │
         ├─→ [R1] trait 설계 리뷰 (code-reviewer)
         │
         ├─→ Phase 0 Upbit SDK ─────────────→ Phase 5a Upbit MarketStream
         ├─→ Phase 0 Bybit SDK ─────────────→ Phase 5a Bybit MarketStream
         ├─→ Phase 0 arb-strategy scaffold
         │         │
         │         ├─→ Phase 1 (통계엔진)
         │         │         │
         │         │         ├─→ [R2] 통계 정확성 리뷰 (trader-reviewer)
         │         │         │
         │         │         ├─→ Phase 2 (스프레드)
         │         │         │         │
         │         │         │         ├─→ Phase 3 (시그널/포지션)
         │         │         │         │         │
         │         │         │         │    [R3] 금융 로직 리뷰 (trader-reviewer) ← GATE
         │         │         │         │    [R4] Phase 2-3 코드 리뷰 (code-reviewer)
         │         │         │         │         │
         │         │         │         │         ├─→ Phase 4 (백테스트)
         │         │         │         │                   │
         │         │         │         │              [R5] 백테스트 검증 (trader-reviewer)
         │         │         │         │              [R6] Phase 4 코드 리뷰 (code-reviewer)
         │         │         │         │                   │
         └─────────┴─────────┴─────────┴───────→ Phase 5a (모니터)
                                                          │
                                                   Phase 5b (재연결)
                                                          │
                                                   Phase 6 (통합)
                                                          │
                                                   [R7] 최종 리뷰 (trader + reviewer)
```

---

## 리뷰 전략

### 리뷰 유형

| 유형 | 에이전트 | 역할 | 실행 방식 |
|------|---------|------|----------|
| **금융 로직 리뷰** | trader | PnL 공식, 수수료 모델, liquidation, 포지션 사이징, 통계 정확성 검증 | 스펙 대비 구현 일치 여부 검증 |
| **코드 리뷰** | reviewer | Rust 코드 품질, 안전성, 성능, 에러 처리, trait 설계 | 코드 리뷰 + 개선 제안 |

### 리뷰 게이트 (Gate) vs 비동기 리뷰 (Async)

| 구분 | 설명 | 효과 |
|------|------|------|
| **Gate** | 리뷰 통과해야 다음 Wave 진행. 이슈 발견 시 구현자에게 피드백 후 수정. | 금융 로직 오류의 하류 전파 방지 |
| **Async** | 다음 Wave와 병렬로 리뷰 진행. 이슈는 발견 즉시 해당 teammate에게 전달. | 속도 유지하면서 품질 확보 |

### 리뷰 포인트 배치

| ID | 시점 | 유형 | 에이전트 | Gate/Async | 리뷰 대상 |
|----|------|------|---------|------------|----------|
| **R1** | Wave 1 완료 후 | 코드 리뷰 | code-reviewer | **Async** | arb-exchange trait 설계, MarketStream 인터페이스, 에러 타입 |
| **R2** | Phase 1 완료 후 | 금융 리뷰 | trader-reviewer | **Async** | 통계 유틸리티 (mean, stddev, z_score), CandleWindow, 수수료 계산 |
| **R3** | Phase 3 완료 후 | 금융 리뷰 | trader-reviewer | **GATE** | Signal 로직, PnL 계산, liquidation 공식, 포지션 사이징, 자본 관리 |
| **R4** | Phase 3 완료 후 | 코드 리뷰 | code-reviewer | **Async** | Phase 2-3 코드 (SpreadCalculator, Signal, PositionManager, PnL) |
| **R5** | Phase 4 완료 후 | 금융 리뷰 | trader-reviewer | **Async** | BacktestSimulator 로직, BacktestResult 집계, max_drawdown, regime 감지 |
| **R6** | Phase 4 완료 후 | 코드 리뷰 | code-reviewer | **Async** | Phase 4 코드 (페이지네이션, 시뮬레이터, 출력 모듈) |
| **R7** | Phase 6 완료 후 | 최종 리뷰 | **양쪽 모두** | **GATE** | 전체 코드베이스 통합 리뷰 (trader: 전략 정합성, reviewer: 코드 품질) |

> **R3가 유일한 중간 GATE인 이유**: Phase 3의 PnL/Signal/Liquidation 로직은 Phase 4 백테스트의 핵심 입력이다. 여기서 금융 로직 오류가 있으면 백테스트 결과 자체가 무의미해진다. 다른 리뷰는 Async로 진행하여 속도를 유지한다.

---

## 병렬 실행 Wave (리뷰 포함)

| Wave | 병렬 스트림 | 설명 |
|------|-----------|------|
| **Wave 1** | exchange-traits 단독 | arb-exchange trait/type/error 변경 |
| **Wave 1→2 전환** | **R1** (Async) | code-reviewer: trait 설계 리뷰 (Wave 2와 병렬) |
| **Wave 2** | **4개 병렬** | Upbit SDK / Bybit SDK+Bithumb / arb-strategy Phase 0→1 / R1 리뷰 |
| **Wave 2→3 전환** | **R2** (Async) | trader-reviewer: 통계엔진 리뷰 (Wave 3와 병렬) |
| **Wave 3** | **3~4개 병렬** | strategy Phase 2→3 / Upbit MarketStream / Bybit MarketStream / R2 리뷰 |
| **Wave 3→4 게이트** | **R3** (GATE) + **R4** (Async) | trader-reviewer: 금융 로직 **통과 필수** / code-reviewer: Phase 2-3 리뷰 |
| **Wave 4** | **3개 병렬** | Phase 4 백테스트 / Phase 5a 모니터 / R4 리뷰 |
| **Wave 4→5 전환** | **R5** + **R6** (Async) | trader: 백테스트 검증 / code-reviewer: Phase 4 코드 리뷰 |
| **Wave 5** | 순차 | Phase 5b → Phase 6 |
| **최종** | **R7** (GATE) | trader + reviewer: 전체 통합 리뷰 **통과 필수** |

---

## 팀 구성

### 팀 이름: `arb_poc_team`

```
┌────────────────────────────────────────────────────────────────────┐
│                         Lead (coordinator)                          │
│   workspace Cargo.toml, src/lib.rs, examples/, config, 조율/통합    │
├──────────┬──────────┬──────────────┬────────────┬─────────────────┤
│ exchange │ upbit    │ bybit        │ strategy   │                 │
│ -traits  │ -dev     │ -dev         │ -dev       │                 │
│          │          │              │            │                 │
│ arb-     │ upbit/   │ bybit/       │ arb-       │   trader    code│
│ exchange │          │ bithumb/     │ strategy/  │  -reviewer -rev │
│ crate    │          │              │            │   (금융)   (코드)│
│          │          │              │            │                 │
│ [구현]    │ [구현]    │ [구현]        │ [구현]      │    [리뷰]       │
└──────────┴──────────┴──────────────┴────────────┴─────────────────┘
```

### Teammate 상세

| Teammate | Agent Type | 소유 파일 | 담당 작업 |
|----------|-----------|----------|-----------|
| **exchange-traits** | coder | `crates/arb-exchange/src/` | Phase 0: trait 확장, MarketStream 정의, 에러/타입, 의존성 |
| **upbit-dev** | coder | `crates/arb-exchanges/src/upbit/` | Phase 0: get_candles_before, 정렬 통일 → Phase 5a: MarketStream 구현 |
| **bybit-dev** | coder | `crates/arb-exchanges/src/bybit/`, `src/bithumb/` | Phase 0: get_candles_before, 정렬, Bithumb stub → Phase 5a: MarketStream 구현 |
| **strategy-dev** | coder | `crates/arb-strategy/src/` | Phase 0 scaffold → Phase 1→2→3→4, output/ |
| **trader-reviewer** | trader | (읽기 전용) | 금융 로직 리뷰: 통계, PnL, liquidation, 시그널, 백테스트 검증 |
| **code-reviewer** | reviewer | (읽기 전용) | 코드 리뷰: Rust 품질, 안전성, 성능, trait 설계, 에러 처리 |

### Lead (coordinator) 담당

- `Cargo.toml` (workspace root) — 멤버 추가, 의존성
- `crates/arb-exchanges/Cargo.toml` — tokio, tokio-tungstenite, futures-util 등
- `crates/arb-exchanges/src/mod.rs` — 모듈 re-export
- `src/lib.rs` — strategy re-export
- `examples/zscore_backtest.rs`, `examples/zscore_monitor.rs`
- `config.example.toml` — 전략 설정 템플릿
- Phase 5a 모니터 통합, Phase 5b, Phase 6
- **리뷰 게이트 관리**: R3/R7 통과 여부 판단 및 피드백 전달

---

## 타임라인 시각화

```
시간 →  ┃ Wave 1    ┃ Wave 2           ┃ Wave 3           ┃ R3 Gate ┃ Wave 4        ┃ Wave 5  ┃ R7
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
exchange┃ trait정의  ┃                  ┃                  ┃         ┃               ┃         ┃
-traits ┃ ████████  ┃ (shutdown)       ┃                  ┃         ┃               ┃         ┃
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
upbit   ┃ (대기)    ┃ get_candles_     ┃ MarketStream     ┃ (idle)  ┃               ┃         ┃
-dev    ┃           ┃ before+정렬 ████ ┃ 구현 ████████    ┃         ┃ (shutdown)    ┃         ┃
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
bybit   ┃ (대기)    ┃ get_candles_     ┃ MarketStream     ┃ (idle)  ┃               ┃         ┃
-dev    ┃           ┃ before+stub ████ ┃ 구현 ████████    ┃         ┃ (shutdown)    ┃         ┃
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
strategy┃ (대기)    ┃ scaffold+Phase1  ┃ Phase 2→3        ┃ (대기)  ┃ Phase 4       ┃         ┃
-dev    ┃           ┃ ████████████████ ┃ ████████████████ ┃ R3 fix? ┃ ████████████  ┃(shutdown┃
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
trader  ┃           ┃                  ┃         R2 ████  ┃ R3 ████ ┃      R5 ████  ┃         ┃ R7
-review ┃ (대기)    ┃ (대기)            ┃ 통계리뷰(Async)  ┃ GATE    ┃ 백테스트(Async)┃         ┃████
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
code    ┃           ┃  R1 ████         ┃                  ┃ R4 ████ ┃      R6 ████  ┃         ┃ R7
-review ┃ (대기)    ┃ trait리뷰(Async)  ┃ (idle)           ┃ Async   ┃ 코드리뷰(Async)┃         ┃████
────────╋───────────╋──────────────────╋──────────────────╋─────────╋───────────────╋─────────╋──────
lead    ┃ (조율)    ┃ workspace setup  ┃ (조율)           ┃ Gate    ┃ Phase5a+예제  ┃ 5b+6    ┃ Gate
        ┃           ┃ ████             ┃                  ┃ 관리    ┃ ████████████  ┃ ████    ┃ 관리
```

---

## 리뷰 상세

### R1: Trait 설계 리뷰 (Async)

- **시점**: Wave 1 완료 직후 (Wave 2와 병렬)
- **담당**: code-reviewer
- **대상 파일**: `crates/arb-exchange/src/` 전체
- **검증 항목**:
  - `MarketData` trait의 `get_candles_before`, `market_code` 시그니처 적절성
  - `MarketStream` trait의 `subscribe`/`unsubscribe` 인터페이스 설계
  - `MarketEvent` enum 설계 (Trade, BestQuote)
  - `StreamConfig` 구조체 필드 완전성
  - `ExchangeError` WebSocket 관련 variant 추가 적절성
  - `pub use chrono;` re-export 확인
  - Object safety, Send/Sync bound 검증
- **통과 기준**: 설계 이슈 없으면 pass. 이슈 발견 시 exchange-traits에 피드백 (이미 shutdown이면 lead가 수정)

### R2: 통계 엔진 리뷰 (Async)

- **시점**: Phase 1 완료 후 (Wave 3와 병렬)
- **담당**: trader-reviewer
- **대상 파일**: `crates/arb-strategy/src/common/` (statistics.rs, candle_window.rs, fee.rs, convert.rs)
- **검증 항목**:
  - `mean()`: 전체 순회 평균 정확성
  - `stddev()`: **모집단 표준편차 (N으로 나눔)** 확인 (N-1 아님)
  - `z_score()`: `min_stddev_threshold` guard 동작 확인
  - `CandleWindow`: VecDeque 전체 재계산 방식 정확성, `is_ready()` 로직
  - 수수료 계산: 라운드트립 `(upbit_fee + bybit_fee) × 2` 공식 일치
  - `decimal_to_f64` / `f64_to_decimal`: NaN/Infinity 방어 확인
- **통과 기준**: 공식이 스펙과 일치하면 pass. 오류 발견 시 strategy-dev에 메시지

### R3: 금융 로직 리뷰 (GATE)

- **시점**: Phase 3 완료 후 (Wave 4 진행 전 blocking)
- **담당**: trader-reviewer
- **대상 파일**: `crates/arb-strategy/src/zscore/` (signal.rs, position.rs, pnl.rs, spread.rs)
- **검증 항목**:
  - **스프레드 공식**: `(bybit - upbit_usdt) / upbit_usdt × 100` 부호/방향 확인
  - **합성 가격**: `upbit_coin_krw / usdt_krw` 환산 정확성
  - **Signal 진입 조건**: z_score ≥ entry_z AND profit > 0 AND 포지션 없음 AND 가용 자본 충분
  - **Signal 청산 조건**: z_score ≤ exit_z OR liquidation
  - **expected_profit_pct**: `(current_spread - mean_spread) - roundtrip_fee_pct` 공식
  - **PnL 계산**: Upbit PnL = (exit - entry) × qty, Bybit PnL = (entry - exit) × qty (short)
  - **수수료**: `size_usdt × fee_rate × 2` (진입+청산, notional 근사)
  - **Liquidation 공식**: `entry_price × (1 + 1/leverage - MMR - bybit_taker_fee)`
  - **포지션 사이징**: `single_leg = total_capital × position_ratio`, 필요 자본 = single_leg × 2
  - **가용 자본 확인**: `used_capital + (size_usdt × 2) ≤ total_capital`
  - **USDT notional matching**: 양쪽 동일 USDT 금액, qty 차이 허용
- **통과 기준**: **모든 금융 공식이 스펙과 정확히 일치해야 pass**
- **실패 시**: strategy-dev에 수정 사항 전달 → 수정 후 재리뷰 → 통과 시 Wave 4 진행

### R4: Phase 2-3 코드 리뷰 (Async)

- **시점**: Phase 3 완료 후 (Wave 4와 병렬)
- **담당**: code-reviewer
- **대상 파일**: `crates/arb-strategy/src/zscore/` (spread.rs, signal.rs, position.rs, pnl.rs)
- **검증 항목**:
  - `SpreadCalculator::update()` 시그니처 및 3-way 동기화 로직
  - Forward-fill 구현 정확성 (연속 5분 경고 포함)
  - `Decimal → f64` 변환 경계에서 에러 처리
  - `PositionManager`의 HashMap 관리, `check_liquidation()` 로직
  - `ClosedPosition` 필드 완전성
  - `.unwrap()` 사용 여부 (프로덕션 코드에서 금지)
  - `clone()` 사용 최소화
  - 에러 타입 활용 (`StrategyError`, `PositionError`, `StatisticsError`)

### R5: 백테스트 검증 리뷰 (Async)

- **시점**: Phase 4 완료 후 (Wave 5와 병렬)
- **담당**: trader-reviewer
- **대상 파일**: `crates/arb-strategy/src/zscore/simulator.rs`, `output/`
- **검증 항목**:
  - 워밍업 기간(1440분) 동안 시그널 미발생 확인
  - 매 분마다 liquidation 체크 수행 확인
  - `BacktestResult` 집계 정확성:
    - `total_pnl` = sum(upbit_pnl + bybit_pnl) (gross)
    - `net_pnl` = total_pnl - total_fees
    - `max_drawdown`: equity curve peak-to-trough
    - `win_rate`: winning / total
  - Regime change 감지: rolling mean 2σ 이동 경고
  - 통계적 유의성 경고 (거래 < 30회)
  - `stationarity_metrics` 필드 예약 확인
  - 미청산 포지션 `unrealized_pnl` mark-to-market 계산
  - CSV 출력 형식이 스펙과 일치하는지 확인

### R6: Phase 4 코드 리뷰 (Async)

- **시점**: Phase 4 완료 후 (Wave 5와 병렬)
- **담당**: code-reviewer
- **대상 파일**: `crates/arb-strategy/src/zscore/simulator.rs`, `output/console.rs`, `output/csv.rs`
- **검증 항목**:
  - 페이지네이션 merge 알고리즘: 무한루프 방지, 중복 제거, 오름차순 보장
  - rate limit 딜레이 (`tokio::time::sleep`) 적용 확인
  - `BacktestSimulator<U, B>` generic 설계 적절성
  - CSV writer 에러 처리
  - 콘솔 출력 `tracing` 활용 패턴
  - 메모리 효율: 대량 캔들 처리 시 힙 할당 최소화

### R7: 최종 통합 리뷰 (GATE)

- **시점**: Phase 6 완료 후 (릴리즈 전 blocking)
- **담당**: trader-reviewer + code-reviewer (양쪽 모두)
- **대상**: 전체 코드베이스

**trader-reviewer 검증**:
  - 전체 전략 흐름의 일관성 (데이터 수집 → 스프레드 → Z-Score → 시그널 → 포지션 → PnL)
  - 스펙의 "확정된 세부 요구사항" 테이블 항목별 구현 완료 확인
  - "알려진 한계" 섹션의 각 항목이 코드 주석/문서에 반영되었는지 확인
  - 실전 전환 시 교체 필요 부분(수수료 근사치, Upbit 코인 차감 등)이 TODO로 표시되었는지

**code-reviewer 검증**:
  - `cargo clippy` 경고 없음
  - `cargo test` 전체 통과
  - 공개 API `///` 주석 완전성
  - 코드 주석 한글화 (`.claude/skills/comment.md` 준수)
  - `tracing` 로깅 패턴 (`.claude/skills/logging.md` 준수)
  - 에러 메시지 영어 유지 (국제화 고려)
  - 불필요한 `.unwrap()`, `clone()` 없음
  - 의존성 그래프: `arb-strategy` → `arb-exchange`만 의존 (DI 패턴)

- **통과 기준**: 양쪽 모두 pass 해야 최종 완료

---

## 충돌 방지 전략

### 파일 소유권 규칙

| 공유 파일 | 담당자 | 사유 |
|----------|--------|------|
| `Cargo.toml` (workspace root) | Lead | 멤버 추가, workspace 의존성 관리 |
| `crates/arb-exchanges/Cargo.toml` | Lead | tokio, tokio-tungstenite 등 공유 의존성 |
| `crates/arb-exchanges/src/mod.rs` | Lead | 모듈 re-export 관리 |
| `src/lib.rs` | Lead | strategy re-export |

> **핵심 원칙**: SDK 개발자(upbit-dev, bybit-dev)는 자기 **서브디렉토리만** 수정한다.
> 공유 파일(Cargo.toml, mod.rs)은 반드시 Lead가 수정하여 충돌을 방지한다.
> 리뷰어(trader-reviewer, code-reviewer)는 **읽기 전용**으로 코드를 검토하며 파일을 수정하지 않는다.

### Git 브랜치 전략

Agent team은 단일 워킹 디렉토리에서 작업하므로, **브랜치 분리 없이** 파일 소유권 분리로 충돌을 방지한다.
각 teammate는 자신이 소유한 파일만 수정하며, Lead가 공유 파일 변경을 중앙 관리한다.

---

## 태스크 목록 (TaskList)

### Wave 1: arb-exchange trait 변경

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-01 | `ExchangeName` canonical 통일 (arb-exchange) | exchange-traits | - |
| T-02 | `MarketData` trait 확장: `get_candles_before`, `market_code` | exchange-traits | T-01 |
| T-03 | `ExchangeAdapter` trait에 `get_candles_before` 추가, 매크로 삭제 | exchange-traits | T-02 |
| T-04 | `MarketStream` trait 정의 (MarketEvent, StreamConfig, subscribe, unsubscribe) | exchange-traits | T-01 |
| T-05 | `arb-exchange` Cargo.toml에 `pub use chrono;`, tokio 의존성 추가 | exchange-traits | - |
| T-06 | WebSocket 관련 에러 타입 추가 | exchange-traits | T-04 |

### R1: Trait 설계 리뷰 (Async — Wave 2와 병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-R1 | arb-exchange trait 설계 리뷰 (MarketData, MarketStream, 에러 타입) | code-reviewer | T-01~T-06 |

### Wave 2: SDK 구현 + Strategy scaffold (병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-07 | Lead: workspace Cargo.toml 업데이트 (arb-strategy 멤버, csv 의존성) | Lead | T-05 |
| T-08 | Lead: `arb-exchanges` Cargo.toml에 tokio, tokio-tungstenite, futures-util 추가 | Lead | T-05 |
| T-09 | Lead: `src/lib.rs`에 `pub use arb_strategy as strategy;` 추가 | Lead | T-07 |
| T-10 | Upbit: `get_candles_before` 구현 (to 파라미터, before-1sec, 오름차순) | upbit-dev | T-02, T-08 |
| T-11 | Upbit: 기존 `get_candles` 반환 순서 오름차순 통일 | upbit-dev | T-02 |
| T-12 | Bybit: `get_candles_before` 구현 (end 파라미터, before_ms-1, reverse) | bybit-dev | T-02, T-08 |
| T-13 | Bybit: 기존 `get_candles` 반환 순서 오름차순 통일 | bybit-dev | T-02 |
| T-14 | Bithumb: `get_candles_before` stub 구현 | bybit-dev | T-02 |
| T-15 | `arb-strategy` 크레이트 생성 + Cargo.toml + 모듈 구조 | strategy-dev | T-07 |
| T-16 | `StrategyError`, `StatisticsError`, `PositionError` 에러 타입 정의 | strategy-dev | T-15 |
| T-17 | `common/convert.rs` Decimal ↔ f64 변환 유틸리티 | strategy-dev | T-15 |
| T-18 | `ZScoreConfig` + `validate()` 구현 | strategy-dev | T-16 |
| T-19 | `CandleWindow` (VecDeque + 전체 재계산) 구현 | strategy-dev | T-15 |
| T-20 | 통계 유틸리티 (mean, stddev, z_score) 구현 | strategy-dev | T-19 |
| T-21 | 수수료 계산 모듈 구현 | strategy-dev | T-15 |
| T-22 | Phase 1 단위 테스트 작성 | strategy-dev | T-17~T-21 |

### R2: 통계 엔진 리뷰 (Async — Wave 3와 병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-R2 | Phase 1 통계 엔진 금융 리뷰 (mean, stddev, z_score, 수수료, CandleWindow) | trader-reviewer | T-22 |

### Wave 3: 스프레드/시그널/포지션 + MarketStream (병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-23 | `SpreadCalculator` 구현 (update(), 3-way 동기화, forward-fill) | strategy-dev | T-22 |
| T-24 | 상대 스프레드(%) 계산 + 코인별 윈도우 관리 | strategy-dev | T-23 |
| T-25 | Phase 2 단위 테스트 | strategy-dev | T-24 |
| T-26 | `Signal` enum + 시그널 생성 로직 | strategy-dev | T-25 |
| T-27 | `VirtualPosition` + `PositionManager` + liquidation 체크 | strategy-dev | T-25 |
| T-28 | PnL 계산 (`ClosedPosition`) 구현 | strategy-dev | T-27 |
| T-29 | Phase 3 단위 테스트 | strategy-dev | T-26~T-28 |
| T-30 | Upbit `MarketStream` 구현 (trade 구독, backpressure) | upbit-dev | T-04, T-06, T-10 |
| T-31 | Bybit `MarketStream` 구현 (orderbook.1 구독, backpressure) | bybit-dev | T-04, T-06, T-12 |

### R3: 금융 로직 리뷰 (GATE — Wave 4 진행 전 blocking)

| ID | 태스크 | 담당 | 의존성 | 비고 |
|----|--------|------|--------|------|
| T-R3 | Phase 3 금융 로직 GATE 리뷰 (Signal, PnL, Liquidation, 포지션 사이징) | trader-reviewer | T-29 | **GATE: 통과 필수** |

### R4: Phase 2-3 코드 리뷰 (Async — Wave 4와 병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-R4 | Phase 2-3 코드 리뷰 (SpreadCalculator, Signal, PositionManager, PnL) | code-reviewer | T-29 |

### Wave 4: 백테스트 + 실시간 모니터 (병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-32 | 페이지네이션 merge 알고리즘 구현 | strategy-dev | T-R3 (GATE 통과) |
| T-33 | `BacktestSimulator<U, B>` 순차 시뮬레이션 엔진 | strategy-dev | T-32 |
| T-34 | `BacktestResult` 집계 (max_drawdown, daily_pnl, stationarity_metrics) | strategy-dev | T-33 |
| T-35 | Regime change 감지 경고 구현 | strategy-dev | T-33 |
| T-36 | 콘솔 출력 (`output/console.rs`) | strategy-dev | T-34 |
| T-37 | CSV 출력 (`output/csv.rs`) | strategy-dev | T-34 |
| T-38 | `ZScoreMonitor<U, B>` 기본 구현 (캔들 구성, 이벤트 루프, 시그널) | Lead | T-29, T-30, T-31 |
| T-39 | `examples/zscore_backtest.rs` 예제 | Lead | T-34, T-10, T-12 |
| T-40 | `examples/zscore_monitor.rs` 예제 | Lead | T-38 |

### R5 + R6: 백테스트 리뷰 (Async — Wave 5와 병렬)

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-R5 | Phase 4 백테스트 금융 검증 리뷰 (시뮬레이터 로직, 결과 집계, CSV 형식) | trader-reviewer | T-34 |
| T-R6 | Phase 4 코드 리뷰 (페이지네이션, 시뮬레이터, 출력 모듈) | code-reviewer | T-37 |

### Wave 5: 재연결 + 통합

| ID | 태스크 | 담당 | 의존성 |
|----|--------|------|--------|
| T-41 | `StreamConfig` 기반 exponential backoff 재연결 | Lead | T-38 |
| T-42 | REST fallback 전환/복귀 | Lead | T-41 |
| T-43 | `CancellationToken` 기반 graceful shutdown | Lead | T-41 |
| T-44 | `toml` 크레이트 설정 통합 | Lead | T-39 |
| T-45 | `config.example.toml` 전략 설정 템플릿 | Lead | T-44 |
| T-46 | 전체 통합 테스트 | Lead | T-39, T-40 |

### R7: 최종 통합 리뷰 (GATE — 릴리즈 전 blocking)

| ID | 태스크 | 담당 | 의존성 | 비고 |
|----|--------|------|--------|------|
| T-R7a | 최종 금융 정합성 리뷰 (전략 흐름, 스펙 대조, 알려진 한계 반영) | trader-reviewer | T-46 | **GATE: 통과 필수** |
| T-R7b | 최종 코드 품질 리뷰 (clippy, test, 주석, 로깅, DI 패턴) | code-reviewer | T-46 | **GATE: 통과 필수** |

---

## Teammate 생명주기 관리

### 스폰 순서 및 시점

| 순서 | Teammate | 스폰 시점 | Shutdown 시점 |
|------|----------|----------|--------------|
| 1 | exchange-traits | 팀 생성 직후 | Wave 1 완료 후 (T-06 완료) |
| 2 | code-reviewer | Wave 1 완료 직후 (R1 시작) | R7b 완료 후 |
| 3 | upbit-dev | Wave 2 시작 시 | Phase 5a Upbit MarketStream 완료 후 (T-30) |
| 4 | bybit-dev | Wave 2 시작 시 | Phase 5a Bybit MarketStream 완료 후 (T-31) |
| 5 | strategy-dev | Wave 2 시작 시 | Phase 4 완료 후 (T-37) |
| 6 | trader-reviewer | R2 시작 시 (Phase 1 완료 후) | R7a 완료 후 |

> **토큰 절약**: 리뷰어는 리뷰 대상 코드가 준비될 때 스폰하고, 리뷰 간 idle 기간에는 shutdown하지 않는다 (context 유지가 이후 리뷰에 유리).
> 구현 teammate는 담당 작업 완료 즉시 shutdown하여 토큰을 절약한다.

### Idle 기간 활용

| Teammate | Idle 구간 | 활용 방안 |
|----------|----------|----------|
| upbit-dev | Wave 2 SDK 완료 ~ Wave 3 MarketStream | R1 피드백 반영 (있을 경우) |
| bybit-dev | Wave 2 SDK 완료 ~ Wave 3 MarketStream | R1 피드백 반영 (있을 경우) |
| code-reviewer | R1 완료 ~ R4 시작 | 기존 코드베이스 탐색, 리뷰 기준 정리 |
| trader-reviewer | R2 완료 ~ R3 시작 | 스펙 재검토, 리뷰 체크리스트 준비 |

---

## 사전 준비 (팀 생성 전)

### 1. Agent Teams 활성화

`.claude/settings.json`에 환경변수 추가:

```json
{
  "env": {
    "ENABLE_LSP_TOOL": "true",
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

### 2. 팀 생성 명령

```
TeamCreate: arb_poc_team
```

### 3. Teammate 스폰 순서

1. **exchange-traits** 먼저 스폰 (Wave 1 blocking)
2. exchange-traits 완료 후 **upbit-dev**, **bybit-dev**, **strategy-dev**, **code-reviewer** 동시 스폰 (Wave 2 + R1)
3. Phase 1 완료 후 **trader-reviewer** 스폰 (R2 시작)

### 4. Plan Approval 설정

exchange-traits는 모든 downstream에 영향을 주므로 **plan approval 필수**:

```
Spawn exchange-traits teammate with plan approval required.
```

나머지 teammate는 스펙이 충분히 상세하므로 plan approval 없이 진행.

---

## 예상 토큰 비용

| 항목 | 추정 |
|------|------|
| Lead context | 1x (기본) |
| 구현 Teammate 4개 | 4x (각각 독립 context) |
| 리뷰 Teammate 2개 | 2x (각각 독립 context) |
| 총 배수 | ~7x (단일 세션 대비) |
| 절감 시간 | Wave 2~4에서 ~2~3배 속도 향상 + 리뷰 병렬화 |

> **트레이드오프**: 토큰 ~7배 사용, 시간 ~2~3배 단축.
> 리뷰어를 포함하면 구현과 리뷰가 파이프라인화되어, "구현 완료 후 리뷰" 대비 전체 리드타임 단축.
> 리뷰 GATE(R3, R7)에서의 대기 시간은 금융 로직 정확성 보장을 위한 필수 비용.

---

## 리스크 및 대응

| 리스크 | 대응 |
|--------|------|
| 공유 파일 충돌 | Lead가 Cargo.toml, mod.rs 등 공유 파일 전담 |
| exchange-traits 지연 | 모든 Wave 2가 블록되므로 최우선 완료 |
| strategy-dev 과부하 | Phase 1-4가 순차적이라 분리 어려움. 가장 큰 작업량이므로 충분한 context 제공 |
| MarketStream 구현 복잡도 | Phase 5a/5b 분리로 복잡도 관리 |
| Teammate idle 시간 | upbit-dev/bybit-dev는 Wave 2 후 MarketStream까지 대기 가능. R1 피드백 반영으로 활용 |
| R3 GATE 실패 | strategy-dev에 피드백 전달 → 수정 → 재리뷰. 최대 1~2회 반복 예상 |
| 리뷰어 context 부족 | 리뷰어 스폰 시 스펙 파일 전체 경로 + 리뷰 체크리스트를 prompt에 포함 |
| R7 최종 리뷰 범위 과다 | R1~R6에서 점진적 리뷰가 완료되어 있으므로, R7은 통합 관점에 집중 |
