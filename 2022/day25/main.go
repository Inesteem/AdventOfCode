package main

import (
	"fmt"
)

func intToSNAFU(i int64) string {
	arr := make([]int, 0)
	for ; i > 0; i /= 5 {
		arr = append(arr, int(i%5))
	}
	if len(arr) == 0 {
		return "0"
	}
	res := ""
	offset := 0
	for i := 0; i < len(arr); i++ {
		n := arr[i]
		if n <= 2 {
			res = string(int('0')+arr[i]) + res
		} else if n == 3 {
			offset += 1
			res = "=" + res
		} else if n == 4 {
			offset += 1
			res = "-" + res
		} else {
			res = "0" + res
			offset += (n + 1) % 5
		}
		if offset > 0 {
			if i >= len(arr)-1 {
				arr = append(arr, offset)
			} else {
				arr[i+1] += offset
			}
		}
		offset = 0

	}
	return res
}

func SNAFUToInt(s string) int {

	var val, offset int
	for i, m := len(s)-1, 1; i >= 0; i, m = i-1, m*5 {
		add, nextOffset := 0, 0
		switch s[i] {
		case '-':
			add = 4
			nextOffset = -1
		case '=':
			add = 3
			nextOffset = -1
		default:
			add = int(s[i]) - int('0')
		}
		add += offset
		val += add * m
		offset = nextOffset
	}
	return val

}

func main() {

	lines, _ := readLines("input")

	var sum int64
	for i := range lines {
		sum += int64(SNAFUToInt(lines[i]))
	}
	fmt.Println("star1", intToSNAFU(sum))
}
