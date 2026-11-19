# Chapter 2 — Context-Free Languages & Deterministic Parsing

**Core in all three editions (1e, 2e, 3e); major new Section 2.4 (DPDA, DCFL, DCFG, LR(k)) added in the 3rd edition.**  
**Code:** [src/ch02.rs](../src/ch02.rs) (Rust) · [python/ch02.py](../python/ch02.py) (Python)

> **Big Picture:** Finite automata cannot even count arbitrarily high (they fail on $\{0^n 1^n\}$). Adding a single **LIFO stack** gives **Pushdown Automata (PDAs)**, which match **Context-Free Grammars (CFGs)** — the mathematical backbone of programming language syntax and compilers. Crucially, unlike finite automata, **nondeterminism strictly increases power** for PDAs ($\text{DCFL} \subsetneq \text{CFL}$)!

---

## 2.1 Context-Free Grammars (CFGs)

### Formal Definition (Sipser Def 2.2)
A **Context-Free Grammar** is a 4-tuple $G = (V, \Sigma, R, S)$ where:
1. $V$ is a finite set of **variables (nonterminals)**.
2. $\Sigma$ is a finite set of **terminals** ($V \cap \Sigma = \emptyset$).
3. $R$ is a finite set of **rules (productions)** of the form $A \to w$ where $A \in V$ and $w \in (V \cup \Sigma)^*$.
4. $S \in V$ is the **start variable**.

- **Derivation:** $uAv \Rightarrow uwv$ if $A \to w \in R$. Write $u \Rightarrow^* v$ for zero or more steps.
- **Language:** $L(G) = \{w \in \Sigma^* \mid S \Rightarrow^* w\}$.
- **Leftmost derivation:** Always expand the leftmost variable first.
- **Ambiguity:** A string $w$ is derived **ambiguously** if it has $\ge 2$ distinct parse trees (equivalently, $\ge 2$ distinct leftmost derivations). A grammar is ambiguous if it generates some string ambiguously.
  - *Example:* $E \to E + E \mid E \times E \mid n$ is ambiguous on `n + n * n`. Fixing precedence/associativity gives the unambiguous grammar $E \to E + T \mid T$, $T \to T \times F \mid F$, $F \to (E) \mid n$.
  - **Inherently ambiguous CFL:** Some CFLs have *no* unambiguous CFG (e.g., $\{a^i b^j c^k \mid i = j \text{ or } j = k\}$).

### Chomsky Normal Form (CNF) — Theorem 2.9
A CFG is in **Chomsky Normal Form** if every rule has one of three forms:
- $A \to BC$ (where $B, C \in V \setminus \{S\}$),
- $A \to a$ (where $a \in \Sigma$),
- $S \to \varepsilon$ (allowed **only** for the start variable $S$).

**4-Step Conversion Algorithm (MEMORIZE FOR EXAMS!):**
1. **START:** Add a new start variable $S_0 \to S$ so the start variable never appears on any RHS.
2. **DEL (Eliminate $\varepsilon$-rules):** Find all *nullable* variables ($A \Rightarrow^* \varepsilon$). Remove every $A \to \varepsilon$ ($A \neq S_0$), and for every rule containing nullable variables on the RHS, add all combinations omitting subsets of them (avoiding re-adding $A \to \varepsilon$).
3. **UNIT (Eliminate unit rules $A \to B$):** Whenever $A \Rightarrow^* B$ via unit rules and $B \to u$ is a non-unit rule, add $A \to u$, then delete all unit rules.
4. **TERM & BIN:** Replace any terminal $a$ in a rule of length $\ge 2$ with a fresh variable $U_a \to a$, and break every rule $A \to B_1 B_2 \dots B_k$ ($k \ge 3$) into binary rules $A \to B_1 X_1,\; X_1 \to B_2 X_2, \dots$.

*Fact:* Any derivation of a string $w$ of length $n \ge 1$ in a CNF grammar takes **exactly $2n - 1$ steps**!  
*Code:* `Cfg::to_cnf` in [ch02.rs](../src/ch02.rs) and [ch02.py](../python/ch02.py).

