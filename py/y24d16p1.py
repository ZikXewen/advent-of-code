from sys import stdin
import heapq

mp = [list(x) for x in stdin]
vst = [[False] * len(mp[0]) for _ in range(len(mp))]
sx, sy = next((x, y) for x, v in enumerate(mp) for y, c in enumerate(v) if c == "S")
dir = [(0, 1), (1, 0), (0, -1), (-1, 0)]
q = [(0, sx, sy, 0)]
while len(q):
  w, x, y, d = heapq.heappop(q)
  if vst[x][y]:
    continue
  vst[x][y] = True
  if mp[x][y] == "E":
    print(w)
    break
  for i in range(4):
    nx, ny = x + dir[i][0], y + dir[i][1]
    if not vst[nx][ny] and mp[nx][ny] != "#":
      heapq.heappush(q, (w + (1 if i == d else 1001), nx, ny, i))
