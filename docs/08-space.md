# Chapter 8 — Space Complexity

**All three editions (1e, 2e, 3e).**  
**Code:** [src/ch08.rs](../src/ch08.rs) (Rust) · [python/ch08.py](../python/ch08.py) (Python)

> **Big Picture:** Unlike time, **memory (space) is reusable**! You can erase a work tape and reuse the same cells over and over. This single physical fact makes space complexity behave radically differently from time complexity:
> - **Savitch's Theorem:** Nondeterminism only costs a **quadratic** factor in space ($\text{PSPACE} = \text{NPSPACE}$), NOT exponential!
> - **Immerman–Szelepcsényi Theorem:** Nondeterministic space IS closed under complement ($\text{NL} = \text{coNL}$)!

---

## 8.1 Space Complexity & Savitch's Theorem

### Formal Definitions (Sipser Def 8.1–8.2)
- $\text{SPACE}(f(n))$: languages decided by a deterministic TM scanning at most $O(f(n))$ tape cells.
- $\text{NSPACE}(f(n))$: languages decided by a nondeterministic TM scanning at most $O(f(n))$ cells on every branch.
- **Configuration count bound:** A TM using $f(n)$ space has at most $2^{O(f(n))}$ distinct configurations (assuming $f(n) \ge \log n$). Therefore, any halting $f(n)$-space TM runs in at most $2^{O(f(n))}$ time!

### Theorem 8.5 (Savitch's Theorem — HUGE EXAM FAVORITE!)
For any function $f : \mathbb{N} \to \mathbb{R}^+$ with $f(n) \ge n$ (actually even $f(n) \ge \log n$!):
$$\text{NSPACE}(f(n)) \subseteq \text{SPACE}\bigl(f(n)^2\bigr)$$

**Proof Blueprint (`CAN-YIELD` Divide-and-Conquer):**
Given NTM $N$ using $f(n)$ space, we want to test if $c_{\text{start}}$ can reach $c_{\text{accept}}$ in at most $t = 2^{d f(n)}$ steps. Define recursive procedure $\text{CANYIELD}(c_1, c_2, t)$:
1. If $t = 1$: accept iff $c_1 = c_2$ or $c_1$ directly yields $c_2$ in 1 step of $N$.
2. If $t > 1$: loop over **every** possible midpoint configuration $c_m$ of size $f(n)$:
   - Recursively test $\text{CANYIELD}(c_1, c_m, \lfloor t/2 \rfloor)$.
   - **Reuse the same stack space!** If true, recursively test $\text{CANYIELD}(c_m, c_2, \lceil t/2 \rceil)$.
   - If both return true, accept!
3. **Space analysis:** Recursion depth is $\log_2(2^{d f(n)}) = O(f(n))$. Each stack frame stores one configuration $c_m$ of size $O(f(n))$. Total space = $O(f(n)) \times O(f(n)) = O(f(n)^2)$! $\blacksquare$

*Code:* `reach_within` in [ch08.rs](../src/ch08.rs) and [ch08.py](../python/ch08.py).

---

## 8.2 The Class PSPACE

$$\text{PSPACE} = \bigcup_{k \ge 1} \text{SPACE}(n^k) = \bigcup_{k \ge 1} \text{NSPACE}(n^k) = \text{NPSPACE}$$
- **Master Inclusion Chain (MEMORIZE THIS!):**
  $$\text{L} \subseteq \text{NL} = \text{coNL} \subseteq \text{P} \subseteq \text{NP} \subseteq \text{PSPACE} = \text{NPSPACE} \subseteq \text{EXPTIME}$$
- **Why $\text{NP} \subseteq \text{PSPACE}$?** Given an NP verifier, iterate over all $2^{n^k}$ certificates one by one, **reusing the same $O(n^k)$ space** for each certificate!
- **Why $\text{PSPACE} \subseteq \text{EXPTIME}$?** A polynomial-space TM has at most $2^{O(n^k)}$ configurations, so any decider must halt in exponential time.
- **Note:** We know $\text{L} \subsetneq \text{PSPACE}$ and $\text{P} \subsetneq \text{EXPTIME}$ (by the Hierarchy Theorems in Ch 9!), so at least one inclusion in the chain is strict — we just don't know *which* ones!

---

## 8.3 PSPACE-Completeness

