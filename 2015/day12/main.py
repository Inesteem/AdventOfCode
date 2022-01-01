#!/usr/bin/env python3
import json

def parse(data):
    num = 0
    if type(data) == list:
        for elem in data:
            num += parse(elem)
    elif type(data) == int:
        return data

    elif type(data) == dict:
        for key, val in data.items():
            if key == "red" or val == "red":
                return 0
        for key,val in data.items():
            num += parse(key)
            num += parse(val)
    else:
        assert(type(data) == str)

    return num

if __name__ == '__main__':

    line = open("data", 'r').readlines()
    data=json.loads(line[0])
    print(parse(data))
