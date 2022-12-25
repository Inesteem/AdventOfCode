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

//TODO: do this smarter
func (dir *coord) turnLeft() coord {
	if *dir == GO_RIGHT {
		return GO_UP
	}
	if *dir == GO_UP {
		return GO_LEFT
	}
	if *dir == GO_LEFT {
		return GO_DOWN
	}
	return GO_RIGHT
}

func (dir *coord) turnRight() coord {
	if *dir == GO_RIGHT {
		return GO_DOWN
	}
	if *dir == GO_UP {
		return GO_RIGHT
	}
	if *dir == GO_LEFT {
		return GO_UP
	}
	return GO_LEFT
}

var STAY coord = coord{row: 0, col: 0}
var GO_RIGHT coord = coord{row: 0, col: 1}
var GO_LEFT coord = coord{row: 0, col: -1}
var GO_DOWN coord = coord{row: 1, col: 0}
var GO_UP coord = coord{row: -1, col: 0}

type coord3D struct {
	x, y, z int
}

func (c *coord3D) goIn(d coord3D) {
	c.x += d.x
	c.y += d.y
	c.z += d.z
}

//lets chose y as up/down, x as left/right and z as back/front axis
var COORD3D_UP = coord3D{x: 0, y: 1, z: 0}
var COORD3D_DOWN = coord3D{x: 0, y: -1, z: 0}
var COORD3D_RIGHT = coord3D{x: 1, y: 0, z: 0}
var COORD3D_LEFT = coord3D{x: -1, y: 0, z: 0}
var COORD3D_FRONT = coord3D{x: 0, y: 0, z: 1}
var COORD3D_BACK = coord3D{x: 0, y: 0, z: -1}
