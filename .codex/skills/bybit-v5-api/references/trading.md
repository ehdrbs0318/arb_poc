# Bybit-V5-Api - Trading

**Pages:** 80

---

## Execute Quote

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/execute-quote

**Contents:**
- Execute Quote
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Execute Quote
On this page
Execute Quote
Execute quote – only for the creator of the RFQ.
Up to 50 requests
per second.
info
This endpoint is asynchronous. You must check the
Get Trade History
endpoint or listen to the
Execution
WebSocket topic to confirm if the execution was successful.
HTTP Request
​
POST
/v5/rfq/execute-quote
Request Parameters
​
Parameter
Required
Type
Comments
rfqId
true
string
Inquiry ID
quoteId
true
string
Quote ID
quoteSide
true
string
The direction of the quote is
Buy
or
Sell
. When the direction of the quote is
Buy
, for the maker, the execution direction is the same as the direction in legs, and for the taker, it is opposite. Conversely, the same applies
Response Parameters
​
Parameter
Type
Comments
result
object
> rfqId
string
Inquiry ID
>rfqLinkId
string
> quoteId
string
Quote ID
> status
string
Order status:
PendingFill
: Order has been sent to the matching engine but not yet filled.
Failed
: Order failed
Request Example
​
POST
/v5/rfq/execute-quote
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"rfqId"
:
"1754364447601610516653123084412812"
,
"quoteId"
:
"111"
,
"quoteSide"
:
"Buy"
}
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
"rfqId"
:
"175740700350925204128457980089654"
,
"rfqLinkId"
:
""
,
"quoteId"
:
"1757407015586174663206671159484665"
,
"status"
:
"PendingFill"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1757407058177
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/execute-quote HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115 {  "rfqId":"1754364447601610516653123084412812",  "quoteId": "111",  "quoteSide":"Buy"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "rfqId": "175740700350925204128457980089654",        "rfqLinkId": "",        "quoteId": "1757407015586174663206671159484665",        "status": "PendingFill"    },    "retExtInfo": {},    "time": 1757407058177}
```

---

## Cancel RFQ

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/cancel-rfq

**Contents:**
- Cancel RFQ
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Cancel RFQ
On this page
Cancel RFQ
Cancel RFQ.
Up to 50 requests per second
info
You must pass either rfqId or rfqLinkId.
If both rfqId and rfqLinkId are passed, only rfqId is considered.
HTTP Request
​
POST
/v5/rfq/cancel-rfq
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
Custom inquiry ID
Response Parameters
​
Parameter
Type
Comments
rfqId
string
Inquiry ID
rfqLinkId
string
Custom inquiry ID
Request Example
​
POST
/v5/rfq/cancel-rfq
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"rfqId"
:
"1756871488168105512459181956436945"
}
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
"rfqId"
:
"1756871488168105512459181956436945"
,
"rfqLinkId"
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
1756871494507
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/cancel-rfq HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115{    "rfqId": "1756871488168105512459181956436945"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "rfqId": "1756871488168105512459181956436945",        "rfqLinkId": ""    },    "retExtInfo": {},    "time": 1756871494507}
```

---

## Get Purchase/Redemption Records

**URL:** https://bybit-exchange.github.io/docs/v5/lt/order-record

**Contents:**
- Get Purchase/Redemption Records
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

