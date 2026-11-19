# Chapter 10 — Advanced Topics in Complexity Theory

**All three editions (1e, 2e, 3e); 2nd edition explicitly repairs the probabilistic primality proof (crediting Victor Shoup).**  
**Code:** [src/ch10.rs](../src/ch10.rs) (Rust) · [python/ch10.py](../python/ch10.py) (Python)

> **Big Picture:** What do we do when a problem is NP-hard, or when we relax our computational model to allow **randomness**, **alternation**, **interaction with a prover**, **massive parallelism**, or **cryptographic hardness**? Chapter 10 surveys six pillars of modern complexity theory.

---

## 10.1 Approximation Algorithms

When an optimization problem is NP-hard (like Minimum Vertex Cover), we seek a **polynomial-time $k$-approximation algorithm** guaranteed to output a valid solution within a factor $k$ of optimal ($OPT$).

### 2-Approximation for Minimum Vertex Cover (Theorem 10.1)
- **Algorithm:** While an uncovered edge $(u, v)$ exists, add **both** $u$ and $v$ to the cover and delete all edges incident to $u$ or $v$.
- **Why $\le 2 \cdot OPT$? (Exam Proof — 3 Lines!):**  
  Let $M$ be the set of edges picked by the algorithm. No two edges in $M$ share an endpoint (it is a **matching**). Any optimal vertex cover MUST contain at least one endpoint from every edge in $M$, so $OPT \ge |M|$. Our algorithm outputs $2|M|$ vertices, hence $|C| = 2|M| \le 2 \cdot OPT$! $\blacksquare$
- *Code:* `two_approx_vertex_cover` in [ch10.rs](../src/ch10.rs) and [ch10.py](../python/ch10.py).

---

## 10.2 Probabilistic Algorithms (BPP, Primality, Read-Once Branching Programs)

### The Class BPP (Bounded-Error Probabilistic Polynomial Time)
A probabilistic TM flips unbiased coins at each step. Language $A \in \text{BPP}$ if there is a poly-time probabilistic TM $M$ such that:
- $w \in A \implies \Pr[M \text{ accepts } w] \ge 2/3$,
- $w \notin A \implies \Pr[M \text{ accepts } w] \le 1/3$.
- **Amplification Lemma (Lemma 10.5 — EXAM FAVORITE!):** Running $M$ independently $k$ times and taking the **majority vote** drives the error probability down exponentially to $\le 2^{-\Omega(k)}$ (via Chernoff bounds)! Thus $2/3$ can be replaced by $1/2 + n^{-c}$ or $1 - 2^{-n^c}$ without changing $\text{BPP}$.
- **Related classes:**
  - **RP** (one-sided error: no false positives; false negative $\le 1/2$).
  - **coRP** (one-sided error: no false negatives; false positive $\le 1/2$).
  - **ZPP** $= \text{RP} \cap \text{coRP}$ (zero error, expected poly time — Las Vegas algorithms).

### Probabilistic Primality Testing (2nd Edition Repair Note!)
- **Goal:** Decide $PRIMES = \{\langle n \rangle \mid n \text{ is prime}\}$ in poly($\log n$) time.
- **Why Fermat's Little Theorem alone fails:** **Carmichael numbers** (like $561 = 3 \times 11 \times 17$) are composite yet satisfy $a^{n-1} \equiv 1 \pmod n$ for every $a$ coprime to $n$!
- **Miller–Rabin Test (repaired in 2nd edition, crediting Victor Shoup):**  
  Write $n - 1 = d \cdot 2^s$ ($d$ odd). Pick random base $a \in \{2, \dots, n-2\}$ and compute $a^d, a^{2d}, \dots, a^{2^{s-1}d} \pmod n$ using fast modular exponentiation (`mod_pow`). If $a^d \not\equiv 1 \pmod n$ and none of the squares equals $-1 \pmod n$, then $n$ is **definitely composite** (a nontrivial square root of $1 \pmod n$ was found, or Fermat failed!).
  - **Soundness:** Every prime passes 100% of the time; every odd composite fails for at least $3/4$ of bases $a$ (so $k$ rounds give error $\le 4^{-k}$, putting $PRIMES \in \text{coRP} \subseteq \text{BPP}$!).
  - *(Historical note: In 2002, Agrawal–Kayal–Saxena (AKS) proved $PRIMES \in \text{P}$ deterministically!)*
  - **Edition warning:** Do NOT reproduce the flawed 1st-edition primality proof; always use the repaired strong-pseudoprime / Miller–Rabin analysis.  
  *Code:* `mod_pow`, `is_probably_prime` (catching Carmichael 561!) in [ch10.rs](../src/ch10.rs) and [ch10.py](../python/ch10.py).

