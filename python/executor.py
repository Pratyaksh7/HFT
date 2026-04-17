"""Paper-trading executor. Tracks position, avg entry, realized PnL."""
from exchange import Order


class Executor:
    def __init__(self, exchange, risk):
        self.exchange = exchange
        self.risk = risk
        self.position: float = 0.0
        self.avg_entry: float = 0.0
        self.pnl: float = 0.0

    async def submit(self, symbol: str, signal: dict):
        order = Order(symbol, signal['side'], signal['qty'])
        if not self.risk.check(self.position, order):
            return None

        fill = await self.exchange.submit_order(order)
        signed_qty = order.qty if order.side == 'BUY' else -order.qty

        # Realize PnL when this order reduces/flips an open position.
        if self.position != 0 and (signed_qty > 0) != (self.position > 0):
            close_qty = min(abs(signed_qty), abs(self.position))
            direction = 1 if self.position > 0 else -1
            self.pnl += (fill.fill_price - self.avg_entry) * close_qty * direction

        # Update avg entry when adding to position; reset when flipping/opening.
        new_pos = self.position + signed_qty
        if new_pos != 0 and (self.position == 0 or (new_pos > 0) == (self.position > 0)):
            self.avg_entry = (
                self.avg_entry * abs(self.position) + fill.fill_price * abs(signed_qty)
            ) / abs(new_pos)
        else:
            self.avg_entry = fill.fill_price

        self.position = new_pos
        return fill
