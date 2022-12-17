package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
)

var sensorRegExp = regexp.MustCompile(`Sensor at x=([\d-]*), y=([\d-]*): closest beacon is at x=([\d-]*), y=([\d-]*)`)

func abs(i int64) int64 {
	if i < 0 {
		return -i
	}
	return i
}
func minVal(a, b int64) int64 {
	if a <= b {
		return a
	}
	return b
}
func maxVal(a, b int64) int64 {
	if a >= b {
		return a
	}
	return b
}
func toInt64(s string) int64 {
	i, _ := strconv.ParseInt(s, 10, 64)
	return int64(i)
}

type coord struct {
	x, y int64
}

func (c *coord) tuningFrequency() int64 {
	return c.x*int64(4000000) + c.y
}

func (c *coord) add(o *coord) coord {
	return coord{
		x: c.x + o.x,
		y: c.y + o.y,
	}
}
func (c *coord) eq(o *coord) bool {
	return c.x == o.x && c.y == o.y
}
func (c *coord) dist(o *coord) int64 {
	return maxVal(c.x, o.x) - minVal(c.x, o.x) + maxVal(c.y, o.y) - minVal(c.y, o.y)
}

func (c *coord) subVal(val int64) {
	c.x -= val
	c.y -= val
}
func (c *coord) addVal(val int64) {
	c.x += val
	c.y += val
}
func (c *coord) manhattenDistance(o *coord) int64 {
	return abs(c.x-o.x) + abs(c.y-o.y)
}

func (c *coord) setMaxVals(o coord) {
	c.x = maxVal(c.x, o.x)
	c.y = maxVal(c.y, o.y)
}
func (c *coord) setMinVals(o coord) {
	c.x = minVal(c.x, o.x)
	c.y = minVal(c.y, o.y)
}

type sensor struct {
	position, nearestBeacon coord
	dist                    int64
}

func (s *sensor) getMaxBeaconRange() coord {
	return coord{
		x: s.position.x + s.dist,
		y: s.position.y + s.dist,
	}
}
func (s *sensor) getMinBeaconRange() coord {
	return coord{
		x: s.position.x - s.dist,
		y: s.position.y - s.dist,
	}
}

func parseSensorData(line string) sensor {

	match := sensorRegExp.FindStringSubmatch(line)
	if got, want := len(match), 5; got != want {
		log.Fatalf("%s does not fit format; -want(%d) +got(%d)", line, want, got)
	}

	position := coord{x: toInt64(match[1]), y: toInt64(match[2])}
	nearestBeacon := coord{x: toInt64(match[3]), y: toInt64(match[4])}
	return sensor{
		position:      position,
		nearestBeacon: nearestBeacon,
		dist:          position.manhattenDistance(&nearestBeacon),
	}
}

type field struct {
	sensors          []sensor
	minVals, maxVals coord
}

func (f *field) addSensor(s sensor) {

	if len(f.sensors) == 0 {
		f.minVals = s.getMinBeaconRange()
		f.maxVals = s.getMaxBeaconRange()
	} else {
		f.minVals.setMinVals(s.getMinBeaconRange())
		f.maxVals.setMaxVals(s.getMaxBeaconRange())
	}

	f.sensors = append(f.sensors, s)
	fmt.Println(s, "    ", f.minVals, f.maxVals)
}

func (f *field) unbeaconedArea(c *coord) (bool, *sensor) {
	for si := range f.sensors {
		if c.manhattenDistance(&f.sensors[si].position) <= f.sensors[si].dist {
			return true, &f.sensors[si]
		}
	}
	return false, nil
}

func (f *field) isBeacon(c *coord) bool {

	for si := range f.sensors {
		if c.eq(&f.sensors[si].nearestBeacon) {
			return true
		}
	}
	return false
}

func (f *field) isSensor(c *coord) bool {

	for si := range f.sensors {
		if c.eq(&f.sensors[si].position) {
			return true
		}
	}
	return false
}

func (f *field) print() {

	for y := f.minVals.y; y <= f.maxVals.y; y++ {
		fmt.Printf("%4d: ", y)
		for x := f.minVals.x; x <= f.maxVals.x; x++ {
			c := coord{x: x, y: y}
			if f.isSensor(&c) {
				fmt.Print("S")
			} else if f.isBeacon(&c) {
				fmt.Print("B")
			} else if u, _ := f.unbeaconedArea(&c); u {
				fmt.Print(".")
			} else {
				fmt.Print("#")

			}
		}
		fmt.Println("")
	}
}

func main() {
	field := field{sensors: make([]sensor, 0)}
	//row := int64(10)
	//file, _ := os.Open("test")
	row := int64(2000000)
	file, _ := os.Open("input")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lineStr := scanner.Text()
		field.addSensor(parseSensorData(lineStr))
	}
	star1 := 0
	c := coord{x: field.minVals.x, y: row}
	for c.x <= field.maxVals.x {
		u, _ := field.unbeaconedArea(&c)
		if u && !field.isBeacon(&c) {
			star1++
		}
		c.x++
	}
	sort.Slice(field.sensors, func(i, j int) bool { return field.sensors[i].position.x < field.sensors[j].position.x })

	fmt.Println("\nstar1 : ", star1)

	// search distress beacon
	maxVals := coord{0, 0}
	for i := range field.sensors {
		maxVals.setMaxVals(field.sensors[i].position)
	}
out:
	for y := maxVal(0, field.minVals.y); y <= minVal(4000000, maxVals.y); y++ {
		for x := maxVal(0, field.minVals.x); x <= minVal(4000000, maxVals.x); x++ {
			c := coord{x: x, y: y}
			u, s := field.unbeaconedArea(&c)
			if u { //check those edge cases again, not 100% sure this fits
				if s.position.x > x {
					x = s.position.x + (s.dist - abs(s.position.y-y))
				} else {
					x += (s.dist - abs(s.position.y-y) - (x - s.position.x))
				}
			} else {
				fmt.Println("\nstar2 : ", c.tuningFrequency())
				break out
			}
		}
	}

}
