import re
import sys

field = []


lines = [line.rstrip('\n') for line in open('../data/day3.dat')]
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]

for line in lines:
    shit,x_pos,y_pos,x_len,y_len = re.split('#[\w]+ @ |,|: |x',line)
    print(str(x_pos) + " " + str(y_pos) + " " + str(x_len) + " " + str(y_len) + " ")
    for i in range(0, int(y_pos) + int(y_len)):
        if len(field) <= i:
            field.append([])
        while len(field[i]) < int(x_pos) + int(x_len):
            field[i].append(0)
        if(i >= int(y_pos)):
            for x in range(int(x_pos), int(x_pos) + int(x_len)):
                field[i][x] += 1 
cnt = 0;
for row in field:
    for elem in row:
        if elem == 0:
            sys.stdout.write('.')
        else:
            sys.stdout.write(str(elem))
            if elem > 1:
                cnt += 1
    sys.stdout.write('\n')

sys.stdout.flush()
print(cnt)


for line in lines:
    shit, num, x_pos,y_pos,x_len,y_len = re.split('#| @ |,|: |x',line)
    print(str(num) + " " + str(x_pos) + " " + str(y_pos) + " " + str(x_len) + " " + str(y_len) + " ")
    overlap = 0
    for y in range(int(y_pos), int(y_pos) + int(y_len)):
        for x in range(int(x_pos), int(x_pos) + int(x_len)):
            if field[y][x] > 1:
                overlap = 1
                break
        if overlap == 1: 
            break
    if overlap == 0:
        print(str(num))
        break 


