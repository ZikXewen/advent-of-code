from sys import stdin
from collections import defaultdict

g = defaultdict(set)
an = set()


def bron(r, p, x):
  global an
  if len(p) == len(x) == 0:
    if len(r) > len(an):
      an = r
    return
  for v in p.copy():
    bron(r | set([v]), p & g[v], x & g[v])
    p.remove(v)
    x.add(v)


for line in stdin:
  x, y = line.strip().split("-")
  g[x].add(y)
  g[y].add(x)
bron(set(), set(g.keys()), set())
print(*sorted(an), sep=",")
