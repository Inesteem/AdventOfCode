#!/usr/bin/env python3
# pylint: skip-file
# WARNING: NOT WORKING
from dataclasses import dataclass, field
import re
from enum import IntEnum
import copy

NUM_FLOORS = 4
MAX_STEPS = 1000000000


@dataclass
class Cache():
  cache: dict = field(default_factory=dict)

  def get_cache_key(self, e, floors, currFloor):
    key = ''
    for elem in [e, *floors, currFloor]:
      key += str(elem)

    key += str(currFloor)
    return key

  def has_key(self, key):
    return key in self.cache

  def add_key(self, key, steps_until):
    assert not self.has_key(key)
    self.cache[key] = [steps_until, MAX_STEPS]

  def update_steps_until(self, key, steps):
    assert self.has_key(key)
    self.cache[key][0] = min(steps, self.cache[key][0])

  def update_steps_followup(self, key, steps):
    assert self.has_key(key)
    self.cache[key][1] = steps

  def steps(self, key):
    assert self.has_key(key)
    return self.cache[key][0] + self.cache[key][1]


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
      print('\033[1m', end='')
    print('-------------------------------')
    print(f'g: {" ".join(self.generators)}')
    print(f'm: {" ".join(self.microchips)}')
    print('-------------------------------')
    if currFloor == self.index:
      print('\033[0m', end='')

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


class Elevator(Floor):

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


def applyCombi(combi, floor, e):
  [mc, (e2, t2)] = combi
  # Take from floor
  floor.removeMicrochip(mc)
  floor.remove(e2, t2)

  # Fill Elevator
  e.addMicrochip(mc)
  e.add(e2, t2)


def step(elevator, floors, currFloorIdx, curr_steps):
  old_floor = copy.deepcopy(floors[currFloorIdx])

  # Exit Condition
  if currFloorIdx == NUM_FLOORS - 1 and sum([f.numItems() for f in floors[0:3]
                                            ]) == 0:
    return 1

  # Todo: use functools.cache instead
  key = cache.get_cache_key(elevator, floors, currFloorIdx)
  if cache.has_key(key):
    cache.update_steps_until(key, curr_steps)
    return cache.steps(key)
  else:
    cache.add_key(key, curr_steps)

  # ---------------------------------
  for f in [floors[i] for i in [3, 2, 1, 0]] + [elevator]:
    f.print(currFloorIdx)
  input()
  # ---------------------------------

  min_steps = MAX_STEPS

  e = copy.deepcopy(elevator)
  e.unload(floors[currFloorIdx])
  for combi in allCombis(floors[currFloorIdx].generators,
                         floors[currFloorIdx].microchips):

    floor = copy.deepcopy(floors[currFloorIdx])
    applyCombi(combi, floors[currFloorIdx], e)

    if floor.somethingFries():
      e.clear()
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

  floors[currFloorIdx] = old_floor
  cache.update_steps_followup(key, min_steps + 1)
  return min_steps + 1


if __name__ == '__main__':
  floors = [Floor(index=i) for i in range(NUM_FLOORS)]

  gen_pattern = re.compile(r'([^ ]+) generator')
  mc_pattern = re.compile(r'([^ ]+)-compatible')

  for i, line in enumerate(open('../input/day11_test').readlines()):
    for gen in re.findall(gen_pattern, line):
      floors[i].addGenerator(gen)
    for mc in re.findall(mc_pattern, line):
      floors[i].addMicrochip(mc)

  minSteps = step(
      Elevator(),
      floors,
      0,
      0,
  )
  print('min steps: ', minSteps)
