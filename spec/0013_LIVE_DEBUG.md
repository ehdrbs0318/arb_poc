# 0013_LIVE_DEBUG

## 사용자의 요청

라이브 운용 중 발견되는 이슈를 별도 스펙으로 등록/추적한다.

등록 이슈:

- **LD-0001**: `balance_snapshots`에 Bybit USDT 잔고가 기록될 때,
  주문 가능 자금이 `locked`로 오인식되는 문제 분석 및 수정
- **LD-0002**: 라이브 실행 시 `minutes`/`funding_schedules`/`trades`/`alerts`
  테이블 INSERT 경로가 미연결이거나 실질적으로 동작하지 않는 문제 수정
- **LD-0003**: `sync_from_exchange()` 주기 동기화가 스펙(`0011`)대로 실행되지 않는 문제
- **LD-0004**: minute timer 기반 reconciliation task가 스펙(`0011`)과 다르게 미연결/미동작인 문제
- **LD-0005**: `shutdown_policy` 설정이 runtime 종료 동작 분기에 실제 반영되지 않는 문제
- **LD-0006**: 라이브 모드의 파일 출력(`SessionWriter`, CSV/JSON) 경로 잔존으로 DB 우선 정책과 불일치하는 문제
- **LD-0007**: 종료 시 `DbWriter` drain/flush 대기 누락으로 최종 write 유실 가능한 문제

---

## 목적

1. 라이브 이슈를 재현 데이터와 함께 문서화
2. 원인 분석 결과를 코드 레벨로 고정
3. 수정 범위/검증 항목을 체크리스트로 관리
4. 이후 "전부 수행" 요청 시 즉시 실행 가능한 작업 단위 제공

---

## 이슈 레지스트리

| ID | 제목 | 상태 | 우선순위 | 관련 스펙 |
|---|---|---|---|---|
| LD-0001 | Bybit USDT `available/locked` 오인식 | Done | P0 | `0011`, `0012` |
| LD-0002 | DB INSERT 경로 미연결 (`minutes`, `funding_schedules`, `trades`, `alerts`) | Done | P0 | `0011`, `0012` |
| LD-0003 | `sync_from_exchange()` 5분 주기 미실행 | Open | P0 | `0011` |
| LD-0004 | minute timer reconciliation 미실행 | Open | P0 | `0011` |
| LD-0005 | `shutdown_policy` 미반영 | Open | P1 | `0011` |
| LD-0006 | 라이브 파일 출력 경로 잔존 | Open | P1 | `0011` |
| LD-0007 | 종료 시 `DbWriter` flush 대기 누락 | Open | P0 | `0011` |

---

## LD-0001 — Bybit USDT `available/locked` 오인식

### 1. 현상

사용자 관측 데이터:

```csv
id,created_at,snapshot_group_id,session_id,record_type,cex,currency,available,locked,coin_value,total,position_id,usd_krw,usdt_krw,total_usd,total_usdt
5,2026-02-14 08:40:18.931,1,5,PERIODIC,UPBIT,KRW,2779359.74056633,0.00000000,32036.77901410,2811396.51958043,,1440.6400146484375,1470,1951.49134481,1912.51463917
6,2026-02-14 08:50:45.880,1,6,PERIODIC,UPBIT,KRW,2779359.74056633,0.00000000,32070.89771410,2811430.63828043,,1440.6400146484375,1471,1951.51502783,1911.23768748
7,2026-02-14 08:50:45.880,1,6,PERIODIC,BYBIT,USDT,0.00000000,3003.82500044,0.00000000,3003.82500044,,1440.6400146484375,1471,3003.82500044,3003.82500044
```

`BYBIT/USDT` 행에서:

- `available = 0`
- `locked = total`

로 기록되어, 주문 가능 자금이 전부 잠긴 것처럼 보인다.

### 2. 기대 동작

- `total`은 `equity`를 사용한다 (`0012`와 동일)
- `locked`는 Bybit API의 `locked` 필드를 사용한다 (가능하면 원본 그대로)
- `available`은 **USDT `walletBalance`**로 기록한다 (사용자 운영 정책 반영)
- `coin_value`는 **포지션 가치(`unrealisedPnl`)**로 기록한다
- 따라서 `total`은 `equity`(ground truth), 분해 필드(`available/locked/coin_value`)는 진단용으로 관리한다

