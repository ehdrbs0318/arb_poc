---
name: bybit-v5-api
description: Bybit V5 API documentation for cryptocurrency trading. Covers market data, order management, position management, account operations, asset management, WebSocket streams, spot margin trading, crypto loans, and more. Supports USDT/USDC perpetual, inverse contracts, spot trading, and options.
---

# Bybit-V5-Api Skill

Bybit v5 api documentation for cryptocurrency trading. covers market data, order management, position management, account operations, asset management, websocket streams, spot margin trading, crypto loans, and more. supports usdt/usdc perpetual, inverse contracts, spot trading, and options., generated from official documentation.

## When to Use This Skill

This skill should be triggered when:
- Working with bybit-v5-api
- Asking about bybit-v5-api features or APIs
- Implementing bybit-v5-api solutions
- Debugging bybit-v5-api code
- Learning bybit-v5-api best practices

## Quick Reference

### Common Patterns

**Pattern 1:** API Endpoint

```
POST /v5/position/confirm-pending-mmrRequest
```

**Pattern 2:** API Endpoint

```
POST /v5/position/confirm-pending-mmr
```

**Pattern 3:** API Endpoint

```
POST /v5/rfq/execute-quoteRequest
```

**Pattern 4:** API Endpoint

```
POST /v5/rfq/execute-quote
```

**Pattern 5:** API Endpoint

```
POST /v5/rfq/cancel-rfqRequest
```

**Pattern 6:** API Endpoint

```
POST /v5/rfq/cancel-rfq
```

**Pattern 7:** API Endpoint

```
GET /v5/market/recent-tradeRequest
```

**Pattern 8:** API Endpoint

```
GET /v5/market/recent-trade?category=spot&symbol=BTCUSDT&limit=1
```

### Example Code Patterns

**Example 1** ():
```
{    "op": "auth",    "args": [        "XXXXXX",        1711010121452,        "ec71040eff72b163a36153d770b69d6637bcb29348fbfbb16c269a76595ececf"    ]}
```

**Example 2** ():
```
{    "retCode": 0,    "retMsg": "OK",    "op": "auth",    "connId": "cnt5leec0hvan15eukcg-2t"}
```

**Example 3** ():
```
GET /v5/market/recent-trade?category=spot&symbol=BTCUSDT&limit=1 HTTP/1.1Host: api-testnet.bybit.com
```

**Example 4** ():
```
from pybit.unified_trading import HTTPsession = HTTP(testnet=True)print(session.get_public_trade_history(    category="spot",    symbol="BTCUSDT",    limit=1,))
```

**Example 5** ():
```
{    "op": "subscribe",    "args": [        "insurance.USDT",        "insurance.USDC"    ]}
```

## Reference Files

This skill includes comprehensive documentation in `references/`:

- **account.md** - Account documentation
- **asset.md** - Asset documentation
- **broker.md** - Broker documentation
- **crypto_loan.md** - Crypto Loan documentation
- **earn.md** - Earn documentation
- **institutional_loan.md** - Institutional Loan documentation
- **introduction.md** - Introduction documentation
- **market_data.md** - Market Data documentation
- **other.md** - Other documentation
- **position.md** - Position documentation
- **reference.md** - Reference documentation
- **rfq_trading.md** - Rfq Trading documentation
- **sbe.md** - Sbe documentation
- **spot_margin_uta.md** - Spot Margin Uta documentation
- **trading.md** - Trading documentation
- **user_management.md** - User Management documentation
- **websocket.md** - Websocket documentation

Use `view` to read specific reference files when detailed information is needed.

## Working with This Skill

### For Beginners
Start with the getting_started or tutorials reference files for foundational concepts.

### For Specific Features
Use the appropriate category reference file (api, guides, etc.) for detailed information.

### For Code Examples
The quick reference section above contains common patterns extracted from the official docs.

## Resources

### references/
Organized documentation extracted from official sources. These files contain:
- Detailed explanations
- Code examples with language annotations
- Links to original documentation
- Table of contents for quick navigation

### scripts/
Add helper scripts here for common automation tasks.

### assets/
Add templates, boilerplate, or example projects here.

## Notes

- This skill was automatically generated from official documentation
- Reference files preserve the structure and examples from source docs
- Code examples include language detection for better syntax highlighting
- Quick reference patterns are extracted from common usage examples in the docs

## Updating

To refresh this skill with updated documentation:
1. Re-run the scraper with the same configuration
2. The skill will be rebuilt with the latest information
