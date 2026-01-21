# Bybit-V5-Api - Reference

**Pages:** 11

---

## Enums Definitions

**URL:** https://bybit-exchange.github.io/docs/v5/enum

**Contents:**
- Enums Definitions
  - locale​
  - announcementType​
  - announcementTag​
  - category​
  - orderStatus​
  - timeInForce​
  - createType​
  - execType​
  - orderType​

Enums Definitions
On this page
Enums Definitions
locale
​
de-DE
en-US
es-AR
es-ES
es-MX
fr-FR
kk-KZ
id-ID
uk-UA
ja-JP
ru-RU
th-TH
pt-BR
tr-TR
vi-VN
zh-TW
ar-SA
hi-IN
fil-PH
announcementType
​
new_crypto
latest_bybit_news
delistings
latest_activities
product_updates
maintenance_updates
new_fiat_listings
other
announcementTag
​
Spot
Derivatives
Spot Listings
BTC
ETH
Trading Bots
USDC
Leveraged Tokens
USDT
Margin Trading
Partnerships
Launchpad
Upgrades
ByVotes
Delistings
VIP
Futures
Institutions
Options
WEB3
Copy Trading
Earn
Bybit Savings
Dual Asset
Liquidity Mining
Shark Fin
Launchpool
NFT     GrabPic
Buy Crypto
P2P Trading
Fiat Deposit
Crypto Deposit
Спот
Спот лістинги
Торгові боти
Токени з кредитним плечем
Маржинальна торгівля
Партнерство
Оновлення
Делістинги
Ф'ючерси
Опціони
Копітрейдинг
Bybit Накопичення
Бівалютні інвестиції
Майнінг ліквідності
Купівля криптовалюти
P2P торгівля
Фіатні депозити
Криптодепозити
Копитрейдинг
Торговые боты
Деривативы
P2P
Спот листинги
Деривативи
MT4
Lucky Draw
Unified Trading Account
Єдиний торговий акаунт
Единый торговый аккаунт
Институциональный трейдинг
Інституціональний трейдинг
Делистинг
category
​
spot
linear
USDT perpetual, USDT Futures and USDC contract, including USDC perp, USDC futures
inverse
Inverse contract, including Inverse perp, Inverse futures
option
orderStatus
​
open status
New
order has been placed successfully
PartiallyFilled
Untriggered
Conditional orders are created
closed status
Rejected
PartiallyFilledCanceled
Only spot has this order status
Filled
Cancelled
In derivatives, orders with this status may have an executed qty
Triggered
instantaneous state for conditional orders from Untriggered to New
Deactivated
UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered
timeInForce
​
GTC
GoodTillCancel
IOC
ImmediateOrCancel
FOK
FillOrKill
PostOnly
RPI
features:
Exclusive Matching
: Only match non-algorithmic users; no execution against orders from Open API.
Post-Only Mechanism
: Act as maker orders, adding liquidity
Lower Priority
: Execute after non-RPI orders at the same price level.
Limited Access
: Initially for select market makers across multiple pairs.
Order Book Updates
: Excluded from API but displayed on the GUI.
createType
​
CreateByUser
CreateByFutureSpread
Spread order
CreateByAdminClosing
CreateBySettle
USDC Futures delivery; position closed as a result of the delisting of a contract. This is recorded as a
trade
but not an
order
.
CreateByStopOrder
Futures conditional order
CreateByTakeProfit
Futures take profit order
CreateByPartialTakeProfit
Futures partial take profit order
CreateByStopLoss
Futures stop loss order
CreateByPartialStopLoss
Futures partial stop loss order
CreateByTrailingStop
Futures trailing stop order
CreateByTrailingProfit
Futures trailing take profit order
CreateByLiq
Laddered liquidation to reduce the required maintenance margin
CreateByTakeOver_PassThrough
If the position is still subject to liquidation (i.e., does not meet the required maintenance margin level), the position shall be taken over by the liquidation engine and closed at the bankruptcy price.
CreateByAdl_PassThrough
Auto-Deleveraging(ADL)
CreateByBlock_PassThrough
Order placed via Paradigm
CreateByBlockTradeMovePosition_PassThrough
Order created by move position
CreateByClosing
The close order placed via web or app position area - web/app
CreateByFGridBot
Order created via grid bot - web/app
CloseByFGridBot
Order closed via grid bot - web/app
CreateByTWAP
Order created by TWAP - web/app
CreateByTVSignal
Order created by TV webhook - web/app
CreateByMmRateClose
Order created by Mm rate close function - web/app
CreateByMartingaleBot
Order created by Martingale bot - web/app
CloseByMartingaleBot
Order closed by Martingale bot - web/app
CreateByIceBerg
Order created by Ice berg strategy - web/app
CreateByArbitrage
Order created by arbitrage - web/app
CreateByDdh
Option dynamic delta hedge order - web/app
CreateByBboOrder
BBO order
execType
​
Trade
AdlTrade
Auto-Deleveraging
Funding
Funding fee
BustTrade
Takeover liquidation
Delivery
USDC futures delivery; Position closed by contract delisted
Settle
Inverse futures settlement; Position closed due to delisting
BlockTrade
MovePosition
FutureSpread
Spread leg execution
UNKNOWN
May be returned by a classic account. Cannot query by this type
orderType
​
Market
Limit
UNKNOWN
is not a valid request parameter value. Is only used in some responses. Mainly, it is used when
execType
is
Funding
.
stopOrderType
​
TakeProfit
StopLoss
TrailingStop
Stop
PartialTakeProfit
PartialStopLoss
tpslOrder
spot TP/SL order
OcoOrder
spot Oco order
MmRateClose
On web or app can set MMR to close position
BidirectionalTpslOrder
Spot bidirectional tpsl order
tickDirection
​
PlusTick
price rise
ZeroPlusTick
trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
MinusTick
price drop
ZeroMinusTick
trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it
interval
​
1
3
5
15
30
60
120
240
360
720
minute
D
day
W
week
M
month
intervalTime
​
5min
15min
30min
minute
1h
4h
hour
1d
day
positionIdx
​
0
one-way mode position
1
Buy side of hedge-mode position
2
Sell side of hedge-mode position
positionStatus
​
Normal
Liq
in the liquidation progress
Adl
in the auto-deleverage progress
rejectReason
​
EC_NoError
EC_Others
EC_UnknownMessageType
EC_MissingClOrdID
EC_MissingOrigClOrdID
EC_ClOrdIDOrigClOrdIDAreTheSame
EC_DuplicatedClOrdID
EC_OrigClOrdIDDoesNotExist
EC_TooLateToCancel
EC_UnknownOrderType
EC_UnknownSide
EC_UnknownTimeInForce
EC_WronglyRouted
EC_MarketOrderPriceIsNotZero
EC_LimitOrderInvalidPrice
EC_NoEnoughQtyToFill
EC_NoImmediateQtyToFill
a maker could not be found to fill your order
EC_PerCancelRequest
EC_MarketOrderCannotBePostOnly
EC_PostOnlyWillTakeLiquidity
EC_CancelReplaceOrder
EC_InvalidSymbolStatus
EC_CancelForNoFullFill
EC_BySelfMatch
EC_InCallAuctionStatus
used for pre-market order operation, e.g., during 2nd phase of call auction, cancel order is not allowed, when the cancel request is failed to be rejected by trading server, the request will be rejected by matching box finally
EC_QtyCannotBeZero
EC_MarketOrderNoSupportTIF
EC_ReachMaxTradeNum
EC_InvalidPriceScale
EC_BitIndexInvalid
EC_StopBySelfMatch
EC_InvalidSmpType
EC_CancelByMMP
EC_InvalidUserType
EC_InvalidMirrorOid
EC_InvalidMirrorUid
EC_EcInvalidQty
EC_InvalidAmount
EC_LoadOrderCancel
EC_MarketQuoteNoSuppSell
EC_DisorderOrderID
EC_InvalidBaseValue
EC_LoadOrderCanMatch
EC_SecurityStatusFail
EC_ReachRiskPriceLimit
EC_OrderNotExist
EC_CancelByOrderValueZero
order cancelled as its remaining value is zero
EC_CancelByMatchValueZero
order cancelled as the order it matched with has a remaining value of zero
EC_ReachMarketPriceLimit
accountType
​
UNIFIED
Unified Trading Account
FUND
Funding Account
transferStatus
​
SUCCESS
PENDING
FAILED
depositStatus
​
0
unknown
1
toBeConfirmed
2
processing
3
success (finalised status of a success deposit)
4
deposit failed
10011
pending to be credited to funding pool
10012
Credited to funding pool successfully
withdrawStatus
​
SecurityCheck
Pending
success
CancelByUser
Reject
Fail
BlockchainConfirmed
MoreInformationRequired
Unknown
a rare status
triggerBy
​
LastPrice
IndexPrice
MarkPrice
cancelType
​
CancelByUser
CancelByReduceOnly
cancelled by
reduceOnly
CancelByPrepareLiq
CancelAllBeforeLiq
cancelled in order to attempt
liquidation prevention
by freeing up margin
CancelByPrepareAdl
CancelAllBeforeAdl
cancelled due to
ADL
CancelByAdmin
CancelBySettle
cancelled due to delisting contract
CancelByTpSlTsClear
TP/SL order cancelled when the position is cleared
CancelBySmp
cancelled by
SMP
CancelByDCP
cancelled by DCP triggering
CancelByRebalance
Spread trading: the order price of a single leg order is outside the limit price range.
CancelByOCOTpCanceledBySlTriggered
The take profit order was canceled due to the triggering of the stop loss
CancelByOCOSlCanceledByTpTriggered
The stop loss order was canceled due to the triggering of the take profit
Options:
CancelByUser
CancelByReduceOnly
CancelAllBeforeLiq
cancelled due to liquidation
CancelAllBeforeAdl
cancelled due to ADL
CancelBySettle
CancelByCannotAffordOrderCost
CancelByPmTrialMmOverEquity
CancelByAccountBlocking
CancelByDelivery
CancelByMmpTriggered
CancelByCrossSelfMuch
CancelByCrossReachMaxTradeNum
CancelByDCP
CancelBySmp
optionPeriod
​
BTC:
7
,
14
,
21
,
30
,
60
,
90
,
180
,
270
days
ETH:
7
,
14
,
21
,
30
,
60
,
90
,
180
,
270
days
SOL:
7
,
14
,
21
,
30
,
60
,
90
days
dataRecordingPeriod
​
5min
15min
30min
minute
1h
4h
hour
4d
day
contractType
​
InversePerpetual
LinearPerpetual
LinearFutures
USDT/USDC Futures
InverseFutures
status
​
PreLaunch
Trading
Delivering
Closed
symbolType
​
innovation
adventure
xstocks
curAuctionPhase
​
NotStarted
Pre-market trading is not started
Finished
Pre-market trading is finished
After the auction, if the pre-market contract fails to enter continues trading phase, it will be delisted and phase="Finished"
After the continuous trading, if the pre-market contract fails to be converted to official contract, it will be delisted and phase="Finished"
CallAuction
Auction phase of pre-market trading
only timeInForce=GTC, orderType=Limit order is allowed to submit
TP/SL are not supported; Conditional orders are not supported
cannot
modify
the order at this stage
order price range: [
preOpenPrice
x 0.5,
maxPrice
]
CallAuctionNoCancel
Auction no cancel phase of pre-market trading
only timeInForce=GTC, orderType=Limit order is allowed to submit
TP/SL are not supported; Conditional orders are not supported
cannot
modify and cancel
the order at this stage
order price range: Buy [
lastPrice
x 0.5,
markPrice
x 1.1], Sell [
markPrice
x 0.9,
maxPrice
]
CrossMatching
cross matching phase
cannot
create, modify and cancel
the order at this stage
Candle data is released from this stage
ContinuousTrading
Continuous trading phase
There is no restriction to create, amend, cancel orders
orderbook, public trade data is released from this stage
marginTrading
​
none
Regardless of normal account or UTA account, this trading pair does not support margin trading
both
For both normal account and UTA account, this trading pair supports margin trading
utaOnly
Only for UTA account,this trading pair supports margin trading
normalSpotOnly
Only for normal account, this trading pair supports margin trading
copyTrading
​
none
Regardless of normal account or UTA account, this trading pair does not support copy trading
both
For both normal account and UTA account, this trading pair supports copy trading
utaOnly
Only for UTA account,this trading pair supports copy trading
normalOnly
Only for normal account, this trading pair supports copy trading
type(uta-translog)
​
TRANSFER_IN
Assets that transferred into Unified wallet
TRANSFER_OUT
Assets that transferred out from Unified wallet
TRADE
SETTLEMENT
USDT Perp funding settlement, and USDC Perp funding settlement + USDC 8-hour session settlement
DELIVERY
USDC Futures, Option delivery
LIQUIDATION
ADL
Auto-Deleveraging
AIRDROP
BONUS
Bonus claimed
BONUS_RECOLLECT
Bonus expired
FEE_REFUND
Trading fee refunded
INTEREST
Interest occurred due to borrowing
CURRENCY_BUY
Currency convert, and the liquidation for borrowing asset(UTA loan)
CURRENCY_SELL
Currency convert, and the liquidation for borrowing asset(UTA loan)
BORROWED_AMOUNT_INS_LOAN
PRINCIPLE_REPAYMENT_INS_LOAN
INTEREST_REPAYMENT_INS_LOAN
AUTO_SOLD_COLLATERAL_INS_LOAN
the liquidation for borrowing asset(INS loan)
AUTO_BUY_LIABILITY_INS_LOAN
the liquidation for borrowing asset(INS loan)
AUTO_PRINCIPLE_REPAYMENT_INS_LOAN
AUTO_INTEREST_REPAYMENT_INS_LOAN
TRANSFER_IN_INS_LOAN
Transfer In when in the liquidation of OTC loan
TRANSFER_OUT_INS_LOAN
Transfer Out when in the liquidation of OTC loan
SPOT_REPAYMENT_SELL
One-click repayment currency sell
SPOT_REPAYMENT_BUY
One-click repayment currency buy
TOKENS_SUBSCRIPTION
Spot leverage token subscription
TOKENS_REDEMPTION
Spot leverage token redemption
AUTO_DEDUCTION
Asset auto deducted by system (roll back)
FLEXIBLE_STAKING_SUBSCRIPTION
Byfi flexible stake subscription
FLEXIBLE_STAKING_REDEMPTION
Byfi flexible stake redemption
FIXED_STAKING_SUBSCRIPTION
Byfi fixed stake subscription
FLEXIBLE_STAKING_REFUND
Byfi flexiable stake refund
FIXED_STAKING_REFUND
Byfi fixed stake refund
PREMARKET_TRANSFER_OUT
PREMARKET_DELIVERY_SELL_NEW_COIN
PREMARKET_DELIVERY_BUY_NEW_COIN
PREMARKET_DELIVERY_PLEDGE_PAY_SELLER
PREMARKET_DELIVERY_PLEDGE_BACK
PREMARKET_ROLLBACK_PLEDGE_BACK
PREMARKET_ROLLBACK_PLEDGE_PENALTY_TO_BUYER
CUSTODY_NETWORK_FEE
fireblocks business
CUSTODY_SETTLE_FEE
fireblocks business
CUSTODY_LOCK
fireblocks / copper business
CUSTODY_UNLOCK
fireblocks business
CUSTODY_UNLOCK_REFUND
fireblocks business
LOANS_BORROW_FUNDS
crypto loan
LOANS_PLEDGE_ASSET
crypto loan repayment
BONUS_TRANSFER_IN
BONUS_TRANSFER_OUT
PEF_TRANSFER_IN
PEF_TRANSFER_OUT
PEF_PROFIT_SHARE
ONCHAINEARN_SUBSCRIPTION
tranfer out for on chain earn
ONCHAINEARN_REDEMPTION
tranfer in for on chain earn
ONCHAINEARN_REFUND
tranfer in for on chain earn failed
STRUCTURE_PRODUCT_SUBSCRIPTION
tranfer out for structure product
STRUCTURE_PRODUCT_REFUND
tranfer in for structure product
CLASSIC_WEALTH_MANAGEMENT_SUBSCRIPTION
tranfer out for classic wealth management
PREMIMUM_WEALTH_MANAGEMENT_SUBSCRIPTION
tranfer in for classic wealth management
PREMIMUM_WEALTH_MANAGEMENT_REFUND
tranfer in for classic wealth management refund
LIQUIDITY_MINING_SUBSCRIPTION
tranfer out for liquidity mining
LIQUIDITY_MINING_REFUND
tranfer in for liquidity mining
PWM_SUBSCRIPTION
tranfer out for PWM
PWM_REFUND
tranfer in for PWM
DEFI_INVESTMENT_SUBSCRIPTION
tranfer out for DEFI subscription
DEFI_INVESTMENT_REFUND
transfer in for DEFI refund
DEFI_INVESTMENT_REDEMPTION
tranfer in for DEFI redemption
INSTITUTION_LOAN_IN
Borrowed Amount (INS Loan)
INSTITUTION_PAYBACK_PRINCIPAL_OUT
Principal repayment (INS Loan)
INSTITUTION_PAYBACK_INTEREST_OUT
Interest repayment (INS Loan)
INSTITUTION_EXCHANGE_SELL
Auto sold collateral (INS Loan)
INSTITUTION_EXCHANGE_BUY
Auto buy liability (INS Loan)
INSTITUTION_LIQ_PRINCIPAL_OUT
Auto principal repayment (INS Loan)
INSTITUTION_LIQ_INTEREST_OUT
Auto interest repayment (INS Loan)
INSTITUTION_LOAN_TRANSFER_IN
Transfer in (INS Loan)
INSTITUTION_LOAN_TRANSFER_OUT
Transfer out (INS Loan)
INSTITUTION_LOAN_WITHOUT_WITHDRAW
Transfer out (INS Loan)
INSTITUTION_LOAN_RESERVE_IN
Reserve fund in (INS Loan)
INSTITUTION_LOAN_RESERVE_OUT
Reserve fund out (INS Loan)
SPREAD_FEE_OUT
Spread fee for EU Broker
PLATFORM_TOKEN_MNT_LIQRECALLEDMMNT
Recall MNT
PLATFORM_TOKEN_MNT_LIQRETURNEDMNT
Return MNT
BORROW
Manual loan borrow and auto loan borrow
REPAY
Manual loan repay and auto loan repay
BROKER_ABACCOUNT_FEE
Borker AB fee deduction
EARNING_REDEMPTION_SELL
EARNING_REDEMPTION_BUY
DBS_CASH_OUT
DBS_CASH_IN
DBS_CASH_OUT_TR
DBS_CASH_IN_TR
CUSTODY_CASH_RECOVER_TR
ALPHA_SMALL_TOKEN_REFUND
type(contract-translog)
​
TRANSFER_IN
Assets that transferred into (inverse) derivatives wallet
TRANSFER_OUT
Assets that transferred out from (inverse) derivatives wallet
TRADE
SETTLEMENT
USDT / Inverse Perp funding settlement
DELIVERY
Inverse Futures delivery
LIQUIDATION
ADL
Auto-Deleveraging
AIRDROP
BONUS
Bonus claimed
BONUS_RECOLLECT
Bonus expired
FEE_REFUND
Trading fee refunded
CURRENCY_BUY
Currency convert
CURRENCY_SELL
Currency convert
AUTO_DEDUCTION
Asset auto deducted by system (roll back)
Others
unifiedMarginStatus
​
1
Classic account
3
Unified trading account 1.0
4
Unified trading account 1.0 (pro version)
5
Unified trading account 2.0
6
Unified trading account 2.0 (pro version)
convertAccountType
​
eb_convert_uta
Unified Trading Account
eb_convert_funding
Funding Account
symbol
​
USDT Perpetual
:
BTCUSDT
ETHUSDT
USDT Futures
:
BTCUSDT-21FEB25
ETHUSDT-14FEB25
The types of USDT Futures contracts offered by Bybit include: Weekly, Bi-Weekly, Tri-Weekly, Monthly, Bi-Monthly, Quarterly, Bi-Quarterly, Tri-Quarterly
USDC Perpetual
:
BTCPERP
ETHPERP
USDC Futures
:
BTC-24MAR23
Inverse Perpetual
:
BTCUSD
ETHUSD
Inverse Futures
:
BTCUSDH23
H: First quarter; 23: 2023
BTCUSDM23
M: Second quarter; 23: 2023
BTCUSDU23
U: Third quarter; 23: 2023
BTCUSDZ23
Z: Fourth quarter; 23: 2023
Spot
:
BTCUSDT
ETHUSDC
Option
:
BTC-13FEB25-89000-P-USDT
USDT Option
ETH-28FEB25-2800-C
USDC Option
vipLevel
​
No VIP
VIP-1
VIP-2
VIP-3
VIP-4
VIP-5
VIP-Supreme
PRO-1
PRO-2
PRO-3
PRO-4
PRO-5
PRO-6
adlRankIndicator
​
0
default value of empty position
1
2
3
4
5
smpType
​
default:
None
CancelMaker
CancelTaker
CancelBoth
extraFees.feeType
​
UNKNOWN
TAX
Government tax. Only for Indonesian site
CFX
Indonesian foreign exchange tax. Only for Indonesian site
WHT
EU withholding tax. Only for EU site
GST
Indian GST tax. Only for kyc=Indian users
VAT
ARE VAT tax. Only for kyc=ARE users
extraFees.subFeeType
​
UNKNOWN
TAX_PNN
Tax fee, fiat currency to digital currency. Only for Indonesian site
TAX_PPH
Tax fee, digital currency to fiat currency. Only for Indonesian site
CFX_FIEE
CFX fee, fiat currency to digital currency. Only for Indonesian site
AUT_WITHHOLDING_TAX
EU site withholding tax. Only for EU site
IND_GST
Indian GST tax. Only for kyc=Indian users
ARE_VAT
ARE VAT tax. Only for kyc=ARE users
state
​
scheduled
ongoing
completed
canceled
serviceTypes
​
1
Trading service
2
Trading service via http request
3
Trading service via websocket
4
Private websocket stream
5
Market data service
product
​
1
Futures
2
Spot
3
Option
4
Spread
maintainType
​
1
Planned maintenance
2
Temporary maintenance
3
Incident
env
​
1
Production
2
Production Demo service
bizType
​
SPOT
DERIVATIVES
OPTIONS
msg
​
API limit updated successfully
Requested limit exceeds maximum allowed per user
No permission to operate these UIDs
API cap configuration not found
API cap configuration not found for bizType
Requested limit would exceed institutional quota
groupId
​
1
Major Coins
2
High Growth
3
Mid-Tier Liquidity
4
Mid-Tier Activation
5
Long Tail
6
Innovation Zone
7
Pre-Listing
groupName
​
G1(Major Coins)
Major Coins
G2(High Growth)
High Growth
G3(Mid-Tier Liquidity)
Mid-Tier Liquidity
G4(Mid-Tier Activation)
Mid-Tier Activation
G5(Long Tail)
Long Tail
Innovation-Zone
Innovation Zone
Pre-listing
Pre-listing
Spot Fee Currency Instruction
​
with the example of BTCUSDT:
Is makerFeeRate positive?
TRUE
Side = Buy -> base currency (BTC)
Side = Sell -> quote currency (USDT)
FALSE
IsMakerOrder = TRUE
Side = Buy -> quote currency (USDT)
Side = Sell -> base currency (BTC)
IsMakerOrder = FALSE
Side = Buy -> base currency (BTC)
Side = Sell -> quote currency (USDT)
sbe-orderStatus
​
5
Rejected
6
New
7
Cancelled
8
PartiallyFilled
9
Filled
0
Others
sbe-rejectReason
​
0
EC_NoError
1
EC_Others
2
EC_UnknownMessageType
3
EC_MissingClOrdID
4
EC_OrderNotExist
5
EC_MissingOrigClOrdID
6
EC_ClOrdIDOrigClOrdIDAreTheSame
7
EC_OrigClOrdIDDoesNotExist
8
EC_TooLateToCancel
9
EC_UnknownOrderType
10
EC_UnknownSide
11
EC_UnknownTimeInForce
12
EC_WronglyRouted
13
EC_MarketOrderPriceIsNotZero
14
EC_LimitOrderInvalidPrice
15
EC_NoEnoughQtyToFill
16
EC_NoImmediateQtyToFill
17
EC_QtyCannotBeZero
18
EC_PerCancelRequest
19
EC_MarketOrderCannotBePostOnly
20
EC_PostOnlyWillTakeLiquidity
21
EC_CancelReplaceOrder
22
EC_InvalidSymbolStatus
23
EC_MarketOrderNoSupportTIF
24
EC_ReachMaxTradeNum
25
EC_InvalidPriceScale
28
EC_BySelfMatch
29
EC_InvalidSmpType
30
EC_CancelByMMP
31
EC_InCallAuctionStatus
34
EC_InvalidUserType
35
EC_InvalidMirrorOid
36
EC_InvalidMirrorUid
100
EC_EcInvalidQty
101
EC_InvalidAmount
102
EC_LoadOrderCancel
103
EC_CancelForNoFullFill
104
EC_MarketQuoteNoSuppSell
105
EC_DisorderOrderID
106
EC_InvalidBaseValue
107
EC_LoadOrderCanMatch
108
EC_SecurityStatusFail
110
EC_ReachRiskPriceLimit
111
EC_CancelByOrderValueZero
112
EC_CancelByMatchValueZero
113
EC_CancelByMatchValueZero
200
EC_ReachMarketPriceLimit

