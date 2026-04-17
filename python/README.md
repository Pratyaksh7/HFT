# HFT — Python Stack

Minimal asyncio HFT skeleton. Mock exchange, mean-reversion strategy, paper trading.
Standard library only — no pip install needed.

## Run

```bash
python3 main.py
```

Runs for 30 seconds then prints final PnL. Requires Python 3.10+.

## Files

- `exchange.py`  — `Exchange` base + `MockExchange` (random-walk feed)
- `orderbook.py` — top-of-book tracker
- `strategy.py`  — mean-reversion strategy (annotated)
- `risk.py`      — position-limit check
- `executor.py`  — paper-trading executor, position + PnL
- `main.py`      — entry point and main loop

## Swap in a real exchange

1. In `exchange.py`, create a new subclass of `Exchange`.
2. Implement `ticks()` as an async generator over a WebSocket feed
   (e.g. using the `websockets` library).
3. Implement `submit_order()` to POST to the venue's REST API
   (e.g. via `aiohttp`).
4. Swap `MockExchange(...)` in `main.py` for your new class.
