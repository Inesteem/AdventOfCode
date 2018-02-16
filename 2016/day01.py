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
    def __init__(self, side = 0, steps = 0):
        self.side = side
        self.steps = steps
    def __str__(self):
        return str(self.side) + ': ' + str(self.steps)


def getMoves (path, moves):
    with open(path, "r") as f:
        data = f.readlines()

    for line in data:
        for x in line.split(", "):
            path = str(x.rstrip('\r\n'))
            steps = int(path[1:])
            side = 1
            
            if path[0] == 'L':
                side = -1
            m = Move(side, 1) 
            moves.append(m)
            for i in xrange(steps-1):
                m = Move(0, 1) 
                moves.append(m)

def get_hash(x,y):
    ret = long(0)
    if x > 0:
        ret = x
    else:
        ret = long(long(math.fabs(x)) << 8)
    print 'part: ' + str(ret)

    if y > 0:
        ret += (y << 16)
    else:
        ret += long(long(math.fabs(y)) << 24)
    return ret

places = {}
moves = []
#path = 'input/moves'
path = 'input/test'
if len(sys.argv) > 1:
    path =  str(sys.argv[1])
getMoves(path,moves)
ip = iPos()

places[int(0)] = 0

for move in moves:
    
    ip.move(move)
    h = get_hash(ip.x,ip.y)
    print "ip: " + str(ip) 
    print "hash: " + str(h)
    if(places.get(long(h)) == None):
        places[h] = 0
    else:
        break
  
print 'distance: ' + str(ip.m_dis())

