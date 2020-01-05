import sys 
elf_power = 3
if len(sys.argv) > 1:
    elf_power = int(sys.argv[1])
    
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]
lines = [line.rstrip('\n') for line in open('../data/day15.dat')]

def get_up(pos):
    return [pos[0]-1, pos[1]]

def get_down(pos):
    return [pos[0]+1, pos[1]]

def get_left(pos):
    return [pos[0], pos[1]-1]

def get_right(pos):
    return [pos[0], pos[1]+1]



class Unit:

    def __init__(self, u_id, u_type, u_pos, attack, health):
        self.u_id = u_id 
        self.u_type = u_type 
        self.u_pos =  u_pos
        self.attack = attack 
        self.health = health

    def is_near(self, other):
        x_diff = self.u_pos[0] - other.u_pos[0]
        y_diff = self.u_pos[1] - other.u_pos[1]
        if x_diff < -1 or x_diff > 1:
            return False 
        if y_diff < -1 or y_diff > 1:
            return False
        if x_diff != 0 and y_diff != 0:
            return False
        return True 

    def mh_acquire(self, field, occupied, pos, count):
        if field.tiles[pos[0]][pos[1]].unit or occupied[pos[0]][pos[1]] != -1:
            return False
        occupied[pos[0]][pos[1]] = count 
        field.tiles[pos[0]][pos[1]].path.append([count, pos])
        return True 
        

    def manhatten(self, field, occupied, pos_list):

        while len(pos_list):
            next_pos_list = []
            for pos in pos_list:

                count = occupied[pos[0]][pos[1]] + 1 
                up =    get_up(pos)
                down =  get_down(pos) 
                left =  get_left(pos)
                right = get_right(pos)

                if(self.mh_acquire(field, occupied, up, count)):
                    next_pos_list.append(up) 
                if(self.mh_acquire(field, occupied, down, count)):
                    next_pos_list.append(down) 
                if(self.mh_acquire(field, occupied, left, count)):
                    next_pos_list.append(left) 
                if(self.mh_acquire(field, occupied, right, count)):
                    next_pos_list.append(right) 

            pos_list = next_pos_list 

    def set_path(self, field):
        occupied = [[ -1 for x in range(0, field.width) ] for y in range(0,field.height)]
        occupied[self.u_pos[0]][self.u_pos[1]] = 0
        self.manhatten(field, occupied, [self.u_pos])
        #print(self.u_type)
        #for tiles in occupied:
        #    for o in tiles:
        #        if o > -1 and o < 10:
        #            sys.stdout.write(" " )
        #        sys.stdout.write(str(o) + " " )

        #    print()


    def move(self, field, new_pos):
        field.tiles[self.u_pos[0]][ self.u_pos[1]].unit = None
        field.tiles[new_pos[0]][ new_pos[1]].unit = self
        self.u_pos = new_pos 

    def fight(self, field, units, enemy):
        enemy.health -= self.attack
        if enemy.health <= 0:
            if enemy.u_type == 'E':
                exit(-1)
            field.tiles[enemy.u_pos[0]][enemy.u_pos[1]].unit = None
            enemy.u_pos = [field.height,field.width]
            enemy.u_type = 'D' 

wall = Unit(-1, '#', [-1,-1], -1, -1)


class Tile:
    def __init__(self, unit):
        self.unit = unit
        self.path = []

wall_tile = Tile(wall)

class Field:


    def __init__(self, height, width):
        self.width = width
        self.height = height
        self.tiles = [[ wall_tile for x in range(0, width) ] for y in range(0,height)]

    def order_paths(self, pos):
        path = self.tiles[pos[0]][pos[1]].path 
        # assumption: sorted keeps initial order of identical elements!
        path = sorted(path, key = lambda x : x[0])
        if len(path):
            return path[0]
        return [self.width + self.height, None]


    def at(self, pos):
        return self.tiles[pos[0]][pos[1]]

