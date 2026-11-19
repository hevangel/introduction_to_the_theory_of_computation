"""Ch 2: Context-free languages — CFG to CNF conversion, CYK, nondeterministic PDA,
deterministic PDA (DPDA, 3rd ed. §2.4), and canonical LR(1) parser generator
with Shift/Reduce and Reduce/Reduce conflict diagnostics."""
from collections import deque
from dataclasses import dataclass

EOF_SYMBOL = "$"


def balanced_parentheses(word: str) -> bool:
    """One-type balanced parentheses (a DCFL)."""
    depth = 0
    for ch in word:
        if ch == "(":
            depth += 1
        elif ch == ")" and depth > 0:
            depth -= 1
        else:
            return False
    return depth == 0


def anbn(word: str) -> bool:
    """Recognize a^n b^n for n >= 0."""
    n = 0
    while n < len(word) and word[n] == "a":
        n += 1
    return all(ch == "b" for ch in word[n:]) and len(word) - n == n


def cyk(
    word: str,
    terminals: list[tuple[str, str]],
    binary: list[tuple[str, str, str]],
    start: str,
) -> bool:
    """CYK membership for a CNF grammar on nonempty strings."""
    return cyk_with_epsilon(word, terminals, binary, start, allows_epsilon=False)


def cyk_with_epsilon(
    word: str,
    terminals: list[tuple[str, str]],
    binary: list[tuple[str, str, str]],
    start: str,
    allows_epsilon: bool = False,
) -> bool:
    """CYK membership supporting optional S -> ε."""
    n = len(word)
    if n == 0:
        return allows_epsilon
    dp: list[list[set[str]]] = [[set() for _ in range(n)] for _ in range(n)]
    for i, letter in enumerate(word):
        for var, term in terminals:
            if letter == term:
                dp[i][i].add(var)
    for span in range(2, n + 1):
        for i in range(n - span + 1):
            j = i + span - 1
            for k in range(i, j):
                for a, b, c in binary:
                    if b in dp[i][k] and c in dp[k + 1][j]:
                        dp[i][j].add(a)
    return start in dp[0][n - 1]


@dataclass(frozen=True)
class Symbol:
    kind: str  # "var" or "term"
    name: str


def var(name: str) -> Symbol:
    return Symbol("var", name)


def term(name: str) -> Symbol:
    return Symbol("term", name)


@dataclass(frozen=True)
class Production:
    lhs: str
    rhs: tuple[Symbol, ...]


@dataclass
class CnfGrammar:
    start: str
    allows_epsilon: bool
    terminals: list[tuple[str, str]]
    binary: list[tuple[str, str, str]]

    def accepts(self, tokens: list[str]) -> bool:
        n = len(tokens)
        if n == 0:
            return self.allows_epsilon
        dp: list[list[set[str]]] = [[set() for _ in range(n)] for _ in range(n)]
        for i, tok in enumerate(tokens):
            for v, t in self.terminals:
                if t == tok:
                    dp[i][i].add(v)
        for span in range(2, n + 1):
            for i in range(n - span + 1):
                j = i + span - 1
                for k in range(i, j):
                    for a, b, c in self.binary:
                        if b in dp[i][k] and c in dp[k + 1][j]:
                            dp[i][j].add(a)
        return self.start in dp[0][n - 1]


