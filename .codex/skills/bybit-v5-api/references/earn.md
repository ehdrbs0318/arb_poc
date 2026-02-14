# Bybit-V5-Api - Earn

**Pages:** 5

---

## Cancel Redeem

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/cancel-redeem

**Contents:**
- Cancel Redeem
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Cancel Redeem
On this page
Cancel Redeem
Cancel the withdrawal operation.
HTTP Request
​
POST
/v5/lending/redeem-cancel
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
The order ID of redemption
serialNo
false
string
Serial no. The customised ID of redemption
Response Parameters
​
Parameter
Type
Comments
orderId
string
Order ID
serialNo
string
Serial No
updatedTime
string
Updated timestamp (ms)
Request Example
​
POST
/v5/lending/redeem-cancel
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
1682048277724
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"coin"
:
"BTC"
,
"orderId"
:
"1403517113428086272"
,
"serialNo"
:
null
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
"1403517113428086272"
,
"serialNo"
:
"linear004"
,
"updatedTime"
:
"1682048277963"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1682048278001
}

**Examples:**

Example 1 ():
```
POST /v5/lending/redeem-cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682048277724X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "coin": "BTC",    "orderId": "1403517113428086272",    "serialNo": null}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "1403517113428086272",        "serialNo": "linear004",        "updatedTime": "1682048277963"    },    "retExtInfo": {},    "time": 1682048278001}
```

---

## Redeem

**URL:** https://bybit-exchange.github.io/docs/v5/lt/redeem

**Contents:**
- Redeem
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

On this page
Redeem
Redeem leverage token
HTTP Request
​
POST
/v5/spot-lever-token/redeem
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
quantity
ture
string
Redeem quantity of LT
serialNo
false
string
Serial number
Response Parameters
​
Parameter
Type
Comments
ltCoin
string
Abbreviation of the LT
ltOrderStatus
string
Order status.
1
: completed,
2
: in progress,
3
: failed
quantity
string
Quantity
execQty
string
LT quantity
execAmt
string
Executed amount of LT
redeemId
string
Order ID
serialNo
string
Serial number
valueCoin
string
Quote coin
RUN >>
Request Example
​
HTTP
Python
POST
/v5/spot-lever-token/redeem
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672296416024
X-BAPI-SIGN
:
XXXXX
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"ltCoin"
:
"EOS3L"
,
"quantity"
:
"150"
,
"serialNo"
:
"redeem-001"
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
redeem_leveraged_token
(
ltCoin
=
"EOS3L"
,
quantity
=
"150"
,
serialNo
=
"redeem-001"
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
"execAmt"
:
""
,
"execQty"
:
"150"
,
"ltCoin"
:
"EOS3L"
,
"ltOrderStatus"
:
"2"
,
"quantity"
:
""
,
"redeemId"
:
"2619"
,
"serialNo"
:
"redeem-001"
,
"valueCoin"
:
"EOS3L"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672296417363
}

**Examples:**

Example 1 ():
```
POST /v5/spot-lever-token/redeem HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672296416024X-BAPI-SIGN: XXXXXX-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "ltCoin": "EOS3L",    "quantity": "150",    "serialNo": "redeem-001"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.redeem_leveraged_token(    ltCoin="EOS3L",    quantity="150",    serialNo="redeem-001"))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "execAmt": "",        "execQty": "150",        "ltCoin": "EOS3L",        "ltOrderStatus": "2",        "quantity": "",        "redeemId": "2619",        "serialNo": "redeem-001",        "valueCoin": "EOS3L"    },    "retExtInfo": {},    "time": 1672296417363}
```

---

## Get Yield History

**URL:** https://bybit-exchange.github.io/docs/v5/earn/yield-history

