from re import findall
from sys import stdin
from time import sleep

x, y, t = 101, 103, 100
st = []
v = [[int(x) for x in findall("-?\\d+", s.strip())] for s in stdin]
for t in range(x * y):
  q1, q2, q3, q4 = 0, 0, 0, 0
  for px, py, dx, dy in v:
    i, j = (px + dx * t) % x, (py + dy * t) % y
    if i < x // 2 and j < y // 2:
      q1 += 1
    elif i < x // 2 and j > y // 2:
      q2 += 1
    elif i > x // 2 and j < y // 2:
      q3 += 1
    elif i > x // 2 and j > y // 2:
      q4 += 1
  st.append((q1 * q2 * q3 * q4, t))
st.sort()
for _, t in st:
  v2 = [((px + dx * t) % x, (py + dy * t) % y) for px, py, dx, dy in v]
  arr = [[0] * y for _ in range(x)]
  for i, j in v2:
    arr[i][j] += 1
  print(t)
  for j in range(y):
    for i in range(x):
      print(arr[i][j], end="")
    print("")
  sleep(1)
