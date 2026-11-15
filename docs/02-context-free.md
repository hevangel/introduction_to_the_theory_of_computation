# Chapter 2 — Context-free languages and deterministic parsing

**Core in all editions; substantial new section 2.4 in the third.**

## 2.1 Context-free grammars

A CFG has variables `V`, terminals `Σ`, productions `A→α` and start `S`. A *derivation* repeatedly rewrites variables; the leaves of a parse tree yield a word. Example: `S→aSb | ε` generates `a^n b^n`. This requires unbounded matching depth and is not regular.

Ambiguity occurs when a word has more than one parse tree (or leftmost derivation); a parser's *choice* of tree does not erase the grammar's ambiguity. Chomsky normal form (CNF) uses `A→BC` and `A→a`, with a controlled exception for `S→ε`. It preserves the language after suitable transformations, including eliminating useless, epsilon, and unit rules with care.

## 2.2 Pushdown automata

A PDA extends a finite-state machine with a LIFO stack. Nondeterministic PDAs recognize exactly the CFLs; this generalizes the CFG↔PDA conversion theorem. The [Rust](../src/ch02.rs) and [Python](../python/ch02.py) examples use deterministic stack-like bookkeeping to recognize balanced parentheses and `a^n b^n`. Both accept the empty word. For `a^n b^n`, the state must prevent returning to the `a` phase after reading `b`.

**CYK** illustrates general grammar membership for nonempty input in a CNF grammar: `T[i,j]` stores variables deriving the substring from `i` through `j`. Base cells come from terminal rules; longer spans combine two shorter spans. Runtime `O(n³·|R|)` for a straightforward implementation; space `O(n²|V|)`. The supplied CYK interface deliberately handles `S→ε` separately.

## 2.3 Non-context-free languages

A context-free pumping lemma splits a long member as `uvxyz`; the two pieces `v` and `y` pump **together**, `|vy|>0`, `|vxy|≤p`. A standard target is `{a^n b^n c^n:n≥0}`: pumping one bounded window cannot preserve equality of all three counts. Again the pumping condition is necessary, not sufficient, and a carefully chosen word plus *all permissible decompositions* is required.

CFLs are closed under union, concatenation and star, but generally not intersection or complement. Their intersection with a regular language is a CFL.

## 2.4 NEW in edition 3: deterministic CFLs

Deterministic PDAs (DPDAs) cannot freely choose among alternative actions. At a fixed state and stack top, their epsilon/input behavior must not introduce a choice. **DCFLs form a proper subclass of CFLs**: nondeterminism genuinely adds power for PDAs, unlike for finite automata.

A key useful property is complement closure for properly formulated deterministic CFL recognition, with attention to endmarkers and acceptance conventions. DCFLs are not generally closed under union/intersection, but intersection with regular languages is safe. Don't infer a closure property merely from its regular-language counterpart.

### DCFG, handles and the DPDA connection

Bottom-up parsing replaces a *handle* (a substring corresponding to a production's right side in an appropriate sentential form) by its generating variable. A deterministic CFG formalism enforces that the correct reductions are forced rather than requiring guessing. The third edition develops the relationship between these descriptions and deterministic PDAs; **endmarkers matter** in formal equivalence statements.

### LR(k) parsing

An LR parser scans input **L**eft to right, constructs a **R**ightmost derivation in reverse, and looks ahead up to `k` terminals. Its stack records parser states and grammar symbols; an automaton of LR *items* drives shift/reduce actions. Different items with the same context can create shift/reduce or reduce/reduce conflicts. LR(0) is very restrictive; LR(1) lookahead resolves many conflicts without backtracking.

For an illustrative arithmetic grammar:
```text
E -> E + T | T
T -> T * F | F
F -> ( E ) | n
```
the parser must respect precedence and grouping rather than merely matching parentheses. **This repo does not yet implement an LR(k) parser generator.** The deterministic parenthesis and CYK examples illustrate complementary pieces, **not** a proof of their equivalence to LR parsing.

## Exercises to try

1. Trace the CYK table for the grammar `S→AB, A→a, B→b` on `ab` and `aa`.
2. Give two grammars for balanced parentheses and explain why their syntax differs.
3. Contrast a language that is CFL but nonregular with one that is non-CFL.
4. Write the initial closure of LR(0) items for `S'→S, S→aS | a`; explain the conflict.
