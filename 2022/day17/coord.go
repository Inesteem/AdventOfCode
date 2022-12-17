package main

import ()

type coord struct {
	row, col int
}

func (c *coord) add(o *coord) coord {
	return coord{
		col: c.col + o.col,
		row: c.row + o.row,
	}
}

func (p *coord) goIn(dir coord) {
	p.row += dir.row
	p.col += dir.col
}

func (c *coord) eq(o *coord) bool {
	return c.col == o.col && c.row == o.row
}
func (c *coord) dist(o *coord) int {
	return maxVal(c.col, o.col) - minVal(c.col, o.col) + maxVal(c.row, o.row) - minVal(c.row, o.row)
}

func (c coord) subVal(val int) coord {
	c.col -= val
	c.row -= val
	return c
}
func (c coord) addVal(val int) coord {
	c.col += val
	c.row += val
	return c
}
func (c coord) mulVal(val int) coord {
	c.col *= val
	c.row *= val
	return c
}
func (c *coord) manhattenDistance(o *coord) int {
	return abs(c.col-o.col) + abs(c.row-o.row)
}

func (c *coord) setMaxVals(o coord) {
	c.col = maxVal(c.col, o.col)
	c.row = maxVal(c.row, o.row)
}
func (c *coord) setMinVals(o coord) {
	c.col = minVal(c.col, o.col)
	c.row = minVal(c.row, o.row)
}

var STAY coord = coord{row: 0, col: 0}
var GO_RIGHT coord = coord{row: 0, col: 1}
var GO_LEFT coord = coord{row: 0, col: -1}
var GO_DOWN coord = coord{row: -1, col: 0}
var GO_UP coord = coord{row: 1, col: 0}
var GO_DOWN_RIGHT coord = coord{row: -1, col: 1}
var GO_DOWN_LEFT coord = coord{row: -1, col: -1}
var GO_UP_RIGHT coord = coord{row: 1, col: 1}
var GO_UP_LEFT coord = coord{row: 1, col: -1}