On this page
Get Purchase/Redemption Records
Get purchase or redeem history
HTTP Request
​
GET
/v5/spot-lever-token/order-record
Request Parameters
​
Parameter
Required
Type
Comments
ltCoin
false
string
Abbreviation of the LT, such as BTC3L
orderId
false
string
Order ID
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
500
]
. Default:
100
ltOrderType
false
integer
LT order type.
1
: purchase,
2
: redemption
serialNo
false
string
Serial number
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> ltCoin
string
Abbreviation of the LT, such as BTC3L
> orderId
string
Order ID
> ltOrderType
integer
LT order type.
1
: purchase,
2
: redeem
> orderTime
number
Order time
> updateTime
number
Last update time of the order status
> ltOrderStatus
string
Order status.
1
: completed,
2
: in progress,
3
: failed
> fee
string
Trading fees
> amount
string
Order quantity of the LT
> value
string
Filled value
> valueCoin
string
Quote coin
> serialNo
string
Serial number
RUN >>
Request Example
​
HTTP
Python
GET
/v5/spot-lever-token/order-record?orderId=2611
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672294422027
X-BAPI-SIGN
:
XXXXX
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
get_purchase_redemption_records
(
orderId
=
2611
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
"success"
,
"result"
:
{
"list"
:
[
{
"amount"
:
"222.90757477"
,
"fee"
:
"0"
,
"ltCoin"
:
"EOS3L"
,
"ltOrderStatus"
:
"1"
,
"ltOrderType"
:
"1"
,
"orderId"
:
"2611"
,
"orderTime"
:
"1672737465000"
,
"serialNo"
:
"pruchase-002"
,
"updateTime"
:
"1672737478000"
,
"value"
:
"95.13860435"
,
"valueCoin"
:
"USDT"
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
1672294446137
}

**Examples:**

Example 1 ():
```
GET /v5/spot-lever-token/order-record?orderId=2611 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672294422027X-BAPI-SIGN: XXXXXX-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_purchase_redemption_records(    orderId=2611))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "amount": "222.90757477",                "fee": "0",                "ltCoin": "EOS3L",                "ltOrderStatus": "1",                "ltOrderType": "1",                "orderId": "2611",                "orderTime": "1672737465000",                "serialNo": "pruchase-002",                "updateTime": "1672737478000",                "value": "95.13860435",                "valueCoin": "USDT"            }        ]    },    "retExtInfo": {},    "time": 1672294446137}
```

---

## Cancel All RFQs

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/cancel-all-rfq

**Contents:**
- Cancel All RFQs
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Cancel All RFQs
On this page
Cancel All RFQs
Cancel all active RFQs.
Up to 50 requests per second
info
Inquirer cancels order: Cancel the inquiry, all its corresponding quotes becoming invalid
Quoter cancels the order: The inquiry is not affected, but the quote becomes invalid
HTTP Request
​
POST
/v5/rfq/cancel-all-rfq
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
result
array of objects
> rfqId
string
Inquiry ID
> rfqLinkId
string
Custom inquiry ID
> code
string
Whether or not the cancellations were a success,
0
: success
> msg
string
Cancellation failure reason
Request Example
​
POST
/v5/rfq/cancel-all-rfq
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
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
[
{
"rfqId"
:
"175766967076315412093641573648082"
,
"rfqLinkId"
:
""
,
"code"
:
0
,
"msg"
:
""
}
]
,
"retExtInfo"
:
{
}
,
"time"
:
1757669676581
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/cancel-all-rfq HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": [        {            "rfqId": "175766967076315412093641573648082",            "rfqLinkId": "",            "code": 0,            "msg": ""        }    ],    "retExtInfo": {},    "time": 1757669676581}
```

---

## Cancel All Quotes

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/cancel-all-quotes

**Contents:**
- Cancel All Quotes
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Cancel All Quotes
On this page
Cancel All Quotes
Cancel all active quotes.
Up to 50 requests per second
HTTP Request
​
POST
/v5/rfq/cancel-all-quotes
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
result
Object
> rfqId
string
Inquiry ID
> quoteId
string
Quote ID
> quoteLinkId
string
Custom quote ID
> code
string
Whether or not cancellation was a success,
0
: success
> msg
string
Cancellation failure reason
Request Example
​
POST
/v5/rfq/cancel-all-quotes
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
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
[
{
"rfqId"
:
"175740723913299909861293671607573"
,
"quoteLinkId"
:
""
,
"quoteId"
:
"1757407497684679708210572531298710"
,
"code"
:
0
,
"msg"
:
""
}
]
,
"retExtInfo"
:
{
}
,
"time"
:
1757407503982
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/cancel-all-quotes HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": [        {            "rfqId": "175740723913299909861293671607573",            "quoteLinkId": "",            "quoteId": "1757407497684679708210572531298710",            "code": 0,            "msg": ""        }    ],    "retExtInfo": {},    "time": 1757407503982}
```

---

## Order

**URL:** https://bybit-exchange.github.io/docs/v5/spread/websocket/private/order

**Contents:**
- Order
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

Spread Trading
Websocket Stream
Private
Order
On this page
Order
Subscribe to the order stream to see changes to your orders in
real-time
.
Topic:
spread.order
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
> category
string
Category name,
combination
,
spot_leg
,
future_leg
> symbol
string
Combo or leg's symbol name
> parentOrderId
string
Leg's parent order ID
> orderId
string
Combo or leg's order ID
> orderLinkId
string
Combo's user customised order ID
> side
string
Combo or leg's order side,
Buy
,
Sell
> orderStatus
string
Combo or leg's order status
>
cancelType
string
Cancel type
>
rejectReason
string
Reject reason
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
> price
string
Order price
> qty
string
Order qty
> avgPrice
string
Average filled price
> leavesQty
string
The remaining qty not executed
> leavesValue
string
The estimated value not executed
> cumExecQty
string
Cumulative executed order qty
> cumExecValue
string
Cumulative executed order value
> cumExecFee
string
Deprecated. Cumulative executed trading fee
> orderType
string
Order type.
Market
,
Limit
> isLeverage
string
Account-wide, if Spot Margin is enabled, the spot_leg field in the execution message shows 1, combo is "", and future_leg is 0.
> createdTime
string
Order created timestamp (ms)
> updatedTime
string
Order updated timestamp (ms)
> feeCurrency
string
Deprecated. Trading fee currency for Spot leg only
> createType
string
Order create type
> closedPnl
string
Closed profit and loss for each close position order
> cumFeeDetail
json
Cumulative trading fee details instead of
cumExecFee
and
feeCurrency
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
"spread.order"
]
}
Stream Example
​
{
"topic"
:
"spread.order"
,
"id"
:
"1448939_SOLUSDT_28732003549"
,
"creationTime"
:
1744170555912
,
"data"
:
[
{
"category"
:
"combination"
,
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"parentOrderId"
:
""
,
"orderId"
:
"aa858ea9-f3a0-40b6-ad57-888d47307345"
,
"orderLinkId"
:
""
,
"side"
:
"Buy"
,
"orderStatus"
:
"Filled"
,
"cancelType"
:
"UNKNOWN"
,
"rejectReason"
:
"EC_NoError"
,
"timeInForce"
:
"GTC"
,
"price"
:
"14"
,
"qty"
:
"2"
,
"avgPrice"
:
""
,
"leavesQty"
:
"0"
,
"leavesValue"
:
""
,
"cumExecQty"
:
"2"
,
"cumExecValue"
:
""
,
"cumExecFee"
:
""
,
"orderType"
:
"Limit"
,
"isLeverage"
:
""
,
"createdTime"
:
"1744170534447"
,
"updatedTime"
:
"1744170555905"
,
"feeCurrency"
:
""
,
"createType"
:
"CreateByUser"
,
"closedPnl"
:
""
,
"cumFeeDetail"
:
{
"MNT"
:
"0.00242968"
}
}
,
{
"category"
:
"future_leg"
,
"symbol"
:
"SOLUSDT"
,
"parentOrderId"
:
"aa858ea9-f3a0-40b6-ad57-888d47307345"
,
"orderId"
:
"2948d2dc-f8f1-4485-a83d-0bad3dae2c31"
,
"orderLinkId"
:
""
,
"side"
:
"Buy"
,
"orderStatus"
:
"Filled"
,
"cancelType"
:
"UNKNOWN"
,
"rejectReason"
:
"EC_NoError"
,
"timeInForce"
:
"GTC"
,
"price"
:
"118.2"
,
"qty"
:
"2"
,
"avgPrice"
:
"118.2"
,
"leavesQty"
:
"0"
,
"leavesValue"
:
"0"
,
"cumExecQty"
:
"2"
,
"cumExecValue"
:
"236.4"
,
"cumExecFee"
:
"0.01182"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
""
,
"createdTime"
:
"1744170534447"
,
"updatedTime"
:
"1744170555910"
,
"feeCurrency"
:
""
,
"createType"
:
"CreateByFutureSpread"
,
"closedPnl"
:
"0"
,
"cumFeeDetail"
:
{
"MNT"
:
"0.00242968"
}
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "spread.order"    ]}
```

Example 2 ():
```
{    "topic": "spread.order",    "id": "1448939_SOLUSDT_28732003549",    "creationTime": 1744170555912,    "data": [        {            "category": "combination",            "symbol": "SOLUSDT_SOL/USDT",            "parentOrderId": "",            "orderId": "aa858ea9-f3a0-40b6-ad57-888d47307345",            "orderLinkId": "",            "side": "Buy",            "orderStatus": "Filled",            "cancelType": "UNKNOWN",            "rejectReason": "EC_NoError",            "timeInForce": "GTC",            "price": "14",            "qty": "2",            "avgPrice": "",            "leavesQty": "0",            "leavesValue": "",            "cumExecQty": "2",            "cumExecValue": "",            "cumExecFee": "",            "orderType": "Limit",            "isLeverage": "",            "createdTime": "1744170534447",            "updatedTime": "1744170555905",            "feeCurrency": "",            "createType": "CreateByUser",            "closedPnl": "",            "cumFeeDetail": {                "MNT": "0.00242968"            }        },        {            "category": "future_leg",            "symbol": "SOLUSDT",            "parentOrderId": "aa858ea9-f3a0-40b6-ad57-888d47307345",            "orderId": "2948d2dc-f8f1-4485-a83d-0bad3dae2c31",            "orderLinkId": "",            "side": "Buy",            "orderStatus": "Filled",            "cancelType": "UNKNOWN",            "rejectReason": "EC_NoError",            "timeInForce": "GTC",            "price": "118.2",            "qty": "2",            "avgPrice": "118.2",            "leavesQty": "0",            "leavesValue": "0",            "cumExecQty": "2",            "cumExecValue": "236.4",            "cumExecFee": "0.01182",            "orderType": "Limit",            "isLeverage": "",            "createdTime": "1744170534447",            "updatedTime": "1744170555910",            "feeCurrency": "",            "createType": "CreateByFutureSpread",            "closedPnl": "0",            "cumFeeDetail": {                "MNT": "0.00242968"            }        }    ]}
```

---

## Get Trade History

**URL:** https://bybit-exchange.github.io/docs/v5/order/execution

**Contents:**
- Get Trade History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Get Trade History (2 years)
On this page
Get Trade History
Query users' execution records, sorted by
execTime
in descending order.
tip
Response items will have sorting issues when 'execTime' is the same, it is recommended to sort according to
execId+OrderId+leavesQty
.
If you want to receive real-time execution information, Use the
websocket stream
(recommended).
You may have multiple executions in a single order.
You can query by symbol, baseCoin, orderId and orderLinkId, and if you pass multiple params, the system will process them according to this priority: orderId > orderLinkId > symbol > baseCoin. orderId and orderLinkId have a higher priority and as long as these two parameters are in the input parameters, other input parameters will be ignored.
HTTP Request
​
GET
/v5/execution/list
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
spot
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
orderId
false
string
Order ID
orderLinkId
false
string
User customised order ID
baseCoin
false
string
Base coin, uppercase only
settleCoin
false
string
Settle coin, uppercase only. Only for
linear
,
inverse
,
option
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 7 days by default;
Only startTime is passed, return range between startTime and startTime+7 days
Only endTime is passed, return range between endTime-7 days and endTime
If both are passed, the rule is endTime - startTime <= 7 days
endTime
false
integer
The end timestamp (ms)
execType
false
string
Execution type
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
> orderId
string
Order ID
> orderLinkId
string
User customized order ID
> side
string
Side.
Buy
,
Sell
> orderPrice
string
Order price
> orderQty
string
Order qty
> leavesQty
string
The remaining qty not executed
>
createType
string
Order create type
Spot, Option do not have this key
>
orderType
string
Order type.
Market
,
Limit
>
stopOrderType
string
Stop order type. If the order is not stop order, it either returns
UNKNOWN
or
""
> execFee
string
Executed trading fee. You can get spot fee currency instruction
here
> execFeeV2
string
Spot leg transaction fee, only works for execType=
FutureSpread
> execId
string
Execution ID
> execPrice
string
Execution price
> execQty
string
Execution qty
>
execType
string
Executed type
> execValue
string
Executed order value
> execTime
string
Executed timestamp (ms)
> feeCurrency
string
Trading fee currency
> isMaker
boolean
Is maker order.
true
: maker,
false
: taker
> feeRate
string
Trading fee rate
> tradeIv
string
Implied volatility.
Valid for
option
> markIv
string
Implied volatility of mark price.
Valid for
option
> markPrice
string
The mark price of the symbol when executing
> indexPrice
string
The index price of the symbol when executing.
Valid for
option
only
> underlyingPrice
string
The underlying price of the symbol when executing.
Valid for
option
> blockTradeId
string
Paradigm block trade ID
> closedSize
string
Closed position size
> seq
long
Cross sequence, used to associate each fill and each position update
The seq will be the same when conclude multiple transactions at the same time
Different symbols may have the same seq, please use seq + symbol to check unique
> extraFees
string
Trading fee rate information. Currently, this data is returned only for kyc=Indian user or spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum:
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
Java
Node.js
GET
/v5/execution/list?category=linear&limit=1
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
1672283754132
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
get_executions
(
category
=
"linear"
,
limit
=
1
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
tradeHistoryRequest
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
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
execType
(
ExecType
.
Trade
)
.
limit
(
100
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
getTradeHistory
(
tradeHistoryRequest
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
getExecutionList
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
margin
:
'10'
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
"132766%3A2%2C132766%3A2"
,
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
"orderType"
:
"Market"
,
"underlyingPrice"
:
""
,
"orderLinkId"
:
""
,
"side"
:
"Buy"
,
"indexPrice"
:
""
,
"orderId"
:
"8c065341-7b52-4ca9-ac2c-37e31ac55c94"
,
"stopOrderType"
:
"UNKNOWN"
,
"leavesQty"
:
"0"
,
"execTime"
:
"1672282722429"
,
"feeCurrency"
:
""
,
"isMaker"
:
false
,
"execFee"
:
"0.071409"
,
"feeRate"
:
"0.0006"
,
"execId"
:
"e0cbe81d-0f18-5866-9415-cf319b5dab3b"
,
"tradeIv"
:
""
,
"blockTradeId"
:
""
,
"markPrice"
:
"1183.54"
,
"execPrice"
:
"1190.15"
,
"markIv"
:
""
,
"orderQty"
:
"0.1"
,
"orderPrice"
:
"1236.9"
,
"execValue"
:
"119.015"
,
"execType"
:
"Trade"
,
"execQty"
:
"0.1"
,
"closedSize"
:
""
,
"extraFees"
:
""
,
"seq"
:
4688002127
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
1672283754510
}

**Examples:**

Example 1 ():
```
GET /v5/execution/list?category=linear&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672283754132X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_executions(    category="linear",    limit=1,))
```

Example 3 ():
```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var tradeHistoryRequest = TradeOrderRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").execType(ExecType.Trade).limit(100).build();System.out.println(client.getTradeHistory(tradeHistoryRequest));
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getExecutionList({        category: 'linear',        symbol: 'BTCUSDT',        margin: '10',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Trade

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/trade

**Contents:**
- Trade
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
Trade
On this page
Trade
Subscribe to the recent trades stream.
After subscription, you will be pushed trade messages in real-time.
Push frequency:
real-time
Topic:
publicTrade.{symbol}
Note
: option uses baseCoin, e.g., publicTrade.BTC
note
For Futures and Spot, a single message may have up to 1024 trades. As such, multiple messages may be sent for the same
seq
.
Response Parameters
​
Parameter
Type
Comments
id
string
Message id.
Unique field for option
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
Object. Sorted by the time the trade was matched in ascending order
> T
number
The timestamp (ms) that the order is filled
> s
string
Symbol name
> S
string
Side of taker.
Buy
,
Sell
> v
string
Trade size
> p
string
Trade price
>
L
string
Direction of price change.
Unique field for Perps & futures
> i
string
Trade ID
> BT
boolean
Whether it is a block trade order or not
> RPI
boolean
Whether it is a RPI trade or not
> seq
integer
cross sequence
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
trade_stream
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
{
"topic"
:
"publicTrade.BTCUSDT"
,
"type"
:
"snapshot"
,
"ts"
:
1672304486868
,
"data"
:
[
{
"T"
:
1672304486865
,
"s"
:
"BTCUSDT"
,
"S"
:
"Buy"
,
"v"
:
"0.001"
,
"p"
:
"16578.50"
,
"L"
:
"PlusTick"
,
"i"
:
"20f43950-d8dd-5b31-9112-a178eb6023af"
,
"BT"
:
false
,
"seq"
:
1783284617
}
]
}

**Examples:**

Example 1 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.trade_stream(    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 2 ():
```
{    "topic": "publicTrade.BTCUSDT",    "type": "snapshot",    "ts": 1672304486868,    "data": [        {            "T": 1672304486865,            "s": "BTCUSDT",            "S": "Buy",            "v": "0.001",            "p": "16578.50",            "L": "PlusTick",            "i": "20f43950-d8dd-5b31-9112-a178eb6023af",            "BT": false,            "seq": 1783284617        }    ]}
```

---

## Get Unpaid Loans

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/unpaid-loan-order

**Contents:**
- Get Unpaid Loans
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Get Unpaid Loans
On this page
Get Unpaid Loans
Query for your ongoing loans.
Permission: "Spot trade"
HTTP Request
​
GET
/v5/crypto-loan/ongoing-orders
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
loanCurrency
false
string
Loan coin name
collateralCurrency
false
string
Collateral coin name
loanTermType
false
string
1
: fixed term, when query this type,
loanTerm
must be filled
2
: flexible term
By default, query all types
loanTerm
false
string
7
,
14
,
30
,
90
,
180
days, working when
loanTermType
=1
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> collateralAmount
string
Collateral amount
> collateralCurrency
string
Collateral coin
> currentLTV
string
Current LTV
> expirationTime
string
Loan maturity time, keeps
""
for flexible loan
> hourlyInterestRate
string
Hourly interest rate
Flexible loan, it is real-time interest rate
Fixed term loan: it is fixed term interest rate
> loanCurrency
string
Loan coin
> loanTerm
string
Loan term,
7
,
14
,
30
,
90
,
180
days, keep
""
for flexible loan
> orderId
string
Loan order ID
> residualInterest
string
Unpaid interest
> residualPenaltyInterest
string
Unpaid penalty interest
> totalDebt
string
Unpaid principal
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/ongoing-orders?orderId=1793683005081680384
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
1728630979731
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
repay_crypto_loan
(
orderId
=
"1794267532472646144"
,
amount
=
"100"
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
getUnpaidLoanOrders
(
{
orderId
:
'1793683005081680384'
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
"request.success"
,
"result"
:
{
"list"
:
[
{
"collateralAmount"
:
"0.0964687"
,
"collateralCurrency"
:
"BTC"
,
"currentLTV"
:
"0.4161"
,
"expirationTime"
:
"1731149999000"
,
"hourlyInterestRate"
:
"0.0000010633"
,
"loanCurrency"
:
"USDT"
,
"loanTerm"
:
"30"
,
"orderId"
:
"1793683005081680384"
,
"residualInterest"
:
"0.04016"
,
"residualPenaltyInterest"
:
"0"
,
"totalDebt"
:
"1888.005198"
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
1728630980861
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan/ongoing-orders?orderId=1793683005081680384 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728630979731X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.repay_crypto_loan(        orderId="1794267532472646144",        amount="100",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getUnpaidLoanOrders({ orderId: '1793683005081680384' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "collateralAmount": "0.0964687",                "collateralCurrency": "BTC",                "currentLTV": "0.4161",                "expirationTime": "1731149999000",                "hourlyInterestRate": "0.0000010633",                "loanCurrency": "USDT",                "loanTerm": "30",                "orderId": "1793683005081680384",                "residualInterest": "0.04016",                "residualPenaltyInterest": "0",                "totalDebt": "1888.005198"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1728630980861}
```

---

## SBE Public Trade Integration

**URL:** https://bybit-exchange.github.io/docs/v5/sbe/sbe-public-trade

**Contents:**
- SBE Public Trade Integration
- Overview​
- Flow​
  - Ping / Pong (JSON control frames)​
  - Subscribe​
- SBE XML Template (Public Trade)​
- Field Reference​
  - Each tradeItems[i]entry​
    - SideType​
    - BoolEnum​

SBE
Public Trade
SBE Public Trade Integration
On this page
SBE Public Trade Integration
Overview
​
Channel:
private MMWS only (not available on public WS).
WSURL:
wss://<your-public-stream-host>.bybit-aws.com/v5/public-sbe/<category>
.
Topic:
publicTrade.SBE.<symbol>
.
Format:
SBE binary frames (
opcode = 2
), little-endian.
Push frequency
: real-time
Messages are delivered in-order per symbol group. A single packet may contain 1–1024 trades
Flow
​
Ping / Pong (JSON control frames)
​
Send Ping
{
"req_id"
:
"100001"
,
"op"
:
"ping"
}
Receive Pong
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
"xxxxx-xx"
,
"req_id"
:
""
,
"op"
:
"ping"
}
Subscribe
​
Topic format:
publicTrade.SBE.<symbol>
Subscribe request
{
"op"
:
"subscribe"
,
"args"
:
[
"publicTrade.SBE.BTCUSDT"
]
}
Subscription confirmation
{
"id"
:
"trade-001"
,
"topic"
:
"publicTrade.SBE.BTCUSDT"
,
"type"
:
"snapshot"
,
"ts"
:
1760000000000
,
"data"
:
[
...
]
}
SBE XML Template (Public Trade)
​
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
<
enum
name
=
"
SideType
"
encodingType
=
"
uint8
"
>
<
validValue
name
=
"
UNKNOWN
"
>
0
</
validValue
>
<
validValue
name
=
"
BUY
"
>
1
</
validValue
>
<
validValue
name
=
"
SELL
"
>
2
</
validValue
>
<
validValue
name
=
"
NON_REPRESENTABLE
"
>
254
</
validValue
>
</
enum
>
<
enum
name
=
"
BoolEnum
"
encodingType
=
"
uint8
"
>
<
validValue
name
=
"
FALSE
"
>
0
</
validValue
>
<
validValue
name
=
"
TRUE
"
>
1
</
validValue
>
<
validValue
name
=
"
NON_REPRESENTABLE
"
>
254
</
validValue
>
</
enum
>
</
types
>
<!-- Stream event for "publicTrade.sbe.<symbol>" channel -->
<
sbe:
message
name
=
"
PublicTradeEvent
"
id
=
"
20002
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
3
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
group
id
=
"
40
"
name
=
"
tradeItems
"
dimensionType
=
"
groupSize16Encoding
"
description
=
"
trade items
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
fillTime
"
type
=
"
int64
"
description
=
"
The timestamp in microseconds that the order is filled
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
price
"
type
=
"
int64
"
description
=
"
Price mantissa
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
size
"
type
=
"
int64
"
description
=
"
Size mantissa
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
5
"
name
=
"
side
"
type
=
"
SideType
"
description
=
"
Side of taker
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
isBlockTrade
"
type
=
"
BoolEnum
"
description
=
"
Whether it is a block trade order or not
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
isRPI
"
type
=
"
BoolEnum
"
description
=
"
Whether it is a RPI trade or not
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
execId
"
type
=
"
int64
"
description
=
"
Trade ID
"
/>
<
data
id
=
"
100
"
name
=
"
execIdString
"
type
=
"
varString8
"
/>
</
group
>
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
Field Reference
​
Message:
PublicTradeEvent
(id = 20002)
Field Name
ID
SBE Type
Unit / Format
Notes
ts
1
int64
µs
System generation time at push side (dispatcher).
priceExponent
2
int8
exponent
Decimal places for price. Display price = priceMantissa × 10^
priceExponent
.
sizeExponent
3
int8
exponent
Decimal places for size. Display size = sizeMantissa × 10^
sizeExponent
.
tradeItems
40
group(
groupSize16Encoding
)
-
Repeating trade items
symbol
55
varString8
UTF-8
1-byte length + bytes, e.g.,
0x07 "BTCUSDT"
.
Each tradeItems
[i]
entry
​
Field (id)
Type
Description
fillTime (1)
int64
Trade fill timestamp(µs)
price  (2)
int64
Apply priceExponent. Display ask size  =
size × 10^sizeExponent
.
size  (3)
int64
Apply sizeExponent. Display ask size  =
size × 10^sizeExponent
.
seq  (4)
int64
Cross sequence id
side  (5)
SideType
(uint8)
Side of taker
isBlockTrade  (6)
BoolEnum
(uint8)
IsBlockTrade(0 = not blockTrade, 1 = blockTrade)
isRPI  (7)
BoolEnum
(uint8)
IsRPI (0 = not RPI, 1 = RPI)
execId  (8)
int64
Trade ID
SideType
​
0
: UNKOWN
1
: BUY
2
: SELL
254
: NON_REPRESENTABLE
BoolEnum
​
0
: FALSE
1
: TRUE
254
: NON_REPRESENTABLE
Integration Script
​
Python
​
import
json
import
struct
import
websocket
from
typing
import
Tuple
WS_URL
=
"wss://stream-testnet.bybits.org/v5/public-sbe/spot"
SYMBOL
=
"BTCUSDT"
TOPIC
=
f"publicTrade.sbe.
{
SYMBOL
}
"
# ---------------- SBE helpers ----------------
def
apply_exp
(
mantissa
:
int
,
exp
:
int
)
-
>
float
:
# display = mantissa * 10^exp
# exp can be negative
return
mantissa
*
(
10.0
**
exp
)
def
read_varstring8
(
buf
:
bytes
,
off
:
int
)
-
>
Tuple
[
str
,
int
]
:
if
off
+
1
>
len
(
buf
)
:
raise
ValueError
(
"varString8: missing length"
)
ln
=
buf
[
off
]
off
+=
1
if
off
+
ln
>
len
(
buf
)
:
raise
ValueError
(
"varString8: out of range"
)
s
=
buf
[
off
:
off
+
ln
]
.
decode
(
"utf-8"
,
errors
=
"replace"
)
off
+=
ln
return
s
,
off
def
parse_public_trade_event
(
buf
:
bytes
)
-
>
dict
:
# messageHeader: <HHHH
if
len
(
buf
)
<
8
:
raise
ValueError
(
"too short for header"
)
block_len
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
"<HHHH"
,
buf
,
0
)
off
=
8
if
template_id
!=
20002
:
raise
ValueError
(
f"unexpected templateId=
{
template_id
}
"
)
# fixed fields: ts(int64), priceExp(int8), sizeExp(int8)
if
len
(
buf
)
<
off
+
8
+
1
+
1
:
raise
ValueError
(
"too short for fixed fields"
)
ts
=
struct
.
unpack_from
(
"<q"
,
buf
,
off
)
[
0
]
;
off
+=
8
price_exp
=
struct
.
unpack_from
(
"<b"
,
buf
,
off
)
[
0
]
;
off
+=
1
size_exp
=
struct
.
unpack_from
(
"<b"
,
buf
,
off
)
[
0
]
;
off
+=
1
# group header: blockLength(uint16), numInGroup(uint16)
if
len
(
buf
)
<
off
+
4
:
raise
ValueError
(
"too short for group header"
)
grp_block_len
,
num_in_group
=
struct
.
unpack_from
(
"<HH"
,
buf
,
off
)
off
+=
4
trades
=
[
]
for
_
in
range
(
num_in_group
)
:
entry_start
=
off
# Parse fields in-order (don’t assume padding; only skip remaining bytes up to grp_block_len)
fill_time
=
struct
.
unpack_from
(
"<q"
,
buf
,
off
)
[
0
]
;
off
+=
8
price_m
=
struct
.
unpack_from
(
"<q"
,
buf
,
off
)
[
0
]
;
off
+=
8
size_m
=
struct
.
unpack_from
(
"<q"
,
buf
,
off
)
[
0
]
;
off
+=
8
seq
=
struct
.
unpack_from
(
"<q"
,
buf
,
off
)
[
0
]
;
off
+=
8
side
=
struct
.
unpack_from
(
"<B"
,
buf
,
off
)
[
0
]
;
off
+=
1
is_block
=
struct
.
unpack_from
(
"<B"
,
buf
,
off
)
[
0
]
;
off
+=
1
is_rpi
=
struct
.
unpack_from
(
"<B"
,
buf
,
off
)
[
0
]
;
off
+=
1
exec_id
=
struct
.
unpack_from
(
"<q"
,
buf
,
off
)
[
0
]
;
off
+=
8
# Skip any future extension bytes in fixed part
fixed_consumed
=
off
-
entry_start
if
fixed_consumed
<
grp_block_len
:
off
+=
(
grp_block_len
-
fixed_consumed
)
elif
fixed_consumed
>
grp_block_len
:
# schema mismatch vs blockLength
raise
ValueError
(
f"group blockLength too small:
{
grp_block_len
}
<
{
fixed_consumed
}
"
)
exec_id_str
,
off
=
read_varstring8
(
buf
,
off
)
trades
.
append
(
{
"fillTime"
:
fill_time
,
"priceMantissa"
:
price_m
,
"sizeMantissa"
:
size_m
,
"price"
:
apply_exp
(
price_m
,
price_exp
)
,
"size"
:
apply_exp
(
size_m
,
size_exp
)
,
"seq"
:
seq
,
"side"
:
side
,
"isBlockTrade"
:
bool
(
is_block
)
,
"isRPI"
:
bool
(
is_rpi
)
,
"execId"
:
exec_id
,
"execIdString"
:
exec_id_str
,
}
)
symbol
,
off
=
read_varstring8
(
buf
,
off
)
return
{
"header"
:
{
"blockLength"
:
block_len
,
"templateId"
:
template_id
,
"schemaId"
:
schema_id
,
"version"
:
version
,
}
,
"ts"
:
ts
,
"priceExponent"
:
price_exp
,
"sizeExponent"
:
size_exp
,
"symbol"
:
symbol
,
"tradeItems"
:
trades
,
"parsed_length"
:
off
,
}
# ---------------- WS handlers ----------------
def
on_open
(
ws
)
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
"subscribed:"
,
TOPIC
)
def
on_message
(
ws
,
message
)
:
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
evt
=
parse_public_trade_event
(
message
)
# print first trade only (example)
if
evt
[
"tradeItems"
]
:
t0
=
evt
[
"tradeItems"
]
[
0
]
print
(
evt
[
"symbol"
]
,
"trades="
,
len
(
evt
[
"tradeItems"
]
)
,
"first:"
,
t0
[
"price"
]
,
"@"
,
t0
[
"size"
]
,
"seq="
,
t0
[
"seq"
]
)
else
:
print
(
"TEXT:"
,
message
)
def
on_error
(
ws
,
err
)
:
print
(
"WS error:"
,
err
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
"closed"
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
Golang
​
package
main
import
(
"encoding/binary"
"encoding/json"
"fmt"
"log"
"math"
"time"
"github.com/gorilla/websocket"
)
const
(
WSURL
=
"wss://stream-testnet.bybits.org/v5/public-sbe/spot"
Symbol
=
"BTCUSDT"
Topic
=
"publicTrade.sbe."
+
Symbol
)
func
applyExp
(
mantissa
int64
,
exp
int8
)
float64
{
return
float64
(
mantissa
)
*
math
.
Pow10
(
int
(
exp
)
)
}
func
readVarString8
(
buf
[
]
byte
,
off
int
)
(
string
,
int
,
error
)
{
if
off
+
1
>
len
(
buf
)
{
return
""
,
off
,
fmt
.
Errorf
(
"varString8: missing length"
)
}
ln
:=
int
(
buf
[
off
]
)
off
++
if
off
+
ln
>
len
(
buf
)
{
return
""
,
off
,
fmt
.
Errorf
(
"varString8: out of range"
)
}
s
:=
string
(
buf
[
off
:
off
+
ln
]
)
off
+=
ln
return
s
,
off
,
nil
}
type
TradeItem
struct
{
FillTimeint64
`json:"fillTime"`
PriceMant
int64
`json:"priceMantissa"`
SizeMantint64
`json:"sizeMantissa"`
Price
float64
`json:"price"`
Size
float64
`json:"size"`
Seqint64
`json:"seq"`
Side
uint8
`json:"side"`
IsBlockTrade
bool
`json:"isBlockTrade"`
IsRPI
bool
`json:"isRPI"`
ExecID
int64
`json:"execId"`
ExecIDString
string
`json:"execIdString"`
}
type
PublicTradeEvent
struct
{
Header
struct
{
BlockLength
uint16
`json:"blockLength"`
TemplateID
uint16
`json:"templateId"`
SchemaID
uint16
`json:"schemaId"`
Versionuint16
`json:"version"`
}
`json:"header"`
Tsint64
`json:"ts"`
PriceExponent
int8
`json:"priceExponent"`
SizeExponent
int8
`json:"sizeExponent"`
TradeItems
[
]
TradeItem
`json:"tradeItems"`
Symbol
string
`json:"symbol"`
ParsedLength
int
`json:"parsed_length"`
}
func
parsePublicTradeEvent
(
buf
[
]
byte
)
(
*
PublicTradeEvent
,
error
)
{
if
len
(
buf
)
<
8
{
return
nil
,
fmt
.
Errorf
(
"too short for header"
)
}
off
:=
0
blk
:=
binary
.
LittleEndian
.
Uint16
(
buf
[
off
:
off
+
2
]
)
tid
:=
binary
.
LittleEndian
.
Uint16
(
buf
[
off
+
2
:
off
+
4
]
)
sid
:=
binary
.
LittleEndian
.
Uint16
(
buf
[
off
+
4
:
off
+
6
]
)
ver
:=
binary
.
LittleEndian
.
Uint16
(
buf
[
off
+
6
:
off
+
8
]
)
off
+=
8
if
tid
!=
20002
{
return
nil
,
fmt
.
Errorf
(
"unexpected templateId=%d"
,
tid
)
}
if
off
+
8
+
1
+
1
>
len
(
buf
)
{
return
nil
,
fmt
.
Errorf
(
"too short for fixed fields"
)
}
ts
:=
int64
(
binary
.
LittleEndian
.
Uint64
(
buf
[
off
:
off
+
8
]
)
)
off
+=
8
priceExp
:=
int8
(
buf
[
off
]
)
off
++
sizeExp
:=
int8
(
buf
[
off
]
)
off
++
// group header
if
off
+
4
>
len
(
buf
)
{
return
nil
,
fmt
.
Errorf
(
"too short for group header"
)
}
grpBlockLen
:=
binary
.
LittleEndian
.
Uint16
(
buf
[
off
:
off
+
2
]
)
numInGroup
:=
binary
.
LittleEndian
.
Uint16
(
buf
[
off
+
2
:
off
+
4
]
)
off
+=
4
items
:=
make
(
[
]
TradeItem
,
0
,
int
(
numInGroup
)
)
for
i
:=
0
;
i
<
int
(
numInGroup
)
;
i
++
{
entryStart
:=
off
needMin
:=
8
+
8
+
8
+
8
+
1
+
1
+
1
+
8
if
off
+
needMin
>
len
(
buf
)
{
return
nil
,
fmt
.
Errorf
(
"too short for trade entry %d"
,
i
)
}
fillTime
:=
int64
(
binary
.
LittleEndian
.
Uint64
(
buf
[
off
:
off
+
8
]
)
)
;
off
+=
8
priceM
:=
int64
(
binary
.
LittleEndian
.
Uint64
(
buf
[
off
:
off
+
8
]
)
)
;
off
+=
8
sizeM
:=
int64
(
binary
.
LittleEndian
.
Uint64
(
buf
[
off
:
off
+
8
]
)
)
;
off
+=
8
seq
:=
int64
(
binary
.
LittleEndian
.
Uint64
(
buf
[
off
:
off
+
8
]
)
)
;
off
+=
8
side
:=
uint8
(
buf
[
off
]
)
;
off
++
isBlock
:=
uint8
(
buf
[
off
]
)
;
off
++
isRpi
:=
uint8
(
buf
[
off
]
)
;
off
++
execID
:=
int64
(
binary
.
LittleEndian
.
Uint64
(
buf
[
off
:
off
+
8
]
)
)
;
off
+=
8
fixedConsumed
:=
off
-
entryStart
if
fixedConsumed
<
int
(
grpBlockLen
)
{
off
+=
int
(
grpBlockLen
)
-
fixedConsumed
}
else
if
fixedConsumed
>
int
(
grpBlockLen
)
{
return
nil
,
fmt
.
Errorf
(
"group blockLength too small: %d < %d"
,
grpBlockLen
,
fixedConsumed
)
}
execIDStr
,
off2
,
err
:=
readVarString8
(
buf
,
off
)
if
err
!=
nil
{
return
nil
,
err
}
off
=
off2
items
=
append
(
items
,
TradeItem
{
FillTime
:
fillTime
,
PriceMant
:
priceM
,
SizeMant
:
sizeM
,
Price
:
applyExp
(
priceM
,
priceExp
)
,
Size
:
applyExp
(
sizeM
,
sizeExp
)
,
Seq
:
seq
,
Side
:
side
,
IsBlockTrade
:
isBlock
!=
0
,
IsRPI
:
isRpi
!=
0
,
ExecID
:
execID
,
ExecIDString
:
execIDStr
,
}
)
}
symbol
,
off2
,
err
:=
readVarString8
(
buf
,
off
)
if
err
!=
nil
{
return
nil
,
err
}
off
=
off2
evt
:=
&
PublicTradeEvent
{
Ts
:
ts
,
PriceExponent
:
priceExp
,
SizeExponent
:
sizeExp
,
TradeItems
:
items
,
Symbol
:
symbol
,
ParsedLength
:
off
,
}
evt
.
Header
.
BlockLength
=
blk
evt
.
Header
.
TemplateID
=
tid
evt
.
Header
.
SchemaID
=
sid
evt
.
Header
.
Version
=
ver
return
evt
,
nil
}
func
main
(
)
{
d
:=
websocket
.
Dialer
{
HandshakeTimeout
:
10
*
time
.
Second
}
c
,
_
,
err
:=
d
.
Dial
(
WSURL
,
nil
)
if
err
!=
nil
{
log
.
Fatal
(
err
)
}
defer
c
.
Close
(
)
sub
,
_
:=
json
.
Marshal
(
map
[
string
]
any
{
"op"
:
"subscribe"
,
"args"
:
[
]
string
{
Topic
}
}
)
if
err
:=
c
.
WriteMessage
(
websocket
.
TextMessage
,
sub
)
;
err
!=
nil
{
log
.
Fatal
(
err
)
}
log
.
Println
(
"subscribed:"
,
Topic
)
for
{
mt
,
msg
,
err
:=
c
.
ReadMessage
(
)
if
err
!=
nil
{
log
.
Fatal
(
err
)
}
if
mt
==
websocket
.
BinaryMessage
{
evt
,
err
:=
parsePublicTradeEvent
(
msg
)
if
err
!=
nil
{
log
.
Println
(
"decode error:"
,
err
)
continue
}
if
len
(
evt
.
TradeItems
)
>
0
{
t0
:=
evt
.
TradeItems
[
0
]
log
.
Printf
(
"%s trades=%d first=%.8f@%.8f seq=%d"
,
evt
.
Symbol
,
len
(
evt
.
TradeItems
)
,
t0
.
Price
,
t0
.
Size
,
t0
.
Seq
)
}
}
else
{
log
.
Println
(
"TEXT:"
,
string
(
msg
)
)
}
}
}

**Examples:**

Example 1 ():
```
{"req_id": "100001", "op": "ping"}
```

Example 2 ():
```
{"success": true,"ret_msg": "pong","conn_id": "xxxxx-xx","req_id": "","op": "ping"}
```

Example 3 ():
```
{"op": "subscribe","args": ["publicTrade.SBE.BTCUSDT"]}
```

Example 4 ():
```
{  "id": "trade-001",  "topic": "publicTrade.SBE.BTCUSDT",  "type": "snapshot",  "ts": 1760000000000,  "data": [...]}
```

---

## Get Public Trades

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/public-trades

**Contents:**
- Get Public Trades
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get Public Trades
On this page
Get Public Trades
Get the recently executed rfq successfully.
Up to 50 requests per second
HTTP Request
​
GET
/v5/rfq/public-trades
Request Parameters
​
Parameter
Required
Type
Comments
startTime
false
integer
The timestamp (ms),
startTime
and
endTime
of the order transaction are 30 days
endTime
false
integer
The closing timestamp (ms),
startTime
and
endTime
of the order are 30 days
limit
false
integer
Return the number of items.
[
1
,
100
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
result
Object
> cursor
string
Refer to the
cursor
request parameter
> list
array
An array of RFQs
>> rfqId
string
Inquiry ID
>> strategyType
string
Policy type
>> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
>> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
>> legs
array of objects
Combination transaction
>>> category
string
category. Valid values include:
linear
,
option
and
spot
>>> symbol
string
The unique instrument ID
>>> side
string
Direction, valid values are
Buy
and
Sell
>>> price
string
Execution price
>>> qty
string
Number of executions
>>> markPrice
string
The futures markPrice at the time of transaction, the spot is indexPrice, and the option is the markPrice of the underlying Price.
Request Example
​
GET
/v5/rfq/public-trades
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
"cursor"
:
"page_token%3D14912%26last_time%3D1756826273947000000%26"
,
"list"
:
[
{
"rfqId"
:
"1756892210565322771637442724834278"
,
"strategyType"
:
"custom"
,
"legs"
:
[
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Sell"
,
"price"
:
"100000"
,
"qty"
:
"0.5"
,
"markPrice"
:
"110320"
}
]
,
"createdAt"
:
"1756892210567"
,
"updatedAt"
:
"1756892215712"
}
,
{
"rfqId"
:
"1756891080435210075162963643082323"
,
"strategyType"
:
"custom"
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
"price"
:
"143843.9"
,
"qty"
:
"0.01"
,
"markPrice"
:
"143843"
}
]
,
"createdAt"
:
"1756891080437"
,
"updatedAt"
:
"1756891081550"
}
,
{
"rfqId"
:
"1756826272870633375460463539530377"
,
"strategyType"
:
"custom"
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
"price"
:
"107600.9"
,
"qty"
:
"1"
,
"markPrice"
:
"108481.73"
}
]
,
"createdAt"
:
"1756826272871"
,
"updatedAt"
:
"1756826273947"
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
1756892357602
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/public-trades HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "cursor": "page_token%3D14912%26last_time%3D1756826273947000000%26",        "list": [            {                "rfqId": "1756892210565322771637442724834278",                "strategyType": "custom",                "legs": [                    {                        "category": "spot",                        "symbol": "BTCUSDT",                        "side": "Sell",                        "price": "100000",                        "qty": "0.5",                        "markPrice": "110320"                    }                ],                "createdAt": "1756892210567",                "updatedAt": "1756892215712"            },            {                "rfqId": "1756891080435210075162963643082323",                "strategyType": "custom",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "price": "143843.9",                        "qty": "0.01",                        "markPrice": "143843"                    }                ],                "createdAt": "1756891080437",                "updatedAt": "1756891081550"            },            {                "rfqId": "1756826272870633375460463539530377",                "strategyType": "custom",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "price": "107600.9",                        "qty": "1",                        "markPrice": "108481.73"                    }                ],                "createdAt": "1756826272871",                "updatedAt": "1756826273947"            }        ]    },    "retExtInfo": {},    "time": 1756892357602}
```

---

## Execution

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/websocket/private/transaction

**Contents:**
- Execution
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

RFQ Trading
WebSocket Stream
Private
Execution
On this page
Execution
Obtain the user's own block trade information. All legs in the same block trade are included in the same update. As long as the user performs block trade as a counterparty, the data will be pushed.
Topic:
rfq.open.trades
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
int
Data created timestamp (ms)
data
array
Object
data
array
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
> quoteSide
string
Return of completed inquiry, executed quote direction,
buy
or
sell
> strategyType
string
Inquiry label
> status
string
Status:
Filled
,
Failed
> rfqDeskCode
string
The unique identification code of the inquiry party, which is not visible when anonymous is set to
true
during inquiry
> quoteDeskCode
string
The unique identification code of the quoting party, which is not visible when anonymous is set to
true
during quotation
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
category. Valid values include:
linear
,
option
and
spot
>> orderId
string
bybit order id
>> symbol
string
symbol name
>> side
string
Direction, valid values are
buy
and
sell
>> price
string
Execution price
>> qty
string
Number of executions
>> markPrice
string
The markPrice (contract) at the time of transaction, and the spot price is indexPrice
>> execFee
string
The fee for taker or maker in the base currency paid to the Exchange executing the Block Trade.
>> execId
string
The unique exec(trade) ID from the exchange
>> resultCode
integer
The status code of the this order. "0" means success
>>resultMessage
string
Error message about resultCode. If resultCode is "0", resultMessage is "".
>> rejectParty
string
Empty if status is
Filled
. Valid values:
Taker
or
Maker
if status is
Rejected
，"rejectParty=
bybit
" to indicate errors that occur on the Bybit side.
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
"rfq.open.trades"
]
}
Stream Example
​
{
"topic"
:
"rfq.open.trades"
,
"creationTime"
:
1757578749474
,
"data"
:
[
{
"rfqId"
:
"1757578410512325974246073709371267"
,
"rfqLinkId"
:
""
,
"quoteId"
:
"1757578719388835162295211364781592"
,
"quoteLinkId"
:
""
,
"quoteSide"
:
"Buy"
,
"strategyType"
:
"custom"
,
"status"
:
"Filled"
,
"rfqDeskCode"
:
"1nu9d1"
,
"quoteDeskCode"
:
"test0904"
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
"price"
:
"91600"
,
"qty"
:
"1"
,
"orderId"
:
"64fe4108-555e-4361-ae2d-3a8d0c292859"
,
"markPrice"
:
"91741.11"
,
"execFee"
:
"-1.374"
,
"execId"
:
"42b8be1e-36cf-4aba-bb75-4602cc11df37"
,
"resultCode"
:
0
,
"resultMessage"
:
""
,
"rejectParty"
:
""
}
]
,
"createdAt"
:
"1757578749361"
,
"updatedAt"
:
"1757578749464"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "rfq.open.trades"    ]}
```

Example 2 ():
```
{  "topic": "rfq.open.trades",  "creationTime": 1757578749474,  "data": [    {      "rfqId": "1757578410512325974246073709371267",      "rfqLinkId": "",      "quoteId": "1757578719388835162295211364781592",      "quoteLinkId": "",      "quoteSide": "Buy",      "strategyType": "custom",      "status": "Filled",      "rfqDeskCode": "1nu9d1",      "quoteDeskCode": "test0904",      "legs": [        {          "category": "linear",          "symbol": "BTCUSDT",          "side": "Buy",          "price": "91600",          "qty": "1",          "orderId": "64fe4108-555e-4361-ae2d-3a8d0c292859",          "markPrice": "91741.11",          "execFee": "-1.374",          "execId": "42b8be1e-36cf-4aba-bb75-4602cc11df37",          "resultCode": 0,          "resultMessage": "",          "rejectParty": ""        }      ],      "createdAt": "1757578749361",      "updatedAt": "1757578749464"    }  ]}
```

---

## Dcp

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/dcp

**Contents:**
- Dcp
  - Subscribe Example​

WebSocket Stream
Private
Dcp
On this page
Dcp
Subscribe to the dcp stream to trigger DCP function.
For example, connection A subscribes "dcp.xxx", connection B does not and connection C subscribes "dcp.xxx".
If A is alive, B is dead, C is alive, then this case will not trigger DCP.
If A is alive, B is dead, C is dead, then this case will not trigger DCP.
If A is dead, B is alive, C is dead, then DCP is triggered when reach the timeWindow threshold
To sum up, for those private connections subscribing "dcp" topic are all dead, then DCP will be triggered.
Topic:
dcp.future
,
dcp.spot
,
dcp.option
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
"dcp.future"
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "dcp.future"    ]}
```

---

## Fast Execution

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/fast-execution

**Contents:**
- Fast Execution
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

WebSocket Stream
Private
Fast Execution
On this page
Fast Execution
Fast execution stream significantly reduces data latency compared original "execution" stream. However, it pushes limited
execution type of trades, and fewer data fields.
All-In-One Topic:
execution.fast
Categorised Topic:
execution.fast.linear
,
execution.fast.inverse
,
execution.fast.spot
info
Supports all Perps, Futures and Spot exceution, and do not support Options for now
You can only receive
execType
=Trade update
Response Parameters
​
Parameter
Type
Comments
topic
string
Topic name
creationTime
number
Data created timestamp (ms)
data
array
Object
>
category
string
Product type
linear
,
inverse
,
spot
> symbol
string
Symbol name
> orderId
string
Order ID
> isMaker
boolean
true
: Maker,
false
: Taker
> orderLinkId
string
User customized order ID
maker trade is always
""
If a maker order in the orderbook is converted to taker (by price amend), orderLinkId is also ""
> execId
string
Execution ID
> execPrice
string
Execution price
> execQty
string
Execution qty
> side
string
Side.
Buy
,
Sell
> execTime
string
Executed timestamp (ms)
> seq
long
Cross sequence, used to associate each fill and each position update
The seq will be the same when conclude multiple transactions at the same time
Different symbols may have the same seq, please use seq + symbol to check unique
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
"execution.fast"
]
}
Stream Example
​
{
"topic"
:
"execution.fast"
,
"creationTime"
:
1716800399338
,
"data"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"ICPUSDT"
,
"execId"
:
"3510f361-0add-5c7b-a2e7-9679810944fc"
,
"execPrice"
:
"12.015"
,
"execQty"
:
"3000"
,
"orderId"
:
"443d63fa-b4c3-4297-b7b1-23bca88b04dc"
,
"isMaker"
:
false
,
"orderLinkId"
:
"test-00001"
,
"side"
:
"Sell"
,
"execTime"
:
"1716800399334"
,
"seq"
:
34771365464
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "execution.fast"    ]}
```

Example 2 ():
```
{    "topic": "execution.fast",    "creationTime": 1716800399338,    "data": [        {            "category": "linear",            "symbol": "ICPUSDT",            "execId": "3510f361-0add-5c7b-a2e7-9679810944fc",            "execPrice": "12.015",            "execQty": "3000",            "orderId": "443d63fa-b4c3-4297-b7b1-23bca88b04dc",            "isMaker": false,            "orderLinkId": "test-00001",            "side": "Sell",            "execTime": "1716800399334",            "seq": 34771365464        }    ]}
```

---

## Get DCP Info

**URL:** https://bybit-exchange.github.io/docs/v5/account/dcp-info

**Contents:**
- Get DCP Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Get DCP Info
On this page
Get DCP Info
Query the DCP configuration of the account. Before calling the interface, please make sure you have applied for the UTA account DCP configuration with your account manager
Only the configured main / sub account can query information from this API. Calling this API by an account always returns empty.
If you only request to activate Spot trading for DCP, the contract and options data will not be returned.
info
Support USDT Perpetuals, USDT Futures, USDC Perpetuals, USDC Futures, Inverse Perpetuals, Inverse Futures
[DERIVATIVES]
Spot
[SPOT]
Options
[OPTIONS]
HTTP Request
​
GET
/v5/account/query-dcp-info
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
dcpInfos
array
<
object
>
DCP config for each product
> product
string
SPOT
,
DERIVATIVES
,
OPTIONS
> dcpStatus
string
Disconnected-CancelAll-Prevention
status:
ON
> timeWindow
string
DCP trigger time window which user pre-set. Between
[3, 300]
seconds, default: 10 sec
Request Example
​
HTTP
Node.js
GET
/v5/account/query-dcp-info
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
1717065530867
X-BAPI-RECV-WINDOW
:
5000
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
getDCPInfo
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
// it means my account enables Spot and Deriviatvies on the backend
// Options is not enabled with DCP
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
"dcpInfos"
:
[
{
"product"
:
"SPOT"
,
"dcpStatus"
:
"ON"
,
"timeWindow"
:
"10"
}
,
{
"product"
:
"DERIVATIVES"
,
"dcpStatus"
:
"ON"
,
"timeWindow"
:
"10"
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
1717065531697
}

**Examples:**

Example 1 ():
```
GET /v5/account/query-dcp-info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1717065530867X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getDCPInfo()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 3 ():
```
// it means my account enables Spot and Deriviatvies on the backend// Options is not enabled with DCP{    "retCode": 0,    "retMsg": "success",    "result": {        "dcpInfos": [            {                "product": "SPOT",                "dcpStatus": "ON",                "timeWindow": "10"            },            {                "product": "DERIVATIVES",                "dcpStatus": "ON",                "timeWindow": "10"            }        ]    },    "retExtInfo": {},    "time": 1717065531697}
```

---

## Get Repayment History

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/repay-history

**Contents:**
- Get Repayment History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Repayment History
On this page
Get Repayment History
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-fixed/repayment-history
Request Parameters
​
Parameter
Required
Type
Comments
repayId
false
string
Repayment order ID
loanCurrency
false
string
Loan coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> details
array
Object
>> loanCurrency
string
Loan coin name
>> repayAmount
long
Repay amount
>> loanId
string
Loan ID. One repayment may involve multiple loan contracts.
> loanCurrency
string
Loan coin name
> repayAmount
long
Repay amount
> repayId
string
Repay order ID
> repayStatus
integer
Status,
1
: success,
2
: processing,
3
: fail
> repayTime
long
Repay time
> repayType
integer
Repay type,
1
: repay by user;
2
: repay by liquidation;
3
: auto repay;
4
: overdue repay;
5
: repay by delisting;
6
: repay by delay liquidation;
7
: repay by currency;
8
: transfer to flexible loan
nextPageCursor
string
Refer to the
cursor
request parameter
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/repayment-history?repayId=1780
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXXXX
X-BAPI-API-KEY
:
XXXXXXX
X-BAPI-TIMESTAMP
:
1752714738425
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
get_repayment_history_fixed_crypto_loan
(
repayId
=
"1780"
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
"details"
:
[
{
"loanCurrency"
:
"ETH"
,
"loanId"
:
"568"
,
"repayAmount"
:
"0.1"
}
,
{
"loanCurrency"
:
"ETH"
,
"loanId"
:
"571"
,
"repayAmount"
:
"1.4"
}
]
,
"loanCurrency"
:
"ETH"
,
"repayAmount"
:
"1.5"
,
"repayId"
:
"1782"
,
"repayStatus"
:
1
,
"repayTime"
:
1752717174353
,
"repayType"
:
1
}
]
,
"nextPageCursor"
:
"1674"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752717183557
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/repayment-history?repayId=1780 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: XXXXXXXX-BAPI-TIMESTAMP: 1752714738425X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_repayment_history_fixed_crypto_loan(    repayId="1780",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "details": [                    {                        "loanCurrency": "ETH",                        "loanId": "568",                        "repayAmount": "0.1"                    },                    {                        "loanCurrency": "ETH",                        "loanId": "571",                        "repayAmount": "1.4"                    }                ],                "loanCurrency": "ETH",                "repayAmount": "1.5",                "repayId": "1782",                "repayStatus": 1,                "repayTime": 1752717174353,                "repayType": 1            }        ],        "nextPageCursor": "1674"    },    "retExtInfo": {},    "time": 1752717183557}
```

---

## Renew Borrow Order

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/renew

**Contents:**
- Renew Borrow Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Renew Borrow Order
On this page
Renew Borrow Order
Permission: "Spot trade"
UID rate limit: 1 req / second
info
The loan funds are released to the Funding wallet.
The collateral funds are deducted from the Funding wallet, so make sure you have enough collateral amount in the Funding wallet.
This endpoint allows you to re-borrow the principal that was previously repaid. The renewal amount is the same as the amount previously repaid on this loan.
HTTP Request
​
POST
/v5/crypto-loan-fixed/renew
Request Parameters
​
Parameter
Required
Type
Comments
loanId
true
string
Loan ID
collateralList
false
array
<
object
>
Collateral coin list, supports putting up to 100 currency in the array
> currency
false
string
Currency used to mortgage
> amount
false
string
Amount to mortgage
Response Parameters
​
Parameter
Type
Comments
orderId
string
Loan order ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/renew
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
1752633649752
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
208
{
"loanId"
:
"2364"
,
"collateralList"
:
{
"currency"
:
"ETH"
,
"amount"
:
"1"
}
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
renew_fixed_crypto_loan
(
loanId
=
"2364"
,
collateralList
=
{
"currency"
:
"ETH"
,
"amount"
:
"1"
,
}
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
"orderId"
:
49
}
,
"retExtInfo"
:
{
}
,
"time"
:
1764142142931
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/renew HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752633649752X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 208{    "loanId": "2364",    "collateralList": {"currency": "ETH","amount": "1"}}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.renew_fixed_crypto_loan(    loanId="2364",    collateralList={        "currency": "ETH",        "amount": "1",    },))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "orderId": 49    },    "retExtInfo": {},    "time": 1764142142931}
```

---

## Get Renew Order Info

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/renew-order

**Contents:**
- Get Renew Order Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Renew Order Info
On this page
Get Renew Order Info
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-fixed/renew-info
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
orderCurrency
false
string
Loan coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> borrowCurrency
string
Borrow currency
> amount
string
loan amount
> autoRepay
integer
1
: Auto Repayment;
2
: Transfer to flexible loan;
0
: No Automatic Repayment. Compatible with existing orders;
> contractNo
string
Contract number
> dueTime
string
Due time
> orderId
integer
Order Id
> loanId
string
Loan Id
> renewLoanNo
string
Renew Loan number
> time
string
timestamps
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/renew-info
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
1752655239825
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
get_renewal_orders_fixed_crypto_loan
(
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
"amount"
:
"11"
,
"autoRepay"
:
2
,
"borrowCurrency"
:
"USDT"
,
"contractNo"
:
"2092164378648656896"
,
"dueTime"
:
"1766750400000"
,
"loanId"
:
"2364"
,
"orderId"
:
49
,
"renewLoanNo"
:
"2092170365690461952"
,
"time"
:
"1764142142913"
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
1764208336537
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/renew-info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752655239825X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_renewal_orders_fixed_crypto_loan())
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "amount": "11",                "autoRepay": 2,                "borrowCurrency": "USDT",                "contractNo": "2092164378648656896",                "dueTime": "1766750400000",                "loanId": "2364",                "orderId": 49,                "renewLoanNo": "2092170365690461952",                "time": "1764142142913"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1764208336537}
```

---

## Batch Cancel Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/batch-cancel

**Contents:**
- Batch Cancel Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Batch Cancel Order
On this page
Batch Cancel Order
This endpoint allows you to cancel more than one open order in a single request.
important
You must specify
orderId
or
orderLinkId
.
If
orderId
and
orderLinkId
is not matched, the system will process
orderId
first.
You can cancel
unfilled
or
partially filled
orders.
A maximum of 20 orders (option), 20 orders (inverse), 20 orders (linear), 10 orders (spot) can be cancelled per request.
HTTP Request
​
POST
/v5/order/cancel-batch
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
option
,
spot
,
inverse
request
true
array
Object
> symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
> orderId
false
string
Order ID. Either
orderId
or
orderLinkId
is required
> orderLinkId
false
string
User customised order ID. Either
orderId
or
orderLinkId
is required
Response Parameters
​
Parameter
Type
Comments
result
Object
> list
array
Object
>> category
string
Product type
>> symbol
string
Symbol name
>> orderId
string
Order ID
>> orderLinkId
string
User customised order ID
retExtInfo
Object
> list
array
Object
>> code
number
Success/error code
>> msg
string
Success/error message
info
The acknowledgement of an cancel order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
RUN >>
Request Example
​
HTTP
Python
Java
.Net
Node.js
POST
/v5/order/cancel-batch
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
1672223356634
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"category"
:
"spot"
,
"request"
:
[
{
"symbol"
:
"BTCUSDT"
,
"orderId"
:
"1666800494330512128"
}
,
{
"symbol"
:
"ATOMUSDT"
,
"orderLinkId"
:
"1666800494330512129"
}
]
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
cancel_batch_order
(
category
=
"spot"
,
request
=
[
{
"symbol"
:
"BTCUSDT"
,
"orderId"
:
"1666800494330512128"
}
,
{
"symbol"
:
"ATOMUSDT"
,
"orderLinkId"
:
"1666800494330512129"
}
]
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
restApi
.
BybitApiTradeRestClient
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
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
var
cancelOrderRequests
=
Arrays
.
asList
(
TradeOrderRequest
.
builder
(
)
.
symbol
(
"BTC-10FEB23-24000-C"
)
.
orderLinkId
(
"9b381bb1-401"
)
.
build
(
)
,
TradeOrderRequest
.
builder
(
)
.
symbol
(
"BTC-10FEB23-24000-C"
)
.
orderLinkId
(
"82ee86dd-001"
)
.
build
(
)
)
;
var
cancelBatchOrders
=
BatchOrderRequest
.
builder
(
)
.
category
(
ProductType
.
OPTION
)
.
request
(
cancelOrderRequests
)
.
build
(
)
;
client
.
createBatchOrder
(
cancelBatchOrders
,
System
.
out
::
println
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
var order1 = new OrderRequest { Symbol = "BTC-10FEB23-24000-C", OrderLinkId = "9b381bb1-401" };
var order2 = new OrderRequest { Symbol = "BTC-10FEB23-24000-C", OrderLinkId = "82ee86dd-001" };
var orderInfoString = await TradeService.CancelBatchOrder(category: Category.LINEAR, request: new List<OrderRequest> { order1, order2 });
Console.WriteLine(orderInfoString);
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
batchCancelOrders
(
'spot'
,
[
{
"symbol"
:
"BTCUSDT"
,
"orderId"
:
"1666800494330512128"
}
,
{
"symbol"
:
"ATOMUSDT"
,
"orderLinkId"
:
"1666800494330512129"
}
,
]
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
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"orderId"
:
"1666800494330512128"
,
"orderLinkId"
:
"spot-btc-03"
}
,
{
"category"
:
"spot"
,
"symbol"
:
"ATOMUSDT"
,
"orderId"
:
""
,
"orderLinkId"
:
"1666800494330512129"
}
]
}
,
"retExtInfo"
:
{
"list"
:
[
{
"code"
:
0
,
"msg"
:
"OK"
}
,
{
"code"
:
170213
,
"msg"
:
"Order does not exist."
}
]
}
,
"time"
:
1713434299047
}

**Examples:**

Example 1 ():
```
POST /v5/order/cancel-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672223356634X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "spot",    "request": [        {            "symbol": "BTCUSDT",            "orderId": "1666800494330512128"        },        {            "symbol": "ATOMUSDT",            "orderLinkId": "1666800494330512129"        }    ]}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.cancel_batch_order(    category="spot",    request=[        {            "symbol": "BTCUSDT",            "orderId": "1666800494330512128"        },        {            "symbol": "ATOMUSDT",            "orderLinkId": "1666800494330512129"        }    ]))
```

Example 3 ():
```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var cancelOrderRequests = Arrays.asList(TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").orderLinkId("9b381bb1-401").build(),                TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").orderLinkId("82ee86dd-001").build());var cancelBatchOrders = BatchOrderRequest.builder().category(ProductType.OPTION).request(cancelOrderRequests).build();client.createBatchOrder(cancelBatchOrders, System.out::println);
```

Example 4 ():
```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;var order1 = new OrderRequest { Symbol = "BTC-10FEB23-24000-C", OrderLinkId = "9b381bb1-401" };var order2 = new OrderRequest { Symbol = "BTC-10FEB23-24000-C", OrderLinkId = "82ee86dd-001" };var orderInfoString = await TradeService.CancelBatchOrder(category: Category.LINEAR, request: new List<OrderRequest> { order1, order2 });Console.WriteLine(orderInfoString);
```

---

## Batch Place Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/batch-place

**Contents:**
- Batch Place Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Batch Place Order
On this page
Batch Place Order
tip
This endpoint allows you to place more than one order in a single request.
Make sure you have sufficient funds in your account when placing an order. Once an order is placed, according to the
funds required by the order, the funds in your account will be frozen by the corresponding amount during the life cycle
of the order.
A maximum of 20 orders (option), 20 orders (inverse), 20 orders (linear), 10 orders (spot) can be placed per request. The returned data list is divided into two lists.
The first list indicates whether or not the order creation was successful and the second list details the created order information. The structure of the two lists are completely consistent.
info
Option rate limt
instruction: its rate limit is count based on the actual number of request sent, e.g., by default, option trading rate limit is 10 reqs per sec, so you can send up to 20 * 10 = 200 orders in one second.
Perpetual, Futures, Spot rate limit instruction
, please check
here
Risk control limit notice:
Bybit will monitor on your API requests. When the total number of orders of a single user (aggregated the number of orders across main account and subaccounts) within a day (UTC 0 - UTC 24) exceeds a certain upper limit, the platform will reserve the right to remind, warn, and impose necessary restrictions.
Customers who use API default to acceptance of these terms and have the obligation to cooperate with adjustments.
HTTP Request
​
POST
/v5/order/create-batch
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
option
,
spot
,
inverse
request
true
array
Object
> symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
> isLeverage
false
integer
Whether to borrow,
spot
** only.
0
(default): false then spot trading,
1
: true then margin trading
> side
true
string
Buy
,
Sell
>
orderType
true
string
Market
,
Limit
> qty
true
string
Order quantity
Spot: set
marketUnit
for market order qty unit,
quoteCoin
for market buy by default,
baseCoin
for market sell by default
Perps, Futures & Option: always use base coin as unit.
Perps & Futures: If you pass
qty
="0" and specify
reduceOnly
=true&
closeOnTrigger
=true, you can close the position up to
maxMktOrderQty
or
maxOrderQty
shown on
Get Instruments Info
of current symbol
> marketUnit
false
string
The unit for
qty
when create
Spot market
orders,
orderFilter
="tpslOrder" and "StopOrder" are supported as well.
baseCoin
: for example, buy BTCUSDT, then "qty" unit is BTC
quoteCoin
: for example, sell BTCUSDT, then "qty" unit is USDT
> price
false
string
Order price
Market order will ignore this field
Please check the min price and price precision from
instrument info
endpoint
If you have position, price needs to be better than liquidation price
> triggerDirection
false
integer
Conditional order param. Used to identify the expected direction of the conditional order.
1
: triggered when market price rises to
triggerPrice
2
: triggered when market price falls to
triggerPrice
Valid for
linear
> orderFilter
false
string
If it is not passed,
Order
by default.
Order
tpslOrder
: Spot TP/SL order, the assets are occupied even before the order is triggered
StopOrder
: Spot conditional order, the assets will not be occupied until the price of the underlying asset reaches the trigger price, and the required assets will be occupied after the Conditional order is triggered
Valid for
spot
only
> triggerPrice
false
string
For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:
triggerPrice > market price
Else,
triggerPrice < market price
For spot, it is the
orderFilter
="tpslOrder", or "StopOrder" trigger price
>
triggerBy
false
string
Conditional order param (Perps & Futures). Trigger price type.
LastPrice
,
IndexPrice
,
MarkPrice
> orderIv
false
string
Implied volatility.
option
only
. Pass the real value, e.g for 10%, 0.1 should be passed.
orderIv
has a higher priority when
price
is passed as well
>
timeInForce
false
string
Time in force
Market order will use
IOC
directly
If not passed,
GTC
is used by default
>
positionIdx
false
integer
Used to identify positions in different position modes. Under hedge-mode, this param is
required
0
: one-way mode
1
: hedge-mode Buy side
2
: hedge-mode Sell side
> orderLinkId
false
string
User customised order ID. A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.
Futures, Perps & Spot: orderLinkId rules
:
optional param
always unique
option
orderLinkId rules
:
required
param
always unique
> takeProfit
false
string
Take profit price
> stopLoss
false
string
Stop loss price
>
tpTriggerBy
false
string
The price type to trigger take profit.
MarkPrice
,
IndexPrice
, default:
LastPrice
.
Valid for
linear
,
inverse
>
slTriggerBy
false
string
The price type to trigger stop loss.
MarkPrice
,
IndexPrice
, default:
LastPrice
Valid for
linear
,
inverse
> reduceOnly
false
boolean
What is a reduce-only order?
true
means your position can only reduce in size if this order is triggered.
You
must
specify it as
true
when you are about to close/reduce the position
When reduceOnly is true, take profit/stop loss cannot be set
Valid for
linear
,
inverse
&
option
> closeOnTrigger
false
boolean
What is a close on trigger order?
For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
Valid for
linear
,
inverse
>
smpType
false
string
Smp execution type.
What is SMP?
> mmp
false
boolean
Market maker protection.
option
only
.
true
means set the order as a market maker protection order.
What is mmp?
> tpslMode
false
string
TP/SL mode
Full
: entire position for TP/SL. Then, tpOrderType or slOrderType must be
Market
Partial
: partial position tp/sl (as there is no size option, so it will create tp/sl orders with the qty you actually fill). Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is
required
and it must be
Partial
Valid for
linear
,
inverse
> tpLimitPrice
false
string
The limit order price when take profit price is triggered
linear
&
inverse
: only works when tpslMode=Partial and tpOrderType=Limit
Spot: it is required when the order has
takeProfit
and
tpOrderType=Limit
> slLimitPrice
false
string
The limit order price when stop loss price is triggered
linear
&
inverse
: only works when tpslMode=Partial and slOrderType=Limit
Spot: it is required when the order has
stopLoss
and
slOrderType=Limit
> tpOrderType
false
string
The order type when take profit is triggered
linear
&
inverse
:
Market
(default),
Limit
. For tpslMode=Full, it only supports tpOrderType=Market
Spot:
Market
: when you set "takeProfit",
Limit
: when you set "takeProfit" and "tpLimitPrice"
> slOrderType
false
string
The order type when stop loss is triggered
linear
&
inverse
:
Market
(default),
Limit
. For tpslMode=Full, it only supports slOrderType=Market
Spot:
Market
: when you set "stopLoss",
Limit
: when you set "stopLoss" and "slLimitPrice"
Response Parameters
​
Parameter
Type
Comments
result
Object
> list
array
Object
>> category
string
Product type
>> symbol
string
Symbol name
>> orderId
string
Order ID
>> orderLinkId
string
User customised order ID
>> createAt
string
Order created time (ms)
retExtInfo
Object
> list
array
Object
>> code
number
Success/error code
>> msg
string
Success/error message
info
The acknowledgement of an place order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
RUN >>
Request Example
​
HTTP
Python
Go
Java
.Net
Node.js
POST
/v5/order/create-batch
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
1672222064519
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"category"
:
"spot"
,
"request"
:
[
{
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
0
,
"qty"
:
"0.05"
,
"price"
:
"30000"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-btc-03"
}
,
{
"symbol"
:
"ATOMUSDT"
,
"side"
:
"Sell"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
0
,
"qty"
:
"2"
,
"price"
:
"12"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-atom-03"
}
]
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
place_batch_order
(
category
=
"spot"
,
request
=
[
{
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
0
,
"qty"
:
"0.05"
,
"price"
:
"30000"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-btc-03"
}
,
{
"symbol"
:
"ATOMUSDT"
,
"side"
:
"Sell"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
0
,
"qty"
:
"2"
,
"price"
:
"12"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-atom-03"
}
]
)
)
import
(
"context"
"fmt"
bybit
"https://github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
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
"request"
:
[
]
map
[
string
]
interface
{
}
{
{
"category"
:
"option"
,
"symbol"
:
"BTC-10FEB23-24000-C"
,
"orderType"
:
"Limit"
,
"side"
:
"Buy"
,
"qty"
:
"0.1"
,
"price"
:
"5"
,
"orderIv"
:
"0.1"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"9b381bb1-401"
,
"mmp"
:
false
,
"reduceOnly"
:
false
,
}
,
{
"category"
:
"option"
,
"symbol"
:
"BTC-10FEB23-24000-C"
,
"orderType"
:
"Limit"
,
"side"
:
"Buy"
,
"qty"
:
"0.1"
,
"price"
:
"5"
,
"orderIv"
:
"0.1"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"82ee86dd-001"
,
"mmp"
:
false
,
"reduceOnly"
:
false
,
}
,
}
,
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
PlaceBatchOrder
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
restApi
.
BybitApiAsyncTradeRestClient
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
ProductType
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
TradeOrderType
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
import
java
.
util
.
Arrays
;
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
var
orderRequests
=
Arrays
.
asList
(
TradeOrderRequest
.
builder
(
)
.
category
(
ProductType
.
OPTION
)
.
symbol
(
"BTC-10FEB23-24000-C"
)
.
side
(
Side
.
BUY
)
.
orderType
(
TradeOrderType
.
LIMIT
)
.
qty
(
"0.1"
)
.
price
(
"5"
)
.
orderIv
(
"0.1"
)
.
timeInForce
(
TimeInForce
.
GOOD_TILL_CANCEL
)
.
orderLinkId
(
"9b381bb1-401"
)
.
mmp
(
false
)
.
reduceOnly
(
false
)
.
build
(
)
,
TradeOrderRequest
.
builder
(
)
.
category
(
ProductType
.
OPTION
)
.
symbol
(
"BTC-10FEB23-24000-C"
)
.
side
(
Side
.
BUY
)
.
orderType
(
TradeOrderType
.
LIMIT
)
.
qty
(
"0.1"
)
.
price
(
"5"
)
.
orderIv
(
"0.1"
)
.
timeInForce
(
TimeInForce
.
GOOD_TILL_CANCEL
)
.
orderLinkId
(
"82ee86dd-001"
)
.
mmp
(
false
)
.
reduceOnly
(
false
)
.
build
(
)
)
;
var
createBatchOrders
=
BatchOrderRequest
.
builder
(
)
.
category
(
ProductType
.
OPTION
)
.
request
(
orderRequests
)
.
build
(
)
;
client
.
createBatchOrder
(
createBatchOrders
,
System
.
out
::
println
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
var order1 = new OrderRequest { Symbol = "XRPUSDT", OrderType = "Limit", Side = "Buy", Qty = "10", Price = "0.6080", TimeInForce = "GTC" };
var order2 = new OrderRequest { Symbol = "BLZUSDT", OrderType = "Limit", Side = "Buy", Qty = "10", Price = "0.6080", TimeInForce = "GTC" };
List<OrderRequest> request = new() { order1, order2 };
var orderInfoString = await TradeService.PlaceBatchOrder(category: Category.LINEAR, request: request);
Console.WriteLine(orderInfoString);
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
batchSubmitOrders
(
'spot'
,
[
{
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
0
,
"qty"
:
"0.05"
,
"price"
:
"30000"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-btc-03"
}
,
{
"symbol"
:
"ATOMUSDT"
,
"side"
:
"Sell"
,
"orderType"
:
"Limit"
,
"isLeverage"
:
0
,
"qty"
:
"2"
,
"price"
:
"12"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-atom-03"
}
,
]
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
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"orderId"
:
"1666800494330512128"
,
"orderLinkId"
:
"spot-btc-03"
,
"createAt"
:
"1713434102752"
}
,
{
"category"
:
"spot"
,
"symbol"
:
"ATOMUSDT"
,
"orderId"
:
"1666800494330512129"
,
"orderLinkId"
:
"spot-atom-03"
,
"createAt"
:
"1713434102752"
}
]
}
,
"retExtInfo"
:
{
"list"
:
[
{
"code"
:
0
,
"msg"
:
"OK"
}
,
{
"code"
:
0
,
"msg"
:
"OK"
}
]
}
,
"time"
:
1713434102753
}

**Examples:**

Example 1 ():
```
POST /v5/order/create-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672222064519X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "spot",    "request": [        {            "symbol": "BTCUSDT",            "side": "Buy",            "orderType": "Limit",            "isLeverage": 0,            "qty": "0.05",            "price": "30000",            "timeInForce": "GTC",            "orderLinkId": "spot-btc-03"        },        {            "symbol": "ATOMUSDT",            "side": "Sell",            "orderType": "Limit",            "isLeverage": 0,            "qty": "2",            "price": "12",            "timeInForce": "GTC",            "orderLinkId": "spot-atom-03"        }    ]}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.place_batch_order(    category="spot",    request=[        {            "symbol": "BTCUSDT",            "side": "Buy",            "orderType": "Limit",            "isLeverage": 0,            "qty": "0.05",            "price": "30000",            "timeInForce": "GTC",            "orderLinkId": "spot-btc-03"        },        {            "symbol": "ATOMUSDT",            "side": "Sell",            "orderType": "Limit",            "isLeverage": 0,            "qty": "2",            "price": "12",            "timeInForce": "GTC",            "orderLinkId": "spot-atom-03"        }    ]))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "https://github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("YOUR_API_KEY", "YOUR_API_SECRET", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "option",    "request": []map[string]interface{}{        {            "category":    "option",            "symbol":      "BTC-10FEB23-24000-C",            "orderType":   "Limit",            "side":        "Buy",            "qty":         "0.1",            "price":       "5",            "orderIv":     "0.1",            "timeInForce": "GTC",            "orderLinkId": "9b381bb1-401",            "mmp":         false,            "reduceOnly":  false,        },        {            "category":    "option",            "symbol":      "BTC-10FEB23-24000-C",            "orderType":   "Limit",            "side":        "Buy",            "qty":         "0.1",            "price":       "5",            "orderIv":     "0.1",            "timeInForce": "GTC",            "orderLinkId": "82ee86dd-001",            "mmp":         false,            "reduceOnly":  false,        },    },}client.NewUtaBybitServiceWithParams(params).PlaceBatchOrder(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.restApi.BybitApiAsyncTradeRestClient;import com.bybit.api.client.domain.ProductType;import com.bybit.api.client.domain.TradeOrderType;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;import java.util.Arrays;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var orderRequests = Arrays.asList(TradeOrderRequest.builder().category(ProductType.OPTION).symbol("BTC-10FEB23-24000-C").side(Side.BUY).orderType(TradeOrderType.LIMIT).qty("0.1")                        .price("5").orderIv("0.1").timeInForce(TimeInForce.GOOD_TILL_CANCEL).orderLinkId("9b381bb1-401").mmp(false).reduceOnly(false).build(),                TradeOrderRequest.builder().category(ProductType.OPTION).symbol("BTC-10FEB23-24000-C").side(Side.BUY).orderType(TradeOrderType.LIMIT).qty("0.1")                        .price("5").orderIv("0.1").timeInForce(TimeInForce.GOOD_TILL_CANCEL).orderLinkId("82ee86dd-001").mmp(false).reduceOnly(false).build());var createBatchOrders = BatchOrderRequest.builder().category(ProductType.OPTION).request(orderRequests).build();client.createBatchOrder(createBatchOrders, System.out::println);
```

---

## Create Borrow Order

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/borrow

**Contents:**
- Create Borrow Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Create Borrow Order
On this page
Create Borrow Order
Permission: "Spot trade"
UID rate limit: 1 req / second
info
The loan funds are released to the Funding wallet.
The collateral funds are deducted from the Funding wallet, so make sure you have enough collateral amount in the Funding wallet.
HTTP Request
​
POST
/v5/crypto-loan-fixed/borrow
Request Parameters
​
Parameter
Required
Type
Comments
orderCurrency
true
string
Currency to borrow
orderAmount
true
string
Amount to borrow
annualRate
true
string
Customizable annual interest rate, e.g.,
0.02
means 2%
term
true
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
autoRepay
false
string
Deprecated. Enable Auto-Repay to have assets in your Funding Account automatically repay your loan upon Borrowing order expiration, preventing overdue penalties. Ensure your Funding Account maintains sufficient amount for repayment to avoid automatic repayment failures.
"true"
: enable, default;
"false"
: disable
repayType
false
string
1
:Auto Repayment (default); Enable "Auto Repayment" to automatically repay your loan using assets in your funding account when it dues, avoiding overdue penalties.
2
:Transfer to flexible loan
collateralList
false
array
<
object
>
Collateral coin list, supports putting up to 100 currency in the array
> currency
false
string
Currency used to mortgage
> amount
false
string
Amount to mortgage
Response Parameters
​
Parameter
Type
Comments
orderId
string
Loan order ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/borrow
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
1752633649752
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
208
{
"orderCurrency"
:
"ETH"
,
"orderAmount"
:
"1.5"
,
"annualRate"
:
"0.022"
,
"term"
:
"30"
,
"autoRepay"
:
"true"
,
"collateralList"
:
{
"currency"
:
"BTC"
,
"amount"
:
"0.1"
}
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
borrow_fixed_crypto_loan
(
loanCurrency
=
"ETH"
,
loanAmount
=
"1.5"
,
annualRate
=
"0.022"
,
term
=
"30"
,
autoRepay
=
"true"
,
collateralList
=
{
"currency"
:
"BTC"
,
"amount"
:
"0.1"
,
}
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
"orderId"
:
"13007"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752633650147
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/borrow HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752633649752X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 208{    "orderCurrency": "ETH",    "orderAmount": "1.5",    "annualRate": "0.022",    "term": "30",    "autoRepay": "true",    "collateralList": {        "currency": "BTC",        "amount": "0.1"    }}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.borrow_fixed_crypto_loan(    loanCurrency="ETH",    loanAmount="1.5",    annualRate="0.022",    term="30",    autoRepay="true",    collateralList={        "currency": "BTC",        "amount": "0.1",    },))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "orderId": "13007"    },    "retExtInfo": {},    "time": 1752633650147}
```

