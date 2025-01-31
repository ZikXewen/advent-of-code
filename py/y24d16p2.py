from sys import stdin
import heapq

mp = [list(x) for x in stdin]

ds = [[[-1] * 4 for _ in range(len(mp[0]))] for _ in range(len(mp))]
sx, sy = next((x, y) for x, v in enumerate(mp) for y, c in enumerate(v) if c == "S")
dir = [(0, 1), (1, 0), (0, -1), (-1, 0)]
q = [(0, sx, sy, 0)]
while len(q):
  w, x, y, d = heapq.heappop(q)
  if ds[x][y][d] != -1:
    continue
  ds[x][y][d] = w
  nx, ny = x + dir[d][0], y + dir[d][1]
  if ds[nx][ny][d] == -1 and mp[nx][ny] != "#":
    heapq.heappush(q, (w + 1, nx, ny, d))
  for i in range(4):
    if ds[x][y][i] == -1:
      heapq.heappush(q, (w + 1000, x, y, i))

de = [[[-1] * 4 for _ in range(len(mp[0]))] for _ in range(len(mp))]
ex, ey = next((x, y) for x, v in enumerate(mp) for y, c in enumerate(v) if c == "E")
q = [(0, ex, ey, i) for i in range(4)]
while len(q):
  w, x, y, d = heapq.heappop(q)
  if de[x][y][d] != -1:
    continue
  de[x][y][d] = w
  nx, ny = x + dir[d][0], y + dir[d][1]
  if de[nx][ny][d] == -1 and mp[nx][ny] != "#":
    heapq.heappush(q, (w + 1, nx, ny, d))
  for i in range(4):
    if de[x][y][i] == -1:
      heapq.heappush(q, (w + 1000, x, y, i))

mn = ds[sx][sy][0] + de[sx][sy][2]
ct = 0
for i in range(len(mp)):
  for j in range(len(mp[0])):
    for k in range(4):
      if ds[i][j][k] == -1:
        continue
      d = ds[i][j][k] + de[i][j][(k + 2) % 4]
      if d < mn:
        mn = d
        ct = 1
        break
      elif d == mn:
        ct += 1
        break
print(ct)
