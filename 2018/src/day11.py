import sys 

if len(sys.argv) < 2:
    print("please submit a seral number!")
    exit(0)

size = 300
serial = int(sys.argv[1])
#9435

class Grid:

    def __init__(self, size, square_size, grid):
        self.size = size
        self.square_size = square_size 
        self.grid = grid 

    def matches(self,grid_size):
        return (grid_size % self.square_size) == 0



def power_level(x, y, serial):
    rid = x + 10
    pl = rid * y 
    pl += serial 
    pl *= rid
    pl = int(pl / 100) % 10
    return pl - 5

def power_of_region(x_pos, y_pos, grid, power_grid, s):
    #assert(x_pos + s <= size)
    #assert(y_pos + s <= size)

    pl = power_grid[y_pos][x_pos]
    for side_len in range(0,s):
        pl += grid[y_pos + s - 1][side_len+x_pos]
        pl += grid[side_len+y_pos][x_pos + s - 1]

    pl -= grid[y_pos + s - 1][x_pos + s - 1] 
    return pl


def power_of_region_advanced(x_pos, y_pos, grid, s, step):
    #assert(x_pos + s <= size)
    #assert(y_pos + s <= size)
    pl = 0
    for y in range(0,s, step):
        for x in range(0,s, step):
            pl += grid[y+y_pos][x+x_pos]

    return pl 




# method 2
grid = [[0 for i in range(size)] for j in range(size)]


max_val=0
max_x=0
max_y=0
max_size=1
for y in range(0,size):
    for x in range(0,size):
        power = (power_level(x+1,y+1,serial))
        grid[y][x] = power 
        if power > max_val:
            max_val=power
            max_x=x
            max_y=y

grids = [Grid(size, 1, grid)]
for square in range(2,301):
#for square in range(2,20):
    new_grid = [[0 for i in range(size)] for j in range(size)]
    for gi in range(len(grids)-1,-1,-1):
        grid_smaller = grids[gi]
        if grid_smaller.square_size < (square/10):
            print(str(square) + " skip ")
            for y in range(0,size-square):
                for x in range(0,size-square):
                    new_grid[y][x] = power_of_region(x,y, grid, grids[-1].grid, square)
        
                    if new_grid[y][x] > max_val:
                        max_val=new_grid[y][x]  
                        max_x=x
                        max_y=y
                        max_size=square 
            break  
        elif grid_smaller.matches(square):
                          
            print(str(square) + " ---> " + str(grid_smaller.square_size))
            for y in range(0,size-square):
                for x in range(0,size-square):
                    power = power_of_region_advanced(x,y, grid_smaller.grid, square,grid_smaller.square_size)
                    new_grid[y][x] = power
                    if power > max_val:
                        max_val=power  
                        max_x=x
                        max_y=y
                        max_size=square 
            break
    grids.append(Grid(size, square, new_grid))

# method 1
#grid = [[0 for i in range(size)] for j in range(size)]
#power_grid = [[0 for i in range(size)] for j in range(size)]
#
#
#max_val=0
#max_x=0
#max_y=0
#max_size=1
#for y in range(0,size):
#    for x in range(0,size):
#        power = (power_level(x+1,y+1,serial))
#        grid[y][x] = power 
#        power_grid[y][x] = power 
#        if power > max_val:
#            max_val=power
#            max_x=x
#            max_y=y
#
#
#
#for square in range(2,301):
##for square in range(2,18):
#    print(str(square))
#    for y in range(0,size-square):
#        for x in range(0,size-square):
#            power_grid[y][x] = power_of_region(x,y, grid, power_grid, square)
#
#            if power_grid[y][x] > max_val:
#                max_val=power_grid[y][x]  
#                max_x=x
#                max_y=y
#                max_size=square 
#
#
print(str(max_val) + ": (" + str(max_x + 1) + "," + str(max_y + 1) + "," + str(max_size) + ")")


# 76: (236,270,11)

