# Bybit-V5-Api - Position

**Pages:** 18

---

## Confirm New Risk Limit

**URL:** https://bybit-exchange.github.io/docs/v5/position/confirm-mmr

**Contents:**
- Confirm New Risk Limit
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Confirm New Risk Limit
On this page
Confirm New Risk Limit
It is only applicable when the user is marked as only reducing positions (please see the isReduceOnly field in
the
Get Position Info
interface). After the user actively adjusts the risk level, this interface
is called to try to calculate the adjusted risk level, and if it passes (retCode=0), the system will remove the position reduceOnly mark.
You are recommended to call
Get Position Info
to check
isReduceOnly
field.
HTTP Request
​
POST
/v5/position/confirm-pending-mmr
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
symbol
true
string
Symbol name
Response Parameters
​
None
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/confirm-pending-mmr
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
1698051123673
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
53
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
}
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
confirmNewRiskRequest
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
confirmPositionRiskLimit
(
confirmNewRiskRequest
,
System
.
out
::
println
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1698051124588
}

**Examples:**

Example 1 ():
```
POST /v5/position/confirm-pending-mmr HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1698051123673X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 53{    "category": "linear",    "symbol": "BTCUSDT"}
```

Example 2 ():
```

```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var confirmNewRiskRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").build();client.confirmPositionRiskLimit(confirmNewRiskRequest, System.out::println);
```

Example 4 ():
```

```

---

## Get Status And Leverage

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/status

**Contents:**
- Get Status And Leverage
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Status And Leverage
On this page
Get Status And Leverage
Query the Spot margin status and leverage
HTTP Request
​
GET
/v5/spot-margin-trade/state
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
spotLeverage
string
Spot margin leverage. Returns
""
if the margin trade is turned off
spotMarginMode
string
Spot margin status.
1
: on,
0
: off
effectiveLeverage
string
actual leverage ratio. Precision retains 2 decimal places, truncate downwards
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/spot-margin-trade/state
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
1692696840996
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
spot_margin_trade_get_status_and_leverage
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
getSpotMarginState
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
"spotLeverage"
:
"10"
,
"spotMarginMode"
:
"1"
,
"effectiveLeverage"
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
1692696841231
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/state HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1692696840996X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.spot_margin_trade_get_status_and_leverage())
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSpotMarginState()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "spotLeverage": "10",        "spotMarginMode": "1",        "effectiveLeverage": "1"    },    "retExtInfo": {},    "time": 1692696841231}
```

---

## Set Leverage

**URL:** https://bybit-exchange.github.io/docs/v5/position/leverage

**Contents:**
- Set Leverage
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Set Leverage
On this page
Set Leverage
info
According to the risk limit, leverage affects the maximum position value that can be opened,
that is, the greater the leverage, the smaller the maximum position value that can be opened,
and vice versa.
Learn more
HTTP Request
​
POST
/v5/position/set-leverage
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
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
buyLeverage
true
string
[
1
, max leverage]
one-way mode:
buyLeverage
must be the same as
sellLeverage
Hedge mode:
isolated margin:
buyLeverage
and
sellLeverage
can be different;
cross margin:
buyLeverage
must be the same as
sellLeverage
sellLeverage
true
string
[
1
, max leverage]
RUN >>
Response Parameters
​
None
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/set-leverage
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
1672281605082
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
"buyLeverage"
:
"6"
,
"sellLeverage"
:
"6"
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
set_leverage
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
buyLeverage
=
"6"
,
sellLeverage
=
"6"
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
setLeverageRequest
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
buyLeverage
(
"5"
)
.
sellLeverage
(
"5"
)
.
build
(
)
;
client
.
setPositionLeverage
(
setLeverageRequest
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
setLeverage
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
buyLeverage
:
'6'
,
sellLeverage
:
'6'
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672281607343
}

**Examples:**

Example 1 ():
```
POST /v5/position/set-leverage HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672281605082X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "BTCUSDT",    "buyLeverage": "6",    "sellLeverage": "6"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_leverage(    category="linear",    symbol="BTCUSDT",    buyLeverage="6",    sellLeverage="6",))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setLeverageRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").buyLeverage("5").sellLeverage("5").build();client.setPositionLeverage(setLeverageRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setLeverage({        category: 'linear',        symbol: 'BTCUSDT',        buyLeverage: '6',        sellLeverage: '6',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Crypto Loan Position

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/crypto-loan-position

**Contents:**
- Get Crypto Loan Position
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Get Crypto Loan Position
On this page
Get Crypto Loan Position
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-common/position
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
borrowList
array
Object
> fixedTotalDebt
string
Total debt of fixed loan (coin)
> fixedTotalDebtUSD
string
Total debt of fixed loan (USD)
> flexibleHourlyInterestRate
string
Flebible loan hourly interest rate
> flexibleTotalDebt
string
Total debt of flexible loan (coin)
> flexibleTotalDebtUSD
string
Total debt of flexible loan (USD)
> loanCurrency
string
Loan coin
collateralList
array
Object
> amount
string
Collateral amount in coin
> amountUSD
string
Collateral amount in USD (after tierd collateral ratio calculation)
> currency
string
Collateral coin
ltv
string
LTV
supplyList
array
Object
> amount
string
Supply amount in coin
> amountUSD
string
Supply amount in USD
> currency
string
Supply coin
totalCollateral
string
Total collateral amount (USD)
totalDebt
string
Total debt (fixed + flexible, in USD)
totalSupply
string
Total supply amount (USD)
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-common/position
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
1752628288472
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
get_position_new_crypto_loan
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
"borrowList"
:
[
{
"fixedTotalDebt"
:
"0"
,
"fixedTotalDebtUSD"
:
"0"
,
"flexibleHourlyInterestRate"
:
"0.0000001361462"
,
"flexibleTotalDebt"
:
"0.08800022"
,
"flexibleTotalDebtUSD"
:
"9355.37"
,
"loanCurrency"
:
"BTC"
}
,
{
"fixedTotalDebt"
:
"0.1"
,
"fixedTotalDebtUSD"
:
"282.8"
,
"flexibleHourlyInterestRate"
:
"0.00000188498892"
,
"flexibleTotalDebt"
:
"0"
,
"flexibleTotalDebtUSD"
:
"0"
,
"loanCurrency"
:
"ETH"
}
]
,
"collateralList"
:
[
{
"amount"
:
"0.12"
,
"amountUSD"
:
"9930.11"
,
"currency"
:
"BTC"
}
,
{
"amount"
:
"2"
,
"amountUSD"
:
"4524.81"
,
"currency"
:
"ETH"
}
,
{
"amount"
:
"4002.12"
,
"amountUSD"
:
"3201.69"
,
"currency"
:
"USDT"
}
,
{
"amount"
:
"1000"
,
"amountUSD"
:
"724.8"
,
"currency"
:
"USDC"
}
]
,
"ltv"
:
"0.524344"
,
"supplyList"
:
[
{
"amount"
:
"800.13041095890410959"
,
"amountUSD"
:
"800.13"
,
"currency"
:
"USDT"
}
]
,
"totalCollateral"
:
"18381.41"
,
"totalDebt"
:
"9638.17"
,
"totalSupply"
:
"800.13"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752627962000
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-common/position HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752628288472X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_position_new_crypto_loan())
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "borrowList": [            {                "fixedTotalDebt": "0",                "fixedTotalDebtUSD": "0",                "flexibleHourlyInterestRate": "0.0000001361462",                "flexibleTotalDebt": "0.08800022",                "flexibleTotalDebtUSD": "9355.37",                "loanCurrency": "BTC"            },            {                "fixedTotalDebt": "0.1",                "fixedTotalDebtUSD": "282.8",                "flexibleHourlyInterestRate": "0.00000188498892",                "flexibleTotalDebt": "0",                "flexibleTotalDebtUSD": "0",                "loanCurrency": "ETH"            }        ],        "collateralList": [            {                "amount": "0.12",                "amountUSD": "9930.11",                "currency": "BTC"            },            {                "amount": "2",                "amountUSD": "4524.81",                "currency": "ETH"            },            {                "amount": "4002.12",                "amountUSD": "3201.69",                "currency": "USDT"            },            {                "amount": "1000",                "amountUSD": "724.8",                "currency": "USDC"            }        ],        "ltv": "0.524344",        "supplyList": [            {                "amount": "800.13041095890410959",                "amountUSD": "800.13",                "currency": "USDT"            }        ],        "totalCollateral": "18381.41",        "totalDebt": "9638.17",        "totalSupply": "800.13"    },    "retExtInfo": {},    "time": 1752627962000}
```

