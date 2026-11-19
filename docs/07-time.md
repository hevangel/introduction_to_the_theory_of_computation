# Chapter 7 — Time Complexity (P, NP, and NP-Completeness)

**All three editions (1e, 2e, 3e; 2e renames §7.5 to *Additional NP-complete Problems*, same core content).**  
**Code:** [src/ch07.rs](../src/ch07.rs) (Rust) · [python/ch07.py](../python/ch07.py) (Python)

> **Big Picture:** Even if a problem is decidable, an algorithm taking $2^n$ steps is useless for $n = 100$. Chapter 7 classifies decidable problems by **worst-case running time**, introduces the two most famous classes in computer science (**P** and **NP**), proves the **Cook–Levin Theorem** ($SAT$ is NP-complete), and builds a web of polynomial-time reductions across graph, logic, and number problems.

---

## 7.1 Measuring Complexity

- **Time complexity** $f(n)$ of a decider $M$: the maximum number of steps $M$ takes on any input of length $n$.
- **Asymptotic notation:**
  - $f(n) = O(g(n))$: $\exists c > 0, n_0$ such that $f(n) \le c \cdot g(n)$ for all $n \ge n_0$.
  - $f(n) = o(g(n))$: $\lim_{n \to \infty} f(n)/g(n) = 0$ (strictly smaller order).
- **Time complexity class:** $\text{TIME}(t(n)) = \{L \mid L \text{ is decided by a } O(t(n))\text{-time deterministic single-tape TM}\}$.
- **Model-dependence vs. Polynomial Robustness:**
  - Theorem 7.8: Every $t(n)$-time **multitape** TM has an equivalent **$O(t(n)^2)$-time** single-tape TM.
  - Theorem 7.11: Every $t(n)$-time **nondeterministic** single-tape TM has an equivalent **$2^{O(t(n))}$-time** deterministic single-tape TM.
  - Because deterministic models differ by at most a **polynomial** factor, the class **P** is **model-independent**!

---

## 7.2 The Class P

$$\text{P} = \bigcup_{k \ge 1} \text{TIME}(n^k)$$
**Intuition:** Problems **solvable** in polynomial time on a deterministic computer.

