import sys 
import re

free = 0
wet = 1
spring = 2
water = 3
cley = 4


hint_r = 1
hint_d = 0
hint_l = -1

origin = [0,500]
#depth_point = [0,500]
#origin = [0,4]
pruned = {}
#38369 not the right answer
#37277 to low
#correct answer: 38364

def count_water_tiles(chunks, x_min, x_max, y_min, y_max):
    water_tiles = 0
    for y in range(y_min, y_max + 1):
        for x in range(x_min-1, x_max + 1):
            #if chunks[y][x] != free and chunks[y][x]  != cley:
            if chunks[y][x] == water:
                water_tiles += 1
    return water_tiles


def omit_spring(chunks, pos, second_chance):
    if chunks[pos[0]][pos[1]] != spring:
    #    second_chance[0] = True
        return False 
    if second_chance[0] == False: 
        return False
    return True 


def print_chunks(chunks, xi, yi):
    for y in range(max(xi[0],0), min(xi[1],len(chunks))):
        for x in range(max(yi[0],0), min(yi[1],len(chunks[y]))):
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
                if (y,x) in pruned:
                    sys.stdout.write('*')
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
    chunks[pos[0]][pos[1]] = wet
    return pos 

def go_right(chunks, old_pos):
    pos = [old_pos[0],old_pos[1]] 
    pos[1] = pos[1] + 1
    if pos[1] >= len(chunks[0]):
        return None
    if chunks[pos[0]][pos[1]] >= water:
        return old_pos 
    chunks[pos[0]][pos[1]] = wet
    return pos 
 

def go_down(chunks, old_pos):
    pos = [old_pos[0],old_pos[1]] 
    pos[0] = pos[0] + 1
    if pos[0] >= len(chunks):
        return None
    if chunks[pos[0]][pos[1]] >= water:
        return old_pos 
    if chunks[pos[0]][pos[1]] != spring:
        chunks[pos[0]][pos[1]] = wet
    return pos 
 

def render_water_advanced(chunks, falls, second_chance):
    pos = falls[0]
    l_spring = (falls[0][0], falls[0][1])

##    chunks[pos[0]][pos[1]] = spring 

    n_pos = [pos[0],pos[1]]
    while True:
        n_pos = go_down(chunks,n_pos) 
        if n_pos == None:
            pruned[l_spring] = True 
            return 
        if n_pos == pos:
            break 
        #print(str(pos) + " =? " + str(n_pos) + " -> " + str(n_pos == pos))
        pos = [n_pos[0],n_pos[1]]


    while True:
        
        outer_left = []
        l_pos = [pos[0],pos[1]]
        n_pos = [pos[0],pos[1]]

        while True:
            n_pos = go_left(chunks,n_pos) 
            if n_pos == None:
                outer_left = None
                break 
            if n_pos == l_pos:
                outer_left = [n_pos[0],n_pos[1]] 
                break 
            tmp = [n_pos[0],n_pos[1]]
            tmp = go_down(chunks,tmp)
            if tmp != n_pos:
                if (tmp[0],tmp[1]) in pruned:
                    outer_left = None
                    break  
                if not omit_spring(chunks, tmp, second_chance):
                    chunks[tmp[0]][tmp[1]] = spring
                    falls.append(tmp)
                break;
            l_pos = [n_pos[0],n_pos[1]]

        outer_right = []
        l_pos = [pos[0],pos[1]]
        n_pos = [pos[0],pos[1]]

        while True:
            n_pos = go_right(chunks,n_pos) 
            if n_pos == None:
                outer_right = None
                break 
            if n_pos == l_pos:
                outer_right = [n_pos[0],n_pos[1]] 
                break 
            tmp = [n_pos[0],n_pos[1]]
            tmp = go_down(chunks,tmp)
            if tmp != n_pos:
                if (tmp[0],tmp[1]) in pruned:
                    outer_right = None
                    break  
                if not omit_spring(chunks, tmp, second_chance):
                    chunks[tmp[0]][tmp[1]] = spring
                    falls.append(tmp)
                break;
            l_pos = [n_pos[0],n_pos[1]]

        if outer_left == None :
            if outer_right == None or len(outer_right) != 0:
                pruned[l_spring] = True 
            return 
 
        if outer_right == None :
            if  len(outer_left) != 0:
                pruned[l_spring] = True 
            return            

        if len(outer_left) == 0 or  len(outer_right) == 0:
            return 
        x = outer_left[1]
        while x <= outer_right[1]:
            chunks[outer_left[0]][x] = water
            x = x + 1


        pos[0] = pos[0] - 1
        if chunks[pos[0]][pos[1]] == cley:
            x = outer_left[1]
            while x <= outer_right[1]:
                if chunks[pos[0]][x] == wet:
                    falls.append([pos[0],x])
                x = x + 1
            return
        chunks[pos[0]][pos[1]] = wet




    






if len(sys.argv) > 2:
    lines = [line.rstrip('\n') for line in open('../data/day17_t.py')]
    origin = [0,4]
elif len(sys.argv) > 1:
    lines = [line.rstrip('\n') for line in open('../data/test.dat')]
else:
    lines = [line.rstrip('\n') for line in open('../data/day17.dat')]


h_veins = []
v_veins = []

