package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

const EMPTY rune = ' '

func toint(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
}

// readLines reads a whole file into memory
// and returns a slice of its lines.
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

type stack struct {
	chars      []rune
	currHeight int
}

func (s *stack) init(height int, numStacks int) {
	s.currHeight = 0
	s.chars = make([]rune, height*numStacks)
	for i := range s.chars {
		s.chars[i] = EMPTY
	}
}

func (s *stack) addCrate(pos int, crate rune) {
	if crate == EMPTY {
		return
	}
	s.chars[pos] = crate
	if s.currHeight <= pos {
		s.currHeight = pos + 1
	}
}

func (s *stack) moveCrates(num int, o *stack, keepOrder bool) {
	//TODO error check
	for i := 1; i <= num; i++ {
		dst_idx := o.currHeight - 1 + i
		if keepOrder {
			dst_idx = o.currHeight + num - i

		}
		o.chars[dst_idx] = s.chars[s.currHeight-i]
		s.chars[s.currHeight-i] = EMPTY
	}
	s.currHeight -= num
	o.currHeight += num
}
func (s stack) top() rune {
	if s.currHeight == 0 {
		return EMPTY
	}
	return s.chars[s.currHeight-1]
}
func (s stack) print() {
	for i, c := range s.chars {
		if i < s.currHeight {
			fmt.Print("[", string(c), "]")
		} else if c != EMPTY {
			fmt.Print(" ", string(c), " ")
		} else {
			//fmt.Print(" . ")

		}
	}
	fmt.Println()
}

func getStacks(file []string) ([]stack, int) {
	var numStacks int
	var stackHeight int
	for i, line := range file {
		// init stacks
		if i == 0 {
			numStacks = int((len(line) + 1) / 4)
		}
		if len(line) == 0 {
			stackHeight = int(i - 1)
		}
	}

	stacks := make([]stack, numStacks)
	for i := range stacks {
		stacks[i].init(stackHeight, numStacks)
	}
	return stacks, stackHeight
}

func fillStacks(file []string, stacks []stack, startHeight int) {

	for i := 0; i < startHeight; i++ {
		line := file[i]

		for s := 0; s < len(stacks); s += 1 {
			stacks[s].addCrate(int(startHeight-i-1), rune(line[s*4+1]))
		}
	}
}
func printStacks(stacks []stack) {
	fmt.Println("------------------------------------")
	for i, s := range stacks {
		fmt.Print(i, "[", s.currHeight, "] : ")
		s.print()
	}
	fmt.Println("------------------------------------")
}
func moveCrates(file []string, stacks []stack, startIdx int, keepOrder bool) {
	r := regexp.MustCompile(`move (?P<num>[\d]+) from (?P<src>[\d]+) to (?P<dest>[\d]+)`)
	for i := startIdx; i < len(file); i++ {
		matches := r.FindStringSubmatch(file[i])
		num := toint(matches[1])
		src := toint(matches[2]) - 1
		dst := toint(matches[3]) - 1
		fmt.Printf("[%d]  %d: %d -> %d\n", i-startIdx, num, src, dst)
		if src == dst || num == 0 {
			continue
		}
		stacks[src].moveCrates(num, &stacks[dst], keepOrder)

		printStacks(stacks)
		//var b []byte = make([]byte, 1)
		//os.Stdin.Read(b)
	}
}

func main() {
	isStar1 := false
	file, err := readLines("test")
	if err != nil {
		log.Fatal(err)
	}

	stacks, startHeight := getStacks(file)
	fillStacks(file, stacks, startHeight)
	printStacks(stacks)
	moveCrates(file, stacks, startHeight+2, !isStar1)
	star := ""
	for _, s := range stacks {
		star += string(s.top())
	}
	if isStar1 {
		fmt.Printf("star1: >%s<\n", star)
	} else {
		fmt.Printf("star2: >%s<\n", star)

	}
}
