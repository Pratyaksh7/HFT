package main

import "time"

// OrderBook — top-of-book tracker.
// For a real HFT system, extend with full bid/ask depth from the venue's diff stream.
type OrderBook struct {
	LastPrice float64
	LastTs    time.Time
}

func (b *OrderBook) Update(t Tick) {
	b.LastPrice = t.Price
	b.LastTs = t.Ts
}
