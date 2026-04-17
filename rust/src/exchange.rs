use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
    pub ts: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub side: Side,
    pub qty: f64,
}

#[derive(Debug)]
pub struct Fill {
    pub order: Order,
    pub fill_price: f64,
    pub ts: Instant,
}

/// MockExchange: random-walk feed + mock fills at latest price.
///
/// To swap for a real venue (Binance, Alpaca, Coinbase, ...):
///   - Replace this struct with one backed by `tokio-tungstenite` (WebSocket)
///     for the market-data feed and `reqwest` (REST) for order submission.
///   - Keep the same public API: `start()` returns `mpsc::Receiver<Tick>`,
///     `submit_order()` returns `Fill`.
pub struct MockExchange {
    symbol: String,
    price: Arc<Mutex<f64>>,
    interval_ms: u64,
}

impl MockExchange {
    pub fn new(symbol: &str, base_price: f64, interval_ms: u64) -> Self {
        Self {
            symbol: symbol.to_string(),
            price: Arc::new(Mutex::new(base_price)),
            interval_ms,
        }
    }

    pub fn start(&self) -> mpsc::Receiver<Tick> {
        let (tx, rx) = mpsc::channel(64);
        let symbol = self.symbol.clone();
        let price = self.price.clone();
        let interval_ms = self.interval_ms;

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_millis(interval_ms));
            loop {
                ticker.tick().await;
                let new_price = {
                    // Random walk: price moves +/- 0.1% per tick.
                    let drift = (rand::thread_rng().gen::<f64>() - 0.5) * 0.002;
                    let mut p = price.lock().unwrap();
                    *p += drift * *p;
                    *p
                };
                if tx
                    .send(Tick {
                        symbol: symbol.clone(),
                        price: new_price,
                        ts: Instant::now(),
                    })
                    .await
                    .is_err()
                {
                    break; // receiver dropped
                }
            }
        });

        rx
    }

    /// Mock fill at current price. Zero slippage, zero latency.
    pub async fn submit_order(&self, order: Order) -> Fill {
        let fill_price = *self.price.lock().unwrap();
        Fill {
            order,
            fill_price,
            ts: Instant::now(),
        }
    }
}
