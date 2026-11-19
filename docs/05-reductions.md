# Chapter 5 — Reducibility & Rice's Theorem

**All three editions (1e, 2e, 3e); 2nd edition adds important solved-problem coverage of Rice's Theorem.**  
**Code:** [src/ch05.rs](../src/ch05.rs) (Rust) · [python/ch05.py](../python/ch05.py) (Python)

> **Big Picture:** Once we know **one** problem is undecidable ($A_{\text{TM}}$), we don't need diagonalization ever again! Instead, we use **reducibility**: show that if you could solve a new problem $B$, you could use it to solve $A_{\text{TM}}$. Therefore $B$ must also be undecidable. **This is the single most tested proof technique on Theory of Computation finals.**

---

## 5.1 Undecidable Problems from Language Theory (Standard Reductions)

### The Golden Rule of Reductions (NEVER GET THIS BACKWARDS!)
To prove a target language $B$ is **undecidable**:
1. **Assume for contradiction** that a decider $R$ decides $B$.
2. **Construct a decider $S$ for a KNOWN undecidable problem** (usually $A_{\text{TM}}$, $HALT_{\text{TM}}$, or $E_{\text{TM}}$) using $R$ as a black-box subroutine!
3. Conclude $B$ must be undecidable.

> **Mnemonic:** Always reduce **KNOWN HARD $\longrightarrow$ TARGET** ($A_{\text{TM}} \le B$). Reducing $B \to A_{\text{TM}}$ proves NOTHING about $B$!

### Classic Undecidability Proofs (MEMORIZE THESE 4!)

#### 1. $HALT_{\text{TM}} = \{\langle M, w \rangle \mid M \text{ halts on } w\}$ (Theorem 5.1)
Assume $R$ decides $HALT_{\text{TM}}$. Build decider $S$ for $A_{\text{TM}}$ on input $\langle M, w \rangle$:
1. Run $R$ on $\langle M, w \rangle$. If $R$ rejects (meaning $M$ loops on $w$), **reject**.
2. If $R$ accepts (meaning $M$ halts on $w$), **simulate $M$ on $w$** until it halts; accept if $M$ accepts, reject if $M$ rejects. $\blacksquare$

