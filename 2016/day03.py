import math
import sys

def reverse(string):
    string = string[::-1]
    return string

class Triangle:
    def __init__(self, a = 0, b = 0, c = 0):
        self.a = a
        self.b = b
        self.c = c
    def __str__(self):
        return str(self.x) + ', ' + str(self.y)

    def check(self):
        if self.a + self.b <= self.c :
            return False

        if self.b + self.c <= self.a :
            return False

        if self.c + self.a <= self.b :
            return False

        return True
        

def getTriangles(path, triangles):
    with open(path, "r") as f:
        data = f.readlines()

    for line in data:
        a,b,c = (str(x) for x in line.split())
        c = str(c.rstrip('\r\n'))
        t = Triangle(int(a), int(b), int(c))
        triangles.append(t)



tris = []

getTriangles('input/triangles',tris)
num = 0
for tri in tris:
    if tri.check():
        num += 1
    
print num


