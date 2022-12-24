package main

import (
	"fmt"
	"regexp"
)

var opR = regexp.MustCompile("([a-z]+): ([a-z]+) (.) ([a-z]+)")
var basicR = regexp.MustCompile("([a-z]+): ([0-9]+)")

type operation int

const (
	NOP = iota
	ADD
	SUB
	MUL
	DIV
)

type monkey struct {
	left  int
	right int
	val   int64
	op    operation
}

func (m *monkey) getVal(monkeys []monkey) int64 {
	if m.op == NOP {
		return m.val
	}
	l := monkeys[m.left].getVal(monkeys)
	r := monkeys[m.right].getVal(monkeys)
	if m.op == MUL {
		return l * r
	}
	if m.op == DIV {
		return l / r
	}
	if m.op == ADD {
		return l + r
	}
	return l - r
}

func toOp(s string) operation {
	if s == "*" {
		return MUL
	}
	if s == "-" {
		return SUB
	}
	if s == "/" {
		return DIV
	}
	return ADD
}
func parseMonkeyOp(line string, m *stringToIntMap, monkeys []monkey) {
	match := opR.FindStringSubmatch(line)
	if len(match) == 5 {
		idx := m.getIdx(match[1])
		monkeys[idx].left = m.getIdx(match[2])
		monkeys[idx].op = toOp(match[3])
		monkeys[idx].right = m.getIdx(match[4])
	} else {
		match := basicR.FindStringSubmatch(line)
		assert(len(match) == 3, "wrong match")
		idx := m.getIdx(match[1])
		monkeys[idx].val = int64(toInt(match[2]))
	}
}

func main() {
	lines, _ := readLines("input")
	monkeys := make([]monkey, len(lines))

	sMap := newStringToIntMap()
	for i := range lines {
		parseMonkeyOp(lines[i], &sMap, monkeys)

	}

	fmt.Println("star1:", monkeys[sMap.getIdx("root")].getVal(monkeys))

}
