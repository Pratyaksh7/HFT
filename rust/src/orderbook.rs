use crate::exchange::Tick;
use std::time::Instant;

/// Top-of-book tracker. Holds last price + timestamp.
/// Extend with full bid/ask depth for real use.
pub struct OrderBook {
    pub last_price: Option<f64>,
    pub last_ts: Option<Instant>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            last_price: None,
            last_ts: None,
        }
    }

    pub fn update(&mut self, tick: &Tick) {
        self.last_price = Some(tick.price);
        self.last_ts = Some(tick.ts);
    }
}