### 3. 원인 분석 (현재 코드 기준)

#### 3.1 잘못된 변환 로직

파일: `crates/arb-exchanges/src/bybit/client.rs`

```rust
fn convert_balance(b: crate::bybit::types::BybitCoinBalance) -> Balance {
    let locked = b.wallet_balance - b.available_to_withdraw;

    Balance {
        currency: b.coin,
        balance: b.available_to_withdraw,
        locked,
        ...
    }
}
```

문제점:

1. `locked`를 API `locked` 필드가 아니라 `wallet - availableToWithdraw`로 계산
2. `availableToWithdraw`가 0(또는 공백 파싱으로 0)일 때 `locked ≈ wallet_balance`
3. 결과적으로 주문 가능 자금이 `locked`로 과대 계상됨

#### 3.2 스냅샷 기록 경로는 단순 전달

파일: `crates/arb-strategy/src/zscore/balance_recorder.rs`

- `build_bybit_row()`는 `Balance`의 `balance/locked`를 그대로 DB row로 기록
- 즉, 오염 지점은 recorder가 아니라 exchange 변환 레이어

#### 3.3 문서/구현 불일치

`0012_BALANCE_SNAPSHOT` 문서는 Bybit 분해 필드에서:

- `locked = API locked`
- `available = availableToWithdraw`(기존 문서 기준)

을 명시하지만, 실제 구현은 `locked = wallet - availableToWithdraw`를 사용 중

### 4. 영향 범위

1. `balance_snapshots` 분석 왜곡
2. 라이브 로그에서 잔고 상태 오판
3. `main.rs` 초기 Bybit 잔고 조회(`get_balance("USDT")`) 해석 혼선
4. 향후 리스크/알림(잔고 부족 경고) 정확도 저하 가능

### 5. 수정 원칙

1. `locked` 파생 계산(`wallet - availableToWithdraw`) 제거
2. Bybit API 원본 필드 우선 사용 (`locked`, `equity`, `unrealisedPnl`)
3. `available`의 의미를 "출금 가능"과 "주문 가능" 중 무엇으로 쓸지 명시하고 코드/문서 동기화
4. 스냅샷은 `total` 중심(ground truth) + 분해 필드는 진단용으로 유지

---

## 구현 플랜 (LD-0001)

### Phase 1: 진단 로그 보강 (원인 확정)

- 대상: `BybitClient::get_balances()` 또는 `convert_balance()` 호출 경로
- 목표: USDT 항목의 원본 필드(`walletBalance`, `availableToWithdraw`, `locked`, `equity`, `unrealisedPnl`)를 디버그 로그로 1회 확인

### Phase 2: 매핑 수정

- `convert_balance()`에서 `locked`를 API `locked.unwrap_or(0)`로 변경
- 파생식 `wallet - availableToWithdraw` 제거
- `available = walletBalance`로 확정 (`availableToWithdraw`는 deprecated 필드이므로 사용하지 않음)
- `coin_value = unrealisedPnl`, `total = equity` 매핑을 문서/코드에 고정

### Phase 3: 스냅샷/초기화 경로 검증

- `balance_recorder`에서 Bybit row가 수정된 `Balance` 값을 그대로 반영하는지 확인
- `main.rs`의 BalanceTracker 초기화(Bybit USDT)가 비정상 0으로 시작하지 않는지 확인

### Phase 4: 테스트 추가

- 단위 테스트: `convert_balance()`
  - 케이스 1: `availableToWithdraw=0`, `locked=0`, `wallet>0` → `locked=0` 보장
  - 케이스 2: `locked` 값 존재 시 그대로 매핑
  - 케이스 3: `equity`/`unrealisedPnl` 유지 확인
- 통합 테스트: `build_bybit_row()`의 `available/locked/total` 일관성 검증

### Phase 5: 문서 동기화

- `spec/0012_BALANCE_SNAPSHOT.md`의 매핑 정의를 구현과 일치시킴
- 필요 시 `CLAUDE.md`의 관련 설명 업데이트

---

## 체크리스트

### A. 원인 확인

- [x] 원본 Bybit 잔고 필드 로그 캡처 (`walletBalance`, `availableToWithdraw`, `locked`, `equity`, `unrealisedPnl`)
- [x] `locked = wallet - availableToWithdraw`가 실제 오인식 원인임을 로그로 확인

### B. 코드 수정

