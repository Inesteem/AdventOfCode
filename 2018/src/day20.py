import sys

go_E = 2
go_W = 0
go_N = 1
go_S = 3


directions = {'W' : [0,-2],
              'N' : [-2,0], 
              'E' : [0,2], 
              'S' : [2,0]}

def next_pos(pos, d):
    return [pos[0] + d[0], pos[1] + d[1]]


end_rooms = []





class Node:

    def __init__(self, pos):
        self.childs = []
        self.pos = [pos[0],pos[1]]
        self.str = ""
        
    def walk(self,i,regex):
        while True:
            if regex[i] == '(':
                i = self.add_childs(i+1,regex)
                continue 

            if regex[i] == ')' or regex[i] == '|' or regex[i] == '$':
                return i

            self.pos = next_pos(self.pos, directions[regex[i]])
            self.str += regex[i]
            i += 1

    def add_childs(self, i, regex):
        self.childs.append(Node(self.pos))
        while True:
            i = self.childs[-1].walk(i,regex)

            if regex[i] == ')':
                return i + 1

            if regex[i] == '|':
                self.childs.append(Node(self.pos))
            
            i += 1
            
    def count_doors(self, d):
        d = d + len(self.str)
        if len(self.childs) == 0:
            end_rooms.append([d, self.pos])
        else:
            for c in self.childs:
                c.count_doors(d)

    def print(self, indent):
        print(indent + "doors : " + str(self.str) + "  (" + str(len(self.str)) + ")")
        print(indent + "childs: " + str(len(self.childs)))
        print(indent + "pos   : " + str(self.pos))
        print()
        for c in self.childs:
            c.print(indent + ' ')


    

#regex = list("ENWWW(NEEE|SSE(EE|N))$")
#regex = list("ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$")#23
#regex= list("WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")#31
lines = [line.rstrip('\n') for line in open('../data/day20.dat')]# it is not 4190
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]
regex = list(lines[0])

root = Node([0,0])

root.walk(0,regex)
#root.print("")

root.count_doors(0)
end_rooms = sorted(end_rooms)

while True:
    r = end_rooms[-1]
    for e in end_rooms:
        if e == r:
            print(r)
            exit(0)

        if e[1] == r[1]:
            del end_rooms[-1] 
            break