### CYK Parsing Algorithm (Dynamic Programming for CNF)
Given a CNF grammar $G$ and input $w = w_1 \dots w_n$ ($n \ge 1$):
- Table $T[i, j]$ stores all variables $A \in V$ such that $A \Rightarrow^* w_i \dots w_j$.
- **Base case:** $T[i, i] = \{A \mid A \to w_i \in R\}$.
- **Recurrence (for span length $2 \dots n$):** $A \in T[i, j]$ iff $\exists k \in [i, j-1]$ and rule $A \to BC$ with $B \in T[i, k]$ and $C \in T[k+1, j]$.
- **Accept** iff $S \in T[1, n]$. Time: $O(n^3 |R|)$; Space: $O(n^2 |V|)$.  
*Code:* `cyk`, `cyk_with_epsilon`, `CnfGrammar::accepts` in [ch02.rs](../src/ch02.rs) and [ch02.py](../python/ch02.py).

---

## 2.2 Pushdown Automata (PDAs)

### Formal Definition (Sipser Def 2.13)
A **(Nondeterministic) Pushdown Automaton** is a 6-tuple $P = (Q, \Sigma, \Gamma, \delta, q_0, F)$ where:
- $\Gamma$ is a finite **stack alphabet** (often includes bottom-of-stack marker `$`).
- $\delta : Q \times \Sigma_\varepsilon \times \Gamma_\varepsilon \to \mathcal{P}(Q \times \Gamma_\varepsilon)$.
- Transition $(r, b) \in \delta(q, a, x)$ means: in state $q$, read $a$ (or $\varepsilon$), pop $x$ from stack (or $\varepsilon$), move to state $r$, and push $b$ (or $\varepsilon$).
- **Acceptance (Sipser convention):** By **accept state** when all input is consumed (stack need not be empty, though using a bottom marker `$` lets you test stack emptiness!).

### Theorem 2.20: A language is Context-Free $\iff$ some PDA recognizes it.
- **($\Rightarrow$) CFG $\to$ PDA:** Push `$` then $S$. In a single main loop state:
  - If top of stack is variable $A$, nondeterministically pop $A$ and push the RHS of some rule $A \to w$ (in reverse so the leftmost symbol is on top).
  - If top of stack is terminal $a$, read $a$ from input and pop $a$.
  - If top of stack is `$`, pop `$` and move to $q_{\text{accept}}$.
- **($\Leftarrow$) PDA $\to$ CFG:** First normalize PDA to have a single accept state, empty its stack before accepting, and either push *or* pop on every move (not both). Create variable $A_{pq}$ generating all strings taking the PDA from state $p$ (empty stack) to state $q$ (empty stack):
  1. $A_{pp} \to \varepsilon$ for all $p$.
  2. $A_{pq} \to A_{pr} A_{rq}$ for all $p, q, r$ (stack returns to empty at intermediate state $r$).
  3. $A_{pq} \to a A_{rs} b$ whenever $p \xrightarrow{a, \varepsilon \to u} r$ pushes $u$ and $s \xrightarrow{b, u \to \varepsilon} q$ pops that same $u$.

*Code:* `Pda::accepts` (demonstrated on even-length palindromes $\{ww^{\mathcal{R}}\}$, which requires nondeterminism!) in [ch02.rs](../src/ch02.rs) and [ch02.py](../python/ch02.py).

---

## 2.3 Non-Context-Free Languages & The CFL Pumping Lemma

### Theorem 2.34 (Pumping Lemma for Context-Free Languages)
If $A$ is a CFL, there exists a pumping length $p \ge 1$ such that every string $s \in A$ with $|s| \ge p$ can be split into **five** pieces $s = uvxyz$ satisfying:
1. $u v^i x y^i z \in A$ for every $i \ge 0$ ($v$ and $y$ pump **simultaneously**!),
2. $|vy| > 0$ (at least one of $v, y$ is nonempty — **note:** one of them *can* be empty!),
3. $|vxy| \le p$.

**Why it's true (1-line intuition):** In a parse tree for a sufficiently long string, some root-to-leaf path has length $> |V|$, so some variable $R$ repeats on that path. The lower $R$-subtree yields $x$, the upper $R$-subtree yields $vxy$; replacing the upper with the lower pumps down ($i=0$), and copying the upper into the lower pumps up ($i \ge 2$).

