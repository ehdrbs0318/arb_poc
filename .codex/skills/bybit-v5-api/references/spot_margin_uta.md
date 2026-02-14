# Bybit-V5-Api - Spot Margin Uta

**Pages:** 6

---

## Get Historical Interest Rate

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/historical-interest

**Contents:**
- Get Historical Interest Rate
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Historical Interest Rate
On this page
Get Historical Interest Rate
You can query up to six months borrowing interest rate of Margin trading.
info
Need authentication, the api key needs "Spot" permission
Only supports Unified account
It is public data, i.e., different users get the same historical interest rate for the same VIP/Pro
HTTP Request
​
GET
/v5/spot-margin-trade/interest-rate-history
Request Parameters
​
Parameter
Required
Type
Comments
currency
true
string
Coin name, uppercase only
vipLevel
false
string
VIP level
Please note that "No VIP" should be passed like "No%20VIP" in the query string
If not passed, it returns your account's VIP level data
startTime
false
integer
The start timestamp (ms)
Either both time parameters are passed or neither is passed.
Returns 7 days data when both are not passed
Supports up to 30 days interval when both are passed
endTime
false
integer
The end timestamp (ms)
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
> timestamp
long
timestamp
> currency
string
coin name
> hourlyBorrowRate
string
Hourly borrowing rate
> vipLevel
string
VIP/Pro level
Request Example
​
HTTP
Python
GET
/v5/spot-margin-trade/interest-rate-history?currency=USDC&vipLevel=No%20VIP&startTime=1721458800000&endTime=1721469600000
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
1721891663064
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
spot_margin_trade_get_historical_interest_rate
(
currency
=
"BTC"
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
"timestamp"
:
1721469600000
,
"currency"
:
"USDC"
,
"hourlyBorrowRate"
:
"0.000014621596"
,
"vipLevel"
:
"No VIP"
}
,
{
"timestamp"
:
1721466000000
,
"currency"
:
"USDC"
,
"hourlyBorrowRate"
:
"0.000014621596"
,
"vipLevel"
:
"No VIP"
}
,
{
"timestamp"
:
1721462400000
,
"currency"
:
"USDC"
,
"hourlyBorrowRate"
:
"0.000014621596"
,
"vipLevel"
:
"No VIP"
}
,
{
"timestamp"
:
1721458800000
,
"currency"
:
"USDC"
,
"hourlyBorrowRate"
:
"0.000014621596"
,
"vipLevel"
:
"No VIP"
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
1721899048991
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/interest-rate-history?currency=USDC&vipLevel=No%20VIP&startTime=1721458800000&endTime=1721469600000 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1721891663064X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.spot_margin_trade_get_historical_interest_rate(    currency="BTC"))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "timestamp": 1721469600000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            },            {                "timestamp": 1721466000000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            },            {                "timestamp": 1721462400000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            },            {                "timestamp": 1721458800000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            }        ]    },    "retExtInfo": "{}",    "time": 1721899048991}
```

---

## Get Max Borrowable Amount

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/max-borrowable

**Contents:**
- Get Max Borrowable Amount
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Max Borrowable Amount
On this page
Get Max Borrowable Amount
HTTP Request
​
GET
/v5/spot-margin-trade/max-borrowable
Request Parameters
​
Parameter
Required
Type
Comments
currency
true
string
Coin name, uppercase only
Response Parameters
​
Parameter
Type
Comments
currency
string
Coin name, uppercase only
maxLoan
string
Max borrowable amount
Request Example
​
HTTP
Python
Node.js
GET
/v5/spot-margin-trade/max-borrowable?currency=BTC
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
"Success"
,
"result"
:
{
"maxLoan"
:
"17.54689892"
,
"currency"
:
"BTC"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1756261353733
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/max-borrowable?currency=BTC HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1692696840996X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "maxLoan": "17.54689892",        "currency": "BTC"    },    "retExtInfo": {},    "time": 1756261353733}
```

---

## Get VIP Margin Data

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/vip-margin

