
import re
import sys 

scores = [3,7]
pos_1 = 0
pos_2 = 1


if len(sys.argv) != 2:
    print("usage: prog numIter")
    exit(-1)

goal = [int(x) for x in list(sys.argv[1])]
matched = 0

if goal[0] == scores[0] and goal[1] == scores[1]:
    matched = 2
    if matched == len(goal):
        print(0)
        exit(0)
elif goal[0] == scores[1]:
    matched = 1
    if matched == len(goal):
        print(1)
        exit(0)



def check_goal(goal, scores, matched):

    if goal[matched] == scores[-1]:
        return True 
    else:
        return False



while True:
    new_score = scores[pos_1] + scores[pos_2]
    if new_score > 9: #can never be > 19
        scores.append(1)
        if(check_goal(goal, scores, matched)):
            matched = matched + 1
            if matched == len(goal):
                break
        elif(check_goal(goal, scores, 0)):
            matched = 1
            if matched == len(goal):
                break
        else:
            matched = 0

        scores.append(new_score - 10)
    else:
        scores.append(new_score)
# break condition for star 1
#    if len(scores) >= int(sys.argv[1]) + 10:
#        break
# break condition for star 2

    if(check_goal(goal, scores, matched)):
        matched = matched + 1
        if matched == len(goal):
            break
    elif(check_goal(goal, scores, 0)):
        matched = 1
        if matched == len(goal):
            break
    else:
        matched = 0
    

    pos_1 = (1+pos_1+scores[pos_1]) % len(scores)
    pos_2 = (1+pos_2+scores[pos_2]) % len(scores)
            
#print("whole sequence:")
#for score in scores:
#    sys.stdout.write(str(score) + " ")
 
print(str(len(scores) - len(goal)))

#star 1 answer:

#print()
#print("1. answer:")

#for i in range(int(sys.argv[1]), int(sys.argv[1]) + 10):
#    sys.stdout.write(str(scores[i]))

#print()
