import math
def reverse(string):
    string = string[::-1]
    return string


class Grid:
    def __init__(self):
        self.nodes = []
        self.width = 0
        self.height = 0
        self.current = [0,0]
        self.dir = 0
    def __repr__(self):
        return str(self)
    def __str__(self):
        sstr = str(self.current) + " [" + str(self.width) + "x" + str(self.height) + "]:\n" #+str(self.nodes)
        tmp = self.nodes[self.current[1]][self.current[0]]
        if(self.infected()):
            self.nodes[self.current[1]][self.current[0]] = '!'
        else:
            self.nodes[self.current[1]][self.current[0]] = 'o'
        for line in self.nodes:
            sstr += ''.join(line) + "\n" 
       
        self.nodes[self.current[1]][self.current[0]] =tmp 
        return sstr

    def infected(self):
        x = self.current[0]
        y = self.current[1]
        return (self.nodes[y][x] == '#')

    def turnRight(self):
        self.dir = (self.dir + 1) % 4

    def turnLeft(self):
        self.dir = (self.dir - 1) % 4

    def goForward(self):
        x = self.current[0]
        y = self.current[1]
        if self.dir == 0: #UP
            if y == 0:
                self.nodes.insert(0,[])
                for i in xrange(self.width):
                    self.nodes[0].append(".")
                self.height += 1
            else: 
                self.current[1] -= 1
        elif self.dir == 2: #DOWN
            if y == self.height - 1:
                self.nodes.append([])
                for i in xrange(self.width):
                    self.nodes[self.height].append(".")
                self.height += 1
            self.current[1] += 1
        elif self.dir == 3: #LEFT
            if  x == 0:
                for i in xrange(self.height):
                    self.nodes[i].insert(0,".")
                self.width += 1
            else: 
                self.current[0] -= 1
        else:  #RIGHT
            if x == self.width - 1:
                for i in xrange(self.height):
                    self.nodes[i].append(".")
                self.width += 1
            self.current[0] += 1


    def swapNode(self):
        x = self.current[0]
        y = self.current[1]
        if self.nodes[y][x] == '.':
            self.nodes[y][x] = '#'
        else:
            self.nodes[y][x] = '.'



def getGrid (grid):
    with open("data/grid", "r") as f:
        data = f.readlines()

    for line in data:
        line = line.rstrip('\r\n')
        l = []
        for c in line:
            l.append(c)
        grid.nodes.append(l)
        grid.height += 1

    grid.width = len(grid.nodes[0])
    grid.current = [int(grid.width/2),int(grid.height/2)]
    

grid = Grid()
getGrid(grid)

infectedNodes = 0
bursts = 0
while bursts != 10000:
    if grid.infected():
        grid.turnRight()
       # infectedNodes -= 1
    else:
        grid.turnLeft()
        infectedNodes += 1

    grid.swapNode()
    grid.goForward()
    bursts += 1
print grid
print "infected nodes : " + str(infectedNodes)
print "bursts: " + str(bursts)
