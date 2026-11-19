# Chapter 9 — Intractability

**All three editions (1e, 2e, 3e).**  
**Code:** [src/ch09.rs](../src/ch09.rs) (Rust) · [python/ch09.py](../python/ch09.py) (Python)

> **Big Picture:** In Chapters 7–8, we couldn't prove $\text{P} \neq \text{NP}$ or $\text{NP} \neq \text{PSPACE}$. Can we prove *any* decidable problem is provably intractable (strictly harder than P)? **YES!** Chapter 9 proves unconditional separations using **diagonalization** (Hierarchy Theorems), explains why diagonalization *alone* cannot resolve $\text{P} \stackrel{?}{=} \text{NP}$ (**Relativization / Baker–Gill–Solovay**), and introduces **Circuit Complexity** as a non-relativizing alternative.

---

## 9.1 Hierarchy Theorems (More Time/Space = Strictly More Power!)

### Constructible Functions
To prevent pathological functions that take too long just to compute their own resource bound:
- **$f(n)$ is space-constructible** ($f(n) \ge \log n$) if the map $1^n \mapsto \text{binary}(f(n))$ is computable in $O(f(n))$ space.
- **$t(n)$ is time-constructible** ($t(n) \ge n \log n$) if $1^n \mapsto \text{binary}(t(n))$ is computable in $O(t(n))$ time.
- All standard bounds ($n, n^2, n^k, 2^n, \log n$) are constructible.

### Theorem 9.3 (Space Hierarchy Theorem)
For any space-constructible $f : \mathbb{N} \to \mathbb{N}$, there exists a language $A$ decidable in $O(f(n))$ space but **NOT** in $o(f(n))$ space:
$$\text{SPACE}(o(f(n))) \subsetneq \text{SPACE}(f(n))$$
- **Proof idea (Diagonalization with Space Counter):** Build TM $D$ on input $w$:
  1. Compute $f(|w|)$ and mark off $f(|w|)$ tape cells (if a simulation ever tries to exceed them, reject).
  2. Parse $w = \langle M \rangle 10^*$ (the padding $10^*$ ensures $M$ is tested on arbitrarily long inputs!).
  3. Simulate $M$ on $w$ for at most $2^{f(|w|)}$ steps (to guarantee $D$ halts even if $M$ loops in $f(|w|)$ space!).
  4. If $M$ accepts, **reject**; if $M$ rejects, **accept**!
- **Immediate Corollaries:**
  - $\text{NL} \subsetneq \text{PSPACE}$ (since $\text{NL} \subseteq \text{SPACE}(\log^2 n) \subsetneq \text{SPACE}(n) \subseteq \text{PSPACE}$).
  - $\text{PSPACE} \subsetneq \text{EXPSPACE}$.

### Theorem 9.10 (Time Hierarchy Theorem)
For any time-constructible $t : \mathbb{N} \to \mathbb{N}$, there exists a language $A$ decidable in $O(t(n))$ time but **NOT** in $o(t(n) / \log t(n))$ time:
$$\text{TIME}\!\left(o\!\left(\frac{t(n)}{\log t(n)}\right)\right) \subsetneq \text{TIME}(t(n))$$
- **Why the extra $\log t(n)$ factor?** Simulating an arbitrary $k$-tape TM $M$ on a fixed universal TM incurs a $O(\log t(n))$ overhead (Hennie–Stearns simulation) plus maintaining a binary step clock!
- **Immediate Corollary:** **$\text{P} \subsetneq \text{EXPTIME}$** (provably intractable decidable problems exist!).
- **EXPSPACE-complete example:** Equivalence of regular expressions with **exponentiation** ($EQ_{\text{REX}\uparrow}$, where $R^k$ abbreviates $k$ copies) is EXPSPACE-complete — provably requires exponential space (and thus is NOT in P)!

---

## 9.2 Relativization — Why Diagonalization Fails for $\text{P} \stackrel{?}{=} \text{NP}$

Recall oracle machines $M^A$ from Chapter 6. Let $\text{P}^A$ and $\text{NP}^A$ be the classes P and NP when machines are given an oracle for language $A$.

### Theorem 9.18 (Baker–Gill–Solovay Theorem — 1975)
There exist two oracles $A$ and $B$ such that:
1. **$\text{P}^A = \text{NP}^A$** (take $A = TQBF$! Since $TQBF$ is PSPACE-complete, $\text{P}^{TQBF} = \text{NP}^{TQBF} = \text{PSPACE}$!).
2. **$\text{P}^B \neq \text{NP}^B$**!