---

## Get Rate Limit

**URL:** https://bybit-exchange.github.io/docs/v5/rate-limit/rules-for-pros/apilimit-query

**Contents:**
- Get Rate Limit
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Rate Limit
API Rate Limit Rules for PROs
Get Rate Limit
On this page
Get Rate Limit
API rate limit: 50 req per second
info
A master account can query its own and its subaccounts' API rate limit.
A subaccount can only query its own API rate limit.
HTTP Request
​
GET
/v5/apilimit/query
Request Parameters
​
Parameter
Required
Type
Comments
uids
true
string
Multiple UIDs separated by commas
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> uids
string
Multiple UIDs separated by commas
>
bizType
string
Business type
> rate
integer
API rate limit per second
Request Example
​
GET
/v5/apilimit/query?uids=290118
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
"uids"
:
"290118"
,
"bizType"
:
"SPOT"
,
"rate"
:
600
}
,
{
"uids"
:
"290118"
,
"bizType"
:
"DERIVATIVES"
,
"rate"
:
400
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
1754894341984
}

**Examples:**

Example 1 ():
```
GET /v5/apilimit/query?uids=290118 HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728460942776X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 2
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "uids": "290118",                "bizType": "SPOT",                "rate": 600            },            {                "uids": "290118",                "bizType": "DERIVATIVES",                "rate": 400            }        ]    },    "retExtInfo": {},    "time": 1754894341984}
```

