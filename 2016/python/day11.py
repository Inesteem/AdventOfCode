#!/usr/bin/env python3
from dataclasses import dataclass, field
import re
from enum import IntEnum
import copy


@dataclass
class Cache():
    cache: dict = field(default_factory=dict)

    def get_cache_key(self, e, floors, currFloor):
        key = ""
        for elem in [e, *floors, currFloor]:
            key += str(elem)

        key += str(currFloor)
        return key

    def is_better(self, key, numSteps):
        if key in self.cache and self.cache[key] < numSteps:
            return True
        self.cache[key] = numSteps
        return False

    def val(self, key):
        return self.cache[key]


cache = Cache()


class Type(IntEnum):
    GENERATOR = 0
    MICROCHIP = 1


class Direction(IntEnum):
    UP = 0
    DOWN = 1


@dataclass
class Floor(object):
    microchips: set = field(default_factory=set)
    generators: set = field(default_factory=set)
    index: int = 0

    def print(self, currFloor):
        #print(f'on the {self.index} floor are ')
        if currFloor == self.index:
            print('\033[1m', end="")
        print("-------------------------------")
        print(f'g: {" ".join(self.generators)}')
        print(f'm: {" ".join(self.microchips)}')
        print("-------------------------------")
        if currFloor == self.index:
            print('\033[0m', end="")

    def addGenerator(self, gen):
        if gen is None:
            return
        assert gen not in self.generators
        self.generators.add(gen)

    def addMicrochip(self, mc):
        if mc is None:
            return
        assert mc not in self.microchips
        self.microchips.add(mc)

    def removeGenerator(self, gen):
        if gen is None:
            return
        assert gen in self.generators, f'no {gen} generator'
        self.generators.remove(gen)

    def removeMicrochip(self, mc):
        if mc is None:
            return
        assert mc in self.microchips, f'no {mc} microchip'
        self.microchips.remove(mc)

    def somethingFries(self):
        if len(self.generators) == 0:
            return False

        for mc in self.microchips:
            if mc not in self.generators:
                return True
        return False

    def numItems(self):
        return len(self.generators) + len(self.microchips)

    def hasGenerator(self, gen):
        return gen in self.generators

    def __str__(self):
        return str(self.index) + str(self.generators) + str(self.microchips)

    def numMicrochips(self):
        return len(self.microchips)

    def remove(self, e, t):
        if t == Type.GENERATOR:
            self.removeGenerator(e)
        else:
            self.removeMicrochip(e)

    def add(self, e, t):
        if t == Type.GENERATOR:
            self.addGenerator(e)
        else:
            self.addMicrochip(e)

    # Elevator part (needs to be hashable aswell):
    def full(self):
        return self.numItems() == 2

    def canMove(self):
        return self.numMicrochips() > 0

    def clear(self):
        self.microchips = set()
        self.generators = set()

    def unload(self, floor):
        for mc in self.microchips:
            floor.addMicrochip(mc)
        for gen in self.generators:
            floor.addGenerator(gen)

        self.clear()


def allCombis(gens, mcs):
    x = [(mc, (None, None)) for mc in mcs]
    if len(mcs) > 0 and len(gens) > 0:
        x = [[x, (y, Type.GENERATOR)] for x in mcs for y in gens if x == y] + x
    if len(mcs) > 1:
        x = [[x, (y, Type.MICROCHIP)] for x in mcs for y in mcs if x != y] + x
    return x


NUM_FLOORS = 4
MAX_STEPS = 1000000000


def applyCombi(combi, floor, e):
    [mc, (e2, t2)] = combi
    # Take from floor
    floor.removeMicrochip(mc)
    floor.remove(e2, t2)

    # Fill Elevator
    e.addMicrochip(mc)
    e.add(e2, t2)


def step(elevator, floors, currFloorIdx, curr_steps):
    floor = floors[currFloorIdx]

    if currFloorIdx == NUM_FLOORS - 1 and sum(
        [f.numItems() for f in floors[0:3]]) == 0:
        return 1

    key = cache.get_cache_key(elevator, floors, currFloorIdx)
    if cache.is_better(key, curr_steps):
        return 1000000000

    # ---------------------------------
    for f in [floors[i] for i in [3, 2, 1, 0]] + [elevator]:
        f.print(currFloorIdx)
    input()
    # ---------------------------------

    min_steps = MAX_STEPS

    e = copy.deepcopy(elevator)
    e.unload(floor)
    for combi in allCombis(floor.generators, floor.microchips):

        floor = copy.deepcopy(floors[currFloorIdx])
        applyCombi(combi, floors[currFloorIdx], e)

        if floor.somethingFries():
            floors[currFloorIdx] = floor
            continue

        # Go UP
        if currFloorIdx < NUM_FLOORS - 1:
            min_steps = min(min_steps,
                            step(e, floors, currFloorIdx + 1, curr_steps + 1))
            e.clear()
            floors[currFloorIdx] = copy.deepcopy(floor)

        # Go Down
        if currFloorIdx > 0:
            min_steps = min(min_steps,
                            step(e, floors, currFloorIdx - 1, curr_steps + 1))
            e.clear()
            floors[currFloorIdx] = floor

    floors[currFloorIdx] = floor
    print(min_steps + 1)
    return min_steps + 1


if __name__ == '__main__':
    floors = [Floor(index=i) for i in range(NUM_FLOORS)]

    gen_pattern = re.compile(r'([^ ]+) generator')
    mc_pattern = re.compile(r'([^ ]+)-compatible')

    for i, line in enumerate(open("../input/day11_test").readlines()):
        for gen in re.findall(gen_pattern, line):
            floors[i].addGenerator(gen)
        for mc in re.findall(mc_pattern, line):
            floors[i].addMicrochip(mc)

    minSteps = step(
        Floor(),
        floors,
        0,
        0,
    )
    print("min steps: ", minSteps)
