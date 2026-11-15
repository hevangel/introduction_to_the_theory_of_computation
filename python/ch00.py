"""Chapter 0: finite graph reachability as a proof by construction."""
from collections import deque

def reachable(graph: list[list[int]], start: int) -> list[bool]:
    seen = [False] * len(graph)
    seen[start] = True
    todo = deque([start])
    while todo:
        for nxt in graph[todo.popleft()]:
            if not seen[nxt]:
                seen[nxt] = True
                todo.append(nxt)
    return seen
