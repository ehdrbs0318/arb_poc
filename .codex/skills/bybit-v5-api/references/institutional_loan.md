# Bybit-V5-Api - Institutional Loan

**Pages:** 2

---

## Get Product Info

**URL:** https://bybit-exchange.github.io/docs/v5/otc/margin-product-info

**Contents:**
- Get Product Info
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Institutional Loan
Get Product Info
On this page
Get Product Info
tip
When queried without an API key, this endpoint returns public product data
If your UID is bound with an OTC loan, then you can get your private product data by calling with your API key
If your UID is not bound with an OTC loan but you passed your API key, this endpoint returns public product data
HTTP Request
​
GET
/v5/ins-loan/product-infos
Request Parameters
​
Parameter
Required
Type
Comments
productId
false
string
Product ID. If not passed, returns all products
Response Parameters
​
Parameter
Type
Comments
marginProductInfo
array
Object
> productId
string
Product ID
> leverage
string
The maximum leverage for this loan product
> supportSpot
integer
Whether or not Spot is supported. 0:false; 1:true
> supportContract
integer
Whether USDT Perpetuals are supported. 0:false; 1:true
> supportMarginTrading
integer
Whether or not Spot margin trading is supported. 0:false; 1:true
> deferredLiquidationLine
string
Line for deferred liquidation
> deferredLiquidationTime
string
Time for deferred liquidation
> withdrawLine
string
Restrict line for withdrawal
> transferLine
string
Restrict line for transfer
> spotBuyLine
string
Restrict line for Spot buy
> spotSellLine
string
Restrict line for Spot trading
> contractOpenLine
string
Restrict line for USDT Perpetual open position
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
The whitelist of spot trading pairs
If
supportSpot
="0", then it returns "[]"
If empty array, then you can trade any symbols
If not empty, then you can only trade listed symbols
> contractSymbols
array
The whitelist of contract trading pairs
If
supportContract
="0", then it returns "[]"
If empty array, then you can trade any symbols
If not empty, then you can only trade listed symbols
> supportUSDCContract
integer
Whether or not USDC contracts are supported.
'0'
:false;
'1'
:true
> supportUSDCOptions
integer
Whether or not Options are supported.
'0'
:false;
'1'
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
Restrict line to trade USDT Perpetual
> USDCContractCloseLine
string
Restrict line to trade USDC Contract
> USDCOptionsCloseLine
string
Restrict line to trade Option
> USDCContractSymbols
array
The whitelist of USDC contract trading pairs
If
supportContract
="0", then it returns "[]"
If no whitelist symbols, it is
[]
, and you can trade any
If supportUSDCContract="0", it is
[]
> USDCOptionsSymbols
array
The whitelist of Option symbols
If
supportContract
="0", then it returns "[]"
If no whitelisted, it is
[]
, and you can trade any
If supportUSDCOptions="0", it is
[]
> marginLeverage
string
The allowable maximum leverage for Spot margin trading. If
supportMarginTrading
=0, then it returns ""
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
/v5/ins-loan/product-infos?productId=91
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
get_product_info
(
productId
=
"91"
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
getInstitutionalLendingProductInfo
(
{
productId
:
'91'
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
"marginProductInfo"
:
[
{
"productId"
:
"91"
,
"leverage"
:
"4.00000000"
,
"supportSpot"
:
1
,
"supportContract"
:
0
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
""
,
"spotSellLine"
:
""
,
"contractOpenLine"
:
""
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
"0"
,
"transferRatio"
:
"0"
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
0
,
"supportUSDCOptions"
:
0
,
"USDTPerpetualOpenLine"
:
""
,
"USDCContractOpenLine"
:
""
,
"USDCOptionsOpenLine"
:
""
,
"USDTPerpetualCloseLine"
:
""
,
"USDCContractCloseLine"
:
""
,
"USDCOptionsCloseLine"
:
""
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
"marginLeverage"
:
"0"
,
"USDTPerpetualLeverage"
:
[
]
,
"USDCContractLeverage"
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
1689747746332
}

**Examples:**

Example 1 ():
```
GET /v5/ins-loan/product-infos?productId=91 HTTP/1.1Host: api-testnet.bybit.com
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_product_info(productId="91"))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getInstitutionalLendingProductInfo({    productId: '91',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "marginProductInfo": [            {                "productId": "91",                "leverage": "4.00000000",                "supportSpot": 1,                "supportContract": 0,                "withdrawLine": "",                "transferLine": "",                "spotBuyLine": "",                "spotSellLine": "",                "contractOpenLine": "",                "liquidationLine": "0.75",                "stopLiquidationLine": "0.35000000",                "contractLeverage": "0",                "transferRatio": "0",                "spotSymbols": [],                "contractSymbols": [],                "supportUSDCContract": 0,                "supportUSDCOptions": 0,                "USDTPerpetualOpenLine": "",                "USDCContractOpenLine": "",                "USDCOptionsOpenLine": "",                "USDTPerpetualCloseLine": "",                "USDCContractCloseLine": "",                "USDCOptionsCloseLine": "",                "USDCContractSymbols": [],                "USDCOptionsSymbols": [],                "marginLeverage": "0",                "USDTPerpetualLeverage": [],                "USDCContractLeverage": [],                "deferredLiquidationLine":"",                "deferredLiquidationTime":"",            }        ]    },    "retExtInfo": {},    "time": 1689747746332}
```

---

## Bind Or Unbind UID

**URL:** https://bybit-exchange.github.io/docs/v5/otc/bind-uid

**Contents:**
- Bind Or Unbind UID
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Institutional Loan
Bind Or Unbind UID
On this page
Bind Or Unbind UID
For the institutional loan product, you can bind new UIDs to the risk unit or unbind UID from the risk unit.
info
The risk unit designated UID cannot be unbound.
The UID you want to bind must be upgraded to UTA Pro.
HTTP Request
​
POST
/v5/ins-loan/association-uid
Request Parameters
​
Parameter
Required
Type
Comments
uid
true
string
UID
Bind
a) the key used must be from one of UIDs in the risk unit;
b) input UID must not have an INS loan
Unbind
a) the key used must be from one of UIDs in the risk unit;
b) input UID cannot be the same as the UID used to access the API
operate
true
string
0
: bind,
1
: unbind
Response Parameters
​
Parameter
Type
Comments
uid
string
UID
operate
string
0
: bind,
1
: unbind
Request Example
​
HTTP
Python
Node.js
POST
/v5/ins-loan/association-uid
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1699257853101
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
43
{
"uid"
:
"592324"
,
"operate"
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
bind_or_unbind_uid
(
uid
=
"592324"
,
operate
=
"0"
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
bindOrUnbindUID
(
{
uid
:
'yourUID'
,
operate
:
'0'
,
// 0 for bind, 1 for unbind
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
"uid"
:
"592324"
,
"operate"
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
1699257746135
}

**Examples:**

Example 1 ():
```
POST /v5/ins-loan/association-uid HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1699257853101X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/jsonContent-Length: 43{    "uid": "592324",    "operate": "0"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.bind_or_unbind_uid(uid="592324", operate="0"))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .bindOrUnbindUID({    uid: 'yourUID',    operate: '0', // 0 for bind, 1 for unbind  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "uid": "592324",        "operate": "0"    },    "retExtInfo": {},    "time": 1699257746135}
```

---
