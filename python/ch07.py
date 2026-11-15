"""Chapter 7: polynomial-time verification versus exponential search."""
from itertools import combinations

def verifies_vertex_cover(n: int, edges: list[tuple[int, int]],
                          cover: list[int]) -> bool:
    selected = set(cover)
    return all(0 <= v < n for v in selected) and all(
        0 <= u < n and 0 <= v < n and (u in selected or v in selected)
        for u, v in edges)

def minimum_vertex_cover(n: int, edges: list[tuple[int, int]]) -> list[int]:
    assert n <= 20
    for k in range(n + 1):
        for subset in combinations(range(n), k):
            if verifies_vertex_cover(n, edges, list(subset)):
                return list(subset)
    raise AssertionError("full vertex set always covers a valid graph")

def subset_sum_nonnegative(values: list[int], target: int) -> bool:
    assert target >= 0 and all(x >= 0 for x in values)
    reachable = [False] * (target + 1)
    reachable[0] = True
    for x in values:
        for t in range(target, x - 1, -1):
            reachable[t] |= reachable[t - x]
    return reachable[target]
