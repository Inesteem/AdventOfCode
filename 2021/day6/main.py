#!/usr/bin/env python3
#star1
#days=80
days=256

firstSpawn = 8
spawnTime = 6

dp = [-1 for i in range(days+1)]
def numChilds(timeLeft):
    global dp
    if timeLeft < 0:
        return 0

    if dp[timeLeft] == -1:
        dp[timeLeft] = 1 + numChilds(timeLeft - 7) + numChilds(timeLeft - 9)
    
    return dp[timeLeft]

if __name__ == '__main__':

    fishes =  [int(x) for x in input().split(",")]
    num = 0
    for fish in fishes:
        num += 1 + numChilds(days - 1 - fish)

    print(num)
