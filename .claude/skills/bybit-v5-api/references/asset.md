# Bybit-V5-Api - Asset

**Pages:** 52

---

## Get Exchange History

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert-small-balance/exchange-history

**Contents:**

- Get Exchange History
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert Small Balances
Get Exchange History
On this page
Get Exchange History
info
API key permission:
Convert
API rate limit:
10 req /s
You can query all small-balance exchange records made via API or web/app from both the Unified and Funding wallets.
HTTP Request
​
GET
/v5/asset/covert/small-balance-history
Request Parameters
​
Parameter
Required
Type
Comments
accountType
false
string
eb_convert_uta
,
eb_convert_funding
quoteId
false
string
Quote ID, highest priority when querying
startTime
false
string
The start timestamp (ms)
endTime
false
string
The end timestamp (ms)
cursor
false
string
Page number
size
false
string
Page size, default is 50, maximum is 100
Response Parameters
​
Parameter
Type
Comments
cursor
string
Current page number
size
string
Curreng page size
lastPage
string
Last page number
totalCount
string
Total number of records
records
array
<
object
>
> accountType
> string
> eb_convert_uta
: unified wallet,
> eb_convert_funding
: funding wallet
> exchangeTxId
> string
> Exchange transaction ID
> toCoin
> string
> Target currency
> toAmount
> string
> Actual total amount received
> subRecords
> array
<
> object
>
details
> > fromCoin
> > string
> > Source currency
> > fromAmount
> > string
> > Source currency amount
> > toCoin
> > string
> > Target currency
> > toAmount
> > string
> > Actual amount received
> > feeCoin
> > string
> > Exchange fee currency
> > feeAmount
> > string
> > Exchange fee
> > status
> > string
> > init
> > ,
> > processing
> > ,
> > success
> > ,
> > failure
> > ,
> > partial_fulfillment
> > taxFeeInfo
> > object
>>> totalAmount
> > > string
> > > Tax fee amount
> > > feeCoin
> > > string
> > > Tax fee currency
> > > taxFeeItems
> > > array
> > > Tax fee items
> status
> > > string
> > > init
> > > ,
> > > processing
> > > ,
> > > success
> > > ,
> > > failure
> > > ,
> > > partial_fulfillment
> createdAt
> > > string
> > > Quote created timestamp
> exchangeSource
> > > string
> > > Exchange source
> > > small_asset_uta
> > > ,
> > > small_asset_funding
> feeCoin
> > > string
> > > Exchange fee currency
> totalFeeAmount
> > > string
> > > Total exchange fee amount
> totalTaxFeeInfo
> > > object
> > totalAmount
> > > string
> > > Total tax fee amount
> > feeCoin
> > > string
> > > Tax fee currency
> > taxFeeItems
> > > array
> > > Tax fee items
> > > Request Example
> > > ​
> > > HTTP
> > > Python
> > > Node.js
> > > GET
> > > /v5/asset/covert/small-balance-history?quoteId=1010075157602517596339322880&accountType=eb_convert_uta
> > > HTTP/1.1
> > > Host
:
> > > api-testnet.bybit.com
> > > X-BAPI-SIGN
:
> > > XXXXXX
> > > X-BAPI-API-KEY
:
> > > XXXXXX
> > > X-BAPI-TIMESTAMP
:
> > > 1766134218672
> > > X-BAPI-RECV-WINDOW
:
> > > 5000
> > > from
> > > pybit
> > > .
> > > unified_trading
> > > import
> > > HTTP
> > > session
> > > =
> > > HTTP
(
> > > testnet
> > > =
> > > True
> > > ,
> > > api_key
> > > =
"xxxxxxxxxxxxxxxxxx"
> > > ,
> > > api_secret
> > > =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> > > ,
)
> > > print
(
> > > session
> > > .
> > > get_exchange_history_small_balance
(
> > > quoteId
> > > =
"1010075157602517596339322880"
> > > ,
> > > accountType
> > > =
"eb_convert_uta"
> > > ,
)
)
> > > Response Example
> > > ​
> > > {
"retCode"
:
> > > 0
> > > ,
"retMsg"
:
"ok"
> > > ,
"result"
:
> > > {
"cursor"
:
"1"
> > > ,
"size"
:
"50"
> > > ,
"lastPage"
:
"1"
> > > ,
"totalCount"
:
"1"
> > > ,
"records"
:
[
> > > {
"accountType"
:
"eb_convert_uta"
> > > ,
"exchangeTxId"
:
"1010075157602517596339322880"
> > > ,
"toCoin"
:
"USDC"
> > > ,
"toAmount"
:
"0.000728325793503221"
> > > ,
"subRecords"
:
[
> > > {
"fromCoin"
:
"SOL"
> > > ,
"fromAmount"
:
"0.000003"
> > > ,
"toCoin"
:
"USDC"
> > > ,
"toAmount"
:
"0.000363439538230885"
> > > ,
"feeCoin"
:
"USDC"
> > > ,
"feeAmount"
:
"0.000007417133433283"
> > > ,
"status"
:
"success"
> > > ,
"taxFeeInfo"
:
> > > {
"totalAmount"
:
"0"
> > > ,
"feeCoin"
:
""
> > > ,
"taxFeeItems"
:
[
]
> > > }
> > > }
> > > ,
> > > {
"fromCoin"
:
"XRP"
> > > ,
"fromAmount"
:
"0.0002"
> > > ,
"toCoin"
:
"USDC"
> > > ,
"toAmount"
:
"0.000364886255272336"
> > > ,
"feeCoin"
:
"USDC"
> > > ,
"feeAmount"
:
"0.000007446658270864"
> > > ,
"status"
:
"success"
> > > ,
"taxFeeInfo"
:
> > > {
"totalAmount"
:
"0"
> > > ,
"feeCoin"
:
""
> > > ,
"taxFeeItems"
:
[
]
> > > }
> > > }
]
> > > ,
"status"
:
"success"
> > > ,
"createdAt"
:
"1766128195000"
> > > ,
"exchangeSource"
:
"small_asset_uta"
> > > ,
"feeCoin"
:
"USDC"
> > > ,
"totalFeeAmount"
:
"0.000014863791704147"
> > > ,
"totalTaxFeeInfo"
:
> > > {
"totalAmount"
:
"0"
> > > ,
"feeCoin"
:
""
> > > ,
"taxFeeItems"
:
[
]
> > > }
> > > }
]
> > > }
> > > ,
"retExtInfo"
:
> > > {
> > > }
> > > ,
"time"
:
> > > 1766129394948
> > > }

**Examples:**

Example 1 ():

```
GET /v5/asset/covert/small-balance-history?quoteId=1010075157602517596339322880&accountType=eb_convert_uta HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1766134218672X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_exchange_history_small_balance(    quoteId="1010075157602517596339322880",    accountType="eb_convert_uta",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "cursor": "1",        "size": "50",        "lastPage": "1",        "totalCount": "1",        "records": [            {                "accountType": "eb_convert_uta",                "exchangeTxId": "1010075157602517596339322880",                "toCoin": "USDC",                "toAmount": "0.000728325793503221",                "subRecords": [                    {                        "fromCoin": "SOL",                        "fromAmount": "0.000003",                        "toCoin": "USDC",                        "toAmount": "0.000363439538230885",                        "feeCoin": "USDC",                        "feeAmount": "0.000007417133433283",                        "status": "success",                        "taxFeeInfo": {                            "totalAmount": "0",                            "feeCoin": "",                            "taxFeeItems": []                        }                    },                    {                        "fromCoin": "XRP",                        "fromAmount": "0.0002",                        "toCoin": "USDC",                        "toAmount": "0.000364886255272336",                        "feeCoin": "USDC",                        "feeAmount": "0.000007446658270864",                        "status": "success",                        "taxFeeInfo": {                            "totalAmount": "0",                            "feeCoin": "",                            "taxFeeItems": []                        }                    }                ],                "status": "success",                "createdAt": "1766128195000",                "exchangeSource": "small_asset_uta",                "feeCoin": "USDC",                "totalFeeAmount": "0.000014863791704147",                "totalTaxFeeInfo": {                    "totalAmount": "0",                    "feeCoin": "",                    "taxFeeItems": []                }            }        ]    },    "retExtInfo": {},    "time": 1766129394948}
```

---

## Get Coin Exchange Records

**URL:** https://bybit-exchange.github.io/docs/v5/asset/exchange

**Contents:**