---

## Get Flexible Loans

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/flexible/unpaid-loan-order

**Contents:**
- Get Flexible Loans
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Flexible Loan
Get Flexible Loans
On this page
Get Flexible Loans
Query for your ongoing loans
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-flexible/ongoing-coin
Request Parameters
​
Parameter
Required
Type
Comments
loanCurrency
false
string
Loan coin name
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> hourlyInterestRate
string
Latest hourly flexible interest rate
> loanCurrency
string
Loan coin
> totalDebt
string
Unpaid principal and interest
> unpaidAmount
string
Unpaid principal
> unpaidInterest
string
Unpaid interest
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-flexible/ongoing-coin?loanCurrency=BTC
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
1752570124973
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
get_flexible_loans_flexible_crypto_loan
(
loanCurrency
=
"BTC"
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
"hourlyInterestRate"
:
"0.0000018847396"
,
"loanCurrency"
:
"ETH"
,
"totalDebt"
:
"0.10000019"
,
"unpaidAmount"
:
"0.1"
,
"unpaidInterest"
:
"0.00000019"
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
1760452029499
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-flexible/ongoing-coin?loanCurrency=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752570124973X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_flexible_loans_flexible_crypto_loan(    loanCurrency="BTC",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "hourlyInterestRate": "0.0000018847396",                "loanCurrency": "ETH",                "totalDebt": "0.10000019",                "unpaidAmount": "0.1",                "unpaidInterest": "0.00000019"            }        ]    },    "retExtInfo": {},    "time": 1760452029499}
```

---

## Trade

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/websocket/public/public-transaction

**Contents:**
- Trade
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

RFQ Trading
WebSocket Stream
Public
Trade
On this page
Trade
Latest block trade information. All legs in the same block trade are included in the same update. Data will be pushed whenever there is a block trade.
Topic:
rfq.open.public.trades
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
int
Data created timestamp (ms)
data
array
Object
> rfqId
string
Inquiry ID
>strategyType
string
Policy type
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
Product type:
spot
,
linear
,
option
>> symbol
string
symbol name
>> side
string
Inquiry direction: Valid values are
buy
and
sell
>> price
string
Execution price
>> qty
string
Number of executions
>> markPrice
string
The markPrice (contract) at the time of transaction, and the spot price is indexPrice
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
"rfq.open.public.trades"
]
}
Stream Example
​
{
"topic"
:
"rfq.open.public.trades"
,
"creationTime"
:
1757579314358
,
"data"
:
[
{
"rfqId"
:
"1757579281847749169219132657134900"
,
"strategyType"
:
"custom"
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
"Sell"
,
"price"
:
"91600"
,
"qty"
:
"1"
,
"markPrice"
:
"90216.29"
}
]
,
"createdAt"
:
"1757579314213"
,
"updatedAt"
:
"1757579314347"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "rfq.open.public.trades"    ]}
```

Example 2 ():
```
{  "topic": "rfq.open.public.trades",  "creationTime": 1757579314358,  "data": [    {      "rfqId": "1757579281847749169219132657134900",      "strategyType": "custom",      "legs": [        {          "category": "linear",          "symbol": "BTCUSDT",          "side": "Sell",          "price": "91600",          "qty": "1",          "markPrice": "90216.29"        }      ],      "createdAt": "1757579314213",      "updatedAt": "1757579314347"    }  ]}
```

---

## Order Price Limit

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/public/order-price-limit

**Contents:**
- Order Price Limit
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

WebSocket Stream
Public
Order Price Limit
On this page
Order Price Limit
Subscribe to Get Order Price Limit.
For derivative trading order price limit, refer to
announcement
For spot trading order price limit, refer to
announcement
Push frequency:
300ms
Topic:
priceLimit.{symbol}
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
Object.
> symbol
string
Symbol name
> buyLmt
string
Highest Bid Price
> sellLmt
string
Lowest Ask Price
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
"priceLimit.BTCUSDT"
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
price_limit_stream
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
{
"topic"
:
"priceLimit.BTCUSDT"
,
"data"
:
{
"symbol"
:
"BTCUSDT"
,
"buyLmt"
:
"114450.00"
,
"sellLmt"
:
"103550.00"
}
,
"ts"
:
1750059683782
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "priceLimit.BTCUSDT"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="linear",)def handle_message(message):    print(message)ws.price_limit_stream(    symbol="BTCUSDT",    callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "topic": "priceLimit.BTCUSDT",    "data": {        "symbol": "BTCUSDT",        "buyLmt": "114450.00",        "sellLmt": "103550.00"    },    "ts": 1750059683782}
```

---

## Create RFQ

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/create-rfq

**Contents:**
- Create RFQ
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Create RFQ
On this page
Create RFQ
Create RFQ.
Up to 50 requests
per second.
info
Only supports UTA2.0 accounts
Only supports full position and combined margin mode
Not supported by demo users
Cannot choose oneself as the bidder
HTTP Request
​
POST
/v5/rfq/create-rfq
Request Parameters
​
Parameter
Required
Type
Comments
counterparties
true
array
Spread combination symbol name
rfqLinkId
false
string
Custom RFQ ID
The length should be between 1-32 bits
Combination of letters (case sensitive) and numbers
An rfqLinkId expires after three months – after which it can be reused
Open orders must have a unique ID whereas orders that have reached a final/terminated status do not have to be unique.
anonymous
false
boolean
Whether or not it is anonymous inquiry. The default value is
false
. When it is
true
the identity of the inquiring party will not be revealed even after the transaction is concluded.
strategyType
false
string
Strategy type, if it is a custom inquiry, strategyType is
custom
, if it is a product combination provided by the system, it is the combination type; the default is
custom
; non-custom combinations have rate optimization, currently 50%; the transaction rate between LPs is currently 30%
list
true
array of objects
Combination transaction list
Use
Get RFQ Configuration
to confirm the maximum length of the combination (
maxLegs
)
The base coin and settle coin of all combinations must be the same
Symbols under the same category must be unique
> category
true
string
Product type: Unified account:
spot
,
linear
,
option
> symbol
true
string
Name of the transaction contract. No inquiries are allowed in the last 30 minutes before contract settlement
> side
true
string
Inquiry transaction direction:
Buy
,
Sell
> qty
true
string
If the number of transactions exceeds the position size, the position will then open in the reverse direction
Response Parameters
​
Parameter
Type
Comments
result
array
Order ID
list
array of objects
> rfqId
string
Inquiry ID
> rfqLinkId
string
Custom inquiry ID
> status
string
Status of the RFQ:
Active
Canceled
Filled
Expired
Failed
> expiresAt
string
The inquiry's expiration time (ms)
> deskCode
string
Inquiring party's unique identification code
Request Example
​
POST
/v5/rfq/create-rfq
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"counterparties"
:
[
"LP4"
,
"LP5"
]
,
"rfqLinkId"
:
"rfq00993"
,
"anonymous"
:
false
,
"strategyType"
:
"custom"
,
"list"
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
"buy"
,
"qty"
:
"2"
}
,
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"buy"
,
"qty"
:
"2"
}
,
{
"category"
:
"option"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"sell"
,
"qty"
:
"2"
}
]
}
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
"rfqId"
:
"17526315514105706281"
,
"rfqLinkId"
:
"rfq00993"
,
"status"
:
"Active"
,
"expiresAt"
:
"1752632151414"
,
"deskCode"
:
"LP2"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752631551419
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/create-rfq HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115{    "counterparties": ["LP4","LP5"],    "rfqLinkId":"rfq00993",    "anonymous": false,    "strategyType": "custom",    "list": [        {            "category": "linear",            "symbol": "BTCUSDT",            "side":"buy",            "qty":"2"        },        {            "category": "spot",            "symbol": "BTCUSDT",            "side":"buy",            "qty":"2"        },        {            "category": "option",            "symbol": "BTCUSDT",            "side":"sell",            "qty":"2"        }    ]}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "rfqId": "17526315514105706281",        "rfqLinkId": "rfq00993",        "status": "Active",        "expiresAt": "1752632151414",        "deskCode": "LP2"    },    "retExtInfo": {},    "time": 1752631551419}
```

---

## Trade Notify

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/trade-notify

**Contents:**
- Trade Notify
- Trade Notify​
  - Webhook URL​
  - Webhook Method​
  - Authentication​
  - Headers​
  - Request Body​

Asset
Fiat-Convert
Trade Notify
On this page
Trade Notify
Trade Notify
​
Webhook URL
​
Webhook_url
: Provided in the
trade-execute
API.
Webhook Method
​
HTTP Method
:
POST
Authentication
​
Share the
IP whitelist
with each other.
Headers
​
Content-Type
:
application/json
timestamp
:
xxx
publicKey
:
xxx
Request Body
​
The request body is in
JSON
format with the following fields:
Field Name
Type
Description
tradeNo
string
Trade order number
status
string
Trade status:
processing
,
success
, or
failed
quoteTxId
string
Quote transaction ID. System generated, used to confirm the quote
exchangeRate
string
Exchange rate
fromCoin
string
Convert from coin (coin to sell)
fromCoinType
string
Coin type of
fromCoin
, either
fiat
or
crypto
toCoin
string
Convert to coin (coin to buy)
toCoinType
string
Coin type of
toCoin
, either
fiat
or
crypto
fromAmount
string
From coin amount (amount to sell)
toAmount
string
To coin amount (amount to buy according to the exchange rate)
createdAt
string
Trade created time

**Examples:**

Example 1 ():
```
Content-Type: application/jsontimestamp: xxxpublicKey: xxx
```

Example 2 ():
```

```

---

## Get Borrow Contract Info

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/borrow-contract

