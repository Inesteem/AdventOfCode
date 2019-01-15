
import re
import sys 

def append_dots(pots,rules, num_dots):
    dots = num_dots * "."
    if (dots + pots)[0:5] in rules:
        return True
    return False

def get_pot_count(pots, start_idx):
    pot_num = 0
    idx = start_idx 
    for pot in pots:
        if pot == '#':
            pot_num += idx 
        idx += 1
    return pot_num 

lines = [line.rstrip('\n') for line in open('../data/day12.dat')]

pots=lines[0][len("initial state: "):]

rules = {}

for i in range(2,len(lines)):
    rule=re.split(' => ',lines[i])
    rules[rule[0]] = rule[1];


start_idx = 0
pot_num = 0
last_pot_num = 0
#for gen in range(0,50000000000):
for gen in range(0,1000):

#    print(str(gen) + ": " + pots)
    pot_num = get_pot_count(pots,start_idx)
    diff = pot_num - last_pot_num 
    print(str(gen) + ": " +  str(pot_num) + "  " + str(diff))
    if gen > 1000:
        assert(diff == 42)
    last_pot_num = pot_num 
    if append_dots(pots,rules,4):
        pots = "...." + pots + "....." 
        start_idx -= 2
    elif append_dots(pots,rules,3):
        pots = "..." + pots + "....."
        start_idx -= 1
    else:
        pots = ".." + pots + "....."

    new_gen = ""

    for i in range(0,len(pots) - 4):
        key = pots[i:i+5]
        next_pot = "."
        if key in rules:
            next_pot = rules[key]
        
#        if next_pot == "." and len(new_gen) == 0:
#            continue
        new_gen += next_pot
    pots = new_gen
    pot_num = get_pot_count(pots, start_idx)
    while pots[0] == ".":
        pots = pots[1:]
        start_idx += 1
    while pots[-1] == ".":
        pots = pots[:-1]


    print(str(start_idx) + "  " +  str(pot_num))    
    

print(str(pot_num + (50000000000 - 1000) * 42))
#print(str(pot_num + (2000 - 1000) * 42))

#print(get_pot_count(".#....##....#####...#######....#.#..##.",-3))