---

## Get Pre-upgrade Closed PnL

**URL:** https://bybit-exchange.github.io/docs/v5/pre-upgrade/close-pnl

**Contents:**
- Get Pre-upgrade Closed PnL
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Pre-upgrade
Get Pre-upgrade Closed PnL
On this page
Get Pre-upgrade Closed PnL
Query user's closed profit and loss records from before you upgraded the account to a Unified account. The results are sorted by
updatedTime
in descending order.
it only supports to query USDT perpetual, Inverse perpetual and Inverse Futures.
info
By
category
="linear", you can query USDT Perps data occurred during classic account
By
category
="inverse", you can query Inverse Contract data occurred during
classic account or
UTA1.0
HTTP Request
​
GET
/v5/pre-upgrade/position/closed-pnl
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
> side
string
Buy
,
Side
> qty
string
Order qty
> orderPrice
string
Order price
>
orderType
string
Order type.
Market
,
Limit
> execType
string
Exec type.
Trade
,
BustTrade
,
SessionSettlePnL
,
Settle
> closedSize
string
Closed size
> cumEntryValue
string
Cumulated Position value
> avgEntryPrice
string
Average entry price
> cumExitValue
string
Cumulated exit position value
> avgExitPrice
string
Average exit price
> closedPnl
string
Closed PnL
> fillCount
string
The number of fills in a single order
> leverage
string
leverage
> createdTime
string
The created time (ms)
> updatedTime
string
The updated time (ms)
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
GET
/v5/pre-upgrade/position/closed-pnl?category=linear&symbol=BTCUSDT
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
1682580911998
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
"67836246-460e-4c52-a009-af0c3e1d12bc"
,
"side"
:
"Sell"
,
"qty"
:
"0.200"
,
"orderPrice"
:
"27203.40"
,
"orderType"
:
"Market"
,
"execType"
:
"Trade"
,
"closedSize"
:
"0.200"
,
"cumEntryValue"
:
"5588.88"
,
"avgEntryPrice"
:
"27944.40"
,
"cumExitValue"
:
"5726.4252"
,
"avgExitPrice"
:
"28632.13"
,
"closedPnl"
:
"204.25510011"
,
"fillCount"
:
"22"
,
"leverage"
:
"10"
,
"createdTime"
:
"1682487465732"
,
"updatedTime"
:
"1682487465732"
}
]
,
"category"
:
"linear"
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
1682580912259
}

**Examples:**

Example 1 ():
```
GET /v5/pre-upgrade/position/closed-pnl?category=linear&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682580911998X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "symbol": "BTCUSDT",                "orderId": "67836246-460e-4c52-a009-af0c3e1d12bc",                "side": "Sell",                "qty": "0.200",                "orderPrice": "27203.40",                "orderType": "Market",                "execType": "Trade",                "closedSize": "0.200",                "cumEntryValue": "5588.88",                "avgEntryPrice": "27944.40",                "cumExitValue": "5726.4252",                "avgExitPrice": "28632.13",                "closedPnl": "204.25510011",                "fillCount": "22",                "leverage": "10",                "createdTime": "1682487465732",                "updatedTime": "1682487465732"            }        ],        "category": "linear",        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1682580912259}
```

---

## Get Position Tiers

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/position-tiers

