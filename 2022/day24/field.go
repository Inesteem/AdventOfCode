package main

import (
	"fmt"
)

const (
	WALL = iota
	FREE
	BLIZZARD
	START
)

var Dirs []coord = []coord{GO_UP, GO_LEFT, GO_RIGHT, GO_DOWN, STAY}

type blizzard struct {
	pos coord
	dir coord
}
type Field struct {
	m         map[rune]int
	blizzards []blizzard
	field     [][]int
	currPos   coord
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
		m: map[rune]int{
			'E': START,
			'#': WALL,
			'.': FREE,
			'^': BLIZZARD,
			'v': BLIZZARD,
			'>': BLIZZARD,
			'<': BLIZZARD,
		},
		blizzards: make([]blizzard, 0),
		field:     Init2DArrayInt(width, height),
	}
}

func (f *Field) at(r, c int) int {
	return f.field[r][c]
}

func (f *Field) BlizzardAt(row, col int) bool {
	c := f.at(row, col)
	return c != WALL && c != FREE
}
func (f *Field) wallAt(r, c int) bool {
	return f.at(r, c) == WALL
}
func (f *Field) freeAt(r, c int) bool {
	return f.at(r, c) == FREE
}

func (f *Field) addMapping(r rune, i int) {
	f.m[r] = i
}

func getDirRune(dir coord) rune {
	switch dir {
	case GO_RIGHT:
		return '>'
	case GO_UP:
		return '^'
	case GO_LEFT:
		return '<'
	}

	assert(dir == GO_DOWN, "wrong dir")
	return 'v'
}

func getDir(r rune) coord {
	switch r {
	case '<':
		return GO_LEFT
	case '>':
		return GO_RIGHT
	case '^':
		return GO_UP
	}
	assert(r == 'v', "wrong rune: "+string(r))
	return GO_DOWN
}

func (f *Field) parseLine(row int, line string) {
	for col := range line {
		val, ok := f.m[rune(line[col])]
		assert(ok, "unmapped; error while parsing "+string(line[col]))
		f.field[row][col+1] = val
		if val == BLIZZARD {
			blizzard := blizzard{
				pos: coord{row: row, col: col + 1},
				dir: getDir(rune(line[col])),
			}
			f.blizzards = append(f.blizzards, blizzard)
		}
	}
}

func (f *Field) print(currPos coord) {
	for row := len(f.field) - 1; row >= 0; row-- {
		for col := range f.field[row] {

			if currPos.row == row && currPos.col == col {
				assert(f.freeAt(row, col), "expedition should be on free field")
				fmt.Print("E")
				continue
			}

			switch f.field[row][col] {
			case BLIZZARD:
				pos := coord{row: row, col: col}
				num := 0
				var dir coord
				for i := range f.blizzards {
					if f.blizzards[i].pos == pos {
						num += 1
						dir = f.blizzards[i].dir
					}
				}
				assert(num > 0, "should find the blizzard")
				if num == 1 {
					fmt.Print(string(getDirRune(dir)))
				} else {
					fmt.Print(num)
				}
				continue
			case WALL:
				fmt.Print("#")
				continue
			case FREE:
				fmt.Print(".")
				continue
			default:
				fmt.Print(" ")
			}

		}
		fmt.Println()
	}

}

func (f *Field) getEmptyPos(row int) coord {
	for col := range f.field[row] {
		if f.field[row][col] == FREE {
			return coord{row: row, col: col}
		}
	}
	assert(false, "there always needs to be a goal in life")
	return coord{}
}
func (f *Field) blizzardsMove() {
	for i := range f.blizzards {
		b := &f.blizzards[i]
		dir := b.dir
		f.field[b.pos.row][b.pos.col] = FREE
		b.pos.goIn(dir)
		if f.at(b.pos.row, b.pos.col) == WALL {
			b.pos.goIn(dir.mulVal(4))
			b.pos.row = pythonModulo(b.pos.row, len(f.field))
			b.pos.col = pythonModulo(b.pos.col, len(f.field[b.pos.row]))
		}
	}
	for i := range f.blizzards {
		b := &f.blizzards[i]
		f.field[b.pos.row][b.pos.col] = BLIZZARD
	}
}

type Item struct {
	step int
	pos  coord
}

type void struct{}

var visit void

func (f *Field) expeditionMove(start, goal coord, lTime int) int {
	visited := make([]map[coord]void, lTime)
	for i := 0; i < lTime; i++ {
		visited[i] = make(map[coord]void, 0)
	}

	todo := []Item{Item{step: 0, pos: start}}
	currStep := -1
	visitedIdx := -1
	for i := 0; i < len(todo); i++ {
		item := &todo[i]

		if item.step > currStep {
			visitedIdx = (visitedIdx + 1) % lTime
			f.blizzardsMove()
			currStep = item.step

		}

		for _, dir := range Dirs {
			nextPos := item.pos
			nextPos.goIn(dir)
			if f.freeAt(nextPos.row, nextPos.col) {
				if _, found := visited[visitedIdx][nextPos]; found {
					continue
				}
				if nextPos.eq(&goal) {
					return currStep + 1
				}
				visited[visitedIdx][nextPos] = visit
				todo = append(todo, Item{step: currStep + 1, pos: nextPos})
			}
		}

	}

	assert(false, "one should reach goal")
	return -1
}
