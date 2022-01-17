#!/usr/bin/env python3

def gauss(n):
    return (n*(n+1)) // 2

def code(row, col):

    start_diag = row+col
    num = gauss(start_diag+1)-start_diag + col
    
    code = 20151125
    for i in range(1,num):
        code *= 252533
        code %= 33554393

    return code

if __name__ == '__main__':
    #(row,col) = (int(x) for x in input().split())
    #for row in range(6):
    #    for col in range(6):
    #        print(code(row, col), end=" ")
    #    print()
    print(code(2981 - 1, 3075 - 1))