**Contents:**
- Get Position Tiers
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Position Tiers
On this page
Get Position Tiers
info
If
currency
is passed in the input parameter, query by currency; if
currency
is not passed in the input parameter, query all configured currencies
HTTP Request
​
GET
/v5/spot-margin-trade/position-tiers
Request Parameters
​
Parameter
Required
Type
Comments
currency
false
string
Coin name, uppercase only
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> currency
string
Coin name, uppercase only
> positionTiersRatioList
string
Object
>> tier
string
Tiers. Display from small to large
>> borrowLimit
string
Tiers Accumulation Borrow limit
>> positionMMR
string
Loan Maintenance Margin Rate. Precision 8 decimal places
>> positionIMR
string
Loan Initial Margin Rate. Precision 8 decimal places
>> maxLeverage
string
Max Loan Leverage
Request Example
​
HTTP
Python
Node.js
GET
/v5/spot-margin-trade/position-tiers?currency=BTC
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
1692696840996
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
"currency"
:
"BTC"
,
"positionTiersRatioList"
:
[
{
"tier"
:
"1"
,
"borrowLimit"
:
"390"
,
"positionMMR"
:
"0.04"
,
"positionIMR"
:
"0.2"
,
"maxLeverage"
:
"5"
}
,
{
"tier"
:
"2"
,
"borrowLimit"
:
"391"
,
"positionMMR"
:
"0.04"
,
"positionIMR"
:
"0.25"
,
"maxLeverage"
:
"4"
}
,
{
"tier"
:
"3"
,
"borrowLimit"
:
"392"
,
"positionMMR"
:
"0.04"
,
"positionIMR"
:
"0.33333333"
,
"maxLeverage"
:
"3"
}
,
{
"tier"
:
"4"
,
"borrowLimit"
:
"393"
,
"positionMMR"
:
"0.04"
,
"positionIMR"
:
"0.5"
,
"maxLeverage"
:
"2"
}
]
}
]
}
,
"retExtInfo"
:
"{}"
,
"time"
:
1756272543440
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/position-tiers?currency=BTC HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1692696840996X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "currency": "BTC",                "positionTiersRatioList": [                    {                        "tier": "1",                        "borrowLimit": "390",                        "positionMMR": "0.04",                        "positionIMR": "0.2",                        "maxLeverage": "5"                    },                    {                        "tier": "2",                        "borrowLimit": "391",                        "positionMMR": "0.04",                        "positionIMR": "0.25",                        "maxLeverage": "4"                    },                    {                        "tier": "3",                        "borrowLimit": "392",                        "positionMMR": "0.04",                        "positionIMR": "0.33333333",                        "maxLeverage": "3"                    },                    {                        "tier": "4",                        "borrowLimit": "393",                        "positionMMR": "0.04",                        "positionIMR": "0.5",                        "maxLeverage": "2"                    }                ]            }        ]    },    "retExtInfo": "{}",    "time": 1756272543440}
```

---

## Get Staked Position

**URL:** https://bybit-exchange.github.io/docs/v5/earn/position

**Contents:**
- Get Staked Position
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Earn
Get Staked Position
On this page
Get Staked Position
info
API key needs "Earn" permission
note
For Flexible Saving, fully redeemed position is also returned in the response
For Onchain, only active position will be returned in the response
HTTP Request
​
GET
/v5/earn/position
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
productId
false
string
Product ID
coin
false
string
Coin name
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
> productId
string
Product ID
> amount
string
Total staked amount
> totalPnl
string
Return the profit of the current position. Only has value in Onchain non-LST mode
> claimableYield
string
Yield accrues on an hourly basis and is distributed at 00:30 UTC daily. If you unstake your assets before yield distribution, any undistributed yield will be credited to your account along with your principal. Onchain products do not return values
> id
string
Position Id. Only for Onchain
> status
string
Processing
,
Active
. Only for Onchain
> orderId
string
Order Id. Only for Onchain
> estimateRedeemTime
string
Estimate redeem time, in milliseconds. Only for Onchain
> estimateStakeTime
string
Estimate stake time, in milliseconds. Only for Onchain
> estimateInterestCalculationTime
string
Estimated Interest accrual time, in milliseconds. Only for Onchain
> settlementTime
string
Settlement time, in milliseconds. Only has value for Onchain
Fixed
product
Request Example
​
HTTP
Python
Node.js
GET
/v5/earn/position?category=FlexibleSaving&coin=USDT
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
1739944576277
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
get_staked_position
(
category
=
"FlexibleSaving"
,
coin
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
"BTC"
,
"productId"
:
"8"
,
"amount"
:
"0.1"
,
"totalPnl"
:
"0.000027397260273973"
,
"claimableYield"
:
"0"
,
"id"
:
"326"
,
"status"
:
"Active"
,
"orderId"
:
"1a5a8945-e042-4dd5-a93f-c0f0577377ad"
,
"estimateRedeemTime"
:
""
,
"estimateStakeTime"
:
""
,
"estimateInterestCalculationTime"
:
"1744243200000"
,
"settlementTime"
:
"1744675200000"
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
1739944577575
}

**Examples:**

Example 1 ():
```
GET /v5/earn/position?category=FlexibleSaving&coin=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739944576277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_staked_position(    category="FlexibleSaving",    coin="USDT",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "coin": "BTC",                "productId": "8",                "amount": "0.1",                "totalPnl": "0.000027397260273973",                "claimableYield": "0",                "id": "326",                "status": "Active",                "orderId": "1a5a8945-e042-4dd5-a93f-c0f0577377ad",                "estimateRedeemTime": "",                "estimateStakeTime": "",                "estimateInterestCalculationTime": "1744243200000",                "settlementTime": "1744675200000"            }        ]    },    "retExtInfo": {},    "time": 1739944577575}
```

---

## Set Trading Stop

**URL:** https://bybit-exchange.github.io/docs/v5/position/trading-stop

**Contents:**
- Set Trading Stop
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Set Trading Stop
On this page
Set Trading Stop
Set the take profit, stop loss or trailing stop for the position.
tip
Passing these parameters will create conditional orders by the system internally. The system will cancel these orders if the position is closed, and adjust the qty according to the size of the open position.
info
New version of TP/SL function supports both holding entire position TP/SL orders and holding partial position TP/SL orders.
Full position TP/SL orders: This API can be used to modify the parameters of existing TP/SL orders.
Partial position TP/SL orders: This API can only add partial position TP/SL orders.
note
Under the new version of TP/SL function, when calling this API to perform one-sided take profit or stop loss modification
on existing TP/SL orders on the holding position, it will cause the paired tp/sl orders to lose binding relationship.
This means that when calling the cancel API through the tp/sl order ID, it will only cancel the corresponding one-sided
take profit or stop loss order ID.
HTTP Request
​
POST
/v5/position/trading-stop
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
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
tpslMode
true
string
TP/SL mode
Full
: entire position TP/SL
Partial
: partial position TP/SL
positionIdx
true
integer
Used to identify positions in different position modes.
0
: one-way mode
1
: hedge-mode Buy side
2
: hedge-mode Sell side
takeProfit
false
string
Cannot be less than 0, 0 means cancel TP
stopLoss
false
string
Cannot be less than 0, 0 means cancel SL
trailingStop
false
string
Trailing stop by price distance. Cannot be less than 0, 0 means cancel TS
tpTriggerBy
false
string
Take profit trigger price type
slTriggerBy
false
string
Stop loss trigger price type
activePrice
false
string
Trailing stop trigger price. Trailing stop will be triggered when this price is reached
only
tpSize
false
string
Take profit size
valid for TP/SL partial mode, note: the value of tpSize and slSize must equal
slSize
false
string
Stop loss size
valid for TP/SL partial mode, note: the value of tpSize and slSize must equal
tpLimitPrice
false
string
The limit order price when take profit price is triggered. Only works when tpslMode=Partial and tpOrderType=Limit
slLimitPrice
false
string
The limit order price when stop loss price is triggered. Only works when tpslMode=Partial and slOrderType=Limit
tpOrderType
false
string
The order type when take profit is triggered.
Market
(default),
Limit
For tpslMode=Full, it only supports tpOrderType="Market"
slOrderType
false
string
The order type when stop loss is triggered.
Market
(default),
Limit
For tpslMode=Full, it only supports slOrderType="Market"
Response Parameters
​
None
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/trading-stop
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
1672283124270
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
"XRPUSDT"
,
"takeProfit"
:
"0.6"
,
"stopLoss"
:
"0.2"
,
"tpTriggerBy"
:
"MarkPrice"
,
"slTriggerBy"
:
"IndexPrice"
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
"tpSize"
:
"50"
,
"slSize"
:
"50"
,
"tpLimitPrice"
:
"0.57"
,
"slLimitPrice"
:
"0.21"
,
"positionIdx"
:
0
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
set_trading_stop
(
category
=
"linear"
,
symbol
=
"XRPUSDT"
,
takeProfit
=
"0.6"
,
stopLoss
=
"0.2"
,
tpTriggerBy
=
"MarkPrice"
,
slTriggerB
=
"IndexPrice"
,
tpslMode
=
"Partial"
,
tpOrderType
=
"Limit"
,
slOrderType
=
"Limit"
,
tpSize
=
"50"
,
slSize
=
"50"
,
tpLimitPrice
=
"0.57"
,
slLimitPrice
=
"0.21"
,
positionIdx
=
0
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
setTradingStopRequest
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
"XRPUSDT"
)
.
takeProfit
(
"0.6"
)
.
stopLoss
(
"0.2"
)
.
tpTriggerBy
(
TriggerBy
.
MARK_PRICE
)
.
slTriggerBy
(
TriggerBy
.
LAST_PRICE
)
.
tpslMode
(
TpslMode
.
PARTIAL
)
.
tpOrderType
(
TradeOrderType
.
LIMIT
)
.
slOrderType
(
TradeOrderType
.
LIMIT
)
.
tpSize
(
"50"
)
.
slSize
(
"50"
)
.
tpLimitPrice
(
"0.57"
)
.
slLimitPrice
(
"0.21"
)
.
build
(
)
;
client
.
setTradingStop
(
setTradingStopRequest
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
setTradingStop
(
{
category
:
'linear'
,
symbol
:
'XRPUSDT'
,
takeProfit
:
'0.6'
,
stopLoss
:
'0.2'
,
tpTriggerBy
:
'MarkPrice'
,
slTriggerBy
:
'IndexPrice'
,
tpslMode
:
'Partial'
,
tpOrderType
:
'Limit'
,
slOrderType
:
'Limit'
,
tpSize
:
'50'
,
slSize
:
'50'
,
tpLimitPrice
:
'0.57'
,
slLimitPrice
:
'0.21'
,
positionIdx
:
0
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672283125359
}

**Examples:**

Example 1 ():
```
POST /v5/position/trading-stop HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672283124270X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category":"linear",    "symbol": "XRPUSDT",    "takeProfit": "0.6",    "stopLoss": "0.2",    "tpTriggerBy": "MarkPrice",    "slTriggerBy": "IndexPrice",    "tpslMode": "Partial",    "tpOrderType": "Limit",    "slOrderType": "Limit",    "tpSize": "50",    "slSize": "50",    "tpLimitPrice": "0.57",    "slLimitPrice": "0.21",    "positionIdx": 0}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_trading_stop(    category="linear",    symbol="XRPUSDT",    takeProfit="0.6",    stopLoss="0.2",    tpTriggerBy="MarkPrice",    slTriggerB="IndexPrice",    tpslMode="Partial",    tpOrderType="Limit",    slOrderType="Limit",    tpSize="50",    slSize="50",    tpLimitPrice="0.57",    slLimitPrice="0.21",    positionIdx=0,))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setTradingStopRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("XRPUSDT").takeProfit("0.6").stopLoss("0.2").tpTriggerBy(TriggerBy.MARK_PRICE).slTriggerBy(TriggerBy.LAST_PRICE)                .tpslMode(TpslMode.PARTIAL).tpOrderType(TradeOrderType.LIMIT).slOrderType(TradeOrderType.LIMIT).tpSize("50").slSize("50").tpLimitPrice("0.57").slLimitPrice("0.21").build();client.setTradingStop(setTradingStopRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setTradingStop({        category: 'linear',        symbol: 'XRPUSDT',        takeProfit: '0.6',        stopLoss: '0.2',        tpTriggerBy: 'MarkPrice',        slTriggerBy: 'IndexPrice',        tpslMode: 'Partial',        tpOrderType: 'Limit',        slOrderType: 'Limit',        tpSize: '50',        slSize: '50',        tpLimitPrice: '0.57',        slLimitPrice: '0.21',        positionIdx: 0,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Position

