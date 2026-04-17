# HFT — Node.js Stack

Minimal runnable HFT skeleton. Mock exchange, mean-reversion strategy, paper trading.
No external dependencies — pure Node.js (ES modules).

## Run

```bash
node index.js
```

Runs for 30 seconds then prints final PnL. Requires Node.js 18+.

## Files

- `exchange.js`  — `Exchange` base + `MockExchange` (random-walk feed)
- `orderbook.js` — top-of-book tracker
- `strategy.js`  — mean-reversion strategy (annotated)
- `risk.js`      — position-limit check
- `executor.js`  — paper-trading executor, position + PnL
- `index.js`     — wires everything together and runs the loop

## Swap in a real exchange

1. In `exchange.js`, subclass `Exchange` (e.g. `BinanceExchange`).
2. In `start()`, open a WebSocket (e.g. `ws` package) to the venue's feed and emit
   `tick` events: `{ symbol, price, ts }`.
3. In `submitOrder()`, POST to the REST order endpoint (e.g. via `fetch`).
4. Replace `new MockExchange(...)` in `index.js` with your new class.
