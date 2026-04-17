# HFT — Rust Stack

Minimal tokio-based HFT skeleton. Mock exchange, mean-reversion strategy, paper trading.

## Run

```bash
cargo run --release
```

Runs for 30 seconds then prints final PnL. Requires Rust 1.75+ (stable).

## Files

- `src/exchange.rs`  — `MockExchange` (random-walk feed, mock fills)
- `src/orderbook.rs` — top-of-book tracker
- `src/strategy.rs`  — mean-reversion strategy (annotated)
- `src/risk.rs`      — position-limit check
- `src/executor.rs`  — paper-trading executor, position + PnL
- `src/main.rs`      — entry point, main loop

## Dependencies

- `tokio` — async runtime
- `rand`  — random walk for mock feed

## Swap in a real exchange

1. Replace `MockExchange` in `src/exchange.rs` with a struct keeping the same
   public API: `start() -> mpsc::Receiver<Tick>` and `submit_order(Order) -> Fill`.
2. Use `tokio-tungstenite` for the WebSocket market-data feed.
3. Use `reqwest` for REST order submission.
4. No other files need to change.
