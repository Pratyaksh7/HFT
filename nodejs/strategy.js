// Mean-Reversion Strategy
// -------------------------------------------------------------------
// Idea: prices tend to revert to a short-term average. When the price
// drifts far from that average, we bet on a bounce back.
//
// How it works each tick:
//   1. Maintain a rolling window of the last N prices.
//   2. Compute the window's mean and standard deviation.
//   3. Build "bands": upper = mean + k*std, lower = mean - k*std.
//   4. If price < lower -> BUY  (expect rebound up toward mean).
//      If price > upper -> SELL (expect revert down toward mean).
//      Otherwise        -> hold.
//
// Good for: range-bound / choppy markets.
// Bad for : strong trends (price keeps going, we keep losing).
export class MeanReversionStrategy {
  constructor(window = 20, k = 2.0) {
    this.window = window; // number of ticks in rolling window
    this.k = k;           // bands = mean +/- k * stddev
    this.prices = [];
  }

  onTick(tick) {
    this.prices.push(tick.price);
    if (this.prices.length > this.window) this.prices.shift();
    if (this.prices.length < this.window) return null; // warming up

    const mean = this.prices.reduce((a, b) => a + b, 0) / this.window;
    const variance = this.prices.reduce((a, b) => a + (b - mean) ** 2, 0) / this.window;
    const std = Math.sqrt(variance);

    const upper = mean + this.k * std;
    const lower = mean - this.k * std;

    if (tick.price < lower) return { side: 'BUY', qty: 0.01, symbol: tick.symbol };
    if (tick.price > upper) return { side: 'SELL', qty: 0.01, symbol: tick.symbol };
    return null;
  }
}
