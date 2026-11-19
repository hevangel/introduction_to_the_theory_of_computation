"""Ch 7: Time complexity — P (2-SAT via Tarjan SCC), NP verifiers (VERTEX-COVER, CLIQUE, HAMPATH),
pseudo-polynomial DP (SUBSET-SUM), and polynomial-time reductions
(CLIQUE <=_P VERTEX-COVER, 3SAT <=_P SUBSET-SUM)."""


def verifies_vertex_cover(n: int, edges: list[tuple[int, int]], cover: list[int]) -> bool:
    """Polynomial-time verifier for VERTEX-COVER."""
    s = set(cover)
    return all(0 <= v < n for v in s) and all(u in s or v in s for u, v in edges)


def minimum_vertex_cover(n: int, edges: list[tuple[int, int]]) -> list[int]:
    """Exact minimum vertex cover by brute force (small graphs only)."""
    best = list(range(n))
    for mask in range(1 << n):
        cover = [v for v in range(n) if (mask >> v) & 1]
        if len(cover) < len(best) and verifies_vertex_cover(n, edges, cover):
            best = cover
    return best


def subset_sum_nonnegative(values: list[int], target: int) -> bool:
    """Pseudo-polynomial O(n * target) DP for SUBSET-SUM."""
    dp = [False] * (target + 1)
    dp[0] = True
    for x in values:
        for s in range(target, x - 1, -1):
            dp[s] = dp[s] or dp[s - x]
    return dp[target]


def verifies_clique(n: int, edges: list[tuple[int, int]], clique: list[int], k: int) -> bool:
    """Polynomial-time verifier for CLIQUE."""
    if len(clique) != k:
        return False
    s = set(clique)
    if len(s) != k or any(not (0 <= v < n) for v in s):
        return False
    edge_set = {(u, v) for u, v in edges} | {(v, u) for u, v in edges}
    return all((clique[i], clique[j]) in edge_set for i in range(k) for j in range(i + 1, k))


def verifies_hampath(
    n: int, edges: list[tuple[int, int]], s: int, t: int, path: list[int]
) -> bool:
    """Polynomial-time verifier for directed HAMPATH from s to t."""
    if len(path) != n or path[0] != s or path[-1] != t:
        return False
    if len(set(path)) != n or any(not (0 <= v < n) for v in path):
        return False
    edge_set = set(edges)
    return all((path[i], path[i + 1]) in edge_set for i in range(n - 1))


def two_sat_solve(num_vars: int, clauses: list[tuple[int, int]]) -> list[bool] | None:
    """Polynomial-time 2-SAT solver (2SAT ∈ P) via implication graph + Tarjan SCC."""
    def lit_node(lit: int) -> int:
        v = abs(lit) - 1
        return 2 * v if lit > 0 else 2 * v + 1

    total = 2 * num_vars
    adj: list[list[int]] = [[] for _ in range(total)]
    for a, b in clauses:
        na, nb = lit_node(a), lit_node(b)
        adj[na ^ 1].append(nb)
        adj[nb ^ 1].append(na)

    index = 0
    indices = [-1] * total
    lowlink = [0] * total
    on_stack = [False] * total
    stack: list[int] = []
    comp = [0] * total
    comp_count = 0

    def dfs(u: int) -> None:
        nonlocal index, comp_count
        indices[u] = lowlink[u] = index
        index += 1
        stack.append(u)
        on_stack[u] = True
        for v in adj[u]:
            if indices[v] == -1:
                dfs(v)
                lowlink[u] = min(lowlink[u], lowlink[v])
            elif on_stack[v]:
                lowlink[u] = min(lowlink[u], indices[v])
        if lowlink[u] == indices[u]:
            while True:
                w = stack.pop()
                on_stack[w] = False
                comp[w] = comp_count
                if w == u:
                    break
            comp_count += 1

    for u in range(total):
        if indices[u] == -1:
            dfs(u)

    assignment = [False] * num_vars
    for v in range(num_vars):
        if comp[2 * v] == comp[2 * v + 1]:
            return None
        assignment[v] = comp[2 * v] < comp[2 * v + 1]
    return assignment


def clique_to_vertex_cover(
    n: int, edges: list[tuple[int, int]], k: int
) -> tuple[int, list[tuple[int, int]], int]:
    """Polynomial-time reduction CLIQUE <=_P VERTEX-COVER via complement graph."""
    edge_set = {(min(u, v), max(u, v)) for u, v in edges}
    comp_edges = [
        (u, v) for u in range(n) for v in range(u + 1, n) if (u, v) not in edge_set
    ]
    return n, comp_edges, max(0, n - k)


def three_sat_to_subset_sum(
    num_vars: int, clauses: list[tuple[int, int, int]]
) -> tuple[list[int], int]:
    """Sipser Theorem 7.56: table-based polynomial-time reduction 3SAT <=_P SUBSET-SUM."""
    l, k = num_vars, len(clauses)
    total_cols = l + k

    def col_pow(col: int) -> int:
        return 10 ** (total_cols - 1 - col)

    numbers: list[int] = []
    for i in range(l):
        var_id = i + 1
        y_i = col_pow(i)
        z_i = col_pow(i)
        for j, clause in enumerate(clauses):
            if var_id in clause:
                y_i += col_pow(l + j)
            if -var_id in clause:
                z_i += col_pow(l + j)
        numbers.extend([y_i, z_i])
    for j in range(k):
        slack = col_pow(l + j)
        numbers.extend([slack, slack])

    target = sum(col_pow(i) for i in range(l)) + sum(3 * col_pow(l + j) for j in range(k))
    return numbers, target
