"""Risk check: reject orders that would exceed position limits.

Extend with: max-loss cutoff, per-symbol exposure, max order size,
hard kill-switch, rate limits, etc.
"""


class RiskManager:
    def __init__(self, max_position: float):
        self.max_position = max_position

    def check(self, current_position: float, order) -> bool:
        delta = order.qty if order.side == 'BUY' else -order.qty
        return abs(current_position + delta) <= self.max_position
