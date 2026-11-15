# Chapter 1 — Regular languages: finite memory

**All three editions; second edition adds notable solved-problem coverage of Myhill–Nerode.**

## 1.1 DFA: computation is a state transition

A deterministic finite automaton is \`(Q,Σ,δ,q₀,F)\`. The transition function \`δ:Q×Σ→Q\` is total; extend it to strings by repeated transitions. A word is accepted iff the final state lies in \`F\`. Determinism means exactly one next state, **not** that the machine must always accept.

[Rust](../src/ch01.rs) and [Python](../python/ch01.py) recognize binary words containing \`01\`. The states record: not yet seen a useful prefix; just saw a \`0\`; have seen \`01\`. The accepting state is absorbing. Prove correctness by induction on the consumed prefix and its state invariant.

## 1.2 NFA and subset construction

An NFA allows a **set** of next states and optionally epsilon transitions. Its semantics are existential: accept if **at least one** path ends accepting. Our second executable example tracks possible NFA states for words ending \`ab\`: the starting state can keep scanning or guess that an \`a\` begins the final suffix.

**NFA ≡ DFA in expressive power**: a DFA can represent each reachable subset of NFA states (and epsilon closures). There can be exponentially many subsets, but finite memory remains finite. Epsilon closure is essential for general NFAs; the sample NFA uses no epsilon moves.

## 1.3 Regular expressions and closure

The theoretical regex grammar starts with \`∅\`, \`ε\`, and literal symbols, then constructs **union**, **concatenation**, and **Kleene star**. Example: \`(0|1)*01(0|1)*\` describes the same language as the first DFA. This is not identical to every programming language's extended regex engine: backreferences go beyond classical regular expressions.

Regular languages are closed under union, concat, star, complement, and intersection (using product constructions). To prove equivalence of a regex and automaton, show translations in both directions; testing examples alone isn't enough.

## 1.4 Nonregularity: pumping lemma

For a regular language, sufficiently long accepted words can be split \`xyz\` with \`|xy|≤p\`, \`|y|>0\`, and \`xy^iz\` accepted for all \`i≥0\`. To disprove regularity of \`{0^n1^n : n≥0}\`, choose \`0^p1^p\`; every eligible \`y\` consists only of zeroes. Pumping it changes the zero count while preserving the one count.

**Pitfall:** this is a necessary condition, not a full characterization. Showing that some word pumps does not prove regularity.

## Myhill–Nerode: don't miss this in edition 2

Define \`x ~L y\` when, for every suffix \`z\`, \`xz∈L ⇔ yz∈L\`. These are *indistinguishable prefixes*. A language is regular iff this equivalence has **finitely many** equivalence classes; the number of classes is the size of its minimal complete DFA. The \`01\`-substring language has three such classes, exactly matching the three logical states above.

The theorem is discussed in **selected solutions**, not made a numbered main-body section in edition 2. Include it in your study regardless of edition.

## Checkpoints

- Give a DFA for binary words with an even number of \`1\` symbols.
- Explain why NFAs do not recognize more languages than DFAs.
- Find infinitely many distinguishable prefixes for \`{0^n1^n}\`.
- Construct the product automaton for intersection of two regular languages.
