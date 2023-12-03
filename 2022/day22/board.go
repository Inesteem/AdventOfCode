package main

import (
	"fmt"
)

const (
	VOID = iota
	WALL
	FREE
	L
	R
	U
	D
)

func get_dir(dir *coord) int {
	if *dir == GO_UP {
		return U
	}
	if *dir == GO_LEFT {
		return L
	}
	if *dir == GO_RIGHT {
		return R
	}
	return D

}

type Field struct {
	m     map[rune]int
	field [][]int
	path  [][]int
}

func Init2DArrayInt(width, height int) [][]int {
	arr := make([][]int, height)
	if width == 0 {
		return arr
	}
	for i := 0; i < height; i++ {
		arr[i] = make([]int, width)
	}
	return arr
}

func InitField(width, height int) Field {
	return Field{
		m:     make(map[rune]int, 0),
		field: Init2DArrayInt(width, height),
		path:  Init2DArrayInt(width, height),
	}
}

func (f *Field) at(r, c *int) int {
	height := len(f.field)
	if *r < 0 {
		*r += height
	} else {
		*r %= height
	}

	width := len(f.field[*r])
	if *c < 0 {
		*c += width
	} else {
		*c %= width
	}

	return f.field[*r][*c]
}

func (f *Field) voidAt(r, c *int) bool {
	return f.at(r, c) == VOID
}
func (f *Field) freeAt(r, c *int) bool {
	return f.at(r, c) == FREE
}

func (f *Field) addMapping(r rune, i int) {
	f.m[r] = i
}

func (f *Field) parseLine(row int, line string) {
	for col := range line {
		val, ok := f.m[rune(line[col])]
		assert(ok, "unmapped; error while parsing "+string(line[col]))
		f.field[row][col] = val
	}
}

func (f *Field) print() {
	for row := range f.field {
		for col := range f.field[row] {
			switch f.path[row][col] {
			case R:
				fmt.Print(">")
				continue
			case U:
				fmt.Print("^")
				continue
			case L:
				fmt.Print("<")
				continue
			case D:
				fmt.Print("v")
				continue
			}
			switch f.field[row][col] {
			case VOID:
				fmt.Print(" ")
				continue
			case WALL:
				fmt.Print("#")
				continue
			case FREE:
				fmt.Print(".")
				continue
			case D:
				fmt.Print("v")
				continue
			}
		}
		fmt.Println()
	}

}
func (f *Field) goIn(pos coord, dir *coord, num int) coord {
	lastFree := pos
	f.path[pos.row][pos.col] = get_dir(dir)
	for ; num > 0; num -= 1 {
		pos = pos.add(dir)
		if f.voidAt(&pos.row, &pos.col) {
			num += 1
			continue
		}
		if f.freeAt(&pos.row, &pos.col) {
			lastFree = pos
			f.path[pos.row][pos.col] = get_dir(dir)
			continue
		}
		break
	}
	return lastFree
}
func (f *Field) findLeftTopFreeTile() coord {

	for r := range f.field {
		for c := range f.field[r] {
			if !f.freeAt(&r, &c) {
				continue
			}
			return coord{
				row: r, col: c,
			}
		}
	}
	assert(false, "not found")
	return coord{}

}

func (f *Field) goInStar2(pos coord, dir *coord, num int, pattern *cubePattern) coord {
	lastFree := pos
	f.path[pos.row][pos.col] = get_dir(dir)
	for ; num > 0; num -= 1 {
		currFace := pattern.getFace(pos)
		pos = pos.add(dir)
		if f.voidAt(&pos.row, &pos.col) {
			fmt.Println(currFace, "->", pattern.getNextFace(currFace, *dir))
			continue
		}
		//	if f.freeAt(&pos.row, &pos.col) {
		lastFree = pos
		f.path[pos.row][pos.col] = get_dir(dir)
		continue
		//	}
		break
	}
	return lastFree
}