- [x] `crates/arb-exchanges/src/bybit/client.rs`: 파생 `locked` 계산 제거
- [x] `crates/arb-exchanges/src/bybit/client.rs`: `locked`를 API 필드 기반으로 매핑
- [x] `crates/arb-exchange/src/types.rs`: `Balance` 필드 의미 주석 점검/수정

### C. 스냅샷 경로 검증

- [x] `crates/arb-strategy/src/zscore/balance_recorder.rs`: 수정된 매핑 반영 확인
- [x] PERIODIC 스냅샷에서 Bybit `available/locked`가 현실적인 값인지 확인

### D. 테스트

- [x] `arb-exchanges` 단위 테스트 추가/수정
- [x] `arb-strategy` 스냅샷 조립 테스트 추가/수정
- [x] `cargo test -p arb-exchanges`
- [x] `cargo test -p arb-strategy`

### E. 문서

- [x] `spec/0012_BALANCE_SNAPSHOT.md` 동기화
- [x] 본 문서(`0013_LIVE_DEBUG`) 상태 갱신 (Open -> In Progress -> Done)

---

## 완료 기준 (LD-0001)

아래 조건을 모두 만족하면 완료:

1. 신규 스냅샷에서 Bybit `available/locked`가 더 이상 전액 lock 형태로 왜곡되지 않는다
2. `total = equity` 보장은 유지된다
3. 관련 테스트가 통과한다
4. `0012` 문서와 구현이 일치한다

---

## LD-0002 — DB INSERT 경로 미연결 (`minutes`, `funding_schedules`, `trades`, `alerts`)

### 1. 현상

라이브를 장시간(예: 30분) 실행해도 아래 테이블이 비어있거나 기대보다 비정상적으로 적게 기록된다.

- `minutes`
- `funding_schedules`
- `trades`
- `alerts`

반면 `sessions`, `positions`, `balance_snapshots`는 기록이 발생한다.

### 2. 원인 분석 (현재 코드 기준)

#### 2.1 `minutes`

- `MinuteRepository.insert_minute()` 및 `DbWriteRequest::InsertMinute`는 존재
- 하지만 producer(`db_writer.send(DbWriteRequest::InsertMinute(...))`)가 없음
- 현재 분봉은 `SessionWriter.append_minute()`로 `minutes.csv` 파일만 기록

#### 2.2 `funding_schedules`

- `FundingRepository.upsert_funding()` 및 `DbWriteRequest::UpsertFunding`는 존재
- producer 없음
- 펀딩비 관련 config/타입은 있으나, 라이브 루프에서 DB upsert를 호출하는 경로가 없음

#### 2.3 `trades`

- `TradeRepository.insert_trade()` 및 `DbWriteRequest::InsertTrade`는 존재
- producer 없음
- 라이브 청산 결과는 메모리/CSV(`append_trade`) 중심으로만 반영

#### 2.4 `alerts`

- DB insert 함수(`AlertRepository.insert_alert`)는 존재
- 텔레그램 fallback closure도 생성됨
- 하지만 운영 경로에서 `AlertService.send(...)` 호출이 연결되지 않아, 실질 DB 적재가 거의 발생하지 않음

### 3. 목표 동작

1. `minutes`: 분 완결마다 코인별 1행 INSERT
2. `funding_schedules`: 주기 갱신 시 코인별 UPSERT
3. `trades`: 포지션 청산(일반/강제/비상) 시 1행 INSERT
4. `alerts`: 핵심 운영 이벤트는 **항상 DB 감사로그 INSERT** (텔레그램 성공/실패와 무관)

### 4. 설계 원칙

1. 기존 비동기 원칙 유지: select! 루프에서 직접 DB write 금지
2. DB write 단일 경로: `DbWriter` 채널을 통한 비동기 전송
3. Sim/LIVE 분리 유지: 시뮬레이션은 기존 CSV 중심 동작 유지 가능
4. 최소 침습 변경: 기존 `monitor_core`/`LivePolicy` 구조를 활용해 producer만 연결
5. idempotency 보장: 재시작/재처리 시 중복 INSERT 방지 규칙 명시
6. 중요도별 backpressure 정책 분리:
   - `trades`/`positions`/`alerts`는 유실 비허용
   - `minutes`/`funding_schedules`는 제한적 유실 허용(카운터/로그 필수)

### 5. 상세 매핑/예외 규칙

