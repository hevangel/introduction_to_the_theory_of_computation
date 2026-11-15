"""Chapter 4: DFA language equivalence via finite product exploration."""
from collections import deque

def dfa_equivalent(a: list[tuple[int, int]], af: list[bool],
                   b: list[tuple[int, int]], bf: list[bool]) -> bool:
    assert a and b and len(a) == len(af) and len(b) == len(bf)
    q = deque([(0, 0)])
    seen = {(0, 0)}
    while q:
        x, y = q.popleft()
        if af[x] != bf[y]:
            return False
        for symbol in range(2):
            p = a[x][symbol], b[y][symbol]
            if p not in seen:
                seen.add(p)
                q.append(p)
    return True