---

## Rate Limit Rules

**URL:** https://bybit-exchange.github.io/docs/v5/rate-limit

**Contents:**
- Rate Limit Rules
- IP Limit​
  - HTTP IP limit​
  - Websocket IP limit​
- API Rate Limit​
  - API Rate Limit Table​
    - Trade​
    - Position​
    - Account​
    - Asset​

Rate Limit
Rate Limit Rules
On this page
Rate Limit Rules
IP Limit
​
HTTP IP limit
​
You are allowed to send
600 requests within a 5-second window per IP
by default. This limit applies to all traffic directed to
api.bybit.com
,
api.bybick.com
, and local site hostnames such as
api.bybit.kz
.
If you encounter the error
"403, access too frequent"
, it indicates that your IP has exceeded the allowed request frequency. In this case, you should terminate all HTTP sessions and wait for at least 10 minutes. The ban will be lifted automatically.
We do not recommend running your application at the very edge of these limits in case abnormal network activity results
in an unexpected violation.
Websocket IP limit
​
Do not establish more than 500 connections within a 5-minute window. This limit applies to all connections directed to
stream.bybit.com
as well as local site hostnames such as
stream.bybit.kz
.
Do not frequently connect and disconnect the connection
Do not establish more than 1,000 connections per IP for market data. The connection limits are counted separately for Spot, Linear, Inverse, and Options markets
API Rate Limit
​
caution
If you receive
"retCode": 10006, "retMsg": "Too many visits!"
in the JSON response, you have hit the API rate limit.
The API rate limit is based on the
rolling time window per second and UID
. In other words, it is per second per UID.
Every request to the API returns response header shown in the code panel:
X-Bapi-Limit-Status
- your remaining requests for current endpoint
X-Bapi-Limit
- your current limit for current endpoint
X-Bapi-Limit-Reset-Timestamp
- the timestamp indicating when your request limit resets if you have exceeded your rate_limit. Otherwise, this is just the current timestamp (it may not exactly match
timeNow
).
Http Response Header Example
▶Response Headers
Content-Type
:
application/json; charset=utf-8
Content-Length
:
141
X-Bapi-Limit
:
100
X-Bapi-Limit-Status
:
99
X-Bapi-Limit-Reset-Timestamp
:
1672738134824
API Rate Limit Table
​
Trade
​
Method
Path
UTA2.0 Pro
upgradable
inverse
linear
option
spot
POST
/v5/order/create
10/s
10/s
20/s
Y
/v5/order/amend
10/s
10/s
10/s
Y
/v5/order/cancel
10/s
10/s
20/s
Y
/v5/order/cancel-all
10/s
1/s
20/s
Y
/v5/order/create-batch
10/s
10/s
20/s
Y
/v5/order/amend-batch
10/s
10/s
20/s
Y
/v5/order/cancel-batch
10/s
10/s
20/s
Y
/v5/order/disconnected-cancel-all
5/s
N
GET
/v5/order/realtime
50/s
N
/v5/order/history
50/s
N
/v5/execution/list
50/s
N
/v5/order/spot-borrow-check
-
50/s
N
Position
​
Method
Path
UTA2.0 Pro
upgradable
inverse
linear
option
spot
GET
/v5/position/list
50/s
-
N
/v5/position/closed-pnl
50/s
-
-
N
POST
/v5/position/set-leverage
10/s
10/s
-
-
N
Account
​
Method
Path
Limit
upgradable
GET
/v5/account/wallet-balance
accountType=UNIFIED
50/s
N
/v5/account/withdrawal
50/s
N
/v5/account/borrow-history
50/s
N
/v5/account/borrow
1/s
N
/v5/account/repay
1/s
N
/v5/account/no-convert-repay
1/s
N
/v5/account/collateral-info
50/s
N
/v5/asset/coin-greeks
50/s
N
/v5/account/transaction-log
accountType=UNIFIED
50/s
N
/v5/account/fee-rate
category=linear
10/s
N
category=spot
5/s
N
category=option
5/s
N
category=inverse
10/s
N
Asset
​
Method
Path
Limit
upgradable
GET
/v5/asset/transfer/query-asset-info
60 req/min
N
/v5/asset/transfer/query-transfer-coin-list
60 req/min
N
/v5/asset/transfer/query-inter-transfer-list
60 req/min
N
/v5/asset/transfer/query-sub-member-list
60 req/min
N
/v5/asset/transfer/query-universal-transfer-list
5 req/s
N
/v5/asset/transfer/query-account-coins-balance
5 req/s
N
/v5/asset/deposit/query-record
100 req/min
N
/v5/asset/deposit/query-sub-member-record
300 req/min
N
/v5/asset/deposit/query-address
300 req/min
N
/v5/asset/deposit/query-sub-member-address
300 req/min
N
/v5/asset/withdraw/query-record
300 req/min
N
/v5/asset/coin/query-info
5 req/s
N
/v5/asset/exchange/order-record
600 req/min
N
POST
/v5/asset/transfer/inter-transfer
60 req/min
N
/v5/asset/transfer/save-transfer-sub-member
20 req/s
N
/v5/asset/transfer/universal-transfer
5 req/s
N
/v5/asset/withdraw/create
5 req/s
N
/v5/asset/withdraw/cancel
60 req/min
N
User
​
Method
Path
Limit
upgradable
POST
v5/user/create-sub-member
1 req/s
N
/v5/user/create-sub-api
1 req/s
N
/v5/user/frozen-sub-member
5 req/s
N
/v5/user/update-api
5 req/s
N
/v5/user/update-sub-api
5 req/s
N
/v5/user/delete-api
5 req/s
N
/v5/user/delete-sub-api
5 req/s
N
GET
/v5/user/query-sub-members
10 req/s
N
/v5/user/query-api
10 req/s
N
/v5/user/aff-customer-info
10 req/s
N
Spot Margin Trade (UTA)
​
For now, there is no limit for endpoints under this category
Spread Trading
​
Method
Path
Limit
Upgradable
POST
Create Spread Order
20 req/s
N
POST
Amend Spread Order
20 req/s
N
POST
Cancel Spread Order
20 req/s
N
POST
Cancel All Spread Orders
5 req/s
N
GET
Get Spread Open Orders
50 req/s
N
GET
Get Spread Order History
50 req/s
N
GET
Get Spread Trade History
50 req/s
N
Instructions for batch endpoints
​
tip
The batch order endpoint, which includes operations for creating, amending, and canceling, has its own rate limit and
does not share it with single requests,
e.g., let's say the rate limit of single create order endpoint is 100/s, and batch create order endpoint
is 100/s, so in this case, I can place 200 linear orders in one second if I use both endpoints to place orders
When category = linear spot or inverse
​
API for batch create/amend/cancel order, the frequency of the API will be consistent with the current configuration,
but the counting consumption will be consumed according to the actual number of orders. (Number of consumption = number
of requests * number of orders included in a single request), and the configuration of business lines is independent of each other.
The batch APIs allows 1-10 orders/request. For example, if a batch order request is made once and contains 5 orders,
then the request limit will consume 5.
If part of the last batch of orders requested within 1s exceeds the limit, the part that exceeds the limit will fail, and
the part that does not exceed the limit will succeed. For example, in the 1 second, the remaining limit is 5, but a batch request
containing 8 orders is placed at this time, then the first 5 orders will be successfully placed, and the 6-8th orders
will report an error exceeding the limit, and these orders will fail.

