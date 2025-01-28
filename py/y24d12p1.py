from sys import stdin

v = [list(line.strip()) for line in stdin]
n, m = len(v), len(v[0])
vst = [[False] * m for _ in range(n)]
ans = 0


def flood(i, j):
  area, peri = 1, 0
  vst[i][j] = True
  for di, dj in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
    ni, nj = i + di, j + dj
    if 0 <= ni < n and 0 <= nj < m and v[i][j] == v[ni][nj]:
      if not vst[ni][nj]:
        a, p = flood(ni, nj)
        area += a
        peri += p
    else:
      peri += 1
  return area, peri


for i in range(n):
  for j in range(m):
    if not vst[i][j]:
      a, p = flood(i, j)
      ans += a * p

print(ans)
