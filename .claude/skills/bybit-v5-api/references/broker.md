# Bybit-V5-Api - Broker

**Pages:** 6

---

## Issue Voucher

**URL:** https://bybit-exchange.github.io/docs/v5/broker/reward/issue-voucher

**Contents:**
- Issue Voucher
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Broker
Exchange Broker
Reward
Issue Voucher
On this page
Issue Voucher
HTTP Request
​
POST
/v5/broker/award/distribute-award
Request Parameters
​
Parameter
Required
Type
Comments
accountId
true
string
User ID
awardId
true
string
Voucher ID
specCode
true
string
Customised unique spec code, up to 8 characters
amount
true
string
Issue amount
Spot airdrop supports up to 16 decimals
Other types supports up to 4 decimals
brokerId
true
string
Broker ID
Response Parameters
​
None
Request Example
​
HTTP
Python
Node.js
POST
/v5/broker/award/distribute-award
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
1726110531734
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
128
{
"accountId"
:
"2846381"
,
"awardId"
:
"123456"
,
"specCode"
:
"award-001"
,
"amount"
:
"100"
,
"brokerId"
:
"v-28478"
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
issue_voucher
(
accountId
=
"2846381"
,
awardId
=
"123456"
,
specCode
=
"award-001"
,
amount
=
"100"
,
brokerId
=
"v-28478"
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
issueBrokerVoucher
(
{
accountId
:
'2846381'
,
awardId
:
'123456'
,
specCode
:
'award-001'
,
amount
:
'100'
,
brokerId
:
'v-28478'
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
}

**Examples:**

Example 1 ():
```
POST /v5/broker/award/distribute-award HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1726110531734X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 128{    "accountId": "2846381",    "awardId": "123456",    "specCode": "award-001",    "amount": "100",    "brokerId": "v-28478"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.issue_voucher(    accountId="2846381",    awardId="123456",    specCode="award-001",    amount="100",    brokerId="v-28478",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .issueBrokerVoucher({    accountId: '2846381',    awardId: '123456',    specCode: 'award-001',    amount: '100',    brokerId: 'v-28478',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": ""}
```

---

## Application Process

**URL:** https://bybit-exchange.github.io/docs/v5/broker/api-broker/guidance

**Contents:**
- Application Process
- 1. Information Submission​
- 2. Merchant Initialization​
- API Integration​
  - 1. Construct Authorization Page​
  - 2. Authorization Success Callback​
  - 3. Obtain Access Token​
    - Request Example​
    - Response Example​
  - 4. Obtain OpenAPI​

Broker
API Broker
OAuth Integration Guidance
On this page
Application Process
1. Information Submission
​
Submit the following information to Bybit Business via this Email:
broker_program@bybit.com
:
Bybit UID
: Used to log in to the OAuth management backend.
OpenAPI Whitelist IP
: Only applicable to OpenAPI; the OAuth management backend has no IP restrictions.
2. Merchant Initialization
​
Log in to Bybit
using the corresponding UID.
Access the OAuth Admin Portal
:
Visit
https://www.bybit.com/app/user/oauth-admin
Configure
Application Name
,
Email
, upload
logo
, etc.
Core Parameter
redirect_uri
:
Multiple callback addresses can be configured.
The
redirect_uri
passed when invoking the page must be configured in the management backend.
If the passed value does not match the configuration, it defaults to the first address.
After Successful Application
:
You will receive
client_id
and
client_secret
.
Important
: Securely store this information and do not share it with others.
API Integration
​
1. Construct Authorization Page
​
https://www.bybit.com/en/oauth?client_id
=
{
client_id
}
&
response_type
=
code
&
redirect_uri
=
{
redirect_uri
}
&
scope
=
openapi
&
state
=
{
state
}
Parameter
Description
client_id
Obtained after merchant initialization.
response_type
Fixed value:
code
.
scope
Pass
openapi
; other values require confirmation with Bybit.
state
Random string.
redirect_uri
The address to redirect to after user authorization; must be configured in the management backend.
2. Authorization Success Callback
​
After the user confirms authorization, the page redirects (301) to
redirect_uri
with the parameter
code
.
Example
:
If
redirect_uri = https://www.example.com/callback
, the callback URL will be:
https://www.example.com/callback/?response_type
=
code
&
code
=
sSn87036PCFub1g0FGigexSjT
&
scope
=
openapi
&
state
=
1234abc
Parameter
Description
code
Core parameter; used by the merchant backend to obtain
access_token
.
3. Obtain Access Token
​
URL
:
https://api2.bybit.com/oauth/v1/public/access_token
Method
:
POST
Request Example
​
curl
-v -X POST
{
url
}
\
-H
'Content-Type: application/x-www-form-urlencoded'
\
-H
'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36'
\
-d
'client_id={client_id}'
\
-d
'client_secret={client_secret}'
\
-d
'code={code}'
# Note: Code can only be used once.
Response Example
​
{
"access_token"
:
"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3NjcwODM5NDEsIkNsaWVudElEIjoiQThmMzNFeEVTeEhjIiwiR3JhbnRNZW1iZXJJRCI6MTA2MzEwNzQxLCJBcHByb3ZlZFNjb3BlIjpbIm9wZW5hcGkiXSwiTm9uY2UiOiJPNmZ0QkdTYVdEIn0.Vq46cxPIzKmWz5fFwU4fQuF-IDqFJDOIelNLnH8r2Oo"
,
"refresh_token"
:
"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3Njk1ODk1NDEsIkNsaWVudElEIjoiQThmMzNFeEVTeEhjIiwiR3JhbnRNZW1iZXJJRCI6MTA2MzEwNzQxLCJBcHByb3ZlZFNjb3BlIjpbIm9wZW5hcGkiXSwiTm9uY2UiOiIwaVZMWVY3Z1pGIn0.ByGH8d5XtSQnkbxeyiXd56iJUTddBWjqFK8_EcAw48w"
,
"token_type"
:
"bearer"
,
"expires_in"
:
86400
,
"refresh_token_expires_in"
:
2592000
}
4. Obtain OpenAPI
​
URL
:
https://api2.bybit.com/oauth/v1/resource/restrict/openapi
Method
:
GET
Authorization
: Include the
Authorization
header formatted as
"Bearer {access_token}"
.
Example
: If
access_token = "12345"
, then
Authorization = "Bearer 12345"
.
Request Example
​
curl
{
url
}
\
-H
"Authorization: Bearer {access_token}"
Response Example
​
{
"ret_code"
:
0
,
"ret_msg"
:
"success"
,
"result"
:
{
"api_key"
:
"xxxxxxx"
,
"api_secret"
:
"xxxxx"
}
}
Notes
​
The
code
parameter from the authorization callback is single-use and expires quickly.
Store
client_secret
and
api_secret
securely and never expose them publicly.

**Examples:**

Example 1 ():
```
https://www.bybit.com/en/oauth?client_id={client_id}&response_type=code&redirect_uri={redirect_uri}&scope=openapi&state={state}
```

Example 2 ():
```
https://www.example.com/callback/?response_type=code&code=sSn87036PCFub1g0FGigexSjT&scope=openapi&state=1234abc
```

Example 3 ():
```
curl -v -X POST {url} \  -H 'Content-Type: application/x-www-form-urlencoded' \  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36' \  -d 'client_id={client_id}' \  -d 'client_secret={client_secret}' \  -d 'code={code}'    # Note: Code can only be used once.
```

Example 4 ():
```
{    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3NjcwODM5NDEsIkNsaWVudElEIjoiQThmMzNFeEVTeEhjIiwiR3JhbnRNZW1iZXJJRCI6MTA2MzEwNzQxLCJBcHByb3ZlZFNjb3BlIjpbIm9wZW5hcGkiXSwiTm9uY2UiOiJPNmZ0QkdTYVdEIn0.Vq46cxPIzKmWz5fFwU4fQuF-IDqFJDOIelNLnH8r2Oo",    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3Njk1ODk1NDEsIkNsaWVudElEIjoiQThmMzNFeEVTeEhjIiwiR3JhbnRNZW1iZXJJRCI6MTA2MzEwNzQxLCJBcHByb3ZlZFNjb3BlIjpbIm9wZW5hcGkiXSwiTm9uY2UiOiIwaVZMWVY3Z1pGIn0.ByGH8d5XtSQnkbxeyiXd56iJUTddBWjqFK8_EcAw48w",    "token_type": "bearer",    "expires_in": 86400,    "refresh_token_expires_in": 2592000}
```

---

## Get Issued Voucher

**URL:** https://bybit-exchange.github.io/docs/v5/broker/reward/get-issue-voucher

**Contents:**
- Get Issued Voucher
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Broker
Exchange Broker
Reward
Get Issued Voucher
On this page
Get Issued Voucher
HTTP Request
​
POST
/v5/broker/award/distribution-record
Request Parameters
​
Parameter
Required
Type
Comments
accountId
true
string
User ID
awardId
true
string
Voucher ID
specCode
true
string
Customised unique spec code, up to 8 characters
withUsedAmount
false
boolean
Whether or not to return the amount used by the user
true
false
(default)
Response Parameters
​
Parameter
Type
Comments
accountId
string
User ID
awardId
string
Voucher ID
specCode
string
Spec code
amount
string
Amount of voucher
isClaimed
boolean
true
,
false
startAt
string
Claim start timestamp (sec)
endAt
string
Claim end timestamp (sec)
effectiveAt
string
Voucher effective timestamp (sec) after claimed
ineffectiveAt
string
Voucher inactive timestamp (sec) after claimed
usedAmount
string
Amount used by the user
Request Example
​
HTTP
Python
Node.js
POST
/v5/broker/award/distribution-record
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
1726112099846
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
111
{
"accountId"
:
"5714139"
,
"awardId"
:
"189528"
,
"specCode"
:
"demo000"
,
"withUsedAmount"
:
false
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
get_issued_voucher
(
id
=
"80209"
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
getBrokerIssuedVoucher
(
{
id
:
'80209'
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
"accountId"
:
"5714139"
,
"awardId"
:
"189528"
,
"specCode"
:
"demo000"
,
"amount"
:
"1"
,
"isClaimed"
:
true
,
"startAt"
:
"1725926400"
,
"endAt"
:
"1733788800"
,
"effectiveAt"
:
"1726531200"
,
"ineffectiveAt"
:
"1733817600"
,
"usedAmount"
:
""
,
}
}

**Examples:**

Example 1 ():
```
POST /v5/broker/award/distribution-record HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1726112099846X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 111{    "accountId": "5714139",    "awardId": "189528",    "specCode": "demo000",    "withUsedAmount": false}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_issued_voucher(    id="80209",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getBrokerIssuedVoucher({    id: '80209',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "accountId": "5714139",        "awardId": "189528",        "specCode": "demo000",        "amount": "1",        "isClaimed": true,        "startAt": "1725926400",        "endAt": "1733788800",        "effectiveAt": "1726531200",        "ineffectiveAt": "1733817600",        "usedAmount": "",    }}
```

---

## Get Voucher Spec

**URL:** https://bybit-exchange.github.io/docs/v5/broker/reward/voucher

**Contents:**
- Get Voucher Spec
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Broker
Exchange Broker
Reward
Get Voucher Spec
On this page
Get Voucher Spec
HTTP Request
​
POST
/v5/broker/award/info
Request Parameters
​
Parameter
Required
Type
Comments
id
true
string
Voucher ID
Response Parameters
​
Parameter
Type
Comments
id
string
Voucher ID
coin
string
Coin
amountUnit
string
AWARD_AMOUNT_UNIT_USD
AWARD_AMOUNT_UNIT_COIN
productLine
string
Product line
subProductLine
string
Sub product line
totalAmount
Object
Total amount of voucher
usedAmount
string
Used amount of voucher
Request Example
​
HTTP
Python
Node.js
POST
/v5/broker/award/info
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
1726107086048
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
22
{
"id"
:
"80209"
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
get_voucher_spec
(
id
=
"80209"
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
getBrokerVoucherSpec
(
{
accountId
:
'5714139'
,
awardId
:
'189528'
,
specCode
:
'demo000'
,
withUsedAmount
:
false
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
"id"
:
"80209"
,
"coin"
:
"USDT"
,
"amountUnit"
:
"AWARD_AMOUNT_UNIT_USD"
,
"productLine"
:
"PRODUCT_LINE_CONTRACT"
,
"subProductLine"
:
"SUB_PRODUCT_LINE_CONTRACT_DEFAULT"
,
"totalAmount"
:
"10000"
,
"usedAmount"
:
"100"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1726107086313
}

**Examples:**

Example 1 ():
```
POST /v5/broker/award/info HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1726107086048X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 22{    "id": "80209"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_voucher_spec(    id="80209",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getBrokerVoucherSpec({    accountId: '5714139',    awardId: '189528',    specCode: 'demo000',    withUsedAmount: false,})  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "80209",        "coin": "USDT",        "amountUnit": "AWARD_AMOUNT_UNIT_USD",        "productLine": "PRODUCT_LINE_CONTRACT",        "subProductLine": "SUB_PRODUCT_LINE_CONTRACT_DEFAULT",        "totalAmount": "10000",        "usedAmount": "100"    },    "retExtInfo": {},    "time": 1726107086313}
```

---

## Get Earning

**URL:** https://bybit-exchange.github.io/docs/v5/broker/exchange-broker/exchange-earning

**Contents:**
- Get Earning
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Broker
Exchange Broker
Get Earning
On this page
Get Earning
info
Use exchange broker master account to query
The data can support up to past 1 months until T-1. To extract data from over a month ago, please contact your Relationship Manager
begin
&
end
are either entered at the same time or not entered, and latest 7 days data are returned by default
API rate limit: 10 req / sec
HTTP Request
​
GET
/v5/broker/earnings-info
Request Parameters
​
Parameter
Required
Type
Comments
bizType
false
string
Business type.
SPOT
,
DERIVATIVES
,
OPTIONS
,
CONVERT
begin
false
string
Begin date, in the format of YYYYMMDD, e.g, 20231201, search the data from 1st Dec 2023 00:00:00 UTC (include)
end
false
string
End date, in the format of YYYYMMDD, e.g, 20231201, search the data before 2nd Dec 2023 00:00:00 UTC (exclude)
uid
false
string
To get results for a specific subaccount: Enter the subaccount UID
To get results for all subaccounts: Leave the field empty
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
1000
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
totalEarningCat
Object
Category statistics for total earning data
> spot
array
Object. Earning for Spot trading. If do not have any rebate, keep empty array
>> coin
string
Rebate coin name
>> earning
string
Rebate amount of the coin
> derivatives
array
Object. Earning for Derivatives trading. If do not have any rebate, keep empty array
>> coin
string
Rebate coin name
>> earning
string
Rebate amount of the coin
> options
array
Object. Earning for Option trading. If do not have any rebate, keep empty array
>> coin
string
Rebate coin name
>> earning
string
Rebate amount of the coin
> convert
array
Object. Earning for Convert trading. If do not have any rebate, keep empty array
>> coin
string
Rebate coin name
>> earning
string
Rebate amount of the coin
> total
array
Object. Sum earnings of all categories. If do not have any rebate, keep empty array
>> coin
string
Rebate coin name
>> earning
string
Rebate amount of the coin
details
array
Object. Detailed trading information for each sub UID and each category
> userId
string
Sub UID
> bizType
string
Business type.
SPOT
,
DERIVATIVES
,
OPTIONS
,
CONVERT
> symbol
string
Symbol name
> coin
string
Rebate coin name
> earning
string
Rebate amount
> markupEarning
string
Earning generated from markup fee rate
> baseFeeEarning
string
Earning generated from base fee rate
> orderId
string
Order ID
> execId
string
Trade ID
> execTime
string
Order execution timestamp (ms)
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
/v5/broker/earnings-info?begin=20231129&end=20231129&uid=117894077
HTTP/1.1
Host
:
api-testnet.bybit.com
X-BAPI-API-KEY
:
xxxxxxxxxxxxxxxxxx
X-BAPI-TIMESTAMP
:
1701399431920
X-BAPI-RECV-WINDOW
:
5000
X-BAPI-SIGN
:
32d2aa1bc205ddfb89849b85e2a8b7e23b1f8f69fe95d6f2cb9c87562f9086a6
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
get_exchange_broker_earnings
(
begin
=
"20231129"
,
end
=
"20231129"
,
uid
=
"117894077"
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
getExchangeBrokerEarnings
(
{
bizType
:
'SPOT'
,
begin
:
'20231201'
,
end
:
'20231207'
,
limit
:
1000
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
"totalEarningCat"
:
{
"spot"
:
[
]
,
"derivatives"
:
[
{
"coin"
:
"USDT"
,
"earning"
:
"0.00027844"
}
]
,
"options"
:
[
]
,
"total"
:
[
{
"coin"
:
"USDT"
,
"earning"
:
"0.00027844"
}
]
}
,
"details"
:
[
{
"userId"
:
"117894077"
,
"bizType"
:
"DERIVATIVES"
,
"symbol"
:
"DOGEUSDT"
,
"coin"
:
"USDT"
,
"earning"
:
"0.00016166"
,
"markupEarning"
:
"0.000032332"
,
"baseFeeEarning"
:
"0.000129328"
,
"orderId"
:
"ec2132f2-a7e0-4a0c-9219-9f3cbcd8e878"
,
"execId"
:
"c8f418a0-2ccc-594f-ae72-effedf24d0c4"
,
"execTime"
:
"1701275846033"
}
,
{
"userId"
:
"117894077"
,
"bizType"
:
"DERIVATIVES"
,
"symbol"
:
"TRXUSDT"
,
"coin"
:
"USDT"
,
"earning"
:
"0.00011678"
,
"markupEarning"
:
"0.000023356"
,
"baseFeeEarning"
:
"0.000093424"
,
"orderId"
:
"28b29c2b-ba14-450e-9ce7-3cee0c1fa6da"
,
"execId"
:
"632c7705-7f3a-5350-b69c-d41a8b3d0697"
,
"execTime"
:
"1701245285017"
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
1701398193964
}

**Examples:**

Example 1 ():
```
GET /v5/broker/earnings-info?begin=20231129&end=20231129&uid=117894077 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1701399431920X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: 32d2aa1bc205ddfb89849b85e2a8b7e23b1f8f69fe95d6f2cb9c87562f9086a6Content-Type: application/json
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.get_exchange_broker_earnings(    begin="20231129",    end="20231129",    uid="117894077",))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .getExchangeBrokerEarnings({    bizType: 'SPOT',    begin: '20231201',    end: '20231207',    limit: 1000,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "totalEarningCat": {            "spot": [],            "derivatives": [                {                    "coin": "USDT",                    "earning": "0.00027844"                }            ],            "options": [],            "total": [                {                    "coin": "USDT",                    "earning": "0.00027844"                }            ]        },        "details": [            {                "userId": "117894077",                "bizType": "DERIVATIVES",                "symbol": "DOGEUSDT",                "coin": "USDT",                "earning": "0.00016166",                "markupEarning": "0.000032332",                "baseFeeEarning": "0.000129328",                "orderId": "ec2132f2-a7e0-4a0c-9219-9f3cbcd8e878",                "execId": "c8f418a0-2ccc-594f-ae72-effedf24d0c4",                "execTime": "1701275846033"            },            {                "userId": "117894077",                "bizType": "DERIVATIVES",                "symbol": "TRXUSDT",                "coin": "USDT",                "earning": "0.00011678",                "markupEarning": "0.000023356",                "baseFeeEarning": "0.000093424",                "orderId": "28b29c2b-ba14-450e-9ce7-3cee0c1fa6da",                "execId": "632c7705-7f3a-5350-b69c-d41a8b3d0697",                "execTime": "1701245285017"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1701398193964}
```

---

## Get Broker Earning

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/earning

**Contents:**
- Get Broker Earning
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Get Broker Earning
On this page
Get Broker Earning
danger
This endpoint has been deprecated, please move to new
Get Exchange Broker Earning
info
Use exchange broker master account to query
The data can support up to past 6 months until T-1
startTime
&
endTime
are either entered at the same time or not entered
HTTP Request
​
GET
/v5/broker/earning-record
Request Parameters
​
Parameter
Required
Type
Comments
bizType
false
string
Business type.
SPOT
,
DERIVATIVES
,
OPTIONS
startTime
false
integer
The start timestamp(ms)
endTime
false
integer
The end timestamp(ms)
limit
false
integer
Limit for data size per page.
[
1
,
1000
]
. Default:
1000
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
> userId
string
UID
> bizType
string
Business type
> symbol
string
Symbol name
> coin
string
Coin name. The currency of earning
> earning
string
Commission
> orderId
string
Order ID
> execTime
string
Execution timestamp (ms)
nextPageCursor
string
Refer to the
cursor
request parameter
Request Example
​
HTTP
Python
GET
/v5/broker/earning-record?bizType=SPOT&startTime=1686240000000&endTime=1686326400000&limit=1
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
1686708862669
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
"success"
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
"xxxx"
,
"bizType"
:
"SPOT"
,
"symbol"
:
"BTCUSDT"
,
"coin"
:
"BTC"
,
"earning"
:
"0.000015"
,
"orderId"
:
"1531607271849858304"
,
"execTime"
:
"1686306035957"
}
]
,
"nextPageCursor"
:
"0%2C1"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1686708863283
}

**Examples:**

Example 1 ():
```
GET /v5/broker/earning-record?bizType=SPOT&startTime=1686240000000&endTime=1686326400000&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1686708862669X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

Example 2 ():
```

```

Example 3 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "userId": "xxxx",                "bizType": "SPOT",                "symbol": "BTCUSDT",                "coin": "BTC",                "earning": "0.000015",                "orderId": "1531607271849858304",                "execTime": "1686306035957"            }        ],        "nextPageCursor": "0%2C1"    },    "retExtInfo": {},    "time": 1686708863283}
```

---
