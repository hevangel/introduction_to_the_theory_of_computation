# Chapter 0 — Mathematical and proof foundations

**Core across all three editions.** Theory of computation begins with a *precise question*: given a representation of an input, what is the set of valid strings, and what mathematical evidence establishes an answer?

## 0.1 Three questions, three fields

*Automata theory* classifies languages recognized under memory restrictions. *Computability* asks whether any algorithm solves a specified problem. *Complexity* asks how resources (time, tape cells, random bits, circuit size) grow **when** a solution is computable. A language is a set of finite strings, not an English language: if \`Σ={0,1}\`, then \`Σ*\` includes the empty word \`ε\` and every finite binary word.

## 0.2 Mathematical objects

Sets have membership and union/intersection/complement operations. Sequences are ordered and may repeat entries; tuples have fixed arity. A function assigns **exactly one** output to each input in its domain. Relations are subsets of Cartesian products. A directed graph is \`G=(V,E)\`, with \`E ⊆ V×V\`. Boolean connectives combine propositions; quantifiers matter more than prose: \`∀x ∃y\` need not imply \`∃y ∀x\`.

A language \`L ⊆ Σ*\` corresponds to a Boolean characteristic function **only if** membership is decidable. Even when no algorithm computes this function, the mathematical set still exists.

## 0.3 What constitutes a proof?

A *definition* stipulates meaning. A *theorem* makes a claim; a *proof* supplies a valid finite argument. For a statement \`P(n)\`, induction needs **base** \`P(0)\` and **step** \`P(n)⇒P(n+1)\`. Construction exhibits an object; contradiction assumes a negated claim and derives impossibility. An exhaustive loop proves a result only over the exhaustive **finite domain actually checked**, not all inputs.

## Code laboratory: reachable vertices

Both [Rust](../src/ch00.rs) and [Python](../python/ch00.py) implement BFS. Its invariant: every discovered vertex is reachable from the start, and every undiscovered vertex directly adjacent to a processed vertex will be scheduled. It terminates because vertices are enqueued once. If all reachable vertices have been discovered on termination, the output is exactly the reachability set.

*Resource estimate*: \`O(|V|+|E|)\` time with adjacency lists, \`O(|V|)\` auxiliary space. This is a mathematical claim about a model, not a stopwatch reading.

## Your own exercises

1. Show by induction that the number of binary strings of length \`n\` is \`2^n\`.
2. Build a graph where depth-first traversal visits vertices in a different order but finds the same reachability set.
3. Find a relation that is symmetric and transitive yet not reflexive.

**Edition note:** proof techniques and mathematical language remain the toolkit in editions 1, 2, and 3.
