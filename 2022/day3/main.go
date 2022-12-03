package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func getPriority(r byte) int8 {
	if r >= 'a' && r <= 'z' {
		return int8(r-'a') + 1
	}
	return int8(r-'A') + 27
}

func getItemMap(s string) map[byte]int32 {
	m := make(map[byte]int32)

	for _, c := range s {
		m[byte(c)] += 1
	}
	return m
}

func getSharedCharacter(s string) byte {
	m := getItemMap(s[0 : len(s)/2])
	for i := len(s) / 2; i < len(s); i++ {
		if m[s[i]] != 0 {
			//fmt.Println(string(s[i:i+1]), ":", getPriority(s[i]))
			return s[i]
		}
	}
	return 0
}
func getBadge(group [3]string) byte {
	m0 := getItemMap(group[0])
	m1 := getItemMap(group[1])
	m2 := getItemMap(group[2])

	for key, _ := range m1 {
		m1[key] = m0[key]
	}
	for key, _ := range m2 {
		if m1[key] > 0 {
			return byte(key)
		}
	}
	return 0
}

func main() {
	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var star1 int64
	var star2 int64
	scanner := bufio.NewScanner(file)
	var arr [3]string
	i := 0
	for scanner.Scan() {
		lineStr := scanner.Text()
		arr[i] = lineStr
		star1 += int64(getPriority(getSharedCharacter(lineStr)))
		i = (i + 1) % 3
		if i == 0 {
			star2 += int64(getPriority(getBadge(arr)))
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println("star1: ", star1)
	fmt.Println("star2: ", star2)
}
