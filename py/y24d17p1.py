from re import findall
from sys import stdin

a, b, c, *v = [int(x) for x in findall("\\d+", stdin.read())]
pc, out = 0, []
while pc < len(v) - 1:
  match v[pc + 1]:
    case 4:
      cb = a
    case 5:
      cb = b
    case 6:
      cb = c
    case _:
      cb = v[pc + 1]
  match v[pc]:
    case 0:
      a = a >> cb
    case 1:
      b = b ^ v[pc + 1]
    case 2:
      b = cb % 8
    case 3:
      if a != 0:
        pc = v[pc + 1] - 2
    case 4:
      b = b ^ c
    case 5:
      out.append(cb % 8)
    case 6:
      b = a >> cb
    case 7:
      c = a >> cb
  pc += 2
print(*out, sep=',')
