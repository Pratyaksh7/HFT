use crate::exchange::{Order, Side};

/// Position-limit check.
/// Extend with: max loss, per-symbol exposure, kill-switch, rate limits.
pub struct RiskManager {
    pub max_position: f64,
}

impl RiskManager {
    pub fn check(&self, current_position: f64, order: &Order) -> bool {
        let delta = if order.side == Side::Buy {
            order.qty
        } else {
            -order.qty
        };
        (current_position + delta).abs() <= self.max_position
    }
}