@dataclass
class Cfg:
    start: str
    productions: list[Production]

    def to_cnf(self) -> CnfGrammar:
        """Convert CFG to Chomsky Normal Form (Sipser Theorem 2.9)."""
        new_start = f"{self.start}0"
        prods: set[Production] = set(self.productions)
        prods.add(Production(new_start, (var(self.start),)))

        # Step 2: Nullable variables
        nullable: set[str] = set()
        changed = True
        while changed:
            changed = False
            for p in prods:
                if p.lhs not in nullable and all(
                    s.kind == "var" and s.name in nullable for s in p.rhs
                ):
                    nullable.add(p.lhs)
                    changed = True
        allows_epsilon = new_start in nullable

        # Eliminate ε-rules
        no_eps: set[Production] = set()
        for p in prods:
            null_pos = [i for i, s in enumerate(p.rhs) if s.kind == "var" and s.name in nullable]
            for mask in range(1 << len(null_pos)):
                omit = {null_pos[b] for b in range(len(null_pos)) if mask & (1 << b)}
                new_rhs = tuple(s for i, s in enumerate(p.rhs) if i not in omit)
                if new_rhs:
                    no_eps.add(Production(p.lhs, new_rhs))

        # Step 3: Eliminate unit rules A -> B
        all_vars = {p.lhs for p in no_eps} | {
            s.name for p in no_eps for s in p.rhs if s.kind == "var"
        }
        unit_reach: dict[str, set[str]] = {v: {v} for v in all_vars}
        changed = True
        while changed:
            changed = False
            for p in no_eps:
                if len(p.rhs) == 1 and p.rhs[0].kind == "var":
                    b = p.rhs[0].name
                    before = len(unit_reach[p.lhs])
                    unit_reach[p.lhs] |= unit_reach[b]
                    if len(unit_reach[p.lhs]) > before:
                        changed = True

        no_unit: set[Production] = set()
        for a, targets in unit_reach.items():
            for b in targets:
                for p in no_eps:
                    if p.lhs == b and not (len(p.rhs) == 1 and p.rhs[0].kind == "var"):
                        no_unit.add(Production(a, p.rhs))

        # Step 4: Terminal isolation & binarization
        terminals: set[tuple[str, str]] = set()
        binary: set[tuple[str, str, str]] = set()
        term_map: dict[str, str] = {}
        fresh_id = 0

        def get_term_var(t_name: str) -> str:
            if t_name not in term_map:
                v_name = f"T_{t_name}"
                term_map[t_name] = v_name
                terminals.add((v_name, t_name))
            return term_map[t_name]

        for p in no_unit:
            if len(p.rhs) == 1 and p.rhs[0].kind == "term":
                terminals.add((p.lhs, p.rhs[0].name))
            else:
                rhs_vars = [
                    s.name if s.kind == "var" else get_term_var(s.name) for s in p.rhs
                ]
                cur = p.lhs
                for i in range(len(rhs_vars) - 2):
                    fresh_id += 1
                    nxt = f"X_{fresh_id}"
                    binary.add((cur, rhs_vars[i], nxt))
                    cur = nxt
                binary.add((cur, rhs_vars[-2], rhs_vars[-1]))

        return CnfGrammar(new_start, allows_epsilon, sorted(terminals), sorted(binary))


@dataclass(frozen=True)
class PdaTransition:
    src: int
    inp: str | None
    pop: str | None
    dst: int
    push: tuple[str, ...] = ()


@dataclass
class Pda:
    """Nondeterministic Pushdown Automaton with bounded BFS simulation."""
    start: int
    accept: set[int]
    transitions: list[PdaTransition]

    def accepts(self, word: str, max_configs: int = 2000) -> bool:
        init = (self.start, 0, ())
        queue: deque[tuple[int, int, tuple[str, ...]]] = deque([init])
        visited = {init}
        explored = 0
        while queue:
            q, pos, stack = queue.popleft()
            if pos == len(word) and q in self.accept:
                return True
            explored += 1
            if explored > max_configs:
                break
            for tr in self.transitions:
                if tr.src != q:
                    continue
                if tr.inp is None:
                    nxt_pos = pos
                elif pos < len(word) and word[pos] == tr.inp:
                    nxt_pos = pos + 1
                else:
                    continue
                if tr.pop is not None:
                    if not stack or stack[-1] != tr.pop:
                        continue
                    nxt_stack = list(stack[:-1])
                else:
                    nxt_stack = list(stack)
                for sym in reversed(tr.push):
                    nxt_stack.append(sym)
                cfg = (tr.dst, nxt_pos, tuple(nxt_stack))
                if cfg not in visited:
                    visited.add(cfg)
                    queue.append(cfg)
        return False