#### Worked Example 1: $B = \{a^n b^n c^n \mid n \ge 0\}$
- Choose $s = a^p b^p c^p \in B$.
- Because $|vxy| \le p$, the substring $vxy$ can touch **at most two** of the three symbol types (`a`, `b`, `c`) — it can NEVER contain both `a` and `c`!
- Pump $i = 2$: $u v^2 x y^2 z$ increases the count of 1 or 2 symbol types (since $|vy| > 0$) while leaving at least one symbol type at $p$. Thus the three counts are no longer equal $\implies$ not in $B$. Contradiction!

#### Worked Example 2: $D = \{ww \mid w \in \{0, 1\}^*\}$
- Choose $s = 0^p 1^p 0^p 1^p \in D$. (**Trap:** $0^p 1 0^p 1$ is NOT hard enough for CFL pumping — you need $0^p 1^p 0^p 1^p$!)
- Case 1: $vxy$ straddles the midpoint (inside $1^p 0^p$). Pump down ($i = 0$): removes $1$s from the first half and/or $0$s from the second half, destroying equality.
- Case 2: $vxy$ lies wholly in the first half $0^p 1^p$ (or wholly in the second half). Pumping up ($i = 2$) shifts the midpoint into the $1^p$ block (or $0^p$ block), making the two halves mismatch. Contradiction!

### CFL Closure Properties (MAJOR EXAM FAVORITE!)

| Operation | CFL Closed? | Proof / Counterexample |
|---|---|---|
| Union ($L_1 \cup L_2$) | **Yes** | New start $S \to S_1 \mid S_2$ |
| Concatenation ($L_1 \circ L_2$) | **Yes** | New start $S \to S_1 S_2$ |
| Kleene Star ($L_1^*$) | **Yes** | $S \to S S_1 \mid \varepsilon$ |
| Reversal ($L^{\mathcal{R}}$) | **Yes** | Reverse RHS of every production |
| **Intersection with Regular** ($L \cap R$) | **Yes!** | Product of PDA for $L$ and DFA for $R$! |
| **Intersection of two CFLs** ($L_1 \cap L_2$) | **NO!** | $\{a^n b^n c^m\} \cap \{a^m b^n c^n\} = \{a^n b^n c^n\}$ |
| **Complement** ($\overline{L}$) | **NO!** | If closed under $\overline{\cdot}$ and $\cup$, De Morgan would imply closed under $\cap$! |

---

## 2.4 Deterministic Context-Free Languages (NEW in 3rd Edition!)

Section 2.4 is the **major addition of the 3rd edition** and is increasingly tested in courses using Sipser 3e.

### Deterministic Pushdown Automaton (DPDA)
A PDA is **deterministic** if for every $q \in Q$, $a \in \Sigma$, and $x \in \Gamma$:
$$|\delta(q, a, x)| + |\delta(q, a, \varepsilon)| + |\delta(q, \varepsilon, x)| + |\delta(q, \varepsilon, \varepsilon)| \le 1$$
In plain English: at any state and stack top, the machine has **at most one** legal move (no choice between reading vs $\varepsilon$-moving, or popping vs ignoring stack).
- A language is a **Deterministic CFL (DCFL)** iff some DPDA recognizes it.
- **Strict containment:** $\text{Regular} \subsetneq \text{DCFL} \subsetneq \text{CFL}$.
  - $\{a^n b^n\}$ and balanced parentheses are DCFLs.
  - $\{ww^{\mathcal{R}}\}$ and $\{a^i b^j c^k \mid i = j \text{ or } j = k\}$ are CFLs but **NOT** DCFLs!

### Closure Properties of DCFLs (Contrast with CFLs!)
- **Closed under Complement!** (Sipser 3e Theorem 2.42): Unlike general CFLs, DCFLs ARE closed under complement! (*Subtlety:* you must handle reading loops / $\varepsilon$-moves after input ends by converting the DPDA to read-once-per-accept-region form, not just naively flipping $F$.)
- **Closed under Intersection with a Regular Language** (product of DPDA and DFA stays deterministic!).
- **NOT closed under Union or Intersection!**
  - *Proof:* $L_1 = \{a^i b^j c^k \mid i = j\}$ and $L_2 = \{a^i b^j c^k \mid j = k\}$ are both DCFLs, but $L_1 \cap L_2 = \{a^n b^n c^n\}$ is not even a CFL! By De Morgan and complement closure, DCFLs cannot be closed under union either!

