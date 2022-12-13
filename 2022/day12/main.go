package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
)

//TODO: simplify, implemented bastard dikstra priority queue because I was not
// sure what second assignment would bring - but with current solution I just need
// a FIFO
const star int = 1

type coord struct {
	row, col int
}

var GO_RIGHT coord = coord{row: 0, col: 1}
var GO_LEFT coord = coord{row: 0, col: -1}
var GO_DOWN coord = coord{row: 1, col: 0}
var GO_UP coord = coord{row: -1, col: 0}

var dirs [4]coord = [4]coord{GO_RIGHT, GO_LEFT, GO_DOWN, GO_UP}

func (p *coord) goIn(dir coord) {
	p.row += dir.row
	p.col += dir.col
}

func (p *coord) equals(c coord) bool {
	return p.row == c.row && p.col == c.col
}

// copied from here: https://pkg.go.dev/container/heap
// An Item is something we manage in a priority queue.
type Item struct {
	position coord // The value of the item; arbitrary.
	priority int   // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and position of an Item in the queue.
func (pq *PriorityQueue) update(item *Item, position coord, priority int) {
	item.position = position
	item.priority = priority
	heap.Fix(pq, item.index)
}

// end copy

func getHeight(r rune) int {
	if r == 'S' {
		return 0
	}
	if r == 'E' {
		return 25
	}

	return int(r) - int('a')
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

type heightMap struct {
	field      [][]int
	start, end coord
}

func (h *heightMap) at(c coord) int {
	return h.field[c.row][c.col]
}
func (h *heightMap) diff(c1, c2 coord) int {
	return h.at(c2) - h.at(c1)
}
func (h *heightMap) valid(coord coord) bool {
	if coord.row < 0 || coord.col < 0 {
		return false
	}
	return coord.row < len(h.field) && coord.col < len(h.field[coord.row])
}

func visit(visited [][]bool, coord coord) {
	visited[coord.row][coord.col] = true
}

func alreadyVisited(visited [][]bool, coord coord) bool {
	if visited[coord.row][coord.col] {
		return true
	}
	return false
}

func searchField(heightMap heightMap) int {
	todo := make(PriorityQueue, 0)
	visited := make([][]bool, len(heightMap.field))
	for row := range heightMap.field {
		visited[row] = make([]bool, len(heightMap.field[row]))
		for col, c := range heightMap.field[row] {

			if c != 0 {
				continue
			}

			item := &Item{
				position: coord{row: row, col: col},
				priority: 0,
			}
			heap.Push(&todo, item)
			visit(visited, coord{row: row, col: col})
		}

	}
	heap.Init(&todo)

	// assert goal != start
	goal := heightMap.end

	for todo.Len() > 0 {
		item := heap.Pop(&todo).(*Item)

		curr := item.position

		for i := range dirs {
			//next position
			next := coord{row: curr.row, col: curr.col}
			next.goIn(dirs[i])

			//in range
			if !heightMap.valid(next) {
				continue
			}

			//height constraint
			diff := heightMap.diff(curr, next)
			if diff > 1 {
				continue
			}

			//visited constraint
			if alreadyVisited(visited, next) {
				continue
			}
			visit(visited, next)

			//check goal
			if next.equals(goal) {
				return (-item.priority) + 1
			}

			item := &Item{
				position: next,
				priority: item.priority - 1,
			}
			heap.Push(&todo, item)
		}
	}
	for i := range visited {
		for j := range visited[i] {
			if i == goal.row && j == goal.col {
				fmt.Print("O")
			} else if visited[i][j] {
				//fmt.Print("#")
				fmt.Print(".")
			} else {
				fmt.Print(string(rune(heightMap.at(coord{i, j}) + int('a'))))
				//fmt.Print(".")
			}
		}
		fmt.Println("")
	}
	return -1
}

func getHeightMap(lines []string) heightMap {
	h := heightMap{field: make([][]int, len(lines))}

	for row := range lines {
		h.field[row] = make([]int, len(lines[row]))
		for col, c := range lines[row] {
			h.field[row][col] = getHeight(c)
			if c == 'S' {
				h.start = coord{row: row, col: col}
			} else if c == 'E' {
				h.end = coord{row: row, col: col}
			}
		}
	}
	return h
}

func main() {
	lines, _ := readLines("input")

	h := getHeightMap(lines)
	fmt.Println("star1: ", searchField(h))
}
