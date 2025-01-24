from sys import stdin

v = [[int(c) for c in line.strip()] for line in stdin]
mem = set()
ans = 0


def dfs(i, j):
  if (i, j) in mem:
    return 0
  mem.add((i, j))
  if v[i][j] == 9:
    return 1
  res = 0
  for di, dj in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
    if (
      i + di in range(len(v))
      and j + dj in range(len(v[0]))
      and v[i + di][j + dj] == v[i][j] + 1
    ):
      res += dfs(i + di, j + dj)
  return res


for i in range(len(v)):
  for j in range(len(v[0])):
    if not v[i][j]:
      mem = set()
      ans += dfs(i, j)

print(ans)
