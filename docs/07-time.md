# Chapter 7 — Time complexity, P, NP, and reductions

**Core in all editions. Some editions call the final collection of hard problems "examples," others "additional problems"; do not drop the content.**

## 7.1 Measuring computation

State the input size `n`, machine model and worst-case step count. `O(f(n))` means an eventual upper bound up to a constant; `o(f(n))` means a strictly smaller asymptotic growth rate. Polynomial changes between reasonable deterministic computation models motivate using polynomial time as a robustness class, but model choice still matters for exact bounds.

## 7.2 P and NP

**P**: languages decidable in deterministic polynomial time. **NP**: languages with polynomial-length certificates verifiable deterministically in polynomial time (equivalently, accepted in nondeterministic polynomial time). Verification is **not** the same as searching for a certificate.

[Python](../python/ch07.py) and [Rust](../src/ch07.rs) check a proposed vertex cover in linear time in input representation, then find a minimum cover via exhaustive subsets. They also implement nonnegative subset sum dynamic programming: `O(n·target)`, which is **pseudo-polynomial** when target is represented in binary, not polynomial in the input bit length. This is a useful cautionary example.

## 7.3 NP-completeness and Cook–Levin

A language is NP-hard if every NP language polynomially reduces to it; NP-complete if it is also in NP. **SAT** is NP-complete (Cook–Levin). Conceptually, encode the space-time tableau of a polynomial-time nondeterministic computation in a Boolean formula: variables describe cells, states and moves; constraints enforce a legal accepting run. This is not merely "try a bunch of inputs"; it is a size-bounded mathematical simulation.

Typical subsequent reductions include **Vertex Cover**, **Hamiltonian Path**, and **Subset Sum**, which illustrate graph, path and arithmetic forms of nondeterministic choice. Compare with chapter 5: there the goal was undecidability using *computable* reductions, here it is **resource-bounded hardness**.

## P vs NP remains unresolved

The fact that we don't know polynomial algorithms for NP-complete problems does not prove none exist. If any NP-complete problem is in P, then P=NP. The code enumerates examples; it cannot establish P≠NP.

## Checkpoints

- Exhibit a certificate for a particular vertex-cover instance.
- Compare `O(n·target)` with input bits for target `2^k`.
- State exactly what must be proved for NP-completeness.
- Build the 3-coloring reduction in [chapter 5](05-reductions.md) and assess encoding size.
