---
name: upbit-api
description: Upbit 암호화폐 거래소 REST API 및 WebSocket 레퍼런스. 시세 조회(Quotation), 거래(Exchange), 실시간 데이터(WebSocket) API 사용법, 인증 방법, 요청/응답 형식을 포함합니다.
---

# Upbit API Skill

Upbit은 한국 최대 암호화폐 거래소 중 하나로, REST API와 WebSocket을 통해 시세 조회, 거래, 자산 관리 기능을 제공합니다.

## When to Use This Skill

- 암호화폐 시세 조회 (Ticker, 캔들, 호가)
- Upbit 거래소에서 주문 생성/취소
- 계좌 잔고 및 자산 조회
- 실시간 WebSocket 데이터 스트리밍
- 입출금 관리

## Quick Reference

### Base URLs

| 용도 | URL |
|-----|-----|
| REST API | `https://api.upbit.com/v1` |
| WebSocket (공개) | `wss://api.upbit.com/websocket/v1` |
| WebSocket (비공개) | `wss://api.upbit.com/websocket/v1/private` |

### API 카테고리

| 카테고리 | 인증 | 설명 |
|---------|------|------|
| Quotation API | 불필요 | 시세, 호가, 캔들 조회 |
| Exchange API | JWT 필수 | 주문, 자산, 입출금 |
| WebSocket | 공개/비공개 | 실시간 데이터 스트리밍 |

---

## 1. 인증 (Authentication)

### JWT 토큰 생성

```python
import jwt
import uuid
import hashlib
from urllib.parse import urlencode

ACCESS_KEY = "your-access-key"
SECRET_KEY = "your-secret-key"

# 기본 페이로드 (쿼리 파라미터 없는 경우)
payload = {
    "access_key": ACCESS_KEY,
    "nonce": str(uuid.uuid4())
}

# 쿼리 파라미터가 있는 경우
params = {"market": "KRW-BTC", "side": "bid"}
query_string = urlencode(params)
query_hash = hashlib.sha512(query_string.encode()).hexdigest()

payload = {
    "access_key": ACCESS_KEY,
    "nonce": str(uuid.uuid4()),
    "query_hash": query_hash,
    "query_hash_alg": "SHA512"
}

token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")
headers = {"Authorization": f"Bearer {token}"}
```

### API Key 권한 그룹

| 권한 | 설명 |
|-----|------|
| 자산조회 | 계좌 잔고 조회 |
| 주문하기 | 주문 생성/취소/조회 |
| 출금하기 | 디지털 자산 및 KRW 출금 |
| 입금하기 | 입금 주소 생성/조회 |
| 서비스 정보 | API 키 목록, 서비스 상태 |

---

## 2. Quotation API (시세 조회)

### 현재가 조회 (Ticker)

```bash
GET https://api.upbit.com/v1/ticker?markets=KRW-BTC,KRW-ETH
```

**응답 필드:**

| 필드 | 타입 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `trade_price` | number | 현재가 |
| `change` | string | 변화 상태 (RISE/EVEN/FALL) |
| `change_rate` | number | 변화율 |
| `acc_trade_volume_24h` | number | 24시간 누적 거래량 |
| `highest_52_week_price` | number | 52주 최고가 |
| `lowest_52_week_price` | number | 52주 최저가 |

```python
import requests

url = "https://api.upbit.com/v1/ticker"
params = {"markets": "KRW-BTC,KRW-ETH"}
response = requests.get(url, params=params)
print(response.json())
```

### 분봉 캔들 (Minute Candles)

```bash
GET https://api.upbit.com/v1/candles/minutes/{unit}
```

**지원 단위:** 1, 3, 5, 10, 15, 30, 60, 240분

| 파라미터 | 필수 | 설명 |
|---------|------|------|
| `unit` | O | 분 단위 (path) |
| `market` | O | 페어 코드 |
| `to` | X | 종료 시간 (ISO 8601) |
| `count` | X | 개수 (최대 200) |

```python
url = "https://api.upbit.com/v1/candles/minutes/15"
params = {"market": "KRW-BTC", "count": 100}
response = requests.get(url, params=params)
```

### 호가 조회 (Orderbook)

```bash
GET https://api.upbit.com/v1/orderbook?markets=KRW-BTC&count=15
```

**응답:**
```json
{
  "market": "KRW-BTC",
  "timestamp": 1751606867762,
  "total_ask_size": 10.37591054,
  "total_bid_size": 9.49577219,
  "orderbook_units": [
    {"ask_price": 148520000, "bid_price": 148490000, "ask_size": 0.01, "bid_size": 0.04}
  ]
}
```

