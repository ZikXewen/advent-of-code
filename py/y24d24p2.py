# Only generate throws for manual checking
from sys import stdin

gates = [
  (x, o, y, z)
  for line in stdin.read().strip().split("\n\n")[1].split("\n")
  for x, o, y, _, z in [line.split(" ")]
]


def find_gate(fx, fy, fo):
  for x, o, y, z in gates:
    if (x == fx and y == fy and o == fo) or (x == fy and y == fx and o == fo):
      return z
  raise Exception(fx, fy, fo)


if (z := find_gate("x00", "y00", "XOR")) != "z00":
  raise Exception(z)

c = find_gate("x00", "y00", "AND")
for i in range(1, 45):
  s1 = find_gate(f"x{i:02}", f"y{i:02}", "XOR")
  s2 = find_gate(f"x{i:02}", f"y{i:02}", "AND")
  s3 = find_gate(c, s1, "AND")
  z = find_gate(c, s1, "XOR")
  c = find_gate(s2, s3, "OR")

ans = []  # MANUAL
print(*sorted(ans), sep=",")
