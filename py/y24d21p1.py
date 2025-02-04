from sys import stdin


# < v ^ > unless blocked
# replaces ^ with 0
def draw(p):
  (sy, sx), (ey, ex) = [divmod("<v> 0A123456789".find(c), 3) for c in p]
  ret = "<" * (sx - ex) + "v" * (sy - ey) + "0" * (ey - sy) + ">" * (ex - sx)
  return (ret[::-1] if (sy, ex) == (1, 0) or (ey, sx) == (1, 0) else ret) + "A"


def rec(s, d):
  if d == 0:
    return len(s)
  return sum(rec(draw(p), d - 1) for p in zip("A" + s, s))


print(sum(rec(x.strip(), 3) * int(x[:3]) for x in stdin))
