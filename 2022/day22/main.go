package main

import (
	"fmt"
)

func parseInt(line string, idx int) (nextIdx, num int) {
	for idx < len(line) && int(line[idx]) >= int('0') && int(line[idx]) <= int('9') {
		num *= 10
		num += int(line[idx]) - int('0')
		idx += 1
	}
	return idx, num
}

func star1(pos coord, dir coord) int {
	points := 0
	if dir == GO_DOWN {
		points = 1
	} else if dir == GO_LEFT {
		points = 2
	} else if dir == GO_UP {
		points = 3
	}

	points += 1000 * (pos.row + 1)
	points += 4 * (pos.col + 1)
	return points
}

func main() {
	lines, _ := readLines("input")

	maxRow := 0
	for i := range lines[0 : len(lines)-2] {
		maxRow = maxVal(len(lines[i]), maxRow)
	}

	field := InitField(maxRow, len(lines)-2)
	field.addMapping(' ', VOID)
	field.addMapping('#', WALL)
	field.addMapping('.', FREE)

	for row := range lines[0 : len(lines)-2] {
		field.parseLine(row, lines[row])
	}

	cubeWidth := getCubeWidth(&field)
	cubeHeight := getCubeHeight(&field)
	fmt.Println(getCubePattern(&field, cubeWidth, cubeHeight))
	return
	//star1

	pos := field.findLeftTopFreeTile()
	dir := GO_RIGHT
	cmd := lines[len(lines)-1]
	num := 0

	for r := 0; r < len(cmd); r += 1 {
		r, num = parseInt(cmd, r)
		pos = field.goIn(pos, &dir, num)
		if r < len(cmd) {
			if cmd[r] == 'R' {
				dir = dir.turnRight()
			} else {
				dir = dir.turnLeft()
			}
		}
	}
	fmt.Println(pos)

	fmt.Println("star1", star1(pos, dir))
}
