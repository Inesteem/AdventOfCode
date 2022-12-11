package main

import (
	"bufio"
	"fmt"
	"log"
	"math/big"
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
	maxRound = 1000
)

//star1 config
//var divBy = *big.NewInt(int64(3))
var divBy = big.NewInt(int64(1))
var zero = big.NewInt(int64(0))

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
func toInt(s string) int {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int(i)
}

type operation struct {
	op       Operation
	i        *big.Int
	withSelf bool
}

func (o *operation) doOp(val *big.Int) *big.Int {
	i := o.i
	if o.withSelf {
		i = val
	}

	if o.op == Add {
		return val.Add(val, i)
	}
	return val.Mul(val, i)

}
func getOp(line string) operation {
	match := opReg.FindStringSubmatch(line)
	assert(len(match) == 3,
		fmt.Sprintf("wrong length for operation, -want %d +got %d", 3, len(match)))

	op := Mul
	i := 0

	if match[1][0] == '+' {
		op = Add
	}
	if match[2] != "old" {
		i = toInt(match[2])
	}
	return operation{
		op:       op,
		i:        big.NewInt(int64(i)),
		withSelf: match[2] == "old",
	}
}

type monkey struct {
	items     []*big.Int
	next      int
	prev      int
	op        operation
	divBy     *big.Int
	inspected int64
}

func (m *monkey) inspectItems(monkeys []monkey) {
	items := m.items
	m.inspected += int64(len(items))
	//fmt.Println(m.inspected, len(items))
	for _, item := range items {
		//fmt.Print(item, " -> ")
		item = m.op.doOp(item)
		item = item.Div(item, divBy)
		item_tmp := new(big.Int)
		item_tmp.Set(item)
		//fmt.Println(item)
		if (item_tmp.Mod(item_tmp, m.divBy)).Cmp(zero) == 0 {
			//fmt.Println(" t ->", m.next)
			monkeys[m.next].stash(item)
		} else {
			//fmt.Println(" f ->", m.prev)
			monkeys[m.prev].stash(item)
		}
	}
}
func (m *monkey) clearStash() {
	m.items = make([]*big.Int, 0)
}

func (m *monkey) stash(item *big.Int) {
	//fmt.Println("stash : ", item)
	m.items = append(m.items, item)
}

func (m *monkey) parseString(lines []string) {
	m.items = make([]*big.Int, 0)

	match := itemReg.FindStringSubmatch(lines[0])
	assert(len(match) > 1, "no items found")
	items := strings.Split(match[1], ",")
	for i := range items {
		m.stash(big.NewInt(int64(toInt(strings.TrimSpace(items[i])))))
	}
	m.op = getOp(lines[1])

	match = testReg.FindStringSubmatch(lines[2])
	assert(len(match) == 2, "no test immediate found")
	m.divBy = big.NewInt(int64(toInt(match[1])))

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
	for i := 1; i < len(lines); i += 7 {
		m := monkey{}
		fmt.Println(lines[i-1])
		m.parseString(lines[i:])
		monkeys = append(monkeys, m)
	}

	for round := 0; round < maxRound; round++ {
		for i := range monkeys {
			fmt.Println(fmt.Sprintf("\n%d: Monkey %d:", round, i))
			monkeys[i].inspectItems(monkeys)
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

	fmt.Println("star : ", inspected, inspected[len(inspected)-1]*inspected[len(inspected)-2])
}
