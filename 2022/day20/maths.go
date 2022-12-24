package main

import ()

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
