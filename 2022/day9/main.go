package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

const ropeLen int = 10

type void struct{}

var visit void

func maxVal(a, b int) int {
	if a >= b {
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
func minVal(a, b int) int {
	if a <= b {
		return a
	}
	return b
}

func sign(a int) int {
	if a < 0 {
		return -1
	}
	return 1
}
func assert(cond bool, msg string) {
	if !cond {
		log.Fatal("assertion failed:", msg)
	}
}

func toInt(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
}

func getDiff(a, b int) int {
	if a > b {
		return a - b
	}
	return b - a
}

type point2D struct {
	row, col int
}

var GO_RIGHT point2D = point2D{row: 0, col: 1}
var GO_LEFT point2D = point2D{row: 0, col: -1}
var GO_DOWN point2D = point2D{row: 1, col: 0}
var GO_UP point2D = point2D{row: -1, col: 0}

func (p *point2D) goIn(dir point2D, times int) {
	p.row += dir.row * times
	p.col += dir.col * times
}

func toDir(r byte) point2D {
	switch r {
	case 'D':
		return GO_DOWN
	case 'U':
		return GO_UP
	case 'L':
		return GO_LEFT
	case 'R':
		return GO_RIGHT
	}
	log.Fatalf("toDir: %s not convertible", r)
	return GO_RIGHT
}

func adaptTail(H, T *point2D) {
	dr := H.row - T.row
	sr := sign(dr)
	dc := H.col - T.col
	sc := sign(dc)

	if abs(dr) <= 1 && abs(dc) <= 1 {
		return
	}

	if abs(dr) >= 1 {
		T.row += sr * 1
	}
	if abs(dc) >= 1 {
		T.col += sc * 1
	}
}

func main() {
	rope := make([]point2D, ropeLen)

	visited := make(map[point2D]void)
	visited[rope[0]] = visit

	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		dir := toDir(lineStr[0])
		times := toInt(lineStr[2:])

		for i := 0; i < times; i++ {
			rope[0].goIn(dir, 1)
			for i := 1; i < ropeLen; i += 1 {
				adaptTail(&rope[i-1], &rope[i])
			}
			visited[rope[ropeLen-1]] = visit
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Println("star1 ", len(visited))
}
