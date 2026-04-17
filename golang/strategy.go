package main

import "math"

// Mean-Reversion Strategy
// -----------------------
// Idea: prices tend to revert to a short-term average. When price drifts
// far from that average, we bet on a bounce back.
//
// Each tick:
//   1. Keep a rolling window of the last N prices.
//   2. Compute window mean and stddev.
//   3. Build bands: upper = mean + k*std, lower = mean - k*std.
//   4. price < lower -> BUY   (expect bounce up)
//      price > upper -> SELL  (expect revert down)
//      else          -> hold
//
// Good for range-bound markets; bad in strong trends.
type MeanReversionStrategy struct {
	window int
	k      float64
	prices []float64
}

type Signal struct {
	Side string
	Qty  float64
}

func NewMeanReversionStrategy(window int, k float64) *MeanReversionStrategy {
	return &MeanReversionStrategy{
		window: window,
		k:      k,
		prices: make([]float64, 0, window),
	}
}

func (s *MeanReversionStrategy) OnTick(t Tick) *Signal {
	s.prices = append(s.prices, t.Price)
	if len(s.prices) > s.window {
		s.prices = s.prices[1:]
	}
	if len(s.prices) < s.window {
		return nil // warming up
	}

	var sum float64
	for _, p := range s.prices {
		sum += p
	}
	mean := sum / float64(s.window)

	var variance float64
	for _, p := range s.prices {
		variance += (p - mean) * (p - mean)
	}
	std := math.Sqrt(variance / float64(s.window))

	if t.Price < mean-s.k*std {
		return &Signal{Side: "BUY", Qty: 0.01}
	}
	if t.Price > mean+s.k*std {
		return &Signal{Side: "SELL", Qty: 0.01}
	}
	return nil
}
