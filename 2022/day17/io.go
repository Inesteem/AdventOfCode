package main

import (
	"bufio"
	"log"
	"os"
)

func input() {

	var b []byte = make([]byte, 1)
	os.Stdin.Read(b)
}

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
