// Top-of-book tracker. Holds last traded price + timestamp.
// For a real HFT system, extend this to maintain full bid/ask depth
// from the venue's depth/diff stream.
export class OrderBook {
  constructor() {
    this.lastPrice = null;
    this.lastTs = null;
  }

  update(tick) {
    this.lastPrice = tick.price;
    this.lastTs = tick.ts;
  }
}
