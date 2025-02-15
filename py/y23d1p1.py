from sys import stdin

print(
  sum(
    int(
      next(c for c in line if c.isdigit()) + next(c for c in line[::-1] if c.isdigit())
    )
    for line in stdin
  )
)
