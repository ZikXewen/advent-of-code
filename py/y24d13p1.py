from re import findall
from sys import stdin

print(
  int(
    sum(
      [
        3 * i + j
        for i, j in [
          ((d * x - c * y) / (a * d - b * c), (a * y - b * x) / (a * d - b * c))
          for a, b, c, d, x, y in [
            [int(x) for x in findall("\\d+", s)] for s in stdin.read().split("\n\n")
          ]
        ]
        if i.is_integer() and j.is_integer()
      ]
    )
  )
)
