#!/usr/bin/env python3
import re

FREE = "."
GEAR = "*"

gear_map = {}


def key(a, b):
    return f"{a}:{b}"


def add_to_gear(row, col, num):
    k = key(row, col)
    if k not in gear_map:
        gear_map[k] = [num]
    else:
        gear_map[k] += [num]


def calc(num, board, row, col_start, l):
    useful = False
    for c in range(0, l + 2):
        useful = useful or board[row - 1][col_start - 1 + c] != FREE
        if board[row - 1][col_start - 1 + c] == GEAR:
            add_to_gear(row - 1, col_start - 1 + c, num)
        useful = useful or board[row + 1][col_start - 1 + c] != FREE
        if board[row + 1][col_start - 1 + c] == GEAR:
            add_to_gear(row + 1, col_start - 1 + c, num)
    useful = useful or board[row][col_start -
                                  1] != FREE or board[row][col_start +
                                                           l] != FREE

    if board[row][col_start - 1] == GEAR:
        add_to_gear(row, col_start - 1, num)
    if board[row][col_start + l] == GEAR:
        add_to_gear(row, col_start + l, num)
    if useful:
        return num
    return 0


if __name__ == "__main__":
    with open("input") as f:
        lines = f.readlines()
        width = len(lines[0]) - 1
        board = [["."] * (width + 2)]
        for line in lines:
            row = ["."]
            row += [c for c in line.strip()]
            row += ["."]
            board.append(row)
        board += [["."] * (width + 2)]

        star1 = 0
        for row in range(len(board)):
            col = 0
            while col < len(board[0]):
                c = board[row][col]
                if c.isdigit():
                    l = 0
                    num = 0
                    while c.isdigit():
                        num *= 10
                        num += int(c)
                        col += 1
                        l += 1
                        c = board[row][col]
                    star1 += calc(num, board, row, col - l, l)
                col += 1

        print("star1", star1)

        star2 = 0
        for gear in gear_map:
            #    print(gear, gear_map[gear])
            if len(gear_map[gear]) == 2:
                star2 += gear_map[gear][0] * gear_map[gear][1]
        print("star2", star2)


def stuff():
    with open("test2") as f:
        lines = f.readlines()
        width = len(lines[0]) - 1
        board = "." * (width + 2)
        for line in lines:
            board += "."
            board += line.strip()
            board += "."
        board += "." * (width + 2)

        #for res in re.findall("[0-9]+", board):
        #    print(res)
        for i in range(1, 1 + 1):
            prev = "[\.]{" + str(2 + i) + "}"
            prev += "[.]{" + str(width - i) + "}"
            prev += "\."
            after = "\."
            after += "[.]{" + str(width - i) + "}"
            after += "[\.]{" + str(2 + i) + "}"
            pattern = "(?=" + prev + "([0-9]{" + str(i) + "})" + after + ")"
            print(pattern, board)

            matches = re.finditer(pattern, board)
            results = [int(match.group(1)) for match in matches]
            for res in results:
                print(i, res)
