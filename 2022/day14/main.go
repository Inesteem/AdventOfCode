package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

type stuff int

var space stuff = 0
var wall stuff = 1
var sand stuff = 2
var route stuff = 3

func input() {

	var b []byte = make([]byte, 1)
	os.Stdin.Read(b)
}

func toInt(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
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
func assert(cond bool, msg string) {
	if !cond {
		log.Fatal("assertion failed:", msg)
	}
}

type coord struct {
	row, col int
}

var GO_UP_STRAIGHT coord = coord{row: -1, col: 0}
var GO_DOWN_STRAIGHT coord = coord{row: 1, col: 0}
var GO_DOWN_RIGHT coord = coord{row: 1, col: 1}
var GO_DOWN_LEFT coord = coord{row: 1, col: -1}

var dirs [3]coord = [3]coord{GO_DOWN_STRAIGHT, GO_DOWN_LEFT, GO_DOWN_RIGHT}

func (p *coord) setMin(c coord) {
	p.row = minVal(p.row, c.row)
	p.col = minVal(p.col, c.col)
}
func (p *coord) setMax(c coord) {
	p.row = maxVal(p.row, c.row)
	p.col = maxVal(p.col, c.col)
}
func (p *coord) goIn(dir coord) {
	p.row += dir.row
	p.col += dir.col
}

func newCoord(c string) coord {
	pos := strings.Split(c, ",")
	assert(len(pos) == 2, "wrongly parsed coord: "+c)

	//normalize such that sand is pooring from center of world 0 0
	return coord{col: toInt(pos[0]) - 500, row: toInt(pos[1])}
}

type floorList []int
type world struct {
	blocked  map[coord]stuff
	colFloor map[int]floorList
	origin   coord
	minVals  coord
	maxVals  coord
	numSand  int
}

func (w *world) print() {
	for r := w.minVals.row; r <= w.maxVals.row; r++ {
		for c := w.minVals.col; c <= w.maxVals.col; c++ {
			if w.origin.col == c && w.origin.row == r {
				fmt.Print("+")
				continue
			}
			content := w.at(coord{row: r, col: c})
			if content == sand {
				fmt.Print("o")
			} else if content == wall {
				fmt.Print("#")
			} else if content == route {
				fmt.Print("~")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}
func newWorld() world {
	return world{
		blocked:  make(map[coord]stuff),
		colFloor: make(map[int]floorList),
		origin:   coord{0, 0},
		minVals:  coord{0, 0},
		maxVals:  coord{0, 0},
		numSand:  0,
	}
}
func (w *world) addLine(line string) {

	coords := strings.Split(line, " -> ")
	last := newCoord(coords[0])
	w.minVals.setMin(last)
	w.maxVals.setMax(last)
	for i := range coords[1:] {
		curr := newCoord(coords[i+1])
		w.minVals.setMin(curr)
		w.maxVals.setMax(curr)

		for row := minVal(last.row, curr.row); row <= maxVal(last.row, curr.row); row++ {
			c := coord{row: row, col: curr.col}
			w.blocked[c] = wall
		}
		for col := minVal(last.col, curr.col); col <= maxVal(last.col, curr.col); col++ {
			c := coord{row: curr.row, col: col}
			w.blocked[c] = wall
			w.colFloor[c.col] = append(w.colFloor[c.col], c.row)
		}
		last = curr
	}
}
func (w *world) at(coord coord) stuff {
	v, ok := w.blocked[coord]
	if ok {
		return v
	}
	return space
}
func (w *world) arrange(k int) {

	sort.Slice(w.colFloor[k],
		func(i, j int) bool {
			return w.colFloor[k][i] < w.colFloor[k][j]
		})
}
func (w *world) prepare() {
	for k := range w.colFloor {
		w.arrange(k)
	}
}

func (w *world) freeAt(coord coord) bool {
	stuff := w.at(coord)
	return stuff != wall && stuff != sand
}

func (w *world) fallSand(pos coord) bool {
	w.blocked[pos] = route
	//binary search?
	row := pos.row

	for _, r := range w.colFloor[pos.col] {
		if r > pos.row {
			row = r
			break
		}
	}

	if row == pos.row {
		return false
	}

	//pos.row = row - 1

	for _, d := range dirs {
		d.goIn(pos)
		if !w.freeAt(d) {
			continue
		}
		return w.fallSand(d)
	}
	w.numSand += 1
	w.blocked[pos] = sand
	return true
}

func (w *world) addSand() bool {
	return w.fallSand(w.origin)
}

func main() {

	world := newWorld()
	file, _ := os.Open("input")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		world.addLine(lineStr)
	}
	world.prepare()
	world.print()
	fmt.Println()
	for world.addSand() {
		//input()
	}
	world.print()
	fmt.Println("star1", world.numSand)

}