A language $B$ is **PSPACE-complete** if:
1. $B \in \text{PSPACE}$, AND
2. Every $A \in \text{PSPACE}$ is polynomial-time reducible to $B$ ($A \le_P B$). *(Why $\le_P$ and not $\le_{\text{PSPACE}}$? Because a PSPACE reduction would be too powerful — every nontrivial language in PSPACE would become PSPACE-complete!)*

### Theorem 8.9: $TQBF$ is PSPACE-Complete!
$$TQBF = \{\langle \phi \rangle \mid \phi \text{ is a true fully quantified Boolean formula}\}$$
Example: $\forall x \exists y [(x \lor y) \land (\neg x \lor \neg y)] \in TQBF$ (choose $y = \neg x$).
- **In PSPACE:** Recursively strip quantifiers: for $\exists x\, \psi$, try $x=0$ then $x=1$ (reusing space!); for $\forall x\, \psi$, verify both $x=0$ and $x=1$. Recursion depth = number of variables $m \le n$.  
  *Code:* `eval_tqbf` in [ch08.rs](../src/ch08.rs) and [ch08.py](../python/ch08.py).
- **PSPACE-hard (Savitch-style formula!):** Express $\phi_{c_1, c_2, t}$ ("$c_1$ yields $c_2$ in $\le t$ steps") recursively!
  - Naive doubling $\exists c_m (\phi_{c_1, c_m, t/2} \land \phi_{c_m, c_2, t/2})$ blows up formula size exponentially!
  - **Clever $\forall$-trick (keeps formula size $O(f(n)^2)$!):**
    $$\phi_{c_1, c_2, t} \equiv \exists c_m\, \forall c_3\, \forall c_4\, \Bigl[\bigl((c_3 = c_1 \land c_4 = c_m) \lor (c_3 = c_m \land c_4 = c_2)\bigr) \to \phi_{c_3, c_4, t/2}\Bigr]$$
    Only **one** recursive call per level $\implies$ polynomial formula size! $\blacksquare$

### Games & Automata in PSPACE
- **$FORMULA\text{-}GAME$ (Thm 8.11):** Two players (E and A) alternate picking truth values for $\exists x_1 \forall x_2 \dots \psi$; Player E has a winning strategy $\iff \phi \in TQBF$ (**PSPACE-complete**).
- **$GG$ (Generalized Geography, Thm 8.14):** Two players alternate extending a simple path in a directed graph; first player stuck loses (**PSPACE-complete** via reduction from $FORMULA\text{-}GAME$).
- **$ALL_{\text{NFA}} = \{\langle N \rangle \mid L(N) = \Sigma^*\}$:** **PSPACE-complete** (whereas $ALL_{\text{DFA}} \in \text{P}$ and $ALL_{\text{CFG}}$ is undecidable!).

---

## 8.4–8.6 The Classes L, NL, and $\text{NL} = \text{coNL}$

### Sublinear Space Model (Read-Only Input Tape!)
To measure space $< n$ (like $O(\log n)$), we use a **2-tape TM**:
- Tape 1: **read-only input tape** (does NOT count toward space!).
- Tape 2: **read/write work tape** (ONLY cells used here count!).
- $\text{L} = \text{SPACE}(\log n)$ (deterministic log-space — enough to store $O(1)$ pointers/counters of size $\log n$!).
- $\text{NL} = \text{NSPACE}(\log n)$ (nondeterministic log-space).
- **Examples:** $\{0^k 1^k\} \in \text{L}$ (two $\log n$-bit counters!). $PATH \in \text{NL}$ (guess next vertex along path using $O(\log n)$ bits for current vertex + step counter).

### Log-Space Reducibility ($\le_L$) & NL-Completeness
- $A \le_L B$ if a **log-space transducer** (read-only input, $O(\log n)$ work tape, write-only output tape) computes $f(w)$ with $w \in A \iff f(w) \in B$.
- **Theorem 8.23:** If $A \le_L B$ and $B \in \text{L}$, then $A \in \text{L}$. *(Subtlety: you cannot store $f(w)$ on the work tape! Instead, whenever the L-decider for $B$ asks for the $i$-th symbol of $f(w)$, re-run the transducer from scratch and discard the first $i-1$ output symbols!)*
- **Theorem 8.25:** **$PATH$ is NL-complete!** (And $2SAT$ is also NL-complete!)
- **Corollary 8.26:** $\text{NL} \subseteq \text{P}$ (an NL machine has only $n^{O(1)}$ configurations, so its configuration graph has polynomial size and BFS decides reachability in poly time!).

