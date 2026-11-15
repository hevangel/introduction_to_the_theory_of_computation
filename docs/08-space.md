# Chapter 8 — Space complexity, Savitch, PSPACE, L and NL

**Retained throughout all three published editions.** (It was added in the move from the earlier preliminary edition to the first published edition.)

## 8.1 Space is not time

Space counts work-tape cells used as a function of input length; the read-only input tape is usually excluded by convention. A machine can spend vast time while reusing little space. Classes include \`L=DSPACE(log n)\`, \`NL=NSPACE(log n)\`, and \`PSPACE=∪k DSPACE(n^k)\`.

## 8.2 Savitch's theorem

For space-constructible \`s(n)≥log n\`, \`NSPACE(s(n)) ⊆ DSPACE(s(n)^2)\`. The central idea: to decide whether configuration \`u\` reaches \`v\` in at most \`2^k\` steps, existentially guess an intermediate configuration \`m\` and recursively check two shorter paths. A deterministic algorithm loops over **all** possible mids; recursion depth is logarithmic in the bound, while each frame stores a small number of configurations.

[Rust](../src/ch08.rs) and [Python](../python/ch08.py) implement finite-graph reachability by recursive midpoint splitting. For a graph with \`N\` vertices and a path length bound \`t\`, it uses \`O(log t)\` recursive frames, each with \`O(log N)\` bits for vertex indices in an idealized model. It may take enormous time and actual runtime stack frames have implementation overhead. This is an intuition for Savitch, not a machine-model-precise proof.

## 8.3 PSPACE-completeness

**TQBF** (true quantified Boolean formulas) is PSPACE-complete. Alternating quantifiers model choices: \`∃x\` means one choice suffices; \`∀x\` means all choices must work. Evaluating recursively requires polynomial depth/space yet can explore exponentially many branches. Generalized games such as Geography express similar alternating strategies. The reductions, not the games' names, establish hardness.

## 8.4 NL and its complement

Directed **s–t reachability** is NL-complete. A nondeterministic log-space machine guesses a path, storing only a current vertex and counter. The surprising theorem **NL=coNL** states nondeterministic log-space classes are closed under complement, proved via counting reachable vertices (not by trivially negating an NTM's answer).

## Checkpoints

- Distinguish DFS/BFS memory from recursive midpoint memory.
- Write a three-quantifier QBF and evaluate it as a game tree.
- Explain why \`NL\` and \`NP\` are different resource classes.
- Prove the midpoint recurrence for path lengths including 0 and 1.
