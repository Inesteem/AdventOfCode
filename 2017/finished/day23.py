import math
def reverse(string):
    string = string[::-1]
    return string

class RegMem:
    def __init__(self):
        self.mem = [1,0,0,0,0,0,0,0]
    def __repr__(self):
        return str(self)
    def __str__(self):
        sstr = ""
        for i in xrange(len(self.mem)):
            sstr += str(self.mem[i]) + " "
        return sstr

    def set(self,reg,val):
        idx = ord(reg) - ord('a')
        assert idx >= 0 and idx < len(self.mem), " out of range in set"   
        self.mem[idx] = val

    def get(self,reg):
        idx = ord(reg) - ord('a')
        assert idx >= 0 and idx < len(self.mem), " out of range in get"
        return self.mem[idx]           

class Instr:
    def __init__(self):
        self.name = ""
    def __repr__(self):
        return str(self)
    def __str__(self):
        sstr =  str(self.name) + " " + str(self.op1) + " " + str(self.op2) + " " 
        return sstr

    def getLeftOp(self, mem):
        try: 
            return int(self.op1)
        except ValueError:
            return mem.get(self.op1)

    def getRightOp(self, mem):
        try: 
            return int(self.op2)
        except ValueError:
            return mem.get(self.op2)
 
  
def getInstructions (ll):
    with open("data/instructions", "r") as f:
        data = f.readlines()

    for line in data:
        instr = Instr()
        instr.name, instr.op1, instr.op2 = (str(x) for x in line.split(" "))
        instr.op2 = instr.op2.rstrip('\r\n')
        ll.append(instr)


b = 106700
c = 123700
h = 0
while True:
    f = 1
    for d in xrange(2,b):
        if b % d == 0:
            f = 0
            break

    if(f == 0):
        h += 1
    if(b == c):
        break   
    b += 17
        

print h




#mem = RegMem()
#ll = []
#getInstructions(ll)
#i = 0
#cnt = 0
#while i < len(ll):
#
#    if ll[i].name == "set":
#        mem.set(ll[i].op1[0], ll[i].getRightOp(mem))
#    elif ll[i].name == "sub":
#        sub = mem.get(ll[i].op1[0])
#        sub = sub - ll[i].getRightOp(mem)
#        mem.set(ll[i].op1[0],sub)
#    elif ll[i].name == "mul":
#        mul = mem.get(ll[i].op1[0])
#        mul = mul * ll[i].getRightOp(mem)
#        mem.set(ll[i].op1[0],mul)
#        cnt += 1
#    elif ll[i].name == "jnz":
#        l = ll[i].getLeftOp(mem)
#        r = ll[i].getRightOp(mem)
#        if l != 0:
#            i += r
#            if i < 0 or i >= len(ll):
#                break
#            continue
#
#    elif ll[i].name == "jp":
#        l = ll[i].getLeftOp(mem)
#        r = ll[i].getRightOp(mem)
#        if l > 0:
#            i += r
#            if i < 0 or i >= len(ll):
#                break
#            continue
#
#
#    print str(ll[i].name) + ": " + str(mem)
#            
#    i += 1
#
#print cnt
#print mem.get('h')          
