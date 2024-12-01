str = '''
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
'''
arr = []
idx = -1
str_arr = str.split('\n')
for line in str_arr:
    if line == "":
        idx += 1
        arr.append(0)
    else:
        arr[idx] += int(line)

arr.sort()
print(arr)
total = arr[len(arr)-1] + arr[len(arr)-2] + arr[len(arr)-3]
print(total)
