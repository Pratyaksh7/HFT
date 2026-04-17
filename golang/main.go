package main

import (
	"fmt"
	"time"
)

const runSeconds = 30

func main() {
	exchange := NewMockExchange("BTCUSDT", 50000, 100)
	book := &OrderBook{}
	strategy := NewMeanReversionStrategy(20, 2.0) // 20-tick window, 2-stddev bands
	risk := &RiskManager{MaxPosition: 1.0}        // max 1 BTC
	executor := NewExecutor(exchange, risk)

	timeout := time.After(runSeconds * time.Second)
	stopOnce := false

	for {
		select {
		case tick, ok := <-exchange.Ticks():
			if !ok {
				fmt.Printf("Final PnL: %.2f | Final position: %.4f\n", executor.Pnl, executor.Position)
				return
			}
			book.Update(tick)
			if sig := strategy.OnTick(tick); sig != nil {
				t0 := time.Now()
				executor.Submit(tick.Symbol, sig)
				latUs := time.Since(t0).Microseconds()
				fmt.Printf("[%d] %s %.4f@%.2f | pos=%.4f pnl=%.2f lat=%dus\n",
					tick.Ts.UnixMilli(), sig.Side, sig.Qty, tick.Price,
					executor.Position, executor.Pnl, latUs)
			}
		case <-timeout:
			if !stopOnce {
				exchange.Stop()
				stopOnce = true
			}
		}
	}
}
