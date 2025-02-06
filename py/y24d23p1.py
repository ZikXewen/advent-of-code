from sys import stdin
from collections import defaultdict

g = defaultdict(set)
for line in stdin:
  x, y = sorted(line.strip().split("-"))
  g[x].add(y)

print(
  len(
    [
      (x, y, z)
      for x, d in g.items()
      for y in d
      for z in d
      if y < z and y in g and z in g[y] and "t" in [x[0], y[0], z[0]]
    ]
  )
)
