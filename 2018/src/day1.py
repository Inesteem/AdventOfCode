fname = "../data/day1.txt"
#fname = "../data/test.txt"
with open(fname) as f:
    content = f.readlines()
    # you may also want to remove whitespace characters like `\n` at the end of each line
    content = [x.strip() for x in content]
    
#part 1

frequency = 0

for c in content:
    frequency = frequency + int(c)

print(frequency) 

#part 2

frequencies = set([0])

frequency = 0
found = False
while not found:
    for c in content:
        frequency = frequency + int(c)
        if frequency in frequencies:
            print("found first twice frequency : " + str(frequency))
            found = True
            break
        frequencies.add(frequency)


