package main

import (
	"fmt"
)

// returns upper bound to the loop time, which is the number of iterations
// after which the start state of blizzards was reached again
// There could be found a smaller upper bound by comparing the field with start
// but this should suffice
func getLoopTime(field *Field) int {
	min := minVal(len(field.field), len(field.field[0])) - 4
	max := maxVal(len(field.field), len(field.field[0])) - 4
	if min == max {
		return min
	}

	upperBound := min * max
	for i := max; i < upperBound; i += max {
		if i%min == 0 {
			return i
		}
	}

	return upperBound
}

func main() {
	lines, _ := readLines("input")

	field := InitField(len(lines[0])+2, len(lines)+2)

	for row := range lines {
		field.parseLine(len(lines)-row, lines[row])
	}

	start := field.getEmptyPos(len(field.field) - 2)
	goal := field.getEmptyPos(1)
	lTime := getLoopTime(&field)

	minSteps := field.expeditionMove(start, goal, lTime)
	fmt.Println("star1: ", minSteps)
	minSteps += field.expeditionMove(goal, start, lTime)
	minSteps += field.expeditionMove(start, goal, lTime)
	fmt.Println("star2: ", minSteps)
}
