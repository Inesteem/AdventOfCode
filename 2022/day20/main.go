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
		//star 1
		//nodes = append(nodes, list.Append(int64(num)))
		nodes = append(nodes, list.Append(int64(num)*811589153))
	}
	list.toRingBuf()
	posZero := -1
	//star 1
	//for j := 0; j < 1; j++ {
	for j := 0; j < 10; j++ {
		for i := range nodes {
			data := nodes[i].data
			if data == 0 {
				posZero = i
			} else if data < 0 {
				nodes[i].moveLeft((-data) % (list.Size() - 1))
			} else {
				nodes[i].moveRight(data % (list.Size() - 1))
			}

		}

	}

	a := nodes[posZero].getNodeAfter(1000 % list.Size())
	b := a.getNodeAfter(1000 % list.Size())
	c := b.getNodeAfter(1000 % list.Size())
	fmt.Println("star:", a.data+b.data+c.data)
}
