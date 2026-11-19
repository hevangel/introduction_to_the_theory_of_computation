# Chapter 4 — Decidability

**All three editions (1e, 2e, 3e).**  
**Code:** [src/ch04.rs](../src/ch04.rs) (Rust) · [python/ch04.py](../python/ch04.py) (Python)

> **Big Picture:** Now that we have Turing machines, we ask: *Which computational problems about automata, grammars, and programs can be solved algorithmically (decided), and which ones are fundamentally impossible?* Chapter 4 builds the master decidability table for Regular and Context-Free languages, then uses **Cantor's diagonalization** to prove our very first undecidable problem: $A_{\text{TM}}$.

---

## 4.1 Decidable Languages (Regular & Context-Free)

### Decidable Problems for Regular Languages
Every standard question about DFAs, NFAs, and Regexes is **decidable**!

| Problem | Language Definition | Decider Algorithm (Proof Blueprint) |
|---|---|---|
| **$A_{\text{DFA}}$** (Thm 4.1) | $\{\langle B, w \rangle \mid B \text{ is a DFA accepting } w\}$ | Simulate $B$ on $w$ step-by-step; accept iff $B$ ends in $F$. Always halts in $|w|$ steps! |
| **$A_{\text{NFA}}$** (Thm 4.2) | $\{\langle B, w \rangle \mid B \text{ is an NFA accepting } w\}$ | Convert NFA $B$ to equivalent DFA $D$ (subset construction), then run $A_{\text{DFA}}$ decider on $\langle D, w \rangle$. |
| **$A_{\text{REX}}$** (Thm 4.3) | $\{\langle R, w \rangle \mid R \text{ is a regex generating } w\}$ | Convert regex $R$ to NFA $N$ (Thompson), then run $A_{\text{NFA}}$ decider on $\langle N, w \rangle$. |
| **$E_{\text{DFA}}$** (Thm 4.4) | $\{\langle A \rangle \mid A \text{ is a DFA and } L(A) = \emptyset\}$ | Run BFS/DFS from $q_0$ to mark all reachable states. Accept iff **no accept state is marked**. |
| **$EQ_{\text{DFA}}$** (Thm 4.5) | $\{\langle A, B \rangle \mid A, B \text{ are DFAs and } L(A) = L(B)\}$ | Build DFA $C$ for the **symmetric difference** $L(C) = (L(A) \cap \overline{L(B)}) \cup (\overline{L(A)} \cap L(B))$ using product + complement; run $E_{\text{DFA}}$ decider on $\langle C \rangle$! |

*Code:* `a_dfa`, `e_dfa`, `eq_dfa`, `dfa_equivalent` in [ch04.rs](../src/ch04.rs) and [ch04.py](../python/ch04.py).

### Decidable Problems for Context-Free Languages
For CFGs, **membership** and **emptiness** are decidable, whereas **universality ($ALL_{\text{CFG}}$)** and **equivalence ($EQ_{\text{CFG}}$)** are **UNDECIDABLE** (proved in Chapter 5)!

| Problem | Language Definition | Decider Algorithm (Proof Blueprint) |
|---|---|---|
| **$A_{\text{CFG}}$** (Thm 4.7) | $\{\langle G, w \rangle \mid G \text{ is a CFG generating } w\}$ | Convert $G$ to **Chomsky Normal Form** $G'$. Any derivation of $w$ ($|w| = n \ge 1$) takes **exactly $2n - 1$ steps**! Check all derivations of length $2n-1$ (or run **CYK** in $O(n^3)$ time). For $w = \varepsilon$, check if $S_0 \to \varepsilon$. |
| **$E_{\text{CFG}}$** (Thm 4.8) | $\{\langle G \rangle \mid G \text{ is a CFG and } L(G) = \emptyset\}$ | **Bottom-up marking:** (1) Mark all terminals. (2) Repeat: mark variable $A$ if there is a rule $A \to U_1 \dots U_k$ where *every* $U_i$ is already marked. (3) Accept iff $S$ is **not** marked! |
| **Every CFL is decidable** (Thm 4.9) | For any fixed CFG $G$, $L(G)$ is decidable | Hardcode $\langle G \rangle$ and run the $A_{\text{CFG}}$ decider on $\langle G, w \rangle$! |

*Code:* `a_cfg` and `e_cfg` in [ch04.rs](../src/ch04.rs) and [ch04.py](../python/ch04.py).

**Why does naive BFS over derivations FAIL for $A_{\text{CFG}}$?**  
Because a general CFG can have $\varepsilon$-rules ($A \to \varepsilon$) or unit cycles ($A \to B \to A$), allowing arbitrarily long derivations of a short string! Converting to **CNF** eliminates those and guarantees exact derivation length $2|w| - 1$.

---

## 4.2 Undecidability & Diagonalization

### Countable vs. Uncountable Sets (Cantor's Theorem)
- A set $A$ is **countable** if it is finite or has a bijection with $\mathbb{N} = \{1, 2, 3, \dots\}$.
  - $\mathbb{N}$, $\mathbb{Z}$, $\mathbb{Q}$, $\Sigma^*$, and **the set of all Turing Machines** are all **countable** (every TM has a finite string encoding $\langle M \rangle \in \Sigma^*$).
