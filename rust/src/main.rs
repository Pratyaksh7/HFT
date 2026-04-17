mod exchange;
mod executor;
mod orderbook;
mod risk;
mod strategy;

use exchange::MockExchange;
use executor::Executor;
use orderbook::OrderBook;
use risk::RiskManager;
use std::time::{Duration, Instant};
use strategy::MeanReversionStrategy;
use tokio::time::timeout;

const RUN_SECONDS: u64 = 30;

#[tokio::main]
async fn main() {
    let exchange = MockExchange::new("BTCUSDT", 50_000.0, 100);
    let mut book = OrderBook::new();
    let mut strategy = MeanReversionStrategy::new(20, 2.0); // 20-tick window, 2-stddev bands
    let risk = RiskManager { max_position: 1.0 };           // max 1 BTC
    let mut executor = Executor::new(&exchange, &risk);

    let mut rx = exchange.start();
    let deadline = Instant::now() + Duration::from_secs(RUN_SECONDS);

    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match timeout(remaining, rx.recv()).await {
            Ok(Some(tick)) => {
                book.update(&tick);
                if let Some(sig) = strategy.on_tick(&tick) {
                    let t0 = Instant::now();
                    executor.submit(&tick.symbol, &sig).await;
                    let lat_us = t0.elapsed().as_micros();
                    println!(
                        "{:?} {:?} {:.4}@{:.2} | pos={:.4} pnl={:.2} lat={}us",
                        tick.ts, sig.side, sig.qty, tick.price,
                        executor.position, executor.pnl, lat_us
                    );
                }
            }
            _ => break, // timeout or channel closed
        }
    }

    println!(
        "Final PnL: {:.2} | Final position: {:.4}",
        executor.pnl, executor.position
    );
}
