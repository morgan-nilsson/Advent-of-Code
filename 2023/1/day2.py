string = '''two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
'''

nums = ['zero','one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']

str_arr = string.split('\n')
arr = []

def findFirst(line):
    for index in range(len(line)):
        if line[index:index+1].isdigit():
            sub_idx = 0
            while line[index:index+sub_idx+1].isdigit():
                sub_idx += 1
            return line[index:index+sub_idx]
        for num_idx in range(len(nums)):
            val = line[index:index+6].find(nums[num_idx])
            if val != -1:
                return str(num_idx)


for line in str_arr:
    first = findFirst(line)
    print(first)
    last = findFirst(line[::-1])
    print(last)
    if not first or not last:
        print('not')
        exit()
    arr.append(int(first) + int(last))
print(arr)
