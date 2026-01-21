# Comment Style Guide (주석 스타일 가이드)

이 프로젝트의 주석 작성 규칙을 정의합니다.

## 기본 원칙

- **주석 언어**: 한글로 작성
- **기술 용어**: 영어로 유지 (trait, API, JWT, HMAC, async, await, Result, Option 등)
- **에러 메시지**: 코드 내 에러 메시지 문자열은 영어로 유지 (국제화 고려)

## 주석 유형

### 1. 모듈 레벨 문서 주석 (`//!`)

파일 최상단에 모듈 전체를 설명합니다.

```rust
//! Binance 거래소 API 클라이언트
//!
//! 이 모듈은 Binance REST API와 WebSocket 연결을 제공합니다.
//!
//! # 기능
//!
//! - 시세 조회 (ticker, orderbook)
//! - 주문 실행 (limit, market)
//! - 계정 정보 조회
```

### 2. API 문서 주석 (`///`)

public 함수, struct, enum, trait 등에 사용합니다.

```rust
/// 지정된 심볼의 현재 시세를 조회합니다.
///
/// # 인자
///
/// * `symbol` - 거래 심볼 (예: "BTCUSDT")
///
/// # 반환값
///
/// 성공 시 `Ticker` 구조체를 반환합니다.
///
/// # 에러
///
/// * `ExchangeError::InvalidSymbol` - 잘못된 심볼인 경우
/// * `ExchangeError::NetworkError` - 네트워크 연결 실패 시
///
/// # 예제
///
/// ```rust
/// let ticker = client.get_ticker("BTCUSDT").await?;
/// println!("현재가: {}", ticker.price);
/// ```
pub async fn get_ticker(&self, symbol: &str) -> Result<Ticker, ExchangeError> {
    // ...
}
```

### 3. 일반 주석 (`//`)

코드 블록이나 복잡한 로직을 설명합니다.

```rust
// HMAC-SHA256 서명 생성
let signature = hmac_sha256(&secret_key, &query_string);

// Rate limit 초과 방지를 위한 지연
tokio::time::sleep(Duration::from_millis(100)).await;
```

## 섹션 헤더 번역 규칙

문서 주석 내 섹션 헤더는 다음과 같이 번역합니다:

| 영어 | 한글 |
|------|------|
| `# Arguments` | `# 인자` |
| `# Returns` | `# 반환값` |
| `# Errors` | `# 에러` |
| `# Example` / `# Examples` | `# 예제` |
| `# Panics` | `# 패닉` |
| `# Safety` | `# 안전성` |
| `# Features` | `# 기능` |
| `# Notes` | `# 참고` |
| `# See Also` | `# 관련 항목` |

## 기술 용어 영어 유지 목록

다음 용어들은 번역하지 않고 영어로 유지합니다:

### Rust 키워드 및 타입
- trait, struct, enum, impl, async, await, unsafe
- Result, Option, Vec, HashMap, Arc, Mutex
- lifetime, borrow, ownership, reference

### 암호화/보안
- HMAC, SHA256, JWT, API key, secret key, signature, nonce

### 네트워크/프로토콜
- REST API, WebSocket, HTTP, HTTPS, endpoint, request, response

### 거래소/금융
- ticker, orderbook, bid, ask, spread, latency
- market order, limit order, stop-loss

### 일반 개발
- callback, handler, client, server, cache, timeout

## 에러 메시지 규칙

코드 내 에러 메시지는 국제화를 고려하여 영어로 작성합니다:

```rust
// 올바른 예
return Err(ExchangeError::InvalidSymbol("Invalid trading symbol".into()));

// 피해야 할 예
return Err(ExchangeError::InvalidSymbol("잘못된 거래 심볼".into()));
```

## 예시: 완전한 모듈

```rust
//! Binance 거래소 클라이언트 모듈
//!
//! Binance REST API를 통해 시세 조회 및 주문을 수행합니다.
//!
//! # 예제
//!
//! ```rust
//! use arb_poc::exchanges::binance::BinanceClient;
//!
//! let client = BinanceClient::new(&config)?;
//! let ticker = client.get_ticker("BTCUSDT").await?;
//! ```

use crate::exchange::{Exchange, ExchangeError, Ticker};

/// Binance REST API 클라이언트
///
/// HTTP 요청을 통해 Binance 거래소와 통신합니다.
/// HMAC-SHA256 인증을 지원합니다.
pub struct BinanceClient {
    /// API endpoint URL
    base_url: String,
    /// 인증용 API key
    api_key: String,
    /// 서명 생성용 secret key
    secret_key: String,
}

impl BinanceClient {
    /// 새로운 Binance 클라이언트를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `config` - Binance API 설정 정보
    ///
    /// # 반환값
    ///
    /// 초기화된 `BinanceClient` 인스턴스
    ///
    /// # 에러
    ///
    /// * `ExchangeError::InvalidConfig` - 설정 값이 유효하지 않은 경우
    pub fn new(config: &BinanceConfig) -> Result<Self, ExchangeError> {
        // API key 유효성 검증
        if config.api_key.is_empty() {
            return Err(ExchangeError::InvalidConfig("API key is required".into()));
        }

        Ok(Self {
            base_url: config.base_url.clone(),
            api_key: config.api_key.clone(),
            secret_key: config.secret_key.clone(),
        })
    }
}
```
