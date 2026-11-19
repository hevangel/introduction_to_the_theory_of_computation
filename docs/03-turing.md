# Chapter 3 — The Church–Turing Thesis

**All three editions (1e, 2e, 3e).**  
**Code:** [src/ch03.rs](../src/ch03.rs) (Rust) · [python/ch03.py](../python/ch03.py) (Python)

> **Big Picture:** Finite automata have fixed memory; PDAs have one restricted LIFO stack. By allowing **random-access read/write memory on an unbounded tape**, we arrive at the **Turing Machine (TM)** — a model so powerful that *every* reasonable general-purpose computing model (multitape TMs, nondeterministic TMs, RAMs, Python, Rust, lambda calculus) computes the **exact same class of languages**.

---

## 3.1 Turing Machines

### Formal Definition (Sipser Def 3.3)
A **Turing Machine** is a 7-tuple $M = (Q, \Sigma, \Gamma, \delta, q_0, q_{\text{accept}}, q_{\text{reject}})$ where:
1. $Q$ is a finite set of states.
2. $\Sigma$ is the **input alphabet**, NOT containing the blank symbol $\sqcup$.
3. $\Gamma$ is the **tape alphabet**, with $\sqcup \in \Gamma$ and $\Sigma \subseteq \Gamma$.
4. $\delta : Q \times \Gamma \to Q \times \Gamma \times \{\text{L}, \text{R}\}$ is the transition function.
5. $q_0 \in Q$ is the start state.
6. $q_{\text{accept}} \in Q$ and $q_{\text{reject}} \in Q$ are the accept and reject states, with $q_{\text{accept}} \neq q_{\text{reject}}$.

### Configurations & Computation
- A **configuration** $uqv$ represents: current state $q$, tape contents $uv$ (followed by infinitely many blanks), and tape head positioned on the **first symbol of $v$**.
- **Start configuration:** $q_0 w$.
- **Halting configurations:** Any configuration whose state is $q_{\text{accept}}$ or $q_{\text{reject}}$ (these take effect **immediately**, unlike DFAs/PDAs which wait for the end of input!).
- Three possible outcomes on input $w$: **accept**, **reject**, or **loop forever** (never halt).

### Turing-Recognizable vs. Turing-Decidable (CRITICAL EXAM DISTINCTION!)
- **Turing-recognizable (recursively enumerable / r.e.):** $L = L(M)$ for some TM $M$ that accepts all $w \in L$, and either rejects *or loops* on $w \notin L$.
- **Turing-decidable (recursive):** $L = L(M)$ for some **decider** $M$ that **halts on every input** (accepts $w \in L$, rejects $w \notin L$).
- Every decidable language is recognizable, but (as Chapter 4 proves!) **not every recognizable language is decidable** ($A_{\text{TM}}$ is recognizable but undecidable!).

### Worked Textbook TMs (Implemented in Rust & Python!)
1. **$M_2$ deciding $A = \{0^{2^n} \mid n \ge 0\}$:**  
   Repeatedly sweep left-to-right across the tape crossing off every other `0` with `x`.
   - If a sweep finds **exactly one** `0`, **accept**.
   - If a sweep finds an **odd number $> 1$** of `0`s, **reject**.
   - Otherwise rewind head to the left end and repeat.  
   *Code:* `tm_power_of_two_zeros` in [ch03.rs](../src/ch03.rs) and [ch03.py](../python/ch03.py).