@dataclass
class Dpda:
    """Deterministic Pushdown Automaton (Sipser 3rd edition §2.4)."""
    start: int
    accept: set[int]
    transitions: list[PdaTransition]

    def is_deterministic(self) -> bool:
        for i, a in enumerate(self.transitions):
            for b in self.transitions[i + 1 :]:
                if a.src != b.src:
                    continue
                in_conflict = a.inp is None or b.inp is None or a.inp == b.inp
                st_conflict = a.pop is None or b.pop is None or a.pop == b.pop
                if in_conflict and st_conflict:
                    return False
        return True

    def accepts(self, word: str, max_steps: int = 1000) -> bool:
        if not self.is_deterministic():
            raise ValueError("Transition table violates DPDA determinism")
        q = self.start
        pos = 0
        stack: list[str] = []
        for _ in range(max_steps):
            cur_in = word[pos] if pos < len(word) else None
            cur_top = stack[-1] if stack else None
            match = None
            for tr in self.transitions:
                if (
                    tr.src == q
                    and (tr.inp is None or tr.inp == cur_in)
                    and (tr.pop is None or tr.pop == cur_top)
                ):
                    match = tr
                    break
            if match is None:
                break
            if match.inp is not None:
                pos += 1
            if match.pop is not None:
                stack.pop()
            for sym in reversed(match.push):
                stack.append(sym)
            q = match.dst
        return pos == len(word) and q in self.accept


@dataclass(frozen=True)
class Lr1Item:
    prod_idx: int
    dot: int
    lookahead: str


