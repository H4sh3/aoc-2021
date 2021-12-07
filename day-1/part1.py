with open('input.txt', 'r') as f:
    data = f.readlines()
data = [int(x) for x in data]


cnt = 0
last = None
for e in data:
    if last is None:
        last = e
        continue

    if e > last:
        cnt += 1
    last = e

print(cnt)
