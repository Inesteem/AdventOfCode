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
	EQ
)

func toStr(op operation) string {
	if op == ADD {
		return "+"
	}
	if op == SUB {
		return "-"
	}
	if op == MUL {
		return "*"
	}
	if op == DIV {
		return "/"
	}
	return "?"
}

type monkey struct {
	left    int
	right   int
	val     int64
	op      operation
	isRoot  bool
	isHuman bool
	cached  int64
	valid   bool
}

func doInverseOp(result, l, r int64, lValid bool, op operation) int64 {
	part := l
	if !lValid {
		part = r
	}

	if lValid {
		if op == DIV {
			return part / result
		}
		if op == SUB {
			return part - result
		}
	} else {
		if op == DIV {
			return result * part
		}
		if op == SUB {
			return result + part
		}
	}

	if op == MUL {
		return result / part
	}
	if op == ADD {
		return result - part
	}
	//EQ
	return result
}
func doOp(l, r int64, op operation) int64 {
	if op == MUL {
		return l * r
	}
	if op == DIV {
		return l / r
	}
	if op == ADD {
		return l + r
	}
	//will work for eq and sub
	return l - r
}

func (m *monkey) getValEq(monkeys []monkey) (error, int64) {
	if m.op == NOP {
		if m.isHuman {
			return fmt.Errorf("human trash found"), 0
		}
		m.cached = m.val
		m.valid = true
		return nil, m.val
	}
	errL, l := monkeys[m.left].getValEq(monkeys)
	errR, r := monkeys[m.right].getValEq(monkeys)

	if errL != nil {
		return errL, r
	}
	if errR != nil {
		return errR, l

	}
	m.cached = doOp(l, r, m.op)
	m.valid = true
	return nil, m.cached
}

func (m *monkey) followTheTrash(monkeys []monkey, val int64) int64 {

	if m.op == NOP {
		assert(m.isHuman, "no trash found")
		return val
	}
	l := &monkeys[m.left]
	r := &monkeys[m.right]

	assert(l.valid != r.valid, "assumption: exactly one should be invalid")

	res := doInverseOp(val, l.cached, r.cached, l.valid, m.op)
	if l.valid {
		fmt.Println(val, "=", l.cached, toStr(m.op), res)
		return r.followTheTrash(monkeys, res)
	}
	fmt.Println(val, "=", res, toStr(m.op), r.cached)
	return l.followTheTrash(monkeys, res)

}

func (m *monkey) getVal(monkeys []monkey) int64 {
	if m.op == NOP {
		return m.val
	}
	l := monkeys[m.left].getVal(monkeys)
	r := monkeys[m.right].getVal(monkeys)
	return doOp(l, r, m.op)
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
	root := &monkeys[sMap.getIdx("root")]
	humn := &monkeys[sMap.getIdx("humn")]
	root.isRoot = true
	humn.isHuman = true

	fmt.Println("star1:", root.getVal(monkeys))

	root.op = EQ
	err, val := root.getValEq(monkeys)
	assert(err != nil, "should be error")
	fmt.Println("star2:", root.followTheTrash(monkeys, val))
}
