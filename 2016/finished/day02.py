import math
import sys

def reverse(string):
    string = string[::-1]
    return string

class Move:
    def __init__(self, a = 0, b = 0, c = 0):
        self.a = a
        self.b = b
        self.c = c
    def __str__(self):
        return str(self.x) + ', ' + str(self.y)

    def check(self):


        return True
        

def getTrinagles(path, triangles):
    with open(path, "r") as f:
        data = f.readlines()

    for line in data:
        line = str(line.rstrip('\r\n'))
        m_line = []
        for d in line:
            m = Move(0,0)
            if d == 'L':
                m.x = -1
            elif d == 'R':
                m.x = +1
            elif d == 'U':
                m.y = -1
            else:
                m.y = +1

            m_line.append(m)
        moves.append(m_line)


numgrid = [
            ['*','*','1','*','*'],
            ['*','2','3','4','*'],
            ['5','6','7','8','9'],
            ['*','A','B','C','*'],
            ['*','*','D','*','*']
          ]
moves = []

getMoves('input/code',moves)
#getMoves('input/test',moves)
numbers = []
pos = [0,2]
    
for m_list in moves:
    for m in m_list:
        print m
        m.doMove(numgrid, pos)
    numbers.append(numgrid[pos[1]][pos[0]])            

    
print numbers



