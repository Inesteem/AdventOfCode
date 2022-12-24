package main

import (
	"fmt"
)

func main() {
	lines, _ := readLines("input")
	list := initDoublyList()
	nodes := make([]*node, 0)
	for i := range lines {
		num := toInt(lines[i])
		nodes = append(nodes, list.Append(num)*811589153)
	}
	list.toRingBuf()
	posZero := -1
	for i := range nodes {
		data := nodes[i].data
		if data == 0 {
			posZero = i
		} else if data < 0 {
			nodes[i].moveLeft((-data) % list.Size())
		} else {
			nodes[i].moveRight(data % list.Size())
		}

	}

	a := nodes[posZero].getNodeAfter(1000 % list.Size())
	b := nodes[posZero].getNodeAfter(2000 % list.Size())
	c := nodes[posZero].getNodeAfter(3000 % list.Size())
	fmt.Println("star1:", a.data+b.data+c.data)
}
