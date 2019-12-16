import re;

condition1 = re.compile(r"(..).*\1")
condition2 = re.compile(r"(.).\1")

count_nice = 0
with open("input.txt", 'r') as f:
    for line in f.readlines():
        line = line.rstrip()
        print("Testing `%s`... " % line, end="")
        if None != condition1.search(line) \
                and None != condition2.search(line):
            count_nice += 1
            print(" nice")
        else:
            print(" naughty")

print(count_nice)