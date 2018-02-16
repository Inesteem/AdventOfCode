import math
import collections
def reverse(string):
    string = string[::-1]
    return string


tape = collections.deque('0')
state = 'A'
steps = 12173597
pos = 0
crc = 0

while steps > 0:
    steps -= 1
    if pos < 0:
        tape.appendleft('0')
        pos = 0 
    elif pos == len(tape):
        tape.append('0')

    if state == 'A':
        if tape[pos] == '0':
            tape[pos] = '1'
            pos += 1
            state = 'B'
            crc += 1
        else:
            tape[pos] = '0'
            pos -= 1
            state = 'C'
            crc -= 1

    elif state == 'B':
        if tape[pos] == '0':
            tape[pos] = '1'
            pos -= 1
            state = 'A'
            crc += 1
        else:
            tape[pos] = '1'
            pos += 1
            state = 'D'   

    elif state == 'C':
        if tape[pos] == '0':
            tape[pos] = '1'
            pos += 1
            state = 'A'
            crc += 1
        else:
            tape[pos] = '0'
            pos -= 1
            state = 'E'
            crc -= 1

    elif state == 'D':
        if tape[pos] == '0':
            tape[pos] = '1'
            state = 'A'
            crc += 1
        else:
            tape[pos] = '0'
            state = 'B'
            crc -= 1
        pos += 1

    elif state == 'E':
        if tape[pos] == '0':
            tape[pos] = '1'
            state = 'F'
            crc += 1
        else:
            tape[pos] = '1'
            state = 'C'   
        pos -= 1

    else:
        if tape[pos] == '0':
            tape[pos] = '1'
            state = 'D'
            crc += 1
        else:
            tape[pos] = '1'
            state = 'A'   
        pos += 1

print 'crc: ' + str(crc)
