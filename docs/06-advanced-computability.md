# Chapter 6 — Self-reference, logic, oracle reductions, information

**Optional advanced survey retained in all published editions.**

## 6.1 The recursion theorem

A program can be made to refer to its own description. The recursion/fixed-point theorem formalizes this: for any effective transformation of program descriptions there exists a program whose behavior matches the transformed version of its own description. A *quine* (program printing its own source) is a useful intuition, not a full proof of the theorem. The theorem supports self-reference without circular assumptions.

## 6.2 Decidable logical theories

A logical *theory* describes which statements follow from axioms. Some restricted structures admit algorithms that decide sentence truth; sufficiently expressive systems encode arithmetic or computation and encounter undecidability. **Syntax**, **proof search**, and **truth in a structure** are distinct decision problems. The existence of a proof enumeration does not always yield a decider for arbitrary truth.

## 6.3 Turing reducibility

`A ≤T B` allows an algorithm for A to ask adaptive queries to an **oracle** for B. Unlike a single mapping `A ≤m B`, it may make multiple queries, with later queries depending on answers. This produces a richer comparison of degrees of uncomputability. An oracle is a mathematical abstraction, not a real callable API with a secretly correct answer for HALT.

## 6.4 Descriptions and algorithmic information

A string has low **Kolmogorov complexity** when a short program can produce it. Fix a universal description language: `K(x)` is the minimum program length generating `x` (up to an additive constant between reasonable universal choices). This quantity is **not computable** in general; otherwise one can derive a paradox by effectively finding strings that cannot have short descriptions.

The [Rust](../src/ch06.rs) and [Python](../python/ch06.py) examples enumerate finite binary strings in length order and encode runs of repeated characters. Enumeration explains effective listing of strings. Run-length encoding illustrates *one* compressor; it does **not** compute, approximate with a certified guarantee, or decide Kolmogorov complexity. A highly random-looking string may compress under a different coding scheme.

## Checkpoints

1. Explain why an enumerator need not decide when it has printed every answer.
2. Distinguish quines, fixed points and the halting diagonal argument.
3. Show a legal use of an oracle in a hypothetical algorithm.
4. Explain why a compressor's failure to shorten a string cannot certify incompressibility.
