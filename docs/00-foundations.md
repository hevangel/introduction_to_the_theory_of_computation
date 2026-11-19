# Chapter 0 — Introduction & Mathematical Foundations

**All three editions (1e, 2e, 3e).**  
**Code:** [src/ch00.rs](../src/ch00.rs) (Rust) · [python/ch00.py](../python/ch00.py) (Python)

> **Why this chapter matters for your final exam:** Every theorem in this course is a mathematical statement about sets, strings, functions, graphs, and formal languages. Most lost exam points happen not because a student missed the automata idea, but because of sloppy set notation, an invalid induction base case, or a broken proof by contradiction.

---

## 0.1 Automata, Computability, and Complexity — The Big Picture

Theory of Computation asks one central question: **What are the fundamental capabilities and limitations of computers?** Sipser divides the course into three parts:

| Part | Chapters | Central Question | Typical Result |
|---|---|---|---|
| **Automata & Languages** | 1–2 | What can simple restricted machines (finite memory, one stack) compute? | Regular languages (DFA/NFA/Regex) $\subsetneq$ Context-free languages (CFG/PDA). |
| **Computability Theory** | 3–6 | What can *any* computer solve at all, given unlimited time and memory? | The Halting Problem ($HALT_{\text{TM}}$) and $A_{\text{TM}}$ are **undecidable**. |
| **Complexity Theory** | 7–10 | Among decidable problems, which ones are *practically* solvable (polynomial time/space)? | $\text{L} \subseteq \text{NL} = \text{coNL} \subseteq \text{P} \subseteq \text{NP} \subseteq \text{PSPACE} \subseteq \text{EXPTIME}$. |

---

## 0.2 Mathematical Notions and Terminology

### Sets, Subsets, and Power Sets
- A **set** is an unordered collection of distinct objects (no duplicates). Write $a \in A$ ("$a$ is an element of $A$"), $\emptyset$ for the empty set, and $|A|$ for cardinality.
- **Subset:** $A \subseteq B$ means every element of $A$ is in $B$. **Proper subset:** $A \subsetneq B$ means $A \subseteq B$ and $A \neq B$.
- **Set operations:**
  - Union: $A \cup B = \{x \mid x \in A \text{ or } x \in B\}$
  - Intersection: $A \cap B = \{x \mid x \in A \text{ and } x \in B\}$
  - Difference: $A \setminus B = \{x \mid x \in A \text{ and } x \notin B\}$
  - Complement (relative to universe $U$): $\overline{A} = U \setminus A$
- **Power set:** $\mathcal{P}(A) = \{S \mid S \subseteq A\}$. If $|A| = n$, then $|\mathcal{P}(A)| = 2^n$.  
  *Exam connection:* In Chapter 1, the subset construction converts an $n$-state NFA into a DFA with up to $2^n$ states — literally $\mathcal{P}(Q)$!  
  *Code:* `power_set` in [ch00.rs](../src/ch00.rs) and [ch00.py](../python/ch00.py).

### Sequences, Tuples, and Cartesian Products
- A **sequence** is an ordered list (duplicates allowed), written $(a_1, a_2, \dots, a_k)$ (a **$k$-tuple**).
- **Cartesian product:** $A \times B = \{(a, b) \mid a \in A, b \in B\}$, with $|A \times B| = |A| \cdot |B|$.
- $A^k = \underbrace{A \times A \times \dots \times A}_{k\text{ times}}$.

### Functions and Relations
- A **function** $f : D \to R$ maps each element of domain $D$ to **exactly one** element of range (codomain) $R$.
  - **Injective (one-to-one):** $f(x) = f(y) \implies x = y$.
  - **Surjective (onto):** $\forall y \in R, \exists x \in D$ such that $f(x) = y$.
  - **Bijective (correspondence):** Both injective and surjective. Two sets have the same cardinality iff there is a bijection between them.
