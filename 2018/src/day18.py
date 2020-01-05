import sys 
import re

free =  0
tree =  1
lyard = 2
border = 3

def count_ressources(land):
    num_t = 0
    num_y = 0
    for line in last:
        num_t += line.count(tree)
        num_y += line.count(lyard)
    return num_t * num_y


def print_land(land):
    for l1 in land:
        print()
        for l2 in l1:
            if l2 == free:
                sys.stdout.write('.')
            elif l2 == tree:
                sys.stdout.write('|')
            elif l2 == lyard:
                sys.stdout.write('#')
            else:
                sys.stdout.write(' ')

    print()

def next_state(land, y, x):
    place = land[y][x]
    nums = [0,0,0,0]
    for i in range(0,3):
        nums[land[y-1][x-1+i]] += 1
        nums[land[y+1][x-1+i]] += 1

    nums[land[y][x+1]] += 1
    nums[land[y][x-1]] += 1

    if place == free:
        if nums[tree] >= 3:
            return tree
    elif place == tree:
        if nums[lyard] >= 3:
            return lyard
    elif not (nums[tree] > 0 and nums[lyard] > 0):
        return free 

    return place 

def simulate(land1, land2):
    for y in range(1, len(land1)-1):
        for x in range(1, len(land1[y])-1):
            land2[y][x] = next_state(land1, y, x)



if len(sys.argv) > 1:
    lines = [line.rstrip('\n') for line in open('../data/test.dat')]
else:
    lines = [line.rstrip('\n') for line in open('../data/day18.dat')]


land1 = [[border for x in range(len(lines[0])+2)] for y in range(len(lines)+2)]
land2 = [[border for x in range(len(lines[0])+2)] for y in range(len(lines)+2)]


for y in range(len(lines)):
    for x in range(len(lines[y])):
        l = lines[y][x]
        if l == '#':
            land1[y+1][x+1] = lyard
        elif l == '|':
            land1[y+1][x+1] = tree
        else:
            land1[y+1][x+1] = free
    #land1.append(list(line))
    
    #land2.append(list(line))

print_land(land1)
last = land1 
r = 100


#cycle = [190143,203895,204486,202272,207172,208351,211140,212248,219349,218584,218286,213244,210630,205800,205412,201916,193120,
cycle = [189090, 190143, 187525, 190740, 189601,195471,195426,199758,198062,201684,200349,202515,203895,204486,202272,207172,208351,211140,212248,219349,218584,218286,213244,210630,205800,205412,201916,193120]

start = 16049
end = 1000000000 - 1
diff = end - start
print(cycle[diff%len(cycle)])
#answer: 202272



exit(0)

for i in range(1000000000):
    if i%2 == 0:
        simulate(land1, land2)
        last = land2 
    else: 
        simulate(land2, land1)
        last = land1 

#    print_land(last) 
    if i == r:
        print(str(i) + " - " + str(count_ressources(last)))
        r = int(input()) + i

print(str(num_t) + " * " + str(num_y) + " = " + str(num_y * num_t))
