import math
def reverse(string):
    string = string[::-1]
    return string

class Node:
    def __init__(self, value = [0,0], next = None, prev = None):
        self.value = value
        self.next = next
        self.prev = prev

    def __str__(self):
        return 'Node ['+str(self.value)+']'

    def is_way(self):
        return self.value[0] == self.value[1]

    def get_v_sum(self):
        return self.value[0] + self.value[1]

class LinkedList:
    def __init__(self):
        self.first = None
        self.last = None
        self.value = 0
        self.num = 0

    def insert(self, x):
        self.value += x[0] + x[1]
        self.num += 1
        if self.first == None:
            self.first = Node(x, None)
            self.last = self.first
        elif self.last == self.first:
            self.last = Node(x, None,  self.first)
            self.first.next = self.last
        else:
            current = Node(x, None,  self.last)
            self.last.next = current
            self.last = current
        

    def remove_back(self):
        if self.last == None:
            return
        elif self.last == self.first:
            self.clear()
        else: 
            self.value -= self.last.get_v_sum()
            self.last = self.last.prev
            self.last.next = None
            self.num -= 1

    def get_port(self):
        if self.last == None:
             return 0
        return self.last.value[1]

    def __str__(self):
        if self.first != None:
            current = self.first
            out = 'LinkedList [\n' +str(current.value) +'\n'
            while current.next != None:
                current = current.next
                out += str(current.value) + '\n'
            return out + ']'
        return 'LinkedList []'

    def clear(self):
        self.__init__()

    def copy(self,other):
        self.__init__()
        node = other.first
        while node != None:
            self.insert(node.value)
            node = node.next
            
    def calc_value(self):
        node = self.first
        summ = 0
        while node != None:
            summ += node.get_v_sum()
            node = node.next
        return summ
 
def getBridges (bridges):
    with open("data/bridges", "r") as f:
        data = f.readlines()

    for line in data:
        l_str,r_str = (str(x) for x in line.split("/"))
        r = int(r_str.rstrip('\r\n'))
        l = int(l_str)

        if bridges.get(l) == None:
            bridges[l] = {r:1}
        elif bridges[l].get(r) == None:
            bridges[l][r] = 1
        else: 
            bridges[l][r] += 1

        if bridges.get(r) == None:
            bridges[r] = {l:1}
        elif bridges[r].get(l) == None:
            bridges[r][l] = 1
        else: 
            bridges[r][l] += 1


def buildBridge(bridges,current_bridge,cpu_bridge):
    l_port = current_bridge.get_port()
    r_ports = bridges.get(l_port)
    if r_ports != None :
        if r_ports.get(l_port) != None and r_ports[l_port] > 0:
                current_bridge.insert([l_port,l_port])
                r_ports[l_port] -= 2
                buildBridge(bridges,current_bridge,cpu_bridge)
                current_bridge.remove_back()
                r_ports[l_port] += 2
            
        else: 
            for r_port in r_ports:
                if r_ports[r_port] > 0:
                    current_bridge.insert([l_port,r_port])
                    r_ports[r_port] -= 1
                    bridges[r_port][l_port] -= 1
                    buildBridge(bridges,current_bridge,cpu_bridge)
                    current_bridge.remove_back()
                    r_ports[r_port] += 1
                    bridges[r_port][l_port] += 1

    if current_bridge.num > cpu_bridge.num:    
            cpu_bridge.copy(current_bridge)              
    elif current_bridge.num == cpu_bridge.num:    
        if current_bridge.value > cpu_bridge.value:
            cpu_bridge.copy(current_bridge)              



bridges = {}
getBridges(bridges)


cpu_bridge = LinkedList()
current_bridge = LinkedList()

buildBridge(bridges,current_bridge,cpu_bridge)
print cpu_bridge
print cpu_bridge.value
print cpu_bridge.num



