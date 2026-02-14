# Logging Skill

이 문서는 `arb_poc` 프로젝트의 로깅 규칙 및 패턴을 정의합니다.

## 개요

이 프로젝트는 `tracing` 크레이트를 사용하여 구조화된 로깅을 구현합니다. 모든 로깅은 `src/logging/mod.rs` 모듈을 통해 수행됩니다.

## 로깅 초기화

애플리케이션 시작 시 로깅 시스템을 초기화해야 합니다.

```rust
use arb_poc::logging::{init_logging, LogConfig};

fn main() {
    // 환경 변수에서 설정 로드
    let config = LogConfig::from_env();
    init_logging(&config).expect("로깅 초기화 실패");

    // 또는 개발 환경용 설정
    let config = LogConfig::development();
    init_logging(&config).expect("로깅 초기화 실패");
}
```

## 로그 레벨 사용 지침

| 레벨 | 용도 | 예시 |
|------|------|------|
| `error` | 복구 불가능한 에러, 시스템 장애 | API 인증 실패, 치명적 오류 |
| `warn` | 복구 가능한 문제, 주의 필요 사항 | WebSocket 연결 해제, 재시도 |
| `info` | 중요한 비즈니스 이벤트 | 주문 생성/체결, 시스템 시작/종료, 차익거래 기회 발견 |
| `debug` | 개발/디버깅용 상세 정보 | API 요청/응답, 가격 업데이트 |
| `trace` | 매우 상세한 추적 정보 | 호가창 변경, 차익거래 기회 없음 |

### 레벨별 사용 예시

```rust
use tracing::{trace, debug, info, warn, error};

// error: 치명적 오류
error!(exchange = %exchange, error = %e, "API 인증 실패");

// warn: 복구 가능한 문제
warn!(exchange = %exchange, "WebSocket 연결 해제, 재연결 시도 중");

// info: 중요 비즈니스 이벤트
info!(order_id = %id, symbol = %symbol, "주문 체결 완료");

// debug: 개발용 상세 정보
debug!(endpoint = %url, latency_ms = %ms, "API 응답 수신");

// trace: 매우 상세한 추적
trace!(bid = %bid, ask = %ask, "호가 업데이트");
```

## 구조화된 로깅 필드 규칙

### 필수 필드

거래 관련 로그에는 다음 필드를 포함해야 합니다:

| 필드 | 타입 | 설명 |
|------|------|------|
| `exchange` | String | 거래소 이름 (예: "upbit", "binance") |
| `symbol` | String | 거래 심볼 (예: "KRW-BTC", "BTCUSDT") |

### 주문 관련 필드

| 필드 | 타입 | 설명 |
|------|------|------|
| `order_id` | String | 주문 ID |
| `side` | String | "buy" 또는 "sell" |
| `order_type` | String | "limit", "market" 등 |
| `price` | Decimal | 주문 가격 |
| `quantity` | Decimal | 주문 수량 |
| `status` | String | 주문 상태 |

### API 관련 필드

| 필드 | 타입 | 설명 |
|------|------|------|
| `endpoint` | String | API 엔드포인트 |
| `method` | String | HTTP 메서드 |
| `status` | u16 | HTTP 상태 코드 |
| `latency_ms` | u64 | 응답 지연 시간 |

### 차익거래 관련 필드

| 필드 | 타입 | 설명 |
|------|------|------|
| `buy_exchange` | String | 매수 거래소 |
| `sell_exchange` | String | 매도 거래소 |
| `buy_price` | Decimal | 매수 가격 |
| `sell_price` | Decimal | 매도 가격 |
| `profit_bps` | Decimal | 예상 수익률 (basis points) |

## Span 사용법

span은 관련된 로그를 그룹화하고 컨텍스트를 제공합니다.

### 거래소별 Span

```rust
use arb_poc::logging::exchange_span;

// 거래소별 작업 추적
{
    let _guard = exchange_span("upbit", Some("KRW-BTC"));
    // 이 스코프 내의 모든 로그는 exchange=upbit, symbol=KRW-BTC를 포함
    tracing::info!("시세 조회 시작");
    // ... 작업 수행 ...
    tracing::info!("시세 조회 완료");
} // span 자동 종료
```

### 작업별 Span

```rust
use arb_poc::logging::operation_span;

// 특정 작업 추적
{
    let _guard = operation_span("fetch_orderbook", "binance", Some("BTCUSDT"));
    // 호가창 조회 작업 수행
}
```

### 수동 Span 생성

```rust
use tracing::{info_span, instrument};

// 함수에 span 적용
#[instrument(skip(client), fields(exchange = "upbit"))]
async fn fetch_ticker(client: &UpbitClient, symbol: &str) -> Result<Ticker, Error> {
    // 함수 내 모든 로그에 자동으로 컨텍스트 추가
    tracing::debug!("티커 조회 중");
    // ...
}

// 수동 span 생성
let span = info_span!("arbitrage_check", pair = %symbol);
let _enter = span.enter();
```

