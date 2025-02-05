from sys import stdin
from functools import reduce

MOD = 16777216


def cal(x):
  x = ((x << 6) ^ x) % MOD
  x = ((x >> 5) ^ x) % MOD
  x = ((x << 11) ^ x) % MOD
  return x


print(sum(reduce(lambda acc, _: cal(acc), range(2000), int(x)) for x in stdin))