### Classic Problems in P (Know These for Exams!)
1. **$PATH = \{\langle G, s, t \rangle \mid G \text{ has a directed path from } s \text{ to } t\}$:** BFS/DFS in $O(|V| + |E|)$.
2. **$RELPRIME = \{\langle x, y \rangle \mid \gcd(x, y) = 1\}$:** Euclidean algorithm in $O(\log(\max(x, y)))$ — **Crucial:** input size is $\log x + \log y$ (binary encoding), NOT $x + y$! Trial division takes $O(\sqrt{x}) = O(2^{n/2})$ (**exponential** in input length!).
3. **Every CFL is in P (Theorem 7.16):** Convert fixed CFG $G$ to CNF and run **CYK** in $O(n^3)$ time!
4. **$2\text{SAT} \in \text{P}$:** Build the **implication graph** (each clause $(a \lor b)$ adds edges $\neg a \to b$ and $\neg b \to a$), find Strongly Connected Components (Tarjan's algorithm) in $O(|V| + |E|)$ time; satisfiable iff no variable $x$ shares an SCC with $\neg x$!  
   *Code:* `two_sat_solve` in [ch07.rs](../src/ch07.rs) and [ch07.py](../python/ch07.py).

---

## 7.3 The Class NP

### Two Equivalent Definitions of NP (Theorem 7.20 — KNOW BOTH!)
1. **Verifier Definition (Def 7.18):** $A \in \text{NP}$ iff there exists a **polynomial-time deterministic verifier** $V$ such that:
   $$A = \{w \mid \exists c \text{ with } |c| \le \text{poly}(|w|) \text{ such that } V \text{ accepts } \langle w, c \rangle\}$$
   Here $c$ is a **certificate (witness)** — a short proof of membership!
2. **Nondeterministic TM Definition:** $\text{NP} = \bigcup_{k \ge 1} \text{NTIME}(n^k)$ — languages decided in polynomial time by a **nondeterministic TM** (guess certificate $c$ in poly time, then run deterministic verifier $V$!).

**Examples of NP verifiers (implemented in code!):**
- **$CLIQUE$:** Certificate = set of $k$ vertices; verify all $\binom{k}{2}$ edges exist (`verifies_clique`).
- **$VERTEX\text{-}COVER$:** Certificate = set of $k$ vertices; verify every edge has an endpoint in the set (`verifies_vertex_cover`).
- **$HAMPATH$:** Certificate = permutation of $n$ vertices starting at $s$ and ending at $t$; verify all consecutive edges exist (`verifies_hampath`).
- **$SUBSET\text{-}SUM$:** Certificate = subset of numbers; verify sum equals $t$.
  - **HUGE EXAM TRAP:** Why doesn't our $O(n \cdot t)$ DP (`subset_sum_nonnegative`) prove $\text{P} = \text{NP}$? Because the input is encoded in **binary** (length $L \approx \log_2 t$), so $t \approx 2^L$! Thus $O(n \cdot t)$ is **pseudo-polynomial** (exponential in input bit-length $L$)!

### $\text{P}$ vs. $\text{NP}$ and $\text{coNP}$
- Clearly $\text{P} \subseteq \text{NP}$ (ignore certificate, run P decider) and $\text{P} \subseteq \text{coNP}$ ($\text{coNP} = \{\overline{A} \mid A \in \text{NP}\}$).
- Open Millennium Prize questions: Is $\text{P} = \text{NP}$? Is $\text{NP} = \text{coNP}$? (If $\text{P} = \text{NP}$, then $\text{NP} = \text{coNP}$ since P is closed under complement.)

---

## 7.4 NP-Completeness & The Cook–Levin Theorem

### Polynomial-Time Mapping Reducibility ($\le_P$)
$A \le_P B$ if there is a **polynomial-time computable** function $f : \Sigma^* \to \Sigma^*$ such that $w \in A \iff f(w) \in B$.
- **Theorem 7.31:** If $A \le_P B$ and $B \in \text{P}$, then $A \in \text{P}$.

### Definition of NP-Complete (Def 7.34)
A language $B$ is **NP-complete** if:
1. **$B \in \text{NP}$** (don't forget this step on exams — show a polynomial-time verifier!), AND
2. **$B$ is NP-hard:** every $A \in \text{NP}$ satisfies $A \le_P B$ (in practice, prove $C \le_P B$ for some *already known* NP-complete problem $C$, by transitivity of $\le_P$!).

### Theorem 7.37 (Cook–Levin Theorem): $SAT$ and $3SAT$ are NP-Complete!
**Proof Blueprint (Tableau Method — Understand the 4 Constraints!):**
Given any $A \in \text{NP}$ decided by NTM $N$ in $n^k$ steps, build a CNF formula $\phi$ simulating an $n^k \times n^k$ **tableau** (grid of cells, where row $i$ is configuration $C_i$ of $N$ on $w$). Boolean variable $x_{i, j, s}$ means "cell $(i, j)$ holds symbol $s$".
$$\phi = \phi_{\text{cell}} \land \phi_{\text{start}} \land \phi_{\text{move}} \land \phi_{\text{accept}}$$
1. **$\phi_{\text{cell}}$:** Every cell $(i, j)$ contains **at least one** and **at most one** symbol.
2. **$\phi_{\text{start}}$:** Row 1 equals the start configuration $\# q_0 w_1 \dots w_n \sqcup \dots \sqcup \#$.
3. **$\phi_{\text{move}}$:** Every $2 \times 3$ window of adjacent cells across rows $i$ and $i+1$ is **legal** according to $\delta$ of $N$! (Why $2 \times 3$? A tape head moves at most 1 cell left or right per step, so legality is completely local!)
4. **$\phi_{\text{accept}}$:** At least one cell in the tableau contains $q_{\text{accept}}$.
- Total size of $\phi$ is $O(n^{2k})$ (polynomial!), and $\phi$ is satisfiable $\iff N$ accepts $w$.
- Converting arbitrary CNF clauses to 3-literal clauses (splitting $(a \lor b \lor c \lor d)$ into $(a \lor b \lor z) \land (\neg z \lor c \lor d)$) gives **$3SAT$ is NP-complete**!

---

## 7.5 Additional NP-Complete Problems (The Classic Reduction Web!)

Memorize this reduction tree — almost every final exam asks you to reproduce or adapt one of these!

```text
SAT ──► 3SAT ──┬──► CLIQUE ──► VERTEX-COVER
               ├──► HAMPATH ──► UHAMPATH
               └──► SUBSET-SUM
```

1. **$3SAT \le_P CLIQUE$ (Thm 7.32):**  
   Each of the $k$ clauses becomes a group of 3 vertices (one per literal). Connect every pair of vertices in **different** groups EXCEPT contradictory literals ($x_i$ and $\neg x_i$).  
   $\phi$ is satisfiable $\iff G$ has a $k$-clique!  
   *Code:* `three_sat_to_clique` in [ch05.rs](../src/ch05.rs) and [ch05.py](../python/ch05.py).
2. **$CLIQUE \le_P VERTEX\text{-}COVER$ (via Complement Graph):**  
   $S$ is a $k$-clique in $G$ $\iff$ $S$ is an independent set of size $k$ in $\overline{G}$ $\iff$ $V \setminus S$ is a **vertex cover of size $|V| - k$ in $\overline{G}$**!  
   *Code:* `clique_to_vertex_cover` in [ch07.rs](../src/ch07.rs) and [ch07.py](../python/ch07.py).
3. **$3SAT \le_P SUBSET\text{-}SUM$ (Thm 7.56):**  
   Create a table of base-10 integers with $l$ variable columns and $k$ clause columns (no carries because each column sums to $\le 6 < 10$!).
   - For each variable $x_i$, add $y_i$ (1 in col $i$, and 1 in each clause col where $x_i$ appears) and $z_i$ (1 in col $i$, and 1 in each clause col where $\neg x_i$ appears).
   - For each clause $c_j$, add two slack numbers $g_j = h_j = 1$ in col $c_j$.
   - Target $t$: `1` in every variable column (forces picking *either* $y_i$ or $z_i$, never both!) and `3` in every clause column (achievable using slack $\in \{0, 1, 2\}$ iff at least 1 true literal hits $c_j$!).  
   *Code:* `three_sat_to_subset_sum` in [ch07.rs](../src/ch07.rs) and [ch07.py](../python/ch07.py).
4. **$3SAT \le_P HAMPATH$ (Thm 7.46):**  
   Build a diamond chain gadget for each variable $x_i$ (traverse left-to-right = True, right-to-left = False) and a clause node $c_j$ that can be detoured into iff a literal in $c_j$ is traversed in the satisfying direction.

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Prove $INDEPENDENT\text{-}SET = \{\langle G, k \rangle \mid G \text{ has } k \text{ pairwise non-adjacent vertices}\}$ is NP-complete.  
   *Outline:* (1) In NP: certificate = $k$ vertices; check no edge between any pair in $O(k^2)$ time. (2) NP-hard: reduce $CLIQUE \le_P INDEPENDENT\text{-}SET$ via $f(\langle G, k \rangle) = \langle \overline{G}, k \rangle$ (poly time).
2. **Q:** Why does the reduction $3SAT \le_P 2SAT$ fail?  
   *Outline:* Because $2SAT \in \text{P}$ (solvable via Tarjan's SCC on the implication graph), so if $3SAT \le_P 2SAT$ existed, it would prove $\text{P} = \text{NP}$!
3. **Q:** Suppose you prove $A \le_P B$ where $B$ is NP-complete. Does that prove $A$ is NP-complete?  
   *Outline:* **NO!** That only proves $A \in \text{NP}$ (if $B \in \text{NP}$). To prove $A$ is NP-hard, you must reduce **FROM** an NP-complete problem **TO** $A$: $B \le_P A$!