**Examples:**

Example 1 ():
```
▶Response HeadersContent-Type: application/json; charset=utf-8Content-Length: 141X-Bapi-Limit: 100X-Bapi-Limit-Status: 99X-Bapi-Limit-Reset-Timestamp: 1672738134824
```

---

## Borrow

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/borrow

**Contents:**
- Borrow
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Borrow
On this page
Borrow
Permission: "Spot trade"
info
The loan funds are released to the Funding wallet.
The collateral funds are deducted from the Funding wallet, so make sure you have enough collateral amount in the Funding wallet.
HTTP Request
​
POST
/v5/crypto-loan/borrow
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
false
string
Amount to borrow
Required
when collateral amount is not filled
loanTerm
false
string
Loan term
flexible term:
null
or not passed
fixed term:
7
,
14
,
30
,
90
,
180
days
collateralCurrency
true
string
Currency used to mortgage
collateralAmount
false
string
Amount to mortgage
Required
when loan amount is not filled
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
/v5/crypto-loan/borrow
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
1728629356551
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
140
{
"loanCurrency"
:
"USDT"
,
"loanAmount"
:
"550"
,
"collateralCurrency"
:
"BTC"
,
"loanTerm"
:
null
,
"collateralAmount"
:
null
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
borrow_crypto_loan
(
loanCurrency
=
"USDT"
,
loanAmount
=
"550"
,
collateralCurrency
=
"BTC"
,
loanTerm
=
None
,
collateralAmount
=
None
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
borrowCryptoLoan
(
{
loanCurrency
:
'USDT'
,
loanAmount
:
'550'
,
collateralCurrency
:
'BTC'
,
loanTerm
:
null
,
collateralAmount
:
null
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
"orderId"
:
"1794267532472646144"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1728629357820
}

**Examples:**

Example 1 ():
```
POST /v5/crypto-loan/borrow HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728629356551X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 140{    "loanCurrency": "USDT",    "loanAmount": "550",    "collateralCurrency": "BTC",    "loanTerm": null,    "collateralAmount": null}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.borrow_crypto_loan(        loanCurrency="USDT",        loanAmount="550",        collateralCurrency="BTC",        loanTerm=None,        collateralAmount=None,))
```

Example 3 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'xxxxxxxxxxxxxxxxxx',  secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client  .borrowCryptoLoan({    loanCurrency: 'USDT',    loanAmount: '550',    collateralCurrency: 'BTC',    loanTerm: null,    collateralAmount: null,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

Example 4 ():
```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "orderId": "1794267532472646144"    },    "retExtInfo": {},    "time": 1728629357820}
```

---

## Set Rate Limit

**URL:** https://bybit-exchange.github.io/docs/v5/rate-limit/rules-for-pros/apilimit-set

**Contents:**
- Set Rate Limit
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Rate Limit
API Rate Limit Rules for PROs
Set Rate Limit
On this page
Set Rate Limit
API rate limit: 50 req per second
info
If the UID requesting this endpoint is a master account, UIDs passed to the
uids
parameter must be subaccounts of the master account.
If the UID requesting this endpoint is not a master account, the UID passed to the
uids
parameter must be the UID of the subaccount requesting this endpoint.
Only institutional users can request this endpoint.
HTTP Request
​
POST
/v5/apilimit/set
Request Parameters
​
Parameter
Required
Type
Comments
list
true
array
Object
> uids
true
string
Multiple UIDs separated by commas
>
bizType
true
string
Business type
> rate
true
integer
API rate limit per second
Response Parameters
​
Parameter
Type
Comments
list
array
Object
> uids
string
Multiple UIDs separated by commas
>
bizType
string
Business type
> rate
integer
API rate limit per second
> success
boolean
Whether or not the request was successful
>
msg
string
Result message
Request Example
​
POST
/v5/apilimit/set
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
1711420489915
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"list"
:
[
{
"uids"
:
"106293838"
,
"bizType"
:
"DERIVATIVES"
,
"rate"
:
50
}
]
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
"result"
:
[
{
"uids"
:
"290118"
,
"bizType"
:
"SPOT"
,
"rate"
:
600
,
"success"
:
true
,
"msg"
:
"API limit updated successfully"
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
1754894296913
}

**Examples:**

Example 1 ():
```
POST /v5/apilimit/set HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1711420489915X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "list": [        {            "uids": "106293838",            "bizType": "DERIVATIVES",            "rate": 50        }    ]}
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "result": [            {                "uids": "290118",                "bizType": "SPOT",                "rate": 600,                "success": true,                "msg": "API limit updated successfully"            }        ]    },    "retExtInfo": {},    "time": 1754894296913}
```

---

## Get Rate Limit Cap

**URL:** https://bybit-exchange.github.io/docs/v5/rate-limit/rules-for-pros/apilimit-query-cap

**Contents:**
- Get Rate Limit Cap
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Rate Limit
API Rate Limit Rules for PROs
Get Rate Limit Cap
On this page
Get Rate Limit Cap
API rate limit: 50 req per second
info
Get your institutions's total rate limit usage and cap, across the board.
Main UIDs or sub UIDs can query this endpoint, but a main UID can only see the rate limits of subs below it, and not the subs of other main UIDs.
HTTP Request
​
GET
/v5/apilimit/query-cap
Request Parameters
​
None
Response Parameters
​
Parameter
Type
Comments
list
array
Object
>
bizType
string
Business type
> totalRate
integer
Total API rate limit usage accross all subaccounts and master account
> insCap
integer
Institutional-level API rate limit per second (depends on your pro level)
> uidCap
integer
UID-level API rate limit per second
Request Example
​
GET
/v5/apilimit/query-cap
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
"insCap"
:
"30000"
,
"uidCap"
:
"600"
,
"totalRate"
:
"29882"
,
"bizType"
:
"SPOT"
}
,
{
"insCap"
:
"30000"
,
"uidCap"
:
"600"
,
"totalRate"
:
"29882"
,
"bizType"
:
"OPTIONS"
}
,
{
"insCap"
:
"40000"
,
"uidCap"
:
"800"
,
"totalRate"
:
"39932"
,
"bizType"
:
"DERIVATIVES"
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
1758857589872
}

**Examples:**

Example 1 ():
```
GET /v5/apilimit/query-cap HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728460942776X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 2
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "insCap": "30000",                "uidCap": "600",                "totalRate": "29882",                "bizType": "SPOT"            },            {                "insCap": "30000",                "uidCap": "600",                "totalRate": "29882",                "bizType": "OPTIONS"            },            {                "insCap": "40000",                "uidCap": "800",                "totalRate": "39932",                "bizType": "DERIVATIVES"            }        ]    },    "retExtInfo": {},    "time": 1758857589872}
```

---

## Get All Rate Limits

**URL:** https://bybit-exchange.github.io/docs/v5/rate-limit/rules-for-pros/apilimit-query-all

**Contents:**
- Get All Rate Limits
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Rate Limit
API Rate Limit Rules for PROs
Get All Rate Limits
On this page
Get All Rate Limits
API rate limit: 50 req per second
info
Query for all your UID-level rate limits, including all master accounts and subaccounts.
HTTP Request
​
GET
/v5/apilimit/query-all
Request Parameters
​
Parameter
Required
Type
Comments
limit
false
string
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
uids
false
string
Multiple UIDs across different master accounts, separated by commas. Returns all master accounts and subaccounts by default
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
> uids
string
Multiple UIDs separated by commas
>
bizType
string
Business type
> rate
integer
API Rate limit per second
Request Example
​
GET
/v5/apilimit/query-all
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
"uids"
:
"104270393,1674166,1190923,101446030"
,
"bizType"
:
"SPOT"
,
"rate"
:
223
}
,
{
"uids"
:
"104074050,104394193,104126066"
,
"bizType"
:
"OPTIONS"
,
"rate"
:
223
}
,
{
"uids"
:
"104154966,103803484,103995540,100445068"
,
"bizType"
:
"DERIVATIVES"
,
"rate"
:
298
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
1758857701702
}

**Examples:**

Example 1 ():
```
GET /v5/apilimit/query-all HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1728460942776X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 2
```

Example 2 ():
```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "uids": "104270393,1674166,1190923,101446030",                "bizType": "SPOT",                "rate": 223            },            {                "uids": "104074050,104394193,104126066",                "bizType": "OPTIONS",                "rate": 223            },            {                "uids": "104154966,103803484,103995540,100445068",                "bizType": "DERIVATIVES",                "rate": 298            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1758857701702}
```

---

## Switch Cross/Isolated Margin

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/cross-isolate

**Contents:**
- Switch Cross/Isolated Margin
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Switch Cross/Isolated Margin
On this page
Switch Cross/Isolated Margin
Select cross margin mode or isolated margin mode per symbol level
HTTP Request
​
POST
/v5/position/switch-isolated
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
UTA2.0
: not supported
UTA1.0
:
inverse
Classic:
linear
(USDT Preps),
inverse
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
tradeMode
true
integer
0
: cross margin.
1
: isolated margin
buyLeverage
true
string
The value must be equal to
sellLeverage
value
sellLeverage
true
string
The value must be equal to
buyLeverage
value
RUN >>
Response Parameters
​
None
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/switch-isolated
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
1675248447965
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
Content-Length
:
121
{
"category"
:
"linear"
,
"symbol"
:
"ETHUSDT"
,
"tradeMode"
:
1
,
"buyLeverage"
:
"10"
,
"sellLeverage"
:
"10"
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
switch_margin_mode
(
category
=
"linear"
,
symbol
=
"ETHUSDT"
,
tradeMode
=
1
,
buyLeverage
=
"10"
,
sellLeverage
=
"10"
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
*
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
position
.
*
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
position
.
request
.
*
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
newAsyncPositionRestClient
(
)
;
var
switchMarginRequest
=
PositionDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTC-31MAR23"
)
.
tradeMode
(
MarginMode
.
CROSS_MARGIN
)
.
buyLeverage
(
"5"
)
.
sellLeverage
(
"5"
)
.
build
(
)
;
client
.
swithMarginRequest
(
switchMarginRequest
,
System
.
out
::
println
)
;
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
switchIsolatedMargin
(
{
category
:
'linear'
,
symbol
:
'ETHUSDT'
,
tradeMode
:
1
,
buyLeverage
:
'10'
,
sellLeverage
:
'10'
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
1675248433635
}

**Examples:**

Example 1 ():
```
POST /v5/position/switch-isolated HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1675248447965X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 121{    "category": "linear",    "symbol": "ETHUSDT",    "tradeMode": 1,    "buyLeverage": "10",    "sellLeverage": "10"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.switch_margin_mode(    category="linear",    symbol="ETHUSDT",    tradeMode=1,    buyLeverage="10",    sellLeverage="10",))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var switchMarginRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTC-31MAR23").tradeMode(MarginMode.CROSS_MARGIN).buyLeverage("5").sellLeverage("5").build();client.swithMarginRequest(switchMarginRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .switchIsolatedMargin({        category: 'linear',        symbol: 'ETHUSDT',        tradeMode: 1,        buyLeverage: '10',        sellLeverage: '10',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Set TP/SL Mode

**URL:** https://bybit-exchange.github.io/docs/v5/abandon/tpsl-mode

**Contents:**
- Set TP/SL Mode
  - HTTP Request​
  - Request Parameters​
  - Response Parameters​
  - Request Example​
  - Response Example​

Abandoned Endpoints
Set TP/SL Mode (deprecated)
On this page
Set TP/SL Mode
tip
To some extent, this endpoint is
deprecated
because now tpsl is based on order level. This API was used for position level
change before.
However, you still can use it to set an implicit tpsl mode for a certain symbol because when you don't
pass "tpslMode" in the place order or trading stop request, system will get the tpslMode by the default setting.
Set TP/SL mode to Full or Partial
info
For partial TP/SL mode, you can set the TP/SL size smaller than position size.
HTTP Request
​
POST
/v5/position/set-tpsl-mode
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
inverse
symbol
true
string
Symbol name, like
BTCUSDT
, uppercase only
tpSlMode
true
string
TP/SL mode.
Full
,
Partial
Response Parameters
​
Parameter
Type
Comments
tpSlMode
string
Full
,
Partial
RUN >>
Request Example
​
HTTP
Python
Java
Node.js
POST
/v5/position/set-tpsl-mode
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
1672279325035
X-BAPI-RECV-WINDOW
:
5000
Content-Type
:
application/json
{
"symbol"
:
"XRPUSDT"
,
"category"
:
"linear"
,
"tpSlMode"
:
"Full"
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
set_tp_sl_mode
(
symbol
=
"XRPUSDT"
,
category
=
"linear"
,
tpSlMode
=
"Full"
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
*
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
position
.
*
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
position
.
request
.
*
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
newAsyncPositionRestClient
(
)
;
var
setTpSlRequest
=
PositionDataRequest
.
builder
(
)
.
category
(
CategoryType
.
LINEAR
)
.
symbol
(
"BTCUSDT"
)
.
tpslMode
(
TpslMode
.
PARTIAL
)
.
build
(
)
;
client
.
swithMarginRequest
(
setTpSlRequest
,
System
.
out
::
println
)
;
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
setTPSLMode
(
{
symbol
:
'XRPUSDT'
,
category
:
'linear'
,
tpSlMode
:
'Full'
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
"tpSlMode"
:
"Full"
}
,
"retExtInfo"
:
{
}
,
"time"
:
1672279322666
}

**Examples:**

Example 1 ():
```
POST /v5/position/set-tpsl-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: xxxxxxxxxxxxxxxxxxX-BAPI-TIMESTAMP: 1672279325035X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "symbol": "XRPUSDT",    "category": "linear",    "tpSlMode": "Full"}
```

Example 2 ():
```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="xxxxxxxxxxxxxxxxxx",    api_secret="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",)print(session.set_tp_sl_mode(    symbol="XRPUSDT",    category="linear",    tpSlMode="Full",))
```

Example 3 ():
```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setTpSlRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").tpslMode(TpslMode.PARTIAL).build();client.swithMarginRequest(setTpSlRequest, System.out::println);
```

Example 4 ():
```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'xxxxxxxxxxxxxxxxxx',    secret: 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',});client    .setTPSLMode({        symbol: 'XRPUSDT',        category: 'linear',        tpSlMode: 'Full',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

---

## Error Codes

**URL:** https://bybit-exchange.github.io/docs/v5/error

**Contents:**
- Error Codes
- HTTP Code​
- WS OE General code​
- UTA​
- Spot Trade​
- Spot Margin Trade​
- Asset​
  - Fiat Convert​
  - Convert Small Balances​
- Crypto Loan (New)​

Error Codes
On this page
Error Codes
HTTP Code
​
Code
Description
400
Bad request. Need to send the request with
GET
/
POST
(must be capitalized)
401
Invalid request. 1. Need to use the correct key to access; 2. Need to put authentication params in the request header
403
Forbidden request. Possible causes: 1. IP rate limit breached; 2. You send GET request with an empty json body; 3. You are using U.S IP
404
Cannot find path. Possible causes: 1. Wrong path; 2. Category value does not match account mode
429
System level frequency protection. Please retry when encounter this
WS OE General code
​
Code
Description
10404
1. op type is not found; 2.
category
is not correct/supported
10429
System level frequency protection
10003
Too many sessions under the same UID
10016
1. internal server error; 2. Service is restarting
10019
ws trade service is restarting, do not accept new request, but the request in the process is not affected. You can build new connection to be routed to normal service
20003
Too frequent requests under the same session
20006
reqId is duplicated
UTA
​
Code
Description
0
OK
-1
request expired: o@0, now[] diff[]
429
The trading service is experiencing a high server load. Please retry if you encounter this issue.
-2015
(Spot) Your api key has expired
33004
(Derivatives) Your api key has expired
10000
Server Timeout
10001
Request parameter error
10002
The request time exceeds the time window range.
10003
API key is invalid. Check whether the key and domain are matched, there are 4 env: mainnet, testnet, mainnet-demo, testnet-demo
10004
Error sign, please check your signature generation algorithm.
10005
Permission denied, please check your API key permissions.
10006
Too many visits. Exceeded the API Rate Limit.
10007
User authentication failed.
10008
Common banned, please check your account mode
10009
IP has been banned.
10010
Unmatched IP, please check your API key's bound IP addresses.
10014
Invalid duplicate request.
10016
Server error.
10017
Route not found.
10018
Exceeded the IP Rate Limit.
10024
Compliance rules triggered
10027
Transactions are banned.
10029
The requested symbol is invalid, please check symbol whitelist
10028
The API can only be accessed by unified account users.
30133
OTC loan: The symbol you select for USDT Perpetual is not allowed by Institutional Lending
30134
OTC loan: The symbol you select for USDC Contract is not allowed by Institutional Lending
30135
The leverage you select for USDT Perpetual trading cannot exceed the maximum leverage allowed by Institutional Lending.
30136
The leverage you select for USDC Perpetual or Futures trading cannot exceed the maximum leverage allowed by Institutional Lending.
30208
Failed to submit order(s). The order price is higher than the maximum buying price
40004
the order is modified during the process of replacing , please check the order status again
100028
The API cannot be accessed by unified account users.
110001
Order does not exist
110003
Order price exceeds the
allowable range
.
110004
Wallet balance is insufficient
110005
position status error
110006
The assets are estimated to be unable to cover the position margin
110007
Available balance is insufficient
110008
The order has been completed or cancelled.
110009
The number of stop orders exceeds the maximum allowable limit
110010
The order has been cancelled
110011
Liquidation will be triggered immediately by this adjustment
110012
Insufficient available balance.
110013
Cannot set leverage due to risk limit level.
110014
Insufficient available balance to add additional margin.
110015
The position is in cross margin mode.
110016
The quantity of contracts requested exceeds the risk limit, please adjust your risk limit level before trying again
110017
orderQty will be truncated to zero
110018
User ID is illegal.
110019
Order ID is illegal.
110020
Not allowed to have more than 500 active orders.
110021
Not allowed to exceeded position limits due to Open Interest.
110022
Quantity has been restricted and orders cannot be modified to increase the quantity.
110023
Currently you can only reduce your position on this contract. please check our announcement or contact customer service for details.
110024
You have an existing position, so the position mode cannot be switched.
110025
Position mode has not been modified.
110026
Cross/isolated margin mode has not been modified.
110027
Margin has not been modified.
110028
You have existing open orders, so the position mode cannot be switched.
110029
Hedge mode is not supported for this symbol.
110030
Duplicate orderId
110031
Non-existing risk limit info, please check the risk limit rules.
110032
Order is illegal
110033
You can't set margin without an open position
110034
There is no net position
110035
Cancellation of orders was not completed before liquidation
110036
You are not allowed to change leverage due to cross margin mode.
110037
User setting list does not have this symbol
110038
You are not allowed to change leverage due to portfolio margin mode.
110039
Maintenance margin rate is too high. This may trigger liquidation.
110040
The order will trigger a forced liquidation, please re-submit the order.
110041
Skip liquidation is not allowed when a position or maker order exists
110042
Currently,due to pre-delivery status, you can only reduce your position on this contract.
110043
Set leverage has not been modified.
110044
Available margin is insufficient.
110045
Wallet balance is insufficient.
110046
Liquidation will be triggered immediately by this adjustment.
110047
Risk limit cannot be adjusted due to insufficient available margin.
110048
Risk limit cannot be adjusted as the current/expected position value exceeds the revised risk limit.
110049
Tick notes can only be numbers
110050
Invalid coin
110051
The user's available balance cannot cover the lowest price of the current market
110052
Your available balance is insufficient to set the price
110053
The user's available balance cannot cover the current market price and upper limit price
110054
This position has at least one take profit link order, so the take profit and stop loss mode cannot be switched
110055
This position has at least one stop loss link order, so the take profit and stop loss mode cannot be switched
110056
This position has at least one trailing stop link order, so the take profit and stop loss mode cannot be switched
110057
Conditional order or limit order contains TP/SL related params
110058
You can't set take profit and stop loss due to insufficient size of remaining position size.
110059
Not allowed to  modify the TP/SL of a partially filled open order
110060
Under full TP/SL mode, it is not allowed to modify TP/SL
110061
Not allowed to have more than 20 TP/SLs under Partial tpSlMode
110062
There is no MMP information of the institution found.
110063
Settlement in progress! {{key0}} not available for trading.
110064
The modified contract quantity cannot be less than or equal to the filled quantity.
110065
MMP hasn't yet been enabled for your account. Please contact your BD manager.
110066
Trading is currently not allowed.
110067
Unified account is not supported.
110068
Leveraged trading is not allowed.
110069
Ins lending customer is not allowed to trade.
110070
ETP symbols cannot be traded.
110071
Sorry, we're revamping the Unified Margin Account! Currently, new upgrades are not supported. If you have any questions, please contact our 24/7 customer support.
110072
OrderLinkedID is duplicate
110073
Set margin mode failed
110074
This contract is not live
110075
RiskId not modified
110076
Only isolated mode can set auto-add-margin
110077
Pm mode cannot support
110078
Added margin more than max can reduce margin
110079
The order is processing and can not be operated, please try again later
110080
Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated (trade/risk limit/leverage)
110082
You cannot lift Reduce-Only restrictions, as no Reduce-Only restrictions are applied to your position
110083
Reduce-Only restrictions must be lifted for both Long and Short positions at the same time
110085
The risk limit and margin ratio for this contract has been updated, please select a supported risk limit and place your order again
110086
Current order leverage exceeds the maximum available for your current Risk Limit tier. Please lower leverage before placing an order
110087
Leverage for Perpetual or Futures contracts cannot exceed the maximum allowed for your Institutional loan
110088
Please Upgrade to UTA to trade
110089
Exceeds the maximum risk limit level
110090
Order placement failed as your position may exceed the max limit. Please adjust your leverage to {{leverage}} or below to increase the max. position limit
110092
expect Rising, but trigger_price
[XXXXX]
<= current
[XXXXX]
??laste
110093
expect Falling, but trigger_price
[XXXXX]
>= current
[XXXXX]
??last
110094
Order notional value below the lower limit
110095
You cannot create, modify or cancel Pre-Market Perpetual orders during the Call Auction.
110096
Pre-Market Perpetual Trading does not support Portfolio Margin mode.
110097
Non-UTA users cannot access Pre-Market Perpetual Trading. To place, modify or cancel Pre-Market Perpetual orders, please upgrade your Standard Account to UTA.
110098
Only Good-Till-Canceled (GTC) orders are supported during Call Auction.
110099
You cannot create TP/SL orders during the Call Auction for Pre-Market Perpetuals.
110100
You cannot place, modify, or cancel Pre-Market Perpetual orders when you are in Demo Trading.
110101
Trading inverse contracts under Cross and Portfolio modes requires enabling the settlement asset as collateral.
110102
The user does not support trading Inverse contracts - copy trading pro, Ins loan account are not supported
110103
Only Post-Only orders are available at this stage
110104
The LTV for ins Loan has exceeded the limit, and opening inverse contracts is prohibited
110105
The LTV for ins Loan has exceeded the limit, and trading inverse contracts is prohibited
110106
Restrictions on Ins Loan; inverse contracts are not on the whitelist and are not allowed for trading
110107
Restrictions on ins Loan; leverage exceeding the limit for inverse contracts is not allowed.
110108
Allowable range: 1 to 10000 tick size
110109
Allowable range: 0.01% to 10%
110110
Spread trading is not available in isolated margin trading mode
110111
To access spread trading, upgrade to the latest version of UTA
110112
Spread trading is not available for Copy Trading
110113
Spread trading is not available in hedge mode
110114
You have a Spread trading order in progress. Please try again later
110115
The cancellation of a combo single-leg order can only be done by canceling the combo order
110116
The entry price of a single leg, derived from the combo order price, exceeds the limit price
110117
The modification of a combo single-leg order can only be done by modifying the combo order
110118
Unable to retrieve a pruce of the market order due to low liquidity
110119
Order failed. RPI orders are restricted to approved market makers only
110120
Order price cannot be smaller than xxxx, the price limitation
110121
Order price cannot be higher than xxxx, the price limitation
170346
Settle coin is not a collateral coin, cannot trade
170360
symbol
[XXXX]
cannot trade. Used for spread trading in particular when collateral is not turned on
181017
OrderStatus must be final status
182100
Compulsory closing of positions, no repayment allowed
182101
Failed repayment, insufficient collateral balance
182102
Failed repayment, there are no liabilities in the current currency
182103
Institutional lending users are not supported
182108
Switching failed, margin verification failed, please re-adjust the currency status
182110
Failed to switch
182111
The requested currency has a non guaranteed gold currency or does not support switching status currencies
182112
Duplicate currency, please re-adjust
3100181
UID can not be null
3100197
Temporary banned due to the upgrade to UTA
3200316
USDC Options Trading Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for USDC Options trading.
3200317
USDC Options Open Position Restriction: The current LTV ratio for your Institutional Lending has reached the maximum allowable amount for opening USDC Options positions.
3100326
BaseCoin is required
3200403
isolated margin can not create order
3200419
Unable to switch to Portfolio margin due to active pre-market Perpetual orders and positions
3200320
Operations Restriction: The current LTV ratio of your Institutional Lending has hit the liquidation threshold. Assets in your account are being liquidated. (margin mode or spot leverage)
3400208
You have unclosed hedge mode or isolated mode USDT perpetual positions
3400209
You have USDT perpetual positions, so upgrading is prohibited for 10 minutes before and after the hour every hour
3400210
The risk rate of your Derivatives account is too high
3400211
Once upgraded, the estimated risk rate will be too high
3400212
You have USDC perpetual positions or Options positions, so upgrading is prohibited for 10 minutes before and after the hour every hour
3400213
The risk rate of your USDC Derivatives account is too high
3400052
You have uncancelled USDC perpetual orders
3400053
You have uncancelled Options orders
3400054
You have uncancelled USDT perpetual orders
3400214
Server error, please try again later
3400071
The net asset is not satisfied
3401010
Cannot switch to PM mode (for copy trading master trader)
3400139
The total value of your positions and orders has exceeded the risk limit for a Perpetual or Futures contract
34040
Not modified. Indicates you already set this TP/SL value or you didn't pass a required parameter
500010
The subaccount specified does not belong to the parent account
500011
The Uid 592334 provided is not associated with a Unified Trading Account
Spot Trade
​
Code
Description
170001
Internal error.
170005
Too many new orders; current limit is %s orders per %s.
170007
Timeout waiting for response from backend server.
170010
Purchase failed: Exceed the maximum position limit of leveraged tokens, the current available limit is %s USDT
170011
"Purchase failed: Exceed the maximum position limit of innovation tokens,
170019
the current available limit is ''{{.replaceKey0}}'' USDT"
170031
The feature has been suspended
170032
Network error. Please try again later
170033
margin Insufficient account balance
170034
Liability over flow in spot leverage trade!
170035
Submitted to the system for processing!
170036
You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site or the Bybit app
170037
Cross Margin Trading not yet supported by the selected coin
170105
Parameter '%s' was empty.
170115
Invalid timeInForce.
170116
Invalid orderType.
170117
Invalid side.
170121
Invalid symbol.
170124
Order amount too large.
170130
Data sent for paramter '%s' is not valid.
170131
Balance insufficient
170132
Order price too high.
170133
Order price lower than the minimum.
170134
Order price decimal too long.
170371
Order price cannot be lower than {}, the price limitation
170372
Order price cannot be higher than 0, the price limitation
170381
Order quantity too large.
170382
Order quantity too large.
170136
Order quantity lower than the minimum.
170137
Order volume decimal too long
170139
Order has been filled.
170140
Order value exceeded lower limit
170141
Duplicate clientOrderId
170142
Order has been cancelled
170143
Cannot be found on order book
170144
Order has been locked
170145
This order type does not support cancellation
170146
Order creation timeout
170147
Order cancellation timeout
170148
Market order amount decimal too long
170149
Create order failed
170150
Cancel order failed
170151
The trading pair is not open yet
170157
The trading pair is not available for api trading
170159
Market Order is not supported within the first %s minutes of newly launched pairs due to risk control.
170190
Cancel order has been finished
170191
Can not cancel order, please try again later
170192
Order price cannot be higher than %s .
170193
Buy order price cannot be higher than %s.
170194
Sell order price cannot be lower than %s.
170195
Please note that your order may not be filled. ETP buy order price deviates from risk control
170196
Please note that your order may not be filled. ETP sell order price deviates from risk control
170197
Your order quantity to buy is too large. The filled price may deviate significantly from the market price. Please try again
170198
Your order quantity to sell is too large. The filled price may deviate significantly from the market price. Please try again
170199
Your order quantity to buy is too large. The filled price may deviate significantly from the nav. Please try again.
170200
Your order quantity to sell is too large. The filled price may deviate significantly from the nav. Please try again.
170201
Invalid orderFilter parameter
170202
Please enter the TP/SL price.
170203
trigger price cannot be higher than 110% price.
170204
trigger price cannot be lower than 90% of qty.
170206
Stop_limit Order is not supported within the first 5 minutes of newly launched pairs
170207
The loan amount of the platform is not enough.
170210
New order rejected.
170212
Cancel order request processing
170213
Order does not exist.
170215
Spot Trading (Buy) Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for buy orders
170216
The leverage you select for Spot Trading cannot exceed the maximum leverage allowed by Institutional Lending
170217
Only LIMIT-MAKER order is supported for the current pair.
170218
The LIMIT-MAKER order is rejected due to invalid price.
170219
UID {{xxx}} is not available to this feature
170220
Spot Trading Restriction: The current LTV ratio of your institutional lending has reached the maximum allowable amount for Spot trading
170221
This coin does not exist.
170222
Too many requests in this time frame.
170223
Your Spot Account with Institutional Lending triggers an alert or liquidation.
170224
You're not a user of the Innovation Zone.
170226
Your Spot Account for Margin Trading is being liquidated.
170227
This feature is not supported.
170228
The purchase amount of each order exceeds the estimated maximum purchase amount.
170229
The sell quantity per order exceeds the estimated maximum sell quantity.
170230
Operations Restriction: Due to the deactivation of Margin Trading for institutional loan
170234
System Error
170241
To proceed with trading, users must read through and confirm that they fully understand the project's risk disclosure document. For App users, please update your Bybit App to version 4.16.0 to process.
170310
Order modification timeout
170311
Order modification failed
170312
The current order does not support modification
170313
The modified contract quantity cannot be less than to the filled quantity
170341
Request order quantity exceeds maximum limit
170344
Symbol is not supported on Margin Trading
170348
Please go to (
https://www.bybit-tr.com
) to proceed.
170355
RPI orders are restricted to approved market makers only
170358
The current site does not support ETP
170359
TThe current site does not support leveraged trading
170709
OTC loan: The select trading pair is not in the whitelist pair
170810
Cannot exceed maximum of 500 conditional, TP/SL and active orders.
Spot Margin Trade
​
Code
Description
176002
Query user account info error. Confirm that if you have completed quiz in GUI
176003
Query user loan history error
176004
Query order history start time exceeds end time
176005
Failed to borrow
176006
Repayment Failed
176007
User not found
176008
You haven't enabled Cross Margin Trading yet. To do so, please head to the PC trading site
176009
You haven't enabled Cross Margin Trading yet. Confirm that if you have turned on margin trade
176010
Failed to locate the coins to borrow
176011
Cross Margin Trading not yet supported by the selected coin
176012
Pair not available
176013
Cross Margin Trading not yet supported by the selected pair
176014
Repeated repayment requests
176015
Insufficient available balance
176016
No repayment required
176017
Repayment amount has exceeded the total liability
176018
Settlement in progress
176019
Liquidation in progress
176020
Failed to locate repayment history
176021
Repeated borrowing requests
176022
Coins to borrow not generally available yet
176023
Pair to borrow not generally available yet
176024
Invalid user status
176025
Amount to borrow cannot be lower than the min. amount to borrow (per transaction)
176026
Amount to borrow cannot be larger than the max. amount to borrow (per transaction)
176027
Amount to borrow cannot be higher than the max. amount to borrow per user
176028
Amount to borrow has exceeded Bybit's max. amount to borrow
176029
Amount to borrow has exceeded the user's estimated max. amount to borrow
176030
Query user loan info error
176031
Number of decimals for borrow amount has exceeded the maximum precision
176034
The leverage ratio is out of range
176035
Failed to close the leverage switch during liquidation
176036
Failed to adjust leverage switch during forced liquidation
176037
For non-unified transaction users, the operation failed
176038
The spot leverage is closed and the current operation is not allowed
176039
Borrowing, current operation is not allowed
176040
There is a spot leverage order, and the adjustment of the leverage switch failed!
176132
Number of decimals for repay amount has exceeded the maximum precision
176133
Liquidation may be triggered! Please adjust your transaction amount and try again
176134
Account has been upgraded (upgrading) to UTA
176135
Failed to get bond data
176136
Failed to get borrow data
176137
Failed to switch user status
176138
You need to repay all your debts before closing your disabling cross margin account
176139
Sorry, you are not eligible to enable cross margin, as you have already enabled OTC lending
176201
Account exception. Check if the UID is bound to an institutional loan
182021
Cannot enable spot margin while in isolated margin mode. Please switch to cross margin mode or portfolio margin mode to trade spot with margin.
182104
This action could not be completed as your Unified Margin Account's IM/MM utilization rate has exceeded the threshold
182105
Adjustment failed, user is upgrading
182106
Adjustment failed, user forced liquidation in progress.
182107
Adjustment failed, Maintenance Margin Rate too high
Asset
​
Code
Description
131001
openapi svc error
131002
Parameter error
131002
Withdraw address chain or destination tag are not equal
131003
Internal error
131004
KYC needed
131065
Your KYC information is incomplete, please go to the KYC information page of the web or app to complete the information. kyc=India client may encounter this
131066
This address does not support withdrawals for the time being. Please switch to another address for withdrawing
131067
Travel rule verification failed, please contact the target exchange. Travel rule for KR user
131068
Travel rule information is insufficient, please provide additional details. Travel rule for KR user
131069
Unable to withdraw to the receipt, please contact the target the exchange. Travel rule for KR user
131070
The recipient's name is mismatched with the targeted exchange. Travel rule for KR user
131071
The recipient has not undergone KYC verification. Travel rule for KR user
131072
Your withdrawal currency is not supported by the target exchange. Travel rule for KR user
131073
Your withdrawal address has not been included in the target exchange. Travel rule for KR user
131074
Beneficiary info is required, please refer to the latest api document. Travel rule for KR user
131075
InternalAddressCannotBeYourself
131076
internal transfer not support subaccounts
131077
receive user not exist
131078
receive user deposit has been banned
131079
receive user need kyc
131080
User left retry times is zero
131081
Do not input memo/tag,please.
131082
Do not repeat the request
131083
Withdraw only allowed from address book
131084
Withdraw failed because of Uta Upgrading
131085
Withdrawal amount is greater than your availale balance (the deplayed withdrawal is triggered)
131086
Withdrawal amount exceeds risk limit (the risk limit of margin trade is triggered)
131087
your current account spot risk level is too high, withdrawal is prohibited, please adjust and try again
131088
The withdrawal amount exceeds the remaining withdrawal limit of your identity verification level. The current available amount for withdrawal : %s
131089
User sensitive operation, withdrawal is prohibited within 24 hours
131090
User withdraw has been banned
131091
Blocked login status does not allow withdrawals
131092
User status is abnormal
131093
The withdrawal address is not in the whitelist
131094
UserId is not in the whitelist
131095
Withdrawl amount exceeds the 24 hour platform limit
131096
Withdraw amount does not satify the lower limit or upper limit
131097
Withdrawal of this currency has been closed
131098
Withdrawal currently is not availble from new address
131099
Hot wallet status can cancel the withdraw
131200
Service error
131201
Internal error
131202
Invalid memberId
131203
Request parameter error
131204
Account info error
131205
Query transfer error
131206
cannot be transfer
131207
Account not exist
131208
Forbid transfer
131209
Get subMember relation error
131210
Amount accuracy error
131211
fromAccountType can't be the same as toAccountType
131212
Insufficient balance
131213
TransferLTV check error
131214
TransferId exist
131215
Amount error
131216
Query balance error
131217
Risk check error
131226
Due to security reasons, we are unable to proceed with the current action. Should you have any enquiries, please reach out to our Customer Support
131227
subaccount do not have universal transfer permission
131228
your balance is not enough. Please check transfer safe amount
131229
Due to compliance requirements, the current currency is not allowed to transfer
131230
The system is busy, please try again later
131231
Transfers into this account are not supported
131232
Transfers out this account are not supported
131233
can not transfer the coin that not supported for islamic account
140001
Switching the PM spot hedging switch is not allowed in non PM mode
140002
Institutional lending users do not support PM spot hedging
140003
You have position(s) being liquidated, please try again later.
140004
Operations Restriction: The current LTV ratio of your Institutional Loan has hit the liquidation threshold. Assets in your account are being liquidated.
140005
Risk level after switching modes exceeds threshold
141004
sub member is not normal
141025
This subaccount has assets and cannot be deleted
181000
category is null
181001
category only support linear or option or spot.
181002
symbol is null.
181003
side is null.
181004
side only support Buy or Sell.
181005
orderStatus is wrong
181006
startTime is not number
181007
endTime is not number
181008
Parameter startTime and endTime are both needed
181009
Parameter startTime needs to be smaller than endTime
181010
The time range between startTime and endTime cannot exceed 7 days
181011
limit is not a number
181012
symbol not exist
181013
Only support settleCoin: usdc
181014
Classic account is not supported
181018
Invalid expDate.
181019
Parameter expDate can't be earlier than 2 years
182000
symbol related quote price is null
182200
Please upgrade UTA first.
182201
You must enter 2 time parameters.
182202
The start time must be less than the end time
182203
Please enter valid characters
182204
Coin does not exist
182205
User level does not exist
700000
accountType/quoteTxId  cannot be null
700001
quote fail:no dealer can used
700004
order does not exist
700007
Large Amount Limit
700012
UTA upgrading, don't allow to apply for quote
Fiat Convert
​
Code
Description
400000
invalid request.
400001
broker not found.
400002
broker invalid.
400003
broker quotation invalid.
400004
sub-account doesn't exist.
400005
request amount out of quota limit.
400006
funding account not sufficient funds.
400007
sub-account funding account not sufficient funds.
500000
bybit internal error.
Convert Small Balances
​
Code
Description
790000
system error. please try again later
790001
sign verification failed
700000
params error
700001
quote fail:no dealer can used
700002
quote fail:not support quote type
700004
order not exist
700005
Your Available Balance is insufficient or your wallet not exist
700006
Low amount limit
700007
Large amount limit
700008
quote fail: price time out
700009
quoteTxId has already been used
700010
loan user can not perform conversion
700011
illegal operation
700012
uta upgrading,  convert unavailable.
700013
the current coin does not support convert
700016
rate is less than current rate
700021
exist processing exchange order, please try again later
700022
This operation is not currently supported
Crypto Loan (New)
​
Code
Description
148001
This currency is not supported for flexible savings.
148002
The entered amount is below the minimum borrowable amount.
148003
Exceeds the allowed decimal precision for this currency.
148004
This currency cannot be used as collateral.
148005
Exceeds the allowed decimal precision for this collateral currency.
148006
The amount of collateral exceeds the upper limit of the platform.
148007
Borrow amount cannot be negative.
148008
Collateral amount cannot be negative.
148009
LTV exceeds the risk threshold.
148010
Insufficient available quota.
148011
Insufficient balance in the funding pool .
148012
Insufficient collateral amount.
148013
Non-borrowing users cannot adjust collateral.
148014
This currency is not supported.
148015
Loan term exceeds the allowed range.
148016
The specified lending rate is not supported.
148017
The interest rate exceeds the allowed decimal precision.
148018
Exceeded the maximum number of open orders.
148019
The system is busy, please try again later.
148020
Insufficient platform lending quota.
148021
Operation conflict detected. Please try again later.
148022
Insufficient assets for lending.
148023
Loan order not found.
148024
Loan cancellation failed: the order may have been completed or has an invalid amount.
148025
Lending order cancellation failed: the order may have been completed or has an invalid amount.
148026
Failed to create repayment. Please try again later.
148027
No active loan found for this account. Operation not allowed.
148028
Repayment amount exceeds the supported precision for the currency.
148029
Insufficient balance in the repayment account.
148030
Deposit order not found.
148031
Operation not allowed during liquidation.
148032
No outstanding debt. Repayment is not allowed.
148033
This loan order cannot be repaid.
148034
Please wait and try again later.
148035
Please wait and try again later.
148036
Failed to adjust collateral amount. Please try again later.
148037
Insufficient assets or adjustment amount exceeds the maximum allowed.
148038
Repayment amount cannot exceed the debt amount of the position.
148039
Duplicate collateral assets detected. Please review and resubmit.
148040
Pledge token is error.
148041
Repay order is exist.
148042
Exceeds the allowed decimal precision for this currency.
Crypto Loan (legacy)
​
Code
Description
177002
Server is busy, please wait and try again
177003
Illegal characters found in a parameter
177004
Precision is over the maximum defined for this asset
177005
Order does not exist
177006
We don't have this asset
177007
Your borrow amount has exceed maximum borrow amount
177008
Borrow is banned for this asset
177009
Borrow amount is less than minimum borrow amount
177010
Repay amount exceeds borrow amount
177011
Balance is not enough
177012
The system doesn't have enough asset now
177013
adjustment amount exceeds minimum collateral amount
177014
Individual loan quota reached
177015
Collateral amount has reached the limit. Please reduce your collateral amount or try with other collaterals
177016
Minimum collateral amount is not enough
177017
This coin cannot be used as collateral
177018
duplicate request
177019
Your input param is invalid
177020
The account does not support the asset
177021
Repayment failed
Institutional Loan
​
Code
Description
3777002
UID cannot be bound repeatedly.
3777003
UID cannot be unbound because the UID has not been bound to a risk unit.
3777004
The main UID of the risk unit cannot be unbound.
3777005
You have unsettled lending or borrowing orders. Please try again later.
3777006
UID cannot be bound, please try again with a different UID."
3777007
UID cannot be bound, please upgrade to UTA Pro."
3777012
Your request is currently being processed. Please wait and try again later
3777027
UID cannot be bound, leveraged trading closure failed.
3777029
You currently have orders for pre-market trading that can’t be bind UIDs
3777030
This account has activated copyPro and cannot bind uid
3777039
The repayment amount exceeds the outstanding debt, or there is no outstanding liability.
3777040
Insufficient balance.
3777042
The uid is invalid
3777043
There is a repayment order currently being processed. Please try again later.
Exchange Broker
​
Code
Description
3500402
Parameter verification failed for 'limit'.
3500403
Only available to exchange broker main-account
3500404
Invalid Cursor
3500405
Parameter "startTime" and "endTime" need to be input in pairs.
3500406
Out of query time range.
3500407
Parameter begin and end need to be input in pairs.
Reward
​
Code
Description
400001
invalid parameter
400101
The voucher was recycled
400102
The voucher has exceeded the redemption date (expired)
400103
The voucher is not available for redemption
400105
Budget exceeded
403001
Account rejected, check if the input accountId valid, account banned, or kyc issue
404001
resource not found
404011
Insufficient inventory
409011
VIP level limit
500001
Internal server error
Earn
​
Code
Description
180001
Invalid parameter
180002
Invalid coin
180003
User banned
180004
Site not allowed. Only users from Bybit global site can access
180005
Compliance wallet not reach
180006
Validation failed
180007
Product not available
180008
Invalid Product
180009
product is forbidden
180010
User not allowed
180011
User not VIP
180012
Purchase share is invalid
180013
Stake over maximum share
180014
Redeem share invlaid
180015
Products share not enough
180016
Balance not enough
180017
Invalid risk user
180018
internal error
180019
empty order link id
User
​
Code
Description
81007
Bybit Europe is not supported create API Key
20096
need KYC authentication
Set API rate limit
​
Code
Description
3500002
Current user is not an institutional user
3500153
No permission to operate these UIDs
3500153
You do not have permission to query other UIDs
RFQ
​
Code
Description
110300
The RFQ order does not exist
110301
The Quote order does not exist
110302
Demo user is prohibited
110303
RFQ value is less than the min limit
110304
Cannot be self-executed
110305
Quote UID is not in counterparties
110306
Quote legs do not match
110307
Quote order already exists for this RFQ
110308
RFQ strategy legs size is not correct
110309
RFQ strategy side is not correct
110310
RFQ strategy qty is not correct
110311
RFQ strategy symbol is not correct
110312
No permission to execute quote
110313
RFQ only supports one-way position mode
110314
Order amount is less than min trade amount
110315
Order qty exceeds the upper limit
110316
RFQ is not available for Copy Trading
110317
Counterparty cannot be self
110318
There are too many counterparties to choose from
110319
Order amount is greater than max trade amount
110320
Symbols that have not enabled manual loan are not supported
110321
Symbol is not supported
110323
Quotations cannot be made by non-ByBit registration institutions
111008
The spot asset is not be enabled as collateral asset
Manual Loan
​
Code
Description
34022001
System error. Please try again later.
34022003
System error. Please try again later.
34022027
Invalid request parameters.
34022030
Borrowing demand is high, and the fund pool is currently low. Please wait a moment.
34022031
Risk rate limit exceeded. Please reduce your borrow amount in the Unified Trading Account.
34022033
Borrowing precision must be an integer multiple.
34022034
The minimum repayment amount must be an integer multiple.
34022035
You cannot repay while interest is being calculated.
34022036
Please enable Margin Trading to continue.
34022038
Repayment is in progress. Please do not repeat the operation.
34022010
The borrowed asset does not exist.
34022041
Currently, your account has no borrowed coins. No repayments are needed.
34022044
Repayment unsuccessful.
34022045
Borrowing unsuccessful.
34022011
Amount must be at least.
34022014
Decimal precision cannot exceed 18 digits.
34022047
CopyTrade not supported.
34022048
Borrowing is not allowed during liquidation.
34022049
Insufficient collateral balance.
34022050
Repayment failed. You currently have spot hedging liabilities. Please close your derivatives positions before repayment.
34022051
Institutional loan in progress.
34022052
Institutional loan transactions banned.
35000011
You have existing pending loan orders. Please try again later.
34022053
Please contact the sales to enable the manual borrowing feature.
34022094
This coin does not support repayment through coin exchange.

---

## API Rate Limit Rules for VIPs

**URL:** https://bybit-exchange.github.io/docs/v5/rate-limit/rules-for-vips

**Contents:**
- API Rate Limit Rules for VIPs
- API Rate Limit Rules for VIPs​

Rate Limit
API Rate Limit Rules for VIPs
On this page
API Rate Limit Rules for VIPs
API Rate Limit Rules for VIPs
​
Unified Account
Level\Product
Futures
Option
Spot
Default
10/s
10/s
20/s
VIP 1
20/s
20/s
25/s
VIP 2
40/s
40/s
30/s
VIP 3
60/s
60/s
40/s
VIP 4
60/s
60/s
40/s
VIP 5
60/s
60/s
40/s
VIP Supreme
60/s
60/s
40/s

---
