package main

import (
	"regexp"
	"strings"
)

var oreCostReg = regexp.MustCompile("([0-9]+) ore")
var clayCostReg = regexp.MustCompile("([0-9]+) clay")
var obsidianCostReg = regexp.MustCompile("([0-9]+) obsidian")
var blueprintReg = regexp.MustCompile("Blueprint ([0-9]+):")

func getNumber(line string, reg *regexp.Regexp) int {
	match := reg.FindStringSubmatch(line)
	if len(match) == 2 {
		return toInt(match[1])
	}
	return 0
}

func parseCosts(line string) [3]int {
	return [3]int{
		getNumber(line, oreCostReg),
		getNumber(line, clayCostReg),
		getNumber(line, obsidianCostReg),
	}
}

func parseBlueprint(line string) blueprint {
	lines := strings.Split(line, ".")
	maxRobots := [4]int{0, 0, 0, 100}
	costs := [4][3]int{
		parseCosts(lines[0]),
		parseCosts(lines[1]),
		parseCosts(lines[2]),
		parseCosts(lines[3])}

	for i := 0; i < 4; i++ {
		maxRobots[0] = maxVal(maxRobots[0], costs[i][0])
		maxRobots[1] = maxVal(maxRobots[1], costs[i][1])
		maxRobots[2] = maxVal(maxRobots[2], costs[i][2])
	}

	return blueprint{
		number: getNumber(lines[0], blueprintReg),
		costs:  costs,
		state: blueprintState{
			maxRobots: maxRobots,
			robots:    [4]int{1, 0, 0, 0},
			resources: [4]int{0, 0, 0, 0},
		},
	}
}
