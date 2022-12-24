package main

import (
	"fmt"
)

var MINUTES = 24

const (
	ORE = iota
	CLAY
	OBSIDIAN
	GEODE
	NUM
)

type blueprintState struct {
	maxRobots [NUM]int
	robots    [NUM]int
	resources [NUM]int
}

func (bps blueprintState) addResources(resources [NUM]int) blueprintState {
	for i := range bps.resources {
		bps.resources[i] += resources[i]
	}
	return bps
}

func (bps *blueprintState) canBuyGeodeRobotEachTurn(bp *blueprint) bool {
	for i := 0; i < GEODE; i++ {
		if bp.costs[GEODE][i] > bps.robots[i] {
			return false
		}
	}
	return true
}

func (bps blueprintState) buyRobot(costs [4][3]int, pos int) (canBuy bool, nextState blueprintState) {
	for i := range costs[pos] {
		if bps.resources[i] < costs[pos][i] {
			return false, bps
		}
		bps.resources[i] -= costs[pos][i]
	}
	bps.robots[pos] += 1
	return true, bps
}

type blueprint struct {
	number int
	costs  [4][3]int
	state  blueprintState
}

func getKey(list []int) (key int64) {
	for _, c := range list {
		key <<= 4
		key += int64(c)
	}
	return key
}

// information about resources is encoded in number of robots and timestamps
type stateKey struct {
	robotKey    int64
	resourceKey int64
	timestep    int
}
type stateMap map[stateKey]int

// number of potential additional geodes, if one geode robot would be added in every
// timestep that is left (as upper bound, we may not be able to do so)
func (bps *blueprintState) potentialMax(timestep int) int {
	N := MINUTES - timestep
	return ((N * (N + 1)) / 2) + bps.robots[GEODE]*N + bps.resources[GEODE]
}

func (b *blueprint) getMostGeodes(timestep int, bps blueprintState, sm *stateMap, currMax int) int {
	if timestep == MINUTES || bps.potentialMax(timestep) <= currMax {
		return bps.resources[GEODE]
	}

	//Cache
	sk := stateKey{
		robotKey:    getKey(bps.robots[0:NUM]),
		resourceKey: getKey(bps.resources[0:GEODE]), //skip geodes
		timestep:    timestep,
	}
	val, ok := (*sm)[sk]
	if ok {
		return val
	}

	maxGeodes := 0
	canBuyAll := true
	for i := GEODE; i >= 0; i-- {
		if bps.robots[i] >= bps.maxRobots[i] {
			continue
		}
		canBuy, bpsNext := bps.buyRobot(b.costs, i)
		if canBuy {
			geodes := b.getMostGeodes(timestep+1, bpsNext.addResources(bps.robots), sm, currMax)
			maxGeodes = maxVal(maxGeodes, geodes)
			currMax = maxVal(currMax, maxGeodes)
		} else {
			canBuyAll = false
		}

		if bps.canBuyGeodeRobotEachTurn(b) {
			break
		}
	}
	if !canBuyAll {
		geodes := b.getMostGeodes(timestep+1, bps.addResources(bps.robots), sm, currMax)
		maxGeodes = maxVal(maxGeodes, geodes)
		currMax = maxVal(currMax, maxGeodes)
	}
	(*sm)[sk] = maxGeodes
	return maxGeodes
}

func main() {
	lines, _ := readLines("input")

	star1, star2 := 0, 1
	for i := 0; i < len(lines); i++ {
		stateMap := make(stateMap, 0)
		blueprint := parseBlueprint(lines[i])
		fmt.Println(blueprint)
		geodes := blueprint.getMostGeodes(0, blueprint.state, &stateMap, 0)
		fmt.Println("Blueprint", blueprint.number, "produces", geodes, "geodes")
		star1 += blueprint.number * geodes
	}

	MINUTES = 32
	for i := 0; i < 3; i++ {
		stateMap := make(stateMap, 0)
		blueprint := parseBlueprint(lines[i])
		fmt.Println(blueprint)
		geodes := blueprint.getMostGeodes(0, blueprint.state, &stateMap, 0)
		fmt.Println("Blueprint", blueprint.number, "produces", geodes, "geodes")
		star2 *= geodes
	}
	fmt.Println("star1:", star1)
	fmt.Println("star2:", star2)
}
