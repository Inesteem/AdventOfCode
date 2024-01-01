#!/usr/bin/env python3

BROKEN = '#'
WHOLE = '.'
POSSIBLY_BROKEN= '?'

def cnt_broken(s : list[str]):
  num = 0
  ret = []
  for c in s:
    if c == BROKEN:
      num += 1
    else:
      if num:
        ret += [str(num)]
      num = 0
  if num:
    ret += [str(num)]
  #print("".join(s), " -> ", ",".join(ret))
  return ",".join(ret)

def get_codes(s, code, pos = 0):
  if pos == len(s):
    if cnt_broken(s) == code:
      return 1
    return 0

  num_codes = 0
  if s[pos] == POSSIBLY_BROKEN:
    s[pos] = WHOLE
    num_codes += get_codes(s, code, pos+1)
    s[pos] = BROKEN
    num_codes += get_codes(s, code, pos+1)
    s[pos] = POSSIBLY_BROKEN
    return num_codes

  return get_codes(s, code, pos + 1)

def main():
  lines=[l.rstrip() for l in open("input", "r").readlines()]

  num_codes = 0
  for line in lines:
    [s, code]=line.split()
    print(s, code)
    num_codes += get_codes([c for c in s],code)
    print()

  print(num_codes)

if __name__ == '__main__':
  main()
