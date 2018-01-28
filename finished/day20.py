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
        t = Triple()
        t.x = self.x+other.x
        t.y = self.y+other.y
        t.z = self.z+other.z
        return t 
    def __sub__(self, other):
        t = Triple()
        t.x = self.x-other.x
        t.y = self.y-other.y
        t.z = self.z-other.z
        return t 
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
    def ham(self,other):
        x_tmp = math.fabs(self.x-other.x)
        x_tmp += math.fabs(self.y-other.y)
        x_tmp += math.fabs(self.z-other.z)
        return x_tmp
    def sim(self):
        self.y+=self.z
        self.x+=self.y
        return self 

def getPixel(): 
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
    return l_pix

def star39(data): 
    l_pix = getPixel()
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

def getDist(l_elem):
    l_dist = []
    for i in xrange(len(l_elem)):
        l_tmp = []
        for j in xrange(len(l_elem)):
            if i == j:
                continue
            t = Triple()
            t.x = l_elem[i].i
            t.y = l_elem[j].i
            t.z = l_elem[i].x.ham(l_elem[j].x) 
            l_tmp.append(t)
        l_dist.append(l_tmp)
    return l_dist

def remColission(l_elem):
    l_remidx = []
    for i in xrange(len(l_elem)):
        l_tmp = []
        for j in xrange(len(l_elem[i])):
            if l_elem[i][j].z == 0:
                l_remidx.append(i)
                break
    return l_remidx


def remNotAppr(l_dist1, l_dist2):
    assert len(l_dist1) == len(l_dist2)
    l_remidx = []
    for i in xrange(len(l_dist1)):
        appr = False
        for j in xrange(len(l_dist1[i])):
            if(l_dist2[i][j].z < l_dist1[i][j].z):
                appr = True
                break
        if not appr: 
            l_remidx.append(i)

    return l_remidx

def printDList(l):
    for i in xrange(len(l)):
        print l[i]
    print(" ")

l_pix = getPixel()

l_surrender = []
l_dist = []
l_dist_old = []

iterations = 0
while True:
    print "---------------"
    print ""
    print "left: " + str(len(l_pix)) + " elems to parse ! "
    print "saved: " + str(len(l_surrender)) + " elems ! "
    print ""
#    printDList(l_pix)
    #first step
    l_dist = getDist(l_pix)
    l_kill1 = remColission(l_dist)
#    printDList(l_dist_old)
#    printDList(l_dist)
    for i in xrange(len(l_kill1)):
        del l_pix[l_kill1[i]-i]               
        del l_dist[l_kill1[i]-i]   
        if len(l_dist_old) != 0:
            del l_dist_old[l_kill1[i]-i]   
            for j in xrange(len(l_dist_old)):
                del l_dist_old[j][l_kill1[i]-i]   
                        
    print "killed " + str(len(l_kill1)) + " elems in step " + str(iterations)    
    print ""


    if(len(l_dist_old) != 0):

        l_save = remNotAppr(l_dist_old, l_dist)

        if len(l_save) == len(l_pix):
            break
        
     #   for i in xrange(len(l_save)):
     #       l_surrender.append(l_pix[l_save[i]-i])
     #       del l_pix[l_save[i]-i]               
     #       del l_dist[l_save[i]-i]   
     #       for j in xrange(len(l_dist)):
     #           del l_dist[j][l_save[i]-i]   
        
        print "saved  " + str(len(l_save)) + " elems in step " + str(iterations)     
        print " " 

    #third step : simulation
    l_dist_old = list(l_dist)
    for i in xrange(len(l_pix)):
        l_pix[i] = l_pix[i].sim()
    iterations += 1
#print l_pix
print "living : " + str(len(l_surrender) + len(l_pix))
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