**URL:** https://bybit-exchange.github.io/docs/v5/websocket/private/position

**Contents:**
- Position
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

WebSocket Stream
Private
Position
On this page
Position
Subscribe to the position stream to see changes to your position data in
real-time
.
All-In-One Topic:
position
Categorised Topic:
position.linear
,
position.inverse
,
position.option
info
All-In-One topic and Categorised topic
cannot
be in the same subscription request
All-In-One topic: Allow you to listen to all categories (linear, inverse, option) websocket updates
Categorised Topic: Allow you to listen only to specific category websocket updates
tip
Every time when you create/amend/cancel an order, the position topic will generate a new message (regardless if there's any actual change)
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
linear
,
inverse
,
option
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
Position size
>
positionIdx
integer
Used to identify positions in different position modes
> positionValue
string
Position value
> riskId
integer
Risk tier ID
for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
> riskLimitValue
string
Risk limit value, become meaningless when auto risk-limit tier is applied
for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
> entryPrice
string
Average entry price
For USDC Perp & Futures, it indicates average entry price, and it will not be changed with 8-hour session settlement
> markPrice
string
Mark price
> leverage
string
Position leverage
for portfolio margin mode, this field returns "", which means leverage rules are invalid
> breakEvenPrice
string
Break even price, only for
linear
,
inverse
.
breakeven_price = (entry_price
qty - realized_pnl) / (qty - abs(qty)
max(taker fee rate, 0.00055))
> autoAddMargin
integer
Whether to add margin automatically when using isolated margin mode
0
: false
1
: true
> positionIM
string
Initial margin, the same value as
positionIMByMp
, please note this change
The New Margin Calculation: Adjustments and Implications
Portfolio margin mode: returns ""
> positionMM
string
Maintenance margin, the same value as
positionMMByMp
Portfolio margin mode: returns ""
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
> takeProfit
string
Take profit price
> stopLoss
string
Stop loss price
> trailingStop
string
Trailing stop
> unrealisedPnl
string
Unrealised profit and loss
> curRealisedPnl
string
The realised PnL for the current holding position
> sessionAvgPrice
string
USDC contract session avg price, it is the same figure as avg entry price shown in the web UI
> delta
string
Delta. It is only pushed when you subscribe to the option position.
> gamma
string
Gamma. It is only pushed when you subscribe to the option position.
> vega
string
Vega. It is only pushed when you subscribe to the option position.
> theta
string
Theta. It is only pushed when you subscribe to the option position.
> cumRealisedPnl
string
Cumulative realised pnl
Futures & Perp: it is the all time cumulative realised P&L
Option: it is the realised P&L when you hold that position
>
positionStatus
string
Position status.
Normal
,
Liq
,
Adl
>
adlRankIndicator
integer
Auto-deleverage rank indicator.
What is Auto-Deleveraging?
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
> createdTime
string
Timestamp of the first time a position was created on this symbol (ms)
> updatedTime
string
Position data updated timestamp (ms)
> seq
long
Cross sequence, used to associate each fill and each position update
Different symbols may have the same seq, please use seq + symbol to check unique
Returns
"-1"
if the symbol has never been traded
Returns the seq updated by the last transaction when there are setting like leverage, risk limit
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
> positionIMByMp
string
Initial margin calculated by mark price, the same value as
positionIM
Portfolio margin mode: returns ""
> positionMMByMp
string
Maintenance margin calculated by mark price, the same value as
positionMM
Portfolio margin mode: returns ""
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
"position"
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
position_stream
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
"1003076014fb7eedb-c7e6-45d6-a8c1-270f0169171a"
,
"topic"
:
"position"
,
"creationTime"
:
1697682317044
,
"data"
:
[
{
"positionIdx"
:
2
,
"tradeMode"
:
0
,
"riskId"
:
1
,
"riskLimitValue"
:
"2000000"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
""
,
"size"
:
"0"
,
"entryPrice"
:
"0"
,
"leverage"
:
"10"
,
"breakEvenPrice"
:
"93556.73034991"
,
"positionValue"
:
"0"
,
"positionBalance"
:
"0"
,
"markPrice"
:
"28184.5"
,
"positionIM"
:
"0"
,
"positionIMByMp"
:
"0"
,
"positionMM"
:
"0"
,
"positionMMByMp"
:
"0"
,
"takeProfit"
:
"0"
,
"stopLoss"
:
"0"
,
"trailingStop"
:
"0"
,
"unrealisedPnl"
:
"0"
,
"curRealisedPnl"
:
"1.26"
,
"cumRealisedPnl"
:
"-25.06579337"
,
"sessionAvgPrice"
:
"0"
,
"createdTime"
:
"1694402496913"
,
"updatedTime"
:
"1697682317038"
,
"tpslMode"
:
"Full"
,
"liqPrice"
:
"0"
,
"bustPrice"
:
""
,
"category"
:
"linear"
,
"positionStatus"
:
"Normal"
,
"adlRankIndicator"
:
0
,
"autoAddMargin"
:
0
,
"leverageSysUpdatedTime"
:
""
,
"mmrSysUpdatedTime"
:
""
,
"seq"
:
8327597863
,
"isReduceOnly"
:
false
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "position"    ]}
```

Example 2 ():
```
from pybit.unified_trading import WebSocketfrom time import sleepws = WebSocket(    testnet=True,    channel_type="private",    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)def handle_message(message):    print(message)ws.position_stream(callback=handle_message)while True:    sleep(1)
```

Example 3 ():
```
{    "id": "1003076014fb7eedb-c7e6-45d6-a8c1-270f0169171a",    "topic": "position",    "creationTime": 1697682317044,    "data": [        {            "positionIdx": 2,            "tradeMode": 0,            "riskId": 1,            "riskLimitValue": "2000000",            "symbol": "BTCUSDT",            "side": "",            "size": "0",            "entryPrice": "0",            "leverage": "10",            "breakEvenPrice":"93556.73034991",            "positionValue": "0",            "positionBalance": "0",            "markPrice": "28184.5",            "positionIM": "0",            "positionIMByMp": "0",            "positionMM": "0",            "positionMMByMp": "0",            "takeProfit": "0",            "stopLoss": "0",            "trailingStop": "0",            "unrealisedPnl": "0",            "curRealisedPnl": "1.26",            "cumRealisedPnl": "-25.06579337",            "sessionAvgPrice": "0",            "createdTime": "1694402496913",            "updatedTime": "1697682317038",            "tpslMode": "Full",            "liqPrice": "0",            "bustPrice": "",            "category": "linear",            "positionStatus": "Normal",            "adlRankIndicator": 0,            "autoAddMargin": 0,            "leverageSysUpdatedTime": "",            "mmrSysUpdatedTime": "",            "seq": 8327597863,            "isReduceOnly": false        }    ]}
```

---

## Get Closed Options Positions

**URL:** https://bybit-exchange.github.io/docs/v5/position/close-position

**Contents:**
- Get Closed Options Positions
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Get Closed Options Positions (6 months)
On this page
Get Closed Options Positions
Query user's closed options positions, sorted by
closeTime
in descending order.
info
Only supports users to query closed options positions in the last 6 months.
Fee and price are displayed with trailing zeroes up to 8 decimal places.
HTTP Request
​
GET
/v5/position/get-closed-positions
Request Parameters
​
Parameter
Required
Type
Comments
category
true
string
option
symbol
false
string
Symbol name
startTime
false
integer
The start timestamp (ms)
startTime and endTime are not passed, return 1 days by default
Only startTime is passed, return range between startTime and startTime+1 days
Only endTime is passed, return range between endTime-1 days and endTime
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
nextPageCursor
string
Refer to the
cursor
request parameter
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
> totalOpenFee
string
Total open fee
> deliveryFee
string
Delivery fee
> totalCloseFee
string
Total close fee
> qty
string
Order qty
> closeTime
integer
The closed time (ms)
> avgExitPrice
string
Average exit price
> deliveryPrice
string
Delivery price
> openTime
integer
The opened time (ms)
> avgEntryPrice
string
Average entry price
> totalPnl
string
Total PnL
Request Example
​
HTTP
Python
GET
/v5/position/get-closed-positions?category=option&limit=1
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
1672284128523
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
get_closed_options_positions
(
category
=
"option"
,
limit
=
"1"
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
"nextPageCursor"
:
"1749726002161%3A0%2C1749715220240%3A1"
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
"BTC-12JUN25-104019-C-USDT"
,
"side"
:
"Sell"
,
"totalOpenFee"
:
"0.94506647"
,
"deliveryFee"
:
"0.32184533"
,
"totalCloseFee"
:
"0.00000000"
,
"qty"
:
"0.02"
,
"closeTime"
:
1749726002161
,
"avgExitPrice"
:
"107281.77405000"
,
"deliveryPrice"
:
"107281.77405031"
,
"openTime"
:
1749722990063
,
"avgEntryPrice"
:
"3371.50000000"
,
"totalPnl"
:
"0.90760719"
}
,
{
"symbol"
:
"BTC-12JUN25-104000-C-USDT"
,
"side"
:
"Buy"
,
"totalOpenFee"
:
"0.86379999"
,
"deliveryFee"
:
"0.32287622"
,
"totalCloseFee"
:
"0.00000000"
,
"qty"
:
"0.02"
,
"closeTime"
:
1749715220240
,
"avgExitPrice"
:
"107625.40470150"
,
"deliveryPrice"
:
"107625.40470159"
,
"openTime"
:
1749710568608
,
"avgEntryPrice"
:
"3946.50000000"
,
"totalPnl"
:
"-7.60858218"
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
1749736532193
}

**Examples:**

Example 1 ():
```
GET /v5/position/get-closed-positions?category=option&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672284128523X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_closed_options_positions(    category="option",    limit="1",))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "1749726002161%3A0%2C1749715220240%3A1",        "category": "option",        "list": [            {                "symbol": "BTC-12JUN25-104019-C-USDT",                "side": "Sell",                "totalOpenFee": "0.94506647",                "deliveryFee": "0.32184533",                "totalCloseFee": "0.00000000",                "qty": "0.02",                "closeTime": 1749726002161,                "avgExitPrice": "107281.77405000",                "deliveryPrice": "107281.77405031",                "openTime": 1749722990063,                "avgEntryPrice": "3371.50000000",                "totalPnl": "0.90760719"            },            {                "symbol": "BTC-12JUN25-104000-C-USDT",                "side": "Buy",                "totalOpenFee": "0.86379999",                "deliveryFee": "0.32287622",                "totalCloseFee": "0.00000000",                "qty": "0.02",                "closeTime": 1749715220240,                "avgExitPrice": "107625.40470150",                "deliveryPrice": "107625.40470159",                "openTime": 1749710568608,                "avgEntryPrice": "3946.50000000",                "totalPnl": "-7.60858218"            }        ]    },    "retExtInfo": {},    "time": 1749736532193}
```

---

## Set Auto Add Margin

**URL:** https://bybit-exchange.github.io/docs/v5/position/auto-add-margin

**Contents:**
- Set Auto Add Margin
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Set Auto Add Margin
On this page
Set Auto Add Margin
Turn on/off auto-add-margin for
isolated
margin position
HTTP Request
​
POST
/v5/position/set-auto-add-margin
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
(USDT Contract, USDC Contract)
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
autoAddMargin
true
integer
Turn on/off.
0
: off.
1
: on
positionIdx
false
integer
Used to identify positions in different position modes. For hedge mode position, this param is
required
0
: one-way mode
1
: hedge-mode Buy side
2
: hedge-mode Sell side
Response Parameters
​
None
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/set-auto-add-margin
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN-TYPE
:
2
X-BAPI-SIGN
:
XXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1675255134857
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
"autoAddmargin"
:
1
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
set_auto_add_margin
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
autoAddmargin
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
setAutoAddMarginRequest
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
autoAddMargin
(
AutoAddMargin
.
ON
)
.
build
(
)
;
client
.
setAutoAddMargin
(
setAutoAddMarginRequest
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
setAutoAddMargin
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
autoAddMargin
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1675255135069
}

**Examples:**

Example 1 ():
```
POST /v5/position/set-auto-add-margin HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675255134857X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "BTCUSDT",    "autoAddmargin": 1,    "positionIdx": null}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_auto_add_margin(    category="linear",    symbol="BTCUSDT",    autoAddmargin=1,))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setAutoAddMarginRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").autoAddMargin(AutoAddMargin.ON).build();client.setAutoAddMargin(setAutoAddMarginRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setAutoAddMargin({        category: 'linear',        symbol: 'BTCUSDT',        autoAddMargin: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Move Position History

