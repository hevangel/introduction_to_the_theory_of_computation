"""Ch 1: Regular languages — DFA, ε-NFA, subset construction, product construction,
Myhill–Nerode minimization, and Thompson's regex-to-NFA compiler."""
from collections import deque
from dataclasses import dataclass


def contains_01(word: str) -> bool:
    """DFA recognizing binary words containing substring '01'."""
    state = 0
    table = {(0, "0"): 1, (0, "1"): 0, (1, "0"): 1, (1, "1"): 2, (2, "0"): 2, (2, "1"): 2}
    for ch in word:
        if (state, ch) not in table:
            return False
        state = table[(state, ch)]
    return state == 2


def ends_ab_nfa(word: str) -> bool:
    """NFA recognizing words over {a, b} ending in 'ab'."""
    states = {0}
    for ch in word:
        nxt: set[int] = set()
        for q in states:
            if (q, ch) == (0, "a"):
                nxt |= {0, 1}
            elif (q, ch) == (0, "b"):
                nxt.add(0)
            elif (q, ch) == (1, "b"):
                nxt.add(2)
        states = nxt
    return 2 in states


@dataclass
class Dfa:
    """General Deterministic Finite Automaton (Q, Σ, δ, q0, F)."""
    num_states: int
    alphabet: list[str]
    transitions: list[dict[str, int]]
    start: int
    accept: set[int]

    def accepts(self, word: str) -> bool:
        q = self.start
        for ch in word:
            if ch not in self.transitions[q]:
                return False
            q = self.transitions[q][ch]
        return q in self.accept

    def complement(self) -> "Dfa":
        return Dfa(
            num_states=self.num_states,
            alphabet=list(self.alphabet),
            transitions=[dict(t) for t in self.transitions],
            start=self.start,
            accept={s for s in range(self.num_states) if s not in self.accept},
        )

    def product(self, other: "Dfa", is_intersection: bool = True) -> "Dfa":
        assert self.alphabet == other.alphabet, "DFAs must share the same alphabet"
        n1, n2 = self.num_states, other.num_states
        transitions: list[dict[str, int]] = [{} for _ in range(n1 * n2)]
        accept: set[int] = set()
        for q1 in range(n1):
            for q2 in range(n2):
                sid = q1 * n2 + q2
                a1, a2 = q1 in self.accept, q2 in other.accept
                if (is_intersection and (a1 and a2)) or (not is_intersection and (a1 or a2)):
                    accept.add(sid)
                for sym in self.alphabet:
                    if sym in self.transitions[q1] and sym in other.transitions[q2]:
                        transitions[sid][sym] = self.transitions[q1][sym] * n2 + other.transitions[q2][sym]
        return Dfa(n1 * n2, list(self.alphabet), transitions, self.start * n2 + other.start, accept)

    def reachable_states(self) -> set[int]:
        seen = {self.start}
        q = deque([self.start])
        while q:
            u = q.popleft()
            for v in self.transitions[u].values():
                if v not in seen:
                    seen.add(v)
                    q.append(v)
        return seen

    def myhill_nerode_classes(self) -> list[list[int]]:
        """Table-filling Myhill–Nerode equivalence classes on reachable states."""
        reach = sorted(self.reachable_states())
        m = len(reach)
        idx = {s: i for i, s in enumerate(reach)}
        dist = [[False] * m for _ in range(m)]
        for i in range(m):
            for j in range(i + 1, m):
                if (reach[i] in self.accept) != (reach[j] in self.accept):
                    dist[i][j] = dist[j][i] = True
        changed = True
        while changed:
            changed = False
            for i in range(m):
                for j in range(i + 1, m):
                    if dist[i][j]:
                        continue
                    for sym in self.alphabet:
                        t1 = self.transitions[reach[i]].get(sym)
                        t2 = self.transitions[reach[j]].get(sym)
                        if (t1 is None) != (t2 is None) or (
                            t1 is not None and t2 is not None and dist[idx[t1]][idx[t2]]
                        ):
                            dist[i][j] = dist[j][i] = True
                            changed = True
                            break
        used = [False] * m
        classes: list[list[int]] = []
        for i in range(m):
            if not used[i]:
                cls = [reach[j] for j in range(i, m) if not dist[i][j]]
                for j in range(i, m):
                    if not dist[i][j]:
                        used[j] = True
                classes.append(cls)
        return classes

    def minimize(self) -> "Dfa":
        """Collapse Myhill–Nerode equivalence classes into the unique minimal complete DFA."""
        classes = self.myhill_nerode_classes()
        state_to_class = {s: cid for cid, cls in enumerate(classes) for s in cls}
        transitions: list[dict[str, int]] = [{} for _ in classes]
        accept: set[int] = set()
        for cid, cls in enumerate(classes):
            rep = cls[0]
            if rep in self.accept:
                accept.add(cid)
            for sym in self.alphabet:
                if sym in self.transitions[rep]:
                    transitions[cid][sym] = state_to_class[self.transitions[rep][sym]]
        return Dfa(len(classes), list(self.alphabet), transitions, state_to_class[self.start], accept)


