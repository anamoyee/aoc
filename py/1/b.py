import pathlib as p

import tcrutils as tcr
from tcrutils import c

s = p.Path("../input.txt").read_text()

s = s.strip()

list_of_pairs: list[tuple[int, int]] = [tuple(int(x) for x in line.split(" ", maxsplit=1)) for line in s.split("\n")]

l, r = zip(*list_of_pairs, strict=True)

l = list(l)
r = list(r)

l.sort()

lout = []

for n in l:
	lout.append(n * r.count(n))

c(lout)
c(sum(lout))