## 제공되는 로깅 헬퍼 함수

`arb_poc::logging` 모듈에서 제공하는 헬퍼 함수들입니다:

### 주문 이벤트

```rust
use arb_poc::logging::log_order_event;

log_order_event("주문 생성", &order);
log_order_event("주문 체결", &order);
log_order_event("주문 취소", &order);
```

### 거래 체결

```rust
use arb_poc::logging::log_trade_execution;

log_trade_execution("upbit", "KRW-BTC", "buy", price, quantity, &order_id);
```

### 가격 업데이트

```rust
use arb_poc::logging::log_price_update;

log_price_update("binance", "BTCUSDT", bid, ask, Some(last_price));
```

### 차익거래 기회

```rust
use arb_poc::logging::log_arbitrage_opportunity;

log_arbitrage_opportunity(
    "upbit",      // 매수 거래소
    "binance",    // 매도 거래소
    "BTC",        // 심볼
    buy_price,
    sell_price,
    profit_bps,   // 수익률 (basis points)
);
```

### API 요청/응답

```rust
use arb_poc::logging::{log_api_request, log_api_response, log_api_error};

log_api_request("upbit", "/v1/ticker", "GET");
log_api_response("upbit", "/v1/ticker", 200, 45);
log_api_error("upbit", "/v1/orders", "인증 실패");
```

### WebSocket 상태

```rust
use arb_poc::logging::log_websocket_status;

log_websocket_status("upbit", "connected", Some("wss://api.upbit.com/websocket/v1"));
log_websocket_status("upbit", "disconnected", None);
log_websocket_status("upbit", "reconnecting", None);
```

### 시스템 이벤트

```rust
use arb_poc::logging::{log_system_start, log_system_shutdown};

log_system_start("0.1.0", &["upbit", "binance", "bithumb"]);
log_system_shutdown("사용자 종료 요청");
```

## 환경 변수 설정

| 환경 변수 | 설명 | 기본값 |
|-----------|------|--------|
| `RUST_LOG` | 로그 레벨 | `info` |
| `LOG_FILE` | 로그 파일 경로 (설정 시 파일 로깅 활성화) | 없음 |
| `LOG_CONSOLE` | 콘솔 출력 활성화 | `true` |
| `LOG_SHOW_TARGET` | 대상(모듈 경로) 표시 | `true` |
| `LOG_SHOW_FILE_LINE` | 파일/라인 번호 표시 | `false` |
| `LOG_SHOW_SPAN_EVENTS` | span 이벤트 표시 | `false` |

### 로그 레벨 세분화

`RUST_LOG` 환경 변수로 모듈별 로그 레벨을 설정할 수 있습니다:

```bash
# 전체 info, 특정 모듈만 debug
export RUST_LOG="info,arb_poc::exchanges=debug"

# 전체 warn, arb_poc만 info
export RUST_LOG="warn,arb_poc=info"

# 매우 상세한 로깅
export RUST_LOG="arb_poc=trace"
```

## 성능 고려사항

### 로깅 오버헤드 최소화

1. **핫 패스에서 trace/debug 레벨 사용**: 프로덕션에서 비활성화됨
2. **비용이 큰 계산 지연**: 로그 레벨 체크 후 계산

```rust
// 좋은 예: 레벨 체크 후 계산
if tracing::enabled!(tracing::Level::DEBUG) {
    let expensive_value = compute_expensive_value();
    debug!(value = %expensive_value, "계산 결과");
}

// 나쁜 예: 항상 계산
debug!(value = %compute_expensive_value(), "계산 결과");
```

3. **String 할당 최소화**: `%`(Display) 대신 `?`(Debug) 사용 고려

```rust
// String 할당 없음
debug!(status = ?order.status, "상태 확인");

// String 할당 발생
debug!(status = %order.status.to_string(), "상태 확인");
```

## 모범 사례

### DO

- 구조화된 필드 사용 (`info!(field = %value, "message")`)
- 일관된 필드 이름 사용
- 적절한 로그 레벨 선택
- span으로 관련 로그 그룹화
- 헬퍼 함수 활용

### DON'T

- 민감 정보 로깅 (API 키, 비밀번호 등)
- 핫 패스에서 과도한 로깅
- 문자열 포맷팅으로 로그 메시지 생성 (`format!()` 사용 금지)
- `.unwrap()` 사용 시 panic 로그 없이 종료

## 테스트에서의 로깅

테스트에서는 `tracing-test` 크레이트를 사용하거나, 로깅을 초기화하지 않고 실행할 수 있습니다. 로깅 시스템이 초기화되지 않으면 로그는 자동으로 무시됩니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // 로깅 초기화 없이 테스트 가능
        // 로그 호출은 무시됨
    }
}
```
