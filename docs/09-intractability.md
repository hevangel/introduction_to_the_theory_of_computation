# Chapter 9 — Proven limits: hierarchies, relativization and circuits

**Retained in all three editions; sometimes titled "Provable Intractability" or "Intractability."**

## 9.1 Time and space hierarchy

For appropriate constructible bounds, more time (or more space) really allows strictly more languages. A diagonal machine simulates machines of a lower resource budget and deliberately disagrees at a carefully chosen input. These theorems prove unconditional separations **between sufficiently different bounds**, not the famous close question P versus NP.

A proof must account for universal-simulation overhead, padding and constructibility. Saying "my computer needed more time" is not a hierarchy theorem.

## 9.2 Relativization and its limits

Give a Turing machine an oracle `A`; define `P^A` and `NP^A`. There are oracles relative to which P and NP are equal, and others relative to which they are unequal. Therefore a proof technique that relativizes in every oracle world cannot by itself resolve P vs NP. This is a **barrier for a class of proof methods**, not a result that P vs NP is meaningless.

## 9.3 Circuit complexity

A Boolean circuit is a finite acyclic network of gates computing a fixed input-length function. Circuit *families* specify one network per input length. There are `2^(2^n)` Boolean functions of `n` inputs but far fewer circuits of very small size; counting establishes that *some* functions need large circuits. Producing explicit super-polynomial lower bounds for general circuits is vastly harder.

The [Rust](../src/ch09.rs) and [Python](../python/ch09.py) functions count Boolean functions for tiny arities and list a two-input XOR truth table. The count is only **intuition**. It is not an explicit lower bound nor proof of any unresolved complexity separation. Circuit size, depth, uniformity and fan-in are separate metrics.

## Checkpoints

1. State the difference between an existential counting argument and an explicit lower bound.
2. Explain a hypothetical oracle query for SAT and why it is not a real polynomial SAT algorithm.
3. Identify where a diagonal lower-bound argument must pay for simulation overhead.
4. Why does a fixed input size permit a finite lookup table without solving asymptotic complexity?
