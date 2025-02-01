from sys import stdin
from queue import Queue

size = 70
lim = 1024

v = [["."] * (size + 1) for _ in range(size + 1)]
for [i, j] in [[int(x) for x in line.split(",")] for line in stdin][:lim]:
  v[i][j] = "#"
q = Queue()
q.put((0, 0, 0))
while not q.empty():
  i, j, k = q.get()
  if (i, j) == (size, size):
    print(k)
    break
  for di, dj in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
    ni, nj = i + di, j + dj
    if 0 <= ni <= size and 0 <= nj <= size and v[ni][nj] == ".":
      v[ni][nj] = "O"
      q.put((ni, nj, k + 1))
