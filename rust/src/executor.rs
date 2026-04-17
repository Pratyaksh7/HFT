use crate::exchange::{Fill, MockExchange, Order, Side};
use crate::risk::RiskManager;
use crate::strategy::Signal;

/// Paper-trading executor.
/// Tracks open position, average entry, and realized PnL.
pub struct Executor<'a> {
    exchange: &'a MockExchange,
    risk: &'a RiskManager,
    pub position: f64,
    pub avg_entry: f64,
    pub pnl: f64,
}

impl<'a> Executor<'a> {
    pub fn new(exchange: &'a MockExchange, risk: &'a RiskManager) -> Self {
        Self {
            exchange,
            risk,
            position: 0.0,
            avg_entry: 0.0,
            pnl: 0.0,
        }
    }

    pub async fn submit(&mut self, symbol: &str, sig: &Signal) -> Option<Fill> {
        let order = Order {
            symbol: symbol.to_string(),
            side: sig.side,
            qty: sig.qty,
        };
        if !self.risk.check(self.position, &order) {
            return None;
        }

        let fill = self.exchange.submit_order(order).await;
        let signed_qty = if sig.side == Side::Buy { sig.qty } else { -sig.qty };

        // Realize PnL when this order reduces/flips an open position.
        if self.position != 0.0 && signed_qty.signum() != self.position.signum() {
            let close_qty = signed_qty.abs().min(self.position.abs());
            let direction = self.position.signum();
            self.pnl += (fill.fill_price - self.avg_entry) * close_qty * direction;
        }

        // Update avg entry when adding to position; reset on flip/open.
        let new_pos = self.position + signed_qty;
        if new_pos != 0.0
            && (self.position == 0.0 || new_pos.signum() == self.position.signum())
        {
            self.avg_entry = (self.avg_entry * self.position.abs()
                + fill.fill_price * signed_qty.abs())
                / new_pos.abs();
        } else {
            self.avg_entry = fill.fill_price;
        }
        self.position = new_pos;
        Some(fill)
    }
}
