package main

import (
	"fmt"
)

type form struct {
	form, check []coord
	height      int
}

//####
var line form = form{
	height: 1,
	form:   []coord{STAY, GO_RIGHT, GO_RIGHT, GO_RIGHT},
}

//.#.
//###
//.#.
var cross form = form{
	height: 3,
	form:   []coord{GO_RIGHT, GO_DOWN_LEFT, GO_RIGHT, GO_RIGHT, GO_DOWN_LEFT},
}

//..#
//..#
//###
var theL form = form{
	height: 3,
	form:   []coord{GO_RIGHT.mulVal(2), GO_DOWN, GO_DOWN, GO_LEFT, GO_LEFT},
}

//#
//#
//#
//#
var theI form = form{
	height: 4,
	form:   []coord{STAY, GO_DOWN, GO_DOWN, GO_DOWN},
}

//##
//##
var block form = form{
	height: 2,
	form:   []coord{STAY, GO_RIGHT, GO_DOWN, GO_LEFT},
}

var forms []form = []form{line, cross, theL, theI, block}

type field struct {
	field         [][]int
	width, height int
	heighest      int
}

func (f *field) isValid(c coord) bool {
	return c.row >= 0 && c.row < len(f.field) &&
		c.col >= 0 && c.col < len(f.field[0])
}

func (f *field) isFormAddable(c coord, form form) bool {

	for i := range form.form {
		c.goIn(form.form[i])
		if !f.isValid(c) || f.field[c.row][c.col] > 0 {
			return false
		}
	}
	return true
}

func (f *field) applyForm(c coord, form form, set int) bool {
	if set > 0 && !f.isFormAddable(c, form) {
		return false
	}
	for i := range form.form {
		c.goIn(form.form[i])
		f.field[c.row][c.col] += set
	}
	return true
}

func (f *field) extendIfNeeded(formHeight int) {
	for len(f.field) < f.heighest+formHeight+3 {
		f.field = append(f.field, make([]int, f.width))
		f.height += 1
	}
}

func createField(height, width, heighest int) field {
	f := make([][]int, height)
	for i := range f {
		f[i] = make([]int, width)
	}
	return field{field: f, height: height, width: width, heighest: heighest}
}

func (f *field) moveForm(curr, dir coord, form form) (next coord, ok bool) {
	next = curr
	next.goIn(dir)

	assert(f.applyForm(curr, form, -1), "should remove")
	if !f.applyForm(next, form, 1) {
		assert(f.applyForm(curr, form, 1), "add again")
		return curr, false
	}

	return next, true
}

func (f *field) print() {
	for r := f.height - 1; r >= 0; r-- {
		for c := 0; c < f.width; c++ {
			if f.field[r][c] == 0 {
				fmt.Print(".")
			} else if f.field[r][c] == 1 {
				fmt.Print("#")
			} else {
				fmt.Print("O")

			}
		}
		fmt.Println()
	}

}

func (f *field) getProfileforRow(row int) int64 {
	key := int64(0)
	for i := 0; i < 64; i++ {
		row -= i / 7
		if row < 0 {
			break
		}
		col := i % 7
		key *= 2
		key += int64(f.field[row][col])
	}
	return key
}

type cacheKey struct {
	jetIdx, formIdx int
	profile         int64
}
type cacheContent struct {
	heightest, numStones int
}

type cache struct {
	cache map[cacheKey]cacheContent
}

func newCache() cache {
	return cache{
		cache: make(map[cacheKey]cacheContent, 0),
	}
}

func (c *cache) foundRepetition(profile int64, jetIdx, formIdx int) (key cacheKey, ok bool) {
	key = cacheKey{
		jetIdx:  jetIdx,
		formIdx: formIdx,
		profile: profile,
	}

	if _, ok = c.cache[key]; ok {
		return key, true
	}
	return key, false
}
func (c *cache) getVal(key cacheKey) cacheContent {
	val, _ := c.cache[key]
	return val
}
func (c *cache) addKey(key cacheKey, numStones, heighest int) {

	c.cache[key] = cacheContent{
		heightest: heighest,
		numStones: numStones,
	}
}

func main() {
	lines, _ := readLines("input")

	alreadySeen := newCache()
	field := createField(4, 7, 0)

	moveOk := false
	currPos := coord{row: field.height - 1, col: 2}
	numStones := 0

	currForm := -1
	maxNum := 1000000000000
	//maxNum := 2022 <-star1
	heightOffSet := 0
	for currJet := 0; numStones <= maxNum; currJet++ {
		currJet %= len(lines[0])

		if !moveOk {
			numStones += 1
			if currForm >= 0 {
				field.heighest = maxVal(field.heighest, currPos.row+1)

				key, ok := alreadySeen.foundRepetition(field.getProfileforRow(currPos.row), currJet, currForm)
				if ok {
					val := alreadySeen.getVal(key)
					fmt.Println("repeated at", key, val)
					stonesLeft := (maxNum - numStones)
					stonesSinceLast := numStones - val.numStones
					heightSinceLast := field.heighest - val.heightest
					repeatPattern := stonesLeft / stonesSinceLast
					heightOffSet += repeatPattern * heightSinceLast
					numStones += repeatPattern * stonesSinceLast
					alreadySeen = newCache()
				}

				alreadySeen.addKey(key, numStones, field.heighest)
			}

			currForm = (currForm + 1) % len(forms)
			field.extendIfNeeded(forms[currForm].height)
			currPos = coord{row: field.heighest + 2 + forms[currForm].height, col: 2}
			assert(field.applyForm(currPos, forms[currForm], 1), "should be free")
		}
		if lines[0][currJet] == '<' {
			currPos, _ = field.moveForm(currPos, GO_LEFT, forms[currForm])
		} else {
			currPos, _ = field.moveForm(currPos, GO_RIGHT, forms[currForm])
		}

		currPos, moveOk = field.moveForm(currPos, GO_DOWN, forms[currForm])
	}

	fmt.Println("height", field.heighest+heightOffSet)
}
