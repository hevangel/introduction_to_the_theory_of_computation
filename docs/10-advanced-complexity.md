# Chapter 10 — Approximation, randomness, alternation, proofs, parallelism, cryptography

**Advanced survey retained from the first published edition through the third.** Preserve all six subtopics; the first published edition added this chapter relative to the earlier preliminary edition.

## 10.1 Approximation algorithms

When exact optimization is hard, an approximation algorithm returns a feasible solution with a proven cost ratio. For **minimum vertex cover**, select an uncovered edge and add **both** endpoints, repeating until every edge is covered. The chosen edges form a maximal matching; any valid cover needs at least one endpoint of each matching edge. Hence this algorithm's output has size at most **2·OPT**.

Both [Rust](../src/ch10.rs) and [Python](../python/ch10.py) implement that procedure; contrast their answer with the exact exponential solver from chapter 7. This is a guaranteed approximation, not an average empirical claim.

## 10.2 Probabilistic algorithms and BPP

A randomized algorithm gets random bits in addition to its input. **BPP** consists of decision languages with polynomial-time algorithms whose answers have bounded error probability (say, at most 1/3 on either yes or no inputs). Independent repetition with majority vote can drive error lower.

The sample seeded Monte Carlo `pi_estimate` uses random points in a square as a **sampling demonstration only**, not a BPP decision procedure. Avoid confusing pseudorandom sampling, randomized certificates, and arbitrary heuristics with the definition of BPP.

**Primality warning:** the author notes that a gap in the original edition's probabilistic primality analysis was repaired for the second edition (crediting Victor Shoup). Don't teach a flawed old probability argument; modern unconditional deterministic polynomial-time primality also exists (AKS). The repository uses `mod_pow` to illustrate a component often used in number-theoretic algorithms, *not* a cryptographically secure implementation.

Read-once **branching programs** model bounded decisions where each input bit is queried at most once along a computation. Their size/width are measures of constrained computation and may permit lower bounds unlike unrestricted circuits.

## 10.3 Alternation and the polynomial hierarchy

Alternating computation switches existential and universal choices. In polynomial time with a bounded number of alternating quantifier blocks, the levels `Σ_k^P` and `Π_k^P` form the **polynomial hierarchy**. These are refinements beyond NP/coNP; don't mistake a chain of increasingly complex toy tests for a class-separation proof.

## 10.4 Interactive proof systems

An all-powerful prover convinces a resource-bounded randomized verifier through interaction. Completeness means correct statements have convincing proofs; soundness means incorrect ones cannot convince with high probability. A classic theorem is **IP=PSPACE**. Graph nonisomorphism illustrates the role of verifier randomness and interaction.

## 10.5 Parallel computation

Boolean circuit **depth** measures parallel time while size measures total work. `NC` captures problems solvable by uniform polynomial-size families of polylogarithmic-depth circuits. **P-complete** problems are commonly regarded as inherently sequential under efficient parallel reductions, but proving `P≠NC` remains open.

## 10.6 Cryptography

Secret-key encryption and public-key encryption rely on computational assumptions, not the impossibility of solving a problem with unlimited resources. **One-way functions** are efficient to compute but conjecturally difficult to invert on average; **trapdoor** functions allow efficient inversion with secret information. Mathematical modular exponentiation helps explain implementations, but this sample is **not safe for real cryptographic applications**.

## Checkpoints

- Prove the 2-approximation ratio for vertex cover, including empty graphs.
- State the formal two-sided error guarantee of BPP and its amplification.
- Distinguish `IP=PSPACE` from `P=PSPACE`.
- Contrast circuit depth vs circuit size; distinguish NC from NP.
- Explain why modular arithmetic does not itself provide secure encryption.
