# Bybit-V5-Api - Market Data

**Pages:** 56

---

## Get Recent Public Trades

**URL:** https://bybit-exchange.github.io/docs/v5/market/recent-trade

**Contents:**
- Get Recent Public Trades
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Recent Public Trades
On this page
Get Recent Public Trades
Query recent public trading history in Bybit.
Covers: Spot / USDT contract / USDC contract / Inverse contract / Option
You can download archived historical trades from the
website
HTTP Request
​
GET
/v5/market/recent-trade
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
spot
,
linear
,
inverse
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
required
for spot/linear/inverse
optional for option
baseCoin
false
string
Base coin, uppercase only
Apply to
option
only
If the field is not passed, return
BTC
data by default
optionType
false
string
Option type.
Call
or
Put
. Apply to
option
only
limit
false
integer
Limit for data size per page
spot
:
[1,60]
, default:
60
others:
[1,1000]
, default:
500
Response Parameters
​
Parameter
Type
Comments
category
string
Products category
list
array
Object
> execId
string
Execution ID
> symbol
string
Symbol name
> price
string
Trade price
> size
string
Trade size
> side
string
Side of taker
Buy
,
Sell
> time
string
Trade time (ms)
> isBlockTrade
boolean
Whether the trade is block trade
> isRPITrade
boolean
Whether the trade is RPI trade
> mP
string
Mark price, unique field for
option
> iP
string
Index price, unique field for
option
> mIv
string
Mark iv, unique field for
option
> iv
string
iv, unique field for
option
> seq
string
cross sequence
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/recent-trade?category=spot&symbol=BTCUSDT&limit=1
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_public_trade_history
(
category
=
"spot"
,
symbol
=
"BTCUSDT"
,
limit
=
1
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetPublicRecentTrades
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
recentTrade
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
OPTION
)
.
symbol
(
"ETH-30JUN23-2050-C"
)
.
build
(
)
;
client
.
getRecentTradeData
(
recentTrade
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getPublicTradingHistory
(
{
category
:
'spot'
,
symbol
:
'BTCUSDT'
,
limit
:
1
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"spot"
,
"list"
:
[
{
"execId"
:
"2100000000007764263"
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"16618.49"
,
"size"
:
"0.00012"
,
"side"
:
"Buy"
,
"time"
:
"1672052955758"
,
"isBlockTrade"
:
false
,
"isRPITrade"
:
true
,
"seq"
:
"123456"
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
1672053054358
}

**Examples:**

Example 1 ():
```
GET /v5/market/recent-trade?category=spot&symbol=BTCUSDT&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_public_trade_history(    category="spot",    symbol="BTCUSDT",    limit=1,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetPublicRecentTrades(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var recentTrade = MarketDataRequest.builder().category(CategoryType.OPTION).symbol("ETH-30JUN23-2050-C").build();client.getRecentTradeData(recentTrade, System.out::println);
```

---

## Insurance Pool

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/insurance-pool

**Contents:**
- Insurance Pool
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
Insurance Pool
On this page
Insurance Pool
Subscribe to get the update of insurance pool balance
Push frequency:
1s
Topic:
USDT contracts:
insurance.USDT
USDC contracts:
insurance.USDC
(
note
: all USDC Perpetuals, USDC Futures have their own shared insurance pools)
Inverse contracts:
insurance.inverse
info
Shared insurance pool data is
not
pushed, please refer to Rest API
Get Insurance
to understand which symbols belong to isolated or shared insurance pools.
No event will be published if the balances of all insurance pools remain unchanged.
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
,
delta
ts
number
The timestamp (ms) that the system generates the data
data
Object
> coin
string
Insurance pool coin
> symbols
string
Symbol name
> balance
string
Balance
> updateTime
string
Data updated timestamp (ms)
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
"insurance.USDT"
,
"insurance.USDC"
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
insurance_pool_stream
(
contract_group
=
[
"USDT"
,
"USDC"
]
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
{
"topic"
:
"insurance.USDT"
,
"type"
:
"delta"
,
"ts"
:
1747722930000
,
"data"
:
[
{
"coin"
:
"USDT"
,
"symbols"
:
"GRIFFAINUSDT"
,
"balance"
:
"25614.92972633"
,
"updateTime"
:
"1747722930000"
}
,
{
"coin"
:
"USDT"
,
"symbols"
:
"CGPTUSDT"
,
"balance"
:
"100000.27064825"
,
"updateTime"
:
"1747722930000"
}
,
{
"coin"
:
"USDT"
,
"symbols"
:
"GOATUSDT"
,
"balance"
:
"20352.32665441"
,
"updateTime"
:
"1747722930000"
}
,
{
"coin"
:
"USDT"
,
"symbols"
:
"XTERUSDT"
,
"balance"
:
"19998.81533291"
,
"updateTime"
:
"1747722930000"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "insurance.USDT",        "insurance.USDC"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.insurance_pool_stream(    contract_group=["USDT", "USDC"],    callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "topic": "insurance.USDT",    "type": "delta",    "ts": 1747722930000,    "data": [        {            "coin": "USDT",            "symbols": "GRIFFAINUSDT",            "balance": "25614.92972633",            "updateTime": "1747722930000"        },        {            "coin": "USDT",            "symbols": "CGPTUSDT",            "balance": "100000.27064825",            "updateTime": "1747722930000"        },        {            "coin": "USDT",            "symbols": "GOATUSDT",            "balance": "20352.32665441",            "updateTime": "1747722930000"        },        {            "coin": "USDT",            "symbols": "XTERUSDT",            "balance": "19998.81533291",            "updateTime": "1747722930000"        }    ]}
```

---

## Get Fee Group Structure

**URL:** https://bybit-exchange.github.io/docs/v5/market/fee-group-info

**Contents:**
- Get Fee Group Structure
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Fee Group Structure
On this page
Get Fee Group Structure
Query for the
group fee structure
and fee rates.
note
The new grouped fee structure only applies to Pro-level and Market Maker clients. It does not apply to retail traders.
For more details please refer to the
fee structure update announcement
.
Covers: USDT Perpetual / USDT Delivery / USDC Perpetual / USDC Delivery / Inverse Contracts
info
Weighted maker volume
= Σ(Maker volume on pair × Group weighting factor (
weightingFactor
))
Weighted maker share
= (Your total weighted maker volume ÷ Bybit's total weighted maker volume)
Note: Bybit's total weighted maker volume is not provided by the API. Weighted maker share will be provided in the monthly MM report
.
HTTP Request
​
GET
/v5/market/fee-group-info
Request Parameters
​
Parameter
Required
Type
Comments
productType
true
string
Product type.
contract
only
groupId
false
string
Group ID.
1
,
2
,
3
,
4
,
5
,
6
,
7
Response Parameters
​
Parameter
Type
Comments
list
array
List of fee group objects
>
groupName
string
Fee group name
> weightingFactor
integer
Group weighting factor
> symbolsNumbers
integer
Symbols number
> symbols
array
Symbol name
> feeRates
object
Fee rate details for different categories.
pro
,
marketMaker
>> pro
array
Pro-level fee structures
>>> level
string
Pro level name.
Pro 1
,
Pro 2
,
Pro 3
,
Pro 4
,
Pro 5
,
Pro 6
>>> takerFeeRate
string
Taker fee rate
>>> makerFeeRate
string
Maker fee rate
>>> makerRebate
string
Maker rebate fee rate
>> marketMaker
array
Market Maker-level fee structures
>>> level
string
Market Maker level name.
MM 1
,
MM 2
,
MM 3
>>> takerFeeRate
string
Taker fee rate
>>> makerFeeRate
string
Maker fee rate
>>> makerRebate
string
Maker rebate fee rate
> updateTime
string
Latest data update timestamp (ms)
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/fee-group-info?productType=contract&groupId=1
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"groupName"
:
"G1(Major Coins)"
,
"weightingFactor"
:
1
,
"symbolsNumbers"
:
4
,
"symbols"
:
[
"ETHUSDT"
,
"XRPUSDT"
,
"SOLUSDT"
,
"BTCUSDT"
]
,
"feeRates"
:
{
"pro"
:
[
{
"level"
:
"Pro 1"
,
"takerFeeRate"
:
"0.00028"
,
"makerFeeRate"
:
"0.0001"
,
"makerRebate"
:
""
}
,
{
"level"
:
"Pro 2"
,
"takerFeeRate"
:
"0.00025"
,
"makerFeeRate"
:
"0.00005"
,
"makerRebate"
:
""
}
,
{
"level"
:
"Pro 3"
,
"takerFeeRate"
:
"0.00022"
,
"makerFeeRate"
:
"0.000025"
,
"makerRebate"
:
""
}
,
{
"level"
:
"Pro 4"
,
"takerFeeRate"
:
"0.0002"
,
"makerFeeRate"
:
"0.00001"
,
"makerRebate"
:
""
}
,
{
"level"
:
"Pro 5"
,
"takerFeeRate"
:
"0.00018"
,
"makerFeeRate"
:
"0"
,
"makerRebate"
:
""
}
,
{
"level"
:
"Pro 6"
,
"takerFeeRate"
:
"0.00015"
,
"makerFeeRate"
:
"0"
,
"makerRebate"
:
""
}
]
,
"marketMaker"
:
[
{
"level"
:
"MM 1"
,
"takerFeeRate"
:
""
,
"makerFeeRate"
:
""
,
"makerRebate"
:
"-0.0000075"
}
,
{
"level"
:
"MM 2"
,
"takerFeeRate"
:
""
,
"makerFeeRate"
:
""
,
"makerRebate"
:
"-0.000015"
}
,
{
"level"
:
"MM 3"
,
"takerFeeRate"
:
""
,
"makerFeeRate"
:
""
,
"makerRebate"
:
"-0.000025"
}
]
}
,
"updateTime"
:
"1753240500012"
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
1758627388542
}

**Examples:**

Example 1 ():
```
GET /v5/market/fee-group-info?productType=contract&groupId=1 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```

```

---

## Get Historical Volatility

**URL:** https://bybit-exchange.github.io/docs/v5/market/iv

**Contents:**
- Get Historical Volatility
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Historical Volatility
On this page
Get Historical Volatility
Query option historical volatility
Covers: Option
info
The data is hourly.
If both
startTime
and
endTime
are not specified, it will return the most recent 1 hours worth of data.
startTime
and
endTime
are a pair of params. Either both are passed or they are not passed at all.
This endpoint can query the last 2 years worth of data, but make sure
[
endTime
-
startTime
]
<= 30 days.
HTTP Request
​
GET
/v5/market/historical-volatility
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
option
baseCoin
false
string
Base coin, uppercase only. Default: return BTC data
quoteCoin
false
string
Quote coin,
USD
or
USDT
. Default: return quoteCoin=USD
period
false
integer
Period. If not specified, it will return data with a 7-day average by default
startTime
false
integer
The start timestamp (ms)
endTime
false
integer
The end timestamp (ms)
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> period
integer
Period
> value
string
Volatility
> time
string
Timestamp (ms)
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
GET
/v5/market/historical-volatility?category=option&baseCoin=ETH&period=30
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_historical_volatility
(
category
=
"option"
,
baseCoin
=
"ETH"
,
period
=
30
,
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
historicalVolatilityRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
OPTION
)
.
optionPeriod
(
7
)
.
build
(
)
;
client
.
getHistoricalVolatility
(
historicalVolatilityRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getHistoricalVolatility
(
{
category
:
'option'
,
baseCoin
:
'ETH'
,
period
:
30
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"SUCCESS"
,
"category"
:
"option"
,
"result"
:
[
{
"period"
:
30
,
"value"
:
"0.45024716"
,
"time"
:
"1672052400000"
}
]
}

**Examples:**

Example 1 ():
```
GET /v5/market/historical-volatility?category=option&baseCoin=ETH&period=30 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_historical_volatility(    category="option",    baseCoin="ETH",    period=30,))
```

Example 3 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var historicalVolatilityRequest = MarketDataRequest.builder().category(CategoryType.OPTION).optionPeriod(7).build();client.getHistoricalVolatility(historicalVolatilityRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,});client    .getHistoricalVolatility({        category: 'option',        baseCoin: 'ETH',        period: 30,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Open Orders

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/open-order

**Contents:**
- Get Open Orders
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Get Open Orders
On this page
Get Open Orders
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/spread/order/realtime
Request Parameters
​
Parameter
Required
Type
Comments
symbol
false
string
Spread combination symbol name
baseCoin
false
string
Base coin
orderId
false
string
Spread combination order ID
orderLinkId
false
string
User customised order ID
limit
false
integer
Limit for data size per page.
[
1
,
50
]
. Default:
20
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
list
array
<
object
>
Order info
> symbol
string
Spread combination symbol name
> baseCoin
string
Base coin
> orderType
string
Order type,
Market
,
Limit
> orderLinkId
string
User customised order ID
> side
string
Side,
Buy
,
Sell
> timeInForce
string
Time in force,
GTC
,
FOK
,
IOC
,
PostOnly
> orderId
string
Spread combination order ID
> leavesQty
string
The remaining qty not executed
> orderStatus
string
Order status,
New
,
PartiallyFilled
> cumExecQty
string
Cumulative executed order qty
> price
string
Order price
> qty
string
Order qty
> createdTime
string
Order created timestamp (ms)
> updatedTime
string
Order updated timestamp (ms)
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
GET
/v5/spread/order/realtime
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXXX
X-BAPI-API-KEY
:
XXXXXX
X-BAPI-TIMESTAMP
:
1744096099520
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"nextPageCursor"
:
"aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4%3A1744096099767%2Caaaee090-fab3-42ea-aea0-c9fbfe6c4bc4%3A1744096099767"
,
"list"
:
[
{
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"orderType"
:
"Limit"
,
"updatedTime"
:
"1744096099771"
,
"orderLinkId"
:
""
,
"side"
:
"Buy"
,
"orderId"
:
"aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4"
,
"leavesQty"
:
"0.1"
,
"orderStatus"
:
"New"
,
"cumExecQty"
:
"0"
,
"price"
:
"-4"
,
"qty"
:
"0.1"
,
"createdTime"
:
"1744096099767"
,
"timeInForce"
:
"PostOnly"
,
"baseCoin"
:
"SOL"
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
1744096103435
}

**Examples:**

Example 1 ():
```
GET /v5/spread/order/realtime HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744096099520X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4%3A1744096099767%2Caaaee090-fab3-42ea-aea0-c9fbfe6c4bc4%3A1744096099767",        "list": [            {                "symbol": "SOLUSDT_SOL/USDT",                "orderType": "Limit",                "updatedTime": "1744096099771",                "orderLinkId": "",                "side": "Buy",                "orderId": "aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4",                "leavesQty": "0.1",                "orderStatus": "New",                "cumExecQty": "0",                "price": "-4",                "qty": "0.1",                "createdTime": "1744096099767",                "timeInForce": "PostOnly",                "baseCoin": "SOL"            }        ]    },    "retExtInfo": {},    "time": 1744096103435}
```

---

## Set Risk Limit

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/set-risk-limit

**Contents:**
- Set Risk Limit
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Set Risk Limit
On this page
Set Risk Limit
Since bybit has launched auto risk limit on 12 March 2024, please click
here
to learn more, so it will not take effect even you set it successfully.
HTTP Request
​
POST
/v5/position/set-risk-limit
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type
Unified account:
linear
,
inverse
Classic account:
linear
,
inverse
.
Please note that
category
is
not
involved with business logic
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
riskId
true
integer
Risk limit ID
positionIdx
false
integer
Used to identify positions in different position modes. For hedge mode, it is
required
0
: one-way mode
1
: hedge-mode Buy side
2
: hedge-mode Sell side
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
riskId
integer
Risk limit ID
riskLimitValue
string
The position limit value corresponding to this risk ID
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/set-risk-limit
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672282269774
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"riskId"
:
4
,
"positionIdx"
:
null
}
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
set_risk_limit
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
riskId
=
4
,
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
position
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
position
.
request
.
*
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncPositionRestClient
(
)
;
var
setRiskLimitRequest
=
PositionDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
riskId
(
4
)
.
build
(
)
;
client
.
setRiskLimit
(
setRiskLimitRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
setRiskLimit
(
{
category
:
'linear'
,
symbol
:
'BTCUSDT'
,
riskId
:
4
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"riskId"
:
4
,
"riskLimitValue"
:
"8000000"
,
"category"
:
"linear"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672282270571
}

**Examples:**

Example 1 ():
```
POST /v5/position/set-risk-limit HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672282269774X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "BTCUSDT",    "riskId": 4,    "positionIdx": null}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_risk_limit(    category="linear",    symbol="BTCUSDT",    riskId=4,))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setRiskLimitRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").riskId(4).build();client.setRiskLimit(setRiskLimitRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setRiskLimit({        category: 'linear',        symbol: 'BTCUSDT',        riskId: 4,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Lending Market

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/supply-market

**Contents:**
- Get Lending Market
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Lending Market
On this page
Get Lending Market
info
Does not need authentication.
If you want to supply, you can use this endpoint to check whether there are any suitable counterparty borrow orders available.
HTTP Request
​
GET
/v5/crypto-loan-fixed/supply-order-quote
Request Parameters
​
Parameter
Required
Type
Comments
orderCurrency
true
string
Coin name
term
false
string
Fixed term
7
: 7 days;
14
: 14 days;
30
: 30 days;
90
: 90 days;
180
: 180 days
orderBy
true
string
Order by,
apy
: annual rate;
term
;
quantity
sort
false
integer
0
: ascend, default;
1
: descend
limit
false
integer
Limit for data size per page.
[
1
,
100
]
. Default:
10
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> orderCurrency
string
Coin name
> term
integer
Fixed term
7
: 7 days;
14
: 14 days;
30
: 30 days;
90
: 90 days;
180
: 180 days
> annualRate
string
Annual rate
> qty
string
Quantity
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/supply-order-quote?orderCurrency=USDT&orderBy=apy
HTTP/1.1
Host
:
api-testnet.bybit.com
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_lending_market_fixed_crypto_loan
(
orderCurrency
=
"USDT"
,
orderBy
=
"apy"
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
"ok"
,
"result"
:
{
"list"
:
[
{
"annualRate"
:
"0.02"
,
"orderCurrency"
:
"USDT"
,
"qty"
:
"1000.1234"
,
"term"
:
60
}
,
{
"annualRate"
:
"0.022"
,
"orderCurrency"
:
"USDT"
,
"qty"
:
"212.1234"
,
"term"
:
7
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
1752652136224
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/supply-order-quote?orderCurrency=USDT&orderBy=apy HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_lending_market_fixed_crypto_loan(    orderCurrency="USDT",    orderBy="apy",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "annualRate": "0.02",                "orderCurrency": "USDT",                "qty": "1000.1234",                "term": 60            },            {                "annualRate": "0.022",                "orderCurrency": "USDT",                "qty": "212.1234",                "term": 7            }        ]    },    "retExtInfo": {},    "time": 1752652136224}
```

---

## Get Leveraged Token Market

**URL:** https://bybit-exchange.github.io/docs/v5/lt/leverage-token-reference

**Contents:**
- Get Leveraged Token Market
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

On this page
Get Leveraged Token Market
Get leverage token market information
HTTP Request
​
GET
/v5/spot-lever-token/reference
Request Parameters
​
Parameter
Required
Type
Comments
ltCoin
true
string
Abbreviation of the LT, such as BTC3L
Response Parameters
​
Parameter
Type
Comments
ltCoin
string
Abbreviation of the LT, such as BTC3L
nav
string
net value
navTime
string
Update time for net asset value (in milliseconds and UTC time zone)
circulation
string
Circulating supply in the secondary market
basket
string
basket
leverage
string
Real leverage calculated by last traded price
RUN >>
Request Example
​
HTTP
Python
GET
/v5/spot-lever-token/reference?ltCoin=BTC3S
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_leveraged_token_market
(
ltCoin
=
"BTC3L"
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
"OK"
,
"result"
:
{
"basket"
:
"-132.460000082171973364"
,
"circulation"
:
"30097.901900052619091704"
,
"leverage"
:
"-2.666924651755770729"
,
"ltCoin"
:
"BTC3S"
,
"nav"
:
"27.692082719770373048"
,
"navTime"
:
"1672991679858"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672991679937
}

**Examples:**

Example 1 ():
```
GET /v5/spot-lever-token/reference?ltCoin=BTC3S HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_leveraged_token_market(    ltCoin="BTC3L",))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "basket": "-132.460000082171973364",        "circulation": "30097.901900052619091704",        "leverage": "-2.666924651755770729",        "ltCoin": "BTC3S",        "nav": "27.692082719770373048",        "navTime": "1672991679858"    },    "retExtInfo": {},    "time": 1672991679937}
```

---

## Get Funding Rate History

**URL:** https://bybit-exchange.github.io/docs/v5/market/history-fund-rate

**Contents:**
- Get Funding Rate History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Funding Rate History
On this page
Get Funding Rate History
Query for historical funding rates. Each symbol has a different funding interval. For example, if the interval is 8 hours and the current time is UTC 12, then it returns the last funding rate, which settled at UTC 8.
To query the funding rate interval, please refer to the
instruments-info
endpoint.
Covers: USDT and USDC perpetual / Inverse perpetual
info
Passing only
startTime
returns an error.
Passing only
endTime
returns 200 records up till
endTime
.
Passing neither returns 200 records up till the current time.
HTTP Request
​
GET
/v5/market/funding/history
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
linear
,
inverse
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
startTime
false
integer
The start timestamp (ms)
endTime
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
200
]
. Default:
200
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
> fundingRate
string
Funding rate
> fundingRateTimestamp
string
Funding rate timestamp (ms)
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/funding/history?category=linear&symbol=ETHPERP&limit=1
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_funding_rate_history
(
category
=
"linear"
,
symbol
=
"ETHPERP"
,
limit
=
1
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetFundingRateHistory
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
fundingHistoryRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"
BTCUSD
)
.
startTime
(
1632046800000L
)
.
endTime
(
1632133200000L
)
.
limit
(
150
)
.
build
(
)
;
client
.
getFundingHistory
(
fundingHistoryRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getFundingRateHistory
(
{
category
:
'linear'
,
symbol
:
'ETHPERP'
,
limit
:
1
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"linear"
,
"list"
:
[
{
"symbol"
:
"ETHPERP"
,
"fundingRate"
:
"0.0001"
,
"fundingRateTimestamp"
:
"1672041600000"
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
1672051897447
}

**Examples:**

Example 1 ():
```
GET /v5/market/funding/history?category=linear&symbol=ETHPERP&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP()print(session.get_funding_rate_history(    category="linear",    symbol="ETHPERP",    limit=1,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetFundingRateHistory(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var fundingHistoryRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSD).startTime(1632046800000L).endTime(1632133200000L).limit(150).build();client.getFundingHistory(fundingHistoryRequest, System.out::println);
```

---

## Get ADL Alert

**URL:** https://bybit-exchange.github.io/docs/v5/market/adl-alert

**Contents:**
- Get ADL Alert
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get ADL Alert
On this page
Get ADL Alert
Query for
ADL
(auto-deleveraging mechanism) alerts and insurance pool information.
Covers: USDT Perpetual / USDT Delivery / USDC Perpetual / USDC Delivery / Inverse Contracts
tip
Data update frequency: every 1 minute.
info
ADL trigger and stop conditions are based on the following three cases:
Contract PnL drawdown ADL (based on the new grouped insurance pool mechanism, see examples 1 and 2)
Trigger condition
:
balance
(insurance fund balance) >
adlTriggerThreshold
(trigger threshold for contract PnL drawdown ADL)
and
pnlRatio
<
insurancePnlRatio
(PnL ratio threshold for triggering ADL)
Where:
pnlRatio
: drawdown ratio of the symbol in the last 8 hours
Formula:
pnlRatio
= (Symbol's current PnL - Symbol's 8h max PnL) / Insurance pool's 8h max balance (
maxBalance
)
Note: the symbol's Current PnL and 8h Max PnL are not provided by the API
.
Insurance pool 8h max balance (
maxBalance
)
: the maximum balance of the grouped insurance pool in the last 8 hours
Stop condition
:
pnlRatio
>
adlStopRatio
(stop ratio threshold for ADL)
Insurance pool equity drawdown ADL (original mechanism, see example 3)
Trigger condition
:
balance
(insurance fund balance) ≤ 0
Stop condition
:
balance
(insurance fund balance) > 0
Excessive margin loss of a symbol after removing it from a grouped insurance pool (can be regarded as a special case of pool equity drawdown ADL)
To ensure pool safety, the risk control team may remove a symbol from its grouped pool and temporarily establish it as a new independent insurance pool.
Trigger condition
:
balance
(insurance fund balance) ≤ 0
Stop condition
:
balance
(insurance fund balance) > 0
ADL examples: Triggered by PnL Drawdown and Insurance Pool Balance
Example 1: Pool has no significant profit in the last 8 hours, then symbol loss exceeds the PnL ratio threshold (
insurancePnlRatio
), ADL will be triggered
Assume symbols A, B, and C share the same pool with an initial 8h
balance
of
1M USDT
A incurs a loss of
350K
Calculation:
pnlRatio
= -35%
balance
= 1M
adlTriggerThreshold
= 1 (a constant set by Bybit)
insurancePnlRatio
= -0.3 (a constant set by Bybit)
Condition check:
balance
(1M) >
adlTriggerThreshold
(1)
pnlRatio
(-0.35) <
insurancePnlRatio
(-0.3)
→ Contract PnL drawdown ADL is triggered
The system calculates the bankruptcy price at
-30% drawdown
so ADL closes
50K
worth of user positions to keep A's
pnlRatio
at -30%
Stop condition
: ADL stops if A's
pnlRatio
>
adlStopRatio
(-0.25, a constant set by Bybit)
Recovery methods
:
Platform injects funds into the pool and adjusts A's PnL
Pool continues to take A's positions and earns maintenance margin through liquidation on the market
Example 2: Pool has significant profit in the last 8 hours, but symbol loss exceeds the PnL ratio threshold (
insurancePnlRatio
), ADL will still be triggered
Assume symbols A, B, C share the same pool, initial
balance
=
1M USDT
A gains profit through liquidation, pool 8h Max Balance =
2M USDT
(A's PnL = +1M)
Later A incurs a loss of
600K
Calculation:
pnlRatio
= -30%
balance
= 2M
adlTriggerThreshold
= 1 (a constant set by Bybit)
insurancePnlRatio
= -0.3 (a constant set by Bybit)
Condition check:
balance
(2M) >
adlTriggerThreshold
(1)
pnlRatio
(-0.30) ≤
insurancePnlRatio
(-0.3)
→ Contract PnL drawdown ADL is triggered
The system calculates the bankruptcy price at
-30% drawdown
Stop condition
: ADL stops if A's
pnlRatio
>
adlStopRatio
(-0.25, a constant set by Bybit)
Recovery methods
:
Platform injects funds into the pool and adjusts A's PnL
Pool continues to take A's positions and earns maintenance margin through liquidation on the market
Example 3: Pool balance reaches zero which triggers ADL
Assume symbols A, B, C, D share the same pool, initial
balance
=
1M USDT
Although none of the
pnlRatio
values for the symbols reach -30%, the pool
balance
drops to 0
Condition check:
balance
(0) ≤ 0
→ Insurance pool equity ADL is triggered
The system redistributes bankruptcy loss across symbols based on their PnL when pool balance = 0
Stop condition
: ADL stops if
balance
> 0
Subscribe to the
ADL WebSocket topic
for faster updates.
HTTP Request
​
GET
/v5/market/adlAlert
Request Parameters
​
Parameter
Required
Type
Comments
symbol
false
string
Contract name, e.g.
BTCUSDT
. Uppercase only
Response Parameters
​
Parameter
Type
Comments
updateTime
string
Latest data update timestamp (ms)
list
array
Object
> coin
string
Token of the insurance pool
> symbol
string
Trading pair name
> balance
string
Balance of the insurance fund. Used to determine if ADL is triggered. For shared insurance pool, the "balance" field will follow a T+1 refresh mechanism and will be updated daily at 00:00 UTC.
> maxBalance
string
Deprecated, always return "". Maximum balance of the insurance pool in the last 8 hours
> insurancePnlRatio
string
PnL ratio threshold for triggering
contract PnL drawdown ADL
ADL is triggered when the symbol's PnL drawdown ratio in the last 8 hours exceeds this value
> pnlRatio
string
Symbol's PnL drawdown ratio in the last 8 hours. Used to determine whether ADL is triggered or stopped
> adlTriggerThreshold
string
Trigger threshold for
contract PnL drawdown ADL
This condition is only effective when the insurance pool balance is greater than this value; if so, an 8 hours drawdown exceeding n% may trigger ADL
> adlStopRatio
string
Stop ratio threshold for
contract PnL drawdown ADL
ADL stops when the symbol's 8 hours drawdown ratio falls below this value
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/adlAlert&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"updatedTime"
:
"1757733960000"
,
"list"
:
[
{
"coin"
:
"USDT"
,
"symbol"
:
"BTCUSDT"
,
"balance"
:
"92203504694.99632"
,
"maxBalance"
:
"92231510324.75948"
,
"insurancePnlRatio"
:
"-0.3"
,
"pnlRatio"
:
"-0.560973"
,
"adlTriggerThreshold"
:
"10000"
,
"adlStopRatio"
:
"-0.25"
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
1757734022014
}

**Examples:**

Example 1 ():
```
GET /v5/market/adlAlert&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```

```

---

## Get Instruments Info

**URL:** https://bybit-exchange.github.io/docs/v5/spread/market/instrument

**Contents:**
- Get Instruments Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Market
Get Instruments Info
On this page
Get Instruments Info
Query for the instrument specification of spread combinations.
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/spread/instrument
Request Parameters
​
Parameter
Required
Type
Comments
symbol
false
string
Spread combination symbol name
baseCoin
false
string
Base coin, uppercase only
limit
false
integer
Limit for data size per page.
[
1
,
500
]
. Default:
200
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
list
array
<
object
>
instrument info
> symbol
string
Spread combination symbol name
> contractType
string
Product type
FundingRateArb
: perpetual & spot combination
CarryTrade
: futures & spot combination
FutureSpread
: different expiry futures combination
PerpBasis
: futures & perpetual
> status
string
Spread status.
Trading
,
Settling
> baseCoin
string
Base coin
> quoteCoin
string
Quote coin
> settleCoin
string
Settle coin
> tickSize
string
The step to increase/reduce order price
> minPrice
string
Min. order price
> maxPrice
string
Max. order price
> lotSize
string
Order qty precision
> minSize
string
Min. order qty
> maxSize
string
Max. order qty
> launchTime
string
Launch timestamp (ms)
> deliveryTime
string
Delivery timestamp (ms)
> legs
array
<
object
>
Legs information
>> symbol
string
Legs symbol name
>> contractType
string
Legs contract type.
LinearPerpetual
,
LinearFutures
,
Spot
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
GET
/v5/spread/instrument?limit=1
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"contractType"
:
"FundingRateArb"
,
"status"
:
"Trading"
,
"baseCoin"
:
"SOL"
,
"quoteCoin"
:
"USDT"
,
"settleCoin"
:
"USDT"
,
"tickSize"
:
"0.0001"
,
"minPrice"
:
"-1999.9998"
,
"maxPrice"
:
"1999.9998"
,
"lotSize"
:
"0.1"
,
"minSize"
:
"0.1"
,
"maxSize"
:
"50000"
,
"launchTime"
:
"1743675300000"
,
"deliveryTime"
:
"0"
,
"legs"
:
[
{
"symbol"
:
"SOLUSDT"
,
"contractType"
:
"LinearPerpetual"
}
,
{
"symbol"
:
"SOLUSDT"
,
"contractType"
:
"Spot"
}
]
}
]
,
"nextPageCursor"
:
"first%3D100008%26last%3D100008"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1744076802479
}

**Examples:**

Example 1 ():
```
GET /v5/spread/instrument?limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "symbol": "SOLUSDT_SOL/USDT",                "contractType": "FundingRateArb",                "status": "Trading",                "baseCoin": "SOL",                "quoteCoin": "USDT",                "settleCoin": "USDT",                "tickSize": "0.0001",                "minPrice": "-1999.9998",                "maxPrice": "1999.9998",                "lotSize": "0.1",                "minSize": "0.1",                "maxSize": "50000",                "launchTime": "1743675300000",                "deliveryTime": "0",                "legs": [                    {                        "symbol": "SOLUSDT",                        "contractType": "LinearPerpetual"                    },                    {                        "symbol": "SOLUSDT",                        "contractType": "Spot"                    }                ]            }        ],        "nextPageCursor": "first%3D100008%26last%3D100008"    },    "retExtInfo": {},    "time": 1744076802479}
```

---

## Get Quotes (real-time)

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/quote-realtime

**Contents:**
- Get Quotes (real-time)
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get Quotes (real-time)
On this page
Get Quotes (real-time)
Get real-time quote information.
Up to 50 requests per second
info
Obtain quote information sent or received by users, query from rfq-egine, without delay
Pass both quoteId and quoteLinkId, with quoteId as the standard and priority: quoteId > quoteLinkId > rfqId
Sorted in descending order by createdAt.
Return all non-final quotes
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/rfq/quote-realtime
Request Parameters
​
Parameter
Required
Type
Comments
rfqId
false
string
Inquiry ID
quoteId
false
string
Quote ID
quoteLinkId
false
string
Quote custom ID, traderType is
request
, this field is invalid
traderType
false
string
Trader type,
quote
,
request
. Default:
quote
Response Parameters
​
Parameter
Type
Comments
list
array
An array of quotes
> rfqId
string
Inquiry ID
> rfqLinkId
string
Custom RFQ ID. Not publicly disclosed.
> quoteId
string
Quote ID
> quoteLinkId
string
Custom quote ID. Not publicly disclosed.
> expiresAt
string
The quote's expiration time (ms)
> deskCode
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
>> status
string
Status of the RFQ:
Active
PendingFill
Canceled
Filled
Expired
Failed
>> execQuoteSide
string
Execute the quote direction,
Buy
or
Sell
. When the quote direction is
Buy
, for maker, the execution direction is the same as the direction in legs, and opposite for taker. Conversely, the same applies
>> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
>> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
>> quoteBuyList
array of objects
Quote
Buy
Direction
>>> category
string
Product type:
spot
,
linear
,
option
>>> symbol
string
The unique instrument ID
>>> price
string
Quote price
>>> qty
string
Order quantity of the instrument.
>> quoteSellList
array of objects
Quote
Sell
Direction
>>> category
string
Product type:
spot
,
linear
,
option
>>> symbol
string
The unique instrument ID
>>> price
string
Quote price
>>> qty
string
Order quantity of the instrument.
Request Example
​
GET
/v5/rfq/quote-realtime
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676430842094
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"rfqLinkId"
:
""
,
"rfqId"
:
"175740578143743543930777169307022"
,
"quoteId"
:
"1757405933130044334361923221559805"
,
"quoteLinkId"
:
""
,
"expiresAt"
:
"1757405993126"
,
"status"
:
"Active"
,
"deskCode"
:
"test0904"
,
"execQuoteSide"
:
""
,
"quoteBuyList"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"113790"
,
"qty"
:
"0.5"
}
]
,
"quoteSellList"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"110500"
,
"qty"
:
"0.5"
}
]
,
"createdAt"
:
"1757405933126"
,
"updatedAt"
:
"1757405933126"
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
1757405978376
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/quote-realtime HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "rfqLinkId": "",                "rfqId": "175740578143743543930777169307022",                "quoteId": "1757405933130044334361923221559805",                "quoteLinkId": "",                "expiresAt": "1757405993126",                "status": "Active",                "deskCode": "test0904",                "execQuoteSide": "",                "quoteBuyList": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "price": "113790",                        "qty": "0.5"                    }                ],                "quoteSellList": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "price": "110500",                        "qty": "0.5"                    }                ],                "createdAt": "1757405933126",                "updatedAt": "1757405933126"            }        ]    },    "retExtInfo": {},    "time": 1757405978376}
```

---

## Get Convert History

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/query-trade-history

**Contents:**
- Get Convert History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Fiat-Convert
Get Convert History
On this page
Get Convert History
Returns all the convert history
HTTP Request
​
GET
/v5/fiat/query-trade-history
Request Parameters
​
Parameter
Required
Type
Comments
index
false
integer
Page number,started from 1, default 1
limit
false
integer
Page Size
[20-100]
20 records by default,up to 100 records, return 100 when exceeds 100
startTime
false
string
Query start time(Millisecond timestamp)
endTime
false
string
Query end time(Millisecond timestamp)
Response Parameters
​
Parameter
Type
Comments
result
array
Array of quotes
> tradeNo
string
Trade order No
> status
string
Trade status:
processing
success
failed
> quoteTxId
string
Quote transaction ID. It is system generated, and it is used to confirm quote
> exchangeRate
string
Exchange rate
> fromCoin
string
Convert from coin (coin to sell)
> fromCoinType
string
From coin type.
fiat
or
crypto
> toCoin
string
Convert to coin (coin to buy)
> toCoinType
string
To coin type.
fiat
or
crypto
> fromAmount
string
From coin amount (amount to sell)
> toAmount
string
To coin amount (amount to buy according to exchange rate)
> createdAt
string
Trade created timee (Millisecond timestamp)
> subUserId
string
The user's sub userId in bybit
Request Example
​
HTTP
GET
/v5/fiat/trade-query-history
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1720074159814
X-BAPI-RECV-WINDOW
:
5000
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"success"
,
"result"
:
[
{
"tradeNo"
:
"TradeNo123456"
,
"status"
:
"success"
,
"quoteTaxId"
:
"QuoteTaxId123456"
,
"exchangeRate"
:
"1.0"
,
"fromCoin"
:
"GEL"
,
"fromCoinType"
:
"fiat"
,
"toCoin"
:
"USDT"
,
"toCoinType"
:
"crypto"
,
"fromAmount"
:
"100"
,
"toAmount"
:
"100"
,
"createdAt"
:
"1764560093588"
,
"subUserId"
:
"123456"
}
]
}

**Examples:**

Example 1 ():
```
GET /v5/fiat/trade-query-history HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720074159814X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": [        {            "tradeNo": "TradeNo123456",            "status": "success",            "quoteTaxId": "QuoteTaxId123456",            "exchangeRate": "1.0",            "fromCoin": "GEL",            "fromCoinType": "fiat",            "toCoin": "USDT",            "toCoinType": "crypto",            "fromAmount": "100",            "toAmount": "100",            "createdAt": "1764560093588",            "subUserId": "123456"        }    ]}
```

---

## Stake / Redeem

**URL:** https://bybit-exchange.github.io/docs/v5/earn/create-order

**Contents:**
- Stake / Redeem
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Earn
Stake / Redeem
On this page
Stake / Redeem
info
API key needs "Earn" permission
note
In times of high demand for loans in the market for a specific cryptocurrency, the redemption of the principal
may encounter delays and is expected to be processed within 48 hours. The redemption of on-chain products may take up to a few days to complete. Once the redemption request is initiated,
it cannot be cancelled.
HTTP Request
​
POST
/v5/earn/place-order
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
FlexibleSaving
,
OnChain
Remarks
: currently, only flexible savings and on chain is supported
orderType
true
string
Stake
,
Redeem
accountType
true
string
FUND
,
UNIFIED
. Onchain only supports FUND
amount
true
string
Stake amount needs to satisfy minStake and maxStake
Both stake and redeem amount need to satify precision requirement
coin
true
string
Coin name
productId
true
string
Product ID
orderLinkId
true
string
Customised order ID, used to prevent from replay
support up to 36 characters
The same orderLinkId can't be used in 30 mins
redeemPositionId
false
string
The position ID that the user needs to redeem. Only is required in Onchain non-LST mode
toAccountType
false
string
FUND
,
UNIFIED
. Onchain LST mode supports
FUND
and
UNIFIED
(Private wealth management custodial subaccount only supports
UNIFIED
). Onchain non-LST mode only supports
FUND
Response Parameters
​
Parameter
Type
Comments
orderId
string
Order ID
orderLinkId
string
Order link ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/earn/place-order
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1739936605822
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
190
{
"category"
:
"FlexibleSaving"
,
"orderType"
:
"Redeem"
,
"accountType"
:
"FUND"
,
"amount"
:
"0.35"
,
"coin"
:
"BTC"
,
"productId"
:
"430"
,
"orderLinkId"
:
"btc-earn-001"
}
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
stake_or_redeem
(
category
=
"FlexibleSaving"
,
orderType
=
"Redeem"
,
accountType
=
"FUND"
,
amount
=
"0.35"
,
coin
=
"BTC"
,
productId
=
"430"
,
orderLinkId
=
"btc-earn-001"
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
"orderId"
:
"0572b030-6a0b-423f-88c4-b6ce31c0c82d"
,
"orderLinkId"
:
"btc-earn-001"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1739936607117
}

**Examples:**

Example 1 ():
```
POST /v5/earn/place-order HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739936605822X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 190{    "category": "FlexibleSaving",    "orderType": "Redeem",    "accountType": "FUND",    "amount": "0.35",    "coin": "BTC",    "productId": "430",    "orderLinkId": "btc-earn-001"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.stake_or_redeem(    category="FlexibleSaving",    orderType="Redeem",    accountType="FUND",    amount="0.35",    coin="BTC",    productId="430",    orderLinkId="btc-earn-001"))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "orderId": "0572b030-6a0b-423f-88c4-b6ce31c0c82d",        "orderLinkId": "btc-earn-001"    },    "retExtInfo": {},    "time": 1739936607117}
```

---

## Get RFQ Configuration

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/rfq-config

**Contents:**
- Get RFQ Configuration
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get RFQ Configuration
On this page
Get RFQ Configuration
RFQ Config.
Up to 50 requests
per second.
info
Query for information on the quoting party that can participate in your transaction, your own deskCode and other configuration information.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/rfq/config
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
result
array
Order ID
list
Object
> deskCode
string
Your deskCode, a unique identification code
> maxLegs
integer
Maximum number of legs
> maxLP
integer
The maximum number of LPs (liquidity providers) selected in the inquiry
> maxActiveRfq
integer
The maximum number of unfinished inquiry orders allowed by a user
> rfqExpireTime
integer
Inquiry expiration time (mins)
> minLimitQtySpotOrder
integer
Spot minimum order quantity
>minLimitQtyContractOrder
integer
Contract minimum order quantity
> minLimitQtyOptionOrder
integer
Option minimum order
> strategyTypes
array
Product strategy
>> strategyName
string
Strategy name
> counterparties
array
Information on the quoters who can participate in the transaction
>> traderName
string
Name of the quoter
>> deskCode
string
The unique identification code of the quoting party
>> type
string
Quoter type.
LP
is an automated market maker connected via API, null means a normal quoting party
Request Example
​
GET
/v5/rfq/create-rfq
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676430842094
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"deskCode"
:
"1nu9d1"
,
"maxLegs"
:
25
,
"maxLP"
:
50
,
"rfqExpireTime"
:
10
,
"maxActiveRfq"
:
10
,
"minLimitQtySpotOrder"
:
10
,
"minLimitQtyContractOrder"
:
10
,
"minLimitQtyOptionOrder"
:
1
,
"strategyTypes"
:
[
{
"strategyName"
:
"custom"
}
,
{
"strategyName"
:
"FundingRate"
}
,
{
"strategyName"
:
"CarryTrade"
}
,
...
,
]
,
"counterparties"
:
[
{
"traderName"
:
"1zQkH0y7Y3acALM"
,
"deskCode"
:
"gIMhjitYqE9WG5F"
,
"type"
:
"LP"
}
,
{
"traderName"
:
"Bernie LP"
,
"deskCode"
:
"Bernie"
,
"type"
:
"LP"
}
,
...
,
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
1756870672013
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/create-rfq HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "deskCode": "1nu9d1",        "maxLegs": 25,        "maxLP": 50,        "rfqExpireTime": 10,        "maxActiveRfq": 10,        "minLimitQtySpotOrder": 10,        "minLimitQtyContractOrder": 10,        "minLimitQtyOptionOrder": 1,        "strategyTypes": [            {                "strategyName": "custom"            },            {                "strategyName": "FundingRate"            },            {                "strategyName": "CarryTrade"            },            ...,        ],        "counterparties": [            {                "traderName": "1zQkH0y7Y3acALM",                "deskCode": "gIMhjitYqE9WG5F",                "type": "LP"            },            {                "traderName": "Bernie LP",                "deskCode": "Bernie",                "type": "LP"            },            ...,        ]    },    "retExtInfo": {},    "time": 1756870672013}
```

---

## Get Open Interest

**URL:** https://bybit-exchange.github.io/docs/v5/market/open-interest

**Contents:**
- Get Open Interest
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Open Interest
On this page
Get Open Interest
Get the
open interest
of each symbol.
Covers: USDT contract / USDC contract / Inverse contract
info
The upper limit time you can query is the launch time of the symbol.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/open-interest
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
linear
,
inverse
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
intervalTime
true
string
Interval time.
5min
,
15min
,
30min
,
1h
,
4h
,
1d
startTime
false
integer
The start timestamp (ms)
endTime
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
200
]
. Default:
50
cursor
false
string
Cursor. Used to paginate
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
symbol
string
Symbol name
list
array
Object
> openInterest
string
Open interest. The value is the sum of both sides.
The unit of value, e.g., BTCUSD(inverse) is USD, BTCUSDT(linear) is BTC
> timestamp
string
The timestamp (ms)
nextPageCursor
string
Used to paginate
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/open-interest?category=inverse&symbol=BTCUSD&intervalTime=5min&startTime=1669571100000&endTime=1669571400000
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_open_interest
(
category
=
"inverse"
,
symbol
=
"BTCUSD"
,
intervalTime
=
"5min"
,
startTime
=
1669571100000
,
endTime
=
1669571400000
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetOpenInterests
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
openInterest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
marketInterval
(
MarketInterval
.
FIVE_MINUTES
)
.
build
(
)
;
client
.
getOpenInterest
(
openInterest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getOpenInterest
(
{
category
:
'inverse'
,
symbol
:
'BTCUSD'
,
intervalTime
:
'5min'
,
startTime
:
1669571100000
,
endTime
:
1669571400000
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"symbol"
:
"BTCUSD"
,
"category"
:
"inverse"
,
"list"
:
[
{
"openInterest"
:
"461134384.00000000"
,
"timestamp"
:
"1669571400000"
}
,
{
"openInterest"
:
"461134292.00000000"
,
"timestamp"
:
"1669571100000"
}
]
,
"nextPageCursor"
:
""
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672053548579
}

**Examples:**

Example 1 ():
```
GET /v5/market/open-interest?category=inverse&symbol=BTCUSD&intervalTime=5min&startTime=1669571100000&endTime=1669571400000 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_open_interest(    category="inverse",    symbol="BTCUSD",    intervalTime="5min",    startTime=1669571100000,    endTime=1669571400000,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetOpenInterests(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var openInterest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.FIVE_MINUTES).build();client.getOpenInterest(openInterest, System.out::println);
```

---

## Get LTV

**URL:** https://bybit-exchange.github.io/docs/v5/otc/ltv-convert

**Contents:**
- Get LTV
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Institutional Loan
Get LTV
On this page
Get LTV
Get your loan-to-value (LTV) ratio.
important
In cases where an institutional user makes frequent transfers, LTV calculations may become inaccurate, and this endpoint will return retCode = 100016, retMsg = "Transfers within your risk unit are too frequent. Please reduce the transfer frequency and try again."
If you encounter this error, it is recommended to reduce the transfer frequency first and retry
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
When a user is in a state such as liquidation, transfer, or manual repayment, LTV is not calculated. We have added a new
liqStatus
to represent these states. When
liqStatus
!= 0,
ltvInfo
returns empty strings for
ltv
,
unpaidAmount
and
balance
, and
unpaidInfo
and
balanceInfo
return empty arrays.
HTTP Request
​
GET
/v5/ins-loan/ltv-convert
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
ltvInfo
array
Object
> ltv
string
Risk rate
ltv is calculated in real time
If you have an INS loan, it is highly recommended to query this data every second. Liquidation occurs when it reachs 0.9 (90%)
. When
liqStatus
!= 0, empty string is returned.
> rst
string
Remaining liquidation time (UTC time in seconds). When it is not triggered, it is displayed as an empty string. When
liqStatus
!= 0, empty string is returned.
> parentUid
string
The designated Risk Unit ID that was used to bind with the INS loan
> subAccountUids
array
Bound user ID
> unpaidAmount
string
Total debt(USDT). When
liqStatus
!= 0, empty string is returned.
> unpaidInfo
array
Debt details. When
liqStatus
!= 0, empty array is returned.
>> token
string
coin
>> unpaidQty
string
Unpaid principle
>> unpaidInterest
string
Useless field, please ignore this for now
> balance
string
Total asset (margin coins converted to USDT). Please read
here
to understand the calculation. When
liqStatus
!= 0, empty string is returned.
> balanceInfo
array
Asset details. When
liqStatus
!= 0, empty array is returned.
>> token
string
Margin coin
>> price
string
Margin coin price
>> qty
string
Margin coin quantity
>> convertedAmount
string
Margin conversion amount
> liqStatus
integer
Liquidation status.
0
: Normal
1
: Under liquidation
2
: Manual repayment in progress
3
: Transfer in progress
liqStatus
integer
Liquidation status.
0
: Normal
1
: Under liquidation
2
: Manual repayment in progress
3
: Transfer in progress
Request Example
​
HTTP
Python
Node.js
GET
/v5/ins-loan/ltv-convert
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1686638165351
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXX
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_ltv
(
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getInstitutionalLendingLTVWithLadderConversionRate
(
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
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
"ltvInfo"
:
[
{
"ltv"
:
"0.75"
,
"rst"
:
""
,
"parentUid"
:
"xxxxx"
,
"subAccountUids"
:
[
"60568258"
]
,
"unpaidAmount"
:
"30"
,
"unpaidInfo"
:
[
{
"token"
:
"USDT"
,
"unpaidQty"
:
"30"
,
"unpaidInterest"
:
"0"
}
]
,
"balance"
:
"40"
,
"balanceInfo"
:
[
{
"token"
:
"USDT"
,
"price"
:
"1"
,
"qty"
:
"40"
,
"convertedAmount"
:
"40"
}
]
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
1686638166323
}
When `liqStatus` !=
0
:
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
"ltvInfo"
:
[
{
"ltv"
:
""
,
"parentUid"
:
"100331354"
,
"subAccountUids"
:
[
"100334094"
,
"100334098"
]
,
"unpaidAmount"
:
""
,
"unpaidInfo"
:
[
]
,
"balance"
:
""
,
"balanceInfo"
:
[
]
,
"rst"
:
""
,
"liqStatus"
:
3
}
]
,
"liqStatus"
:
3
}
,
"retExtInfo"
:
{
}
,
"time"
:
1766462020703
}

**Examples:**

Example 1 ():
```
GET /v5/ins-loan/ltv-convert HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1686638165351X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_ltv())
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInstitutionalLendingLTVWithLadderConversionRate()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "ltvInfo": [            {                "ltv": "0.75",                "rst": "",                "parentUid": "xxxxx",                "subAccountUids": [                    "60568258"                ],                "unpaidAmount": "30",                "unpaidInfo": [                    {                        "token": "USDT",                        "unpaidQty": "30",                        "unpaidInterest": "0"                    }                ],                "balance": "40",                "balanceInfo": [                    {                        "token": "USDT",                        "price": "1",                        "qty": "40",                        "convertedAmount": "40"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1686638166323}When `liqStatus` != 0:{    "retCode": 0,    "retMsg": "",    "result": {        "ltvInfo": [            {                "ltv": "",                "parentUid": "100331354",                "subAccountUids": [                    "100334094",                    "100334098"                ],                "unpaidAmount": "",                "unpaidInfo": [],                "balance": "",                "balanceInfo": [],                "rst": "",                "liqStatus": 3            }        ],        "liqStatus": 3    },    "retExtInfo": {},    "time": 1766462020703}
```

---

## Get All Coins Balance

**URL:** https://bybit-exchange.github.io/docs/v5/asset/balance/all-balance

**Contents:**
- Get All Coins Balance
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Balances
Get All Coins Balance
On this page
Get All Coins Balance
You could get all coin balance of all account types under the master account, and sub account.
important
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/asset/transfer/query-account-coins-balance
Request Parameters
​
Parameter
Required
Type
Comments
memberId
false
string
User Id. It is
required
when you use master api key to check sub account coin balance
accountType
true
string
Account type
coin
false
string
Coin name, uppercase only
Query all coins if not passed
Can query multiple coins, separated by comma.
USDT,USDC,ETH
Note:
this field is
mandatory
for accountType=
UNIFIED
, and supports up to 10 coins each request
withBonus
false
integer
0
(default): not query bonus.
1
: query bonus
Response Parameters
​
Parameter
Type
Comments
accountType
string
Account type
memberId
string
UserID
balance
array
Object
> coin
string
Currency
> walletBalance
string
Wallet balance
> transferBalance
string
Transferable balance
> bonus
string
Bonus
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/transfer/query-account-coins-balance?accountType=FUND&coin=USDC
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1675866354698
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_coins_balance
(
accountType
=
"FUND"
,
coin
=
"USDC"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getAllCoinsBalance
(
{
accountType
:
'FUND'
,
coin
:
'USDC'
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"success"
,
"result"
:
{
"memberId"
:
"XXXX"
,
"accountType"
:
"FUND"
,
"balance"
:
[
{
"coin"
:
"USDC"
,
"transferBalance"
:
"0"
,
"walletBalance"
:
"0"
,
"bonus"
:
""
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
1675866354913
}

**Examples:**

Example 1 ():
```
GET /v5/asset/transfer/query-account-coins-balance?accountType=FUND&coin=USDC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675866354698X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_coins_balance(    accountType="FUND",    coin="USDC",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getAllCoinsBalance({ accountType: 'FUND', coin: 'USDC' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "memberId": "XXXX",        "accountType": "FUND",        "balance": [            {                "coin": "USDC",                "transferBalance": "0",                "walletBalance": "0",                "bonus": ""            }        ]    },    "retExtInfo": {},    "time": 1675866354913}
```

---

## Kline

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/kline

**Contents:**
- Kline
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
Kline
On this page
Kline
Subscribe to the klines stream.
tip
If
confirm
=true, this means that the candle has closed. Otherwise, the candle is still open and updating.
Available intervals:
1
3
5
15
30
(min)
60
120
240
360
720
(min)
D
(day)
W
(week)
M
(month)
Push frequency:
1-60s
Topic:
kline.{interval}.{symbol}
e.g., kline.30.BTCUSDT
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
array
Object
> start
number
The start timestamp (ms)
> end
number
The end timestamp (ms)
>
interval
string
Kline interval
> open
string
Open price
> close
string
Close price
> high
string
Highest price
> low
string
Lowest price
> volume
string
Trade volume
> turnover
string
Turnover
> confirm
boolean
Whether the tick is ended or not
> timestamp
number
The timestamp (ms) of the last matched order in the candle
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
kline_stream
(
interval
=
5
,
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
{
"topic"
:
"kline.5.BTCUSDT"
,
"data"
:
[
{
"start"
:
1672324800000
,
"end"
:
1672325099999
,
"interval"
:
"5"
,
"open"
:
"16649.5"
,
"close"
:
"16677"
,
"high"
:
"16677"
,
"low"
:
"16608"
,
"volume"
:
"2.081"
,
"turnover"
:
"34666.4005"
,
"confirm"
:
false
,
"timestamp"
:
1672324988882
}
]
,
"ts"
:
1672324988882
,
"type"
:
"snapshot"
}

**Examples:**

Example 1 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.kline_stream(    interval=5,    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 2 ():
```
{    "topic": "kline.5.BTCUSDT",    "data": [        {            "start": 1672324800000,            "end": 1672325099999,            "interval": "5",            "open": "16649.5",            "close": "16677",            "high": "16677",            "low": "16608",            "volume": "2.081",            "turnover": "34666.4005",            "confirm": false,            "timestamp": 1672324988882        }    ],    "ts": 1672324988882,    "type": "snapshot"}
```

---

## Set MMP

**URL:** https://bybit-exchange.github.io/docs/v5/account/set-mmp

**Contents:**
- Set MMP
- What is MMP?​
- How to enable MMP​
- Applicable​
- Some points to note​
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Set MMP
On this page
Set MMP
info
What is MMP?
​
Market Maker Protection
(MMP) is an automated mechanism designed to protect market makers (MM) against liquidity risks
and over-exposure in the market. It prevents simultaneous trade executions on quotes provided by the MM within a short time span.
The MM can automatically pull their quotes if the number of contracts traded for an underlying asset exceeds the configured
threshold within a certain time frame. Once MMP is triggered, any pre-existing MMP orders will be
automatically cancelled
,
and new orders tagged as MMP will be
rejected
for a specific duration — known as the frozen period — so that MM can
reassess the market and modify the quotes.
How to enable MMP
​
Send an email to Bybit (
financial.inst@bybit.com
) or contact your business development (BD) manager to apply for MMP.
After processed, the default settings are as below table:
Parameter
Type
Comments
Default value
baseCoin
string
Base coin
BTC
window
string
Time window (millisecond)
5000
frozenPeriod
string
Frozen period (millisecond)
100
qtyLimit
string
Quantity limit
100
deltaLimit
string
Delta limit
100
Applicable
​
Effective for
options
only. When you place an
option
order, set
mmp
=true, which means you mark this order as a mmp order.
Some points to note
​
Only maker order qty and delta will be counted into
qtyLimit
and
deltaLimit
.
qty_limit
is the sum of absolute value of qty of each trade executions.
delta_limit
is the absolute value of the sum of qty*delta. If any of these reaches or exceeds the limit amount, the account's market maker protection will be triggered.
HTTP Request
​
POST
/v5/account/mmp-modify
Request Parameters
​
Parameter
Required
Type
Comments
baseCoin
true
string
Base coin, uppercase only
window
true
string
Time window (ms)
frozenPeriod
true
string
Frozen period (ms). "0" means the trade will remain frozen until manually reset
qtyLimit
true
string
Trade qty limit (positive and up to 2 decimal places)
deltaLimit
true
string
Delta limit (positive and up to 2 decimal places)
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/account/mmp-modify
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1675833524616
X-BAPI-RECV-WINDOW
:
50000
Content-Type
:
application/json
{
"baseCoin"
:
"ETH"
,
"window"
:
"5000"
,
"frozenPeriod"
:
"100000"
,
"qtyLimit"
:
"50"
,
"deltaLimit"
:
"20"
}
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
set_mmp
(
baseCoin
=
"ETH"
,
window
=
"5000"
,
frozenPeriod
=
"100000"
,
qtyLimit
=
"50"
,
deltaLimit
=
"20"
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
setMMP
(
{
baseCoin
:
'ETH'
,
window
:
'5000'
,
frozenPeriod
:
'100000'
,
qtyLimit
:
'50'
,
deltaLimit
:
'20'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"success"
}

**Examples:**

Example 1 ():
```
POST /v5/account/mmp-modify HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675833524616X-BAPI-RECV-WINDOW: 50000Content-Type: application/json{    "baseCoin": "ETH",    "window": "5000",    "frozenPeriod": "100000",    "qtyLimit": "50",    "deltaLimit": "20"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_mmp(    baseCoin="ETH",    window="5000",    frozenPeriod="100000",    qtyLimit="50",    deltaLimit="20"))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setMMP({        baseCoin: 'ETH',        window: '5000',        frozenPeriod: '100000',        qtyLimit: '50',        deltaLimit: '20',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success"}
```

---

## Get Orderbook

**URL:** https://bybit-exchange.github.io/docs/v5/market/orderbook

**Contents:**
- Get Orderbook
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Orderbook
On this page
Get Orderbook
Query for orderbook depth data.
Covers: Spot / USDT contract / USDC contract / Inverse contract / Option
Contract: 1000-level of orderbook data
Spot: 1000-level of orderbook data
Option: 25-level of orderbook data
info
The response is in the snapshot format.
Retail Price Improvement (RPI)
orders will not be included in the response message and will not be visible over API.
HTTP Request
​
GET
/v5/market/orderbook
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
spot
,
linear
,
inverse
,
option
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
limit
false
integer
Limit size for each bid and ask
spot
:
[
1
,
200
]
. Default:
1
.
linear
&
inverse
:
[
1
,
500
]
. Default:
25
.
option
:
[
1
,
25
]
. Default:
1
.
Response Parameters
​
Parameter
Type
Comments
s
string
Symbol name
b
array
Bid, buyer. Sorted by price in descending order
> b
[0]
string
Bid price
> b
[1]
string
Bid size
a
array
Ask, seller. Sorted by price in ascending order
> a
[0]
string
Ask price
> a
[1]
string
Ask size
ts
integer
The timestamp (ms) that the system generates the data
u
integer
Update ID, is always in sequence
For contract, corresponds to
u
in the 1000-level
WebSocket orderbook stream
For spot, corresponds to
u
in the 1000-level
WebSocket orderbook stream
seq
integer
Cross sequence
You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier.
cts
integer
The timestamp from the matching engine when this orderbook data is produced. It can be correlated with
T
from
public trade channel
RUN >>
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/orderbook?category=spot&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_orderbook
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
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetOrderBookInfo
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
orderbookRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
SPOT
)
.
symbol
(
"BTCUSDT"
)
.
build
(
)
;
client
.
getMarketOrderBook
(
orderbookRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getOrderbook
(
{
category
:
'linear'
,
symbol
:
'BTCUSDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"s"
:
"BTCUSDT"
,
"a"
:
[
[
"65557.7"
,
"16.606555"
]
]
,
"b"
:
[
[
"65485.47"
,
"47.081829"
]
]
,
"ts"
:
1716863719031
,
"u"
:
230704
,
"seq"
:
1432604333
,
"cts"
:
1716863718905
}
,
"retExtInfo"
:
{
}
,
"time"
:
1716863719382
}

**Examples:**

Example 1 ():
```
GET /v5/market/orderbook?category=spot&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_orderbook(    category="linear",    symbol="BTCUSDT",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetOrderBookInfo(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var orderbookRequest = MarketDataRequest.builder().category(CategoryType.SPOT).symbol("BTCUSDT").build();client.getMarketOrderBook(orderbookRequest,System.out::println);
```

---

## Ticker

**URL:** https://bybit-exchange.github.io/docs/v5/spread/websocket/public/ticker

**Contents:**
- Ticker
  - Response Parameters​
  - Subscribe Example​
  - Event Example​

Spread Trading
Websocket Stream
Public
Ticker
On this page
Ticker
Subscribe to the ticker stream.
Push frequency:
100ms
Topic:
tickers.{symbol}
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
map
Object
> symbol
string
Spread combination symbol name
> bidPrice
string
Bid 1 price
> bidSize
string
Bid 1 size
> askPrice
string
Ask 1 price
> askSize
string
Ask 1 size
> lastPrice
string
Last trade price
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> prevPrice24h
string
Price 24 hours ago
> volume24h
string
Volume for 24h
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
"tickers.SOLUSDT_SOL/USDT"
]
}
Event Example
​
{
"topic"
:
"tickers.SOLUSDT_SOL/USDT"
,
"ts"
:
1744168585009
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
"SOLUSDT_SOL/USDT"
,
"bidPrice"
:
"20.3359"
,
"bidSize"
:
"1.7"
,
"askPrice"
:
""
,
"askSize"
:
""
,
"lastPrice"
:
"21.8182"
,
"highPrice24h"
:
"24.2356"
,
"lowPrice24h"
:
"-3"
,
"prevPrice24h"
:
"22.1468"
,
"volume24h"
:
"23309.9"
}
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "tickers.SOLUSDT_SOL/USDT"    ]}
```

Example 2 ():
```
{    "topic": "tickers.SOLUSDT_SOL/USDT",    "ts": 1744168585009,    "type": "snapshot",    "data": {        "symbol": "SOLUSDT_SOL/USDT",        "bidPrice": "20.3359",        "bidSize": "1.7",        "askPrice": "",        "askSize": "",        "lastPrice": "21.8182",        "highPrice24h": "24.2356",        "lowPrice24h": "-3",        "prevPrice24h": "22.1468",        "volume24h": "23309.9"    }}
```

---

## Get Delivery Record

**URL:** https://bybit-exchange.github.io/docs/v5/asset/delivery

**Contents:**
- Get Delivery Record
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Get Delivery Record (2 years)
On this page
Get Delivery Record
Query delivery records of Invese Futures, USDC Futures, USDT Futures and Options, sorted by
deliveryTime
in descending order
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/asset/delivery-record
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type
inverse
(inverse futures),
linear
(USDT/USDC futures),
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 30 days by default
Only startTime is passed, return range between startTime and startTime + 30 days
Only endTime is passed, return range between endTime - 30 days and endTime
If both are passed, the rule is endTime - startTime <= 30 days
endTime
false
integer
The end timestamp (ms)
expDate
false
string
Expiry date.
25MAR22
. Default: return all
limit
false
integer
Limit for data size per page.
[
1
,
50
]
. Default:
20
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> deliveryTime
number
Delivery time (ms)
> symbol
string
Symbol name
> side
string
Buy
,
Sell
> position
string
Executed size
> entryPrice
string
Avg entry price
> deliveryPrice
string
Delivery price
> strike
string
Exercise price
> fee
string
Trading fee
> deliveryRpl
string
Realized PnL of the delivery
nextPageCursor
string
Refer to the
cursor
request parameter
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/delivery-record?expDate=29DEC22&category=option
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672362112944
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_option_delivery_record
(
category
=
"option"
,
expDate
=
"29DEC22"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getDeliveryRecord
(
{
category
:
'option'
,
expDate
:
'29DEC22'
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"nextPageCursor"
:
"132791%3A0%2C132791%3A0"
,
"category"
:
"option"
,
"list"
:
[
{
"symbol"
:
"BTC-29DEC22-16000-P"
,
"side"
:
"Buy"
,
"deliveryTime"
:
1672300800860
,
"strike"
:
"16000"
,
"fee"
:
"0.00000000"
,
"position"
:
"0.01"
,
"deliveryPrice"
:
"16541.86369547"
,
"deliveryRpl"
:
"3.5"
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
1672362116184
}

**Examples:**

Example 1 ():
```
GET /v5/asset/delivery-record?expDate=29DEC22&category=option HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672362112944X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_option_delivery_record(    category="option",    expDate="29DEC22",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getDeliveryRecord({ category: 'option', expDate: '29DEC22' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "132791%3A0%2C132791%3A0",        "category": "option",        "list": [            {                "symbol": "BTC-29DEC22-16000-P",                "side": "Buy",                "deliveryTime": 1672300800860,                "strike": "16000",                "fee": "0.00000000",                "position": "0.01",                "deliveryPrice": "16541.86369547",                "deliveryRpl": "3.5"            }        ]    },    "retExtInfo": {},    "time": 1672362116184}
```

---

## Get Insurance Pool

**URL:** https://bybit-exchange.github.io/docs/v5/market/insurance

**Contents:**
- Get Insurance Pool
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Insurance Pool
On this page
Get Insurance Pool
Query for Bybit
insurance pool
data (BTC/USDT/USDC etc)
info
The isolated insurance pool balance is updated every 1 minute, and shared insurance pool balance is updated every 24 hours
Please note that you may receive data from the previous minute. This is due to multiple backend containers starting
at different times, which may cause a slight delay. You can always rely on the latest minute data for accuracy.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/insurance
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
coin, uppercase only. Default: return all insurance coins
Response Parameters
​
Parameter
Type
Comments
updatedTime
string
Data updated time (ms)
list
array
Object
> coin
string
Coin
> symbols
string
symbols with
"BTCUSDT,ETHUSDT,SOLUSDT"
mean these contracts are shared with one insurance pool
For an isolated insurance pool, it returns one contract
> balance
string
Balance
> value
string
USD value
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/insurance?coin=USDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_insurance
(
coin
=
"USDT"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarketInsurance
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
insuranceRequest
=
MarketDataRequest
.
builder
(
)
.
coin
(
"BTC"
)
.
build
(
)
;
var
insuranceData
=
client
.
getInsurance
(
insuranceRequest
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getInsurance
(
{
coin
:
'USDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"updatedTime"
:
"1714003200000"
,
"list"
:
[
{
"coin"
:
"USDT"
,
"symbols"
:
"MERLUSDT,10000000AIDOGEUSDT,ZEUSUSDT"
,
"balance"
:
"902178.57602476"
,
"value"
:
"901898.0963091522"
}
,
{
"coin"
:
"USDT"
,
"symbols"
:
"SOLUSDT,OMNIUSDT,ALGOUSDT"
,
"balance"
:
"14454.51626125"
,
"value"
:
"14449.515598975464"
}
,
{
"coin"
:
"USDT"
,
"symbols"
:
"XLMUSDT,WUSDT"
,
"balance"
:
"23.45018235"
,
"value"
:
"22.992864174376344"
}
,
{
"coin"
:
"USDT"
,
"symbols"
:
"AGIUSDT,WIFUSDT"
,
"balance"
:
"10002"
,
"value"
:
"9998.896846613574"
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
1714028451228
}

**Examples:**

Example 1 ():
```
GET /v5/market/insurance?coin=USDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_insurance(    coin="USDT",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetMarketInsurance(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var insuranceRequest = MarketDataRequest.builder().coin("BTC").build();var insuranceData = client.getInsurance(insuranceRequest);
```

---

## Get RPI Orderbook

**URL:** https://bybit-exchange.github.io/docs/v5/market/rpi-orderbook

**Contents:**
- Get RPI Orderbook
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get RPI Orderbook
On this page
Get RPI Orderbook
Query for orderbook depth data.
Covers: Spot / USDT contract / USDC contract / Inverse contract /
Contract: 50-level of RPI orderbook data
Spot: 50-level of RPI orderbook data
info
The response is in the snapshot format.
HTTP Request
​
GET
/v5/market/rpi_orderbook
Request Parameters
​
Parameter
Required
Type
Comments
category
false
string
Product type.
spot
,
linear
,
inverse
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
limit
true
integer
Limit size for each bid and ask:
[1, 50]
Response Parameters
​
Parameter
Type
Comments
s
string
Symbol name
> b
array
Bids. For
snapshot
stream. Sorted by price in descending order
>> b
[0]
string
Bid price
>> b
[1]
string
None RPI bid size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
>> b
[2]
string
RPI bid size
When a bid RPI order crosses with a non-RPI ask price, the quantity of the bid RPI becomes invalid and is hidden
> a
array
Asks. For
snapshot
stream. Sorted by price in ascending order
>> a
[0]
string
Ask price
>> a
[1]
string
None RPI ask size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
>> a
[2]
string
RPI ask size
When an ask RPI order crosses with a non-RPI bid price, the quantity of the ask RPI becomes invalid and is hidden
ts
integer
The timestamp (ms) that the system generates the data
u
integer
Update ID, is always in sequence corresponds to
u
in the 50-level
WebSocket RPI orderbook stream
seq
integer
Cross sequence
You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier.
cts
integer
The timestamp from the matching engine when this orderbook data is produced. It can be correlated with
T
from
public trade channel
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/rpi_orderbook?category=spot&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"s"
:
"BTCUSDT"
,
"a"
:
[
[
"116600.00"
,
"4.428"
,
"0.000"
]
]
,
"b"
:
[
[
"116599.90"
,
"3.721"
,
"0.000"
]
]
,
"ts"
:
1758078286128
,
"u"
:
28419362
,
"seq"
:
454803359210
,
"cts"
:
1758078286118
}
,
"retExtInfo"
:
{
}
,
"time"
:
1758078286162
}

**Examples:**

Example 1 ():
```
GET /v5/market/rpi_orderbook?category=spot&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```

```

---

## Get Risk Limit

**URL:** https://bybit-exchange.github.io/docs/v5/market/risk-limit

**Contents:**
- Get Risk Limit
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Risk Limit
On this page
Get Risk Limit
Query for the
risk limit
margin parameters. This information is also displayed on the website
here
.
Covers: USDT contract / USDC contract / Inverse contract
info
category=
linear
returns a data set of 15 symbols in each response. Please use the
cursor
param to get the next data set.
symbol
support
Trading
status and
PreLaunch
Pre-Market contracts
status trading pairs.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/risk-limit
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
linear
,
inverse
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the data set
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> id
integer
Risk ID
> symbol
string
Symbol name
> riskLimitValue
string
Position limit
> maintenanceMargin
number
Maintain margin rate
> initialMargin
number
Initial margin rate
> isLowestRisk
integer
1
: true,
0
: false
> maxLeverage
string
Allowed max leverage
> mmDeduction
string
The maintenance margin deduction value when risk limit tier changed
nextPageCursor
string
Refer to the
cursor
request parameter
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/risk-limit?category=inverse&symbol=BTCUSD
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_risk_limit
(
category
=
"inverse"
,
symbol
=
"BTCUSD"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarketRiskLimits
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
riskMimitRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
INVERSE
)
.
symbol
(
"ADAUSD"
)
.
build
(
)
;
client
.
getRiskLimit
(
riskMimitRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getRiskLimit
(
{
category
:
'inverse'
,
symbol
:
'BTCUSD'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"inverse"
,
"list"
:
[
{
"id"
:
1
,
"symbol"
:
"BTCUSD"
,
"riskLimitValue"
:
"150"
,
"maintenanceMargin"
:
"0.5"
,
"initialMargin"
:
"1"
,
"isLowestRisk"
:
1
,
"maxLeverage"
:
"100.00"
,
"mmDeduction"
:
""
}
,
....
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
1672054488010
}

**Examples:**

Example 1 ():
```
GET /v5/market/risk-limit?category=inverse&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_risk_limit(    category="inverse",    symbol="BTCUSD",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetMarketRiskLimits(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var riskMimitRequest = MarketDataRequest.builder().category(CategoryType.INVERSE).symbol("ADAUSD").build();client.getRiskLimit(riskMimitRequest, System.out::println);
```

---

## Orderbook

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/orderbook

**Contents:**
- Orderbook
  - Depths​
  - Process snapshot/delta​
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
Orderbook
On this page
Orderbook
Subscribe to the orderbook stream. Supports different depths.
info
Retail Price Improvement (RPI)
orders will not be included in the messages.
Depths
​
Linear & inverse:
Level 1 data, push frequency:
10ms
Level 50 data, push frequency:
20ms
Level 200 data, push frequency:
100ms
Level 1000 data, push frequency:
200ms
Spot:
Level 1 data, push frequency:
10ms
Level 50 data, push frequency:
20ms
Level 200 data, push frequency:
200ms
Level 1000 data, push frequency:
200ms
Option:
Level 25 data, push frequency:
20ms
Level 100 data, push frequency:
100ms
Topic:
orderbook.{depth}.{symbol}
e.g., orderbook.1.BTCUSDT
Process snapshot/delta
​
To process
snapshot
and
delta
messages, please follow these rules:
Once you have subscribed successfully, you will receive a
snapshot
. The WebSocket will keep pushing
delta
messages every time the orderbook changes. If you receive a new
snapshot
message, you will have to reset your local orderbook. If there is a problem on Bybit's end, a
snapshot
will be re-sent, which is guaranteed to contain the latest data.
To apply
delta
updates:
If you receive an amount that is
0
, delete the entry
If you receive an amount that does not exist, insert it
If the entry exists, you simply update the value
See working code examples of this logic in the
FAQ
.
info
Linear, inverse, spot level 1 data: if 3 seconds have elapsed without a change in the orderbook, a
snapshot
message will be pushed again, and the field
u
will be the
same as that in the previous message.
Linear, inverse, spot level 1 data has
snapshot
message only
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
,
delta
ts
number
The timestamp (ms) that the system generates the data
data
map
Object
> s
string
Symbol name
> b
array
Bids. For
snapshot
stream. Sorted by price in descending order
>> b
[0]
string
Bid price
>> b
[1]
string
Bid size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
> a
array
Asks. For
snapshot
stream. Sorted by price in ascending order
>> a
[0]
string
Ask price
>> a
[1]
string
Ask size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
> u
integer
Update ID
Occasionally, you'll receive "u"=1, which is a snapshot data due to the restart of the service. So please overwrite your local orderbook
For level 1 of linear, inverse Perps and Futures, the snapshot data will be pushed again when there is no change in 3 seconds, and the "u" will be the same as that in the previous message
> seq
integer
Cross sequence
You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier.
cts
number
The timestamp from the matching engine when this orderbook data is produced. It can be correlated with
T
from
public trade channel
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
orderbook_stream
(
depth
=
50
,
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
Snapshot
Delta
{
"topic"
:
"orderbook.50.BTCUSDT"
,
"type"
:
"snapshot"
,
"ts"
:
1672304484978
,
"data"
:
{
"s"
:
"BTCUSDT"
,
"b"
:
[
...
,
[
"16493.50"
,
"0.006"
]
,
[
"16493.00"
,
"0.100"
]
]
,
"a"
:
[
[
"16611.00"
,
"0.029"
]
,
[
"16612.00"
,
"0.213"
]
,
...
,
]
,
"u"
:
18521288
,
"seq"
:
7961638724
}
,
"cts"
:
1672304484976
}
{
"topic"
:
"orderbook.50.BTCUSDT"
,
"type"
:
"delta"
,
"ts"
:
1687940967466
,
"data"
:
{
"s"
:
"BTCUSDT"
,
"b"
:
[
[
"30247.20"
,
"30.028"
]
,
[
"30245.40"
,
"0.224"
]
,
[
"30242.10"
,
"1.593"
]
,
[
"30240.30"
,
"1.305"
]
,
[
"30240.00"
,
"0"
]
]
,
"a"
:
[
[
"30248.70"
,
"0"
]
,
[
"30249.30"
,
"0.892"
]
,
[
"30249.50"
,
"1.778"
]
,
[
"30249.60"
,
"0"
]
,
[
"30251.90"
,
"2.947"
]
,
[
"30252.20"
,
"0.659"
]
,
[
"30252.50"
,
"4.591"
]
]
,
"u"
:
177400507
,
"seq"
:
66544703342
}
,
"cts"
:
1687940967464
}

**Examples:**

Example 1 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.orderbook_stream(    depth=50,    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 2 ():
```
{    "topic": "orderbook.50.BTCUSDT",    "type": "snapshot",    "ts": 1672304484978,    "data": {        "s": "BTCUSDT",        "b": [            ...,            [                "16493.50",                "0.006"            ],            [                "16493.00",                "0.100"            ]        ],        "a": [            [                "16611.00",                "0.029"            ],            [                "16612.00",                "0.213"            ],            ...,        ],    "u": 18521288,    "seq": 7961638724    },    "cts": 1672304484976}
```

Example 3 ():
```
{    "topic": "orderbook.50.BTCUSDT",    "type": "delta",    "ts": 1687940967466,    "data": {        "s": "BTCUSDT",        "b": [            [                "30247.20",                "30.028"            ],            [                "30245.40",                "0.224"            ],            [                "30242.10",                "1.593"            ],            [                "30240.30",                "1.305"            ],            [                "30240.00",                "0"            ]        ],        "a": [            [                "30248.70",                "0"            ],            [                "30249.30",                "0.892"            ],            [                "30249.50",                "1.778"            ],            [                "30249.60",                "0"            ],            [                "30251.90",                "2.947"            ],            [                "30252.20",                "0.659"            ],            [                "30252.50",                "4.591"            ]        ],        "u": 177400507,        "seq": 66544703342    },    "cts": 1687940967464}
```

---

## Get Mark Price Kline

**URL:** https://bybit-exchange.github.io/docs/v5/market/mark-kline

**Contents:**
- Get Mark Price Kline
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Mark Price Kline
On this page
Get Mark Price Kline
Query for historical
mark price
klines. Charts are returned in groups based on the requested interval.
Covers: USDT contract / USDC contract / Inverse contract
HTTP Request
​
GET
/v5/market/mark-price-kline
Request Parameters
​
Parameter
Required
Type
Comments
category
false
string
Product type.
linear
,
inverse
When
category
is not passed, use
linear
by default
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
interval
true
string
Kline interval.
1
,
3
,
5
,
15
,
30
,
60
,
120
,
240
,
360
,
720
,
D
,
M
,
W
start
false
integer
The start timestamp (ms)
end
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
200
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
symbol
string
Symbol name
list
array
An string array of individual candle
Sort in reverse by
startTime
> list
[0]
: startTime
string
Start time of the candle (ms)
> list
[1]
: openPrice
string
Open price
> list
[2]
: highPrice
string
Highest price
> list
[3]
: lowPrice
string
Lowest price
> list
[4]
: closePrice
string
Close price.
Is the last traded price when the candle is not closed
RUN >>
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/mark-price-kline?category=linear&symbol=BTCUSDT&interval=15&start=1670601600000&end=1670608800000&limit=1
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_mark_price_kline
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
interval
=
15
,
start
=
1670601600000
,
end
=
1670608800000
,
limit
=
1
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"interval"
:
"1"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarkPriceKline
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
marketKLineRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
marketInterval
(
MarketInterval
.
WEEKLY
)
.
build
(
)
;
client
.
getMarketPriceLinesData
(
marketKLineRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getMarkPriceKline
(
{
category
:
'linear'
,
symbol
:
'BTCUSD'
,
interval
:
'15'
,
start
:
1670601600000
,
end
:
1670608800000
,
limit
:
1
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"symbol"
:
"BTCUSDT"
,
"category"
:
"linear"
,
"list"
:
[
[
"1670608800000"
,
"17164.16"
,
"17164.16"
,
"17121.5"
,
"17131.64"
]
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
1672026361839
}

**Examples:**

Example 1 ():
```
GET /v5/market/mark-price-kline?category=linear&symbol=BTCUSDT&interval=15&start=1670601600000&end=1670608800000&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_mark_price_kline(    category="linear",    symbol="BTCUSDT",    interval=15,    start=1670601600000,    end=1670608800000,    limit=1,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetMarkPriceKline(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getMarketPriceLinesData(marketKLineRequest, System.out::println);
```

---

## Get Wallet Balance

**URL:** https://bybit-exchange.github.io/docs/v5/account/wallet-balance

**Contents:**
- Get Wallet Balance
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Get Wallet Balance
On this page
Get Wallet Balance
Obtain wallet balance, query asset information of each currency. By default, currency
information with assets or liabilities of 0 is not returned.
info
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
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/account/wallet-balance
Request Parameters
​
Parameter
Required
Type
Comments
accountType
true
string
Account type
UNIFIED
. To get Funding wallet balance, please go to this
endpoint
coin
false
string
Coin name, uppercase only
If not passed, it returns non-zero asset info
You can pass multiple coins to query, separated by comma.
USDT,USDC
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> accountType
string
Account type
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
Porfolio Margin: total Equity - Haircut - totalInitialMargin.
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
Equity of coin.  Asset Equity = Asset Wallet Balance + Asset Perp UPL + Asset Future UPL + Asset Option Value =
walletBalance
-
spotBorrow
+
unrealisedPnl
+ Asset Option Value
>> usdValue
string
USD value of coin
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
Borrow amount of current coin = spot liabilities + derivatives liabilities
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
>> marginCollateral
boolean
Whether it can be used as a margin collateral currency (platform),
true
: YES,
false
: NO
When marginCollateral=false, then collateralSwitch is meaningless
>> collateralSwitch
boolean
Whether the collateral is turned on by user (user),
true
: ON,
false
: OFF
When marginCollateral=true, then collateralSwitch is meaningful
>> spotBorrow
string
Borrow amount by spot margin trade and manual borrow amount (does not include borrow amount by spot margin active order).
spotBorrow
field corresponding to spot liabilities is detailed in the
announcement
.
>> free
string
Deprecated
since there is no Spot wallet any more
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
>> availableToBorrow
string
Deprecated
field, always return
""
. Please refer to
availableToBorrow
in the
Get Collateral Info
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/account/wallet-balance?accountType=UNIFIED&coin=BTC
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672125440406
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_wallet_balance
(
accountType
=
"UNIFIED"
,
coin
=
"BTC"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getWalletBalance
(
{
accountType
:
'UNIFIED'
,
coin
:
'BTC'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"totalEquity"
:
"3.31216591"
,
"accountIMRate"
:
"0"
,
"accountIMRateByMp"
:
"0"
,
"totalMarginBalance"
:
"3.00326056"
,
"totalInitialMargin"
:
"0"
,
"totalInitialMarginByMp"
:
"0"
,
"accountType"
:
"UNIFIED"
,
"totalAvailableBalance"
:
"3.00326056"
,
"accountMMRate"
:
"0"
,
"accountMMRateByMp"
:
"0"
,
"totalPerpUPL"
:
"0"
,
"totalWalletBalance"
:
"3.00326056"
,
"accountLTV"
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
"availableToBorrow"
:
"3"
,
"bonus"
:
"0"
,
"accruedInterest"
:
"0"
,
"availableToWithdraw"
:
"0"
,
"totalOrderIM"
:
"0"
,
"equity"
:
"0"
,
"totalPositionMM"
:
"0"
,
"usdValue"
:
"0"
,
"spotHedgingQty"
:
"0.01592413"
,
"unrealisedPnl"
:
"0"
,
"collateralSwitch"
:
true
,
"borrowAmount"
:
"0.0"
,
"totalPositionIM"
:
"0"
,
"walletBalance"
:
"0"
,
"cumRealisedPnl"
:
"0"
,
"locked"
:
"0"
,
"marginCollateral"
:
true
,
"coin"
:
"BTC"
,
"spotBorrow"
:
"0"
}
]
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
1690872862481
}

**Examples:**

Example 1 ():
```
GET /v5/account/wallet-balance?accountType=UNIFIED&coin=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672125440406X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_wallet_balance(    accountType="UNIFIED",    coin="BTC",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');    const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getWalletBalance({        accountType: 'UNIFIED',        coin: 'BTC',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "totalEquity": "3.31216591",                "accountIMRate": "0",                "accountIMRateByMp": "0",                "totalMarginBalance": "3.00326056",                "totalInitialMargin": "0",                "totalInitialMarginByMp": "0",                "accountType": "UNIFIED",                "totalAvailableBalance": "3.00326056",                "accountMMRate": "0",                "accountMMRateByMp": "0",                "totalPerpUPL": "0",                "totalWalletBalance": "3.00326056",                "accountLTV": "0",                "totalMaintenanceMargin": "0",                "totalMaintenanceMarginByMp": "0",                "coin": [                    {                        "availableToBorrow": "3",                        "bonus": "0",                        "accruedInterest": "0",                        "availableToWithdraw": "0",                        "totalOrderIM": "0",                        "equity": "0",                        "totalPositionMM": "0",                        "usdValue": "0",                        "spotHedgingQty": "0.01592413",                        "unrealisedPnl": "0",                        "collateralSwitch": true,                        "borrowAmount": "0.0",                        "totalPositionIM": "0",                        "walletBalance": "0",                        "cumRealisedPnl": "0",                        "locked": "0",                        "marginCollateral": true,                        "coin": "BTC",                        "spotBorrow": "0"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1690872862481}
```

---

## Get Orderbook

**URL:** https://bybit-exchange.github.io/docs/v5/spread/market/orderbook

**Contents:**
- Get Orderbook
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Market
Get Orderbook
On this page
Get Orderbook
Query spread orderbook depth data.
HTTP Request
​
GET
/v5/spread/orderbook
Request Parameters
​
Parameter
Required
Type
Comments
symbol
true
string
Spread combination symbol name
limit
false
integer
Limit size for each bid and ask
[
1
,
25
]
. Default:
1
Response Parameters
​
Parameter
Type
Comments
s
string
Spread combination symbol name
b
array
Bid, buyer. Sorted by price in descending order
> b
[0]
string
Bid price
> b
[1]
string
Bid size
a
array
Ask, seller. Sorted by price in ascending order
> a
[0]
string
Ask price
> a
[1]
string
Ask size
ts
integer
The timestamp (ms) that the system generates the data
u
integer
Update ID. Is always in sequence. Corresponds to
u
in the 25-level
WebSocket orderbook stream
seq
integer
Cross sequence
cts
integer
The timestamp from the matching engine when this orderbook data is produced. It can be correlated with
T
from
public trade channel
Request Example
​
GET
/v5/spread/orderbook?symbol=SOLUSDT_SOL/USDT&limit=1
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"Success"
,
"result"
:
{
"s"
:
"SOLUSDT_SOL/USDT"
,
"b"
:
[
[
"21.0000"
,
"0.1"
]
]
,
"a"
:
[
[
"23.0107"
,
"4.6"
]
]
,
"u"
:
46977
,
"ts"
:
1744077242177
,
"seq"
:
213110
,
"cts"
:
1744076329043
}
,
"retExtInfo"
:
{
}
,
"time"
:
1744077243583
}

**Examples:**

Example 1 ():
```
GET /v5/spread/orderbook?symbol=SOLUSDT_SOL/USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "s": "SOLUSDT_SOL/USDT",        "b": [            [                "21.0000",                "0.1"            ]        ],        "a": [            [                "23.0107",                "4.6"            ]        ],        "u": 46977,        "ts": 1744077242177,        "seq": 213110,        "cts": 1744076329043    },    "retExtInfo": {},    "time": 1744077243583}
```

---

## Get Position Info

**URL:** https://bybit-exchange.github.io/docs/v5/position

**Contents:**
- Get Position Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Get Position Info
On this page
Get Position Info
Query real-time position data, such as position size, cumulative realized PNL, etc.
info
category="inverse"
You can query all open positions with
/v5/position/list?category=inverse
;
Cannot query multiple symbols in one request
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/position/list
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type
linear
,
inverse
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
If
symbol
passed, it returns data regardless of having position or not.
If
symbol
=null and
settleCoin
specified, it returns position size greater than zero.
baseCoin
false
string
Base coin, uppercase only.
option
only
. Return all option positions if not passed
settleCoin
false
string
Settle coin
linear
: either
symbol
or
settleCoin
is
required
.
symbol
has a higher priority
limit
false
integer
Limit for data size per page.
[
1
,
200
]
. Default:
20
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
nextPageCursor
string
Refer to the
cursor
request parameter
list
array
Object
>
positionIdx
integer
Position idx, used to identify positions in different position modes
0
: One-Way Mode
1
: Buy side of both side mode
2
: Sell side of both side mode
> riskId
integer
Risk tier ID
for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
> riskLimitValue
string
Risk limit value, become meaningless when auto risk-limit tier is applied
for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
> symbol
string
Symbol name
> side
string
Position side.
Buy
: long,
Sell
: short
return an empty string
""
for an empty position
> size
string
Position size, always positive
> avgPrice
string
Average entry price
For USDC Perp & Futures, it indicates average entry price, and it will not be changed with 8-hour session settlement
> positionValue
string
Position value
> autoAddMargin
integer
Whether to add margin automatically when using isolated margin mode
0
: false
1
: true
>
positionStatus
String
Position status.
Normal
,
Liq
,
Adl
> leverage
string
Position leverage
for portfolio margin mode, this field returns "", which means leverage rules are invalid
> breakEvenPrice
string
Break even price, Only for
linear
,
inverse
.
breakeven_price = (entry_price
qty - realized_pnl) / (qty - abs(qty)
max(taker fee rate, 0.00055))
> markPrice
string
Mark price
> liqPrice
string
Position liquidation price
Isolated margin:
it is the real price for isolated and cross positions, and keeps
""
when liqPrice <= minPrice or liqPrice >= maxPrice
Cross margin:
it is an
estimated
price for cross positions(because the unified mode controls the risk rate according to the account), and keeps
""
when liqPrice <= minPrice or liqPrice >= maxPrice
this field is empty for Portfolio Margin Mode, and no liquidation price will be provided
> positionIM
string
Initial margin, the same value as
positionIMByMp
, please note this change
The New Margin Calculation: Adjustments and Implications
Portfolio margin mode: returns ""
> positionIMByMp
string
Initial margin calculated by mark price, the same value as
positionIM
Portfolio margin mode: returns ""
> positionMM
string
Maintenance margin, the same value as
positionMMByMp
Portfolio margin mode: returns ""
> positionMMByMp
string
Maintenance margin calculated by mark price, the same value as
positionMM
Portfolio margin mode: returns ""
> takeProfit
string
Take profit price
> stopLoss
string
Stop loss price
> trailingStop
string
Trailing stop (the distance from market price)
> sessionAvgPrice
string
USDC contract session avg price, it is the same figure as avg entry price shown in the web UI
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
> unrealisedPnl
string
Unrealised PnL
> curRealisedPnl
string
The realised PnL for the current holding position
> cumRealisedPnl
string
Cumulative realised pnl
Futures & Perps: it is the all time cumulative realised P&L
Option: always "", meaningless
>
adlRankIndicator
integer
Auto-deleverage rank indicator.
What is Auto-Deleveraging?
> createdTime
string
Timestamp of the first time a position was created on this symbol (ms)
> updatedTime
string
Position updated timestamp (ms)
> seq
long
Cross sequence, used to associate each fill and each position update
Different symbols may have the same seq, please use seq + symbol to check unique
Returns
"-1"
if the symbol has never been traded
Returns the seq updated by the last transaction when there are settings like leverage, risk limit
> isReduceOnly
boolean
Useful when Bybit lower the risk limit
true
: Only allowed to reduce the position. You can consider a series of measures, e.g., lower the risk limit, decrease leverage or reduce the position, add margin, or cancel orders, after these operations, you can call
confirm new risk limit
endpoint to check if your position can be removed the reduceOnly mark
false
: There is no restriction, and it means your position is under the risk when the risk limit is systematically adjusted
Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
> mmrSysUpdatedTime
string
Useful when Bybit lower the risk limit
When isReduceOnly=
true
: the timestamp (ms) when the MMR will be forcibly adjusted by the system
When isReduceOnly=
false
: the timestamp when the MMR had been adjusted by system
It returns the timestamp when the system operates, and if you manually operate, there is no timestamp
Keeps
""
by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp
Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
> leverageSysUpdatedTime
string
Useful when Bybit lower the risk limit
When isReduceOnly=
true
: the timestamp (ms) when the leverage will be forcibly adjusted by the system
When isReduceOnly=
false
: the timestamp when the leverage had been adjusted by system
It returns the timestamp when the system operates, and if you manually operate, there is no timestamp
Keeps
""
by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp
Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
> tpslMode
string
Deprecated
, always "Full"
> bustPrice
string
Deprecated
, always
""
> positionBalance
string
Deprecated
, can refer to
positionIM
or
positionIMByMp
field
> tradeMode
integer
Deprecated
, always
0
, check
Get Account Info
to know the margin mode
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
GET
/v5/position/list?category=inverse&symbol=BTCUSD
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672280218882
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_positions
(
category
=
"inverse"
,
symbol
=
"BTCUSD"
,
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
position
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
position
.
request
.
*
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncPositionRestClient
(
)
;
var
positionListRequest
=
PositionDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
build
(
)
;
client
.
getPositionInfo
(
positionListRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getPositionInfo
(
{
category
:
'inverse'
,
symbol
:
'BTCUSD'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"positionIdx"
:
0
,
"riskId"
:
1
,
"riskLimitValue"
:
"150"
,
"symbol"
:
"BTCUSD"
,
"side"
:
"Sell"
,
"size"
:
"300"
,
"avgPrice"
:
"27464.50441675"
,
"positionValue"
:
"0.01092319"
,
"tradeMode"
:
0
,
"positionStatus"
:
"Normal"
,
"autoAddMargin"
:
1
,
"adlRankIndicator"
:
2
,
"leverage"
:
"10"
,
"breakEvenPrice"
:
"93556.73034991"
,
"positionBalance"
:
"0.00139186"
,
"markPrice"
:
"28224.50"
,
"liqPrice"
:
""
,
"bustPrice"
:
"999999.00"
,
"positionMM"
:
"0.0000015"
,
"positionMMByMp"
:
"0.0000015"
,
"positionIM"
:
"0.00010923"
,
"positionIMByMp"
:
"0.00010923"
,
"tpslMode"
:
"Full"
,
"takeProfit"
:
"0.00"
,
"stopLoss"
:
"0.00"
,
"trailingStop"
:
"0.00"
,
"unrealisedPnl"
:
"-0.00029413"
,
"curRealisedPnl"
:
"0.00013123"
,
"cumRealisedPnl"
:
"-0.00096902"
,
"seq"
:
5723621632
,
"isReduceOnly"
:
false
,
"mmrSysUpdateTime"
:
""
,
"leverageSysUpdatedTime"
:
""
,
"sessionAvgPrice"
:
""
,
"createdTime"
:
"1676538056258"
,
"updatedTime"
:
"1697673600012"
}
]
,
"nextPageCursor"
:
""
,
"category"
:
"inverse"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1697684980172
}

**Examples:**

Example 1 ():
```
GET /v5/position/list?category=inverse&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672280218882X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_positions(    category="inverse",    symbol="BTCUSD",))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var positionListRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").build();client.getPositionInfo(positionListRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getPositionInfo({        category: 'inverse',        symbol: 'BTCUSD',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get USDC Session Settlement

**URL:** https://bybit-exchange.github.io/docs/v5/asset/settlement

**Contents:**
- Get USDC Session Settlement
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Get USDC Session Settlement (2 years)
On this page
Get USDC Session Settlement
Query session settlement records of USDC perpetual and futures
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/asset/settlement-record
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type
linear
(USDC contract)
symbol
false
string
Symbol name, like
BTCPERP
, uppercase only
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 30 days by default
Only startTime is passed, return range between startTime and startTime + 30 days
Only endTime is passed, return range between endTime-30 days and endTime
If both are passed, the rule is endTime - startTime <= 30 days
endTime
false
integer
The end time. timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
50
]
. Default:
20
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
> side
string
Buy
,
Sell
> size
string
Position size
> sessionAvgPrice
string
Settlement price
> markPrice
string
Mark price
> realisedPnl
string
Realised PnL
> createdTime
string
Created time (ms)
nextPageCursor
string
Refer to the
cursor
request parameter
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/settlement-record?category=linear
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672284883483
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_usdc_contract_settlement
(
category
=
"linear"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getSettlementRecords
(
{
category
:
'linear'
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"nextPageCursor"
:
"116952%3A1%2C116952%3A1"
,
"category"
:
"linear"
,
"list"
:
[
{
"realisedPnl"
:
"-71.28"
,
"symbol"
:
"BTCPERP"
,
"side"
:
"Buy"
,
"markPrice"
:
"16620"
,
"size"
:
"1.5"
,
"createdTime"
:
"1672214400000"
,
"sessionAvgPrice"
:
"16620"
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
1672284884285
}

**Examples:**

Example 1 ():
```
GET /v5/asset/settlement-record?category=linear HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672284883483X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_usdc_contract_settlement(    category="linear",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSettlementRecords({ category: 'linear' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "116952%3A1%2C116952%3A1",        "category": "linear",        "list": [            {                "realisedPnl": "-71.28",                "symbol": "BTCPERP",                "side": "Buy",                "markPrice": "16620",                "size": "1.5",                "createdTime": "1672214400000",                "sessionAvgPrice": "16620"            }        ]    },    "retExtInfo": {},    "time": 1672284884285}
```

---

## Get Withdrawable Amount

**URL:** https://bybit-exchange.github.io/docs/v5/asset/balance/delay-amount

**Contents:**
- Get Withdrawable Amount
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Balances
Get Withdrawable Amount
On this page
Get Withdrawable Amount
info
How can partial funds be subject to delayed withdrawal requests?
On-chain deposit
: If the number of on-chain confirmations has not reached a risk-controlled level, a portion of the funds will be frozen for a period of time until they are unfrozen.
Buying crypto
: If there is a risk, the funds will be frozen for a certain period of time and cannot be withdrawn.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/asset/withdraw/withdrawable-amount
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
Coin name, uppercase only
Response Parameters
​
Parameter
Type
Comments
limitAmountUsd
string
The frozen amount due to risk, in USD
withdrawableAmount
Object
> SPOT
Object
Spot wallet, it is not returned if spot wallet is removed
>> coin
string
Coin name
>> withdrawableAmount
string
Amount that can be withdrawn
>> availableBalance
string
Available balance
> FUND
Object
Funding wallet
>> coin
string
Coin name
>> withdrawableAmount
string
Amount that can be withdrawn
>> availableBalance
string
Available balance
> UTA
Object
Unified wallet
>> coin
string
Coin name
>> withdrawableAmount
string
Amount that can be withdrawn
>> availableBalance
string
Available balance
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/withdraw/withdrawable-amount?coin=USDT
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1677565621998
X-BAPI-RECV-WINDOW
:
50000
X-BAPI-SIGN
:
XXXXXX
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_withdrawable_amount
(
coin
=
"USDT"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getWithdrawableAmount
(
{
coin
:
'USDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"success"
,
"result"
:
{
"limitAmountUsd"
:
"595051.7"
,
"withdrawableAmount"
:
{
"FUND"
:
{
"coin"
:
"USDT"
,
"withdrawableAmount"
:
"155805.847"
,
"availableBalance"
:
"155805.847"
}
,
"UTA"
:
{
"coin"
:
"USDT"
,
"withdrawableAmount"
:
"498751.0882"
,
"availableBalance"
:
"498751.0882"
}
}
}
,
"retExtInfo"
:
{
}
,
"time"
:
1754009688289
}

**Examples:**

Example 1 ():
```
GET /v5/asset/withdraw/withdrawable-amount?coin=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1677565621998X-BAPI-RECV-WINDOW: 50000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_withdrawable_amount(    coin="USDT",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getWithdrawableAmount({    coin: 'USDT',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "limitAmountUsd": "595051.7",        "withdrawableAmount": {            "FUND": {                "coin": "USDT",                "withdrawableAmount": "155805.847",                "availableBalance": "155805.847"            },            "UTA": {                "coin": "USDT",                "withdrawableAmount": "498751.0882",                "availableBalance": "498751.0882"            }        }    },    "retExtInfo": {},    "time": 1754009688289}
```

---

## RPI Orderbook

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/orderbook-rpi

**Contents:**
- RPI Orderbook
  - Depths​
  - Response Parameters​
  - Subscribe Example​
  - Subscribe Success Response​
  - Response Example​

WebSocket Stream
Public
RPI Orderbook
On this page
RPI Orderbook
Subscribe to the orderbook stream including RPI quote
Depths
​
Spot, Perpetual & Futures:
Level 50 data, push frequency:
100ms
Topic:
orderbook.rpi.{symbol}
e.g., orderbook.rpi.BTCUSDT
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
,
delta
ts
number
The timestamp (ms) that the system generates the data
data
map
Object
> s
string
Symbol name
> b
array
Bids. For
snapshot
stream. Sorted by price in descending order
>> b
[0]
string
Bid price
>> b
[1]
string
None RPI bid size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
>> b
[2]
string
RPI bid size
When a bid RPI order crosses with a non-RPI ask price, the quantity of the bid RPI becomes invalid and is hidden
> a
array
Asks. For
snapshot
stream. Sorted by price in ascending order
>> a
[0]
string
Ask price
>> a
[1]
string
None RPI ask size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
>> a
[2]
string
RPI ask size
When an ask RPI order crosses with a non-RPI bid price, the quantity of the ask RPI becomes invalid and is hidden
> u
integer
Update ID
Occasionally, you'll receive "u"=1, which is a snapshot data due to the restart of the service. So please overwrite your local orderbook
> seq
integer
Cross sequence
You can use this field to compare different levels orderbook data, and for the smaller seq, then it means the data is generated earlier.
cts
number
The timestamp from the matching engine when this orderbook data is produced. It can be correlated with
T
from
public trade channel
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
"orderbook.rpi.BTCUSDT"
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
rpi_orderbook_stream
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
Subscribe Success Response
​
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
"f6b17b77-48b6-4c5c-b5ec-4a1c733f5763"
,
"op"
:
"subscribe"
}
Response Example
​
{
"topic"
:
"orderbook.rpi.BTCUSDT"
,
"ts"
:
1752472188075
,
"type"
:
"delta"
,
"data"
:
{
"s"
:
"BTCUSDT"
,
"b"
:
[
[
"121975.1"
,
"0.114259"
,
"0"
]
,
[
"121969.9"
,
"0"
,
"0"
]
,
[
"121960.5"
,
"0"
,
"0.163986"
]
]
,
"a"
:
[
[
"121990.8"
,
"0.441585"
,
"0.78821"
]
,
[
"121996.1"
,
"0.016393"
,
"0"
]
,
[
"122018.5"
,
"0"
,
"0"
]
]
,
"u"
:
2258980
,
"seq"
:
79683241099
}
,
"cts"
:
1752472188067
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "orderbook.rpi.BTCUSDT"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.rpi_orderbook_stream(    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "success": true,    "ret_msg": "subscribe",    "conn_id": "f6b17b77-48b6-4c5c-b5ec-4a1c733f5763",    "op": "subscribe"}
```

Example 4 ():
```
{    "topic": "orderbook.rpi.BTCUSDT",    "ts": 1752472188075,    "type": "delta",    "data": {        "s": "BTCUSDT",        "b": [            [                "121975.1",                "0.114259",                "0"            ],            [                "121969.9",                "0",                "0"            ],            [                "121960.5",                "0",                "0.163986"            ]        ],        "a": [            [                "121990.8",                "0.441585",                "0.78821"            ],            [                "121996.1",                "0.016393",                "0"            ],            [                "122018.5",                "0",                "0"            ]        ],        "u": 2258980,        "seq": 79683241099    },    "cts": 1752472188067}
```

---

## Get RFQs (real-time)

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/rfq-realtime

**Contents:**
- Get RFQs (real-time)
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get RFQs (real-time)
On this page
Get RFQs (real-time)
Obtain real-time inquiry information.
Up to 50 requests per second
info
Obtain RFQs in real-time.
If both rfqId and rfqLinkId are passed, only rfqId is considered.
Sorted in descending order by createdAt.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/rfq/rfq-realtime
Request Parameters
​
Parameter
Required
Type
Comments
rfqId
false
string
Inquiry ID
rfqLinkId
false
string
Custom inquiry ID, traderType is quote, this field is invalid
traderType
false
string
Trader type,
quote
,
request
. Default:
quote
Response Parameters
​
Parameter
Type
Comments
list
array
An array of RFQs
> rfqId
string
Inquiry ID
> rfqLinkId
string
Custom RFQ ID. Not publicly disclosed.
>counterparties
array of srings
List of bidders
> expiresAt
string
The inquiry's expiration time (ms)
> strategyType
string
Inquiry label
> status
string
Status of the RFQ:
Active
PendingFill
Canceled
Filled
Expired
Failed
> acceptOtherQuoteStatus
string
Whether to accept non-LP quotes. The default value is
false
:
false
: Default value, do not accept non-LP quotes.
true
: Accept non-LP quotes
> deskCode
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
> legs
array of objects
Combination transaction
>> category
string
category. Valid values include: "linear", "option" and "spot"
>> symbol
string
The unique instrument ID
>> side
string
Inquiry direction: Valid values are
Buy
and
Sell
.
>> qty
string
Order quantity of the instrument.
Request Example
​
GET
/v5/rfq/rfq-realtime
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676430842094
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"rfqLinkId"
:
""
,
"rfqId"
:
"1756885055799241492396882271696580"
,
"counterparties"
:
[
"hashwave2"
]
,
"strategyType"
:
"custom"
,
"expiresAt"
:
"1756885655801"
,
"status"
:
"Active"
,
"acceptOtherQuoteStatus"
:
"false"
,
"deskCode"
:
"1nu9d1"
,
"createdAt"
:
"1756885055801"
,
"updatedAt"
:
"1756885055801"
,
"legs"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"qty"
:
"1"
}
]
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
1756885059062
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/rfq-realtime HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "rfqLinkId": "",                "rfqId": "1756885055799241492396882271696580",                "counterparties": [                    "hashwave2"                ],                "strategyType": "custom",                "expiresAt": "1756885655801",                "status": "Active",                "acceptOtherQuoteStatus":"false",                "deskCode": "1nu9d1",                "createdAt": "1756885055801",                "updatedAt": "1756885055801",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "qty": "1"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1756885059062}
```

---

## Get Borrowing Market

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/borrow-market

**Contents:**
- Get Borrowing Market
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Borrowing Market
On this page
Get Borrowing Market
info
Does not need authentication.
If you want to borrow, you can use this endpoint to check whether there are any suitable counterparty supply orders available.
HTTP Request
​
GET
/v5/crypto-loan-fixed/borrow-order-quote
Request Parameters
​
Parameter
Required
Type
Comments
orderCurrency
true
string
Coin name
orderBy
true
string
Order by,
apy
: annual rate;
term
;
quantity
term
false
string
Fixed term
7
: 7 days;
14
: 14 days;
30
: 30 days;
90
: 90 days;
180
: 180 days
sort
false
integer
0
: ascend, default;
1
: descend
limit
false
integer
Limit for data size per page.
[
1
,
100
]
. Default:
10
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> orderCurrency
string
Coin name
> term
integer
Fixed term
7
: 7 days;
14
: 14 days;
30
: 30 days;
90
: 90 days;
180
: 180 days
> annualRate
string
Annual rate
> qty
string
Quantity
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/borrow-order-quote?orderCurrency=USDT&orderBy=apy
HTTP/1.1
Host
:
api-testnet.bybit.com
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_borrowing_market_fixed_crypto_loan
(
orderCurrency
=
"USDT"
,
orderBy
=
"apy"
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
"ok"
,
"result"
:
{
"list"
:
[
{
"annualRate"
:
"0.04"
,
"orderCurrency"
:
"USDT"
,
"qty"
:
"988.78"
,
"term"
:
14
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
1752719158890
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/borrow-order-quote?orderCurrency=USDT&orderBy=apy HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_borrowing_market_fixed_crypto_loan(    orderCurrency="USDT",    orderBy="apy",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "annualRate": "0.04",                "orderCurrency": "USDT",                "qty": "988.78",                "term": 14            }        ]    },    "retExtInfo": {},    "time": 1752719158890}
```

---

## Get Bybit Server Time

**URL:** https://bybit-exchange.github.io/docs/v5/market/time

**Contents:**
- Get Bybit Server Time
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Bybit Server Time
On this page
Get Bybit Server Time
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/time
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
timeSecond
string
Bybit server timestamp (sec)
timeNano
string
Bybit server timestamp (nano)
RUN >>
Request Example
​
HTTP
Python
Java
Go
Node.js
GET
/v5/market/time
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
)
print
(
session
.
get_server_time
(
)
)
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
client
.
getServerTime
(
System
.
out
::
println
)
;
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
client
.
NewUtaBybitServiceNoParams
(
)
.
GetServerTime
(
context
.
Background
(
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getServerTime
(
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"timeSecond"
:
"1688639403"
,
"timeNano"
:
"1688639403423213947"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1688639403423
}

**Examples:**

Example 1 ():
```
GET /v5/market/time HTTP/1.1Host: api.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_server_time())
```

Example 3 ():
```
import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();client.getServerTime(System.out::println);
```

Example 4 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))client.NewUtaBybitServiceNoParams().GetServerTime(context.Background())
```

---

## Get Index Price Kline

**URL:** https://bybit-exchange.github.io/docs/v5/market/index-kline

**Contents:**
- Get Index Price Kline
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Index Price Kline
On this page
Get Index Price Kline
Query for historical
index price
klines. Charts are returned in groups based on the requested interval.
Covers: USDT contract / USDC contract / Inverse contract
HTTP Request
​
GET
/v5/market/index-price-kline
Request Parameters
​
Parameter
Required
Type
Comments
category
false
string
Product type.
linear
,
inverse
When
category
is not passed, use
linear
by default
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
interval
true
string
Kline interval.
1
,
3
,
5
,
15
,
30
,
60
,
120
,
240
,
360
,
720
,
D
,
W
,
M
start
false
integer
The start timestamp (ms)
end
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
200
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
symbol
string
Symbol name
list
array
An string array of individual candle
Sort in reverse by
startTime
> list
[0]
: startTime
string
Start time of the candle (ms)
> list
[1]
: openPrice
string
Open price
> list
[2]
: highPrice
string
Highest price
> list
[3]
: lowPrice
string
Lowest price
> list
[4]
: closePrice
string
Close price.
Is the last traded price when the candle is not closed
RUN >>
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/index-price-kline?category=inverse&symbol=BTCUSDZ22&interval=1&start=1670601600000&end=1670608800000&limit=2
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_index_price_kline
(
category
=
"inverse"
,
symbol
=
"BTCUSDZ22"
,
interval
=
1
,
start
=
1670601600000
,
end
=
1670608800000
,
limit
=
2
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"interval"
:
"1"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetIndexPriceKline
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
marketKLineRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
marketInterval
(
MarketInterval
.
WEEKLY
)
.
build
(
)
;
client
.
getIndexPriceLinesData
(
marketKLineRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getIndexPriceKline
(
{
category
:
'inverse'
,
symbol
:
'BTCUSDZ22'
,
interval
:
'1'
,
start
:
1670601600000
,
end
:
1670608800000
,
limit
:
2
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"symbol"
:
"BTCUSDZ22"
,
"category"
:
"inverse"
,
"list"
:
[
[
"1670608800000"
,
"17167.00"
,
"17167.00"
,
"17161.90"
,
"17163.07"
]
,
[
"1670608740000"
,
"17166.54"
,
"17167.69"
,
"17165.42"
,
"17167.00"
]
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
1672026471128
}

**Examples:**

Example 1 ():
```
GET /v5/market/index-price-kline?category=inverse&symbol=BTCUSDZ22&interval=1&start=1670601600000&end=1670608800000&limit=2 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_index_price_kline(    category="inverse",    symbol="BTCUSDZ22",    interval=1,    start=1670601600000,    end=1670608800000,    limit=2,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetIndexPriceKline(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getIndexPriceLinesData(marketKLineRequest, System.out::println);
```

---

## Get Recent Public Trades

**URL:** https://bybit-exchange.github.io/docs/v5/spread/market/recent-trade

**Contents:**
- Get Recent Public Trades
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Market
Get Recent Public Trades
On this page
Get Recent Public Trades
Query recent public spread trading history in Bybit.
HTTP Request
​
GET
/v5/spread/recent-trade
Request Parameters
​
Parameter
Required
Type
Comments
symbol
true
string
Spread combination symbol name
limit
false
integer
Limit for data size per page
[
1
,
1000
]
, default:
500
Response Parameters
​
Parameter
Type
Comments
list
array
<
object
>
Public trade info
> execId
string
Execution ID
> symbol
string
Spread combination symbol name
> price
string
Trade price
> size
string
Trade size
> side
string
Side of taker
Buy
,
Sell
> time
string
Trade time (ms)
> seq
string
Cross sequence
Request Example
​
GET
/v5/spread/recent-trade?symbol=SOLUSDT_SOL/USDT&limit=2
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"Success"
,
"result"
:
{
"list"
:
[
{
"execId"
:
"c8512970-d6fb-5039-93a5-b4196dffbe88"
,
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"price"
:
"20.2805"
,
"size"
:
"3.3"
,
"side"
:
"Sell"
,
"time"
:
"1744078324035"
,
"seq"
:
"123456"
}
,
{
"execId"
:
"92b0002e-c49d-5618-a195-4140d7e10a2b"
,
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"price"
:
"20.843"
,
"size"
:
"2.2"
,
"side"
:
"Buy"
,
"time"
:
"1744078322010"
,
"seq"
:
"123450"
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
1744078324682
}

**Examples:**

Example 1 ():
```
GET /v5/spread/recent-trade?symbol=SOLUSDT_SOL/USDT&limit=2 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "list": [            {                "execId": "c8512970-d6fb-5039-93a5-b4196dffbe88",                "symbol": "SOLUSDT_SOL/USDT",                "price": "20.2805",                "size": "3.3",                "side": "Sell",                "time": "1744078324035",                "seq":"123456"            },            {                "execId": "92b0002e-c49d-5618-a195-4140d7e10a2b",                "symbol": "SOLUSDT_SOL/USDT",                "price": "20.843",                "size": "2.2",                "side": "Buy",                "time": "1744078322010",                "seq":"123450"            }        ]    },    "retExtInfo": {},    "time": 1744078324682}
```

---

## Orderbook

**URL:** https://bybit-exchange.github.io/docs/v5/spread/websocket/public/orderbook

**Contents:**
- Orderbook
  - Depths​
  - Process snapshot/delta​
  - Response Parameters​
  - Subscribe Example​
  - Event Example​

Spread Trading
Websocket Stream
Public
Orderbook
On this page
Orderbook
Subscribe to the spread orderbook stream.
Depths
​
Level 25 data, push frequency:
20ms
Topic:
orderbook.{depth}.{symbol}
e.g., orderbook.25.SOLUSDT_SOL/USDT
Process snapshot/delta
​
To process
snapshot
and
delta
messages, please follow these rules:
Once you have subscribed successfully, you will receive a
snapshot
. The WebSocket will keep pushing
delta
messages every time the orderbook changes. If you receive a new
snapshot
message, you will have to reset your local orderbook. If there is a problem on Bybit's end, a
snapshot
will be re-sent, which is guaranteed to contain the latest data.
To apply
delta
updates:
If you receive an amount that is
0
, delete the entry
If you receive an amount that does not exist, insert it
If the entry exists, you simply update the value
See working code examples of this logic in the
FAQ
.
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
,
delta
ts
number
The timestamp (ms) that the system generates the data
data
map
Object
> s
string
Symbol name
> b
array
Bids. For
snapshot
stream. Sorted by price in descending order
>> b
[0]
string
Bid price
>> b
[1]
string
Bid size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
> a
array
Asks. For
snapshot
stream. Sorted by price in ascending order
>> a
[0]
string
Ask price
>> a
[1]
string
Ask size
The delta data has size=0, which means that all quotations for this price have been filled or cancelled
> u
integer
Update ID
Occasionally, you'll receive "u"=1, which is a snapshot data due to the restart of the service. So please overwrite your local orderbook
> seq
integer
Cross sequence
cts
number
The timestamp from the matching engine when this orderbook data is produced. It can be correlated with
T
from
public trade channel
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
"orderbook.25.SOLUSDT_SOL/USDT"
]
}
Event Example
​
{
"topic"
:
"orderbook.25.SOLUSDT_SOL/USDT"
,
"ts"
:
1744165512257
,
"type"
:
"delta"
,
"data"
:
{
"s"
:
"SOLUSDT_SOL/USDT"
,
"b"
:
[
]
,
"a"
:
[
[
"22.3755"
,
"4.7"
]
]
,
"u"
:
64892
,
"seq"
:
299084
}
,
"cts"
:
1744165512234
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": ["orderbook.25.SOLUSDT_SOL/USDT"]}
```

Example 2 ():
```
{    "topic": "orderbook.25.SOLUSDT_SOL/USDT",    "ts": 1744165512257,    "type": "delta",    "data": {        "s": "SOLUSDT_SOL/USDT",        "b": [],        "a": [            [                "22.3755",                "4.7"            ]        ],        "u": 64892,        "seq": 299084    },    "cts": 1744165512234}
```

---

## Get Transaction Log

**URL:** https://bybit-exchange.github.io/docs/v5/account/transaction-log

**Contents:**
- Get Transaction Log
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Get Transaction Log (UTA)
On this page
Get Transaction Log
Query for transaction logs in your Unified account. It supports up to 2 years worth of data.
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/account/transaction-log
Request Parameters
​
Parameter
Required
Type
Comments
accountType
false
string
Account Type.
UNIFIED
category
false
string
Product type
spot
,
linear
,
option
,
inverse
currency
false
string
Currency, uppercase only
baseCoin
false
string
BaseCoin, uppercase only. e.g., BTC of BTCPERP
type
false
string
Types of transaction logs
transSubType
false
string
movePosition
, used to filter trans logs of Move Position only
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 24 hours by default
Only startTime is passed, return range between startTime and startTime+24 hours
Only endTime is passed, return range between endTime-24 hours and endTime
If both are passed, the rule is endTime - startTime <= 7 days
endTime
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
50
]
. Default:
20
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
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
Unique id
> symbol
string
Symbol name
> category
string
Product type
> side
string
Side.
Buy
,
Sell
,
None
> transactionTime
string
Transaction timestamp (ms)
>
type
string
Type
> transSubType
string
Transaction sub type,
movePosition
, used for the logs generated by move position.
""
by default
> qty
string
Quantity
Spot: the negative means the qty of this currency is decreased, the positive means the qty of this currency is increased
Perps & Futures: it is the quantity for each trade entry and it does not have direction
> size
string
Size. The rest position size after the trade is executed, and it has direction, i.e., short with "-"
> currency
string
e.g., USDC, USDT, BTC, ETH
> tradePrice
string
Trade price
> funding
string
Funding fee
Positive fee value means receive funding; negative fee value means pay funding. This is opposite to the
execFee
from
Get Trade History
.
For USDC Perp, as funding settlement and session settlement occur at the same time, they are represented in a single record at settlement. Please refer to
funding
to understand funding fee, and
cashFlow
to understand 8-hour P&L.
> fee
string
Trading fee
Positive fee value means expense
Negative fee value means rebates
> cashFlow
string
Cash flow, e.g., (1) close the position, and unRPL converts to RPL, (2) 8-hour session settlement for USDC Perp and Futures, (3) transfer in or transfer out. This does not include trading fee, funding fee
> change
string
Change = cashFlow + funding - fee
> cashBalance
string
Cash balance. This is the wallet balance after a cash change
> feeRate
string
When type=
TRADE
, then it is trading fee rate
When type=
SETTLEMENT
, it means funding fee rate. For side=Buy, feeRate=market fee rate; For side=Sell, feeRate= - market fee rate
> bonusChange
string
The change of bonus
> tradeId
string
Trade ID
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
> extraFees
string
Trading fee rate information. Currently, this data is returned only for spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum:
feeType
,
subFeeType
nextPageCursor
string
Refer to the
cursor
request parameter
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/account/transaction-log?accountType=UNIFIED&category=linear&currency=USDT
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672132480085
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_transaction_log
(
accountType
=
"UNIFIED"
,
category
=
"linear"
,
currency
=
"USDT"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getTransactionLog
(
{
accountType
:
'UNIFIED'
,
category
:
'linear'
,
currency
:
'USDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"nextPageCursor"
:
"21963%3A1%2C14954%3A1"
,
"list"
:
[
{
"transSubType"
:
""
,
"id"
:
"592324_XRPUSDT_161440249321"
,
"symbol"
:
"XRPUSDT"
,
"side"
:
"Buy"
,
"funding"
:
"-0.003676"
,
"orderLinkId"
:
""
,
"orderId"
:
"1672128000-8-592324-1-2"
,
"fee"
:
"0.00000000"
,
"change"
:
"-0.003676"
,
"cashFlow"
:
"0"
,
"transactionTime"
:
"1672128000000"
,
"type"
:
"SETTLEMENT"
,
"feeRate"
:
"0.0001"
,
"bonusChange"
:
""
,
"size"
:
"100"
,
"qty"
:
"100"
,
"cashBalance"
:
"5086.55825002"
,
"currency"
:
"USDT"
,
"category"
:
"linear"
,
"tradePrice"
:
"0.3676"
,
"tradeId"
:
"534c0003-4bf7-486f-aa02-78cee36825e4"
,
"extraFees"
:
""
}
,
{
"transSubType"
:
""
,
"id"
:
"592324_XRPUSDT_161440249321"
,
"symbol"
:
"XRPUSDT"
,
"side"
:
"Buy"
,
"funding"
:
""
,
"orderLinkId"
:
"linear-order"
,
"orderId"
:
"592b7e41-78fd-42e2-9aa3-91e1835ef3e1"
,
"fee"
:
"0.01908720"
,
"change"
:
"-0.0190872"
,
"cashFlow"
:
"0"
,
"transactionTime"
:
"1672121182224"
,
"type"
:
"TRADE"
,
"feeRate"
:
"0.0006"
,
"bonusChange"
:
"-0.1430544"
,
"size"
:
"100"
,
"qty"
:
"88"
,
"cashBalance"
:
"5086.56192602"
,
"currency"
:
"USDT"
,
"category"
:
"linear"
,
"tradePrice"
:
"0.3615"
,
"tradeId"
:
"5184f079-88ec-54c7-8774-5173cafd2b4e"
,
"extraFees"
:
""
}
,
{
"transSubType"
:
""
,
"id"
:
"592324_XRPUSDT_161407743011"
,
"symbol"
:
"XRPUSDT"
,
"side"
:
"Buy"
,
"funding"
:
""
,
"orderLinkId"
:
"linear-order"
,
"orderId"
:
"592b7e41-78fd-42e2-9aa3-91e1835ef3e1"
,
"fee"
:
"0.00260280"
,
"change"
:
"-0.0026028"
,
"cashFlow"
:
"0"
,
"transactionTime"
:
"1672121182224"
,
"type"
:
"TRADE"
,
"feeRate"
:
"0.0006"
,
"bonusChange"
:
""
,
"size"
:
"12"
,
"qty"
:
"12"
,
"cashBalance"
:
"5086.58101322"
,
"currency"
:
"USDT"
,
"category"
:
"linear"
,
"tradePrice"
:
"0.3615"
,
"tradeId"
:
"8569c10f-5061-5891-81c4-a54929847eb3"
,
"extraFees"
:
""
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
1672132481405
}

**Examples:**

Example 1 ():
```
GET /v5/account/transaction-log?accountType=UNIFIED&category=linear&currency=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672132480085X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_transaction_log(    accountType="UNIFIED",    category="linear",    currency="USDT",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getTransactionLog({        accountType: 'UNIFIED',        category: 'linear',        currency: 'USDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "21963%3A1%2C14954%3A1",        "list": [            {                "transSubType": "",                "id": "592324_XRPUSDT_161440249321",                "symbol": "XRPUSDT",                "side": "Buy",                "funding": "-0.003676",                "orderLinkId": "",                "orderId": "1672128000-8-592324-1-2",                "fee": "0.00000000",                "change": "-0.003676",                "cashFlow": "0",                "transactionTime": "1672128000000",                "type": "SETTLEMENT",                "feeRate": "0.0001",                "bonusChange": "",                "size": "100",                "qty": "100",                "cashBalance": "5086.55825002",                "currency": "USDT",                "category": "linear",                "tradePrice": "0.3676",                "tradeId": "534c0003-4bf7-486f-aa02-78cee36825e4",                "extraFees": ""            },            {                "transSubType": "",                "id": "592324_XRPUSDT_161440249321",                "symbol": "XRPUSDT",                "side": "Buy",                "funding": "",                "orderLinkId": "linear-order",                "orderId": "592b7e41-78fd-42e2-9aa3-91e1835ef3e1",                "fee": "0.01908720",                "change": "-0.0190872",                "cashFlow": "0",                "transactionTime": "1672121182224",                "type": "TRADE",                "feeRate": "0.0006",                "bonusChange": "-0.1430544",                "size": "100",                "qty": "88",                "cashBalance": "5086.56192602",                "currency": "USDT",                "category": "linear",                "tradePrice": "0.3615",                "tradeId": "5184f079-88ec-54c7-8774-5173cafd2b4e",                "extraFees": ""            },            {                "transSubType": "",                "id": "592324_XRPUSDT_161407743011",                "symbol": "XRPUSDT",                "side": "Buy",                "funding": "",                "orderLinkId": "linear-order",                "orderId": "592b7e41-78fd-42e2-9aa3-91e1835ef3e1",                "fee": "0.00260280",                "change": "-0.0026028",                "cashFlow": "0",                "transactionTime": "1672121182224",                "type": "TRADE",                "feeRate": "0.0006",                "bonusChange": "",                "size": "12",                "qty": "12",                "cashBalance": "5086.58101322",                "currency": "USDT",                "category": "linear",                "tradePrice": "0.3615",                "tradeId": "8569c10f-5061-5891-81c4-a54929847eb3",                "extraFees": ""            }        ]    },    "retExtInfo": {},    "time": 1672132481405}
```

---

## Get Premium Index Price Kline

**URL:** https://bybit-exchange.github.io/docs/v5/market/premium-index-kline

**Contents:**
- Get Premium Index Price Kline
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Premium Index Price Kline
On this page
Get Premium Index Price Kline
Query for historical
premium index
klines. Charts are returned in groups based on the requested interval.
Covers: USDT and USDC perpetual
HTTP Request
​
GET
/v5/market/premium-index-price-kline
Request Parameters
​
Parameter
Required
Type
Comments
category
false
string
Product type.
linear
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
interval
true
string
Kline interval.
1
,
3
,
5
,
15
,
30
,
60
,
120
,
240
,
360
,
720
,
D
,
W
,
M
start
false
integer
The start timestamp (ms)
end
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
200
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
symbol
string
Symbol name
list
array
An string array of individual candle
Sort in reverse by
start
> list
[0]
string
Start time of the candle (ms)
> list
[1]
string
Open price
> list
[2]
string
Highest price
> list
[3]
string
Lowest price
> list
[4]
string
Close price.
Is the last traded price when the candle is not closed
RUN >>
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/premium-index-price-kline?category=linear&symbol=BTCUSDT&interval=D&start=1652112000000&end=1652544000000
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_premium_index_price_kline
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
inverval
=
"D"
,
start
=
1652112000000
,
end
=
1652544000000
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"interval"
:
"1"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetPremiumIndexPriceKline
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
marketKLineRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
marketInterval
(
MarketInterval
.
WEEKLY
)
.
build
(
)
;
client
.
getPremiumIndexPriceLinesData
(
marketKLineRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getPremiumIndexPriceKline
(
{
category
:
'linear'
,
symbol
:
'BTCUSDT'
,
interval
:
'D'
,
start
:
1652112000000
,
end
:
1652544000000
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"symbol"
:
"BTCUSDT"
,
"category"
:
"linear"
,
"list"
:
[
[
"1652486400000"
,
"-0.000587"
,
"-0.000344"
,
"-0.000480"
,
"-0.000344"
]
,
[
"1652400000000"
,
"-0.000989"
,
"-0.000561"
,
"-0.000587"
,
"-0.000587"
]
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
1672765216291
}

**Examples:**

Example 1 ():
```
GET /v5/market/premium-index-price-kline?category=linear&symbol=BTCUSDT&interval=D&start=1652112000000&end=1652544000000 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP()print(session.get_premium_index_price_kline(    category="linear",    symbol="BTCUSDT",    inverval="D",    start=1652112000000,    end=1652544000000,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetPremiumIndexPriceKline(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getPremiumIndexPriceLinesData(marketKLineRequest, System.out::println);
```

---

## Get Order Price Limit

**URL:** https://bybit-exchange.github.io/docs/v5/market/order-price-limit

**Contents:**
- Get Order Price Limit
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Order Price Limit
On this page
Get Order Price Limit
For derivative trading order price limit, refer to
announcement
For spot trading order price limit, refer to
announcement
HTTP Request
​
GET
/v5/market/price-limit
Request Parameters
​
Parameter
Required
Type
Comments
category
false
string
Product type.
spot
,
linear
,
inverse
When
category
is not passed, use
linear
by default
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
Response Parameters
​
Parameter
Type
Comments
symbol
string
Symbol name
buyLmt
string
Highest Bid Price
sellLmt
string
Lowest Ask Price
ts
string
timestamp in milliseconds
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/price-limit?category=linear&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
"symbol"
:
"BTCUSDT"
,
"buyLmt"
:
"105878.10"
,
"sellLmt"
:
"103781.60"
,
"ts"
:
"1750302284491"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1750302285376
}

**Examples:**

Example 1 ():
```
GET /v5/market/price-limit?category=linear&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_price_limit(    category="linear",    symbol="BTCUSDT",))
```

Example 3 ():
```

```

Example 4 ():
```

```

---

## Get New Delivery Price

**URL:** https://bybit-exchange.github.io/docs/v5/market/new-delivery-price

**Contents:**
- Get New Delivery Price
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get New Delivery Price
On this page
Get New Delivery Price
Get historical option delivery prices.
Covers: Option
info
It is recommended to query this endpoint 1 minute after settlement is completed, because the data returned by this endpoint may be delayed by 1 minute.
By default, the most recent 50 records are returned in reverse order of "deliveryTime".
HTTP Request
​
GET
/v5/market/new-delivery-price
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
Valid for
option
only
baseCoin
true
string
Base coin, uppercase only.
Valid for
option
only
settleCoin
false
string
Settle coin, uppercase only. Default:
USDT
.
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> deliveryPrice
string
Delivery price
> deliveryTime
string
Delivery timestamp (ms)
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/new-delivery-price?category=option&baseCoin=BTC
HTTP/1.1
Host
:
api-testnet.bybit.com
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
"category"
:
"option"
,
"list"
:
[
{
"deliveryPrice"
:
"111675.89830854"
,
"deliveryTime"
:
"1756080000000"
}
,
{
"deliveryPrice"
:
"114990.41430239"
,
"deliveryTime"
:
"1755993600000"
}
,
{
"deliveryPrice"
:
"115792.27557281"
,
"deliveryTime"
:
"1755907200000"
}
,
{
"deliveryPrice"
:
"113162.32041387"
,
"deliveryTime"
:
"1755820800000"
}
,
{
"deliveryPrice"
:
"113852.00497157"
,
"deliveryTime"
:
"1755734400000"
}
,
{
"deliveryPrice"
:
"113604.53226162"
,
"deliveryTime"
:
"1755648000000"
}
,
{
"deliveryPrice"
:
"114828.99222851"
,
"deliveryTime"
:
"1755561600000"
}
,
{
"deliveryPrice"
:
"115321.04746356"
,
"deliveryTime"
:
"1755475200000"
}
,
{
"deliveryPrice"
:
"117969.66726839"
,
"deliveryTime"
:
"1755388800000"
}
,
{
"deliveryPrice"
:
"117622.21555318"
,
"deliveryTime"
:
"1755302400000"
}
,
{
"deliveryPrice"
:
"118846.72206411"
,
"deliveryTime"
:
"1755216000000"
}
,
{
"deliveryPrice"
:
"121778.983223"
,
"deliveryTime"
:
"1755129600000"
}
,
{
"deliveryPrice"
:
"119383.31934289"
,
"deliveryTime"
:
"1755043200000"
}
,
{
"deliveryPrice"
:
"119030.19489407"
,
"deliveryTime"
:
"1754956800000"
}
,
{
"deliveryPrice"
:
"121725.4933271"
,
"deliveryTime"
:
"1754870400000"
}
,
{
"deliveryPrice"
:
"117780.91332268"
,
"deliveryTime"
:
"1754784000000"
}
,
{
"deliveryPrice"
:
"116795.39864682"
,
"deliveryTime"
:
"1754697600000"
}
,
{
"deliveryPrice"
:
"116880.31622213"
,
"deliveryTime"
:
"1754611200000"
}
,
{
"deliveryPrice"
:
"114782.09402227"
,
"deliveryTime"
:
"1754524800000"
}
,
{
"deliveryPrice"
:
"114212.80688625"
,
"deliveryTime"
:
"1754438400000"
}
,
{
"deliveryPrice"
:
"114046.80650192"
,
"deliveryTime"
:
"1754352000000"
}
,
{
"deliveryPrice"
:
"114668.76736223"
,
"deliveryTime"
:
"1754265600000"
}
,
{
"deliveryPrice"
:
"113691.29780823"
,
"deliveryTime"
:
"1754179200000"
}
,
{
"deliveryPrice"
:
"113947.55450439"
,
"deliveryTime"
:
"1754092800000"
}
,
{
"deliveryPrice"
:
"114786.86096974"
,
"deliveryTime"
:
"1754006400000"
}
,
{
"deliveryPrice"
:
"118693.64929462"
,
"deliveryTime"
:
"1753920000000"
}
,
{
"deliveryPrice"
:
"118218.22353841"
,
"deliveryTime"
:
"1753833600000"
}
,
{
"deliveryPrice"
:
"118953.66791589"
,
"deliveryTime"
:
"1753747200000"
}
,
{
"deliveryPrice"
:
"118894.70314174"
,
"deliveryTime"
:
"1753660800000"
}
,
{
"deliveryPrice"
:
"118137.86446229"
,
"deliveryTime"
:
"1753574400000"
}
,
{
"deliveryPrice"
:
"117344.01937262"
,
"deliveryTime"
:
"1753488000000"
}
,
{
"deliveryPrice"
:
"115166.35343924"
,
"deliveryTime"
:
"1753401600000"
}
,
{
"deliveryPrice"
:
"118217.70562761"
,
"deliveryTime"
:
"1753315200000"
}
,
{
"deliveryPrice"
:
"118444.57154255"
,
"deliveryTime"
:
"1753228800000"
}
,
{
"deliveryPrice"
:
"118155.53638794"
,
"deliveryTime"
:
"1753142400000"
}
,
{
"deliveryPrice"
:
"119370.88939816"
,
"deliveryTime"
:
"1753056000000"
}
,
{
"deliveryPrice"
:
"118080.35649338"
,
"deliveryTime"
:
"1752969600000"
}
,
{
"deliveryPrice"
:
"118197.36884665"
,
"deliveryTime"
:
"1752883200000"
}
,
{
"deliveryPrice"
:
"119644.49252705"
,
"deliveryTime"
:
"1752796800000"
}
,
{
"deliveryPrice"
:
"118316.40871555"
,
"deliveryTime"
:
"1752710400000"
}
,
{
"deliveryPrice"
:
"118216.19126195"
,
"deliveryTime"
:
"1752624000000"
}
,
{
"deliveryPrice"
:
"116746.02994227"
,
"deliveryTime"
:
"1752537600000"
}
,
{
"deliveryPrice"
:
"122778.73513717"
,
"deliveryTime"
:
"1752451200000"
}
,
{
"deliveryPrice"
:
"117973.83741111"
,
"deliveryTime"
:
"1752364800000"
}
,
{
"deliveryPrice"
:
"117741.30111399"
,
"deliveryTime"
:
"1752278400000"
}
,
{
"deliveryPrice"
:
"117851.19238216"
,
"deliveryTime"
:
"1752192000000"
}
,
{
"deliveryPrice"
:
"111263.21196833"
,
"deliveryTime"
:
"1752105600000"
}
,
{
"deliveryPrice"
:
"108721.62176788"
,
"deliveryTime"
:
"1752019200000"
}
,
{
"deliveryPrice"
:
"108410.57999842"
,
"deliveryTime"
:
"1751932800000"
}
,
{
"deliveryPrice"
:
"108969.06709828"
,
"deliveryTime"
:
"1751846400000"
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
1756110714178
}

**Examples:**

Example 1 ():
```
GET /v5/market/new-delivery-price?category=option&baseCoin=BTC HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```

```

---

## SBE BBO Integration

**URL:** https://bybit-exchange.github.io/docs/v5/sbe/bbo/sbe-bbo

**Contents:**
- SBE BBO Integration
- Overview​
- Connection​
- Subscription Flow​
  - Send subscription request​
  - Subscription confirmation​
  - Receive data​
  - Decode example​
- SBE Message Structure​
  - SBE XML Schema​

SBE
BBO
SBE BBO Integration
On this page
SBE BBO Integration
Overview
​
Channel:
private MMWS only (not available on public WS).
Topic:
ob.rpi.1.sbe.{symbol}
.
Format:
SBE binary frames (
opcode = 2
), little-endian.
Depth:
Real-time Level 1 Orderbook data.
Units:
timestamps in microseconds (µs); price/size are mantissas with exponents.
From January 15, 2026, sbe connection will only be avaiable via
v5/public-sbe
Connection
​
For Level 1 data on
linear / inverse / spot
: if the order book does not change within
3 seconds
, the system will push a
snapshot
again, and the field
u
will be
the same as
in the previous message.
Under extreme market conditions, both the producer and the publisher may apply
merge and drop
strategies; therefore,
continuity of
u
is not guaranteed
.
Subscription Flow
​
Send subscription request
​
{
"op"
:
"subscribe"
,
"args"
:
[
"ob.rpi.1.sbe.BTCUSDT"
]
}
Topic format:
ob.rpi.1.sbe.<symbol>
Example symbols:
BTCUSDT
,
ETHUSDT
, etc.
Subscription confirmation
​
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
"d30fdpbboasp1pjbe7r0"
,
"req_id"
:
"xxx"
,
"op"
:
"subscribe"
}
Receive data
​
b"R\x00 N\x01\x00\x00\x00\xdb\x84\xd0k\x00\x00\x00\x00f\xb7\x003\x99\x01\x00\x00\x02\x06\xa1\xcb\xa1\x00\x00\x00\x00\x00\xe7\xda\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\xc8\xa1\x00\x00\x00\x00\x00 N\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x008\x01\x00\x00\x00\x00\x00\x00v\xba\x003\x99\x01\x00\x00\x07BTCUSDT"
Decode example
​
{
"header"
:
{
"block_length"
:
82
,
"template_id"
:
20000
,
"schema_id"
:
1
,
"version"
:
0
}
,
"seq"
:
1808827611
,
"cts"
:
1757497309030
,
"price_exponent"
:
2
,
"size_exponent"
:
6
,
"ask_price"
:
1060342500
,
"ask_normal_size"
:
776935000000
,
"ask_rpi_size"
:
0
,
"bid_price"
:
1060250000
,
"bid_normal_size"
:
20000000000
,
"bid_rpi_size"
:
0
,
"u"
:
312
,
"ts"
:
1757497309814
,
"symbol"
:
"BTCUSDT"
,
"parsed_length"
:
98
}
SBE Message Structure
​
SBE XML Schema
​
The
templateId = 20000
identifies the message type.
Validate that
templateId = 20000
to confirm it is a Level 1 Orderbook event.
<?xml version="1.0" encoding="UTF-8"?>
<
sbe:
messageSchema
xmlns:
sbe
=
"
http://fixprotocol.io/2016/sbe
"
xmlns:
mbx
=
"
https://bybit-exchange.github.io/docs/v5/intro
"
package
=
"
quote.sbe
"
id
=
"
1
"
version
=
"
0
"
semanticVersion
=
"
1.0.0
"
description
=
"
Bybit market data streams SBE message schema
"
byteOrder
=
"
littleEndian
"
headerType
=
"
messageHeader
"
>
<
types
>
<
composite
name
=
"
messageHeader
"
description
=
"
Template ID and length of message root
"
>
<
type
name
=
"
blockLength
"
primitiveType
=
"
uint16
"
/>
<
type
name
=
"
templateId
"
primitiveType
=
"
uint16
"
/>
<
type
name
=
"
schemaId
"
primitiveType
=
"
uint16
"
/>
<
type
name
=
"
version
"
primitiveType
=
"
uint16
"
/>
</
composite
>
<
composite
name
=
"
varString8
"
description
=
"
Variable length UTF-8 string.
"
>
<
type
name
=
"
length
"
primitiveType
=
"
uint8
"
/>
<
type
name
=
"
varData
"
length
=
"
0
"
primitiveType
=
"
uint8
"
semanticType
=
"
String
"
characterEncoding
=
"
UTF-8
"
/>
</
composite
>
</
types
>
<!-- Stream event for "ob.rpi.1.sbe.<symbol>" channel -->
<
sbe:
message
name
=
"
BestOBRpiEvent
"
id
=
"
20000
"
>
<
field
id
=
"
1
"
name
=
"
ts
"
type
=
"
int64
"
description
=
"
The timestamp in microseconds that the system generates the data
"
/>
<
field
id
=
"
2
"
name
=
"
seq
"
type
=
"
int64
"
description
=
"
Cross sequence ID
"
/>
<
field
id
=
"
3
"
name
=
"
cts
"
type
=
"
int64
"
description
=
"
The timestamp in microseconds from the matching engine when this orderbook data is produced.
"
/>
<
field
id
=
"
4
"
name
=
"
u
"
type
=
"
int64
"
description
=
"
Update Id
"
/>
<
field
id
=
"
5
"
name
=
"
askNormalPrice
"
type
=
"
int64
"
mbx:
exponent
=
"
priceExponent
"
description
=
"
Mantissa for the best ask normal price
"
/>
<
field
id
=
"
6
"
name
=
"
askNormalSize
"
type
=
"
int64
"
mbx:
exponent
=
"
sizeExponent
"
description
=
"
Mantissa for the best ask normal size
"
/>
<
field
id
=
"
7
"
name
=
"
askRpiPrice
"
type
=
"
int64
"
mbx:
exponent
=
"
priceExponent
"
description
=
"
Mantissa for the best ask rpi price
"
/>
<
field
id
=
"
8
"
name
=
"
askRpiSize
"
type
=
"
int64
"
mbx:
exponent
=
"
sizeExponent
"
description
=
"
Mantissa for the best ask rpi size
"
/>
<
field
id
=
"
9
"
name
=
"
bidNormalPrice
"
type
=
"
int64
"
mbx:
exponent
=
"
priceExponent
"
description
=
"
Mantissa for the best bid normal price
"
/>
<
field
id
=
"
10
"
name
=
"
bidNormalSize
"
type
=
"
int64
"
mbx:
exponent
=
"
sizeExponent
"
description
=
"
Mantissa for the best bid normal size
"
/>
<
field
id
=
"
11
"
name
=
"
bidRpiPrice
"
type
=
"
int64
"
mbx:
exponent
=
"
priceExponent
"
description
=
"
Mantissa for the best bid rpi price
"
/>
<
field
id
=
"
12
"
name
=
"
bidRpiSize
"
type
=
"
int64
"
mbx:
exponent
=
"
sizeExponent
"
description
=
"
Mantissa for the best bid rpi size
"
/>
<
field
id
=
"
13
"
name
=
"
priceExponent
"
type
=
"
int8
"
description
=
"
Price exponent for decimal point positioning
"
/>
<
field
id
=
"
14
"
name
=
"
sizeExponent
"
type
=
"
int8
"
description
=
"
Size exponent for decimal point positioning
"
/>
<
data
id
=
"
55
"
name
=
"
symbol
"
type
=
"
varString8
"
/>
</
sbe:
message
>
</
sbe:
messageSchema
>
Message Structure Details
​
Message Header (8 bytes)
​
Field
Type
Size (bytes)
Description
blockLength
uint16
2
Message body length
templateId
uint16
2
Fixed = 20000
schemaId
uint16
2
Fixed = 1
version
uint16
2
Fixed = 0
Message Body (
BestOBRpiEvent
)
​
ID
Field
Type
Description
1
ts
int64
Snapshot timestamp (µs)
2
seq
int64
Unique message sequence number
3
cts
int64
Trade timestamp (µs)
4
u
int64
Update ID
5
askNormalPrice
int64
Best ask price mantissa
6
askNormalSize
int64
Best ask size (normal) mantissa
7
askRpiPrice
int64
Best RPI ask price mantissa
8
askRpiSize
int64
Best RPI ask size mantissa
9
bidNormalPrice
int64
Best bid price mantissa
10
bidNormalSize
int64
Best bid size (normal) mantissa
11
bidRpiPrice
int64
Best bid price (RPI) mantissa
12
bidRpiSize
int64
Best bid size (RPI) mantissa
13
priceExponent
int8
Price exponent
14
sizeExponent
int8
Size exponent
55
symbol
varStr
Trading pair (e.g.,
BTCUSDT
)
Optimisation
​
New field definitions (sell side as example; buy side is analogous):
Field
Definition
askNormalPrice
No RPI order best ask price
askNormalSize
No RPI order best ask size
askRpiPrice
RPI order best ask price
askRpiSize
RPI order best ask size
The current logic might result in:
Price
normalQty
rpiQty
1000
0
100
This means that users without RPI permissions won't know at what price they can take their orders, making this message useless. To address this, we adjust the message content as follows.
Case 1:
askNormalSize != 0 && askRpiSize != 0
​
This is a normal response based on the actual situation. This case conveys the same meaning as the original message.
Example:
Field
Definition
Example
askNormalPrice
No RPI order best ask price
1000
askNormalSize
No RPI order best ask size
200
askRpiPrice
RPI order best ask price
1000
askRpiSize
RPI order best ask size
300
Case 2:
askNormalSize != 0 && askRpiSize == 0
​
askRpiPrice
is assigned the value
askNormalPrice
, and
askRpiSize = 0
.
In this case, the
askRpiPrice
value will not be searched further.
Example:
Field
Definition
Example
Note
askNormalPrice
No RPI order best ask price
1000
askNormalSize
No RPI order best ask size
200
askRpiPrice
RPI order best ask price
1000
This itself has no meaning. The price field is assigned a non-RPI sell price.
askRpiSize
RPI order best ask size
0
Case 3:
askNormalSize == 0 && askRpiSize != 0
​
askNormalPrice =
the actual non-RPI asking price.
In this case,
askNormalPrice
is retrieved and returned.
Example:
Field
Definition
Example
askNormalPrice
No RPI order best ask price
1200
askNormalSize
No RPI order best ask size
100
askRpiPrice
RPI order best ask price
1000
askRpiSize
RPI order best ask size
20
Case 4
​
When the market is so bad that there is no liquidity,
no message is pushed
.
Integration Example
​
import
json
import
logging
import
struct
import
threading
import
time
from
datetime
import
datetime
from
typing
import
Dict
,
Any
import
websocket
logging
.
basicConfig
(
filename
=
"logfile_wrapper.log"
,
level
=
logging
.
INFO
,
format
=
"%(asctime)s %(levelname)s %(message)s"
,
)
# Change symbol/topic as you wish
TOPIC
=
"ob.rpi.1.sbe.BTCUSDT"
WS_URL
=
"wss://stream-testnet.bybits.org/v5/public-sbe/spot"
class
SBEBestOBRpiParser
:
"""
Parser for BestOBRpiEvent (template_id = 20000) per XML schema:
ts(int64), seq(int64), cts(int64), u(int64),
askNormalPrice(int64), askNormalSize(int64),
askRpiPrice(int64), askRpiSize(int64),
bidNormalPrice(int64), bidNormalSize(int64),
bidRpiPrice(int64), bidRpiSize(int64),
priceExponent(int8), sizeExponent(int8),
symbol(varString8)
All values are little-endian.
"""
def
__init__
(
self
)
-
>
None
:
# Header: blockLength, templateId, schemaId, version
self
.
header_fmt
=
"<HHHH"
self
.
header_sz
=
struct
.
calcsize
(
self
.
header_fmt
)
# 12 x int64 + 2 x int8:
# ts, seq, cts, u,
# askNormalPrice, askNormalSize, askRpiPrice, askRpiSize,
# bidNormalPrice, bidNormalSize, bidRpiPrice, bidRpiSize,
# priceExponent, sizeExponent
self
.
body_fmt
=
"<"
+
(
"q"
*
12
)
+
"bb"
self
.
body_sz
=
struct
.
calcsize
(
self
.
body_fmt
)
self
.
target_template_id
=
20000
def
_parse_header
(
self
,
data
:
bytes
)
-
>
Dict
[
str
,
Any
]
:
if
len
(
data
)
<
self
.
header_sz
:
raise
ValueError
(
"insufficient data for SBE header"
)
block_length
,
template_id
,
schema_id
,
version
=
struct
.
unpack_from
(
self
.
header_fmt
,
data
,
0
)
return
{
"block_length"
:
block_length
,
"template_id"
:
template_id
,
"schema_id"
:
schema_id
,
"version"
:
version
,
}
@staticmethod
def
_parse_varstring8
(
data
:
bytes
,
offset
:
int
)
-
>
tuple
[
str
,
int
]
:
if
offset
+
1
>
len
(
data
)
:
raise
ValueError
(
"insufficient data for varString8 length"
)
(
length
,
)
=
struct
.
unpack_from
(
"<B"
,
data
,
offset
)
offset
+=
1
if
offset
+
length
>
len
(
data
)
:
raise
ValueError
(
"insufficient data for varString8 bytes"
)
s
=
data
[
offset
:
offset
+
length
]
.
decode
(
"utf-8"
)
offset
+=
length
return
s
,
offset
@staticmethod
def
_apply_exponent
(
value
:
int
,
exponent
:
int
)
-
>
float
:
# Exponent is for decimal point positioning.
# If exponent = 2 and value=1060342500 -> 10603425.00
return
value
/
(
10
**
exponent
)
if
exponent
>=
0
else
value
*
(
10
**
(
-
exponent
)
)
def
parse
(
self
,
data
:
bytes
)
-
>
Dict
[
str
,
Any
]
:
hdr
=
self
.
_parse_header
(
data
)
if
hdr
[
"template_id"
]
!=
self
.
target_template_id
:
raise
NotImplementedError
(
f"unsupported template_id=
{
hdr
[
'template_id'
]
}
"
)
if
len
(
data
)
<
self
.
header_sz
+
self
.
body_sz
:
raise
ValueError
(
"insufficient data for BestOBRpiEvent body"
)
fields
=
struct
.
unpack_from
(
self
.
body_fmt
,
data
,
self
.
header_sz
)
(
ts
,
seq
,
cts
,
u
,
ask_np_m
,
ask_ns_m
,
ask_rp_m
,
ask_rs_m
,
bid_np_m
,
bid_ns_m
,
bid_rp_m
,
bid_rs_m
,
price_exp
,
size_exp
,
)
=
fields
offset
=
self
.
header_sz
+
self
.
body_sz
symbol
,
offset
=
self
.
_parse_varstring8
(
data
,
offset
)
# Apply exponents
ask_np
=
self
.
_apply_exponent
(
ask_np_m
,
price_exp
)
ask_ns
=
self
.
_apply_exponent
(
ask_ns_m
,
size_exp
)
ask_rp
=
self
.
_apply_exponent
(
ask_rp_m
,
price_exp
)
ask_rs
=
self
.
_apply_exponent
(
ask_rs_m
,
size_exp
)
bid_np
=
self
.
_apply_exponent
(
bid_np_m
,
price_exp
)
bid_ns
=
self
.
_apply_exponent
(
bid_ns_m
,
size_exp
)
bid_rp
=
self
.
_apply_exponent
(
bid_rp_m
,
price_exp
)
bid_rs
=
self
.
_apply_exponent
(
bid_rs_m
,
size_exp
)
return
{
"header"
:
hdr
,
"ts"
:
ts
,
"seq"
:
seq
,
"cts"
:
cts
,
"u"
:
u
,
"price_exponent"
:
price_exp
,
"size_exponent"
:
size_exp
,
"symbol"
:
symbol
,
# Normal book (best)
"ask_normal_price"
:
ask_np
,
"ask_normal_size"
:
ask_ns
,
"bid_normal_price"
:
bid_np
,
"bid_normal_size"
:
bid_ns
,
# RPI book (best)
"ask_rpi_price"
:
ask_rp
,
"ask_rpi_size"
:
ask_rs
,
"bid_rpi_price"
:
bid_rp
,
"bid_rpi_size"
:
bid_rs
,
"parsed_length"
:
offset
,
}
parser
=
SBEBestOBRpiParser
(
)
# --------------------------- WebSocket handlers ---------------------------
def
on_message
(
ws
,
message
)
:
try
:
# Binary SBE frames; text frames for control/acks/errors
if
isinstance
(
message
,
(
bytes
,
bytearray
)
)
:
decoded
=
parser
.
parse
(
message
)
logging
.
info
(
"SBE %s seq=%s u=%s "
"NORM bid=%.8f@%.8f ask=%.8f@%.8f "
"RPI bid=%.8f@%.8f ask=%.8f@%.8f ts=%s"
,
decoded
[
"symbol"
]
,
decoded
[
"seq"
]
,
decoded
[
"u"
]
,
decoded
[
"bid_normal_price"
]
,
decoded
[
"bid_normal_size"
]
,
decoded
[
"ask_normal_price"
]
,
decoded
[
"ask_normal_size"
]
,
decoded
[
"bid_rpi_price"
]
,
decoded
[
"bid_rpi_size"
]
,
decoded
[
"ask_rpi_price"
]
,
decoded
[
"ask_rpi_size"
]
,
decoded
[
"ts"
]
,
)
print
(
f"
{
decoded
[
'symbol'
]
}
u=
{
decoded
[
'u'
]
}
"
f"NORM:
{
decoded
[
'bid_normal_price'
]
:
.8f
}
x
{
decoded
[
'bid_normal_size'
]
:
.8f
}
"
f"|
{
decoded
[
'ask_normal_price'
]
:
.8f
}
x
{
decoded
[
'ask_normal_size'
]
:
.8f
}
"
f"RPI:
{
decoded
[
'bid_rpi_price'
]
:
.8f
}
x
{
decoded
[
'bid_rpi_size'
]
:
.8f
}
"
f"|
{
decoded
[
'ask_rpi_price'
]
:
.8f
}
x
{
decoded
[
'ask_rpi_size'
]
:
.8f
}
"
f"(seq=
{
decoded
[
'seq'
]
}
ts=
{
decoded
[
'ts'
]
}
)"
)
else
:
try
:
obj
=
json
.
loads
(
message
)
logging
.
info
(
"TEXT %s"
,
obj
)
print
(
obj
)
except
json
.
JSONDecodeError
:
logging
.
warning
(
"non-JSON text frame: %r"
,
message
)
except
Exception
as
e
:
logging
.
exception
(
"decode error: %s"
,
e
)
print
(
"decode error:"
,
e
)
def
on_error
(
ws
,
error
)
:
print
(
"WS error:"
,
error
)
logging
.
error
(
"WS error: %s"
,
error
)
def
on_close
(
ws
,
*
_
)
:
print
(
"### connection closed ###"
)
logging
.
info
(
"connection closed"
)
def
on_open
(
ws
)
:
print
(
"opened"
)
sub
=
{
"op"
:
"subscribe"
,
"args"
:
[
TOPIC
]
}
ws
.
send
(
json
.
dumps
(
sub
)
)
print
(
"subscribed:"
,
TOPIC
)
threading
.
Thread
(
target
=
ping_per
,
args
=
(
ws
,
)
,
daemon
=
True
)
.
start
(
)
threading
.
Thread
(
target
=
manage_subscription
,
args
=
(
ws
,
)
,
daemon
=
True
)
.
start
(
)
def
manage_subscription
(
ws
)
:
# demo: unsubscribe/resubscribe once
time
.
sleep
(
20
)
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
"unsubscribe"
,
"args"
:
[
TOPIC
]
}
)
)
print
(
"unsubscribed:"
,
TOPIC
)
time
.
sleep
(
5
)
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
"subscribe"
,
"args"
:
[
TOPIC
]
}
)
)
print
(
"resubscribed:"
,
TOPIC
)
def
ping_per
(
ws
)
:
while
True
:
try
:
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
"ping"
}
)
)
except
Exception
:
return
time
.
sleep
(
10
)
def
on_pong
(
ws
,
*
_
)
:
print
(
"pong received"
)
def
on_ping
(
ws
,
*
_
)
:
print
(
"ping received @"
,
datetime
.
now
(
)
.
strftime
(
"%Y-%m-%d %H:%M:%S"
)
)
def
connWS
(
)
:
ws
=
websocket
.
WebSocketApp
(
WS_URL
,
on_open
=
on_open
,
on_message
=
on_message
,
on_error
=
on_error
,
on_close
=
on_close
,
on_ping
=
on_ping
,
on_pong
=
on_pong
,
)
ws
.
run_forever
(
ping_interval
=
20
,
ping_timeout
=
10
)
if
__name__
==
"__main__"
:
websocket
.
enableTrace
(
False
)
connWS
(
)

**Examples:**

Example 1 ():
```
{  "op": "subscribe",  "args": ["ob.rpi.1.sbe.BTCUSDT"]}
```

Example 2 ():
```
{  "success": true,  "ret_msg": "",  "conn_id": "d30fdpbboasp1pjbe7r0",  "req_id": "xxx",  "op": "subscribe"}
```

Example 3 ():
```
b"R\x00 N\x01\x00\x00\x00\xdb\x84\xd0k\x00\x00\x00\x00f\xb7\x003\x99\x01\x00\x00\x02\x06\xa1\xcb\xa1\x00\x00\x00\x00\x00\xe7\xda\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\xc8\xa1\x00\x00\x00\x00\x00 N\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x008\x01\x00\x00\x00\x00\x00\x00v\xba\x003\x99\x01\x00\x00\x07BTCUSDT"
```

Example 4 ():
```
{  "header": {    "block_length": 82,    "template_id": 20000,    "schema_id": 1,    "version": 0  },  "seq": 1808827611,  "cts": 1757497309030,  "price_exponent": 2,  "size_exponent": 6,  "ask_price": 1060342500,  "ask_normal_size": 776935000000,  "ask_rpi_size": 0,  "bid_price": 1060250000,  "bid_normal_size": 20000000000,  "bid_rpi_size": 0,  "u": 312,  "ts": 1757497309814,  "symbol": "BTCUSDT",  "parsed_length": 98}
```

---

## Get Single Coin Balance

**URL:** https://bybit-exchange.github.io/docs/v5/asset/balance/account-coin-balance

**Contents:**
- Get Single Coin Balance
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Balances
Get Single Coin Balance
On this page
Get Single Coin Balance
Query the balance of a specific coin in a specific
account type
. Supports querying sub UID's balance.
Also, you can check the transferable amount from master to sub account, sub to master account or sub to sub account, especially
for user who has an institutional loan.
important
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/asset/transfer/query-account-coin-balance
Request Parameters
​
Parameter
Required
Type
Comments
memberId
false
string
UID.
Required
when querying sub UID balance with master api key
toMemberId
false
string
UID.
Required
when querying the transferable balance between different UIDs
accountType
true
string
Account type
toAccountType
false
string
To account type.
Required
when querying the transferable balance between different account types
coin
true
string
Coin, uppercase only
withBonus
false
integer
0
(default): not query bonus.
1
: query bonus
withTransferSafeAmount
false
integer
Whether query delay withdraw/transfer safe amount
0
(default): false,
1
: true
What is
delay withdraw amount
?
withLtvTransferSafeAmount
false
integer
For OTC loan users in particular, you can check the transferable amount under risk level
0
(default): false,
1
: true
toAccountType
is mandatory
Response Parameters
​
Parameter
Type
Comments
accountType
string
Account type
bizType
integer
Biz type
accountId
string
Account ID
memberId
string
Uid
balance
Object
> coin
string
Coin
> walletBalance
string
Wallet balance
> transferBalance
string
Transferable balance
> bonus
string
bonus
> transferSafeAmount
string
Safe amount to transfer. Keep
""
if not query
> ltvTransferSafeAmount
string
Transferable amount for ins loan account. Keep
""
if not query
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/transfer/query-account-coin-balance?accountType=UNIFIED&coin=USDT&toAccountType=FUND&withLtvTransferSafeAmount=1
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
xxxxx
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1690254520644
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_coin_balance
(
accountType
=
"UNIFIED"
,
coin
=
"BTC"
,
memberId
=
592324
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getCoinBalance
(
{
accountType
:
'UNIFIED'
,
coin
:
'USDT'
,
toAccountType
:
'FUND'
,
withLtvTransferSafeAmount
:
1
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"success"
,
"result"
:
{
"accountType"
:
"UNIFIED"
,
"bizType"
:
1
,
"accountId"
:
"1631385"
,
"memberId"
:
"1631373"
,
"balance"
:
{
"coin"
:
"USDT"
,
"walletBalance"
:
"11999"
,
"transferBalance"
:
"11999"
,
"bonus"
:
"0"
,
"transferSafeAmount"
:
""
,
"ltvTransferSafeAmount"
:
"7602.4861"
}
}
,
"retExtInfo"
:
{
}
,
"time"
:
1690254521256
}

**Examples:**

Example 1 ():
```
GET /v5/asset/transfer/query-account-coin-balance?accountType=UNIFIED&coin=USDT&toAccountType=FUND&withLtvTransferSafeAmount=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: xxxxxX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1690254520644X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_coin_balance(    accountType="UNIFIED",    coin="BTC",    memberId=592324,))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getCoinBalance({    accountType: 'UNIFIED',    coin: 'USDT',    toAccountType: 'FUND',    withLtvTransferSafeAmount: 1,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "accountType": "UNIFIED",        "bizType": 1,        "accountId": "1631385",        "memberId": "1631373",        "balance": {            "coin": "USDT",            "walletBalance": "11999",            "transferBalance": "11999",            "bonus": "0",            "transferSafeAmount": "",            "ltvTransferSafeAmount": "7602.4861"        }    },    "retExtInfo": {},    "time": 1690254521256}
```

---

## Get Borrow Quota (Spot)

**URL:** https://bybit-exchange.github.io/docs/v5/order/spot-borrow-quota

**Contents:**
- Get Borrow Quota (Spot)
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Get Borrow Quota (Spot)
On this page
Get Borrow Quota (Spot)
Query the available balance for Spot trading and Margin trading
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/order/spot-borrow-check
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type
spot
symbol
true
string
Symbol name
side
true
string
Transaction side.
Buy
,
Sell
Response Parameters
​
Parameter
Type
Comments
symbol
string
Symbol name, like
BTCUSDT
, uppercase only
side
string
Side
maxTradeQty
string
The maximum base coin qty can be traded
If spot margin trade on and symbol is margin trading pair, it returns available balance + max.borrowable quantity = min(The maximum quantity that a single user can borrow on the platform, The maximum quantity that can be borrowed calculated by IMR MMR of UTA account, The available quantity of the platform's capital pool)
Otherwise, it returns actual available balance
up to 4 decimals
maxTradeAmount
string
The maximum quote coin amount can be traded
If spot margin trade on and symbol is margin trading pair, it returns available balance + max.borrowable amount = min(The maximum amount that a single user can borrow on the platform, The maximum amount that can be borrowed calculated by IMR MMR of UTA account, The available amount of the platform's capital pool)
Otherwise, it returns actual available balance
up to 8 decimals
spotMaxTradeQty
string
No matter your Spot margin switch on or not, it always returns actual qty of base coin you can trade or you have (borrowable qty is not included), up to 4 decimals
spotMaxTradeAmount
string
No matter your Spot margin switch on or not, it always returns actual amount of quote coin you can trade or you have (borrowable amount is not included), up to 8 decimals
borrowCoin
string
Borrow coin
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
GET
/v5/order/spot-borrow-check?category=spot&symbol=BTCUSDT&side=Buy
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672228522214
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_borrow_quota
(
category
=
"spot"
,
symbol
=
"BTCUSDT"
,
side
=
"Buy"
,
)
)
import
com
.
bybit
.
api
.
client
.
config
.
BybitApiConfig
;
import
com
.
bybit
.
api
.
client
.
domain
.
trade
.
request
.
TradeOrderRequest
;
import
com
.
bybit
.
api
.
client
.
domain
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
trade
.
*
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
,
BybitApiConfig
.
TESTNET_DOMAIN
)
.
newTradeRestClient
(
)
;
var
getBorrowQuotaRequest
=
TradeOrderRequest
.
builder
(
)
.
category
(
CategoryType
.
SPOT
)
.
symbol
(
"BTCUSDT"
)
.
side
(
Side
.
BUY
)
.
build
(
)
;
System
.
out
.
println
(
client
.
getBorrowQuota
(
getBorrowQuotaRequest
)
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getSpotBorrowCheck
(
'BTCUSDT'
,
'Buy'
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"symbol"
:
"BTCUSDT"
,
"maxTradeQty"
:
"6.6065"
,
"side"
:
"Buy"
,
"spotMaxTradeAmount"
:
"9004.75628594"
,
"maxTradeAmount"
:
"218014.01330797"
,
"borrowCoin"
:
"USDT"
,
"spotMaxTradeQty"
:
"0.2728"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1698895841534
}

**Examples:**

Example 1 ():
```
GET /v5/order/spot-borrow-check?category=spot&symbol=BTCUSDT&side=Buy HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672228522214X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_borrow_quota(    category="spot",    symbol="BTCUSDT",    side="Buy",))
```

Example 3 ():
```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var getBorrowQuotaRequest = TradeOrderRequest.builder().category(CategoryType.SPOT).symbol("BTCUSDT").side(Side.BUY).build();System.out.println(client.getBorrowQuota(getBorrowQuotaRequest));
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getSpotBorrowCheck('BTCUSDT', 'Buy')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Tickers

**URL:** https://bybit-exchange.github.io/docs/v5/market/tickers

**Contents:**
- Get Tickers
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Tickers
On this page
Get Tickers
Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
Covers: Spot / USDT contract / USDC contract / Inverse contract / Option
info
If category=
option
,
symbol
or
baseCoin
must be passed.
HTTP Request
​
GET
/v5/market/tickers
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
spot
,
linear
,
inverse
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
baseCoin
false
string
Base coin, uppercase only. Apply to
option
only
expDate
false
string
Expiry date. e.g., 25DEC22. Apply to
option
only
Response Parameters
​
Linear/Inverse
Option
Spot
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
> lastPrice
string
Last price
> indexPrice
string
Index price
> markPrice
string
Mark price
> prevPrice24h
string
Market price 24 hours ago
> price24hPcnt
string
Percentage change of market price relative to 24h
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> prevPrice1h
string
Market price an hour ago
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
> fundingRate
string
Funding rate
> nextFundingTime
string
Next funding time (ms)
> predictedDeliveryPrice
string
Predicated delivery price. It has a value 30 mins before delivery
> basisRate
string
Basis rate
> basis
string
Basis
> deliveryFeeRate
string
Delivery fee rate
> deliveryTime
string
Delivery timestamp (ms), applicable to expiry futures only
> ask1Size
string
Best ask size
> bid1Price
string
Best bid price
> ask1Price
string
Best ask price
> bid1Size
string
Best bid size
> preOpenPrice
string
Estimated pre-market contract open price
Meaningless once the market opens
> preQty
string
Estimated pre-market contract open qty
The value is meaningless once the market opens
>
curPreListingPhase
string
The current pre-market contract phase
> fundingIntervalHour
string
Funding interval hour
This value currently only supports whole hours
> fundingCap
string
Funding rate upper and lower limits
> basisRateYear
string
Annual basis rate
Only for Futures,For Perpetual,it will return ""
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
> bid1Price
string
Best bid price
> bid1Size
string
Best bid size
> bid1Iv
string
Best bid iv
> ask1Price
string
Best ask price
> ask1Size
string
Best ask size
> ask1Iv
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
> markIv
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
Predicated delivery price. It has a value 30 mins before delivery
> change24h
string
The change in the last 24 hous
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
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
> lastPrice
string
Last price
> prevPrice24h
string
Market price 24 hours ago
> price24hPcnt
string
Percentage change of market price relative to 24h
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> turnover24h
string
Turnover for 24h
> volume24h
string
Volume for 24h
> usdIndexPrice
string
USD index price
used to calculate USD value of the assets in Unified account
non-collateral margin coin returns ""
Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
RUN >>
Request Example
​
Inverse
Option
Spot
HTTP
Python
Go
Java
Node.js
GET
/v5/market/tickers?category=inverse&symbol=BTCUSD
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_tickers
(
category
=
"inverse"
,
symbol
=
"BTCUSD"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"inverse"
,
"symbol"
:
"BTCUSD"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarketTickers
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
tickerReueqt
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
INVERSE
)
.
symbol
(
"BTCUSD"
)
.
build
(
)
;
client
.
getMarketTickers
(
tickerReueqt
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getTickers
(
{
category
:
'inverse'
,
symbol
:
'BTCUSDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
HTTP
Python
Go
Java
Node.js
GET
/v5/market/tickers?category=option&symbol=BTC-30DEC22-18000-C
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_tickers
(
category
=
"option"
,
symbol
=
"BTC-30DEC22-18000-C"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"option"
,
"symbol"
:
"BTC-30DEC22-18000-C"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarketTickers
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
tickerReueqt
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
OPTION
)
.
symbol
(
"BTC-30DEC22-18000-C"
)
.
build
(
)
;
client
.
getMarketTickers
(
tickerReueqt
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getTickers
(
{
category
:
'option'
,
symbol
:
'BTC-30DEC22-18000-C'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
HTTP
Python
GO
Java
Node.js
GET
/v5/market/tickers?category=spot&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_tickers
(
category
=
"spot"
,
symbol
=
"BTCUSDT"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarketTickers
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
*
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
tickerReueqt
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
SPOT
)
.
symbol
(
"BTCUSDT"
)
.
build
(
)
;
client
.
getMarketTickers
(
tickerReueqt
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getTickers
(
{
category
:
'spot'
,
symbol
:
'BTCUSDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
Inverse
Option
Spot
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"inverse"
,
"list"
:
[
{
"symbol"
:
"BTCUSD"
,
"lastPrice"
:
"120635.50"
,
"indexPrice"
:
"114890.92"
,
"markPrice"
:
"114898.43"
,
"prevPrice24h"
:
"105595.90"
,
"price24hPcnt"
:
"0.142425"
,
"highPrice24h"
:
"131309.30"
,
"lowPrice24h"
:
"102007.60"
,
"prevPrice1h"
:
"119806.10"
,
"openInterest"
:
"240113967"
,
"openInterestValue"
:
"2089.79"
,
"turnover24h"
:
"115.6907"
,
"volume24h"
:
"13713832.0000"
,
"fundingRate"
:
"0.0001"
,
"nextFundingTime"
:
"1760371200000"
,
"predictedDeliveryPrice"
:
""
,
"basisRate"
:
""
,
"deliveryFeeRate"
:
""
,
"deliveryTime"
:
"0"
,
"ask1Size"
:
"9854"
,
"bid1Price"
:
"103401.00"
,
"ask1Price"
:
"109152.80"
,
"bid1Size"
:
"1063"
,
"basis"
:
""
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
,
"fundingIntervalHour"
:
"8"
,
"basisRateYear"
:
""
,
"fundingCap"
:
"0.005"
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
1760352369814
}
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"option"
,
"list"
:
[
{
"symbol"
:
"BTC-30DEC22-18000-C"
,
"bid1Price"
:
"0"
,
"bid1Size"
:
"0"
,
"bid1Iv"
:
"0"
,
"ask1Price"
:
"435"
,
"ask1Size"
:
"0.66"
,
"ask1Iv"
:
"5"
,
"lastPrice"
:
"435"
,
"highPrice24h"
:
"435"
,
"lowPrice24h"
:
"165"
,
"markPrice"
:
"0.00000009"
,
"indexPrice"
:
"16600.55"
,
"markIv"
:
"0.7567"
,
"underlyingPrice"
:
"16590.42"
,
"openInterest"
:
"6.3"
,
"turnover24h"
:
"2482.73"
,
"volume24h"
:
"0.15"
,
"totalVolume"
:
"99"
,
"totalTurnover"
:
"1967653"
,
"delta"
:
"0.00000001"
,
"gamma"
:
"0.00000001"
,
"vega"
:
"0.00000004"
,
"theta"
:
"-0.00000152"
,
"predictedDeliveryPrice"
:
"0"
,
"change24h"
:
"86"
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
1672376592395
}
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"spot"
,
"list"
:
[
{
"symbol"
:
"BTCUSDT"
,
"bid1Price"
:
"20517.96"
,
"bid1Size"
:
"2"
,
"ask1Price"
:
"20527.77"
,
"ask1Size"
:
"1.862172"
,
"lastPrice"
:
"20533.13"
,
"prevPrice24h"
:
"20393.48"
,
"price24hPcnt"
:
"0.0068"
,
"highPrice24h"
:
"21128.12"
,
"lowPrice24h"
:
"20318.89"
,
"turnover24h"
:
"243765620.65899866"
,
"volume24h"
:
"11801.27771"
,
"usdIndexPrice"
:
"20784.12009279"
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
1673859087947
}

**Examples:**

Example 1 ():
```
GET /v5/market/tickers?category=inverse&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_tickers(    category="inverse",    symbol="BTCUSD",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "inverse", "symbol": "BTCUSD"}client.NewUtaBybitServiceWithParams(params).GetMarketTickers(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var tickerReueqt = MarketDataRequest.builder().category(CategoryType.INVERSE).symbol("BTCUSD").build();client.getMarketTickers(tickerReueqt, System.out::println);
```

---

## Get Delivery Price

**URL:** https://bybit-exchange.github.io/docs/v5/market/delivery-price

**Contents:**
- Get Delivery Price
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Delivery Price
On this page
Get Delivery Price
Get the delivery price.
Covers: USDT futures / USDC futures / Inverse futures / Option
info
Option: only returns those symbols which are
DELIVERING
(UTC 8 - UTC 12) when
symbol
is not specified.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/delivery-price
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
linear
,
inverse
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
baseCoin
false
string
Base coin, uppercase only. Default:
BTC
.
Valid for
option
only
settleCoin
false
string
Settle coin, uppercase only. Default:
USDC
.
limit
false
integer
Limit for data size per page.
[
1
,
200
]
. Default:
50
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
> deliveryPrice
string
Delivery price
> deliveryTime
string
Delivery timestamp (ms)
nextPageCursor
string
Refer to the
cursor
request parameter
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/delivery-price?category=option&symbol=ETH-26DEC22-1400-C
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_option_delivery_price
(
category
=
"option"
,
symbol
=
"ETH-26DEC22-1400-C"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"ETH-26DEC22-1400-C"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetDeliveryPrice
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
deliveryPriceRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
OPTION
)
.
baseCoin
(
"BTC"
)
.
limit
(
10
)
.
build
(
)
;
client
.
getDeliveryPrice
(
deliveryPriceRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getDeliveryPrice
(
{
category
:
'option'
,
symbol
:
'ETH-26DEC22-1400-C'
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"success"
,
"result"
:
{
"category"
:
"option"
,
"nextPageCursor"
:
""
,
"list"
:
[
{
"symbol"
:
"ETH-26DEC22-1400-C"
,
"deliveryPrice"
:
"1220.728594450"
,
"deliveryTime"
:
"1672041600000"
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
1672055336993
}

**Examples:**

Example 1 ():
```
GET /v5/market/delivery-price?category=option&symbol=ETH-26DEC22-1400-C HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP()print(session.get_option_delivery_price(    category="option",    symbol="ETH-26DEC22-1400-C",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "ETH-26DEC22-1400-C"}client.NewUtaBybitServiceWithParams(params).GetDeliveryPrice(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var deliveryPriceRequest = MarketDataRequest.builder().category(CategoryType.OPTION).baseCoin("BTC").limit(10).build();client.getDeliveryPrice(deliveryPriceRequest, System.out::println);
```

---

## Get Coin Greeks

**URL:** https://bybit-exchange.github.io/docs/v5/account/coin-greeks

**Contents:**
- Get Coin Greeks
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Get Coin Greeks
On this page
Get Coin Greeks
Get current account Greeks information
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/asset/coin-greeks
Request Parameters
​
Parameter
Required
Type
Comments
baseCoin
false
string
Base coin, uppercase only. If not passed, all supported base coin greeks will be returned by default
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> baseCoin
string
Base coin. e.g.,
BTC
,
ETH
,
SOL
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
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/coin-greeks?baseCoin=BTC
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672287887610
X-BAPI-RECV-WINDOW
:
5000
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
api_key
=
"xxxxxxxxxxxxxxxxxx"
,
api_secret
=
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
,
)
print
(
session
.
get_coin_greeks
(
baseCoin
=
"BTC"
,
)
)
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
key
:
'xxxxxxxxxxxxxxxxxx'
,
secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
,
}
)
;
client
.
getCoinGreeks
(
'BTC'
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"baseCoin"
:
"BTC"
,
"totalDelta"
:
"0.00004001"
,
"totalGamma"
:
"-0.00000009"
,
"totalVega"
:
"-0.00039689"
,
"totalTheta"
:
"0.01243824"
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
1672287887942
}

**Examples:**

Example 1 ():
```
GET /v5/asset/coin-greeks?baseCoin=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672287887610X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_coin_greeks(    baseCoin="BTC",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getCoinGreeks('BTC')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "baseCoin": "BTC",                "totalDelta": "0.00004001",                "totalGamma": "-0.00000009",                "totalVega": "-0.00039689",                "totalTheta": "0.01243824"            }        ]    },    "retExtInfo": {},    "time": 1672287887942}
```

---

## ADL Alert

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/adl-alert

**Contents:**
- ADL Alert
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
ADL Alert
On this page
ADL Alert
Subscribe to ADL alerts and insurance pool information.
Covers: USDT Perpetual / USDT Delivery / USDC Perpetual / USDC Delivery / Inverse Contracts
Push frequency:
1s
Topic:
adlAlert.{coin}
Available filters:
adlAlert.USDT
for USDT Perpetual/Delivery
adlAlert.USDC
for USDC Perpetual/Delivery
adlAlert.inverse
for Inverse contracts.
For more information on how ADL is triggered, see the
ADL endpoint
.
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> c
string
Token of the insurance pool
> s
string
Trading pair name
> b
string
Balance of the insurance fund. Used to determine if ADL is triggered. For shared insurance pool, the "b" field will follow a T+1 refresh mechanism and will be updated daily at 00:00 UTC.
> mb
string
Deprecated, always return "". Maximum balance of the insurance pool in the last 8 hours
> i_pr
string
PnL ratio threshold for triggering
contract PnL drawdown ADL
ADL is triggered when the symbol's PnL drawdown ratio in the last 8 hours exceeds this value
> pr
string
Symbol's PnL drawdown ratio in the last 8 hours. Used to determine whether ADL is triggered or stopped
> adl_tt
string
Trigger threshold for
contract PnL drawdown ADL
This condition is only effective when the insurance pool balance is greater than this value; if so, an 8 hours drawdown exceeding n% may trigger ADL
> adl_sr
string
Stop ratio threshold for
contract PnL drawdown ADL
ADL stops when the symbol's 8 hours drawdown ratio falls below this value
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
"adlAlert.USDT"
]
}
Response Example
​
{
"topic"
:
"adlAlert.USDT"
,
"type"
:
"snapshot"
,
"ts"
:
1757736794000
,
"data"
:
[
{
"c"
:
"USDT"
,
"s"
:
"FWOGUSDT"
,
"b"
:
-5421.29889888
,
"mb"
:
-5421.29889888
,
"i_pr"
:
-0.3
,
"pr"
:
0
,
"adl_tt"
:
10000
,
"adl_sr"
:
-0.25
}
,
{
"c"
:
"USDT"
,
"s"
:
"ZORAUSDT"
,
"b"
:
19873.46255153
,
"mb"
:
19874.97612833
,
"i_pr"
:
-0.3
,
"pr"
:
0.000174
,
"adl_tt"
:
10000
,
"adl_sr"
:
-0.25
}
,
{
"c"
:
"USDT"
,
"s"
:
"BERAUSDT"
,
"b"
:
453.36427074
,
"mb"
:
453.36427074
,
"i_pr"
:
-0.3
,
"pr"
:
0.24576
,
"adl_tt"
:
10000
,
"adl_sr"
:
-0.25
}
,
...
,
]
}

**Examples:**

Example 1 ():
```
{"op": "subscribe", "args": ["adlAlert.USDT"]}
```

Example 2 ():
```
{  "topic": "adlAlert.USDT",  "type": "snapshot",  "ts": 1757736794000,  "data": [    {      "c": "USDT",      "s": "FWOGUSDT",      "b": -5421.29889888,      "mb": -5421.29889888,      "i_pr": -0.3,      "pr": 0,      "adl_tt": 10000,      "adl_sr": -0.25    },    {      "c": "USDT",      "s": "ZORAUSDT",      "b": 19873.46255153,      "mb": 19874.97612833,      "i_pr": -0.3,      "pr": 0.000174,      "adl_tt": 10000,      "adl_sr": -0.25    },    {      "c": "USDT",      "s": "BERAUSDT",      "b": 453.36427074,      "mb": 453.36427074,      "i_pr": -0.3,      "pr": 0.24576,      "adl_tt": 10000,      "adl_sr": -0.25    },    ...,  ]}
```

---

## Get Instruments Info

**URL:** https://bybit-exchange.github.io/docs/v5/market/instrument

**Contents:**
- Get Instruments Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Instruments Info
On this page
Get Instruments Info
Query for the instrument specification of online trading pairs.
Covers: Spot / USDT contract / USDC contract / Inverse contract / Option
info
Spot does not support pagination, so
limit
,
cursor
are invalid.
When querying by
baseCoin
, regardless of if category=
linear
or
inverse
, the result will contain USDT contract, USDC contract and Inverse contract symbols.
caution
This endpoint returns 500 entries by default. There are now more than 500
linear
symbols on the platform. As a result, you will need to use
cursor
for pagination or
limit
to get all entries.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/instruments-info
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
spot
,
linear
,
inverse
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
symbolType
false
string
SymbolType:The region to which the trading pair belongs,only for
linear
,
inverse
,
spot
status
false
string
Symbol status filter
linear
&
inverse
&
spot
By default returns only
Trading
symbols
option
By default returns
PreLaunch
,
Trading
, and
Delivering
Spot has
Trading
only
linear
&
inverse
: when status=PreLaunch, it returns
Pre-Market contracts
baseCoin
false
string
Base coin, uppercase only
Applies to
linear
,
inverse
,
option
only
option
: returns BTC by default
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
500
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Linear/Inverse
Option
Spot
Parameter
Type
Comments
category
string
Product type
nextPageCursor
string
Cursor. Used to pagination
list
array
Object
> symbol
string
Symbol name
>
contractType
string
Contract type
>
status
string
Instrument status
> baseCoin
string
Base coin
> quoteCoin
string
Quote coin
>
symbolType
string
the region to which the trading pair belongs
> launchTime
string
Launch timestamp (ms)
> deliveryTime
string
Delivery timestamp (ms)
Expired futures delivery time
Perpetual delisting time
> deliveryFeeRate
string
Delivery fee rate
> priceScale
string
Price scale
> leverageFilter
Object
Leverage attributes
>> minLeverage
string
Minimum leverage
>> maxLeverage
string
Maximum leverage
>> leverageStep
string
The step to increase/reduce leverage
> priceFilter
Object
Price attributes
>> minPrice
string
Minimum order price
>> maxPrice
string
Maximum order price
>> tickSize
string
The step to increase/reduce order price
> lotSizeFilter
Object
Size attributes
>> minNotionalValue
string
Minimum notional value
>> maxOrderQty
string
Maximum quantity for Limit and PostOnly order
>> maxMktOrderQty
string
Maximum quantity for Market order
>> minOrderQty
string
Minimum order quantity
>> qtyStep
string
The step to increase/reduce order quantity
>> postOnlyMaxOrderQty
string
deprecated, please use
maxOrderQty
> unifiedMarginTrade
boolean
Whether to support unified margin trade
> fundingInterval
integer
Funding interval (minute)
> settleCoin
string
Settle coin
>
copyTrading
string
Copy trade symbol or not
> upperFundingRate
string
Upper limit of funding date
> lowerFundingRate
string
Lower limit of funding date
> displayName
string
The USDC futures & perpetual name displayed in the Web or App
> forbidUplWithdrawal
boolean
Whether to prohibit unrealised profit withdrawal
> riskParameters
object
Risk parameters for limit order price. Note that the
formula changed
in Jan 2025
>> priceLimitRatioX
string
Ratio X
>> priceLimitRatioY
string
Ratio Y
> isPreListing
boolean
Whether the contract is a pre-market contract
When the pre-market contract is converted to official contract, it will be false
> preListingInfo
object
If isPreListing=false, preListingInfo=null
If isPreListing=true, preListingInfo is an object
>>
curAuctionPhase
string
The current auction phase
>> phases
array<object>
Each phase time info
>>>
phase
string
pre-market trading phase
>>> startTime
string
The start time of the phase, timestamp(ms)
>>> endTime
string
The end time of the phase, timestamp(ms)
>> auctionFeeInfo
object
Action fee info
>>> auctionFeeRate
string
The trading fee rate during auction phase
There is no trading fee until entering continues trading phase
>>> takerFeeRate
string
The taker fee rate during continues trading phase
>>> makerFeeRate
string
The maker fee rate during continues trading phase
>> skipCallAuction
boolean
false
,
true
Whether the pre-market contract skips the call auction phase
Parameter
Type
Comments
category
string
Product type
nextPageCursor
string
Cursor. Used to pagination
list
array
Object
> symbol
string
Symbol name
> optionsType
string
Option type.
Call
,
Put
>
status
string
Instrument status
> baseCoin
string
Base coin
> quoteCoin
string
Quote coin
> settleCoin
string
Settle coin
> launchTime
string
Launch timestamp (ms)
> deliveryTime
string
Delivery timestamp (ms)
> deliveryFeeRate
string
Delivery fee rate
> priceFilter
Object
Price attributes
>> minPrice
string
Minimum order price
>> maxPrice
string
Maximum order price
>> tickSize
string
The step to increase/reduce order price
> lotSizeFilter
Object
Size attributes
>> maxOrderQty
string
Maximum order quantity
>> minOrderQty
string
Minimum order quantity
>> qtyStep
string
The step to increase/reduce order quantity
> displayName
string
The option name displayed in the Web or App
Parameter
Type
Comments
category
string
Product type
list
array
Object
> symbol
string
Symbol name
> baseCoin
string
Base coin
> quoteCoin
string
Quote coin
> innovation
string
deprecated, please use
symbolType
>
symbolType
string
the region to which the trading pair belongs
>
status
string
Instrument status
>
marginTrading
string
Whether or not this symbol supports margin trading
This is to identify if the symbol supports margin trading under different account modes
You may find some symbols do not support margin buy or margin sell, so you need to go to
Collateral Info (UTA)
to check if that coin is borrowable
When the lending pool has insufficient balance to lend out funds (can happen during big market movements) then this will switch to
none
until there is sufficient balance to re-enable margin trading
> stTag
string
Whether or not it has an
special treatment label
.
0
: false,
1
: true
> lotSizeFilter
Object
Size attributes
>> basePrecision
string
The precision of base coin
>> quotePrecision
string
The precision of quote coin
>> minOrderQty
string
Minimum order quantity, deprecated, no longer check
minOrderQty
, check
minOrderAmt
instead
>> maxOrderQty
string
Maximum order quantity, deprecated, please refer to
maxLimitOrderQty
,
maxMarketOrderQty
based on order type
>> minOrderAmt
string
Minimum order amount
>> maxOrderAmt
string
Maximum order amount, deprecated, no longer check
maxOrderAmt
, check
maxLimitOrderQty
and
maxMarketOrderQty
instead
>> maxLimitOrderQty
string
Maximum Limit order quantity
For post-only and retail price improvement (RPI) orders, the maximum limit order quantity is 5x
maxLimitOrderQty
>> maxMarketOrderQty
string
Maximum Market order quantity
>> postOnlyMaxLimitOrderSize
string
Maximum limit order size for Post-only and RPI orders
> priceFilter
Object
Price attributes
>> tickSize
string
The step to increase/reduce order price
> riskParameters
Object
Risk parameters for limit order price, refer to
announcement
>> priceLimitRatioX
string
Ratio X
>> priceLimitRatioY
string
Ratio Y
RUN >>
Request Example
​
Linear
Option
Spot
HTTP
Python
Go
Java
Node.js
GET
/v5/market/instruments-info?category=linear&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_instruments_info
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
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetInstrumentInfo
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
instrumentInfoRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
instrumentStatus
(
InstrumentStatus
.
TRADING
)
.
limit
(
500
)
.
build
(
)
;
client
.
getInstrumentsInfo
(
instrumentInfoRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getInstrumentsInfo
(
{
category
:
'linear'
,
symbol
:
'BTCUSDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
HTTP
Python
Go
Java
Node.js
GET
/v5/market/instruments-info?category=option&symbol=ETH-3JAN23-1250-P
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_instruments_info
(
category
=
"option"
,
symbol
=
"ETH-3JAN23-1250-P"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"option"
,
"symbol"
:
"ETH-3JAN23-1250-P"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetInstrumentInfo
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
instrumentInfoRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
OPTION
)
.
symbol
(
"ETH-3JAN23-1250-P"
)
.
instrumentStatus
(
InstrumentStatus
.
TRADING
)
.
limit
(
500
)
.
build
(
)
;
client
.
getInstrumentsInfo
(
instrumentInfoRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getInstrumentsInfo
(
{
category
:
'option'
,
symbol
:
'ETH-3JAN23-1250-P'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
HTTP
Python
Go
Java
Node.js
GET
/v5/market/instruments-info?category=spot&symbol=BTCUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_instruments_info
(
category
=
"spot"
,
symbol
=
"BTCUSDT"
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetInstrumentInfo
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
*
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
instrumentInfoRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
SPOT
)
.
symbol
(
"BTCUSDT"
)
.
instrumentStatus
(
InstrumentStatus
.
TRADING
)
.
limit
(
500
)
.
build
(
)
;
client
.
getInstrumentsInfo
(
instrumentInfoRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getInstrumentsInfo
(
{
category
:
'spot'
,
symbol
:
'BTCUSDT'
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
Linear
Option
Spot
// official USDT Perpetual instrument structure
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"linear"
,
"list"
:
[
{
"symbol"
:
"BTCUSDT"
,
"contractType"
:
"LinearPerpetual"
,
"status"
:
"Trading"
,
"baseCoin"
:
"BTC"
,
"quoteCoin"
:
"USDT"
,
"launchTime"
:
"1585526400000"
,
"deliveryTime"
:
"0"
,
"deliveryFeeRate"
:
""
,
"priceScale"
:
"2"
,
"leverageFilter"
:
{
"minLeverage"
:
"1"
,
"maxLeverage"
:
"100.00"
,
"leverageStep"
:
"0.01"
}
,
"priceFilter"
:
{
"minPrice"
:
"0.10"
,
"maxPrice"
:
"1999999.80"
,
"tickSize"
:
"0.10"
}
,
"lotSizeFilter"
:
{
"maxOrderQty"
:
"1190.000"
,
"minOrderQty"
:
"0.001"
,
"qtyStep"
:
"0.001"
,
"postOnlyMaxOrderQty"
:
"1190.000"
,
"maxMktOrderQty"
:
"500.000"
,
"minNotionalValue"
:
"5"
}
,
"unifiedMarginTrade"
:
true
,
"fundingInterval"
:
480
,
"settleCoin"
:
"USDT"
,
"copyTrading"
:
"both"
,
"upperFundingRate"
:
"0.00375"
,
"lowerFundingRate"
:
"-0.00375"
,
"isPreListing"
:
false
,
"preListingInfo"
:
null
,
"riskParameters"
:
{
"priceLimitRatioX"
:
"0.01"
,
"priceLimitRatioY"
:
"0.02"
}
,
"symbolType"
:
""
}
]
,
"nextPageCursor"
:
""
}
,
"retExtInfo"
:
{
}
,
"time"
:
1735809771618
}
// Pre-market Perpetual instrument structure
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"linear"
,
"list"
:
[
{
"symbol"
:
"BIOUSDT"
,
"contractType"
:
"LinearPerpetual"
,
"status"
:
"PreLaunch"
,
"baseCoin"
:
"BIO"
,
"quoteCoin"
:
"USDT"
,
"launchTime"
:
"1735032510000"
,
"deliveryTime"
:
"0"
,
"deliveryFeeRate"
:
""
,
"priceScale"
:
"4"
,
"leverageFilter"
:
{
"minLeverage"
:
"1"
,
"maxLeverage"
:
"5.00"
,
"leverageStep"
:
"0.01"
}
,
"priceFilter"
:
{
"minPrice"
:
"0.0001"
,
"maxPrice"
:
"1999.9998"
,
"tickSize"
:
"0.0001"
}
,
"lotSizeFilter"
:
{
"maxOrderQty"
:
"70000"
,
"minOrderQty"
:
"1"
,
"qtyStep"
:
"1"
,
"postOnlyMaxOrderQty"
:
"70000"
,
"maxMktOrderQty"
:
"14000"
,
"minNotionalValue"
:
"5"
}
,
"unifiedMarginTrade"
:
true
,
"fundingInterval"
:
480
,
"settleCoin"
:
"USDT"
,
"copyTrading"
:
"none"
,
"upperFundingRate"
:
"0.05"
,
"lowerFundingRate"
:
"-0.05"
,
"isPreListing"
:
true
,
"preListingInfo"
:
{
"curAuctionPhase"
:
"ContinuousTrading"
,
"phases"
:
[
{
"phase"
:
"CallAuction"
,
"startTime"
:
"1735113600000"
,
"endTime"
:
"1735116600000"
}
,
{
"phase"
:
"CallAuctionNoCancel"
,
"startTime"
:
"1735116600000"
,
"endTime"
:
"1735116900000"
}
,
{
"phase"
:
"CrossMatching"
,
"startTime"
:
"1735116900000"
,
"endTime"
:
"1735117200000"
}
,
{
"phase"
:
"ContinuousTrading"
,
"startTime"
:
"1735117200000"
,
"endTime"
:
""
}
]
,
"auctionFeeInfo"
:
{
"auctionFeeRate"
:
"0"
,
"takerFeeRate"
:
"0.001"
,
"makerFeeRate"
:
"0.0004"
}
}
,
"riskParameters"
:
{
"priceLimitRatioX"
:
"0.05"
,
"priceLimitRatioY"
:
"0.1"
}
,
"symbolType"
:
""
}
]
,
"nextPageCursor"
:
"first%3DBIOUSDT%26last%3DBIOUSDT"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1735810114435
}
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"option"
,
"nextPageCursor"
:
""
,
"list"
:
[
{
"symbol"
:
"BTC-27MAR26-70000-P-USDT"
,
"status"
:
"Trading"
,
"baseCoin"
:
"BTC"
,
"quoteCoin"
:
"USDT"
,
"settleCoin"
:
"USDT"
,
"optionsType"
:
"Put"
,
"launchTime"
:
"1743669649256"
,
"deliveryTime"
:
"1774598400000"
,
"deliveryFeeRate"
:
"0.00015"
,
"priceFilter"
:
{
"minPrice"
:
"5"
,
"maxPrice"
:
"1110000"
,
"tickSize"
:
"5"
}
,
"lotSizeFilter"
:
{
"maxOrderQty"
:
"500"
,
"minOrderQty"
:
"0.01"
,
"qtyStep"
:
"0.01"
}
,
"displayName"
:
"BTCUSDT-27MAR26-70000-P"
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
1672712537130
}
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"category"
:
"spot"
,
"list"
:
[
{
"symbol"
:
"BTCUSDT"
,
"baseCoin"
:
"BTC"
,
"quoteCoin"
:
"USDT"
,
"innovation"
:
"0"
,
"status"
:
"Trading"
,
"marginTrading"
:
"utaOnly"
,
"stTag"
:
"0"
,
"lotSizeFilter"
:
{
"basePrecision"
:
"0.000001"
,
"quotePrecision"
:
"0.0000001"
,
"minOrderQty"
:
"0.000011"
,
"maxOrderQty"
:
"83"
,
"minOrderAmt"
:
"5"
,
"maxOrderAmt"
:
"8000000"
,
"maxLimitOrderQty"
:
"83"
,
"maxMarketOrderQty"
:
"41.5"
,
"postOnlyMaxLimitOrderSize"
:
"60000"
}
,
"priceFilter"
:
{
"tickSize"
:
"0.1"
}
,
"riskParameters"
:
{
"priceLimitRatioX"
:
"0.005"
,
"priceLimitRatioY"
:
"0.01"
}
,
"symbolType"
:
""
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
1760027412300
}

**Examples:**

Example 1 ():
```
GET /v5/market/instruments-info?category=linear&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_instruments_info(    category="linear",    symbol="BTCUSDT",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT"}client.NewUtaBybitServiceWithParams(params).GetInstrumentInfo(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var instrumentInfoRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").instrumentStatus(InstrumentStatus.TRADING).limit(500).build();client.getInstrumentsInfo(instrumentInfoRequest,System.out::println);
```

---

## Get Index Price Components

**URL:** https://bybit-exchange.github.io/docs/v5/market/index-components

**Contents:**
- Get Index Price Components
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Index Price Components
On this page
Get Index Price Components
HTTP Request
​
GET
/v5/market/index-price-components
Request Parameters
​
Parameter
Required
Type
Comments
indexName
true
string
Index name, like
BTCUSDT
Response Parameters
​
Parameter
Type
Comments
indexName
string
Name of the index (e.g., BTCUSDT)
lastPrice
string
Last price of the index
updateTime
string
Timestamp of the last update in milliseconds
components
array
List of components contributing to the index price
> exchange
string
Name of the exchange
> spotPair
string
Spot trading pair on the exchange (e.g., BTCUSDT)
> equivalentPrice
string
Equivalent price
> multiplier
string
Multiplier used for the component price
> price
string
Actual price
> weight
string
Weight in the index calculation
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/index-price-components?indexName=1000BTTUSDT
HTTP/1.1
Host
:
api-testnet.bybit.com
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
"indexName"
:
"1000BTTUSDT"
,
"lastPrice"
:
"0.0006496"
,
"updateTime"
:
"1758182745072"
,
"components"
:
[
{
"exchange"
:
"GateIO"
,
"spotPair"
:
"BTT_USDT"
,
"equivalentPrice"
:
"0.0006485"
,
"multiplier"
:
"1000"
,
"price"
:
"0.0006485"
,
"weight"
:
"0.1383220862762299"
}
,
{
"exchange"
:
"Bybit"
,
"spotPair"
:
"BTTUSDT"
,
"equivalentPrice"
:
"0.0006502"
,
"multiplier"
:
"1000"
,
"price"
:
"0.0006502"
,
"weight"
:
"0.0407528429737999"
}
,
{
"exchange"
:
"Bitget"
,
"spotPair"
:
"BTTUSDT"
,
"equivalentPrice"
:
"0.000648"
,
"multiplier"
:
"1000"
,
"price"
:
"0.000648"
,
"weight"
:
"0.1629044859431618"
}
,
{
"exchange"
:
"BitMart"
,
"spotPair"
:
"BTT_USDT"
,
"equivalentPrice"
:
"0.000649"
,
"multiplier"
:
"1000"
,
"price"
:
"0.000649"
,
"weight"
:
"0.0432327388538453"
}
,
{
"exchange"
:
"Binance"
,
"spotPair"
:
"BTTCUSDT"
,
"equivalentPrice"
:
"0.00065"
,
"multiplier"
:
"1000"
,
"price"
:
"0.00065"
,
"weight"
:
"0.5322401401714303"
}
,
{
"exchange"
:
"Mexc"
,
"spotPair"
:
"BTTUSDT"
,
"equivalentPrice"
:
"0.0006517"
,
"multiplier"
:
"1000"
,
"price"
:
"0.0006517"
,
"weight"
:
"0.0825477057815328"
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
1758182745621
}

**Examples:**

Example 1 ():
```
GET /v5/market/index-price-components?indexName=1000BTTUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```

```

---

## Get Long Short Ratio

**URL:** https://bybit-exchange.github.io/docs/v5/market/long-short-ratio

**Contents:**
- Get Long Short Ratio
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Long Short Ratio
On this page
Get Long Short Ratio
This refers to the net long and short positions as percentages of all position holders during the selected time.
Long account ratio = Number of holders with long positions / Total number of holders
Short account ratio = Number of holders with short positions / Total number of holders
Long-short account ratio = Long account ratio / Short account ratio
info
The earliest query start time is July 20, 2020
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/market/account-ratio
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
Product type.
linear
(USDT Contract),
inverse
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
period
true
string
Data recording period.
5min
,
15min
,
30min
,
1h
,
4h
,
1d
startTime
false
string
The start timestamp (ms)
endTime
false
string
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
500
]
. Default:
50
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> symbol
string
Symbol name
> buyRatio
string
The ratio of the number of long position
> sellRatio
string
The ratio of the number of short position
> timestamp
string
Timestamp (ms)
nextPageCursor
string
Refer to the
cursor
request parameter
RUN >>
Request Example
​
HTTP
Python
GO
Java
Node.js
GET
/v5/market/account-ratio?category=linear&symbol=BTCUSDT&period=1h&limit=2&startTime=1696089600000&endTime=1696262400000
HTTP/1.1
Host
:
api-testnet.bybit.com
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"period"
:
"5min"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetLongShortRatio
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
marketAccountRatioRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
dataRecordingPeriod
(
DataRecordingPeriod
.
FIFTEEN_MINUTES
)
.
limit
(
10
)
.
build
(
)
;
client
.
getMarketAccountRatio
(
marketAccountRatioRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getLongShortRatio
(
{
category
:
'linear'
,
symbol
:
'BTCUSDT'
,
period
:
'1h'
,
limit
:
100
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"list"
:
[
{
"symbol"
:
"BTCUSDT"
,
"buyRatio"
:
"0.49"
,
"sellRatio"
:
"0.51"
,
"timestamp"
:
"1696262400000"
}
,
{
"symbol"
:
"BTCUSDT"
,
"buyRatio"
:
"0.4927"
,
"sellRatio"
:
"0.5073"
,
"timestamp"
:
"1696258800000"
}
]
,
"nextPageCursor"
:
"lastid%3D0%26lasttime%3D1696258800"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1731567491688
}

**Examples:**

Example 1 ():
```
GET /v5/market/account-ratio?category=linear&symbol=BTCUSDT&period=1h&limit=2&startTime=1696089600000&endTime=1696262400000 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```

```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "linear", "symbol": "BTCUSDT", "period": "5min"}client.NewUtaBybitServiceWithParams(params).GetLongShortRatio(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketAccountRatioRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").dataRecordingPeriod(DataRecordingPeriod.FIFTEEN_MINUTES).limit(10).build();client.getMarketAccountRatio(marketAccountRatioRequest, System.out::println);
```

---

## Get Tickers

**URL:** https://bybit-exchange.github.io/docs/v5/spread/market/tickers

**Contents:**
- Get Tickers
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Market
Get Tickers
On this page
Get Tickers
Query for the latest price snapshot, best bid/ask price, and trading volume of different spread combinations in the last 24 hours.
info
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/spread/tickers
Request Parameters
​
Parameter
Required
Type
Comments
symbol
true
string
Spread combination symbol name
Response Parameters
​
Parameter
Type
Comments
list
array
<
object
>
Ticker info
> symbol
string
Spread combination symbol name
> bidPrice
string
Bid 1 price
> bidSize
string
Bid 1 size
> askPrice
string
Ask 1 price
> askSize
string
Ask 1 size
> lastPrice
string
Last trade price
> highPrice24h
string
The highest price in the last 24 hours
> lowPrice24h
string
The lowest price in the last 24 hours
> prevPrice24h
string
Price 24 hours ago
> volume24h
string
Volume for 24h
Request Example
​
GET
/v5/spread/tickers?symbol=SOLUSDT_SOL/USDT
HTTP/1.1
Host
:
api-testnet.bybit.com
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"Success"
,
"result"
:
{
"list"
:
[
{
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"bidPrice"
:
""
,
"bidSize"
:
""
,
"askPrice"
:
""
,
"askSize"
:
""
,
"lastPrice"
:
"19.444"
,
"highPrice24h"
:
"23.8353"
,
"lowPrice24h"
:
"0"
,
"prevPrice24h"
:
"20"
,
"volume24h"
:
"24694.9"
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
1744079413254
}

**Examples:**

Example 1 ():
```
GET /v5/spread/tickers?symbol=SOLUSDT_SOL/USDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "list": [            {                "symbol": "SOLUSDT_SOL/USDT",                "bidPrice": "",                "bidSize": "",                "askPrice": "",                "askSize": "",                "lastPrice": "19.444",                "highPrice24h": "23.8353",                "lowPrice24h": "0",                "prevPrice24h": "20",                "volume24h": "24694.9"            }        ]    },    "retExtInfo": {},    "time": 1744079413254}
```

---

## Get Kline

**URL:** https://bybit-exchange.github.io/docs/v5/market/kline

**Contents:**
- Get Kline
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Market
Get Kline
On this page
Get Kline
Query for historical klines (also known as candles/candlesticks). Charts are returned in groups based on the requested interval.
Covers: Spot / USDT contract / USDC contract  / Inverse contract
HTTP Request
​
GET
/v5/market/kline
Request Parameters
​
Parameter
Required
Type
Comments
category
false
string
Product type.
spot
,
linear
,
inverse
When
category
is not passed, use
linear
by default
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
interval
true
string
Kline interval.
1
,
3
,
5
,
15
,
30
,
60
,
120
,
240
,
360
,
720
,
D
,
W
,
M
start
false
integer
The start timestamp (ms)
end
false
integer
The end timestamp (ms)
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
200
Response Parameters
​
Parameter
Type
Comments
category
string
Product type
symbol
string
Symbol name
list
array
An string array of individual candle
Sort in reverse by
startTime
> list
[0]
: startTime
string
Start time of the candle (ms)
> list
[1]
: openPrice
string
Open price
> list
[2]
: highPrice
string
Highest price
> list
[3]
: lowPrice
string
Lowest price
> list
[4]
: closePrice
string
Close price.
Is the last traded price when the candle is not closed
> list
[5]
: volume
string
Trade volume
USDT or USDC contract: unit is base coin (e.g., BTC)
Inverse contract: unit is quote coin (e.g., USD)
> list
[6]
: turnover
string
Turnover.
USDT or USDC contract: unit is quote coin (e.g., USDT)
Inverse contract: unit is base coin (e.g., BTC)
RUN >>
Request Example
​
HTTP
Python
Go
Java
Node.js
GET
/v5/market/kline?category=inverse&symbol=BTCUSD&interval=60&start=1670601600000&end=1670608800000
HTTP/1.1
Host
:
api-testnet.bybit.com
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
)
print
(
session
.
get_kline
(
category
=
"inverse"
,
symbol
=
"BTCUSD"
,
interval
=
60
,
start
=
1670601600000
,
end
=
1670608800000
,
)
)
import
(
"context"
"fmt"
bybit
"github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
""
,
""
,
bybit
.
WithBaseURL
(
bybit
.
TESTNET
)
)
params
:=
map
[
string
]
interface
{
}
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"interval"
:
"1"
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
GetMarketKline
(
context
.
Background
(
)
)
import
com
.
bybit
.
api
.
client
.
domain
.
CategoryType
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
*
;
import
com
.
bybit
.
api
.
client
.
domain
.
market
.
request
.
MarketDataRequest
;
import
com
.
bybit
.
api
.
client
.
service
.
BybitApiClientFactory
;
var
client
=
BybitApiClientFactory
.
newInstance
(
)
.
newAsyncMarketDataRestClient
(
)
;
var
marketKLineRequest
=
MarketDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
marketInterval
(
MarketInterval
.
WEEKLY
)
.
build
(
)
;
client
.
getMarketLinesData
(
marketKLineRequest
,
System
.
out
::
println
)
;
const
{
RestClientV5
}
=
require
(
'bybit-api'
)
;
const
client
=
new
RestClientV5
(
{
testnet
:
true
,
}
)
;
client
.
getKline
(
{
category
:
'inverse'
,
symbol
:
'BTCUSD'
,
interval
:
'60'
,
start
:
1670601600000
,
end
:
1670608800000
,
}
)
.
then
(
(
response
)
=>
{
console
.
log
(
response
)
;
}
)
.
catch
(
(
error
)
=>
{
console
.
error
(
error
)
;
}
)
;
Response Example
​
{
"retCode"
:
0
,
"retMsg"
:
"OK"
,
"result"
:
{
"symbol"
:
"BTCUSD"
,
"category"
:
"inverse"
,
"list"
:
[
[
"1670608800000"
,
"17071"
,
"17073"
,
"17027"
,
"17055.5"
,
"268611"
,
"15.74462667"
]
,
[
"1670605200000"
,
"17071.5"
,
"17071.5"
,
"17061"
,
"17071"
,
"4177"
,
"0.24469757"
]
,
[
"1670601600000"
,
"17086.5"
,
"17088"
,
"16978"
,
"17071.5"
,
"6356"
,
"0.37288112"
]
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
1672025956592
}

**Examples:**

Example 1 ():
```
GET /v5/market/kline?category=inverse&symbol=BTCUSD&interval=60&start=1670601600000&end=1670608800000 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_kline(    category="inverse",    symbol="BTCUSD",    interval=60,    start=1670601600000,    end=1670608800000,))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("", "", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "spot", "symbol": "BTCUSDT", "interval": "1"}client.NewUtaBybitServiceWithParams(params).GetMarketKline(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.domain.CategoryType;import com.bybit.api.client.domain.market.*;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var marketKLineRequest = MarketDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").marketInterval(MarketInterval.WEEKLY).build();client.getMarketLinesData(marketKLineRequest, System.out::println);
```

---
