#!/usr/bin/env python3

from collections import defaultdict
from dataclasses import dataclass
import re
import sys

@dataclass
class Edge:
  left : str = ""
  right : str = ""

  def __repr__(self):
    return f"({self.left} {self.right})"

  def __str__(self):
    return self.__repr__()


def steps(edges, instructions, start ="AAA", end="ZZZ"):
  node = start
  i = 0
  while re.match(end, node) == None:
    d = instructions[i%len(instructions)]
    if d == "L":
      node = edges[node].left
    else:
      node = edges[node].right
    i += 1
  return i

def gcd(x, y):

   while(y):
       x, y = y, x % y
   return x

def _lcm(x, y):
   lcm = (x*y)//gcd(x,y)
   return lcm

def lcm(args : list[int]):
  if len(args) == 1:
    return args[0]
  res = _lcm(args[0], args[1])
  for arg in args[2:]:
    res = _lcm(res, arg)
  return res

def main():
  lines = [l.strip() for l in open(sys.argv[1], "r").readlines()]

  instructions = lines[0]

  edges = defaultdict(Edge)
  start_nodes = set()
  end_nodes = set()

  for l in lines[2:]:
    s = l.split(" ")
    edges[s[0]] = Edge(s[2][1:-1], s[3][:-1])

    if s[0][2] == "A":
      start_nodes.add(s[0])
    elif s[0][2] == "Z":
      end_nodes.add(s[0])

  print(start_nodes, end_nodes)
  #print("star1", steps(edges, instructions))
  
  node_steps = []
  for node in start_nodes:
    node_steps += [steps(edges, instructions, node, "..Z")]
  print(lcm(node_steps))
 # nodes = [n for n in start_nodes]
 # step=0
 # while not all([n in end_nodes for n in nodes]):
 #   d = instructions[step%len(instructions)]
 #   for i in range(len(nodes)):
 #     if d == "L":
 #       nodes[i] = edges[nodes[i]].left
 #     else:
 #       nodes[i] = edges[nodes[i]].right
 #   step += 1
 #   print(step)
 # print("star2",step)


if __name__ == "__main__":
  main()
