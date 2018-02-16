import math
import sys

def reverse(string):
    string = string[::-1]
    return string

class iPos:
    def __init__(self):
        self.airt = 0
        self.x = 0
        self.y = 0
    def __str__(self):
        return str(self.airt) + ' ['+str(self.x)+','+str(self.y)+']'

    def move(self, move):
        self.airt += move.side
        self.airt %= 4
        
        if self.airt == 0: # NORTH
            self.y += move.steps 
        elif self.airt == 1: 
            self.x += move.steps
        elif self.airt == 2: 
            self.y -= move.steps
        else: 
            self.x -= move.steps

    def m_dis(self):
        return int(math.fabs(self.x) + math.fabs(self.y))

class Move:
    def __init__(self):
        self.side = 0
        self.steps = 0
    def __str__(self):
        return str(self.side) + ': ' + str(self.steps)


def getMoves (path, moves):
    with open(path, "r") as f:
        data = f.readlines()

    for line in data:
        for x in line.split(", "):
            m = Move() 
            path = str(x.rstrip('\r\n'))
            if path[0] == 'L':
                m.side = -1
            else:
                m.side = 1

            m.steps = int(path[1:])
            moves.append(m)


moves = []
#path = 'input/moves'
path = 'input/test'
if len(sys.argv) > 1:
    path =  str(sys.argv[1])
getMoves(path,moves)
ip = iPos()

for move in moves:
    ip.move(move)
    print ip 
  
print 'distance: ' + str(ip.m_dis())

