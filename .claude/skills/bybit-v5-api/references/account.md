# Bybit-V5-Api - Account

**Pages:** 37

---

## Different Account Modes

**URL:** https://bybit-exchange.github.io/docs/v5/acct-mode

**Contents:**

- Different Account Modes
- UTA 2.0​
- UTA 1.0​
- Classic Account​
- Determine account mode through API​
- API usage changes for UTA 2.0​

On this page
Different Account Modes
There are three main account modes that have existed on the Bybit platform, namely classic account (now unavailable),
unified account 1.0, and unified account 2.0.
This guide serves to assist users with old accounts into upgrading to the latest version. If you registered your account
in 2025 or after then you can safely ignore this guide.
UTA 2.0
​
This account mode is the ultimate version of the unified account, integrating inverse contracts, USDT perpetual, USDT
Futures, USDC perpetual, USDC Futures,
spot and options into a unified trading system. In cross margin and portifolio margin modes, margin is shared among all
trades.
UTA 1.0
​
Under this account mode, inverse contract transactions are in a separate trading account, and the corresponding margin
currency needs to be deposited
into the "inverse derivatives account" before trading, and the margins are not shared between each other. For USDT
perpetual, USDT Futures, USDC perpetual, USDC
Futures, spot and options are all traded within the "unified trading"
Classic Account
​
Under this account mode, contract transactions and spot transactions are separated. Inverse contracts and USDT perpetual
transactions are completed in
the "derivatives account", and spot transactions are completed in the "spot account"
Determine account mode through API
​
Use the key of the corresponding account to call
Get Account Info
, look at the field
unifiedMarginStatus
1
: classic account
3
: uta1.0
4
: uta1.0 (pro version)
5
: uta2.0
6
: uta2.0 (pro version)
P.S. uta or uta (pro), they are the same thing, but pro has a slight performance advantage when trading via API
API usage changes for UTA 2.0
​
API category
API
uta2.0
uta1.0
category=inverse
category=inverse
Market
Get Instruments Info
"unifiedMarginTrade" is true after UTA2.0 is implemented
"unifiedMarginTrade" is false
Trade
Place Order
Inverse Futures no longer support hedge mode, so "positionIdx" is always
0
Inverse Futures support hedge mode, so "positionIdx" can be
0
,
1
,
2
Get Open & Closed Orders
To query the final status orders, use
openOnly
=1, and only retain the latest 500 orders.
To query the final status orders, use
openOnly
=2
Get Order History

1.

orderStatus
is not passed, and all final orders are queried by default

2. Parameters
   baseCoin
   and
   settleCoin
   are supported
3. Active order query is not supported, and some final orders are limited to query
4. Cancelled orders save up to 24 hours
5. Only orders generated after the upgrade can be queried
1.

orderStatus
is not passed, and the default query is active and final orders

2. The parameters
   baseCoin
   and
   settleCoin
   are not supported
3. Active orders and various final orders are always supported
4. No such restriction
   Get Trade History
1. Supports
   baseCoin
   query;
2. The returned createType has a value
3. Only transactions generated after the upgrade can be queried
1.

baseCoin
query is not supported;

2. The returned createType is always empty string
   ""
   Batch Place Order
   Support inverse contract
   Not support inverse contract
   Batch Amend Order
   Support inverse contract
   Not support inverse contract
   Batch Cancel Order
   Support inverse contract
   Not support inverse contract
   Set Disconnect Cancel All
   Support inverse contract, inverse trading orders will be cancelled when dcp is triggered
   Not support inverse contract, inverse trading orders will not be cancelled when dcp is triggered
   Pre-upgrade
   Get Pre-upgrade Order History
   Supports querying orders generated when it is a classic account or unified account 1.0

-

Get Pre-upgrade Trade History
Supports querying transactions generated when it is a classic account or unified account 1.0

-

Get Pre-upgrade Closed PnL
Supports querying close pnl generated when it is a classic account or unified account 1.0

-

Position
Get Position Info

1. Passing multiple symbols is not supported
2. In the response, there are changes in the meaning or use of "tradeMode", "liqPrice", "bustPrice" fields
1. Supports passing multiple symbols
   Get Closed PnL
   Only the close pnl generated after the upgrade can be queried.

-

Set Leverage
Inverse perpetual and inverse Futures only support one-way position mode, and the leverage of buy and sell must be equal
Inverse Futures support hedge-mode positions, and the leverage of buy and sell can be unequal
Switch Cross/Isolated Margin
The margin mode has become the account dimension, and this interface is no longer applicable
Inverse contracts support the use of this interface
Switch Position Mode
Inverse Futures no longer supports hedge-mode positions
Inverse Futures supports hedge-mode positions
Account
Get Wallet Balance
Not support accountType=CONTRACT
Support accountType=CONTRACT
Get Transaction Log (UTA)
Transaction logs for inverse contracts will be included
The transaction log of the inverse contract needs to go through the interface below
Get Transaction Log(Classic)
After upgrading to 2.0, this interface is no longer applicable.
Data from uta 1.0 or classic account can still be obtained
Asset
Get Delivery Record
Support inverse futures delivery records
Not support inverse futures delivery records
All interfaces involving accountType in this directory
CONTRACT is no longer supported because "inverse derivatives account" does not exist anymore
Support CONTRACT (inverse derivatives account)
WebSocket Stream/Trade
Websocket Trade Guideline
Support inverse contract
Not support inverse contract

---

## Get MMP State

**URL:** https://bybit-exchange.github.io/docs/v5/account/get-mmp-state

**Contents:**