#### 5.1 `trades.position_id` 매핑

- `ClosedPosition.id`(메모리 포지션 ID)를 직접 쓰지 않는다.
- DB `trades.position_id`는 `VirtualPosition.db_id`를 사용한다.
- `db_id`가 없는 청산 이벤트는 `trades` INSERT를 skip하고 `warn!` + 카운터 증가.

#### 5.2 `minutes` 중복 방지

- `(session_id, coin, ts)` 기준 중복 방지.
- 마이그레이션으로 unique index 추가 또는 UPSERT/INSERT IGNORE 적용.
- 재시작 복구 구간에서도 같은 분봉이 중복 적재되지 않아야 한다.

#### 5.3 `funding_schedules` 파싱 예외

- 소스: Bybit `get_tickers_linear()`.
- `next_funding_time` 파싱 실패/0이면 해당 코인 upsert skip + `warn!`.
- `funding_rate` 누락 시 0.0 fallback 허용(명시적 로그 필요).

#### 5.4 `alerts` 저장 의미

- 정책 확정: 핵심 운영 이벤트는 항상 DB INSERT.
- 텔레그램은 best-effort 채널이며 실패해도 DB 기록은 보장되어야 한다.
- 기존 fallback-only 동작은 감사 추적 요구사항을 충족하지 못하므로 변경한다.

---

## 구현 플랜 (LD-0002)

### Phase 1: 공통 DB write producer 연결점 추가

- `ExecutionPolicy`에 라이브 전용 DB 이벤트 훅 추가(기본 no-op)
  - 분봉 1건 기록 훅
  - 거래 1건 기록 훅
  - 펀딩 스케줄 기록 훅
- `LivePolicy`에서 해당 훅을 구현해 `DbWriter`로 전송
- `SimPolicy`는 기존 no-op 유지
- `main.rs`에서 `AlertService` 인스턴스를 `_alert_service`로 버리지 않고
  런타임 수명 전체 동안 보관(즉시 drop 방지)

### Phase 2: `minutes` INSERT 구현

- `monitor_core::finalize_and_process()`에서 `MinuteRecord` 생성 직후 policy 훅 호출
- `LivePolicy`는 `DbWriteRequest::InsertMinute` 전송
- 코인별 INSERT 실패는 warn 로그 + 모니터링 카운터 증가
- 중복 방지 적용:
  - schema unique index 또는
  - repository UPSERT/INSERT IGNORE
- 1분당 중복 레코드 발생 시 카운터 증가 + 원인 로그

### Phase 3: `trades` INSERT 구현

- `LivePolicy::record_trade()`에서 **LIVE 파일 출력 없이** `DbWriteRequest::InsertTrade` 전송
- 일반 청산/TTL 청산/강제 청산/비상 청산 경로 모두 동일 함수 사용 보장
- 매핑 규칙 고정:
  - `position_id = VirtualPosition.db_id` (필수)
  - `session_id = 현재 세션 ID`
  - `side`는 `entry/exit/adjustment` 규약으로 고정
  - `adjustment_cost`는 비상/수량 보정 거래에서만 채움
- `db_id` 미존재 시 insert skip + 경고 + 카운터
  (유령 trade row 생성 금지)

### Phase 4: `funding_schedules` UPSERT 구현

- 라이브 minute 타이머 기반으로 펀딩 정보 갱신 task 연결
- Bybit 펀딩 정보 조회 후 `DbWriteRequest::UpsertFunding` 전송
- 코인별 실패는 부분 허용(partial success), 전체 루프는 지속
- 파싱 예외 처리:
  - `next_funding_time == 0` 또는 파싱 실패 시 skip
  - `funding_rate` 누락 시 0 fallback + debug/warn 로그

### Phase 5: `alerts` 실사용 경로 연결

- `AlertService` 인스턴스를 `LivePolicy`/리스크 경로에 주입
- kill switch, leg failure, emergency close failure, reconciliation mismatch 등 핵심 이벤트에서 `send` 호출
- DB 감사로그 always-write 보장:
  - 텔레그램 성공 여부와 무관하게 DB `alerts` INSERT
  - 텔레그램 실패는 추가 경고 이벤트로 처리

### Phase 6: backpressure 정책 정교화

