package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

const spaceNeeded = 30000000
const spaceTotal = 70000000

var cd = regexp.MustCompile(`. cd (.*)`)

var ls = regexp.MustCompile(`. ls`)
var file = regexp.MustCompile(`(?P<size>[\d]+) (?P<name>.*)`)
var dir = regexp.MustCompile(`dir (?P<name>.*)`)

type Node struct {
	parent   *Node
	children map[string]*Node
	name     string
	size     int
}

func toInt(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
}

func (n *Node) getSize() int {
	if n.size > 0 {
		return n.size
	}
	for _, c := range n.children {
		n.size += c.getSize()
	}
	return n.size
}

func (n Node) star1() int {
	if len(n.children) == 0 {
		return 0
	}
	sum := 0
	for _, c := range n.children {
		sum += c.star1()
	}
	if n.size <= 100000 {
		sum += n.size
	}
	return sum
}

func (n Node) star2(spaceFree int) int {
	if len(n.children) == 0 || n.size+spaceFree < spaceNeeded {
		return spaceNeeded
	}
	minSpaceDel := n.size
	for _, c := range n.children {
		msd := c.star2(spaceFree)
		if msd < minSpaceDel {
			minSpaceDel = msd
		}
	}
	return minSpaceDel
}
func (n *Node) hasChild(name string) bool {
	_, found := n.children[name]
	return found
}

func (n *Node) getChild(name string) *Node {
	c, found := n.children[name]
	if !found {
		log.Fatalf("file %s not found in dir %s", name, n.name)
	}
	return c
}
func (n *Node) addChild(name string, size int) {
	if n.hasChild(name) {
		log.Fatalf("file %s already in dir %s", name, n.name)
	}
	n.children[name] = &Node{name: name, size: size, parent: n, children: make(map[string]*Node)}
}

func doCmd(line string, currDir *Node, root *Node) *Node {
	m := cd.FindStringSubmatch(line)
	if len(m) > 0 {
		name := m[1]
		if name == "/" {
			return root
		}
		if name == ".." {
			return currDir.parent
		}
		return currDir.getChild(name)
	}

	m = file.FindStringSubmatch(line)
	if len(m) > 0 {
		size := toInt(m[1])
		name := m[2]
		if !currDir.hasChild(name) {
			currDir.addChild(name, size)
		}
	}

	m = dir.FindStringSubmatch(line)
	if len(m) > 0 {
		name := m[1]
		currDir.addChild(name, 0)
	}

	return currDir
}

func main() {
	root := Node{name: "/", children: make(map[string]*Node)}
	root.parent = &root
	currDir := &root

	file, err := os.Open("input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		currDir = doCmd(lineStr, currDir, &root)
	}
	root.getSize()
	fmt.Println("star1:", root.star1())
	fmt.Println("star2:", root.star2(spaceTotal-root.size))

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
