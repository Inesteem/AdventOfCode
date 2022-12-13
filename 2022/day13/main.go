package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
)

const (
	emptyChar int = 0
)

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

//> 0 is right order
func cmpVal(l, r int) int {
	return r - l
}

type value struct {
	number int
	list   []value
}

func (v *value) isEmpty() bool {
	return v.isLeaf() && v.number == emptyChar
}
func (v *value) isLeaf() bool {
	return v.list == nil || len(v.list) == 0
}

func (l *value) eq(r *value) bool {
	if l.number != r.number {
		return false
	}
	if len(l.list) != len(r.list) {
		return false
	}
	for i := range l.list {
		if !l.list[i].eq(&r.list[i]) {
			return false
		}
	}
	return true
}
func (l *value) cmp(r *value, msg string) int {
	fmt.Print(msg, "Compare ")
	l.print()
	fmt.Print(" vs ")
	r.print()
	fmt.Println()
	if l.isLeaf() {
		if r.isLeaf() {
			// will work even for emty nodes because all
			// valid numbers are > 0 (which is emptyChar)
			return cmpVal(l.number, r.number)
		}
		if l.isEmpty() {
			return 1
		}
		v := value{number: emptyChar, list: []value{*l}}
		return v.cmp(r, msg+" ")
	} else if r.isLeaf() {
		if r.isEmpty() {
			return -1
		}
		v := value{number: emptyChar, list: []value{*r}}
		return l.cmp(&v, msg+" ")
	}

	for i := 0; i < len(l.list) && i < len(r.list); i++ {
		res := l.list[i].cmp(&r.list[i], msg+" ")
		if res == 0 {
			continue
		}
		return res
	}
	fmt.Println("checking length")
	return cmpVal(len(l.list), len(r.list))
}
func (v *value) print() {
	if len(v.list) > 0 {
		fmt.Print("[")
		for i := range v.list {
			v.list[i].print()
			if i < len(v.list)-1 {
				fmt.Print(",")
			}
		}
		fmt.Print("]")
	} else if v.number != emptyChar {
		fmt.Print(v.number - 1)
	} else {
		fmt.Print("[]")
	}
}

func toInt(s string) (val, idx int) {
	val = 0
	for i := range s {
		if s[i] == ',' || s[i] == ']' {
			return val, i
		}
		val *= 10
		val += int(s[i]) - int('0')
	}
	log.Fatal("wrong number parsed", s)
	return -1, -1
}

func getValue(line string) (val value, idx int) {
	v := value{number: emptyChar, list: make([]value, 0)}
	i := 0
	for i < len(line) {
		if line[i] == '[' {
			v_tmp, j := getValue(line[i+1:])
			i += j + 1
			v.list = append(v.list, v_tmp)
		} else if line[i] == ']' {
			return v, i + 1
		} else if line[i] == ',' {
			i += 1
		} else {
			val, j := toInt(line[i:])
			i += j
			v.list = append(v.list, value{number: val + 1})
		}
	}
	return v, -1
}

func inOrder(val int) bool {
	return val > 0
}

type t []value

func (values t) Len() int {
	return len(values)
}
func (values t) Swap(i, j int) {
	values[i], values[j] = values[j], values[i]
}

func (values t) Less(i, j int) bool {
	return inOrder(values[i].cmp(&values[j], ""))
}
func main() {
	star1 := 0
	star2 := 1
	v1, _ := getValue("[2]]")
	v2, _ := getValue("[6]]")
	var values []value = []value{v1, v2}

	lines, _ := readLines("input")
	for i := 0; i < len(lines); i += 3 {
		fmt.Println("\n== Pair", (i/3)+1, "==")
		l, _ := getValue(lines[i][1:])
		r, _ := getValue(lines[i+1][1:])
		ordered := inOrder(l.cmp(&r, ""))
		fmt.Println("--->", ordered)
		if ordered {
			star1 += (i / 3) + 1
		}
		values = append(values, l)
		values = append(values, r)
	}
	fmt.Println("star1: ", star1)
	sort.Sort(t(values))
	for i := range values {
		values[i].print()
		fmt.Println()
		if values[i].eq(&v1) || values[i].eq(&v2) {
			star2 *= i + 1
		}
	}
	fmt.Println("star2: ", star2)
}