def exit_fight(rounds, units):

    print("there were "+ str(rounds) +" rounds of fighting")
    credits = 0
    for u in units:
        if u.u_type == 'D':
            break
        print(str(u.u_type) + " -> hp left ("+ str(u.u_pos) +"):" + str(u.health))
        credits += u.health
    print("total score: " + str(credits) + " * " + str(rounds) + " = " + str(credits*rounds))
    exit(0)

#units stored ordered in terms of 'reading' dir top down, left to right
units = []
field = Field(len(lines), len(lines[0])) 
uid = 0
for y in range(0,len(lines)):
    for x in range(0,len(lines[y])):
        symbol = lines[y][x]
        if symbol == '#':
            continue 
        elif symbol == '.':
            field.tiles[y][x] = Tile(None) 
        else:
            if symbol == 'E':
                unit = Unit(uid, symbol,[y,x], elf_power, 200)
            else:
                unit = Unit(uid, symbol,[y,x], 3, 200)
            uid +=1
            units.append(unit)
            field.tiles[y][x] = Tile(unit)
   
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########

def append_enemy(field, pos, enemy_list, e_type):
    unit = field.at(pos).unit 
    if(unit != None and unit.u_type == e_type):
        enemy_list.append(unit)

fight_round = -1
action = True
while action:
    action = False 
    fight_round += 1
  #  print("After " + str(fight_round) + " round(s)")
  #  for tiles in field.tiles:
  #      tmp = []
  #      for tile in tiles:
  #          if tile.unit == None:
  #              sys.stdout.write('.')
  #          else:
  #              sys.stdout.write(tile.unit.u_type)
  #          if tile.unit and tile != wall_tile:
  #              tmp.append(tile.unit)
  #      for u in tmp:
  #          sys.stdout.write(" " + str(u.u_type) + "(" + str(u.health) + ")")
  #          
  #      print()
  #              
  #  #input()

    for u in units:
        # define enemy type
        e_type = 'G'
        if u.u_type == 'G':
            e_type = 'E'
        elif u.u_type == 'D':
            continue

        # collect all possible enemy units
        enemies_far = []
        enemies_near = []
        for u2 in units:
            if u2.u_type == e_type:
                if u2.is_near(u):
                    enemies_near.append(u2)
                else:
                    enemies_far.append(u2)
        #move if necessary
        if len(enemies_near) == 0:
            for e in enemies_far:
                e.set_path(field)

            up =    get_up(u.u_pos)
            down =  get_down(u.u_pos) 
            left =  get_left(u.u_pos)
            right = get_right(u.u_pos)

            mins = [None,None,None,None]
            mins[0] = field.order_paths(up)
            mins[1] = field.order_paths(left)
            mins[2] = field.order_paths(right)
            mins[3] = field.order_paths(down)
            #print(mins)
            mins = sorted(mins, key = lambda x : x[0])
            #print(mins)

            #reset paths
            for tiles in field.tiles:
                for tile in tiles:
                    tile.path = []

            if mins[0][1] != None:

                u.move(field, mins[0][1])
                action = True 

                up =    get_up(u.u_pos)
                down =  get_down(u.u_pos) 
                left =  get_left(u.u_pos)
                right = get_right(u.u_pos)

                append_enemy(field, up,     enemies_near, e_type)
                append_enemy(field, left,   enemies_near, e_type)
                append_enemy(field, right,  enemies_near, e_type)
                append_enemy(field, down,   enemies_near, e_type)

        if len(enemies_near) != 0:
            #print("enemies near: figth")
            enemies_near = sorted(enemies_near, key = lambda x : x.health)
            u.fight(field, units, enemies_near[0])
            action = True 
        

    # sort units to keep reading dir 
    units= sorted(sorted(units, key = lambda x : x.u_pos[1]), key = lambda x : x.u_pos[0])

exit_fight(fight_round-1,units)
