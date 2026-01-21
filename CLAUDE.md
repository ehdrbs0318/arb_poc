# arb_poc

Rust 기반 Arbitrage(차익거래) PoC(Proof of Concept) 프로젝트

## 프로젝트 개요

이 프로젝트는 **여러 암호화폐 거래소 간의 차익거래** 기회를 탐지하고 실행하는 시스템의 프로토타입입니다.

## 아키텍처

### 핵심 설계 원칙

이 프로젝트는 **모듈 구조를 적극적으로 활용**합니다:

```
┌─────────────────────────────────────────────────────────┐
│                    Strategy Layer                        │
│              (추상화된 Exchange API 사용)                 │
├─────────────────────────────────────────────────────────┤
│                 Exchange Abstraction                     │
│    (trait MarketData, trait OrderManagement, etc.)       │
├────────────────┬────────────────┬───────────────────────┤
│     Upbit      │    Bithumb     │       Bybit           │
│      SDK       │      SDK       │        SDK            │
│   (한국/KRW)    │   (한국/KRW)    │    (글로벌/USDT)      │
└────────────────┴────────────────┴───────────────────────┘
```

### 레이어 설명

1. **Strategy Layer (전략 계층)**
   - 차익거래 로직 구현
   - **추상화된 거래소 API만 사용** (구체적인 거래소 구현에 의존하지 않음)
   - 거래소 간 가격 비교, 주문 실행 결정

2. **Exchange Abstraction (거래소 추상화 계층)**
   - 모든 거래소가 구현해야 하는 공통 trait 정의
   - `Exchange`, `OrderBook`, `Account`, `WebSocketStream` 등
   - 전략 코드와 거래소 구현 사이의 계약(contract) 역할

3. **Exchange SDK (거래소 SDK 계층)**
   - 각 거래소별 독립적인 모듈
   - **SDK 형태로 전략 코드에서 사용됨**
   - 거래소 API 호출, 인증, 웹소켓 연결 등 구현
   - 새 거래소 추가 시 이 계층에 모듈만 추가하면 됨

### 모듈 구조

```
src/
├── main.rs                # 메인 진입점
├── lib.rs                 # 라이브러리 루트
├── config/                # 설정 관리
│   └── mod.rs
├── logging/               # 로깅 모듈
│   └── mod.rs
├── exchange/              # 거래소 추상화 계층
│   ├── mod.rs
│   ├── traits.rs          # MarketData, OrderManagement trait 정의
│   ├── types.rs           # 공통 타입 (Order, Ticker, OrderBook, etc.)
│   ├── error.rs           # 거래소 관련 에러 타입
│   ├── adapter.rs         # 객체 안전 어댑터 trait
│   ├── factory.rs         # 거래소 팩토리 함수
│   ├── manager.rs         # ExchangeManager (다중 거래소 관리)
│   └── market.rs          # 마켓 코드 변환 유틸리티
├── exchanges/             # 각 거래소 SDK 구현
│   ├── mod.rs
│   ├── upbit/             # Upbit 거래소
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── types.rs
│   │   └── auth.rs
│   ├── bithumb/           # Bithumb 거래소
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── types.rs
│   │   └── auth.rs
│   └── bybit/             # Bybit 거래소
│       ├── mod.rs
│       ├── client.rs
│       ├── types.rs
│       └── auth.rs
└── strategy/              # 전략 계층 (TODO)
    └── ...
```

## 설정 및 보안

### 민감 정보 관리

**중요**: API 키, 시크릿 등 민감한 정보는 **절대 Git에 커밋하지 않습니다**.

```
# .gitignore에 반드시 포함
config.toml
config.local.toml
.env
*.pem
*.key
secrets/
```

### 설정 파일 구조

```toml
# config.example.toml (Git에 포함 - 템플릿)
[binance]
api_key = "YOUR_API_KEY_HERE"
secret_key = "YOUR_SECRET_KEY_HERE"

[upbit]
api_key = "YOUR_API_KEY_HERE"
secret_key = "YOUR_SECRET_KEY_HERE"

# config.toml (Git에서 제외 - 실제 값)
[binance]
api_key = "실제_API_키"
secret_key = "실제_시크릿_키"
```

### 설정 로딩 우선순위

