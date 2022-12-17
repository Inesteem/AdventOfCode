package main

import (
	"strconv"
)

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}
func minVal(a, b int) int {
	if a <= b {
		return a
	}
	return b
}
func maxVal(a, b int) int {
	if a >= b {
		return a
	}
	return b
}
func toInt(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
}
