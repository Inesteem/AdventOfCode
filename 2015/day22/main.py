#!/usr/bin/env python3
#shitty rust, just do it in python very quick
from enum import IntEnum
class Spell(IntEnum):
    MISSILE = 53
    DRAIN = 73
    SHIELD = 113
    POISON = 173
    RECHARGE = 229

SHIELD_TIME = 6
POISON_TIME = 6
RECHARGE_TIME = 5 

minMana = 10000000000
star2=True

def battle(pTurn, pHit, pMan, bHit, bDmg, mana, sLeft, pLeft, rLeft, action):
    global minMana

    if pTurn and star2:
        pHit -= 1
        if pHit <= 0:
            return -1

    #RECHARGE
    if rLeft >= 0:
        pMan += 101

    #SHIELD
    pArm = 0
    if sLeft >= 0:
        pArm = 7

    #POISON
    if pLeft >= 0:
        bHit -= 3

    if pTurn:
        if int(action) > pMan:
            return -1

        pMan -= int(action)
        mana += int(action)

        if action == Spell.MISSILE:
            bHit -= 4
        elif action == Spell.DRAIN:
            pHit += 2
            bHit -= 2
        elif action == Spell.SHIELD:
            if sLeft > 0:
                return -1
            sLeft = SHIELD_TIME
        elif action == Spell.POISON:
            if pLeft > 0:
                return -1
            pLeft = POISON_TIME
        else:
            if rLeft > 0:
               return -1
            rLeft = RECHARGE_TIME

    else: #BOSS TURN
        pHit -= max(1, bDmg-pArm)


    if bHit <= 0: #BOSS DEAD
        minMana = min(mana, minMana)
        return mana
    
    if pHit <= 0: #PLAYER DEAD
        return -1

    if not pTurn:
        return battle(not pTurn, pHit, pMan, bHit, bDmg, mana, sLeft-1, pLeft-1, rLeft-1, action)
    return start_beef(not pTurn, pHit, pMan, bHit, bDmg, mana, sLeft-1, pLeft-1, rLeft-1)



def start_beef(pTurn, pHit, pMan, bHit, bDmg, mana, sLeft, pLeft, rLeft):
    if mana >= minMana:
        return -1
    for action in Spell:
        battle(pTurn, pHit, pMan, bHit, bDmg, mana, sLeft, pLeft, rLeft, action)

    return minMana

def main():
    print(start_beef(pTurn=True, pHit = 50, pMan = 500, bHit = 51, bDmg = 9, mana = 0, sLeft = -1, pLeft = -1, rLeft = -1))

if __name__ == '__main__':
    main()
