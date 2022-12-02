package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Figure int8

const (
	Error    Figure = 0
	Rock            = 1
	Paper           = 2
	Scissors        = 3
	Win             = 4
	Lose            = 5
	Draw            = 6
)

// Rock      Paper Scissors
var winning = [...]Figure{Error, Scissors, Rock, Paper}
var loosing = [...]Figure{Error, Paper, Scissors, Rock}

func getFigure(r byte, star1 bool) Figure {
	switch r {
	case 'A':
		return Rock
	case 'B':
		return Paper
	case 'C':
		return Scissors
	case 'X':
		if star1 {
			return Rock
		}
		return Lose
	case 'Y':
		if star1 {
			return Paper
		}
		return Draw
	case 'Z':
		if star1 {
			return Scissors
		}
		return Win
	}
	return Error
}

func get_score(mine Figure, theirs Figure, star1 bool) int64 {
	if !star1 {
		if mine == Draw {
			mine = theirs
		} else if mine == Lose {
			mine = winning[theirs]
		} else {

			mine = loosing[theirs]
		}
	}

	if mine == theirs {
		return int64(mine) + 3
	}
	if winning[mine] == theirs {
		return int64(mine) + 6

	}
	return int64(mine)
}

func main() {
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	//rounds := []round{}
	var star1 int64
	var star2 int64
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		//rounds = append(rounds, round{theirs: getFigure(lineStr[0]), mine: getFigure(lineStr[2])})
		star1 += get_score(getFigure(lineStr[2], true), getFigure(lineStr[0], true), true)
		star2 += get_score(getFigure(lineStr[2], false), getFigure(lineStr[0], false), false)

	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println("star1: ", star1)
	fmt.Println("star2: ", star2)
}
