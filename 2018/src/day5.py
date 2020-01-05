import re
import sys
import math
import datetime


lines = [line.rstrip('\n') for line in open('../data/day5.dat')]
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]
assert len(lines) == 1


best_len = len(lines[0])

for i in range(0,26):
    polymer = list(lines[0])
    c = str(chr(ord('a')+i))
    C = c.upper()
    print("checking " + c + " and " + C )
    polymer = list("".join(filter(lambda x : x != c and x != C, polymer)))

    changed = True
    while changed:
        changed = False
        for i in range(0, len(polymer)-1):

            if polymer[i].istitle() and not polymer[i+1].istitle():
                if polymer[i] == polymer[i+1].upper():
                    changed = True
                    polymer[i] = '-'
                    polymer[i+1] = '-'
                    i += 1
            elif polymer[i+1].istitle() and not polymer[i].istitle():
                if polymer[i+1] == polymer[i].upper():
                    changed = True
                    polymer[i] = '-'
                    polymer[i+1] = '-'
                    i += 1

        polymer = list("".join(filter(lambda x : x != "-", polymer)))
    if len(polymer) < best_len:
        best_len = len(polymer)
        print(len(polymer))
#            print(polymer)


print(best_len)
