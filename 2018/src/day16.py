import sys 
import re

reg_file = [0,0,0,0]

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



#instr_mem = [ [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr] for i in range(0,16)]

instr_mem =  [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr] 



if len(sys.argv) > 1:
    lines = [line.rstrip('\n') for line in open('../data/test.dat')]
else:
    lines = [line.rstrip('\n') for line in open('../data/day16.dat')]

dreier=0
last_idx = 0
for i in range(0, len(lines), 4):
    if len(lines[i]) == 0:
        last_idx = i
        break
    print("line " + str(i))
    before =  re.split(',|\[|]',lines[i])
    instr = [ int(x) for x in lines[i+1].split(' ')]
    after =  re.split(',|\[|]',lines[i+2])

    if before[0] != "Before: ":
        break 

    regs_b = [int(before[1]), int(before[2]),int(before[3]),int(before[4])]
    regs_a = [int(after[1]), int(after[2]),int(after[3]),int(after[4])]

    valid_funcs = 0
    for func in instr_mem:
        tmp = regs_b.copy()
        try:
            func(tmp, instr[1], instr[2], instr[3])
            if tmp == regs_a:
                valid_funcs += 1 
        except:
           print("exception")
    if valid_funcs >= 3:
        print(valid_funcs)
        dreier += 1


print("flotte dreier: " + str(dreier))
