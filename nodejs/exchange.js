import { EventEmitter } from 'events';

// Base Exchange interface.
// To swap for a real exchange (Binance, Alpaca, Coinbase, etc.):
//   1. Subclass `Exchange` (e.g. `class BinanceExchange extends Exchange`).
//   2. In start(), open a WebSocket to the venue's market-data feed.
//   3. On each incoming message, emit a 'tick' event: { symbol, price, ts }.
//   4. In submitOrder(), POST to the REST order endpoint and return the fill.
export class Exchange extends EventEmitter {
  async submitOrder(order) { throw new Error('submitOrder: not implemented'); }
  start(intervalMs) { throw new Error('start: not implemented'); }
  stop() { throw new Error('stop: not implemented'); }
}

// MockExchange: simulates a market feed with a random walk and fills orders
// instantly at the latest price (zero slippage, zero latency).
export class MockExchange extends Exchange {
  constructor(symbol, basePrice) {
    super();
    this.symbol = symbol;
    this.price = basePrice;
    this.timer = null;
  }

  start(intervalMs = 100) {
    this.timer = setInterval(() => {
      // Random walk: price moves +/- 0.1% per tick.
      const drift = (Math.random() - 0.5) * 0.002 * this.price;
      this.price += drift;
      this.emit('tick', { symbol: this.symbol, price: this.price, ts: Date.now() });
    }, intervalMs);
  }

  stop() {
    if (this.timer) clearInterval(this.timer);
    this.timer = null;
  }

  async submitOrder(order) {
    // Mock fill at current price. Real exchange would return fill price from API.
    return { ...order, fillPrice: this.price, status: 'filled', ts: Date.now() };
  }
}