- `DbWriter` 채널 overflow 시 테이블별 정책 적용:
  - `trades`/`alerts`: 드랍 금지(재시도/우선 큐)
  - `minutes`/`funding_schedules`: 드랍 허용 + 카운터/로그
- `final_failure_count`를 테이블별로 분해해 모니터링 가능하게 확장

### Phase 7: 문서/운영 검증

- `0011_LIVE_TRADING`, `0012_BALANCE_SNAPSHOT`, 본 문서(`0013`) 동기화
- 30분 라이브 실행 후 테이블 적재량 검증 쿼리 제공

---

## 체크리스트 (LD-0002)

### A. wiring

- [x] `LivePolicy`에 `DbWriter` 주입
- [x] policy 훅(분봉/거래/펀딩) 추가 및 Live 구현
- [x] SimPolicy 기본 no-op 유지
- [x] `AlertService` 핸들을 main 수명 주기에 보관 (`_alert_service` drop 제거)

### B. minutes

- [x] 분 완결 시 `DbWriteRequest::InsertMinute` 전송
- [x] `minutes` 적재 확인 (코인 수 × 분 수 근사치)
- [x] `(session_id, coin, ts)` 중복 방지(UNIQUE/UPSERT) 적용

### C. trades

- [x] 청산 성공 시 `DbWriteRequest::InsertTrade` 전송
- [x] 일반/TTL/강제/비상 청산 경로 모두 누락 없이 적재 확인
- [x] `position_id = db_id` 매핑 검증 (`ClosedPosition.id` 미사용)
- [x] `db_id` 없는 케이스 skip + 경고/카운터 검증

### D. funding_schedules

- [x] 주기 갱신 task에서 `DbWriteRequest::UpsertFunding` 전송
- [x] `funding_schedules`의 코인별 upsert 동작 확인
- [x] `next_funding_time` parse 실패/0 skip 규칙 검증

### E. alerts

- [x] 주요 이벤트에 `AlertService.send` 연결
- [x] 텔레그램 성공/실패와 무관하게 `alerts` DB 적재 확인
- [x] 텔레그램 실패 시 추가 오류 알림/로그 확인

### F. 테스트/검증

- [x] `cargo test -p arb-strategy`
- [x] `cargo test -p arb-db`
- [x] `cargo clippy`
- [x] 30분 실행 후 적재 검증:
- [x] `SELECT COUNT(*) FROM minutes WHERE session_id = ?`
- [x] `SELECT COUNT(*) FROM trades WHERE session_id = ?`
- [x] `SELECT COUNT(*) FROM funding_schedules`
- [x] `SELECT COUNT(*) FROM alerts WHERE session_id = ?`
- [x] 정량 기준 검증:
- [x] `minutes` >= `active_coin_count * 25` (30분 기준, 시작 워밍업 제외 허용 오차 반영)
- [x] `trades` row 수 == 해당 세션 `ClosedPosition` 수 (또는 skip 사유 카운터와 합 일치)
- [x] `funding_schedules` row 수 >= 활성 코인 수 (최소 1회 upsert 기준)

---

## 완료 기준 (LD-0002)

아래 조건을 모두 만족하면 완료:

1. 30분 라이브 실행 시 `minutes`가 코인 수에 비례해 지속 적재된다 (`active_coin_count * 25` 이상)
2. 청산 발생 시 `trades`가 `position_id=db_id` 규칙으로 누락 없이 적재된다
3. 펀딩 갱신 루프에서 `funding_schedules`가 코인별로 upsert된다 (파싱 실패 코인은 skip 로그 존재)
4. 핵심 운영 이벤트에서 `alerts`가 텔레그램 성공/실패와 무관하게 DB에 기록된다
5. backpressure 정책이 중요 이벤트(`trades`/`alerts`) 유실을 방지한다
6. 관련 테스트/정적검사가 통과한다

---

## LD-0003 — `sync_from_exchange()` 5분 주기 미실행

### 1. 현상

- 라이브 실행 중 `sync_from_exchange()` 주기 로그/동작이 관측되지 않아 내부 예약 잔고와 거래소 실잔고 drift 누적 위험이 존재한다.

### 2. 기대 동작

- `minute_timer`에서 5분마다 별도 `tokio::spawn(sync_from_exchange(...))` 실행
- 주기 허용 오차: **5분 ± 15초**
- 누락 허용: 연속 2회 누락 금지 (1회 누락 시 즉시 warn + 카운터 증가)
- drift 임계치 초과 시 `warn` + 보정
- `InsufficientFunds` 계열 주문 실패 시 즉시 강제 sync

