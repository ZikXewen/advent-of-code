from sys import stdin

v = [int(c) * [-1 if i % 2 else (i // 2)] for i, c in enumerate(stdin.read().strip())]
v = [y for x in v for y in x]
j = len(v) - 1
ans = 0
for i, c in enumerate(v):
  while v[j] == -1:
    j -= 1
  if i > j:
    break
  if c != -1:
    ans += i * c
  else:
    ans += i * v[j]
    j -= 1
print(ans)