**Contents:**
- Get Borrow Contract Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Borrow Contract Info
On this page
Get Borrow Contract Info
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-fixed/borrow-contract-info
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
loanId
false
string
Loan ID
orderCurrency
false
string
Loan coin name
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
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> annualRate
string
Annual rate for the borrowing
> autoRepay
string
Deprecated.
"true"
: enable auto repay, default;
"false"
: disable auto repay
> borrowCurrency
string
Loan coin
> borrowTime
string
Loan order timestamp
> interestPaid
string
Paid interest
> loanId
string
Loan contract ID
> orderId
string
Loan order ID
> repaymentTime
string
Time to repay
> residualPenaltyInterest
string
Unpaid interest
> residualPrincipal
string
Unpaid principal
> status
integer
Loan order status
1
: unrepaid;
2
: fully repaid;
3
: overdue
> term
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
> repayType
string
1
:Auto Repayment;
2
:Transfer to flexible loan;
0
: No Automatic Repayment. Compatible with existing orders;
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/borrow-contract-info?orderCurrency=ETH
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
1752652691909
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
get_borrowing_contract_info_fixed_crypto_loan
(
collateralCurrency
=
"ETH"
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
"0.022"
,
"autoRepay"
:
"true"
,
"borrowCurrency"
:
"ETH"
,
"borrowTime"
:
"1752633756068"
,
"interestPaid"
:
"0.002531506849315069"
,
"loanId"
:
"571"
,
"orderId"
:
"13007"
,
"repayType"
:
"1"
,
"repaymentTime"
:
"1755225756068"
,
"residualPenaltyInterest"
:
"0"
,
"residualPrincipal"
:
"1.4"
,
"status"
:
1
,
"term"
:
"30"
}
,
{
"annualRate"
:
"0.022"
,
"autoRepay"
:
"true"
,
"borrowCurrency"
:
"ETH"
,
"borrowTime"
:
"1752633696068"
,
"interestPaid"
:
"0.00018082191780822"
,
"loanId"
:
"570"
,
"orderId"
:
"13007"
,
"repayType"
:
"1"
,
"repaymentTime"
:
"1755225696068"
,
"residualPenaltyInterest"
:
"0"
,
"residualPrincipal"
:
"0.1"
,
"status"
:
1
,
"term"
:
"30"
}
]
,
"nextPageCursor"
:
"568"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752652692603
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/borrow-contract-info?orderCurrency=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752652691909X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_borrowing_contract_info_fixed_crypto_loan(    collateralCurrency="ETH",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "annualRate": "0.022",                "autoRepay": "true",                "borrowCurrency": "ETH",                "borrowTime": "1752633756068",                "interestPaid": "0.002531506849315069",                "loanId": "571",                "orderId": "13007",                "repayType": "1",                "repaymentTime": "1755225756068",                "residualPenaltyInterest": "0",                "residualPrincipal": "1.4",                "status": 1,                "term": "30"            },            {                "annualRate": "0.022",                "autoRepay": "true",                "borrowCurrency": "ETH",                "borrowTime": "1752633696068",                "interestPaid": "0.00018082191780822",                "loanId": "570",                "orderId": "13007",                "repayType": "1",                "repaymentTime": "1755225696068",                "residualPenaltyInterest": "0",                "residualPrincipal": "0.1",                "status": 1,                "term": "30"            }        ],        "nextPageCursor": "568"    },    "retExtInfo": {},    "time": 1752652692603}
```

---

## Amend Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/amend-order

**Contents:**
- Amend Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Amend Order
On this page
Amend Order
info
You can only modify
unfilled
or
partially filled
orders.
HTTP Request
​
POST
/v5/order/amend
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
spot
,
option
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
orderId
false
string
Order ID. Either
orderId
or
orderLinkId
is required
orderLinkId
false
string
User customised order ID. Either
orderId
or
orderLinkId
is required
orderIv
false
string
Implied volatility.
option
only
. Pass the real value, e.g for 10%, 0.1 should be passed
triggerPrice
false
string
For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:
triggerPrice > market price
Else,
triggerPrice < market price
For spot, it is the TP/SL and Conditional order trigger price
qty
false
string
Order quantity after modification. Do not pass it if not modify the qty
price
false
string
Order price after modification. Do not pass it if not modify the price
tpslMode
false
string
TP/SL mode
Full
: entire position for TP/SL. Then, tpOrderType or slOrderType must be
Market
Partial
: partial position tp/sl. Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is
required
and it must be
Partial
Valid for
linear
&
inverse
takeProfit
false
string
Take profit price after modification. If pass "0", it means cancel the existing take profit of the order. Do not pass it if you do not want to modify the take profit
stopLoss
false
string
Stop loss price after modification. If pass "0", it means cancel the existing stop loss of the order. Do not pass it if you do not want to modify the stop loss
tpTriggerBy
false
string
The price type to trigger take profit. When set a take profit, this param is
required
if no initial value for the order
slTriggerBy
false
string
The price type to trigger stop loss. When set a take profit, this param is
required
if no initial value for the order
triggerBy
false
string
Trigger price type
tpLimitPrice
false
string
Limit order price when take profit is triggered. Only working when original order sets partial limit tp/sl.
Option not supported
slLimitPrice
false
string
Limit order price when stop loss is triggered. Only working when original order sets partial limit tp/sl.
Option not supported`
info
The acknowledgement of an amend order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
RUN >>
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
User customised order ID
Request Example
​
HTTP
Python
Java
.Net
Node.js
POST
/v5/order/amend
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
1672217108106
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
"ETHPERP"
,
"orderLinkId"
:
"linear-004"
,
"triggerPrice"
:
"1145"
,
"qty"
:
"0.15"
,
"price"
:
"1050"
,
"takeProfit"
:
"0"
,
"stopLoss"
:
"0"
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
amend_order
(
category
=
"linear"
,
symbol
=
"ETHPERP"
,
orderLinkId
=
"linear-004"
,
triggerPrice
=
"1145"
,
qty
=
"0.15"
,
price
=
"1050"
,
takeProfit
=
"0"
,
stopLoss
=
"0"
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
restApi
.
BybitApiTradeRestClient
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
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
var
amendOrderRequest
=
TradeOrderRequest
.
builder
(
)
.
orderId
(
"1523347543495541248"
)
.
category
(
ProductType
.
LINEAR
)
.
symbol
(
"XRPUSDT"
)
.
price
(
"0.5"
)
// setting a new price, for example
.
qty
(
"15"
)
// and a new quantity
.
build
(
)
;
var
amendedOrder
=
client
.
amendOrder
(
amendOrderRequest
)
;
System
.
out
.
println
(
amendedOrder
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");
var orderInfoString = await TradeService.AmendOrder(orderId: "1523347543495541248", category:Category.LINEAR, symbol: "XRPUSDT", price:"0.5", qty:"15");
Console.WriteLine(orderInfoString);
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
amendOrder
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
orderLinkId
:
'linear-004'
,
triggerPrice
:
'1145'
,
qty
:
'0.15'
,
price
:
'1050'
,
takeProfit
:
'0'
,
stopLoss
:
'0'
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
"orderId"
:
"c6f055d9-7f21-4079-913d-e6523a9cfffa"
,
"orderLinkId"
:
"linear-004"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672217093461
}

**Examples:**

Example 1 ():
```
POST /v5/order/amend HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672217108106X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "ETHPERP",    "orderLinkId": "linear-004",    "triggerPrice": "1145",    "qty": "0.15",    "price": "1050",    "takeProfit": "0",    "stopLoss": "0"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.amend_order(    category="linear",    symbol="ETHPERP",    orderLinkId="linear-004",    triggerPrice="1145",    qty="0.15",    price="1050",    takeProfit="0",    stopLoss="0",))
```

Example 3 ():
```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var amendOrderRequest = TradeOrderRequest.builder().orderId("1523347543495541248").category(ProductType.LINEAR).symbol("XRPUSDT")                        .price("0.5")  // setting a new price, for example                        .qty("15")  // and a new quantity                        .build();var amendedOrder = client.amendOrder(amendOrderRequest);System.out.println(amendedOrder);
```

Example 4 ():
```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfoString = await TradeService.AmendOrder(orderId: "1523347543495541248", category:Category.LINEAR, symbol: "XRPUSDT", price:"0.5", qty:"15");Console.WriteLine(orderInfoString);
```

---

## Get Stake/Redeem Order History

**URL:** https://bybit-exchange.github.io/docs/v5/earn/order-history

**Contents:**
- Get Stake/Redeem Order History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Earn
Get Stake/Redeem Order History
On this page
Get Stake/Redeem Order History
info
API key needs "Earn" permission
HTTP Request
​
GET
/v5/earn/order
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
: currently, only flexible savings and OnChain is supported
orderId
false
string
Order ID.
For category =
OnChain
, either orderId or orderLinkId is
required
if both are passed, make sure they're matched, otherwise returning empty result
orderLinkId
false
string
Order link ID
Remarks
: Always return the latest one if order link id is ever reused when querying by orderLinkId only
productId
false
string
Product ID
startTime
false
integer
The start timestamp (ms).
1. If both are not provided, the default is to return data from the last 7 days.
2. If both are provided, the difference between the endTime and startTime must be less than or equal to 7 days.
endTime
false
integer
The endTime timestamp (ms)
limit
false
integer
Limit for data size per page. Range:
[1, 100]
. Default: 50
cursor
false
string
Cursor, use the returned
nextPageCursor
to query data for the next page.
Response Parameters
​
Parameter
Type
Comments
nextPageCursor
string
Refer to the
cursor
request parameter
list
array
Object
> coin
string
Coin name
> orderValue
string
amount
> orderType
string
Redeem
,
Stake
> orderId
string
Order ID
> orderLinkId
string
Order link ID
> status
string
Order status
Success
,
Fail
,
Pending
> createdAt
string
Order created time, in milliseconds
> productId
string
Product ID
> updatedAt
string
Order updated time, in milliseconds
> swapOrderValue
string
Swap order value. Only for LST Onchain.
> estimateRedeemTime
string
Estimate redeem time, in milliseconds. Only for Onchain
> estimateStakeTime
string
Estimate stake time, in milliseconds. Only for Onchain
Request Example
​
HTTP
Python
Node.js
GET
/v5/earn/order?orderId=9640dc23-df1a-448a-ad24-e1a48028a51f&category=OnChain
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
1739937044221
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
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
get_stake_or_redemption_history
(
category
=
"OnChain"
,
orderId
=
"9640dc23-df1a-448a-ad24-e1a48028a51f"
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
"coin"
:
"USDT"
,
"orderValue"
:
"1000"
,
"orderType"
:
"Stake"
,
"orderId"
:
"ad98d473-4e17-46da-ab30-5563f62a97fa"
,
"orderLinkId"
:
""
,
"status"
:
"Success"
,
"createdAt"
:
"1759983689000"
,
"productId"
:
"428"
,
"updatedAt"
:
"1759983689000"
,
"swapOrderValue"
:
""
,
"estimateRedeemTime"
:
""
,
"estimateStakeTime"
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
1759983699446
}

**Examples:**

Example 1 ():
```
GET /v5/earn/order?orderId=9640dc23-df1a-448a-ad24-e1a48028a51f&category=OnChain HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739937044221X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_stake_or_redemption_history(    category="OnChain",    orderId="9640dc23-df1a-448a-ad24-e1a48028a51f",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "coin": "USDT",                "orderValue": "1000",                "orderType": "Stake",                "orderId": "ad98d473-4e17-46da-ab30-5563f62a97fa",                "orderLinkId": "",                "status": "Success",                "createdAt": "1759983689000",                "productId": "428",                "updatedAt": "1759983689000",                "swapOrderValue": "",                "estimateRedeemTime": "",                "estimateStakeTime": ""            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1759983699446}
```

---

## Trade

**URL:** https://bybit-exchange.github.io/docs/v5/spread/websocket/public/trade

**Contents:**
- Trade
  - Response Parameters​
  - Subscribe Example​
  - Response Example​

Spread Trading
Websocket Stream
Public
Trade
On this page
Trade
Subscribe to the public trades stream.
After subscription, you will be pushed trade messages in real-time.
Push frequency:
real-time
Topic:
publicTrade.{symbol}
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
Object. Sorted by the time the trade was matched in ascending order
> T
number
The timestamp (ms) that the order is filled
> s
string
Symbol name
> S
string
Side of taker.
Buy
,
Sell
> v
string
Trade size
> p
string
Trade price
>
L
string
Direction of price change
> i
string
Trade ID
> seq
integer
Cross sequence
Subscribe Example
​
{
"op"
:
"subscribe"
,
"id"
:
"test-001-perp"
,
"args"
:
[
"publicTrade.SOLUSDT_SOL/USDT"
]
}
Response Example
​
{
"topic"
:
"publicTrade.SOLUSDT_SOL/USDT"
,
"ts"
:
1744170142723
,
"type"
:
"snapshot"
,
"data"
:
[
{
"T"
:
1744170142720
,
"s"
:
"SOLUSDT_SOL/USDT"
,
"S"
:
"Sell"
,
"v"
:
"2.5"
,
"p"
:
"19.3928"
,
"L"
:
"MinusTick"
,
"i"
:
"31d0fc58-933b-57b3-8378-f73da06da843"
,
"seq"
:
1783284617
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "id": "test-001-perp",    "args": [        "publicTrade.SOLUSDT_SOL/USDT"    ]}
```

Example 2 ():
```
{    "topic": "publicTrade.SOLUSDT_SOL/USDT",    "ts": 1744170142723,    "type": "snapshot",    "data": [        {            "T": 1744170142720,            "s": "SOLUSDT_SOL/USDT",            "S": "Sell",            "v": "2.5",            "p": "19.3928",            "L": "MinusTick",            "i": "31d0fc58-933b-57b3-8378-f73da06da843",            "seq": 1783284617        }    ]}
```

---

## Place Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/create-order

**Contents:**
- Place Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Place Order
On this page
Place Order
This endpoint supports to create the order for Spot, Margin trading, USDT perpetual, USDT futures, USDC perpetual, USDC futures, Inverse Futures and Options.
info
Supported order type (
orderType
):
Limit order:
orderType
=
Limit
, it is necessary to specify order qty and price.
Market order
:
orderType
=
Market
, execute at the best price in the Bybit market until the transaction is completed. When selecting a market order, the "price" can be empty. In the futures trading system, in order to protect traders against the serious slippage of the Market order, Bybit trading engine will convert the market order into an IOC limit order for matching. If there are no orderbook entries within price slippage limit, the order will not be executed. If there is insufficient liquidity, the order will be cancelled. The slippage threshold refers to the percentage that the order price deviates from the mark price. You can learn more here:
Adjustments to Bybit's Derivative Trading Price Limit Mechanism
Supported timeInForce strategy:
GTC
IOC
FOK
PostOnly
: If the order would be filled immediately when submitted, it will be
cancelled
. The purpose of this is to protect your order during the submission process. If the matching system cannot entrust the order to the order book due to price changes on the market, it will be cancelled.
RPI
: Retail Price Improvement order. Assigned market maker can place this kind of order, and it is a post only order, only match with the order from Web or APP.
How to create a conditional order:
When submitting an order, if
triggerPrice
is set, the order will be automatically converted into a conditional order. In addition, the conditional order does not occupy the margin. If the margin is insufficient after the conditional order is triggered, the order will be cancelled.
Take profit / Stop loss
: You can set TP/SL while placing orders. Besides, you could modify the position's TP/SL.
Order quantity
: The quantity of perpetual contracts you are going to buy/sell. For the order quantity, Bybit only supports positive number at present.
Order price
: Place a limit order, this parameter is
required
. If you have position, the price should be higher than the
liquidation price
.
For the minimum unit of the price change, please refer to the
priceFilter
>
tickSize
field in the
instruments-info
endpoint.
orderLinkId
: You can customize the active order ID. We can link this ID to the order ID in the system. Once the
active order is successfully created, we will send the unique order ID in the system to you. Then, you can use this order
ID to cancel active orders, and if both orderId and orderLinkId are entered in the parameter input, Bybit will prioritize the orderId to process the corresponding order. Meanwhile, your customized order ID should be no longer than 36 characters and should be
unique
.
Open orders up limit:
Perps & Futures:
a) Each account can hold a maximum of
500
active
orders simultaneously
per symbol.
b)
conditional
orders: each account can hold a maximum of
10 active orders
simultaneously
per symbol
.
Spot:
500 orders in total, including a maximum of 30 open TP/SL orders, a maximum of 30 open conditional orders for each symbol per account
Option:
a maximum of 50 open orders per account
Rate limit:
Please refer to
rate limit table
. If you need to raise the rate limit, please contact your client manager or submit an application via
here
Risk control limit notice:
Bybit will monitor on your API requests. When the total number of orders of a single user (aggregated the number of orders across main account and subaccounts) within a day (UTC 0 - UTC 24) exceeds a certain upper limit, the platform will reserve the right to remind, warn, and impose necessary restrictions.
Customers who use API default to acceptance of these terms and have the obligation to cooperate with adjustments.
Reduce only orders:
If reduceOnly=true and order qty > max order qty, the order will automatically be split up into multiple orders.
HTTP Request
​
POST
/v5/order/create
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
spot
,
option
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
isLeverage
false
integer
Whether to borrow.
0
(default): false, spot trading
1
: true, margin trading,
make sure you turn on margin trading, and set the relevant currency as collateral
side
true
string
Buy
,
Sell
orderType
true
string
Market
,
Limit
qty
true
string
Order quantity
Spot: Market Buy order by value by default, you can set
marketUnit
field to choose order by value or qty for market orders
Perps, Futures & Option: always order by qty
Perps & Futures: if you pass
qty
="0" and specify
reduceOnly
=true&
closeOnTrigger
=true, you can close the position up to
maxMktOrderQty
or
maxOrderQty
shown on
Get Instruments Info
of current symbol
marketUnit
false
string
Select the unit for
qty
when create
Spot market
orders
baseCoin
: for example, buy BTCUSDT, then "qty" unit is BTC
quoteCoin
: for example, sell BTCUSDT, then "qty" unit is USDT
slippageToleranceType
false
string
Slippage tolerance Type for
market order
,
TickSize
,
Percent
take profit, stoploss, conditional orders are not supported
TickSize
:
the highest price of Buy order = ask1 +
slippageTolerance
x tickSize;
the lowest price of Sell order = bid1 -
slippageTolerance
x tickSize
Percent
:
the highest price of Buy order = ask1 x (1 +
slippageTolerance
x 0.01);
the lowest price of Sell order = bid1 x (1 -
slippageTolerance
x 0.01)
slippageTolerance
false
string
Slippage tolerance value
TickSize
: range is
[1, 10000]
, integer only
Percent
: range is
[0.01, 10]
, up to 2 decimals
price
false
string
Order price
Market order will ignore this field
Please check the min price and price precision from
instrument info
endpoint
If you have position, price needs to be better than liquidation price
triggerDirection
false
integer
Conditional order param. Used to identify the expected direction of the conditional order.
1
: triggered when market price rises to
triggerPrice
2
: triggered when market price falls to
triggerPrice
Valid for
linear
&
inverse
orderFilter
false
string
If it is not passed,
Order
by default.
Order
tpslOrder
: Spot TP/SL order, the assets are occupied even before the order is triggered
StopOrder
: Spot conditional order, the assets will not be occupied until the price of the underlying asset reaches the trigger price, and the required assets will be occupied after the Conditional order is triggered
Valid for
spot
only
triggerPrice
false
string
For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:
triggerPrice > market price
Else,
triggerPrice < market price
For spot, it is the TP/SL and Conditional order trigger price
triggerBy
false
string
Trigger price type, Conditional order param for Perps & Futures.
LastPrice
IndexPrice
MarkPrice
Valid for
linear
&
inverse
orderIv
false
string
Implied volatility.
option
only
. Pass the real value, e.g for 10%, 0.1 should be passed.
orderIv
has a higher priority when
price
is passed as well
timeInForce
false
string
Time in force
Market order will always use
IOC
If not passed,
GTC
is used by default
positionIdx
false
integer
Used to identify positions in different position modes. Under hedge-mode, this param is
required
0
: one-way mode
1
: hedge-mode Buy side
2
: hedge-mode Sell side
orderLinkId
false
string
User customised order ID. A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.
Futures & Perps: orderLinkId rules
:
optional param
always unique
option
orderLinkId rules
:
required
param
always unique
takeProfit
false
string
Take profit price
Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order
stopLoss
false
string
Stop loss price
Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order
tpTriggerBy
false
string
The price type to trigger take profit.
MarkPrice
,
IndexPrice
, default:
LastPrice
. Valid for
linear
&
inverse
slTriggerBy
false
string
The price type to trigger stop loss.
MarkPrice
,
IndexPrice
, default:
LastPrice
. Valid for
linear
&
inverse
reduceOnly
false
boolean
What is a reduce-only order?
true
means your position can only reduce in size if this order is triggered.
You
must
specify it as
true
when you are about to close/reduce the position
When reduceOnly is true, take profit/stop loss cannot be set
Valid for
linear
,
inverse
&
option
closeOnTrigger
false
boolean
What is a close on trigger order?
For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
Valid for
linear
&
inverse
smpType
false
string
Smp execution type.
What is SMP?
mmp
false
boolean
Market maker protection.
option
only
.
true
means set the order as a market maker protection order.
What is mmp?
tpslMode
false
string
TP/SL mode
Full
: entire position for TP/SL. Then, tpOrderType or slOrderType must be
Market
Partial
: partial position tp/sl (as there is no size option, so it will create tp/sl orders with the qty you actually fill). Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is
required
and it must be
Partial
Valid for
linear
&
inverse
tpLimitPrice
false
string
The limit order price when take profit price is triggered
linear
&
inverse
: only works when tpslMode=Partial and tpOrderType=Limit
Spot: it is required when the order has
takeProfit
and "tpOrderType"=
Limit
slLimitPrice
false
string
The limit order price when stop loss price is triggered
linear
&
inverse
: only works when tpslMode=Partial and slOrderType=Limit
Spot: it is required when the order has
stopLoss
and "slOrderType"=
Limit
tpOrderType
false
string
The order type when take profit is triggered
linear
&
inverse
:
Market
(default),
Limit
. For tpslMode=Full, it only supports tpOrderType=Market
Spot:
Market
: when you set "takeProfit",
Limit
: when you set "takeProfit" and "tpLimitPrice"
slOrderType
false
string
The order type when stop loss is triggered
linear
&
inverse
:
Market
(default),
Limit
. For tpslMode=Full, it only supports slOrderType=Market
Spot:
Market
: when you set "stopLoss",
Limit
: when you set "stopLoss" and "slLimitPrice"
bboSideType
false
string
Queue
: use the order price on the orderbook in the same direction as the
side
Counterparty
: use the order price on the orderbook in the opposite direction as the
side
Valid for
linear
&
inverse
bboLevel
false
string
1
,
2
,
3
,
4
,
5
Valid for
linear
&
inverse
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
User customised order ID
info
The acknowledgement of an place order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
RUN >>
Request Example
​
HTTP
Python
Go
Java
.Net
Node.js
POST
/v5/order/create
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
1672211928338
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
// Spot Limit order with market tp sl
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.01"
,
"price"
:
"28000"
,
"timeInForce"
:
"PostOnly"
,
"takeProfit"
:
"35000"
,
"stopLoss"
:
"27000"
,
"tpOrderType"
:
"Market"
,
"slOrderType"
:
"Market"
}
// Spot Limit order with limit tp sl
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.01"
,
"price"
:
"28000"
,
"timeInForce"
:
"PostOnly"
,
"takeProfit"
:
"35000"
,
"stopLoss"
:
"27000"
,
"tpLimitPrice"
:
"36000"
,
"slLimitPrice"
:
"27500"
,
"tpOrderType"
:
"Limit"
,
"slOrderType"
:
"Limit"
}
// Spot PostOnly normal order
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"15600"
,
"timeInForce"
:
"PostOnly"
,
"orderLinkId"
:
"spot-test-01"
,
"isLeverage"
:
0
,
"orderFilter"
:
"Order"
}
// Spot TP/SL order
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"15600"
,
"triggerPrice"
:
"15000"
,
"timeInForce"
:
"Limit"
,
"orderLinkId"
:
"spot-test-02"
,
"isLeverage"
:
0
,
"orderFilter"
:
"tpslOrder"
}
// Spot margin normal order (UTA)
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"15600"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-test-limit"
,
"isLeverage"
:
1
,
"orderFilter"
:
"Order"
}
// Spot Market Buy order, qty is quote currency
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Market"
,
"qty"
:
"200"
,
"timeInForce"
:
"IOC"
,
"orderLinkId"
:
"spot-test-04"
,
"isLeverage"
:
0
,
"orderFilter"
:
"Order"
}
// USDT Perp open long position (one-way mode)
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
"orderType"
:
"Limit"
,
"qty"
:
"1"
,
"price"
:
"25000"
,
"timeInForce"
:
"GTC"
,
"positionIdx"
:
0
,
"orderLinkId"
:
"usdt-test-01"
,
"reduceOnly"
:
false
,
"takeProfit"
:
"28000"
,
"stopLoss"
:
"20000"
,
"tpslMode"
:
"Partial"
,
"tpOrderType"
:
"Limit"
,
"slOrderType"
:
"Limit"
,
"tpLimitPrice"
:
"27500"
,
"slLimitPrice"
:
"20500"
}
// USDT Perp close long position (one-way mode)
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
"Sell"
,
"orderType"
:
"Limit"
,
"qty"
:
"1"
,
"price"
:
"30000"
,
"timeInForce"
:
"GTC"
,
"positionIdx"
:
0
,
"orderLinkId"
:
"usdt-test-02"
,
"reduceOnly"
:
true
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
place_order
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
orderType
=
"Limit"
,
qty
=
"0.1"
,
price
=
"15600"
,
timeInForce
=
"PostOnly"
,
orderLinkId
=
"spot-test-postonly"
,
isLeverage
=
0
,
orderFilter
=
"Order"
,
)
)
import
(
"context"
"fmt"
bybit
"https://github.com/bybit-exchange/bybit.go.api"
)
client
:=
bybit
.
NewBybitHttpClient
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
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
"side"
:
"Buy"
,
"positionIdx"
:
0
,
"orderType"
:
"Limit"
,
"qty"
:
"0.001"
,
"price"
:
"10000"
,
"timeInForce"
:
"GTC"
,
}
client
.
NewUtaBybitServiceWithParams
(
params
)
.
PlaceOrder
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
restApi
.
BybitApiAsyncTradeRestClient
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
ProductType
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
TradeOrderType
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
PositionIdx
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
Side
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
TimeInForce
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
service
.
BybitApiClientFactory
;
import
java
.
util
.
Map
;
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
Map
<
String
,
Object
>
order
=
Map
.
of
(
"category"
,
"option"
,
"symbol"
,
"BTC-29DEC23-10000-P"
,
"side"
,
"Buy"
,
"orderType"
,
"Limit"
,
"orderIv"
,
"0.1"
,
"qty"
,
"0.1"
,
"price"
,
"5"
,
"orderLinkId"
,
"test_orderLinkId_1"
)
;
client
.
createOrder
(
order
,
System
.
out
::
println
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");
var orderInfo = await tradeService.PlaceOrder(category: Category.LINEAR, symbol: "BLZUSDT", side: Side.BUY, orderType: OrderType.MARKET, qty: "15", timeInForce: TimeInForce.GTC);
Console.WriteLine(orderInfo);
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
// Submit a market order
client
.
submitOrder
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
side
:
'Buy'
,
orderType
:
'Market'
,
qty
:
'1'
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
'Market order result'
,
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
'Market order error'
,
error
)
;
}
)
;
// Submit a limit order
client
.
submitOrder
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
side
:
'Buy'
,
orderType
:
'Limit'
,
qty
:
'1'
,
price
:
'55000'
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
'Limit order result'
,
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
'Limit order error'
,
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
"orderId"
:
"1321003749386327552"
,
"orderLinkId"
:
"spot-test-postonly"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672211918471
}

**Examples:**

Example 1 ():
```
POST /v5/order/create HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672211928338X-BAPI-RECV-WINDOW: 5000Content-Type: application/json// Spot Limit order with market tp sl{"category": "spot","symbol": "BTCUSDT","side": "Buy","orderType": "Limit","qty": "0.01","price": "28000","timeInForce": "PostOnly","takeProfit": "35000","stopLoss": "27000","tpOrderType": "Market","slOrderType": "Market"}// Spot Limit order with limit tp sl{"category": "spot","symbol": "BTCUSDT","side": "Buy","orderType": "Limit","qty": "0.01","price": "28000","timeInForce": "PostOnly","takeProfit": "35000","stopLoss": "27000","tpLimitPrice": "36000","slLimitPrice": "27500","tpOrderType": "Limit","slOrderType": "Limit"}// Spot PostOnly normal order{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","timeInForce":"PostOnly","orderLinkId":"spot-test-01","isLeverage":0,"orderFilter":"Order"}// Spot TP/SL order{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","triggerPrice": "15000", "timeInForce":"Limit","orderLinkId":"spot-test-02","isLeverage":0,"orderFilter":"tpslOrder"}// Spot margin normal order (UTA){"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","timeInForce":"GTC","orderLinkId":"spot-test-limit","isLeverage":1,"orderFilter":"Order"}// Spot Market Buy order, qty is quote currency{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Market","qty":"200","timeInForce":"IOC","orderLinkId":"spot-test-04","isLeverage":0,"orderFilter":"Order"}// USDT Perp open long position (one-way mode){"category":"linear","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"1","price":"25000","timeInForce":"GTC","positionIdx":0,"orderLinkId":"usdt-test-01","reduceOnly":false,"takeProfit":"28000","stopLoss":"20000","tpslMode":"Partial","tpOrderType":"Limit","slOrderType":"Limit","tpLimitPrice":"27500","slLimitPrice":"20500"}// USDT Perp close long position (one-way mode){"category": "linear", "symbol": "BTCUSDT", "side": "Sell", "orderType": "Limit", "qty": "1", "price": "30000", "timeInForce": "GTC", "positionIdx": 0, "orderLinkId": "usdt-test-02", "reduceOnly": true}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.place_order(    category="spot",    symbol="BTCUSDT",    side="Buy",    orderType="Limit",    qty="0.1",    price="15600",    timeInForce="PostOnly",    orderLinkId="spot-test-postonly",    isLeverage=0,    orderFilter="Order",))
```

Example 3 ():
```
import (    "context"    "fmt"    bybit "https://github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("YOUR_API_KEY", "YOUR_API_SECRET", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{        "category":    "linear",        "symbol":      "BTCUSDT",        "side":        "Buy",        "positionIdx": 0,        "orderType":   "Limit",        "qty":         "0.001",        "price":       "10000",        "timeInForce": "GTC",    }client.NewUtaBybitServiceWithParams(params).PlaceOrder(context.Background())
```

Example 4 ():
```
import com.bybit.api.client.restApi.BybitApiAsyncTradeRestClient;import com.bybit.api.client.domain.ProductType;import com.bybit.api.client.domain.TradeOrderType;import com.bybit.api.client.domain.trade.PositionIdx;import com.bybit.api.client.domain.trade.Side;import com.bybit.api.client.domain.trade.TimeInForce;import com.bybit.api.client.domain.trade.TradeOrderRequest;import com.bybit.api.client.service.BybitApiClientFactory;import java.util.Map;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();Map<String, Object> order =Map.of(                  "category", "option",                  "symbol", "BTC-29DEC23-10000-P",                  "side", "Buy",                  "orderType", "Limit",                  "orderIv", "0.1",                  "qty", "0.1",                  "price", "5",                  "orderLinkId", "test_orderLinkId_1"                );client.createOrder(order, System.out::println);
```

---

## Execution

**URL:** https://bybit-exchange.github.io/docs/v5/spread/websocket/private/execution

**Contents:**
- Execution
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

Spread Trading
Websocket Stream
Private
Execution
On this page
Execution
Topic:
spread.execution
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
<
object
>
> category
string
Combo or single leg,
combination
,
spot_leg
,
future_leg
> symbol
string
Combo or leg symbol name
> isLeverage
string
Account-wide, if Spot Margin is enabled, the spot_leg field in the execution message shows 1, combo is "", and future_leg is 0.
> orderId
string
Order ID, leg is ""
> orderLinkId
string
User customized order ID, leg is ""
> side
string
Side.
Buy
,
Sell
> orderPrice
string
Order price
> orderQty
string
Order qty
> leavesQty
string
The remaining qty not executed
>
createType
string
Order create type
> orderType
string
Order type.
Market
,
Limit
> execFee
string
Leg exec fee, deprecated for Spot leg
> execFeeV2
string
Leg exec fee, used for Spot leg only
> feeCoin
string
Leg fee currency
> parentExecId
string
Combo's Execution ID, leg's event has the value
> execId
string
Execution ID
> execPrice
string
Execution price
> execQty
string
Execution qty
> execPnl
string
Profit and Loss for each close position execution
>
execType
string
Executed type
> execValue
string
Executed order value
> execTime
string
Executed timestamp (ms)
> isMaker
boolean
Is maker order.
true
: maker,
false
: taker
> feeRate
string
Trading fee rate
> markPrice
string
The mark price of the symbol when executing
> closedSize
string
Closed position size
> seq
long
Cross sequence
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
"spread.execution"
]
}
Stream Example
​
// Combo execution
{
"topic"
:
"spread.execution"
,
"id"
:
"cvqes8141ilt347i9l20"
,
"creationTime"
:
1744104992226
,
"data"
:
[
{
"category"
:
"combination"
,
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"closedSize"
:
""
,
"execFee"
:
""
,
"execId"
:
"82c82077-0caa-5304-894d-58a50a342bd7"
,
"parentExecId"
:
""
,
"execPrice"
:
"20.9848"
,
"execQty"
:
"2"
,
"execType"
:
"Trade"
,
"execValue"
:
""
,
"feeRate"
:
""
,
"markPrice"
:
""
,
"leavesQty"
:
"0"
,
"orderId"
:
"5e010c35-2b44-4f03-8081-8fa31fb73376"
,
"orderLinkId"
:
""
,
"orderPrice"
:
"21"
,
"orderQty"
:
"2"
,
"orderType"
:
"Limit"
,
"side"
:
"Buy"
,
"execTime"
:
"1744104992220"
,
"isLeverage"
:
""
,
"isMaker"
:
false
,
"seq"
:
241321
,
"createType"
:
"CreateByUser"
,
"execPnl"
:
""
}
]
}
//Future leg execution
{
"topic"
:
"spread.execution"
,
"id"
:
"1448939_SOLUSDT_28731107101"
,
"creationTime"
:
1744104992229
,
"data"
:
[
{
"category"
:
"future_leg"
,
"symbol"
:
"SOLUSDT"
,
"closedSize"
:
"0"
,
"execFee"
:
"0.039712"
,
"execId"
:
"99a18f80-d3b5-4c6f-a1f1-8c5870e3f3bc"
,
"parentExecId"
:
"82c82077-0caa-5304-894d-58a50a342bd7"
,
"execPrice"
:
"124.1"
,
"execQty"
:
"2"
,
"execType"
:
"FutureSpread"
,
"execValue"
:
"248.2"
,
"feeRate"
:
"0.00016"
,
"markPrice"
:
"119"
,
"leavesQty"
:
"0"
,
"orderId"
:
""
,
"orderLinkId"
:
""
,
"orderPrice"
:
"124.1"
,
"orderQty"
:
"2"
,
"orderType"
:
"Limit"
,
"side"
:
"Buy"
,
"execTime"
:
"1744104992224"
,
"isLeverage"
:
"0"
,
"isMaker"
:
false
,
"seq"
:
28731107101
,
"createType"
:
"CreateByFutureSpread"
,
"execPnl"
:
"0"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "spread.execution"    ]}
```

Example 2 ():
```
// Combo execution{     "topic": "spread.execution",     "id": "cvqes8141ilt347i9l20",     "creationTime": 1744104992226,     "data": [          {               "category": "combination",               "symbol": "SOLUSDT_SOL/USDT",               "closedSize": "",               "execFee": "",               "execId": "82c82077-0caa-5304-894d-58a50a342bd7",               "parentExecId": "",               "execPrice": "20.9848",               "execQty": "2",               "execType": "Trade",               "execValue": "",               "feeRate": "",               "markPrice": "",               "leavesQty": "0",               "orderId": "5e010c35-2b44-4f03-8081-8fa31fb73376",               "orderLinkId": "",               "orderPrice": "21",               "orderQty": "2",               "orderType": "Limit",               "side": "Buy",               "execTime": "1744104992220",               "isLeverage": "",               "isMaker": false,               "seq": 241321,               "createType": "CreateByUser",               "execPnl": ""          }     ]}//Future leg execution{     "topic": "spread.execution",     "id": "1448939_SOLUSDT_28731107101",     "creationTime": 1744104992229,     "data": [          {               "category": "future_leg",               "symbol": "SOLUSDT",               "closedSize": "0",               "execFee": "0.039712",               "execId": "99a18f80-d3b5-4c6f-a1f1-8c5870e3f3bc",               "parentExecId": "82c82077-0caa-5304-894d-58a50a342bd7",               "execPrice": "124.1",               "execQty": "2",               "execType": "FutureSpread",               "execValue": "248.2",               "feeRate": "0.00016",               "markPrice": "119",               "leavesQty": "0",               "orderId": "",               "orderLinkId": "",               "orderPrice": "124.1",               "orderQty": "2",               "orderType": "Limit",               "side": "Buy",               "execTime": "1744104992224",               "isLeverage": "0",               "isMaker": false,               "seq": 28731107101,               "createType": "CreateByFutureSpread",               "execPnl": "0"          }     ]}
```

---

## Cancel Order

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/cancel-order

**Contents:**
- Cancel Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Cancel Order
On this page
Cancel Order
HTTP Request
​
POST
/v5/spread/order/cancel
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Spread combination order ID. Either
orderId
or
orderLinkId
is
required
orderLinkId
false
string
User customised order ID. Either
orderId
or
orderLinkId
is
required
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
User customised order ID
info
The acknowledgement of an cancel order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
Request Example
​
POST
/v5/spread/order/cancel
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXXXX
X-BAPI-API-KEY
:
XXXXXXX
X-BAPI-TIMESTAMP
:
1744090699418
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
48
{
"orderLinkId"
:
"1744072052193428476"
}
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
"orderId"
:
"4496253b-b55b-4407-8c5c-29629d169caf"
,
"orderLinkId"
:
"1744072052193428476"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1744090702715
}

**Examples:**

Example 1 ():
```
POST /v5/spread/order/cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: XXXXXXXX-BAPI-TIMESTAMP: 1744090699418X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 48{    "orderLinkId": "1744072052193428476"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "4496253b-b55b-4407-8c5c-29629d169caf",        "orderLinkId": "1744072052193428476"    },    "retExtInfo": {},    "time": 1744090702715}
```

---

## Set Limit Price Behaviour

**URL:** https://bybit-exchange.github.io/docs/v5/account/set-price-limit

**Contents:**
- Set Limit Price Behaviour
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Set Limit Price Behaviour
On this page
Set Limit Price Behaviour
You can configure how the system behaves when your limit order price exceeds the highest bid or lowest ask price.
You can query configuration by
Get Trade Behaviour Setting
.
Spot
Maximum Buy Price
:
Min
[Max(Index, Index × (1 + y%) + 2-Minute Average Premium), Index × (1 + z%)]
Lowest price for Sell
:
Max
[Min(Index, Index × (1 – y%) + 2-Minute Average Premium), Index × (1 – z%)]
Futures
Maximum Buy Price
:
min( max( index , markprice
( 1 + x% ）), markprice
( 1 + y%) )
Lowest price for Sell
:
max ( min( index , markprice
( 1 - x% )) , markprice ( 1 - y%) )
Default Setting
Spot:
modifyEnable = false.
If the order price exceeds the boundary, the system rejects the request.
Corresponding to
Get Limit Price Behaviour
that
lpaSpot = false , lpaPerp = true
Futures:
modifyEnable = true.
If the order price exceeds the boundary, the system will automatically adjust the price to the nearest allowed boundary (i.e., highest bid or lowest ask).
Corresponding to
Get Limit Price Behaviour
that
lpaSpot = true , lpaPerp = false
Setting either
linear
or
inverse
will set behaviour for
all futures
.
HTTP Request
​
POST
/v5/account/set-limit-px-action
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
linear
,
inverse
,
spot
modifyEnable
true
boolean
true
: allow the syetem to modify the order price
false
: reject your order request
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/account/set-limit-px-action
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
1753255927950
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
52
{
"category"
:
"spot"
,
"modifyEnable"
:
true
}
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1753255927952
}

**Examples:**

Example 1 ():
```
POST /v5/account/set-limit-px-action HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1753255927950X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52{    "category": "spot",    "modifyEnable": true}
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {},    "retExtInfo": {},    "time": 1753255927952}
```

---

## Toggle Margin Trade

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/switch-mode

**Contents:**
- Toggle Margin Trade
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Toggle Margin Trade
On this page
Toggle Margin Trade
Turn on / off spot margin trade
caution
Your account needs to activate spot margin first; i.e., you must have finished the quiz on web / app.
HTTP Request
​
POST
/v5/spot-margin-trade/switch-mode
Request Parameters
​
Parameter
Required
Type
Comments
spotMarginMode
true
string
1
: on,
0
: off
Response Parameters
​
Parameter
Type
Comments
spotMarginMode
string
Spot margin status.
1
: on,
0
: off
RUN >>
Request Example
​
HTTP
Python
Node.js
POST
/v5/spot-margin-trade/switch-mode
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
1672297794480
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"spotMarginMode"
:
"0"
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
spot_margin_trade_toggle_margin_trade
(
spotMarginMode
=
"0"
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
toggleSpotMarginTrade
(
'0'
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
"spotMarginMode"
:
"0"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672297795542
}

**Examples:**

Example 1 ():
```
POST /v5/spot-margin-trade/switch-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672297794480X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "spotMarginMode": "0"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.spot_margin_trade_toggle_margin_trade(    spotMarginMode="0",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .toggleSpotMarginTrade('0')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "spotMarginMode": "0"    },    "retExtInfo": {},    "time": 1672297795542}
```

---

## Cancel All Orders

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/cancel-all

**Contents:**
- Cancel All Orders
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Cancel All Orders
On this page
Cancel All Orders
Cancel all open orders
HTTP Request
​
POST
/v5/spread/order/cancel-all
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
When a symbol is specified, all orders for that symbol will be cancelled regardless of the
cancelAll
field.
When a symbol is not specified and
cancelAll
=true, all orders, regardless of the symbol, will be cancelled
cancelAll
false
boolean
true
,
false
info
The acknowledgement of cancel all orders request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
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
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
success
string
The field can be ignored
Request Example
​
POST
/v5/spread/order/cancel-all
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
1744090967121
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
49
{
"symbol"
:
null
,
"cancelAll"
:
true
}
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
"orderId"
:
"11ec47f3-f0a2-4b2a-b302-236f2a2d53a2"
,
"orderLinkId"
:
""
}
]
,
"success"
:
"1"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1744090940933
}

**Examples:**

Example 1 ():
```
POST /v5/spread/order/cancel-all HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744090967121X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 49{    "symbol": null,    "cancelAll": true}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "orderId": "11ec47f3-f0a2-4b2a-b302-236f2a2d53a2",                "orderLinkId": ""            }        ],        "success": "1"    },    "retExtInfo": {},    "time": 1744090940933}
```

---

## Create Quote

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/create-quote

**Contents:**
- Create Quote
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Create Quote
On this page
Create Quote
Create a quote.
Up to 50 requests
per second. The quoting party sends a quote in response to the inquirier.
info
Only support UTA2.0 accounts
Cannot quote for your own inquiry
One request reports in two directions
You must pass at least one quoteBuyList and quoteSellList
If you would like to quote a spot quote, please ensure the corresponding collateral asset is enabled using
Set Collateral Coin
or
Batch Set Collateral Coin
HTTP Request
​
POST
/v5/rfq/create-quote
Request Parameters
​
Parameter
Required
Type
Comments
rfqId
true
string
Inquiry ID
quoteLinkId
false
string
Custom quote ID:
The length should be between 1-32 bits
Combination of letters (case sensitive) and numbers
An rfqLinkId expires after three months – after which it can be reused
Open orders must have a unique ID whereas orders that have reached a final/terminated status do not have to be unique.
anonymous
false
boolean
Whether or not it is anonymous quote. The default value is
false
. When it is
true
the identity of the quoting party will not be revealed even after the transaction is concluded.
expireIn
false
integer
Duration of the quote (in secs).
[
10
,
120
]
. Default:
60
quoteBuyList
false
array of objects
Quote direction
In the
Buy
direction, for the maker (the quoting party), the execution direction is the same as the direction of the legs
For the taker (the inquiring party) it is opposite direction
> category
true
string
Product type: Unified account:
spot
,
linear
,
option
> symbol
true
string
Name of the trading contract
> price
true
string
Quote price
quoteSellList
false
array of objects
Ask direction
In the
Sell
direction, for the maker (the quoting party), the execution direction is opposite to the direction of the legs
For the taker (the inquiring party) it is the same direction
> category
true
string
Product type: Unified account:
spot
,
linear
,
option
> symbol
true
string
Name of the trading contract
> price
true
string
Quote price
Response Parameters
​
Parameter
Type
Comments
result
object
> rfqId
string
Inquiry ID
> quoteId
string
Quote ID
> quoteLinkId
string
Custom quote ID
> expiresAt
string
The quote's expiration time (ms)
> deskCode
string
Quoter's unique identification code
> status
string
Status of quotation:
Active
Canceled
Filled
Expired
Failed
Request Example
​
POST
/v5/rfq/create-quote
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"rfqId"
:
"1754364447601610516653123084412812"
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
"106000"
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
"126500"
}
]
}
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
"deskCode"
:
"test0904"
,
"status"
:
"Active"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1757405933132
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/create-quote HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115{  "rfqId":"1754364447601610516653123084412812",   "quoteBuyList": [        {            "category": "linear",            "symbol": "BTCUSDT",            "price": "106000"        }    ],    "quoteSellList":[        {            "category": "linear",            "symbol": "BTCUSDT",            "price": "126500"        }    ]}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "rfqId": "175740578143743543930777169307022",        "quoteId": "1757405933130044334361923221559805",        "quoteLinkId": "",        "expiresAt": "1757405993126",        "deskCode": "test0904",        "status": "Active"    },    "retExtInfo": {},    "time": 1757405933132}
```

---

## Order

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/order

**Contents:**
- Order
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

WebSocket Stream
Private
Order
On this page
Order
Subscribe to the order stream to see changes to your orders in
real-time
.
All-In-One Topic:
order
Categorised Topic:
order.spot
,
order.linear
,
order.inverse
,
order.option
info
All-In-One topic and Categorised topic
cannot
be in the same subscription request
All-In-One topic: Allow you to listen to all categories (spot, linear, inverse, option) websocket updates
Categorised Topic: Allow you to listen only to specific category websocket updates
tip
You may receive two orderStatus=
Filled
messages when the cancel request is accepted but the order is executed at the same time. Generally, one
message contains "orderStatus=Filled, rejectReason=EC_NoError", and another message contains "orderStatus=Filled, cancelType=CancelByUser, rejectReason=EC_OrigClOrdIDDoesNotExist".
The first message tells you the order is executed, and the second message tells you the followed cancel request is rejected due to order is executed.
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
> category
string
Product type
spot
,
linear
,
inverse
,
option
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
> parentOrderLinkId
string
Indicates the linked parent order for attached take-profit and stop-loss orders. Supported for futures and options.
Amending
take-profit or stop-loss orders does not change the parentOrderLinkId
Futures
: using
set trading stop
to update attached TP/SL from the original order does not change the parentOrderLinkId.
Options
: using
set trading stop
to update attached TP/SL from the original order will change the parentOrderLinkId.
Futures & Options
: if TP/SL is set via
set trading stop
for a position that originally has no attached TP/SL, the parentOrderLinkId is meaningless.
> isLeverage
string
Whether to borrow.
0
: false,
1
: true
> blockTradeId
string
Block trade ID
> symbol
string
Symbol name
> price
string
Order price
> brokerOrderPrice
string
Dedicated field for EU liquidity provider
> qty
string
Order qty
> side
string
Side.
Buy
,
Sell
>
positionIdx
integer
Position index. Used to identify positions in different position modes
>
orderStatus
string
Order status
>
createType
string
Order create type, Spot, Option do not have this key
>
cancelType
string
Cancel type
>
rejectReason
string
Reject reason
> avgPrice
string
Average filled price, returns
""
for those orders without avg price
> leavesQty
string
The remaining qty not executed
> leavesValue
string
The remaining value not executed
> cumExecQty
string
Cumulative executed order qty
> cumExecValue
string
Cumulative executed order value
> cumExecFee
string
inverse
,
option
: Cumulative executed trading fee.
linear
,
spot
: Deprecated. Use
cumFeeDetail
instead.
After upgraded to the Unified account, you can use
execFee
for each fill in
Execution
topic
> closedPnl
string
Closed profit and loss for each close position order. The figure is the same as "closedPnl" from
Get Closed PnL
> feeCurrency
string
Deprecated. Trading fee currency for Spot only. Please understand Spot trading fee currency
here
>
timeInForce
string
Time in force
>
orderType
string
Order type.
Market
,
Limit
. For TP/SL orders, is the order type after the order was triggered
>
stopOrderType
string
Stop order type
> ocoTriggerBy
string
The trigger type of Spot OCO order.
OcoTriggerByUnknown
,
OcoTriggerByTp
,
OcoTriggerBySl
> orderIv
string
Implied volatility
> marketUnit
string
The unit for
qty
when create
Spot market
orders.
baseCoin
,
quoteCoin
> slippageToleranceType
string
Spot and Futures market order slippage tolerance type
TickSize
,
Percent
,
UNKNOWN
(default)
> slippageTolerance
string
Slippage tolerance value
> triggerPrice
string
Trigger price. If
stopOrderType
=
TrailingStop
, it is activate price. Otherwise, it is trigger price
> takeProfit
string
Take profit price
> stopLoss
string
Stop loss price
> tpslMode
string
TP/SL mode,
Full
: entire position for TP/SL.
Partial
: partial position tp/sl. Spot does not have this field, and Option returns always ""
> tpLimitPrice
string
The limit order price when take profit price is triggered
> slLimitPrice
string
The limit order price when stop loss price is triggered
>
tpTriggerBy
string
The price type to trigger take profit
>
slTriggerBy
string
The price type to trigger stop loss
> triggerDirection
integer
Trigger direction.
1
: rise,
2
: fall
>
triggerBy
string
The price type of trigger price
> lastPriceOnCreated
string
Last price when place the order
> reduceOnly
boolean
Reduce only.
true
means reduce position size
> closeOnTrigger
boolean
Close on trigger.
What is a close on trigger order?
> placeType
string
Place type,
option
used.
iv
,
price
>
smpType
string
SMP execution type
> smpGroup
integer
Smp group ID. If the UID has no group, it is
0
by default
> smpOrderId
string
The counterparty's orderID which triggers this SMP execution
> createdTime
string
Order created timestamp (ms)
> updatedTime
string
Order updated timestamp (ms)
> cumFeeDetail
json
linear
,
spot
: Cumulative trading fee details instead of
cumExecFee
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
"order"
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
order_stream
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
"5923240c6880ab-c59f-420b-9adb-3639adc9dd90"
,
"topic"
:
"order"
,
"creationTime"
:
1672364262474
,
"data"
:
[
{
"symbol"
:
"ETH-30DEC22-1400-C"
,
"orderId"
:
"5cf98598-39a7-459e-97bf-76ca765ee020"
,
"side"
:
"Sell"
,
"orderType"
:
"Market"
,
"cancelType"
:
"UNKNOWN"
,
"price"
:
"72.5"
,
"qty"
:
"1"
,
"orderIv"
:
""
,
"timeInForce"
:
"IOC"
,
"orderStatus"
:
"Filled"
,
"orderLinkId"
:
""
,
"lastPriceOnCreated"
:
""
,
"reduceOnly"
:
false
,
"leavesQty"
:
""
,
"leavesValue"
:
""
,
"cumExecQty"
:
"1"
,
"cumExecValue"
:
"75"
,
"avgPrice"
:
"75"
,
"blockTradeId"
:
""
,
"positionIdx"
:
0
,
"cumExecFee"
:
"0.358635"
,
"closedPnl"
:
"0"
,
"createdTime"
:
"1672364262444"
,
"updatedTime"
:
"1672364262457"
,
"rejectReason"
:
"EC_NoError"
,
"stopOrderType"
:
""
,
"tpslMode"
:
""
,
"triggerPrice"
:
""
,
"takeProfit"
:
""
,
"stopLoss"
:
""
,
"tpTriggerBy"
:
""
,
"slTriggerBy"
:
""
,
"tpLimitPrice"
:
""
,
"slLimitPrice"
:
""
,
"triggerDirection"
:
0
,
"triggerBy"
:
""
,
"closeOnTrigger"
:
false
,
"category"
:
"option"
,
"placeType"
:
"price"
,
"smpType"
:
"None"
,
"smpGroup"
:
0
,
"smpOrderId"
:
""
,
"feeCurrency"
:
""
,
"cumFeeDetail"
:
{
"MNT"
:
"0.00242968"
}
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "order"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)def handle_message(message):    print(message)ws.order_stream(callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "id": "5923240c6880ab-c59f-420b-9adb-3639adc9dd90",    "topic": "order",    "creationTime": 1672364262474,    "data": [        {            "symbol": "ETH-30DEC22-1400-C",            "orderId": "5cf98598-39a7-459e-97bf-76ca765ee020",            "side": "Sell",            "orderType": "Market",            "cancelType": "UNKNOWN",            "price": "72.5",            "qty": "1",            "orderIv": "",            "timeInForce": "IOC",            "orderStatus": "Filled",            "orderLinkId": "",            "lastPriceOnCreated": "",            "reduceOnly": false,            "leavesQty": "",            "leavesValue": "",            "cumExecQty": "1",            "cumExecValue": "75",            "avgPrice": "75",            "blockTradeId": "",            "positionIdx": 0,            "cumExecFee": "0.358635",            "closedPnl": "0",            "createdTime": "1672364262444",            "updatedTime": "1672364262457",            "rejectReason": "EC_NoError",            "stopOrderType": "",            "tpslMode": "",            "triggerPrice": "",            "takeProfit": "",            "stopLoss": "",            "tpTriggerBy": "",            "slTriggerBy": "",            "tpLimitPrice": "",            "slLimitPrice": "",            "triggerDirection": 0,            "triggerBy": "",            "closeOnTrigger": false,            "category": "option",            "placeType": "price",            "smpType": "None",            "smpGroup": 0,            "smpOrderId": "",            "feeCurrency": "",            "cumFeeDetail": {                "MNT": "0.00242968"            }        }    ]}
```

