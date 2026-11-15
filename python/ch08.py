"""Chapter 8: a recursive midpoint version of Savitch reachability."""
def reach_within(graph: list[list[int]], u: int, v: int, steps: int) -> bool:
    if steps == 0:
        return u == v
    if steps == 1:
        return u == v or v in graph[u]
    left = steps // 2
    return any(reach_within(graph, u, middle, left) and
               reach_within(graph, middle, v, steps - left)
               for middle in range(len(graph)))
