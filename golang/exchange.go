package main

import (
	"math/rand"
	"time"
)

type Tick struct {
	Symbol string
	Price  float64
	Ts     time.Time
}

type Order struct {
	Symbol string
	Side   string // "BUY" or "SELL"
	Qty    float64
}

type Fill struct {
	Order     Order
	FillPrice float64
	Ts        time.Time
}

// Exchange interface.
// To swap for a real venue (Binance, Alpaca, etc.):
//   - Ticks(): spawn a goroutine that reads a WebSocket feed and pushes ticks on the channel.
//   - SubmitOrder(): call the venue's REST order endpoint and return the fill.
type Exchange interface {
	Ticks() <-chan Tick
	SubmitOrder(Order) Fill
	Stop()
}

// MockExchange: random-walk feed, instant fills at latest price.
type MockExchange struct {
	symbol   string
	price    float64
	interval time.Duration
	ch       chan Tick
	done     chan struct{}
}

func NewMockExchange(symbol string, basePrice float64, intervalMs int) *MockExchange {
	m := &MockExchange{
		symbol:   symbol,
		price:    basePrice,
		interval: time.Duration(intervalMs) * time.Millisecond,
		ch:       make(chan Tick, 64),
		done:     make(chan struct{}),
	}
	go m.run()
	return m
}

func (m *MockExchange) run() {
	t := time.NewTicker(m.interval)
	defer t.Stop()
	for {
		select {
		case <-m.done:
			close(m.ch)
			return
		case <-t.C:
			// Random walk: price moves +/- 0.1% per tick.
			m.price += (rand.Float64() - 0.5) * 0.002 * m.price
			m.ch <- Tick{Symbol: m.symbol, Price: m.price, Ts: time.Now()}
		}
	}
}

func (m *MockExchange) Ticks() <-chan Tick { return m.ch }

func (m *MockExchange) Stop() {
	select {
	case <-m.done: // already closed
	default:
		close(m.done)
	}
}

// Mock fill at current price, zero slippage, zero latency.
func (m *MockExchange) SubmitOrder(o Order) Fill {
	return Fill{Order: o, FillPrice: m.price, Ts: time.Now()}
}
