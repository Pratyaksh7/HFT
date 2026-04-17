"""
Mean-Reversion Strategy
-----------------------
Idea: prices tend to revert to a short-term average. When the price
drifts far from that average, we bet on a bounce back.

Each tick:
  1. Keep a rolling window of the last N prices.
  2. Compute the window's mean and standard deviation.
  3. Build "bands": upper = mean + k*std, lower = mean - k*std.
  4. If price < lower -> BUY   (expect bounce up to the mean).
     If price > upper -> SELL  (expect revert down to the mean).
     Otherwise        -> hold.

Good for range-bound / choppy markets.
Bad for strong trends (price keeps going, we keep losing).
"""
import statistics
from collections import deque


class MeanReversionStrategy:
    def __init__(self, window: int = 20, k: float = 2.0):
        self.window = window  # number of ticks in rolling window
        self.k = k            # bands = mean +/- k * stddev
        self.prices: deque[float] = deque(maxlen=window)

    def on_tick(self, tick):
        self.prices.append(tick.price)
        if len(self.prices) < self.window:
            return None  # warming up

        mean = statistics.fmean(self.prices)
        std = statistics.pstdev(self.prices)
        upper = mean + self.k * std
        lower = mean - self.k * std

        if tick.price < lower:
            return {'side': 'BUY', 'qty': 0.01}
        if tick.price > upper:
            return {'side': 'SELL', 'qty': 0.01}
        return None