@dataclass
class Nfa:
    """General NFA with ε-transitions keyed by None."""
    num_states: int
    alphabet: list[str]
    transitions: list[dict[str | None, set[int]]]
    start: int
    accept: set[int]

    def epsilon_closure(self, states: set[int]) -> frozenset[int]:
        closure = set(states)
        stack = list(states)
        while stack:
            u = stack.pop()
            for v in self.transitions[u].get(None, ()):
                if v not in closure:
                    closure.add(v)
                    stack.append(v)
        return frozenset(closure)

    def accepts(self, word: str) -> bool:
        cur = self.epsilon_closure({self.start})
        for ch in word:
            moved: set[int] = set()
            for u in cur:
                moved |= self.transitions[u].get(ch, set())
            cur = self.epsilon_closure(moved)
        return any(s in self.accept for s in cur)

    def to_dfa(self) -> Dfa:
        """Subset construction (Sipser Theorem 1.39)."""
        start_set = self.epsilon_closure({self.start})
        subset_to_id: dict[frozenset[int], int] = {start_set: 0}
        queue = deque([start_set])
        transitions: list[dict[str, int]] = [{}]
        accept: set[int] = set()
        while queue:
            cur = queue.popleft()
            cid = subset_to_id[cur]
            if any(s in self.accept for s in cur):
                accept.add(cid)
            for sym in self.alphabet:
                moved: set[int] = set()
                for u in cur:
                    moved |= self.transitions[u].get(sym, set())
                nxt = self.epsilon_closure(moved)
                if nxt not in subset_to_id:
                    subset_to_id[nxt] = len(transitions)
                    transitions.append({})
                    queue.append(nxt)
                transitions[cid][sym] = subset_to_id[nxt]
        return Dfa(len(transitions), list(self.alphabet), transitions, 0, accept)


def regex_to_nfa(pattern: str) -> Nfa:
    """Thompson's construction from a classical regex (literals, concat, |, *, (), ε)."""
    pos = 0

    def peek() -> str | None:
        return pattern[pos] if pos < len(pattern) else None

    def parse_union():
        nonlocal pos
        node = parse_concat()
        while peek() == "|":
            pos += 1
            node = ("union", node, parse_concat())
        return node

    def parse_concat():
        items = []
        while (c := peek()) is not None and c not in ")|":
            items.append(parse_star())
        if not items:
            return ("empty",)
        acc = items[0]
        for item in items[1:]:
            acc = ("concat", acc, item)
        return acc

    def parse_star():
        nonlocal pos
        node = parse_atom()
        while peek() == "*":
            pos += 1
            node = ("star", node)
        return node

    def parse_atom():
        nonlocal pos
        c = peek()
        if c == "(":
            pos += 1
            inner = parse_union()
            if peek() != ")":
                raise ValueError("Unclosed '(' in regex")
            pos += 1
            return inner
        if c == "ε":
            pos += 1
            return ("empty",)
        if c is not None and c not in ")|*":
            pos += 1
            return ("lit", c)
        raise ValueError(f"Unexpected token {c!r} in regex")

    ast = parse_union()
    if pos != len(pattern):
        raise ValueError("Trailing characters in regex")

    transitions: list[dict[str | None, set[int]]] = []
    alphabet: set[str] = set()

    def new_state() -> int:
        transitions.append({})
        return len(transitions) - 1

    def add_edge(u: int, sym: str | None, v: int) -> None:
        transitions[u].setdefault(sym, set()).add(v)

    def build(node) -> tuple[int, int]:
        kind = node[0]
        if kind == "empty":
            s, f = new_state(), new_state()
            add_edge(s, None, f)
            return s, f
        if kind == "lit":
            alphabet.add(node[1])
            s, f = new_state(), new_state()
            add_edge(s, node[1], f)
            return s, f
        if kind == "concat":
            s1, f1 = build(node[1])
            s2, f2 = build(node[2])
            add_edge(f1, None, s2)
            return s1, f2
        if kind == "union":
            s = new_state()
            s1, f1 = build(node[1])
            s2, f2 = build(node[2])
            f = new_state()
            add_edge(s, None, s1)
            add_edge(s, None, s2)
            add_edge(f1, None, f)
            add_edge(f2, None, f)
            return s, f
        if kind == "star":
            s = new_state()
            s1, f1 = build(node[1])
            f = new_state()
            add_edge(s, None, s1)
            add_edge(s, None, f)
            add_edge(f1, None, s1)
            add_edge(f1, None, f)
            return s, f
        raise AssertionError(f"Unknown AST node: {kind}")

    start, finish = build(ast)
    return Nfa(len(transitions), sorted(alphabet), transitions, start, {finish})


def regex_matches(pattern: str, word: str) -> bool:
    return regex_to_nfa(pattern).accepts(word)
