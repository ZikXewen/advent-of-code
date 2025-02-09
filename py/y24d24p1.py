from sys import stdin
from operator import and_, or_, xor

init, gates = stdin.read().strip().split("\n\n")
mp = {x: int(y) for s in init.split("\n") for x, y in [s.split(": ")]} | {
  z: (x, o, y) for s in gates.split("\n") for x, o, y, _, z in [s.split(" ")]
}
mx = max(int(z[1:]) for s in gates.split("\n") if (z := s.split(" ")[4])[0] == "z")
op = {"AND": and_, "OR": or_, "XOR": xor}


def calc(z):
  if not isinstance(mp[z], int):
    x, o, y = mp[z]
    mp[z] = op[o](calc(x), calc(y))
  return mp[z]


print(sum(calc(f"z{z:02}") << z for z in range(mx + 1)))
