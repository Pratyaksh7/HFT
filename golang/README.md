# HFT — Go Stack

Minimal goroutine-based HFT skeleton. Mock exchange, mean-reversion strategy, paper trading.
Standard library only — no external modules.

## Run

```bash
go run .
```

Runs for 30 seconds then prints final PnL. Requires Go 1.21+.

## Files

- `exchange.go`  — `Exchange` interface + `MockExchange` (random-walk feed)
- `orderbook.go` — top-of-book tracker
- `strategy.go`  — mean-reversion strategy (annotated)
- `risk.go`      — position-limit check
- `executor.go`  — paper-trading executor, position + PnL
- `main.go`      — entry point, main select loop

## Swap in a real exchange

1. Create a new file implementing the `Exchange` interface (`Ticks`, `SubmitOrder`, `Stop`).
2. In `Ticks()`, spawn a goroutine that reads a WebSocket feed (e.g. via
   `github.com/gorilla/websocket`) and pushes ticks on the returned channel.
3. In `SubmitOrder()`, POST to the venue's REST order endpoint via `net/http`.
4. Replace `NewMockExchange(...)` in `main.go` with your new constructor.
