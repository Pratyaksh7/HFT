// Paper-trading executor.
// Tracks open position, average entry, and realized PnL.
// Real execution: replace exchange.submitOrder() with a real venue call.
export class Executor {
  constructor(exchange, risk) {
    this.exchange = exchange;
    this.risk = risk;
    this.position = 0;
    this.avgEntry = 0;
    this.pnl = 0;
  }

  async submit(order) {
    if (!this.risk.check(this.position, order)) return null;

    const fill = await this.exchange.submitOrder(order);
    const signedQty = order.side === 'BUY' ? order.qty : -order.qty;

    // If this order reduces or flips the current position, book realized PnL.
    if (this.position !== 0 && Math.sign(signedQty) !== Math.sign(this.position)) {
      const closeQty = Math.min(Math.abs(signedQty), Math.abs(this.position));
      const direction = this.position > 0 ? 1 : -1;
      this.pnl += (fill.fillPrice - this.avgEntry) * closeQty * direction;
    }

    // Update average entry when adding to position; reset when flipping/opening.
    const newPos = this.position + signedQty;
    if (newPos !== 0 && (this.position === 0 || Math.sign(newPos) === Math.sign(this.position))) {
      this.avgEntry =
        (this.avgEntry * Math.abs(this.position) + fill.fillPrice * Math.abs(signedQty)) /
        Math.abs(newPos);
    } else {
      this.avgEntry = fill.fillPrice;
    }

    this.position = newPos;
    return fill;
  }
}
