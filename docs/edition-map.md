# What changed across the three published editions?

**Read this before assuming a topic disappeared.** The table below distinguishes *documented addition*, *retained core*, *editorial change*, and *not confirmed*. A blank in a table of contents does not prove deletion from the book (it may be a solved problem or a renamed subheading). The preliminary paperback predates the first edition and is not one of the three editions covered.

## Edition-to-edition audit

| Area | First (1997) | Second (2006) | Third (released July 2012, ©2013) | Retention action |
|---|---|---|---|---|
| Overall 0–10 architecture | Present | Retained | Retained | Study all eleven chapters |
| Ch 0 mathematical toolkit and proof forms | Present | Retained | Retained | Keep construction, contradiction, induction |
| Ch 1 DFA/NFA, regex, pumping | Present | Retained | Retained | Study automata-to-regex equivalences |
| **Myhill–Nerode theorem** | Not a principal numbered main-text section | **Discussed in selected solved problems** | Kept in problems/solutions | **Do not miss equivalence classes/minimization**, even if only following body headings |
| Ch 2 CFG/PDA/normal form/CFL pumping | Present | Retained | Retained | Keep nondeterministic PDA theory |
| **Deterministic CFL, DPDA, deterministic CFG, LR(k)** | **Not in numbered body** | **Not in numbered body** | **New section 2.4** with DPDA, properties, DCFG, DPDA–DCFG relation, LR(k) parsing | **Study independently; major third-edition addition** |
| Ch 3 Church–Turing, multitape, nondeterminism, enumerators | Present | Retained | Retained | Keep recognizer vs decider distinction |
| Ch 4 decidability, diagonalization, HALT | Present | Retained | Retained | Keep recognizability/complements |
| Ch 5 computation-history reductions and mapping reductions | Present | Retained | Retained | Keep correct reduction direction |
| **Rice's theorem** | Not a principal numbered main-text section | **Discussed in selected solved problems** | Kept in exercises/solutions | **Do not skip** nontrivial semantic properties |
| Ch 6 recursion theorem, logic, Turing reducibility, information | Present | Retained | Retained | Keep all four sections |
| Ch 7 P, NP, NP-completeness/Cook–Levin, examples | Present | Retained; the examples subsection is styled *Additional NP-complete Problems* | Retained | Treat wording as **rename**, not removal |
| Ch 8 Savitch, PSPACE, L/NL/coNL | Present | Retained | Retained | Don't confuse NL with NP |
| Ch 9 hierarchy, relativization, circuits | Present | Retained | Retained | Formal lower bounds are not experiments |
| Ch 10 approximation, BPP, read-once programs, alternation, IP, NC, cryptography | Present | Retained | Retained | Preserve advanced survey topics |
| Chapter-level solutions | Exercises/problems, without the second edition's selected-solutions section | **Selected Solutions added to every chapter; additional problems added** | More exercises/problems/solutions | Use this companion's *original* checkpoints; do not copy publisher solutions |
| Probabilistic primality discussion | Earlier proof had an analysis gap | Author explicitly credits a repair by Victor Shoup | Retained in revised text | Don't teach the faulty earlier analysis as a theorem |
| Substantial deletions from the first to third published editions | — | **None verified at the chapter/numbered-section level** | **None verified at that level** | Do not claim detailed problem-by-problem deletion audit |

## First → second: what you might miss

The author explicitly says the chief change was the addition of **Selected Solutions** to all chapters, accompanied by new problems. He also explains his choice to treat **Myhill–Nerode** and **Rice's theorem** in the solved problems rather than the main narrative. Thus a reader who only transcribes headings from the first edition may entirely overlook two powerful standard results. He also records a correction to the analysis of a probabilistic primality algorithm. These are not new top-level chapters. See the original discussion in the second-edition preface. Theoretical background is in [01](01-regular.md), [05](05-reductions.md), and [10](10-advanced-complexity.md).

## Second → third: the major new piece

The new **section 2.4** explains **deterministic pushdown automata (DPDA)** and deterministic context-free languages (DCFL), explores closure results, develops deterministic grammar concepts, connects grammar and DPDA descriptions, and turns to parsing and **LR(k)** grammars. It is optional for later textbook chapters but *not optional for this union-of-editions project*. The author and publisher separately describe it as the main significant addition. This project's [chapter 2 lesson](02-context-free.md) teaches the mathematical ideas; the Rust/Python implementation currently gives a deterministic stack recognizer plus CYK, **not** a complete LR parser generator.

## What about removals?

Comparing published chapter structures, no major theory area is identified as deleted. It would be misleading to invent a list of removed algorithms based solely on page numbers, editorial renamings or displaced problems. A complete line-by-line comparison of every exercise and example across all printings has **not** been performed. In particular, do not conflate the smaller **1996 preliminary edition** with the first published edition; the *first edition* expanded the preliminary edition with final chapters 8–10 and expanded chapter 6.

## Edition coverage checklist

- [x] All published first-edition chapter topics in 0–10 mapped
- [x] Second-edition selected-solutions **change documented** (publisher exercises not reproduced)
- [x] Myhill–Nerode and Rice's theorem explained outside the main headings
- [x] Corrected primality analysis **flagged**; no reproduction of earlier flawed proof
- [x] Third-edition deterministic context-free and parsing material included in lesson
- [x] Renamed vs removed heading distinction
- [ ] Full **problem-by-problem** three-way inventory (requires licensed access and meticulous comparison)
- [ ] Full proof formalization and full Rust/Python implementations of all topics
- [ ] Complete canonical LR(0)/LR(1) generator with conflict diagnostics

## Primary and bibliographic sources

1. **Michael Sipser, official book page:** https://math.mit.edu/~sipser/book — explicitly dates third edition to July 2012 and names the new deterministic-CFL section.
2. **Cengage, third edition:** https://www.cengage.com/c/introduction-to-the-theory-of-computation-3e-sipser/9781133187790 — publisher describes new theoretical deterministic-CFL/LR(k) treatment.
3. **First-edition table of contents (1997 bibliographic record):** https://openlibrary.org/books/OL996653M/Introduction_to_the_theory_of_computation
4. **MIT OpenCourseWare, first/second comparison and assigned chapters:** https://mitocw.ups.edu.ec/courses/mathematics/18-404j-theory-of-computation-fall-2006/readings/ — refers to a combined first/second-edition table of contents.
5. **Second-edition author preface and contents** (ISBN 0-534-95097-3, 2006), in library holdings. This is the basis for the statements about new selected solutions, Myhill–Nerode, Rice's theorem, and the primality proof repair.
6. **Third-edition author preface and contents** (ISBN 978-1-13318-779-0, copyright 2013), especially section 2.4.
7. **Official errata:** https://math.mit.edu/~sipser/itoc-errs3.1.html — check before relying on printed statements, theorem wording, or problem numbers.

Only short **topic names and editorial facts** are used from those sources; the lessons and code are independent original explanations.