- Get MMP State
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get MMP State
On this page
Get MMP State
HTTP Request
​
GET
/v5/account/mmp-state
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
Response Parameters
​
Parameter
Type
Comments
result
array
Object
> baseCoin
> string
> Base coin
> mmpEnabled
> boolean
> Whether the account is enabled mmp
> window
> string
> Time window (ms)
> frozenPeriod
> string
> Frozen period (ms)
> qtyLimit
> string
> Trade qty limit
> deltaLimit
> string
> Delta limit
> mmpFrozenUntil
> string
> Unfreeze timestamp (ms)
> mmpFrozen
> boolean
> Whether the mmp is triggered.
> true
: mmpFrozenUntil is meaningful
> false
: please ignore the value of mmpFrozenUntil
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/account/mmp-reset
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
> 1675842997277
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"baseCoin"
:
"ETH"
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
> get_mmp_state
(
> baseCoin
> =
"ETH"
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
> getMMPState
(
'ETH'
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
"result"
:
[
{
"baseCoin"
:
"BTC"
,
"mmpEnabled"
:
true
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
"0.01"
,
"deltaLimit"
:
"0.01"
,
"mmpFrozenUntil"
:
"1675760625519"
,
"mmpFrozen"
:
false
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
> 1675843188984
> }

**Examples:**

Example 1 ():

```
POST /v5/account/mmp-reset HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "baseCoin": "ETH"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_mmp_state(    baseCoin="ETH",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getMMPState('ETH')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "result": [            {                "baseCoin": "BTC",                "mmpEnabled": true,                "window": "5000",                "frozenPeriod": "100000",                "qtyLimit": "0.01",                "deltaLimit": "0.01",                "mmpFrozenUntil": "1675760625519",                "mmpFrozen": false            }        ]    },    "retExtInfo": {},    "time": 1675843188984}
```

---

## Get SMP Group ID

**URL:** https://bybit-exchange.github.io/docs/v5/account/smp-group

**Contents:**

- Get SMP Group ID
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get SMP Group ID
On this page
Get SMP Group ID
Query the SMP group ID of self match prevention
HTTP Request
​
GET
/v5/account/smp-group
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
smpGroup
integer
Smp group ID. If the UID has no group, it is
0
by default
Request Example
​
HTTP
Python
Node.js
GET
/v5/account/smp-group
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
1702363848192
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
getSMPGroup
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
"success"
,
"result"
:
{
"smpGroup"
:
0
}
,
"retExtInfo"
:
{
}
,
"time"
:
1702363848539
}

**Examples:**

Example 1 ():

```
GET /v5/account/smp-group HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1702363848192X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSMPGroup()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "smpGroup": 0    },    "retExtInfo": {},    "time": 1702363848539}
```

---

## Get Tiered Collateral Ratio

**URL:** https://bybit-exchange.github.io/docs/v5/spot-margin-uta/tier-collateral-ratio

**Contents:**

- Get Tiered Collateral Ratio
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Spot Margin Trade (UTA)
Get Tiered Collateral Ratio
On this page
Get Tiered Collateral Ratio
UTA loan tiered collateral ratio
info
Does not need authentication.
HTTP Request
​
GET
/v5/spot-margin-trade/collateral
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
> string
> Coin name
> collateralRatioList
> array
> Object
>> maxQty
> > string
> > Upper limit(in coin) of the tiered range,
""
> > means positive infinity
> > minQty
> > string
> > lower limit(in coin) of the tiered range
> > collateralRatio
> > string
> > Collateral ratio
> > Request Example
> > ​
> > HTTP
> > Python
> > Node.js
> > GET
> > /v5/spot-margin-trade/collateral?currency=BTC
> > HTTP/1.1
> > Host
:
> > api-testnet.bybit.com
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
)
> > print
(
> > session
> > .
> > get_tiered_collateral_ratio
(
> > currency
> > =
"BTC"
> > ,
)
)
> > Response Example
> > ​
> > {
"retCode"
:
> > 0
> > ,
"retMsg"
:
"OK"
> > ,
"result"
:
> > {
"list"
:
[
> > {
"currency"
:
"BTC"
> > ,
"collateralRatioList"
:
[
{
"minQty"
:
"0"
,
"maxQty"
:
"1000000"
,
"collateralRatio"
:
"0.85"
}
,
{
"minQty"
:
"1000000"
,
"maxQty"
:
""
,
"collateralRatio"
:
"0"
}
]
> > }
]
> > }
> > ,
"retExtInfo"
:
"{}"
> > ,
"time"
:
> > 1739848984945
> > }

**Examples:**

Example 1 ():

```
GET /v5/spot-margin-trade/collateral?currency=BTC HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_tiered_collateral_ratio(    currency="BTC",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "currency": "BTC",                "collateralRatioList": [                    {                        "minQty": "0",                        "maxQty": "1000000",                        "collateralRatio": "0.85"                    },                    {                        "minQty": "1000000",                        "maxQty": "",                        "collateralRatio": "0"                    }                ]            }        ]    },    "retExtInfo": "{}",    "time": 1739848984945}
```

---

## Get Transferable Amount (Unified)

**URL:** https://bybit-exchange.github.io/docs/v5/account/unified-trans-amnt

**Contents:**

- Get Transferable Amount (Unified)
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get Transferable Amount (Unified)
On this page
Get Transferable Amount (Unified)
Query the available amount to transfer of a specific coin in the Unified wallet.
info
Formula of Asset Available Balance for withdraw:
Reverse calculate Asset Available Amount = X, using
totalAvailableBalance
in
Get Wallet Balance
and the asset's tiered collateral ratio
Asset Available Balance for withdraw = min(X, asset spot Available balance - spot hedging qty for portfolio margin mode)
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data
delivery
HTTP Request
​
GET
/v5/account/withdrawal
Request Parameters
​
Parameter
Required
Type
Comments
coinName
true
string
Coin name, uppercase only. Supports up to 20 coins per request, use comma to separate.
BTC,USDC,USDT,SOL
Response Parameters
​
Parameter
Type
Comments
availableWithdrawal
string
Transferable amount for the 1st coin in the request
availableWithdrawalMap
Object
Transferable amount map for each requested coin. In the map, key is the requested coin, and value is the accordingly
amount(string)
e.g., "availableWithdrawalMap":{"BTC":"4.54549050","SOL":"33.16713007","XRP":"10805.54548970","ETH":"17.76451865"}
Request Example
​
HTTP
Python
Node.js
GET
/v5/account/withdrawal?coinName=BTC,SOL,ETH,XRP
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
1739861239242
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
"availableWithdrawal"
:
"4.54549050"
,
"availableWithdrawalMap"
:
{
"BTC"
:
"4.54549050"
,
"SOL"
:
"33.16713007"
,
"XRP"
:
"10805.54548970"
,
"ETH"
:
"17.76451865"
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
1739858984601
}

**Examples:**

Example 1 ():

```
GET /v5/account/withdrawal?coinName=BTC,SOL,ETH,XRP HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1739861239242X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "availableWithdrawal": "4.54549050",        "availableWithdrawalMap": {            "BTC": "4.54549050",            "SOL": "33.16713007",            "XRP": "10805.54548970",            "ETH": "17.76451865"        }    },    "retExtInfo": {},    "time": 1739858984601}
```

---

## Reset MMP

**URL:** https://bybit-exchange.github.io/docs/v5/account/reset-mmp

**Contents:**

- Reset MMP
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Reset MMP
On this page
Reset MMP
info
Once the mmp triggered, you can unfreeze the account by this endpoint, then
qtyLimit
and
deltaLimit
will be reset to 0.
If the account is not frozen, reset action can also remove previous accumulation, i.e.,
qtyLimit
and
deltaLimit
will be reset to 0.
HTTP Request
​
POST
/v5/account/mmp-reset
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
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/account/mmp-reset
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
1675842997277
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"baseCoin"
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
reset_mmp
(
baseCoin
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
resetMMP
(
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
}

**Examples:**

Example 1 ():

```
POST /v5/account/mmp-reset HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "baseCoin": "ETH"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.reset_mmp(    baseCoin="ETH",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .resetMMP('ETH')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success"}
```

---

## Get Sub Account All API Keys

**URL:** https://bybit-exchange.github.io/docs/v5/user/list-sub-apikeys

**Contents:**

- Get Sub Account All API Keys
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

User
Get Sub Account All API Keys
On this page
Get Sub Account All API Keys
Query all api keys information of a sub UID.
tip
Any permission can access this endpoint
Only master account can call this endpoint
HTTP Request
​
GET
/v5/user/sub-apikeys
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
limit
false
integer
Limit for data size per page.
[
1
,
20
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
result
array
Object
> id
> string
> Unique ID. Internal use
> ips
> array
<
> string
>
IP bound
> apiKey
> string
> Api key
> note
> string
> The remark
> status
> integer
> 1
: permanent,
> 2
: expired,
> 3
: within the validity period,
> 4
: expires soon (less than 7 days)
> expiredAt
> datetime
> The expiry day of the api key. Only for those api key with no IP bound or the password has been changed
> createdAt
> datetime
> The create day of the api key
> type
> integer
> The type of api key.
> 1
: personal,
> 2
: connected to the third-party app
> permissions
> Object
> The types of permission
>> ContractTrade
> > array
> > Permission of contract trade
> > Order
> > ,
> > Position
> > Spot
> > array
> > Permission of spot
> > SpotTrade
> > Wallet
> > array
> > Permission of wallet
> > AccountTransfer
> > ,
> > SubMemberTransferList
> > Options
> > array
> > Permission of USDC Contract. It supports trade option and USDC perpetual.
> > OptionsTrade
> > Derivatives
> > array
> > DerivativesTrade
> > Exchange
> > array
> > Permission of convert
> > ExchangeHistory
> > Earn
> > array
> > Permission of earn product
> > Earn
> > Affiliate
> > array
> > Not applicable to sub account, always
[]
> > BlockTrade
> > array
> > Not applicable to subaccount, always
[]
> > NFT
> > array
> > Deprecated
> > , always
[]
> > CopyTrading
> > array
> > Deprecated
> > , always
[]
> secret
> > string
> > Always
"******"
> readOnly
boolean
true
,
false
> deadlineDay
integer
The remaining valid days of api key. Only for those api key with no IP bound or the password has been changed
> flag
string
Api key type
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
/v5/user/sub-apikeys?subMemberId=100400345
HTTP/1.1
Host
:
api.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1699515251088
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
XXXXXX
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
getSubAccountAllApiKeys
(
{
subMemberId
:
'subUID'
,
limit
:
20
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
"result"
:
[
{
"id"
:
"24828209"
,
"ips"
:
[
"*"
]
,
"apiKey"
:
"XXXXXX"
,
"note"
:
"UTA"
,
"status"
:
3
,
"expiredAt"
:
"2023-12-01T02:36:06Z"
,
"createdAt"
:
"2023-08-25T06:42:39Z"
,
"type"
:
1
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
"SubMemberTransferList"
]
,
"Options"
:
[
"OptionsTrade"
]
,
"Derivatives"
:
[
"DerivativesTrade"
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
"ExchangeHistory"
]
,
"NFT"
:
[
]
,
"Affiliate"
:
[
]
,
"Earn"
:
[
]
}
,
"secret"
:
"******"
> > ,
"readOnly"
:
> > false
> > ,
"deadlineDay"
:
> > 21
> > ,
"flag"
:
"hmac"
> > }
]
> > ,
"nextPageCursor"
:
""
> > }
> > ,
"retExtInfo"
:
> > {
> > }
> > ,
"time"
:
> > 1699515251698
> > }

**Examples:**

Example 1 ():

```
GET /v5/user/sub-apikeys?subMemberId=100400345 HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1699515251088X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSubAccountAllApiKeys({    subMemberId: 'subUID',    limit: 20,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "",    "result": {        "result": [            {                "id": "24828209",                "ips": [                    "*"                ],                "apiKey": "XXXXXX",                "note": "UTA",                "status": 3,                "expiredAt": "2023-12-01T02:36:06Z",                "createdAt": "2023-08-25T06:42:39Z",                "type": 1,                "permissions": {                    "ContractTrade": [                        "Order",                        "Position"                    ],                    "Spot": [                        "SpotTrade"                    ],                    "Wallet": [                        "AccountTransfer",                        "SubMemberTransferList"                    ],                    "Options": [                        "OptionsTrade"                    ],                    "Derivatives": [                        "DerivativesTrade"                    ],                    "CopyTrading": [],                    "BlockTrade": [],                    "Exchange": [                        "ExchangeHistory"                    ],                    "NFT": [],                    "Affiliate": [],                    "Earn": []                },                "secret": "******",                "readOnly": false,                "deadlineDay": 21,                "flag": "hmac"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1699515251698}
```

---

## Get Collateral Info

**URL:** https://bybit-exchange.github.io/docs/v5/account/collateral-info

**Contents:**

- Get Collateral Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get Collateral Info
On this page
Get Collateral Info
Get the collateral information of the current unified margin account, including loan interest rate, loanable amount,
collateral conversion rate, whether it can be mortgaged as margin, etc.
HTTP Request
​
GET
/v5/account/collateral-info
Request Parameters
​
Parameter
Required
Type
Comments
currency
false
string
Asset currency of all current collateral, uppercase only
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> currency
> string
> Currency of all current collateral
> hourlyBorrowRate
> string
> Hourly borrow rate
> maxBorrowingAmount
> string
> Max borrow amount. This value is shared across main-sub UIDs
> freeBorrowingLimit
> string
> The maximum limit for interest-free borrowing
> Only the borrowing caused by contracts unrealised loss has interest-free amount
> Spot margin borrowing always has interest
> freeBorrowAmount
> string
> The amount of borrowing within your total borrowing amount that is exempt from interest charges
> borrowAmount
> string
> Borrow amount
> otherBorrowAmount
> string
> The sum of borrowing amount for other accounts under the same main account
> availableToBorrow
> string
> Available amount to borrow. This value is shared across main-sub UIDs
> borrowable
> boolean
> Whether currency can be borrowed
> borrowUsageRate
> string
> Borrow usage rate: sum of main & sub accounts borrowAmount/maxBorrowingAmount, it is an actual value, 0.5 means 50%
> marginCollateral
> boolean
> Whether it can be used as a margin collateral currency (platform),
> true
: YES,
> false
: NO
> When marginCollateral=false, then collateralSwitch is meaningless
> collateralSwitch
> boolean
> Whether the collateral is turned on by user (user),
> true
: ON,
> false
: OFF
> When marginCollateral=true, then collateralSwitch is meaningful
> collateralRatio
> string
> Deprecated
> field. Due to the new Tiered Collateral value logic, this field will no longer be accurate starting on February 19,
2025. Please refer to
Get Tiered Collateral Ratio
> freeBorrowingAmount
> string
> Deprecated
> field, always return
""
> , please refer to
> freeBorrowingLimit
> RUN >>
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/account/collateral-info?currency=BTC
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
> 1672127952719
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
> get_collateral_info
(
> currency
> =
"BTC"
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
> getCollateralInfo
(
'BTC'
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
"list"
:
[
{
"availableToBorrow"
:
"3"
,
"freeBorrowingAmount"
:
""
,
"freeBorrowAmount"
:
"0"
,
"maxBorrowingAmount"
:
"3"
,
"hourlyBorrowRate"
:
"0.00000147"
,
"borrowUsageRate"
:
"0"
,
"collateralSwitch"
:
true
,
"borrowAmount"
:
"0"
,
"borrowable"
:
true
,
"currency"
:
"BTC"
,
"otherBorrowAmount"
:
"0"
,
"marginCollateral"
:
true
,
"freeBorrowingLimit"
:
"0"
,
"collateralRatio"
:
"0.95"
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
> 1691565901952
> }

**Examples:**

Example 1 ():

```
GET /v5/account/collateral-info?currency=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672127952719X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_collateral_info(    currency="BTC",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getCollateralInfo('BTC')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "availableToBorrow": "3",                "freeBorrowingAmount": "",                "freeBorrowAmount": "0",                "maxBorrowingAmount": "3",                "hourlyBorrowRate": "0.00000147",                "borrowUsageRate": "0",                "collateralSwitch": true,                "borrowAmount": "0",                "borrowable": true,                "currency": "BTC",                "otherBorrowAmount": "0",                "marginCollateral": true,                "freeBorrowingLimit": "0",                "collateralRatio": "0.95"            }        ]    },    "retExtInfo": {},    "time": 1691565901952}
```

---

## Get Collateral Adjustment History

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/ltv-adjust-history

**Contents:**

- Get Collateral Adjustment History
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (New)
Get Collateral Adjustment History
On this page
Get Collateral Adjustment History
Query for your LTV adjustment history.
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-common/adjustment-history
Request Parameters
​
Parameter
Required
Type
Comments
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
> string
> Collateral coin
> amount
> string
> amount
> adjustId
> long
> Collateral adjustment transaction ID
> adjustTime
> long
> Adjust timestamp
> preLTV
> string
> LTV before the adjustment
> afterLTV
> string
> LTV after the adjustment
> direction
> integer
> The direction of adjustment,
> 0
: add collateral;
> 1
: reduce collateral
> status
> integer
> The status of adjustment,
> 1
: success;
> 2
: processing;
> 3
: fail
> nextPageCursor
> string
> Refer to the
> cursor
> request parameter
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/crypto-loan-common/adjustment-history?limit=2&collateralCurrency=BTC
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXXX
> X-BAPI-API-KEY
:
> XXXXXX
> X-BAPI-TIMESTAMP
:
> 1752628288472
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
> get_ltv_adjustment_history_new_crypto_loan
(
> limit
> =
"2"
> ,
> collateralCurrency
> =
"BTC"
> ,
)
)
> Response Example
> ​
> {
"retCode"
:
> 0
> ,
"retMsg"
:
"ok"
> ,
"result"
:
> {
"list"
:
[
{
"adjustId"
:
27511
,
"adjustTime"
:
1752627997907
,
"afterLTV"
:
"0.813743"
,
"amount"
:
"0.08"
,
"collateralCurrency"
:
"BTC"
,
"direction"
:
1
,
"preLTV"
:
"0.524602"
,
"status"
:
1
}
,
{
"adjustId"
:
27491
,
"adjustTime"
:
1752218558913
,
"afterLTV"
:
"0.41983"
,
"amount"
:
"0.03"
,
"collateralCurrency"
:
"BTC"
,
"direction"
:
1
,
"preLTV"
:
"0.372314"
,
"status"
:
1
}
]
> ,
"nextPageCursor"
:
"27491"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1752628288732
> }

**Examples:**

Example 1 ():

```
GET /v5/crypto-loan-common/adjustment-history?limit=2&collateralCurrency=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752628288472X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_ltv_adjustment_history_new_crypto_loan(    limit="2",    collateralCurrency="BTC",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "adjustId": 27511,                "adjustTime": 1752627997907,                "afterLTV": "0.813743",                "amount": "0.08",                "collateralCurrency": "BTC",                "direction": 1,                "preLTV": "0.524602",                "status": 1            },            {                "adjustId": 27491,                "adjustTime": 1752218558913,                "afterLTV": "0.41983",                "amount": "0.03",                "collateralCurrency": "BTC",                "direction": 1,                "preLTV": "0.372314",                "status": 1            }        ],        "nextPageCursor": "27491"    },    "retExtInfo": {},    "time": 1752628288732}
```

---

## Set Collateral Coin

**URL:** https://bybit-exchange.github.io/docs/v5/account/set-collateral

**Contents:**

- Set Collateral Coin
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Set Collateral Coin
On this page
Set Collateral Coin
You can decide whether the assets in the Unified account needs to be collateral coins.
HTTP Request
​
POST
/v5/account/set-collateral-switch
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
You can get collateral coin from
here
USDT, USDC cannot be set
collateralSwitch
true
string
ON
: switch on collateral,
OFF
: switch off collateral
Response Parameters
​
None
RUN >>
Request Example
​
HTTP
Python
Node.js
POST
/v5/account/set-collateral-switch
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
1690513916181
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
55
{
"coin"
:
"BTC"
,
"collateralSwitch"
:
"ON"
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
set_collateral_coin
(
coin
=
"BTC"
,
collateralSwitch
=
"ON"
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
setCollateralCoin
(
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1690515818656
}

**Examples:**

Example 1 ():

```
POST /v5/account/set-collateral-switch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1690513916181X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 55{    "coin": "BTC",    "collateralSwitch": "ON"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_collateral_coin(    coin="BTC",    collateralSwitch="ON"))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .setCollateralCoin({    coin: 'BTC',    collateralSwitch: 'ON',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "SUCCESS",    "result": {},    "retExtInfo": {},    "time": 1690515818656}
```

---

## Get Transferable Coin

**URL:** https://bybit-exchange.github.io/docs/v5/asset/transfer/transferable-coin

**Contents:**

- Get Transferable Coin
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Transfer
Get Transferable Coin
On this page
Get Transferable Coin
Query the transferable coin list between each
account type
HTTP Request
​
GET
/v5/asset/transfer/query-transfer-coin-list
Request Parameters
​
Parameter
Required
Type
Comments
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
list
array
A list of coins (as strings)
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/transfer/query-transfer-coin-list?fromAccountType=UNIFIED&toAccountType=CONTRACT
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
1672144322595
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
get_transferable_coin
(
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
getTransferableCoinList
(
'UNIFIED'
,
'CONTRACT'
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
"BTC"
,
"ETH"
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
1672144322954
}

**Examples:**

Example 1 ():

```
GET /v5/asset/transfer/query-transfer-coin-list?fromAccountType=UNIFIED&toAccountType=CONTRACT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672144322595X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_transferable_coin(    fromAccountType="UNIFIED",    toAccountType="CONTRACT",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getTransferableCoinList('UNIFIED', 'CONTRACT')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            "BTC",            "ETH"        ]    },    "retExtInfo": {},    "time": 1672144322954}
```

---

## Get Pre-upgrade Transaction Log

**URL:** https://bybit-exchange.github.io/docs/v5/pre-upgrade/transaction-log

**Contents:**

- Get Pre-upgrade Transaction Log
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Pre-upgrade
Get Pre-upgrade Transaction Log
On this page
Get Pre-upgrade Transaction Log
Query transaction logs which occurred in the USDC Derivatives wallet before the account was upgraded to a Unified
account.
By category="linear", you can query USDC Perps transaction logs occurred during classic account
By category="option", you can query Options transaction logs occurred during classic account
You can get USDC Perpetual, Option records.
info
USDC Perpeual & Option support the recent 6 months data. Please download older data via GUI
HTTP Request
​
GET
/v5/pre-upgrade/account/transaction-log
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
baseCoin
false
string
BaseCoin, uppercase only. e.g., BTC of BTCPERP
type
false
string
Types of transaction logs
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
Cursor. Used for pagination
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> symbol
> string
> Symbol name
> category
> string
> Product type
> side
> string
> Side.
> Buy
> ,
> Sell
> ,
> None
> transactionTime
> string
> Transaction timestamp (ms)
>
type
string
Type
> qty
> string
> Quantity
> size
> string
> Size
> currency
> string
> USDC、USDT、BTC、ETH
> tradePrice
> string
> Trade price
> funding
> string
> Funding fee
> Positive value means receiving funding fee
> Negative value means deducting funding fee
> fee
> string
> Trading fee
> Positive fee value means expense
> Negative fee value means rebates
> cashFlow
> string
> Cash flow
> change
> string
> Change
> cashBalance
> string
> Cash balance
> feeRate
> string
> When type=
> TRADE
> , then it is trading fee rate
> When type=
> SETTLEMENT
> , it means funding fee rate. For side=Buy, feeRate=market fee rate; For side=Sell, feeRate= - market fee rate
> bonusChange
> string
> The change of bonus
> tradeId
> string
> Trade ID
> orderId
> string
> Order ID
> orderLinkId
> string
> User customised order ID
> nextPageCursor
> string
> Cursor. Used for pagination
> Request Example
> ​
> HTTP
> Python
> GET
> /v5/pre-upgrade/account/transaction-log?category=option
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
> 1686808288265
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
"funding"
:
""
,
"orderLinkId"
:
""
,
"orderId"
:
""
,
"fee"
:
"0"
,
"change"
:
"0"
,
"cashFlow"
:
"0"
,
"transactionTime"
:
"1686729604507"
,
"type"
:
"DELIVERY"
,
"feeRate"
:
"0"
,
"bonusChange"
:
""
,
"size"
:
"0"
,
"qty"
:
"0.5"
,
"cashBalance"
:
"1001.1438885"
,
"currency"
:
"USDC"
,
"category"
:
"option"
,
"tradePrice"
:
"1740.25036667"
,
"tradeId"
:
""
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
> 1686809006792
> }

**Examples:**

Example 1 ():

```
GET /v5/pre-upgrade/account/transaction-log?category=option HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1686808288265X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():

```

```

Example 3 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "21%3A0%2C21%3A0",        "list": [            {                "symbol": "ETH-14JUN23-1750-C",                "side": "Buy",                "funding": "",                "orderLinkId": "",                "orderId": "",                "fee": "0",                "change": "0",                "cashFlow": "0",                "transactionTime": "1686729604507",                "type": "DELIVERY",                "feeRate": "0",                "bonusChange": "",                "size": "0",                "qty": "0.5",                "cashBalance": "1001.1438885",                "currency": "USDC",                "category": "option",                "tradePrice": "1740.25036667",                "tradeId": ""            }        ]    },    "retExtInfo": {},    "time": 1686809006792}
```

---

## Manual Repay Without Asset Conversion

**URL:** https://bybit-exchange.github.io/docs/v5/account/no-convert-repay

**Contents:**

- Manual Repay Without Asset Conversion
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Manual Repay Without Asset Conversion
On this page
Manual Repay Without Asset Conversion
info
If
coin
is passed in input parameter and
amount
is not, the coin will be repaid in full.
important
When repaying, system will only use the spot available balance of the debt currency. Users can perform a manual repay
without converting their other assets.
To check the spot available amount to repay, you can call this API:
Get Available Amount to Repay
Repayment is prohibited between 04:00 and 05:30 per hour. Interest is calculated based on the BorrowAmount at 05:00 per
hour.
System repays floating-rate liabilities first, followed by fixed-rate
BYUSDT will not be used for repayment.
HTTP Request
​
POST
/v5/account/no-convert-repay
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
coin name, uppercase only
amount
false
string
Repay amount. If
coin
is not passed in input parameter,
amount
can not be passed in input parameter
Response Parameters
​
Parameter
Type
Comments
result
array
Object
> resultStatus
> string
> P
: Processing
> SU
: Success
> FA
: Failed
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/account/no-convert-repay
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
> 1675842997277
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"coin"
:
"BTC"
> ,
"amount"
:
"0.01"
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
"resultStatus"
:
"P"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1756295680801
> }

**Examples:**

Example 1 ():

```
POST /v5/account/no-convert-repay HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "coin":"BTC",    "amount":"0.01"}
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "resultStatus": "P"    },    "retExtInfo": {},    "time": 1756295680801}
```

---

## Manual Repay

**URL:** https://bybit-exchange.github.io/docs/v5/account/repay

**Contents:**

- Manual Repay
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Manual Repay
On this page
Manual Repay
info
If neither
coin
nor
amount
is passed in input parameter, then repay all the liabilities.
If
coin
is passed in input parameter and
amount
is not, the coin will be repaid in full.
important
When repaying, the system will first use the spot available balance of the debt currency. If that’s not enough, the
remaining amount will be repaid by converting other assets according to the
liquidation order
.
If you only want to repay using your spot balance and don't want to trigger currency convert repayment, please refer to
Manual Repay Without Asset Conversion
Repayment is prohibited between 04:00 and 05:30 per hour. Interest is calculated based on the BorrowAmount at 05:00 per
hour.
System repays floating-rate liabilities first, followed by fixed-rate
BYUSDT will not be used for repayment.
MNT will temporarily not be used for repayment, and repaying MNT liabilities through convert-repay is not supported.
However, you may still use
Manual Repay Without Asset Conversion
to repay MNT using your existing balance.
HTTP Request
​
POST
/v5/account/repay
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
coin name, uppercase only
amount
false
string
Repay amount. If
coin
is not passed in input parameter,
amount
can not be passed in input parameter
Response Parameters
​
Parameter
Type
Comments
result
array
Object
> resultStatus
> string
> P
: Processing
> SU
: Success
> FA
: Failed
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/account/repay
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
> 1675842997277
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"coin"
:
"BTC"
> ,
"amount"
:
"0.01"
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
"resultStatus"
:
"P"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1756295680801
> }

**Examples:**

Example 1 ():

```
POST /v5/account/repay HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "coin":"BTC",    "amount":"0.01"}
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "resultStatus": "P"    },    "retExtInfo": {},    "time": 1756295680801}
```

---

## Enable Universal Transfer for Sub UID

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/enable-unitransfer-subuid

**Contents:**

- Enable Universal Transfer for Sub UID
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Enable Universal Transfer for Sub UID
On this page
Enable Universal Transfer for Sub UID
info
You no longer need to configure transferable sub UIDs. Now, all sub UIDs are automatically enabled for universal
transfer.
Transfer between sub-sub or main-sub
Use this endpoint to enable a subaccount to take part in a universal transfer. It is a one-time switch which, once
thrown, enables a subaccount permanently. If not set, your subaccount cannot use universal transfers.
caution
Can query by the master UID's api key
only
HTTP Request
​
POST
/v5/asset/transfer/save-transfer-sub-member
Request Parameters
​
Parameter
Required
Type
Comments
subMemberIds
true
array
This list has a
single item
. Separate multiple UIDs by comma, e.g.,
"uid1,uid2,uid3"
Response Parameters
​
None
Request Example
​
HTTP
Python
POST
/v5/asset/transfer/save-transfer-sub-member
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
1672147595971
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"subMemberIds"
:
[
"554117,592324,592334"
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
enable_universal_transfer_for_sub_uid
(
subMemberIds
=
[
"554117,592324,592334"
]
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672147593188
}

**Examples:**

Example 1 ():

```
POST /v5/asset/transfer/save-transfer-sub-member HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672147595971X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "subMemberIds": ["554117,592324,592334"]}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.enable_universal_transfer_for_sub_uid(    subMemberIds=["554117,592324,592334"],))
```

Example 3 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {},    "retExtInfo": {},    "time": 1672147593188}
```

---

## Set Deposit Account

**URL:** https://bybit-exchange.github.io/docs/v5/asset/deposit/set-deposit-acct

**Contents:**

- Set Deposit Account
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Deposit
Set Deposit Account
On this page
Set Deposit Account
Set auto transfer account after deposit. The same function as the setting for Deposit on
web GUI
info
Your funds will be deposited into
FUND
wallet by default. You can set the wallet for auto-transfer after deposit by this API.
Only
main
UID can access.
HTTP Request
​
POST
/v5/asset/deposit/deposit-to-account
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
,
FUND
Response Parameters
​
Parameter
Type
Comments
status
integer
Request result:
1
: SUCCESS
0
: FAIL
RUN >>
Request Example
​
HTTP
Python
Node.js
POST
/v5/asset/deposit/deposit-to-account
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
1676887913670
X-BAPI-RECV-WINDOW
:
50000
Content-Type
:
application/json
{
"accountType"
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
set_deposit_account
(
accountType
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
setDepositAccount
(
{
accountType
:
'CONTRACT'
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
1676887914363
}

**Examples:**

Example 1 ():

```
POST /v5/asset/deposit/deposit-to-account HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676887913670X-BAPI-RECV-WINDOW: 50000Content-Type: application/json{    "accountType": "CONTRACT"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_deposit_account(    accountType="CONTRACT",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .setDepositAccount({    accountType: 'CONTRACT'  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "status": 1    },    "retExtInfo": {},    "time": 1676887914363}
```

---

## Manual Borrow

**URL:** https://bybit-exchange.github.io/docs/v5/account/borrow

**Contents:**

- Manual Borrow
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Manual Borrow
On this page
Manual Borrow
info
Borrowing via OpenAPI endpoint supports floating-rate borrowing only.
HTTP Request
​
POST
/v5/account/borrow
Request Parameters
​
Parameter
Required
Type
Comments
coin
true
string
coin name, uppercase only
amount
true
string
Borrow amount
Response Parameters
​
Parameter
Type
Comments
result
array
Object
> coin
> string
> coin name, uppercase only
> amount
> string
> Borrow amount
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/account/borrow
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
> 1675842997277
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"coin"
:
"BTC"
> ,
"amount"
:
"0.01"
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
"coin"
:
"BTC"
> ,
"amount"
:
"0.01"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1756197991955
> }

**Examples:**

Example 1 ():

```
POST /v5/account/borrow HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "coin":"BTC",    "amount":"0.01"}
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "coin": "BTC",        "amount": "0.01"    },    "retExtInfo": {},    "time": 1756197991955}
```

---

## Get Transaction Log

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/contract-transaction-log

**Contents:**

- Get Transaction Log
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Get Transaction Log (Classic)
On this page
Get Transaction Log
Query transaction logs in the derivatives wallet (classic account), and inverse derivatives account (upgraded to UTA)
Permission
: "Contract - Position"
Apply to
: classic account,
UTA1.0
(inverse)
HTTP Request
​
GET
/v5/account/contract-transaction-log
Request Parameters
​
Parameter
Required
Type
Comments
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
list
array
Object
> id
> string
> Unique id
> symbol
> string
> Symbol name
> category
> string
> Product type
> side
> string
> Side.
> Buy
> ,
> Sell
> ,
> None
> transactionTime
> string
> Transaction timestamp (ms)
>
type
string
Type
> qty
> string
> Quantity
> Perps & Futures: it is the quantity for each trade entry and it does not have direction
> size
> string
> Size. The rest position size after the trade is executed, and it has direction, i.e., short with "-"
> currency
> string
> currency
> tradePrice
> string
> Trade price
> funding
> string
> Funding fee
> Positive value means deducting funding fee
> Negative value means receiving funding fee
> fee
> string
> Trading fee
> Positive fee value means expense
> Negative fee value means rebates
> cashFlow
> string
> Cash flow, e.g., (1) close the position, and unRPL converts to RPL, (2) transfer in or transfer out. This does not
> include trading fee, funding fee
> change
> string
> Change = cashFlow - funding - fee
> cashBalance
> string
> Cash balance. This is the wallet balance after a cash change
> feeRate
> string
> When type=
> TRADE
> , then it is trading fee rate
> When type=
> SETTLEMENT
> , it means funding fee rate. For side=Buy, feeRate=market fee rate; For side=Sell, feeRate= - market fee rate
> bonusChange
> string
> The change of bonus
> tradeId
> string
> Trade ID
> orderId
> string
> Order ID
> orderLinkId
> string
> User customised order ID
> nextPageCursor
> string
> Refer to the
> cursor
> request parameter
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/account/contract-transaction-log?limit=1&symbol=BTCUSD
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
> 1714035117255
> X-BAPI-RECV-WINDOW
:
> 5000
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
> getClassicTransactionLogs
(
> {
> limit
:
> 1
> ,
> symbol
:
'BTCUSD'
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
"OK"
> ,
"result"
:
> {
"list"
:
[
{
"id"
:
"467153"
,
"symbol"
:
"BTCUSD"
,
"category"
:
"inverse"
,
"side"
:
"Sell"
,
"transactionTime"
:
"1714032000000"
,
"type"
:
"SETTLEMENT"
,
"qty"
:
"1000"
,
"size"
:
"-1000"
,
"currency"
:
"BTC"
,
"tradePrice"
:
"63974.88"
,
"funding"
:
"-0.00000156"
,
"fee"
:
""
,
"cashFlow"
:
"0.00000000"
,
"change"
:
"0.00000156"
,
"cashBalance"
:
"1.1311"
,
"feeRate"
:
"-0.00010000"
,
"bonusChange"
:
""
,
"tradeId"
:
"423a565c-f1b6-4c81-bc62-760cd7dd89e7"
,
"orderId"
:
""
,
"orderLinkId"
:
""
}
]
> ,
"nextPageCursor"
:
"cursor_id%3D467153%26"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1714035117258
> }

**Examples:**

Example 1 ():

```
GET /v5/account/contract-transaction-log?limit=1&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1714035117255X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getClassicTransactionLogs({    limit: 1,    symbol: 'BTCUSD',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "id": "467153",                "symbol": "BTCUSD",                "category": "inverse",                "side": "Sell",                "transactionTime": "1714032000000",                "type": "SETTLEMENT",                "qty": "1000",                "size": "-1000",                "currency": "BTC",                "tradePrice": "63974.88",                "funding": "-0.00000156",                "fee": "",                "cashFlow": "0.00000000",                "change": "0.00000156",                "cashBalance": "1.1311",                "feeRate": "-0.00010000",                "bonusChange": "",                "tradeId": "423a565c-f1b6-4c81-bc62-760cd7dd89e7",                "orderId": "",                "orderLinkId": ""            }        ],        "nextPageCursor": "cursor_id%3D467153%26"    },    "retExtInfo": {},    "time": 1714035117258}
```

---

## Get Max. Allowed Collateral Reduction Amount

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/reduce-max-collateral-amt

**Contents:**

- Get Max. Allowed Collateral Reduction Amount
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (New)
Get Max. Allowed Collateral Reduction Amount
On this page
Get Max. Allowed Collateral Reduction Amount
Retrieve the maximum redeemable amount of your collateral asset based on LTV.
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
GET
/v5/crypto-loan-common/max-collateral-amount
Request Parameters
​
Parameter
Required
Type
Comments
currency
true
string
Collateral coin
Response Parameters
​
Parameter
Type
Comments
maxCollateralAmount
string
Maximum reduction amount
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan-common/max-collateral-amount?currency=BTC
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
1752627687351
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
get_max_allowed_collateral_reduction_amount_new_crypto_loan
(
collateralCurrency
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
"maxCollateralAmount"
:
"0.08585184"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752627687596
}

**Examples:**

Example 1 ():

```
GET /v5/crypto-loan-common/max-collateral-amount?currency=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752627687351X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_max_allowed_collateral_reduction_amount_new_crypto_loan(    collateralCurrency="BTC",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "maxCollateralAmount": "0.08585184"    },    "retExtInfo": {},    "time": 1752627687596}
```

---

## Obtain Max Loan Amount

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/max-loan-amt

**Contents:**

- Obtain Max Loan Amount
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (New)
Obtain Max Loan Amount
On this page
Obtain Max Loan Amount
Permission: "Spot trade"
UID rate limit: 5 req / second
HTTP Request
​
POST
/v5/crypto-loan-common/max-loan
Request Parameters
​
Parameter
Required
Type
Comments
currency
true
string
Coin to borrow
collateralList
false
array
<
object
>
> amount
> true
> string
> Collateral amount. Only check funding account balance
> ccy
> true
> string
> Collateral coin. Both
> amount
> &
> ccy
> are required, when you pass "collateralList"
> Response Parameters
> ​
> Parameter
> Type
> Comments
> currency
> string
> Coin to borrow
> maxLoan
> string
> Based on your current collateral, and with the option to add more collateral, you can borrow up to
> maxLoan
> notionalUsd
> string
> Nontional USD value
> remainingQuota
> string
> The
> remaining
> individual platform borrowing limit (shared between main and sub accounts)
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/crypto-loan-common/max-loan
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXXX
> X-BAPI-API-KEY
:
> XXXXXX
> X-BAPI-TIMESTAMP
:
> 1768532512103
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> Content-Length
:
> 208
> {
"currency"
:
"BTC"
> ,
"collateralList"
:
[
{
"ccy"
:
"XRP"
,
"amount"
:
"1000"
}
,
{
"ccy"
:
"USDT"
,
"amount"
:
"1000"
}
]
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
"ok"
> ,
"result"
:
> {
"currency"
:
"BTC"
> ,
"maxLoan"
:
"0.1722"
> ,
"notionalUsd"
:
"16456.06"
> ,
"remainingQuota"
:
"9999999.9421"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1768533990031
> }

**Examples:**

Example 1 ():

```
POST /v5/crypto-loan-common/max-loan HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1768532512103X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 208{    "currency": "BTC",    "collateralList": [        {            "ccy": "XRP",            "amount": "1000"        },        {            "ccy": "USDT",            "amount": "1000"        }    ]}
```

Example 2 ():

```

