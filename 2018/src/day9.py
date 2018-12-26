import sys

player_num = 439
worth = 71307 * 100

current = 1

player_scores = player_num * [0]

#print( player_scores )

circle = [0,1]
#print ("[0]: (0)")
#print ("[1]:  0  (1)")
insert_idx = 0
for turn in range(2,worth+1):
    if turn % 23 == 0:
        player_scores[turn % player_num] += turn 
        remove_idx = (current - 7) % len(circle)
        player_scores[turn % player_num] += circle[remove_idx] 
        del circle[remove_idx]
        current = (remove_idx) % len(circle)
    else:
        if current == len(circle) - 1:
            insert_idx = 1
        elif current == len(circle) - 2:
            insert_idx = len(circle)
        else:
            insert_idx = (current + 2)% (len(circle) )
        current = insert_idx
        circle.insert(insert_idx, turn)


#    sys.stdout.write("[" + str(turn) + "]: ")
#    for mi in range(0,len(circle)):
#        if mi == current:
#            sys.stdout.write("("+str(circle[mi])+") ")
#        else:
#            sys.stdout.write(" "+str(circle[mi])+"  ")
#        
#    sys.stdout.write("\n")

print(player_scores)    
print(max(player_scores))    
