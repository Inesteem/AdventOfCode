#!/usr/bin/env python3
from typing import Optional 

ROCK = '#'
ASH = '.'



def check_sym(nums : list[int], idx : int):
  offs = 0
  while idx - offs -1 >= 0 and idx + offs < len(nums):
    if nums[idx - offs - 1] != nums[idx + offs]:
      return 0
    offs += 1

  return offs

def check_sym_nums(nums : list[int])->int:
  for r in range(1,len(nums)):
    if nums[r-1] == nums[r]:
      s = check_sym(nums, r)
      if s:
        return r
  return 0

class Ground:
  def __init__(self):
    self.board = []
    self.rows = []
    self.cols = []

  def add_line(self, line : str):
    self.board += [line]
    num = 0
    if not len(self.cols):
      self.cols = [0] * len(line)
    for i,c in enumerate(line):
      num *= 2
      self.cols[i] *= 2
      if c==ROCK:
        num += 1
        self.cols[i] += 1
    self.rows.append(num)

  def check_sym_rows(self):
    return check_sym_nums(self.rows)

  def check_sym_cols(self):
    return check_sym_nums(self.cols)

  def __repr__(self):
    s ="\n".join(self.board)
    #s = "rows: " + ",".join(map(str,self.rows))
    #s += "\ncols: "+",".join(map(str,self.cols))
    #s += "\n"
    return s

  def __str__(self):
    return self.__repr__()

def readInput() -> Optional[str]:
  lines=[l for l in open("input", "r").readlines()]
  for line in lines:
    yield line

  yield None

def readGround() -> Optional[Ground]:
  ground=Ground()
  lines = readInput()
  while line := next(lines):
    if not len(line.rstrip()):
      yield ground
      ground = Ground()
      continue
    ground.add_line(line.rstrip())
  yield ground
  yield None

def main():
  grounds = readGround()
  star1 = 0
  while ground := next(grounds):
    print(ground)
    print(ground.check_sym_cols() , ground.check_sym_rows())
    star1 += ground.check_sym_cols() + 100 * ground.check_sym_rows()

  print(star1)

if __name__ == '__main__':
  main()
