### EXAMPLE PYTHON MODULE
# Define some variables:
import re
import math 
numberone = 1
ageofqueen = 78

# define some functions
def printhello():
    print "hello"
    
def timesfour(input):
    print input * 4
    
# define a class
class Triple:
    def __init__(self,x,y,z,i):
        self.x = x
        self.y = y
        self.z = z
        self.i = i
    def __init__(self):
        self.x = 0
        self.y = 0
        self.z = 0
        self.i =-1 
    def __repr__(self):
        sstr = "<"
        if self.i != -1:
            sstr += "["+str(self.i)+"]"
        sstr += str(self.x)+ ","+str(self.y)+","+str(self.z) +">"
        return sstr
    def __str__(self):
        sstr = "<"
        if self.i != -1:
            sstr += "["+str(self.i)+"]"
        sstr += str(self.x)+ ","+str(self.y)+","+str(self.z) +">"
        return sstr
    def __add__(self, other):
        return Triple(self.x+other.x, self.y+other.y, self.z+other.z,-1) 
    def __iadd__(self, other):
        self.x+=other.x
        self.y+=other.y 
        self.z+=other.z
        return self 
    def ham(self):
        x_tmp = math.fabs(self.x)
        x_tmp += math.fabs(self.y)
        x_tmp += math.fabs(self.z)
        return x_tmp

with open("data/testfile2", "r") as f:
    data = f.readlines()
 
l_pix = []
idx = 0
for line in data:
    pos = Triple()
    acc = Triple()
    vec = Triple()
    pix = Triple()
    line = line.translate(None, 'pva=<> ')
    #line = re.sub('pva=<>,', ' ', line)
    pos.x,pos.y,pos.z,vec.x,vec.y,vec.z,acc.x,acc.y,acc.z=(int(x) for x in line.split(",")) 
    pix.x = pos
    pix.y = vec
    pix.z = acc
    pix.i = idx
    l_pix.append(pix)
    idx += 1


l_ham = []
for a in l_pix:
    l_ham.append(a.z.ham())


i_min = min(l_ham)
print i_min

l_elem = [p for p in l_pix if p.z.ham() <= i_min]
print l_elem


l_ham = []
for a in l_elem:
    l_ham.append(a.y.ham())
i_min = min(l_ham)

l_elem = [p for p in l_elem if p.y.ham() <= i_min]

print l_elem




#appr = True
#while appr:
#    appr = False
#    for i in xrange(len(l_elem)):
#        ham1 = l_elem[i].x.ham()
#        l_elem[i].y += l_elem[i].z
#        l_elem[i].x += l_elem[i].y
#        ham2 = l_elem[i].x.ham()
#        if ham1 >= ham2:
#            appr = True
#        
#l_ham = []
#for a in l_elem:
#    l_ham.append(a.x.ham())
#
#i_min = min(l_ham)
#print i_min
#l_elem = [p for p in l_elem if p.x.ham() <= i_min]
#print l_elem

#for i in range(10000):
#    for i in xrange(len(l_pix)):
#        l_pix[i].y += l_pix[i].z
#        l_pix[i].x += l_pix[i].y
#
#l_ham = []
#for a in l_pix:
#    l_ham.append(a.x.ham())
#
#i_min = min(l_ham)
#l_elem = [p for p in l_pix if p.x.ham() <= i_min]
#print l_elem
