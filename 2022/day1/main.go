package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	calories := []int{0}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		if len(lineStr) > 0 {
			num, _ := strconv.Atoi(lineStr)
			calories[len(calories)-1] += num
		} else {
			calories = append(calories, 0)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	sort.Slice(calories, func(i, j int) bool {
		return calories[i] > calories[j]
	})
	fmt.Println("star1: ", calories[0])
	fmt.Println("star2: ", calories[0]+calories[1]+calories[2])
}
