import re
import sys
import math
import datetime 


class Dependency():

    def __init__(self, d_ip):
        self.d_ip = d_ip
        self.links = 0
        self.childs = []
        self.valid = True  
        return None




lines = [line.rstrip('\n') for line in open('../data/day7.dat')]
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]

deps = {}


max_id = 0

for line in lines:
    splitted =  line.split(' ')
    parent = ord(splitted[1]) - ord('A')
    child = ord(splitted[7]) - ord('A')

    if not parent in deps:
        deps[parent] = Dependency(parent)

    if not child in deps:
        deps[child] = Dependency(child)

    deps[child].links += 1
    deps[parent].childs.append(child)

    max_id = max(max_id, max(child,parent))


#for d_ip,dep in deps.items():
#    print(str(chr(ord('A') + d_ip)) + " " + str(dep.links))
#    for c in dep.childs:
#        print(" --- " + str(chr(ord('A') + c)))
print(str(max_id) + " " + str(chr(ord('A')+max_id)))
answer = ""
something_valid = True 
while something_valid:
    something_valid = False 
    for d in range(0,max_id+1):
        if not d in deps:
            continue
        if not deps[d].valid:
            continue

        something_valid = True 
        if deps[d].links == 0:
            deps[d].valid = False
            print(str(chr(deps[d].d_ip + ord('A'))))
            for d2 in deps[d].childs:
                deps[d2].links -= 1
            answer += str(chr(ord('A') + d))
            break

print(answer)
