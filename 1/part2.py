with open('input.txt', 'r') as f:
    data = f.readlines()

data = [int(d) for d in data]

sums = []
for i in range(len(data)):
    sliding_window = data[i:3+i]
    if len(sliding_window) < 3:
        break

    sums.append(sum(sliding_window))

cnt = 0
last = None
for e in sums:
    if last is None:
        last = e
        continue

    if e > last:
        cnt += 1
    last = e

print(cnt)