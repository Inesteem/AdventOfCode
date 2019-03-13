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



instr_mem = [ [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr] for i in range(0,16)]

#instr_mem =  [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr] 



if len(sys.argv) > 1:
    lines = [line.rstrip('\n') for line in open('../data/test.dat')]
else:
    lines = [line.rstrip('\n') for line in open('../data/day16.dat')]

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

    for fi in range(len(instr_mem[instr[0]])-1, -1, -1):
        tmp = regs_b.copy()
        try:
            instr_mem[instr[0]][fi](tmp, instr[1], instr[2], instr[3])
            if tmp != regs_a:
                del instr_mem[instr[0]][fi]
        except:
                del instr_mem[instr[0]][fi]

changes = True
while changes:
    changes = False 
    for fi in range(0, len(instr_mem)):
        if len(instr_mem[fi]) == 1:
            for fj in range(0, len(instr_mem)):
                if fi == fj or len(instr_mem[fj]) == 1:
                    continue;
                try:
                    instr_mem[fj].remove(instr_mem[fi][0])
                    changes = True 
                except:
                    pass 
    
print("execute file:") 
while i < len(lines):
    if len(lines[i]) == 0:
        i=i+1
        continue

    instr = [ int(x) for x in lines[i].split(' ')]
    instr_mem[instr[0]][0](reg_file, instr[1], instr[2], instr[3])
    print("lines " + str(i))
    i=i+1

print(reg_file)
