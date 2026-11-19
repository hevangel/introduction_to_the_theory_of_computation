"""Ch 0: Mathematical foundations — sets, relations, graphs, and diagonalization."""
from collections import deque
from typing import TypeVar

T = TypeVar("T")


def reachable(graph: list[list[int]], start: int) -> list[bool]:
    """Breadth-first reachability in a directed graph."""
    seen = [False] * len(graph)
    if not (0 <= start < len(graph)):
        return seen
    q = deque([start])
    seen[start] = True
    while q:
        u = q.popleft()
        for v in graph[u]:
            if 0 <= v < len(graph) and not seen[v]:
                seen[v] = True
                q.append(v)
    return seen


def power_set(items: list[T]) -> list[list[T]]:
    """Enumerate all 2^|A| subsets of a finite list."""
    n = len(items)
    result: list[list[T]] = []
    for mask in range(1 << n):
        result.append([items[i] for i in range(n) if mask & (1 << i)])
    return result


def is_equivalence_relation(n: int, pairs: list[tuple[int, int]]) -> bool:
    """Verify reflexivity, symmetry, and transitivity on {0, ..., n-1}."""
    mat = [[False] * n for _ in range(n)]
    for a, b in pairs:
        if not (0 <= a < n and 0 <= b < n):
            return False
        mat[a][b] = True
    if any(not mat[i][i] for i in range(n)):
        return False
    for i in range(n):
        for j in range(n):
            if mat[i][j] and not mat[j][i]:
                return False
    for i in range(n):
        for j in range(n):
            if mat[i][j]:
                for k in range(n):
                    if mat[j][k] and not mat[i][k]:
                        return False
    return True


def equivalence_classes(n: int, pairs: list[tuple[int, int]]) -> list[list[int]] | None:
    """Partition {0, ..., n-1} into equivalence classes if `pairs` is an equivalence relation."""
    if not is_equivalence_relation(n, pairs):
        return None
    adj: list[set[int]] = [set() for _ in range(n)]
    for a, b in pairs:
        adj[a].add(b)
    visited = [False] * n
    classes: list[list[int]] = []
    for i in range(n):
        if not visited[i]:
            cls = sorted(adj[i])
            for x in cls:
                visited[x] = True
            classes.append(cls)
    return classes


def cantor_diagonal_bitstring(rows: list[str]) -> str:
    """Return an n-bit string differing from rows[i] at bit i (Cantor diagonalization)."""
    out = []
    for i, row in enumerate(rows):
        ch = row[i] if i < len(row) else "0"
        out.append("1" if ch == "0" else "0")
    return "".join(out)


def has_cycle(graph: list[list[int]]) -> bool:
    """Detect a directed cycle via 3-color DFS."""
    color = [0] * len(graph)  # 0=unvisited, 1=visiting, 2=done

    def dfs(u: int) -> bool:
        color[u] = 1
        for v in graph[u]:
            if 0 <= v < len(graph):
                if color[v] == 1 or (color[v] == 0 and dfs(v)):
                    return True
        color[u] = 2
        return False

    return any(color[u] == 0 and dfs(u) for u in range(len(graph)))
