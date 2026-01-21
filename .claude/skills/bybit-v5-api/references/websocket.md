# Bybit-V5-Api - Websocket

**Pages:** 7

---

## Greek

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/greek

**Contents:**
- Greek
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

WebSocket Stream
Private
Greek
On this page
Greek
Subscribe to the greeks stream to see changes to your greeks data in
real-time
.
option
only.
Topic:
greeks
Response Parameters
​
Parameter
Type
Comments
id
string
Message ID
topic
string
Topic name
creationTime
number
Data created timestamp (ms)
data
array
Object
> baseCoin
string
Base coin
> totalDelta
string
Delta value
> totalGamma
string
Gamma value
> totalVega
string
Vega value
> totalTheta
string
Theta value
Subscribe Example
​
{
"op"
:
"subscribe"
,
"args"
:
[
"greeks"
]
}
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"private"
,
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
greek_stream
(
callback
=
handle_message
)
while
True
:
sleep
(
1
)
Stream Example
​
{
"id"
:
"592324fa945a30-2603-49a5-b865-21668c29f2a6"
,
"topic"
:
"greeks"
,
"creationTime"
:
1672364262482
,
"data"
:
[
{
"baseCoin"
:
"ETH"
,
"totalDelta"
:
"0.06999986"
,
"totalGamma"
:
"-0.00000001"
,
"totalVega"
:
"-0.00000024"
,
"totalTheta"
:
"0.00001314"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "greeks"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)def handle_message(message):    print(message)ws.greek_stream(callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "id": "592324fa945a30-2603-49a5-b865-21668c29f2a6",    "topic": "greeks",    "creationTime": 1672364262482,    "data": [        {            "baseCoin": "ETH",            "totalDelta": "0.06999986",            "totalGamma": "-0.00000001",            "totalVega": "-0.00000024",            "totalTheta": "0.00001314"        }    ]}
```

---

## Connect

**URL:** https://bybit-exchange.github.io/docs/v5/ws/connect

**Contents:**
- Connect
- Authentication​
- IP Limits​
- Public channel - Args limits​
- How to Send the Heartbeat Packet​
- How to Subscribe to Topics​
  - Understanding WebSocket Filters​
  - Understanding WebSocket Filters: Unsubscription​
- Understanding the Subscription Response​

Spread Trading
Websocket Stream
Connect
On this page
Connect
WebSocket public stream
:
Mainnet:
Spot:
wss://stream.bybit.com/v5/public/spot
USDT, USDC perpetual & USDT Futures:
wss://stream.bybit.com/v5/public/linear
Inverse contract:
wss://stream.bybit.com/v5/public/inverse
Spread trading:
wss://stream.bybit.com/v5/public/spread
USDT/USDC Options:
wss://stream.bybit.com/v5/public/option
Testnet:
Spot:
wss://stream-testnet.bybit.com/v5/public/spot
USDT,USDC perpetual & USDT Futures:
wss://stream-testnet.bybit.com/v5/public/linear
Inverse contract:
wss://stream-testnet.bybit.com/v5/public/inverse
Spread trading:
wss://stream-testnet.bybit.com/v5/public/spread
USDT/USDC Options:
wss://stream-testnet.bybit.com/v5/public/option
WebSocket private stream
:
Mainnet:
wss://stream.bybit.com/v5/private
Testnet:
wss://stream-testnet.bybit.com/v5/private
WebSocket Order Entry
:
Mainnet:
wss://stream.bybit.com/v5/trade
(Spread trading is not supported)
Testnet:
wss://stream-testnet.bybit.com/v5/trade
(Spread trading is not supported)
WebSocket GET System Status
:
Mainnet:
wss://stream.bybit.com/v5/public/misc/status
Testnet:
wss://stream-testnet.bybit.com/v5/public/misc/status
info
If your account is registered from
www.bybit-tr.com
, please use
stream.bybit-tr.com
for mainnet access
If your account is registered from
www.bybit.kz
, please use
stream.bybit.kz
for mainnet access
If your account is registered from
www.bybitgeorgia.ge
, please use
stream.bybitgeorgia.ge
for mainnet access
Customise Private Connection Alive Time
For private stream and order entry, you can customise alive duration by adding a param
max_active_time
, the lowest value is
30s
(30 seconds), the highest value is
600s
(10 minutes). You can also pass
1m
,
2m
etc when you try to configure by minute level. e.g.,
wss://stream-testnet.bybit.com/v5/private?max_active_time=1m
.
In general, if there is no "ping-pong" and no stream data sent from server end, the connection will be cut off after 10 minutes. When you have a particular need, you can configure connection alive time by
max_active_time
.
Since ticker scans every 30s, so it is not fully exact, i.e., if you configure 45s, and your last update or ping-pong is occurred on
2023-08-15 17:27:23
, your disconnection time maybe happened on
2023-08-15 17:28:15
Authentication
​
info
Public
topics do not require authentication. The following section applies to
private
topics only.
Apply for authentication when establishing a connection.
Note: if you're using
pybit
,
bybit-api
, or another high-level library, you can ignore this code - as authentication is handled for you.
{
"req_id"
:
"10001"
,
// optional
"op"
:
"auth"
,
"args"
:
[
"api_key"
,
1662350400000
,
// expires; is greater than your current timestamp
"signature"
]
}
# based on: https://github.com/bybit-exchange/pybit/blob/master/pybit/_http_manager.py
import
hmac
import
json
import
time
import
websocket
api_key
=
""
api_secret
=
""
# Generate expires.
expires
=
int
(
(
time
.
time
(
)
+
1
)
*
1000
)
# Generate signature.
signature
=
str
(
hmac
.
new
(
bytes
(
api_secret
,
"utf-8"
)
,
bytes
(
f"GET/realtime
{
expires
}
"
,
"utf-8"
)
,
digestmod
=
"sha256"
)
.
hexdigest
(
)
)
ws
=
websocket
.
WebSocketApp
(
url
=
url
,
.
.
.
)
# Authenticate with API.
ws
.
send
(
json
.
dumps
(
{
"op"
:
"auth"
,
"args"
:
[
api_key
,
expires
,
signature
]
}
)
)
Successful authentication sample response
{
"success"
:
true
,
"ret_msg"
:
""
,
"op"
:
"auth"
,
"conn_id"
:
"cejreaspqfh3sjdnldmg-p"
}
note
Example signature algorithms can be found
here
.
caution
Due to network complexity, your may get disconnected at any time. Please follow the instructions below to ensure that you receive WebSocket messages on time:
Keep connection alive by
sending the heartbeat packet
Reconnect as soon as possible if disconnected
IP Limits
​
Do not frequently connect and disconnect the connection.
Do not build over 500 connections in 5 minutes. This is counted per WebSocket domain.
Public channel - Args limits
​
Regardless of Perpetual, Futures, Options or Spot, for one public connection, you cannot have length of "args" array over 21,000 characters.
Spot can input up to 10 args for each subscription request sent to one connection
Options can input up to 2000 args for a single connection
No args limit for Futures and Spread for now
How to Send the Heartbeat Packet
​
How to Send
// req_id is a customised ID, which is optional
ws
.
send
(
JSON
.
stringify
(
{
"req_id"
:
"100001"
,
"op"
:
"ping"
}
)
)
;
Pong message example of public channels
Spot
Linear/Inverse
Option/Spread
{
"success"
:
true
,
"ret_msg"
:
"pong"
,
"conn_id"
:
"0970e817-426e-429a-a679-ff7f55e0b16a"
,
"op"
:
"ping"
}
{
"success"
:
true
,
"ret_msg"
:
"pong"
,
"conn_id"
:
"465772b1-7630-4fdc-a492-e003e6f0f260"
,
"req_id"
:
""
,
"op"
:
"ping"
}
{
"args"
:
[
"1672916271846"
]
,
"op"
:
"pong"
}
Pong message example of private channels
{
"req_id"
:
"test"
,
"op"
:
"pong"
,
"args"
:
[
"1675418560633"
]
,
"conn_id"
:
"cfcb4ocsvfriu23r3er0-1b"
}
caution
To avoid network or program issues, we recommend that you send the
ping
heartbeat packet every
20
seconds to maintain the WebSocket connection.
How to Subscribe to Topics
​
Understanding WebSocket Filters
​
How to subscribe with a filter
// Subscribing level 1 orderbook
{
"req_id"
:
"test"
,
// optional
"op"
:
"subscribe"
,
"args"
:
[
"orderbook.1.BTCUSDT"
]
}
Subscribing with multiple symbols and topics is supported.
{
"req_id"
:
"test"
,
// optional
"op"
:
"subscribe"
,
"args"
:
[
"orderbook.1.BTCUSDT"
,
"publicTrade.BTCUSDT"
,
"orderbook.1.ETHUSDT"
]
}
Understanding WebSocket Filters: Unsubscription
​
You can dynamically subscribe and unsubscribe from topics without unsubscribing from the WebSocket like so:
{
"op"
:
"unsubscribe"
,
"args"
:
[
"publicTrade.ETHUSD"
]
,
"req_id"
:
"customised_id"
}
Understanding the Subscription Response
​
Topic subscription response message example
Private
Public Spot
Linear/Inverse
Option/Spread
{
"success"
:
true
,
"ret_msg"
:
""
,
"op"
:
"subscribe"
,
"conn_id"
:
"cejreassvfrsfvb9v1a0-2m"
}
{
"success"
:
true
,
"ret_msg"
:
"subscribe"
,
"conn_id"
:
"2324d924-aa4d-45b0-a858-7b8be29ab52b"
,
"req_id"
:
"10001"
,
"op"
:
"subscribe"
}
{
"success"
:
true
,
"ret_msg"
:
""
,
"conn_id"
:
"3cd84cb1-4d06-4a05-930a-2efe5fc70f0f"
,
"req_id"
:
""
,
"op"
:
"subscribe"
}
{
"success"
:
true
,
"conn_id"
:
"aa01fbfffe80af37-00000001-000b37b9-7147f432704fd28c-00e1c172"
,
"data"
:
{
"failTopics"
:
[
]
,
"successTopics"
:
[
"orderbook.100.BTC-6JAN23-18000-C"
]
}
,
"type"
:
"COMMAND_RESP"
}

**Examples:**

Example 1 ():
```
{    "req_id": "10001", // optional    "op": "auth",    "args": [        "api_key",        1662350400000, // expires; is greater than your current timestamp        "signature"    ]}
```

Example 2 ():
```
# based on: https://github.com/bybit-exchange/pybit/blob/master/pybit/_http_manager.pyimport hmacimport jsonimport timeimport websocketapi_key = ""api_secret = ""# Generate expires.expires = int((time.time() + 1) * 1000)# Generate signature.signature = str(hmac.new(    bytes(api_secret, "utf-8"),    bytes(f"GET/realtime{expires}", "utf-8"), digestmod="sha256").hexdigest())ws = websocket.WebSocketApp(    url=url,    ...)# Authenticate with API.ws.send(    json.dumps({        "op": "auth",        "args": [api_key, expires, signature]    }))
```

Example 3 ():
```
{    "success": true,    "ret_msg": "",    "op": "auth",    "conn_id": "cejreaspqfh3sjdnldmg-p"}
```

Example 4 ():
```
// req_id is a customised ID, which is optionalws.send(JSON.stringify({"req_id": "100001", "op": "ping"}));
```

---

## Ticker

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/ticker

**Contents:**
- Ticker
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
Ticker
On this page
Ticker
Subscribe to the ticker stream.
note
This topic utilises the snapshot field and delta field. If a response param is not found in the message, then its value has not changed.
Spot & Option tickers message are
snapshot
only
Push frequency: Derivatives & Options -
100ms
, Spot -
50ms
Topic:
tickers.{symbol}
Response Parameters
​
Linear/Inverse
Option
Spot
Parameter
Type
Comments
topic
string
Topic name
type
string
Data type.
snapshot
,
delta
cs
integer
Cross sequence
ts
number
The timestamp (ms) that the system generates the data
data
array
Object
> symbol
string
Symbol name
>
tickDirection
string
Tick direction
> price24hPcnt
string
Percentage change of market price in the last 24 hours
> lastPrice
string
Last price
> prevPrice24h
string
Market price 24 hours ago
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> prevPrice1h
string
Market price an hour ago
> markPrice
string
Mark price
> indexPrice
string
Index price
> openInterest
string
Open interest size
> openInterestValue
string
Open interest value
> turnover24h
string
Turnover for 24h
> volume24h
string
Volume for 24h
> nextFundingTime
string
Next funding timestamp (ms)
> fundingRate
string
Funding rate
> bid1Price
string
Best bid price
> bid1Size
string
Best bid size
> ask1Price
string
Best ask price
> ask1Size
string
Best ask size
> deliveryTime
datetime
Delivery date time (UTC+0), applicable to expired futures only
> basisRate
string
Basis rate.
Unique field for inverse futures & USDT/USDC futures
> deliveryFeeRate
string
Delivery fee rate.
Unique field for inverse futures & USDT/USDC futures
> predictedDeliveryPrice
string
Predicated delivery price.
Unique field for inverse futures & USDT/USDC futures
> preOpenPrice
string
Estimated pre-market contract open price
The value is meaningless when entering continuous trading phase
USDC Futures and Inverse Futures do not have this field
> preQty
string
Estimated pre-market contract open qty
The value is meaningless when entering continuous trading phase
USDC Futures and Inverse Futures do not have this field
>
curPreListingPhase
string
The current pre-market contract phase
USDC Futures and Inverse Futures do not have this field
> fundingIntervalHour
string
Funding interval hour
This value currently only supports whole hours
Only for Perpetual,For Futures,this field will not return
> fundingCap
string
Funding rate upper and lower limits
Only for Perpetual,For Futures,this field will not return
> basisRateYear
string
Annual basis rate
Only for Futures,For Perpetual,this field will not return
Parameter
Type
Comments
topic
string
Topic name
type
string
Data type.
snapshot
id
string
message ID
ts
number
The timestamp (ms) that the system generates the data
data
array
Object
> symbol
string
Symbol name
> bidPrice
string
Best bid price
> bidSize
string
Best bid size
> bidIv
string
Best bid iv
> askPrice
string
Best ask price
> askSize
string
Best ask size
> askIv
string
Best ask iv
> lastPrice
string
Last price
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> markPrice
string
Mark price
> indexPrice
string
Index price
> markPriceIv
string
Mark price iv
> underlyingPrice
string
Underlying price
> openInterest
string
Open interest size
> turnover24h
string
Turnover for 24h
> volume24h
string
Volume for 24h
> totalVolume
string
Total volume
> totalTurnover
string
Total turnover
> delta
string
Delta
> gamma
string
Gamma
> vega
string
Vega
> theta
string
Theta
> predictedDeliveryPrice
string
Predicated delivery price. It has value when 30 min before delivery
> change24h
string
The change in the last 24 hous
Parameter
Type
Comments
topic
string
Topic name
ts
number
The timestamp (ms) that the system generates the data
type
string
Data type.
snapshot
cs
integer
Cross sequence
data
array
Object
> symbol
string
Symbol name
> lastPrice
string
Last price
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> prevPrice24h
string
Percentage change of market price relative to 24h
> volume24h
string
Volume for 24h
> turnover24h
string
Turnover for 24h
> price24hPcnt
string
Percentage change of market price relative to 24h
> usdIndexPrice
string
USD index price
used to calculate USD value of the assets in Unified account
non-collateral margin coin returns ""
Subscribe Example
​
Linear
Option
Spot
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"linear"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
ticker_stream
(
symbol
=
"BTCUSDT"
,
callback
=
handle_message
)
while
True
:
sleep
(
1
)
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"option"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
ticker_stream
(
symbol
=
"tickers.BTC-22JAN23-17500-C"
,
callback
=
handle_message
)
while
True
:
sleep
(
1
)
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"spot"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
ticker_stream
(
symbol
=
"BTCUSDT"
,
callback
=
handle_message
)
while
True
:
sleep
(
1
)
Response Example
​
Linear
Option
Spot
LinearPerpetual
{
"topic"
:
"tickers.BTCUSDT"
,
"type"
:
"snapshot"
,
"data"
:
{
"symbol"
:
"BTCUSDT"
,
"tickDirection"
:
"MinusTick"
,
"price24hPcnt"
:
"-0.158315"
,
"lastPrice"
:
"66666.60"
,
"prevPrice24h"
:
"79206.20"
,
"highPrice24h"
:
"79266.30"
,
"lowPrice24h"
:
"65076.90"
,
"prevPrice1h"
:
"66666.60"
,
"markPrice"
:
"66666.60"
,
"indexPrice"
:
"115418.19"
,
"openInterest"
:
"492373.72"
,
"openInterestValue"
:
"32824881841.75"
,
"turnover24h"
:
"4936790807.6521"
,
"volume24h"
:
"73191.3870"
,
"fundingIntervalHour"
:
"8"
,
"fundingCap"
:
"0.005"
,
"nextFundingTime"
:
"1760342400000"
,
"fundingRate"
:
"-0.005"
,
"bid1Price"
:
"66666.60"
,
"bid1Size"
:
"23789.165"
,
"ask1Price"
:
"66666.70"
,
"ask1Size"
:
"23775.469"
,
"preOpenPrice"
:
""
,
"preQty"
:
""
,
"curPreListingPhase"
:
""
}
,
"cs"
:
9532239429
,
"ts"
:
1760325052630
}
LinearFutures
{
"topic"
:
"tickers.BTC-26DEC25"
,
"type"
:
"snapshot"
,
"data"
:
{
"symbol"
:
"BTC-26DEC25"
,
"tickDirection"
:
"ZeroMinusTick"
,
"price24hPcnt"
:
"0"
,
"lastPrice"
:
"109401.50"
,
"prevPrice24h"
:
"109401.50"
,
"highPrice24h"
:
"109401.50"
,
"lowPrice24h"
:
"109401.50"
,
"prevPrice1h"
:
"109401.50"
,
"markPrice"
:
"121144.63"
,
"indexPrice"
:
"114132.51"
,
"openInterest"
:
"6.622"
,
"openInterestValue"
:
"802219.74"
,
"turnover24h"
:
"0.0000"
,
"volume24h"
:
"0.0000"
,
"deliveryTime"
:
"2025-12-26T08:00:00Z"
,
"basisRate"
:
"0.06129209"
,
"deliveryFeeRate"
:
"0"
,
"predictedDeliveryPrice"
:
"0.00"
,
"basis"
:
"-4730.84"
,
"basisRateYear"
:
"0.30655351"
,
"nextFundingTime"
:
""
,
"fundingRate"
:
""
,
"bid1Price"
:
"111254.50"
,
"bid1Size"
:
"0.176"
,
"ask1Price"
:
"131001.00"
,
"ask1Size"
:
"0.580"
}
,
"cs"
:
31337927919
,
"ts"
:
1760409119857
}
{
"id"
:
"tickers.BTC-6JAN23-17500-C-2480334983-1672917511074"
,
"topic"
:
"tickers.BTC-6JAN23-17500-C"
,
"ts"
:
1672917511074
,
"data"
:
{
"symbol"
:
"BTC-6JAN23-17500-C"
,
"bidPrice"
:
"0"
,
"bidSize"
:
"0"
,
"bidIv"
:
"0"
,
"askPrice"
:
"10"
,
"askSize"
:
"5.1"
,
"askIv"
:
"0.514"
,
"lastPrice"
:
"10"
,
"highPrice24h"
:
"25"
,
"lowPrice24h"
:
"5"
,
"markPrice"
:
"7.86976724"
,
"indexPrice"
:
"16823.73"
,
"markPriceIv"
:
"0.4896"
,
"underlyingPrice"
:
"16815.1"
,
"openInterest"
:
"49.85"
,
"turnover24h"
:
"446802.8473"
,
"volume24h"
:
"26.55"
,
"totalVolume"
:
"86"
,
"totalTurnover"
:
"1437431"
,
"delta"
:
"0.047831"
,
"gamma"
:
"0.00021453"
,
"vega"
:
"0.81351067"
,
"theta"
:
"-19.9115368"
,
"predictedDeliveryPrice"
:
"0"
,
"change24h"
:
"-0.33333334"
}
,
"type"
:
"snapshot"
}
{
"topic"
:
"tickers.BTCUSDT"
,
"ts"
:
1673853746003
,
"type"
:
"snapshot"
,
"cs"
:
2588407389
,
"data"
:
{
"symbol"
:
"BTCUSDT"
,
"lastPrice"
:
"21109.77"
,
"highPrice24h"
:
"21426.99"
,
"lowPrice24h"
:
"20575"
,
"prevPrice24h"
:
"20704.93"
,
"volume24h"
:
"6780.866843"
,
"turnover24h"
:
"141946527.22907118"
,
"price24hPcnt"
:
"0.0196"
,
"usdIndexPrice"
:
"21120.2400136"
}
}

**Examples:**

Example 1 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.ticker_stream(    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="option",)def handle_message(message):    print(message)ws.ticker_stream(    symbol="tickers.BTC-22JAN23-17500-C",    callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="spot",)def handle_message(message):    print(message)ws.ticker_stream(    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 4 ():
```
LinearPerpetual{  "topic": "tickers.BTCUSDT",  "type": "snapshot",  "data": {    "symbol": "BTCUSDT",    "tickDirection": "MinusTick",    "price24hPcnt": "-0.158315",    "lastPrice": "66666.60",    "prevPrice24h": "79206.20",    "highPrice24h": "79266.30",    "lowPrice24h": "65076.90",    "prevPrice1h": "66666.60",    "markPrice": "66666.60",    "indexPrice": "115418.19",    "openInterest": "492373.72",    "openInterestValue": "32824881841.75",    "turnover24h": "4936790807.6521",    "volume24h": "73191.3870",    "fundingIntervalHour": "8",    "fundingCap": "0.005",    "nextFundingTime": "1760342400000",    "fundingRate": "-0.005",    "bid1Price": "66666.60",    "bid1Size": "23789.165",    "ask1Price": "66666.70",    "ask1Size": "23775.469",    "preOpenPrice": "",    "preQty": "",    "curPreListingPhase": ""  },  "cs": 9532239429,  "ts": 1760325052630}LinearFutures{  "topic": "tickers.BTC-26DEC25",  "type": "snapshot",  "data": {    "symbol": "BTC-26DEC25",    "tickDirection": "ZeroMinusTick",    "price24hPcnt": "0",    "lastPrice": "109401.50",    "prevPrice24h": "109401.50",    "highPrice24h": "109401.50",    "lowPrice24h": "109401.50",    "prevPrice1h": "109401.50",    "markPrice": "121144.63",    "indexPrice": "114132.51",    "openInterest": "6.622",    "openInterestValue": "802219.74",    "turnover24h": "0.0000",    "volume24h": "0.0000",    "deliveryTime": "2025-12-26T08:00:00Z",    "basisRate": "0.06129209",    "deliveryFeeRate": "0",    "predictedDeliveryPrice": "0.00",    "basis": "-4730.84",    "basisRateYear": "0.30655351",    "nextFundingTime": "",    "fundingRate": "",    "bid1Price": "111254.50",    "bid1Size": "0.176",    "ask1Price": "131001.00",    "ask1Size": "0.580"  },  "cs": 31337927919,  "ts": 1760409119857}
```

---

## System Status

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/system/system-status

**Contents:**
- System Status
- URL​
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
System
System Status
On this page
System Status
Listen to the system status when there is a platform maintenance or service incident.
info
Please note currently system maintenance that may result in short interruption (lasting less than 10 seconds) or websocket disconnection (users can immediately reconnect) will not be announced.
URL
​
Mainnet:
wss://stream.bybit.com/v5/public/misc/status
info
EU users registered from "
www.bybit.eu"
, please use
wss://stream.bybit.eu/v5/public/misc/status
Topic:
system.status
Response Parameters
​
Parameter
Type
Comments
topic
string
Topic name
ts
number
The timestamp (ms) that the system generates the data
data
array
Object
> id
string
Id. Unique identifier
> title
string
Title of system maintenance
>
state
string
System state
> begin
string
Start time of system maintenance, timestamp in milliseconds
> end
string
End time of system maintenance, timestamp in milliseconds. Before maintenance is completed, it is the expected end time; After maintenance is completed, it will be changed to the actual end time.
> href
string
Hyperlink to system maintenance details. Default value is empty string
>
serviceTypes
array
<
int
>
Service Type
>
product
array
<
int
>
Product
> uidSuffix
array
<
int
>
Affected UID tail number
>
maintainType
string
Maintenance type
>
env
string
Environment
Subscribe Example
​
JSON
Python
{
"op"
:
"subscribe"
,
"args"
:
[
"system.status"
]
}
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"misc/status"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
system_status_stream
(
callback
=
handle_message
)
while
True
:
sleep
(
1
)
Response Example
​
{
"topic"
:
"system.status"
,
"ts"
:
1751858399649
,
"data"
:
[
{
"id"
:
"4d95b2a0-587f-11f0-bcc9-56f28c94d6ea"
,
"title"
:
"t06"
,
"state"
:
"completed"
,
"begin"
:
"1751596902000"
,
"end"
:
"1751597011000"
,
"href"
:
""
,
"serviceTypes"
:
[
2
,
3
,
4
,
5
]
,
"product"
:
[
1
,
2
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
1
,
"env"
:
1
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "system.status"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="misc/status",)def handle_message(message):    print(message)ws.system_status_stream(    callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "topic": "system.status",    "ts": 1751858399649,    "data": [        {            "id": "4d95b2a0-587f-11f0-bcc9-56f28c94d6ea",            "title": "t06",            "state": "completed",            "begin": "1751596902000",            "end": "1751597011000",            "href": "",            "serviceTypes": [                2,                3,                4,                5            ],            "product": [                1,                2            ],            "uidSuffix": [],            "maintainType": 1,            "env": 1        }    ]}
```

---

## Get System Status

**URL:** https://bybit-exchange.github.io/docs/v5/system-status

**Contents:**
- Get System Status
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Get System Status
On this page
Get System Status
Get the system status when there is a platform maintenance or service incident.
info
Please note currently system maintenance that may result in short interruption (lasting less than 10 seconds) or websocket disconnection (users can immediately reconnect) will not be announced.
HTTP Request
​
GET
/v5/system/status
Request Parameters
​
Parameter
Required
Type
Comments
id
false
string
id. Unique identifier
state
false
string
system state
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> id
string
Id. Unique identifier
> title
string
Title of system maintenance
>
state
string
System state
> begin
string
Start time of system maintenance, timestamp in milliseconds
> end
string
End time of system maintenance, timestamp in milliseconds. Before maintenance is completed, it is the expected end time; After maintenance is completed, it will be changed to the actual end time.
> href
string
Hyperlink to system maintenance details. Default value is empty string
>
serviceTypes
array
<
int
>
Service Type
>
product
array
<
int
>
Product
> uidSuffix
array
<
int
>
Affected UID tail number
>
maintainType
string
Maintenance type
>
env
string
Environment
Request Example
​
HTTP
Python
GET
/v5/system/status
HTTP/1.1
Host
:
api.bybit.com
from
pybit
.
unified_trading
import
HTTP
session
=
HTTP
(
testnet
=
True
,
)
print
(
session
.
get_price_limit
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
)
)
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
""
,
"result"
:
{
"list"
:
[
{
"id"
:
"4d95b2a0-587f-11f0-bcc9-56f28c94d6ea"
,
"title"
:
"t06"
,
"state"
:
"completed"
,
"begin"
:
"1751596902000"
,
"end"
:
"1751597011000"
,
"href"
:
""
,
"serviceTypes"
:
[
2
,
3
,
4
,
5
]
,
"product"
:
[
1
,
2
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
1
,
"env"
:
1
}
,
{
"id"
:
"19bb6f82-587f-11f0-bcc9-56f28c94d6ea"
,
"title"
:
"t05"
,
"state"
:
"completed"
,
"begin"
:
"1751254200000"
,
"end"
:
"1751254500000"
,
"href"
:
""
,
"serviceTypes"
:
[
1
,
4
]
,
"product"
:
[
1
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
3
,
"env"
:
1
}
,
{
"id"
:
"25f4bc8c-533c-11f0-bcc9-56f28c94d6ea"
,
"title"
:
"t04"
,
"state"
:
"completed"
,
"begin"
:
"1751017967000"
,
"end"
:
"1751018096000"
,
"href"
:
""
,
"serviceTypes"
:
[
2
]
,
"product"
:
[
2
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
1
,
"env"
:
1
}
,
{
"id"
:
"679a9c5f-533b-11f0-bcc9-56f28c94d6ea"
,
"title"
:
"t03"
,
"state"
:
"completed"
,
"begin"
:
"1751017532000"
,
"end"
:
"1751017658000"
,
"href"
:
""
,
"serviceTypes"
:
[
5
,
4
]
,
"product"
:
[
1
,
2
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
2
,
"env"
:
1
}
,
{
"id"
:
"c8990f96-5332-11f0-8fd3-c241b123dd9e"
,
"title"
:
"t02"
,
"state"
:
"completed"
,
"begin"
:
"1751013817000"
,
"end"
:
"1751013890000"
,
"href"
:
""
,
"serviceTypes"
:
[
5
,
4
,
3
,
2
,
1
]
,
"product"
:
[
4
,
3
,
2
,
1
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
2
,
"env"
:
1
}
,
{
"id"
:
"f9d6842d-5331-11f0-8fd3-c241b123dd9e"
,
"title"
:
"t01"
,
"state"
:
"completed"
,
"begin"
:
"1751012688000"
,
"end"
:
"1751012760000"
,
"href"
:
""
,
"serviceTypes"
:
[
1
,
2
,
3
,
4
,
5
]
,
"product"
:
[
1
,
2
,
3
,
4
]
,
"uidSuffix"
:
[
]
,
"maintainType"
:
3
,
"env"
:
2
}
]
}
,
"retExtInfo"
:
{
}
,
"time"
:
1751858399649
}

**Examples:**

Example 1 ():
```
GET /v5/system/status HTTP/1.1Host: api.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_price_limit(    category="linear",    symbol="BTCUSDT",))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "id": "4d95b2a0-587f-11f0-bcc9-56f28c94d6ea",                "title": "t06",                "state": "completed",                "begin": "1751596902000",                "end": "1751597011000",                "href": "",                "serviceTypes": [                    2,                    3,                    4,                    5                ],                "product": [                    1,                    2                ],                "uidSuffix": [],                "maintainType": 1,                "env": 1            },            {                "id": "19bb6f82-587f-11f0-bcc9-56f28c94d6ea",                "title": "t05",                "state": "completed",                "begin": "1751254200000",                "end": "1751254500000",                "href": "",                "serviceTypes": [                    1,                    4                ],                "product": [                    1                ],                "uidSuffix": [],                "maintainType": 3,                "env": 1            },            {                "id": "25f4bc8c-533c-11f0-bcc9-56f28c94d6ea",                "title": "t04",                "state": "completed",                "begin": "1751017967000",                "end": "1751018096000",                "href": "",                "serviceTypes": [                    2                ],                "product": [                    2                ],                "uidSuffix": [],                "maintainType": 1,                "env": 1            },            {                "id": "679a9c5f-533b-11f0-bcc9-56f28c94d6ea",                "title": "t03",                "state": "completed",                "begin": "1751017532000",                "end": "1751017658000",                "href": "",                "serviceTypes": [                    5,                    4                ],                "product": [                    1,                    2                ],                "uidSuffix": [],                "maintainType": 2,                "env": 1            },            {                "id": "c8990f96-5332-11f0-8fd3-c241b123dd9e",                "title": "t02",                "state": "completed",                "begin": "1751013817000",                "end": "1751013890000",                "href": "",                "serviceTypes": [                    5,                    4,                    3,                    2,                    1                ],                "product": [                    4,                    3,                    2,                    1                ],                "uidSuffix": [],                "maintainType": 2,                "env": 1            },            {                "id": "f9d6842d-5331-11f0-8fd3-c241b123dd9e",                "title": "t01",                "state": "completed",                "begin": "1751012688000",                "end": "1751012760000",                "href": "",                "serviceTypes": [                    1,                    2,                    3,                    4,                    5                ],                "product": [                    1,                    2,                    3,                    4                ],                "uidSuffix": [],                "maintainType": 3,                "env": 2            }        ]    },    "retExtInfo": {},    "time": 1751858399649}
```

---

## Wallet

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/wallet

**Contents:**
- Wallet
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

WebSocket Stream
Private
Wallet
On this page
Wallet
Subscribe to the wallet stream to see changes to your wallet in
real-time
.
info
There is no snapshot event given at the time when the subscription is successful
The unrealised PnL change does not trigger an event
Under the new logic of UTA manual borrow,
spotBorrow
field corresponding to spot liabilities is detailed in the
announcement
.
Old
walletBalance
= New
walletBalance
-
spotBorrow
Topic:
wallet
Response Parameters
​
Parameter
Type
Comments
id
string
Message ID
topic
string
Topic name
creationTime
number
Data created timestamp (ms)
data
array
Object
> accountType
string
Account type
UNIFIED
> accountIMRate
string
Account IM rate
You can refer to this
Glossary
to understand the below fields calculation and mearning
All account wide fields are
not
applicable to isolated margin
> accountMMRate
string
Account MM rate
> totalEquity
string
Account total equity (USD): ∑Asset Equity By USD value of each asset
> totalWalletBalance
string
Account wallet balance (USD): ∑Asset Wallet Balance By USD value of each asset
> totalMarginBalance
string
Account margin balance (USD): totalWalletBalance + totalPerpUPL
> totalAvailableBalance
string
Account available balance (USD),
Cross Margin: totalMarginBalance - Haircut - totalInitialMargin.
Porfolio Margin: total Equity - Haircut - totalInitialMargin
> totalPerpUPL
string
Account Perps and Futures unrealised p&l (USD): ∑Each Perp and USDC Futures upl by base coin
> totalInitialMargin
string
Account initial margin (USD): ∑Asset Total Initial Margin Base Coin
> totalMaintenanceMargin
string
Account maintenance margin (USD): ∑ Asset Total Maintenance Margin Base Coin
> accountIMRateByMp
string
You can
ignore
this field, and refer to
accountIMRate
, which has the same calculation
> accountMMRateByMp
string
You can
ignore
this field, and refer to
accountMMRate
, which has the same calculation
> totalInitialMarginByMp
string
You can
ignore
this field, and refer to
totalInitialMargin
, which has the same calculation
> totalMaintenanceMarginByMp
string
You can
ignore
this field, and refer to
totalMaintenanceMargin
, which has the same calculation
> accountLTV
string
Deprecated
field
> coin
array
Object
>> coin
string
Coin name, such as BTC, ETH, USDT, USDC
>> equity
string
Equity of coin. Asset Equity = Asset Wallet Balance + Asset Perp UPL + Asset Future UPL + Asset Option Value =
walletBalance
-
spotBorrow
+
unrealisedPnl
+ Asset Option Value
>> usdValue
string
USD value of coin. If this coin cannot be collateral, then it is 0
>> walletBalance
string
Wallet balance of coin
>> locked
string
Locked balance due to the Spot open order
>> spotHedgingQty
string
The spot asset qty that is used to hedge in the portfolio margin, truncate to 8 decimals and "0" by default
>> borrowAmount
string
Borrow amount of coin = spot liabilities + derivatives liabilities
>> accruedInterest
string
Accrued interest
>> totalOrderIM
string
Pre-occupied margin for order. For portfolio margin mode, it returns ""
>> totalPositionIM
string
Sum of initial margin of all positions + Pre-occupied liquidation fee. For portfolio margin mode, it returns ""
>> totalPositionMM
string
Sum of maintenance margin for all positions. For portfolio margin mode, it returns ""
>> unrealisedPnl
string
Unrealised P&L
>> cumRealisedPnl
string
Cumulative Realised P&L
>> bonus
string
Bonus
>> collateralSwitch
boolean
Whether it can be used as a margin collateral currency (platform)
When marginCollateral=false, then collateralSwitch is meaningless
>> marginCollateral
boolean
Whether the collateral is turned on by user (user)
When marginCollateral=true, then collateralSwitch is meaningful
>> spotBorrow
string
Borrow amount by spot margin trade and manual borrow amount(does not include borrow amount by spot margin active order).
spotBorrow
field corresponding to spot liabilities is detailed in the
announcement
.
>> free
string
Deprecated
since there is no Spot wallet any more
>> availableToBorrow
string
Deprecated
field, always return
""
. Please refer to
availableToBorrow
in the
Get Collateral Info
>> availableToWithdraw
string
Deprecated
for
accountType=UNIFIED
from 9 Jan, 2025
Transferable balance: you can use
Get Transferable Amount (Unified)
or
Get All Coins Balance
instead
Derivatives available balance:
isolated margin
: walletBalance - totalPositionIM - totalOrderIM - locked - bonus
cross & portfolio margin
: look at field
totalAvailableBalance
(USD), which needs to be converted into the available balance of accordingly coin through index price
Spot (margin) available balance: refer to
Get Borrow Quota (Spot)
Subscribe Example
​
{
"op"
:
"subscribe"
,
"args"
:
[
"wallet"
]
}
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"private"
,
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
wallet_stream
(
callback
=
handle_message
)
while
True
:
sleep
(
1
)
Stream Example
​
{
"id"
:
"592324d2bce751-ad38-48eb-8f42-4671d1fb4d4e"
,
"topic"
:
"wallet"
,
"creationTime"
:
1700034722104
,
"data"
:
[
{
"accountIMRate"
:
"0"
,
"accountIMRateByMp"
:
"0"
,
"accountMMRate"
:
"0"
,
"accountMMRateByMp"
:
"0"
,
"totalEquity"
:
"10262.91335023"
,
"totalWalletBalance"
:
"9684.46297164"
,
"totalMarginBalance"
:
"9684.46297164"
,
"totalAvailableBalance"
:
"9556.6056555"
,
"totalPerpUPL"
:
"0"
,
"totalInitialMargin"
:
"0"
,
"totalInitialMarginByMp"
:
"0"
,
"totalMaintenanceMargin"
:
"0"
,
"totalMaintenanceMarginByMp"
:
"0"
,
"coin"
:
[
{
"coin"
:
"BTC"
,
"equity"
:
"0.00102964"
,
"usdValue"
:
"36.70759517"
,
"walletBalance"
:
"0.00102964"
,
"availableToWithdraw"
:
"0.00102964"
,
"availableToBorrow"
:
""
,
"borrowAmount"
:
"0"
,
"accruedInterest"
:
"0"
,
"totalOrderIM"
:
""
,
"totalPositionIM"
:
""
,
"totalPositionMM"
:
""
,
"unrealisedPnl"
:
"0"
,
"cumRealisedPnl"
:
"-0.00000973"
,
"bonus"
:
"0"
,
"collateralSwitch"
:
true
,
"marginCollateral"
:
true
,
"locked"
:
"0"
,
"spotHedgingQty"
:
"0.01592413"
,
"spotBorrow"
:
"0"
}
]
,
"accountLTV"
:
"0"
,
"accountType"
:
"UNIFIED"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "wallet"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)def handle_message(message):    print(message)ws.wallet_stream(callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "id": "592324d2bce751-ad38-48eb-8f42-4671d1fb4d4e",    "topic": "wallet",    "creationTime": 1700034722104,    "data": [        {            "accountIMRate": "0",            "accountIMRateByMp": "0",            "accountMMRate": "0",            "accountMMRateByMp": "0",            "totalEquity": "10262.91335023",            "totalWalletBalance": "9684.46297164",            "totalMarginBalance": "9684.46297164",            "totalAvailableBalance": "9556.6056555",            "totalPerpUPL": "0",            "totalInitialMargin": "0",            "totalInitialMarginByMp": "0",            "totalMaintenanceMargin": "0",            "totalMaintenanceMarginByMp": "0",            "coin": [                {                    "coin": "BTC",                    "equity": "0.00102964",                    "usdValue": "36.70759517",                    "walletBalance": "0.00102964",                    "availableToWithdraw": "0.00102964",                    "availableToBorrow": "",                    "borrowAmount": "0",                    "accruedInterest": "0",                    "totalOrderIM": "",                    "totalPositionIM": "",                    "totalPositionMM": "",                    "unrealisedPnl": "0",                    "cumRealisedPnl": "-0.00000973",                    "bonus": "0",                    "collateralSwitch": true,                    "marginCollateral": true,                    "locked": "0",                    "spotHedgingQty": "0.01592413",                    "spotBorrow": "0"                }            ],            "accountLTV": "0",            "accountType": "UNIFIED"        }    ]}
```

---

## All Liquidation

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/all-liquidation

**Contents:**
- All Liquidation
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
All Liquidation
On this page
All Liquidation
Subscribe to the liquidation stream, push all liquidations that occur on Bybit.
Covers: USDT contract / USDC contract / Inverse contract
Push frequency:
500ms
Topic:
allLiquidation.{symbol}
e.g., allLiquidation.BTCUSDT
Response Parameters
​
Parameter
Type
Comments
topic
string
Topic name
type
string
Data type.
snapshot
ts
number
The timestamp (ms) that the system generates the data
data
Object
> T
number
The updated timestamp (ms)
> s
string
Symbol name
> S
string
Position side.
Buy
,
Sell
. When you receive a
Buy
update, this means that a long position has been liquidated
> v
string
Executed size
> p
string
Bankruptcy price
Subscribe Example
​
from
pybit
.
unified_trading
import
WebSocket
from
time
import
sleep
ws
=
WebSocket
(
testnet
=
True
,
channel_type
=
"linear"
,
)
def
handle_message
(
message
)
:
print
(
message
)
ws
.
all_liquidation_stream
(
"ROSEUSDT"
,
handle_message
)
while
True
:
sleep
(
1
)
Response Example
​
{
"topic"
:
"allLiquidation.ROSEUSDT"
,
"type"
:
"snapshot"
,
"ts"
:
1739502303204
,
"data"
:
[
{
"T"
:
1739502302929
,
"s"
:
"ROSEUSDT"
,
"S"
:
"Sell"
,
"v"
:
"20000"
,
"p"
:
"0.04499"
}
]
}

**Examples:**

Example 1 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.all_liquidation_stream("ROSEUSDT", handle_message)while True:    sleep(1)
```

Example 2 ():
```
{    "topic": "allLiquidation.ROSEUSDT",    "type": "snapshot",    "ts": 1739502303204,    "data": [        {            "T": 1739502302929,            "s": "ROSEUSDT",            "S": "Sell",            "v": "20000",            "p": "0.04499"        }    ]}
```

---
