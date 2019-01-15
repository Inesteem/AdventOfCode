import sys


class Element:
    def __init__(self, value, idx, _next_, _prev_):
        self.value = value 
        self.next_i = _next_
        self.prev_i = _prev_
        self.idx = idx 
        return None

    def insert_after(self, num, elem_i, elements):
        if num > 0:
            elements[self.next_i].insert_after(num - 1, elem_i, elements)
        else: 
            elements[elem_i].next_i = self.next_i 
            elements[elem_i].prev_i = self.idx 
            elements[self.next_i].prev_i = elem_i 
            self.next_i = elem_i 

    #kein check für den <= 2 element case
    def remove_before(self, num, elements):
        if num > 0:
            return elements[self.prev_i].remove_before(num - 1, elements)
        else: 

            elements[self.prev_i].next_i = self.next_i 
            elements[self.next_i].prev_i = self.prev_i 

            return self  

    def __str__(self):
        return str(self.prev_i) + " <-- " + str(self.idx) + " --> " + str(self.next_i)

player_num = 439
worth = 7130700 #* 100


player_scores = player_num * [0]

elements = [Element(0,0,0,0)]

current = 0 
next_free = 1


print ("[0]: (0)")
for turn in range(1,worth+1):
    if turn % 23 == 0:
        
        removed_elem = elements[current].remove_before(7, elements)
        current = removed_elem.next_i 
        player_scores[turn % player_num] += turn + removed_elem.value 

    else:
        elements.append(Element(turn,next_free,0,0))
        elements[current].insert_after(1, next_free, elements)
        current = next_free 
        next_free += 1
         
#    if turn % 23 == 0:
#        player_scores[turn % player_num] += turn 
#        remove_idx = (current - 7) % len(circle)
#        player_scores[turn % player_num] += circle[remove_idx] 
#        del circle[remove_idx]
#        current = (remove_idx) % len(circle)
#
#    else:
#        if current == len(circle) - 1:
#            insert_idx = 1
#        elif current == len(circle) - 2:
#            insert_idx = len(circle)
#        else:
#            insert_idx = (current + 2)% (len(circle) )
#        current = insert_idx
#        circle.insert(insert_idx, turn)


#    sys.stdout.write("[" + str(turn) + "]: ")
#    idx = 0
#    for mi in range(0,next_free):
#
#        if idx == current:
#            sys.stdout.write("("+str(elements[idx].value)+") ")
#        else:
#            sys.stdout.write(" "+str(elements[idx].value)+"  ")
#        idx = elements[idx].next_i 
#    sys.stdout.write("\n")

print(player_scores)    
print(max(player_scores))    
