from sys import stdin
from operator import sub
from collections import defaultdict
# from itertools import accumulate

MOD = 16777216


def cal(x):
  x = ((x << 6) ^ x) % MOD
  x = ((x >> 5) ^ x) % MOD
  x = ((x << 11) ^ x) % MOD
  return x


ans = defaultdict(int)
for x in stdin:
  arr = [(x := int(x)) % 10] + [(x := cal(x)) % 10 for _ in range(2000)]
  # arr = list(accumulate(range(2000), lambda acc, _: cal(acc), initial=int(x)))
  dif = list(map(sub, arr, arr[1:]))
  mem = set()
  for i in range(1997):
    if (k := tuple(dif[i : i + 4])) not in mem:
      mem.add(k)
      ans[k] += arr[i + 4] % 10
print(max(ans.values()))
