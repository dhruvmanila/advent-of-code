FREQ_CHANGE_LIST = []
test = [+3, +3, +4, -2, -4]

with open("input/01.txt") as inp:
    for line in inp:
        FREQ_CHANGE_LIST.append(int(line.strip()))

print(FREQ_CHANGE_LIST)