**URL:** https://bybit-exchange.github.io/docs/v5/position/move-position-history

**Contents:**
- Get Move Position History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Get Move Position History
On this page
Get Move Position History
You can query moved position data by master UID api key
HTTP Request
​
GET
/v5/position/move-history
Request Parameters
​
Parameter
Required
Type
Comments
category
false
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
startTime
false
number
The order creation start timestamp. The interval is 7 days
endTime
false
number
The order creation end timestamp. The interval is 7 days
status
false
string
Order status.
Processing
,
Filled
,
Rejected
blockTradeId
false
string
Block trade ID
limit
false
string
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
list
array
Object
> blockTradeId
string
Block trade ID
>
category
string
Product type.
linear
,
spot
,
option
> orderId
string
Bybit order ID
> userId
integer
User ID
> symbol
string
Symbol name
> side
string
Order side from taker's perspective.
Buy
,
Sell
> price
string
Order price
> qty
string
Order quantity
> execFee
string
The fee for taker or maker in the base currency paid to the Exchange executing the block trade
> status
string
Block trade status.
Processing
,
Filled
,
Rejected
> execId
string
The unique trade ID from the exchange
> resultCode
integer
The result code of the order.
0
means success
> resultMessage
string
The error message.
""
when resultCode=0
> createdAt
number
The timestamp (ms) when the order is created
> updatedAt
number
The timestamp (ms) when the order is updated
> rejectParty
string
""
means the status=
Filled
Taker
,
Maker
when status=
Rejected
bybit
means error is occurred on the Bybit side
nextPageCursor
string
Used to get the next page data
Request Example
​
HTTP
Python
Java
Node.js
GET
/v5/position/move-history?limit=1&status=Filled
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
1697523024244
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
get_move_position_history
(
limit
=
"1"
,
status
=
"Filled"
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
movePositionsHistoryRequest
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
status
(
MovePositionStatus
.
Processing
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
getMovePositionHistory
(
movePositionsHistoryRequest
)
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
"blockTradeId"
:
"1a82e5801af74b67b7ad71ba00a7391a"
,
"category"
:
"option"
,
"orderId"
:
"8e09c5b8-f651-4cec-968d-52764cac11ec"
,
"userId"
:
592324
,
"symbol"
:
"BTC-14OCT23-27000-C"
,
"side"
:
"Buy"
,
"price"
:
"6"
,
"qty"
:
"0.99"
,
"execFee"
:
"0"
,
"status"
:
"Filled"
,
"execId"
:
"677ad344-6bb4-4ace-baca-128fcffcaca7"
,
"resultCode"
:
0
,
"resultMessage"
:
""
,
"createdAt"
:
1697186522865
,
"updatedAt"
:
1697186523289
,
"rejectParty"
:
""
}
]
,
"nextPageCursor"
:
"page_token%3D1241742%26"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1697523024386
}