---

## Get Order History

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/order-history

**Contents:**
- Get Order History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Get Order History
On this page
Get Order History
info
orderId & orderLinkId has a higher priority than startTime & endTime
Fully cancelled orders are stored for up to 24 hours.
Single leg orders can also be found with "createType"=
CreateByFutureSpread
via
Get Order History
HTTP Request
​
GET
/v5/spread/order/history
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
startTime
false
long
The start timestamp (ms)
startTime and endTime are not passed, return 7 days by default
Only startTime is passed, return range between startTime and startTime+7 days
Only endTime is passed, return range between endTime-7 days and endTime
If both are passed, the rule is endTime - startTime <= 7 days
endTime
false
long
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
<
object
>
Order info
> symbol
string
Spread combination symbol name
> orderType
string
Order type,
Market
,
Limit
> orderLinkId
string
User customised order ID
> orderId
string
Spread combination order ID
> contractType
string
Combo type
FundingRateArb
: perpetual & spot combination
CarryTrade
: futures & spot combination
FutureSpread
: different expiry futures combination
PerpBasis
: futures & perpetual
>
cxlRejReason
string
Reject reason
>
orderStatus
string
Order status,
Rejected
,
Cancelled
,
Filled
> price
string
Order price
> orderQty
string
Order qty
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
> baseCoin
string
Base coin
> createdAt
string
Order created timestamp (ms)
> updatedAt
string
Order updated timestamp (ms)
> side
string
Side,
Buy
,
Sell
> leavesQty
string
The remaining qty not executed. It is meaningless for a cancelled order
> settleCoin
string
Settle coin
> cumExecQty
string
Cumulative executed order qty
> qty
string
Order qty
> leg1Symbol
string
Leg1 symbol name
> leg1ProdType
string
Leg1 product type,
Futures
,
Spot
> leg1OrderId
string
Leg1 order ID
> leg1Side
string
Leg1 order side
> leg2ProdType
string
Leg2 product type,
Futures
,
Spot
> leg2OrderId
string
Leg2 order ID
> leg2Symbol
string
Leg2 symbol name
> leg2Side
string
Leg2 orde side
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
GET
/v5/spread/order/history?orderId=aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4
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
1744100522465
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
"Success"
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
"orderLinkId"
:
""
,
"orderId"
:
"aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4"
,
"contractType"
:
"FundingRateArb"
,
"orderStatus"
:
"Cancelled"
,
"createdAt"
:
"1744096099767"
,
"price"
:
"-4"
,
"leg2Symbol"
:
"SOLUSDT"
,
"orderQty"
:
"0.1"
,
"timeInForce"
:
"PostOnly"
,
"baseCoin"
:
"SOL"
,
"updatedAt"
:
"1744098396079"
,
"side"
:
"Buy"
,
"leg2Side"
:
"Sell"
,
"leavesQty"
:
"0"
,
"leg1Side"
:
"Buy"
,
"settleCoin"
:
"USDT"
,
"cumExecQty"
:
"0"
,
"qty"
:
"0.1"
,
"leg1OrderId"
:
"82335b0a-b7d9-4ea5-9230-e71271a65100"
,
"leg2OrderId"
:
"1924011967786517249"
,
"leg2ProdType"
:
"Spot"
,
"leg1ProdType"
:
"Futures"
,
"leg1Symbol"
:
"SOLUSDT"
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
1744102655725
}

**Examples:**

Example 1 ():
```
GET /v5/spread/order/history?orderId=aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744100522465X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "nextPageCursor": "aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4%3A1744096099767%2Caaaee090-fab3-42ea-aea0-c9fbfe6c4bc4%3A1744096099767",        "list": [            {                "symbol": "SOLUSDT_SOL/USDT",                "orderType": "Limit",                "orderLinkId": "",                "orderId": "aaaee090-fab3-42ea-aea0-c9fbfe6c4bc4",                "contractType": "FundingRateArb",                "orderStatus": "Cancelled",                "createdAt": "1744096099767",                "price": "-4",                "leg2Symbol": "SOLUSDT",                "orderQty": "0.1",                "timeInForce": "PostOnly",                "baseCoin": "SOL",                "updatedAt": "1744098396079",                "side": "Buy",                "leg2Side": "Sell",                "leavesQty": "0",                "leg1Side": "Buy",                "settleCoin": "USDT",                "cumExecQty": "0",                "qty": "0.1",                "leg1OrderId": "82335b0a-b7d9-4ea5-9230-e71271a65100",                "leg2OrderId": "1924011967786517249",                "leg2ProdType": "Spot",                "leg1ProdType": "Futures",                "leg1Symbol": "SOLUSDT"            }        ]    },    "retExtInfo": {},    "time": 1744102655725}
```

---

## Pre Check Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/pre-check-order

**Contents:**
- Pre Check Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Pre Check Order
On this page
Pre Check Order
This endpoint is used to calculate the changes in IMR and MMR of UTA account before and after placing an order.
info
This endpoint supports orders with category =
inverse
,
linear
,
option
.
Only Cross Margin mode and Portfolio Margin mode are supported, isolated margin mode is not supported.
category =
inverse
is not supported in Cross Margin mode.
Conditional order is not supported.
If
retCode
is neither 0 nor 110007,
result
will return an empty json.
future_order_id
,
future_order_link_id
will be displayed in the
retExtInfo
json.
If
retCode
is 110007,
result
will return an empty json.
future_order_id
,
future_order_link_id
,
post_imr_e4
, and
post_mmr_e4
will be displayed in the
retExtInfo
json.
HTTP Request
​
POST
/v5/order/pre-check
Request Parameters
​
refer to
create order request
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
User customised order ID
preImrE4
int
Initial margin rate before checking, keep four decimal places. For examples, 30 means IMR = 30/1e4 = 0.30%
preMmrE4
int
Maintenance margin rate before checking, keep four decimal places. For examples, 30 means MMR = 30/1e4 = 0.30%
postImrE4
int
Initial margin rate calculated after checking, keep four decimal places. For examples, 30 means IMR = 30/1e4 = 0.30%
postMmrE4
int
Maintenance margin rate calculated after checking, keep four decimal places. For examples, 30 means MMR = 30/1e4 = 0.30%
Request Example
​
HTTP
Python
Node.js
POST
/v5/order/pre-check
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
1672211928338
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
// Spot Limit order with market tp sl
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.01"
,
"price"
:
"28000"
,
"timeInForce"
:
"PostOnly"
,
"takeProfit"
:
"35000"
,
"stopLoss"
:
"27000"
,
"tpOrderType"
:
"Market"
,
"slOrderType"
:
"Market"
}
// Spot Limit order with limit tp sl
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.01"
,
"price"
:
"28000"
,
"timeInForce"
:
"PostOnly"
,
"takeProfit"
:
"35000"
,
"stopLoss"
:
"27000"
,
"tpLimitPrice"
:
"36000"
,
"slLimitPrice"
:
"27500"
,
"tpOrderType"
:
"Limit"
,
"slOrderType"
:
"Limit"
}
// Spot PostOnly normal order
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"15600"
,
"timeInForce"
:
"PostOnly"
,
"orderLinkId"
:
"spot-test-01"
,
"isLeverage"
:
0
,
"orderFilter"
:
"Order"
}
// Spot TP/SL order
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"15600"
,
"triggerPrice"
:
"15000"
,
"timeInForce"
:
"Limit"
,
"orderLinkId"
:
"spot-test-02"
,
"isLeverage"
:
0
,
"orderFilter"
:
"tpslOrder"
}
// Spot margin normal order (UTA)
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"15600"
,
"timeInForce"
:
"GTC"
,
"orderLinkId"
:
"spot-test-limit"
,
"isLeverage"
:
1
,
"orderFilter"
:
"Order"
}
// Spot Market Buy order, qty is quote currency
{
"category"
:
"spot"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Market"
,
"qty"
:
"200"
,
"timeInForce"
:
"IOC"
,
"orderLinkId"
:
"spot-test-04"
,
"isLeverage"
:
0
,
"orderFilter"
:
"Order"
}
// USDT Perp open long position (one-way mode)
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
"orderType"
:
"Limit"
,
"qty"
:
"1"
,
"price"
:
"25000"
,
"timeInForce"
:
"GTC"
,
"positionIdx"
:
0
,
"orderLinkId"
:
"usdt-test-01"
,
"reduceOnly"
:
false
,
"takeProfit"
:
"28000"
,
"stopLoss"
:
"20000"
,
"tpslMode"
:
"Partial"
,
"tpOrderType"
:
"Limit"
,
"slOrderType"
:
"Limit"
,
"tpLimitPrice"
:
"27500"
,
"slLimitPrice"
:
"20500"
}
// USDT Perp close long position (one-way mode)
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
"Sell"
,
"orderType"
:
"Limit"
,
"qty"
:
"1"
,
"price"
:
"30000"
,
"timeInForce"
:
"GTC"
,
"positionIdx"
:
0
,
"orderLinkId"
:
"usdt-test-02"
,
"reduceOnly"
:
true
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
pre_check_order
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
orderType
=
"Limit"
,
qty
=
"0.1"
,
price
=
"28000"
,
timeInForce
=
"PostOnly"
,
takeProfit
=
"35000"
,
stopLoss
=
"27000"
,
tpOrderType
=
"Market"
,
slOrderType
=
"Market"
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
"orderId"
:
"24920bdb-4019-4e37-ad1c-876e3a855ac3"
,
"orderLinkId"
:
"test129"
,
"preImrE4"
:
30
,
"preMmrE4"
:
21
,
"postImrE4"
:
357
,
"postMmrE4"
:
294
}
,
"retExtInfo"
:
{
}
,
"time"
:
1749541599589
}

**Examples:**

Example 1 ():
```
POST /v5/order/pre-check HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672211928338X-BAPI-RECV-WINDOW: 5000Content-Type: application/json// Spot Limit order with market tp sl{"category": "spot","symbol": "BTCUSDT","side": "Buy","orderType": "Limit","qty": "0.01","price": "28000","timeInForce": "PostOnly","takeProfit": "35000","stopLoss": "27000","tpOrderType": "Market","slOrderType": "Market"}// Spot Limit order with limit tp sl{"category": "spot","symbol": "BTCUSDT","side": "Buy","orderType": "Limit","qty": "0.01","price": "28000","timeInForce": "PostOnly","takeProfit": "35000","stopLoss": "27000","tpLimitPrice": "36000","slLimitPrice": "27500","tpOrderType": "Limit","slOrderType": "Limit"}// Spot PostOnly normal order{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","timeInForce":"PostOnly","orderLinkId":"spot-test-01","isLeverage":0,"orderFilter":"Order"}// Spot TP/SL order{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","triggerPrice": "15000", "timeInForce":"Limit","orderLinkId":"spot-test-02","isLeverage":0,"orderFilter":"tpslOrder"}// Spot margin normal order (UTA){"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","timeInForce":"GTC","orderLinkId":"spot-test-limit","isLeverage":1,"orderFilter":"Order"}// Spot Market Buy order, qty is quote currency{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Market","qty":"200","timeInForce":"IOC","orderLinkId":"spot-test-04","isLeverage":0,"orderFilter":"Order"}// USDT Perp open long position (one-way mode){"category":"linear","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"1","price":"25000","timeInForce":"GTC","positionIdx":0,"orderLinkId":"usdt-test-01","reduceOnly":false,"takeProfit":"28000","stopLoss":"20000","tpslMode":"Partial","tpOrderType":"Limit","slOrderType":"Limit","tpLimitPrice":"27500","slLimitPrice":"20500"}// USDT Perp close long position (one-way mode){"category": "linear", "symbol": "BTCUSDT", "side": "Sell", "orderType": "Limit", "qty": "1", "price": "30000", "timeInForce": "GTC", "positionIdx": 0, "orderLinkId": "usdt-test-02", "reduceOnly": true}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.pre_check_order(    category="spot",    symbol="BTCUSDT",    side="Buy",    orderType="Limit",    qty="0.1",    price="28000",    timeInForce="PostOnly",    takeProfit="35000",    stopLoss="27000",    tpOrderType="Market",    slOrderType="Market",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "24920bdb-4019-4e37-ad1c-876e3a855ac3",        "orderLinkId": "test129",        "preImrE4": 30,        "preMmrE4": 21,        "postImrE4": 357,        "postMmrE4": 294    },    "retExtInfo": {},    "time": 1749541599589}
```

---

## Adjust Collateral Amount

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/adjust-collateral