### Theorem 8.27 (Immerman–Szelepcsényi Theorem): $\text{NL} = \text{coNL}$!
**Goal:** Show $\overline{PATH} = \{\langle G, s, t \rangle \mid \text{NO path from } s \text{ to } t\} \in \text{NL}$.

**Intuition (Inductive Counting):** You only have $O(\log n)$ memory — you **cannot** store a `visited[]` array of $n$ bits! Instead, you compute the exact **count** $c_i$ = number of vertices reachable from $s$ in $\le i$ steps!
1. Start with $c_0 = 1$ (only $s$).
2. **Inductive step (compute $c_i$ from $c_{i-1}$):** Initialize $c_i = 0$. Loop over every candidate vertex $v \in V$:
   - To check whether $v$ is reachable in $\le i$ steps without missing any predecessor, loop over all $u \in V$ and nondeterministically guess a path of length $\le i-1$ from $s$ to $u$.
   - Count how many distinct $u$'s you successfully verified (`seen_prev`). If `seen_prev < c_{i-1}`, **reject this branch** (you missed a reachable vertex!).
   - If any verified $u$ equals $v$ or has an edge $(u, v)$, increment $c_i$.
3. **Final check (using $c_{n-1}$):** Loop over all $u \in V$, verify paths of length $\le n-1$ to $c_{n-1}$ distinct vertices; if any equals $t$, reject; if all $c_{n-1}$ verified vertices are $\neq t$, **accept**!
- Total space: a handful of $O(\log n)$-bit counters (`i`, `v`, `u`, `c_prev`, `c_cur`, `seen_prev`)! $\blacksquare$

*Code:* `immerman_szelepcsenyi_non_reach` in [ch08.rs](../src/ch08.rs) and [ch08.py](../python/ch08.py).

---

## Common Final Exam Pitfalls
1. **Confusing $\text{NL} = \text{coNL}$ with $\text{NP} = \text{coNP}$:** Immerman–Szelepcsényi works for **space** (and implies $\text{NSPACE}(f(n)) = \text{coNSPACE}(f(n))$ for $f(n) \ge \log n$), NOT for time! $\text{NP} \stackrel{?}{=} \text{coNP}$ remains wide open.
2. **Forgetting the read-only input tape** when defining $\text{L}$ and $\text{NL}$.
3. **Using $\le_P$ for NL-completeness:** Never! Because $\text{NL} \subseteq \text{P}$, polynomial-time reductions would collapse all nontrivial problems in $\text{NL}$ into NL-complete. Always use **log-space reductions ($\le_L$)** for NL-completeness!

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Does Savitch's Theorem imply $\text{L} = \text{NL}$?  
   *Outline:* **NO!** For $f(n) = \log n$, Savitch gives $\text{NL} = \text{NSPACE}(\log n) \subseteq \text{SPACE}(\log^2 n)$, which is larger than $\text{L} = \text{SPACE}(\log n)$.
2. **Q:** Why does `eval_tqbf` use only polynomial space even though it may take exponential time?  
   *Outline:* At any moment, only one path from root to leaf of the quantifier recursion tree is stored on the stack (depth $\le n$). When the `x = false` branch finishes, its memory is freed and reused by the `x = true` branch.
3. **Q:** Show that $E_{\text{DFA}} \in \text{NL}$ and $EQ_{\text{DFA}} \in \text{NL}$.  
   *Outline:* $\overline{E_{\text{DFA}}}$ is literally $PATH$ on the DFA's state graph (guess a path from $q_0$ to $F$), so $\overline{E_{\text{DFA}}} \in \text{NL}$. By Immerman–Szelepcsényi ($\text{NL} = \text{coNL}$), $E_{\text{DFA}} \in \text{NL}$! Similarly, $\overline{EQ_{\text{DFA}}}$ guesses a string leading to a disagreeing pair $(x, y)$ in the product graph in $O(\log n)$ space, so $EQ_{\text{DFA}} \in \text{coNL} = \text{NL}$!
