# Chapter 1 — Regular Languages

**All three editions (1e, 2e, 3e); 2nd edition adds important solved-problem coverage of the Myhill–Nerode Theorem.**  
**Code:** [src/ch01.rs](../src/ch01.rs) (Rust) · [python/ch01.py](../python/ch01.py) (Python)

> **Big Picture:** Regular languages are the languages recognizable by machines with a **fixed, finite amount of memory** (independent of input length $n$). Remarkably, four completely different formalisms — **DFAs**, **NFAs**, **Regular Expressions**, and **finite Myhill–Nerode index** — all define the exact same class of languages!

---

## 1.1 Finite Automata (DFAs)

### Formal Definition (Sipser Def 1.5)
A **Deterministic Finite Automaton (DFA)** is a 5-tuple $M = (Q, \Sigma, \delta, q_0, F)$ where:
1. $Q$ is a finite set of **states**.
2. $\Sigma$ is a finite **alphabet**.
3. $\delta : Q \times \Sigma \to Q$ is the **transition function** (must be **total**: exactly one outgoing arrow for every state and every symbol in $\Sigma$!).
4. $q_0 \in Q$ is the **start state**.
5. $F \subseteq Q$ is the set of **accept states**.

### Acceptance & Language
On input $w = w_1 w_2 \dots w_n \in \Sigma^*$, $M$ **accepts** $w$ if there exists a sequence of states $r_0, r_1, \dots, r_n \in Q$ such that:
- $r_0 = q_0$,
- $r_{i+1} = \delta(r_i, w_{i+1})$ for $0 \le i < n$,
- $r_n \in F$.

The **language recognized by $M$** is $L(M) = \{w \in \Sigma^* \mid M \text{ accepts } w\}$. A language $A$ is **regular** iff some DFA recognizes it.

### How to Design a DFA on an Exam
Think of each state as **what you need to remember about the prefix read so far**:
- *Example (`contains_01` in code):* Recognize $\{w \in \{0,1\}^* \mid w \text{ contains } 01\}$.
  - $q_0$: haven't seen `01`, and the last char (if any) was NOT `0`.
  - $q_1$: haven't seen `01`, and the last char WAS `0`.
  - $q_2$ (accept): have already seen `01` (absorbing/trap-accept state).
- **Proof technique:** State the **invariant** for each state and prove it by induction on $|w|$.

### Regular Operations & Product Construction
For languages $A, B \subseteq \Sigma^*$:
- **Union:** $A \cup B = \{x \mid x \in A \text{ or } x \in B\}$
- **Concatenation:** $A \circ B = \{xy \mid x \in A, y \in B\}$
- **Star (Kleene star):** $A^* = \{x_1 x_2 \dots x_k \mid k \ge 0, \text{ each } x_i \in A\}$ (always contains $\varepsilon$!).

**Product Construction (Closure under $\cup$, $\cap$, $\setminus$):**  
Given DFAs $M_1 = (Q_1, \Sigma, \delta_1, q_1, F_1)$ and $M_2 = (Q_2, \Sigma, \delta_2, q_2, F_2)$, build $M = (Q_1 \times Q_2, \Sigma, \delta, (q_1, q_2), F)$ where:
$$\delta((r_1, r_2), a) = (\delta_1(r_1, a), \delta_2(r_2, a))$$
- For **Union** $L(M_1) \cup L(M_2)$: $F = (F_1 \times Q_2) \cup (Q_1 \times F_2)$.
- For **Intersection** $L(M_1) \cap L(M_2)$: $F = F_1 \times F_2$.
- For **Complement** $\overline{L(M_1)}$: keep $M_1$ identical, just flip $F_1 \mapsto Q_1 \setminus F_1$! (**Warning:** This ONLY works on a complete DFA, NEVER directly on an NFA!)

*Code:* `Dfa::accepts`, `Dfa::complement`, `Dfa::product` in [ch01.rs](../src/ch01.rs) and [ch01.py](../python/ch01.py).

---

## 1.2 Nondeterminism (NFAs) and Subset Construction