- A **binary relation** $R$ on $A$ is a subset $R \subseteq A \times A$.
- An **equivalence relation** $\sim$ on $A$ satisfies three axioms for all $x, y, z \in A$:
  1. **Reflexive:** $x \sim x$
  2. **Symmetric:** $x \sim y \implies y \sim x$
  3. **Transitive:** $(x \sim y \land y \sim z) \implies x \sim z$
  - *Key theorem:* Every equivalence relation partitions $A$ into disjoint **equivalence classes** $[x] = \{y \in A \mid y \sim x\}$.  
  - *Exam connection:* The **Myhill–Nerode Theorem** (Chapter 1) is literally an equivalence relation on $\Sigma^*$!  
  - *Code:* `is_equivalence_relation` and `equivalence_classes` in [ch00.rs](../src/ch00.rs) and [ch00.py](../python/ch00.py).

### Graphs and Trees
- **Undirected graph** $G = (V, E)$: vertices $V$ and unordered edges $E \subseteq \{\{u, v\} \mid u, v \in V\}$.
- **Directed graph (digraph)** $G = (V, E)$: ordered pairs $(u, v) \in V \times V$.
- **Degree** of $v$: number of incident edges. **Handshaking Lemma:** $\sum_{v \in V} \deg(v) = 2|E|$ (hence every graph has an even number of odd-degree vertices).
- **Path** from $u$ to $v$: sequence of vertices connected by edges. A **simple path** repeats no vertices. A **cycle** is a path starting and ending at the same vertex.
- **Tree:** a connected, acyclic undirected graph ($|E| = |V| - 1$).  
  *Code:* `reachable` (BFS) and `has_cycle` (3-color DFS) in [ch00.rs](../src/ch00.rs) and [ch00.py](../python/ch00.py).

### Strings and Formal Languages (Crucial Notation!)
- **Alphabet** $\Sigma$: a **finite, nonempty** set of symbols (e.g., $\Sigma = \{0, 1\}$).
- **String (word)** $w$ over $\Sigma$: a finite sequence of symbols from $\Sigma$.
  - Length $|w|$: number of symbols in $w$.
  - **Empty string** $\varepsilon$: the unique string of length $0$. (**Never** confuse $\varepsilon$ with $\emptyset$! $\{\varepsilon\}$ has size 1; $\emptyset$ has size 0.)
  - **Reverse** $w^{\mathcal{R}}$: symbols of $w$ in reverse order.
  - **Concatenation** $xy$: string $x$ followed by $y$, with $x\varepsilon = \varepsilon x = x$.
  - **Power** $w^k$: $w$ concatenated $k$ times ($w^0 = \varepsilon$).
  - **Kleene star** $\Sigma^*$: the set of *all* finite strings over $\Sigma$ (including $\varepsilon$). Note $\Sigma^*$ is **countably infinite**!
  - **Shortlex (string) order:** order strings by length first, then lexicographically: $\varepsilon, 0, 1, 00, 01, 10, 11, \dots$
- **Language** $L$: **any subset** $L \subseteq \Sigma^*$.
  - A language can be finite, countably infinite, empty ($\emptyset$), or $\{\varepsilon\}$.
  - The set of *all* languages over $\Sigma$ is $\mathcal{P}(\Sigma^*)$, which is **uncountable**! (This is why uncomputable languages must exist in Chapter 4.)

### Boolean Logic
- Operations: $\neg$ (NOT), $\land$ (AND), $\lor$ (OR), $\oplus$ (XOR), $\to$ (implication: $P \to Q \equiv \neg P \lor Q$), $\leftrightarrow$ (iff).
- **De Morgan's Laws:** $\neg(P \land Q) \equiv \neg P \lor \neg Q$ and $\neg(P \lor Q) \equiv \neg P \land \neg Q$.
- **Distributive Laws:** $P \land (Q \lor R) \equiv (P \land Q) \lor (P \land R)$ and $P \lor (Q \land R) \equiv (P \lor Q) \land (P \lor R)$.
- **Contrapositive:** $(P \to Q) \equiv (\neg Q \to \neg P)$. (**Never** confuse with the converse $Q \to P$!)

---

## 0.3–0.4 Definitions, Theorems, and Proof Techniques

Every proof on a Theory of Computation final exam uses one of five templates:

### 1. Proof by Construction
To prove "there exists an object with property $P$" (e.g., "the class of regular languages is closed under union"), explicitly **construct** the object (e.g., the product DFA) and **prove** it satisfies property $P$.

