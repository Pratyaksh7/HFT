"""Wire everything together and run the loop."""
import asyncio
import time

from exchange import MockExchange
from orderbook import OrderBook
from strategy import MeanReversionStrategy
from risk import RiskManager
from executor import Executor

RUN_SECONDS = 30


async def main() -> None:
    exchange = MockExchange('BTCUSDT', 50000)
    book = OrderBook()
    strategy = MeanReversionStrategy(window=20, k=2.0)  # 20-tick window, 2-stddev bands
    risk = RiskManager(max_position=1.0)                # max 1 BTC
    executor = Executor(exchange, risk)

    async def stop_after(seconds: float) -> None:
        await asyncio.sleep(seconds)
        exchange.stop()

    asyncio.create_task(stop_after(RUN_SECONDS))

    async for tick in exchange.ticks():
        book.update(tick)
        signal = strategy.on_tick(tick)
        if not signal:
            continue

        t0 = time.perf_counter_ns()
        await executor.submit(tick.symbol, signal)
        lat_us = (time.perf_counter_ns() - t0) / 1000

        print(
            f"[{tick.ts:.3f}] {signal['side']} {signal['qty']}@{tick.price:.2f} | "
            f"pos={executor.position:.4f} pnl={executor.pnl:.2f} lat={lat_us:.1f}us"
        )

    print(f"Final PnL: {executor.pnl:.2f} | Final position: {executor.position:.4f}")


if __name__ == '__main__':
    asyncio.run(main())
