import md5
import argparse

parser = argparse.ArgumentParser(prog='door hacker', usage='%(prog)s [options]')
parser.add_argument('--id')
args = parser.parse_args()

door_id = 'abc'
if args.id != None:
    door_id  = args.id


index = 0
pw = ''
while len(pw) < 8:
    hash_t = door_id + str(index)
    m = md5.new()
    m.update(hash_t)
    tmp = m.hexdigest()
    if tmp[:5] == "00000":
        pw += tmp[5]
        print pw
        print tmp
        print "index: " + str(index)
        print "hash: " + hash_t
    index += 1

print pw

