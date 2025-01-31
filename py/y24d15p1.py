from sys import stdin

mp, mv = stdin.read().split("\n\n")
mp = [list(x) for x in mp.split("\n")]
x, y = next((i, j) for i, v in enumerate(mp) for j, x in enumerate(v) if x == "@")
dir = {"^": (-1, 0), "v": (1, 0), "<": (0, -1), ">": (0, 1)}

for c in mv:
  if c in dir:
    dx, dy = dir[c]
    nx, ny = next(
      (x + dx * i, y + dy * i)
      for i in range(1, len(mp))
      if mp[x + dx * i][y + dy * i] != "O"
    )
    if mp[nx][ny] == ".":
      mp[nx][ny] = "O"
      mp[x][y] = "."
      mp[x := x + dx][y := y + dy] = "@"
print(sum(100 * i + j for i, v in enumerate(mp) for j, x in enumerate(v) if x == "O"))
