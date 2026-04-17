// Risk check: reject orders that would blow past a position limit.
// Extend with: max-loss cutoff, per-symbol exposure, max order size,
// hard kill-switch, rate limits, etc.
export class RiskManager {
  constructor(maxPosition) {
    this.maxPosition = maxPosition;
  }

  check(currentPosition, order) {
    const delta = order.side === 'BUY' ? order.qty : -order.qty;
    const newPos = currentPosition + delta;
    return Math.abs(newPos) <= this.maxPosition;
  }
}
