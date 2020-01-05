import sys
import re 
from operator import attrgetter
import string
from time import sleep

#lines = [line.rstrip('\n') for line in open('../data/test.dat')]
lines = [line.rstrip('\n') for line in open('../data/day10.dat')]



def extr_pos_x(stars, func):
    return func(stars,key=attrgetter('pos.x')).pos.x

def extr_pos_y(stars, func):
    return func(stars,key=attrgetter('pos.y')).pos.y

def add_pair(p1, p2):
    return Pair(p1.x + p2.x, p2.y + p1.y)

class Pair:

    def __init__(self, x, y):
        self.x = x
        self.y = y 


class Star:

    def __init__(self, pos_x, pos_y,vel_x, vel_y):
        self.pos = Pair(int(pos_x), int(pos_y))
        self.vel = Pair(int(vel_x), int(vel_y))  

    def move(self):
        self.pos = add_pair(self.pos, self.vel)

    def __str__(self):
        return "(" + str(self.pos.x) +"," + str(self.pos.y) + ")  <" + str(self.vel.x) + "," + str(self.vel.y) + ">"

stars = []


for line in lines:
    elements = re.split('position=<|> velocity=<|,|>',line)
    pos_x = elements[1]
    pos_y = elements[2]
    vel_x = elements[3]
    vel_y = elements[4]
    stars.append(Star(pos_x,pos_y, vel_x, vel_y))
#    print(str(pos_x) + "  " + str(pos_y) + "  " + str(vel_x) + "  " + str(vel_y))
#    print(str(stars[len(stars)-1]))

sec = 0
while True:
    stars = sorted(stars, key=lambda z: (z.pos.y, z.pos.x))
    y = extr_pos_y(stars,min)
    x_min = extr_pos_x(stars,min)
    x_max = extr_pos_x(stars,max)
    x = x_min 
    if x_max - x_min < 100:
        print("--- " + str(sec) + " ---")
    for star in stars:
        if x_max - x_min < 100:
            while star.pos.y > y:
                while x <= x_max:
                    sys.stdout.write(".")
                    x += 1
                y += 1
                x = x_min
                print("")

            while star.pos.x > x:
                sys.stdout.write(".")
                x += 1

            if star.pos.x == x:
                sys.stdout.write(str("#"))
                x += 1
        star.move()
    sec += 1
    if x_max - x_min < 100:
        sleep(1)
        print("")
        print("")

