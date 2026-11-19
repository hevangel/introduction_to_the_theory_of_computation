"""Ch 5: Reducibility — mapping reductions (3COLOR -> SAT, 3SAT -> CLIQUE),
Linear Bounded Automaton (A_LBA decider via cycle detection), and Bounded PCP."""
from python.ch03 import Move, TuringMachine


def three_color_to_cnf(n: int, edges: list[tuple[int, int]]) -> list[list[int]]:
    """Mapping reduction: 3-COLORING <=_m SAT."""
    def var(v: int, c: int) -> int:
        return 3 * v + c + 1

    clauses: list[list[int]] = []
    for v in range(n):
        clauses.append([var(v, 0), var(v, 1), var(v, 2)])
        for a in range(3):
            for b in range(a + 1, 3):
                clauses.append([-var(v, a), -var(v, b)])
    for u, v in edges:
        for c in range(3):
            clauses.append([-var(u, c), -var(v, c)])
    return clauses


def brute_sat(cnf: list[list[int]], variables: int) -> bool:
    """Brute-force SAT solver for small formulas."""
    for mask in range(1 << variables):
        if all(
            any((((mask >> (abs(lit) - 1)) & 1) == 1) == (lit > 0) for lit in clause)
            for clause in cnf
        ):
            return True
    return False


def lba_accepts(tm: TuringMachine, word: str) -> bool:
    """Decider for A_LBA (Sipser Theorem 5.9) via configuration cycle detection."""
    tape = list(word) if word else [tm.blank]
    n = len(tape)
    state = tm.start
    head = 0
    visited: set[tuple[int, int, tuple[str, ...]]] = set()
    while True:
        if state == tm.accept:
            return True
        if state == tm.reject:
            return False
        cfg = (state, head, tuple(tape))
        if cfg in visited:
            return False  # Guaranteed infinite loop
        visited.add(cfg)
        action = tm.transitions.get((state, tape[head]))
        if action is None:
            return False
        next_state, write_sym, move = action
        tape[head] = write_sym
        state = next_state
        if move is Move.LEFT and head > 0:
            head -= 1
        elif move is Move.RIGHT and head + 1 < n:
            head += 1


def bounded_pcp(dominoes: list[tuple[str, str]], max_len: int) -> list[int] | None:
    """Bounded PCP solver up to `max_len` dominoes (unbounded PCP is undecidable)."""
    def dfs(seq: list[int], top: str, bot: str) -> list[int] | None:
        if seq and top == bot:
            return list(seq)
        if len(seq) == max_len:
            return None
        if not (top.startswith(bot) or bot.startswith(top)):
            return None
        for i, (t, b) in enumerate(dominoes):
            seq.append(i)
            res = dfs(seq, top + t, bot + b)
            if res is not None:
                return res
            seq.pop()
        return None

    return dfs([], "", "")


def three_sat_to_clique(
    clauses: list[tuple[int, int, int]],
) -> tuple[int, list[tuple[int, int]], int]:
    """Mapping reduction 3SAT <=_m CLIQUE (Sipser Theorem 7.32)."""
    k = len(clauses)
    edges: list[tuple[int, int]] = []
    for c1 in range(k):
        for l1 in range(3):
            u = 3 * c1 + l1
            for c2 in range(c1 + 1, k):
                for l2 in range(3):
                    v = 3 * c2 + l2
                    if clauses[c1][l1] != -clauses[c2][l2]:
                        edges.append((u, v))
    return 3 * k, edges, k