**Contents:**
- Adjust Collateral Amount
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Adjust Collateral Amount
On this page
Adjust Collateral Amount
You can increase or reduce your collateral amount. When you reduce, please obey the
max. allowed reduction amount
.
Permission: "Spot trade"
info
The adjusted collateral amount will be returned to or deducted from the Funding wallet.
HTTP Request
​
POST
/v5/crypto-loan/adjust-ltv
Request Parameters
​
Parameter
Required
Type
Comments
orderId
true
string
Loan order ID
amount
true
string
Adjustment amount
direction
true
string
0
: add collateral;
1
: reduce collateral
Response Parameters
​
Parameter
Type
Comments
adjustId
string
Collateral adjustment transaction ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan/adjust-ltv
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
1728635421137
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
85
{
"orderId"
:
"1794267532472646144"
,
"amount"
:
"0.001"
,
"direction"
:
"1"
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
adjust_collateral_amount
(
orderId
=
"1794267532472646144"
,
amount
=
"0.001"
,
direction
=
"1"
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
adjustCollateralAmount
(
{
orderId
:
'1794267532472646144'
,
amount
:
'0.001'
,
direction
:
'1'
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
"request.success"
,
"result"
:
{
"adjustId"
:
"1794318409405331968"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1728635422833
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan/adjust-ltv HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728635421137X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 85{    "orderId": "1794267532472646144",    "amount": "0.001",    "direction": "1"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.adjust_collateral_amount(    orderId="1794267532472646144",    amount="0.001",    direction="1",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .adjustCollateralAmount({    orderId: '1794267532472646144',    amount: '0.001',    direction: '1',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "adjustId": "1794318409405331968"    },    "retExtInfo": {},    "time": 1728635422833}
```

---

## Get Repayment History

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/flexible/repay-orders

**Contents:**
- Get Repayment History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Flexible Loan
Get Repayment History
On this page
Get Repayment History
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-flexible/repayment-history
Request Parameters
​
Parameter
Required
Type
Comments
repayId
false
string
Repayment tranaction ID
loanCurrency
false
string
Loan coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> loanCurrency
string
Loan coin
> repayAmount
string
Repayment amount
> repayId
string
Repayment transaction ID
> repayStatus
integer
Repayment status,
1
: success;
2
: processing;
3
: fail
> repayTime
long
Repay timestamp
> repayType
integer
Repayment type,
1
: repay by user;
2
: repay by liquidation;
5
: repay by delisting;
6
: repay by delay liquidation;
7
: repay by currency
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-flexible/repayment-history?loanCurrency=BTC
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
1752570746227
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
get_repayment_history_flexible_crypto_loan
(
loanCurrency
=
"BTC"
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
"loanCurrency"
:
"BTC"
,
"repayAmount"
:
"0.007"
,
"repayId"
:
"1773"
,
"repayStatus"
:
1
,
"repayTime"
:
1752570731274
,
"repayType"
:
1
}
,
{
"loanCurrency"
:
"BTC"
,
"repayAmount"
:
"0.006"
,
"repayId"
:
"1772"
,
"repayStatus"
:
1
,
"repayTime"
:
1752570726038
,
"repayType"
:
1
}
,
{
"loanCurrency"
:
"BTC"
,
"repayAmount"
:
"0.005"
,
"repayId"
:
"1771"
,
"repayStatus"
:
1
,
"repayTime"
:
1752569614528
,
"repayType"
:
1
}
]
,
"nextPageCursor"
:
"1769"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752570745493
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-flexible/repayment-history?loanCurrency=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752570746227X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_repayment_history_flexible_crypto_loan(    loanCurrency="BTC",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "loanCurrency": "BTC",                "repayAmount": "0.007",                "repayId": "1773",                "repayStatus": 1,                "repayTime": 1752570731274,                "repayType": 1            },            {                "loanCurrency": "BTC",                "repayAmount": "0.006",                "repayId": "1772",                "repayStatus": 1,                "repayTime": 1752570726038,                "repayType": 1            },            {                "loanCurrency": "BTC",                "repayAmount": "0.005",                "repayId": "1771",                "repayStatus": 1,                "repayTime": 1752569614528,                "repayType": 1            }        ],        "nextPageCursor": "1769"    },    "retExtInfo": {},    "time": 1752570745493}
```

---

## Get Borrow Order Info

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/borrow-order

**Contents:**
- Get Borrow Order Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Borrow Order Info
On this page
Get Borrow Order Info
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-fixed/borrow-order-info
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
orderCurrency
false
string
Loan coin name
state
false
string
Borrow order status,
1
: matching;
2
: partially filled and cancelled;
3
: Fully filled;
4
: Cancelled
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
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> annualRate
string
Annual rate for the borrowing
> orderId
long
Loan order ID
> orderTime
string
Order created time
> filledQty
string
Filled qty
> orderQty
string
Order qty
> orderCurrency
string
Coin name
> state
integer
Borrow order status,
1
: matching;
2
: partially filled and cancelled;
3
: Fully filled;
4
: Cancelled;
5
: fail
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
> repayType
string
1
:Auto Repayment;
2
:Transfer to flexible loan;
0
: No Automatic Repayment. Compatible with existing orders;
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/borrow-order-info?orderId=13010
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
1752655239825
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
"0.01"
,
"filledQty"
:
"0"
,
"orderCurrency"
:
"MANA"
,
"orderId"
:
13010
,
"orderQty"
:
"2000"
,
"orderTime"
:
"1752654035179"
,
"repayType"
:
"2"
,
"state"
:
1
,
"term"
:
30
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
1752655241090
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/borrow-order-info?orderId=13010 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752655239825X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "annualRate": "0.01",                "filledQty": "0",                "orderCurrency": "MANA",                "orderId": 13010,                "orderQty": "2000",                "orderTime": "1752654035179",                "repayType": "2",                "state": 1,                "term": 30            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1752655241090}
```

---

## Get Repayment Orders

**URL:** https://bybit-exchange.github.io/docs/v5/otc/repay-info

**Contents:**
- Get Repayment Orders
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Institutional Loan
Get Repayment Orders
On this page
Get Repayment Orders
Get a list of your loan repayment orders (orders which repaid the loan).
tip
Get the past 2 years data by default
Get up to the past 2 years of data
HTTP Request
​
GET
/v5/ins-loan/repaid-history
Request Parameters
​
Parameter
Required
Type
Comments
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
Limit for data size.
[
1
,
100
]
. Default:
100
Response Parameters
​
Parameter
Type
Comments
repayInfo
array
Object
> repayOrderId
string
Repaid order ID
> repaidTime
string
Repaid timestamp (ms)
> token
string
Repaid coin
> quantity
string
Repaid principle
> interest
string
Repaid interest
> businessType
string
Repaid type.
1
：normal repayment;
2
：repaid by liquidation
> status
string
1
：success;
2
：fail
Request Example
​
HTTP
Python
Node.js
GET
/v5/ins-loan/repaid-history
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN-TYPE
:
2
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1678687944725
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
get_repayment_info
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
getInstitutionalLendingRepayOrders
(
{
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
""
,
"result"
:
{
"repayInfo"
:
[
{
"repayOrderId"
:
"8189"
,
"repaidTime"
:
"1663126393000"
,
"token"
:
"USDT"
,
"quantity"
:
"30000"
,
"interest"
:
"0"
,
"businessType"
:
"1"
,
"status"
:
"1"
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
1669366648366
}

**Examples:**

Example 1 ():
```
GET /v5/ins-loan/repaid-history HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1678687944725X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_repayment_info())
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInstitutionalLendingRepayOrders({    limit: 100,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "repayInfo": [            {                "repayOrderId": "8189",                "repaidTime": "1663126393000",                "token": "USDT",                "quantity": "30000",                "interest": "0",                "businessType": "1",                "status": "1"            }        ]    },    "retExtInfo": {},    "time": 1669366648366}
```

---

## Get Convert Status

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/query-trade

**Contents:**
- Get Convert Status
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Fiat-Convert
Get Convert Status
On this page
Get Convert Status
Returns the details of this convert.
HTTP Request
​
GET
/v5/fiat/trade-query
Request Parameters
​
Parameter
Required
Type
Comments
tradeNo
false
string
Trade order No,tradeNo or merchantRequestId must be provided
merchantRequestId
false
string
Customised request ID,tradeNo or merchantRequestId must be provided
Response Parameters
​
Parameter
Type
Comments
result
object
object
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
Trade created time
> subUserId
string
The user's sub userId in bybit
Request Example
​
HTTP
GET
/v5/fiat/trade-query?tradeNo=TradeNo123456
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
"1764558832014"
,
"subUserId"
:
"123456"
}
}

**Examples:**

Example 1 ():
```
GET /v5/fiat/trade-query?tradeNo=TradeNo123456 HTTP/1.1  Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720074159814X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "tradeNo": "TradeNo123456",        "status": "success",        "quoteTaxId": "QuoteTaxId123456",        "exchangeRate": "1.0",        "fromCoin": "GEL",        "fromCoinType": "fiat",        "toCoin": "USDT",        "toCoinType": "crypto",        "fromAmount": "100",        "toAmount": "100",        "createdAt": "1764558832014",        "subUserId": "123456"    }}
```

---

## Get Completed Loan History

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/completed-loan-order

**Contents:**
- Get Completed Loan History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Get Completed Loan History
On this page
Get Completed Loan History
Query for the last 6 months worth of your completed (fully paid off) loans.
Permission: "Spot trade"
HTTP Request
​
GET
/v5/crypto-loan/borrow-history
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
loanCurrency
false
string
Loan coin name
collateralCurrency
false
string
Collateral coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> borrowTime
string
The timestamp to borrow
> collateralCurrency
string
Collateral coin
> expirationTime
string
Loan maturity time, keeps
""
for flexible loan
> hourlyInterestRate
string
Hourly interest rate
Flexible loan, it is real-time interest rate
Fixed term loan: it is fixed term interest rate
> initialCollateralAmount
string
Initial amount to mortgage
> initialLoanAmount
string
Initial loan amount
> loanCurrency
string
Loan coin
> loanTerm
string
Loan term,
7
,
14
,
30
,
90
,
180
days, keep
""
for flexible loan
> orderId
string
Loan order ID
> repaidInterest
string
Total interest repaid
> repaidPenaltyInterest
string
Total penalty interest repaid
> status
integer
Loan order status
1
: fully repaid manually;
2
: fully repaid by liquidation
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/borrow-history?orderId=1793683005081680384
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
1728630979731
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
get_completed_loan_history
(
orderId
=
"1793683005081680384"
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
getCompletedLoanOrderHistory
(
{
orderId
:
'1794267532472646144'
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
"request.success"
,
"result"
:
{
"list"
:
[
{
"borrowTime"
:
"1728546174028"
,
"collateralCurrency"
:
"BTC"
,
"expirationTime"
:
"1729148399000"
,
"hourlyInterestRate"
:
"0.0000010241"
,
"initialCollateralAmount"
:
"0.0494727"
,
"initialLoanAmount"
:
"1"
,
"loanCurrency"
:
"ETH"
,
"loanTerm"
:
"7"
,
"orderId"
:
"1793569729874260992"
,
"repaidInterest"
:
"0.00000515"
,
"repaidPenaltyInterest"
:
"0"
,
"status"
:
1
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
1728632014857
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan/borrow-history?orderId=1793683005081680384 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728630979731X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_completed_loan_history(        orderId="1793683005081680384",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getCompletedLoanOrderHistory({ orderId: '1794267532472646144' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "borrowTime": "1728546174028",                "collateralCurrency": "BTC",                "expirationTime": "1729148399000",                "hourlyInterestRate": "0.0000010241",                "initialCollateralAmount": "0.0494727",                "initialLoanAmount": "1",                "loanCurrency": "ETH",                "loanTerm": "7",                "orderId": "1793569729874260992",                "repaidInterest": "0.00000515",                "repaidPenaltyInterest": "0",                "status": 1            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1728632014857}
```

---

## Get Supply Order Info

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/supply-order

**Contents:**
- Get Supply Order Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Supply Order Info
On this page
Get Supply Order Info
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-fixed/supply-order-info
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Supply order ID
orderCurrency
false
string
Supply coin name
state
false
string
Supply order status,
1
: matching;
2
: partially filled and cancelled;
3
: Fully filled;
4
: Cancelled
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
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> annualRate
string
Annual rate for the supply
> orderId
long
Supply order ID
> orderTime
string
Order created time
> filledQty
string
Filled qty
> orderQty
string
Order qty
> orderCurrency
string
Coin name
> state
integer
Supply order status,
1
: matching;
2
: partially filled and cancelled;
3
: Fully filled;
4
: Cancelled;
5
: fail
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
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/supply-order-info?orderId=13564
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
1752655992606
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
get_lending_orders_fixed_crypto_loan
(
orderId
=
"13564"
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
"0.01"
,
"filledQty"
:
"800"
,
"orderCurrency"
:
"USDT"
,
"orderId"
:
13564
,
"orderQty"
:
"1020"
,
"orderTime"
:
"1752482751043"
,
"state"
:
2
,
"term"
:
7
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
1752655993869
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/supply-order-info?orderId=13564 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752655992606X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_lending_orders_fixed_crypto_loan(    orderId="13564",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "annualRate": "0.01",                "filledQty": "800",                "orderCurrency": "USDT",                "orderId": 13564,                "orderQty": "1020",                "orderTime": "1752482751043",                "state": 2,                "term": 7            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1752655993869}
```

---

## Get Order History

**URL:** https://bybit-exchange.github.io/docs/v5/order/order-list

**Contents:**
- Get Order History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Get Order History (2 years)
On this page
Get Order History
Query order history. As order creation/cancellation is
asynchronous
, the data returned from this endpoint may delay. If you want to get
real-time order information, you could query this
endpoint
or rely on the
websocket stream
(recommended).
rule
The orders in the
last 7 days
:
support querying all
closed status
except "Cancelled", "Rejected", "Deactivated" status
The orders in the
last 24 hours
:
the orders with "Cancelled" (fully cancelled order), "Rejected", "Deactivated" can be query
The orders
beyond 7 days
:
supports querying orders which have fills only, i.e., fully filled, partial filled but cancelled orders
HTTP Request
​
GET
/v5/order/history
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
spot
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
Base coin, uppercase only
settleCoin
false
string
Settle coin, uppercase only
orderId
false
string
Order ID
orderLinkId
false
string
User customised order ID
orderFilter
false
string
Order
: active order
StopOrder
: conditional order for Futures and Spot
tpslOrder
: spot TP/SL order
OcoOrder
: spot OCO orders
BidirectionalTpslOrder
: Spot bidirectional TPSL order
all kinds of orders are returned by default
orderStatus
false
string
Order status
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 7 days by default
Only startTime is passed, return range between startTime and startTime+7 days
Only endTime is passed, return range between endTime-7 days and endTime
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
category
string
Product type
list
array
Object
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
> parentOrderLinkId
string
Indicates the linked parent order for attached take-profit and stop-loss orders. Supported for futures and options.
Amending
take-profit or stop-loss orders does not change the parentOrderLinkId
Futures
: using
set trading stop
to update attached TP/SL from the original order does not change the parentOrderLinkId.
Options
: using
set trading stop
to update attached TP/SL from the original order will change the parentOrderLinkId.
Futures & Options
: if TP/SL is set via
set trading stop
for a position that originally has no attached TP/SL, the parentOrderLinkId is meaningless.
> blockTradeId
string
Block trade ID
> symbol
string
Symbol name
> price
string
Order price
> qty
string
Order qty
> side
string
Side.
Buy
,
Sell
> isLeverage
string
Whether to borrow.
0
: false,
1
: true.
>
positionIdx
integer
Position index. Used to identify positions in different position modes
>
orderStatus
string
Order status
>
createType
string
Order create type
Only for category=linear or inverse
Spot, Option do not have this key
>
cancelType
string
Cancel type
>
rejectReason
string
Reject reason
> avgPrice
string
Average filled price, returns
""
for those orders without avg price
> leavesQty
string
The remaining qty not executed
> leavesValue
string
The estimated value not executed
> cumExecQty
string
Cumulative executed order qty
> cumExecValue
string
Cumulative executed order value
> cumExecFee
string
inverse
,
option
: Cumulative executed trading fee.
linear
,
spot
: Deprecated. Use
cumFeeDetail
instead.
>
timeInForce
string
Time in force
>
orderType
string
Order type.
Market
,
Limit
. For TP/SL orders, is the order type after the order was triggered
Block trade Roll Back
,
Block trade-Limit
: Unique enum values for Unified account block trades
>
stopOrderType
string
Stop order type
> orderIv
string
Implied volatility
> marketUnit
string
The unit for
qty
when create
Spot market
orders.
baseCoin
,
quoteCoin
> slippageToleranceType
string
Spot and Futures market order slippage tolerance type
TickSize
,
Percent
,
UNKNOWN
(default)
> slippageTolerance
string
Slippage tolerance value
> triggerPrice
string
Trigger price. If
stopOrderType
=
TrailingStop
, it is activate price. Otherwise, it is trigger price
> takeProfit
string
Take profit price
> stopLoss
string
Stop loss price
> tpslMode
string
TP/SL mode,
Full
: entire position for TP/SL.
Partial
: partial position tp/sl. Spot does not have this field, and Option returns always ""
> ocoTriggerBy
string
The trigger type of Spot OCO order.
OcoTriggerByUnknown
,
OcoTriggerByTp
,
OcoTriggerBySl
> tpLimitPrice
string
The limit order price when take profit price is triggered
> slLimitPrice
string
The limit order price when stop loss price is triggered
>
tpTriggerBy
string
The price type to trigger take profit
>
slTriggerBy
string
The price type to trigger stop loss
> triggerDirection
integer
Trigger direction.
1
: rise,
2
: fall
>
triggerBy
string
The price type of trigger price
> lastPriceOnCreated
string
Last price when place the order, Spot is not applicable
> basePrice
string
Last price when place the order, Spot has this field only
> reduceOnly
boolean
Reduce only.
true
means reduce position size
> closeOnTrigger
boolean
Close on trigger.
What is a close on trigger order?
> placeType
string
Place type,
option
used.
iv
,
price
>
smpType
string
SMP execution type
> smpGroup
integer
Smp group ID. If the UID has no group, it is
0
by default
> smpOrderId
string
The counterparty's orderID which triggers this SMP execution
> createdTime
string
Order created timestamp (ms)
> updatedTime
string
Order updated timestamp (ms)
> extraFees
string
Trading fee rate information. Currently, this data is returned only for spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum:
feeType
,
subFeeType
> cumFeeDetail
json
linear
,
spot
: Cumulative trading fee details instead of
cumExecFee
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
Java
Node.js
GET
/v5/order/history?category=linear&limit=1
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
1672221263407
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
get_order_history
(
category
=
"linear"
,
limit
=
1
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
orderHistory
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
LINEAR
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
System
.
out
.
println
(
client
.
getOrderHistory
(
orderHistory
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
getHistoricOrders
(
{
category
:
'linear'
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
"list"
:
[
{
"orderId"
:
"14bad3a1-6454-43d8-bcf2-5345896cf74d"
,
"orderLinkId"
:
"YLxaWKMiHU"
,
"blockTradeId"
:
""
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"26864.40"
,
"qty"
:
"0.003"
,
"side"
:
"Buy"
,
"isLeverage"
:
""
,
"positionIdx"
:
1
,
"orderStatus"
:
"Cancelled"
,
"cancelType"
:
"UNKNOWN"
,
"rejectReason"
:
"EC_PostOnlyWillTakeLiquidity"
,
"avgPrice"
:
"0"
,
"leavesQty"
:
"0.000"
,
"leavesValue"
:
"0"
,
"cumExecQty"
:
"0.000"
,
"cumExecValue"
:
"0"
,
"cumExecFee"
:
"0"
,
"timeInForce"
:
"PostOnly"
,
"orderType"
:
"Limit"
,
"stopOrderType"
:
"UNKNOWN"
,
"orderIv"
:
""
,
"triggerPrice"
:
"0.00"
,
"takeProfit"
:
"0.00"
,
"stopLoss"
:
"0.00"
,
"tpTriggerBy"
:
"UNKNOWN"
,
"slTriggerBy"
:
"UNKNOWN"
,
"triggerDirection"
:
0
,
"triggerBy"
:
"UNKNOWN"
,
"lastPriceOnCreated"
:
"0.00"
,
"reduceOnly"
:
false
,
"closeOnTrigger"
:
false
,
"smpType"
:
"None"
,
"smpGroup"
:
0
,
"smpOrderId"
:
""
,
"tpslMode"
:
""
,
"tpLimitPrice"
:
""
,
"slLimitPrice"
:
""
,
"placeType"
:
""
,
"slippageToleranceType"
:
"UNKNOWN"
,
"slippageTolerance"
:
""
,
"createdTime"
:
"1684476068369"
,
"updatedTime"
:
"1684476068372"
,
"extraFees"
:
""
,
"cumFeeDetail"
:
{
"MNT"
:
"0.00242968"
}
}
]
,
"nextPageCursor"
:
"page_token%3D39380%26"
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
1684766282976
}

**Examples:**

Example 1 ():
```
GET /v5/order/history?category=linear&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672221263407X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_order_history(    category="linear",    limit=1,))
```

Example 3 ():
```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var orderHistory = TradeOrderRequest.builder().category(CategoryType.LINEAR).limit(10).build();System.out.println(client.getOrderHistory(orderHistory));
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getHistoricOrders({        category: 'linear',        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Loan Repayment History

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/repay-transaction

**Contents:**
- Get Loan Repayment History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Get Loan Repayment History
On this page
Get Loan Repayment History
Query for loan repayment transactions. A loan may be repaid in multiple repayments.
Permission: "Spot trade"
info
Supports querying for the last 6 months worth of completed loan orders.
Only successful repayments can be queried for.
HTTP Request
​
GET
/v5/crypto-loan/repayment-history
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
repayId
false
string
Repayment tranaction ID
loanCurrency
false
string
Loan coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> collateralCurrency
string
Collateral coin
> collateralReturn
string
Amount of collateral returned as a result of this repayment.
"0"
if this isn't the final loan repayment
> loanCurrency
string
Loan coin
> loanTerm
string
Loan term,
7
,
14
,
30
,
90
,
180
days, keep
""
for flexible loan
> orderId
string
Loan order ID
> repayAmount
string
Repayment amount
> repayId
string
Repayment transaction ID
> repayStatus
integer
Repayment status,
1
: success;
2
: processing
> repayTime
string
Repay timestamp
> repayType
string
Repayment type,
1
: repay by user;
2
: repay by liquidation
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/repayment-history?repayId=1794271131730737664
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
1728633716794
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
get_loan_repayment_history
(
repayId
=
"1794271131730737664"
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
getRepaymentHistory
(
{
repayId
:
'1794271131730737664'
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
"request.success"
,
"result"
:
{
"list"
:
[
{
"collateralCurrency"
:
"BTC"
,
"collateralReturn"
:
"0"
,
"loanCurrency"
:
"USDT"
,
"loanTerm"
:
""
,
"orderId"
:
"1794267532472646144"
,
"repayAmount"
:
"100"
,
"repayId"
:
"1794271131730737664"
,
"repayStatus"
:
1
,
"repayTime"
:
"1728629786875"
,
"repayType"
:
"1"
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
1728633717935
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan/repayment-history?repayId=1794271131730737664 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728633716794X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_loan_repayment_history(        repayId="1794271131730737664",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getRepaymentHistory({ repayId: '1794271131730737664' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "collateralCurrency": "BTC",                "collateralReturn": "0",                "loanCurrency": "USDT",                "loanTerm": "",                "orderId": "1794267532472646144",                "repayAmount": "100",                "repayId": "1794271131730737664",                "repayStatus": 1,                "repayTime": "1728629786875",                "repayType": "1"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1728633717935}
```

---

## Set Disconnect Cancel All

**URL:** https://bybit-exchange.github.io/docs/v5/order/dcp

**Contents:**
- Set Disconnect Cancel All
- What is Disconnection Protect (DCP)?​
- How to enable DCP​
- Applicable​
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Set DCP
On this page
Set Disconnect Cancel All
info
What is Disconnection Protect (DCP)?
​
Based on the websocket private connection and heartbeat mechanism, Bybit provides disconnection protection function. The
timing starts from the first disconnection. If the Bybit server does not receive the reconnection from the client for
more than 10 (default) seconds and resumes the heartbeat "ping", then the client is in the state of "disconnection protect",
all active
futures / spot / option
orders of the client will be cancelled automatically. If within 10 seconds, the client reconnects
and resumes the heartbeat "ping", the timing will be reset and restarted at the next disconnection.
How to enable DCP
​
If you need to turn it on/off, you can contact your client manager for consultation and application. The default time window is 10 seconds.
Applicable
​
Effective for
Inverse Perp / Inverse Futures / USDT Perp / USDT Futures / USDC Perp / USDC Futures / Spot / options
tip
After the request is successfully sent, the system needs a certain time to take effect. It is recommended to query or set again after 10 seconds
You can use
this endpoint
to get your current DCP configuration.
Your private websocket connection
must
subscribe
"dcp" topic
in order to trigger DCP successfully
HTTP Request
​
POST
/v5/order/disconnected-cancel-all
Request Parameters
​
Parameter
Required
Type
Comments
product
false
string
OPTIONS
(default),
DERIVATIVES
,
SPOT
timeWindow
true
integer
Disconnection timing window time.
[
3
,
300
]
, unit: second
Response Parameters
​
None
Request Example
​
HTTP
Python
Java
Node.js
POST v5/order/disconnected-cancel-all HTTP/1.1
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
1675852742375
X-BAPI-RECV-WINDOW
:
50000
Content-Type
:
application/json
{
"timeWindow"
:
40
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
set_dcp
(
timeWindow
=
40
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
setDcpOptionsRequest
=
TradeOrderRequest
.
builder
(
)
.
timeWindow
(
40
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
setDisconnectCancelAllTime
(
setDcpOptionsRequest
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
setDisconnectCancelAllWindow
(
'option'
,
40
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
POST v5/order/disconnected-cancel-all HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675852742375X-BAPI-RECV-WINDOW: 50000Content-Type: application/json{  "timeWindow": 40}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_dcp(    timeWindow=40,))
```

Example 3 ():
```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var setDcpOptionsRequest = TradeOrderRequest.builder().timeWindow(40).build();System.out.println(client.setDisconnectCancelAllTime(setDcpOptionsRequest));
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setDisconnectCancelAllWindow('option', 40)    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Trade History

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/trade-history

**Contents:**
- Get Trade History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Get Trade History
On this page
Get Trade History
info
In self-trade cases, both the maker and taker single-leg trades will be returned in the same request.
Single leg executions can also be found with "execType"=
FutureSpread
via
Get Trade History
HTTP Request
​
GET
/v5/spread/execution/list
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
orderId
false
string
Spread combination order ID
orderLinkId
false
string
User customised order ID
startTime
false
long
The start timestamp (ms)
startTime and endTime are not passed, return 7 days by default
Only startTime is passed, return range between startTime and startTime+7 days
Only endTime is passed, return range between endTime-7 days and endTime
If both are passed, the rule is endTime - startTime <= 7 days
endTime
false
long
The end timestamp (ms)
limit
false
integer
Limit for parent order data size per page.
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
Trade info
> symbol
string
Spread combination symbol name
> orderLinkId
string
User customised order ID
> side
string
Side,
Buy
,
Sell
> orderId
string
Spread combination order ID
> execPrice
string
Combo Exec price
> execTime
string
Combo exec timestamp (ms)
> execType
string
Combo exec type,
Trade
> execQty
string
Combo exec qty
> execId
string
Combo exec ID
> legs
array
<
object
>
Legs execution info
>> symbol
string
Leg symbol name
>> side
string
Leg order side,
Buy
,
Sell
>> execPrice
string
Leg exec price
>> execTime
string
Leg exec timestamp (ms)
>> execValue
string
Leg exec value
>>
execType
string
Leg exec type
>> category
string
Leg category,
linear
,
spot
>> execQty
string
Leg exec qty
>> execFee
string
Leg exec fee, deprecated for Spot leg
>> execFeeV2
string
Leg exec fee, used for Spot leg only
>> feeCurrency
string
Leg fee currency
>> execId
string
Leg exec ID
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
GET
/v5/spread/execution/list?orderId=5e010c35-2b44-4f03-8081-8fa31fb73376
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
XXXXX
X-BAPI-TIMESTAMP
:
1744105738529
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
"Success"
,
"result"
:
{
"nextPageCursor"
:
"82c82077-0caa-5304-894d-58a50a342bd7%3A1744104992219%2C82c82077-0caa-5304-894d-58a50a342bd7%3A1744104992219"
,
"list"
:
[
{
"symbol"
:
"SOLUSDT_SOL/USDT"
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
"5e010c35-2b44-4f03-8081-8fa31fb73376"
,
"execPrice"
:
"21"
,
"legs"
:
[
{
"symbol"
:
"SOLUSDT"
,
"side"
:
"Buy"
,
"execPrice"
:
"124.1"
,
"execTime"
:
"1744104992224"
,
"execValue"
:
"248.2"
,
"execType"
:
"FutureSpread"
,
"category"
:
"linear"
,
"execQty"
:
"2"
,
"execFee"
:
"0.039712"
,
"execId"
:
"99a18f80-d3b5-4c6f-a1f1-8c5870e3f3bc"
}
,
{
"symbol"
:
"SOLUSDT"
,
"side"
:
"Sell"
,
"execPrice"
:
"103.1152"
,
"execTime"
:
"1744104992224"
,
"execValue"
:
"206.2304"
,
"execType"
:
"FutureSpread"
,
"category"
:
"spot"
,
"execQty"
:
"2"
,
"execFee"
:
"0.06186912"
,
"execId"
:
"2110000000061481958"
}
]
,
"execTime"
:
"1744104992220"
,
"execType"
:
"Trade"
,
"execQty"
:
"2"
,
"execId"
:
"82c82077-0caa-5304-894d-58a50a342bd7"
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
1744105105169
}

**Examples:**

Example 1 ():
```
GET /v5/spread/execution/list?orderId=5e010c35-2b44-4f03-8081-8fa31fb73376 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1744105738529X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "nextPageCursor": "82c82077-0caa-5304-894d-58a50a342bd7%3A1744104992219%2C82c82077-0caa-5304-894d-58a50a342bd7%3A1744104992219",        "list": [            {                "symbol": "SOLUSDT_SOL/USDT",                "orderLinkId": "",                "side": "Buy",                "orderId": "5e010c35-2b44-4f03-8081-8fa31fb73376",                "execPrice": "21",                "legs": [                    {                        "symbol": "SOLUSDT",                        "side": "Buy",                        "execPrice": "124.1",                        "execTime": "1744104992224",                        "execValue": "248.2",                        "execType": "FutureSpread",                        "category": "linear",                        "execQty": "2",                        "execFee": "0.039712",                        "execId": "99a18f80-d3b5-4c6f-a1f1-8c5870e3f3bc"                    },                    {                        "symbol": "SOLUSDT",                        "side": "Sell",                        "execPrice": "103.1152",                        "execTime": "1744104992224",                        "execValue": "206.2304",                        "execType": "FutureSpread",                        "category": "spot",                        "execQty": "2",                        "execFee": "0.06186912",                        "execId": "2110000000061481958"                    }                ],                "execTime": "1744104992220",                "execType": "Trade",                "execQty": "2",                "execId": "82c82077-0caa-5304-894d-58a50a342bd7"            }        ]    },    "retExtInfo": {},    "time": 1744105105169}
```

---

## Get Order Records

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/order-record

**Contents:**
- Get Order Records
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Get Order Records
On this page
Get Order Records
Get lending or redeem history
HTTP Request
​
GET
/v5/lending/history-order
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
Coin name
orderId
false
string
Order ID
startTime
false
long
The start timestamp (ms)
endTime
false
long
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
orderType
false
string
Order type.
1
: deposit,
2
: redemption,
3
: Payment of proceeds
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> coin
string
Coin name
> createdTime
string
Created timestamp (ms)
> orderId
string
Order ID
> quantity
string
quantity
> serialNo
string
Serial No
> status
string
Order status.
0
: Initial,
1
: Processing,
2
: Success,
10
: Failed,
11
: Cancelled
> updatedTime
string
Updated timestamp (ms)
Request Example
​
GET
/v5/lending/history-order?orderNo=1403517113428086272
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
1682049395799
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
"OK"
,
"result"
:
{
"list"
:
[
{
"coin"
:
"BTC"
,
"createdTime"
:
"1682048277963"
,
"orderId"
:
"1403517113428086272"
,
"orderType"
:
"2"
,
"quantity"
:
"0.1"
,
"serialNo"
:
"14035171132183710722373"
,
"status"
:
"2"
,
"updatedTime"
:
"1682048278245"
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
1682049395967
}

**Examples:**

Example 1 ():
```
GET /v5/lending/history-order?orderNo=1403517113428086272 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682049395799X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "coin": "BTC",                "createdTime": "1682048277963",                "orderId": "1403517113428086272",                "orderType": "2",                "quantity": "0.1",                "serialNo": "14035171132183710722373",                "status": "2",                "updatedTime": "1682048278245"            }        ]    },    "retExtInfo": {},    "time": 1682049395967}
```

---

## Get Loan Orders

**URL:** https://bybit-exchange.github.io/docs/v5/otc/loan-info

**Contents:**
- Get Loan Orders
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Institutional Loan
Get Loan Orders
On this page
Get Loan Orders
Get up to 2 years worth of historical loan orders.
HTTP Request
​
GET
/v5/ins-loan/loan-order
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID. If not passed, returns all orders sorted by
loanTime
in descending order
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
Limit for data size.
[
1
,
100
]
, Default:
10
Response Parameters
​
Parameter
Type
Comments
loanInfo
array
Object
> orderId
string
Loan order ID
> orderProductId
string
Product ID
> parentUid
string
The designated UID that was used to bind with the INS loan
> loanTime
string
Loan timestamp, in milliseconds
> loanCoin
string
Loan coin
> loanAmount
string
Loan amount
> unpaidAmount
string
Unpaid principal
> unpaidInterest
string
Unpaid interest
> repaidAmount
string
Repaid principal
> repaidInterest
string
Repaid interest
> interestRate
string
Daily interest rate
> status
string
1
：outstanding;
2
：paid off
> leverage
string
The maximum leverage for this loan product
> supportSpot
string
Whether to support spot.
0
:false;
1
:true
> supportContract
string
Whether to support contract .
0
:false;
1
:true
> withdrawLine
string
Restrict line for withdrawal
> transferLine
string
Restrict line for transfer
> spotBuyLine
string
Restrict line for SPOT buy
> spotSellLine
string
Restrict line for SPOT sell
> contractOpenLine
string
Restrict line for USDT Perpetual open position
> deferredLiquidationLine
string
Line for deferred liquidation
> deferredLiquidationTime
string
Time for deferred liquidation
> reserveToken
string
Reserve token
> reserveQuantity
string
Reserve token qty
> liquidationLine
string
Line for liquidation
> stopLiquidationLine
string
Line for stop liquidation
> contractLeverage
string
The allowed default leverage for USDT Perpetual
> transferRatio
string
The transfer ratio for loan funds to transfer from Spot wallet to Contract wallet
> spotSymbols
array
The whitelist of spot trading pairs. If there is no whitelist, then "[]"
> contractSymbols
array
The whitelist of contract trading pairs
If
supportContract
="0", then this is "[]"
If there is no whitelist, this is "[]"
> supportUSDCContract
string
Whether to support USDC contract.
"0"
:false;
"1"
:true
> supportUSDCOptions
string
Whether to support Option.
"0"
:false;
"1"
:true
> supportMarginTrading
string
Whether to support Spot margin trading.
"0"
:false;
"1"
:true
> USDTPerpetualOpenLine
string
Restrict line to open USDT Perpetual position
> USDCContractOpenLine
string
Restrict line to open USDC Contract position
> USDCOptionsOpenLine
string
Restrict line to open Option position
> USDTPerpetualCloseLine
string
Restrict line to trade USDT Perpetual position
> USDCContractCloseLine
string
Restrict line to trade USDC Contract position
> USDCOptionsCloseLine
string
Restrict line to trade Option position
> USDCContractSymbols
array
The whitelist of USDC contract trading pairs
If no whitelist symbols, it is
[]
, and you can trade any
If supportUSDCContract="0", it is
[]
> USDCOptionsSymbols
array
The whitelist of Option symbols
If no whitelisted, it is
[]
, and you can trade any
If supportUSDCOptions="0", it is
[]
> marginLeverage
string
The allowable maximum leverage for Spot margin
> USDTPerpetualLeverage
array
Object
If supportContract="0", it is
[]
If no whitelist USDT perp symbols, it returns all trading symbols and leverage by default
If there are whitelist symbols, it return those whitelist data
>> symbol
string
Symbol name
>> leverage
string
Maximum leverage
> USDCContractLeverage
array
Object
If supportUSDCContract="0", it is
[]
If no whitelist USDC contract symbols, it returns all trading symbols and leverage by default
If there are whitelist symbols, it return those whitelist data
>> symbol
string
Symbol name
>> leverage
string
Maximum leverage
Request Example
​
HTTP
Python
Node.js
GET
/v5/ins-loan/loan-order
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1678687874060
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
get_loan_orders
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
getInstitutionalLendingLoanOrders
(
{
limit
:
10
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
""
,
"result"
:
{
"loanInfo"
:
[
{
"orderId"
:
"1468005106166530304"
,
"orderProductId"
:
"96"
,
"parentUid"
:
"1631521"
,
"loanTime"
:
"1689735916000"
,
"loanCoin"
:
"USDT"
,
"loanAmount"
:
"204"
,
"unpaidAmount"
:
"52.07924201"
,
"unpaidInterest"
:
"0"
,
"repaidAmount"
:
"151.92075799"
,
"repaidInterest"
:
"0"
,
"interestRate"
:
"0.00019178"
,
"status"
:
"1"
,
"leverage"
:
"4"
,
"supportSpot"
:
"1"
,
"supportContract"
:
"1"
,
"withdrawLine"
:
""
,
"transferLine"
:
""
,
"spotBuyLine"
:
"0.71"
,
"spotSellLine"
:
"0.71"
,
"contractOpenLine"
:
"0.71"
,
"liquidationLine"
:
"0.75"
,
"stopLiquidationLine"
:
"0.35000000"
,
"contractLeverage"
:
"7"
,
"transferRatio"
:
"1"
,
"spotSymbols"
:
[
]
,
"contractSymbols"
:
[
]
,
"supportUSDCContract"
:
"1"
,
"supportUSDCOptions"
:
"1"
,
"USDTPerpetualOpenLine"
:
"0.71"
,
"USDCContractOpenLine"
:
"0.71"
,
"USDCOptionsOpenLine"
:
"0.71"
,
"USDTPerpetualCloseLine"
:
"0.71"
,
"USDCContractCloseLine"
:
"0.71"
,
"USDCOptionsCloseLine"
:
"0.71"
,
"USDCContractSymbols"
:
[
]
,
"USDCOptionsSymbols"
:
[
]
,
"deferredLiquidationLine"
:
""
,
"deferredLiquidationTime"
:
""
,
"marginLeverage"
:
"4"
,
"USDTPerpetualLeverage"
:
[
{
"symbol"
:
"SUSHIUSDT"
,
"leverage"
:
"7"
}
,
{
"symbol"
:
"INJUSDT"
,
"leverage"
:
"7"
}
,
{
"symbol"
:
"RDNTUSDT"
,
"leverage"
:
"7"
}
,
{
"symbol"
:
"ZRXUSDT"
,
"leverage"
:
"7"
}
,
{
"symbol"
:
"HIGHUSDT"
,
"leverage"
:
"7"
}
,
{
"symbol"
:
"WAVESUSDT"
,
"leverage"
:
"7"
}
,
...
{
"symbol"
:
"ACHUSDT"
,
"leverage"
:
"7"
}
,
{
"symbol"
:
"SUNUSDT"
,
"leverage"
:
"7"
}
]
,
"USDCContractLeverage"
:
[
{
"symbol"
:
"BTCPERP"
,
"leverage"
:
"8"
}
,
{
"symbol"
:
"BTC-Futures"
,
"leverage"
:
"8"
}
,
...
{
"symbol"
:
"ETH-Futures"
,
"leverage"
:
"8"
}
,
{
"symbol"
:
"SOLPERP"
,
"leverage"
:
"8"
}
,
{
"symbol"
:
"ETHPERP"
,
"leverage"
:
"8"
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
1689745773187
}

**Examples:**

Example 1 ():
```
GET /v5/ins-loan/loan-order HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1678687874060X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_loan_orders())
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInstitutionalLendingLoanOrders({    limit: 10,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "loanInfo": [            {                "orderId": "1468005106166530304",                "orderProductId": "96",                "parentUid": "1631521",                "loanTime": "1689735916000",                "loanCoin": "USDT",                "loanAmount": "204",                "unpaidAmount": "52.07924201",                "unpaidInterest": "0",                "repaidAmount": "151.92075799",                "repaidInterest": "0",                "interestRate": "0.00019178",                "status": "1",                "leverage": "4",                "supportSpot": "1",                "supportContract": "1",                "withdrawLine": "",                "transferLine": "",                "spotBuyLine": "0.71",                "spotSellLine": "0.71",                "contractOpenLine": "0.71",                "liquidationLine": "0.75",                "stopLiquidationLine": "0.35000000",                "contractLeverage": "7",                "transferRatio": "1",                "spotSymbols": [],                "contractSymbols": [],                "supportUSDCContract": "1",                "supportUSDCOptions": "1",                "USDTPerpetualOpenLine": "0.71",                "USDCContractOpenLine": "0.71",                "USDCOptionsOpenLine": "0.71",                "USDTPerpetualCloseLine": "0.71",                "USDCContractCloseLine": "0.71",                "USDCOptionsCloseLine": "0.71",                "USDCContractSymbols": [],                "USDCOptionsSymbols": [],                "deferredLiquidationLine":"",                "deferredLiquidationTime":"",                "marginLeverage": "4",                "USDTPerpetualLeverage": [                    {                        "symbol": "SUSHIUSDT",                        "leverage": "7"                    },                    {                        "symbol": "INJUSDT",                        "leverage": "7"                    },                    {                        "symbol": "RDNTUSDT",                        "leverage": "7"                    },                    {                        "symbol": "ZRXUSDT",                        "leverage": "7"                    },                    {                        "symbol": "HIGHUSDT",                        "leverage": "7"                    },                    {                        "symbol": "WAVESUSDT",                        "leverage": "7"                    },                    ...                    {                        "symbol": "ACHUSDT",                        "leverage": "7"                    },                    {                        "symbol": "SUNUSDT",                        "leverage": "7"                    }                ],                "USDCContractLeverage": [                    {                        "symbol": "BTCPERP",                        "leverage": "8"                    },                    {                        "symbol": "BTC-Futures",                        "leverage": "8"                    },                    ...                    {                        "symbol": "ETH-Futures",                        "leverage": "8"                    },                    {                        "symbol": "SOLPERP",                        "leverage": "8"                    },                    {                        "symbol": "ETHPERP",                        "leverage": "8"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1689745773187}
```

---

## Get Pre-upgrade Order History

**URL:** https://bybit-exchange.github.io/docs/v5/pre-upgrade/order-list

**Contents:**
- Get Pre-upgrade Order History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Pre-upgrade
Get Pre-upgrade Order History
On this page
Get Pre-upgrade Order History
After the account is upgraded to a Unified account, you can get the orders which occurred before the upgrade.
By category="linear", you can query USDT Perps, USDC Perps data occurred during classic account
By category="spot", you can query Spot data occurred during classic account
By category="option", you can query Options data occurred during classic account
By category="inverse", you can query Inverse Contract data occurred during
classic account or
UTA1.0
info
can get all status in 7 days
can only get filled orders beyond 7 days
USDC Perpeual & Option support the recent 6 months data. Please download older data via GUI
HTTP Request
​
GET
/v5/pre-upgrade/order/history
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
,
spot
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only.
If not passed, return settleCoin=USDT by default
To get USDC perp, please pass symbol
baseCoin
false
string
Base coin, uppercase only. Used for
option
query
orderId
false
string
Order ID
orderLinkId
false
string
User customised order ID
orderFilter
false
string
Order
: active order,
StopOrder
: conditional order
orderStatus
false
string
Order status. Not supported for
spot
category
startTime
false
integer
The start timestamp (ms)
startTime
and
endTime
must be passed together or both are not passed
endTime - startTime <= 7 days
If both are not passed, it returns recent 7 days by default
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
category
string
Product type
list
array
Object
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
> blockTradeId
string
Block trade ID
> symbol
string
Symbol name
> price
string
Order price
> qty
string
Order qty
> side
string
Side.
Buy
,
Sell
> isLeverage
string
Useless field for those orders before upgraded
>
positionIdx
integer
Position index. Used to identify positions in different position modes
>
orderStatus
string
Order status
>
cancelType
string
Cancel type
>
rejectReason
string
Reject reason
> avgPrice
string
Average filled price. If unfilled, it is
""
, and also for those orders have partilly filled but cancelled at the end
> leavesQty
string
The remaining qty not executed
> leavesValue
string
The estimated value not executed
> cumExecQty
string
Cumulative executed order qty
> cumExecValue
string
Cumulative executed order value
> cumExecFee
string
Cumulative executed trading fee
>
timeInForce
string
Time in force
>
orderType
string
Order type.
Market
,
Limit
>
stopOrderType
string
Stop order type
> orderIv
string
Implied volatility
> triggerPrice
string
Trigger price. If
stopOrderType
=
TrailingStop
, it is activate price. Otherwise, it is trigger price
> takeProfit
string
Take profit price
> stopLoss
string
Stop loss price
>
tpTriggerBy
string
The price type to trigger take profit
>
slTriggerBy
string
The price type to trigger stop loss
> triggerDirection
integer
Trigger direction.
1
: rise,
2
: fall
>
triggerBy
string
The price type of trigger price
> lastPriceOnCreated
string
Last price when place the order
> reduceOnly
boolean
Reduce only.
true
means reduce position size
> closeOnTrigger
boolean
Close on trigger.
What is a close on trigger order?
> placeType
string
Place type,
option
used.
iv
,
price
>
smpType
string
SMP execution type
> smpGroup
integer
Smp group ID. If the UID has no group, it is
0
by default
> smpOrderId
string
The counterparty's orderID which triggers this SMP execution
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
/v5/pre-upgrade/order/history?category=linear&limit=1&orderStatus=Filled
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
1682576940304
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
"OK"
,
"result"
:
{
"list"
:
[
{
"orderId"
:
"67836246-460e-4c52-a009-af0c3e1d12bc"
,
"orderLinkId"
:
""
,
"blockTradeId"
:
""
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"27203.40"
,
"qty"
:
"0.200"
,
"side"
:
"Sell"
,
"isLeverage"
:
""
,
"positionIdx"
:
0
,
"orderStatus"
:
"Filled"
,
"cancelType"
:
"UNKNOWN"
,
"rejectReason"
:
"EC_NoError"
,
"avgPrice"
:
"28632.126000"
,
"leavesQty"
:
"0.000"
,
"leavesValue"
:
"0"
,
"cumExecQty"
:
"0.200"
,
"cumExecValue"
:
"5726.4252"
,
"cumExecFee"
:
"3.43585512"
,
"timeInForce"
:
"IOC"
,
"orderType"
:
"Market"
,
"stopOrderType"
:
"UNKNOWN"
,
"orderIv"
:
""
,
"triggerPrice"
:
"0.00"
,
"takeProfit"
:
"0.00"
,
"stopLoss"
:
"0.00"
,
"tpTriggerBy"
:
"UNKNOWN"
,
"slTriggerBy"
:
"UNKNOWN"
,
"triggerDirection"
:
0
,
"triggerBy"
:
"UNKNOWN"
,
"lastPriceOnCreated"
:
"0.00"
,
"reduceOnly"
:
true
,
"closeOnTrigger"
:
true
,
"smpType"
:
"None"
,
"smpGroup"
:
0
,
"smpOrderId"
:
""
,
"createdTime"
:
"1682487465732"
,
"updatedTime"
:
"1682487465735"
,
"placeType"
:
""
}
]
,
"nextPageCursor"
:
"page_token%3D69406%26"
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
1682576940540
}

**Examples:**

Example 1 ():
```
GET /v5/pre-upgrade/order/history?category=linear&limit=1&orderStatus=Filled HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682576940304X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "orderId": "67836246-460e-4c52-a009-af0c3e1d12bc",                "orderLinkId": "",                "blockTradeId": "",                "symbol": "BTCUSDT",                "price": "27203.40",                "qty": "0.200",                "side": "Sell",                "isLeverage": "",                "positionIdx": 0,                "orderStatus": "Filled",                "cancelType": "UNKNOWN",                "rejectReason": "EC_NoError",                "avgPrice": "28632.126000",                "leavesQty": "0.000",                "leavesValue": "0",                "cumExecQty": "0.200",                "cumExecValue": "5726.4252",                "cumExecFee": "3.43585512",                "timeInForce": "IOC",                "orderType": "Market",                "stopOrderType": "UNKNOWN",                "orderIv": "",                "triggerPrice": "0.00",                "takeProfit": "0.00",                "stopLoss": "0.00",                "tpTriggerBy": "UNKNOWN",                "slTriggerBy": "UNKNOWN",                "triggerDirection": 0,                "triggerBy": "UNKNOWN",                "lastPriceOnCreated": "0.00",                "reduceOnly": true,                "closeOnTrigger": true,                "smpType": "None",                "smpGroup": 0,                "smpOrderId": "",                "createdTime": "1682487465732",                "updatedTime": "1682487465735",                "placeType": ""            }        ],        "nextPageCursor": "page_token%3D69406%26",        "category": "linear"    },    "retExtInfo": {},    "time": 1682576940540}
```

---

## Execution

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/execution

**Contents:**
- Execution
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

WebSocket Stream
Private
Execution
On this page
Execution
Subscribe to the execution stream to see your executions in
real-time
.
tip
You may have multiple executions for one order in a single message.
All-In-One Topic:
execution
Categorised Topic:
execution.spot
,
execution.linear
,
execution.inverse
,
execution.option
info
All-In-One topic and Categorised topic
cannot
be in the same subscription request
All-In-One topic: Allow you to listen to all categories (spot, linear, inverse, option) websocket updates
Categorised Topic: Allow you to listen only to specific category websocket updates
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
>
category
string
Product type
spot
,
linear
,
inverse
,
option
> symbol
string
Symbol name
> isLeverage
string
Whether to borrow.
0
: false,
1
: true
> orderId
string
Order ID
> orderLinkId
string
User customized order ID
> side
string
Side.
Buy
,
Sell
> orderPrice
string
Order price
> orderQty
string
Order qty
> leavesQty
string
The remaining qty not executed
>
createType
string
Order create type
Spot, Option do not have this key
>
orderType
string
Order type.
Market
,
Limit
>
stopOrderType
string
Stop order type. If the order is not stop order, any type is not returned
> execFee
string
Executed trading fee. You can get spot fee currency instruction
here
> execId
string
Execution ID
> execPrice
string
Execution price
> execQty
string
Execution qty
> execPnl
string
Profit and Loss for each close position execution. The value keeps consistent with the field "cashFlow" in the
Get Transaction Log
>
execType
string
Executed type
> execValue
string
Executed order value
> execTime
string
Executed timestamp (ms)
> isMaker
boolean
Is maker order.
true
: maker,
false
: taker
> feeRate
string
Trading fee rate
> tradeIv
string
Implied volatility. valid for
option
> markIv
string
Implied volatility of mark price. valid for
option
> markPrice
string
The mark price of the symbol when executing. valid for
option
> indexPrice
string
The index price of the symbol when executing. valid for
option
> underlyingPrice
string
The underlying price of the symbol when executing. valid for
option
> blockTradeId
string
Paradigm block trade ID
> closedSize
string
Closed position size
> extraFees
List
Extra trading fee information. Currently, this data is returned only for kyc=Indian user or spot orders placed on the Indonesian site or spot fiat currency orders placed on the EU site. In other cases, an empty string is returned. Enum:
feeType
,
subFeeType
> seq
long
Cross sequence, used to associate each fill and each position update
The seq will be the same when conclude multiple transactions at the same time
Different symbols may have the same seq, please use seq + symbol to check unique
> feeCurrency
string
Trading fee currency
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
"execution"
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
execution_stream
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
"topic"
:
"execution"
,
"id"
:
"386825804_BTCUSDT_140612148849382"
,
"creationTime"
:
1746270400355
,
"data"
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
"closedSize"
:
"0.5"
,
"execFee"
:
"26.3725275"
,
"execId"
:
"0ab1bdf7-4219-438b-b30a-32ec863018f7"
,
"execPrice"
:
"95900.1"
,
"execQty"
:
"0.5"
,
"execType"
:
"Trade"
,
"execValue"
:
"47950.05"
,
"feeRate"
:
"0.00055"
,
"tradeIv"
:
""
,
"markIv"
:
""
,
"blockTradeId"
:
""
,
"markPrice"
:
"95901.48"
,
"indexPrice"
:
""
,
"underlyingPrice"
:
""
,
"leavesQty"
:
"0"
,
"orderId"
:
"9aac161b-8ed6-450d-9cab-c5cc67c21784"
,
"orderLinkId"
:
""
,
"orderPrice"
:
"94942.5"
,
"orderQty"
:
"0.5"
,
"orderType"
:
"Market"
,
"stopOrderType"
:
"UNKNOWN"
,
"side"
:
"Sell"
,
"execTime"
:
"1746270400353"
,
"isLeverage"
:
"0"
,
"isMaker"
:
false
,
"seq"
:
140612148849382
,
"marketUnit"
:
""
,
"execPnl"
:
"0.05"
,
"createType"
:
"CreateByUser"
,
"extraFees"
:
[
{
"feeCoin"
:
"USDT"
,
"feeType"
:
"GST"
,
"subFeeType"
:
"IND_GST"
,
"feeRate"
:
"0.0000675"
,
"fee"
:
"0.006403779"
}
]
,
"feeCurrency"
:
"USDT"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "execution"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)def handle_message(message):    print(message)ws.execution_stream(callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "topic": "execution",    "id": "386825804_BTCUSDT_140612148849382",    "creationTime": 1746270400355,    "data": [        {            "category": "linear",            "symbol": "BTCUSDT",            "closedSize": "0.5",            "execFee": "26.3725275",            "execId": "0ab1bdf7-4219-438b-b30a-32ec863018f7",            "execPrice": "95900.1",            "execQty": "0.5",            "execType": "Trade",            "execValue": "47950.05",            "feeRate": "0.00055",            "tradeIv": "",            "markIv": "",            "blockTradeId": "",            "markPrice": "95901.48",            "indexPrice": "",            "underlyingPrice": "",            "leavesQty": "0",            "orderId": "9aac161b-8ed6-450d-9cab-c5cc67c21784",            "orderLinkId": "",            "orderPrice": "94942.5",            "orderQty": "0.5",            "orderType": "Market",            "stopOrderType": "UNKNOWN",            "side": "Sell",            "execTime": "1746270400353",            "isLeverage": "0",            "isMaker": false,            "seq": 140612148849382,            "marketUnit": "",            "execPnl": "0.05",            "createType": "CreateByUser",            "extraFees":[{"feeCoin":"USDT","feeType":"GST","subFeeType":"IND_GST","feeRate":"0.0000675","fee":"0.006403779"}],            "feeCurrency": "USDT"        }    ]}
```

---

## Confirm a Quote

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/confirm-quote

**Contents:**
- Confirm a Quote
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Asset
Fiat-Convert
Confirm a Quote
On this page
Confirm a Quote
info
The exchange is async; please check the final status by calling the convert history API.
Make sure you confirm the quote before it expires.
HTTP Request
​
POST
/v5/fiat/trade-execute
Request Parameters
​
Parameter
Required
Type
Comments
quoteTxId
true
string
The quote tx ID from
Request a Quote
subUserId
true
string
The user's sub userId in bybit
webhookUrl
false
string
API URL to call when order is successful or failed (max 256 characters)
MerchantRequestId
false
string
Customised request ID(maximum length of 36)
Generally it is useless, but it is convenient to track the quote request internally if you fill this field
Response Parameters
​
Parameter
Type
Comments
tradeNo
string
Trade order No
merchantRequestId
string
Customised request ID
Request Example
​
HTTP
POST
/v5/fiat/trade-execute
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
1720071899789
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
52
{
"quoteTxId"
:
"QuoteTaxId123456"
,
"subUserId"
:
"43456"
}
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
"tradeNo"
:
"TradeNo123456"
,
"merchantRequestId"
:
""
}
}

**Examples:**

Example 1 ():
```
POST /v5/fiat/trade-execute HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720071899789X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52{    "quoteTxId": "QuoteTaxId123456",    "subUserId":"43456"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "tradeNo": "TradeNo123456",        "merchantRequestId": ""    }}
```

---

## Get Supply Contract Info

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/supply-contract%20copy

**Contents:**
- Get Supply Contract Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Get Supply Contract Info
On this page
Get Supply Contract Info
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-fixed/supply-contract-info
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Supply order ID
supplyId
false
string
Supply contract ID
supplyCurrency
false
string
Supply coin name
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
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> annualRate
string
Annual rate for the supply
> supplyCurrency
string
Supply coin
> supplyTime
string
Supply timestamp
> supplyAmount
string
Supply amount
> interestPaid
string
Paid interest
> supplyId
string
Supply contract ID
> orderId
string
Supply order ID
> redemptionTime
string
Planned time to redeem
> penaltyInterest
string
Overdue interest
> actualRedemptionTime
string
Actual time to redeem
> status
integer
Supply contract status
1
: Supplying;
2
: Redeemed
> term
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
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-fixed/supply-contract-info?supplyCurrency=USDT
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
1752654376532
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
get_lending_contract_info_fixed_crypto_loan
(
supplyCurrency
=
"USDT"
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
"actualRedemptionTime"
:
"1753087596082"
,
"annualRate"
:
"0.01"
,
"interest"
:
"0.13041095890410959"
,
"orderId"
:
"13564"
,
"penaltyInterest"
:
"0"
,
"redemptionTime"
:
"1753087596082"
,
"status"
:
1
,
"supplyAmount"
:
"800"
,
"supplyCurrency"
:
"USDT"
,
"supplyId"
:
"567"
,
"supplyTime"
:
"1752482796082"
,
"term"
:
"7"
}
]
,
"nextPageCursor"
:
"567"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752654377461
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-fixed/supply-contract-info?supplyCurrency=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752654376532X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_lending_contract_info_fixed_crypto_loan(    supplyCurrency="USDT",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "actualRedemptionTime": "1753087596082",                "annualRate": "0.01",                "interest": "0.13041095890410959",                "orderId": "13564",                "penaltyInterest": "0",                "redemptionTime": "1753087596082",                "status": 1,                "supplyAmount": "800",                "supplyCurrency": "USDT",                "supplyId": "567",                "supplyTime": "1752482796082",                "term": "7"            }        ],        "nextPageCursor": "567"    },    "retExtInfo": {},    "time": 1752654377461}
```

---

## Collateral Repayment

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/repay-collateral

**Contents:**
- Collateral Repayment
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Collateral Repayment
On this page
Collateral Repayment
Permission: "Spot trade"
UID rate limit: 1 req / second
info
fixed currency offset logic
From Currency Perspective
Orders with the closest maturity date will be sorted in descending order.
If the maturity date is the same, the order with the higher interest rate will be prioritized.
If the interest rates are the same, the order will be processed randomly.Orders will be processed sequentially. Within an order, interest will be repaid first, followed by principal.
From Order Perspective
Interest will be repaid first, followed by principal.
HTTP Request
​
POST
/v5/crypto-loan-fixed/repay-collateral
Request Parameters
​
Parameter
Required
Type
Comments
loanId
false
string
Loan contract ID. If not passed, the fixed currency offset logic will apply.
loanCurrency
true
string
Loan coin name
collateralCoin
true
string
Collateral currencies: Use commas to separate multiple collateral currencies
amount
true
string
Repay amount
Response Parameters
​
Parameter
Type
Comments
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/repay-collateral
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
1752656296791
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
50
{
"loanCurrency"
:
"ETH"
,
"amount"
:
"0.1"
,
"collateralCoin"
:
"USDT"
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
collateral_repayment_fixed_crypto_loan
(
loanCurrency
=
"ETH"
,
amount
=
"0.1"
,
collateralCoin
=
"USDT"
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1756973819393
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/repay-collateral HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752656296791X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 50{  "loanCurrency": "ETH",  "amount": "0.1",  "collateralCoin":"USDT"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.collateral_repayment_fixed_crypto_loan(    loanCurrency="ETH",    amount="0.1",    collateralCoin="USDT",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {},    "retExtInfo": {},    "time": 1756973819393}
```

---

## Cancel Supply Order

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/cancel-supply

**Contents:**
- Cancel Supply Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Cancel Supply Order
On this page
Cancel Supply Order
Permission: "Spot trade"
UID rate limit: 1 req / second
HTTP Request
​
POST
/v5/crypto-loan-fixed/supply-order-cancel
Request Parameters
​
Parameter
Required
Type
Comments
orderId
true
string
Order ID of fixed supply order
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/supply-order-cancel
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
1752652612736
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
26
{
"orderId"
:
"13577"
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
cancel_lending_order_fixed_crypto_loan
(
orderId
=
"13577"
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752652613638
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/supply-order-cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752652612736X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 26{    "orderId": "13577"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.cancel_lending_order_fixed_crypto_loan(    orderId="13577",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {},    "retExtInfo": {},    "time": 1752652613638}
```

---

## Self Match Prevention

**URL:** https://bybit-exchange.github.io/docs/v5/smp

**Contents:**
- Self Match Prevention
- What is SMP?​
- How to set SMP?​
- What is SMP Trade Group?​
- How to manage my UIDs & SMP Trade Group?​

Self Match Prevention
On this page
Self Match Prevention
What is SMP?
​
With the Self Match Prevention function users can choose the execution method when placing an order. When the counterparty
is the same UID or belongs to the same specified SMP group, the execution can be effected accordingly:
Cancel maker: Cancel the maker order when executed; taker order remains.
Cancel taker: Cancel the taker order when executed; maker order remains.
Cancel both: Cancel both taker and maker orders when executed.
Once an order is placed in the orderbook, its smpType becomes invalid. This means that, for example, if you place a taker order without an smpType (
smpType=None
) that matches against your existing maker order set with
smpType=CancelMaker
, the taker will execute. This is because the maker order's
smpType=CancelMaker
became invalid once it placed in the orderbook.
How to set SMP?
​
Check request params of
Place Order
. Specify parameter
smpType
when placing the order
What is SMP Trade Group?
​
SMP is available for any user by UID level.
SMP Trade Group Management is only available for institutions at present.
SMP Trade Group
: refers to a group of UIDs. When any of the UIDs in this group places an order and the SMP execution
policy is selected, it will be triggered when the maker order under any of the UIDs in this group is matched.
More details
:
Each UID can only join one SMP Trade Group.
SMP Trade Group is a UID-level management group, so when a main-account joins an SMP Trade Group, all the subaccounts
under this main-account will automatically join the Trade Group as well.
If the main-account has already joined an SMP Trade Group, and a subaccount is created after it, this new subaccount will automatically join the same SMP Trade Group by default.
A subaccount does not have to be tied to the same SMP Trading Group as the main-account. It is only the default behaviour. It can be reset into different groups manually if needed.
The relationship between SMP Trade Group and UIDs can be changed: when a UID joins a new SMP Trade Group or is
removed from an SMP Trade Group. This operation will not affect the pre-existing orders, it will only affect the newly placed orders after
the relationship has changed.
Notes
:
Based on this, we strongly suggest that when there will be an SMP Trade Group change, you should cancel all
pre-existing orders to avoid an unexpected execution.
The SMP Trade Group has a higher priority on SMP execution, so an individual UID is only taken into account when there is no SMP Trade Group on either side.
Once the order is standing in the orderbook, its SMP flag doesn't matter any more. The system always follows the tag on the latter order.
Examples
:
1st of Jan: UID1 joins SMP Trade Group A, and places Order1;
2nd of Jan: UID1 is removed from SMP Trade Group A, but Order1 is still active and "New"
case1: If UID1 joined SMP Trade Group B, and placed Order2, if Order2 meets Order1, it will be executed since they belong to two different groups.
case2: If UID1 did not join any other groups after being removed from SMP Trade Group A, and placed Order2, if Order2 meets Order1, the SMP will be triggered because UID1 did not have a group when it placed Order2, so SMP was triggered at the UID level (the same UID1).
How to manage my UIDs & SMP Trade Group?
​
You can contact your institutional business manager or email Bybit via:
institutional_services@bybit.com

---

## Get Loan LTV Adjustment History

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/ltv-adjust-history

**Contents:**
- Get Loan LTV Adjustment History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Get Loan LTV Adjustment History
On this page
Get Loan LTV Adjustment History
Query for your LTV adjustment history.
Permission: "Spot trade"
info
Support querying last 6 months adjustment transactions
Only the ltv adjustment transactions launched by the user can be queried
HTTP Request
​
GET
/v5/crypto-loan/adjustment-history
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
adjustId
false
string
Collateral adjustment transaction ID
collateralCurrency
false
string
Collateral coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> collateralCurrency
string
Collateral coin
> orderId
string
Loan order ID
> adjustId
string
Collateral adjustment transaction ID
> adjustTime
string
Adjust timestamp
> preLTV
string
LTV before the adjustment
> afterLTV
string
LTV after the adjustment
> direction
integer
The direction of adjustment,
0
: add collateral;
1
: reduce collateral
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/adjustment-history?adjustId=1794318409405331968
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
1728635871668
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
get_crypto_loan_ltv_adjustment_history
(
adjustId
=
"1794318409405331968"
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
getLoanLTVAdjustmentHistory
(
{
adjustId
:
'1794271131730737664'
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
"request.success"
,
"result"
:
{
"list"
:
[
{
"adjustId"
:
"1794318409405331968"
,
"adjustTime"
:
"1728635422814"
,
"afterLTV"
:
"0.7164"
,
"amount"
:
"0.001"
,
"collateralCurrency"
:
"BTC"
,
"direction"
:
1
,
"orderId"
:
"1794267532472646144"
,
"preLTV"
:
"0.6546"
}
]
,
"nextPageCursor"
:
"1844656778923966466"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1728635873329
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan/adjustment-history?adjustId=1794318409405331968 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728635871668X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_crypto_loan_ltv_adjustment_history(    adjustId="1794318409405331968",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getLoanLTVAdjustmentHistory({ adjustId: '1794271131730737664' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "adjustId": "1794318409405331968",                "adjustTime": "1728635422814",                "afterLTV": "0.7164",                "amount": "0.001",                "collateralCurrency": "BTC",                "direction": 1,                "orderId": "1794267532472646144",                "preLTV": "0.6546"            }        ],        "nextPageCursor": "1844656778923966466"    },    "retExtInfo": {},    "time": 1728635873329}
```

---

## Get Trade Behaviour Setting

**URL:** https://bybit-exchange.github.io/docs/v5/account/get-user-setting-config

**Contents:**
- Get Trade Behaviour Setting
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Get Trade Behaviour Setting
On this page
Get Trade Behaviour Setting
You can get configuration how the system behaves when your limit order price exceeds the highest bid or lowest ask price.
Spot
Maximum Buy Price
:
Min
[Max(Index, Index × (1 + y%) + 2-Minute Average Premium), Index × (1 + z%)]
Lowest price for Sell
:
Max
[Min(Index, Index × (1 – y%) + 2-Minute Average Premium), Index × (1 – z%)]
Futures
Maximum Buy Price
:
min( max( index , markprice
( 1 + x% ）), markprice
( 1 + y%) )
Lowest price for Sell
:
max ( min( index , markprice
( 1 - x% )) , markprice ( 1 - y%) )
Default Setting
Spot:
lpaSpot = false.
If the order price exceeds the boundary, the system rejects the request.
Futures:
lpaPerp = false.
If the order price exceeds the boundary, the system will automatically adjust the price to the nearest allowed boundary (i.e., highest bid or lowest ask).
HTTP Request
​
GET
/v5/account/user-setting-config
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
Object
> lpaSpot
boolean
true
: If the order price exceeds the boundary, the system will automatically adjust the price to the nearest allowed boundary
false
: If the order price exceeds the boundary, the system rejects the request.
> lpaPerp
boolean
true
: If the order price exceeds the boundary, the system rejects the request.
false
: If the order price exceeds the boundary, the system will automatically adjust the price to the nearest allowed boundary
Request Example
​
HTTP
Python
Node.js
GET
/v5/account/user-setting-config
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
1753255927950
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
52
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
"lpaSpot"
:
true
,
"lpaPerp"
:
false
}
,
"retExtInfo"
:
{
}
,
"time"
:
1756794317787
}

**Examples:**

Example 1 ():
```
GET /v5/account/user-setting-config HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1753255927950X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "lpaSpot": true,        "lpaPerp": false    },    "retExtInfo": {},    "time": 1756794317787}
```

---

## Get Trade History

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/trade-list

**Contents:**
- Get Trade History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get Trade History
On this page
Get Trade History
Obtain transaction information.
Up to 50 requests per second
info
Field query priority: rfqId > rfqLinkId  quoteId > quoteLinkId
HTTP Request
​
GET
/v5/rfq/trade-list
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
Custom ID for RFQ; specify rfqLinkId to only check the last 3 months
quoteId
false
string
Quote ID
quoteLinkId
false
string
quote custom ID; specifying quoteLinkId can only check the last 3 months
traderType
false
string
Trader type,
quote
,
request
, default
quote
status
false
string
Status :
Filled
Failed
limit
false
integer
Return the number of items.
[
1
,
100
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
result
Object
> cursor
string
Refer to the
cursor
request parameter
> list
array
An array of RFQs
>> rfqId
string
Inquiry ID
>> rfqLinkId
string
Custom RFQ ID. Not publicly disclosed.
>> quoteId
string
Return the completed RFQ and the executed quote id.
>> quoteLinkId
string
Custom quote ID. Not publicly disclosed.
>> quoteSide
string
Return of completed inquiry, execution of quote direction,
Buy
or
Sell
>> strategyType
string
Inquiry label
>>status
string
Status :
Filled
Failed
>> rfqDeskCode
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
>> quoteDeskCode
string
The unique identification code of the quoting party, which is not visible when anonymous is set to
true
during quotation
>> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
>> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
>> legs
array of objects
Combination transaction
>>> category
string
category. Valid values include:
linear
,
option
and
spot
>>> orderId
string
bybit order id
>>> symbol
string
The unique instrument ID
>>> side
string
Direction, valid values are
Buy
and
Sell
>>> price
string
Execution price
>>> qty
string
Number of executions
>>> markPrice
string
The futures markPrice at the time of transaction, the spot is indexPrice, and the option is the markPrice of the underlying Price.
>>> execFee
string
The fee for taker or maker in the base currency paid to the Exchange executing the Block Trade.
>>> execId
string
The unique exec(trade) ID from the exchange
>>> resultCode
integer
The status code of the this order. "0" means success
>>> resultMessage
string
Error message about resultCode. If resultCode is "0", resultMessage is "".
>>> rejectParty
string
Empty if status is
Filled
.Valid values:
Taker
or
Maker
if status is
Rejected
，"rejectParty='bybit'" to indicate errors that occur on the Bybit side.
Request Example
​
GET
/v5/rfq/trade-list
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
"cursor"
:
""
,
"list"
:
[
{
"rfqId"
:
"1755159541420049734454484077021786"
,
"quoteId"
:
"175515955714692291558309160384918"
,
"quoteSide"
:
"Buy"
,
"strategyType"
:
"PerpBasis"
,
"status"
:
"Failed"
,
"rfqDeskCode"
:
"1nu9d1"
,
"quoteDeskCode"
:
"lines100412673"
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
"BTCUSDT-15AUG25"
,
"side"
:
"Sell"
,
"price"
:
"108887"
,
"qty"
:
"1"
,
"orderId"
:
"db852bcd-052e-49b7-ba10-059622e1219b"
,
"markPrice"
:
""
,
"execFee"
:
"0"
,
"execId"
:
""
,
"resultCode"
:
111002
,
"resultMessage"
:
"Rejected caused by another legs"
,
"rejectParty"
:
""
}
,
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
"price"
:
"132038"
,
"qty"
:
"1"
,
"orderId"
:
"69667acb-7048-48d7-90b9-ccbdfd423130"
,
"markPrice"
:
""
,
"execFee"
:
"0"
,
"execId"
:
""
,
"resultCode"
:
110007
,
"resultMessage"
:
"Insufficient available balance"
,
"rejectParty"
:
"taker"
}
]
,
"createdAt"
:
"1755159541421"
,
"updatedAt"
:
"1755159654501"
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
1756891941267
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/trade-list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "cursor": "",        "list": [            {                "rfqId": "1755159541420049734454484077021786",                "quoteId": "175515955714692291558309160384918",                "quoteSide": "Buy",                "strategyType": "PerpBasis",                "status": "Failed",                "rfqDeskCode": "1nu9d1",                "quoteDeskCode": "lines100412673",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT-15AUG25",                        "side": "Sell",                        "price": "108887",                        "qty": "1",                        "orderId": "db852bcd-052e-49b7-ba10-059622e1219b",                        "markPrice": "",                        "execFee": "0",                        "execId": "",                        "resultCode": 111002,                        "resultMessage": "Rejected caused by another legs",                        "rejectParty": ""                    },                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "price": "132038",                        "qty": "1",                        "orderId": "69667acb-7048-48d7-90b9-ccbdfd423130",                        "markPrice": "",                        "execFee": "0",                        "execId": "",                        "resultCode": 110007,                        "resultMessage": "Insufficient available balance",                        "rejectParty": "taker"                    }                ],                "createdAt": "1755159541421",                "updatedAt": "1755159654501"            }        ]    },    "retExtInfo": {},    "time": 1756891941267}
```

---

## Get RFQs

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/rfq-list

**Contents:**
- Get RFQs
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get RFQs
On this page
Get RFQs
Obtain historical inquiry information.
Up to 50 requests per second
info
Obtain historical RFQs. This data is not real-time. Please see
Get RFQs (real-time)
.
If both rfqId and rfqLinkId are passed, only rfqId is considered.
Sort in reverse order according to the creation time of rfq and return it.
HTTP Request
​
GET
/v5/rfq/rfq-list
Request Parameters
​
Parameter
Required
Type
Comments
rfqId
false**
string
Inquiry ID
rfqLinkId
false
string
Custom ID for RFQ; specify rfqLinkId to only check the last 3 months, traderType is quote, this field is invalid
traderType
false
string
Trader type,
quote
,
request
. Default:
quote
status
false
string
Status of the RFQ:
Active
Canceled
Filled
Expired
Failed
limit
false
integer
Return the number of items.
[
1
,
100
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
result
array
Object
> cursor
string
Refer to the
cursor
request parameter
> list
array
An array of RFQs
>> rfqId
string
Inquiry ID
>> rfqLinkId
string
Custom RFQ ID. Not publicly disclosed.
>> counterparties
array of srings
List of bidders
>> expiresAt
string
The inquiry's expiration time (ms)
>> strategyType
string
Inquiry label
>> status
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
>> deskCode
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
>> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
>> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
>> legs
array of objects
Combination transaction
>>> category
string
category. Valid values include:
linear
,
option
and
spot
>>> symbol
string
The unique instrument ID
>>> side
string
Inquiry direction: Valid values are
Buy
and
Sell
.
>>> qty
string
Order quantity of the instrument.
Request Example
​
GET
/v5/rfq/rfq-list
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
"cursor"
:
""
,
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
,
{
"rfqLinkId"
:
""
,
"rfqId"
:
"1756874158983736854420161904593980"
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
"1756874758985"
,
"status"
:
"Expired"
,
"deskCode"
:
"1nu9d1"
,
"createdAt"
:
"1756874158985"
,
"updatedAt"
:
"1756874764046"
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
,
{
"rfqLinkId"
:
""
,
"rfqId"
:
"1756871488168105512459181956436945"
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
"1756872088171"
,
"status"
:
"Canceled"
,
"deskCode"
:
"1nu9d1"
,
"createdAt"
:
"1756871488171"
,
"updatedAt"
:
"1756871494505"
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
1756885352116
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/rfq-list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "cursor": "",        "list": [            {                "rfqLinkId": "",                "rfqId": "1756885055799241492396882271696580",                "counterparties": [                    "hashwave2"                ],                "strategyType": "custom",                "expiresAt": "1756885655801",                "status": "Active",                "acceptOtherQuoteStatus":"false",                "deskCode": "1nu9d1",                "createdAt": "1756885055801",                "updatedAt": "1756885055801",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "qty": "1"                    }                ]            },            {                "rfqLinkId": "",                "rfqId": "1756874158983736854420161904593980",                "counterparties": [                    "hashwave2"                ],                "strategyType": "custom",                "expiresAt": "1756874758985",                "status": "Expired",                "deskCode": "1nu9d1",                "createdAt": "1756874158985",                "updatedAt": "1756874764046",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "qty": "1"                    }                ]            },            {                "rfqLinkId": "",                "rfqId": "1756871488168105512459181956436945",                "counterparties": [                    "hashwave2"                ],                "strategyType": "custom",                "expiresAt": "1756872088171",                "status": "Canceled",                "deskCode": "1nu9d1",                "createdAt": "1756871488171",                "updatedAt": "1756871494505",                "legs": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "side": "Buy",                        "qty": "1"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1756885352116}
```

---

## Create Supply Order

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/supply

**Contents:**
- Create Supply Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Create Supply Order
On this page
Create Supply Order
Permission: "Spot trade"
UID rate limit: 1 req / second
HTTP Request
​
POST
/v5/crypto-loan-fixed/supply
Request Parameters
​
Parameter
Required
Type
Comments
orderCurrency
true
string
Currency to supply
orderAmount
true
string
Amount to supply
annualRate
true
string
Customizable annual interest rate, e.g.,
0.02
means 2%
term
true
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
Response Parameters
​
Parameter
Type
Comments
orderId
string
Supply order ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/supply
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
1752652261840
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
104
{
"orderCurrency"
:
"USDT"
,
"orderAmount"
:
"2002.21"
,
"annualRate"
:
"0.35"
,
"term"
:
"7"
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
create_lending_order_fixed_crypto_loan
(
orderCurrency
=
"USDT"
,
orderAmount
=
"2002.21"
,
annualRate
=
"0.35"
,
term
=
"7"
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
"orderId"
:
"13007"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752633650147
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/supply HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752652261840X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 104{    "orderCurrency": "USDT",    "orderAmount": "2002.21",    "annualRate": "0.35",    "term": "7"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.create_lending_order_fixed_crypto_loan(    orderCurrency="USDT",    orderAmount="2002.21",    annualRate="0.35",    term="7",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "orderId": "13007"    },    "retExtInfo": {},    "time": 1752633650147}
```

---

## Cancel All Orders

**URL:** https://bybit-exchange.github.io/docs/v5/order/cancel-all

**Contents:**
- Cancel All Orders
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Cancel All Orders
On this page
Cancel All Orders
Cancel all open orders
info
Support cancel orders by
symbol
/
baseCoin
/
settleCoin
. If you pass multiple of these params, the system will process one of param, which priority is
symbol
>
baseCoin
>
settleCoin
.
NOTE
: category=
option
, you can cancel all option open orders without passing any of those three params. However, for "linear" and "inverse", you must specify one of those three params.
NOTE
: category=
spot
, you can cancel all spot open orders (normal order by default) without passing other params.
info
Spot
: no limit
Futures
: cancel up to 500 orders (System
picks up 500 orders randomly to cancel
when you have over 500 orders)
Options
: no limit
HTTP Request
​
POST
/v5/order/cancel-all
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
spot
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
linear
&
inverse
:
Required
if not passing baseCoin or settleCoin
baseCoin
false
string
Base coin, uppercase only.
linear
&
inverse
: If cancel all by baseCoin, it will cancel all of the corresponding category's orders.
Required
if not passing symbol or settleCoin
settleCoin
false
string
Settle coin, uppercase only
linear
&
inverse
:
Required
if not passing symbol or baseCoin
option
: USDT or USDC
Not support
spot
orderFilter
false
string
category=
spot
, you can pass
Order
,
tpslOrder
,
StopOrder
,
OcoOrder
,
BidirectionalTpslOrder
If not passed,
Order
by default
category=
linear
or
inverse
, you can pass
Order
,
StopOrder
,
OpenOrder
If not passed, all kinds of orders will be cancelled, like active order, conditional order, TP/SL order and trailing stop order
category=
option
, you can pass
Order
,
StopOrder
If not passed, all kinds of orders will be cancelled, like active order, conditional order, TP/SL order and trailing stop order
stopOrderType
false
string
Stop order type
Stop
Only used for category=
linear
or
inverse
and orderFilter=
StopOrder
,you can cancel conditional orders except TP/SL order and Trailing stop orders with this param
info
The acknowledgement of create/amend/cancel order requests indicates that the request was sucessfully accepted. The request is asynchronous so please use the websocket to confirm the order status.
RUN >>
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
success
string
"1": success, "0": fail.
UTA1.0
(inverse) does not return this field
Request Example
​
HTTP
Python
Java
.Net
Node.js
POST
/v5/order/cancel-all
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
1672219779140
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
null
,
"settleCoin"
:
"USDT"
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
cancel_all_orders
(
category
=
"linear"
,
settleCoin
=
"USDT"
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
restApi
.
BybitApiTradeRestClient
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
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
var
cancelAllOrdersRequest
=
TradeOrderRequest
.
builder
(
)
.
category
(
ProductType
.
LINEAR
)
.
baseCoin
(
"USDT"
)
.
build
(
)
;
client
.
cancelAllOrder
(
cancelAllOrdersRequest
,
System
.
out
::
println
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");
var orderInfoString = await TradeService.CancelAllOrder(category: Category.LINEAR, baseCoin:"USDT");
Console.WriteLine(orderInfoString);
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
cancelAllOrders
(
{
category
:
'linear'
,
settleCoin
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
"list"
:
[
{
"orderId"
:
"1616024329462743808"
,
"orderLinkId"
:
"1616024329462743809"
}
,
{
"orderId"
:
"1616024287544869632"
,
"orderLinkId"
:
"1616024287544869633"
}
]
,
"success"
:
"1"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1707381118116
}

**Examples:**

Example 1 ():
```
POST /v5/order/cancel-all HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672219779140X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{  "category": "linear",  "symbol": null,  "settleCoin": "USDT"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.cancel_all_orders(    category="linear",    settleCoin="USDT",))
```

Example 3 ():
```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var cancelAllOrdersRequest = TradeOrderRequest.builder().category(ProductType.LINEAR).baseCoin("USDT").build();client.cancelAllOrder(cancelAllOrdersRequest, System.out::println);
```

Example 4 ():
```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfoString = await TradeService.CancelAllOrder(category: Category.LINEAR, baseCoin:"USDT");Console.WriteLine(orderInfoString);
```

---

## Cancel Borrow Order

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/cancel-borrow

**Contents:**
- Cancel Borrow Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Cancel Borrow Order
On this page
Cancel Borrow Order
Permission: "Spot trade"
UID rate limit: 1 req / second
HTTP Request
​
POST
/v5/crypto-loan-fixed/borrow-order-cancel
Request Parameters
​
Parameter
Required
Type
Comments
orderId
true
string
Order ID of fixed borrow order
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/borrow-order-cancel
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
1752652457987
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
26
{
"orderId"
:
"13009"
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
create_lending_order_fixed_crypto_loan
(
orderId
=
"13009"
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752652458684
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/borrow-order-cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752652457987X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 26{    "orderId": "13009"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.create_lending_order_fixed_crypto_loan(    orderId="13009",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {},    "retExtInfo": {},    "time": 1752652458684}
```

---

## Amend Order

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/amend-order

**Contents:**
- Amend Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Amend Order
On this page
Amend Order
info
You can only modify
unfilled
or
partially filled
orders.
HTTP Request
​
POST
/v5/spread/order/amend
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
orderId
false
string
Spread combination order ID. Either
orderId
or
orderLinkId
is
required
orderLinkId
false
string
User customised order ID. Either
orderId
or
orderLinkId
is
required
qty
false
string
Order quantity after modification. Either
qty
or
price
is
required
price
false
string
Order price after modification
Either
qty
or
price
is
required
price="" means the price remains unchanged, while price="0" updates the price to 0.
info
The acknowledgement of an amend order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
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
User customised order ID
Request Example
​
POST
/v5/spread/order/amend
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"orderLinkId"
:
"1744072052193428475"
,
"price"
:
"14"
,
"qty"
:
"0.2"
}
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
"orderId"
:
"b0e6c938-9731-4122-8552-01e6dc06b303"
,
"orderLinkId"
:
"1744072052193428475"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1744083952599
}

**Examples:**

Example 1 ():
```
POST /v5/spread/order/amend HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115{    "symbol": "SOLUSDT_SOL/USDT",    "orderLinkId": "1744072052193428475",    "price": "14",    "qty": "0.2"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "b0e6c938-9731-4122-8552-01e6dc06b303",        "orderLinkId": "1744072052193428475"    },    "retExtInfo": {},    "time": 1744083952599}
```

---

## Get Pre-upgrade Trade History

**URL:** https://bybit-exchange.github.io/docs/v5/pre-upgrade/execution

**Contents:**
- Get Pre-upgrade Trade History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Pre-upgrade
Get Pre-upgrade Trade History
On this page
Get Pre-upgrade Trade History
Get users' execution records which occurred before you upgraded the account to a Unified account, sorted by
execTime
in descending order
It supports to query USDT perpetual, USDC perpetual, Inverse perpetual, Inverse futures, Spot and Option.
By category="linear", you can query USDT Perps, USDC Perps data occurred during classic account
By category="spot", you can query Spot data occurred during classic account
By category="option", you can query Options data occurred during classic account
By category="inverse", you can query Inverse Contract data occurred during
classic account or
UTA1.0
info
USDC Perpeual & Option support the recent 6 months data. Please download older data via GUI
HTTP Request
​
GET
/v5/pre-upgrade/execution/list
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
,
spot
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
orderId
false
string
Order ID
orderLinkId
false
string
User customised order ID
baseCoin
false
string
Base coin, uppercase only. Used for
option
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 7 days by default
Only startTime is passed, return range between startTime and startTime+7 days
Only endTime is passed, return range between endTime-7 days and endTime
If both are passed, the rule is endTime - startTime <= 7 days
endTime
false
integer
The end timestamp (ms)
execType
false
string
Execution type
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
> orderId
string
Order ID
> orderLinkId
string
User customized order ID
> side
string
Side.
Buy
,
Sell
> orderPrice
string
Order price
> orderQty
string
Order qty
> leavesQty
string
The remaining qty not executed
>
orderType
string
Order type.
Market
,
Limit
>
stopOrderType
string
Stop order type. If the order is not stop order, any type is not returned
> execFee
string
Executed trading fee
> execId
string
Execution ID
> execPrice
string
Execution price
> execQty
string
Execution qty
>
execType
string
Executed type
> execValue
string
Executed order value
> execTime
string
Executed timestamp (ms)
> isMaker
boolean
Is maker order.
true
: maker,
false
: taker
> feeRate
string
Trading fee rate
> tradeIv
string
Implied volatility
> markIv
string
Implied volatility of mark price
> markPrice
string
The mark price of the symbol when executing
> indexPrice
string
The index price of the symbol when executing
> underlyingPrice
string
The underlying price of the symbol when executing
> blockTradeId
string
Paradigm block trade ID
> closedSize
string
Closed position size
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
GET
/v5/pre-upgrade/execution/list?category=linear&limit=1&execType=Funding&symbol=BTCUSDT
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
1682580752432
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
"orderId"
:
"1682553600-BTCUSDT-592334-Sell"
,
"orderLinkId"
:
""
,
"side"
:
"Sell"
,
"orderPrice"
:
"0.00"
,
"orderQty"
:
"0.000"
,
"leavesQty"
:
"0.000"
,
"orderType"
:
"UNKNOWN"
,
"stopOrderType"
:
"UNKNOWN"
,
"execFee"
:
"0.6364003"
,
"execId"
:
"11f1c4ed-ff20-4d73-acb7-96e43a917f25"
,
"execPrice"
:
"28399.90"
,
"execQty"
:
"0.011"
,
"execType"
:
"Funding"
,
"execValue"
:
"312.3989"
,
"execTime"
:
"1682553600000"
,
"isMaker"
:
false
,
"feeRate"
:
"0.00203714"
,
"tradeIv"
:
""
,
"markIv"
:
""
,
"markPrice"
:
"28399.90"
,
"indexPrice"
:
""
,
"underlyingPrice"
:
""
,
"blockTradeId"
:
""
,
"closedSize"
:
"0.000"
}
]
,
"nextPageCursor"
:
"page_token%3D96184191%26"
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
1682580752717
}

**Examples:**

Example 1 ():
```
GET /v5/pre-upgrade/execution/list?category=linear&limit=1&execType=Funding&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682580752432X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "symbol": "BTCUSDT",                "orderId": "1682553600-BTCUSDT-592334-Sell",                "orderLinkId": "",                "side": "Sell",                "orderPrice": "0.00",                "orderQty": "0.000",                "leavesQty": "0.000",                "orderType": "UNKNOWN",                "stopOrderType": "UNKNOWN",                "execFee": "0.6364003",                "execId": "11f1c4ed-ff20-4d73-acb7-96e43a917f25",                "execPrice": "28399.90",                "execQty": "0.011",                "execType": "Funding",                "execValue": "312.3989",                "execTime": "1682553600000",                "isMaker": false,                "feeRate": "0.00203714",                "tradeIv": "",                "markIv": "",                "markPrice": "28399.90",                "indexPrice": "",                "underlyingPrice": "",                "blockTradeId": "",                "closedSize": "0.000"            }        ],        "nextPageCursor": "page_token%3D96184191%26",        "category": "linear"    },    "retExtInfo": {},    "time": 1682580752717}
```

---

## How To Start Copy Trading

**URL:** https://bybit-exchange.github.io/docs/v5/copytrade

**Contents:**
- How To Start Copy Trading
- Become A Master Trader​
- Create The API KEY​
- Understand The Scope​
- Place The Copy Trading Order​

How To Start Copy Trading
On this page
How To Start Copy Trading
Become A Master Trader
​
Please go
here
to apply to become a Master Trader
Create The API KEY
​
"Contract - Orders & Positions" are mandatory permissions for Copy Trading orders
Understand The Scope
​
From time being copy trading accounts can only trade USDT Perpetual symbols. Please
check the field
copyTrading
from
Get Instruments Info
Place The Copy Trading Order
​
Use V5
Place Order
endpoint to place a Copy Trading order
POST
/v5/order/create
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
1698376189371
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
207
{
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"category"
:
"linear"
,
"qty"
:
"0.1"
,
"price"
:
"29000"
,
"timeInForce"
:
"GTC"
,
"positionIdx"
:
1
}

**Examples:**

Example 1 ():
```
POST /v5/order/create HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1698376189371X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 207{    "symbol": "BTCUSDT",    "side": "Buy",    "orderType": "Limit",    "category": "linear",    "qty": "0.1",    "price": "29000",    "timeInForce": "GTC",    "positionIdx": 1}
```

---

## Get Borrowing History

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/flexible/loan-orders

**Contents:**
- Get Borrowing History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Flexible Loan
Get Borrowing History
On this page
Get Borrowing History
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-flexible/borrow-history
Request Parameters
​
Parameter
Required
Type
Comments
orderId
false
string
Loan order ID
loanCurrency
false
string
Loan coin name
limit
false
string
Limit for data size per page.
[
1
,
100
]
. Default:
10
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
> borrowTime
long
The timestamp to borrow
> initialLoanAmount
string
Loan amount
> loanCurrency
string
Loan coin
> orderId
string
Loan order ID
> status
integer
Loan order status
1
: success;
2
: processing;
3
: fail
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-flexible/borrow-history?limit=2
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
1752570519918
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
get_borrowing_history_flexible_crypto_loan
(
limit
=
"2"
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
"borrowTime"
:
1752569950643
,
"initialLoanAmount"
:
"0.006"
,
"loanCurrency"
:
"BTC"
,
"orderId"
:
"1364"
,
"status"
:
1
}
,
{
"borrowTime"
:
1752569209643
,
"initialLoanAmount"
:
"0.1"
,
"loanCurrency"
:
"BTC"
,
"orderId"
:
"1363"
,
"status"
:
1
}
]
,
"nextPageCursor"
:
"1363"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752570519414
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-flexible/borrow-history?limit=2 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752570519918X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_borrowing_history_flexible_crypto_loan(    limit="2",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "borrowTime": 1752569950643,                "initialLoanAmount": "0.006",                "loanCurrency": "BTC",                "orderId": "1364",                "status": 1            },            {                "borrowTime": 1752569209643,                "initialLoanAmount": "0.1",                "loanCurrency": "BTC",                "orderId": "1363",                "status": 1            }        ],        "nextPageCursor": "1363"    },    "retExtInfo": {},    "time": 1752570519414}
```

---

## Get Quotes

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/quote-list

**Contents:**
- Get Quotes
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Get Quotes
On this page
Get Quotes
Obtain historical quote information.
Up to 50 requests per second
info
Obtain historical quotes. This data is not real-time. Please see Get RFQs (real-time).
If both quoteId and quoteLinkId are passed, only both is considered.
If both rfqId and rfqLinkId are passed, only rfqId is considered.
Sorted in descending order by createdAt.
HTTP Request
​
GET
/v5/rfq/quote-list
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
Custom quote ID. If traderType is
request
this field is invalid
traderType
false
string
Trader type,
quote
,
request
. Default:
quote
status
false
string
Status of the RFQ:
Active
Canceled
PendingFill
Filled
Expired
Failed
limit
false
integer
Return the number of items.
[
1
,
100
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
result
Object
> cursor
string
Refer to the
cursor
request parameter
> list
array
An array of quotes
>> rfqId
string
Inquiry ID
>> rfqLinkId
string
Custom RFQ ID. Not publicly disclosed.
>> quoteId
string
Quote ID
>> quoteLinkId
string
Custom quote ID. Not publicly disclosed.
>> expiresAt
string
The quote's expiration time (ms)
>> deskCode
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the quote was created
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
Order price in the quote currency of the instrument.
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
Order price in the quote currency of the instrument.
>>> qty
string
Order quantity of the instrument.
Request Example
​
GET
/v5/rfq/quote-list
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
"cursor"
:
""
,
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
"Expired"
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
"1757405999156"
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
1757406548275
}

**Examples:**

Example 1 ():
```
GET /v5/rfq/quote-list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "cursor": "",        "list": [            {                "rfqLinkId": "",                "rfqId": "175740578143743543930777169307022",                "quoteId": "1757405933130044334361923221559805",                "quoteLinkId": "",                "expiresAt": "1757405993126",                "status": "Expired",                "deskCode": "test0904",                "execQuoteSide": "",                "quoteBuyList": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "price": "113790",                        "qty": "0.5"                    }                ],                "quoteSellList": [                    {                        "category": "linear",                        "symbol": "BTCUSDT",                        "price": "110500",                        "qty": "0.5"                    }                ],                "createdAt": "1757405933126",                "updatedAt": "1757405999156"            }        ]    },    "retExtInfo": {},    "time": 1757406548275}
```

---

## Get Max. Allowed Collateral Reduction Amount

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/reduce-max-collateral-amt

**Contents:**
- Get Max. Allowed Collateral Reduction Amount
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Get Max. Allowed Collateral Reduction Amount
On this page
Get Max. Allowed Collateral Reduction Amount
Query for the maximum amount by which collateral may be reduced by.
Permission: "Spot trade"
HTTP Request
​
GET
/v5/crypto-loan/max-collateral-amount
Request Parameters
​
Parameter
Required
Type
Comments
orderId
true
string
Loan coin ID
Response Parameters
​
Parameter
Type
Comments
maxCollateralAmount
string
Max. reduction collateral amount
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/max-collateral-amount?orderId=1794267532472646144
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
1728634289933
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
get_max_allowed_collateral_reduction_amount
(
orderId
=
"1794267532472646144"
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
getMaxAllowedReductionCollateralAmount
(
{
orderId
:
'1794267532472646144'
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
"request.success"
,
"result"
:
{
"maxCollateralAmount"
:
"0.00210611"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1728634291554
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan/max-collateral-amount?orderId=1794267532472646144 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728634289933X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_max_allowed_collateral_reduction_amount(        orderId="1794267532472646144",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getMaxAllowedReductionCollateralAmount({ orderId: '1794267532472646144' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "maxCollateralAmount": "0.00210611"    },    "retExtInfo": {},    "time": 1728634291554}
```

