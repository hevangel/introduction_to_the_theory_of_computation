# What changed across the three published editions?

**Read this before assuming a topic disappeared.** The table below distinguishes *documented addition*, *retained core*, *editorial change*, and *not confirmed*. A blank in a table of contents does not prove deletion from the book (it may be a solved problem or a renamed subheading). The preliminary paperback predates the first edition and is not one of the three editions covered.

## Edition-to-edition audit

| Area | First (1997) | Second (2006) | Third (released July 2012, ©2013) | Retention & Implementation Status |
|---|---|---|---|---|
| Overall 0–10 architecture | Present | Retained | Retained | All 11 chapters covered in exam-ready study notes + Rust & Python |
| Ch 0 mathematical toolkit & proof forms | Present | Retained | Retained | Notes + BFS, cycle detection, power set, equivalence classes, diagonalization |
| Ch 1 DFA/NFA, regex, pumping | Present | Retained | Retained | Notes + general DFA, ε-NFA, subset & product constructions, Thompson regex |
| **Myhill–Nerode theorem** | Not a principal numbered main-text section | **Discussed in selected solved problems** | Kept in problems/solutions | **Implemented:** table-filling equivalence classes + DFA minimization (`src/ch01.rs`, `python/ch01.py`) |
| Ch 2 CFG/PDA/normal form/CFL pumping | Present | Retained | Retained | Notes + CFG→CNF conversion, CYK (with ε), nondeterministic PDA |
| **Deterministic CFL, DPDA, DCFG, LR(k)** | **Not in numbered body** | **Not in numbered body** | **New section 2.4** with DPDA, properties, DCFG, DPDA–DCFG relation, LR(k) parsing | **Implemented:** `Dpda` determinism checker/simulator + **canonical LR(1) parser generator with Shift/Reduce & Reduce/Reduce conflict diagnostics** (`src/ch02.rs`, `python/ch02.py`) |
| Ch 3 Church–Turing, multitape, nondeterminism, enumerators | Present | Retained | Retained | Notes + general single-tape TM, Sipser $M_2$ ($0^{2^n}$), 2-tape $w\#w$ TM |
| Ch 4 decidability, diagonalization, $A_{\text{TM}}$ | Present | Retained | Retained | Notes + deciders for $A_{\text{DFA}}$, $E_{\text{DFA}}$, $EQ_{\text{DFA}}$, $E_{\text{CFG}}$, $A_{\text{CFG}}$ |
| Ch 5 computation-history reductions & mapping reductions | Present | Retained | Retained | Notes + $A_{\text{LBA}}$ cycle-detecting decider, Bounded PCP, 3COLOR→SAT, 3SAT→CLIQUE |
| **Rice's theorem** | Not a principal numbered main-text section | **Discussed in selected solved problems** | Kept in exercises/solutions | Full formal statement, 5-line proof blueprint, and apply/fail exam guide in [docs/05-reductions.md](05-reductions.md) |
| Ch 6 recursion theorem, logic, Turing reducibility, information | Present | Retained | Retained | Notes + quine (`SELF`), Cantor pairing, $(\mathbb{Z}_m, +)$ first-order model checker, RLE |
| Ch 7 P, NP, Cook–Levin, NP-completeness | Present | Retained (§7.5 styled *Additional NP-complete Problems*) | Retained | Notes + 2-SAT (Tarjan SCC), NP verifiers, CLIQUE→VC, 3SAT→SUBSET-SUM, DP |
| Ch 8 Savitch, PSPACE, L/NL/coNL | Present | Retained | Retained | Notes + Savitch `CAN-YIELD`, TQBF solver, Immerman–Szelepcsényi inductive counting |
| Ch 9 hierarchy, relativization, circuits | Present | Retained | Retained | Notes + Boolean circuit evaluator, XOR circuit, Shannon counting, Baker–Gill–Solovay oracle demo |
| Ch 10 approximation, BPP, ROBP, alternation, IP, NC, crypto | Present | Retained | Retained | Notes + 2-approx VC, Miller–Rabin, ROBP, GNI verifier, parallel prefix sum (NC), RSA |
| Chapter-level solutions | Exercises/problems, no selected-solutions section | **Selected Solutions added to every chapter** | More exercises/problems/solutions | Original exam-style practice questions + solution outlines in every chapter |
| Probabilistic primality discussion | Earlier proof had an analysis gap | Author explicitly credits a repair by Victor Shoup | Retained in revised text | Repaired Miller–Rabin analysis taught & implemented (catching Carmichael 561) |
| Substantial deletions from 1st to 3rd published editions | — | **None verified at chapter/section level** | **None verified at that level** | Do not claim problem-by-problem deletion audit |

## Edition coverage checklist

- [x] All published first-edition chapter topics in 0–10 mapped and explained as standalone exam study notes
- [x] Second-edition selected-solutions **change documented** (publisher exercises not reproduced)
- [x] Myhill–Nerode theorem explained in notes and implemented in Rust + Python (`myhill_nerode_classes`, `minimize`)
- [x] Rice's theorem explained in notes with full proof blueprint and exam pitfalls
- [x] Corrected primality analysis (Miller–Rabin / Victor Shoup repair) taught and implemented in Rust + Python
- [x] Third-edition Section 2.4 (DPDA, DCFL, DCFG, LR(k)) explained in notes and implemented in Rust + Python
- [x] Complete canonical LR(1) parser generator with Shift/Reduce and Reduce/Reduce conflict diagnostics in Rust + Python
- [x] Mirrored, tested Rust and Python implementations across all 11 chapters (0–10)
- [ ] Full **problem-by-problem** three-way inventory (requires licensed access to all printings; omitted intentionally to respect copyright)

## Primary and bibliographic sources

1. **Michael Sipser, official book page:** https://math.mit.edu/~sipser/book — explicitly dates third edition to July 2012 and names the new deterministic-CFL section.
2. **Cengage, third edition:** https://www.cengage.com/c/introduction-to-the-theory-of-computation-3e-sipser/9781133187790 — publisher describes new theoretical deterministic-CFL/LR(k) treatment.
3. **First-edition table of contents (1997 bibliographic record):** https://openlibrary.org/books/OL996653M/Introduction_to_the_theory_of_computation
4. **MIT OpenCourseWare (18.404J):** https://mitocw.ups.edu.ec/courses/mathematics/18-404j-theory-of-computation-fall-2006/readings/
5. **Second-edition author preface and contents** (ISBN 0-534-95097-3, 2006).
6. **Third-edition author preface and contents** (ISBN 978-1-13318-779-0, copyright 2013), especially Section 2.4.
7. **Official errata:** https://math.mit.edu/~sipser/itoc-errs3.1.html

Only short **topic names and editorial facts** are used from those sources; all study notes, proof blueprints, practice questions, and Rust/Python code are independent original material.
