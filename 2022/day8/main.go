package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

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

func maxTree(a, b int) int {
	if a >= b {
		return a
	}
	return b
}

func getVisibility(trees [][]int) [][]bool {
	maxR := len(trees)
	maxC := len(trees[0])
	visible := make([][]bool, len(trees))
	//left -> right
	for r := 1; r < maxR-1; r++ {
		visible[r] = make([]bool, len(trees[r]))
		heighest := trees[r][0]
		for c := 1; c < maxC-1; c++ {
			visible[r][c] = trees[r][c] > heighest
			heighest = maxTree(heighest, trees[r][c])
		}
	}

	//right -> left
	for r := 1; r < maxR-1; r++ {
		heighest := trees[r][maxC-1]
		for c := maxC - 2; c > 0; c-- {
			visible[r][c] = visible[r][c] || (trees[r][c] > heighest)
			heighest = maxTree(heighest, trees[r][c])
		}
	}

	//up -> down
	for c := 1; c < maxC-1; c++ {
		heighest := trees[0][c]
		for r := 1; r < maxR-1; r++ {
			visible[r][c] = visible[r][c] || (trees[r][c] > heighest)
			heighest = maxTree(heighest, trees[r][c])
		}
	}

	// down -> up
	for c := 1; c < maxC-1; c++ {
		heighest := trees[maxR-1][c]
		for r := maxR - 2; r > 0; r-- {
			visible[r][c] = visible[r][c] || (trees[r][c] > heighest)
			heighest = maxTree(heighest, trees[r][c])
		}
	}

	return visible
}

func getNumVisible(trees [][]int) int {
	visible := getVisibility(trees)

	sum := 2*len(trees) + 2*(len(trees[0])) - 4
	for r := 1; r < len(visible)-1; r++ {
		for c := 1; c < len(visible[r])-1; c++ {
			if visible[r][c] {
				sum += 1
			}
		}
	}
	return sum
}

func main() {

	lines, err := readLines("input")
	if err != nil {
		log.Fatal(err)
	}

	trees := getTrees(lines)
	visible := getNumVisible(trees)
	fmt.Println("star1:", visible)

}
