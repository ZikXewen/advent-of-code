from sys import stdin

keys, locks = [], []
for arr in [
  [list(line) for line in chunk.split("\n")]
  for chunk in stdin.read().strip().split("\n\n")
]:
  nx = [(next(j for j in range(6) if arr[j + 1][i] != arr[j][i])) for i in range(5)]
  if arr[0][0] == ".":
    keys.append(nx)
  else:
    locks.append(nx)

print(sum(1 for x in keys for y in locks if all(x[i] - y[i] >= 0 for i in range(5))))
