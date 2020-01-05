import math
def reverse(string):
    string = string[::-1]
    return string

class Image:
    def __init__(self):
        self.pixels = []
        self.size = 0
    def __repr__(self):
        return str(self)
    def __str__(self):
        sstr ="[" + str(self.size) + "]: "+str(self.pixels)
        return sstr
    def getImgs(self,l):
        div2 = not (self.size&0x1)
        num = 3
        if div2:    
            num = 2
        numParts = self.size / num
        if numParts == 1:
            l.append(self)
            return
        #l =  [Image()]*numParts*numParts
        for p in xrange(numParts*numParts):
            i = Image() 
            i.size = num
            l.append(i)

        for j in xrange(len(self.pixels)):
            line = [self.pixels[j][i:i + num] for i in xrange(0, len(self.pixels[j]), num)] 
            for i in xrange(len(line)):
                l[int(j/num)*numParts+i].pixels.append(line[i])

    def flip(self):
        for row in xrange(self.size):
            self.pixels[row] = reverse(self.pixels[row]);


    def rotate(self):
        new_pixels = []
        new_pixels_tmp = [] 
    
        for row in xrange(self.size):
            new_pixels.append(list(self.pixels[row])) 
            new_pixels_tmp.append(list(self.pixels[row])) 


        for row in xrange(self.size):
            #left = bottom 
            new_pixels_tmp[row][0] = new_pixels[self.size-1][row]            
            #top = left
            new_pixels_tmp[0][row] = new_pixels[self.size-1-row][0]            
            #right = top
            new_pixels_tmp[row][self.size-1] = new_pixels[0][row]            
            #bottom = right
            new_pixels_tmp[self.size-1][row] = new_pixels[self.size-1-row][self.size-1]            
        self.pixels = []
        for row in xrange(self.size):
            string = ""
            for c in new_pixels_tmp[row]:
                string += c
            self.pixels.append(string)
        
        
    

    def applyRule(self,rules):
        for rule in rules:
            if rule.size != self.size:
                continue   
            same = True
            for row in xrange(self.size):
                if self.pixels[row] != rule.f[row]:
                    same = False
                    break
            if same == True:
                self.pixels = rule.t
                self.size += 1
                return True
        return False 

def mergeImg(imgList):
    if len(imgList) == 0:
        return
    if len(imgList) == 1:
        return imgList[0]

    ret = Image()
    numImgs = math.sqrt(len(imgList))
    assert numImgs == int(numImgs), "no valid image list!"
    numImgs = int(numImgs)

    size = imgList[0].size
    assert (size == 2 ) or ( size == 3 ) or (size == 4), "no valid part images"

    for img_r in xrange(numImgs):
        for row in xrange(size):
            stri = ""
            for img_c in xrange(numImgs):
                stri += imgList[img_r * numImgs + img_c].pixels[row]
            ret.pixels.append(stri)  
            
    ret.size = numImgs * size
    return ret

class Rule:
    def __init__(self):
        self.f = []
        self.t = []
        self.size = 0
    def __repr__(self):
        return str(self)
    def __str__(self):
        sstr ="[" + str(self.size) + "]: "+str(self.f)+ " => "+str(self.t)
        return sstr



def getRules (ll):
    with open("data/testfile2", "r") as f:
        data = f.readlines()

    for line in data:
        rule = Rule()
        l,r=(str(x) for x in line.split("=>"))
        r = r.rstrip('\r\n')
        rule.size = l.count('/') + 1
        
        rule.f = l.split('/')
        rule.t = r.split('/')
        for i in xrange(len(rule.f)):
            rule.f[i] = rule.f[i].replace(" ","")
        for i in xrange(len(rule.t)):
            rule.t[i] = rule.t[i].replace(" ","")
        ll.append(rule)

rules = []
getRules(rules)

img = Image()
img.size = 3
img.pixels = [".#.","..#","###"]

#img.size = 9 
#img.pixels = [  "123456789",
#                "abcdefghi",
#                "987654321",
#                "=========",
#                "wasdfhklf",
#                ".........",
#                "=========",
#                "zzzzzzzzz",
#                "........."]
#img.size = 4 
#img.pixels = [  "0123",
#                "4567",
#                "89AB",
#                "CDEF"]
#

for steps in xrange(18):

    l_pics = []
    img.getImgs(l_pics)

    for pic in l_pics:
        for i in xrange(4):
            if pic.applyRule(rules):
                break
            pic.flip()
            if pic.applyRule(rules):
                break

            pic.flip()
            pic.rotate()
            if pic.applyRule(rules):
                break
    print steps 
    img = mergeImg(l_pics)

    on = 0
    for row in xrange(img.size):
        on += img.pixels[row].count('#')
    print on

        

print img
on = 0
for row in xrange(img.size):
    on += img.pixels[row].count('#')

print on
