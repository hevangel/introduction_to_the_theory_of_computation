"""Ch 8: Space complexity — Savitch's theorem (CAN-YIELD), TQBF (PSPACE-complete),
and Immerman–Szelepcsényi inductive counting (NL = coNL)."""
from dataclasses import dataclass


def reach_within(graph: list[list[int]], u: int, v: int, steps: int) -> bool:
    """Savitch's divide-and-conquer CAN-YIELD reachability (Sipser Theorem 8.5)."""
    if u == v:
        return True
    if steps == 0:
        return False
    if steps == 1:
        return v in graph[u]
    half = steps // 2
    return any(
        reach_within(graph, u, mid, half) and reach_within(graph, mid, v, steps - half)
        for mid in range(len(graph))
    )


@dataclass(frozen=True)
class QVar:
    name: str


@dataclass(frozen=True)
class QNot:
    inner: object


@dataclass(frozen=True)
class QAnd:
    left: object
    right: object


@dataclass(frozen=True)
class QOr:
    left: object
    right: object


@dataclass(frozen=True)
class QForAll:
    var: str
    body: object


@dataclass(frozen=True)
class QExists:
    var: str
    body: object


def eval_tqbf(formula: object) -> bool:
    """Polynomial-space recursive decider for TQBF (Sipser Theorem 8.9)."""
    env: dict[str, bool] = {}

    def _eval(f: object) -> bool:
        if isinstance(f, QVar):
            return env[f.name]
        if isinstance(f, QNot):
            return not _eval(f.inner)
        if isinstance(f, QAnd):
            return _eval(f.left) and _eval(f.right)
        if isinstance(f, QOr):
            return _eval(f.left) or _eval(f.right)
        if isinstance(f, QForAll):
            for val in (False, True):
                env[f.var] = val
                if not _eval(f.body):
                    return False
            return True
        if isinstance(f, QExists):
            for val in (False, True):
                env[f.var] = val
                if _eval(f.body):
                    return True
            return False
        raise TypeError(f"Unsupported QBF node: {f!r}")

    return _eval(formula)


def immerman_szelepcsenyi_non_reach(graph: list[list[int]], start: int, target: int) -> bool:
    """Immerman–Szelepcsényi inductive counting for non-reachability (NL = coNL, Theorem 8.27)."""
    n = len(graph)
    if n == 0 or not (0 <= start < n and 0 <= target < n):
        return True
    c_prev = 1
    for i in range(1, n):
        c_cur = 0
        for v in range(n):
            seen_prev = 0
            v_reach = False
            for u in range(n):
                if reach_within(graph, start, u, i - 1):
                    seen_prev += 1
                    if u == v or v in graph[u]:
                        v_reach = True
            assert seen_prev == c_prev
            if v_reach:
                c_cur += 1
        c_prev = c_cur

    count = 0
    for u in range(n):
        if reach_within(graph, start, u, n - 1):
            if u == target:
                return False
            count += 1
    return count == c_prev
