"""Top-of-book tracker.

Holds only the last traded price + timestamp. For a real HFT system,
extend to maintain full bid/ask depth from the venue's diff stream.
"""


class OrderBook:
    def __init__(self) -> None:
        self.last_price: float | None = None
        self.last_ts: float | None = None

    def update(self, tick) -> None:
        self.last_price = tick.price
        self.last_ts = tick.ts