**Examples:**

Example 1 ():
```
GET /v5/position/move-history?limit=1&status=Filled HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1697523024244X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_move_position_history(    limit="1",    status="Filled",))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var movePositionsHistoryRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").status(MovePositionStatus.Processing).build();System.out.println(client.getMovePositionHistory(movePositionsHistoryRequest));
```

Example 4 ():
```

```

---

## Switch Position Mode

**URL:** https://bybit-exchange.github.io/docs/v5/position/position-mode

**Contents:**
- Switch Position Mode
  - Example​
  - The position-switch ability for each contract​
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Switch Position Mode
On this page
Switch Position Mode
It supports to switch the position mode for
USDT perpetual
and
Inverse futures
. If you are in one-way Mode, you can only open one position on Buy or Sell side. If you are in hedge mode, you can open both Buy and Sell side positions simultaneously.
tip
Priority for configuration to take effect: symbol > coin > system default
System default: one-way mode
If the request is by coin (settleCoin), then all symbols based on this setteCoin that do not have position and open order will be batch switched, and new listed symbol based on this settleCoin will be the same mode you set.
Example
​
System default
coin
symbol
Initial setting
one-way
never configured
never configured
Result
All USDT perpetual trading pairs are one-way mode
Change 1
-
-
Set BTCUSDT to hedge-mode
Result
BTCUSDT becomes hedge-mode, and all other symbols keep one-way mode
list new symbol ETHUSDT
ETHUSDT is one-way mode  (inherit default rules)
Change 2
-
Set USDT to hedge-mode
-
Result
All current trading pairs with no positions or orders are hedge-mode, and no adjustments will be made for trading pairs with positions and orders
list new symbol SOLUSDT
SOLUSDT is hedge-mode (Inherit coin rule)
Change 3
-
-
Set ASXUSDT to one-mode
Take effect result
AXSUSDT is one-way mode, other trading pairs have no change
list new symbol BITUSDT
BITUSDT is hedge-mode (Inherit coin rule)
The position-switch ability for each contract
​
UTA2.0
USDT perpetual
Support one-way & hedge-mode
USDT futures
Support one-way
only
USDC perpetual
Support one-way
only
Inverse perpetual
Support one-way
only
Inverse futures
Support one-way
only
HTTP Request
​
POST
/v5/position/switch-mode
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
, USDT Contract
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only. Either
symbol
or
coin
is
required
.
symbol
has a higher priority
coin
false
string
Coin, uppercase only
mode
true
integer
Position mode.
0
: Merged Single.
3
: Both Sides
RUN >>
Response Parameters
​
None
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/switch-mode
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
1675249072041
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
87
{
"category"
:
"inverse"
,
"symbol"
:
"BTCUSDH23"
,
"coin"
:
null
,
"mode"
:
0
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
switch_position_mode
(
category
=
"inverse"
,
symbol
=
"BTCUSDH23"
,
mode
=
0
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
newPositionRestClient
(
)
;
var
switchPositionMode
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
positionMode
(
PositionMode
.
BOTH_SIDES
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
switchPositionMode
(
switchPositionMode
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
switchPositionMode
(
{
category
:
'inverse'
,
symbol
:
'BTCUSDH23'
,
mode
:
0
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1675249072814
}

**Examples:**

Example 1 ():
```
POST /v5/position/switch-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675249072041X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 87{    "category":"inverse",    "symbol":"BTCUSDH23",    "coin": null,    "mode": 0}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.switch_position_mode(    category="inverse",    symbol="BTCUSDH23",    mode=0,))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newPositionRestClient();var switchPositionMode = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").positionMode(PositionMode.BOTH_SIDES).build();System.out.println(client.switchPositionMode(switchPositionMode));
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .switchPositionMode({        category: 'inverse',        symbol: 'BTCUSDH23',        mode: 0,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Get Closed PnL

**URL:** https://bybit-exchange.github.io/docs/v5/position/close-pnl

**Contents:**
- Get Closed PnL
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Get Closed PnL (2 years)
On this page
Get Closed PnL
Query user's closed profit and loss records
HTTP Request
​
GET
/v5/position/closed-pnl
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
(USDT Contract, USDC Contract)
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
> side
string
Buy
,
Sell
> qty
string
Order qty
> orderPrice
string
Order price
>
orderType
string
Order type.
Market
,
Limit
> execType
string
Exec type
Trade
,
BustTrade
SessionSettlePnL
Settle
,
MovePosition
> closedSize
string
Closed size
> cumEntryValue
string
Cumulated Position value
> avgEntryPrice
string
Average entry price
> cumExitValue
string
Cumulated exit position value
> avgExitPrice
string
Average exit price
> closedPnl
string
Closed PnL
> fillCount
string
The number of fills in a single order
> leverage
string
leverage
> openFee
string
Open position trading fee
> closeFee
string
Close position trading fee
> createdTime
string
The created time (ms)
> updatedTime
string
The updated time (ms)
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
/v5/position/closed-pnl?category=linear&limit=1
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
1672284128523
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
get_closed_pnl
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
closPnlRequest
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
build
(
)
;
client
.
getClosePnlList
(
closPnlRequest
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
getClosedPnL
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
"nextPageCursor"
:
"5a373bfe-188d-4913-9c81-d57ab5be8068%3A1672214887231423699%2C5a373bfe-188d-4913-9c81-d57ab5be8068%3A1672214887231423699"
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
"leverage"
:
"3"
,
"updatedTime"
:
"1672214887236"
,
"side"
:
"Sell"
,
"orderId"
:
"5a373bfe-188d-4913-9c81-d57ab5be8068"
,
"closedPnl"
:
"-47.4065323"
,
"avgEntryPrice"
:
"1194.97516667"
,
"qty"
:
"3"
,
"cumEntryValue"
:
"3584.9255"
,
"createdTime"
:
"1672214887231"
,
"orderPrice"
:
"1122.95"
,
"closedSize"
:
"3"
,
"avgExitPrice"
:
"1180.59833333"
,
"execType"
:
"Trade"
,
"fillCount"
:
"4"
,
"cumExitValue"
:
"3541.795"
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
1672284129153
}

**Examples:**

Example 1 ():
```
GET /v5/position/closed-pnl?category=linear&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672284128523X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_closed_pnl(    category="linear",    limit=1,))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var closPnlRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).build();client.getClosePnlList(closPnlRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getClosedPnL({        category: 'linear',        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Set Leverage

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/set-leverage

**Contents:**
- Set Leverage
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Set Leverage
On this page
Set Leverage
Set the user's maximum leverage in spot cross margin
caution
Your account needs to activate spot margin first; i.e., you must have finished the quiz on web / app.
The updated leverage must be less than or equal to the maximum leverage of the currency
HTTP Request
​
POST
/v5/spot-margin-trade/set-leverage
Request Parameters
​
Parameter
Required
Type
Comments
leverage
true
string
Leverage.
[
2
,
10
]
.
currency
false
string
Coin name, uppercase only
RUN >>
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/spot-margin-trade/set-leverage
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
1672299806626
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"leverage"
:
"4"
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
spot_margin_trade_set_leverage
(
leverage
=
"4"
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
setSpotMarginLeverage
(
'4'
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672710944282
}

**Examples:**

Example 1 ():
```
POST /v5/spot-margin-trade/set-leverage HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672299806626X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "leverage": "4"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.spot_margin_trade_set_leverage(    leverage="4",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .setSpotMarginLeverage('4')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1672710944282}
```

---

## Get Leverage Token Info

**URL:** https://bybit-exchange.github.io/docs/v5/lt/leverage-token-info

**Contents:**
- Get Leverage Token Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

On this page
Get Leverage Token Info
Query leverage token information
HTTP Request
​
GET
/v5/spot-lever-token/info
Request Parameters
​
Parameter
Required
Type
Comments
ltCoin
false
string
Abbreviation of the LT, such as
BTC3L
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
Abbreviation
> ltName
string
Full name of leveraged token
> maxPurchase
string
Single maximum purchase amount
> minPurchase
string
Single minimum purchase amount
> maxPurchaseDaily
string
Maximum purchase amount in a single day
> maxRedeem
string
Single Maximum redemption quantity
> minRedeem
string
Single Minimum redemption quantity
> maxRedeemDaily
string
Maximum redemption quantity in a single day
> purchaseFeeRate
string
Purchase fee rate
> redeemFeeRate
string
Redeem fee rate
>
ltStatus
string
Whether the leverage token can be purchased or redeemed
> fundFee
string
Funding fee charged daily for users holding leveraged token
> fundFeeTime
string
The time to charge funding fee
> manageFeeRate
string
Management fee rate
> manageFeeTime
string
The time to charge management fee
> value
string
Nominal asset value
> netValue
string
Net value
> total
string
Total purchase upper limit
RUN >>
Request Example
​
HTTP
Python
GET
/v5/spot-lever-token/info?ltCoin=BTC3L
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
get_leveraged_token_info
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
"list"
:
[
{
"fundFee"
:
"299.70622821"
,
"fundFeeTime"
:
"1672992000000"
,
"ltCoin"
:
"BTC3L"
,
"ltName"
:
"3X Long"
,
"ltStatus"
:
"1"
,
"manageFeeRate"
:
"0.00005"
,
"manageFeeTime"
:
"1673053200000"
,
"maxPurchase"
:
"10000"
,
"maxPurchaseDaily"
:
"200000"
,
"maxRedeem"
:
"14434"
,
"maxRedeemDaily"
:
"2100000"
,
"minPurchase"
:
"100"
,
"minRedeem"
:
"144"
,
"netValue"
:
"0.376482201140738147"
,
"purchaseFeeRate"
:
"0.0005"
,
"redeemFeeRate"
:
"0.0005"
,
"total"
:
"5000000"
,
"value"
:
"49464463114.022994974075443169"
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
1672991427073
}

**Examples:**

Example 1 ():
```
GET /v5/spot-lever-token/info?ltCoin=BTC3L HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_leveraged_token_info(    ltCoin="BTC3L",))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "fundFee": "299.70622821",                "fundFeeTime": "1672992000000",                "ltCoin": "BTC3L",                "ltName": "3X Long",                "ltStatus": "1",                "manageFeeRate": "0.00005",                "manageFeeTime": "1673053200000",                "maxPurchase": "10000",                "maxPurchaseDaily": "200000",                "maxRedeem": "14434",                "maxRedeemDaily": "2100000",                "minPurchase": "100",                "minRedeem": "144",                "netValue": "0.376482201140738147",                "purchaseFeeRate": "0.0005",                "redeemFeeRate": "0.0005",                "total": "5000000",                "value": "49464463114.022994974075443169"            }        ]    },    "retExtInfo": {},    "time": 1672991427073}
```

---

## Move Position

**URL:** https://bybit-exchange.github.io/docs/v5/position/move-position

**Contents:**
- Move Position
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Move Position
On this page
Move Position
You can move positions between sub-master, master-sub, or sub-sub UIDs when necessary
info
The endpoint can only be called by master UID api key
UIDs must be the same master-sub account relationship
The trades generated from move-position endpoint will not be displayed in the Recent Trade (Rest API & Websocket)
There is no trading fee
fromUid
and
toUid
both should be Unified trading accounts, and they need to be one-way mode when moving the positions
Please note that once executed, you will get execType=
MovePosition
entry from
Get Trade History
,
Get Closed Pnl
, and stream from
Execution
.
HTTP Request
​
POST
/v5/position/move-positions
Request Parameters
​
Parameter
Required
Type
Comments
fromUid
true
string
From UID
Must be UTA
Must be in one-way mode for Futures
toUid
true
string
To UID
Must be UTA
Must be in one-way mode for Futures
list
true
array
Object. Up to 25 legs per request
>
category
true
string
Product type
linear
,
spot
,
option
,
inverse
> symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
> price
true
string
Trade price
linear
&
inverse
: the price needs to be between
[95% of mark price, 105% of mark price]
spot
&
option
: the price needs to follow the price rule from
Instruments Info
> side
true
string
Trading side of
fromUid
For example,
fromUid
has a long position, when side=
Sell
, then once executed, the position of
fromUid
will be reduced or open a short position depending on
qty
input
> qty
true
string
Executed qty
The value must satisfy the qty rule from
Instruments Info
, in particular, category=
linear
is able to input
maxOrderQty
* 5
Response Parameters
​
Parameter
Type
Comments
retCode
integer
Result code.
0
means request is successfully accepted
retMsg
string
Result message
result
map
Object
> blockTradeId
string
Block trade ID
> status
string
Status.
Processing
,
Rejected
> rejectParty
string
""
means initial validation is passed, please check the order status via
Get Move Position History
Taker
,
Maker
when status=
Rejected
bybit
means error is occurred on the Bybit side
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/move-positions
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
1697447928051
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"fromUid"
:
"100307601"
,
"toUid"
:
"592324"
,
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
"price"
:
"100"
,
"side"
:
"Sell"
,
"qty"
:
"0.01"
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
move_position
(
fromUid
=
"100307601"
,
toUid
=
"592324"
,
list
=
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
"price"
:
"100"
,
"side"
:
"Sell"
,
"qty"
:
"0.01"
,
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
movePositionsRequest
=
Arrays
.
asList
(
MovePositionDetailsRequest
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
.
getCategoryTypeId
(
)
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
SELL
.
getTransactionSide
(
)
)
.
price
(
"100"
)
.
qty
(
"0.01"
)
.
build
(
)
,
MovePositionDetailsRequest
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
.
getCategoryTypeId
(
)
)
.
symbol
(
"ETHUSDT"
)
.
side
(
Side
.
SELL
.
getTransactionSide
(
)
)
.
price
(
"100"
)
.
qty
(
"0.01"
)
.
build
(
)
)
;
var
batchMovePositionsRequest
=
BatchMovePositionRequest
.
builder
(
)
.
fromUid
(
"123456"
)
.
toUid
(
"456789"
)
.
list
(
movePositionsRequest
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
batchMovePositions
(
batchMovePositionsRequest
)
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
"blockTradeId"
:
"e9bb926c95f54cf1ba3e315a58b8597b"
,
"status"
:
"Processing"
,
"rejectParty"
:
""
}
}

**Examples:**

Example 1 ():
```
POST /v5/position/move-positions HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1697447928051X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "fromUid": "100307601",    "toUid": "592324",    "list": [        {            "category": "spot",            "symbol": "BTCUSDT",            "price": "100",            "side": "Sell",            "qty": "0.01"        }    ]}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.move_position(    fromUid="100307601",    toUid="592324",    list=[        {            "category": "spot",            "symbol": "BTCUSDT",            "price": "100",            "side": "Sell",            "qty": "0.01",        }    ]))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var movePositionsRequest = Arrays.asList(MovePositionDetailsRequest.builder().category(CategoryType.SPOT.getCategoryTypeId()).symbol("BTCUSDT").side(Side.SELL.getTransactionSide()).price("100").qty("0.01").build(),                MovePositionDetailsRequest.builder().category(CategoryType.SPOT.getCategoryTypeId()).symbol("ETHUSDT").side(Side.SELL.getTransactionSide()).price("100").qty("0.01").build());var batchMovePositionsRequest = BatchMovePositionRequest.builder().fromUid("123456").toUid("456789").list(movePositionsRequest).build();System.out.println(client.batchMovePositions(batchMovePositionsRequest));
```

Example 4 ():
```

```

---

## Add Or Reduce Margin

**URL:** https://bybit-exchange.github.io/docs/v5/position/manual-add-margin

**Contents:**
- Add Or Reduce Margin
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Position
Add Or Reduce Margin
On this page
Add Or Reduce Margin
Manually add or reduce margin for
isolated
margin position
HTTP Request
​
POST
/v5/position/add-margin
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
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
margin
true
string
Add or reduce. To add, then
10
; To reduce, then
-10
. Support up to 4 decimal
positionIdx
false
integer
Used to identify positions in different position modes. For hedge mode position, this param is
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
symbol
string
Symbol name
positionIdx
integer
Position idx, used to identify positions in different position modes
0
: One-Way Mode
1
: Buy side of both side mode
2
: Sell side of both side mode
riskId
integer
Risk limit ID
riskLimitValue
string
Risk limit value
size
string
Position size
avgPrice
string
Average entry price
liqPrice
string
Liquidation price
bustPrice
string
Bankruptcy price
markPrice
string
Last mark price
positionValue
string
Position value
leverage
string
Position leverage
autoAddMargin
integer
Whether to add margin automatically.
0
: false,
1
: true
positionStatus
String
Position status.
Normal
,
Liq
,
Adl
positionIM
string
Initial margin
positionMM
string
Maintenance margin
takeProfit
string
Take profit price
stopLoss
string
Stop loss price
trailingStop
string
Trailing stop (The distance from market price)
unrealisedPnl
string
Unrealised PnL
cumRealisedPnl
string
Cumulative realised pnl
createdTime
string
Timestamp of the first time a position was created on this symbol (ms)
updatedTime
string
Position updated timestamp (ms)
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/add-margin
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
1684234363665
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
97
{
"category"
:
"inverse"
,
"symbol"
:
"ETHUSD"
,
"margin"
:
"0.01"
,
"positionIdx"
:
0
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
add_or_reduce_margin
(
category
=
"linear"
,
symbol
=
"BTCUSDT"
,
margin
=
"10"
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
updateMarginRequest
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
INVERSE
)
.
symbol
(
"ETHUSDT"
)
.
margin
(
"0.0001"
)
.
build
(
)
;
client
.
modifyPositionMargin
(
updateMarginRequest
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
addOrReduceMargin
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
"category"
:
"inverse"
,
"symbol"
:
"ETHUSD"
,
"positionIdx"
:
0
,
"riskId"
:
11
,
"riskLimitValue"
:
"500"
,
"size"
:
"200"
,
"positionValue"
:
"0.11033265"
,
"avgPrice"
:
"1812.70004844"
,
"liqPrice"
:
"1550.80"
,
"bustPrice"
:
"1544.20"
,
"markPrice"
:
"1812.90"
,
"leverage"
:
"12"
,
"autoAddMargin"
:
0
,
"positionStatus"
:
"Normal"
,
"positionIM"
:
"0.01926611"
,
"positionMM"
:
"0"
,
"unrealisedPnl"
:
"0.00001217"
,
"cumRealisedPnl"
:
"-0.04618929"
,
"stopLoss"
:
"0.00"
,
"takeProfit"
:
"0.00"
,
"trailingStop"
:
"0.00"
,
"createdTime"
:
"1672737740039"
,
"updatedTime"
:
"1684234363788"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1684234363789
}

**Examples:**

Example 1 ():
```
POST /v5/position/add-margin HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1684234363665X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 97{    "category": "inverse",    "symbol": "ETHUSD",    "margin": "0.01",    "positionIdx": 0}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.add_or_reduce_margin(    category="linear",    symbol="BTCUSDT",    margin="10"))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var updateMarginRequest = PositionDataRequest.builder().category(CategoryType.INVERSE).symbol("ETHUSDT").margin("0.0001").build();client.modifyPositionMargin(updateMarginRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .addOrReduceMargin({        category: 'linear',        symbol: 'BTCUSDT',        margin: '10',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---