### Formal Definition (Sipser Def 1.37)
A **Nondeterministic Finite Automaton (NFA)** is a 5-tuple $N = (Q, \Sigma, \delta, q_0, F)$ where:
$$\delta : Q \times \Sigma_\varepsilon \to \mathcal{P}(Q), \qquad \Sigma_\varepsilon = \Sigma \cup \{\varepsilon\}$$
- **Existential semantics:** $N$ accepts $w$ iff **at least one** computation branch ends in an accept state after consuming all of $w$.
- A branch may split (multiple choices), die (empty set $\emptyset$), or move without reading input ($\varepsilon$-transition).

### Theorem 1.39 (Subset Construction): Every NFA has an Equivalent DFA
**Proof Blueprint (MEMORIZE THIS FOR THE EXAM!):**  
Given NFA $N = (Q, \Sigma, \delta, q_0, F)$, define the **$\varepsilon$-closure** $E(R)$ of a set of states $R \subseteq Q$ as all states reachable from $R$ using $0$ or more $\varepsilon$-transitions. Construct DFA $M = (Q', \Sigma, \delta', q'_0, F')$:
1. $Q' = \mathcal{P}(Q)$ (each DFA state is a *subset* of NFA states — up to $2^{|Q|}$ states).
2. $q'_0 = E(\{q_0\})$.
3. $\delta'(R, a) = \bigcup_{r \in R} E(\delta(r, a))$.
4. $F' = \{R \in Q' \mid R \cap F \neq \emptyset\}$ (accept if the subset contains *at least one* NFA accept state).

**Corollary (Theorem 1.40):** A language is regular iff some NFA recognizes it.

