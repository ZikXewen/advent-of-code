from sys import stdin

pat, des = stdin.read().strip().split("\n\n")
pat = pat.split(", ")


def rec(line, cur=0, mem=None):
  if mem is None:
    mem = [-1] * len(line)
  if cur == len(line):
    return 1
  if mem[cur] == -1:
    mem[cur] = any(
      (nxt := cur + len(s)) <= len(line) and line[cur:nxt] == s and rec(line, nxt, mem)
      for s in pat
    )
  return mem[cur]


print(sum(map(rec, des.split("\n"))))