---

## Get Open & Closed Orders

**URL:** https://bybit-exchange.github.io/docs/v5/order/open-order

**Contents:**
- Get Open & Closed Orders
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Get Open & Closed Orders
On this page
Get Open & Closed Orders
Primarily query unfilled or partially filled orders in
real-time
, but also supports querying recent 500 closed status (Cancelled, Filled) orders. Please see the usage of request param
openOnly
.
And to query older order records, please use the
order history
interface.
tip
You can query filled, cancelled, and rejected orders to the most recent 500 orders for spot, linear, inverse and option categories
You can query by symbol, baseCoin, orderId and orderLinkId, and if you pass multiple params, the system will process them according to this priority: orderId > orderLinkId > symbol > baseCoin.
The records are sorted by the
createdTime
from newest to oldest.
info
After a server release or restart, filled, cancelled, and rejected orders of Unified account should only be queried through
order history
.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data delivery
HTTP Request
​
GET
/v5/order/realtime
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
spot
,
option
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only. For
linear
, either
symbol
,
baseCoin
,
settleCoin
is
required
baseCoin
false
string
Base coin, uppercase only
Supports
linear
,
inverse
&
option
option
: it returns all option open orders by default
settleCoin
false
string
Settle coin, uppercase only
linear
: either
symbol
,
baseCoin
or
settleCoin
is
required
spot
: not supported
option
: USDT or USDC
orderId
false
string
Order ID
orderLinkId
false
string
User customised order ID
openOnly
false
integer
0
(default): query open status orders (e.g., New, PartiallyFilled)
only
1
: Query a maximum of recent 500 closed status records are kept under each account each category (e.g., Cancelled, Rejected, Filled orders).
If the Bybit service is restarted due to an update, this part of the data will be cleared and accumulated again, but the order records will still be queried in
order history
openOnly
param will be ignored when query by
orderId
or
orderLinkId
orderFilter
false
string
Order
: active order,
StopOrder
: conditional order for Futures and Spot,
tpslOrder
: spot TP/SL order,
OcoOrder
: Spot oco order,
BidirectionalTpslOrder
: Spot bidirectional TPSL order
all kinds of orders by default
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
nextPageCursor
string
Refer to the
cursor
request parameter
list
array
Object
> orderId
string
Order ID
> orderLinkId
string
User customised order ID
> parentOrderLinkId
string
Indicates the linked parent order for attached take-profit and stop-loss orders. Supported for futures and options.
Amending
take-profit or stop-loss orders does not change the parentOrderLinkId
Futures
: using
set trading stop
to update attached TP/SL from the original order does not change the parentOrderLinkId.
Options
: using
set trading stop
to update attached TP/SL from the original order will change the parentOrderLinkId.
Futures & Options
: if TP/SL is set via
set trading stop
for a position that originally has no attached TP/SL, the parentOrderLinkId is meaningless.
> blockTradeId
string
Paradigm block trade ID
> symbol
string
Symbol name
> price
string
Order price
> qty
string
Order qty
> side
string
Side.
Buy
,
Sell
> isLeverage
string
Whether to borrow
0
: false,
1
: true
>
positionIdx
integer
Position index. Used to identify positions in different position modes.
>
orderStatus
string
Order status
>
createType
string
Order create type
Spot does not have this key
>
cancelType
string
Cancel type
>
rejectReason
string
Reject reason
> avgPrice
string
Average filled price, returns
""
for those orders without avg price
> leavesQty
string
The remaining qty not executed
> leavesValue
string
The estimated value not executed
> cumExecQty
string
Cumulative executed order qty
> cumExecValue
string
Cumulative executed order value
> cumExecFee
string
inverse
,
option
: Cumulative executed trading fee.
linear
,
spot
: Deprecated. Use
cumFeeDetail
instead.
>
timeInForce
string
Time in force
>
orderType
string
Order type.
Market
,
Limit
. For TP/SL orders, is the order type after the order was triggered
>
stopOrderType
string
Stop order type
> orderIv
string
Implied volatility
> marketUnit
string
The unit for
qty
when create
Spot market
orders.
baseCoin
,
quoteCoin
> triggerPrice
string
Trigger price. If
stopOrderType
=
TrailingStop
, it is activate price. Otherwise, it is trigger price
> takeProfit
string
Take profit price
> stopLoss
string
Stop loss price
> tpslMode
string
TP/SL mode,
Full
: entire position for TP/SL.
Partial
: partial position tp/sl. Spot does not have this field, and Option returns always ""
> ocoTriggerBy
string
The trigger type of Spot OCO order.
OcoTriggerByUnknown
,
OcoTriggerByTp
,
OcoTriggerByBySl
> tpLimitPrice
string
The limit order price when take profit price is triggered
> slLimitPrice
string
The limit order price when stop loss price is triggered
>
tpTriggerBy
string
The price type to trigger take profit
>
slTriggerBy
string
The price type to trigger stop loss
> triggerDirection
integer
Trigger direction.
1
: rise,
2
: fall
>
triggerBy
string
The price type of trigger price
> lastPriceOnCreated
string
Last price when place the order, Spot is not applicable
> basePrice
string
Last price when place the order, Spot has this field only
> reduceOnly
boolean
Reduce only.
true
means reduce position size
> closeOnTrigger
boolean
Close on trigger.
What is a close on trigger order?
> placeType
string
Place type,
option
used.
iv
,
price
>
smpType
string
SMP execution type
> smpGroup
integer
Smp group ID. If the UID has no group, it is
0
by default
> smpOrderId
string
The counterparty's orderID which triggers this SMP execution
> createdTime
string
Order created timestamp (ms)
> updatedTime
string
Order updated timestamp (ms)
> cumFeeDetail
json
linear
,
spot
: Cumulative trading fee details instead of
cumExecFee
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
GET /v5/order/realtime?symbol=ETHUSDT&category=linear&openOnly=0&limit=1  HTTP/1.1
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
1672219525810
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
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
get_open_orders
(
category
=
"linear"
,
symbol
=
"ETHUSDT"
,
openOnly
=
0
,
limit
=
1
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
openLinearOrdersResult
=
client
.
getOpenOrders
(
openOrderRequest
.
category
(
CategoryType
.
LINEAR
)
.
openOnly
(
1
)
.
build
(
)
)
;
System
.
out
.
println
(
openLinearOrdersResult
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
getActiveOrders
(
{
category
:
'linear'
,
symbol
:
'ETHUSDT'
,
openOnly
:
0
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
"list"
:
[
{
"orderId"
:
"fd4300ae-7847-404e-b947-b46980a4d140"
,
"orderLinkId"
:
"test-000005"
,
"blockTradeId"
:
""
,
"symbol"
:
"ETHUSDT"
,
"price"
:
"1600.00"
,
"qty"
:
"0.10"
,
"side"
:
"Buy"
,
"isLeverage"
:
""
,
"positionIdx"
:
1
,
"orderStatus"
:
"New"
,
"cancelType"
:
"UNKNOWN"
,
"rejectReason"
:
"EC_NoError"
,
"avgPrice"
:
"0"
,
"leavesQty"
:
"0.10"
,
"leavesValue"
:
"160"
,
"cumExecQty"
:
"0.00"
,
"cumExecValue"
:
"0"
,
"cumExecFee"
:
"0"
,
"timeInForce"
:
"GTC"
,
"orderType"
:
"Limit"
,
"stopOrderType"
:
"UNKNOWN"
,
"orderIv"
:
""
,
"triggerPrice"
:
"0.00"
,
"takeProfit"
:
"2500.00"
,
"stopLoss"
:
"1500.00"
,
"tpTriggerBy"
:
"LastPrice"
,
"slTriggerBy"
:
"LastPrice"
,
"triggerDirection"
:
0
,
"triggerBy"
:
"UNKNOWN"
,
"lastPriceOnCreated"
:
""
,
"reduceOnly"
:
false
,
"closeOnTrigger"
:
false
,
"smpType"
:
"None"
,
"smpGroup"
:
0
,
"smpOrderId"
:
""
,
"tpslMode"
:
"Full"
,
"tpLimitPrice"
:
""
,
"slLimitPrice"
:
""
,
"placeType"
:
""
,
"createdTime"
:
"1684738540559"
,
"updatedTime"
:
"1684738540561"
,
"cumFeeDetail"
:
{
"MNT"
:
"0.00242968"
}
}
]
,
"nextPageCursor"
:
"page_args%3Dfd4300ae-7847-404e-b947-b46980a4d140%26symbol%3D6%26"
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
1684765770483
}

**Examples:**

Example 1 ():
```
GET /v5/order/realtime?symbol=ETHUSDT&category=linear&openOnly=0&limit=1  HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672219525810X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_open_orders(    category="linear",    symbol="ETHUSDT",    openOnly=0,    limit=1,))
```

Example 3 ():
```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var openLinearOrdersResult = client.getOpenOrders(openOrderRequest.category(CategoryType.LINEAR).openOnly(1).build());System.out.println(openLinearOrdersResult);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getActiveOrders({        category: 'linear',        symbol: 'ETHUSDT',        openOnly: 0,        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Create Order

**URL:** https://bybit-exchange.github.io/docs/v5/spread/trade/create-order

**Contents:**
- Create Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spread Trading
Trade
Create Order
On this page
Create Order
Place a spread combination order.
Up to 50 open orders
per account.
HTTP Request
​
POST
/v5/spread/order/create
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
side
true
string
Order side.
Buy
,
Sell
orderType
true
string
Limit
,
Market
qty
true
string
Order qty
price
false
string
Order price
orderLinkId
false
string
User customised order ID, a max of 45 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.
timeInForce
false
string
Time in force
.
IOC
,
FOK
,
GTC
,
PostOnly
info
The acknowledgement of an place order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
Response Parameters
​
Parameter
Type
Comments
orderId
string
Spread combination order ID
orderLinkId
string
User customised order ID
Request Example
​
POST
/v5/spread/order/create
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
1744079410023
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
191
{
"symbol"
:
"SOLUSDT_SOL/USDT"
,
"side"
:
"Buy"
,
"orderType"
:
"Limit"
,
"qty"
:
"0.1"
,
"price"
:
"21"
,
"orderLinkId"
:
"1744072052193428479"
,
"timeInForce"
:
"PostOnly"
}
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
"orderId"
:
"1b00b997-d825-465e-ad1d-80b0eb1955af"
,
"orderLinkId"
:
"1744072052193428479"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1744075839332
}

**Examples:**

Example 1 ():
```
POST /v5/spread/order/create HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744079410023X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 191{    "symbol": "SOLUSDT_SOL/USDT",    "side": "Buy",    "orderType": "Limit",    "qty": "0.1",    "price": "21",    "orderLinkId": "1744072052193428479",    "timeInForce": "PostOnly"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "1b00b997-d825-465e-ad1d-80b0eb1955af",        "orderLinkId": "1744072052193428479"    },    "retExtInfo": {},    "time": 1744075839332}
```

---

## Batch Set Collateral Coin

**URL:** https://bybit-exchange.github.io/docs/v5/account/batch-set-collateral

**Contents:**
- Batch Set Collateral Coin
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Account
Batch Set Collateral Coin
On this page
Batch Set Collateral Coin
HTTP Request
​
POST
/v5/account/set-collateral-switch-batch
Request Parameters
​
Parameter
Required
Type
Comments
request
true
array
Object
> coin
true
string
Coin name, uppercase only
You can get collateral coin from
here
USDT, USDC cannot be set
> collateralSwitch
true
string
ON
: switch on collateral,
OFF
: switch off collateral
Response Parameters
​
Parameter
Type
Comments
result
Object
> list
array
Object
>> coin
string
Coin name
>> collateralSwitch
string
ON
: switch on collateral,
OFF
: switch off collateral
Request Example
​
HTTP
Python
Node.js
POST
/v5/account/set-collateral-switch-batch
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
1704782042755
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
371
{
"request"
:
[
{
"coin"
:
"MATIC"
,
"collateralSwitch"
:
"OFF"
}
,
{
"coin"
:
"BTC"
,
"collateralSwitch"
:
"OFF"
}
,
{
"coin"
:
"ETH"
,
"collateralSwitch"
:
"OFF"
}
,
{
"coin"
:
"SOL"
,
"collateralSwitch"
:
"OFF"
}
]
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
batch_set_collateral_coin
(
request
=
[
{
"coin"
:
"BTC"
,
"collateralSwitch"
:
"ON"
,
}
,
{
"coin"
:
"ETH"
,
"collateralSwitch"
:
"ON"
,
}
]
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
batchSetCollateralCoin
(
{
request
:
[
{
coin
:
'BTC'
,
collateralSwitch
:
'ON'
,
}
,
{
coin
:
'ETH'
,
collateralSwitch
:
'OFF'
,
}
,
]
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
"result"
:
{
"list"
:
[
{
"coin"
:
"MATIC"
,
"collateralSwitch"
:
"OFF"
}
,
{
"coin"
:
"BTC"
,
"collateralSwitch"
:
"OFF"
}
,
{
"coin"
:
"ETH"
,
"collateralSwitch"
:
"OFF"
}
,
{
"coin"
:
"SOL"
,
"collateralSwitch"
:
"OFF"
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
1704782042913
}

**Examples:**

Example 1 ():
```
POST /v5/account/set-collateral-switch-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1704782042755X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 371{    "request": [        {            "coin": "MATIC",            "collateralSwitch": "OFF"        },        {            "coin": "BTC",            "collateralSwitch": "OFF"        },        {            "coin": "ETH",            "collateralSwitch": "OFF"        },        {            "coin": "SOL",            "collateralSwitch": "OFF"        }    ]}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.batch_set_collateral_coin(  request=[    {      "coin": "BTC",      "collateralSwitch": "ON",    },    {      "coin": "ETH",      "collateralSwitch": "ON",    }  ]))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .batchSetCollateralCoin({    request: [      {        coin: 'BTC',        collateralSwitch: 'ON',      },      {        coin: 'ETH',        collateralSwitch: 'OFF',      },    ],  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "SUCCESS",    "result": {        "list": [            {                "coin": "MATIC",                "collateralSwitch": "OFF"            },            {                "coin": "BTC",                "collateralSwitch": "OFF"            },            {                "coin": "ETH",                "collateralSwitch": "OFF"            },            {                "coin": "SOL",                "collateralSwitch": "OFF"            }        ]    },    "retExtInfo": {},    "time": 1704782042913}
```

---

## Accept non-LP Quote

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/accept-other-quote

**Contents:**
- Accept non-LP Quote
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Accept non-LP Quote
On this page
Accept non-LP Quote
Accept non-LP Quote.
Up to 50 requests
per second.
info
Accepts non-LP quotes.
HTTP Request
​
POST
/v5/rfq/accept-other-quote
Request Parameters
​
Parameter
Required
Type
Comments
rfqId
true
string
Inquiry ID
Response Parameters
​
Parameter
Type
Comments
result
object
> rfqId
string
Inquiry ID
Request Example
​
POST
/v5/rfq/accept-other-quote
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"rfqId"
:
"1754364447601610516653123084412812"
,
}
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
"rfqId"
:
"1754364447601610516653123084412812"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1757405933132
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/accept-other-quote HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115{  "rfqId":"1754364447601610516653123084412812", }
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "rfqId": "1754364447601610516653123084412812"    },    "retExtInfo": {},    "time": 1757405933132}
```

---

## Cancel Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/cancel-order

**Contents:**
- Cancel Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Cancel Order
On this page
Cancel Order
important
You must specify
orderId
or
orderLinkId
to cancel the order.
If
orderId
and
orderLinkId
do not match, the system will process
orderId
first.
You can only cancel
unfilled
or
partially filled
orders.
HTTP Request
​
POST
/v5/order/cancel
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
spot
,
option
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
orderId
false
string
Order ID. Either
orderId
or
orderLinkId
is
required
orderLinkId
false
string
User customised order ID. Either
orderId
or
orderLinkId
is
required
orderFilter
false
string
Spot trading
only
Order
tpslOrder
StopOrder
If not passed,
Order
by default
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
User customised order ID
info
The acknowledgement of an cancel order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
RUN >>
Request Example
​
HTTP
Python
Java
.Net
Node.js
POST
/v5/order/cancel
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
1672217376681
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
"BTCPERP"
,
"orderLinkId"
:
null
,
"orderId"
:
"c6f055d9-7f21-4079-913d-e6523a9cfffa"
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
cancel_order
(
category
=
"linear"
,
symbol
=
"BTCPERP"
,
orderId
=
"c6f055d9-7f21-4079-913d-e6523a9cfffa"
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
restApi
.
BybitApiTradeRestClient
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
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
var
cancelOrderRequest
=
TradeOrderRequest
.
builder
(
)
.
category
(
ProductType
.
SPOT
)
.
symbol
(
"XRPUSDT"
)
.
orderId
(
"1523347543495541248"
)
.
build
(
)
;
var
canceledOrder
=
client
.
cancelOrder
(
cancelOrderRequest
)
;
System
.
out
.
println
(
canceledOrder
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");
var orderInfoString = await TradeService.CancelOrder(orderId: "1523347543495541248", category: Category.SPOT, symbol: "XRPUSDT");
Console.WriteLine(orderInfoString);
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
cancelOrder
(
{
category
:
'linear'
,
symbol
:
'BTCPERP'
,
orderId
:
'c6f055d9-7f21-4079-913d-e6523a9cfffa'
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
"orderId"
:
"c6f055d9-7f21-4079-913d-e6523a9cfffa"
,
"orderLinkId"
:
"linear-004"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672217377164
}

**Examples:**

Example 1 ():
```
POST /v5/order/cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672217376681X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{  "category": "linear",  "symbol": "BTCPERP",  "orderLinkId": null,  "orderId":"c6f055d9-7f21-4079-913d-e6523a9cfffa"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.cancel_order(    category="linear",    symbol="BTCPERP",    orderId="c6f055d9-7f21-4079-913d-e6523a9cfffa",))
```

Example 3 ():
```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var cancelOrderRequest = TradeOrderRequest.builder().category(ProductType.SPOT).symbol("XRPUSDT").orderId("1523347543495541248").build();var canceledOrder = client.cancelOrder(cancelOrderRequest);System.out.println(canceledOrder);
```

Example 4 ():
```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfoString = await TradeService.CancelOrder(orderId: "1523347543495541248", category: Category.SPOT, symbol: "XRPUSDT");Console.WriteLine(orderInfoString);
```

---

## Batch Amend Order

**URL:** https://bybit-exchange.github.io/docs/v5/order/batch-amend

**Contents:**
- Batch Amend Order
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Trade
Batch Amend Order
On this page
Batch Amend Order
tip
This endpoint allows you to amend more than one open order in a single request.
You can modify
unfilled
or
partially filled
orders. Conditional orders are not supported.
A maximum of 20 orders (option), 20 orders (inverse), 20 orders (linear), 10 orders (spot) can be amended per request.
HTTP Request
​
POST
/v5/order/amend-batch
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
option
,
spot
,
inverse
request
true
array
Object
> symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
> orderId
false
string
Order ID. Either
orderId
or
orderLinkId
is required
> orderLinkId
false
string
User customised order ID. Either
orderId
or
orderLinkId
is required
> orderIv
false
string
Implied volatility.
option
only
. Pass the real value, e.g for 10%, 0.1 should be passed
> triggerPrice
false
string
For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:
triggerPrice > market price
Else,
triggerPrice < market price
For spot, it is for tpslOrder or stopOrder trigger price
> qty
false
string
Order quantity after modification. Do not pass it if not modify the qty
> price
false
string
Order price after modification. Do not pass it if not modify the price
> tpslMode
false
string
TP/SL mode
Full
: entire position for TP/SL. Then, tpOrderType or slOrderType must be
Market
Partial
: partial position tp/sl. Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is
required
and it must be
Partial
> takeProfit
false
string
Take profit price after modification. If pass "0", it means cancel the existing take profit of the order. Do not pass it if you do not want to modify the take profit
> stopLoss
false
string
Stop loss price after modification. If pass "0", it means cancel the existing stop loss of the order. Do not pass it if you do not want to modify the stop loss
>
tpTriggerBy
false
string
The price type to trigger take profit. When set a take profit, this param is
required
if no initial value for the order
>
slTriggerBy
false
string
The price type to trigger stop loss. When set a take profit, this param is
required
if no initial value for the order
>
triggerBy
false
string
Trigger price type
> tpLimitPrice
false
string
Limit order price when take profit is triggered. Only working when original order sets partial limit tp/sl
> slLimitPrice
false
string
Limit order price when stop loss is triggered. Only working when original order sets partial limit tp/sl
Response Parameters
​
Parameter
Type
Comments
result
Object
> list
array
Object
>> category
string
Product type
>> symbol
string
Symbol name
>> orderId
string
Order ID
>> orderLinkId
string
User customised order ID
retExtInfo
Object
> list
array
Object
>> code
number
Success/error code
>> msg
string
Success/error message
info
The acknowledgement of an amend order request indicates that the request was sucessfully accepted. This request is asynchronous so please use the websocket to confirm the order status.
RUN >>
Request Example
​
HTTP
Python
Java
.Net
Node.js
POST
/v5/order/amend-batch
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
1672222935987
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"category"
:
"option"
,
"request"
:
[
{
"symbol"
:
"ETH-30DEC22-500-C"
,
"qty"
:
null
,
"price"
:
null
,
"orderIv"
:
"6.8"
,
"orderId"
:
"b551f227-7059-4fb5-a6a6-699c04dbd2f2"
}
,
{
"symbol"
:
"ETH-30DEC22-700-C"
,
"qty"
:
null
,
"price"
:
"650"
,
"orderIv"
:
null
,
"orderId"
:
"fa6a595f-1a57-483f-b9d3-30e9c8235a52"
}
]
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
amend_batch_order
(
category
=
"option"
,
request
=
[
{
"category"
:
"option"
,
"symbol"
:
"ETH-30DEC22-500-C"
,
"orderIv"
:
"6.8"
,
"orderId"
:
"b551f227-7059-4fb5-a6a6-699c04dbd2f2"
}
,
{
"category"
:
"option"
,
"symbol"
:
"ETH-30DEC22-700-C"
,
"price"
:
"650"
,
"orderId"
:
"fa6a595f-1a57-483f-b9d3-30e9c8235a52"
}
]
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
restApi
.
BybitApiAsyncTradeRestClient
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
ProductType
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
TradeOrderType
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
import
java
.
util
.
Arrays
;
BybitApiClientFactory
factory
=
BybitApiClientFactory
.
newInstance
(
"YOUR_API_KEY"
,
"YOUR_API_SECRET"
)
;
BybitApiAsyncTradeRestClient
client
=
factory
.
newAsyncTradeRestClient
(
)
;
var
amendOrderRequests
=
Arrays
.
asList
(
TradeOrderRequest
.
builder
(
)
.
symbol
(
"BTC-10FEB23-24000-C"
)
.
qty
(
"0.1"
)
.
price
(
"5"
)
.
orderLinkId
(
"9b381bb1-401"
)
.
build
(
)
,
TradeOrderRequest
.
builder
(
)
.
symbol
(
"BTC-10FEB23-24000-C"
)
.
qty
(
"0.1"
)
.
price
(
"5"
)
.
orderLinkId
(
"82ee86dd-001"
)
.
build
(
)
)
;
var
amendBatchOrders
=
BatchOrderRequest
.
builder
(
)
.
category
(
ProductType
.
OPTION
)
.
request
(
amendOrderRequests
)
.
build
(
)
;
client
.
createBatchOrder
(
amendBatchOrders
,
System
.
out
::
println
)
;
using bybit.net.api.ApiServiceImp;
using bybit.net.api.Models.Trade;
var order1 = new OrderRequest { Symbol = "XRPUSDT", OrderId = "xxxxxxxxxx", Qty = "10", Price = "0.6080" };
var order2 = new OrderRequest { Symbol = "BLZUSDT", OrderId = "xxxxxxxxxx", Qty = "15", Price = "0.6090" };
var orderInfoString = await TradeService.AmendBatchOrder(category:Category.LINEAR, request: new List<OrderRequest> { order1, order2 });
Console.WriteLine(orderInfoString);
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
batchAmendOrders
(
'option'
,
[
{
symbol
:
'ETH-30DEC22-500-C'
,
orderIv
:
'6.8'
,
orderId
:
'b551f227-7059-4fb5-a6a6-699c04dbd2f2'
,
}
,
{
symbol
:
'ETH-30DEC22-700-C'
,
price
:
'650'
,
orderId
:
'fa6a595f-1a57-483f-b9d3-30e9c8235a52'
,
}
,
]
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
"category"
:
"option"
,
"symbol"
:
"ETH-30DEC22-500-C"
,
"orderId"
:
"b551f227-7059-4fb5-a6a6-699c04dbd2f2"
,
"orderLinkId"
:
""
}
,
{
"category"
:
"option"
,
"symbol"
:
"ETH-30DEC22-700-C"
,
"orderId"
:
"fa6a595f-1a57-483f-b9d3-30e9c8235a52"
,
"orderLinkId"
:
""
}
]
}
,
"retExtInfo"
:
{
"list"
:
[
{
"code"
:
0
,
"msg"
:
"OK"
}
,
{
"code"
:
0
,
"msg"
:
"OK"
}
]
}
,
"time"
:
1672222808060
}

**Examples:**

Example 1 ():
```
POST /v5/order/amend-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672222935987X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "option",    "request": [        {            "symbol": "ETH-30DEC22-500-C",            "qty": null,            "price": null,            "orderIv": "6.8",            "orderId": "b551f227-7059-4fb5-a6a6-699c04dbd2f2"        },        {            "symbol": "ETH-30DEC22-700-C",            "qty": null,            "price": "650",            "orderIv": null,            "orderId": "fa6a595f-1a57-483f-b9d3-30e9c8235a52"        }    ]}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.amend_batch_order(    category="option",    request=[        {            "category": "option",            "symbol": "ETH-30DEC22-500-C",            "orderIv": "6.8",            "orderId": "b551f227-7059-4fb5-a6a6-699c04dbd2f2"        },        {            "category": "option",            "symbol": "ETH-30DEC22-700-C",            "price": "650",            "orderId": "fa6a595f-1a57-483f-b9d3-30e9c8235a52"        }    ]))
```

Example 3 ():
```
import com.bybit.api.client.restApi.BybitApiAsyncTradeRestClient;import com.bybit.api.client.domain.ProductType;import com.bybit.api.client.domain.TradeOrderType;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;import java.util.Arrays;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var amendOrderRequests = Arrays.asList(TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").qty("0.1").price("5").orderLinkId("9b381bb1-401").build(),                TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").qty("0.1").price("5").orderLinkId("82ee86dd-001").build());var amendBatchOrders = BatchOrderRequest.builder().category(ProductType.OPTION).request(amendOrderRequests).build();client.createBatchOrder(amendBatchOrders, System.out::println);
```

Example 4 ():
```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;var order1 = new OrderRequest { Symbol = "XRPUSDT", OrderId = "xxxxxxxxxx", Qty = "10", Price = "0.6080" };var order2 = new OrderRequest { Symbol = "BLZUSDT", OrderId = "xxxxxxxxxx", Qty = "15", Price = "0.6090" };var orderInfoString = await TradeService.AmendBatchOrder(category:Category.LINEAR, request: new List<OrderRequest> { order1, order2 });Console.WriteLine(orderInfoString);
```

---

## Cancel Quote

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/trade/cancel-quote

**Contents:**
- Cancel Quote
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

RFQ Trading
Trade
Cancel Quote
On this page
Cancel Quote
Cancel a quote.
Up to 50 requests per second
info
You must pass one of the following params: quoteId, rfqId, and quoteLinkId.
If quoteId, rfqId, and quoteLinkId are all passed, they are read in this priority: quoteId > quoteLinkId > rfqId.
HTTP Request
​
POST
/v5/rfq/cancel-quote
Request Parameters
​
Parameter
Required
Type
Comments
quoteId
false
string
Quote ID
rfqId
false
string
Inquiry ID
quoteLinkId
false
string
Custom quote ID
Response Parameters
​
Parameter
Type
Comments
result
object
rfqId
string
Inquiry ID
quoteId
string
Quote ID
quoteLinkId
string
Custom quote ID
Request Example
​
POST
/v5/rfq/cancel-quote
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
1744083949347
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
115
{
"quoteId"
:
"1754364447601610516653123084412812"
}
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
"rfqId"
:
"175740723913299909861293671607573"
,
"quoteId"
:
"1757407443083427576602342578477746"
,
"quoteLinkId"
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
1757407457635
}

**Examples:**

Example 1 ():
```
POST /v5/rfq/cancel-quote HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1744083949347X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 115{    "quoteId":"1754364447601610516653123084412812"  }
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "rfqId": "175740723913299909861293671607573",        "quoteId": "1757407443083427576602342578477746",        "quoteLinkId": ""    },    "retExtInfo": {},    "time": 1757407457635}
```

---
