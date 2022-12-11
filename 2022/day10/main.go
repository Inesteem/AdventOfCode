package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func toInt(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
}

func isAdd(op []string) bool {
	return op[0] == "addx"
}

func doOp(op []string, reg *int) int {

	if op[0] == "noop" {
		return 1
	}

	i := toInt(op[1])
	*reg += i
	return 2
}

func inSpriteRange(reg, cycle int) bool {
	nx := cycle % 40
	return reg >= nx-1 && reg <= nx+1
}

func main() {

	crt := make([]bool, 6*40)

	cycles := [6]int{20, 60, 100, 140, 180, 220}
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	reg := 1
	scanner := bufio.NewScanner(file)
	i := 1
	j := 0

	s := 0
	for scanner.Scan() {
		lineStr := scanner.Text()
		op := strings.Split(lineStr, " ")
		if isAdd(op) {
			crt[i-1] = inSpriteRange(reg, i)
			if j < len(cycles) && i+1 == cycles[j] {
				s += (1 + i) * reg
				fmt.Println(i, reg, (1+i)*reg)
				j += 1
			}
		}
		i += doOp(op, &reg)
		crt[i-2] = inSpriteRange(reg, i-1)
		if j < len(cycles) && i == cycles[j] {
			s += i * reg
			fmt.Println(i, reg, i*reg)
			j += 1
		}
	}
	fmt.Println("star1", s)

	fmt.Print("?") //TODO(fix) whats going wrong here?
	for i := 0; i < 6*40-1; i++ {
		if (i+1)%40 == 0 {
			fmt.Println("")
		}
		if crt[i] {
			fmt.Print("#")
		} else {
			fmt.Print(".")
		}
	}
	fmt.Println("")
}
