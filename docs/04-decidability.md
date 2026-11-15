# Chapter 4 — Decidable, recognizable, and impossible decisions

**Retained in all three editions.**

## 4.1 Positive decision results

For DFAs, membership can be decided by running transitions. Emptiness is graph reachability from the start to any accepting state; equivalence can be decided by checking whether the *symmetric-difference product machine* has a reachable accepting state.

The [Rust](../src/ch04.rs) and [Python](../python/ch04.py) implementations do product-state search for equivalence of complete binary DFAs. Any state pair with mismatched acceptance yields a distinguishing word (the example returns only a Boolean). Once all reachable pairs have been examined, no distinguishing word exists.

Certain CFG questions, notably membership and emptiness, are decidable; **CFG equivalence is not decidable in general**. Do not generalize every DFA decision procedure to CFGs.

## 4.2 The halting problem

Let \`HALT = {<M,w> : M halts on w}\`. If an algorithm \`H\` decided HALT for all encoded programs and inputs, construct \`D(x)\`: when \`H(x,x)\` says halt, loop forever; otherwise halt. Asking \`D(D)\` produces a contradiction. Therefore no total, correct \`H\` exists.

The classic universal acceptance language \`A_TM\` is recognizable but not decidable. Its complement is not Turing-recognizable. Proof technique: a language is decidable iff both it and its complement are recognizable; dovetail the two recognizers and return whichever accepts first.

## 4.3 Diagonalization and uncountability

Programs are encodable as finite strings and thus countable; infinite binary sequences are uncountable via diagonalization. Consequently, most possible languages over a nonempty finite alphabet are not Turing-recognizable. This is an existence result, not a practical algorithm for finding a named hard language.

## Why the code is finite

The product-automaton decider works because the search space has at most \`|QA|·|QB|\` state pairs. For HALT no analogous finite bound covers arbitrary programs. Setting a million-step cutoff merely creates a bounded question and cannot distinguish a very slow halting program from a diverging one.

## Checkpoints

1. Construct equivalent DFAs with different numbers of states.
2. Exhibit a CFG question that is decidable and one that is not.
3. Explain why an acceptance simulation that has not halted yet cannot safely answer "no."
4. Reconstruct the diagonal contradiction without running the paradoxical program.