x_max = 0
y_max = 0
x_min = 100000
y_min = 100000


for line in lines:
    vein=re.split(',|=|,|\.\.',line)
    if vein[0] == 'x':
        v_veins.append([int(vein[1]), int(vein[3]), int(vein[4])])
        y_max = max(y_max, int(vein[4]))        
        y_min = min(y_min, int(vein[3]))        
        x_max = max(x_max, int(vein[1]))        
        x_min = min(x_min, int(vein[1]))        
    else:
        h_veins.append([int(vein[1]), int(vein[3]), int(vein[4])])
        x_max = max(x_max, int(vein[4]))        
        x_min = min(x_min, int(vein[3]))        
        y_max = max(y_max, int(vein[1]))        
        y_min = min(y_min, int(vein[1]))        

chunks = [[free for i in range(x_max+1)] for j in range(y_max+1)]
chunks[origin[0]][origin[1]] = spring 

falls = [[origin[0],origin[1]]]


for v in h_veins:
#    print("h: "+str(v))
    for x in range(v[1],v[2]+1):
        chunks[v[0]][x] = cley 

for v in v_veins:
#    print("v: "+str(v))
    for y in range(v[1],v[2]+1):
        chunks[y][v[0]] = cley 


#start simulation
print_chunks(chunks, [0,17], [0,17])
i = 0
second_chance=[True]
wpos=[0,0]
while len(falls):
    
    w_pos = falls[0]
#    if i > 1000 and i%100000 == 0:
#        print_chunks(chunks, [w_pos[0]-15,w_pos[0]+15], [w_pos[1]-45,w_pos[1]+45])
#        print()
    render_water_advanced(chunks, falls, second_chance)
    if i > 1000 and i%100000 == 0:
        #print_chunks(chunks, [0,y_max+10], [0,x_max+10])
        #exit(0)
    #if True:
#        print("pos : " + str(w_pos))
#        print_chunks(chunks, [w_pos[0]-15,w_pos[0]+15], [w_pos[1]-45,w_pos[1]+45])
#        print(str(i) + " -  pruned: " + str(len(pruned)) + " " + str(pruned))
        water_tiles = count_water_tiles(chunks, x_min, x_max, y_min, y_max)
        print("tiles: " + str(water_tiles) + "  springs: " + str(len(falls)))
        #input()
    del falls[0]
        
#    print()
#    print(falls)
#    print()
#    print_chunks(chunks, [0,18], [0,18])
#    input()
#    if(len(falls) == 0 and second_chance[0] == True):
#        
#        second_chance[0]=False 
#        falls = [[origin[0],origin[1]]]
    i += 1

print()
#print_chunks(chunks, [0,14], [500-6,500+6])
print("x: " + str(x_min) + " -> " + str(x_max))
print("y: " + str(y_min) + " -> " + str(y_max))
print_chunks(chunks, [w_pos[0]-15,w_pos[0]+15], [w_pos[1]-45,w_pos[1]+45])
print("finished after " + str(i) + " iterations, counting water")

water_tiles = count_water_tiles(chunks, x_min, x_max, y_min, y_max)

print()
print("wet or water tiles: " + str(water_tiles))
    

#def render_water(chunks, pos, hint):
#
#    #print()
#    #print_chunks(chunks, [0,14], [500-6,500+6])
#    #input()
#
#    went_down = False 
#
#    n_pos = [pos[0],pos[1]]
#    while True:
#        n_pos = go_down(chunks,n_pos) 
#        if n_pos == None:
#            return 
#        if n_pos == pos:
#            break;
#        #print(str(pos) + " =? " + str(n_pos) + " -> " + str(n_pos == pos))
#        pos = [n_pos[0],n_pos[1]]
#        went_down = True
#        hint = hint_d 
#
#        #print()
#        #print_chunks(chunks, [0,14], [500-6,500+6])
#        #input()
#    if pos[0] >  depth_point[0]:
#        depth_point = [pos[0], pos[1]]
#
#    if(went_down or hint != hint_r):
#        left_pos = [pos[0],pos[1]]
#        left_pos = go_left(chunks, left_pos)
#        if left_pos != None and left_pos != pos:
#            left_pos = render_water(chunks, left_pos, hint_l)
#        if(hint == hint_l):
#            return left_pos
#    else:
#        left_pos = None
#    if(went_down or hint != hint_l):
#        right_pos = [pos[0],pos[1]]
#        right_pos = go_right(chunks, right_pos)
#        if right_pos != None and right_pos != pos:
#            right_pos = render_water(chunks, right_pos, hint_r)
#        if(hint == hint_r):
#            return right_pos
#    else:
#        right_pos = None
#
#
#    if(left_pos == None or right_pos == None):
#        return None 
#    if(right_pos[0] == pos[0]):
#        if( left_pos[0] > right_pos[0]):
#            return left_pos
#
#    if(left_pos[0] == pos[0]):
#        if(right_pos[0] > left_pos[0]):
#            return right_pos
#    if(right_pos == pos):
#        return left_pos
#    return right_pos 
#
#
##    if(left_pos == None):
##        if(right_pos == None):
##            return None 
##        return right_pos
##
##    if(right_pos == None or right_pos == pos):
##        return left_pos 
##
##    return right_pos 
#
#
#
