from sys import stdin
from re import findall

ex = "one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9"
mp = {x: (i % 9) + 1 for i, x in enumerate(ex.split("|"))}

print(
  sum(
    10 * mp[x[0]] + mp[x[-1]] for line in stdin for x in [findall(f"(?=({ex}))", line)]
  )
)