1. 환경 변수 (`BINANCE_API_KEY` 등)
2. `config.local.toml` (로컬 개발용)
3. `config.toml` (기본 설정)
4. `config.example.toml` (폴백, 개발 모드에서만)

## 기술 스택

- **언어**: Rust (Edition 2024)
- **빌드 시스템**: Cargo
- **IDE**: RustRover

## 프로젝트 구조

### 프로젝트 구조

```
arb_poc/
├── src/                      # 소스 코드
│   ├── main.rs               # 메인 진입점
│   ├── lib.rs                # 라이브러리 루트
│   ├── config/               # 설정 관리
│   ├── logging/              # 로깅 모듈
│   ├── exchange/             # 거래소 추상화 계층
│   └── exchanges/            # 거래소별 SDK 구현
│       ├── upbit/
│       ├── bithumb/
│       └── bybit/
├── examples/                 # 사용 예제
│   ├── upbit_public.rs
│   ├── upbit_private.rs
│   ├── bithumb_public.rs
│   ├── bithumb_private.rs
│   ├── bybit_public.rs
│   ├── bybit_private.rs
│   └── exchange_manager.rs
├── Cargo.toml                # 프로젝트 설정 및 의존성
├── Cargo.lock                # 의존성 잠금 파일
├── config.example.toml       # 설정 템플릿 (Git 포함)
├── config.toml               # 실제 설정 (Git 제외)
└── .claude/
    ├── agents/               # 커스텀 에이전트 정의
    │   ├── coder.md          # Rust 코드 작성 전문가
    │   ├── trader.md         # 금융공학/퀀트 전문가
    │   ├── reviewer.md       # 코드 리뷰 전문가
    │   └── supervisor.md     # 작업 조율 담당
    └── skills/               # 스킬 정의
        ├── comment.md        # 주석 스타일 가이드
        ├── logging.md        # 로깅 스타일 가이드
        ├── upbit-api/        # Upbit API 레퍼런스
        ├── bithumb-api/      # Bithumb API 레퍼런스
        └── bybit-v5-api/     # Bybit V5 API 레퍼런스
```

## 빌드 및 실행 명령어

```bash
# 빌드
cargo build

# 릴리즈 빌드
cargo build --release

# 실행
cargo run

# 테스트
cargo test

# 코드 포맷팅
cargo fmt

# 린트 검사
cargo clippy

# 문서 생성
cargo doc --open
```

## 개발 규칙

### 코드 스타일

- `cargo fmt`로 코드 포맷팅 유지
- `cargo clippy` 경고 없이 통과해야 함
- 함수/변수: `snake_case`
- 타입/트레이트: `CamelCase`
- 상수: `SCREAMING_SNAKE_CASE`

### 에러 처리

- 프로덕션 코드에서 `.unwrap()` 사용 금지
- `Result<T, E>`와 `?` 연산자 활용
- 커스텀 에러 타입은 `thiserror` 사용 권장

### 문서화

- 공개 API에는 `///` 주석 필수
- 모듈 레벨 문서는 `//!` 사용
- **코드 주석은 한글로 작성** (`.claude/skills/comment.md` 참조)
- 기술 용어(trait, API, JWT 등)는 영어로 유지
- 에러 메시지 문자열은 영어로 유지 (국제화 고려)

### Git 커밋

- 커밋 메시지는 **한국어**로 작성
- 의미 있는 단위로 커밋

## 에이전트 협업 가이드

### 필수 규칙: Supervisor 중심 워크플로우

> **모든 사용자 명령은 반드시 `supervisor` 에이전트가 위임받습니다.**
> `supervisor`가 작업을 분석하고, 필요한 다른 에이전트를 호출하는 방식으로 진행합니다.

```
┌──────────┐
│   User   │
└────┬─────┘
     │ 모든 명령
     ▼
┌──────────────┐
│  supervisor  │ ◄── 작업 분석, 계획 수립, 품질 관리
└──────┬───────┘
       │ 위임
       ▼
┌──────┴───────┬────────────┬────────────┐
│    coder     │   trader   │  reviewer  │
│  (코드 작성)  │ (금융 로직) │ (코드 리뷰) │
└──────────────┴────────────┴────────────┘
```

### 에이전트 역할

