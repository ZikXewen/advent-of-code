from sys import stdin
from queue import Queue

size = 70

_v = [["."] * (size + 1) for _ in range(size + 1)]
for line in stdin:
  i, j = [int(x) for x in line.split(",")]
  _v[i][j] = "#"
  v = [[c for c in x] for x in _v]
  found = False
  q = Queue()
  q.put((0, 0, 0))
  while not q.empty():
    i, j, k = q.get()
    if (i, j) == (size, size):
      found = True
      break
    for di, dj in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
      ni, nj = i + di, j + dj
      if 0 <= ni <= size and 0 <= nj <= size and v[ni][nj] == ".":
        v[ni][nj] = "O"
        q.put((ni, nj, k + 1))
  if not found:
    print(line)
    break
