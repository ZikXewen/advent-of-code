from sys import stdin
from re import findall

print(
  sum(
    i + 1
    for i, line in enumerate(stdin)
    if all(int(x) <= 12 for x in findall("\\d+(?= red)", line))
    and all(int(x) <= 13 for x in findall("\\d+(?= green)", line))
    and all(int(x) <= 14 for x in findall("\\d+(?= blue)", line))
  )
)
