# Chapter 5 — Reductions: reusing impossibility and hardness

**Core in all editions; Rice's theorem is easy to miss when studying edition 2 by numbered headings only.**

## 5.1 The logic of a reduction

A many-one / mapping reduction \`A ≤m B\` is a **total computable function** \`f\` satisfying \`x∈A iff f(x)∈B\`. If B were decidable then A would be decidable by computing \`f\` and calling B's decider. Therefore **if A is undecidable and A ≤m B, then B is undecidable**. Beware reversing this arrow.

The chapter's classic undecidability demonstrations include TM language questions and reductions via **computation histories**, sequences of configurations whose local consistency is checkable. Post's Correspondence Problem (PCP) converts the existence of a valid computation into a tile-matching instance; knowing what a configuration looks like matters more than memorizing the tiles.

## 5.2 A working polynomial mapping reduction

[Rust](../src/ch05.rs) and [Python](../python/ch05.py) encode **graph 3-coloring into CNF-SAT**. Introduce Boolean variable \`x_(v,c)\` for vertex \`v\` and color \`c\`. Enforce one or more colors per vertex, no two colors simultaneously, and different colors on each edge. Then
\`\`\`text
graph has a proper 3-coloring  <=>  generated formula is satisfiable
\`\`\`
The formula has \`3|V|\` variables and \`O(|V|+|E|)\` clauses. This is a **polynomial-time reduction**. The accompanying SAT solver checks every assignment, however, and is **exponential**—the reducer's efficiency does not make SAT easy.

## 5.3 Rice's theorem: edition 2 selected-problem material

Every nontrivial **semantic property** of the partial function/language recognized by an arbitrary program is undecidable: e.g. "does this TM recognize the empty language?" or "does it accept every string?" Nontrivial means some computable behaviors have the property and some do not. **Syntactic** questions ("does the source contain this token?") are not covered. Also distinguish program semantics from finite input/output tests.

The second-edition preface explicitly places Rice's theorem in solved problems, not its main narrative; retain it here because it gives a powerful unifying view.

## 5.4 Reductions in complexity

For NP-hardness, the mapping must run in **polynomial time**. A general computable reduction does not imply NP-hardness. For an NP-completeness proof, show **both** that the new language lies in NP and that a known NP-complete problem polynomially reduces to it.

## Checkpoints

- Verify both directions of the 3-coloring↔SAT encoding.
- Explain why a bounded brute SAT implementation does not refute NP-completeness.
- Give a trivial property that Rice's theorem does *not* prohibit deciding.
- Prove that composable mapping reductions preserve undecidability in the intended direction.