**Contents:**
- Get Yield History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Earn
Get Yield History
On this page
Get Yield History
You can get the past 3 months data
info
API key needs "Earn" permission
HTTP Request
​
GET
/v5/earn/yield
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
> productId
string
Product ID
> coin
string
Coin name："BTC", "ETH"
> id
string
Unique key (guaranteed to be unique only under the same user)
> amount
string
Yield Amount.Example: 10
> yieldType
string
Yield type:
Normal
,
Bonus
(Flexible saving only supports
Normal
)
> distributionMode
string
Distribution type:
Auto
,
Manual
,
Reinvest
Auto
: Automatically distributed daily
Manual
: Distributed when the user redeems
Reinvest
: Reinvestment (not yet available)
> effectiveStakingAmount
string
Effective staking amount, e.g., 1000.00
> orderId
string
Redemption order UUID ,For
FlexibleSaving
,Only returns order ID if
distribution_mode
is
Manual
> status
string
Order status:
Pending
,
Success
,
Fail
> createdAt
string
Order creation time in milliseconds, e.g., 1684738540561
Request Example
​
HTTP
Python
Node.js
GET
/v5/earn/yield?category=FlexibleSaving
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
"yield"
:
[
{
"productId"
:
"428"
,
"coin"
:
"USDT"
,
"id"
:
"1002096"
,
"amount"
:
"0.0608"
,
"yieldType"
:
"Normal"
,
"distributionMode"
:
"Manual"
,
"effectiveStakingAmount"
:
"1000"
,
"orderId"
:
"05a7012d-c4d6-493a-8c6b-023a1038944a"
,
"status"
:
"Success"
,
"createdAt"
:
"1759993805000"
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
1759993815641
}

**Examples:**

Example 1 ():
```
GET /v5/earn/yield?category=FlexibleSaving HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739937044221X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "yield": [            {                "productId": "428",                "coin": "USDT",                "id": "1002096",                "amount": "0.0608",                "yieldType": "Normal",                "distributionMode": "Manual",                "effectiveStakingAmount": "1000",                "orderId": "05a7012d-c4d6-493a-8c6b-023a1038944a",                "status": "Success",                "createdAt": "1759993805000"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1759993815641}
```

---

## Get Hourly Yield History

**URL:** https://bybit-exchange.github.io/docs/v5/earn/hourly-yield

**Contents:**
- Get Hourly Yield History
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Earn
Get Hourly Yield History
On this page
Get Hourly Yield History
info
API key needs "Earn" permission
HTTP Request
​
GET
/v5/earn/hourly-yield
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
> productId
string
Product ID
> coin
string
Coin name："BTC", "ETH"
> id
string
Unique key (guaranteed to be unique only under the same user)
> amount
string
Yield Amount.Example: 10
> effectiveStakingAmount
string
Effective staking amount, e.g., 1000.00
> status
string
Order status:
Pending
,
Success
,
Fail
> hourlyDate
string
Hourly yield time(ms) eg: 1755478800000
> createdAt
string
Order creation time in milliseconds, e.g., 1684738540561
Request Example
​
HTTP
Python
Node.js
GET
/v5/earn/hourly-yield?category=FlexibleSaving
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
"productId"
:
"428"
,
"coin"
:
"USDT"
,
"amount"
:
"0.060810502283105022"
,
"effectiveStakingAmount"
:
"1000"
,
"hourlyDate"
:
"1759989600000"
,
"status"
:
"Success"
,
"createdAt"
:
"1759989603000"
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
1759993045287
}

**Examples:**

Example 1 ():
```
GET /v5/earn/hourly-yield?category=FlexibleSaving HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739937044221X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "productId": "428",                "coin": "USDT",                "amount": "0.060810502283105022",                "effectiveStakingAmount": "1000",                "hourlyDate": "1759989600000",                "status": "Success",                "createdAt": "1759989603000"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1759993045287}
```

---

## Get Product Info

**URL:** https://bybit-exchange.github.io/docs/v5/earn/product-info

