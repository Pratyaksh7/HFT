# HFT — 4 Independent Stacks

Each stack is runnable standalone with identical feature set across languages.

- `nodejs/` — Node.js, ES modules
- `python/` — asyncio, stdlib only
- `golang/` — goroutines, stdlib only
- `rust/` — tokio

## Features (per stack)

- Mock exchange with swappable interface for real exchange (Binance/Alpaca/etc.)
- Top-of-book tracker
- Mean-reversion strategy (commented for learning)
- Position-limit risk check
- Paper-trading executor (position + realized PnL)
- Per-order latency logging

## Run

See each stack's README.md.

## Swap to real exchange

Every stack isolates mock behavior behind a single `Exchange` abstraction. Implement the same two methods (`ticks` / feed + `submit_order`) against a real API.
# HFT
# HFT
