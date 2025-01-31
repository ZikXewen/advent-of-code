# Reversed:
# B = A % 8
# B = B ^ 1
# C = A >> B
# A = A >> 3
# B = B ^ 4
# B = B ^ C
# print B % 8
# if A goto 0

# Translated:
# Find A such that
# while A
#   print(((A % 8) ^ 5 ^ (A >> ((A % 8) ^ 1))) % 8)
#   A >>= 3
# outputs 2,4,1,1,7,5,0,3,1,4,4,0,5,5,3,0

from re import findall
from sys import stdin

v = [int(x) for x in findall("\\d+", stdin.read())][3:]


def rec(va, po):
  if po < 0:
    return va
  for i in range(8):
    A = (va << 3) + i
    if ((A % 8) ^ 5 ^ (A >> ((A % 8) ^ 1))) % 8 == v[po] and (ret := rec(A, po - 1)):
      return ret
  return 0


print(rec(0, len(v) - 1))