```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "currency": "BTC",        "maxLoan": "0.1722",        "notionalUsd": "16456.06",        "remainingQuota": "9999999.9421"    },    "retExtInfo": {},    "time": 1768533990031}
```

---

## Upgrade to Unified Account Pro

**URL:** https://bybit-exchange.github.io/docs/v5/account/upgrade-unified-account

**Contents:**

- Upgrade to Unified Account Pro
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Upgrade to Unified Account Pro
On this page
Upgrade to Unified Account Pro
Upgrade Guidance
Check your current account status by calling this
Get Account Info
if unifiedMarginStatus=5, then it is
UTA2.0
, you can call below upgrade endpoint to
UTA2.0
Pro. Check
Get Account Info
after a while and if unifiedMarginStatus=6, then the account has successfully upgraded to
UTA2.0
Pro.
info
please note belows:
Please avoid upgrading during these period:
every hour
50th minute to 5th minute of next hour
Please ensure: there is no open orders when upgrade from
UTA2.0
to
UTA2.0
Pro
During the account upgrade process, the data of
Rest API/Websocket stream
may be inaccurate due to the fact that the account-related
asset data is in the processing state. It is recommended to query and use it after the upgrade is completed.
HTTP Request
​
POST
/v5/account/upgrade-to-uta
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
unifiedUpdateStatus
string
Upgrade status.
FAIL
,
PROCESS
,
SUCCESS
unifiedUpdateMsg
Object
If
PROCESS
,
SUCCESS
, it returns
null
> msg
> array
> Error message array. Only
> FAIL
> will have this field
> RUN >>
> Request Example
> ​
> HTTP
> Python
> GO
> Java
> .Net
> Node.js
> POST
> /v5/account/upgrade-to-uta
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
> 1672125123533
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
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
> upgrade_to_unified_trading_account
(
)
)
> import
(
"context"
"fmt"
> bybit
"github.com/bybit-exchange/bybit.go.api"
)
> client
:=
> bybit
> .
> NewBybitHttpClient
(
"YOUR_API_KEY"
> ,
"YOUR_API_SECRET"
)
> client
> .
> NewUtaBybitServiceNoParams
(
)
> .
> UpgradeToUTA
(
> context
> .
> Background
(
)
)
> import
> com
> .
> bybit
> .
> api
> .
> client
> .
> config
> .
> BybitApiConfig
> ;
> import
> com
> .
> bybit
> .
> api
> .
> client
> .
> domain
> .
> account
> .
> request
> .
> AccountDataRequest
> ;
> import
> com
> .
> bybit
> .
> api
> .
> client
> .
> domain
> .
> account
> .
> AccountType
> ;
> import
> com
> .
> bybit
> .
> api
> .
> client
> .
> service
> .
> BybitApiClientFactory
> ;
> var
> client
> =
> BybitApiClientFactory
> .
> newInstance
(
"YOUR_API_KEY"
> ,
"YOUR_API_SECRET"
> ,
> BybitApiConfig
> .
> TESTNET_DOMAIN
)
> .
> newAccountRestClient
(
)
> ;
> System
> .
> out
> .
> println
(
> client
> .
> upgradeAccountToUTA
(
)
)
> ;
> using bybit.net.api;
> using bybit.net.api.ApiServiceImp;
> using bybit.net.api.Models;
> BybitAccountService accountService = new(apiKey: "xxxxxx", apiSecret: "xxxxx");
> Console.WriteLine(await accountService.UpgradeAccount());
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
> upgradeToUnifiedAccount
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
""
> ,
"result"
:
> {
"unifiedUpdateStatus"
:
"FAIL"
> ,
"unifiedUpdateMsg"
:
> {
"msg"
:
[
"Update account failed. You have outstanding liabilities in your Spot account."
,
"Update account failed. Please close the usdc perpetual positions in USDC Account."
,
"unable to upgrade, please cancel the usdt perpetual open orders in USDT account."
,
"unable to upgrade, please close the usdt perpetual positions in USDT account."
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
> 1672125124195
> }

**Examples:**

Example 1 ():

```
POST /v5/account/upgrade-to-uta HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672125123533X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.upgrade_to_unified_trading_account())
```

Example 3 ():

```
import (    "context"    "fmt"    bybit "github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("YOUR_API_KEY", "YOUR_API_SECRET")client.NewUtaBybitServiceNoParams().UpgradeToUTA(context.Background())
```

Example 4 ():

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.account.request.AccountDataRequest;import com.bybit.api.client.domain.account.AccountType;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newAccountRestClient();System.out.println(client.upgradeAccountToUTA());
```

---

## Set Spot Hedging

**URL:** https://bybit-exchange.github.io/docs/v5/account/set-spot-hedge

**Contents:**

- Set Spot Hedging
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Set Spot Hedging
On this page
Set Spot Hedging
You can turn on/off Spot hedging feature in Portfolio margin
HTTP Request
​
POST
/v5/account/set-hedging-mode
Request Parameters
​
Parameter
Required
Type
Comments
setHedgingMode
true
string
ON
,
OFF
Response Parameters
​
Parameter
Type
Comments
retCode
integer
Result code
retMsg
string
Result message
RUN >>
Request Example
​
HTTP
Python
Node.js
POST
/v5/account/set-hedging-mode
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
1700117968580
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
31
{
"setHedgingMode"
:
"OFF"
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
setSpotHedging
(
{
setHedgingMode
:
'ON'
|
'OFF'
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
}

**Examples:**

Example 1 ():

```
POST /v5/account/set-hedging-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1700117968580X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 31{    "setHedgingMode": "OFF"}
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .setSpotHedging({    setHedgingMode: 'ON' | 'OFF',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "SUCCESS"}
```

---

## Get Sub Account Deposit Records

**URL:** https://bybit-exchange.github.io/docs/v5/broker/exchange-broker/sub-deposit-record

**Contents:**

- Get Sub Account Deposit Records
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Broker
Exchange Broker
Get Sub Account Deposit Records
On this page
Get Sub Account Deposit Records
Exchange broker can query subaccount's deposit records by
main
UID's API key without specifying uid.
API rate limit: 300 req / min
tip
endTime

-

startTime
should be less than 30 days. Queries for the last 30 days worth of records by default.
HTTP Request
​
GET
/v5/broker/asset/query-sub-member-deposit-record
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
false
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
> subMemberId
> string
> Sub account user ID
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
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/broker/asset/query-sub-member-deposit-record?coin=USDT&limit=1
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
> get_subaccount_deposit_records
(
> coin
> =
"USDT"
> ,
> limit
> =
> 1
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
> getBrokerSubAccountDeposits
(
> {
> limit
:
> 50
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
GET /v5/broker/asset/query-sub-member-deposit-record?coin=USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672192441294X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_subaccount_deposit_records(    coin="USDT",    limit=1,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getBrokerSubAccountDeposits({    limit: 50,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1672192441742}
```

---

## Set Margin Mode

**URL:** https://bybit-exchange.github.io/docs/v5/account/set-margin-mode

**Contents:**

- Set Margin Mode
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Set Margin Mode
On this page
Set Margin Mode
Default is regular margin mode
HTTP Request
​
POST
/v5/account/set-margin-mode
Request Parameters
​
Parameter
Required
Type
Comments
setMarginMode
true
string
ISOLATED_MARGIN
,
REGULAR_MARGIN
(i.e. Cross margin),
PORTFOLIO_MARGIN
Response Parameters
​
Parameter
Type
Comments
reasons
array
Object. If requested successfully, it is an empty array
> reasonCode
> string
> Fail reason code
> reasonMsg
> string
> Fail reason msg
> RUN >>
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/account/set-margin-mode
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
> 1672134396332
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> {
"setMarginMode"
:
"PORTFOLIO_MARGIN"
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
> set_margin_mode
(
> setMarginMode
> =
"PORTFOLIO_MARGIN"
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
> setMarginMode
(
'PORTFOLIO_MARGIN'
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
> 3400045
> ,
"retMsg"
:
"Set margin mode failed"
> ,
"result"
:
> {
"reasons"
:
[
{
"reasonCode"
:
"3400000"
,
"reasonMsg"
:
"Equity needs to be equal to or greater than 1000 USDC"
}
]
> }
> }

**Examples:**

Example 1 ():

```
POST /v5/account/set-margin-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672134396332X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "setMarginMode": "PORTFOLIO_MARGIN"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_margin_mode(    setMarginMode="PORTFOLIO_MARGIN",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setMarginMode('PORTFOLIO_MARGIN')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():

```
{    "retCode": 3400045,    "retMsg": "Set margin mode failed",    "result": {        "reasons": [            {                "reasonCode": "3400000",                "reasonMsg": "Equity needs to be equal to or greater than 1000 USDC"            }        ]    }}
```

---

## Get Account Borrowable/Collateralizable Limit

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/acct-borrow-collateral

**Contents:**

- Get Account Borrowable/Collateralizable Limit
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (legacy)
Get Account Borrowable/Collateralizable Limit
On this page
Get Account Borrowable/Collateralizable Limit
Query for the minimum and maximum amounts your account can borrow and how much collateral you can put up.
Permission: "Spot trade"
HTTP Request
​
GET
/v5/crypto-loan/borrowable-collateralisable-number
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
collateralCurrency
true
string
Collateral coin name
Response Parameters
​
Parameter
Type
Comments
collateralCurrency
string
Collateral coin name
loanCurrency
string
Loan coin name
maxCollateralAmount
string
Max. limit to mortgage
maxLoanAmount
string
Max. limit to borrow
minCollateralAmount
string
Min. limit to mortgage
minLoanAmount
string
Min. limit to borrow
Request Example
​
HTTP
Python
Node.js
GET
/v5/crypto-loan/borrowable-collateralisable-number?loanCurrency=USDT&collateralCurrency=BTC
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
1728627083198
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
get_account_borrowable_or_collateralizable_limit
(
loanCurrency
=
"USDT"
,
collateralCurrency
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
getAccountBorrowCollateralLimit
(
{
loanCurrency
:
'USDT'
,
collateralCurrency
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
"request.success"
,
"result"
:
{
"collateralCurrency"
:
"BTC"
,
"loanCurrency"
:
"USDT"
,
"maxCollateralAmount"
:
"164.957732055526752104"
,
"maxLoanAmount"
:
"8000000"
,
"minCollateralAmount"
:
"0.000412394330138818"
,
"minLoanAmount"
:
"20"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1728627084863
}

**Examples:**

Example 1 ():

```
GET /v5/crypto-loan/borrowable-collateralisable-number?loanCurrency=USDT&collateralCurrency=BTC HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728627083198X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_account_borrowable_or_collateralizable_limit(    loanCurrency="USDT",    collateralCurrency="BTC",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getAccountBorrowCollateralLimit({    loanCurrency: 'USDT',    collateralCurrency: 'BTC',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "collateralCurrency": "BTC",        "loanCurrency": "USDT",        "maxCollateralAmount": "164.957732055526752104",        "maxLoanAmount": "8000000",        "minCollateralAmount": "0.000412394330138818",        "minLoanAmount": "20"    },    "retExtInfo": {},    "time": 1728627084863}
```

---

## Get Fee Rate

**URL:** https://bybit-exchange.github.io/docs/v5/account/fee-rate

**Contents:**

- Get Fee Rate
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get Fee Rate
On this page
Get Fee Rate
Get the trading fee rate.
HTTP Request
​
GET
/v5/account/fee-rate
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
, uppercase only. Valid for
linear
,
inverse
,
spot
baseCoin
false
string
Base coin, uppercase only.
SOL
,
BTC
,
ETH
. Valid for
option
Response Parameters
​
Parameter
Type
Comments
category
string
Product type.
spot
,
option
.
Derivatives does not have this field
list
array
Object
> symbol
> string
> Symbol name. Keeps
""
> for Options
> baseCoin
> string
> Base coin.
> SOL
> ,
> BTC
> ,
> ETH
> Derivatives does not have this field
> Keeps
""
> for Spot
> takerFeeRate
> string
> Taker fee rate
> makerFeeRate
> string
> Maker fee rate
> RUN >>
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/account/fee-rate?symbol=ETHUSDT
> HTTP/1.1
> Host
:
> api.bybit.com
> X-BAPI-SIGN
:
> XXXXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1676360412362
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
> get_fee_rates
(
> symbol
> =
"ETHUSDT"
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
> getFeeRate
(
> {
> category
:
'linear'
> ,
> symbol
:
'ETHUSDT'
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
"OK"
> ,
"result"
:
> {
"list"
:
[
{
"symbol"
:
"ETHUSDT"
,
"takerFeeRate"
:
"0.0006"
,
"makerFeeRate"
:
"0.0001"
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
> 1676360412576
> }

**Examples:**

Example 1 ():

```
GET /v5/account/fee-rate?symbol=ETHUSDT HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676360412362X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_fee_rates(    symbol="ETHUSDT",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getFeeRate({        category: 'linear',        symbol: 'ETHUSDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "symbol": "ETHUSDT",                "takerFeeRate": "0.0006",                "makerFeeRate": "0.0001"            }        ]    },    "retExtInfo": {},    "time": 1676360412576}
```

---

## Get Sub UID

**URL:** https://bybit-exchange.github.io/docs/v5/asset/sub-uid-list

**Contents:**

- Get Sub UID
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Asset
Get Sub UID
On this page
Get Sub UID
Query the sub UIDs under a main UID. It returns up to 2000 sub accounts, if you need more, please call this
endpoint
.
info
Query by the master UID's api key
only
HTTP Request
​
GET
/v5/asset/transfer/query-sub-member-list
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
subMemberIds
array
<
string
>
All sub UIDs under the main UID
transferableSubMemberIds
array
<
string
>
All sub UIDs that have universal transfer enabled
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/asset/transfer/query-sub-member-list
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
1672147239931
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
getSubUID
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
"success"
,
"result"
:
{
"subMemberIds"
:
[
"554117"
,
"592324"
,
"592334"
,
"1055262"
,
"1072055"
,
"1119352"
]
,
"transferableSubMemberIds"
:
[
"554117"
,
"592324"
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
1672147241320
}

**Examples:**

Example 1 ():

```
GET /v5/asset/transfer/query-sub-member-list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672147239931X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_sub_uid())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getSubUID()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "subMemberIds": [            "554117",            "592324",            "592334",            "1055262",            "1072055",            "1119352"        ],        "transferableSubMemberIds": [            "554117",            "592324"        ]    },    "retExtInfo": {},    "time": 1672147241320}
```

---

## Get Lending Account Info

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/account-info

**Contents:**

- Get Lending Account Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Abandoned Endpoints
Get Lending Account Info
On this page
Get Lending Account Info
HTTP Request
​
GET
/v5/lending/account
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
Response Parameters
​
Parameter
Type
Comments
coin
string
Coin name
principalInterest
string
User Redeemable interest
principalQty
string
Leftover quantity you can redeem for today (measured from 0 - 24 UTC), formula: min(the rest amount of principle, the
amount that the user can redeem on the day)
principalTotal
string
Total amount redeemable by user
quantity
string
Current deposit quantity
Request Example
​
GET
/v5/lending/account?coin=ETH
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
1682049556563
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
"coin"
:
"BTC"
,
"principalInterest"
:
"0"
,
"principalQty"
:
"1"
,
"principalTotal"
:
"1"
,
"quantity"
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
1682049706988
}

**Examples:**

Example 1 ():

```
GET /v5/lending/account?coin=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1682049556563X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "coin": "BTC",        "principalInterest": "0",        "principalQty": "1",        "principalTotal": "1",        "quantity": "1"    },    "retExtInfo": {},    "time": 1682049706988}
```

---

## Get Collateral Coins

**URL:** https://bybit-exchange.github.io/docs/v5/crypto-loan/collateral-coin

**Contents:**

- Get Collateral Coins
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (legacy)
Get Collateral Coins
On this page
Get Collateral Coins
info
Does not need authentication.
HTTP Request
​
GET
/v5/crypto-loan/collateral-data
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
> array
> Object
>> collateralAccuracy
> > integer
> > Valid collateral coin precision
> > initialLTV
> > string
> > The Initial LTV ratio determines the initial amount of coins that can be borrowed. The initial LTV ratio may vary for
> > different collateral
> > marginCallLTV
> > string
> > If the LTV ratio (Loan Amount/Collateral Amount) reaches the threshold, you will be required to add more collateral to
> > your loan
> > liquidationLTV
> > string
> > If the LTV ratio (Loan Amount/Collateral Amount) reaches the threshold, Bybit will liquidate your collateral assets to
> > repay your loan and interest in full
> > maxLimit
> > string
> > Collateral limit
> vipLevel
> > string
> > VIP level
> > Request Example
> > ​
> > HTTP
> > Python
> > Node.js
> > GET
> > /v5/crypto-loan/collateral-data?currency=ETH&vipLevel=PRO1
> > HTTP/1.1
> > Host
:
> > api.bybit.com
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
)
> > print
(
> > session
> > .
> > get_collateral_coins
(
> > currency
> > =
"ETH"
> > ,
> > vipLevel
> > =
"PRO1"
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
> > getCollateralCoins
(
> > {
> > currency
:
'ETH'
> > ,
> > vipLevel
:
'PRO1'
> > ,
> > }
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
"request.success"
> > ,
"result"
:
> > {
"vipCoinList"
:
[
> > {
"list"
:
[
{
"collateralAccuracy"
:
8
,
"currency"
:
"ETH"
,
"initialLTV"
:
"0.8"
,
"liquidationLTV"
:
"0.95"
,
"marginCallLTV"
:
"0.87"
,
"maxLimit"
:
"32000"
}
]
> > ,
"vipLevel"
:
"PRO1"
> > }
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
> > 1728618590498
> > }

**Examples:**

Example 1 ():

```
GET /v5/crypto-loan/collateral-data?currency=ETH&vipLevel=PRO1 HTTP/1.1Host: api.bybit.com
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_collateral_coins(    currency="ETH",    vipLevel="PRO1",))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getCollateralCoins({    currency: 'ETH',    vipLevel: 'PRO1',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "vipCoinList": [            {                "list": [                    {                        "collateralAccuracy": 8,                        "currency": "ETH",                        "initialLTV": "0.8",                        "liquidationLTV": "0.95",                        "marginCallLTV": "0.87",                        "maxLimit": "32000"                    }                ],                "vipLevel": "PRO1"            }        ]    },    "retExtInfo": {},    "time": 1728618590498}
```

---

## Get Borrow History

**URL:** https://bybit-exchange.github.io/docs/v5/account/borrow-history

**Contents:**

- Get Borrow History
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get Borrow History (2 years)
On this page
Get Borrow History
Get interest records, sorted in reverse order of creation time.
HTTP Request
​
GET
/v5/account/borrow-history
Request Parameters
​
Parameter
Required
Type
Comments
currency
false
string
USDC
,
USDT
,
BTC
,
ETH
etc, uppercase only
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
list
array
Object
> currency
> string
> USDC
> ,
> USDT
> ,
> BTC
> ,
> ETH
> createdTime
> integer
> Created timestamp (ms)
> borrowCost
> string
> Interest
> hourlyBorrowRate
> string
> Hourly Borrow Rate
> InterestBearingBorrowSize
> string
> Interest Bearing Borrow Size
> costExemption
> string
> Cost exemption
> borrowAmount
> string
> Total borrow amount
> unrealisedLoss
> string
> Unrealised loss
> freeBorrowedAmount
> string
> The borrowed amount for interest free
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
> /v5/account/borrow-history?currency=BTC&limit=1
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
> 1672277745427
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
> get_borrow_history
(
> currency
> =
"BTC"
> ,
> limit
> =
> 1
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
> getBorrowHistory
(
> {
> currency
:
'USDT'
> ,
> startTime
:
> 1670601600000
> ,
> endTime
:
> 1673203200000
> ,
> limit
:
> 30
> ,
> cursor
:
'nextPageCursorToken'
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
"OK"
> ,
"result"
:
> {
"nextPageCursor"
:
"2671153%3A1%2C2671153%3A1"
> ,
"list"
:
[
{
"borrowAmount"
:
"1.06333265702840778"
,
"costExemption"
:
"0"
,
"freeBorrowedAmount"
:
"0"
,
"createdTime"
:
1697439900204
,
"InterestBearingBorrowSize"
:
"1.06333265702840778"
,
"currency"
:
"BTC"
,
"unrealisedLoss"
:
"0"
,
"hourlyBorrowRate"
:
"0.000001216904"
,
"borrowCost"
:
"0.00000129"
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
> 1697442206478
> }

**Examples:**

Example 1 ():

```
GET /v5/account/borrow-history?currency=BTC&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672277745427X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_borrow_history(    currency="BTC",    limit=1,))
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getBorrowHistory({    currency: 'USDT',    startTime: 1670601600000,    endTime: 1673203200000,    limit: 30,    cursor: 'nextPageCursorToken',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "2671153%3A1%2C2671153%3A1",        "list": [            {                "borrowAmount": "1.06333265702840778",                "costExemption": "0",                "freeBorrowedAmount": "0",                "createdTime": 1697439900204,                "InterestBearingBorrowSize": "1.06333265702840778",                "currency": "BTC",                "unrealisedLoss": "0",                "hourlyBorrowRate": "0.000001216904",                "borrowCost": "0.00000129"            }        ]    },    "retExtInfo": {},    "time": 1697442206478}
```

---

## Get Account Instruments Info

**URL:** https://bybit-exchange.github.io/docs/v5/account/instrument

**Contents:**

- Get Account Instruments Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get Account Instruments Info
On this page
Get Account Instruments Info
Query for the instrument specification of online trading pairs that available to users.
Covers: Spot / USDT contract / USDC contract / Inverse contract
caution
Spot does not support pagination, so
limit
,
cursor
are invalid.
This endpoint returns 200 entries by default. There are now more than 200
linear
symbols on the platform. As a result, you will need to use
cursor
for pagination or
limit
to get all entries.
Custodial sub-accounts do not support queries.
During periods of extreme market volatility, this interface may experience increased latency or temporary delays in data
delivery
HTTP Request
​
GET
/v5/account/instruments-info
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
200
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
Linear/Inverse
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
> string
> Symbol name
>
contractType
string
Contract type
>
status
string
Instrument status
> baseCoin
> string
> Base coin
> quoteCoin
> string
> Quote coin
>
symbolType
string
the region to which the trading pair belongs
> launchTime
> string
> Launch timestamp (ms)
> deliveryTime
> string
> Delivery timestamp (ms)
> Expired futures delivery time
> Perpetual delisting time
> deliveryFeeRate
> string
> Delivery fee rate
> priceScale
> string
> Price scale
> leverageFilter
> Object
> Leverage attributes
>> minLeverage
> > string
> > Minimum leverage
> > maxLeverage
> > string
> > Maximum leverage
> > leverageStep
> > string
> > The step to increase/reduce leverage
> priceFilter
> > Object
> > Price attributes
> > minPrice
> > string
> > Minimum order price
> > maxPrice
> > string
> > Maximum order price
> > tickSize
> > string
> > The step to increase/reduce order price
> lotSizeFilter
> > Object
> > Size attributes
> > minNotionalValue
> > string
> > Minimum notional value
> > maxOrderQty
> > string
> > Maximum quantity for Limit and PostOnly order
> > maxMktOrderQty
> > string
> > Maximum quantity for Market order
> > minOrderQty
> > string
> > Minimum order quantity
> > qtyStep
> > string
> > The step to increase/reduce order quantity
> > postOnlyMaxOrderQty
> > string
> > deprecated, please use
> > maxOrderQty
> unifiedMarginTrade
> > boolean
> > Whether to support unified margin trade
> fundingInterval
> > integer
> > Funding interval (minute)
> settleCoin
> > string
> > Settle coin
>
copyTrading
string
Copy trade symbol or not
> upperFundingRate
> string
> Upper limit of funding date
> lowerFundingRate
> string
> Lower limit of funding date
> displayName
> string
> The USDC futures & perpetual name displayed in the Web or App
> riskParameters
> object
> Risk parameters for limit order price. Note that the
> formula changed
> in Jan 2025
>> priceLimitRatioX
> > string
> > Ratio X
> > priceLimitRatioY
> > string
> > Ratio Y
> isPreListing
> > boolean
> > Whether the contract is a pre-market contract
> > When the pre-market contract is converted to official contract, it will be false
> preListingInfo
> > object
> > If isPreListing=false, preListingInfo=null
> > If isPreListing=true, preListingInfo is an object
>>
curAuctionPhase
string
The current auction phase
> > phases
> > array<object>
> > Each phase time info
>>>
phase
string
pre-market trading phase
> > > startTime
> > > string
> > > The start time of the phase, timestamp(ms)
> > > endTime
> > > string
> > > The end time of the phase, timestamp(ms)
> > auctionFeeInfo
> > > object
> > > Action fee info
> > > auctionFeeRate
> > > string
> > > The trading fee rate during auction phase
> > > There is no trading fee until entering continues trading phase
> > > takerFeeRate
> > > string
> > > The taker fee rate during continues trading phase
> > > makerFeeRate
> > > string
> > > The maker fee rate during continues trading phase
> > skipCallAuction
> > > boolean
> > > false
> > > ,
> > > true
> > > Whether the pre-market contract skips the call auction phase
> isPublicRpi
> > > boolean
> > > Whether RPI Is Openly Provided to Market Makers or not.
> > > true: RPI Is Openly Provided to Market Makers
> > > false: RPI Is Not Openly Provided to Market Makers
> myRpiPermission
> > > boolean
> > > Whether the Current User Has RPI Permissions or not
> > > true: Has RPI Permissions
> > > false: Does Not Have RPI Permissions
> > > Parameter
> > > Type
> > > Comments
> > > category
> > > string
> > > Product type
> > > list
> > > array
> > > Object
> symbol
> > > string
> > > Symbol name
> baseCoin
> > > string
> > > Base coin
> quoteCoin
> > > string
> > > Quote coin
> innovation
> > > string
> > > deprecated, please use
> > > symbolType
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
Margin trade symbol or not
This is to identify if the symbol support margin trading under different account modes
You may find some symbols not supporting margin buy or margin sell, so you need to go to
Collateral Info (UTA)
to check if that coin is borrowable
> stTag
> string
> Whether or not it has an
> special treatment label
> .
> 0
: false,
> 1
: true
> lotSizeFilter
> Object
> Size attributes
>> basePrecision
> > string
> > The precision of base coin
> > quotePrecision
> > string
> > The precision of quote coin
> > minOrderQty
> > string
> > Minimum order quantity, deprecated, no longer check
> > minOrderQty
> > , check
> > minOrderAmt
> > instead
> > maxOrderQty
> > string
> > Maximum order quantity, deprecated, please refer to
> > maxLimitOrderQty
> > ,
> > maxMarketOrderQty
> > based on order type
> > minOrderAmt
> > string
> > Minimum order amount
> > maxOrderAmt
> > string
> > Maximum order amount, deprecated, no longer check
> > maxOrderAmt
> > , check
> > maxLimitOrderQty
> > and
> > maxMarketOrderQty
> > instead
> > maxLimitOrderQty
> > string
> > Maximum Limit order quantity
> > maxMarketOrderQty
> > string
> > Maximum Market order quantity
> > postOnlyMaxLimitOrderSize
> > string
> > Maximum limit order size for Post-only and RPI orders
> priceFilter
> > Object
> > Price attributes
> > tickSize
> > string
> > The step to increase/reduce order price
> riskParameters
> > Object
> > Risk parameters for limit order price, refer to
> > announcement
> > priceLimitRatioX
> > string
> > Ratio X
> > priceLimitRatioY
> > string
> > Ratio Y
> isPublicRpi
> > boolean
> > Whether RPI Is Openly Provided to Market Makers or not.
> > true: RPI Is Openly Provided to Market Makers
> > false: RPI Is Not Openly Provided to Market Makers
> myRpiPermission
> > boolean
> > Whether the Current User Has RPI Permissions or not
> > true: Has RPI Permissions
> > false: Does Not Have RPI Permissions
> > Request Example
> > ​
> > Linear
> > Spot
> > HTTP
> > GET
> > /v5/account/instruments-info?category=linear&symbol=1000000BABYDOGEUSDT
> > HTTP/1.1
> > Host
:
> > api-testnet.bybit.com
> > HTTP
> > GET
> > /v5/account/instruments-info?category=spot&symbol=BTCUSDT
> > HTTP/1.1
> > Host
:
> > api-testnet.bybit.com
> > Response Example
> > ​
> > Linear
> > Spot
> > // official USDT Perpetual instrument structure
> > {
"retCode"
:
> > 0
> > ,
"retMsg"
:
"OK"
> > ,
"result"
:
> > {
"category"
:
"linear"
> > ,
"list"
:
[
{
"symbol"
:
"1000000BABYDOGEUSDT"
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
"1000000BABYDOGE"
,
"quoteCoin"
:
"USDT"
,
"launchTime"
:
"1718098044000"
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
"7"
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
"25.00"
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
"0.0000001"
,
"maxPrice"
:
"1.9999998"
,
"tickSize"
:
"0.0000001"
}
,
"lotSizeFilter"
:
{
"maxOrderQty"
:
"60000000"
,
"minOrderQty"
:
"100"
,
"qtyStep"
:
"100"
,
"postOnlyMaxOrderQty"
:
"60000000"
,
"maxMktOrderQty"
:
"12000000"
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
240
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
"0.02"
,
"lowerFundingRate"
:
"-0.02"
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
"0.15"
,
"priceLimitRatioY"
:
"0.3"
}
,
"displayName"
:
""
,
"symbolType"
:
"innovation"
,
"myRpiPermission"
:
true
,
"isPublicRpi"
:
true
}
]
> > ,
"nextPageCursor"
:
""
> > }
> > ,
"retExtInfo"
:
> > {
> > }
> > ,
"time"
:
> > 1760510800094
> > }
> > {
"retCode"
:
> > 0
> > ,
"retMsg"
:
"OK"
> > ,
"result"
:
> > {
"category"
:
"spot"
> > ,
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
"0.00000001"
,
"minOrderQty"
:
"0.000001"
,
"maxOrderQty"
:
"17000"
,
"minOrderAmt"
:
"5"
,
"maxOrderAmt"
:
"1999999999"
,
"maxLimitOrderQty"
:
"17000"
,
"maxMarketOrderQty"
:
"8500"
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
"0.01"
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
"0.05"
}
,
"symbolType"
:
""
,
"isPublicRpi"
:
true
,
"myRpiPermission"
:
true
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
> > 1760682563907
> > }

**Examples:**

Example 1 ():

```
GET /v5/account/instruments-info?category=linear&symbol=1000000BABYDOGEUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():

```
GET /v5/account/instruments-info?category=spot&symbol=BTCUSDT HTTP/1.1Host: api-testnet.bybit.com
```

Example 3 ():

```
// official USDT Perpetual instrument structure{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "linear",        "list": [            {                "symbol": "1000000BABYDOGEUSDT",                "contractType": "LinearPerpetual",                "status": "Trading",                "baseCoin": "1000000BABYDOGE",                "quoteCoin": "USDT",                "launchTime": "1718098044000",                "deliveryTime": "0",                "deliveryFeeRate": "",                "priceScale": "7",                "leverageFilter": {                    "minLeverage": "1",                    "maxLeverage": "25.00",                    "leverageStep": "0.01"                },                "priceFilter": {                    "minPrice": "0.0000001",                    "maxPrice": "1.9999998",                    "tickSize": "0.0000001"                },                "lotSizeFilter": {                    "maxOrderQty": "60000000",                    "minOrderQty": "100",                    "qtyStep": "100",                    "postOnlyMaxOrderQty": "60000000",                    "maxMktOrderQty": "12000000",                    "minNotionalValue": "5"                },                "unifiedMarginTrade": true,                "fundingInterval": 240,                "settleCoin": "USDT",                "copyTrading": "none",                "upperFundingRate": "0.02",                "lowerFundingRate": "-0.02",                "isPreListing": false,                "preListingInfo": null,                "riskParameters": {                    "priceLimitRatioX": "0.15",                    "priceLimitRatioY": "0.3"                },                "displayName": "",                "symbolType": "innovation",                "myRpiPermission": true,                "isPublicRpi": true            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1760510800094}
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "spot",        "list": [            {                "symbol": "BTCUSDT",                "baseCoin": "BTC",                "quoteCoin": "USDT",                "innovation": "0",                "status": "Trading",                "marginTrading": "utaOnly",                "stTag": "0",                "lotSizeFilter": {                    "basePrecision": "0.000001",                    "quotePrecision": "0.00000001",                    "minOrderQty": "0.000001",                    "maxOrderQty": "17000",                    "minOrderAmt": "5",                    "maxOrderAmt": "1999999999",                    "maxLimitOrderQty": "17000",                    "maxMarketOrderQty": "8500",                    "postOnlyMaxLimitOrderSize":"60000"                },                "priceFilter": {                    "tickSize": "0.01"                },                "riskParameters": {                    "priceLimitRatioX": "0.05",                    "priceLimitRatioY": "0.05"                },                "symbolType": "",                "isPublicRpi": true,                "myRpiPermission": true            }        ]    },    "retExtInfo": {},    "time": 1760682563907}
```

---

## Get Account Info

**URL:** https://bybit-exchange.github.io/docs/v5/account/account-info

**Contents:**

- Get Account Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Get Account Info
On this page
Get Account Info
Query the account information, like margin mode, account mode, etc.
HTTP Request
​
GET
/v5/account/info
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
unifiedMarginStatus
integer
Account status
marginMode
string
ISOLATED_MARGIN
,
REGULAR_MARGIN
,
PORTFOLIO_MARGIN
isMasterTrader
boolean
Whether this account is a leader (copytrading).
true
,
false
spotHedgingStatus
string
Whether the unified account enables Spot hedging.
ON
,
OFF
updatedTime
string
Account data updated timestamp (ms)
dcpStatus
string
deprecated, always
OFF
. Please use
Get DCP Info
timeWindow
integer
deprecated, always
0
. Please use
Get DCP Info
smpGroup
integer
deprecated, always
0
. Please query
Get SMP Group ID
endpoint
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/account/info
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
1672129307221
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
get_account_info
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
getAccountInfo
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
"marginMode"
:
"REGULAR_MARGIN"
,
"updatedTime"
:
"1697078946000"
,
"unifiedMarginStatus"
:
4
,
"dcpStatus"
:
"OFF"
,
"timeWindow"
:
10
,
"smpGroup"
:
0
,
"isMasterTrader"
:
false
,
"spotHedgingStatus"
:
"OFF"
}
}

