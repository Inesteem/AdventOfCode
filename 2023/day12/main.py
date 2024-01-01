#!/usr/bin/env python3
from cachetools import cached
from cachetools.keys import hashkey

BROKEN = '#'
WHOLE = '.'
POSSIBLY_BROKEN= '?'

@cached(cache={}, key=lambda s, c, s_l, c_l, b=0, p1=0, p2=0:
        hashkey(s,c,b,p1,p2,s_l[p1] if p1 < len(s_l) else -1,c_l[p2] if p2 < len(c_l)
                else -1))
def get_codes(s_str, code_str, s : list[str], code : list[int], num_broken = 0,
              pos = 0, c_pos = 0):
  if pos == len(s):
    if num_broken > 0:
      if c_pos == len(code)-1:
        if code[c_pos] == num_broken:
          return 1
      return 0
    if c_pos == len(code):
      return 1
    return 0

  if s[pos] == WHOLE:
    if num_broken > 0:
      if c_pos == len(code):
        return 0
      if num_broken != code[c_pos]:
        return 0
      return get_codes(s_str, code_str, s, code, 0, pos + 1, c_pos +1)
    return get_codes(s_str, code_str, s, code, 0, pos + 1, c_pos)
    
  if s[pos] == BROKEN:
    return get_codes(s_str, code_str, s, code, num_broken+1, pos + 1, c_pos)

  num_codes = 0
  s[pos] = WHOLE
  num_codes += get_codes(s_str, code_str, s, code, num_broken, pos, c_pos)
  s[pos] = BROKEN
  num_codes += get_codes(s_str, code_str, s, code, num_broken, pos, c_pos)
  s[pos] = POSSIBLY_BROKEN
  return num_codes

def main():
  lines=[l.rstrip() for l in open("input", "r").readlines()]

  num_codes = 0
  for line in lines:
    [s, code]=line.split()
    codes= get_codes(s, code, [c for c in ((s+'?')*5)[:-1]], [int(c) for c in code.split(",")]*5)
    print(codes)
    num_codes +=codes 
    print()

  print(num_codes)

if __name__ == '__main__':
  main()
