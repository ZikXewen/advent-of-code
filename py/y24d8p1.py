from sys import stdin
from collections import defaultdict

v = defaultdict(list)
for i, line in enumerate(stdin):
  mi = i
  mj = len(line) - 2
  for j, c in enumerate(line):
    if c != "." and c != "\n":
      v[c].append((i, j))
print(
  len(
    {
      (2 * x1 - x2, 2 * y1 - y2)
      for items in v.values()
      for x1, y1 in items
      for x2, y2 in items
      if (x1 != x2 or y1 != y2)
      and 0 <= 2 * x1 - x2 <= mi
      and 0 <= 2 * y1 - y2 <= mj
    }
  )
)
