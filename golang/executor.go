package main

import "math"

// Executor — paper trading.
// Tracks open position, average entry price, and realized PnL.
type Executor struct {
	exchange Exchange
	risk     *RiskManager
	Position float64
	AvgEntry float64
	Pnl      float64
}

func NewExecutor(ex Exchange, risk *RiskManager) *Executor {
	return &Executor{exchange: ex, risk: risk}
}

func (e *Executor) Submit(symbol string, sig *Signal) *Fill {
	order := Order{Symbol: symbol, Side: sig.Side, Qty: sig.Qty}
	if !e.risk.Check(e.Position, order) {
		return nil
	}

	fill := e.exchange.SubmitOrder(order)
	signedQty := order.Qty
	if order.Side == "SELL" {
		signedQty = -order.Qty
	}

	// Realize PnL when this order reduces/flips an open position.
	if e.Position != 0 && ((signedQty > 0) != (e.Position > 0)) {
		closeQty := math.Min(math.Abs(signedQty), math.Abs(e.Position))
		direction := 1.0
		if e.Position < 0 {
			direction = -1.0
		}
		e.Pnl += (fill.FillPrice - e.AvgEntry) * closeQty * direction
	}

	// Update avg entry when adding to position; reset on flip/open.
	newPos := e.Position + signedQty
	if newPos != 0 && (e.Position == 0 || ((newPos > 0) == (e.Position > 0))) {
		e.AvgEntry = (e.AvgEntry*math.Abs(e.Position) + fill.FillPrice*math.Abs(signedQty)) / math.Abs(newPos)
	} else {
		e.AvgEntry = fill.FillPrice
	}
	e.Position = newPos
	return &fill
}
