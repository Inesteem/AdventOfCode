#fname = "../data/day2.txt"
fname = "../data/test.txt"
with open(fname) as f:
    content = f.readlines()
    # you may also want to remove whitespace characters like `\n` at the end of each line
    content = [x.strip() for x in content]
    
#part 1
crc2 = 0
crc3 = 0

for cstr in content:
    for c in cstr:
        if cstr.count(c) == 2:
            crc2 += 1
            break


for cstr in content:
    for c in cstr:
        if cstr.count(c) == 3:
            crc3 += 1
            break
crc = crc2 * crc3 
print("checksum: " + str(crc))

#part2


merged_strings = {}

for ai in range(0,len(content)):
    for bi in range(0,len(content)):
        if ai == bi:
            continue 
        
        u=zip(content[ai],content[bi])
        res = ""
        cnt = 0
        for i,j in u:
            if i==j:
                res += i
            else:
                cnt += 1
        merged_strings[cnt] = res 

for key, value in merged_strings.items():
        print (str(key) + " " +  value)
