# 0005 — 세션 출력: 거래 내역 · 분봉 통계 · 요약 파일 저장

## 목적

실시간 모니터링(zscore_monitor) 세션의 결과를 파일로 저장하여 전략을 사후 분석·평가할 수 있도록 한다.

## 요구사항

### 1. 세션 디렉토리

| 항목 | 내용 |
|------|------|
| 기본 경로 | `./output/<timestamp>/` |
| 타임스탬프 형식 | `YYYY-MM-DD_HH-mm-ss` (세션 시작 시각, UTC) |
| 예시 | `output/2026-02-09_20-30-00/` |
| 자동 생성 | 세션 시작 시 디렉토리 자동 생성 |

### 2. 저장 파일 구성

세션 디렉토리 아래에 항목별로 별개 파일을 생성한다. CSV + JSON 이중 저장.

```
output/2026-02-09_20-30-00/
├── trades.csv              # 거래 내역 (실시간 append)
├── trades.json             # 거래 내역 (종료 시 일괄)
├── minutes.csv             # 분봉 통계 (실시간 append, 전 코인 단일 파일)
├── minutes.json            # 분봉 통계 (종료 시 일괄)
├── summary.json            # 종료 시 요약 (JSON only)
└── summary.txt             # 종료 시 요약 (사람이 읽기 쉬운 텍스트)
```

### 3. 거래 내역 (trades)

#### 저장 시점
- **실시간**: 진입/청산 발생 즉시 `trades.csv`에 append
- **종료 시**: 전체 거래 목록을 `trades.json`으로 일괄 저장

#### CSV 컬럼

| 컬럼 | 타입 | 설명 |
|------|------|------|
| `coin` | String | 코인 심볼 (BTC, XRP 등) |
| `entry_time` | DateTime | 진입 시각 (UTC ISO 8601) |
| `exit_time` | DateTime | 청산 시각 (UTC ISO 8601) |
| `holding_minutes` | u64 | 보유 시간 (분) |
| `size_usdt` | Decimal | 포지션 크기 (USDT, 단일 leg) |
| `upbit_entry_price` | Decimal | Upbit 진입가 (USD 환산) |
| `bybit_entry_price` | Decimal | Bybit 진입가 (USDT) |
| `upbit_exit_price` | Decimal | Upbit 청산가 (USD 환산) |
| `bybit_exit_price` | Decimal | Bybit 청산가 (USDT) |
| `entry_spread_pct` | f64 | 진입 스프레드 (%) |
| `exit_spread_pct` | f64 | 청산 스프레드 (%) |
| `entry_z_score` | f64 | 진입 Z-Score |
| `exit_z_score` | f64 | 청산 Z-Score |
| `entry_usd_krw` | f64 | 진입 시 USD/KRW 환율 |
| `exit_usd_krw` | f64 | 청산 시 USD/KRW 환율 |
| `upbit_pnl` | Decimal | Upbit PnL |
| `bybit_pnl` | Decimal | Bybit PnL |
| `upbit_fees` | Decimal | Upbit 수수료 |
| `bybit_fees` | Decimal | Bybit 수수료 |
| `total_fees` | Decimal | 총 수수료 |
| `net_pnl` | Decimal | 순 PnL |
| `is_liquidated` | bool | 강제 청산 여부 |

#### JSON 구조
`trades.json`: `ClosedPosition` 배열을 그대로 직렬화.

### 4. 분봉 통계 (minutes)

#### 저장 시점
- **워밍업 데이터**: 워밍업 완료 후 과거 캔들 기반 통계를 일괄 기록
- **실시간**: 매 분 완결 시 `minutes.csv`에 append
- **종료 시**: 전체 분봉 통계를 `minutes.json`으로 일괄 저장

#### CSV 컬럼 (전 코인 단일 파일, `coin` 컬럼으로 구분)

| 컬럼 | 타입 | 설명 |
|------|------|------|
| `timestamp` | DateTime | 분봉 시각 (UTC ISO 8601) |
| `coin` | String | 코인 심볼 |
| `upbit_close` | f64 | Upbit 종가 (USD 환산) |
| `bybit_close` | f64 | Bybit 종가 (USDT) |
| `usd_krw` | f64 | 해당 분의 USD/KRW 환율 |
| `spread_pct` | f64 | 스프레드 (%) |
| `mean` | f64 | rolling mean |
| `stddev` | f64 | rolling stddev |
| `z_score` | f64 | Z-Score |
| `position` | String | 포지션 상태 ("OPEN" / "NONE") |
| `source` | String | 데이터 출처 ("warmup" / "live") |

