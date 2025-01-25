from sys import stdin

mem = {}


def rec(x, i):
  if i == 0:
    return 1
  if x == 0:
    return rec(1, i - 1)
  if (x, i) in mem:
    return mem[(x, i)]
  mem[(x, i)] = (
    rec(2024 * x, i - 1)
    if len(str(x)) % 2
    else (
      rec(int(str(x)[: len(str(x)) // 2]), i - 1)
      + rec(int(str(x)[len(str(x)) // 2 :]), i - 1)
    )
  )
  return mem[(x, i)]


print(sum([rec(int(x), 75) for x in stdin.read().strip().split()]))
