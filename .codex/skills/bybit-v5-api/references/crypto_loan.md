# Bybit-V5-Api - Crypto Loan

**Pages:** 7

---

## Borrow

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/flexible/borrow

**Contents:**
- Borrow
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Flexible Loan
Borrow
On this page
Borrow
Permission: "Spot trade"
UID rate limit: 1 req / second
info
The loan funds are released to the Funding wallet.
The collateral funds are deducted from the Funding wallet, so make sure you have enough collateral amount in the Funding wallet.
HTTP Request
​
POST
/v5/crypto-loan-flexible/borrow
Request Parameters
​
Parameter
Required
Type
Comments
loanCurrency
true
string
Loan coin name
loanAmount
true
string
Amount to borrow
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
/v5/crypto-loan-flexible/borrow
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
1752569210041
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
244
{
"loanCurrency"
:
"BTC"
,
"loanAmount"
:
"0.1"
,
"collateralList"
:
[
{
"currency"
:
"USDT"
,
"amount"
:
"1000"
}
,
{
"currency"
:
"ETH"
,
"amount"
:
"1"
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
borrow_flexible_crypto_loan
(
loanCurrency
=
"BTC"
,
loanAmount
=
"0.1"
,
collateralList
=
[
{
"currency"
:
"USDT"
,
"amount"
:
"1000"
}
,
{
"currency"
:
"ETH"
,
"amount"
:
"1"
}
]
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
1752569209682
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-flexible/borrow HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752569210041X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 244{    "loanCurrency": "BTC",    "loanAmount": "0.1",    "collateralList": [        {            "currency": "USDT",            "amount": "1000"        },        {            "currency": "ETH",            "amount": "1"        }    ]}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.borrow_flexible_crypto_loan(    loanCurrency="BTC",    loanAmount="0.1",    collateralList=[        {            "currency": "USDT",            "amount": "1000"        },        {            "currency": "ETH",            "amount": "1"        }    ]))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "orderId": "1363"    },    "retExtInfo": {},    "time": 1752569209682}
```

---

## Get Borrowable Coins

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/loan-coin

**Contents:**
- Get Borrowable Coins
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Get Borrowable Coins
On this page
Get Borrowable Coins
info
Does not need authentication.
danger
Borrowed coins can be returned at any time before the due date. You'll be charged 3 times the hourly interest during the overdue period. Your collateral will be liquidated to repay a loan and the interest if you fail to make the repayment 48 hours after the due time.
HTTP Request
​
GET
/v5/crypto-loan/loanable-data
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
VIP0
,
VIP1
,
VIP2
,
VIP3
,
VIP4
,
VIP5
,
VIP99
(supreme VIP)
PRO1
,
PRO2
,
PRO3
,
PRO4
,
PRO5
,
PRO6
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
>> borrowingAccuracy
integer
The number of decimal places (precision) of this coin
>> currency
string
Coin name
>> flexibleHourlyInterestRate
string
Flexible hourly floating interest rate
Flexible Crypto Loans offer an hourly floating interest rate, calculated based on the actual borrowing time per hour, with the option for early repayment
Is
""
if the coin does not support flexible loan
>> hourlyInterestRate7D
string
Hourly interest rate for 7 days loan. Is
""
if the coin does not support 7 days loan
>> hourlyInterestRate14D
string
Hourly interest rate for 14 days loan. Is
""
if the coin does not support 14 days loan
>> hourlyInterestRate30D
string
Hourly interest rate for 30 days loan. Is
""
if the coin does not support 30 days loan
>> hourlyInterestRate90D
string
Hourly interest rate for 90 days loan. Is
""
if the coin does not support 90 days loan
>> hourlyInterestRate180D
string
Hourly interest rate for 180 days loan. Is
""
if the coin does not support 180 days loan
>> maxBorrowingAmount
string
Max. amount to borrow
>> minBorrowingAmount
string
Min. amount to borrow
> vipLevel
string
VIP level
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/loanable-data?currency=USDT&vipLevel=VIP0
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
get_borrowable_coins
(
currency
=
"USDT"
,
vipLevel
=
"VIP0"
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
getBorrowableCoins
(
{
currency
:
'USDT'
,
vipLevel
:
'VIP0'
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
"vipCoinList"
:
[
{
"list"
:
[
{
"borrowingAccuracy"
:
4
,
"currency"
:
"USDT"
,
"flexibleHourlyInterestRate"
:
"0.0000090346"
,
"hourlyInterestRate14D"
:
"0.0000207796"
,
"hourlyInterestRate180D"
:
""
,
"hourlyInterestRate30D"
:
"0.00002349"
,
"hourlyInterestRate7D"
:
"0.0000180692"
,
"hourlyInterestRate90D"
:
""
,
"maxBorrowingAmount"
:
"8000000"
,
"minBorrowingAmount"
:
"20"
}
]
,
"vipLevel"
:
"VIP0"
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
1728619315868
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan/loanable-data?currency=USDT&vipLevel=VIP0 HTTP/1.1Host: api.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_borrowable_coins(    currency="USDT",    vipLevel="VIP0",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getBorrowableCoins({    currency: 'USDT',    vipLevel: 'VIP0',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "vipCoinList": [            {                "list": [                    {                        "borrowingAccuracy": 4,                        "currency": "USDT",                        "flexibleHourlyInterestRate": "0.0000090346",                        "hourlyInterestRate14D": "0.0000207796",                        "hourlyInterestRate180D": "",                        "hourlyInterestRate30D": "0.00002349",                        "hourlyInterestRate7D": "0.0000180692",                        "hourlyInterestRate90D": "",                        "maxBorrowingAmount": "8000000",                        "minBorrowingAmount": "20"                    }                ],                "vipLevel": "VIP0"            }        ]    },    "retExtInfo": {},    "time": 1728619315868}
```

---

## Repay

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/fixed/repay

**Contents:**
- Repay
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Fixed Loan
Repay
On this page
Repay
Permission: "Spot trade"
UID rate limit: 1 req / second
HTTP Request
​
POST
/v5/crypto-loan-fixed/fully-repay
Request Parameters
​
Parameter
Required
Type
Comments
loanId
false
string
Loan contract ID. Either
loanId
or
loanCurrency
needs to be passed
loanCurrency
false
string
Loan coin. Either
loanId
or
loanCurrency
needs to be passed
Response Parameters
​
Parameter
Type
Comments
repayId
string
Repayment transaction ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-fixed/fully-repay
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
"loanId"
:
"570"
,
"loanCurrency"
:
"ETH"
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
repay_fixed_crypto_loan
(
loanId
=
"570"
,
loanCurrency
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
"repayId"
:
"1771"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752569614549
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-fixed/fully-repay HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752656296791X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 50{    "loanId": "570",    "loanCurrency": "ETH"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.repay_fixed_crypto_loan(    loanId="570",    loanCurrency="ETH",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "repayId": "1771"    },    "retExtInfo": {},    "time": 1752569614549}
```

---

## Get Borrowable Coins

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/loan-coin

**Contents:**
- Get Borrowable Coins
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Get Borrowable Coins
On this page
Get Borrowable Coins
info
Does not need authentication.
HTTP Request
​
GET
/v5/crypto-loan-common/loanable-data
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
VIP0
,
VIP1
,
VIP2
,
VIP3
,
VIP4
,
VIP5
,
VIP99
(supreme VIP)
PRO1
,
PRO2
,
PRO3
,
PRO4
,
PRO5
,
PRO6
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
Coin name
> fixedBorrowable
boolean
Whether support fixed loan
> fixedBorrowingAccuracy
integer
Coin precision for fixed loan
> flexibleBorrowable
boolean
Whether support flexible loan
> flexibleBorrowingAccuracy
integer
Coin precision for flexible loan
> maxBorrowingAmount
string
Max borrow limit
> minFixedBorrowingAmount
string
Minimum amount for each fixed loan order
> minFlexibleBorrowingAmount
string
Minimum amount for each flexible loan order
> vipLevel
string
VIP level
> flexibleAnnualizedInterestRate
integer
The annualized interest rate for flexible borrowing. If the loan currency does not support flexible borrowing, it will always be """"
> annualizedInterestRate7D
string
The lowest annualized interest rate for fixed borrowing for 7 days that the market can currently provide. If there is no lending in the current market, then it is empty string
> annualizedInterestRate14D
string
The lowest annualized interest rate for fixed borrowing for 14 days that the market can currently provide. If there is no lending in the current market, then it is empty string
> annualizedInterestRate30D
string
The lowest annualized interest rate for fixed borrowing for 30 days that the market can currently provide. If there is no lending in the current market, then it is empty string
> annualizedInterestRate60D
string
The lowest annualized interest rate for fixed borrowing for 60 days that the market can currently provide. If there is no lending in the current market, then it is empty string
> annualizedInterestRate90D
string
The lowest annualized interest rate for fixed borrowing for 90 days that the market can currently provide. If there is no lending in the current market, then it is empty string
> annualizedInterestRate180D
string
The lowest annualized interest rate for fixed borrowing for 180 days that the market can currently provide. If there is no lending in the current market, then it is empty string
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-common/loanable-data?currency=ETH&vipLevel=VIP5
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
get_borrowable_coins_new_crypto_loan
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
"currency"
:
"ETH"
,
"fixedBorrowable"
:
true
,
"fixedBorrowingAccuracy"
:
6
,
"flexibleBorrowable"
:
true
,
"flexibleBorrowingAccuracy"
:
4
,
"maxBorrowingAmount"
:
"1100"
,
"minFixedBorrowingAmount"
:
"0.1"
,
"minFlexibleBorrowingAmount"
:
"0.001"
,
"vipLevel"
:
"VIP5"
,
"annualizedInterestRate14D"
:
"0.08"
,
"annualizedInterestRate180D"
:
""
,
"annualizedInterestRate30D"
:
""
,
"annualizedInterestRate60D"
:
""
,
"annualizedInterestRate7D"
:
""
,
"annualizedInterestRate90D"
:
""
,
"flexibleAnnualizedInterestRate"
:
"0.001429799316"
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
1752573126653
}

**Examples:**

Example 1 ():
```
GET /v5/crypto-loan-common/loanable-data?currency=ETH&vipLevel=VIP5 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_borrowable_coins_new_crypto_loan())
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "currency": "ETH",                "fixedBorrowable": true,                "fixedBorrowingAccuracy": 6,                "flexibleBorrowable": true,                "flexibleBorrowingAccuracy": 4,                "maxBorrowingAmount": "1100",                "minFixedBorrowingAmount": "0.1",                "minFlexibleBorrowingAmount": "0.001",                "vipLevel": "VIP5",                "annualizedInterestRate14D": "0.08",                "annualizedInterestRate180D": "",                "annualizedInterestRate30D": "",                "annualizedInterestRate60D": "",                "annualizedInterestRate7D": "",                "annualizedInterestRate90D": "",                "flexibleAnnualizedInterestRate": "0.001429799316"            }        ]    },    "retExtInfo": {},    "time": 1752573126653}
```

---

## Repay

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/repay

**Contents:**
- Repay
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (legacy)
Repay
On this page
Repay
Fully or partially repay a loan. If interest is due, that is paid off first, with the loaned amount being paid off only after due interest.
Permission: "Spot trade"
info
The repaid amount will be deducted from the Funding wallet.
The collateral amount will not be auto returned when you don't fully repay the debt, but you can also adjust collateral amount
HTTP Request
​
POST
/v5/crypto-loan/repay
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
Repay amount
Response Parameters
​
Parameter
Type
Comments
repayId
string
Repayment transaction ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan/repay
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-SIGN
:
XXXXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1728629785224
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
61
{
"orderId"
:
"1794267532472646144"
,
"amount"
:
"100"
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
repayCryptoLoan
(
{
orderId
:
'1794267532472646144'
,
amount
:
'100'
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
"repayId"
:
"1794271131730737664"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1728629786884
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan/repay HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728629785224X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 61{    "orderId": "1794267532472646144",    "amount": "100"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.repay_crypto_loan(        orderId="1794267532472646144",        amount="100",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .repayCryptoLoan({    orderId: '1794267532472646144',    amount: '100',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "repayId": "1794271131730737664"    },    "retExtInfo": {},    "time": 1728629786884}
```

