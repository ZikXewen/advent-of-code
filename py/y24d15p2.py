from sys import stdin

mp, mv = stdin.read().split("\n\n")
for i, j in [("#", "##"), (".", ".."), ("@", "@."), ("O", "[]")]:
  mp = mp.replace(i, j)
mp = [list(x) for x in mp.split("\n")]
x, y = next((i, j) for i, v in enumerate(mp) for j, x in enumerate(v) if x == "@")
dir = {"^": (-1, 0), "v": (1, 0), "<": (0, -1), ">": (0, 1)}
st = set()
dx, dy = 0, 0


def fill(x, y):
  if (x, y) in st:
    return 1
  match mp[x][y]:
    case "#":
      return 0
    case ".":
      return 1
    case "[":
      st.add((x, y))
      return fill(x, y + 1) and fill(x + dx, y + dy)
    case "]":
      st.add((x, y))
      return fill(x, y - 1) and fill(x + dx, y + dy)


def get_key(tup):
  return tup[0] * dx + tup[1] * dy


for c in mv:
  if c in dir:
    dx, dy = dir[c]
    st = {(x, y)}
    if fill(x + dx, y + dy):
      for i, j in sorted(list(st), key=get_key, reverse=1):
        mp[i + dx][j + dy] = mp[i][j]
        mp[i][j] = "."
      x += dx
      y += dy

print(sum(100 * i + j for i, v in enumerate(mp) for j, x in enumerate(v) if x == "["))