---

## 3. Exchange API (거래)

### 계좌 잔고 조회

```bash
GET https://api.upbit.com/v1/accounts
```

```python
import jwt, uuid, requests

payload = {"access_key": ACCESS_KEY, "nonce": str(uuid.uuid4())}
token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")
headers = {"Authorization": f"Bearer {token}"}

response = requests.get("https://api.upbit.com/v1/accounts", headers=headers)
```

**응답:**
```json
[
  {
    "currency": "BTC",
    "balance": "0.5",
    "locked": "0.1",
    "avg_buy_price": "50000000",
    "unit_currency": "KRW"
  }
]
```

### 주문 생성

```bash
POST https://api.upbit.com/v1/orders
```

**파라미터:**

| 파라미터 | 필수 | 설명 |
|---------|------|------|
| `market` | O | 페어 코드 (예: KRW-BTC) |
| `side` | O | bid(매수) / ask(매도) |
| `ord_type` | X | limit/price/market/best (기본: limit) |
| `volume` | 조건부 | 주문 수량 |
| `price` | 조건부 | 주문 가격 또는 총액 |
| `time_in_force` | X | ioc/fok/post_only |
| `identifier` | X | 클라이언트 주문 ID |

**주문 유형:**

| 유형 | 설명 | 필수 파라미터 |
|-----|------|-------------|
| `limit` | 지정가 주문 | volume, price |
| `price` | 시장가 매수 | price (총액) |
| `market` | 시장가 매도 | volume |
| `best` | 최유리 지정가 | time_in_force 필수 |

```python
import hashlib
from urllib.parse import urlencode

params = {
    "market": "KRW-BTC",
    "side": "bid",
    "ord_type": "limit",
    "volume": "0.001",
    "price": "50000000"
}

query_string = urlencode(params)
query_hash = hashlib.sha512(query_string.encode()).hexdigest()

payload = {
    "access_key": ACCESS_KEY,
    "nonce": str(uuid.uuid4()),
    "query_hash": query_hash,
    "query_hash_alg": "SHA512"
}

token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")
headers = {"Authorization": f"Bearer {token}"}

response = requests.post(
    "https://api.upbit.com/v1/orders",
    json=params,
    headers=headers
)
```

### 주문 취소

```bash
DELETE https://api.upbit.com/v1/order?uuid={order_uuid}
```

---

## 4. WebSocket API (실시간)

### 연결 설정

```python
import websocket
import json

def on_message(ws, message):
    data = json.loads(message)
    print(data)

def on_open(ws):
    # 구독 메시지 전송
    subscribe = [
        {"ticket": "unique-ticket-id"},
        {"type": "ticker", "codes": ["KRW-BTC", "KRW-ETH"]},
        {"format": "DEFAULT"}
    ]
    ws.send(json.dumps(subscribe))

ws = websocket.WebSocketApp(
    "wss://api.upbit.com/websocket/v1",
    on_message=on_message,
    on_open=on_open
)
ws.run_forever()
```

### 구독 타입

| type | 설명 | 인증 |
|------|------|------|
| `ticker` | 현재가 | 불필요 |
| `trade` | 체결 | 불필요 |
| `orderbook` | 호가 | 불필요 |
| `candle.{unit}` | 캔들 (1, 5, 15, 60, 240, 1D, 1W, 1M) | 불필요 |
| `myOrder` | 내 주문/체결 | JWT 필수 |
| `myAsset` | 내 자산 변동 | JWT 필수 |

### 구독 메시지 구조

```json
[
  {"ticket": "session-unique-id"},
  {
    "type": "ticker",
    "codes": ["KRW-BTC", "KRW-ETH"],
    "is_only_snapshot": false,
    "is_only_realtime": false
  },
  {"format": "DEFAULT"}
]
```

**Format 옵션:**
- `DEFAULT`: 전체 필드명
- `SIMPLE`: 축약 필드명
- `SIMPLE_LIST`: 배열 형태 + 축약

### WebSocket Ticker 응답

```json
{
  "type": "ticker",
  "code": "KRW-BTC",
  "opening_price": 148500000,
  "high_price": 149064000,
  "low_price": 148200000,
  "trade_price": 148956000,
  "prev_closing_price": 148500000,
  "change": "RISE",
  "change_rate": 0.00307,
  "trade_volume": 0.03103806,
  "acc_trade_volume_24h": 2429.58834336,
  "stream_type": "REALTIME",
  "timestamp": 1676965262177
}
```

