import math

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
        print str(num) + " " + str(numParts)
        if numParts == 1:
            l.append([self])
            return
        #l =  [Image()]*numParts*numParts
        for p in xrange(numParts*numParts):
            i = Image() 
            i.size = num
            l.append(i)

        for j in xrange(len(self.pixels)):
            line = [self.pixels[j][i:i + num] for i in xrange(0, len(self.pixels[j]), num)] 
            print line
            for i in xrange(len(line)):
                l[int(j/num)*numParts+i].pixels.append(line[i])

def mergeImg(imgList):
    if len(imgList) == 0:
        return

    ret = Image()
    numImgs = math.sqrt(len(imgList))
    assert numImgs == int(numImgs), "no valid image list!"
    numImgs = int(numImgs)
    size = imgList[0].size
    assert (size == 2 ) or ( size == 3 ), "no valid part images"
    for img_r in xrange(numImgs):
        for img_c in xrange(numImgs):
            for row in xrange(size):
                line = []
                for col in xrange(size):
                    line.append(imgList[img_r * numImgs +
                        img_c].pixels[row][col])
            ret.pixels.append(line)
 
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



def getRules (l):
    with open("data/testfile2", "r") as f:
        data = f.readlines()

    for line in data:
        r = Rule()
        r.f,r.t=(str(x) for x in line.split("=>"))
        r.t = r.t.rstrip('\r\n')
        r.size = r.t.count('/')
        print r
        l.append(r)  

#l = []
#getRules(l)
img = Image()
#img.size = 3
#img.pixels = {".#.","..#","###"}
img.size = 9 
img.pixels = [  "123456789",
                "abcdefghi",
                "987654321",
                "=========",
                "wasdfhklf",
                ".........",
                "=========",
                "zzzzzzzzz",
                "........."]
l_pics = []
img.getImgs(l_pics)
print l_pics


mImg = mergeImg(l_pics)
print "old : " 
print img
print "new : " 
print mImg
