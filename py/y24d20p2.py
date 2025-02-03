from sys import stdin
from queue import Queue

mp = [list(line) for line in stdin]
n, m = len(mp), len(mp[0])


def dis(sx, sy):
  ds = [[-1] * m for _ in range(n)]
  q = Queue()
  q.put((sx, sy, 0))
  ds[sx][sy] = 0
  while not q.empty():
    x, y, w = q.get()
    for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
      nx, ny = x + dx, y + dy
      if mp[nx][ny] != "#" and ds[nx][ny] == -1:
        ds[nx][ny] = w + 1
        q.put((nx, ny, w + 1))
  return ds


ds = dis(*next((i, j) for i, v in enumerate(mp) for j, c in enumerate(v) if c == "S"))
de = dis(*next((i, j) for i, v in enumerate(mp) for j, c in enumerate(v) if c == "E"))

mn = min(
  ds[i][j] + de[i][j]
  for i in range(n)
  for j in range(m)
  if ds[i][j] != -1 and de[i][j] != -1
)

print(
  sum(
    1
    for x in range(n)
    for y in range(m)
    if ds[x][y] != -1
    for dx in range(-20, 21)
    for dy in range(abs(dx) - 20, 21 - abs(dx))
    if (
      0 <= (nx := x + dx) < n
      and 0 <= (ny := y + dy) < m
      and de[nx][ny] != -1
      and ds[x][y] + de[nx][ny] + 100 + abs(dx) + abs(dy) <= mn
    )
  )
)
