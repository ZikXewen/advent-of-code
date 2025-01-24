from sys import stdin

v = [(int(c), -1 if i % 2 else (i // 2)) for i, c in enumerate(stdin.read().strip())]
for i in range(len(v))[::-1]:
  c, x = v[i]
  if x != -1:
    for j in range(i):
      d, y = v[j]
      if y == -1 and d >= c:
        v[i] = c, -1
        v[j] = c, x
        v.insert(j + 1, (d - c, -1))
        break
v = [c * [x] for c, x in v]
v = [x for y in v for x in y]
print(sum([(c != -1) * i * c for i, c in enumerate(v)]))
