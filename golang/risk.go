package main

import "math"

// RiskManager — simple position-limit check.
// Extend with: max loss, per-symbol exposure, kill-switch, rate limits.
type RiskManager struct {
	MaxPosition float64
}

func (r *RiskManager) Check(currentPosition float64, o Order) bool {
	delta := o.Qty
	if o.Side == "SELL" {
		delta = -o.Qty
	}
	return math.Abs(currentPosition+delta) <= r.MaxPosition
}
