package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

var itemReg = regexp.MustCompile(`Starting items:(.*)`)
var opReg = regexp.MustCompile(`Operation: new = old (.) ([\dold]*)`)
var testReg = regexp.MustCompile(`Test: divisible by ([\d]*)`)
var nextReg = regexp.MustCompile(`If true: throw to monkey ([\d]*)`)
var prevReg = regexp.MustCompile(`If false: throw to monkey ([\d]*)`)

const (
	//star1 config
	//maxRound = 20
	//divBy = 3
	maxRound = 1000
	divBy    = 1
)

type Operation int

const (
	Mul Operation = iota
	Add
)

func assert(cond bool, msg string) {
	if !cond {
		log.Fatal("assertion failed:", msg)
	}
}
func toInt(s string) int64 {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int64(i)
}

type operation struct {
	op       Operation
	i        int64
	withSelf bool
}

func (o *operation) doOp(val int64) int64 {
	i := o.i
	if o.withSelf {
		i = val
	}

	if o.op == Add {
		return i + val
	}
	return i * val

}
func getOp(line string) operation {
	match := opReg.FindStringSubmatch(line)
	assert(len(match) == 3,
		fmt.Sprintf("wrong length for operation, -want %d +got %d", 3, len(match)))

	op := Mul
	i := int64(0)

	if match[1][0] == '+' {
		op = Add
	}
	if match[2] != "old" {
		i = toInt(match[2])
	}
	return operation{
		op:       op,
		i:        i,
		withSelf: match[2] == "old",
	}
}

type monkey struct {
	items     []int64
	next      int64
	prev      int64
	op        operation
	divBy     int64
	inspected int64
}

func (m *monkey) inspectItems(monkeys []monkey, magic int64) {
	items := m.items
	m.inspected += int64(len(items))
	for _, item := range items {
		item = m.op.doOp(item) / divBy
		item %= magic
		if (item % m.divBy) == 0 {
			monkeys[m.next].stash(item)
		} else {
			monkeys[m.prev].stash(item)
		}
	}
}

func (m *monkey) clearStash() {
	m.items = make([]int64, 0)
}
func (m *monkey) stash(item int64) {
	m.items = append(m.items, item)
}
func (m *monkey) parseString(lines []string) {
	m.items = make([]int64, 0)

	match := itemReg.FindStringSubmatch(lines[0])
	assert(len(match) > 1, "no items found")
	items := strings.Split(match[1], ",")
	for i := range items {
		m.stash(toInt(strings.TrimSpace(items[i])))
	}
	m.op = getOp(lines[1])

	match = testReg.FindStringSubmatch(lines[2])
	assert(len(match) == 2, "no test immediate found")
	m.divBy = toInt(match[1])

	match = nextReg.FindStringSubmatch(lines[3])
	assert(len(match) == 2, fmt.Sprintf("invalid next; -want %d +got %d", 1, len(match)))
	m.next = toInt(match[1])

	match = prevReg.FindStringSubmatch(lines[4])
	assert(len(match) == 2, fmt.Sprintf("invalid prev; -want %d +got %d", 1, len(match)))
	m.prev = toInt(match[1])
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

func main() {

	lines, err := readLines("test")
	if err != nil {
		log.Fatal(err)
	}
	var monkeys []monkey = make([]monkey, 0)
	var magic int64 = 1
	for i := 1; i < len(lines); i += 7 {
		m := monkey{}
		m.parseString(lines[i:])
		monkeys = append(monkeys, m)
		magic *= m.divBy
	}

	for round := 0; round < maxRound; round++ {
		for i := range monkeys {
			monkeys[i].inspectItems(monkeys, magic)
			monkeys[i].clearStash()
		}
	}
	inspected := make([]int64, len(monkeys))

	for i, m := range monkeys {
		fmt.Println(fmt.Sprintf("\nMonkey %d:", i))
		fmt.Println(len(m.items), ":")
		for _, item := range m.items {
			fmt.Print(item, " ")
		}
		fmt.Println("   [", m.inspected, "]")
		inspected[i] = m.inspected
	}
	sort.Slice(inspected, func(i, j int) bool { return inspected[i] < inspected[j] })

	fmt.Println("star1 : ", inspected, inspected[len(inspected)-1]*inspected[len(inspected)-2])
}