### 3. 체크리스트

- [ ] 5분 주기 spawn 연결 상태 점검/복구
- [ ] 실행 간격 SLA 검증 (5분 ± 15초)
- [ ] 연속 누락 2회 방지 검증 (누락 카운터/알림)
- [ ] drift 보정/로그/카운터 반영 검증
- [ ] 주문 실패 즉시 sync 트리거 검증

---

## LD-0004 — minute timer reconciliation 미실행

### 1. 현상

- 스펙에는 1분(또는 조건부 2분) 주기 reconciliation spawn이 명시되어 있으나 런타임에서 미연결/미실행 정황이 확인됐다.

### 2. 기대 동작

- `minute_timer`에서 reconciliation task를 비동기로 주기 실행
- 실행 주기 규칙: 기본 1분, 열린 포지션 3개 이상이면 2분
- 주기 허용 오차: **±10초**
- 불일치 감지 시 진입 차단/알림/카운터 기록
- 연속 성공 시 차단 자동 해제 규칙 적용

### 3. 체크리스트

- [ ] reconciliation spawn 경로 연결
- [ ] 주기 규칙 검증 (1분 기본, 3개 이상 시 2분)
- [ ] 실행 간격 SLA 검증 (±10초)
- [ ] mismatch 차단 범위(전체/코인 단위) 검증
- [ ] 연속 성공 해제 로직 검증

---

## LD-0005 — `shutdown_policy` runtime 미반영

### 1. 현상

- 설정에 `shutdown_policy = keep|close_all|close_if_profitable`가 존재하지만 종료 시 분기 적용이 누락되어 정책 차등이 동작하지 않는다.

### 2. 기대 동작

- SIGINT/SIGTERM 수신 시 `shutdown_policy` 값별 종료 분기 실행
- 정책별 로그, 세션 상태, 포지션 처리 결과가 명확히 남아야 한다

### 3. 체크리스트

- [ ] 종료 핸들러에서 정책 분기 연결
- [ ] 세 가지 정책별 통합 테스트/수동 검증
- [ ] 세션 종료 reason/status 기록 확인

---

## LD-0006 — 라이브 파일 출력 경로 잔존

### 1. 현상

- 라이브 모드에서 DB 적재와 별개로 `SessionWriter` 기반 CSV/JSON 출력이 남아 있어 `0011`의 DB 우선/파일 I/O 제거 정책과 충돌한다.
- 사용자 의사결정(2026-02-14): **LIVE는 DB-only**, 파일 출력은 시뮬레이션 전용.

### 2. 기대 동작

- 라이브 모드는 `DbSessionWriter`만 사용하고 파일 출력은 시뮬레이션 전용으로 제한
- 라이브 출력 디렉토리 생성/append 경로 제거 또는 feature gate로 차단

### 3. 체크리스트

- [ ] 라이브 진입점에서 FileSessionWriter 경로 제거
- [ ] 시뮬레이션만 파일 출력 유지 검증
- [ ] 문서/운영 가이드 동기화

---

## LD-0007 — 종료 시 `DbWriter` drain/flush 대기 누락

### 1. 현상

- 종료 직전 `DbWriteRequest`가 채널에 남아도 writer task flush 완료를 기다리지 않아 마지막 `balance_snapshots`/`alerts`/`trades` 유실 가능성이 있다.
- 운영 로그에서 `DB writer channel full` 경고가 연속 발생하는 상황과 결합되면 드랍 확률이 커진다.

### 2. 기대 동작

- shutdown 단계에서 producer 중단 -> 채널 drain -> writer join 순서 보장
- 중요 이벤트(`trades`, `alerts`, 최종 snapshot)는 flush 완료 후 종료
- 종료 대기 시간 상한:
  - **soft timeout 10초**: 경고 로그 + flush 진행 상황 출력
  - **hard timeout 30초**: 강제 종료 경로 진입(유실 건수/큐 길이 로그 필수)

### 3. 체크리스트

- [ ] `DbWriter` graceful shutdown 시그널/종료 프로토콜 구현
- [ ] writer task join 대기 추가
- [ ] soft/hard timeout 적용 및 강제 종료 로그 검증
- [ ] 채널 overflow/최종 flush 메트릭 검증