- Get Coin Exchange Records
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Get Coin Exchange Records
On this page
Get Coin Exchange Records
Query the coin exchange records.
info
It sometimes has 5 secs delay
HTTP Request
​
GET
/v5/asset/exchange/order-record
Request Parameters
​
Parameter
Required
Type
Comments
fromCoin
false
string
The currency to convert from, uppercase only. e.g,
BTC
toCoin
false
string
The currency to convert to, uppercase only. e.g,
USDT
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
nextPageCursor
string
Refer to the
cursor
request parameter
orderBody
array
Object
> fromCoin
> string
> The currency to convert from
> fromAmount
> string
> The amount to convert from
> toCoin
> string
> The currency to convert to
> toAmount
> string
> The amount to convert to
> exchangeRate
> string
> Exchange rate
> createdTime
> string
> Exchange created timestamp (sec)
> exchangeTxId
> string
> Exchange transaction ID
> RUN >>
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/asset/exchange/order-record?limit=10
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1672990462492
> X-BAPI-RECV-WINDOW
:
> 5000
> from
> pybit
> .
> unified_trading
> import
> HTTP
> session
> =
> HTTP
(
> testnet
> =
> True
> ,
> api_key
> =
"xxxxxxxxxxxxxxxxxx"
> ,
> api_secret
> =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> ,
)
> print
(
> session
> .
> get_coin_exchange_records
(
> limit
> =
> 10
> ,
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> getCoinExchangeRecords
(
> {
> limit
:
> 10
> }
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"OK"
> ,
"result"
:
> {
"orderBody"
:
[
{
"fromCoin"
:
"BTC"
,
"fromAmount"
:
"0.100000000000000000"
,
"toCoin"
:
"ETH"
,
"toAmount"
:
"1.385866230000000000"
,
"exchangeRate"
:
"13.858662380000000000"
,
"createdTime"
:
"1672197760"
,
"exchangeTxId"
:
"145102533285208544812654440448"
}
]
> ,
"nextPageCursor"
:
"173341:1672197760"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1672990464021
> }

**Examples:**

Example 1 ():

```
GET /v5/asset/exchange/order-record?limit=10 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672990462492X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_coin_exchange_records(    limit=10,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getCoinExchangeRecords({ limit: 10 })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderBody": [            {                "fromCoin": "BTC",                "fromAmount": "0.100000000000000000",                "toCoin": "ETH",                "toAmount": "1.385866230000000000",                "exchangeRate": "13.858662380000000000",                "createdTime": "1672197760",                "exchangeTxId": "145102533285208544812654440448"            }        ],        "nextPageCursor": "173341:1672197760"    },    "retExtInfo": {},    "time": 1672990464021}
```

---

## Get Pre-upgrade Delivery Record

**URL:** https://bybit-exchange.github.io/docs/v5/pre-upgrade/delivery

**Contents:**

- Get Pre-upgrade Delivery Record
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Pre-upgrade
Get Pre-upgrade Delivery Record
On this page
Get Pre-upgrade Delivery Record
Query delivery records of Options before you upgraded the account to a Unified account, sorted by
deliveryTime
in descending order
info
By
category
="option", you can query Options delivery data occurred during classic account
Supports the recent 6 months Options delivery data. Please download older data via GUI
HTTP Request
​
GET
/v5/pre-upgrade/asset/delivery-record
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
option
symbol
false
string
Symbol name, uppercase only
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
Cursor. Used for pagination
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
> number
> Delivery time (ms)
> symbol
> string
> Symbol name
> side
> string
> Buy
> ,
> Sell
> position
> string
> Executed size
> deliveryPrice
> string
> Delivery price
> strike
> string
> Exercise price
> fee
> string
> Trading fee
> deliveryRpl
> string
> Realized PnL of the delivery
> nextPageCursor
> string
> Cursor. Used for pagination
> Request Example
> ​
> HTTP
> Python
> GET
> /v5/pre-upgrade/asset/delivery-record?category=option
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1686809005774
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"OK"
> ,
"result"
:
> {
"nextPageCursor"
:
"21%3A0%2C21%3A0"
> ,
"category"
:
"option"
> ,
"list"
:
[
{
"symbol"
:
"ETH-14JUN23-1750-C"
,
"side"
:
"Buy"
,
"deliveryTime"
:
1686729604507
,
"strike"
:
"1750"
,
"fee"
:
"0"
,
"position"
:
"0.5"
,
"deliveryPrice"
:
"1740.25036667"
,
"deliveryRpl"
:
"0.175"
}
]
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1686796328492
> }

**Examples:**

Example 1 ():

```
GET /v5/pre-upgrade/asset/delivery-record?category=option HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1686809005774X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "21%3A0%2C21%3A0",        "category": "option",        "list": [            {                "symbol": "ETH-14JUN23-1750-C",                "side": "Buy",                "deliveryTime": 1686729604507,                "strike": "1750",                "fee": "0",                "position": "0.5",                "deliveryPrice": "1740.25036667",                "deliveryRpl": "0.175"            }        ]    },    "retExtInfo": {},    "time": 1686796328492}
```

---

## Get Universal Transfer Records

**URL:** https://bybit-exchange.github.io/docs/v5/asset/transfer/unitransfer-list

**Contents:**

- Get Universal Transfer Records
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Transfer
Get Universal Transfer Records
On this page
Get Universal Transfer Records
Query universal transfer records
tip
Main acct api key or Sub acct api key are both supported
Main acct api key needs "SubMemberTransfer" permission
Sub acct api key needs "SubMemberTransferList" permission
info
If startTime and endTime are not provided, the API returns data from the past 7 days by default.
If only startTime is provided, the API returns records from startTime to startTime + 7 days.
If only endTime is provided, the API returns records from endTime - 7 days to endTime.
If both are provided, the maximum allowed range is 7 days (endTime - startTime ≤ 7 days).
HTTP Request
​
GET
/v5/asset/transfer/query-universal-transfer-list
Request Parameters
​
Parameter
Required
Type
Comments
transferId
false
string
UUID. Use the one you generated in
createTransfer
coin
false
string
Coin, uppercase only
status
false
string
Transfer status.
SUCCESS
,
FAILED
,
PENDING
startTime
false
integer
The start timestamp (ms)
Note: the query logic is actually effective based on
second
level
endTime
false
integer
The end timestamp (ms)
Note: the query logic is actually effective based on
second
level
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
> transferId
> string
> Transfer ID
> coin
> string
> Transferred coin
> amount
> string
> Transferred amount
> fromMemberId
> string
> From UID
> toMemberId
> string
> TO UID
> fromAccountType
> string
> From account type
> toAccountType
> string
> To account type
> timestamp
> string
> Transfer created timestamp (ms)
>
status
string
Transfer status
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
/v5/asset/transfer/query-universal-transfer-list?limit=1&cursor=eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9
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
1672190762800
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
get_universal_transfer_records
(
limit
=
1
,
cursor
=
"eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9"
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
getUniversalTransferRecords
(
{
limit
:
1
,
cursor
:
'eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9'
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
"list"
:
[
{
"transferId"
:
"universalTransfer_4c3cfe2f-85cb-11ed-ac09-9e37823c81cd_533285"
,
"coin"
:
"USDC"
,
"amount"
:
"1000"
,
"timestamp"
:
"1672134373000"
,
"status"
:
"SUCCESS"
,
"fromAccountType"
:
"UNIFIED"
,
"toAccountType"
:
"UNIFIED"
,
"fromMemberId"
:
"533285"
,
"toMemberId"
:
"592324"
}
]
,
"nextPageCursor"
:
"eyJtaW5JRCI6MTc4OTYwNSwibWF4SUQiOjE3ODk2MDV9"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672190763079
}

**Examples:**

Example 1 ():

```
GET /v5/asset/transfer/query-universal-transfer-list?limit=1&cursor=eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672190762800X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_universal_transfer_records(    limit=1,    cursor="eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getUniversalTransferRecords({    limit: 1,    cursor: 'eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "transferId": "universalTransfer_4c3cfe2f-85cb-11ed-ac09-9e37823c81cd_533285",                "coin": "USDC",                "amount": "1000",                "timestamp": "1672134373000",                "status": "SUCCESS",                "fromAccountType": "UNIFIED",                "toAccountType": "UNIFIED",                "fromMemberId": "533285",                "toMemberId": "592324"            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTc4OTYwNSwibWF4SUQiOjE3ODk2MDV9"    },    "retExtInfo": {},    "time": 1672190763079}
```

---

## Confirm a Quote

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert/confirm-quote

**Contents:**

- Confirm a Quote
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert
Confirm a Quote
On this page
Confirm a Quote
info
The exchange is async; please check the final status by calling the query result API.
Make sure you confirm the quote before it expires.
HTTP Request
​
POST
/v5/asset/exchange/convert-execute
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
Response Parameters
​
Parameter
Type
Comments
quoteTxId
string
Quote transaction ID
exchangeStatus
string
Exchange status
init
processing
success
failure
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/exchange/convert-execute
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
"10100108106409343501030232064"
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
confirm_a_quote
(
quoteTxId
=
"10100108106409343501030232064"
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
confirmConvertQuote
(
{
quoteTxId
:
'10100108106409343501030232064'
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
"ok"
,
"result"
:
{
"exchangeStatus"
:
"processing"
,
"quoteTxId"
:
"10100108106409343501030232064"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1720071900529
}

**Examples:**

Example 1 ():

```
POST /v5/asset/exchange/convert-execute HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720071899789X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52{    "quoteTxId": "10100108106409343501030232064"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.confirm_a_quote(    quoteTxId="10100108106409343501030232064",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .confirmConvertQuote({    quoteTxId: '10100108106409343501030232064',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "exchangeStatus": "processing",        "quoteTxId": "10100108106409343501030232064"    },    "retExtInfo": {},    "time": 1720071900529}
```

---

## Create Sub UID API Key

**URL:** https://bybit-exchange.github.io/docs/v5/user/create-subuid-apikey

**Contents:**

- Create Sub UID API Key
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Create Sub UID API Key
On this page
Create Sub UID API Key
To create new API key for those newly created sub UID. Use
master user's api key
only
.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
HTTP Request
​
POST
/v5/user/create-sub-api
Request Parameters
​
Parameter
Required
Type
Comments
subuid
true
integer
Sub user Id
note
false
string
Set a remark
readOnly
true
integer
0
：Read and Write.
1
：Read only
ips
false
string
Set the IP bind. example:
"192.168.0.1,192.168.0.2"
note:
don't pass ips or pass with
"*"
means no bind
No ip bound api key will be
invalid after 90 days
api key without IP bound will be invalid after
7 days
once the account password is changed
permissions
true
Object
Tick the types of permission.
one of below types must be passed, otherwise the error is thrown
> ContractTrade
> false
> array
> Contract Trade.
["Order","Position"]
> Spot
> false
> array
> Spot Trade.
["SpotTrade"]
> Options
> false
> array
> USDC Contract.
["OptionsTrade"]
> Wallet
> false
> array
> Wallet.
["AccountTransfer","SubMemberTransferList"]
> Note: Fund Custodial account is not supported
> Exchange
> false
> array
> Convert.
["ExchangeHistory"]
> Earn
> false
> array
> Earn product.
["Earn"]
> Response Parameters
> ​
> Parameter
> Type
> Comments
> id
> string
> Unique id. Internal used
> note
> string
> The remark
> apiKey
> string
> Api key
> readOnly
> integer
> 0
: Read and Write.
> 1
: Read only
> secret
> string
> The secret paired with api key.
> The secret can't be queried by GET api. Please keep it properly
> permissions
> Object
> The types of permission
> ContractTrade
> array
> Permisson of contract trade
> Spot
> array
> Permisson of spot
> Wallet
> array
> Permisson of wallet
> Options
> array
> Permission of USDC Contract. It supports trade option and usdc perpetual.
> Derivatives
> array
> Permission of Unified account
> Exchange
> array
> Permission of convert
> Earn
> array
> Permission of earn product
> BlockTrade
> array
> Not applicable to sub account, always
[]
> Affiliate
> array
> Not applicable to sub account, always
[]
> FiatP2P
> array
> Not applicable to sub account, always
[]
> FiatBybitPay
> array
> Not applicable to sub account, always
[]
> FiatConvertBroker
> array
> Not applicable to sub account, always
[]
> NFT
> array
> Deprecated
> , always
[]
> CopyTrading
> array
> Deprecated
> always
[]
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/user/create-sub-api
> HTTP/1.1
> Host
:
> api.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1676430005459
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"subuid"
:
> 53888000
> ,
"note"
:
"testxxx"
> ,
"readOnly"
:
> 0
> ,
"permissions"
:
> {
"Wallet"
:
[
"AccountTransfer"
]
> }
> }
> from
> pybit
> .
> unified_trading
> import
> HTTP
> session
> =
> HTTP
(
> testnet
> =
> True
> ,
> api_key
> =
"xxxxxxxxxxxxxxxxxx"
> ,
> api_secret
> =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> ,
)
> print
(
> session
> .
> create_sub_api_key
(
> subuid
> =
> 53888000
> ,
> note
> =
"testxxx"
> ,
> readOnly
> =
> 0
> ,
> permissions
> =
> {
"Wallet"
:
[
"AccountTransfer"
]
> }
> ,
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> createSubUIDAPIKey
(
> {
> subuid
:
> 53888000
> ,
> note
:
'testxxx'
> ,
> readOnly
:
> 0
> ,
> permissions
:
> {
> Wallet
:
[
'AccountTransfer'
]
> ,
> }
> ,
> }
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
""
> ,
"result"
:
> {
"id"
:
"16651283"
> ,
"note"
:
"testxxx"
> ,
"apiKey"
:
"xxxxx"
> ,
"readOnly"
:
> 0
> ,
"secret"
:
"xxxxxxxx"
> ,
"permissions"
:
> {
"ContractTrade"
:
[
]
> ,
"Spot"
:
[
]
> ,
"Wallet"
:
[
"AccountTransfer"
]
> ,
"Options"
:
[
]
> ,
"CopyTrading"
:
[
]
> ,
"BlockTrade"
:
[
]
> ,
"Exchange"
:
[
]
> ,
"NFT"
:
[
]
> ,
"Earn"
:
[
"Earn"
]
> }
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1676430007643
> }

**Examples:**

Example 1 ():

```
POST /v5/user/create-sub-api HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430005459X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "subuid": 53888000,    "note": "testxxx",    "readOnly": 0,    "permissions": {        "Wallet": [            "AccountTransfer"        ]    }}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.create_sub_api_key(    subuid=53888000,    note="testxxx",    readOnly=0,    permissions={        "Wallet": [            "AccountTransfer"        ]    },))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .createSubUIDAPIKey({    subuid: 53888000,    note: 'testxxx',    readOnly: 0,    permissions: {      Wallet: ['AccountTransfer'],    },  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "16651283",        "note": "testxxx",        "apiKey": "xxxxx",        "readOnly": 0,        "secret": "xxxxxxxx",        "permissions": {            "ContractTrade": [],            "Spot": [],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": [],            "Earn": ["Earn"]        }    },    "retExtInfo": {},    "time": 1676430007643}
```

---

## Get Convert History

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert/get-convert-history

**Contents:**

- Get Convert History
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert
Get Convert History
On this page
Get Convert History
Returns all confirmed quotes.
info
Starting from September 10, 2025, converts executed on the webpage can also be queried via this API.
HTTP Request
​
GET
/v5/asset/exchange/query-convert-history
Request Parameters
​
Parameter
Required
Type
Comments
accountType
false
string
Wallet type
eb_convert_funding
: funding wallet convert records via API
eb_convert_uta
: uta wallet convert records via API
funding
: normal crypto convert via web/app
funding_fiat
: fiat crypto convert via web/app
funding_fbtc_convert
: FBTC to BTC convert via web/app
funding_block_trade
: block trade convert via web/app
Supports passing multiple types, separated by comma e.g.,
eb_convert_funding,eb_convert_uta
Return all wallet types data if not passed
index
false
integer
Page number
started from 1
1st page by default
limit
false
integer
Page size
20 records by default
up to 100 records, return 100 when exceeds 100
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
Array of quotes
>
accountType
string
Wallet type
eb_convert_funding
: funding wallet convert records via API
eb_convert_uta
: uta wallet convert records via API
funding
: normal crypto convert via web/app
funding_fiat
: fiat crypto convert via web/app
funding_fbtc_convert
: FBTC to BTC convert via web/app
funding_block_trade
: block trade convert via web/app
> exchangeTxId
> string
> Exchange tx ID, same as quote tx ID
> userId
> string
> User ID
> fromCoin
> string
> From coin
> fromCoinType
> string
> From coin type.
> crypto
> toCoin
> string
> To coin
> toCoinType
> string
> To coin type.
> crypto
> fromAmount
> string
> From coin amount (amount to sell)
> toAmount
> string
> To coin amount (amount to buy according to exchange rate)
> exchangeStatus
> string
> Exchange status
> init
> processing
> success
> failure
> extInfo
> object
>> paramType
> > string
> > This field is published when you send it in the
> > Request a Quote
> > paramValue
> > string
> > This field is published when you send it in the
> > Request a Quote
> convertRate
> > string
> > Exchange rate
> createdAt
> > string
> > Quote created time
> > Request Example
> > ​
> > HTTP
> > Python
> > Node.js
> > GET
> > /v5/asset/exchange/query-convert-history?accountType=eb_convert_uta,eb_convert_funding
> > HTTP/1.1
> > Host
:
> > api-testnet.bybit.com
> > X-BAPI-SIGN
:
> > XXXXXX
> > X-BAPI-API-KEY
:
> > xxxxxxxxxxxxxxxxxx
> > X-BAPI-TIMESTAMP
:
> > 1720074159814
> > X-BAPI-RECV-WINDOW
:
> > 5000
> > from
> > pybit
> > .
> > unified_trading
> > import
> > HTTP
> > session
> > =
> > HTTP
(
> > testnet
> > =
> > True
> > ,
> > api_key
> > =
"xxxxxxxxxxxxxxxxxx"
> > ,
> > api_secret
> > =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> > ,
)
> > print
(
> > session
> > .
> > get_convert_history
(
> > accountType
> > =
"eb_convert_uta,eb_convert_funding"
> > ,
)
)
> > const
> > {
> > RestClientV5
> > }
> > =
> > require
(
'bybit-api'
)
> > ;
> > const
> > client
> > =
> > new
> > RestClientV5
(
> > {
> > testnet
:
> > true
> > ,
> > key
:
'xxxxxxxxxxxxxxxxxx'
> > ,
> > secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> > ,
> > }
)
> > ;
> > client
> > .
> > getConvertHistory
(
)
> > .
> > then
(
(
> > response
)
> > =>
> > {
> > console
> > .
> > log
(
> > response
)
> > ;
> > }
)
> > .
> > catch
(
(
> > error
)
> > =>
> > {
> > console
> > .
> > error
(
> > error
)
> > ;
> > }
)
> > ;
> > Response Example
> > ​
> > {
"retCode"
:
> > 0
> > ,
"retMsg"
:
"ok"
> > ,
"result"
:
> > {
"list"
:
[
{
"accountType"
:
"eb_convert_funding"
,
"exchangeTxId"
:
"10100108106409343501030232064"
,
"userId"
:
"XXXXX"
,
"fromCoin"
:
"ETH"
,
"fromCoinType"
:
"crypto"
,
"fromAmount"
:
"0.1"
,
"toCoin"
:
"BTC"
,
"toCoinType"
:
"crypto"
,
"toAmount"
:
"0.00534882723991"
,
"exchangeStatus"
:
"success"
,
"extInfo"
:
{
"paramType"
:
"opFrom"
,
"paramValue"
:
"broker-id-001"
}
,
"convertRate"
:
"0.0534882723991"
,
"createdAt"
:
"1720071899995"
}
,
{
"accountType"
:
"eb_convert_uta"
,
"exchangeTxId"
:
"23070eb_convert_uta408933875189391360"
,
"userId"
:
"XXXXX"
,
"fromCoin"
:
"BTC"
,
"fromCoinType"
:
"crypto"
,
"fromAmount"
:
"0.1"
,
"toCoin"
:
"ETH"
,
"toCoinType"
:
"crypto"
,
"toAmount"
:
"1.773938248611074"
,
"exchangeStatus"
:
"success"
,
"extInfo"
:
{
}
,
"convertRate"
:
"17.73938248611074"
,
"createdAt"
:
"1719974243256"
}
]
> > }
> > ,
"retExtInfo"
:
> > {
> > }
> > ,
"time"
:
> > 1720074457715
> > }

**Examples:**

Example 1 ():

```
GET /v5/asset/exchange/query-convert-history?accountType=eb_convert_uta,eb_convert_funding HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720074159814X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_convert_history(    accountType="eb_convert_uta,eb_convert_funding",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getConvertHistory()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "accountType": "eb_convert_funding",                "exchangeTxId": "10100108106409343501030232064",                "userId": "XXXXX",                "fromCoin": "ETH",                "fromCoinType": "crypto",                "fromAmount": "0.1",                "toCoin": "BTC",                "toCoinType": "crypto",                "toAmount": "0.00534882723991",                "exchangeStatus": "success",                "extInfo": {                    "paramType": "opFrom",                    "paramValue": "broker-id-001"                },                "convertRate": "0.0534882723991",                "createdAt": "1720071899995"            },            {                "accountType": "eb_convert_uta",                "exchangeTxId": "23070eb_convert_uta408933875189391360",                "userId": "XXXXX",                "fromCoin": "BTC",                "fromCoinType": "crypto",                "fromAmount": "0.1",                "toCoin": "ETH",                "toCoinType": "crypto",                "toAmount": "1.773938248611074",                "exchangeStatus": "success",                "extInfo": {},                "convertRate": "17.73938248611074",                "createdAt": "1719974243256"            }        ]    },    "retExtInfo": {},    "time": 1720074457715}
```

---

## Get Sub Deposit Address

**URL:** https://bybit-exchange.github.io/docs/v5/asset/deposit/sub-deposit-addr

**Contents:**

- Get Sub Deposit Address
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Deposit
Get Sub Deposit Address
On this page
Get Sub Deposit Address
Query the deposit address information of SUB account.
info
Use master UID's api key
only
Custodial sub account deposit address cannot be obtained
HTTP Request
​
GET
/v5/asset/deposit/query-sub-member-address
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
Coin, uppercase only
chainType
true
string
Please use the value of
chain
from
coin-info
endpoint
subMemberId
true
string
Sub user ID
Response Parameters
​
Parameter
Type
Comments
coin
string
Coin
chains
array
Object
> chainType
> string
> Chain type
> addressDeposit
> string
> The address for deposit
> tagDeposit
> string
> Tag of deposit
> chain
> string
> Chain
> batchReleaseLimit
> string
> The deposit limit for this coin in this chain.
"-1"
> means no limit
> contractAddress
> string
> The contract address of the coin. Only display last 6 characters, if there is no contract address, it shows
""
> RUN >>
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/asset/deposit/query-sub-member-address?coin=USDT&chainType=TRX&subMemberId=592334
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1672194349421
> X-BAPI-RECV-WINDOW
:
> 5000
> from
> pybit
> .
> unified_trading
> import
> HTTP
> session
> =
> HTTP
(
> testnet
> =
> True
> ,
> api_key
> =
"xxxxxxxxxxxxxxxxxx"
> ,
> api_secret
> =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> ,
)
> print
(
> session
> .
> get_sub_deposit_address
(
> coin
> =
"USDT"
> ,
> chainType
> =
"TRX"
> ,
> subMemberId
> =
> 592334
> ,
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> getSubDepositAddress
(
'USDT'
> ,
'TRX'
> ,
'592334'
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"success"
> ,
"result"
:
> {
"coin"
:
"USDT"
> ,
"chains"
:
> {
"chainType"
:
"TRC20"
> ,
"addressDeposit"
:
"XXXXXX"
> ,
"tagDeposit"
:
""
> ,
"chain"
:
"TRX"
> ,
"batchReleaseLimit"
:
"-1"
> ,
"contractAddress"
:
"gjLj6t"
> }
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1736394845821
> }

**Examples:**

Example 1 ():

```
GET /v5/asset/deposit/query-sub-member-address?coin=USDT&chainType=TRX&subMemberId=592334 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672194349421X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_sub_deposit_address(    coin="USDT",    chainType="TRX",    subMemberId=592334,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSubDepositAddress('USDT', 'TRX', '592334')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "coin": "USDT",        "chains": {            "chainType": "TRC20",            "addressDeposit": "XXXXXX",            "tagDeposit": "",            "chain": "TRX",            "batchReleaseLimit": "-1",            "contractAddress": "gjLj6t"        }    },    "retExtInfo": {},    "time": 1736394845821}
```

---

## Modify Sub API Key

**URL:** https://bybit-exchange.github.io/docs/v5/user/modify-sub-apikey

**Contents:**

- Modify Sub API Key
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Modify Sub API Key
On this page
Modify Sub API Key
Modify the settings of sub api key. Use the sub account api key pending to be modified to call the endpoint or use
master
account api key to manage its sub account api key.
tip
The API key must have one of the below permissions in order to call this endpoint
sub API key: "Account Transfer", "Sub Member Transfer"
master API Key: "Account Transfer", "Sub Member Transfer", "Withdrawal"
HTTP Request
​
POST
/v5/user/update-sub-api
Request Parameters
​
Parameter
Required
Type
Comments
apikey
false
string
Sub account api key
You must pass this param when you use master account manage sub account api key settings
If you use corresponding sub uid api key call this endpoint,
apikey
param cannot be passed, otherwise throwing an error
readOnly
false
integer
0
(default)：Read and Write.
1
：Read only
ips
false
string
Set the IP bind. example:
"192.168.0.1,192.168.0.2"
note:
don't pass ips or pass with
"*"
means no bind
No ip bound api key will be
invalid after 90 days
api key will be invalid after
7 days
once the account password is changed
permissions
false
Object
Tick the types of permission. Don't send this param if you don't want to change the permission
> ContractTrade
> false
> array
> Contract Trade.
["Order","Position"]
> Spot
> false
> array
> Spot Trade.
["SpotTrade"]
> Wallet
> false
> array
> Wallet.
["AccountTransfer", "SubMemberTransferList"]
> Note: fund custodial account is not supported
> Options
> false
> array
> USDC Contract.
["OptionsTrade"]
> Derivatives
> false
> array
["DerivativesTrade"]
> Exchange
> false
> array
> Convert.
["ExchangeHistory"]
> Earn
> false
> array
> Earn product.
["Earn"]
> Response Parameters
> ​
> Parameter
> Type
> Comments
> id
> string
> Unique id. Internal used
> note
> string
> The remark
> apiKey
> string
> Api key
> readOnly
> integer
> 0
: Read and Write.
> 1
: Read only
> secret
> string
> Always
""
> permissions
> Object
> The types of permission
> ContractTrade
> array
> Permisson of contract trade
> Spot
> array
> Permisson of spot
> Wallet
> array
> Permisson of wallet
> Options
> array
> Permission of USDC Contract. It supports trade option and usdc perpetual.
> Derivatives
> array
> Permission of Unified account
> Exchange
> array
> Permission of convert
> Earn
> array
> Permission of Earn
> BlockTrade
> array
> Not applicable to sub account, always
[]
> Affiliate
> array
> Not applicable to sub account, always
[]
> FiatP2P
> array
> Not applicable to sub account, always
[]
> FiatBybitPay
> array
> Not applicable to sub account, always
[]
> FiatConvertBroker
> array
> Not applicable to sub account, always
[]
> NFT
> array
> Deprecated
> , always
[]
> CopyTrading
> array
> Deprecated
> , always
[]
> ips
> array
> IP bound
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/user/update-sub-api
> HTTP/1.1
> Host
:
> api.bybit.com
> X-BAPI-SIGN
:
> XXXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1676431795752
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"readOnly"
:
> 0
> ,
"ips"
:
"*"
,
"permissions"
:
{
"ContractTrade"
:
[
]
,
"Spot"
:
[
"SpotTrade"
]
,
"Wallet"
:
[
"AccountTransfer"
]
,
"Options"
:
[
]
,
"CopyTrading"
:
[
]
,
"BlockTrade"
:
[
]
,
"Exchange"
:
[
]
,
"NFT"
:
[
]
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
modify_sub_api_key
(
readOnly
=
0
,
ips
=
"*"
> ,
> permissions
> =
> {
"ContractTrade"
:
[
]
> ,
"Spot"
:
[
"SpotTrade"
]
> ,
"Wallet"
:
[
"AccountTransfer"
]
> ,
"Options"
:
[
]
> ,
"Derivatives"
:
[
]
> ,
"CopyTrading"
:
[
]
> ,
"BlockTrade"
:
[
]
> ,
"Exchange"
:
[
]
> ,
"NFT"
:
[
]
> }
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> updateSubApiKey
(
> {
> readOnly
:
> 0
> ,
> ips
:
[
'*'
]
> ,
> permissions
:
> {
> ContractTrade
:
[
]
> ,
> Spot
:
[
'SpotTrade'
]
> ,
> Wallet
:
[
'AccountTransfer'
]
> ,
> Options
:
[
]
> ,
> Derivatives
:
[
]
> ,
> CopyTrading
:
[
]
> ,
> BlockTrade
:
[
]
> ,
> Exchange
:
[
]
> ,
> NFT
:
[
]
> ,
> }
> ,
> }
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
""
> ,
"result"
:
> {
"id"
:
"16651472"
> ,
"note"
:
"testxxx"
> ,
"apiKey"
:
"xxxxxx"
> ,
"readOnly"
:
> 0
> ,
"secret"
:
""
> ,
"permissions"
:
> {
"ContractTrade"
:
[
]
> ,
"Spot"
:
[
"SpotTrade"
]
> ,
"Wallet"
:
[
"AccountTransfer"
]
> ,
"Options"
:
[
]
> ,
"Derivatives"
:
[
]
> ,
"CopyTrading"
:
[
]
> ,
"BlockTrade"
:
[
]
> ,
"Exchange"
:
[
]
> ,
"NFT"
:
[
]
> }
> ,
"ips"
:
[
"*"
]
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1676431796263
> }

**Examples:**

Example 1 ():

```
POST /v5/user/update-sub-api HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676431795752X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "readOnly": 0,    "ips": "*",    "permissions": {            "ContractTrade": [],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": []        }}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.modify_sub_api_key(    readOnly=0,    ips="*",    permissions={            "ContractTrade": [],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "Derivatives": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": []        }))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .updateSubApiKey({    readOnly: 0,    ips: ['*'],    permissions: {      ContractTrade: [],      Spot: ['SpotTrade'],      Wallet: ['AccountTransfer'],      Options: [],      Derivatives: [],      CopyTrading: [],      BlockTrade: [],      Exchange: [],      NFT: [],    },  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "16651472",        "note": "testxxx",        "apiKey": "xxxxxx",        "readOnly": 0,        "secret": "",        "permissions": {            "ContractTrade": [],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "Derivatives": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": []        },        "ips": [            "*"        ]    },    "retExtInfo": {},    "time": 1676431796263}
```

---

## Cancel Withdrawal

**URL:** https://bybit-exchange.github.io/docs/v5/asset/withdraw/cancel-withdraw

**Contents:**

- Cancel Withdrawal
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Withdraw
Cancel Withdrawal
On this page
Cancel Withdrawal
Cancel the withdrawal
HTTP Request
​
POST
/v5/asset/withdraw/cancel
Request Parameters
​
Parameter
Required
Type
Comments
id
true
string
Withdrawal ID
Response Parameters
​
Parameter
Type
Comments
status
integer
0
: fail.
1
: success
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/withdraw/cancel
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672197227732
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXX
Content-Type
:
application/json
{
"id"
:
"10197"
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
cancel_withdrawal
(
id
=
"10197"
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
cancelWithdrawal
(
'10197'
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
"status"
:
1
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672197228408
}

**Examples:**

Example 1 ():

```
POST /v5/asset/withdraw/cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672197227732X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "id": "10197"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.cancel_withdrawal(    id="10197",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .cancelWithdrawal('10197')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "status": 1    },    "retExtInfo": {},    "time": 1672197228408}
```

---

## Confirm a Quote

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert-small-balance/confirm-quote

**Contents:**

- Confirm a Quote
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert Small Balances
Confirm a Quote
On this page
Confirm a Quote
info
API key permission:
Convert
API rate limit:
5 req /s
The exchange is async; please check the final status by calling the query
Get Exchange History
.
Make sure you confirm the quote before it expires.
HTTP Request
​
POST
/v5/asset/covert/small-balance-execute
Request Parameters
​
Parameter
Required
Type
Comments
quoteId
true
string
The quote ID from
Request a Quote
Response Parameters
​
Parameter
Type
Comments
quoteId
string
Quote ID
exchangeTxId
string
Exchange ID, the same value as
quoteId
submitTime
string
Submit ts
status
string
init
,
processing
,
success
,
failure
,
partial_fulfillment
msg
string
By default is
""
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/covert/small-balance-execute
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
XXXXXX
X-BAPI-TIMESTAMP
:
1766128195297
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
Content-Length
:
49
{
"quoteId"
:
"1010075157602517596339322880"
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
confirm_a_quote_small_balance
(
quoteId
=
"1010075157602517596339322880"
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
"quoteId"
:
"1010075157602517596339322880"
,
"exchangeTxId"
:
"1010075157602517596339322880"
,
"submitTime"
:
"1766128195512"
,
"status"
:
"processing"
,
"msg"
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
1766128195512
}

**Examples:**

Example 1 ():

```
POST /v5/asset/covert/small-balance-execute HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1766128195297X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 49{    "quoteId": "1010075157602517596339322880"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.confirm_a_quote_small_balance(    quoteId="1010075157602517596339322880",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "quoteId": "1010075157602517596339322880",        "exchangeTxId": "1010075157602517596339322880",        "submitTime": "1766128195512",        "status": "processing",        "msg": ""    },    "retExtInfo": {},    "time": 1766128195512}
```

---

## Get Exchange Entity List

**URL:** https://bybit-exchange.github.io/docs/v5/asset/withdraw/vasp-list

**Contents:**

- Get Exchange Entity List
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Withdraw
Get Exchange Entity List (KOR)
On this page
Get Exchange Entity List
This endpoint is particularly used for
kyc=KOR users
. When withdraw funds, you need to fill entity id.
HTTP Request
​
GET
/v5/asset/withdraw/vasp/list
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
vasp
array
Exchange entity info
> vaspEntityId
> string
> Receiver platform id. When transfer to Upbit or other exchanges that not in the list, please use vaspEntityId='others'
> vaspName
> string
> Receiver platform name
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/asset/withdraw/vasp/list
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1715067106163
> X-BAPI-RECV-WINDOW
:
> 5000
> X-BAPI-SIGN
:
> XXXXXX
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> getExchangeEntities
(
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"success"
> ,
"result"
:
> {
"vasp"
:
[
{
"vaspEntityId"
:
"basic-finance"
,
"vaspName"
:
"Basic-finance"
}
,
{
"vaspEntityId"
:
"beeblock"
,
"vaspName"
:
"Beeblock"
}
,
{
"vaspEntityId"
:
"bithumb"
,
"vaspName"
:
"bithumb"
}
,
{
"vaspEntityId"
:
"cardo"
,
"vaspName"
:
"cardo"
}
,
{
"vaspEntityId"
:
"codevasp"
,
"vaspName"
:
"codevasp"
}
,
{
"vaspEntityId"
:
"codexchange-kor"
,
"vaspName"
:
"CODExchange-kor"
}
,
{
"vaspEntityId"
:
"coinone"
,
"vaspName"
:
"coinone"
}
,
{
"vaspEntityId"
:
"dummy"
,
"vaspName"
:
"Dummy"
}
,
{
"vaspEntityId"
:
"flata-exchange"
,
"vaspName"
:
"flataexchange"
}
,
{
"vaspEntityId"
:
"fobl"
,
"vaspName"
:
"Foblgate"
}
,
{
"vaspEntityId"
:
"hanbitco"
,
"vaspName"
:
"hanbitco"
}
,
{
"vaspEntityId"
:
"hexlant"
,
"vaspName"
:
"hexlant"
}
,
{
"vaspEntityId"
:
"inex"
,
"vaspName"
:
"INEX"
}
,
{
"vaspEntityId"
:
"infiniteblock-corp"
,
"vaspName"
:
"InfiniteBlock Corp"
}
,
{
"vaspEntityId"
:
"kdac"
,
"vaspName"
:
"kdac"
}
,
{
"vaspEntityId"
:
"korbit"
,
"vaspName"
:
"korbit"
}
,
{
"vaspEntityId"
:
"paycoin"
,
"vaspName"
:
"Paycoin"
}
,
{
"vaspEntityId"
:
"qbit"
,
"vaspName"
:
"Qbit"
}
,
{
"vaspEntityId"
:
"tennten"
,
"vaspName"
:
"TENNTEN"
}
,
{
"vaspEntityId"
:
"others"
,
"vaspName"
:
"Others (including Upbit)"
}
]
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1715067106537
> }

**Examples:**

Example 1 ():

```
GET /v5/asset/withdraw/vasp/list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1715067106163X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getExchangeEntities()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "vasp": [            {                "vaspEntityId": "basic-finance",                "vaspName": "Basic-finance"            },            {                "vaspEntityId": "beeblock",                "vaspName": "Beeblock"            },            {                "vaspEntityId": "bithumb",                "vaspName": "bithumb"            },            {                "vaspEntityId": "cardo",                "vaspName": "cardo"            },            {                "vaspEntityId": "codevasp",                "vaspName": "codevasp"            },            {                "vaspEntityId": "codexchange-kor",                "vaspName": "CODExchange-kor"            },            {                "vaspEntityId": "coinone",                "vaspName": "coinone"            },            {                "vaspEntityId": "dummy",                "vaspName": "Dummy"            },            {                "vaspEntityId": "flata-exchange",                "vaspName": "flataexchange"            },            {                "vaspEntityId": "fobl",                "vaspName": "Foblgate"            },            {                "vaspEntityId": "hanbitco",                "vaspName": "hanbitco"            },            {                "vaspEntityId": "hexlant",                "vaspName": "hexlant"            },            {                "vaspEntityId": "inex",                "vaspName": "INEX"            },            {                "vaspEntityId": "infiniteblock-corp",                "vaspName": "InfiniteBlock Corp"            },            {                "vaspEntityId": "kdac",                "vaspName": "kdac"            },            {                "vaspEntityId": "korbit",                "vaspName": "korbit"            },            {                "vaspEntityId": "paycoin",                "vaspName": "Paycoin"            },            {                "vaspEntityId": "qbit",                "vaspName": "Qbit"            },            {                "vaspEntityId": "tennten",                "vaspName": "TENNTEN"            },            {                "vaspEntityId": "others",                "vaspName": "Others (including Upbit)"            }        ]    },    "retExtInfo": {},    "time": 1715067106537}
```

---

## Get Sub Deposit Records (on-chain)

**URL:** https://bybit-exchange.github.io/docs/v5/asset/deposit/sub-deposit-record

**Contents:**

- Get Sub Deposit Records (on-chain)
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Deposit
Get Sub Deposit Records (on-chain)
On this page
Get Sub Deposit Records (on-chain)
Query subaccount's deposit records by
main
UID's API key.
tip
endTime

-

startTime
should be less than 30 days. Queries for the last 30 days worth of records by default.
HTTP Request
​
GET
/v5/asset/deposit/query-sub-member-record
Request Parameters
​
Parameter
Required
Type
Comments
id
false
string
Internal ID: Can be used to uniquely identify and filter the deposit. When combined with other parameters, this field
takes the highest priority
txID
false
string
Transaction ID: Please note that data generated before Jan 1, 2024 cannot be queried using txID
subMemberId
true
string
Sub UID
coin
false
string
Coin, uppercase only
startTime
false
integer
The start timestamp (ms)
Note: the query logic is actually effective based on
second
level
endTime
false
integer
The end timestamp (ms)
Note: the query logic is actually effective based on
second
level
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
rows
array
Object
> id
> string
> Unique ID
> coin
> string
> Coin
> chain
> string
> Chain
> amount
> string
> Amount
> txID
> string
> Transaction ID
>
status
integer
Deposit status
> toAddress
> string
> Deposit target address
> tag
> string
> Tag of deposit target address
> depositFee
> string
> Deposit fee
> successAt
> string
> Deposit's success time
> confirmations
> string
> Number of confirmation blocks
> txIndex
> string
> Transaction sequence number
> blockHash
> string
> Hash number on the chain
> batchReleaseLimit
> string
> The deposit limit for this coin in this chain.
"-1"
> means no limit
> depositType
> string
> The deposit type.
> 0
: normal deposit,
> 10
: the deposit reaches daily deposit limit,
> 20
: abnormal deposit
> fromAddress
> string
> From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives
""
> taxDepositRecordsId
> string
> This field is used for tax purposes by Bybit EU (Austria) users， declare tax id
> taxStatus
> integer
> This field is used for tax purposes by Bybit EU (Austria) users
> 0: No reporting required
> 1: Reporting pending
> 2: Reporting completed
> nextPageCursor
> string
> Refer to the
> cursor
> request parameter
> RUN >>
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/asset/deposit/query-sub-member-record?coin=USDT&limit=1&subMemberId=592334
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1672192441294
> X-BAPI-RECV-WINDOW
:
> 5000
> from
> pybit
> .
> unified_trading
> import
> HTTP
> session
> =
> HTTP
(
> testnet
> =
> True
> ,
> api_key
> =
"xxxxxxxxxxxxxxxxxx"
> ,
> api_secret
> =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> ,
)
> print
(
> session
> .
> get_sub_deposit_records
(
> coin
> =
"USDT"
> ,
> limit
> =
> 1
> ,
> subMemberId
> =
> 592334
> ,
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> getSubAccountDepositRecords
(
> {
> coin
:
'USDT'
> ,
> limit
:
> 1
> ,
> subMemberId
:
'592334'
> ,
> }
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"success"
> ,
"result"
:
> {
"rows"
:
[
]
> ,
"nextPageCursor"
:
""
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1672192441742
> }

**Examples:**

Example 1 ():

```
GET /v5/asset/deposit/query-sub-member-record?coin=USDT&limit=1&subMemberId=592334 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672192441294X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_sub_deposit_records(    coin="USDT",    limit=1,    subMemberId=592334,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSubAccountDepositRecords({    coin: 'USDT',    limit: 1,    subMemberId: '592334',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1672192441742}
```

---

## Delete Sub UID

**URL:** https://bybit-exchange.github.io/docs/v5/user/rm-subuid

**Contents:**

- Delete Sub UID
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Delete Sub UID
On this page
Delete Sub UID
Delete a sub UID. Before deleting the UID, please make sure there is no asset.
Use
master
user's api key**.
tip
The API key must have one of the below permissions in order to call this endpoint
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
HTTP Request
​
POST
/v5/user/del-submember
Request Parameters
​
Parameter
Required
Type
Comments
subMemberId
true
string
Sub UID
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/user/del-submember
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1698907012755
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
Content-Length
:
34
{
"subMemberId"
:
"112725187"
}
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
deleteSubMember
(
{
subMemberId
:
'subUID'
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
1698907012962
}

**Examples:**

Example 1 ():

```
POST /v5/user/del-submember HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1698907012755X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 34{    "subMemberId": "112725187"}
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .deleteSubMember({    subMemberId: 'subUID',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1698907012962}
```

---

## Set Auto Repay Mode

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/set-auto-repay-mode

**Contents:**

- Set Auto Repay Mode
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Spot Margin Trade (UTA)
Set Auto Repay Mode
On this page
Set Auto Repay Mode
Set spot automatic repayment mode
info
If
currency
is not passed, spot automatic repayment will be enabled for all currencies.
If
autoRepayMode
of a currency is set to 1, the system will automatically make repayments without asset conversion to that currency at 0
and 30 minutes every hour.
The amount of repayments without asset conversion is the minimum of available spot balance in that currency and
liability of that currency.
If you missed the automatic repayment batches for 0 and 30 minutes every hour, you can manually make the repayment via
the API. Please refer to
Manual Repay Without Asset Conversion
HTTP Request
​
POST
/v5/spot-margin-trade/set-auto-repay-mode
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
is not passed, spot automatic repayment will be enabled for all currencies.
autoRepayMode
true
string
1
: On
0
: Off
Response Parameters
​
Parameter
Type
Comments
data
array
Object
> currency
> string
> Coin name, uppercase only.
> autoRepayMode
> string
> 1
: On
> 0
: Off
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/spot-margin-trade/set-auto-repay-mode
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1672299806626
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"currency"
:
"ETH"
> ,
"autoRepayMode"
:
"1"
> }
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"success"
> ,
"result"
:
> {
"data"
:
[
{
"currency"
:
"ETH"
,
"autoRepayMode"
:
"1"
}
]
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1766976677678
> }

**Examples:**

Example 1 ():

```
POST /v5/spot-margin-trade/set-auto-repay-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672299806626X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "currency": "ETH",    "autoRepayMode":"1"}
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "data": [            {                "currency": "ETH",                "autoRepayMode": "1"            }        ]    },    "retExtInfo": {},    "time": 1766976677678}
```

---

## Modify Master API Key

**URL:** https://bybit-exchange.github.io/docs/v5/user/modify-master-apikey

**Contents:**

- Modify Master API Key
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Modify Master API Key
On this page
Modify Master API Key
Modify the settings of master api key. Use the api key pending to be modified to call the endpoint. Use
master user's api key
only
.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
info
Only the api key that calls this interface can be modified
HTTP Request
​
POST
/v5/user/update-api
Request Parameters
​
Parameter
Required
Type
Comments
readOnly
false
integer
0
(default): Read and Write.
1
: Read only
ips
false
string
Set the IP bind. example:
"192.168.0.1,192.168.0.2"
note:
don't pass ips or pass with
"*"
means no bind
No ip bound api key will be
invalid after 90 days
api key will be invalid after
7 days
once the account password is changed
permissions
false
Object
Tick the types of permission. Don't send this param if you don't want to change the permission
> ContractTrade
> false
> array
> Contract Trade.
["Order","Position"]
> Spot
> false
> array
> Spot Trade.
["SpotTrade"]
> Wallet
> false
> array
> Wallet.
["AccountTransfer","SubMemberTransfer"]
> Options
> false
> array
> USDC Contract.
["OptionsTrade"]
> Exchange
> false
> array
> Convert.
["ExchangeHistory"]
> Earn
> false
> array
> Earn product.
["Earn"]
> FiatP2P
> false
> array
> P2P
> FiatP2POrder
> ,
> Advertising
> FiatBybitPay
> false
> array
> Bybit Pay
> FaitPayOrder
> FiatConvertBroker
> false
> array
> Fiat convert
> FiatConvertBrokerOrder
> Affiliate
> false
> array
> Affiliate.
["Affiliate"]
> This permission is only useful for affiliate
> If you need this permission, make sure you remove all other permissions
> Derivatives
> false
> array
["DerivativesTrade"]
> BlockTrade
> false
> array
> Blocktrade.
["BlockTrade"]
> Response Parameters
> ​
> Parameter
> Type
> Comments
> id
> string
> Unique id. Internal used
> note
> string
> The remark
> apiKey
> string
> Api key
> readOnly
> integer
> 0
: Read and Write.
> 1
: Read only
> secret
> string
> Always
""
> permissions
> Object
> The types of permission
> ContractTrade
> array
> Permisson of contract trade
> Spot
> array
> Permisson of spot
> Wallet
> array
> Permisson of wallet
> Options
> array
> Permission of USDC Contract. It supports trade option and usdc perpetual.
> Derivatives
> array
> Permission of Unified account
> BlockTrade
> array
> Permission of blocktrade
> Exchange
> array
> Permission of convert
> Earn
> array
> Permission of earn
> Affiliate
> array
> Affiliate permission
> FiatP2P
> array
> Permission of P2P
> FiatBybitPay
> array
> Permission of Bybit pay
> FiatConvertBroker
> array
> Permission of fiat convert
> NFT
> array
> Deprecated
> , always
[]
> CopyTrading
> array
> Deprecated
> , always
[]
> ips
> array
> IP bound
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/user/update-api
> HTTP/1.1
> Host
:
> api.bybit.com
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1676431264739
> X-BAPI-RECV-WINDOW
:
> 5000
> X-BAPI-SIGN
:
> XXXXXX
> Content-Type
:
> application/json
> {
"readOnly"
:
> null
> ,
"ips"
:
"*"
,
"permissions"
:
{
"ContractTrade"
:
[
"Order"
,
"Position"
]
,
"Spot"
:
[
"SpotTrade"
]
,
"Wallet"
:
[
"AccountTransfer"
,
"SubMemberTransfer"
]
,
"Options"
:
[
"OptionsTrade"
]
,
"CopyTrading"
:
[
"CopyTrading"
]
,
"BlockTrade"
:
[
]
,
"Exchange"
:
[
"ExchangeHistory"
]
,
"NFT"
:
[
"NFTQueryProductList"
]
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
modify_master_api_key
(
ips
=
"*"
> ,
> permissions
> =
> {
"ContractTrade"
:
[
"Order"
,
"Position"
]
> ,
"Spot"
:
[
"SpotTrade"
]
> ,
"Wallet"
:
[
"AccountTransfer"
,
"SubMemberTransfer"
]
> ,
"Options"
:
[
"OptionsTrade"
]
> ,
"Derivatives"
:
[
"DerivativesTrade"
]
> ,
"CopyTrading"
:
[
"CopyTrading"
]
> ,
"BlockTrade"
:
[
]
> ,
"Exchange"
:
[
"ExchangeHistory"
]
> ,
"NFT"
:
[
"NFTQueryProductList"
]
> }
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> updateMasterApiKey
(
> {
> ips
:
[
'*'
]
> ,
> permissions
:
> {
> ContractTrade
:
[
'Order'
,
'Position'
]
> ,
> Spot
:
[
'SpotTrade'
]
> ,
> Wallet
:
[
'AccountTransfer'
,
'SubMemberTransfer'
]
> ,
> Options
:
[
'OptionsTrade'
]
> ,
> Derivatives
:
[
'DerivativesTrade'
]
> ,
> CopyTrading
:
[
'CopyTrading'
]
> ,
> BlockTrade
:
[
]
> ,
> Exchange
:
[
'ExchangeHistory'
]
> ,
> NFT
:
[
'NFTQueryProductList'
]
> ,
> }
> ,
> }
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
""
> ,
"result"
:
> {
"id"
:
"13770661"
> ,
"note"
:
"xxxxx"
> ,
"apiKey"
:
"xxxxx"
> ,
"readOnly"
:
> 0
> ,
"secret"
:
""
> ,
"permissions"
:
> {
"ContractTrade"
:
[
"Order"
,
"Position"
]
> ,
"Spot"
:
[
"SpotTrade"
]
> ,
"Wallet"
:
[
"AccountTransfer"
,
"SubMemberTransfer"
]
> ,
"Options"
:
[
"OptionsTrade"
]
> ,
"Derivatives"
:
[
"DerivativesTrade"
]
> ,
"CopyTrading"
:
[
"CopyTrading"
]
> ,
"BlockTrade"
:
[
]
> ,
"Exchange"
:
[
"ExchangeHistory"
]
> ,
"Earn"
:
[
]
> ,
"NFT"
:
[
"NFTQueryProductList"
]
> }
> ,
"ips"
:
[
"*"
]
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1676431265427
> }

**Examples:**

Example 1 ():

```
POST /v5/user/update-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676431264739X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json{    "readOnly": null,    "ips": "*",    "permissions": {            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "CopyTrading": [                "CopyTrading"            ],            "BlockTrade": [],            "Exchange": [                "ExchangeHistory"            ],            "NFT": [                "NFTQueryProductList"            ]        }}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.modify_master_api_key(    ips="*",    permissions={            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "Derivatives": [                "DerivativesTrade"            ],            "CopyTrading": [                "CopyTrading"            ],            "BlockTrade": [],            "Exchange": [                "ExchangeHistory"            ],            "NFT": [                "NFTQueryProductList"            ]        }))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .updateMasterApiKey({    ips: ['*'],    permissions: {      ContractTrade: ['Order', 'Position'],      Spot: ['SpotTrade'],      Wallet: ['AccountTransfer', 'SubMemberTransfer'],      Options: ['OptionsTrade'],      Derivatives: ['DerivativesTrade'],      CopyTrading: ['CopyTrading'],      BlockTrade: [],      Exchange: ['ExchangeHistory'],      NFT: ['NFTQueryProductList'],    },  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "13770661",        "note": "xxxxx",        "apiKey": "xxxxx",        "readOnly": 0,        "secret": "",        "permissions": {            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "Derivatives": [                "DerivativesTrade"            ],            "CopyTrading": [                "CopyTrading"            ],            "BlockTrade": [],            "Exchange": [                "ExchangeHistory"            ],            "Earn": [],            "NFT": [                "NFTQueryProductList"            ]        },        "ips": [            "*"        ]    },    "retExtInfo": {},    "time": 1676431265427}
```

---

## Withdraw

**URL:** https://bybit-exchange.github.io/docs/v5/asset/withdraw

**Contents:**

- Withdraw
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Withdraw
Withdraw
On this page
Withdraw
Withdraw assets from your Bybit account. You can make an off-chain transfer if the target wallet address is from Bybit.
This means that no blockchain fee will be charged.
Note that, although the API rate limit for this endpoint is 5 req/s, there is also a secondary limit: you can only
withdraw once every 10 seconds per chain/coin combination.
tip
Make sure you have whitelisted your wallet address
here
Request by the master UID's api key
only
formula
feeType=0:
withdrawPercentageFee != 0:
handlingFee = inputAmount / (1 - withdrawPercentageFee) * withdrawPercentageFee + withdrawFee
withdrawPercentageFee = 0:
handlingFee = withdrawFee
feeType=1:
withdrawPercentageFee != 0:
handlingFee = withdrawFee + (inputAmount - withdrawFee) * withdrawPercentageFee
withdrawPercentageFee = 0:
handlingFee = withdrawFee
HTTP Request
​
POST
/v5/asset/withdraw/create
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
Coin, uppercase only
chain
false
string
Chain
forceChain
=0 or 1: this field is
required
forceChain
=2: this field can be null
address
true
string
forceChain
=0 or 1: fill wallet address, and make sure you add address in the
address book
first. Please note that the address is case sensitive, so use the exact same address added in address book
forceChain
=2: fill Bybit UID, and it can only be another Bybit
main
account UID. Make sure you add UID in the
address book
first
tag
false
string
Tag
Required
if tag exists in the wallet address list.
Note
: please do not set a tag/memo in the address book if the chain does not support tag
amount
true
string
Withdraw amount
timestamp
true
integer
Current timestamp (ms). Used for preventing from withdraw replay
forceChain
false
integer
Whether or not to force an on-chain withdrawal
0
(default): If the address is parsed out to be an internal address, then internal transfer (
Bybit main account only
)
1
: Force the withdrawal to occur on-chain
2
: Use UID to withdraw
accountType
true
string
Select the wallet to be withdrawn from
FUND
: Funding wallet
UTA
: System transfers the funds to Funding wallet to withdraw
FUND,UTA
: For combo withdrawals, funds will be deducted from the Funding wallet first. If the balance is insufficient, the
remaining amount will be deducted from the UTA wallet.
feeType
false
integer
Handling fee option
0
(default): input amount is the actual amount received, so you have to calculate handling fee manually
1
: input amount is not the actual amount you received, the system will help to deduct the handling fee automatically
requestId
false
string
Customised ID, globally unique, it is used for idempotent verification
A combination of letters (case sensitive) and numbers, which can be pure letters or pure numbers and the length must be
between 1 and 32 digits
beneficiary
false
Object
Travel rule info. It is
required
for kyc/kyb=KOR (Korean), kyc=IND (India) users, and users who registered in
Bybit Turkey(TR)
,
Bybit Kazakhstan(KZ)
, Bybit Indonesia (ID)
> beneficiaryTransactionPurpose
> false
> string
> Purpose of the withdrawal transaction,
> Required
> when KR users withdraw funds to a company via Korean CODE channel
> beneficiaryRepresentativeFirstName
> false
> string
> First name of the beneficiary company's representative,
> Required
> when KR users withdraw funds to a company via Korean CODE channel
> beneficiaryRepresentativeLastName
> false
> string
> Last name of the beneficiary company's representative,
> Required
> when KR users withdraw funds to a company via Korean CODE channel
> vaspEntityId
> false
> string
> Receiver exchange entity Id. Please call this
> endpoint
> to get this ID.
> Required
> param for Korean users
> Ignored by
> TR, KZ users
> vaspEntityId
> false
> string
> Receiver exchange entity Id. Please call this
> endpoint
> to get this ID.
> Required
> param for Korean users
> Ignored by
> TR, KZ users
> beneficiaryName
> false
> string
> Receiver exchange user KYC name
> Rules for Korean users
:
> Please refer to target exchange kyc name
> When vaspEntityId="others", this field can be null
> Rules for TR, KZ, kyc=IND users
: it is a
> required
> param, fill with individual name or company name
> beneficiaryLegalType
> false
> string
> Beneficiary legal type,
> individual
(default),
> company
> Required
> param for TR, KZ, kyc=IND users
> Korean users can ignore
> beneficiaryWalletType
> false
> string
> Beneficiary wallet type,
> 0
: custodial/exchange wallet (default),
> 1
: non custodial/exchane wallet
> Required
> param for TR, KZ, kyc=IND users
> Korean users can ignore
> beneficiaryUnhostedWalletType
> false
> string
> Beneficiary unhosted wallet type,
> 0
: Your own wallet,
> 1
: others' wallet
> Required
> param for TR, KZ, kyc=IND users when "beneficiaryWalletType=1"
> Korean users can ignore
> beneficiaryPoiNumber
> false
> string
> Beneficiary ducument number
> Required
> param for TR, KZ users
> Korean users can ignore
> beneficiaryPoiType
> false
> string
> Beneficiary ducument type
> Required
> param for TR, KZ users: ID card, Passport, driver license, residence permit, Business ID, etc
> Korean users can ignore
> beneficiaryPoiIssuingCountry
> false
> string
> Beneficiary ducument issuing country
> Required
> param for TR, KZ users: refer to
> Alpha-3 country code
> Korean users can ignore
> beneficiaryPoiExpiredDate
> false
> string
> Beneficiary ducument expiry date
> Required
> param for TR, KZ users: yyyy-mm-dd format, e.g., "1990-02-15"
> Korean users can ignore
> beneficiaryAddressCountry
> false
> string
> Beneficiary country
> Required
> param for UAE users only, e.g.,
> IDN
> beneficiaryAddressState
> false
> string
> Beneficiary state
> Required
> param for UAE users only, e.g., "ABC"
> beneficiaryAddressCity
> false
> string
> Beneficiary city
> Required
> param for UAE users only, e.g., "Jakarta"
> beneficiaryAddressBuilding
> false
> string
> Beneficiary building address
> Required
> param for UAE users only
> beneficiaryAddressStreet
> false
> string
> Beneficiary street address
> Required
> param for UAE users only
> beneficiaryAddressPostalCode
> false
> string
> Beneficiary address post code
> Required
> param for UAE users only
> beneficiaryDateOfBirth
> false
> string
> Beneficiary date of birth
> Required
> param for UAE users only
> beneficiaryPlaceOfBirth
> false
> string
> Beneficiary birth place
> Required
> param for UAE users onl
> Response Parameters
> ​
> Parameter
> Type
> Comments
> id
> string
> Withdrawal ID
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/asset/withdraw/create
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1672196570254
> X-BAPI-RECV-WINDOW
:
> 5000
> X-BAPI-SIGN
:
> XXXXX
> Content-Type
:
> application/json
> {
"coin"
:
"USDT"
> ,
"chain"
:
"ETH"
> ,
"address"
:
"0x99ced129603abc771c0dabe935c326ff6c86645d"
> ,
"amount"
:
"24"
> ,
"timestamp"
:
> 1672196561407
> ,
"forceChain"
:
> 0
> ,
"accountType"
:
"FUND"
> }
> from
> pybit
> .
> unified_trading
> import
> HTTP
> session
> =
> HTTP
(
> testnet
> =
> True
> ,
> api_key
> =
"xxxxxxxxxxxxxxxxxx"
> ,
> api_secret
> =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> ,
)
> print
(
> session
> .
> withdraw
(
> coin
> =
"USDT"
> ,
> chain
> =
"ETH"
> ,
> address
> =
"0x99ced129603abc771c0dabe935c326ff6c86645d"
> ,
> amount
> =
"24"
> ,
> timestamp
> =
> 1672196561407
> ,
> forceChain
> =
> 0
> ,
> accountType
> =
"FUND"
> ,
)
)
> const
> {
> RestClientV5
> }
> =
> require
(
'bybit-api'
)
> ;
> const
> client
> =
> new
> RestClientV5
(
> {
> testnet
:
> true
> ,
> key
:
'xxxxxxxxxxxxxxxxxx'
> ,
> secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> ,
> }
)
> ;
> client
> .
> submitWithdrawal
(
> {
> coin
:
'USDT'
> ,
> chain
:
'ETH'
> ,
> address
:
'0x99ced129603abc771c0dabe935c326ff6c86645d'
> ,
> amount
:
'24'
> ,
> timestamp
:
> 1672196561407
> ,
> forceChain
:
> 0
> ,
> accountType
:
'FUND'
> ,
> }
)
> .
> then
(
(
> response
)
> =>
> {
> console
> .
> log
(
> response
)
> ;
> }
)
> .
> catch
(
(
> error
)
> =>
> {
> console
> .
> error
(
> error
)
> ;
> }
)
> ;
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"success"
> ,
"result"
:
> {
"id"
:
"10195"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1672196571239
> }

**Examples:**

Example 1 ():

```
POST /v5/asset/withdraw/create HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672196570254X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "coin": "USDT",    "chain": "ETH",    "address": "0x99ced129603abc771c0dabe935c326ff6c86645d",    "amount": "24",    "timestamp": 1672196561407,    "forceChain": 0,    "accountType": "FUND"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.withdraw(    coin="USDT",    chain="ETH",    address="0x99ced129603abc771c0dabe935c326ff6c86645d",    amount="24",    timestamp=1672196561407,    forceChain=0,    accountType="FUND",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .submitWithdrawal({    coin: 'USDT',    chain: 'ETH',    address: '0x99ced129603abc771c0dabe935c326ff6c86645d',    amount: '24',    timestamp: 1672196561407,    forceChain: 0,    accountType: 'FUND',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "id": "10195"    },    "retExtInfo": {},    "time": 1672196571239}
```

---

## Get Pre-upgrade USDC Session Settlement

**URL:** https://bybit-exchange.github.io/docs/v5/pre-upgrade/settlement

**Contents:**

- Get Pre-upgrade USDC Session Settlement
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Pre-upgrade
Get Pre-upgrade USDC Session Settlement
On this page
Get Pre-upgrade USDC Session Settlement
Query session settlement records of USDC perpetual before you upgrade the account to Unified account.
info
By category="option", you can query USDC Perps settlement data occurred during classic account
USDC Perpeual support the recent 6 months data. Please download older data via GUI
HTTP Request
​
GET
/v5/pre-upgrade/asset/settlement-record
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
symbol
false
string
Symbol name, like
BTCUSDT
, uppercase only
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
Cursor. Used for pagination
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
> string
> Symbol name
> side
> string
> Buy
> ,
> Sell
> size
> string
> Position size
> sessionAvgPrice
> string
> Settlement price
> markPrice
> string
> Mark price
> realisedPnl
> string
> Realised PnL
> createdTime
> string
> Created time (ms)
> nextPageCursor
> string
> Cursor. Used for pagination
> Request Example
> ​
> HTTP
> Python
> GET
> /v5/pre-upgrade/asset/settlement-record?category=linear&symbol=ETHPERP&limit=1
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1686809850982
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"OK"
> ,
"result"
:
> {
"nextPageCursor"
:
"25%3A0%2C25%3A0"
> ,
"category"
:
"linear"
> ,
"list"
:
[
{
"realisedPnl"
:
"45.76"
,
"symbol"
:
"ETHPERP"
,
"side"
:
"Sell"
,
"markPrice"
:
"1668.44"
,
"size"
:
"-0.5"
,
"createdTime"
:
"1686787200000"
,
"sessionAvgPrice"
:
"1668.41"
}
]
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1686809851749
> }

**Examples:**

Example 1 ():

```
GET /v5/pre-upgrade/asset/settlement-record?category=linear&symbol=ETHPERP&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1686809850982X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "25%3A0%2C25%3A0",        "category": "linear",        "list": [            {                "realisedPnl": "45.76",                "symbol": "ETHPERP",                "side": "Sell",                "markPrice": "1668.44",                "size": "-0.5",                "createdTime": "1686787200000",                "sessionAvgPrice": "1668.41"            }        ]    },    "retExtInfo": {},    "time": 1686809851749}
```

---

## Create Universal Transfer

**URL:** https://bybit-exchange.github.io/docs/v5/asset/transfer/unitransfer

**Contents:**

- Create Universal Transfer
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Transfer
Create Universal Transfer
On this page
Create Universal Transfer
Transfer between sub-sub or main-sub.
tip
Use master or sub acct api key to request
To use sub acct api key, it must have "SubMemberTransferList" permission
When use sub acct api key, it can only transfer to main account
If you encounter errorCode:
131228
and msg:
your balance is not enough
, please go to
Get Single Coin Balance
to check transfer safe amount.
You can not transfer between the same UID.
HTTP Request
​
POST
/v5/asset/transfer/universal-transfer
Request Parameters
​
Parameter
Required
Type
Comments
transferId
true
string
UUID
. Please manually generate a UUID
coin
true
string
Coin, uppercase only
amount
true
string
Amount
fromMemberId
true
integer
From UID
toMemberId
true
integer
To UID
fromAccountType
true
string
From account type
toAccountType
true
string
To account type
Response Parameters
​
Parameter
Type
Comments
transferId
string
UUID
status
string
Transfer status
STATUS_UNKNOWN
SUCCESS
PENDING
FAILED
RUN >>
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/transfer/universal-transfer
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672189449697
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXX
Content-Type
:
application/json
{
"transferId"
:
"be7a2462-1138-4e27-80b1-62653f24925e"
,
"coin"
:
"ETH"
,
"amount"
:
"0.5"
,
"fromMemberId"
:
592334
,
"toMemberId"
:
691355
,
"fromAccountType"
:
"CONTRACT"
,
"toAccountType"
:
"UNIFIED"
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
create_universal_transfer
(
transferId
=
"be7a2462-1138-4e27-80b1-62653f24925e"
,
coin
=
"ETH"
,
amount
=
"0.5"
,
fromMemberId
=
592334
,
toMemberId
=
691355
,
fromAccountType
=
"CONTRACT"
,
toAccountType
=
"UNIFIED"
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
createUniversalTransfer
(
{
transferId
:
'be7a2462-1138-4e27-80b1-62653f24925e'
,
coin
:
'ETH'
,
amount
:
'0.5'
,
fromMemberId
:
592334
,
toMemberId
:
691355
,
fromAccountType
:
'CONTRACT'
,
toAccountType
:
'UNIFIED'
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
"transferId"
:
"be7a2462-1138-4e27-80b1-62653f24925e"
,
"status"
:
"SUCCESS"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672189450195
}

**Examples:**

Example 1 ():

```
POST /v5/asset/transfer/universal-transfer HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672189449697X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "transferId": "be7a2462-1138-4e27-80b1-62653f24925e",    "coin": "ETH",    "amount": "0.5",    "fromMemberId": 592334,    "toMemberId": 691355,    "fromAccountType": "CONTRACT",    "toAccountType": "UNIFIED"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.create_universal_transfer(    transferId="be7a2462-1138-4e27-80b1-62653f24925e",    coin="ETH",    amount="0.5",    fromMemberId=592334,    toMemberId=691355,    fromAccountType="CONTRACT",    toAccountType="UNIFIED",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .createUniversalTransfer({    transferId: 'be7a2462-1138-4e27-80b1-62653f24925e',    coin: 'ETH',    amount: '0.5',    fromMemberId: 592334,    toMemberId: 691355,    fromAccountType: 'CONTRACT',    toAccountType: 'UNIFIED',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "transferId": "be7a2462-1138-4e27-80b1-62653f24925e",        "status": "SUCCESS"    },    "retExtInfo": {},    "time": 1672189450195}
```

---

## Create Sub UID

**URL:** https://bybit-exchange.github.io/docs/v5/user/create-subuid

**Contents:**

- Create Sub UID
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Create Sub UID
On this page
Create Sub UID
Create a new sub user id. Use
master
account's api key.
tip
The API key must have one of the below permissions in order to call this endpoint
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
info
Custody account, like copper, fireblock are not supported to create subaccount via this API
HTTP Request
​
POST
/v5/user/create-sub-member
Request Parameters
​
Parameter
Required
Type
Comments
username
true
string
Username of the new sub user.
6-16 characters, must include both numbers and letters.
Cannot be the same as the existing or deleted usernames.
password
false
string
Password for the new sub user.
8-30 characters, must include numbers, upper and lowercase letters.
memberType
true
integer
1
: normal subaccount,
6
:
custodial subaccount
switch
false
integer
0
: turn off quick login (default)
1
: turn on quick login.
isUta
false
boolean
Deprecated
param, always UTA account
note
false
string
Set a remark
Response Parameters
​
Parameter
Type
Comments
uid
string
Sub user Id
username
string
Username of the new sub user.
6-16 characters, must include both numbers and letters.
Cannot be the same as the existing or deleted usernames.
memberType
integer
1
: normal subaccount,
6
:
custodial subaccount
status
integer
The status of the user account
1
: normal
2
: login banned
4
: frozen
remark
string
The remark
Request Example
​
HTTP
Python
Node.js
POST
/v5/user/create-sub-member
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-SIGN
:
XXXXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676429344202
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"username"
:
"xxxxx"
,
"memberType"
:
1
,
"switch"
:
1
,
"note"
:
"test"
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
create_sub_uid
(
username
=
"xxxxx"
,
memberType
=
1
,
switch
=
1
,
note
=
"test"
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
createSubMember
(
{
username
:
'xxxxx'
,
memberType
:
1
,
switch
:
1
,
note
:
'test'
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
"uid"
:
"53888000"
,
"username"
:
"xxxxx"
,
"memberType"
:
1
,
"status"
:
1
,
"remark"
:
"test"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1676429344734
}

**Examples:**

Example 1 ():

```
POST /v5/user/create-sub-member HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676429344202X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "username": "xxxxx",    "memberType": 1,    "switch": 1,    "note": "test"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.create_sub_uid(    username="xxxxx",    memberType=1,    switch=1,    note="test",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .createSubMember({    username: 'xxxxx',    memberType: 1,    switch: 1,    note: 'test',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "uid": "53888000",        "username": "xxxxx",        "memberType": 1,        "status": 1,        "remark": "test"    },    "retExtInfo": {},    "time": 1676429344734}
```

---

## Get Margin Coin Info

**URL:** https://bybit-exchange.github.io/docs/v5/otc/margin-coin-convert-info

**Contents:**

- Get Margin Coin Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Institutional Loan
Get Margin Coin Info
On this page
Get Margin Coin Info
tip
When queried without an API key, this endpoint returns public margin data
If your UID is bound with an OTC loan, then you can get your private margin data by calling with your API key
If your UID is not bound with an OTC loan but you passed your API key, this endpoint returns public margin data
HTTP Request
​
GET
/v5/ins-loan/ensure-tokens-convert
Request Parameters
​
Parameter
Required
Type
Comments
productId
false
string
Product ID. If not passed, returns all margin products. For spot, it returns coins with a
convertRatio
greater than 0.
Response Parameters
​
Parameter
Type
Comments
marginToken
array
Object
> productId
> string
> Product Id
> tokenInfo
> array
> Spot margin coin
>> token
> > string
> > Margin coin
> > convertRatioList
> > array
> > Margin coin convert ratio List
>>> ladder
> > > string
> > > ladder
> > > convertRatio
> > > string
> > > Margin coin convert ratio
> > > Request Example
> > > ​
> > > HTTP
> > > Python
> > > Node.js
> > > GET
> > > /v5/ins-loan/ensure-tokens-convert
> > > HTTP/1.1
> > > Host
:
> > > api-testnet.bybit.com
> > > from
> > > pybit
> > > .
> > > unified_trading
> > > import
> > > HTTP
> > > session
> > > =
> > > HTTP
(
> > > testnet
> > > =
> > > True
> > > ,
> > > api_key
> > > =
"xxxxxxxxxxxxxxxxxx"
> > > ,
> > > api_secret
> > > =
"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
> > > ,
)
> > > print
(
> > > session
> > > .
> > > get_margin_coin_info
(
)
)
> > > const
> > > {
> > > RestClientV5
> > > }
> > > =
> > > require
(
'bybit-api'
)
> > > ;
> > > const
> > > client
> > > =
> > > new
> > > RestClientV5
(
> > > {
> > > testnet
:
> > > true
> > > ,
> > > key
:
'xxxxxxxxxxxxxxxxxx'
> > > ,
> > > secret
:
'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
> > > ,
> > > }
)
> > > ;
> > > client
> > > .
> > > getInstitutionalLendingMarginCoinInfoWithConversionRate
(
> > > {
> > > productId
:
'81'
> > > ,
> > > }
)
> > > .
> > > then
(
(
> > > response
)
> > > =>
> > > {
> > > console
> > > .
> > > log
(
> > > response
)
> > > ;
> > > }
)
> > > .
> > > catch
(
(
> > > error
)
> > > =>
> > > {
> > > console
> > > .
> > > error
(
> > > error
)
> > > ;
> > > }
)
> > ;
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
"marginToken"
:
[
{
"productId"
:
"81"
,
"tokenInfo"
:
[
{
"token"
:
"USDT"
,
"convertRatioList"
:
[
{
"ladder"
:
"0-500"
,
"convertRatio"
:
"0.95"
}
,
{
"ladder"
:
"500-1000"
,
"convertRatio"
:
"0.9"
}
,
{
"ladder"
:
"1000-2000"
,
"convertRatio"
:
"0.8"
}
,
{
"ladder"
:
"2000-4000"
,
"convertRatio"
:
"0.7"
}
,
{
"ladder"
:
"4000-99999999999"
,
"convertRatio"
:
"0.6"
}
]
}
...
]
}
,
{
"productId"
:
"82"
,
"tokenInfo"
:
[
...
{
"token"
:
"USDT"
,
"convertRatioList"
:
[
{
"ladder"
:
"0-1000"
,
"convertRatio"
:
"0.7"
}
,
{
"ladder"
:
"1000-2000"
,
"convertRatio"
:
"0.65"
}
,
{
"ladder"
:
"2000-99999999999"
,
"convertRatio"
:
"0.6"
}
]
}
]
}
,
{
"productId"
:
"84"
,
"tokenInfo"
:
[
...
{
"token"
:
"BTC"
,
"convertRatioList"
:
[
{
"ladder"
:
"0-1000"
,
"convertRatio"
:
"1"
}
,
{
"ladder"
:
"1000-5000"
,
"convertRatio"
:
"0.9"
}
,
{
"ladder"
:
"5000-99999999999"
,
"convertRatio"
:
"0.55"
}
]
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
1683276016497
}

**Examples:**

Example 1 ():

```
GET /v5/ins-loan/ensure-tokens-convert HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_margin_coin_info())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInstitutionalLendingMarginCoinInfoWithConversionRate({    productId: '81',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "marginToken": [            {                "productId": "81",                "tokenInfo": [                    {                        "token": "USDT",                        "convertRatioList": [                            {                                "ladder": "0-500",                                "convertRatio": "0.95"                            },                            {                                "ladder": "500-1000",                                "convertRatio": "0.9"                            },                            {                                "ladder": "1000-2000",                                "convertRatio": "0.8"                            },                            {                                "ladder": "2000-4000",                                "convertRatio": "0.7"                            },                            {                                "ladder": "4000-99999999999",                                "convertRatio": "0.6"                            }                        ]                    }                  ...                ]            },            {                "productId": "82",                "tokenInfo": [                    ...                    {                        "token": "USDT",                        "convertRatioList": [                            {                                "ladder": "0-1000",                                "convertRatio": "0.7"                            },                            {                                "ladder": "1000-2000",                                "convertRatio": "0.65"                            },                            {                                "ladder": "2000-99999999999",                                "convertRatio": "0.6"                            }                        ]                    }                ]            },            {                "productId": "84",                "tokenInfo": [                    ...                    {                        "token": "BTC",                        "convertRatioList": [                            {                                "ladder": "0-1000",                                "convertRatio": "1"                            },                            {                                "ladder": "1000-5000",                                "convertRatio": "0.9"                            },                            {                                "ladder": "5000-99999999999",                                "convertRatio": "0.55"                            }                        ]                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1683276016497}
```

---

## Get Small Balance Coins

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert-small-balance/small-balanc-coins

**Contents:**

- Get Small Balance Coins
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert Small Balances
Get Small Balance Coins
On this page
Get Small Balance Coins
Query small-balance coins with a USDT equivalent of less than 10 USDT, and ensure that the total amount for each
conversion transaction is between 1.0e-8 and 200 USDT.
info
API key permission:
Convert
API rate limit:
10 req /s
HTTP Request
​
GET
/v5/asset/covert/small-balance-list
Request Parameters
​
Parameter
Required
Type
Comments
accountType
true
string
Wallet type
eb_convert_uta
. Only supports the Unified wallet
fromCoin
false
string
Source currency
Response Parameters
​
Parameter
Type
Comments
smallAssetCoins
array
<
object
>
Small balance info
> fromCoin
string
Source currency
> supportConvert
integer
1
: support,
2
: not supported
> availableBalance
string
Available balance, the value might be bigger than the actual balance you can convert
> baseValue
string
USDT equivalent value
> toAmount
string
Ignore
, reserved field
> exchangeRate
string
Ignore
, reserved field
> feeInfo
null
Ignore
, reserved field
> taxFeeInfo
null
Ignore
, reserved field
supportToCoins
array
["MNT","USDT","USDC"]
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/covert/small-balance-list?fromCoin=XRP&accountType=eb_convert_uta
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
1766125546001
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
get_small_balance_coins
(
fromCoin
=
"XRP"
,
accountType
=
"eb_convert_uta"
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
"smallAssetCoins"
:
[
{
"fromCoin"
:
"XRP"
,
"supportConvert"
:
1
,
"availableBalance"
:
"0.0002"
,
"baseValue"
:
"0.00036554008"
,
"toCoin"
:
""
,
"toAmount"
:
""
,
"exchangeRate"
:
""
,
"feeInfo"
:
null
,
"taxFeeInfo"
:
null
}
]
,
"supportToCoins"
:
[
"MNT"
,
"USDT"
,
"USDC"
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
1766125546274
}

**Examples:**

Example 1 ():

```
GET /v5/asset/covert/small-balance-list?fromCoin=XRP&accountType=eb_convert_uta HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1766125546001X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_small_balance_coins(    fromCoin="XRP",    accountType="eb_convert_uta",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "smallAssetCoins": [            {                "fromCoin": "XRP",                "supportConvert": 1,                "availableBalance": "0.0002",                "baseValue": "0.00036554008",                "toCoin": "",                "toAmount": "",                "exchangeRate": "",                "feeInfo": null,                "taxFeeInfo": null            }        ],        "supportToCoins": [            "MNT",            "USDT",            "USDC"        ]    },    "retExtInfo": {},    "time": 1766125546274}
```

---

## 查詢支持的充值幣種信息

**URL:** https://bybit-exchange.github.io/docs/v5/backup/deposit-coin-spec

**Contents:**

- 查詢支持的充值幣種信息
    - HTTP 請求​
    - 請求參數​
    - 響應參數​
    - 請求示例​
    - 響應示例​

On this page
查詢支持的充值幣種信息
通過幣種信息接口，獲取幣鏈組合
tip
該接口不需要做鑒權
HTTP 請求
​
GET
/v5/asset/deposit/query-allowed-list
請求參數
​
參數
是否必需
類型
說明
coin
false
string
充值幣種.
coin
和
chain
必須配對傳遞，否則是無效查詢
chain
false
string
充值鏈名.
coin
和
chain
必須配對傳遞，否則是無效查詢
limit
false
integer
每頁數量限制.
[
1
,
35
]
. 默認:
10
cursor
false
string
游標，用於翻頁
響應參數
​
參數
類型
說明
configList
array
Object
> coin
string
幣種
> chain
string
鏈名
> coinShowName
string
幣種名稱
> chainType
string
鏈的類型
> blockConfirmNumber
integer
充值上賬確認數
> minDepositAmount
string
最低充值金額
nextPageCursor
string
游標，用於翻頁
請求示例
​
HTTP
Python
Node.js
GET
/v5/asset/deposit/query-allowed-list?coin=ETH&chain=ETH
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
1672191495968
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
get_allowed_deposit_coin_info
(
coin
=
"ETH"
,
chain
=
"ETH"
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
getAllowedDepositCoinInfo
(
{
coin
:
"ETH"
,
chain
:
"ETH"
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
響應示例
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
"configList"
:
[
{
"coin"
:
"ETH"
,
"chain"
:
"ETH"
,
"coinShowName"
:
"ETH"
,
"chainType"
:
"ETH"
,
"blockConfirmNumber"
:
10000
,
"minDepositAmount"
:
"0.01"
}
]
,
"nextPageCursor"
:
"eyJwYWdlIjoyLCJsaW1pdCI6MTB9"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672191496219
}

**Examples:**

Example 1 ():

```
GET /v5/asset/deposit/query-allowed-list?coin=ETH&chain=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672191495968X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_allowed_deposit_coin_info(    coin="ETH",    chain="ETH",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getAllowedDepositCoinInfo({    coin:"ETH",    chain:"ETH",  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "configList": [            {                "coin": "ETH",                "chain": "ETH",                "coinShowName": "ETH",                "chainType": "ETH",                "blockConfirmNumber": 10000,                "minDepositAmount": "0.01"            }        ],        "nextPageCursor": "eyJwYWdlIjoyLCJsaW1pdCI6MTB9"    },    "retExtInfo": {},    "time": 1672191496219}
```

---

## Get Internal Deposit Records (off-chain)

**URL:** https://bybit-exchange.github.io/docs/v5/asset/deposit/internal-deposit-record

**Contents:**

- Get Internal Deposit Records (off-chain)
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Deposit
Get Internal Deposit Records (off-chain)
On this page
Get Internal Deposit Records (off-chain)
Query deposit records within the Bybit platform. These transactions are not on the blockchain.
Rules
The maximum difference between the start time and the end time is 30 days
Support to get deposit records by Master or Sub Member Api Key
HTTP Request
​
GET
/v5/asset/deposit/query-internal-record
Request Parameters
​
Parameter
Required
Type
Comments
txID
false
string
Internal transfer transaction ID
startTime
false
integer
Start time (ms). Default value: 30 days before the current time
endTime
false
integer
End time (ms). Default value: current time
coin
false
string
Coin name: for example, BTC. Default value: all
cursor
false
string
Cursor, used for pagination
limit
false
integer
Number of items per page,
[
1
,
50
]
. Default value: 50
Response Parameters
​
Parameter
Type
Comments
rows
array
Object
> id
string
ID
> type
integer
1
: Internal deposit
> coin
string
Deposit coin
> amount
string
Deposit amount
> status
integer
1=Processing
2=Success
3=deposit failed
> address
string
Email address or phone number
> createdTime
string
Deposit created timestamp
> txID
string
Internal transfer transaction ID
> taxDepositRecordsId
string
This field is used for tax purposes by Bybit EU (Austria) users， declare tax id
> taxStatus
integer
This field is used for tax purposes by Bybit EU (Austria) users
0: No reporting required
1: Reporting pending
2: Reporting completed
nextPageCursor
string
cursor information: used for pagination. Default value:
""
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/deposit/query-internal-record
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
1682099024473
X-BAPI-RECV-WINDOW
:
50000
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
get_internal_deposit_records
(
startTime
=
1667260800000
,
endTime
=
1667347200000
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
getInternalDepositRecords
(
{
startTime
:
1667260800000
,
endTime
:
1667347200000
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
"rows"
:
[
{
"id"
:
"1103"
,
"amount"
:
"0.1"
,
"type"
:
1
,
"coin"
:
"ETH"
,
"address"
:
"xxxx***@gmail.com"
,
"status"
:
2
,
"createdTime"
:
"1705393280"
,
"txID"
:
"77c37e5c-d9fa-41e5-bd13-c9b59d95"
，
"taxDepositRecordsId"
:
"0"
,
"taxStatus"
:
0
,
}
]
,
"nextPageCursor"
:
"eyJtaW5JRCI6MTEwMywibWF4SUQiOjExMDN9"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1705395632689
}

**Examples:**

Example 1 ():

```
GET /v5/asset/deposit/query-internal-record HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682099024473X-BAPI-RECV-WINDOW: 50000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_internal_deposit_records(    startTime=1667260800000,    endTime=1667347200000,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInternalDepositRecords({    startTime: 1667260800000,    endTime: 1667347200000,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "id": "1103",                "amount": "0.1",                "type": 1,                "coin": "ETH",                "address": "xxxx***@gmail.com",                "status": 2,                "createdTime": "1705393280",                "txID": "77c37e5c-d9fa-41e5-bd13-c9b59d95"，                "taxDepositRecordsId": "0",                "taxStatus": 0,            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTEwMywibWF4SUQiOjExMDN9"    },    "retExtInfo": {},    "time": 1705395632689}
```

---

## Create Internal Transfer

**URL:** https://bybit-exchange.github.io/docs/v5/asset/transfer/create-inter-transfer

**Contents:**

- Create Internal Transfer
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Transfer
Create Internal Transfer
On this page
Create Internal Transfer
Create the internal transfer between different
account types
under the same UID.
HTTP Request
​
POST
/v5/asset/transfer/inter-transfer
Request Parameters
​
Parameter
Required
Type
Comments
transferId
true
string
UUID
. Please manually generate a UUID
coin
true
string
Coin, uppercase only
amount
true
string
Amount
fromAccountType
true
string
From account type
toAccountType
true
string
To account type
Response Parameters
​
Parameter
Type
Comments
transferId
string
UUID
status
string
Transfer status
STATUS_UNKNOWN
SUCCESS
PENDING
FAILED
RUN >>
Request Example
​
HTTP
Python
Node.js
POST v5/asset/transfer/inter-transfer HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1670986690556
X-BAPI-RECV-WINDOW
:
50000
X-BAPI-SIGN
:
XXXXX
Content-Type
:
application/json
{
"transferId"
:
"42c0cfb0-6bca-c242-bc76-4e6df6cbcb16"
,
"coin"
:
"BTC"
,
"amount"
:
"0.05"
,
"fromAccountType"
:
"UNIFIED"
,
"toAccountType"
:
"CONTRACT"
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
create_internal_transfer
(
transferId
=
"42c0cfb0-6bca-c242-bc76-4e6df6cbcb16"
,
coin
=
"BTC"
,
amount
=
"0.05"
,
fromAccountType
=
"UNIFIED"
,
toAccountType
=
"CONTRACT"
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
createInternalTransfer
(
'42c0cfb0-6bca-c242-bc76-4e6df6cbcb16'
,
'BTC'
,
'0.05'
,
'UNIFIED'
,
'CONTRACT'
,
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
"transferId"
:
"42c0cfb0-6bca-c242-bc76-4e6df6cbab16"
,
"status"
:
"SUCCESS"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1670986962783
}

**Examples:**

Example 1 ():

```
POST v5/asset/transfer/inter-transfer HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1670986690556X-BAPI-RECV-WINDOW: 50000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "transferId": "42c0cfb0-6bca-c242-bc76-4e6df6cbcb16",    "coin": "BTC",    "amount": "0.05",    "fromAccountType": "UNIFIED",    "toAccountType": "CONTRACT"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.create_internal_transfer(    transferId="42c0cfb0-6bca-c242-bc76-4e6df6cbcb16",    coin="BTC",    amount="0.05",    fromAccountType="UNIFIED",    toAccountType="CONTRACT",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .createInternalTransfer(    '42c0cfb0-6bca-c242-bc76-4e6df6cbcb16',    'BTC',    '0.05',    'UNIFIED',    'CONTRACT',  )  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "transferId": "42c0cfb0-6bca-c242-bc76-4e6df6cbab16",        "status": "SUCCESS"    },    "retExtInfo": {},    "time": 1670986962783}
```

---

## Get Convert Status

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert/get-convert-result

**Contents:**

- Get Convert Status
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert
Get Convert Status
On this page
Get Convert Status
You can query the exchange result by sending quoteTxId.
HTTP Request
​
GET
/v5/asset/exchange/convert-result-query
Request Parameters
​
Parameter
Required
Type
Comments
quoteTxId
true
string
Quote tx ID
accountType
true
string
Wallet type
Response Parameters
​
Parameter
Type
Comments
result
object
>
accountType
string
Wallet type
> exchangeTxId
string
Exchange tx ID, same as quote tx ID
> userId
string
User ID
> fromCoin
string
From coin
> fromCoinType
string
From coin type.
crypto
> toCoin
string
To coin
> toCoinType
string
To coin type.
crypto
> fromAmount
string
From coin amount (amount to sell)
> toAmount
string
To coin amount (amount to buy according to exchange rate)
> exchangeStatus
string
Exchange status
init
processing
success
failure
> extInfo
object
Reserved field, ignored for now
> convertRate
string
Exchange rate
> createdAt
string
Quote created time
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/exchange/convert-result-query?quoteTxId=10100108106409343501030232064&accountType=eb_convert_funding
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
1720073659847
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
get_convert_status
(
accountType
=
"eb_convert_funding"
,
quoteTxId
=
"10100108106409343501030232064"
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
getConvertStatus
(
{
quoteTxId
:
'quoteTransactionId'
,
accountType
:
'eb_convert_funding'
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
"ok"
,
"result"
:
{
"result"
:
{
"accountType"
:
"eb_convert_funding"
,
"exchangeTxId"
:
"10100108106409343501030232064"
,
"userId"
:
"XXXXX"
,
"fromCoin"
:
"ETH"
,
"fromCoinType"
:
"crypto"
,
"fromAmount"
:
"0.1"
,
"toCoin"
:
"BTC"
,
"toCoinType"
:
"crypto"
,
"toAmount"
:
"0.00534882723991"
,
"exchangeStatus"
:
"success"
,
"extInfo"
:
{
}
,
"convertRate"
:
"0.0534882723991"
,
"createdAt"
:
"1720071899995"
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
1720073660696
}

**Examples:**

Example 1 ():

```
GET /v5/asset/exchange/convert-result-query?quoteTxId=10100108106409343501030232064&accountType=eb_convert_funding HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720073659847X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_convert_status(    accountType="eb_convert_funding",    quoteTxId="10100108106409343501030232064",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getConvertStatus({    quoteTxId: 'quoteTransactionId',    accountType: 'eb_convert_funding',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "result": {            "accountType": "eb_convert_funding",            "exchangeTxId": "10100108106409343501030232064",            "userId": "XXXXX",            "fromCoin": "ETH",            "fromCoinType": "crypto",            "fromAmount": "0.1",            "toCoin": "BTC",            "toCoinType": "crypto",            "toAmount": "0.00534882723991",            "exchangeStatus": "success",            "extInfo": {},            "convertRate": "0.0534882723991",            "createdAt": "1720071899995"        }    },    "retExtInfo": {},    "time": 1720073660696}
```

---

## Get Asset Info

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/asset-info

**Contents:**

- Get Asset Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Get Asset Info (Spot)
On this page
Get Asset Info
Query Spot asset information
Apply to: classic account
HTTP Request
​
GET
/v5/asset/transfer/query-asset-info
Request Parameters
​
Parameter
Required
Type
Comments
accountType
true
string
Account type.
SPOT
coin
false
string
Coin name, uppercase only
Response Parameters
​
Parameter
Type
Comments
spot
Object
> status
string
account status.
ACCOUNT_STATUS_NORMAL
: normal,
ACCOUNT_STATUS_UNSPECIFIED
: banned
> assets
array
Object
>> coin
string
Coin
> > frozen
string
Freeze amount
> > free
string
Free balance
> > withdraw
string
Amount in withdrawing
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/transfer/query-asset-info?accountType=SPOT&coin=ETH
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
1672136538042
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
get_spot_asset_info
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
getAssetInfo
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
"spot"
:
{
"status"
:
"ACCOUNT_STATUS_NORMAL"
,
"assets"
:
[
{
"coin"
:
"ETH"
,
"frozen"
:
"0"
,
"free"
:
"11.53485"
,
"withdraw"
:
""
}
]
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
1672136539127
}

**Examples:**

Example 1 ():

```
GET /v5/asset/transfer/query-asset-info?accountType=SPOT&coin=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672136538042X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_spot_asset_info(    accountType="FUND",    coin="USDC",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getAssetInfo({ accountType: 'FUND', coin: 'USDC' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "spot": {            "status": "ACCOUNT_STATUS_NORMAL",            "assets": [                {                    "coin": "ETH",                    "frozen": "0",                    "free": "11.53485",                    "withdraw": ""                }            ]        }    },    "retExtInfo": {},    "time": 1672136539127}
```

---

## Redeem Funds

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/redeem

**Contents:**

- Redeem Funds
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Redeem Funds
On this page
Redeem Funds
Withdraw funds from the Bybit asset pool.
tip
There will be two redemption records: one for the redeemed quantity, and the other one is for the total interest
occurred.
HTTP Request
​
POST
/v5/lending/redeem
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
Coin name
quantity
ture
string
Redemption quantity
serialNo
false
string
Serial no. A customised ID, and it will automatically generated if not passed
Response Parameters
​
Parameter
Type
Comments
coin
string
Coin name
createdTime
string
Created timestamp (ms)
orderId
string
Order ID
principalQty
string
Redemption quantity
serialNo
string
Serial No
status
string
Order status.
0
: Initial,
1
: Processing,
2
: Success,
10
: Failed
updatedTime
string
Updated timestamp (ms)
Request Example
​
POST
/v5/lending/redeem
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
"quantity"
:
"0.1"
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
"principalQty"
:
"0.1"
,
"serialNo"
:
"14035171132183710722373"
,
"status"
:
"0"
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
POST /v5/lending/redeem HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682048277724X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "coin": "BTC",    "quantity": "0.1",    "serialNo": null}
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "coin": "BTC",        "createdTime": "1682048277963",        "orderId": "1403517113428086272",        "principalQty": "0.1",        "serialNo": "14035171132183710722373",        "status": "0",        "updatedTime": "1682048277963"    },    "retExtInfo": {},    "time": 1682048278001}
```

---

## Get Master Deposit Address

**URL:** https://bybit-exchange.github.io/docs/v5/asset/deposit/master-deposit-addr

**Contents:**

- Get Master Deposit Address
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Deposit
Get Master Deposit Address
On this page
Get Master Deposit Address
Query the deposit address information of MASTER account.
HTTP Request
​
GET
/v5/asset/deposit/query-address
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
Coin, uppercase only
chainType
false
string
Please use the value of
> > chain
from
coin-info
endpoint
Response Parameters
​
Parameter
Type
Comments
coin
string
Coin
chains
array
Object
> chainType
string
Chain type
> addressDeposit
string
The address for deposit
> tagDeposit
string
Tag of deposit
> chain
string
Chain
> batchReleaseLimit
string
The deposit limit for this coin in this chain.
"-1"
means no limit
> contractAddress
string
The contract address of the coin. Only display last 6 characters, if there is no contract address, it shows
""
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/deposit/query-address?coin=USDT&chainType=ETH
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
1672192792371
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
get_master_deposit_address
(
coin
=
"USDT"
,
chainType
=
"ETH"
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
getMasterDepositAddress
(
'USDT'
,
'ETH'
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
"coin"
:
"USDT"
,
"chains"
:
[
{
"chainType"
:
"Ethereum (ERC20)"
,
"addressDeposit"
:
"XXXXXX"
,
"tagDeposit"
:
""
,
"chain"
:
"ETH"
,
"batchReleaseLimit"
:
"-1"
,
"contractAddress"
:
"831ec7"
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
1736394811459
}

**Examples:**

Example 1 ():

```
GET /v5/asset/deposit/query-address?coin=USDT&chainType=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672192792371X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_master_deposit_address(    coin="USDT",    chainType="ETH",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getMasterDepositAddress('USDT', 'ETH')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "coin": "USDT",        "chains": [            {                "chainType": "Ethereum (ERC20)",                "addressDeposit": "XXXXXX",                "tagDeposit": "",                "chain": "ETH",                "batchReleaseLimit": "-1",                "contractAddress": "831ec7"            }        ]    },    "retExtInfo": {},    "time": 1736394811459}
```

---

## Get Convert Coin List

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert/convert-coin-list

**Contents:**

- Get Convert Coin List
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert
Get Convert Coin List
On this page
Get Convert Coin List
Query for the list of coins you can convert to/from.
HTTP Request
​
GET
/v5/asset/exchange/query-coin-list
Request Parameters
​
Parameter
Required
Type
Comments
accountType
true
string
Wallet type
eb_convert_funding
eb_convert_uta
eb_convert_spot
eb_convert_contract
eb_convert_inverse
coin
false
string
Coin, uppercase only
Convert from coin (coin to sell)
when side=0, coin field is ignored
side
false
integer
0
: fromCoin list, the balance is given if you have it;
1
: toCoin list (coin to buy)
when side=1 and coin field is filled, it returns toCoin list based on coin field
Response Parameters
​
Parameter
Type
Comments
coins
array
<
object
>
Coin spec
> coin
string
Coin
> fullName
string
Full coin name
> icon
string
Coin icon url
> iconNight
string
Coin icon url (dark mode)
> accuracyLength
integer
Coin precision
> coinType
string
crypto
> balance
string
Balance
When side=0, it gives available balance but cannot used to convert. To get an exact balance to convert, you need specify
side=1
and
coin
parameter
> uBalance
string
Coin balance in USDT worth value
> singleFromMinLimit
string
The minimum amount of fromCoin per transaction
> singleFromMaxLimit
string
The maximum amount of fromCoin per transaction
> disableFrom
boolean
true
: the coin is disabled to be fromCoin,
false
: the coin is allowed to be fromCoin
> disableTo
boolean
true
: the coin is disabled to be toCoin,
false
: the coin is allowed to be toCoin
> timePeriod
integer
Reserved field, ignored for now
> singleToMinLimit
string
Reserved field, ignored for now
> singleToMaxLimit
string
Reserved field, ignored for now
> dailyFromMinLimit
string
Reserved field, ignored for now
> dailyFromMaxLimit
string
Reserved field, ignored for now
> dailyToMinLimit
string
Reserved field, ignored for now
> dailyToMaxLimit
string
Reserved field, ignored for now
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/exchange/query-coin-list?side=0&accountType=eb_convert_funding
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
1720064061248
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
get_convert_coin_list
(
side
=
"0"
,
accountType
=
"eb_convert_funding"
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
getConvertCoins
(
{
accountType
:
'eb_convert_spot'
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
"ok"
,
"result"
:
{
"coins"
:
[
{
"coin"
:
"BTC"
,
"fullName"
:
"BTC"
,
"icon"
:
"https://t1.bycsi.com/app/assets/token/0717b8c28c2373bf714c964195411d0f.svg"
,
"iconNight"
:
"https://t1.bycsi.com/app/assets/token/9504b4c841194cc38f04041003ffbfdb.svg"
,
"accuracyLength"
:
8
,
"coinType"
:
"crypto"
,
"balance"
:
"0"
,
"uBalance"
:
"0"
,
"timePeriod"
:
0
,
"singleFromMinLimit"
:
"0.001"
,
"singleFromMaxLimit"
:
"1"
,
"singleToMinLimit"
:
"0"
,
"singleToMaxLimit"
:
"0"
,
"dailyFromMinLimit"
:
"0"
,
"dailyFromMaxLimit"
:
"0"
,
"dailyToMinLimit"
:
"0"
,
"dailyToMaxLimit"
:
"0"
,
"disableFrom"
:
false
,
"disableTo"
:
false
}
,
...
{
"coin"
:
"SOL"
,
"fullName"
:
"SOL"
,
"icon"
:
"https://s1.bycsi.com/app/assets/token/87ca5f1ca7229bdf0d9a16435653007c.svg"
,
"iconNight"
:
"https://t1.bycsi.com/app/assets/token/383a834046655ffe5ef1be1a025791cc.svg"
,
"accuracyLength"
:
8
,
"coinType"
:
"crypto"
,
"balance"
:
"18.05988133"
,
"uBalance"
:
"2458.46990211775033220586588327"
,
"timePeriod"
:
0
,
"singleFromMinLimit"
:
"0.1"
,
"singleFromMaxLimit"
:
"1250"
,
"singleToMinLimit"
:
"0"
,
"singleToMaxLimit"
:
"0"
,
"dailyFromMinLimit"
:
"0"
,
"dailyFromMaxLimit"
:
"0"
,
"dailyToMinLimit"
:
"0"
,
"dailyToMaxLimit"
:
"0"
,
"disableFrom"
:
false
,
"disableTo"
:
false
}
,
...
{
"coin"
:
"ETH"
,
"fullName"
:
"ETH"
,
"icon"
:
"https://s1.bycsi.com/app/assets/token/d6c17c9e767e1810875c702d86ac9f32.svg"
,
"iconNight"
:
"https://t1.bycsi.com/app/assets/token/9613ac8e7d62081f4ca20488ae5b168d.svg"
,
"accuracyLength"
:
8
,
"coinType"
:
"crypto"
,
"balance"
:
"0.80264489"
,
"uBalance"
:
"2596.09751650032773106431534138"
,
"timePeriod"
:
0
,
"singleFromMinLimit"
:
"0.01"
,
"singleFromMaxLimit"
:
"250"
,
"singleToMinLimit"
:
"0"
,
"singleToMaxLimit"
:
"0"
,
"dailyFromMinLimit"
:
"0"
,
"dailyFromMaxLimit"
:
"0"
,
"dailyToMinLimit"
:
"0"
,
"dailyToMaxLimit"
:
"0"
,
"disableFrom"
:
false
,
"disableTo"
:
false
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
1720064061736
}

**Examples:**

Example 1 ():

```
GET /v5/asset/exchange/query-coin-list?side=0&accountType=eb_convert_funding HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720064061248X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_convert_coin_list(    side="0",    accountType="eb_convert_funding",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getConvertCoins({ accountType: 'eb_convert_spot' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "coins": [            {                "coin": "BTC",                "fullName": "BTC",                "icon": "https://t1.bycsi.com/app/assets/token/0717b8c28c2373bf714c964195411d0f.svg",                "iconNight": "https://t1.bycsi.com/app/assets/token/9504b4c841194cc38f04041003ffbfdb.svg",                "accuracyLength": 8,                "coinType": "crypto",                "balance": "0",                "uBalance": "0",                "timePeriod": 0,                "singleFromMinLimit": "0.001",                "singleFromMaxLimit": "1",                "singleToMinLimit": "0",                "singleToMaxLimit": "0",                "dailyFromMinLimit": "0",                "dailyFromMaxLimit": "0",                "dailyToMinLimit": "0",                "dailyToMaxLimit": "0",                "disableFrom": false,                "disableTo": false            },            ...            {                "coin": "SOL",                "fullName": "SOL",                "icon": "https://s1.bycsi.com/app/assets/token/87ca5f1ca7229bdf0d9a16435653007c.svg",                "iconNight": "https://t1.bycsi.com/app/assets/token/383a834046655ffe5ef1be1a025791cc.svg",                "accuracyLength": 8,                "coinType": "crypto",                "balance": "18.05988133",                "uBalance": "2458.46990211775033220586588327",                "timePeriod": 0,                "singleFromMinLimit": "0.1",                "singleFromMaxLimit": "1250",                "singleToMinLimit": "0",                "singleToMaxLimit": "0",                "dailyFromMinLimit": "0",                "dailyFromMaxLimit": "0",                "dailyToMinLimit": "0",                "dailyToMaxLimit": "0",                "disableFrom": false,                "disableTo": false            },            ...            {                "coin": "ETH",                "fullName": "ETH",                "icon": "https://s1.bycsi.com/app/assets/token/d6c17c9e767e1810875c702d86ac9f32.svg",                "iconNight": "https://t1.bycsi.com/app/assets/token/9613ac8e7d62081f4ca20488ae5b168d.svg",                "accuracyLength": 8,                "coinType": "crypto",                "balance": "0.80264489",                "uBalance": "2596.09751650032773106431534138",                "timePeriod": 0,                "singleFromMinLimit": "0.01",                "singleFromMaxLimit": "250",                "singleToMinLimit": "0",                "singleToMaxLimit": "0",                "dailyFromMinLimit": "0",                "dailyFromMaxLimit": "0",                "dailyToMinLimit": "0",                "dailyToMaxLimit": "0",                "disableFrom": false,                "disableTo": false            }        ]    },    "retExtInfo": {},    "time": 1720064061736}
```

---

## Request a Quote

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/quote-apply

**Contents:**

- Request a Quote
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Fiat-Convert
Request a Quote
On this page
Request a Quote
info
Request by the master UID's api key only
HTTP Request
​
POST
/v5/fiat/quote-apply
Request Parameters
​
Parameter
Required
Type
Comments
fromCoin
true
string
Convert from coin (coin to sell)
fromCoinType
true
string
fiat
or
crypto
toCoin
true
string
Convert to coin (coin to buy)
toCoinType
true
string
fiat
or
crypto
requestAmount
true
string
request coin amount (the amount you want to sell)
requestCoinType
false
string
coinType you want to sell,
fiat
or
crypto
, default to
fiat
Response Parameters
​
Parameter
Type
Comments
quoteTxId
string
Quote transaction ID. It is system generated, and it is used to confirm quote
exchangeRate
string
Exchange rate
fromCoin
string
Convert from coin (coin to sell)
fromCoinType
string
From coin type.
fiat
or
crypto
toCoin
string
Convert to coin (coin to buy)
toCoinType
string
To coin type.
fiat
or
crypto
fromAmount
string
From coin amount (amount to sell)
toAmount
string
To coin amount (amount to buy according to exchange rate)
expiredTime
string
The expiry time for this quote (milliseconds)
Request Example
​
HTTP
POST
/v5/fiat/quote-apply
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1720071077014
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
Content-Length
:
172
{
"fromCoin"
:
"ETH"
,
"fromCoinType"
:
"fiat"
,
"toCoin"
:
"BTC"
,
"toCoinType"
:
"crypto"
,
"requestAmount"
:
"0.1"
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
"success"
,
"result"
:
{
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
"ETH"
,
"fromCoinType"
:
"fiat"
,
"toCoin"
:
"BTC"
,
"toCoinType"
:
"crypto"
,
"fromAmount"
:
"0.1"
,
"toAmount"
:
"0.1"
,
"expireTime"
:
"1764561045346"
}
}

**Examples:**

Example 1 ():

```
POST /v5/fiat/quote-apply HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720071077014X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 172{    "fromCoin": "ETH",    "fromCoinType": "fiat",    "toCoin": "BTC",    "toCoinType": "crypto",    "requestAmount": "0.1",}
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "quoteTaxId": "QuoteTaxId123456",        "exchangeRate": "1.0",        "fromCoin": "ETH",        "fromCoinType": "fiat",        "toCoin": "BTC",        "toCoinType": "crypto",        "fromAmount": "0.1",        "toAmount": "0.1",        "expireTime": "1764561045346"    }}
```

---

## Get Fund Custodial Sub Acct

**URL:** https://bybit-exchange.github.io/docs/v5/user/fund-subuid-list

**Contents:**

- Get Fund Custodial Sub Acct
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Get Fund Custodial Sub Acct
On this page
Get Fund Custodial Sub Acct
The institutional client can query the fund custodial sub accounts.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
HTTP Request
​
GET
/v5/user/escrow_sub_members
Request Parameters
​
Parameter
Required
Type
Comments
pageSize
false
string
Data size per page. Return up to 100 records per request
nextCursor
false
string
Cursor. Use the
nextCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
subMembers
array
Object
> uid
string
子帳戶userId
> username
string
用戶名
> memberType
integer
12
: 基金託管子帳戶
> status
integer
帳戶狀態.
1
: 正常
2
: 登陸封禁
4
: 凍結
> accountMode
integer
帳戶模式.
1
: 經典帳戶
3
: UTA帳戶
> remark
string
備註
nextCursor
string
下一頁數據的游標. 返回"0"表示沒有更多的數據了
Request Example
​
HTTP
Python
Node.js
GET
/v5/user/escrow_sub_members?pageSize=2
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
1739763787703
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
"subMembers"
:
[
{
"uid"
:
"104274894"
,
"username"
:
"Private_Wealth_Management"
,
"memberType"
:
12
,
"status"
:
1
,
"remark"
:
"earn fund"
,
"accountMode"
:
3
}
,
{
"uid"
:
"104274884"
,
"username"
:
"Private_Wealth_Management"
,
"memberType"
:
12
,
"status"
:
1
,
"remark"
:
"earn fund"
,
"accountMode"
:
3
}
]
,
"nextCursor"
:
"344"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1739763788699
}

**Examples:**

Example 1 ():

```
GET /v5/user/escrow_sub_members?pageSize=2 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739763787703X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "subMembers": [            {                "uid": "104274894",                "username": "Private_Wealth_Management",                "memberType": 12,                "status": 1,                "remark": "earn fund",                "accountMode": 3            },            {                "uid": "104274884",                "username": "Private_Wealth_Management",                "memberType": 12,                "status": 1,                "remark": "earn fund",                "accountMode": 3            }        ],        "nextCursor": "344"    },    "retExtInfo": {},    "time": 1739763788699}
```

---

## Request a Quote

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert/apply-quote

**Contents:**

- Request a Quote
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert
Request a Quote
On this page
Request a Quote
HTTP Request
​
POST
/v5/asset/exchange/quote-apply
Request Parameters
​
Parameter
Required
Type
Comments
accountType
true
string
Wallet type
fromCoin
true
string
Convert from coin (coin to sell)
toCoin
true
string
Convert to coin (coin to buy)
requestCoin
true
string
Request coin, same as fromCoin
In the future, we may support requestCoin=toCoin
requestAmount
true
string
request coin amount (the amount you want to sell)
fromCoinType
false
string
crypto
toCoinType
false
string
crypto
paramType
false
string
opFrom
, mainly used for API broker user
paramValue
false
string
Broker ID, mainly used for API broker user
requestId
false
string
Customised request ID
a maximum length of 36
Generally it is useless, but it is convenient to track the quote request internally if you fill this field
Response Parameters
​
Parameter
Type
Comments
quoteTxId
string
Quote transaction ID. It is system generated, and it is used to confirm quote and query the result of transaction
exchangeRate
string
Exchange rate
fromCoin
string
From coin
fromCoinType
string
From coin type.
crypto
toCoin
string
To coin
toCoinType
string
To coin type.
crypto
fromAmount
string
From coin amount (amount to sell)
toAmount
string
To coin amount (amount to buy according to exchange rate)
expiredTime
string
The expiry time for this quote (15 seconds)
requestId
string
Customised request ID
extTaxAndFee
array
Compliance-related field. Currently returns an empty array, which may be used in the future
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/exchange/quote-apply
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1720071077014
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
Content-Length
:
172
{
"requestId"
:
"test-00002"
,
"fromCoin"
:
"ETH"
,
"toCoin"
:
"BTC"
,
"accountType"
:
"eb_convert_funding"
,
"requestCoin"
:
"ETH"
,
"requestAmount"
:
"0.1"
,
"paramType"
:
"opFrom"
,
"paramValue"
:
"broker-id-001"
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
request_a_quote
(
requestId
=
"test-00002"
,
fromCoin
=
"ETH"
,
toCoin
=
"BTC"
,
accountType
=
"eb_convert_funding"
,
requestCoin
=
"ETH"
,
requestAmount
=
"0.1"
,
paramType
=
"opFrom"
,
paramValue
=
"broker-id-001"
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
requestConvertQuote
(
{
requestId
:
'test-00002'
,
fromCoin
:
'ETH'
,
toCoin
:
'BTC'
,
accountType
:
'eb_convert_funding'
,
requestCoin
:
'ETH'
,
requestAmount
:
'0.1'
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
"ok"
,
"result"
:
{
"quoteTxId"
:
"10100108106409340067234418688"
,
"exchangeRate"
:
"0.053517914861880000"
,
"fromCoin"
:
"ETH"
,
"fromCoinType"
:
"crypto"
,
"toCoin"
:
"BTC"
,
"toCoinType"
:
"crypto"
,
"fromAmount"
:
"0.1"
,
"toAmount"
:
"0.005351791486188000"
,
"expiredTime"
:
"1720071092225"
,
"requestId"
:
"test-00002"
,
"extTaxAndFee"
:
[
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
1720071077265
}

**Examples:**

Example 1 ():

```
POST /v5/asset/exchange/quote-apply HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720071077014X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 172{    "requestId": "test-00002",    "fromCoin": "ETH",    "toCoin": "BTC",    "accountType": "eb_convert_funding",    "requestCoin": "ETH",    "requestAmount": "0.1",    "paramType": "opFrom",    "paramValue": "broker-id-001"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.request_a_quote(    requestId="test-00002",    fromCoin="ETH",    toCoin="BTC",    accountType="eb_convert_funding",    requestCoin="ETH",    requestAmount="0.1",    paramType="opFrom",    paramValue="broker-id-001",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .requestConvertQuote({    requestId: 'test-00002',    fromCoin: 'ETH',    toCoin: 'BTC',    accountType: 'eb_convert_funding',    requestCoin: 'ETH',    requestAmount: '0.1',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "quoteTxId": "10100108106409340067234418688",        "exchangeRate": "0.053517914861880000",        "fromCoin": "ETH",        "fromCoinType": "crypto",        "toCoin": "BTC",        "toCoinType": "crypto",        "fromAmount": "0.1",        "toAmount": "0.005351791486188000",        "expiredTime": "1720071092225",        "requestId": "test-00002",        "extTaxAndFee":[]    },    "retExtInfo": {},    "time": 1720071077265}
```

---

## Get Sub UID List (Limited)

**URL:** https://bybit-exchange.github.io/docs/v5/user/subuid-list

**Contents:**

- Get Sub UID List (Limited)
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Get Sub UID List (Limited)
On this page
Get Sub UID List (Limited)
Get at most 10k sub UID of master account. Use
master user's api key
only
.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
HTTP Request
​
GET
/v5/user/query-sub-members
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
subMembers
array
Object
> uid
string
Sub user Id
> username
string
Username
> memberType
integer
1
: normal subaccount,
6
: custodial sub account
> status
integer
The status of the user account
1
: normal
2
: login banned
4
: frozen
> accountMode
integer
The account mode of the user account
1
: Classic Account
3
: UTA1.0
4
: UTA1.0 Pro
5
: UTA2.0
6
: UTA2.0 Pro
> remark
string
The remark
Request Example
​
HTTP
Python
Node.js
GET
/v5/user/query-sub-members
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
1676430318405
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
get_sub_uid
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
getSubUIDList
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
"subMembers"
:
[
{
"uid"
:
"106314365"
,
"username"
:
"xxxx02"
,
"memberType"
:
1
,
"status"
:
1
,
"remark"
:
""
,
"accountMode"
:
5
}
,
{
"uid"
:
"106279879"
,
"username"
:
"xxxx01"
,
"memberType"
:
1
,
"status"
:
1
,
"remark"
:
""
,
"accountMode"
:
6
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
1760388036728
}

**Examples:**

Example 1 ():

```
GET /v5/user/query-sub-members HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430318405X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_sub_uid())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSubUIDList()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "subMembers": [            {                "uid": "106314365",                "username": "xxxx02",                "memberType": 1,                "status": 1,                "remark": "",                "accountMode": 5            },            {                "uid": "106279879",                "username": "xxxx01",                "memberType": 1,                "status": 1,                "remark": "",                "accountMode": 6            }        ]    },    "retExtInfo": {},    "time": 1760388036728}
```

---

## Get Affiliate User Info

**URL:** https://bybit-exchange.github.io/docs/v5/affiliate/affiliate-info

**Contents:**

- Get Affiliate User Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Affiliate
Get Affiliate User Info
On this page
Get Affiliate User Info
To use this endpoint, you should have an affiliate account and only tick "affiliate" permission while creating the API
key.
Affiliate site:
https://affiliates.bybit.com
tip
Use master UID only
The api key can only have "Affiliate" permission
The transaction volume and deposit amount are the total amount of the user done on Bybit, and have nothing to do with
commission settlement. Any transaction volume data related to commission settlement is subject to the Affiliate Portal.
HTTP Request
​
GET
/v5/user/aff-customer-info
Request Parameters
​
Parameter
Required
Type
Comments
uid
true
string
The master account UID of affiliate's client
Response Parameters
​
Parameter
Type
Comments
uid
string
UID
vipLevel
string
VIP level
takerVol30Day
string
Taker volume in last 30 days (USDT). All volume related attributes below includes Derivatives, Option, Spot volume
makerVol30Day
string
Maker volume in last 30 days (USDT)
tradeVol30Day
string
Total trading volume in last 30 days (USDT)
depositAmount30Day
string
Deposit amount in last 30 days (USDT), update in 5 mins
takerVol365Day
string
Taker volume in the past year (USDT)
makerVol365Day
string
Maker volume in the past year (USDT)
tradeVol365Day
string
Total trading volume in the past year (USDT)
depositAmount365Day
string
Total deposit amount in the past year (USDT), update in 5 mins
totalWalletBalance
string
Wallet balance range
1
: less than 100 USDT value
2
: [100, 250) USDT value
3
: [250, 500) USDT value
4
: greater than 500 USDT value
depositUpdateTime
string
The update date time (UTC) of deposit data
volUpdateTime
string
The update date of volume data time (UTC)
KycLevel
integer
KYC level.
1
,
2
,
0
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/user/aff-customer-info?uid=1513500
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1685596324209
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
xxxxxx
Content-Type
:
application/json
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
getAffiliateUserInfo
(
{
uid
:
'1513500'
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
"uid"
:
"1513500"
,
"takerVol30Day"
:
"10"
,
"makerVol30Day"
:
"20"
,
"tradeVol30Day"
:
"30"
,
"depositAmount30Day"
:
"90"
,
"takerVol365Day"
:
"100"
,
"makerVol365Day"
:
"500"
,
"tradeVol365Day"
:
"600"
,
"depositAmount365Day"
:
"1300"
,
"totalWalletBalance"
:
"4"
,
"depositUpdateTime"
:
"2023-06-01 05:12:04"
,
"vipLevel"
:
"99"
,
"volUpdateTime"
:
"2023-06-02 00:00:00"
,
"KycLevel"
:
1
}
,
"retExtInfo"
:
{
}
,
"time"
:
1685596324508
}

**Examples:**

Example 1 ():

```
GET /v5/user/aff-customer-info?uid=1513500 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1685596324209X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: xxxxxxContent-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getAffiliateUserInfo({ uid: '1513500' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "uid": "1513500",        "takerVol30Day": "10",        "makerVol30Day": "20",        "tradeVol30Day": "30",        "depositAmount30Day": "90",        "takerVol365Day": "100",        "makerVol365Day": "500",        "tradeVol365Day": "600",        "depositAmount365Day": "1300",        "totalWalletBalance": "4",        "depositUpdateTime": "2023-06-01 05:12:04",        "vipLevel": "99",        "volUpdateTime": "2023-06-02 00:00:00",        "KycLevel": 1    },    "retExtInfo": {},    "time": 1685596324508}
```

---

## Get Withdrawal Records

**URL:** https://bybit-exchange.github.io/docs/v5/asset/withdraw/withdraw-record

**Contents:**

- Get Withdrawal Records
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Withdraw
Get Withdrawal Records
On this page
Get Withdrawal Records
Query withdrawal records.
tip
endTime

-

startTime
should be less than 30 days. Query last 30 days records by default.
Can query by the master UID's api key
only
HTTP Request
​
GET
/v5/asset/withdraw/query-record
Request Parameters
​
Parameter
Required
Type
Comments
withdrawID
false
string
Withdraw ID
txID
false
string
Transaction hash ID
coin
false
string
Coin, uppercase only
withdrawType
false
integer
Withdraw type.
0
(default): on chain.
1
: off chain.
2
: all
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
50
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
rows
array
Object
> txID
string
Transaction ID. It returns
""
when withdrawal failed, withdrawal cancelled
> coin
string
Coin
> chain
string
Chain
> amount
string
Amount
> withdrawFee
string
Withdraw fee
>
status
string
Withdraw status
> toAddress
string
To withdrawal address. Shows the Bybit UID for internal transfers
> tag
string
Tag
> createTime
string
Withdraw created timestamp (ms)
> updateTime
string
Withdraw updated timestamp (ms)
> withdrawId
string
Withdraw ID
> withdrawType
integer
Withdraw type.
0
: on chain.
1
: off chain
> fee
string
> tax
string
> taxRate
string
> taxType
string
nextPageCursor
string
Cursor. Used for pagination
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/withdraw/query-record?coin=USDT&withdrawType=2&limit=2
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
1672194949557
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
get_withdrawal_records
(
coin
=
"USDT"
,
withdrawType
=
2
,
limit
=
2
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
getWithdrawalRecords
(
{
coin
:
'USDT'
,
withdrawType
:
2
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
"success"
,
"result"
:
{
"rows"
:
[
{
"coin"
:
"USDC"
,
"chain"
:
"ETH"
,
"amount"
:
"41.43008"
,
"txID"
:
"0x3d7bddb797f0e86420c982c0723653b8b728fd0ec9953b6b354445848d83a185"
,
"status"
:
"success"
,
"toAddress"
:
"0xE3De6d711e0951d34777b5Cd93c827F822ee8514"
,
"tag"
:
""
,
"withdrawFee"
:
"5"
,
"createTime"
:
"1742738305000"
,
"updateTime"
:
"1742738340000"
,
"withdrawId"
:
"131629076"
,
"withdrawType"
:
0
,
"fee"
:
""
,
"tax"
:
""
,
"taxRate"
:
""
,
"taxType"
:
""
}
,
{
"coin"
:
"USDT"
,
"chain"
:
"SOL"
,
"amount"
:
"951"
,
"txID"
:
"53j7mUftUboJ2TVb1q3zjwNi9gNGWyQ8xhEpkFovzqaTf8LzuZKzr83XjbG62TZWBkWbn27km7SD6Sc9e1BuWUfJ"
,
"status"
:
"success"
,
"toAddress"
:
"DhTEGye1vq2PPr8DPWit4HTDprnvnDiqpVHnHSY1Y82p"
,
"tag"
:
""
,
"withdrawFee"
:
"1"
,
"createTime"
:
"1742729329000"
,
"updateTime"
:
"1742729437000"
,
"withdrawId"
:
"131603458"
,
"withdrawType"
:
0
,
"fee"
:
""
,
"tax"
:
""
,
"taxRate"
:
""
,
"taxType"
:
""
}
]
,
"nextPageCursor"
:
"eyJtaW5JRCI6MTMxNjAzNDU4LCJtYXhJRCI6MTMxNjI5MDc2fQ=="
}
,
"retExtInfo"
:
{
}
,
"time"
:
1750777316807
}

**Examples:**

Example 1 ():

```
GET /v5/asset/withdraw/query-record?coin=USDT&withdrawType=2&limit=2 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672194949557X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_withdrawal_records(    coin="USDT",    withdrawType=2,    limit=2,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getWithdrawalRecords({    coin: 'USDT',    withdrawType: 2,    limit: 2,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "coin": "USDC",                "chain": "ETH",                "amount": "41.43008",                "txID": "0x3d7bddb797f0e86420c982c0723653b8b728fd0ec9953b6b354445848d83a185",                "status": "success",                "toAddress": "0xE3De6d711e0951d34777b5Cd93c827F822ee8514",                "tag": "",                "withdrawFee": "5",                "createTime": "1742738305000",                "updateTime": "1742738340000",                "withdrawId": "131629076",                "withdrawType": 0,                "fee": "",                "tax": "",                "taxRate": "",                "taxType": ""            },            {                "coin": "USDT",                "chain": "SOL",                "amount": "951",                "txID": "53j7mUftUboJ2TVb1q3zjwNi9gNGWyQ8xhEpkFovzqaTf8LzuZKzr83XjbG62TZWBkWbn27km7SD6Sc9e1BuWUfJ",                "status": "success",                "toAddress": "DhTEGye1vq2PPr8DPWit4HTDprnvnDiqpVHnHSY1Y82p",                "tag": "",                "withdrawFee": "1",                "createTime": "1742729329000",                "updateTime": "1742729437000",                "withdrawId": "131603458",                "withdrawType": 0,                "fee": "",                "tax": "",                "taxRate": "",                "taxType": ""            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTMxNjAzNDU4LCJtYXhJRCI6MTMxNjI5MDc2fQ=="    },    "retExtInfo": {},    "time": 1750777316807}
```

---

## Deposit Funds

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/deposit

**Contents:**

- Deposit Funds
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Deposit Funds
On this page
Deposit Funds
Lending funds to Bybit asset pool
info
normal & UMA account: deduct funds from Spot wallet
UTA account: deduct funds from Unified wallet
HTTP Request
​
POST
/v5/lending/purchase
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
Coin name
quantity
true
string
Deposit quantity
serialNo
false
string
Customised ID. If not passed, system will create one by default
Response Parameters
​
Parameter
Type
Comments
coin
string
Coin name
createdTime
string
Created timestamp (ms)
orderId
string
Order ID
quantity
string
Deposit quantity
serialNo
string
Serial No
status
string
Order status.
0
: Initial,
1
: Processing,
2
: Success,
10
: Failed
updatedTime
string
Updated timestamp (ms)
Request Example
​
POST
/v5/lending/purchase
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
1682046368938
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"coin"
:
"USDC"
,
"quantity"
:
"20.00005"
,
"serialNo"
:
"test-00007"
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
"coin"
:
"USDC"
,
"createdTime"
:
"1682046369112"
,
"orderId"
:
"1403501100816928256"
,
"quantity"
:
"20.00005"
,
"serialNo"
:
"test-00007"
,
"status"
:
"0"
,
"updatedTime"
:
"1682046369112"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1682046369120
}

**Examples:**

Example 1 ():

```
POST /v5/lending/purchase HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682046368938X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "coin": "USDC",    "quantity": "20.00005",    "serialNo": "test-00007"}
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "coin": "USDC",        "createdTime": "1682046369112",        "orderId": "1403501100816928256",        "quantity": "20.00005",        "serialNo": "test-00007",        "status": "0",        "updatedTime": "1682046369112"    },    "retExtInfo": {},    "time": 1682046369120}
```

---

## Get Withdrawal Address List

**URL:** https://bybit-exchange.github.io/docs/v5/asset/withdraw/withdraw-address

**Contents:**

- Get Withdrawal Address List
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Withdraw
Get Withdrawal Address List
On this page
Get Withdrawal Address List
Query the withdrawal addresses in the address book.
tip
The API key for querying this endpoint must have withdrawal permissions.
HTTP Request
​
GET
/v5/asset/withdraw/query-address
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
Coin:
When passing
coin=baseCoin
, it refers to the universal addresses.
When passing a coin name, it refers to the regular address on the chain.
chain
false
string
Chain name:
When only passing the chain name, it returns both regular addresses and universal addresses.
When passing the chain name and
coin=baseCoin
, it only returns the universal address corresponding to the chain.
addressType
false
integer
Address type.
0
: OnChain Address Type(Regular Address Type and Universal Address Type).
1
: Internal Transfer Address Type(Invalid "coin" & "chain" Parameters)
2
: On chain address and internal transfer address type (Invalid "coin" & "chain" Parameters)
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
rows
array
Object
> coin
string
Coin
> chain
string
Chain name
> address
string
Address
> tag
string
Address tag
> remark
string
remark
> status
integer
Address status:
0
: Normal.
1
: New Addresses are prohibited from withdrawing coins for 24 Hours.
> addressType
integer
Address type.
0
: OnChain Address Type(Regular Address Type And Universal Address Type)
1
: Internal Transfer Address Type.
2
: Internal Transfer Address Type And OnChain Address Type
> verified
integer
Whether the address has been verified or not:
0
: Unverified Address.
1
: Verified Address.
> createAt
string
Address create time
nextPageCursor
string
Cursor. Used for pagination
Request Example
​
HTTP
GET
/v5/asset/withdraw/query-address
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
1672194949557
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
"rows"
:
[
{
"coin"
:
"USDT"
,
"chain"
:
"ETH"
,
"address"
:
"0x48101adb67d426cb15e46be5f1d9f6ab25f311ea"
,
"tag"
:
""
,
"remark"
:
""
,
"status"
:
0
,
"addressType"
:
0
,
"verified"
:
0
,
"createdAt"
:
"1760951195"
}
,
{
"coin"
:
"baseCoin"
,
"chain"
:
"ETH"
,
"address"
:
"0x48101adb67d426cb15e46be5f1d9f6ab25f311ea"
,
"tag"
:
""
,
"remark"
:
"Universal Address"
,
"status"
:
0
,
"addressType"
:
0
,
"verified"
:
0
,
"createdAt"
:
"1760951332"
}
]
,
"nextPageCursor"
:
"eyJtaW5JRCI6MTA1MDgsIm1heElEIjoxMDUwOX0="
}
,
"retExtInfo"
:
{
}
,
"time"
:
1760960379395
}

**Examples:**

Example 1 ():

```
GET /v5/asset/withdraw/query-address HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672194949557X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "coin": "USDT",                "chain": "ETH",                "address": "0x48101adb67d426cb15e46be5f1d9f6ab25f311ea",                "tag": "",                "remark": "",                "status": 0,                "addressType": 0,                "verified": 0,                "createdAt": "1760951195"            },            {                "coin": "baseCoin",                "chain": "ETH",                "address": "0x48101adb67d426cb15e46be5f1d9f6ab25f311ea",                "tag": "",                "remark": "Universal Address",                "status": 0,                "addressType": 0,                "verified": 0,                "createdAt": "1760951332"            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTA1MDgsIm1heElEIjoxMDUwOX0="    },    "retExtInfo": {},    "time": 1760960379395}
```

---

## Get Trading Pair List

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/query-coin-list

**Contents:**

- Get Trading Pair List
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Fiat-Convert
Get Trading Pair List
On this page
Get Trading Pair List
Query for the list of coins you can convert to/from.
HTTP Request
​
GET
/v5/fiat/query-coin-list
Request Parameters
​
Parameter
Required
Type
Comments
side
false
integer
0
: buy, buy crypto sell fiat;
1
: sell, sell crypto buy fiat
Response Parameters
​
Parameter
Type
Comments
fiats
array
Fiat coin list
> coin
string
Fiat coin code
> fullName
string
Fiat full coin name
> icon
string
Coin icon url
> iconNight
string
Coin icon url (dark mode)
> precision
integer
Fiat precision
> disable
boolean
true
: the coin is disabled,
false
: the coin is allowed
> singleFromMinLimit
string
For buy side, the minimum amount of fiatCoin per transaction
> singleFromMaxLimit
string
For buy side, the maximum amount of fiatCoin per transaction
cryptos
array
Crypto coin list
> coin
string
Fiat coin code
> fullName
string
Fiat full coin name
> icon
string
Coin icon url
> iconNight
string
Coin icon url (dark mode)
> precision
integer
Fiat precision
> disable
boolean
true
: the coin is disabled,
false
: the coin is allowed
> singleFromMinLimit
string
For sell side, the minimum amount of cryptoCoin per transaction
> singleFromMaxLimit
string
For sell side, the maximum amount of cryptoCoin per transaction
Request Example
​
HTTP
GET
/v5/fiat/query-coin-list?side=0
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
1720064061248
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
"fiats"
:
[
{
"coin"
:
"GEL"
,
"fullName"
:
"Georgian Lari"
,
"icon"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/2023-5-4/Tyoe=GEL.svg"
,
"iconNight"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/2023-5-4/Tyoe=GEL.svg"
,
"precision"
:
2
,
"disable"
:
false
,
"singleFromMinLimit"
:
"10"
,
"singleFromMaxLimit"
:
"100000"
}
]
,
"cryptos"
:
[
{
"coin"
:
"USDT"
,
"fullName"
:
"Tether USDT"
,
"icon"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/2024-8-5/8e50959d5f3e45bebf522e0cad456439_1726814031848.svg"
,
"iconNight"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/2024-8-5/8e50959d5f3e45bebf522e0cad456439_1726814031848.svg"
,
"precision"
:
4
,
"disable"
:
false
,
"singleFromMinLimit"
:
"10"
,
"singleFromMaxLimit"
:
"10000"
}
,
{
"coin"
:
"BTC"
,
"fullName"
:
"Bitcoin"
,
"icon"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/20d09e76a0ab401f80bd545ae874c6a3_48x48.svg"
,
"iconNight"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/20d09e76a0ab401f80bd545ae874c6a3_48x48.svg"
,
"precision"
:
8
,
"disable"
:
false
,
"singleFromMinLimit"
:
"0.0001"
,
"singleFromMaxLimit"
:
"1"
}
,
{
"coin"
:
"ETH"
,
"fullName"
:
"Ethereum"
,
"icon"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/40b217058a474e17b5d88653b039055c_48x48.svg"
,
"iconNight"
:
"https://s1.bycsi.com/common-static/wove/fiat-admin/40b217058a474e17b5d88653b039055c_48x48.svg"
,
"precision"
:
8
,
"disable"
:
false
,
"singleFromMinLimit"
:
"0.002"
,
"singleFromMaxLimit"
:
"5"
}
]
}
}

**Examples:**

Example 1 ():

```
GET /v5/fiat/query-coin-list?side=0 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720064061248X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "fiats": [            {                "coin": "GEL",                "fullName": "Georgian Lari",                "icon": "https://s1.bycsi.com/common-static/wove/fiat-admin/2023-5-4/Tyoe=GEL.svg",                "iconNight": "https://s1.bycsi.com/common-static/wove/fiat-admin/2023-5-4/Tyoe=GEL.svg",                "precision": 2,                "disable": false,                "singleFromMinLimit": "10",                "singleFromMaxLimit": "100000"            }        ],        "cryptos": [            {                "coin": "USDT",                "fullName": "Tether USDT",                "icon": "https://s1.bycsi.com/common-static/wove/fiat-admin/2024-8-5/8e50959d5f3e45bebf522e0cad456439_1726814031848.svg",                "iconNight": "https://s1.bycsi.com/common-static/wove/fiat-admin/2024-8-5/8e50959d5f3e45bebf522e0cad456439_1726814031848.svg",                "precision": 4,                "disable": false,                "singleFromMinLimit": "10",                "singleFromMaxLimit": "10000"            },            {                "coin": "BTC",                "fullName": "Bitcoin",                "icon": "https://s1.bycsi.com/common-static/wove/fiat-admin/20d09e76a0ab401f80bd545ae874c6a3_48x48.svg",                "iconNight": "https://s1.bycsi.com/common-static/wove/fiat-admin/20d09e76a0ab401f80bd545ae874c6a3_48x48.svg",                "precision": 8,                "disable": false,                "singleFromMinLimit": "0.0001",                "singleFromMaxLimit": "1"            },            {                "coin": "ETH",                "fullName": "Ethereum",                "icon": "https://s1.bycsi.com/common-static/wove/fiat-admin/40b217058a474e17b5d88653b039055c_48x48.svg",                "iconNight": "https://s1.bycsi.com/common-static/wove/fiat-admin/40b217058a474e17b5d88653b039055c_48x48.svg",                "precision": 8,                "disable": false,                "singleFromMinLimit": "0.002",                "singleFromMaxLimit": "5"            }        ]    }}
```

---

## Get LTV

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/ltv

**Contents:**

- Get LTV
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Get LTV
On this page
Get LTV
HTTP Request
​
GET
/v5/ins-loan/ltv
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
> parentUid
string
User id
> subAccountUids
array
Bound user id
> unpaidAmount
string
Total debt(USDT)
> unpaidInfo
array
Debt details
>> token
string
coin
> > unpaidQty
string
Unpaid principle
> > unpaidInterest
string
Unpaid interest
> balance
string
Total asset. (margin coins converted to USDT). Please read
here
to understand the calculation
> spotBalanceInfo
array
Spot asset details
> > token
string
Spot margin coin
> > price
string
Spot margin coin price
> > qty
string
Spot margin coin quantity
> contractInfo
array
Contract asset details
> > token
string
Contract margin coin
> > price
string
Contract margin coin index price
> > qty
string
Contract margin coin quantity (available balance of Contract account, and it is not involved with LTV calculation)
Request Example
​
GET
/v5/ins-loan/ltv
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1678688069538
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXX
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
"0.1147"
,
"parentUid"
:
"999805"
,
"subAccountUids"
:
[
"999805"
]
,
"unpaidAmount"
:
""
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
"6351.49614274"
,
"unpaidInterest"
:
"264.0137162"
}
]
,
"balance"
:
"57626.875915433333333332400000000"
,
"spotBalanceInfo"
:
[
{
"token"
:
"BTC"
,
"price"
:
"16375.621333333333333332"
,
"qty"
:
"0.2"
}
,
....
{
"token"
:
"XRP"
,
"price"
:
"0.409517"
,
"qty"
:
"10000"
}
]
,
"contractInfo"
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
1669367335608
}

**Examples:**

Example 1 ():

```
GET /v5/ins-loan/ltv HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1678688069538X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "ltvInfo": [            {                "ltv": "0.1147",                "parentUid": "999805",                "subAccountUids": [                    "999805"                ],                "unpaidAmount": "",                "unpaidInfo": [                    {                        "token": "USDT",                        "unpaidQty": "6351.49614274",                        "unpaidInterest": "264.0137162"                    }                ],                "balance": "57626.875915433333333332400000000",                "spotBalanceInfo": [                    {                        "token": "BTC",                        "price": "16375.621333333333333332",                        "qty": "0.2"                    },                    ....                    {                        "token": "XRP",                        "price": "0.409517",                        "qty": "10000"                    }                ],                "contractInfo": [                    {                        "token": "USDT",                        "price": "1",                        "qty": "0"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1669367335608}
```

---

## Get Coin Info

**URL:** https://bybit-exchange.github.io/docs/v5/asset/coin-info

**Contents:**

- Get Coin Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Get Coin Info
On this page
Get Coin Info
Query coin information, including chain information, withdraw and deposit status.
HTTP Request
​
GET
/v5/asset/coin/query-info
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
Coin, uppercase only
Response Parameters
​
Parameter
Type
Comments
rows
array
Object
> name
string
Coin name
> coin
string
Coin
> remainAmount
string
Maximum withdraw amount per transaction
> chains
array
Object
>> chain
string
Chain
> > chainType
string
Chain type
> > confirmation
string
Number of confirmations for deposit: Once this number is reached, your funds will be credited to your account and
available for trading
> > withdrawFee
string
withdraw fee.
If withdraw fee is empty, It means that this coin does not support withdrawal
> > depositMin
string
Min. deposit
> > withdrawMin
string
Min. withdraw
> > minAccuracy
string
The precision of withdraw or deposit
> > chainDeposit
string
The chain status of deposit.
0
: suspend.
1
: normal
> > chainWithdraw
string
The chain status of withdraw.
0
: suspend.
1
: normal
> > withdrawPercentageFee
string
The withdraw fee percentage. It is a real figure, e.g., 0.022 means 2.2%
> > contractAddress
string
Contract address.
""
means no contract address
> > safeConfirmNumber
string
Number of security confirmations: Once this number is reached, your USD equivalent worth funds will be fully unlocked
and available for withdrawal.
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/coin/query-info?coin=MNT
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
1672194580887
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
get_coin_info
(
coin
=
"MNT"
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
getCoinInfo
(
'MNT'
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
"rows"
:
[
{
"name"
:
"MNT"
,
"coin"
:
"MNT"
,
"remainAmount"
:
"10000000"
,
"chains"
:
[
{
"chainType"
:
"Ethereum"
,
"confirmation"
:
"6"
,
"withdrawFee"
:
"3"
,
"depositMin"
:
"0"
,
"withdrawMin"
:
"3"
,
"chain"
:
"ETH"
,
"chainDeposit"
:
"1"
,
"chainWithdraw"
:
"1"
,
"minAccuracy"
:
"8"
,
"withdrawPercentageFee"
:
"0"
,
"contractAddress"
:
"0x3c3a81e81dc49a522a592e7622a7e711c06bf354"
,
"safeConfirmNumber"
:
"65"
}
,
{
"chainType"
:
"Mantle Network"
,
"confirmation"
:
"100"
,
"withdrawFee"
:
"0"
,
"depositMin"
:
"0"
,
"withdrawMin"
:
"10"
,
"chain"
:
"MANTLE"
,
"chainDeposit"
:
"1"
,
"chainWithdraw"
:
"1"
,
"minAccuracy"
:
"8"
,
"withdrawPercentageFee"
:
"0"
,
"contractAddress"
:
""
,
"safeConfirmNumber"
:
"100"
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
1736395486989
}

**Examples:**

Example 1 ():

```
GET /v5/asset/coin/query-info?coin=MNT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672194580887X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_coin_info(    coin="MNT",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getCoinInfo('MNT')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "name": "MNT",                "coin": "MNT",                "remainAmount": "10000000",                "chains": [                    {                        "chainType": "Ethereum",                        "confirmation": "6",                        "withdrawFee": "3",                        "depositMin": "0",                        "withdrawMin": "3",                        "chain": "ETH",                        "chainDeposit": "1",                        "chainWithdraw": "1",                        "minAccuracy": "8",                        "withdrawPercentageFee": "0",                        "contractAddress": "0x3c3a81e81dc49a522a592e7622a7e711c06bf354",                        "safeConfirmNumber": "65"                    },                    {                        "chainType": "Mantle Network",                        "confirmation": "100",                        "withdrawFee": "0",                        "depositMin": "0",                        "withdrawMin": "10",                        "chain": "MANTLE",                        "chainDeposit": "1",                        "chainWithdraw": "1",                        "minAccuracy": "8",                        "withdrawPercentageFee": "0",                        "contractAddress": "",                        "safeConfirmNumber": "100"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1736395486989}
```

---

## Request a Quote

**URL:** https://bybit-exchange.github.io/docs/v5/asset/convert-small-balance/request-quote

**Contents:**

- Request a Quote
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Convert Small Balances
Request a Quote
On this page
Request a Quote
Custody accounts, like copper, fireblock, etc are
not supported
to make a convertion
info
API key permission:
Convert
API rate limit:
5 req /s
In a Unified Trading Account, your
actual executed amounts may be less than your available balance
. If you submit convert requests for multiple cryptocurrencies simultaneously, partial executions may occur. Please
refer to the actual credited amounts.
HTTP Request
​
POST
/v5/asset/covert/get-quote
Request Parameters
​
Parameter
Required
Type
Comments
accountType
true
string
Wallet type
eb_convert_uta
. Only supports the Unified wallet
fromCoinList
true
array
<
string
>
Source currency list
["BTC", "XRP", "ETH"]
, up to 20 coins in one transaction
toCoin
true
string
Target currency, each request supports one of MNT, USDT, or USDC
Response Parameters
​
Parameter
Type
Comments
quoteId
string
Quote transaction ID. It is system generated, and it is used to confirm quote and query the result of transaction
result
object
> quoteCreateTime
string
Quote created ts
> quoteExpireTime
string
Quote expired ts, 30 seconds
> exchangeCoins
array
<
object
>
Quote details
> > fromCoin
string
Source currency
> > supportConvert
integer
1
: support,
2
: not supported
> > availableBalance
string
Withdrawable balance
> > baseValue
string
USDT equivalent value
> > toCoin
string
Target currency
> > toAmount
string
Est.received amount
> > exchangeRate
string
Exchange rate
> > feeInfo
object
Exchange fee info
>>> feeCoin
string
Fee currency
> > > amount
string
Fee
> > > feeRate
string
Fee rate
> > taxFeeInfo
object
Tax fee info
> > > totalAmount
string
Tax fee
> > > feeCoin
string
Tax fee coin
> > > taxFeeItems
array
Tax fee items
> totalFeeInfo
object
Total exchange fee details
> > feeCoin
string
Fee currency
> > amount
string
Total fee
> > feeRate
string
Fee rate
> totalTaxFeeInfo
object
Total tax fee info
> > totalAmount
string
Total tax fee
> > feeCoin
string
Tax fee coin
> > taxFeeItems
array
Tax fee items
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/covert/get-quote
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
XXXXXX
X-BAPI-TIMESTAMP
:
1766126592271
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
Content-Length
:
97
{
"accountType"
:
"eb_convert_uta"
,
"fromCoinList"
:
[
"XRP"
,
"SOL"
]
,
"toCoin"
:
"USDC"
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
request_a_quote_small_balance
(
accountType
=
"eb_convert_uta"
,
fromCoinList
=
[
"XRP"
,
"SOL"
]
,
toCoin
=
"USDC"
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
"quoteId"
:
"1010075157602510902217555968"
,
"result"
:
{
"quoteCreateTime"
:
"1766126593232"
,
"quoteExpireTime"
:
"1766126623231"
,
"exchangeCoins"
:
[
{
"fromCoin"
:
"SOL"
,
"supportConvert"
:
1
,
"availableBalance"
:
"0.000003"
,
"baseValue"
:
"0.00036837"
,
"toCoin"
:
"USDC"
,
"toAmount"
:
"0.00035721396701649"
,
"exchangeRate"
:
"119.07132233883026"
,
"feeInfo"
:
{
"feeCoin"
:
"USDC"
,
"amount"
:
"0.00000729008095952"
,
"feeRate"
:
"0.02"
}
,
"taxFeeInfo"
:
{
"totalAmount"
:
"0"
,
"feeCoin"
:
""
,
"taxFeeItems"
:
[
]
}
}
,
{
"fromCoin"
:
"XRP"
,
"supportConvert"
:
1
,
"availableBalance"
:
"0.0002"
,
"baseValue"
:
"0.00024536"
,
"toCoin"
:
"USDC"
,
"toAmount"
:
"0.000359866676661744"
,
"exchangeRate"
:
"1.79933338330872"
,
"feeInfo"
:
{
"feeCoin"
:
"USDC"
,
"amount"
:
"0.000007344217891056"
,
"feeRate"
:
"0.02"
}
,
"taxFeeInfo"
:
{
"totalAmount"
:
"0"
,
"feeCoin"
:
""
,
"taxFeeItems"
:
[
]
}
}
]
,
"totalFeeInfo"
:
{
"feeCoin"
:
"USDC"
,
"amount"
:
"0.000014634298850576"
,
"feeRate"
:
"0.02"
}
,
"totalTaxFeeInfo"
:
{
"totalAmount"
:
"0"
,
"feeCoin"
:
""
,
"taxFeeItems"
:
[
]
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
1766126593232
}

**Examples:**

Example 1 ():

```
POST /v5/asset/covert/get-quote HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1766126592271X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 97{    "accountType": "eb_convert_uta",    "fromCoinList": ["XRP", "SOL"],    "toCoin": "USDC"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.request_a_quote_small_balance(    accountType="eb_convert_uta",    fromCoinList=["XRP", "SOL"],    toCoin="USDC",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "quoteId": "1010075157602510902217555968",        "result": {            "quoteCreateTime": "1766126593232",            "quoteExpireTime": "1766126623231",            "exchangeCoins": [                {                    "fromCoin": "SOL",                    "supportConvert": 1,                    "availableBalance": "0.000003",                    "baseValue": "0.00036837",                    "toCoin": "USDC",                    "toAmount": "0.00035721396701649",                    "exchangeRate": "119.07132233883026",                    "feeInfo": {                        "feeCoin": "USDC",                        "amount": "0.00000729008095952",                        "feeRate": "0.02"                    },                    "taxFeeInfo": {                        "totalAmount": "0",                        "feeCoin": "",                        "taxFeeItems": []                    }                },                {                    "fromCoin": "XRP",                    "supportConvert": 1,                    "availableBalance": "0.0002",                    "baseValue": "0.00024536",                    "toCoin": "USDC",                    "toAmount": "0.000359866676661744",                    "exchangeRate": "1.79933338330872",                    "feeInfo": {                        "feeCoin": "USDC",                        "amount": "0.000007344217891056",                        "feeRate": "0.02"                    },                    "taxFeeInfo": {                        "totalAmount": "0",                        "feeCoin": "",                        "taxFeeItems": []                    }                }            ],            "totalFeeInfo": {                "feeCoin": "USDC",                "amount": "0.000014634298850576",                "feeRate": "0.02"            },            "totalTaxFeeInfo": {                "totalAmount": "0",                "feeCoin": "",                "taxFeeItems": []            }        }    },    "retExtInfo": {},    "time": 1766126593232}
```

---

## Get Margin Coin Info

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/margin-coin-info

**Contents:**

- Get Margin Coin Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Get Margin Coin Info
On this page
Get Margin Coin Info
HTTP Request
​
GET
/v5/ins-loan/ensure-tokens
Request Parameters
​
Parameter
Required
Type
Comments
productId
false
string
ProductId. If not passed, then return all product margin coin. For spot, it returns coin that convertRation greater than
0.
Response Parameters
​
Parameter
Type
Comments
marginToken
array
Object
> productId
string
Product Id
> spotToken
array
Spot margin coin
>> token
string
Margin coin
> > convertRatio
string
Margin coin convert ratio
> contractToken
array
Contract margin coin
> > token
string
Margin coin
> > convertRatio
string
Margin coin convert ratio
Request Example
​
GET
/v5/ins-loan/ensure-tokens?productId=70
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
"marginToken"
:
[
{
"productId"
:
"70"
,
"spotToken"
:
[
{
"token"
:
"BTC"
,
"convertRatio"
:
"1.00000000"
}
,
{
"token"
:
"ETH"
,
"convertRatio"
:
"1.00000000"
}
,
{
"token"
:
"USDT"
,
"convertRatio"
:
"1"
}
]
,
"contractToken"
:
[
{
"token"
:
"USDT"
,
"convertRatio"
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
1669363954802
}

**Examples:**

Example 1 ():

```
GET /v5/ins-loan/ensure-tokens?productId=70 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "marginToken": [            {                "productId": "70",                "spotToken": [                    {                        "token": "BTC",                        "convertRatio": "1.00000000"                    },                    {                        "token": "ETH",                        "convertRatio": "1.00000000"                    },                    {                        "token": "USDT",                        "convertRatio": "1"                    }                ],                "contractToken": [                    {                        "token": "USDT",                        "convertRatio": "1"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1669363954802}
```

---

## Freeze Sub UID

**URL:** https://bybit-exchange.github.io/docs/v5/user/froze-subuid

**Contents:**

- Freeze Sub UID
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Freeze Sub UID
On this page
Freeze Sub UID
Freeze Sub UID. Use
master user's api key
only
.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
HTTP Request
​
POST
/v5/user/frozen-sub-member
Request Parameters
​
Parameter
Required
Type
Comments
subuid
true
integer
Sub user Id
frozen
true
integer
0
：unfreeze,
1
：freeze
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/user/frozen-sub-member
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-SIGN
:
XXXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676430842094
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"subuid"
:
53888001
,
"frozen"
:
1
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
freeze_sub_uid
(
subuid
=
53888001
,
frozen
=
1
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
setSubUIDFrozenState
(
53888001
,
1
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1676430697553
}

**Examples:**

Example 1 ():

```
POST /v5/user/frozen-sub-member HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "subuid": 53888001,    "frozen": 1}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.freeze_sub_uid(    subuid=53888001,    frozen=1,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .setSubUIDFrozenState(53888001, 1)  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {},    "retExtInfo": {},    "time": 1676430697553}
```

---

## Get Deposit Records (on-chain)

**URL:** https://bybit-exchange.github.io/docs/v5/asset/deposit/deposit-record

**Contents:**

- Get Deposit Records (on-chain)
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Deposit
Get Deposit Records (on-chain)
On this page
Get Deposit Records (on-chain)
Query deposit records
tip
endTime

-

startTime
should be less than 30 days. Query last 30 days records by default.
Support using
main or sub
UID api key to query deposit records respectively.
HTTP Request
​
GET
/v5/asset/deposit/query-record
Request Parameters
​
Parameter
Required
Type
Comments
id
false
string
Internal ID: Can be used to uniquely identify and filter the deposit. When combined with other parameters, this field
takes the highest priority
txID
false
string
Transaction ID: Please note that data generated before Jan 1, 2024 cannot be queried using txID
coin
false
string
Coin, uppercase only
startTime
false
integer
The start timestamp (ms)
Note: the query logic is actually effective based on
second
level
endTime
false
integer
The end timestamp (ms)
Note: the query logic is actually effective based on
second
level
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
rows
array
Object
> coin
string
Coin
> chain
string
Chain
> amount
string
Amount
> txID
string
Transaction ID
>
status
integer
Deposit status
> toAddress
string
Deposit target address
> tag
string
Tag of deposit target address
> depositFee
string
Deposit fee
> successAt
string
Deposit's success time
> confirmations
string
Number of confirmation blocks
> txIndex
string
Transaction sequence number
> blockHash
string
Hash number on the chain
> batchReleaseLimit
string
The deposit limit for this coin in this chain.
"-1"
means no limit
> depositType
string
The deposit type.
0
: normal deposit,
10
: the deposit reaches daily deposit limit,
20
: abnormal deposit
> fromAddress
string
From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives
""
> taxDepositRecordsId
string
This field is used for tax purposes by Bybit EU (Austria) users， declare tax id
> taxStatus
integer
This field is used for tax purposes by Bybit EU (Austria) users
0: No reporting required
1: Reporting pending
2: Reporting completed
> id
string
Unique ID
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
/v5/asset/deposit/query-record?coin=USDT&limit=1
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
1672191991544
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
get_deposit_records
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
getDepositRecords
(
{
coin
:
'USDT'
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
"rows"
:
[
{
"coin"
:
"USDT"
,
"chain"
:
"TRX"
,
"amount"
:
"999.0496"
,
"txID"
:
"04bf3fbad2fc85b107a42cfdc5ff83110092b606ca754efa0f032f8b94b3262e"
,
"status"
:
3
,
"toAddress"
:
"TDGYpm5zPacnEqKV34TJPuhJhHom9hcXAy"
,
"tag"
:
""
,
"depositFee"
:
""
,
"successAt"
:
"1742728163000"
,
"confirmations"
:
"50"
,
"txIndex"
:
"0"
,
"blockHash"
:
"000000000436ab4dabc8a4a87beb2262d2d87f6761a825494c4f1d5ae11b27e8"
,
"batchReleaseLimit"
:
"-1"
,
"depositType"
:
"0"
,
"fromAddress"
:
"TJ7hhYhVhaxNx6BPyq7yFpqZrQULL3JSdb"
,
"taxDepositRecordsId"
:
"0"
,
"taxStatus"
:
0
,
"id"
:
"160237231"
}
]
,
"nextPageCursor"
:
"eyJtaW5JRCI6MTYwMjM3MjMxLCJtYXhJRCI6MTYwMjM3MjMxfQ=="
}
,
"retExtInfo"
:
{
}
,
"time"
:
1750798211884
}

**Examples:**

Example 1 ():

```
GET /v5/asset/deposit/query-record?coin=USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672191991544X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_deposit_records(    coin="USDT",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getDepositRecords({    coin: 'USDT'  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "coin": "USDT",                "chain": "TRX",                "amount": "999.0496",                "txID": "04bf3fbad2fc85b107a42cfdc5ff83110092b606ca754efa0f032f8b94b3262e",                "status": 3,                "toAddress": "TDGYpm5zPacnEqKV34TJPuhJhHom9hcXAy",                "tag": "",                "depositFee": "",                "successAt": "1742728163000",                "confirmations": "50",                "txIndex": "0",                "blockHash": "000000000436ab4dabc8a4a87beb2262d2d87f6761a825494c4f1d5ae11b27e8",                "batchReleaseLimit": "-1",                "depositType": "0",                "fromAddress": "TJ7hhYhVhaxNx6BPyq7yFpqZrQULL3JSdb",                "taxDepositRecordsId": "0",                "taxStatus": 0,                "id": "160237231"            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTYwMjM3MjMxLCJtYXhJRCI6MTYwMjM3MjMxfQ=="    },    "retExtInfo": {},    "time": 1750798211884}
```

---

## Get Lending Coin Info

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/coin-info

**Contents:**

- Get Lending Coin Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Get Lending Coin Info
On this page
Get Lending Coin Info
Get the basic information of lending coins
info
All
v5/lending
APIs need
SPOT
permission.
HTTP Request
​
GET
/v5/lending/info
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
Coin name. Return all currencies by default
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
> maxRedeemQty
string
The maximum redeemable qty per day (measured from 0 - 24 UTC)
> minPurchaseQty
string
The minimum qty that can be deposited per request
> precision
string
Deposit quantity accuracy
> rate
string
Annualized interest rate. e.g. 0.0002 means 0.02%
> loanToPoolRatio
string
Capital utilization rate. e.g. 0.0004 means 0.04%
> actualApy
string
The actual annualized interest rate
Request Example
​
GET
/v5/lending/info?coin=ETH
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
1682045949295
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
"actualApy"
:
"0.003688421873941958"
,
"coin"
:
"ETH"
,
"loanToPoolRatio"
:
"0.16855491872747133044"
,
"maxRedeemQty"
:
"161"
,
"minPurchaseQty"
:
"0.03"
,
"precision"
:
"8"
,
"rate"
:
"0.003411300771389848"
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
1682045942972
}

**Examples:**

Example 1 ():

```
GET /v5/lending/info?coin=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682045949295X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "actualApy": "0.003688421873941958",                "coin": "ETH",                "loanToPoolRatio": "0.16855491872747133044",                "maxRedeemQty": "161",                "minPurchaseQty": "0.03",                "precision": "8",                "rate": "0.003411300771389848"            }        ]    },    "retExtInfo": {},    "time": 1682045942972}
```

---

## Get Reference Price

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/reference-price

**Contents:**

- Get Reference Price
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Fiat-Convert
Get Reference Price
On this page
Get Reference Price
HTTP Request
​
GET
/v5/fiat/reference-price
Request Parameters
​
Parameter
Required
Type
Comments
symbol
true
string
Coin Pair, such as EUR-USDT
Response Parameters
​
Parameter
Type
Comments
result
array
Array of quotes
> symbol
string
Trading pair symbol
> fiat
string
Fiat currency of the trading pair (e.g: "EUR")
> crypto
string
Cryptocurrency of the trading pair (e.g:"USDT")
> timestamp
string
Unix timestamp
> buys
array
Array of buy quote objects
>> unitPrice
string
unitPrice: 1 crypto=x fiat
> > paymentMethod
string
From coin type.
fiat
or
crypto
> sells
array
Array of sell quote objects
> > unitPrice
string
unitPrice: 1 crypto=x fiat
> > paymentMethod
string
From coin type.
fiat
or
crypto
Request Example
​
HTTP
GET
/v5/fiat/reference-price
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
""
,
"result"
:
{
"symbol"
:
"EUR-USDT"
,
"fiat"
:
"EUR"
,
"crypto"
:
"USDT"
,
"timestamp"
:
"1765181161"
,
"buys"
:
[
{
"unitPrice"
:
"0.8581"
,
"paymentMethod"
:
"Cash Balance"
}
,
{
"unitPrice"
:
"0.9297487"
,
"paymentMethod"
:
"Credit Card"
}
,
{
"unitPrice"
:
"0.9807915"
,
"paymentMethod"
:
"Apple Pay"
}
,
{
"unitPrice"
:
"0.8631747"
,
"paymentMethod"
:
"Google Pay"
}
]
,
"sells"
:
[
{
"unitPrice"
:
"0.8581"
,
"paymentMethod"
:
"Cash Balance"
}
,
{
"unitPrice"
:
"0.9297487"
,
"paymentMethod"
:
"Credit Card"
}
,
{
"unitPrice"
:
"0.9807915"
,
"paymentMethod"
:
"Apple Pay"
}
,
{
"unitPrice"
:
"0.8631747"
,
"paymentMethod"
:
"Google Pay"
}
,
{
"unitPrice"
:
"0.8584759"
,
"paymentMethod"
:
"SEPA"
}
]
}
}

**Examples:**

Example 1 ():

```
GET /v5/fiat/reference-price HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720074159814X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "symbol": "EUR-USDT",        "fiat": "EUR",        "crypto": "USDT",        "timestamp": "1765181161",        "buys": [            {                "unitPrice": "0.8581",                "paymentMethod": "Cash Balance"            },            {                "unitPrice": "0.9297487",                "paymentMethod": "Credit Card"            },            {                "unitPrice": "0.9807915",                "paymentMethod": "Apple Pay"            },            {                "unitPrice": "0.8631747",                "paymentMethod": "Google Pay"            }        ],        "sells": [            {                "unitPrice": "0.8581",                "paymentMethod": "Cash Balance"            },            {                "unitPrice": "0.9297487",                "paymentMethod": "Credit Card"            },            {                "unitPrice": "0.9807915",                "paymentMethod": "Apple Pay"            },            {                "unitPrice": "0.8631747",                "paymentMethod": "Google Pay"            },            {                "unitPrice": "0.8584759",                "paymentMethod": "SEPA"            }        ]    }}
```

---

## Get Internal Transfer Records

**URL:** https://bybit-exchange.github.io/docs/v5/asset/transfer/inter-transfer-list

**Contents:**

- Get Internal Transfer Records
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Transfer
Get Internal Transfer Records
On this page
Get Internal Transfer Records
Query the internal transfer records between different
account types
under the same UID.
info
If startTime and endTime are not provided, the API returns data from the past 7 days by default.
If only startTime is provided, the API returns records from startTime to startTime + 7 days.
If only endTime is provided, the API returns records from endTime - 7 days to endTime.
If both are provided, the maximum allowed range is 7 days (endTime - startTime ≤ 7 days).
HTTP Request
​
GET
/v5/asset/transfer/query-inter-transfer-list
Request Parameters
​
Parameter
Required
Type
Comments
transferId
false
string
UUID. Use the one you generated in
createTransfer
coin
false
string
Coin, uppercase only
status
false
string
Transfer status
startTime
false
integer
The start timestamp (ms)
Note: the query logic is actually effective based on
second
level
endTime
false
integer
The end timestamp (ms)
Note: the query logic is actually effective based on
second
level
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
> transferId
string
Transfer ID
> coin
string
Transferred coin
> amount
string
Transferred amount
>
fromAccountType
string
From account type
>
toAccountType
string
To account type
> timestamp
string
Transfer created timestamp (ms)
>
status
string
Transfer status
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
/v5/asset/transfer/inter-transfer-list-query?coin=USDT&limit=1
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
1670988271299
X-BAPI-RECV-WINDOW
:
50000
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
get_internal_transfer_records
(
coin
=
"USDT"
,
limit
=
1
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
getInternalTransferRecords
(
{
coin
:
'USDT'
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
"success"
,
"result"
:
{
"list"
:
[
{
"transferId"
:
"selfTransfer_a1091cc7-9364-4b74-8de1-18f02c6f2d5c"
,
"coin"
:
"USDT"
,
"amount"
:
"5000"
,
"fromAccountType"
:
"SPOT"
,
"toAccountType"
:
"UNIFIED"
,
"timestamp"
:
"1667283263000"
,
"status"
:
"SUCCESS"
}
]
,
"nextPageCursor"
:
"eyJtaW5JRCI6MTM1ODQ2OCwibWF4SUQiOjEzNTg0Njh9"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1670988271677
}

**Examples:**

Example 1 ():

```
GET /v5/asset/transfer/inter-transfer-list-query?coin=USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1670988271299X-BAPI-RECV-WINDOW: 50000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_internal_transfer_records(    coin="USDT",    limit=1,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInternalTransferRecords({    coin: 'USDT',    limit: 1,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {    "list": [        {            "transferId": "selfTransfer_a1091cc7-9364-4b74-8de1-18f02c6f2d5c",            "coin": "USDT",            "amount": "5000",            "fromAccountType": "SPOT",            "toAccountType": "UNIFIED",            "timestamp": "1667283263000",            "status": "SUCCESS"        }    ],    "nextPageCursor": "eyJtaW5JRCI6MTM1ODQ2OCwibWF4SUQiOjEzNTg0Njh9"},    "retExtInfo": {},    "time": 1670988271677}
```

---

## Delete Sub API Key

**URL:** https://bybit-exchange.github.io/docs/v5/user/rm-sub-apikey

**Contents:**

- Delete Sub API Key
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Delete Sub API Key
On this page
Delete Sub API Key
Delete the api key of sub account. Use the sub api key pending to be delete to call the endpoint or use the master api
key
to delete corresponding sub account api key
tip
The API key must have one of the below permissions in order to call this endpoint.
sub API key: "Account Transfer", "Sub Member Transfer"
master API Key: "Account Transfer", "Sub Member Transfer", "Withdrawal"
danger
BE CAREFUL! The Sub account API key will be invalid immediately after calling the endpoint.
HTTP Request
​
POST
/v5/user/delete-sub-api
Request Parameters
​
Parameter
Required
Type
Comments
apikey
false
string
Sub account api key
You must pass this param when you use master account manage sub account api key settings
If you use corresponding sub uid api key call this endpoint,
apikey
param cannot be passed, otherwise throwing an error
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/user/delete-sub-api
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676431922953
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
{
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
delete_sub_api_key
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
deleteSubApiKey
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1676431924719
}

**Examples:**

Example 1 ():

```
POST /v5/user/delete-sub-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676431922953X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json{}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.delete_sub_api_key())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .deleteSubApiKey()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {},    "retExtInfo": {},    "time": 1676431924719}
```

---

## Get Balance

**URL:** https://bybit-exchange.github.io/docs/v5/asset/fiat-convert/balance-query

**Contents:**

- Get Balance
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Fiat-Convert
Get Balance
On this page
Get Balance
HTTP Request
​
GET
/v5/fiat/balance-query
Request Parameters
​
Parameter
Required
Type
Comments
currency
false
string
Fiat
: fiat currency code (ISO 4217) etc: KZT. not set will query all fiat balance list
Response Parameters
​
Parameter
Type
Comments
result
object
object
> totalBalance
string
Total balance
> balance
string
Available balance
> frozenBalance
string
Frozen balance
> currency
string
Currency
Request Example
​
HTTP
GET
/v5/fiat/balance-query
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
"currency"
:
"GEL"
,
"totalBalance"
:
"100000"
,
"balance"
:
"100000"
,
"frozenBalance"
:
"0"
}
]
}

**Examples:**

Example 1 ():

```
GET /v5/fiat/balance-query HTTP/1.1  Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1720074159814X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": [        {            "currency": "GEL",            "totalBalance": "100000",            "balance": "100000",            "frozenBalance": "0"        }    ]}
```

---

## Get Sub UID List (Unlimited)

**URL:** https://bybit-exchange.github.io/docs/v5/user/page-subuid

**Contents:**

- Get Sub UID List (Unlimited)
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Get Sub UID List (Unlimited)
On this page
Get Sub UID List (Unlimited)
This API is applicable to the client who has over 10k sub accounts. Use
master user's api key
only
.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
HTTP Request
​
GET
/v5/user/submembers
Request Parameters
​
Parameter
Required
Type
Comments
pageSize
false
string
Data size per page. Return up to 100 records per request
nextCursor
false
string
Cursor. Use the
nextCursor
token from the response to retrieve the next page of the result set
Response Parameters
​
Parameter
Type
Comments
subMembers
array
Object
> uid
string
Sub user Id
> username
string
Username
> memberType
integer
1
: standard subaccount,
6
:
custodial subaccount
> status
integer
The status of the user account
1
: normal
2
: login banned
4
: frozen
> accountMode
integer
The account mode of the user account
1
: Classic Account
3
: UTA1.0
4
: UTA1.0 Pro
5
: UTA2.0
6
: UTA2.0 Pro
> remark
string
The remark
nextCursor
string
The next page cursor value. "0" means no more pages
Request Example
​
HTTP
Python
GET
/v5/user/submembers?pageSize=1
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
1676430318405
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
get_sub_uid_list_unlimited
(
pageSize
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
""
,
"result"
:
{
"subMembers"
:
[
{
"uid"
:
"106314365"
,
"username"
:
"xxxx02"
,
"memberType"
:
1
,
"status"
:
1
,
"remark"
:
""
,
"accountMode"
:
5
}
,
{
"uid"
:
"106279879"
,
"username"
:
"xxxx01"
,
"memberType"
:
1
,
"status"
:
1
,
"remark"
:
""
,
"accountMode"
:
6
}
]
,
"nextCursor"
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
1760388041006
}

**Examples:**

Example 1 ():

```
GET /v5/user/submembers?pageSize=1 HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430318405X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_sub_uid_list_unlimited(    pageSize="1",))
```

Example 3 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "subMembers": [            {                "uid": "106314365",                "username": "xxxx02",                "memberType": 1,                "status": 1,                "remark": "",                "accountMode": 5            },            {                "uid": "106279879",                "username": "xxxx01",                "memberType": 1,                "status": 1,                "remark": "",                "accountMode": 6            }        ],        "nextCursor": "0"    },    "retExtInfo": {},    "time": 1760388041006}
```

---

## Delete Master API Key

**URL:** https://bybit-exchange.github.io/docs/v5/user/rm-master-apikey

**Contents:**

- Delete Master API Key
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Delete Master API Key
On this page
Delete Master API Key
Delete the api key of master account. Use the api key pending to be delete to call the endpoint. Use
master user's api key
only
.
tip
The API key must have one of the below permissions in order to call this endpoint..
master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"
danger
BE CAREFUL! The API key used to call this interface will be invalid immediately.
HTTP Request
​
POST
/v5/user/delete-api
Request Parameters
​
None
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/user/delete-api
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1676431576621
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
Content-Type
:
application/json
{
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
delete_master_api_key
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
deleteMasterApiKey
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1676431577675
}

**Examples:**

Example 1 ():

```
POST /v5/user/delete-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676431576621X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json{}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.delete_master_api_key())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .deleteMasterApiKey()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {},    "retExtInfo": {},    "time": 1676431577675}
```

---
