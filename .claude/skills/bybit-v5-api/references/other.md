# Bybit-V5-Api - Other

**Pages:** 4

---

## Demo Trading Service

**URL:** https://bybit-exchange.github.io/docs/v5/demo

**Contents:**
- Demo Trading Service
- Introduction​
- Create API Key​
- Usage rules​
- Domain​
- Tips​
- Available API List​
  - Request Demo Trading Funds​
    - HTTP Request​
    - Request Parameters​

Demo Trading Service
On this page
Demo Trading Service
Introduction
​
Bybit v5 Open API supports demo trading account, but please note
not
every API is available for demo trading account because demo trading service is
mainly for trading experience purpose, so that it does not have a complete function compared with the real trading service.
Create API Key
​
You need to log in to your
mainnet
account;
Switch to
Demo Trading
, please note it is an independent account for demo trading only, and it has its own user ID;
Hover the mouse on user avatar, then click "API" to generate api key and secret;
Usage rules
​
Basic trading rules are the same as real trading
Orders generated in demo trading keep
7 days
Default rate limit, not upgradable
Domain
​
Mainnet Demo Trading URL:
Rest API:
https://api-demo.bybit.com
Websocket:
wss://stream-demo.bybit.com
(note that this only supports the private streams; public data is identical to that found on mainnet with
wss://stream.bybit.com
; WS Trade is not supported)
Tips
​
Please note that demo trading is an isolated module. When you create the key from demo trading, please use above domain to connect.
By the way, it is meaningless to use demo trading service in the
testnet
website, so do not create a key from Testnet demo trading.
Available API List
​
Cateogory
Title
Endpoint
Market
All
all endpoints
Trade
Place Order
/v5/order/create
Amend Order
/v5/order/amend
Cancel order
/v5/order/cancel
Get Open Orders
/v5/order/realtime
Cancel All Orders
/v5/order/cancel-all
Get Order History
/v5/order/history
Get Trade History
/v5/execution/list
Batch Place Order
/v5/order/create-batch (linear,option)
Batch Amend Order
/v5/order/amend-batch (linear,option)
Batch Cancel Order
/v5/order/cancel-batch (linear,option)
Position
Get Position Info
/v5/position/list
Set Leverage
/v5/position/set-leverage
Switch Position Mode
/v5/position/switch-mode
Set Trading Stop
/v5/position/trading-stop
Set Auto Add Margin
/v5/position/set-auto-add-margin
Add Or Reduce Margin
/v5/position/add-margin
Get Closed PnL
/v5/position/closed-pnl
Account
Get Wallet Balance
/v5/account/wallet-balance
Get Borrow History
/v5/account/borrow-history
Set Collateral Coin
/v5/account/set-collateral-switch
Get Collateral Info
/v5/account/collateral-info
Get Coin Greeks
/v5/asset/coin-greeks
Get Account Info
/v5/account/info
Get Transaction Log
/v5/account/transaction-log
Set Margin Mode
/v5/account/set-margin-mode
Set Spot Hedging
/v5/account/set-hedging-mode
Asset
Get Delivery Record
/v5/asset/delivery-record
Get USDC Session Settlement
/v5/asset/settlement-record
Spot Margin Trade
Toggle Margin Trade
/v5/spot-margin-trade/switch-mode
Set Leverage
/v5/spot-margin-trade/set-leverage
Get Status And Leverage
/v5/spot-margin-uta/status
WS Private
order,execution,position,wallet,greeks
/v5/private
Request Demo Trading Funds
​
API rate limit: 1 req per minute
HTTP Request
​
POST
/v5/account/demo-apply-money
Request Parameters
​
Parameter
Required
Type
Comments
adjustType
false
integer
0
(default): add demo funds;
1
: reduce demo funds
utaDemoApplyMoney
false
array
> coin
false
string
Applied coin, supports
BTC
,
ETH
,
USDT
,
USDC
> amountStr
false
string
Applied amount, the max applied amount in each request
BTC
: "15"
ETH
: "200"
USDT
: "100000"
USDC
: "100000"
Request Example
​
POST
/v5/account/demo-apply-money
HTTP/1.1
Host
:
api-demo.bybit.com
X-BAPI-SIGN
:
XXXXXXX
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1711420489915
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"adjustType"
:
0
,
"utaDemoApplyMoney"
:
[
{
"coin"
:
"USDT"
,
"amountStr"
:
"109"
}
,
{
"coin"
:
"ETH"
,
"amountStr"
:
"1"
}
]
}
Create Demo Account
​
API rate limit: 5 req per second
Permission: AccountTransfer, SubMemberTransfer or SubMemberTransferList
info
Use product main account or sub account key to call the interface, the domain needs to be "api.bybit.com"
If demo account is existing, this POST request will return the existing UID directly
If using main account key to call, then the generated demo account is under the main account
If using sub account key to call, then the generated demo account is under the sub account
HTTP Request
​
POST
/v5/user/create-demo-member
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
subMemberId
string
Demo account ID
Request Example
​
POST
/v5/user/create-demo-member
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
1728460942776
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
2
{
}
Create Demo Account API Key
​
info
Input generated demo account uid
Use
production main account key
to call the interface, the domain needs to be
"api.bybit.com"
Update Demo Account API Key
​
info
Use
production main account key
to call the interface, the domain needs to be
"api.bybit.com"
Get Demo Account API Key Info
​
info
Use
accordingly demo account key
to call the interface, the domain needs to be
"api-demo.bybit.com"
Delete Demo Account API Key
​
info
Use
production main account key
to call the interface, the domain needs to be
"api.bybit.com"

