"""Exchange abstraction + mock implementation."""
import asyncio
import random
import time
from dataclasses import dataclass
from typing import AsyncIterator


@dataclass
class Tick:
    symbol: str
    price: float
    ts: float


@dataclass
class Order:
    symbol: str
    side: str  # 'BUY' or 'SELL'
    qty: float


@dataclass
class Fill:
    order: Order
    fill_price: float
    ts: float


class Exchange:
    """Base interface.

    To swap in a real exchange (Binance, Alpaca, Coinbase, ...):
      - Implement `ticks()` as an async generator that reads a WebSocket
        feed (e.g. using the `websockets` library) and yields `Tick`s.
      - Implement `submit_order()` to POST to the venue's REST API
        (e.g. via `aiohttp`) and return a `Fill`.
    """

    async def ticks(self) -> AsyncIterator[Tick]:
        raise NotImplementedError
        yield  # pragma: no cover  (makes this an async generator)

    async def submit_order(self, order: Order) -> Fill:
        raise NotImplementedError


class MockExchange(Exchange):
    """Simulates a market feed via random walk, fills orders instantly."""

    def __init__(self, symbol: str, base_price: float, interval_ms: int = 100):
        self.symbol = symbol
        self.price = base_price
        self.interval = interval_ms / 1000.0
        self._running = True

    def stop(self) -> None:
        self._running = False

    async def ticks(self) -> AsyncIterator[Tick]:
        while self._running:
            # Random walk: price moves +/- 0.1% per tick.
            self.price += (random.random() - 0.5) * 0.002 * self.price
            yield Tick(self.symbol, self.price, time.time())
            await asyncio.sleep(self.interval)

    async def submit_order(self, order: Order) -> Fill:
        # Mock fill at current price. Zero slippage, zero latency.
        return Fill(order, self.price, time.time())
