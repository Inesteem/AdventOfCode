#!/usr/bin/env python3

from collections import defaultdict
from dataclasses import dataclass

@dataclass
class Edge:
  left : str = ""
  right : str = ""

  def __repr__(self):
    return f"({self.left} {self.right})"

  def __str__(self):
    return self.__repr__()
def main():
  lines = [l.strip() for l in open("input", "r").readlines()]

  instructions = lines[0]

  edges = defaultdict(Edge)

  for l in lines[2:]:
    s = l.split(" ")
    edges[s[0]] = Edge(s[2][1:-1], s[3][:-1])

  node = "AAA"
  i = 0
  while node != "ZZZ":
    d = instructions[i%len(instructions)]
    if d == "L":
      node = edges[node].left
    else:
      node = edges[node].right
    i += 1
  print("steps", i)

if __name__ == "__main__":
  main()
