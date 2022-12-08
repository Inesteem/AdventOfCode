package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func maxVal(a, b int) int {
	if a >= b {
		return a
	}
	return b
}

func minVal(a, b int) int {
	if a <= b {
		return a
	}
	return b
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func fillArr(arr []int, val int) []int {

	for i := range arr {
		arr[i] = val
	}
	return arr
}
func assert(cond bool, msg string) {
	if !cond {
		log.Fatal("assertion failed:", msg)
	}
}

func readLines(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

func getTrees(lines []string) [][]int {
	trees := make([][]int, len(lines))
	for r, line := range lines {
		trees[r] = make([]int, len(line))
		for c, d := range []rune(line) {
			trees[r][c] = int(d) - '0'
			assert(trees[r][c] <= 9, "tree not between 0 - 9, but "+string(d))
		}

	}
	return trees
}

type check struct {
	star    int
	maxVals []int
}

// todo better name
type point2D struct {
	row, col int
}

func (p point2D) valid(numRows, numCols int) bool {
	if p.row < 0 || p.col < 0 {
		return false
	}
	return p.row < numRows && p.col < numCols
}

var GO_RIGHT point2D = point2D{row: 0, col: 1}
var GO_LEFT point2D = point2D{row: 0, col: -1}
var GO_DOWN point2D = point2D{row: 1, col: 0}
var GO_UP point2D = point2D{row: -1, col: 0}

func getDiff(a, b int) int {
	if a > b {
		return a - b
	}
	return b - a
}

func getDist(pos, dir point2D, b int) int {
	if dir.row != 0 {
		return getDiff(pos.row, b)
	}
	return getDiff(pos.col, b)
	//r := abs(dir.row)
	//c := abs(dir.col)
	//minDist := (pos.row*r + pos.col*c) - (r*b + c*b)
	//return abs(minDist)
}

func set(trees, visible [][]int, check check, pos, dir point2D) {
	if !pos.valid(len(visible), len(visible[0])) {
		return
	}
	r := pos.row
	c := pos.col
	if check.star == 1 {
		if trees[r][c] > check.maxVals[0] {
			visible[r][c] = 1
		}
		check.maxVals[0] = maxVal(check.maxVals[0], trees[r][c])
	} else {
		minDist := 10000
		for i := trees[r][c]; i < 10; i++ {
			minDist = minVal(getDist(pos, dir, check.maxVals[i]), minDist)
		}
		check.maxVals[trees[r][c]] = pos.row //abs(dir.row*pos.row + dir.col*pos.col)
		if dir.col != 0 {
			check.maxVals[trees[r][c]] = pos.col
		}
		visible[r][c] *= minDist
	}

	pos.row += dir.row
	pos.col += dir.col
	set(trees, visible, check, pos, dir)

}

func getVisibility(trees [][]int, star int) [][]int {
	maxR := len(trees)
	maxC := len(trees[0])
	visible := make([][]int, len(trees))

	check := check{star: star, maxVals: make([]int, 10)}
	for r := 0; r < maxR; r++ {
		visible[r] = make([]int, len(trees[r]))

		if star == 2 {
			fillArr(visible[r], 1)
		}

		//left -> right
		fillArr(check.maxVals, star-2)
		set(trees, visible, check,
			point2D{row: r, col: 0},
			GO_RIGHT)

		//right -> left
		if star == 1 {
			fillArr(check.maxVals, -1)
		} else {
			fillArr(check.maxVals, maxC-1)
		}
		set(trees, visible, check,
			point2D{row: r, col: maxC - 1},
			GO_LEFT)
	}

	for c := 0; c < maxC; c++ {
		// down -> up
		fillArr(check.maxVals, star-2)
		set(trees, visible, check,
			point2D{row: 0, col: c},
			GO_DOWN)
		// up -> down
		if star == 1 {
			fillArr(check.maxVals, -1)
		} else {
			fillArr(check.maxVals, maxR-1)
		}
		set(trees, visible, check,
			point2D{row: maxR - 1, col: c},
			GO_UP)
	}

	return visible
}

func getNumVisible(trees [][]int, star int) int {
	visible := getVisibility(trees, star)
	//TODO check borders
	val := 0
	for r := 0; r < len(visible); r++ {
		for c := 0; c < len(visible[r]); c++ {
			if star == 1 {
				val += visible[r][c]
			} else {
				val = maxVal(val, visible[r][c])
			}
		}
	}
	return val
}

// 8
func main() {

	lines, err := readLines("input")
	if err != nil {
		log.Fatal(err)
	}

	trees := getTrees(lines)
	fmt.Println("star1:", getNumVisible(trees, 1))
	fmt.Println("star2:", getNumVisible(trees, 2))

} // 1779
