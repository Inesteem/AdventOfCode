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
        return str(self.a) + ', ' + str(self.b)+ ', ' + str(self.c)

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
    t1 = Triangle()
    t2 = Triangle()
    t3 = Triangle()
    num = 0
    for line in data:
        num += 1
        x1,x2,x3 = (str(x) for x in line.split())
        x3 = str(x3.rstrip('\r\n'))

        if num == 1:
            t1.a = int(x1) 
            t2.a = int(x2) 
            t3.a = int(x3) 
        elif num == 2:
            t1.b = int(x1) 
            t2.b = int(x2) 
            t3.b = int(x3) 
        else:
            t1.c = int(x1) 
            t2.c = int(x2) 
            t3.c = int(x3) 
            triangles.append(t1)
            triangles.append(t2)
            triangles.append(t3)
            t1 = Triangle()
            t2 = Triangle()
            t3 = Triangle()
            num = 0

tris = []

getTriangles('input/triangles',tris)
num = 0
for tri in tris:
    if tri.check():
        num += 1
    print tri 
print num