| 에이전트 | 역할 | 호출 조건 |
|---------|------|----------|
| `supervisor` | **총괄 지휘** - 작업 분해, 에이전트 조율, 품질 관리 | **모든 사용자 요청의 진입점** |
| `coder` | Rust 코드 작성, 디버깅, 최적화 | supervisor가 코딩 작업 위임 시 |
| `trader` | 금융공학, 퀀트 전략, 차익거래 로직 | supervisor가 금융 로직 설계 위임 시 |
| `reviewer` | 코드 리뷰, 안전성/성능 검증 | supervisor가 코드 검토 위임 시 |

### Supervisor의 책임

1. **명령 해석**: 사용자 요청을 분석하여 필요한 작업 식별
2. **작업 분해**: 복잡한 요청을 세부 작업으로 분해
3. **에이전트 선택**: 각 작업에 적합한 에이전트 결정
4. **순서 조율**: 작업 간 의존성을 고려한 실행 순서 결정
5. **품질 보증**: 각 에이전트 결과물 검토 및 통합
6. **사용자 보고**: 최종 결과 종합하여 보고

### 작업 흐름 예시

```
사용자: "Binance 거래소 SDK 모듈을 구현해줘"

supervisor 처리 과정:
1. 요청 분석 → SDK 구현 = 금융 API 이해 + Rust 코딩
2. trader 호출 → Binance API 구조, 필요 기능 정의
3. coder 호출 → trader의 설계를 바탕으로 Rust 코드 구현
4. reviewer 호출 → 구현된 코드 리뷰
5. 통합 및 보고 → 사용자에게 결과 전달
```

### 금지 사항

- 사용자 명령을 supervisor를 거치지 않고 다른 에이전트에게 직접 전달하지 않음
- supervisor 없이 에이전트 간 직접 통신하지 않음
- 단순한 작업이라도 supervisor가 판단하여 위임함

### 도메인 특화 지식

이 프로젝트는 금융 도메인 지식이 필요합니다:

- **Arbitrage**: 동일 자산의 가격 차이를 이용한 무위험 수익 전략
- **Market Making**: 매수/매도 호가 제시로 유동성 공급
- **Order Book**: 매수/매도 주문 대기열
- **Latency**: 주문 실행 지연 시간 (최소화 필수)

## 성능 고려사항

차익거래 시스템은 저지연(low-latency)이 핵심입니다:

- 불필요한 힙 할당 최소화
- `clone()` 사용 지양
- 핫 패스(hot path)에서 락(lock) 회피
- 제로 카피(zero-copy) 패턴 활용
- SIMD 최적화 고려

## 의존성 추가 시 주의사항

새 크레이트 추가 전 검토:
- 컴파일 시간 영향
- 런타임 성능 오버헤드
- 유지보수 상태 및 보안 이력
- 라이선스 호환성

## 스킬 참조

코드 작성 시 다음 스킬 문서를 참조하세요:

| 스킬 | 경로 | 설명 |
|------|------|------|
| 주석 스타일 | `.claude/skills/comment.md` | 한글 주석 작성 규칙 |
| 로깅 | `.claude/skills/logging.md` | tracing 기반 로깅 패턴 |
| Upbit API | `.claude/skills/upbit-api/` | Upbit 거래소 API 레퍼런스 |
| Bithumb API | `.claude/skills/bithumb-api/` | Bithumb 거래소 API 레퍼런스 |
| Bybit API | `.claude/skills/bybit-v5-api/` | Bybit V5 API 레퍼런스 |

## 로깅

이 프로젝트는 `tracing` 크레이트를 사용합니다. 로깅 초기화 및 사용법은 `.claude/skills/logging.md`를 참조하세요.

### 로그 레벨

| 레벨 | 용도 |
|------|------|
| `error` | 치명적 오류, 복구 불가능 |
| `warn` | 복구 가능한 문제 |
| `info` | 중요 비즈니스 이벤트 (주문, 체결 등) |
| `debug` | 개발/디버깅용 상세 정보 |
| `trace` | 매우 상세한 추적 정보 |

### 로깅 초기화 예시

```rust
use arb_poc::logging::{init_logging, LogConfig};

let config = LogConfig::from_env();
init_logging(&config).expect("로깅 초기화 실패");
```
