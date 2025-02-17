from sys import stdin
from re import findall

print(
  sum(
    max(int(x) for x in findall("\\d+(?= red)", line))
    * max(int(x) for x in findall("\\d+(?= green)", line))
    * max(int(x) for x in findall("\\d+(?= blue)", line))
    for i, line in enumerate(stdin)
  )
)
