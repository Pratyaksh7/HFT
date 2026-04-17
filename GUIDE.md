# HFT — Full Guide (4 Independent Stacks)

Minimal runnable High-Frequency Trading skeletons in **Node.js**, **Python**, **Go**, and **Rust**.
Each stack is fully standalone — own files, own build, own run command, no shared code.

---

## 1. Directory Layout

```
hft/
├── README.md        # Short overview
├── GUIDE.md         # This file (full reference)
│
├── nodejs/
│   ├── package.json
│   ├── index.js         # entry point
│   ├── exchange.js      # Exchange base + MockExchange
│   ├── orderbook.js     # top-of-book tracker
│   ├── strategy.js      # mean-reversion strategy (annotated)
│   ├── risk.js          # position-limit check
│   ├── executor.js      # paper-trading executor
│   └── README.md
│
├── python/
│   ├── main.py
│   ├── exchange.py
│   ├── orderbook.py
│   ├── strategy.py
│   ├── risk.py
│   ├── executor.py
│   └── README.md
│
├── golang/
│   ├── go.mod
│   ├── main.go
│   ├── exchange.go
│   ├── orderbook.go
│   ├── strategy.go
│   ├── risk.go
│   ├── executor.go
│   └── README.md
│
└── rust/
    ├── Cargo.toml
    ├── README.md
    └── src/
        ├── main.rs
        ├── exchange.rs
        ├── orderbook.rs
        ├── strategy.rs
        ├── risk.rs
        └── executor.rs
```

---

## 2. Shared Design (Same in All 4 Stacks)

| Module       | Purpose                                                                 |
|--------------|-------------------------------------------------------------------------|
| `exchange`   | `Exchange` abstraction + `MockExchange` (random-walk feed, mock fills)  |
| `orderbook`  | Top-of-book tracker (last price + timestamp)                            |
| `strategy`   | Mean-reversion signal generator                                         |
| `risk`       | Position-limit check before orders submit                               |
| `executor`   | Paper-trading: tracks position, avg entry, realized PnL                 |
| `main/index` | Wires all modules, runs 30s, prints final PnL                           |

### Defaults (identical across stacks)

- Symbol: `BTCUSDT`
- Base price: `50000.0`
- Tick interval: `100ms`
- Mean-reversion window: `20` ticks
- Band width: `k = 2.0` standard deviations
- Order size: `0.01` per signal
- Max position: `1.0`
- Run time: `30s`

### Strategy logic (annotated in code)

1. Keep rolling window of last N prices.
2. Compute mean + stddev.
3. Bands: `upper = mean + k*std`, `lower = mean - k*std`.
4. `price < lower` → **BUY** (expect bounce up).
5. `price > upper` → **SELL** (expect revert down).
6. Else → hold.

Works in range-bound markets. Loses in strong trends.

---

## 3. Run Commands

### Node.js

```bash
cd /home/pratyaksh/Documents/hft/nodejs
node index.js
```
Requires Node.js 18+. No deps.

### Python

```bash
cd /home/pratyaksh/Documents/hft/python
python3 main.py
```
Requires Python 3.10+. Stdlib only.
For live output: `python3 -u main.py` (unbuffered).

### Go

```bash
cd /home/pratyaksh/Documents/hft/golang
go run .
```
Requires Go 1.21+. Stdlib only.
Install Go (if missing): `sudo apt install golang-go` or https://go.dev/dl/

### Rust

```bash
cd /home/pratyaksh/Documents/hft/rust
cargo run --release
```
Requires Rust 1.75+ (stable). Deps: `tokio`, `rand`.
First build pulls + compiles deps (~15s).

---

## 4. Expected Output Format

Every stack logs one line per order submitted:

```
[<timestamp>] <SIDE> <qty>@<price> | pos=<position> pnl=<realized_pnl> lat=<micros>us
```

Example (Python):
```
[1776419658.247] BUY 0.01@49802.94 | pos=0.0100 pnl=0.00 lat=110.6us
[1776419660.768] BUY 0.01@49719.55 | pos=0.0600 pnl=0.00 lat=34.5us
[1776419677543] SELL 0.01@49761.91 | pos=0.0500 pnl=-0.43 lat=101.7us
```

At end of run:
```
Final PnL: <value> | Final position: <value>
```

---

## 5. Observed Latency (Same Laptop, Debug Builds)

| Stack     | Order latency    | Notes                                |
|-----------|------------------|--------------------------------------|
| Rust      | 4 – 12 µs        | Fastest. Release build will be lower.|
| Python    | 17 – 110 µs      | asyncio overhead                     |
| Node.js   | 46 – 717 µs      | First-call JIT warm-up on top end    |
| Go        | (not measured)   | Go toolchain not installed locally   |