### Closure Under Union, Concatenation, and Star via NFAs
NFAs make closure proofs effortless:
- **Union ($N_1 \cup N_2$):** New start state with $\varepsilon$-arrows to $q_{0,1}$ and $q_{0,2}$.
- **Concatenation ($N_1 \circ N_2$):** $\varepsilon$-arrows from every accept state of $N_1$ to $q_{0,2}$; new accept states are $F_2$.
- **Star ($N_1^*$):** New start state (which is ALSO accepting, to accept $\varepsilon$!) with $\varepsilon$-arrow to $q_{0,1}$, plus $\varepsilon$-arrows from every accept state of $N_1$ back to $q_{0,1}$. (**Exam trap:** Don't just make $q_{0,1}$ accepting without adding a new start state — if $q_{0,1}$ has incoming arrows, that could accept unwanted strings!)

*Code:* `Nfa::epsilon_closure`, `Nfa::accepts`, `Nfa::to_dfa` in [ch01.rs](../src/ch01.rs) and [ch01.py](../python/ch01.py).

---

## 1.3 Regular Expressions & Equivalence with Finite Automata

### Inductive Definition (Sipser Def 1.52)
$R$ is a **regular expression** over $\Sigma$ if $R$ is:
- Base cases: $a$ (for $a \in \Sigma$), $\varepsilon$ (the language $\{\varepsilon\}$), or $\emptyset$ (the empty language).
- Inductive cases: $(R_1 \cup R_2)$, $(R_1 \circ R_2)$, or $(R_1^*)$.
- Precedence (highest to lowest): Star `*` > Concatenation > Union `|` ($\cup$).

### Theorem 1.54 (Kleene's Theorem): A language is regular $\iff$ some regular expression describes it.
- **($\Leftarrow$) Regex $\to$ NFA (Thompson's Construction):** Structural induction using the base NFAs for $a, \varepsilon, \emptyset$ and the NFA constructions for $\cup, \circ, *$.  
  *Code:* `regex_to_nfa` and `regex_matches` in [ch01.rs](../src/ch01.rs) and [ch01.py](../python/ch01.py).
- **($\Rightarrow$) DFA $\to$ Regex (GNFA State Elimination):**
  1. Convert DFA to a **Generalized NFA (GNFA)** whose arrows are labeled by regular expressions: add a new start state $q_{\text{start}}$ (no incoming arrows) with $\varepsilon$ to $q_0$, and a single new accept state $q_{\text{accept}}$ (no outgoing arrows) with $\varepsilon$ from every old accept state.
  2. Repeatedly pick any internal state $q_{\text{rip}} \notin \{q_{\text{start}}, q_{\text{accept}}\}$ and eliminate it by updating every pair $(q_i, q_j)$:
     $$R_{\text{new}}(q_i, q_j) = R(q_i, q_j) \cup R(q_i, q_{\text{rip}}) \bigl(R(q_{\text{rip}}, q_{\text{rip}})\bigr)^* R(q_{\text{rip}}, q_j)$$
  3. When only $q_{\text{start}}$ and $q_{\text{accept}}$ remain, the single arrow label is the equivalent regex!

---

## 1.4 Nonregular Languages & The Pumping Lemma

### Theorem 1.70 (Pumping Lemma for Regular Languages)
If $A$ is a regular language, then there exists a number $p \ge 1$ (the **pumping length**) such that for every string $s \in A$ with $|s| \ge p$, $s$ can be split into three pieces $s = xyz$ satisfying:
1. $x y^i z \in A$ for every $i \ge 0$ (including $i = 0$, **pumping down**!),
2. $|y| > 0$,
3. $|xy| \le p$.

**Why it's true (1-line intuition):** Let $p = |Q|$ of a DFA for $A$. Processing a string of length $\ge p$ visits $p+1$ states in the first $p$ steps; by the Pigeonhole Principle, some state repeats. That loop is $y$, and you can repeat it $0, 1, 2, \dots$ times!

### How to Write a Bulletproof Pumping Lemma Proof (Adversary Game!)
To prove $L$ is **not regular**, play a 4-step game against an adversary:
1. **Assume for contradiction** that $L$ is regular, so the Pumping Lemma gives some pumping length $p \ge 1$ (**you do NOT pick $p$** — the adversary picks $p$).
2. **You pick** a specific string $s \in L$ (in terms of $p$) with $|s| \ge p$.
3. **Adversary splits** $s = xyz$ satisfying $|xy| \le p$ and $|y| > 0$. (**You must handle ALL valid splits**, not just one!)
4. **You pick** an integer $i \ge 0$ (usually $i = 0$ or $i = 2$) and prove $xy^iz \notin L$. Contradiction!

#### Worked Example 1: $B = \{0^n 1^n \mid n \ge 0\}$
- Choose $s = 0^p 1^p \in B$ ($|s| = 2p \ge p$).
- Since $|xy| \le p$, both $x$ and $y$ lie entirely inside the leading $0^p$, so $y = 0^k$ with $k \ge 1$.
- Pump with $i = 2$: $xy^2z = 0^{p+k} 1^p \notin B$ since $p + k \neq p$. Contradiction!

#### Worked Example 2: $F = \{ww \mid w \in \{0, 1\}^*\}$
- Choose $s = 0^p 1 0^p 1 \in F$. (**Trap:** $s = 0^p 0^p = 0^{2p}$ FAILS because $y=00$ can be pumped!)
- Since $|xy| \le p$, $y = 0^k$ ($k \ge 1$) lies in the first block of $0$s.
- Pump $i = 2$: $xy^2z = 0^{p+k} 1 0^p 1 \notin F$. Contradiction!

#### Worked Example 3 (Using Closure Properties!): $C = \{w \in \{0,1\}^* \mid \#_0(w) = \#_1(w)\}$
- Shortcut! If $C$ were regular, then $C \cap 0^*1^* = \{0^n 1^n \mid n \ge 0\}$ would be regular (regular languages are closed under intersection). Contradiction!

---

## Myhill–Nerode Theorem (2nd Edition Solved Problems — Essential!)

For any language $L \subseteq \Sigma^*$, define the **distinguishability relation** $\equiv_L$ on $\Sigma^*$:
$$x \equiv_L y \iff \forall z \in \Sigma^*,\; (xz \in L \iff yz \in L)$$
- If $\exists z$ with $xz \in L$ and $yz \notin L$ (or vice versa), $z$ is a **distinguishing extension** of $x$ and $y$.
- The **index** of $L$ is the number of equivalence classes of $\equiv_L$.

### Theorem (Myhill–Nerode)
1. $L$ is regular $\iff$ $\equiv_L$ has **finitely many** equivalence classes.
2. When $L$ is regular, the number of equivalence classes equals the number of states in the **unique minimal complete DFA** for $L$!

**Why students love Myhill–Nerode on exams:**
- Unlike the Pumping Lemma, Myhill–Nerode is an **if-and-only-if** characterization!
- To prove $L$ is nonregular (e.g., $\{0^n 1^n\}$): exhibit an infinite set of pairwise distinguishable prefixes $S = \{0^0, 0^1, 0^2, \dots\}$. For $i \neq j$, suffix $z = 1^i$ distinguishes $0^i$ from $0^j$ ($0^i 1^i \in L$ but $0^j 1^i \notin L$). Infinitely many classes $\implies$ nonregular!
- To prove a minimal DFA needs at least $k$ states: exhibit $k$ pairwise distinguishable strings.
- *Code:* `Dfa::myhill_nerode_classes` and `Dfa::minimize` (table-filling algorithm) in [ch01.rs](../src/ch01.rs) and [ch01.py](../python/ch01.py).

---

## Final Exam Cheat Sheet — Regular Languages

| Property | Closed? | How to Prove |
|---|---|---|
| Union ($A \cup B$) | **Yes** | Product DFA or NFA with new start state |
| Intersection ($A \cap B$) | **Yes** | Product DFA ($F_1 \times F_2$) or De Morgan ($\overline{\overline{A} \cup \overline{B}}$) |
| Complement ($\overline{A}$) | **Yes** | Flip accept states of a **complete DFA** |
| Concatenation ($A \circ B$) | **Yes** | NFA linking $F_1 \xrightarrow{\varepsilon} q_{0,2}$ |
| Kleene Star ($A^*$) | **Yes** | NFA with new accepting start state + loop back |
| Reversal ($A^{\mathcal{R}}$) | **Yes** | Reverse all arrows in DFA/NFA, swap start & accept |
| Homomorphism / Inverse Homomorphism | **Yes** | Replace edge labels / simulate $h(a)$ |

### Common Exam Pitfalls
1. **Complementing an NFA by flipping accept states** — **WRONG!** Convert to a complete DFA first!
2. **Using the Pumping Lemma to prove a language IS regular** — **IMPOSSIBLE!** The Pumping Lemma is necessary, NOT sufficient.
3. **Picking a specific split $xyz$** in a Pumping Lemma proof — **WRONG!** You must show *every* split with $|xy| \le p$ and $|y| > 0$ fails.
4. **Programming regexes $\neq$ formal regexes:** Backreferences like `(a*)\1` recognize nonregular languages!

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Prove $L = \{0^{n^2} \mid n \ge 0\}$ is not regular.  
   *Outline:* Pick $s = 0^{p^2}$. Then $y = 0^k$ with $1 \le k \le p$. For $i = 2$, $|xy^2z| = p^2 + k$. Since $p^2 < p^2 + k \le p^2 + p < (p+1)^2$, $p^2 + k$ is strictly between consecutive squares, so $xy^2z \notin L$.
2. **Q:** What is the minimum number of states in a complete DFA over $\{0, 1\}$ recognizing strings whose 3rd-to-last symbol is `1`?  
   *Outline:* $2^3 = 8$ states. The 8 length-3 strings $\{0,1\}^3$ are pairwise distinguishable under Myhill–Nerode (if two length-3 strings differ at bit $j \in \{0,1,2\}$, append $j$ arbitrary bits to put the differing bit into the 3rd-to-last position).
3. **Q:** Prove regular languages are closed under $HALF(L) = \{x \mid \exists y,\; |x| = |y| \land xy \in L\}$.  
   *Outline:* Given DFA $M$ for $L$, build an NFA that guesses the midpoint state $q_m$, tracks $M$ forward from $q_0$ to $q_m$ on $x$, and simultaneously tracks a *backward* (or existential forward) state from $q_m$ to some $f \in F$ taking one arbitrary symbol step per symbol of $x$.