**Contents:**
- Get VIP Margin Data
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get VIP Margin Data
On this page
Get VIP Margin Data
This margin data is for
Unified account
in particular.
info
Does not need authentication.
HTTP Request
​
GET
/v5/spot-margin-trade/data
Request Parameters
​
Parameter
Required
Type
Comments
vipLevel
false
string
VIP level
currency
false
string
Coin name, uppercase only
Response Parameters
​
Parameter
Type
Comments
vipCoinList
array
Object
> list
array
Object
>> borrowable
boolean
Whether it is allowed to be borrowed
>> collateralRatio
string
Due to the new Tiered Collateral value logic, this field will no longer be accurate starting on February 19, 2025. Please refer to
Get Tiered Collateral Ratio
>> currency
string
Coin name
>> hourlyBorrowRate
string
Borrow interest rate per hour
>> liquidationOrder
string
Liquidation order
>> marginCollateral
boolean
Whether it can be used as a margin collateral currency
>> maxBorrowingAmount
string
Max borrow amount
> vipLevel
string
VIP level
RUN >>
Request Example
​
HTTP
Python
Node.js
GET /v5/spot-margin-trade/data?vipLevel=No VIP&currency=BTC HTTP/1.1
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
spot_margin_trade_get_vip_margin_data
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
getVIPMarginData
(
{
vipLevel
:
'No VIP'
,
currency
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
"success"
,
"result"
:
{
"vipCoinList"
:
[
{
"list"
:
[
{
"borrowable"
:
true
,
"collateralRatio"
:
"0.95"
,
"currency"
:
"BTC"
,
"hourlyBorrowRate"
:
"0.0000015021220000"
,
"liquidationOrder"
:
"11"
,
"marginCollateral"
:
true
,
"maxBorrowingAmount"
:
"3"
}
]
,
"vipLevel"
:
"No VIP"
}
]
}
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/data?vipLevel=No VIP&currency=BTC HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.spot_margin_trade_get_vip_margin_data())
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getVIPMarginData({    vipLevel: 'No VIP',    currency: 'BTC',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "vipCoinList": [            {                "list": [                    {                        "borrowable": true,                        "collateralRatio": "0.95",                        "currency": "BTC",                        "hourlyBorrowRate": "0.0000015021220000",                        "liquidationOrder": "11",                        "marginCollateral": true,                        "maxBorrowingAmount": "3"                    }                ],                "vipLevel": "No VIP"            }        ]    }}
```

---

## Get Auto Repay Mode

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/get-auto-repay-mode

**Contents:**
- Get Auto Repay Mode
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Auto Repay Mode
On this page
Get Auto Repay Mode
Get spot automatic repayment mode
HTTP Request
​
GET
/v5/spot-margin-trade/get-auto-repay-mode
Request Parameters
​
Parameter
Required
Type
Comments
currency
false
string
Coin name, uppercase only. If
currency
is not passed, automatic repay mode for all currencies will be returned.
Response Parameters
​
Parameter
Type
Comments
data
array
Object
> currency
string
Coin name, uppercase only.
> autoRepayMode
string
1
: On
0
: Off
Request Example
​
HTTP
Python
Node.js
GET
/v5/spot-margin-trade/get-auto-repay-mode?currency=ETH
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
"data"
:
[
{
"autoRepayMode"
:
"1"
,
"currency"
:
"ETH"
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
1766977353904
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/get-auto-repay-mode?currency=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672299806626X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "data": [            {                "autoRepayMode": "1",                "currency": "ETH"            }        ]    },    "retExtInfo": {},    "time": 1766977353904}
```

---

## Get Available Amount to Repay

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/repayment-available-amount

**Contents:**
- Get Available Amount to Repay
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Available Amount to Repay
On this page
Get Available Amount to Repay
HTTP Request
​
GET
/v5/spot-margin-trade/repayment-available-amount
Request Parameters
​
Parameter
Required
Type
Comments
currency
true
string
Coin name, uppercase only
Response Parameters
​
Parameter
Type
Comments
currency
string
Coin name, uppercase only
lossLessRepaymentAmount
string
Repayment amount = min(spot coin available balance, coin borrow amount)
Request Example
​
HTTP
Python
Node.js
GET
/v5/spot-margin-trade/repayment-available-amount?currency=BTC
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
"Success"
,
"result"
:
{
"lossLessRepaymentAmount"
:
"0.02000000"
,
"currency"
:
"BTC"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1756273388821
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/repayment-available-amount?currency=BTC HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1692696840996X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "lossLessRepaymentAmount": "0.02000000",        "currency": "BTC"    },    "retExtInfo": {},    "time": 1756273388821}
```

---

## Get Coin State

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/coinstate

**Contents:**
- Get Coin State
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Spot Margin Trade (UTA)
Get Coin State
On this page
Get Coin State
HTTP Request
​
GET
/v5/spot-margin-trade/coinstate
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
arrayList
Object
> currency
string
Coin name, uppercase only
> spotLeverage
string
Spot margin leverage. Returns "" if spot margin mode is turned off
Request Example
​
HTTP
Python
Node.js
GET
/v5/spot-margin-trade/coinstate
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
"Success"
,
"result"
:
{
"list"
:
[
{
"spotLeverage"
:
3
,
"currency"
:
"BTC"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"ETH"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"AVAX"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"EOS"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"XRP"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"USDT"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"GALA"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"DOGE"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"BIT"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"BTC3S"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"BTC3L"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"EUR"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"USDC"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"UNI"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"SOL"
}
,
{
"spotLeverage"
:
4
,
"currency"
:
"ADA"
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
1756273703314
}

**Examples:**

Example 1 ():
```
GET /v5/spot-margin-trade/coinstate HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1692696840996X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():
```

```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "Success",    "result": {        "list": [            {                "spotLeverage": 3,                "currency": "BTC"            },            {                "spotLeverage": 4,                "currency": "ETH"            },            {                "spotLeverage": 4,                "currency": "AVAX"            },            {                "spotLeverage": 4,                "currency": "EOS"            },            {                "spotLeverage": 4,                "currency": "XRP"            },            {                "spotLeverage": 4,                "currency": "USDT"            },            {                "spotLeverage": 4,                "currency": "GALA"            },            {                "spotLeverage": 4,                "currency": "DOGE"            },            {                "spotLeverage": 4,                "currency": "BIT"            },            {                "spotLeverage": 4,                "currency": "BTC3S"            },            {                "spotLeverage": 4,                "currency": "BTC3L"            },            {                "spotLeverage": 4,                "currency": "EUR"            },            {                "spotLeverage": 4,                "currency": "USDC"            },            {                "spotLeverage": 4,                "currency": "UNI"            },            {                "spotLeverage": 4,                "currency": "SOL"            },            {                "spotLeverage": 4,                "currency": "ADA"            }        ]    },    "retExtInfo": {},    "time": 1756273703314}
```

---
