
import re
import sys 
from operator import add

go_right = 2
go_left  = 0
go_up    = 1
go_down  = 3


directions = [[0,-1],[-1,0], [0,1], [1,0]]

def take_intersection(cart):
    if cart.cross == 0:#left
        cart.dir -= 1
    elif cart.cross == 2:#right
        cart.dir += 1
        cart.cross = -1

    cart.cross += 1
    cart.dir %= 4


def match(p1, p2):
    return p1[0] == p2[0] and p1[1] == p2[1]

class Cart:
    def __init__(self, p, d, cid):
        self.pos = p
        self.dir = d
        self.id = cid 
        self.cross = 0
        self.alive = True

    def move(self, lines):
        next_pos = list(map(add, self.pos, directions[self.dir])) 
        
        c = lines[next_pos[0]][next_pos[1]]

        if c == '\\':
            if self.dir == go_right or self.dir == go_left:
                self.dir += 1
            else: 
                self.dir -= 1
        elif c == '/':
            if self.dir == go_right or self.dir == go_left:
                self.dir -= 1
            else: 
                self.dir += 1
        elif c == '+':
            take_intersection(self)

        self.pos = next_pos 
        cart.dir %= 4


    def to_str(self):
        if match(self.dir, go_left):
            return "<"
        
        if match(self.dir, go_right):
            return ">"

        if match(self.dir, go_up):
            return "^"

        return "v"

lines = [line.rstrip('\n') for line in open('../data/day13.dat')]
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]


carts = []
cid = 0
for y in range(0,len(lines)):
    lines[y] = list(lines[y])
    for x in range(0,len(lines[y])):
        c = lines[y][x]
        if c == ">":
            carts.append(Cart([y,x], go_right, cid))
            lines[y][x] = "-"
            cid += 1
        elif c == "<":
            carts.append(Cart([y,x], go_left, cid))
            lines[y][x] ="-"
            cid += 1
        elif c == "^":
            carts.append(Cart([y,x], go_up, cid))
            lines[y][x] ="|"
            cid += 1
        elif c == "v":
            carts.append(Cart([y,x], go_down, cid))
            lines[y][x] ="|"
            cid += 1


while 1:

#    for y in range(0,len(lines)):
#        for x in range(0,len(lines[y])):
#            written = False
#            for cart in carts:
#                if y == cart.pos[0] and x == cart.pos[1]:
#                    sys.stdout.write(cart.to_str())
#                    written = True
#                    break 
#            if not written:
#                sys.stdout.write(lines[y][x])
#
#        print("")
#
#    print("")

    for cart in carts:
        if not cart.alive:
            continue
        cart.move(lines)
        

        for c1 in carts:
            if not c1.alive: 
                continue
            for c2 in carts:
                if c1.id == c2.id: #or not c2.alive: 
                    continue
                if c1.pos == c2.pos:
                    c1.alive = False    
                    c2.alive = False    


    for i in range(len(carts)-1,-1,-1):
        if not carts[i].alive:
            del carts[i]

    if len(carts) == 1:
        print(carts[0].pos[1])
        print(carts[0].pos[0])
        exit(0)
    #input("")