@dataclass
class Lr1Parser:
    """Canonical LR(1) parser generator with Shift/Reduce and Reduce/Reduce diagnostics."""
    productions: list[Production]
    num_states: int
    action: dict[tuple[int, str], tuple[str, int]]
    goto: dict[tuple[int, str], int]
    conflicts: list[dict]

    @classmethod
    def build(cls, cfg: Cfg) -> "Lr1Parser":
        aug_start = f"{cfg.start}'"
        productions = [Production(aug_start, (var(cfg.start),))] + list(cfg.productions)
        nonterminals = {p.lhs for p in productions} | {
            s.name for p in productions for s in p.rhs if s.kind == "var"
        }

        nullable: set[str] = set()
        first: dict[str, set[str]] = {v: set() for v in nonterminals}
        changed = True
        while changed:
            changed = False
            for p in productions:
                all_null = True
                for sym in p.rhs:
                    if sym.kind == "term":
                        if sym.name not in first[p.lhs]:
                            first[p.lhs].add(sym.name)
                            changed = True
                        all_null = False
                        break
                    before = len(first[p.lhs])
                    first[p.lhs] |= first[sym.name]
                    if len(first[p.lhs]) > before:
                        changed = True
                    if sym.name not in nullable:
                        all_null = False
                        break
                if all_null and p.lhs not in nullable:
                    nullable.add(p.lhs)
                    changed = True

        def first_of_seq(seq: tuple[Symbol, ...], tail_la: str) -> set[str]:
            out: set[str] = set()
            all_null = True
            for sym in seq:
                if sym.kind == "term":
                    out.add(sym.name)
                    all_null = False
                    break
                out |= first[sym.name]
                if sym.name not in nullable:
                    all_null = False
                    break
            if all_null:
                out.add(tail_la)
            return out

        def closure(seed: set[Lr1Item]) -> frozenset[Lr1Item]:
            items = set(seed)
            changed_c = True
            while changed_c:
                changed_c = False
                for item in list(items):
                    prod = productions[item.prod_idx]
                    if item.dot < len(prod.rhs) and prod.rhs[item.dot].kind == "var":
                        b = prod.rhs[item.dot].name
                        las = first_of_seq(prod.rhs[item.dot + 1 :], item.lookahead)
                        for pidx, p in enumerate(productions):
                            if p.lhs == b:
                                for la in las:
                                    nxt = Lr1Item(pidx, 0, la)
                                    if nxt not in items:
                                        items.add(nxt)
                                        changed_c = True
            return frozenset(items)

        def goto_fn(items: frozenset[Lr1Item], sym: Symbol) -> frozenset[Lr1Item]:
            moved = {
                Lr1Item(it.prod_idx, it.dot + 1, it.lookahead)
                for it in items
                if it.dot < len(productions[it.prod_idx].rhs)
                and productions[it.prod_idx].rhs[it.dot] == sym
            }
            return closure(moved) if moved else frozenset()

        start_set = closure({Lr1Item(0, 0, EOF_SYMBOL)})
        states: list[frozenset[Lr1Item]] = [start_set]
        state_map: dict[frozenset[Lr1Item], int] = {start_set: 0}
        queue: deque[int] = deque([0])
        transitions: dict[tuple[int, Symbol], int] = {}

        while queue:
            sid = queue.popleft()
            items = states[sid]
            next_syms = {
                productions[it.prod_idx].rhs[it.dot]
                for it in items
                if it.dot < len(productions[it.prod_idx].rhs)
            }
            for sym in sorted(next_syms, key=lambda s: (s.kind, s.name)):
                target = goto_fn(items, sym)
                if not target:
                    continue
                if target not in state_map:
                    state_map[target] = len(states)
                    states.append(target)
                    queue.append(state_map[target])
                transitions[(sid, sym)] = state_map[target]

        action: dict[tuple[int, str], tuple[str, int]] = {}
        goto: dict[tuple[int, str], int] = {}
        conflicts: list[dict] = []

        def insert_action(state: int, la: str, act: tuple[str, int]) -> None:
            key = (state, la)
            if key in action and action[key] != act:
                existing = action[key]
                kinds = {existing[0], act[0]}
                if kinds == {"shift", "reduce"}:
                    s_to = existing[1] if existing[0] == "shift" else act[1]
                    r_prod = existing[1] if existing[0] == "reduce" else act[1]
                    conflicts.append({
                        "type": "shift-reduce",
                        "state": state,
                        "lookahead": la,
                        "shift_to": s_to,
                        "reduce_prod": r_prod,
                    })
                elif kinds == {"reduce"}:
                    conflicts.append({
                        "type": "reduce-reduce",
                        "state": state,
                        "lookahead": la,
                        "prod_a": existing[1],
                        "prod_b": act[1],
                    })
            else:
                action[key] = act

        for (sid, sym), tid in transitions.items():
            if sym.kind == "term":
                insert_action(sid, sym.name, ("shift", tid))
            else:
                goto[(sid, sym.name)] = tid

        for sid, items in enumerate(states):
            for it in sorted(items, key=lambda x: (x.prod_idx, x.dot, x.lookahead)):
                prod = productions[it.prod_idx]
                if it.dot == len(prod.rhs):
                    if it.prod_idx == 0 and it.lookahead == EOF_SYMBOL:
                        insert_action(sid, EOF_SYMBOL, ("accept", 0))
                    else:
                        insert_action(sid, it.lookahead, ("reduce", it.prod_idx))

        return cls(productions, len(states), action, goto, conflicts)

    def parse(self, tokens: list[str]) -> list[int]:
        """Parse tokens; return sequence of reduced production indices or raise ValueError."""
        inp = list(tokens) + [EOF_SYMBOL]
        stack = [0]
        reductions: list[int] = []
        pos = 0
        while True:
            state = stack[-1]
            la = inp[pos]
            act = self.action.get((state, la))
            if act is None:
                raise ValueError(f"Syntax error at token {la!r} (pos {pos}) in state {state}")
            kind, val = act
            if kind == "shift":
                stack.append(val)
                pos += 1
            elif kind == "reduce":
                prod = self.productions[val]
                for _ in range(len(prod.rhs)):
                    stack.pop()
                top = stack[-1]
                stack.append(self.goto[(top, prod.lhs)])
                reductions.append(val)
            elif kind == "accept":
                return reductions
