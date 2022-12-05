package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func Split(r rune) bool {
	return r == ',' || r == '-'
}

type Range struct {
	start int64
	end   int64
}

func (r Range) overlaps(o Range) bool {
	if r.start >= o.start && r.start <= o.end {
		return true
	}
	if r.end <= o.end && r.end >= o.start {
		return true
	}
	return false
}
func (r Range) contains(o Range) bool {
	if r.start > o.start || r.end < o.end {
		return false
	}
	return true
}

func toInt64(s string) int64 {
	i, _ := strconv.ParseInt(s, 10, 64)
	return i
}

func main() {
	file, err := os.Open("test")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var star1 int64
	var star2 int64
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		input := strings.FieldsFunc(lineStr, Split)
		r1 := Range{start: toInt64(input[0]), end: toInt64(input[1])}
		r2 := Range{start: toInt64(input[2]), end: toInt64(input[3])}
		if r1.contains(r2) || r2.contains(r1) {
			star1 += 1
			star2 += 1
		} else if r1.overlaps(r2) {
			star2 += 1
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println("star1: ", star1)
	fmt.Println("star2: ", star2)
}