- **Theorem 4.17 ($\mathbb{R}$ is uncountable):** Cantor's diagonalization shows no list can contain all real numbers in $(0, 1)$ (construct $x$ whose $i$-th decimal digit differs from the $i$-th digit of $f(i)$).
- **Corollary 4.18 (Non-Turing-recognizable languages exist!):**
  - The set of all TMs is **countable**.
  - The set of all languages $\mathcal{B} = \mathcal{P}(\Sigma^*)$ is **uncountable** (bijection with infinite binary sequences $\chi_L$, diagonalized identically).
  - Therefore, almost all languages are NOT even Turing-recognizable!

### Theorem 4.11: $A_{\text{TM}}$ is Undecidable!
$$A_{\text{TM}} = \{\langle M, w \rangle \mid M \text{ is a TM and } M \text{ accepts } w\}$$
Note: $A_{\text{TM}}$ **IS** Turing-recognizable via the **Universal Turing Machine $U$** (simulate $M$ on $w$; if $M$ accepts, accept; if $M$ rejects, reject; if $M$ loops, $U$ loops).

**Proof by Contradiction & Diagonalization (MEMORIZE THIS WORD FOR WORD FOR THE FINAL!):**
1. Assume for contradiction that a decider $H$ decides $A_{\text{TM}}$:
   $$H(\langle M, w \rangle) = \begin{cases} \text{accept} & \text{if } M \text{ accepts } w \\ \text{reject} & \text{if } M \text{ does not accept } w \end{cases}$$
2. Construct a new TM $D$ that uses $H$ as a subroutine:
   - $D = \text{"On input } \langle M \rangle \text{, where } M \text{ is a TM:}$
     1. Run $H$ on input $\langle M, \langle M \rangle \rangle$.
     2. Output the **opposite** of what $H$ outputs (if $H$ accepts, **reject**; if $H$ rejects, **accept**)."
3. Run $D$ on its **own description** $\langle D \rangle$:
   - $D$ accepts $\langle D \rangle \iff H$ rejects $\langle D, \langle D \rangle \rangle \iff D$ does **not** accept $\langle D \rangle$!
4. Contradiction! Therefore, no decider $H$ for $A_{\text{TM}}$ can exist. $\blacksquare$

### Theorem 4.22 (Co-Turing-Recognizability Theorem — HUGE EXAM FAVORITE!)
A language $A$ is **co-Turing-recognizable** if its complement $\overline{A}$ is Turing-recognizable.

> **Theorem 4.22:** $A$ is **decidable** $\iff$ $A$ is **both Turing-recognizable AND co-Turing-recognizable**.

**Proof Blueprint:**
- **($\Rightarrow$):** If $A$ is decidable, both $A$ and $\overline{A}$ are decidable, hence both are Turing-recognizable.
- **($\Leftarrow$):** Let $M_1$ recognize $A$ and $M_2$ recognize $\overline{A}$. Build decider $M$ on input $w$:
  - **Run $M_1(w)$ and $M_2(w)$ in parallel (alternating 1 step each).**
  - If $M_1$ accepts, **accept**; if $M_2$ accepts, **reject**.
  - Since every $w \in \Sigma^*$ belongs to either $A$ or $\overline{A}$, **at least one** of $M_1, M_2$ is guaranteed to accept in finite time, so $M$ always halts! $\blacksquare$

**Immediate Corollary 4.23:** $\overline{A_{\text{TM}}}$ is **NOT Turing-recognizable**!  
*Proof:* $A_{\text{TM}}$ is Turing-recognizable. If $\overline{A_{\text{TM}}}$ were also Turing-recognizable, Theorem 4.22 would imply $A_{\text{TM}}$ is decidable — contradiction!

---

## Final Exam Master Classification Table (Ch 4)

| Language | Decidable? | Recognizable? | co-Recognizable? |
|---|---|---|---|
| $A_{\text{DFA}}, E_{\text{DFA}}, EQ_{\text{DFA}}, A_{\text{NFA}}, A_{\text{REX}}$ | **Yes** | Yes | Yes |
| $A_{\text{CFG}}, E_{\text{CFG}}$ | **Yes** | Yes | Yes |
| $A_{\text{TM}}$ | **No** | **Yes** | **No** |
| $\overline{A_{\text{TM}}}$ | **No** | **No** | **Yes** |

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Show that $INFINITE_{\text{DFA}} = \{\langle A \rangle \mid A \text{ is a DFA and } L(A) \text{ is infinite}\}$ is decidable.  
   *Outline:* Method 1 (Pumping Lemma): A $p$-state DFA accepts an infinite language iff it accepts some string of length $p \le |w| < 2p$; test all finitely many such strings using $A_{\text{DFA}}$. Method 2 (Graph): $L(A)$ is infinite iff there is a cycle reachable from $q_0$ that can reach an accept state.
2. **Q:** Show that $EQ_{\text{DFA}}$ is decidable *without* using complement/symmetric difference.  
   *Outline:* Run BFS on the product state space $Q_A \times Q_B$ starting from $(q_{0,A}, q_{0,B})$; reject if any reachable pair $(x, y)$ has $(x \in F_A) \neq (y \in F_B)$, else accept (this is `dfa_equivalent` in our code!).
3. **Q:** Prove that if $A$ is Turing-recognizable and $A \le_m \overline{A}$ (or $A$ is co-recognizable), then $A$ is decidable.  
   *Outline:* Direct application of Theorem 4.22 (dovetail recognizers for $A$ and $\overline{A}$).