### 5. 종료 시 요약 (summary)

#### 포함 지표

**기본:**

| 항목 | 설명 |
|------|------|
| `session_start` | 세션 시작 시각 |
| `session_end` | 세션 종료 시각 |
| `duration_minutes` | 총 실행 시간 (분) |
| `coins` | 모니터링 코인 목록 |
| `usd_krw_start` | 시작 시 환율 |
| `usd_krw_end` | 종료 시 환율 |
| `total_trades` | 총 거래 수 |
| `winning_trades` | 승리 거래 수 |
| `losing_trades` | 패배 거래 수 |
| `win_rate` | 승률 (%) |
| `total_net_pnl` | 순 PnL 합계 (USDT) |
| `max_drawdown` | 최대 낙폭 (USDT) |
| `total_events` | 총 수신 이벤트 수 |

**상세:**

| 항목 | 설명 |
|------|------|
| `profit_factor` | 총이익 / 총손실 |
| `avg_holding_minutes` | 평균 보유 시간 (분) |
| `daily_pnl` | 일별 PnL 배열 `[{date, pnl}]` |
| `coin_pnl` | 코인별 PnL `[{coin, trades, net_pnl, win_rate}]` |
| `sharpe_ratio` | 일별 PnL 기반 Sharpe Ratio (무위험이자율 0 가정) |
| `total_fees` | 총 수수료 합계 |
| `liquidation_count` | 강제 청산 횟수 |

#### summary.txt 예시

```
=== 세션 요약 ===
기간: 2026-02-09 20:30:00 ~ 2026-02-10 08:30:00 (720분)
코인: BTC, XRP, BERA
환율: 1463.33 → 1465.20

거래: 12건 (승 8 / 패 4, 승률 66.7%)
순 PnL: +23.45 USDT
Max DD: -5.12 USDT
Profit Factor: 2.34
Sharpe Ratio: 1.85

코인별:
  BTC: 5건, +15.20 USDT (승률 80.0%)
  XRP: 4건, +10.25 USDT (승률 75.0%)
  BERA: 3건, -2.00 USDT (승률 33.3%)

일별 PnL:
  2026-02-09: +18.50 USDT
  2026-02-10: +4.95 USDT

총 수수료: 8.40 USDT
강제 청산: 0건
총 이벤트: 54,320건
```

### 6. 설정

`strategy.toml`에 출력 설정 섹션을 추가한다.

```toml
[output]
enabled = true           # 파일 출력 활성화 (기본: true)
dir = "output"           # 출력 기본 디렉토리 (기본: "output")
```

`enabled = false`이면 파일 출력 없이 기존처럼 콘솔만 출력한다.

## 파일 변경 범위 (예상)

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/output/` | **신규** | 출력 모듈 (writer, summary 등) |
| `crates/arb-strategy/src/zscore/monitor.rs` | **수정** | SessionWriter 통합, 분봉/거래 append 호출 |
| `crates/arb-strategy/src/zscore/config.rs` | **수정** | OutputConfig 추가 |
| `crates/arb-strategy/src/zscore/pnl.rs` | **수정** | Serialize derive 추가, 상세 요약 계산 함수 |
| `crates/arb-strategy/src/lib.rs` | **수정** | output 모듈 re-export |
| `examples/zscore_monitor.rs` | **수정** | SessionWriter 초기화 + 결과 저장 호출 |

## 제약 및 고려사항

- CSV append는 `BufWriter`로 매 거래/분봉마다 flush (크래시 시 데이터 보존)
- JSON은 종료 시 일괄 저장 (중간 크래시 시 JSON 없음, CSV로 복구 가능)
- 파일 I/O는 이벤트 루프 밖에서 처리하여 틱 처리 지연 최소화
- `.gitignore`에 `output/` 추가
- `ClosedPosition`에 `Serialize` derive 추가 필요
