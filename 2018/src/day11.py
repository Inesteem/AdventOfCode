import sys 

if len(sys.argv) < 2:
    print("please submit a seral number!")
    exit(0)

size = 300
serial = int(sys.argv[1])



def power_level(x, y, serial):
    rid = x + 10
    pl = rid * y 
    pl += serial 
    pl *= rid
    pl = int(pl / 100) % 10
    return pl - 5

def power_of_region(x_len, y_len, grid, s):
    assert(x_len + s <= size)
    assert(y_len + s <= size)

    pl = 0

    for y in range(y_len, y_len + s):
        for x in range(x_len, x_len + s):
            #sys.stdout.write(str(grid[y][x]) + " ")
            pl += grid[y][x]
        #print("")
    return pl 



grid = size * [size * []] 

#assert(power_level(3,5,8) == 4)
#assert(power_level(122,79,57) == -5)
#assert(power_level(217,196,39) == 0)
#assert(power_level(101,153,71) == 4)

for y in range(0,size):
    for x in range(0,size):
        grid[y][x] = (power_level(x+1,y+1,serial))


max_val=0
max_x=0
max_y=0
for y in range(0,size-3):
    for x in range(0,size-3):
        power = power_of_region(x,y, grid,3)
        if power > max_val:
            max_val=power 
            max_x=x
            max_y=y


print(str(max_val) + ": (" + str(max_x + 1) + "," + str(max_y + 1) + ")")