**How Oracle $B$ is Constructed (Key Exam Insight!):**
Define the unary language $U_B = \{1^n \mid \exists w \in B \text{ with } |w| = n\}$.
- Clearly $U_B \in \text{NP}^B$ for **every** oracle $B$: just nondeterministically guess a string $w$ of length $n$ and query the oracle "$w \in B$?" in 1 step!
- Now construct $B$ in stages $i = 1, 2, 3, \dots$ to diagonalize against every polynomial-time oracle machine $M_i^B$ (running in time $n^i$):
  - Pick $n$ large enough that $n^i < 2^n$ (and larger than any previous oracle query).
  - Run $M_i^B(1^n)$. Whenever $M_i$ queries a string not yet fixed in $B$, answer **NO** and lock it out of $B$.
  - Since $M_i$ makes at most $n^i < 2^n$ queries, at least one string $w^*$ of length $n$ was **never queried**!
  - If $M_i$ accepts $1^n$, leave all length-$n$ strings out of $B$ (so $1^n \notin U_B$).
  - If $M_i$ rejects $1^n$, put $w^*$ into $B$ (so $1^n \in U_B$)!
- Thus NO polynomial-time deterministic oracle TM decides $U_B$, proving $\text{P}^B \neq \text{NP}^B$! $\blacksquare$

*Code:* `oracle_exists_of_length` in [ch09.rs](../src/ch09.rs) and [ch09.py](../python/ch09.py).

**The Philosophical Moral (RELATIVIZATION BARRIER):**  
Any proof technique (like standard diagonalization or simulation) that treats Turing machines as black boxes **relativizes** — it would still hold verbatim with any oracle attached! Because $\text{P}^A = \text{NP}^A$ and $\text{P}^B \neq \text{NP}^B$ both exist, **no relativizing proof can EVER solve $\text{P} \stackrel{?}{=} \text{NP}$**!

---

## 9.3 Circuit Complexity

### Boolean Circuits
- A **Boolean circuit** is a DAG of `AND`, `OR`, and `NOT` gates (fan-in $\le 2$ for AND/OR, 1 for NOT) computing a Boolean function $f : \{0, 1\}^n \to \{0, 1\}$.
- **Size** = number of gates; **Depth** = longest input-to-output path.
- A **circuit family** $C = (C_0, C_1, C_2, \dots)$ has one circuit $C_n$ for each input length $n$ (a **non-uniform** model of computation!).
- **Theorem 9.30:** Every language in $\text{TIME}(t(n))$ has circuits of size $O(t(n)^2)$. Thus $\text{P} \subseteq \text{P/poly}$ (polynomial-size circuit families).
  - **Consequence:** Proving that *any* problem in NP (e.g., $SAT$ or $CLIQUE$) requires super-polynomial circuit size would immediately prove **$\text{P} \neq \text{NP}$**!
- **Warning (Non-uniformity!):** Because each $C_n$ can be chosen independently, $\text{P/poly}$ even contains **undecidable** unary languages like $\{1^n \mid \text{the } n\text{-th TM halts on } \varepsilon\}$!

### Shannon's Counting Argument (Almost All Functions Need Exponential Circuits!)
- How many Boolean functions $f : \{0, 1\}^n \to \{0, 1\}$ exist? Exactly **$2^{2^n}$**! (For $n = 3$: $2^8 = 256$; for $n = 6$: $2^{64} \approx 1.8 \times 10^{19}$.)
- How many circuits of size $s$ exist? At most $(3(n + s)^2)^s \approx s^{O(s)}$.
- When $s \ll 2^n / n$, the number of circuits of size $s$ is **tiny** compared to $2^{2^n}$. Therefore, **almost every Boolean function requires exponential circuit size $\Omega(2^n / n)$** — yet proving even a $5n$ lower bound for an *explicit* function in NP remains a major open problem!

*Code:* `boolean_function_count`, `BooleanCircuit`, `xor_circuit` in [ch09.rs](../src/ch09.rs) and [ch09.py](../python/ch09.py).

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Why can't we use the Time Hierarchy Theorem to prove $\text{P} \neq \text{NP}$?  
   *Outline:* The Time Hierarchy Theorem separates deterministic time classes ($\text{TIME}(n^k) \subsetneq \text{TIME}(n^{k+1})$), giving $\text{P} \subsetneq \text{EXPTIME}$, but it does not tell us where $\text{NP}$ sits between $\text{P}$ and $\text{EXPTIME}$ (and standard diagonalization relativizes!).
2. **Q:** Why does the Space Hierarchy proof pad inputs as $\langle M \rangle 10^*$?  
   *Outline:* Because $M$ may use $c \cdot g(n)$ space where the hidden constant $c$ is arbitrarily large; padding guarantees infinitely many long inputs where $c \cdot g(n) < f(n)$ eventually kicks in!
3. **Q:** Show that a unary undecidable language can still have polynomial-size circuits ($\text{P/poly}$).  
   *Outline:* For any unary language $U \subseteq \{1\}^*$, for each length $n$ either $1^n \in U$ (use constant-1 circuit, size 1) or $1^n \notin U$ (use constant-0 circuit, size 1). Thus EVERY unary language has $O(1)$-size circuits!
