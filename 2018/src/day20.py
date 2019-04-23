import sys

directions = {'W' : [0,-2],
              'N' : [-2,0], 
              'E' : [0,2], 
              'S' : [2,0],
              'I' : [0,0]
              }

def next_pos(pos, d):
    return [pos[0] + d[0], pos[1] + d[1]]


end_rooms = {}



last = 0

class Node:

    def __init__(self, pos):
        self.childs = []
        self.pos = [pos[0],pos[1]]
        self.str = ""

    def walk(self,i,regex):
        p = (i*100)/len(regex)
        global last
        if p > last:
            print(str(p) + "%              " + str(i) +"/" + str(len(regex)))
            last = p
        while True:
            if regex[i] == '(':
                if len(self.childs) > 0:

                    for c in self.childs:
                        i = c.append(i+1, regex)
                    assert regex[i-1] == ')'
                    if int(p) % 8 == 0:
                        l=[]
                        self.merge_subtrees("", l)
                        self.childs=l
                    continue

                else:
                    i = self.add_childs(i+1,regex)
                    #l=[]
                    #self.merge_subtrees("", l)
                    #self.childs=l
                    continue 

            if regex[i] == ')' or regex[i] == '|' or regex[i] == '$':
                return i

            if len(self.childs) > 0:
                l = []
                self.get_all_leafs(l)
                tmpi = i
                for j in range(0, len(l)):
                    i = l[j].walk(tmpi, regex)
                
                return i
            else:
                self.pos = next_pos(self.pos, directions[regex[i]])
                self.str += regex[i]
            i += 1

    def merge_subtrees(self, path, end_nodes):
        if len(self.childs) == 0:
            self.str = path+self.str 
            end_nodes.append(self)
            return 

        for c in self.childs:
            c.merge_subtrees(path+self.str, end_nodes)



    def add_childs(self, i, regex):
        self.childs.append(Node(self.pos))
        while True:
            i = self.childs[-1].walk(i,regex)

            if regex[i] == '$':
                return i
            if regex[i] == ')':
                return i + 1

            if regex[i] == '|':
                self.childs.append(Node(self.pos))
            
            i += 1

    def get_all_leafs(self,l):
        if len(self.childs) == 0:
            l.append(self)
            return
        for c in self.childs:
            c.get_all_leafs(l)


    def append(self,i,regex):
        if len(self.childs) > 0:
            tmpi = i
            for c in self.childs:
                i = c.append(tmpi,regex)
            return i

        self.childs.append(Node(self.pos))
        while True:
            i = self.childs[-1].walk(i,regex)

            if regex[i] == '$':
                return i
            if regex[i] == ')':
                return i + 1 

            if regex[i] == '|':
                self.childs.append(Node(self.pos))
            
            i += 1


        #if len(self.childs) == 0:
        #    return self.add_childs( i, regex)
        #reti = i
        #for c in self.childs:
        #    reti = c.append(i,regex)
        #return reti 
        return i
            
    def count_doors(self, d):
        d = d + len(self.str)
        if len(self.childs) == 0:
            key = (self.pos[0],self.pos[1])
            if key in end_rooms:
                if end_rooms[key] > d:
                    end_rooms[key] = d
            else:
                end_rooms[key] = d


        else:
            for c in self.childs:
                c.count_doors(d)

    def print_paths(self, path):
        if  len(self.childs) == 0:
            print(path + self.str)
        
        for c in self.childs:
            c.print_paths(path + self.str + "-")

    def print(self, indent):
        print(indent + "doors : " + str(self.str) + "  (" + str(len(self.str)) + ")")
        print(indent + "childs: " + str(len(self.childs)))
        print(indent + "pos   : " + str(self.pos))
        print()
        for c in self.childs:
            c.print(indent + '-')


    

#regex = list("ENWWW(NEEE|SSE(EE|N))$")
#regex = list("ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$")#23
#regex= list("WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")#31
lines = [line.rstrip('\n') for line in open('../data/day20.dat')]# it is not 4190
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]
regex = list(lines[0])
#regex = list("(E|SSEENNW)S$")
#regex = list("(W|E)(S(E(NN|W)I|SEN)SS|I)N$")
root = Node([0,0])

root.walk(0,regex)
print("walk finished!")

#root.print('-')
for i in regex:
    sys.stdout.write(i)
print()
root.print_paths("")

root.count_doors(0)

end_rooms = sorted(end_rooms.items(), key=lambda kv: kv[1])
c = 0
print(end_rooms[-1])
for e in end_rooms:
    if e[1] >= 1000:
        c += 1
