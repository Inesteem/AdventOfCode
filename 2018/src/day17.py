import sys 
import re

free = 0
wet = 1
water = 2
cley = 3 
spring = 42


hint_r = 1
hint_d = 0
hint_l = -1

origin = [0,500]
depth_point = [0,500]

def print_chunks(chunks, xi, yi):
    for y in range(max(xi[0],0), min(xi[1],len(chunks)-1)):
        for x in range(max(yi[0],0), min(yi[1],len(chunks[y])-1)):
            c = chunks[y][x]
            if c == free:
                sys.stdout.write('.')
            elif c == wet:
                sys.stdout.write('|')
            elif c == cley:
                sys.stdout.write('#')
            elif c == water:
                sys.stdout.write('~')
            else: 
                sys.stdout.write('+')
        print()


def go_left(chunks, old_pos):
    pos = [old_pos[0],old_pos[1]] 
    pos[1] = pos[1] - 1
    if pos[1] < 0:
        return None
    if chunks[pos[0]][pos[1]] >= water:
        return old_pos 
    chunks[pos[0]][pos[1]]= wet
    return pos 

def go_right(chunks, old_pos):
    pos = [old_pos[0],old_pos[1]] 
    pos[1] = pos[1] + 1
    if pos[1] >= len(chunks[0]):
        return None
    if chunks[pos[0]][pos[1]] >= water:
        return old_pos 
    chunks[pos[0]][pos[1]]= wet
    return pos 
 

def go_down(chunks, old_pos):
    pos = [old_pos[0],old_pos[1]] 
    pos[0] = pos[0] + 1
    if pos[0] >= len(chunks):
        return None
    if chunks[pos[0]][pos[1]] >= water:
        return old_pos 
    chunks[pos[0]][pos[1]]= wet
    return pos 
 

def render_water(chunks, pos, hint):

    #print()
    #print_chunks(chunks, [0,14], [500-6,500+6])
    #input()

    went_down = False 

    n_pos = [pos[0],pos[1]]
    while True:
        n_pos = go_down(chunks,n_pos) 
        if n_pos == None:
            return 
        if n_pos == pos:
            break;
        #print(str(pos) + " =? " + str(n_pos) + " -> " + str(n_pos == pos))
        pos = [n_pos[0],n_pos[1]]
        went_down = True
        hint = hint_d 

        #print()
        #print_chunks(chunks, [0,14], [500-6,500+6])
        #input()
    if pos[0] >  depth_point[0]:
        depth_point = [pos[0], pos[1]]

    if(went_down or hint != hint_r):
        left_pos = [pos[0],pos[1]]
        left_pos = go_left(chunks, left_pos)
        if left_pos != None and left_pos != pos:
            left_pos = render_water(chunks, left_pos, hint_l)
        if(hint == hint_l):
            return left_pos
    else:
        left_pos = None
    if(went_down or hint != hint_l):
        right_pos = [pos[0],pos[1]]
        right_pos = go_right(chunks, right_pos)
        if right_pos != None and right_pos != pos:
            right_pos = render_water(chunks, right_pos, hint_r)
        if(hint == hint_r):
            return right_pos
    else:
        right_pos = None


    if(left_pos == None or right_pos == None):
        return None 
    if(right_pos[0] == pos[0]):
        if left_pos[0] > right_pos[0]):
            return left_pos

    if(left_pos[0] == pos[0]):
        if(right_pos[0] > left_pos[0]):
            return right_pos
    if(right_pos == pos):
        return left_pos
    return right_pos 


#    if(left_pos == None):
#        if(right_pos == None):
#            return None 
#        return right_pos
#
#    if(right_pos == None or right_pos == pos):
#        return left_pos 
#
#    return right_pos 









if len(sys.argv) > 1:
    lines = [line.rstrip('\n') for line in open('../data/test.dat')]
else:
        lines = [line.rstrip('\n') for line in open('../data/day17.dat')]


h_veins = []
v_veins = []

x_max = 0
y_max = 0

for line in lines:
    vein=re.split(',|=|,|\.\.',line)
    if vein[0] == 'x':
        v_veins.append([int(vein[1]), int(vein[3]), int(vein[4])])
        y_max = max(y_max, int(vein[4]))        
        x_max = max(x_max, int(vein[1]))        
    else:
        h_veins.append([int(vein[1]), int(vein[3]), int(vein[4])])
        x_max = max(x_max, int(vein[4]))        
        y_max = max(y_max, int(vein[1]))        

chunks = [[free for i in range(x_max+1)] for j in range(y_max+1)]
chunks[origin[0]][origin[1]] = spring 

print(y_max)
print(x_max)

for v in h_veins:
#    print("h: "+str(v))
    for x in range(v[1],v[2]+1):
        chunks[v[0]][x] = cley 

for v in v_veins:
#    print("v: "+str(v))
    for y in range(v[1],v[2]+1):
        chunks[y][v[0]] = cley 


#start simulation
print_chunks(chunks, [0,14], [500-6,500+6])
i = 0
while True:

    w_pos = render_water(chunks,origin, hint_d)
    if w_pos == None:
        break
    chunks[w_pos[0]][w_pos[1]] = water
#    if(i%500 == 0):
#        print_chunks(chunks, [w_pos[0]-10,w_pos[0]+10], [w_pos[1]-15,w_pos[1]+15])
#        input()
#    i+= 1

print()
#print_chunks(chunks, [0,14], [500-6,500+6])
print_chunks(chunks, [0,100], [500-50,500+10])
print("finished, counting water")

water_tiles = 0
for y in range(len(chunks)):
    for x in range(len(chunks[y])):
        if chunks[y][x] == wet or chunks[y][x] == water:
            water_tiles += 1

print()
print("wet or water tiles: " + str(water_tiles))
    


