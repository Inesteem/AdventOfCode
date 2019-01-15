import re
import sys
import math
import datetime 


class GridPoint():

    def __init__(self, x, y, pid):
        self.x = x
        self.y = y
        self.pid = pid
        self.area = 0
        return None

    def hamming(self, x,y):
        return abs(self.y - y) + abs(self.x - x)

    def left_bounded_through(self, other):
        if other.y >= self.y:
            return False
        return abs(self.x - other.x) <= (self.y - other.y)

    def right_bounded_through(self, other):
        if other.y <= self.y:
            return False
        return abs(self.x - other.x) <= (other.y - self.y)

    def top_bounded_through(self, other):
        if other.x <= self.x:
            return False
        return abs(self.y - other.y) <= (other.x - self.x)

    def bottom_bounded_through(self, other):
        if other.x >= self.x:
            return False
        return abs(self.y - other.y) <= (self.x - other.x)

    def in_area(self, points, x, y):
        for p in points:
        #    print("test: " + str(self.pid) + " vs " + str(p.pid) + ": " + str(x) + " " + str(y))
        #    print(str(p.hamming(x,y)) + " vs " + str(self.hamming(x,y)))
            if ord(p.pid) == ord(self.pid):
                continue 
            if p.hamming(x,y) <= self.hamming(x,y):
                print("test: " + str(x) + " " + str(y) + " :  x")
                return False 
        #print(str(self.pid) + " has point in area : " + str(x) + str(y))
        print("test: " + str(x) + " " + str(y) + " :  o")
        return True 


lines = [line.rstrip('\n') for line in open('../data/day6.dat')]
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]

gridpoints = []

cnt=0
max_area = 0
max_x = 0
max_y = 0
for line in lines:
    x,y  = [ int(z) for z in line.split(', ')]
    if max_x < x:
        max_x = x
    if max_y < y:
        max_y = y
    #print(str(x) + " " + str(y))
    gridpoints.append(GridPoint(x + 1000,y + 1000, chr(ord('A')+cnt)))
    cnt += 1 

max_x += 1000
max_y += 1000

#for i in range(0,len(gridpoints)):
#    bounded=0
#    p1=gridpoints[i]
#    for j in range(0,len(gridpoints)):
#        if i == j:
#            continue
#        p2=gridpoints[j]
#        
#        if p1.left_bounded_through(p2): 
##            print("point " + str(i) + " is left bounded by point " + str(j))
#            bounded |= 1
#        
#        if p1.right_bounded_through(p2): 
##            print("point " + str(i) + " is tight bounded by point " + str(j))
#            bounded |= 2
#
#        if p1.top_bounded_through(p2): 
##            print("point " + str(i) + " is top bounded by point " + str(j))
#            bounded |= 4
#
#        if p1.bottom_bounded_through(p2): 
##            print("point " + str(i) + " is bottom bounded by point " + str(j))
#            bounded |= 8
#    if bounded == 15:
#        print("point " + str(p1.pid)  + " is bounded from all sides!")
#        #up
#        # .......
#        #  .....
#        #   ...
#        #    p
#        flag=True
#        steps = 0
#        while flag == True:
#            flag=False 
#            steps += 1
#            y = p1.y + steps 
#            for x in range(max(0,p1.x-steps),p1.x+steps+1):
#                if p1.in_area(gridpoints,x,y):
#                    flag = True 
#                    p1.area += 1
#            print("-----")
#        print("-----")
#
#        #down
#        #    p   
#        #   ... 
#        #  .....
#        # .......
#
#        flag=True
#        steps = 0
#        while flag == True:
#            flag=False 
#            steps += 1
#            y = p1.y - steps 
#            assert(y >= 0) # bottom bound, should not pass 0
#            for x in range(max(0,p1.x-steps),p1.x+steps+1):
#                if p1.in_area(gridpoints,x,y):
#                    flag = True 
#                    p1.area += 1
#            print("-----")
#        print("-----")
#
#        #right
#        #       -
#        #     - -
#        # p - - -
#        #     - -
#        #       -
#        flag=True
#        steps = 0
#        while flag == True:
#            flag=False 
#            x = p1.x + steps + 1 
#            for y in range(max(0,p1.y-steps),p1.y+steps+1):
#                if p1.in_area(gridpoints,x,y):
#                    flag = True 
#                    p1.area += 1
#            steps += 1
#            print("-----")
#        print("-----")
#
#        #left
#        # -      
#        # - -    
#        # - - - p
#        # - -    
#        # -      
#        flag=True
#        steps = 0
#        while flag == True:
#            flag=False 
#            x = p1.x - steps - 1
#            assert(x >= 0) # left bound, should not pass 0
#            for y in range(max(0,p1.y-steps),p1.y+steps+1):
#                if p1.in_area(gridpoints,x,y):
#                    flag = True 
#                    p1.area += 1
#            steps += 1
#            print("-----")
#        print("-----")
#                      
#        if p1.area > max_area:
#            max_area = p1.area 
#
#        print("area : " + str(p1.area))
#print("max area: " + str(max_area + 1))


#part 2

print("max_x: " + str(max_x))
print("max_y: " + str(max_y))

points_in_area = 0
for y in range(0,max_y + 1000):
    for x in range(0,max_x + 1000):
        cnt = 0
        for p in gridpoints:
            cnt += p.hamming(x,y)
        if cnt < 10000:
            points_in_area += 1

print("points in area : " + str(points_in_area))