### 2. Proof by Contradiction
Assume the statement is **false** ($\neg P$), derive a logical impossibility (e.g., $1 = 0$, or a machine both accepts and rejects the same input), and conclude $P$ must be true.
- *Classic example:* $\sqrt{2}$ is irrational. Suppose $\sqrt{2} = m/n$ in lowest terms ($\gcd(m,n)=1$). Then $2n^2 = m^2 \implies m$ is even ($m=2k$) $\implies 2n^2 = 4k^2 \implies n^2 = 2k^2 \implies n$ is even. Contradiction!
- *Where it appears on finals:* Pumping Lemma proofs (Ch 1 & 2), undecidability via diagonalization (Ch 4), and reductions (Ch 5).

### 3. Proof by Mathematical Induction
To prove $P(n)$ holds for all integers $n \ge n_0$:
1. **Base case:** Prove $P(n_0)$ directly.
2. **Induction hypothesis (IH):** Assume $P(k)$ holds for an arbitrary $k \ge n_0$ (or for **strong induction**, assume $P(j)$ holds for all $n_0 \le j \le k$).
3. **Inductive step:** Prove $P(k + 1)$ using the IH.
- *Where it appears on finals:* Correctness of DFA state invariants, NFA $\equiv$ DFA, CFG derivation length, and structural induction on regexes/parse trees.

### 4. Pigeonhole Principle
If $n + 1$ pigeons are placed into $n$ holes, at least one hole contains $\ge 2$ pigeons.
- *Where it appears on finals:* The **entire proof** of the Regular Pumping Lemma (a DFA with $p$ states processing a string of length $\ge p$ visits $p+1$ states, so some state repeats!).

### 5. Diagonalization (Preview of Ch 4)
Given a list $s_0, s_1, \dots, s_{n-1}$ of $n$-bit strings, construct $d$ by flipping the $i$-th bit of $s_i$: $d[i] = 1 - s_i[i]$. Then $d$ differs from every $s_i$ at position $i$, so $d$ is guaranteed not to be in the list.
- *Code:* `cantor_diagonal_bitstring` in [ch00.rs](../src/ch00.rs) and [ch00.py](../python/ch00.py).

---

## Common Final Exam Pitfalls

1. **Confusing $\emptyset$ and $\{\varepsilon\}$:** $\emptyset \cdot L = \emptyset$, whereas $\{\varepsilon\} \cdot L = L$. Also $\emptyset^* = \{\varepsilon\}$ (zero copies of anything gives $\varepsilon$!).
2. **Proving "only one direction" of an iff ($\iff$):** Whenever a theorem says "$A$ iff $B$" (e.g., "$L(M) = L$"), you must prove **both** $w \in L \implies M \text{ accepts } w$ AND $M \text{ accepts } w \implies w \in L$.
3. **Forgetting the induction base case** or applying the induction hypothesis to $k+1$ instead of $k$.
4. **Assuming a finite set of test cases is a proof:** Code tests check finite instances; proofs quantify over *all* strings in $\Sigma^*$.

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Prove that for any finite alphabet $\Sigma$, $\Sigma^*$ is countably infinite, whereas the set of all languages over $\Sigma$ is uncountable.  
   *Outline:* List $\Sigma^*$ in shortlex order (bijection with $\mathbb{N}$). For languages, represent each language $L \subseteq \Sigma^*$ by its infinite characteristic bitstring $\chi_L$ indexed by shortlex order; apply Cantor's diagonalization to any purported enumeration.
2. **Q:** Let $R$ be the relation on $\mathbb{Z}$ defined by $a \sim b \iff a \equiv b \pmod 3$. Prove $R$ is an equivalence relation and list its equivalence classes.  
   *Outline:* Reflexive ($3 \mid 0$), symmetric ($3 \mid (a-b) \implies 3 \mid (b-a)$), transitive ($3 \mid (a-b)$ and $3 \mid (b-c) \implies 3 \mid (a-c)$). Classes: $[0], [1], [2]$.
3. **Q:** Prove by induction that a full binary tree with $k$ leaves has $2k - 1$ total nodes.  
   *Outline:* Base $k=1$: single root leaf ($2(1)-1=1$). Step: removing two sibling leaves turns their parent into a leaf, reducing leaves by 1 and nodes by 2; by IH the smaller tree has $2(k-1)-1$ nodes, so the original has $2(k-1)-1+2 = 2k-1$.
