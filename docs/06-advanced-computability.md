# Chapter 6 — Advanced Topics in Computability Theory

**All three editions (1e, 2e, 3e).**  
**Code:** [src/ch06.rs](../src/ch06.rs) (Rust) · [python/ch06.py](../python/ch06.py) (Python)

> **Big Picture:** Chapter 6 explores four deep consequences of computability theory: (1) programs can inspect their own source code (**Recursion Theorem**), (2) formal arithmetic is inherently incomplete (**Gödel's Incompleteness Theorem**), (3) undecidable problems form an infinite hierarchy of hardness (**Turing Reducibility & Oracles**), and (4) randomness can be defined via shortest program length (**Kolmogorov Complexity**).

---

## 6.1 The Recursion Theorem (Self-Reference)

### Lemma 6.1 (`SELF`) & Theorem 6.3 (The Recursion Theorem)
- **Lemma 6.1 (Quine / `SELF`):** There exists a TM `SELF` that ignores its input, prints its own description $\langle \text{SELF} \rangle$, and halts!
  - **How it works (2-part trick):** Split `SELF` into two parts $A$ and $B$:
    - $A$ simply prints $\langle B \rangle$ onto the tape and passes control to $B$.
    - $B$ reads whatever description $\langle X \rangle$ is on the tape, computes $q(\langle X \rangle)$ (the description of a machine that prints $\langle X \rangle$), and outputs $\langle q(\langle X \rangle), X \rangle$.
    - Running $A$ then $B$ outputs $\langle A, B \rangle = \langle \text{SELF} \rangle$!
  - *Code:* `self_reproducing_string` in [ch06.rs](../src/ch06.rs) and [ch06.py](../python/ch06.py).
- **Theorem 6.3 (Recursion Theorem):** Let $T$ be any TM that computes a function $t : \Sigma^* \times \Sigma^* \to \Sigma^*$. Then there exists a TM $R$ computing $r(w) = t(\langle R \rangle, w)$.
  - **In plain English:** When designing a Turing Machine $R$, you are **legally allowed** to include the instruction *"Obtain my own description $\langle R \rangle$"* as a built-in step!

### Killer Exam Applications of the Recursion Theorem
1. **3-Line Proof that $A_{\text{TM}}$ is Undecidable (Theorem 6.5):**  
   Assume $H$ decides $A_{\text{TM}}$. Define TM $B$ on input $w$: (1) Obtain own description $\langle B \rangle$. (2) Run $H(\langle B, w \rangle)$. (3) Do the opposite! Contradiction!
2. **$MIN_{\text{TM}} = \{\langle M \rangle \mid M \text{ is a minimal-length TM for } L(M)\}$ is NOT Turing-Recognizable (Theorem 6.7):**  
   Suppose enumerator $E$ enumerates $MIN_{\text{TM}}$. Build TM $C$: (1) Obtain own description $\langle C \rangle$. (2) Run $E$ until it prints some TM $\langle D \rangle$ with $|\langle D \rangle| > |\langle C \rangle|$. (3) Simulate $D$ on input $w$. Then $L(C) = L(D)$, contradicting that $D$ was minimal!
3. **Fixed-Point Theorem (Theorem 6.8):** For every computable function $t : \Sigma^* \to \Sigma^*$, there exists a TM $F$ such that $L(F) = L(t(\langle F \rangle))$.

---

## 6.2 Decidability of Logical Theories

### Models, Sentences, and Theories
- A **model** $\mathcal{M} = (U, R_1, \dots, R_k)$ consists of a universe $U$ and relations/functions.
- The **theory** $\text{Th}(\mathcal{M})$ is the set of all **true first-order sentences** (formulas with no free variables, using $\forall, \exists, \land, \lor, \neg, =$) in model $\mathcal{M}$.

### The Dramatic Boundary Between `+` and `+, ×` (EXAM CLASSIC!)
| Theory | Structure | Decidable? | Why? |
|---|---|---|---|
| **$\text{Th}(\mathbb{N}, +)$** (Presburger Arithmetic, Thm 6.12) | Natural numbers with **addition only** | **DECIDABLE!** | Encode addition as a finite automaton (DFA) on base-2 column vectors; use DFA closure ($\cap, \overline{\cdot}$, projection/NFA) to eliminate $\exists$ and $\forall$ quantifiers! |
| **$\text{Th}(\mathbb{N}, +, \times)$** (Peano Arithmetic, Thm 6.13) | Natural numbers with **addition AND multiplication** | **UNDECIDABLE!** | With both $+$ and $\times$, you can encode TM computation histories as arithmetic formulas, reducing $A_{\text{TM}}$ to $\text{Th}(\mathbb{N}, +, \times)$! |

- *Code:* `eval_fo_mod` in [ch06.rs](../src/ch06.rs) and [ch06.py](../python/ch06.py) decides first-order sentences over finite additive structures $(\mathbb{Z}_m, +)$.

### Gödel's Incompleteness Theorem (Theorem 6.16)
A proof system is **sound** (proves only true statements) and **complete** (proves all true statements).
- **Theorem 6.16:** Every sound, computably verifiable proof system for $\text{Th}(\mathbb{N}, +, \times)$ is **incomplete** — there exist true sentences in $\text{Th}(\mathbb{N}, +, \times)$ that have **no proof**!
- *Proof idea (2 lines):* If a sound proof system were complete, we could decide $\text{Th}(\mathbb{N}, +, \times)$ by enumerating all valid proofs until we find a proof of either $\phi$ or $\neg \phi$. That contradicts Theorem 6.13!

---

## 6.3 Turing Reducibility ($\le_T$) and Oracles

- An **oracle** for language $B$ is a black-box device that answers membership queries "$x \in B$?" in a single step. Write $M^B$ for an oracle TM with oracle $B$.
- **$A$ is Turing reducible to $B$** ($A \le_T B$) if $A$ is **decidable relative to $B$** (some oracle decider $M^B$ decides $A$).
- **Key Difference from Mapping Reducibility ($\le_m$):**
  - $A \le_m B \implies A \le_T B$, but the converse is **FALSE**!
  - Example: $\overline{A_{\text{TM}}} \le_T A_{\text{TM}}$ (query oracle once and flip the answer!), whereas $\overline{A_{\text{TM}}} \not\le_m A_{\text{TM}}$ (since $A_{\text{TM}}$ is recognizable and $\overline{A_{\text{TM}}}$ is not).
- **The Arithmetic Hierarchy:** Even with an oracle for $A_{\text{TM}}$, the halting problem for oracle machines $A_{\text{TM}}^{A_{\text{TM}}}$ is STILL undecidable relative to $A_{\text{TM}}$! There is an infinite hierarchy of strictly harder undecidable problems.

---

## 6.4 A Definition of Information (Kolmogorov Complexity)

### Formal Definition (Sipser Def 6.22)
The **minimal description** $d(x)$ of a binary string $x$ is the shortest string $\langle M, w \rangle$ such that TM $M$ on input $w$ outputs $x$ and halts. The **descriptive (Kolmogorov) complexity** of $x$ is:
$$K(x) = |d(x)|$$

### Key Theorems (Exam Favorites!)
1. **Invariance / Upper Bound (Theorem 6.23):** $\exists c$ such that $\forall x,\; K(x) \le |x| + c$ (hardcode $x$ in a "print input" machine).
2. **Repeated patterns compress (Theorem 6.24):** $K(xx) \le K(x) + c$.
3. **Incompressible strings ALWAYS exist (Theorem 6.26):** A string $x$ is **$c$-compressible** if $K(x) \le |x| - c$, and **incompressible** if $K(x) \ge |x|$.
   - *Pigeonhole proof:* There are $2^n$ binary strings of length $n$, but only $1 + 2 + 4 + \dots + 2^{n-1} = 2^n - 1$ descriptions of length $< n$. Therefore, at least one string of every length $n$ is **incompressible**! (In fact, $> 1 - 2^{-c}$ of all strings cannot be compressed by $c$ bits.)
4. **$K(x)$ is UNCOMPUTABLE (Theorem 6.27):** If a TM could compute $K(x)$, we could write a short program that enumerates strings until it finds one with $K(x) > 1000000$ and prints it — giving a short description of a string with huge Kolmogorov complexity (Berry's Paradox)!
- *Code:* `binary_strings_up_to`, `cantor_pair`/`cantor_unpair`, and `run_length_encode` (an explicit computable *upper bound* on compressibility — NOT $K(x)$!) in [ch06.rs](../src/ch06.rs) and [ch06.py](../python/ch06.py).

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Why does $\overline{A_{\text{TM}}} \le_T A_{\text{TM}}$ hold, while $\overline{A_{\text{TM}}} \le_m A_{\text{TM}}$ fails?  
   *Outline:* An oracle TM can call $A_{\text{TM}}$ and negate its answer. A mapping reduction cannot negate the answer; if $\overline{A_{\text{TM}}} \le_m A_{\text{TM}}$, then recognizability of $A_{\text{TM}}$ would force $\overline{A_{\text{TM}}}$ to be recognizable (contradiction).
2. **Q:** Use the Recursion Theorem to show that $E_{\text{TM}}$ is undecidable.  
   *Outline:* Suppose $R$ decides $E_{\text{TM}}$. Build TM $B$: obtain $\langle B \rangle$; run $R(\langle B \rangle)$; if $R$ accepts ($L(B)=\emptyset$), accept $w$; if $R$ rejects, reject $w$. If $L(B)=\emptyset$ then $B$ accepts everything (contradiction); if $L(B)\neq\emptyset$ then $B$ rejects everything (contradiction).
3. **Q:** At least what fraction of all $n$-bit strings satisfy $K(x) \ge n - 10$?  
   *Outline:* At most $2^{n-10} - 1$ descriptions have length $< n - 10$, so at least $1 - 2^{-10} > 99.9\%$ of $n$-bit strings cannot be compressed by even 10 bits!
