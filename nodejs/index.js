import { MockExchange } from './exchange.js';
import { OrderBook } from './orderbook.js';
import { MeanReversionStrategy } from './strategy.js';
import { RiskManager } from './risk.js';
import { Executor } from './executor.js';

const RUN_SECONDS = 30;

const exchange = new MockExchange('BTCUSDT', 50000);
const book = new OrderBook();
const strategy = new MeanReversionStrategy(20, 2.0); // window=20 ticks, k=2 stddev
const risk = new RiskManager(1.0);                    // max 1 BTC position
const executor = new Executor(exchange, risk);

exchange.on('tick', async (tick) => {
  book.update(tick);
  const signal = strategy.onTick(tick);
  if (!signal) return;

  const t0 = process.hrtime.bigint();
  await executor.submit(signal);
  const latencyUs = Number(process.hrtime.bigint() - t0) / 1000;

  console.log(
    `[${tick.ts}] ${signal.side} ${signal.qty}@${tick.price.toFixed(2)} | ` +
    `pos=${executor.position.toFixed(4)} pnl=${executor.pnl.toFixed(2)} lat=${latencyUs.toFixed(1)}us`
  );
});

exchange.start(100); // one tick every 100ms

setTimeout(() => {
  exchange.stop();
  console.log(`Final PnL: ${executor.pnl.toFixed(2)} | Final position: ${executor.position.toFixed(4)}`);
  process.exit(0);
}, RUN_SECONDS * 1000);
