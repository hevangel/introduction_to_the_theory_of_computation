# Chapter 3 — Turing machines and the Church–Turing thesis

**Retained in all published editions.**

## 3.1 A precise machine

A Turing machine has finite control, a tape alphabet with blank, a tape head, and transitions that read, write and move. Standard versions halt by entering designated accepting or rejecting states; a transition may also run forever. A *recognizer* need only halt on accepted inputs; a *decider* must halt on **every** input with the correct yes/no result.

The [Rust](../src/ch03.rs) and [Python](../python/ch03.py) programs simulate one fixed, simple machine: on unary input `111`, scan right to blank, write `1`, accept, yielding `1111`. Invalid characters reject. A supplied step **budget** causes `Timeout` when insufficient. Timeout is not evidence that a machine will never halt.

### Invariants and cost

Before each step of this specific machine, all scanned tape cells contain `1`; the head points to the first not-yet-scanned position. On an input of length `n`, acceptance takes `n+1` transitions. A general simulator should represent the finite nonblank region sparsely, retain state and head index, and check for explicit halting; the toy machine does not try to be such a universal simulator.

## 3.2 Changes of computational model

Multitape machines, nondeterministic machines, and machines with various tape conventions can simulate the standard single-tape model. They compute the same *decidable/recognizable* language families, while their **time complexity may change**, so complexity comparisons must declare the model.

An *enumerator* prints a sequence of words. A language is Turing-recognizable iff some enumerator lists its members (duplicates are allowed); a fair dovetailing schedule avoids getting stuck forever on one nonterminating computation.

## 3.3 The thesis and its limits

The Church–Turing thesis asserts that effectively calculable procedures can be computed by Turing machines. It is a foundational thesis connecting the informal idea of **effective algorithm** to a precise mathematical model, not a theorem proven from a formal definition of the informal notion. Distinguish it from the **extended** thesis about efficient simulation, which is a stronger and contested efficiency claim.

### Checkpoints

- Model a unary incrementer with explicit state/tape transition rules.
- Explain why more tapes do not extend decidability but can affect time bounds.
- Show how to enumerate outputs from many recognition computations without starvation.
- Produce a three-valued bounded simulation outcome; explain why it cannot be made into a universal HALT decider.