#### 2. $E_{\text{TM}} = \{\langle M \rangle \mid L(M) = \emptyset\}$ (Theorem 5.2)
Assume $R$ decides $E_{\text{TM}}$. Build decider $S$ for $A_{\text{TM}}$ on input $\langle M, w \rangle$:
1. Construct the description of a helper TM $M_1$ (with $M$ and $w$ hardcoded — **do NOT run $M$ yet!**):
   - $M_1 = \text{"On input } x\text{:}$
     1. If $x \neq w$, **reject**.
     2. If $x = w$, run $M$ on $w$ and **accept** if $M$ accepts."
   - Notice: $L(M_1) = \{w\}$ if $M$ accepts $w$, and $L(M_1) = \emptyset$ otherwise!
2. Run $R$ on $\langle M_1 \rangle$. If $R$ accepts ($L(M_1) = \emptyset$), **reject**; if $R$ rejects ($L(M_1) \neq \emptyset$), **accept**. $\blacksquare$

#### 3. $REGULAR_{\text{TM}} = \{\langle M \rangle \mid L(M) \text{ is regular}\}$ (Theorem 5.3)
Assume $R$ decides $REGULAR_{\text{TM}}$. Build decider $S$ for $A_{\text{TM}}$ on input $\langle M, w \rangle$:
1. Construct helper TM $M_2$:
   - $M_2 = \text{"On input } x\text{:}$
     1. If $x$ has the form $0^n 1^n$, **accept**.
     2. Otherwise, run $M$ on $w$ and **accept** if $M$ accepts."
   - Notice: If $M$ accepts $w$, $L(M_2) = \Sigma^*$ (**regular**!). If $M$ does not accept $w$, $L(M_2) = \{0^n 1^n\}$ (**nonregular**!).
2. Run $R$ on $\langle M_2 \rangle$ and output its answer! $\blacksquare$

#### 4. $EQ_{\text{TM}} = \{\langle M_1, M_2 \rangle \mid L(M_1) = L(M_2)\}$ (Theorem 5.4)
Reduce from $E_{\text{TM}}$: let $M_\emptyset$ be a fixed TM that rejects every input ($L(M_\emptyset) = \emptyset$). Then $\langle M \rangle \in E_{\text{TM}} \iff \langle M, M_\emptyset \rangle \in EQ_{\text{TM}}$. $\blacksquare$

---

## Linear Bounded Automata & Computation History Reductions

### Linear Bounded Automaton (LBA)
An **LBA** is a Turing Machine whose tape head is **never allowed to leave the $n$-cell region containing the input** (bounded by left/right endmarkers).
- **Theorem 5.9 ($A_{\text{LBA}}$ is DECIDABLE!):** An LBA with $q$ states, tape alphabet size $g$, and input length $n$ has at most $q \cdot n \cdot g^n$ distinct configurations. By the Pigeonhole Principle, if it runs for more than $q n g^n$ steps without halting, it **must** repeat a configuration and loop forever! Simulate for $q n g^n$ steps (or track visited configurations): if it repeats or times out, reject!  
  *Code:* `lba_accepts` in [ch05.rs](../src/ch05.rs) and [ch05.py](../python/ch05.py).
- **Theorem 5.10 ($E_{\text{LBA}}$ is UNDECIDABLE!):** Proved using **accepting computation histories**!
  - An **accepting computation history** for $M$ on $w$ is a string $\# C_1 \# C_2 \# \dots \# C_\ell \#$ where $C_1 = q_0 w$, $C_\ell$ is an accepting configuration, and each $C_{i+1}$ legally follows from $C_i$ in one step of $M$.
  - Construct an LBA $B$ that accepts input $x$ iff $x$ is a valid accepting computation history for $M$ on $w$ (easy for an LBA to check locally!). Then $L(B) = \emptyset \iff M$ does NOT accept $w$. Thus $E_{\text{LBA}}$ decides $A_{\text{TM}}$ — contradiction!
- **Theorem 5.13 ($ALL_{\text{CFG}} = \{\langle G \rangle \mid L(G) = \Sigma^*\}$ is UNDECIDABLE!):**  
  Construct a CFG $G$ generating **all strings that are NOT valid accepting computation histories** for $M$ on $w$ (reverse every other configuration $C_2^{\mathcal{R}}, C_4^{\mathcal{R}}, \dots$ so a PDA/CFG can spot any mismatch between $C_i$ and $C_{i+1}^{\mathcal{R}}$ using its stack!). Then $L(G) = \Sigma^* \iff M$ does NOT accept $w$.
  - **Immediate Corollary:** $EQ_{\text{CFG}}$ is also **UNDECIDABLE** (compare against a fixed CFG for $\Sigma^*$).

---

## 5.2 A Simple Undecidable Problem: Post Correspondence Problem (PCP)

- **Problem:** Given a finite collection of dominoes $P = \left\{ \left[\frac{t_1}{b_1}\right], \dots, \left[\frac{t_k}{b_k}\right] \right\}$, does there exist a non-empty sequence of indices $i_1, \dots, i_m$ (repeats allowed) such that $t_{i_1} \dots t_{i_m} = b_{i_1} \dots b_{i_m}$?
- **Theorem 5.15:** $PCP$ is **UNDECIDABLE** (proved by reducing $A_{\text{TM}}$ via computation-history dominoes where the bottom string stays one configuration ahead of the top string, until $q_{\text{accept}}$ lets the top catch up!).
- *Code:* `bounded_pcp` in [ch05.rs](../src/ch05.rs) and [ch05.py](../python/ch05.py) solves the **bounded** version (length $\le k$) to let you experiment with domino matches; remember that unbounded PCP has no algorithm!

---

## 5.3 Mapping Reducibility ($\le_m$)

### Formal Definition (Sipser Def 5.20)
Language $A$ is **mapping reducible** to language $B$ (written $A \le_m B$) if there exists a **computable function** $f : \Sigma^* \to \Sigma^*$ (computed by a TM that halts on every input) such that for every $w \in \Sigma^*$:
$$w \in A \iff f(w) \in B$$

### Four Golden Theorems of Mapping Reducibility (EXAM GOLD!)
1. **Theorem 5.22:** If $A \le_m B$ and $B$ is **decidable**, then $A$ is **decidable**.  
   *(Contrapositive — Cor 5.23):* If $A \le_m B$ and $A$ is **undecidable**, then $B$ is **undecidable**.
2. **Theorem 5.28:** If $A \le_m B$ and $B$ is **Turing-recognizable**, then $A$ is **Turing-recognizable**.  
   *(Contrapositive — Cor 5.29):* If $A \le_m B$ and $A$ is **NOT Turing-recognizable**, then $B$ is **NOT Turing-recognizable**!
3. **Complement preservation:** $A \le_m B \iff \overline{A} \le_m \overline{B}$ (using the **exact same function $f$**!).
4. **Why $\le_m$ is stronger than ordinary Turing reduction:** In $E_{\text{TM}}$ above, we **flipped** the output of $R$, so we actually showed $\overline{A_{\text{TM}}} \le_m E_{\text{TM}}$! Since $\overline{A_{\text{TM}}}$ is NOT recognizable, this immediately proves **$E_{\text{TM}}$ is NOT Turing-recognizable**!
   - Even better: **Theorem 5.30:** Neither $EQ_{\text{TM}}$ nor $\overline{EQ_{\text{TM}}}$ is Turing-recognizable (we can show both $A_{\text{TM}} \le_m \overline{EQ_{\text{TM}}}$ and $A_{\text{TM}} \le_m EQ_{\text{TM}}$ — wait: $\overline{A_{\text{TM}}} \le_m EQ_{\text{TM}}$ and $\overline{A_{\text{TM}}} \le_m \overline{EQ_{\text{TM}}}$!).

*Code:* `three_color_to_cnf` and `three_sat_to_clique` in [ch05.rs](../src/ch05.rs) and [ch05.py](../python/ch05.py) give concrete computable mapping reduction functions $f$.

---

## Rice's Theorem (2nd Edition Solved Problems — DO NOT SKIP!)

### Statement (Sipser Problem 5.28)
Let $P$ be any language consisting of TM descriptions $\langle M \rangle$ that satisfies two conditions:
1. **Semantic (Language-Invariant):** Whenever $L(M_1) = L(M_2)$, we have $\langle M_1 \rangle \in P \iff \langle M_2 \rangle \in P$. (That is, $P$ depends *only* on the language $L(M)$, never on the syntax, state count, or runtime of $M$!)
2. **Nontrivial:** There exists at least one TM $M_{\text{in}}$ with $\langle M_{\text{in}} \rangle \in P$ AND at least one TM $M_{\text{out}}$ with $\langle M_{\text{out}} \rangle \notin P$.

**Conclusion:** $P$ is **UNDECIDABLE**!

### Proof Blueprint (5 Lines!)
Assume $R$ decides $P$. Without loss of generality assume $\langle M_\emptyset \rangle \notin P$ (otherwise work with $\overline{P}$, which is also decidable, semantic, and nontrivial). Pick any fixed TM $T$ with $\langle T \rangle \in P$. Given $\langle M, w \rangle$, build $M'$:
- $M'(x)$: run $M$ on $w$; if $M$ accepts, run $T$ on $x$ and accept if $T$ accepts.
- Then $L(M') = L(T)$ (so $\langle M' \rangle \in P$) if $M$ accepts $w$, and $L(M') = \emptyset = L(M_\emptyset)$ (so $\langle M' \rangle \notin P$) otherwise! Run $R(\langle M' \rangle)$ to decide $A_{\text{TM}}$. Contradiction! $\blacksquare$

### When Rice's Theorem APPLIES vs. FAILS (Classic Exam Trap!)
- **Applies (Undecidable!):** "$L(M)$ is empty", "$L(M)$ is finite", "$L(M)$ is regular/CFL", "$\varepsilon \in L(M)$", "$|L(M)| = 42$".
- **FAILS (Does NOT apply!):**
  - "Does $M$ have at least 5 states?" — **NOT semantic** (syntactic property; actually **decidable**!).
  - "Does $M$ halt on $w$ in $\le 100$ steps?" — **NOT semantic** (depends on step count; **decidable**!).
  - "Does $M$ accept a Turing-recognizable language?" — **Trivial** (true of ALL TMs; **decidable**!).
  - *Warning:* Rice's Theorem does NOT say a syntactic property is decidable (e.g., "Does $M$ ever enter state $q_5$ on input $\varepsilon$?" is non-semantic yet still undecidable via reduction!).

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Prove $L = \{\langle M \rangle \mid M \text{ accepts at least two distinct strings}\}$ is undecidable and determine whether it is Turing-recognizable.  
   *Outline:* Undecidable by Rice's theorem (semantic; trivial empty TM is out, $\Sigma^*$ TM is in). **IS** Turing-recognizable: dovetail $M$ on all $s \in \Sigma^*$ and accept once 2 distinct strings are accepted.
2. **Q:** Prove $\overline{E_{\text{TM}}}$ IS Turing-recognizable, while $E_{\text{TM}}$ is NOT Turing-recognizable.  
   *Outline:* $\overline{E_{\text{TM}}}$ is recognized by dovetailing $M$ on all $s \in \Sigma^*$ and accepting if any $s$ is accepted. If $E_{\text{TM}}$ were recognizable, Theorem 4.22 would make $E_{\text{TM}}$ decidable — contradiction!
3. **Q:** Is $L = \{\langle M \rangle \mid M \text{ moves its head left at least once on input } \varepsilon\}$ decidable?  
   *Outline:* **YES!** If $M$ never moves left on $\varepsilon$, after $|Q| + 1$ steps moving right over blanks, some state repeats on blank $\sqcup$, so $M$ loops moving right forever and will never move left. Just simulate $M(\varepsilon)$ for $|Q| + 1$ steps!
