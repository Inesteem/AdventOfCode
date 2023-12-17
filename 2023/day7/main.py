#!/usr/bin/env python3
from collections import defaultdict
from  functools import reduce

class Hand:
  def __init__(self, line: list[str]):
    assert len(line) == 2
    self.cards = line[0]
    self.cmp= line[0].replace("T", "B").replace("J", "C").replace("Q","D").replace("K","E").replace("A","F")
    self.cmp2= line[0].replace("T", "B").replace("J", "0").replace("Q","D").replace("K","E").replace("A","F")
    self.bid = int(line[1])
    self._rank = 0
    self._rank2 = ""

  def __str__(self):
    return self.__repr__()

  def __repr__(self):
    return f"{"".join(self.cards)} {self.bid}"

  def rank(self):
    if not self._rank:
      chars =defaultdict(int)
      for c in self.cards:
        chars[c] += 1
      key = "".join(sorted([str(c) for c in chars.values()], reverse=True))
      self._rank = int(key)
    return str(self._rank)

  def rank2(self):
    if not self._rank2:
      for replace in ["2", "3","4","5","6","7","8","9","T","Q","K","A"]:
        chars = defaultdict(int)
        for c in self.cards.replace("J",replace):
          chars[c] += 1
        key = "".join(sorted([str(c) for c in chars.values()], reverse=True))
        self._rank2 = max(self._rank2, key)
    return str(self._rank2)

def cmpHands(h1 : Hand, h2 : Hand):
  if h1.cards == h2.cards:
    return 0;

  if h1.rank() == h2.rank():
    return -1 if h1.cards < h2.cards else 1

  return -1 if h1.rank() < h2.rank() else 1
  

def testCmp():
   assert Hand(["AAAAA",1]).rank() == "5"
   assert Hand(["AAAAB",1]).rank() == "41"
   assert Hand(["AAA11",1]).rank() == "32"
   assert Hand(["AAA12",1]).rank() == "311"
   assert Hand(["22334",1]).rank() == "221"
   assert Hand(["12345",1]).rank() == "11111"
   assert Hand(["AAJJJ",1]).rank2() == "5"
   assert Hand(["AA1JJ",1]).rank2() == "41"
  
   hands = [Hand(l) for l in [["K", 1],["J", 1],["Q", 1],["T", 1],["9", 1],["A", 1],["1", 1]]] 
   exp=[Hand(l) for l in [["A", 1],["K", 1],["Q", 1],["J", 1],["T",1],["9", 1],["1", 1]]]
   hands.sort(key=lambda h : (h.rank(), h.cmp), reverse=True)
   assert "".join([str(h) for h in hands]) == "".join([str(e) for e in exp])

   hands = [Hand(l) for l in [["11111", 1],["12345", 1],["222QQ",
                                                         1],["3344J",1],["JJJJQ",
                                                                         1],["99123",1]]]
   exp = [Hand(l) for l in [["11111", 1],["JJJJQ", 1],["222QQ",
            1],["3344J",1],["99123",1],["12345", 1]]]
   hands.sort(key=lambda h : (h.rank(), h.cmp), reverse=True)
   print(hands)
   assert "".join([str(h) for h in hands]) == "".join([str(e) for e in exp])



def print_results(msg, hands):
  star1 = reduce(lambda x,y : x+y,[(i+1)*h.bid for i, h in enumerate(hands)])
  print(msg, star1)

def main():
  testCmp()
  hands = [Hand(x.strip().split()) for x in open("input", "r").readlines()]
  hands.sort(key=lambda h : (h.rank(), h.cmp))
  print_results("star1 ",hands)
  print()
  hands.sort(key=lambda h : (h.rank2(), h.cmp2))
  print_results("star2 ",hands)

if __name__ == '__main__':
  main()



