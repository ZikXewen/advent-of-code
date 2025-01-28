from sys import stdin

v = [list(line.strip()) for line in stdin]
n, m = len(v), len(v[0])
vst = [[False] * m for _ in range(n)]
ans = 0
dir = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def val(i, j, x):
  return 0 <= i < n and 0 <= j < m and v[i][j] == x


def flood(i, j):
  area, peri = 1, 0
  vst[i][j] = True
  for di, dj in dir:
    ni, nj = i + di, j + dj
    if val(ni, nj, v[i][j]) and not vst[ni][nj]:
      a, p = flood(ni, nj)
      area += a
      peri += p
  for d1 in range(4):
    d2 = (d1 + 1) % 4
    c1 = val(i + dir[d1][0], j + dir[d1][1], v[i][j])
    c2 = val(i + dir[d2][0], j + dir[d2][1], v[i][j])
    c3 = val(i + dir[d1][0] + dir[d2][0], j + dir[d1][1] + dir[d2][1], v[i][j])
    peri += not c1 and not c2
    peri += c1 and c2 and not c3
  return area, peri


for i in range(n):
  for j in range(m):
    if not vst[i][j]:
      a, p = flood(i, j)
      ans += a * p

print(ans)
