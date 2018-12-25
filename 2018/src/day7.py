import re
import sys
import math
import datetime 

class Worker():

    def __init__(self, w_id):
        self.w_id = w_id
        self.occupied = 0
        self.d_ip = -1

    def work_on(self, d_ip, deps):
        assert(self.occupied == 0)
        assert(deps[d_ip].valid)
        self.occupied = deps[d_ip].get_work_time()
        self.d_ip = d_ip 
        deps[d_ip].valid = False

    def available(self):
        return self.occupied == 0 

    def step(self,deps):
        if self.occupied == 1:
            assert(self.d_ip != -1)
            for c in deps[self.d_ip].childs:
                assert(deps[c].valid == True)
                deps[c].links -= 1
            self.d_ip = -1

        self.occupied = max(0,self.occupied - 1)
    

   


class Dependency():

    def __init__(self, d_ip):
        self.d_ip = d_ip
        self.links = 0
        self.childs = []
        self.valid = True  
        return None

    def get_work_time(self):
        return self.d_ip + 1 + 60

    def free(self):
        return self.links == 0


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

#too low 134
num_worker = 5
workers = []

for i in range(0,num_worker):
    workers.append(Worker(i))

answer = ""
something_valid = True
something_working = True
steps = 0
while something_valid or something_working:

    sys.stdout.write(str(steps) + "   " )
    for w in workers:
        w.step(deps)
    steps += 1
    something_working = False 
    something_valid = False 

    for w in workers:
        sys.stdout.write(str(chr(ord('A') + w.d_ip)) + "   ")
        if not w.available():
            something_working = True 
            continue 

        
        for d in range(0,max_id+1):
            if not d in deps:
                continue
            if not deps[d].valid:
                continue

            something_valid = True 
            if deps[d].free():
                w.work_on(d, deps)
                break

    sys.stdout.write(answer +"\n")
print("needed steps " + str(steps -1))