**Examples:**

Example 1 ():
```
POST /v5/account/demo-apply-money HTTP/1.1Host: api-demo.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1711420489915X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "adjustType": 0,    "utaDemoApplyMoney": [        {            "coin": "USDT",            "amountStr": "109"        },        {            "coin": "ETH",            "amountStr": "1"        }    ]}
```

Example 2 ():
```
POST /v5/user/create-demo-member HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728460942776X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 2{}
```

---

## Purchase

**URL:** https://bybit-exchange.github.io/docs/v5/lt/purchase

**Contents:**
- Purchase
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

On this page
Purchase
Purchase levearge token
HTTP Request
​
POST
/v5/spot-lever-token/purchase
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
ltAmount
true
string
Purchase amount
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
Abbreviation of the LT, such as BTC3L
ltOrderStatus
string
Order status.
1
: completed,
2
: in progress,
3
: failed
execQty
string
Executed qty of LT
execAmt
string
Executed amount of LT
amount
string
Purchase amount
purchaseId
string
Order ID
serialNo
string
Serial number, customised order ID
valueCoin
string
Quote coin
RUN >>
Request Example
​
HTTP
Python
POST
/v5/spot-lever-token/purchase
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1672294730346
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
"amount"
:
"200"
,
"serialNo"
:
"purchase-001"
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
purchase_leveraged_token
(
ltCoin
=
"EOS3L"
,
amount
=
"200"
,
serialNo
=
"purchase-001"
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
"success"
,
"result"
:
{
"amount"
:
"200"
,
"execAmt"
:
""
,
"execQty"
:
""
,
"ltCoin"
:
"EOS3L"
,
"ltOrderStatus"
:
"2"
,
"purchaseId"
:
"2611"
,
"serialNo"
:
"purchase-001"
,
"valueCoin"
:
"USDT"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672293867729
}

**Examples:**

Example 1 ():
```
POST /v5/spot-lever-token/purchase HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672294730346X-BAPI-SIGN: XXXXXX-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "ltCoin": "EOS3L",    "amount": "200",    "serialNo": "purchase-001"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.purchase_leveraged_token(    ltCoin="EOS3L",    amount="200",    serialNo="purchase-001",))
```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "amount": "200",        "execAmt": "",        "execQty": "",        "ltCoin": "EOS3L",        "ltOrderStatus": "2",        "purchaseId": "2611",        "serialNo": "purchase-001",        "valueCoin": "USDT"    },    "retExtInfo": {},    "time": 1672293867729}
```

---

## Set Pledge Token

**URL:** https://bybit-exchange.github.io/docs/v5/backup/set-pledge-token

**Contents:**
- Set Pledge Token
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​

On this page
Set Pledge Token
Set Pledge Token in cross margin
HTTP Request
​
POST
/v5/spot-margin-trade/set-pledge-token
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
coin
pledgeStatus
ture
int
pledge status. 1: on, 0: off
Response Parameters
​
Parameter
Type
Comments
pledgeStatus
string
pledge status：1: on,0: off

---

## Get Announcement

**URL:** https://bybit-exchange.github.io/docs/v5/announcement

**Contents:**
- Get Announcement
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Get Announcement
On this page
Get Announcement
HTTP Request
​
GET
/v5/announcements/index
Request Parameters
​
Parameter
Required
Type
Comments
locale
true
string
Language symbol
type
false
string
Announcement type
tag
false
string
Announcement tag
page
false
integer
Page number. Default:
1
limit
false
integer
Limit for data size per page. Default:
20
Response Parameters
​
Parameter
Type
Comments
total
integer
Total records
list
array
Object
> title
string
Announcement title
> description
string
Announcement description
> type
Object
>> title
string
The title of announcement type
>>
key
string
The key of announcement type
>
tags
array
<
string
>
The tag of announcement
> url
string
Announcement url
> dateTimestamp
number
Timestamp that author fills
> startDataTimestamp
number
The start timestamp (ms) of the event, only valid when
list.type.key == "latest_activities"
> endDataTimestamp
number
The end timestamp (ms) of the event, only valid when
list.type.key == "latest_activities"
> publishTime
number
The published timestamp for the announcement
Request Example
​
HTTP
Python
Java
GET
/v5/announcements/index?locale=en-US&limit=1
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
get_announcement
(
locale
=
"en-US"
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
announcement
.
LanguageSymbol
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
announcementInfoRequest
=
MarketDataRequest
.
builder
(
)
.
locale
(
LanguageSymbol
.
EN_US
)
.
build
(
)
;
client
.
getAnnouncementInfo
(
announcementInfoRequest
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
"total"
:
735
,
"list"
:
[
{
"title"
:
"New Listing: Arbitrum (ARB) — Deposit, Trade and Stake ARB to Share a 400,000 USDT Prize Pool!"
,
"description"
:
"Bybit is excited to announce the listing of ARB on our trading platform!"
,
"type"
:
{
"title"
:
"New Listings"
,
"key"
:
"new_crypto"
}
,
"tags"
:
[
"Spot"
,
"Spot Listings"
]
,
"url"
:
"https://announcements.bybit.com/en-US/article/new-listing-arbitrum-arb-deposit-trade-and-stake-arb-to-share-a-400-000-usdt-prize-pool--bltf662314c211a8616/"
,
"dateTimestamp"
:
1679045608000
,
"startDateTimestamp"
:
1679045608000
,
"endDateTimestamp"
:
1679045608000
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
1679415136117
}

**Examples:**

Example 1 ():
```
GET /v5/announcements/index?locale=en-US&limit=1 HTTP/1.1Host: api.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_announcement(    locale="en-US",    limit=1,))
```

Example 3 ():
```
import com.bybit.api.client.domain.announcement.LanguageSymbol;import com.bybit.api.client.domain.market.request.MarketDataRequest;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncMarketDataRestClient();var announcementInfoRequest = MarketDataRequest.builder().locale(LanguageSymbol.EN_US).build();client.getAnnouncementInfo(announcementInfoRequest, System.out::println);
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "total": 735,        "list": [            {                "title": "New Listing: Arbitrum (ARB) — Deposit, Trade and Stake ARB to Share a 400,000 USDT Prize Pool!",                "description": "Bybit is excited to announce the listing of ARB on our trading platform!",                "type": {                    "title": "New Listings",                    "key": "new_crypto"                },                "tags": [                    "Spot",                    "Spot Listings"                ],                "url": "https://announcements.bybit.com/en-US/article/new-listing-arbitrum-arb-deposit-trade-and-stake-arb-to-share-a-400-000-usdt-prize-pool--bltf662314c211a8616/",                "dateTimestamp": 1679045608000,                "startDateTimestamp": 1679045608000,                "endDateTimestamp": 1679045608000            }        ]    },    "retExtInfo": {},    "time": 1679415136117}
```

---
