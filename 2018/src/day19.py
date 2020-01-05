import sys 
import re



def addr(regs, r1, r2, r_dst):
    regs[r_dst] = regs[r1] + regs[r2]

def addi(regs, r1, imm, r_dst):
    regs[r_dst] = regs[r1] + imm


def mulr(regs, r1, r2, r_dst):
    regs[r_dst] = regs[r1] * regs[r2]

def muli(regs, r1, imm, r_dst):
    regs[r_dst] = regs[r1] * imm


def banr(regs, r1, r2, r_dst):
    regs[r_dst] = regs[r1] & regs[r2]

def bani(regs, r1, imm, r_dst):
    regs[r_dst] = regs[r1] & imm


def borr(regs, r1, r2, r_dst):
    regs[r_dst] = regs[r1] | regs[r2]

def bori(regs, r1, imm, r_dst):
    regs[r_dst] = regs[r1] | imm


def setr(regs, r1, dummy, r_dst):
    regs[r_dst] = regs[r1]

def seti(regs, imm, dummy, r_dst):
    regs[r_dst] = imm



def gtir(regs, imm, r1, r_dst):
    if imm > regs[r1]:
        regs[r_dst] = 1
    else:
        regs[r_dst] = 0 

def gtri(regs, r1, imm, r_dst):
    if regs[r1] > imm:
        regs[r_dst] = 1
    else:
        regs[r_dst] = 0 

def gtrr(regs, r1, r2, r_dst):
    if regs[r1] > regs[r2]:
        regs[r_dst] = 1
    else:
        regs[r_dst] = 0 


def eqir(regs, imm, r1, r_dst):
    if imm == regs[r1]:
        regs[r_dst] = 1
    else:
        regs[r_dst] = 0 

def eqri(regs, r1, imm, r_dst):
    if regs[r1] == imm:
        regs[r_dst] = 1
    else:
        regs[r_dst] = 0 

def eqrr(regs, r1, r2, r_dst):
    if regs[r1] == regs[r2]:
        regs[r_dst] = 1
    else:
        regs[r_dst] = 0 


instr_mem = {
        "addr": addr,
        "addi": addi,
        "mulr": mulr,
        "muli": muli,
        "banr": banr,
        "bani": bani,
        "borr": borr,
        "bori": bori,
        "setr": setr,
        "seti": seti,
        "gtir": gtir,
        "gtri": gtri,
        "gtrr": gtrr,
        "eqir": eqir,
        "eqri": eqri,
        "eqrr": eqrr 
        } 


    
if len(sys.argv) > 1:
    lines = [line.rstrip('\n') for line in open('../data/test.dat')]
else:
    lines = [line.rstrip('\n') for line in open('../data/day19.dat')]

reg_file = [0,0,0,0,0,0]
eip_reg = int(lines[0][4])
eip = 0
del lines[0]

# program given:
r0 = 0
r1 = 0
#r2 = 986 star1
r2 = 10551386 #star2
r5 = 1

while r2 >= r5:
    tmp = r2/r5 
    if(tmp == int(tmp)):
        r0 += r5
#    r3 = 1
#    while r2 >= r3:
#        r1 = r5 * r3
#        if r1 == r2:
#            r0 += r5
#            break
#        r3 += 1
    r5 += 1

print(r0)



print("execute file:") 
i = 0
while eip < len(lines):
    if len(lines[eip]) == 0:
        eip=eip+1
        continue


#    sys.stdout.write("ip="+str(eip) + " " + str(reg_file))

    # reg <- eip
    reg_file[eip_reg] = eip 

    instr = lines[eip].split(' ')
    if instr[0][0] == "#":
        eip_reg = int(instr[1])
    else: 
        instr_mem[instr[0]](reg_file, int(instr[1]), int(instr[2]), int(instr[3]))
    

 #   sys.stdout.write(" " + lines[eip] + " " + str(reg_file))
    # eip -> reg
    eip = reg_file[eip_reg] 

    eip=eip+1

  
        
#    print()
#    input()
print(reg_file)