**Contents:**
- Get Product Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Earn
Get Product Info
On this page
Get Product Info
info
Does not need authentication.
Bybit Saving FAQ
HTTP Request
​
GET
/v5/earn/product
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
coin
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
> category
string
FlexibleSaving
,
OnChain
> estimateApr
string
Estimated APR, e.g.,
3%
,
4.25%
Remarks
: 1)The Est. APR provides a dynamic preview of your potential returns, updated every 10 minutes in response to market conditions.
2) Please note that this is an estimate and may differ from the actual APR you will receive.
3) Platform Reward APRs are not shown
> coin
string
Coin name
> minStakeAmount
string
Minimum stake amount
> maxStakeAmount
string
Maximum stake amount
> precision
string
Amount precision
> productId
string
Product ID
> status
string
Available
,
NotAvailable
> bonusEvents
Array
Bonus
>> apr
string
Yesterday's Rewards APR
>> coin
string
Reward coin
>> announcement
string
Announcement link
> minRedeemAmount
string
Minimum redemption amount. Only has value in Onchain LST mode
> maxRedeemAmount
string
Maximum redemption amount. Only has value in Onchain LST mode
> duration
string
Fixed
,
Flexible
. Product Type
> term
int
Unit: Day. Only when duration =
Fixed
for OnChain
> swapCoin
string
swap coin. Only has value in Onchain LST mode
> swapCoinPrecision
string
swap coin precision. Only has value in Onchain LST mode
> stakeExchangeRate
string
Estimated stake exchange rate. Only has value in Onchain LST mode
> redeemExchangeRate
string
Estimated redeem exchange rate. Only has value in Onchain LST mode
> rewardDistributionType
string
Simple
: Simple interest,
Compound
: Compound interest,
Other
: LST. Only has value for Onchain
> rewardIntervalMinute
int
Frequency of reward distribution (minutes)
> redeemProcessingMinute
string
Estimated redemption minutes
> stakeTime
string
Staking on-chain time, in milliseconds
> interestCalculationTime
string
Interest accrual time, in milliseconds
Request Example
​
HTTP
Python
Node.js
GET
/v5/earn/product?category=FlexibleSaving&coin=BTC
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
get_earn_product_info
(
category
=
"FlexibleSaving"
,
coin
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
""
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
"FlexibleSaving"
,
"estimateApr"
:
"3%"
,
"coin"
:
"BTC"
,
"minStakeAmount"
:
"0.001"
,
"maxStakeAmount"
:
"10"
,
"precision"
:
"8"
,
"productId"
:
"430"
,
"status"
:
"Available"
,
"bonusEvents"
:
[
]
,
"minRedeemAmount"
:
""
,
"maxRedeemAmount"
:
""
,
"duration"
:
""
,
"term"
:
0
,
"swapCoin"
:
""
,
"swapCoinPrecision"
:
""
,
"stakeExchangeRate"
:
""
,
"redeemExchangeRate"
:
""
,
"rewardDistributionType"
:
""
,
"rewardIntervalMinute"
:
0
,
"redeemProcessingMinute"
:
0
,
"stakeTime"
:
""
,
"interestCalculationTime"
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
1739935669110
}

**Examples:**

Example 1 ():
```
GET /v5/earn/product?category=FlexibleSaving&coin=BTC HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_earn_product_info(    category="FlexibleSaving",    coin="BTC",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "category": "FlexibleSaving",                "estimateApr": "3%",                "coin": "BTC",                "minStakeAmount": "0.001",                "maxStakeAmount": "10",                "precision": "8",                "productId": "430",                "status": "Available",                "bonusEvents": [],                "minRedeemAmount": "",                "maxRedeemAmount": "",                "duration": "",                "term": 0,                "swapCoin": "",                "swapCoinPrecision": "",                "stakeExchangeRate": "",                "redeemExchangeRate": "",                "rewardDistributionType": "",                "rewardIntervalMinute": 0,                "redeemProcessingMinute": 0,                "stakeTime": "",                "interestCalculationTime": ""            }        ]    },    "retExtInfo": {},    "time": 1739935669110}
```

---
