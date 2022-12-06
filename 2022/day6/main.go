package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func getIdx(b byte) int {
	return int(b) - int('a')
}
func firstMarker(line string, msg_num int) int {
	var array = new([27]int)
	for i := 0; i < len(line); i++ {
		array[getIdx(line[i])] += 1
		if i < msg_num-1 {
			continue
		}
		for j := i - msg_num + 1; j <= i; j++ {

			if array[getIdx(line[j])] > 1 {
				break
			}
			if j == i {
				return i + 1
			}
		}
		array[getIdx(line[i-msg_num+1])] -= 1
	}
	return -1
}

func main() {
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		fmt.Println(lineStr)
		fmt.Println("star1: ", firstMarker(lineStr, 4))
		fmt.Println("star2: ", firstMarker(lineStr, 14))
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
