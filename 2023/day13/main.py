#!/usr/bin/env python3
from typing import Optional 

ROCK = '#'
ASH = '.'

def bit_count(n):
   count = 0
   while n:
      count += n & 1
      n >>= 1
   return count
 
def have_smug_distance(x, y):
   return bit_count(x ^ y) == 1

def check_sym(nums : list[int], idx : int, smugs:int = 0):
  offs = 0
  while idx - offs - 1 >= 0 and idx + offs < len(nums):
    n1=nums[idx - offs - 1]
    n2=nums[idx + offs]
    if n1 != n2:
      if smugs > 0 and have_smug_distance(n1, n2):
        smugs -= 1
      else:
        return 0
    offs += 1

  return offs

def check_sym_nums(nums : list[int], skip :int, smugs:int = 0)->int:
  for r in range(1,len(nums)):
    if r != skip:
      s = check_sym(nums, r, smugs)
      if s:
        return r

  return 0

class Ground:
  def __init__(self):
    self.board = []
    self.rows = []
    self.cols = []
    self.col_sym = -1
    self.row_sym = -1

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

  def check_sym_rows(self, smugs:int = 0):
    self.row_sym = check_sym_nums(self.rows, self.row_sym, smugs)
    return self.row_sym

  def check_sym_cols(self, smugs:int = 0):
    self.col_sym = check_sym_nums(self.cols, self.col_sym, smugs)
    return self.col_sym

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
  star2 = 0
  while ground := next(grounds):
    print(ground)
    star1 += ground.check_sym_cols() + 100 * ground.check_sym_rows()
    star2 += ground.check_sym_cols(1) + 100 * ground.check_sym_rows(1)

  print("star1:",star1)
  print("star2:",star2)

if __name__ == '__main__':
  main()
