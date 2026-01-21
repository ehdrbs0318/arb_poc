# Bybit-V5-Api - User Management

**Pages:** 3

---

## Get Affiliate User List

**URL:** https://bybit-exchange.github.io/docs/v5/affiliate/affiliate-user-list

**Contents:**
- Get Affiliate User List
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Affiliate
Get Affiliate User List
On this page
Get Affiliate User List
To use this endpoint, you should have an affiliate account and only tick "affiliate" permission while creating the API key.
Affiliate site:
https://affiliates.bybit.com
tip
Use master UID only
The api key can only have "Affiliate" permission
HTTP Request
​
GET
/v5/affiliate/aff-user-list
Request Parameters
​
Parameter
Required
Type
Comments
size
false
integer
Limit for data size per page.
[
0
,
1000
]
. Default:
0
cursor
false
string
Cursor. Use the
nextPageCursor
token from the response to retrieve the next page of the result set
needDeposit
false
boolean
true
: return deposit info;
false
(default): does not return deposit info
need30
false
boolean
true
: return 30 days trading info;
false
(default): does not return 30 days trading info
need365
false
boolean
true
: return 365 days trading info;
false
(default): does not return 365 days trading info
startDate
false
string
Start date of the query period, format
YYYY-MM-DD
endDate
false
string
End date of the query period, format
YYYY-MM-DD
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> userId
string
user Id
> registerTime
string
user register time
> source
string
user registration source, from which referrer code
> remarks
string
The remark
> isKyc
boolean
Whether KYC is completed
> takerVol30Day
string
Taker volume in last 30 days (USDT), update at T + 1. All volume related attributes below includes Derivatives, Option, Spot volume
> makerVol30Day
string
Maker volume in last 30 days (USDT), update at T + 1
> tradeVol30Day
string
Total trading volume in last 30 days (USDT), update at T + 1
> depositAmount30Day
string
Deposit amount in last 30 days (USDT)
> takerVol365Day
string
Taker volume in the past year (USDT), update at T + 1
> makerVol365Day
string
Maker volume in the past year (USDT), update at T + 1
> tradeVol365Day
string
Total trading volume in the past year (USDT), update at T + 1
> depositAmount365Day
string
Total deposit amount in the past year (USDT)
> takerVol
string
Taker volume in
[
startDate
,
endDate
]
(USDT), update at T + 1, includes Derivatives, Option, Spot volume
> makerVol
string
Maker volume in
[
startDate
,
endDate
]
(USDT), update at T + 1, includes Derivatives, Option, Spot volume
> tradeVol
string
Total trading volume in
[
startDate
,
endDate
]
(USDT), update at T + 1, includes Derivatives, Option, Spot volume
> startDate
string
Start date of the query period
> endDate
string
End date of the query period
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
/v5/affiliate/aff-user-list?cursor=0&size=2&need365=true&need30=true&needDeposit=true&startDate=2025-10-21&endDate=2025-10-22
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
get_affiliate_user_list
(
cursor
=
"0"
,
size
=
"2"
,
need365
=
True
,
need30
=
True
,
needDeposit
=
True
,
startDate
=
"2025-10-21"
,
endDate
=
"2025-10-22"
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
getAffiliateUserInfo
(
{
size
:
2
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
"list"
:
[
{
"userId"
:
"1001699821"
,
"registerTime"
:
"0001-01-01"
,
"source"
:
"aff_14650_10087"
,
"remarks"
:
"front_hub_robot"
,
"isKyc"
:
false
,
"takerVol30Day"
:
""
,
"makerVol30Day"
:
""
,
"tradeVol30Day"
:
""
,
"depositAmount30Day"
:
""
,
"takerVol365Day"
:
""
,
"makerVol365Day"
:
""
,
"tradeVol365Day"
:
""
,
"depositAmount365Day"
:
""
,
"takerVol"
:
""
,
"makerVol"
:
""
,
"tradeVol"
:
""
,
"startDate"
:
"2025-09-21"
,
"endDate"
:
"2025-10-21"
}
,
{
"userId"
:
"1001625535"
,
"registerTime"
:
"0001-01-01"
,
"source"
:
"aff_14650_10087"
,
"remarks"
:
"front_hub_robot"
,
"isKyc"
:
false
,
"takerVol30Day"
:
""
,
"makerVol30Day"
:
""
,
"tradeVol30Day"
:
""
,
"depositAmount30Day"
:
""
,
"takerVol365Day"
:
""
,
"makerVol365Day"
:
""
,
"tradeVol365Day"
:
""
,
"depositAmount365Day"
:
""
,
"takerVol"
:
""
,
"makerVol"
:
""
,
"tradeVol"
:
""
,
"startDate"
:
"2025-09-21"
,
"endDate"
:
"2025-10-21"
}
]
,
"nextPageCursor"
:
"16197"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1733205472513
}

**Examples:**

Example 1 ():
```
GET /v5/affiliate/aff-user-list?cursor=0&size=2&need365=true&need30=true&needDeposit=true&startDate=2025-10-21&endDate=2025-10-22 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1685596324209X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: xxxxxxContent-Type: application/json
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_affiliate_user_list(    cursor="0",    size="2",    need365=True,    need30=True,    needDeposit=True,    startDate="2025-10-21",    endDate="2025-10-22",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getAffiliateUserInfo({ size: 2 })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "userId": "1001699821",                "registerTime": "0001-01-01",                "source": "aff_14650_10087",                "remarks": "front_hub_robot",                "isKyc": false,                "takerVol30Day": "",                "makerVol30Day": "",                "tradeVol30Day": "",                "depositAmount30Day": "",                "takerVol365Day": "",                "makerVol365Day": "",                "tradeVol365Day": "",                "depositAmount365Day": "",                "takerVol": "",                "makerVol": "",                "tradeVol": "",                "startDate": "2025-09-21",                "endDate": "2025-10-21"            },            {                "userId": "1001625535",                "registerTime": "0001-01-01",                "source": "aff_14650_10087",                "remarks": "front_hub_robot",                "isKyc": false,                "takerVol30Day": "",                "makerVol30Day": "",                "tradeVol30Day": "",                "depositAmount30Day": "",                "takerVol365Day": "",                "makerVol365Day": "",                "tradeVol365Day": "",                "depositAmount365Day": "",                "takerVol": "",                "makerVol": "",                "tradeVol": "",                "startDate": "2025-09-21",                "endDate": "2025-10-21"            }        ],        "nextPageCursor": "16197"    },    "retExtInfo": {},    "time": 1733205472513}
```

---

## Get UID Wallet Type

**URL:** https://bybit-exchange.github.io/docs/v5/user/wallet-type

**Contents:**
- Get UID Wallet Type
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

User
Get UID Wallet Type
On this page
Get UID Wallet Type
Get available wallet types for the master account or sub account
tip
Master api key: you can get master account and appointed sub account available wallet types, and support up to 200 sub UID in one request.
Sub api key: you can get its own available wallet types
HTTP Request
​
GET
/v5/user/get-member-type
Request Parameters
​
Parameter
Required
Type
Comments
memberIds
false
string
Query itself wallet types when not passed
When use master api key to query sub UID, master UID data is always returned in the top of the array
Multiple sub UID are supported, separated by commas
This param is ignored when you use sub account api key
Response Parameters
​
Parameter
Type
Comments
accounts
array
Object
> uid
string
Master/Sub user Id
>
accountType
array
Wallets array.
FUND
,
UNIFIED
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/user/get-member-type
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
1686884973961
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
// https://api.bybit.com/v5/user/get-member-type
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
getUIDWalletType
(
{
memberIds
:
'subUID1,subUID2'
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
"accounts"
:
[
{
"uid"
:
"533285"
,
"accountType"
:
[
"UNIFIED"
,
"FUND"
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
1686884974151
}

**Examples:**

Example 1 ():
```
GET /v5/user/get-member-type HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1686884973961X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```

```

Example 3 ():
```
// https://api.bybit.com/v5/user/get-member-typeconst { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getUIDWalletType({    memberIds: 'subUID1,subUID2',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "accounts": [            {                "uid": "533285",                "accountType": [                    "UNIFIED",                    "FUND"                ]            }        ]    },    "retExtInfo": {},    "time": 1686884974151}
```

---

## Get API Key Information

**URL:** https://bybit-exchange.github.io/docs/v5/user/apikey-info

**Contents:**
- Get API Key Information
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

User
Get API Key Information
On this page
Get API Key Information
Get the information of the api key. Use the api key pending to be checked to call the endpoint. Both
master and sub user's api key
are applicable.
tip
Any permission can access this endpoint.
HTTP Request
​
GET
/v5/user/query-api
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
id
string
Unique ID. Internal use
note
string
The remark
apiKey
string
Api key
readOnly
integer
0
: Read and Write.
1
: Read only
secret
string
Always
""
permissions
Object
The types of permission
> ContractTrade
array
Permission of contract trade
Order
,
Position
> Spot
array
Permission of spot
SpotTrade
> Wallet
array
Permission of wallet
AccountTransfer
,
SubMemberTransfer
(master account),
SubMemberTransferList
(sub account),
Withdraw
(master account)
> Options
array
Permission of USDC Contract. It supports trade option and USDC perpetual.
OptionsTrade
> Derivatives
array
DerivativesTrade
> Exchange
array
Permission of convert
ExchangeHistory
> Earn
array
Permission of earn product
Earn
> FiatP2P
array
Permission of P2P
FiatP2POrder
,
Advertising
. Not applicable to subaccount, always
[]
> FiatBybitPay
array
Permission of Bybit Pay
FaitPayOrder
. Not applicable to subaccount, always
[]
> FiatConvertBroker
array
Permission of fiat convert
FiatConvertBrokerOrder
. Not applicable to subaccount, always
[]
> BlockTrade
array
Permission of blocktrade. Not applicable to subaccount, always
[]
> Affiliate
array
Permission of Affiliate. Only affiliate can have this permission, otherwise always
[]
> NFT
array
Deprecated
, always
[]
> CopyTrading
array
Deprecated
, always
[]
ips
array
IP bound
type
integer
The type of api key.
1
: personal,
2
: connected to the third-party app
deadlineDay
integer
The remaining valid days of api key. Only for those api key with no IP bound or the password has been changed
expiredAt
datetime
The expiry day of the api key. Only for those api key with no IP bound or the password has been changed
createdAt
datetime
The create day of the api key
unified
integer
Deprecated
field
uta
integer
Whether the account to which the account upgrade to unified trade account.
0
: regular account;
1
: unified trade account
userID
integer
User ID
inviterID
integer
Inviter ID (the UID of the account which invited this account to the platform)
vipLevel
string
VIP Level
mktMakerLevel
string
Market maker level
affiliateID
integer
Affiliate Id.
0
represents that there is no binding relationship.
rsaPublicKey
string
Rsa public key
isMaster
boolean
If this api key belongs to master account or not
parentUid
string
The main account uid. Returns
"0"
when the endpoint is called by main account
kycLevel
string
Personal account kyc level.
LEVEL_DEFAULT
,
LEVEL_1
,
LEVEL_2
kycRegion
string
Personal account kyc region
RUN >>
Request Example
​
HTTP
Python
Node.js
GET
/v5/user/query-api
HTTP/1.1
Host
:
api.bybit.com
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
get_api_key_information
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
getQueryApiKey
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
"id"
:
"13770661"
,
"note"
:
"readwrite api key"
,
"apiKey"
:
"XXXXXX"
,
"readOnly"
:
0
,
"secret"
:
""
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
"Derivatives"
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
"ips"
:
[
"*"
]
,
"type"
:
1
,
"deadlineDay"
:
66
,
"expiredAt"
:
"2023-12-22T07:20:25Z"
,
"createdAt"
:
"2022-10-16T02:24:40Z"
,
"unified"
:
0
,
"uta"
:
0
,
"userID"
:
24617703
,
"inviterID"
:
0
,
"vipLevel"
:
"No VIP"
,
"mktMakerLevel"
:
"0"
,
"affiliateID"
:
0
,
"rsaPublicKey"
:
""
,
"isMaster"
:
true
,
"parentUid"
:
"0"
,
"kycLevel"
:
"LEVEL_DEFAULT"
,
"kycRegion"
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
1697525990798
}

**Examples:**

Example 1 ():
```
GET /v5/user/query-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_api_key_information())
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getQueryApiKey()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "13770661",        "note": "readwrite api key",        "apiKey": "XXXXXX",        "readOnly": 0,        "secret": "",        "permissions": {            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "Derivatives": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": [],            "Affiliate": [],            "Earn": []        },        "ips": [            "*"        ],        "type": 1,        "deadlineDay": 66,        "expiredAt": "2023-12-22T07:20:25Z",        "createdAt": "2022-10-16T02:24:40Z",        "unified": 0,        "uta": 0,        "userID": 24617703,        "inviterID": 0,        "vipLevel": "No VIP",        "mktMakerLevel": "0",        "affiliateID": 0,        "rsaPublicKey": "",        "isMaster": true,        "parentUid": "0",        "kycLevel": "LEVEL_DEFAULT",        "kycRegion": ""    },    "retExtInfo": {},    "time": 1697525990798}
```

---
