import re
import sys
import math
import datetime 

class Node():

    def __init__(self):
        self.childs = []
        self.data= []
 
    def get_value(self, nodes):
        meta_data = 0
        if len(self.childs) == 0:
            for d in self.data:
                meta_data += d
        else: 
            for d in self.data:
                if d == 0 or d > len(self.childs):
                    continue
                meta_data += nodes[self.childs[(d - 1)]].get_value(nodes)
        return meta_data 

class Pair():

    def __init__(self, idx, n_id):
        self.idx = idx
        self.n_id = n_id 



def build_tree(nodes, n_id, n_info, idx):
    this = n_id 
    nodes.append(Node())
    num_childs = n_info[idx]
    num_data = n_info[idx+1]
    idx += 2

    for c in range(0, num_childs):
        n_id += 1
        nodes[this].childs.append(n_id)
        p = build_tree(nodes, n_id, n_info, idx)
        idx = p.idx 
        n_id = p.n_id 
        
    for d in range(0, num_data):
        nodes[this].data.append(n_info[idx])
        idx += 1
    
    return Pair(idx, n_id)


#n_info = [int(z) for z in [line.rstrip('\n') for line in open('../data/test.dat')][0].split(' ')]
n_info = [int(z) for z in [line.rstrip('\n') for line in open('../data/day8.dat')][0].split(' ')]


nodes = []

build_tree(nodes, 0, n_info, 0)

meta_data = 0

for n in nodes:
    for d in n.data:
        meta_data += d
    print(n.childs)
    print(n.data)
    print("-----")

print("meta data : " + str(meta_data))


# part 2

meta_data = nodes[0].get_value(nodes)  
print("meta data : " + str(meta_data))