### 비공개 WebSocket (인증)

```python
import jwt

payload = {
    "access_key": ACCESS_KEY,
    "nonce": str(uuid.uuid4())
}
token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")

ws = websocket.WebSocketApp(
    "wss://api.upbit.com/websocket/v1/private",
    header={"Authorization": f"Bearer {token}"},
    on_message=on_message,
    on_open=on_open
)
```

### 연결 유지 (Keep-Alive)

- 타임아웃: 120초
- PING 프레임 또는 `"PING"` 메시지 전송으로 연결 유지
- 서버 응답: `{"status":"UP"}` (10초 간격)

---

## 5. Rate Limits

### Quotation API (IP 기준)

| 그룹 | 제한 |
|-----|------|
| 시세, 호가, 캔들 | 초당 10회 |

### Exchange API (계정 기준)

| 그룹 | 제한 |
|-----|------|
| 기본 | 초당 30회 |
| 주문 | 초당 8회 |
| 일괄 취소 | 2초당 1회 |

### WebSocket

| 유형 | 제한 |
|-----|------|
| 연결 | 초당 5회 |
| 메시지 | 초당 5회, 분당 100회 |

### Rate Limit 확인

응답 헤더 `Remaining-Req`에서 확인:
```
Remaining-Req: group=default; min=1800; sec=29
```
- `sec`: 현재 잔여 요청 수

### 에러 코드

| 코드 | 설명 |
|-----|------|
| 429 | 요청 한도 초과 |
| 418 | IP/계정 일시 차단 |

---

## 6. 주요 에러 코드

| 에러명 | HTTP | 설명 |
|-------|------|------|
| `invalid_parameter` | 400 | 잘못된 파라미터 |
| `insufficient_funds_bid` | 400 | 매수 잔액 부족 |
| `insufficient_funds_ask` | 400 | 매도 수량 부족 |
| `invalid_price_bid` | 400 | 주문 가격 단위 오류 |
| `notfoundmarket` | 400 | 존재하지 않는 페어 |
| `market_offline` | 403 | 시스템 점검 중 |
| `INVALID_AUTH` | 401 | WebSocket 인증 실패 |

---

## 7. 완전한 예제

### Python: 실시간 시세 + 자동 주문

```python
import websocket
import jwt
import uuid
import hashlib
import requests
import json
from urllib.parse import urlencode

ACCESS_KEY = "your-access-key"
SECRET_KEY = "your-secret-key"
BASE_URL = "https://api.upbit.com/v1"

def create_order(market, side, volume, price):
    """지정가 주문 생성"""
    params = {
        "market": market,
        "side": side,
        "ord_type": "limit",
        "volume": str(volume),
        "price": str(price)
    }

    query_string = urlencode(params)
    query_hash = hashlib.sha512(query_string.encode()).hexdigest()

    payload = {
        "access_key": ACCESS_KEY,
        "nonce": str(uuid.uuid4()),
        "query_hash": query_hash,
        "query_hash_alg": "SHA512"
    }

    token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")
    headers = {"Authorization": f"Bearer {token}"}

    response = requests.post(f"{BASE_URL}/orders", json=params, headers=headers)
    return response.json()

def on_message(ws, message):
    """WebSocket 메시지 핸들러"""
    data = json.loads(message)
    if data.get("type") == "ticker":
        print(f"[{data['code']}] {data['trade_price']:,}원 ({data['change']})")

def on_open(ws):
    """WebSocket 연결 시 구독"""
    subscribe = [
        {"ticket": f"ticker-{uuid.uuid4()}"},
        {"type": "ticker", "codes": ["KRW-BTC", "KRW-ETH"]},
        {"format": "DEFAULT"}
    ]
    ws.send(json.dumps(subscribe))
    print("WebSocket 연결됨")

# WebSocket 실행
ws = websocket.WebSocketApp(
    "wss://api.upbit.com/websocket/v1",
    on_message=on_message,
    on_open=on_open
)
ws.run_forever()
```

---

## Reference Files

더 자세한 정보는 `references/` 디렉토리를 참조하세요:

- **quotation.md** - Quotation API 상세
- **exchange_order.md** - 주문 관련 API
- **exchange_withdrawal.md** - 출금 API
- **exchange_deposit.md** - 입금 API
- **websocket.md** - WebSocket 상세

## Resources

- [Upbit Developer Center](https://docs.upbit.com)
- [API Key 관리](https://upbit.com/mypage/open_api_management)
