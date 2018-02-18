#import math
#import sys
import operator
import re
def reverse(string):
    string = string[::-1]
    return string

class Room:
    def __init__(self):
        self.list = []
        self.ll = ""
        self.id = -1
        self.crc = None
        self.map = {}
    def __str__(self):
        return str(self.id) + ": [" + self.crc + "] " +  re.sub('[,\[\]\']', '', str(self.list))
   
    def check_crc(self):
        #self.ll =  ''.join(sorted(self.ll))
        for c in self.ll:
            if self.map.get(c) == None:
                self.map[c] = 1
            else:
                self.map[c] += 1

        items = sorted(list(self.map.items()))
#        sorted_l = sorted(items)
        sorted_l = sorted(items, key=lambda x: x[1],reverse=True)
        num = 0
        crc = ""
        for i in sorted_l:
            if num < 5:
                crc += i[0]
                num += 1
             
        return crc == self.crc

    def decrypt(self):
        for sl in xrange(len(self.list)):
               s = ""
               for c in xrange(len(self.list[sl])):
                    s += chr((ord(self.list[sl][c]) - ord('a') + self.id)%26 + ord('a'))
               self.list[sl] = s      
#        sorted_map = sorted( self.map.items(), key=operator.itemgetter(1), reverse=True)
#        for i in sorted_map:
#            print i       
        
def getRooms(path, rooms):
    with open(path, "r") as f:
        data = f.readlines()
    for line in data:
        r = Room()
        ll =  line.split("-")
        info = ll[len(ll)-1].rstrip('\r\n')
        r.list = ll[:-1]
        
        for s in r.list:
            r.ll += s
        r.crc = info[info.find("[")+1:info.find("]")]
        r.id = int(info[:info.find("[")])
        rooms.append(r)

rooms = []
idsum = 0
getRooms('input/decoy',rooms)

for r in rooms:
    if r.check_crc():
        idsum += r.id
    r.decrypt()
    print r

print idsum
