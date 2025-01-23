from sys import stdin

ans = 0
for line in stdin:
  res, first, *rest = map(int, line.replace(":", "").split())
  all = [first]
  for nx in rest:
    all = (
      [x + nx for x in all]
      + [x * nx for x in all]
      + [int(str(x) + str(nx)) for x in all]
    )
  if res in all:
    ans += res

print(ans)
