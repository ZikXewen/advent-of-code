from re import findall
from sys import stdin

x, y, t = 101, 103, 100
q1, q2, q3, q4 = 0, 0, 0, 0
for i, j in [
  ((px + dx * t) % x, (py + dy * t) % y)
  for px, py, dx, dy in [[int(x) for x in findall("-?\\d+", s.strip())] for s in stdin]
]:
  if i < x // 2 and j < y // 2:
    q1 += 1
  elif i < x // 2 and j > y // 2:
    q2 += 1
  elif i > x // 2 and j < y // 2:
    q3 += 1
  elif i > x // 2 and j > y // 2:
    q4 += 1
print(q1 * q2 * q3 * q4)
