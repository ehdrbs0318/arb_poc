# Bybit-V5-Api - Rfq Trading

**Pages:** 3

---

## Quote

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/websocket/private/quote

**Contents:**
- Quote
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

RFQ Trading
WebSocket Stream
Private
Quote
On this page
Quote
Obtain the quote information sent or received by the user themselves. Whenever the user sends or receives a quote themselves, the data will be pushed.
Topic:
rfq.open.quotes
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
int
Data created timestamp (ms)
data
array
Object
> rfqId
string
Inquiry ID
> rfqLinkId
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
> quoteId
string
Quote ID
> quoteLinkId
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the quote was created
> expiresAt
string
The quote's expiration time (ms)
> deskCode
string
The unique identification code of the quote party, which is not visible when anonymous is set to
true
during quotation
> status
string
Status of quote:
Active
,
Canceled
,
PendingFill
,
Filled
,
Expired
,
Failed
>execQuoteSide
string
Execute the quote direction,
buy
or
sell
. When the quote direction is 'buy', for maker, the execution direction is the same as the direction in legs, and opposite for taker. Conversely, the same applies
> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
> quoteBuyList
array of objects
Quote buy direction
>> category
string
Product type:
spot
,
linear
,
option
>> symbol
string
symbol name
>> price
string
Quote price
>> qty
string
Quantity
> quoteSellList
array of objects
Quote sell direction
>> category
string
Product type:
spot
,
linear
,
option
>> symbol
string
symbol name
>> price
string
Quote price
>> qty
string
Quantity
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
"rfq.open.quotes"
]
}
Stream Example
​
{
"topic"
:
"rfq.open.quotes"
,
"creationTime"
:
1757578449562
,
"data"
:
[
{
"rfqLinkId"
:
""
,
"rfqId"
:
"1757578410512325974246073709371267"
,
"quoteId"
:
"1757578449553042047579782748460520"
,
"quoteLinkId"
:
""
,
"expiresAt"
:
"1757578509556"
,
"status"
:
"Active"
,
"deskCode"
:
"test0904"
,
"execQuoteSide"
:
""
,
"quoteBuyList"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"95800"
,
"qty"
:
"1"
}
]
,
"quoteSellList"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"price"
:
"95000"
,
"qty"
:
"1"
}
]
,
"createdAt"
:
"1757578449556"
,
"updatedAt"
:
"1757578449556"
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "rfq.open.quotes"    ]}
```

Example 2 ():
```
{  "topic": "rfq.open.quotes",  "creationTime": 1757578449562,  "data": [    {      "rfqLinkId": "",      "rfqId": "1757578410512325974246073709371267",      "quoteId": "1757578449553042047579782748460520",      "quoteLinkId": "",      "expiresAt": "1757578509556",      "status": "Active",      "deskCode": "test0904",      "execQuoteSide": "",      "quoteBuyList": [        {          "category": "linear",          "symbol": "BTCUSDT",          "price": "95800",          "qty": "1"        }      ],      "quoteSellList": [        {          "category": "linear",          "symbol": "BTCUSDT",          "price": "95000",          "qty": "1"        }      ],      "createdAt": "1757578449556",      "updatedAt": "1757578449556"    }  ]}
```

---

## RFQ

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/websocket/private/inquiry

**Contents:**
- RFQ
  - Response Parameters​
  - Subscribe Example​
  - Stream Example​

RFQ Trading
WebSocket Stream
Private
RFQ
On this page
RFQ
Obtain the inquiries (requests for quotes) information sent or received by the user themselves. Whenever the user sends or receives an inquiry themselves, the data will be pushed.
Topic:
rfq.open.rfqs
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
int
Data created timestamp (ms)
data
array of objects
RFQ data: Return and obtain real-time inquiry information Open consistent
> rfqId
string
Inquiry ID
> rfqLinkId
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
>counterparties
Array of strings
List of bidders
> expiresAt
string
The quote's expiration time (ms)
> strategyType
string
Inquiry label
> status
string
Status of the inquiry form:
Active
,
Canceled
,
PendingFill
,
Filled
,
Expired
,
Failed
> acceptOtherQuoteStatus
string
Whether to accept non-LP quotes. The default value is
false
:
false
: Default value, do not accept non-LP quotes.
true
: Accept non-LP quotes
> deskCode
string
The unique identification code of the inquiring party, which is not visible when anonymous was set to
true
when the RFQ was created
> createdAt
string
Time (ms) when the trade is created in epoch, such as 1650380963
> updatedAt
string
Time (ms) when the trade is updated in epoch, such as 1650380964
> legs
array of objects
Combination transaction
>> category
string
Category. Valid values include:
linear
,
option
and
spot
>> symbol
string
symbol name
>> side
string
Inquiry direction. Valid values are
buy
and
sell
>> qty
string
Order quantity of the instrument
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
"rfq.open.rfqs"
]
}
Stream Example
​
{
"topic"
:
"rfq.open.rfqs"
,
"creationTime"
:
1757482013792
,
"data"
:
[
{
"rfqLinkId"
:
""
,
"rfqId"
:
"1757482013783362721227613524547439"
,
"counterparties"
:
[
"test0904"
]
,
"strategyType"
:
"custom"
,
"expiresAt"
:
"1757482613784"
,
"status"
:
"Active"
,
"acceptOtherQuoteStatus"
:
"false"
,
"deskCode"
:
"1nu9d1"
,
"createdAt"
:
"1757482013784"
,
"updatedAt"
:
"1757482013784"
,
"legs"
:
[
{
"category"
:
"linear"
,
"symbol"
:
"BTCUSDT"
,
"side"
:
"Buy"
,
"qty"
:
"5"
}
]
}
]
}

**Examples:**

Example 1 ():
```
{    "op": "subscribe",    "args": [        "rfq.open.rfqs"    ]}
```

Example 2 ():
```
{  "topic": "rfq.open.rfqs",  "creationTime": 1757482013792,  "data": [    {      "rfqLinkId": "",      "rfqId": "1757482013783362721227613524547439",      "counterparties": [        "test0904"      ],      "strategyType": "custom",      "expiresAt": "1757482613784",      "status": "Active",      "acceptOtherQuoteStatus":"false",      "deskCode": "1nu9d1",      "createdAt": "1757482013784",      "updatedAt": "1757482013784",      "legs": [        {          "category": "linear",          "symbol": "BTCUSDT",          "side": "Buy",          "qty": "5"        }      ]    }  ]}
```

---

## Basic Workflow

**URL:** https://bybit-exchange.github.io/docs/v5/rfq/basic-workflow

**Contents:**
- Basic Workflow

RFQ Trading
Basic Workflow
Basic Workflow
Basic concepts
Request for Quote (RFQ) – an inquiry sent by the inquiring party to the quoting party. The request for a quote includes one or more products and quantities that the inquiring party wishes to trade.
Quote – provided in response to the inquiry. Sent by the quoting party to the inquiring party.
Transaction – when the inquirer accepts and executes the quote.
Basic workflow
The inquirier creates an RFQ and sends it to the quoters of their choice.
Different quoting parties send quotes in response to this inquiry.
The inquiring party chooses to execute the best quote to generate the transaction. The transaction executes and is settled.
The inquiring party and the quoting party receive confirmation of the execution.
The transaction details are published on the public market data channel (excluding party information).
Creating an RFQ from the inquirer's perspective
The inquirer uses
/v5/rfq/create-rfq
to create an inquiry. The inquirer can query the information of the products with
/v5/market/instruments-info
, and the quoter information can be queried with
/v5/rfq/config query
.
The inquirer may cancel the inquiry with
/v5/rfq/cancel-rfq
at any time while the inquiry is in force.
The inquirer can use the endpoint
/v5/rfq/accept-other-quote
to accept non-LP OTC quotes, thereby expanding the sources of quotations.
The quoting party, if it is one of the quoting parties selected by the inquiry party, will receive the inquiry information in the
rfq.open.rfqs
WebSocket topic and can make the corresponding quote.
The inquirer, after receiving the offer information in the
rfq.open.quotes
WebSocket topic, can choose the best offer and execute it through the
/v5/rfq/execute-quote
.
Inquirers will receive confirmation of successful trade execution in the
rfq.open.trades
and
rfq.open.rfqs
WebSocket topics.
Inquirers will also receive confirmation of this and other block trades
rfq.open.public.trades
WebSocket topic.
Creating a quote from the quoter's perspective
When a new request for a quote is issued and the quoting party is one of the selected quoting parties, the quoting party will receive this request information in the
rfq.open.rfqs
WebSocket topic.
The quoting party creates a quote and sends it via
/v5/rfq/create-quote
.
Quoters can cancel a valid quote at will with
/v5/rfq/cancel-quote
.
The inquiring party chooses to execute the optimal quote.
Quoters receive status updates on their quotes via the
rfq.open.quotes
WebSocket topic.
Quoters will receive confirmation of the successful execution of their quote on the
rfq.open.trades
and
rfq.open.quotes
WebSocket topics.
The quoting party will also receive confirmation of this transaction and other block trades in the
rfq.open.public.trades
WebSocket topic.

---
