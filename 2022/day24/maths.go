package main

import ()

const MaxUint = ^uint(0)
const MinUint = 0
const MaxInt = int(MaxUint >> 1)
const MinInt = -MaxInt - 1

//TODO :better name, I think it was something like arithmetic modulo or so
func pythonModulo(i, m int) int {
	return ((i % m) + m) % m
}
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