### Read-Once Branching Programs (ROBPs) & Polynomial Identity Testing
- A **Read-Once Branching Program** is a DAG where each internal node queries a variable $x_i$ at most **once** on any path.
- **Problem $EQ_{\text{ROBP}}$:** Given two ROBPs $B_1, B_2$, do they compute the same Boolean function?
- **BPP Algorithm (Theorem 10.12):** Arithmetize each ROBP into a multilinear polynomial $p(x_1, \dots, x_m)$ over a large finite field $\mathbb{F}_q$ (with $q > 3m$), evaluate both polynomials at a **random point** $(r_1, \dots, r_m) \in \mathbb{F}_q^m$, and accept iff they agree! By the **Schwartz–Zippel Lemma**, two distinct degree-$m$ polynomials agree on at most $m/q < 1/3$ of points!
- *Code:* `eval_robp` in [ch10.rs](../src/ch10.rs) and [ch10.py](../python/ch10.py).

---

## 10.3 Alternation & The Polynomial Hierarchy (PH)

- An **Alternating Turing Machine (ATM)** has both **existential ($\exists$)** states (accept if *some* child accepts) and **universal ($\forall$)** states (accept if *all* children accept).
- **Key Equivalences (Theorems 10.18–10.19):**
  $$\text{AP} = \text{PSPACE}, \qquad \text{APSPACE} = \text{EXPTIME}, \qquad \text{AL} = \text{P}$$
  *(Alternating time = deterministic space! Alternating space = exponential deterministic time!)*
- **The Polynomial Hierarchy ($\text{PH}$):** Restrict an ATM to a **constant number $k$** of quantifier alternations:
  - $\Sigma_1^P = \text{NP}$ ($\exists$), $\Pi_1^P = \text{coNP}$ ($\forall$).
  - $\Sigma_2^P$ ($\exists \forall$), $\Pi_2^P$ ($\forall \exists$), e.g., $MIN\text{-}FORMULA \in \Sigma_2^P$.
  - $\text{PH} = \bigcup_k \Sigma_k^P \subseteq \text{PSPACE}$.
  - *Collapse property:* If $\text{P} = \text{NP}$, the entire Polynomial Hierarchy collapses to $\text{P}$!

---

## 10.4 Interactive Proof Systems ($\text{IP} = \text{PSPACE}$)

- Two parties: a **computationally unbounded Prover ($P$)** (can be dishonest!) and a **probabilistic polynomial-time Verifier ($V$)** exchanging messages.
- Language $A \in \text{IP}$ iff:
  - **Completeness:** $w \in A \implies \exists P$ such that $\Pr[V \text{ accepts } w] \ge 2/3$.
  - **Soundness:** $w \notin A \implies \forall P^*$, $\Pr[V \text{ accepts } w] \le 1/3$.
- **Classic Protocol — Graph Non-Isomorphism ($GNI \in \text{IP}$, Theorem 10.23):**
  1. Verifier secretly flips a coin $b \in \{0, 1\}$, picks a random permutation $\pi$, and sends $H = \pi(G_b)$ to Prover.
  2. Prover responds with a bit $b'$; Verifier accepts iff $b' = b$.
  - If $G_0 \not\cong G_1$, unbounded Prover can always tell which graph $H$ came from (accepts with prob 1).
  - If $G_0 \cong G_1$, $H$ is identically distributed whether $b=0$ or $b=1$, so even an all-powerful cheating Prover can guess $b$ with prob at most $1/2$!
  - *Code:* `verify_graph_isomorphism` in [ch10.rs](../src/ch10.rs) and [ch10.py](../python/ch10.py).
- **Shamir's Theorem (Theorem 10.24):** **$\text{IP} = \text{PSPACE}$!** (Proved by arithmetizing $TQBF$ into low-degree polynomials and running the **Sum-Check Protocol**!)

