use crate::exchange::{Side, Tick};
use std::collections::VecDeque;

pub struct Signal {
    pub side: Side,
    pub qty: f64,
}

/// Mean-Reversion Strategy
/// -----------------------
/// Idea: prices tend to revert to a short-term average. When price drifts
/// far from that average, we bet on a bounce back.
///
/// Each tick:
///   1. Keep a rolling window of the last N prices.
///   2. Compute window mean and stddev.
///   3. Build bands: upper = mean + k*std, lower = mean - k*std.
///   4. price < lower -> BUY   (expect bounce up)
///      price > upper -> SELL  (expect revert down)
///      else          -> hold
///
/// Good for range-bound markets; bad in strong trends.
pub struct MeanReversionStrategy {
    window: usize,
    k: f64,
    prices: VecDeque<f64>,
}

impl MeanReversionStrategy {
    pub fn new(window: usize, k: f64) -> Self {
        Self {
            window,
            k,
            prices: VecDeque::with_capacity(window),
        }
    }

    pub fn on_tick(&mut self, tick: &Tick) -> Option<Signal> {
        self.prices.push_back(tick.price);
        if self.prices.len() > self.window {
            self.prices.pop_front();
        }
        if self.prices.len() < self.window {
            return None; // warming up
        }

        let n = self.window as f64;
        let mean: f64 = self.prices.iter().sum::<f64>() / n;
        let variance: f64 =
            self.prices.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / n;
        let std = variance.sqrt();

        if tick.price < mean - self.k * std {
            Some(Signal {
                side: Side::Buy,
                qty: 0.01,
            })
        } else if tick.price > mean + self.k * std {
            Some(Signal {
                side: Side::Sell,
                qty: 0.01,
            })
        } else {
            None
        }
    }
}
