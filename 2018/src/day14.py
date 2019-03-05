
import re
import sys 

scores = [3,7]
pos_1 = 0
pos_2 = 1


if len(sys.argv) != 2:
    print("usage: prog numIter")
    exit(-1)

goal = [for x in sys.argv[1]: int(x)]
print(goal)
exit(0)



while True:
    new_score = scores[pos_1] + scores[pos_2]
    if new_score > 9: #can never ne > 19
        scores.append(1)
        scores.append(new_score - 10)
    else:
        scores.append(new_score)
# break condition for star 1
#    if len(scores) >= int(sys.argv[1]) + 10:
#        break
# break condition for star 2
    if 
    pos_1 = (1+pos_1+scores[pos_1]) % len(scores)
    pos_2 = (1+pos_2+scores[pos_2]) % len(scores)
            
#print("whole sequence:")
#for score in scores:
#    sys.stdout.write(str(score) + " ")

print()
print("1. answer:")

for i in range(int(sys.argv[1]), int(sys.argv[1]) + 10):
    sys.stdout.write(str(scores[i]))

print()