**Examples:**

Example 1 ():

```
GET /v5/account/info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672129307221X-BAPI-RECV-WINDOW: 5000
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_account_info())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .getAccountInfo()    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "marginMode": "REGULAR_MARGIN",        "updatedTime": "1697078946000",        "unifiedMarginStatus": 4,        "dcpStatus": "OFF",        "timeWindow": 10,        "smpGroup": 0,        "isMasterTrader": false,        "spotHedgingStatus": "OFF"    }}
```

---

## Repay Liability

**URL:** https://bybit-exchange.github.io/docs/v5/account/repay-liability

**Contents:**

- Repay Liability
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Account
Repay Liability
On this page
Repay Liability
You can manually repay the liabilities of Unified account
Permission
: USDC Contracts
info
BYUSDT will not be used for repayment.
MNT will temporarily not be used for repayment, and repaying MNT liabilities through convert-repay is not supported.
However, you may still use
Manual Repay Without Asset Conversion
to repay MNT using your existing balance.
HTTP Request
​
POST
/v5/account/quick-repayment
Request Parameters
​
Parameter
Required
Type
Comments
coin
false
string
The coin with liability, uppercase only
Input the specific coin: repay the liability of this coin in particular
No coin specified: repay the liability of all coins
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> coin
> string
> Coin used for repayment
> The order of currencies used to repay liability is based on
> liquidationOrder
> from
> this endpoint
> repaymentQty
> string
> Repayment qty
> Request Example
> ​
> HTTP
> Python
> Node.js
> POST
> /v5/account/quick-repayment
> HTTP/1.1
> Host
:
> api-testnet.bybit.com
> X-BAPI-SIGN
:
> XXXXXX
> X-BAPI-API-KEY
:
> xxxxxxxxxxxxxxxxxx
> X-BAPI-TIMESTAMP
:
> 1701848610019
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
> Content-Length
:
> 22
> {
"coin"
:
"USDT"
> }
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
> repayLiability
(
> {
> coin
:
'USDT'
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
"SUCCESS"
> ,
"result"
:
> {
"list"
:
[
{
"coin"
:
"BTC"
,
"repaymentQty"
:
"0.10549670"
}
,
{
"coin"
:
"ETH"
,
"repaymentQty"
:
"2.27768114"
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
> 1701848610941
> }

**Examples:**

Example 1 ():

```
POST /v5/account/quick-repayment HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1701848610019X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 22{    "coin": "USDT"}
```

Example 2 ():

```

```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .repayLiability({    coin: 'USDT',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "SUCCESS",    "result": {        "list": [            {                "coin": "BTC",                "repaymentQty": "0.10549670"            },            {                "coin": "ETH",                "repaymentQty": "2.27768114"            }        ]    },    "retExtInfo": {},    "time": 1701848610941}
```

---

## Adjust Collateral Amount

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/adjust-collateral

**Contents:**

- Adjust Collateral Amount
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (New)
Adjust Collateral Amount
On this page
Adjust Collateral Amount
You can increase or reduce your collateral amount. When you reduce, please obey the
Get Max. Allowed Collateral Reduction Amount
Permission: "Spot trade"
UID rate limit: 1 req / second
info
The adjusted collateral amount will be returned to or deducted from the Funding wallet.
HTTP Request
​
POST
/v5/crypto-loan-common/adjust-ltv
Request Parameters
​
Parameter
Required
Type
Comments
currency
true
string
Collateral coin
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
long
Collateral adjustment transaction ID
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-common/adjust-ltv
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
1752627997649
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
69
{
"currency"
:
"BTC"
,
"amount"
:
"0.08"
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
adjust_collateral_amount_new_crypto_loan
(
currency
=
"BTC"
,
amount
=
"0.08"
,
direction
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
"ok"
,
"result"
:
{
"adjustId"
:
27511
}
,
"retExtInfo"
:
{
}
,
"time"
:
1752627997915
}

**Examples:**

Example 1 ():

```
POST /v5/crypto-loan-common/adjust-ltv HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752627997649X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 69{    "currency": "BTC",    "amount": "0.08",    "direction": "1"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.adjust_collateral_amount_new_crypto_loan(    currency="BTC",    amount="0.08",    direction="1",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "adjustId": 27511    },    "retExtInfo": {},    "time": 1752627997915}
```

---

## Collateral Repayment

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/flexible/repay-collateral

**Contents:**

- Collateral Repayment
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (New)
Flexible Loan
Collateral Repayment
On this page
Collateral Repayment
Permission: "Spot trade"
UID rate limit: 1 req / second
info
Pay interest first, then repay the principal.
HTTP Request
​
POST
/v5/crypto-loan-flexible/repay-collateral
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
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/crypto-loan-flexible/repay-collateral
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
"USDT"
,
"amount"
:
"500"
,
"collateralCoin"
:
"BTC"
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
collateral_repayment_flexible_crypto_loan
(
loanCurrency
=
"USDT"
,
amount
=
"500"
,
collateralCoin
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
}
,
"retExtInfo"
:
{
}
,
"time"
:
1756971550401
}

**Examples:**

Example 1 ():

```
POST /v5/crypto-loan-flexible/repay-collateral HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1752569628364X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52{  "loanCurrency": "USDT",  "amount": "500",  "collateralCoin":"BTC"}
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.collateral_repayment_flexible_crypto_loan(    loanCurrency="USDT",    amount="500",    collateralCoin="BTC",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {},    "retExtInfo": {},    "time": 1756971550401}
```

---

## Get Account Info

**URL:** https://bybit-exchange.github.io/docs/v5/broker/exchange-broker/account-info

**Contents:**

- Get Account Info
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Broker
Exchange Broker
Get Account Info
On this page
Get Account Info
info
Use exchange broker master account to query
API rate limit: 10 req / sec
HTTP Request
​
GET
/v5/broker/account-info
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
subAcctQty
string
The qty of sub account has been created
maxSubAcctQty
string
The max limit of sub account can be created
baseFeeRebateRate
Object
Rebate percentage of the base fee
> spot
> string
> Rebate percentage of the base fee for spot, e.g., 10.00%
> derivatives
> string
> Rebate percentage of the base fee for derivatives, e.g., 10.00%
> markupFeeRebateRate
> Object
> Rebate percentage of the mark up fee
> spot
> string
> Rebate percentage of the mark up fee for spot, e.g., 10.00%
> derivatives
> string
> Rebate percentage of the mark up fee for derivatives, e.g., 10.00%
> convert
> string
> Rebate percentage of the mark up fee for convert, e.g., 10.00%
> ts
> string
> System timestamp (ms)
> Request Example
> ​
> HTTP
> Python
> Node.js
> GET
> /v5/broker/account-info
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
> 1701399431920
> X-BAPI-RECV-WINDOW
:
> 5000
> Content-Type
:
> application/json
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
> get_exchange_broker_account_info
(
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
> getExchangeBrokerAccountInfo
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
"subAcctQty"
:
"2"
> ,
"maxSubAcctQty"
:
"20"
> ,
"baseFeeRebateRate"
:
> {
"spot"
:
"10.0%"
> ,
"derivatives"
:
"10.0%"
> }
> ,
"markupFeeRebateRate"
:
> {
"spot"
:
"6.00%"
> ,
"derivatives"
:
"9.00%"
> ,
"convert"
:
"3.00%"
> ,
> }
> ,
"ts"
:
"1701395633402"
> }
> ,
"retExtInfo"
:
> {
> }
> ,
"time"
:
> 1701395633403
> }

**Examples:**

Example 1 ():

```
GET /v5/broker/account-info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1701399431920X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_exchange_broker_account_info())
```

Example 3 ():

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getExchangeBrokerAccountInfo()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "subAcctQty": "2",        "maxSubAcctQty": "20",        "baseFeeRebateRate": {            "spot": "10.0%",            "derivatives": "10.0%"        },        "markupFeeRebateRate": {            "spot": "6.00%",            "derivatives": "9.00%",            "convert": "3.00%",        },        "ts": "1701395633402"    },    "retExtInfo": {},    "time": 1701395633403}
```

---

## Get Collateral Coins

**URL:** https://bybit-exchange.github.io/docs/v5/new-crypto-loan/collateral-coin

**Contents:**

- Get Collateral Coins
    - HTTP Request​
    - Request Parameters​
    - Response Parameters​
    - Request Example​
    - Response Example​

Crypto Loan (New)
Get Collateral Coins
On this page
Get Collateral Coins
info
Does not need authentication.
HTTP Request
​
GET
/v5/crypto-loan-common/collateral-data
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
collateralRatioConfigList
array
Object
> collateralRatioList
> array
> Object
>> collateralRatio
> > string
> > Collateral ratio
> > maxValue
> > string
> > Max qty
> > minValue
> > string
> > Min qty
> currencies
> > string
> > Currenies with the same collateral ratio, e.g.,
> > BTC,ETH,XRP
> > currencyLiquidationList
> > array
> > Object
> currency
> > string
> > Coin name
> liquidationOrder
> > integer
> > Liquidation order
> > Request Example
> > ​
> > HTTP
> > Python
> > Node.js
> > GET
> > /v5/crypto-loan-common/collateral-data?currency=BTC
> > HTTP/1.1
> > Host
:
> > api-testnet.bybit.com
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
)
> > print
(
> > session
> > .
> > get_collateral_coins_new_crypto_loan
(
> > currency
> > =
"BTC"
> > ,
> > amount
> > =
"0.08"
> > ,
> > direction
> > =
"1"
> > ,
)
)
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
"collateralRatioConfigList"
:
[
> > {
"collateralRatioList"
:
[
{
"collateralRatio"
:
"0.8"
,
"maxValue"
:
"10000"
,
"minValue"
:
"0"
}
,
{
"collateralRatio"
:
"0.7"
,
"maxValue"
:
"20000"
,
"minValue"
:
"10000"
}
,
{
"collateralRatio"
:
"0.5"
,
"maxValue"
:
"30000"
,
"minValue"
:
"20000"
}
,
{
"collateralRatio"
:
"0.4"
,
"maxValue"
:
"99999999999"
,
"minValue"
:
"30000"
}
]
> > ,
"currencies"
:
"ATOM,AAVE,BTC,BOB"
> > }
]
> > ,
"currencyLiquidationList"
:
[
{
"currency"
:
"BTC"
,
"liquidationOrder"
:
1
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
> > 1752627381571
> > }

**Examples:**

Example 1 ():

```
GET /v5/crypto-loan-common/collateral-data?currency=BTC HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,)print(session.get_collateral_coins_new_crypto_loan(    currency="BTC",    amount="0.08",    direction="1",))
```

Example 3 ():

```

```

Example 4 ():

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "collateralRatioConfigList": [            {                "collateralRatioList": [                    {                        "collateralRatio": "0.8",                        "maxValue": "10000",                        "minValue": "0"                    },                    {                        "collateralRatio": "0.7",                        "maxValue": "20000",                        "minValue": "10000"                    },                    {                        "collateralRatio": "0.5",                        "maxValue": "30000",                        "minValue": "20000"                    },                    {                        "collateralRatio": "0.4",                        "maxValue": "99999999999",                        "minValue": "30000"                    }                ],                "currencies": "ATOM,AAVE,BTC,BOB"            }        ],        "currencyLiquidationList": [            {                "currency": "BTC",                "liquidationOrder": 1            }        ]    },    "retExtInfo": {},    "time": 1752627381571}
```

---
