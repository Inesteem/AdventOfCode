import re
import sys
import math
import datetime



def midnight(time):
    while time.hour == 23:
         time += datetime.timedelta(minutes = 1)
    return time 

class Guard():

    def __init__(self, num):
        self.num = num 
        self.awake = 0
        self.asleep = 0
        self.mins = [0] * 60
        return None


lines = [line.rstrip('\n') for line in open('../data/day4.dat')]
#lines = [line.rstrip('\n') for line in open('../data/test.dat')]

lines.sort();
date_format = "%Y-%m-%d %H:%M"

shift = []

#test  = datetime.datetime.strptime("2000-01-01 23:00", date_format)
#print(rectify_time(test))

awake_since = datetime.datetime.strptime("2000-01-01 00:00", date_format)
asleep_since = datetime.datetime.strptime("2000-01-01 00:00", date_format)
guards = {}
num = -1
for line in lines:
    date_str,command = line.split('] ')
    date_str = date_str[1:]
    print(date_str +  " - " + command)
    if command == "wakes up": 
        awake_since = datetime.datetime.strptime(date_str, date_format)
        diff = round((awake_since - asleep_since).total_seconds()/60)
        guards[num].asleep += diff
        print("awake: " + str(diff))
        for i in range(0,diff):
            shift[len(shift)-1][i+int(asleep_since.minute)] = '#'
            guards[num].mins[i+int(asleep_since.minute)] += 1
    elif command == "falls asleep": 
        asleep_since = datetime.datetime.strptime(date_str, date_format)
        diff = round((asleep_since - awake_since).total_seconds()/60)
        guards[num].awake += diff 
        print("asleep: " + str(diff))
    else:
        print("---------")
        shift.append(['.'] * 60)
        if num != -1:
            if awake_since > asleep_since:
                print("last guard " + str(num) + " was awake!")
                assert awake_since.hour < 1
                one_a_clock = awake_since.replace(hour=1,minute=0)
                diff = round((one_a_clock - awake_since).total_seconds()/60)
                guards[num].awake += diff 
                print(str(awake_since) + " " +str(one_a_clock) + "  " + str(diff))
            else: 
                print("last guard " + str(num) + " was asleep!")
                assert asleep_since.hour < 1
                one_a_clock = asleep_since.replace(hour=1,minute=0)
                diff = round((one_a_clock - asleep_since).total_seconds()/60)
                guards[num].asleep += diff 
                print(str(asleep_since) + " " +str(one_a_clock) + "  " + str(diff))
                for i in range(0,int(diff)):
                    shift[len(shift)-2][i+int(asleep_since.minute)] = '#'
        awake_since = midnight(datetime.datetime.strptime(date_str, date_format))
        print(str(awake_since) + " minutes been asleep " + str(awake_since.minute))
        num = int(command.rpartition('#')[2].split(' ')[0])
        if not num in guards:
            guards[num] = Guard(num)
        #guards[num].asleep +=awake_since.minute 



#strategy 1
asleep = 0
best_num = 0
for num,guard in guards.items():
    print("guard " + str(num) + " was awake for " + str(guard.awake) + " minutes")
    print("guard " + str(num) + " was asleep for " + str(guard.asleep) + " minutes")
    print(guard.mins)
    if guard.asleep > asleep:
        asleep = guard.asleep
        best_num = num 

#for row in shift: 
#    for elem in row:
#        sys.stdout.write(elem + " ")
#    sys.stdout.write('\n')

print(best_num)
best_min = guards[best_num].mins.index(max(guards[best_num].mins)) 
print(str(best_num * best_min))


#strategy 2
best_min = 0
best_num = 0
for num,guard in guards.items():
    guard_best_min = max(guard.mins) 
    if guard_best_min > best_min:
        best_num = num 
        best_min = guard_best_min 


best_min = guards[best_num].mins.index(max(guards[best_num].mins)) 
print(best_min)
print(best_num)
print(str(best_num * best_min))