These include signal eval + risk check + mock fill + PnL update.

---

## 6. Swapping Mock Exchange → Real Exchange

Each stack isolates all mock behavior in a single file (`exchange.*`). Reusing
existing interfaces, implement two methods against a real venue and nothing else changes.

| Stack     | Real-exchange libs (suggested)                     |
|-----------|----------------------------------------------------|
| Node.js   | `ws` (WebSocket feed) + `fetch` (REST orders)      |
| Python    | `websockets` + `aiohttp`                           |
| Go        | `github.com/gorilla/websocket` + `net/http`        |
| Rust      | `tokio-tungstenite` + `reqwest`                    |

### Node.js
```js
// exchange.js
import WebSocket from 'ws';
export class BinanceExchange extends Exchange {
  start() {
    this.ws = new WebSocket('wss://stream.binance.com:9443/ws/btcusdt@trade');
    this.ws.on('message', (msg) => {
      const t = JSON.parse(msg);
      this.emit('tick', { symbol: 'BTCUSDT', price: parseFloat(t.p), ts: t.T });
    });
  }
  async submitOrder(order) {
    // POST signed request to /api/v3/order ... return fill
  }
}
```

### Python
```python
# exchange.py
import websockets, json
class BinanceExchange(Exchange):
    async def ticks(self):
        async with websockets.connect('wss://stream.binance.com:9443/ws/btcusdt@trade') as ws:
            async for msg in ws:
                t = json.loads(msg)
                yield Tick('BTCUSDT', float(t['p']), t['T'] / 1000)
    async def submit_order(self, order):
        # aiohttp POST to /api/v3/order
        ...
```

### Go
```go
// binance.go
import "github.com/gorilla/websocket"
type BinanceExchange struct { ch chan Tick ; done chan struct{} }
func (b *BinanceExchange) Ticks() <-chan Tick { return b.ch }
func (b *BinanceExchange) SubmitOrder(o Order) Fill { /* http.Post */ }
```

### Rust
```rust
// exchange.rs — replace MockExchange, keep the same public API
use tokio_tungstenite::connect_async;
pub struct BinanceExchange { /* ... */ }
impl BinanceExchange {
    pub fn start(&self) -> tokio::sync::mpsc::Receiver<Tick> { /* ... */ }
    pub async fn submit_order(&self, order: Order) -> Fill { /* reqwest */ }
}
```

Swap only the constructor in `main`/`index`. All other modules (strategy, risk, executor) stay untouched.

---

## 7. What's NOT Included (On Purpose — "Minimal Runnable")

- No real WebSocket / REST clients (mock feed only).
- No persistence — all state in memory, lost on exit.
- No full order book depth (top-of-book only).
- No authentication, signing, or API keys.
- No backtest harness (strategy runs only on live mock feed).
- No concurrency/threading beyond each language's default runtime (asyncio / goroutines / tokio / event loop).
- No test suite.

Each of the above is a good next step when extending the skeleton.

---

## 8. Extension Checklist (Ideas)

- [ ] Full depth order book with `BTreeMap`/sorted-map per side
- [ ] Add backtesting mode (replay historical CSV/parquet ticks)
- [ ] Persist fills + PnL to SQLite / Postgres
- [ ] More strategies: momentum, spread arb, pairs trading
- [ ] Risk: drawdown limit, circuit breaker, kill switch
- [ ] Metrics: Prometheus exporter for latency / PnL / position
- [ ] Config file (YAML / JSON) instead of hard-coded constants
- [ ] Unit tests for strategy + executor math
- [ ] Dockerfile per stack
- [ ] CI: run all 4 stacks in a quick smoke test

---

## 9. Quick Troubleshooting

| Problem                                  | Fix                                                         |
|------------------------------------------|-------------------------------------------------------------|
| Python: no output for first ~2s          | Normal. Strategy needs 20 ticks (2s) to warm up.            |
| Python: no output when piped into `head` | Use `python3 -u main.py` to disable stdout buffering.       |
| Go: `go: command not found`              | Install Go: https://go.dev/dl/ or `sudo apt install golang` |
| Rust: first run is slow                  | `cargo build --release` once, then reruns are instant.      |
| No trades in 5s                          | Random walk hasn't exceeded 2-stddev yet. Wait longer.      |

---

## 10. One-Shot Smoke Test All Stacks

```bash
cd /home/pratyaksh/Documents/hft

# Node
( cd nodejs && timeout 12 node index.js )

# Python
( cd python && timeout 12 python3 -u main.py )

# Go (if installed)
( cd golang && timeout 12 go run . )

# Rust
( cd rust && cargo build --release --quiet && timeout 12 ./target/release/hft )
```

Each should print several order lines and a final `Final PnL: ...` summary.
