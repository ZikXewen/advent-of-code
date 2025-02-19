from sys import stdin

cs = [list(line.strip()) for line in stdin]
n, m, ans = len(cs), len(cs[0]), 0

for i in range(n):
  j = 0
  while (j := next((x for x in range(j, m) if cs[i][x].isdigit()), None)) is not None:
    k = next((k for k in range(j, m) if not cs[i][k].isdigit()), m)
    if any(
      True
      for u in range(i - 1, i + 2)
      for v in range(j - 1, k + 1)
      if 0 <= u < n and 0 <= v < m and cs[u][v] != "." and not cs[u][v].isdigit()
    ):
      ans += int("".join(cs[i][j:k]))
    j = k
print(ans)