2. **$M_1$ deciding $B = \{w\#w \mid w \in \{0, 1\}^*\}$:**  
   Zig-zag back and forth matching and crossing off the first unmarked symbol before `#` with the first unmarked symbol after `#`. (Takes $O(n^2)$ time on 1 tape, vs $O(n)$ on 2 tapes — see `two_tape_ww_hash`!)

---

## 3.2 Variants of Turing Machines

**Robustness Theorem:** All the following variants are **equivalent in computability power** to the standard deterministic single-tape TM (they recognize the exact same class of languages, and decide the exact same class of languages!):

### 1. Multitape Turing Machines (Theorem 3.13)
A $k$-tape TM has $k$ tapes and $k$ independent heads: $\delta : Q \times \Gamma^k \to Q \times \Gamma^k \times \{\text{L}, \text{R}, \text{S}\}^k$.
- **Simulation on 1 tape:** Concatenate all $k$ tapes onto a single tape separated by `#`: `# tape_1 # tape_2 # ... # tape_k #`. Mark each head's position using a **dotted symbol** $\dot{a}$. Each multitape step requires sweeping the single tape to read all $k$ dotted symbols, then sweeping again to update them (shifting right if a tape needs to grow).
- *Time overhead:* $t(n)$ steps on $k$ tapes $\to O(t(n)^2)$ steps on 1 tape (polynomial — crucial for Chapter 7!).

### 2. Nondeterministic Turing Machines (Theorem 3.16)
Transition function $\delta : Q \times \Gamma \to \mathcal{P}(Q \times \Gamma \times \{\text{L}, \text{R}\})$. Accepts iff **at least one** branch in the computation tree reaches $q_{\text{accept}}$.
- **Simulation on a Deterministic 3-Tape TM:** Use **Breadth-First Search (BFS)** — NEVER Depth-First Search (DFS)!
  - **Why BFS and NOT DFS? (CLASSIC EXAM QUESTION!)** A branch in the NTM's tree might loop forever! DFS would get stuck down an infinite non-accepting branch and never find an accepting branch to its right. BFS explores all paths of length 1, then length 2, etc., using an address tape over $\{1, \dots, b\}^*$ (where $b$ is max branching factor) to replay paths in shortlex order.
  - **Corollary 3.18:** A language is decidable iff some nondeterministic TM decides it (a nondeterministic *decider* halts on **all** branches, so its tree is finite and BFS halts!).

### 3. Enumerators (Theorem 3.21)
An **enumerator** is a TM with a write-only printer that outputs a (possibly infinite, possibly repeating) list of strings.
- **Theorem 3.21:** A language $A$ is **Turing-recognizable $\iff$ some enumerator enumerates $A$**.
  - **($\Leftarrow$) Enumerator $E \to$ Recognizer $M$:** On input $w$, run $E$; whenever $E$ prints a string equal to $w$, **accept**. (If $w \notin A$, $M$ may loop forever.)
  - **($\Rightarrow$) Recognizer $M \to$ Enumerator $E$:** Use **dovetailing**! Let $s_1, s_2, \dots$ be $\Sigma^*$ in shortlex order. For $i = 1, 2, 3, \dots$: run $M$ for $i$ steps on each of $s_1, s_2, \dots, s_i$; print any string that accepts!
  - **Why dovetailing?** Running $M$ to completion on $s_1$ before trying $s_2$ would hang forever if $M$ loops on $s_1$!
- **Bonus Exam Fact (Problem 3.16):** $A$ is **decidable $\iff$** some enumerator enumerates $A$ in **strictly increasing (lexicographic/shortlex) order**!

---

## 3.3 The Definition of Algorithm & The Church–Turing Thesis

- **Hilbert's 10th Problem (1900):** Find an algorithm to test whether a multivariate polynomial has an integral root ($D = \{p \mid p \text{ has an integral root}\}$).
  - Note: $D$ IS **Turing-recognizable** (just dovetail over all integer tuples!). In 1970, Matiyasevich proved $D$ is **undecidable**.
- **Church–Turing Thesis:** *The intuitive notion of "algorithm" is completely captured by the formal notion of a Turing Machine.* (This is a thesis/definition connecting an informal concept to a formal one — it cannot be mathematically proved, only supported by the equivalence of all known models: TMs, $\lambda$-calculus, RAMs, Post systems, modern programming languages.)
- **Three Levels of TM Descriptions (use High-Level on exams!):**
  1. **Formal:** full 7-tuple and state diagram (only for tiny examples).
  2. **Implementation-level:** English description of tape head movement and marking.
  3. **High-level:** Algorithm pseudocode on encoded objects $\langle O \rangle$ (standard for Chapters 4–10!).

---

## Closure Properties of Decidable & Recognizable Languages

| Operation | Decidable | Turing-Recognizable |
|---|---|---|
| Union ($A \cup B$) | **Yes** | **Yes** (run $M_1, M_2$ in parallel / dovetail!) |
| Intersection ($A \cap B$) | **Yes** | **Yes** (run $M_1$, if accepts run $M_2$) |
| Concatenation ($A \circ B$) | **Yes** | **Yes** (nondeterministically split $w = xy$) |
| Star ($A^*$) | **Yes** | **Yes** (nondeterministically split $w = x_1 \dots x_k$) |
| **Complement ($\overline{A}$)** | **Yes** (swap accept/reject) | **NO!** ($A_{\text{TM}}$ is recognizable, $\overline{A_{\text{TM}}}$ is NOT!) |

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Why can't we prove Turing-recognizable languages are closed under union by running $M_1(w)$ to completion and then running $M_2(w)$?  
   *Outline:* If $w \notin L(M_1)$ and $M_1$ loops forever, we would never start $M_2(w)$ even if $w \in L(M_2)$. Instead, **dovetail** (alternate 1 step of $M_1$ and 1 step of $M_2$) and accept as soon as either accepts.
2. **Q:** Prove that a language $L$ is decidable iff an enumerator prints $L$ in strict shortlex order.  
   *Outline:* ($\Rightarrow$) Iterate $s_1, s_2, \dots \in \Sigma^*$ in shortlex order; run decider $D(s_i)$ and print if it accepts. ($\Leftarrow$) If $L$ is finite, it is regular hence decidable. If $L$ is infinite, on input $w$, run the enumerator until it prints a string $s \ge_{\text{shortlex}} w$; accept iff $s = w$, else reject (guaranteed to halt since $L$ is infinite!).
3. **Q:** What happens if a single-tape TM's transition function allows a `Stay` move `S`? Does that increase computability power?  
   *Outline:* No — replace any `Stay` transition with a `Right` move to a dummy state followed by a `Left` move back to the same cell.