### Endmarkers (`$`), Handles, DCFGs, and LR(k) Parsing
- **Endmarker convention:** Appending a special end-of-input symbol $\dashv$ (or `$`) turns any DCFL $A$ into $A\dashv$ so a DPDA knows for sure when input has ended (preventing prefix ambiguity).
- **Handles & Bottom-Up Parsing:** In a rightmost derivation $S \Rightarrow_{\text{rm}}^* \alpha A w \Rightarrow_{\text{rm}} \alpha \beta w$, the substring $\beta$ (together with rule $A \to \beta$) is the **handle**. Bottom-up parsing scans left-to-right, **shifts** tokens onto a stack, and **reduces** a handle $\beta$ on top of the stack back to $A$ — reconstructing the rightmost derivation **in reverse**!
- **LR($k$) Grammars:** A CFG is **LR($k$)** if every handle can be uniquely identified by looking at the stack contents plus at most $k$ lookahead tokens.
  - **Theorem (Sipser 3e):** A language $A$ is a DCFL $\iff$ $A\dashv$ is generated by an **LR(1)** (in fact, even **LR(0)** with endmarker!) grammar — i.e., **Deterministic CFGs (DCFGs) $\equiv$ DPDAs**!
- **Canonical LR(1) Parser Construction:**
  1. Augment grammar with $S' \to S$.
  2. An **LR(1) item** is $[A \to \alpha \cdot \beta,\; a]$ meaning we have seen $\alpha$, expect $\beta$, and if we reduce $A \to \alpha\beta$, the next lookahead token will be $a \in \Sigma \cup \{\$\}$.
  3. **CLOSURE:** If $[A \to \alpha \cdot B \beta, a]$ is in the item set, add $[B \to \cdot \gamma, b]$ for every $B \to \gamma$ and every $b \in \text{FIRST}(\beta a)$.
  4. **GOTO($I, X$):** Advance the dot past $X$ in all matching items of $I$ and take CLOSURE.
  5. **Conflicts:**
     - **Shift/Reduce conflict:** Same state and lookahead has both `Shift(s)` and `Reduce(r)` (happens in ambiguous grammars like $E \to E + E \mid n$).
     - **Reduce/Reduce conflict:** Same state and lookahead has two different reductions `Reduce(r1)` and `Reduce(r2)`.
  - *Code:* Full `Dpda` simulator and canonical `Lr1Parser` generator with `ShiftReduce` / `ReduceReduce` conflict diagnostics in [ch02.rs](../src/ch02.rs) and [ch02.py](../python/ch02.py)!

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Prove $L = \{w \in \{a, b, c\}^* \mid \#_a(w) = \#_b(w) = \#_c(w)\}$ is not context-free.  
   *Outline:* Use closure under intersection with regular languages! If $L$ were a CFL, then $L \cap a^* b^* c^* = \{a^n b^n c^n \mid n \ge 0\}$ would be a CFL — contradiction!
2. **Q:** Convert $S \to aSb \mid \varepsilon$ into Chomsky Normal Form.  
   *Outline:* $S_0 \to S \mid \varepsilon$; remove $S \to \varepsilon$ giving $S_0 \to S \mid \varepsilon$, $S \to aSb \mid ab$; remove unit rule $S_0 \to S$; isolate terminals $A \to a, B \to b$; binarize $S \to A X, X \to S B$, $S \to AB$ (and same for $S_0$).
3. **Q:** Why does `E -> E + E | n` fail to be LR(1), while `E -> E + T | T; T -> n` succeeds?  
   *Outline:* After shifting `E + E` with lookahead `+`, the ambiguous grammar has both `[E -> E + E ·, +]` (reduce) and `[E -> E · + E, +]` (shift) in the same state — a Shift/Reduce conflict. Separating `E` and `T` forces left-associative reduction before shifting the next `+`.