---

## 10.5 Parallel Computation (The Class NC)

- Model: **Uniform families of Boolean circuits** with **polynomial size** ($n^{O(1)}$ processors/gates) and **polylogarithmic depth** ($O(\log^k n)$ parallel time!).
- **$\text{NC} = \bigcup_{k \ge 1} \text{NC}^k$** ("Nick's Class" — problems efficiently parallelizable).
- **Examples in $\text{NC}$:** Parity, binary addition, prefix sums ($O(\log n)$ depth via tree doubling), matrix multiplication, $PATH$ ($\text{NL} \subseteq \text{NC}^2$).
- **Master Inclusion:** $\text{NC}^1 \subseteq \text{L} \subseteq \text{NL} \subseteq \text{NC}^2 \subseteq \text{NC} \subseteq \text{P}$.
- **P-complete problems** (e.g., Circuit Value Problem $CVP$) are the "inherently sequential" problems in P: if any P-complete problem is in $\text{NC}$, then $\text{NC} = \text{P}$.
- *Code:* `parallel_prefix_sum` ($O(\log n)$ rounds!) in [ch10.rs](../src/ch10.rs) and [ch10.py](../python/ch10.py).

---

## 10.6 Cryptography & One-Way Functions

- **One-way function:** Easy to compute ($f(x)$ in poly time), hard to invert on average (no poly-time algorithm finds $x'$ with $f(x') = f(x)$ except with negligible probability).
- **Trapdoor one-way function:** Easy to invert *only* if you know a secret trapdoor key!
  - **RSA example:** Public key $(n = pq, e)$, encrypt $c = m^e \bmod n$. Without factoring $n$, inverting is believed intractable; with trapdoor $d = e^{-1} \bmod (p-1)(q-1)$, decrypt $m = c^d \bmod n$.
  - *Code:* `rsa_roundtrip` in [ch10.rs](../src/ch10.rs) and [ch10.py](../python/ch10.py).
- **Important distinction:** Cryptography requires **average-case** hardness (random keys must be hard to crack), whereas NP-completeness only addresses **worst-case** hardness!

---

## Final Exam Big-Picture Complexity Map (Chapters 7–10)

$$\text{NC}^1 \subseteq \text{L} \subseteq \text{NL} = \text{coNL} \subseteq \text{NC}^2 \subseteq \text{P} \subseteq \begin{cases} \text{BPP} \\ \text{NP}, \text{coNP} \end{cases} \subseteq \text{PH} \subseteq \text{PSPACE} = \text{NPSPACE} = \text{AP} = \text{IP} \subseteq \text{EXPTIME}$$

---

## Practice Exam Questions (with Solution Outlines)

1. **Q:** Explain why the 2-approximation algorithm for Vertex Cover cannot be beaten to ratio $2$ by simply picking the endpoint of higher degree without further analysis — and prove why picking *both* endpoints of a maximal matching guarantees $\le 2 \cdot OPT$.  
   *Outline:* Every edge in the matching $M$ requires at least 1 vertex in any valid cover, so $OPT \ge |M|$. Picking both endpoints selects $2|M| \le 2 \cdot OPT$ vertices.
2. **Q:** Why does the Fermat primality test fail on $561$, and how does Miller–Rabin catch it?  
   *Outline:* $561 = 3 \times 11 \times 17$ is a Carmichael number, so $a^{560} \equiv 1 \pmod{561}$ for all $\gcd(a, 561)=1$. Miller–Rabin factors $560 = 35 \cdot 2^4$ and checks the squaring chain $2^{35} \equiv 263$, $2^{70} \equiv 166$, $2^{140} \equiv 67$, $2^{280} \equiv 1 \pmod{561}$ — finding $67^2 \equiv 1 \pmod{561}$ with $67 \not\equiv \pm 1$, proving $561$ is composite!
3. **Q:** In the Graph Non-Isomorphism interactive proof, why is it essential that the Verifier's coin flip $b$ and permutation $\pi$ be **private** (hidden from the Prover)?  
   *Outline:* If the Prover could see $b$ or $\pi$, a dishonest Prover could trivially output $b$ even when $G_0 \cong G_1$, breaking soundness!
