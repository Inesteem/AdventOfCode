#!/usr/bin/env python3
from dataclasses import dataclass, field

@dataclass
class Range:
  start : int
  length : int

  def inRange(self, src : int):
    return src >= self.start and src < self.start + self.length
 
  def overlaps(self, other: range):
    return self.inRange(other.start) or (other.start < self.start and
                                         other.end() > self.start) 

  def toString(self):
    return f"{self.start} [{self.length}]"
  
  def schnittmenge(self, other):
    if not self.overlaps(other):
      return Range(0,0)
    # self:   |----------|        |------|
    # other:         |--------|      |-|
    # ---->          |----|          |-|
    if self.inRange(other.start):
      return Range(other.start, min(other.end(), self.end())-other.start)
    # self:         |----------|
    # other:    |--------|
    # ---->         |----|
    elif self.start > other.start and other.end() <= self.end():
      return Range(self.start, other.end() - self.start)
    
    return Range(self.start, self.length)

  def end(self):
    return self.start+self.length

  def _subtract(self, other):
    if not self.overlaps(other):
      return [Range(self.start, self.length)]
    if self.start < other.start:
      # self:    |---|
      # other:        |---|
      # ----->   |---|
      if self.end() < other.start:
        return [Range(self.start, self.length)]

      # self:   |----------|
      # other:         |--------|
      # ---->   |-----|
      if self.end() < other.end():
        return [Range(self.start, other.start - self.start)]
      # self:   |-----------|
      # other:         |-|
      # ---->   |------|  |-|
      return [Range(self.start, other.start - self.start),
              Range(other.end(), self.end() - other.end())]
    # self:    |-----------|
    # other: |---|
    # ---->      |---------|
    if other.end() < self.end():
        return [Range(other.end(), self.end() - other.end())]
    # self:    |-----------|
    # other:  |---------------|
    # ---->      ||
    return [Range(0,0)]

  def subtract(self, other):
    return [s for s in self._subtract(other) if not s.empty()]

  def empty(self):
    return self.length==0
    
class Mapping:
  src: Range
  dst: int

  def __init__(self, dst_start, src_start, length):
    self.src = Range(src_start, length)
    self.dst = dst_start

  def inRange(self, val :int):
    return self.src.inRange(val)

  def transform(self, val:int):
    return val - self.src.start + self.dst
  
  def transformRange(self, r:Range):
    return Range(self.transform(r.start), r.length) 

  def toString(self):
    return f"{self.src.toString()} -> {self.dst}"

@dataclass
class Map:
  mappings : list[Mapping] = field(default_factory=list)

  def add(self, line):
    [dst_start, src_start, length] = [int(x) for x in line.split()]
    self.mappings.append(Mapping(dst_start, src_start, length))

  def sort(self):
    self.mappings = self.mappings.sort(key= lambda m : m.src.start)

  def transform(self, val):
    for m in self.mappings:
      if m.inRange(val):
        return m.transform(val)
    return val

  def destRanges(self, r : Range):
    ranges = []
    for m in self.mappings:
      if r.empty():
        return ranges
      if m.src.overlaps(r):
        for results in [self.destRanges(rs) for rs in r.subtract(m.src)]:
            ranges += results
        ranges += [m.transformRange(r.schnittmenge(m.src))]
        return ranges
    ranges += [r]
    return [r for r in ranges if not r.empty()]


def readMapping(lines, i):
  m = Map()
  while i < len(lines) and len(lines[i]):
    m.add(lines[i])
    i+=1
  return [m, i]



def transform(transformators, seed : int):
  val = seed
  for t in transformators:
    val = t.transform(val)
  return val


def rangeMin(transformators, r: Range):
  if len(transformators) == 0:
    return r.start

  ranges = transformators[0].destRanges(r)
  return min([rangeMin(transformators[1:], r) for r in ranges])


def test():
  r1=Range(start=20,length=5)
  assert r1.overlaps(r1)
  assert r1.schnittmenge(r1) == r1
  assert r1.subtract(r1) == []
  
  for r2 in [Range(start=25,length=5), Range(start=15,length=5)]:
    assert not r1.overlaps(r2)
    assert r1.schnittmenge(r2) == Range(0,0)
    assert r1.subtract(r2) == [r1]

 
  for r2 in [Range(start=15,length=6), Range(start=24,length=5)]:
    assert r1.overlaps(r2)

  assert r1.schnittmenge(Range(24,1)) == Range(24, 1)
  assert r1.schnittmenge(Range(23,1)) == Range(23, 1)
  assert r1.schnittmenge(Range(23,3)) == Range(23, 2)
  assert r1.schnittmenge(Range(24,8)) == Range(24, 1)
  assert r1.schnittmenge(Range(19,6)) == Range(20, 5)
  assert r1.schnittmenge(Range(19,5)) == Range(20, 4)
  assert r1.schnittmenge(Range(19,2)) == Range(20, 1)
  assert r1.schnittmenge(Range(19,1)) == Range(0, 0)


  assert r1.subtract(Range(22,2)) == [Range(20, 2), Range(24, 1)]
  assert r1.subtract(Range(20,2)) == [Range(22, 3)]
  assert r1.subtract(Range(23,2)) == [Range(20, 3)]
  assert r1.subtract(Range(15,5)) == [Range(20, 5)]
  assert r1.subtract(Range(15,6)) == [Range(21, 4)]
  assert r1.subtract(Range(25,5)) == [Range(20, 5)]
  assert r1.subtract(Range(24,6)) == [Range(20, 4)]
  assert r1.subtract(Range(19,7)) == []

def main():

  seeds = None
  seed_to_soil = None
  soil_to_fert = None 
  fert_to_water = None 
  water_to_light = None
  light_to_temp = None 
  temp_to_hum = None 
  hum_to_loc = None
  lines = [l.rstrip() for l in open('input', 'r').readlines()]
  i = 0
  while i < len(lines):
    if lines[i] == "":
      i+=1
      continue
    
    if lines[i][:6] == "seeds:":
      seeds = [int(s) for s in lines[i].split()[1:]]
    if lines[i] == "seed-to-soil map:":
      [seed_to_soil, i] = readMapping(lines, i+1)
    elif lines[i] == "soil-to-fertilizer map:":
      [soil_to_fert, i] = readMapping(lines, i+1)
    elif lines[i] == "fertilizer-to-water map:":
      [fert_to_water, i] = readMapping(lines, i+1)
    elif lines[i] == "water-to-light map:":
      [water_to_light, i] = readMapping(lines, i+1)
    elif lines[i] == "light-to-temperature map:":
      [light_to_temp, i] = readMapping(lines, i+1)
    elif lines[i] == "temperature-to-humidity map:":
      [temp_to_hum, i] = readMapping(lines, i+1)
    elif lines[i] == "humidity-to-location map:":
      [hum_to_loc, i] = readMapping(lines, i+1)
  
    i+=1

  transformators = [seed_to_soil, soil_to_fert, fert_to_water, water_to_light,
         light_to_temp, temp_to_hum, hum_to_loc]
  star1= 999999999999999
  for s in seeds:
    star1=min(star1, transform(transformators, s))

  print("star1:", star1)

  star2 = 999999999999999
  for i in range(0, len(seeds), 2):
    start = seeds[i]
    length = seeds[i+1]
    star2 = min(star2, rangeMin(transformators, Range(start, length)))

  print("star2:", star2)
if __name__ == '__main__':
  main()
