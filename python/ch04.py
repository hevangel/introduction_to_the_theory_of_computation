"""Ch 4: Decidable languages — A_DFA, E_DFA, EQ_DFA, E_CFG, and A_CFG."""
from collections import deque
from python.ch01 import Dfa
from python.ch02 import Cfg


def dfa_equivalent(
    a: list[tuple[int, int]],
    af: list[bool],
    b: list[tuple[int, int]],
    bf: list[bool],
) -> bool:
    """Decide equivalence of two binary-alphabet complete DFAs starting at state 0."""
    seen = {(0, 0)}
    q = deque([(0, 0)])
    while q:
        x, y = q.popleft()
        if af[x] != bf[y]:
            return False
        for sym in (0, 1):
            nxt = (a[x][sym], b[y][sym])
            if nxt not in seen:
                seen.add(nxt)
                q.append(nxt)
    return True


def a_dfa(dfa: Dfa, word: str) -> bool:
    """Decider for A_DFA (Sipser Theorem 4.1)."""
    return dfa.accepts(word)


def e_dfa(dfa: Dfa) -> bool:
    """Decider for E_DFA (Sipser Theorem 4.4): L(A) = ∅ iff no accept state is reachable."""
    return all(s not in dfa.accept for s in dfa.reachable_states())


def eq_dfa(a: Dfa, b: Dfa) -> bool:
    """Decider for EQ_DFA (Sipser Theorem 4.5) via symmetric difference + E_DFA."""
    a_minus_b = a.product(b.complement(), is_intersection=True)
    b_minus_a = a.complement().product(b, is_intersection=True)
    sym_diff = a_minus_b.product(b_minus_a, is_intersection=False)
    return e_dfa(sym_diff)


def e_cfg(cfg: Cfg) -> bool:
    """Decider for E_CFG (Sipser Theorem 4.8): mark generating variables bottom-up."""
    generating: set[str] = set()
    changed = True
    while changed:
        changed = False
        for p in cfg.productions:
            if p.lhs in generating:
                continue
            if all(sym.kind == "term" or sym.name in generating for sym in p.rhs):
                generating.add(p.lhs)
                changed = True
    return cfg.start not in generating


def a_cfg(cfg: Cfg, tokens: list[str]) -> bool:
    """Decider for A_CFG (Sipser Theorem 4.7) via CNF + CYK."""
    return cfg.to_cnf().accepts(tokens)