---

## Repay

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/flexible/repay

**Contents:**
- Repay
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Crypto Loan (New)
Flexible Loan
Repay
On this page
Repay
Fully or partially repay a loan. If interest is due, that is paid off first, with the loaned amount being paid off only after due interest.
Permission: "Spot trade"
UID rate limit: 1 req / second
info
The repaid amount will be deducted from the Funding wallet.
The collateral amount will not be auto returned when you don't fully repay the debt, but you can also adjust collateral amount
HTTP Request
​
POST
/v5/crypto-loan-flexible/repay
Request Parameters
​
Parameter
Required
Type
Comments
loanCurrency
true
string
Loan coin name
amount
true
string
Amount to repay
Response Parameters
​
Parameter
Type
Comments
repayId
string
Repayment transaction ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-flexible/repay
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
1752569628364
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
"loanCurrency"
:
"BTC"
,
"amount"
:
"0.005"
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
repay_flexible_crypto_loan
(
loanCurrency
=
"BTC"
,
loanAmount
=
"0.005"
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
"repayId"
:
"1771"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752569614549
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan-flexible/repay HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752569628364X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52{    "loanCurrency": "BTC",    "amount": "0.005"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.repay_flexible_crypto_loan(    loanCurrency="BTC",    loanAmount="0.005",))
```

Example 3 ():
```

```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "repayId": "1771"    },    "retExtInfo": {},    "time": 1752569614549}
```

---

## Repay

**URL:** https://bybit-exchange.github.io/docs/v5/otc/repay

**Contents:**
- Repay
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Institutional Loan
Repay
On this page
Repay
You can repay the INS loan by calling this API.
info
Only the designated Risk Unit UID is allowed to call this API. To obtain the designated Risk Unit UID, please refer to the
parentUid
from
Get LTV
The repayment is processed asynchronously and usually takes 2–3 minutes.
Pease confirm the repayment status via
Get Repayment Orders
before initiating the next repayment.
Note
that the repayment record will not appear in the response until 2–3 minutes later.
HTTP Request
​
POST
/v5/ins-loan/repay-loan
IMPORTANT
Please note this API can only be used when urgent. Make sure contact RM before executing
When repay, principal amount will be deducted from Unified wallet, the interest
not include
Request Parameters
​
Parameter
Required
Type
Comments
token
true
string
Coin name
quantity
true
string
The qty to be repaid
Response Parameters
​
Parameter
Type
Comments
repayOrderStatus
string
P
: processing
Request Example
​
POST
/v5/ins-loan/repay-loan
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
XXXXX
X-BAPI-TIMESTAMP
:
1767605784035
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXX
Content-Type
:
application/json
Content-Length
:
49
{
"token"
:
"USDT"
,
"quantity"
:
"500000"
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
"repayOrderStatus"
:
"P"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1767580441965
}

**Examples:**

Example 1 ():
```
POST /v5/ins-loan/repay-loan HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1767605784035X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/jsonContent-Length: 49{    "token": "USDT",    "quantity": "500000"}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "repayOrderStatus": "P"    },    "retExtInfo": {},    "time": 1767580441965}
```

---
