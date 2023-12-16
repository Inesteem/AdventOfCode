#!/usr/bin/env python3
from functools import reduce

def race_won(time_left, speed, record):
  return record < time_left * speed


def check_race_ways(time, record):
  num = 0
  for t in range(1, time):
    print(t,race_won(time - t, t, record)) 
    num += 1 if race_won(time - t, t, record) else 0
  return num

# gets the first time where we end winning if we hold the car
# by binary search
def find_end_stime(start, end, time, record):
    if start == end:
      return end
    mid = start + int((end-start) / 2)
    if not race_won(time-mid, mid, record):
      return find_end_stime(start, mid, time, record)
    return find_end_stime(mid+1, end, time, record)


# gets the first time where we start winning if we hold the car
# by binary search
def find_start_stime(start, end, time, record):
    if start == end:
      return start
    mid = start + int((end-start) / 2)
    if race_won(time-mid, mid, record):
      return find_start_stime(start, mid, time, record)
    return find_start_stime(mid+1, end, time, record)

def check_race_ways_bin_search(time, record):
  return find_end_stime(1, time, time, record) - find_start_stime(1, time-1, time, record)

def main():
  lines = [l.strip() for l in open('input', 'r').readlines()]
  assert len(lines) == 2
  time = [int(x) for x in lines[0].split()[1:]]
  distance = [int(x) for x in lines[1].split()[1:]]
  assert len(time) == len(distance)


  start_speed = 0 # in mm per ms
  increase = 1 # mm per ms

  nums = [check_race_ways_bin_search(time[i], distance[i]) for i in range(len(time))]
  print("star1:", reduce(lambda x,y : x*y, nums))

  star2 = check_race_ways_bin_search(int("".join([str(t) for t in time])),
                                     int("".join([str(d) for d in distance])))
  print("star2:",star2)


if __name__ == '__main__':
  main()
