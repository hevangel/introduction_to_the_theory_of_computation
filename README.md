# Theory of Computation — Exam-Ready Study Notes + Rust & Python (Sipser Editions 1–3)

> **Live Interactive Study Portal (GitHub Pages):**  
> **https://hevangel.github.io/introduction_to_the_theory_of_computation/**

**An original, open educational companion and standalone final-exam study guide** to Michael Sipser's *Introduction to the Theory of Computation* (1st, 2nd, and 3rd editions). Designed for undergraduate computer science students taking a Theory of Computation course, each chapter provides formal definitions, plain-English intuition, step-by-step proof blueprints, worked examples, common exam pitfalls, and practice exam questions — paired with runnable, tested implementations in **both Rust and Python**.

This repository **does not reproduce the textbook, its diagrams, or its exercises and solutions**. Every explanation, proof outline, practice question, and code module is independently authored.

## Study Notes & Code by Chapter

| Ch | Topics Covered (1st, 2nd & 3rd Editions) | Study Notes | Rust | Python |
|---:|---|---|---|---|
| 0 | Sets, relations, equivalence classes, graphs, proof techniques, diagonalization | [00 — Foundations](docs/00-foundations.md) | [ch00.rs](src/ch00.rs) | [ch00.py](python/ch00.py) |
| 1 | DFA, ε-NFA, subset & product constructions, Thompson regex, pumping lemma, **Myhill–Nerode minimization (2e)** | [01 — Regular Languages](docs/01-regular.md) | [ch01.rs](src/ch01.rs) | [ch01.py](python/ch01.py) |
| 2 | CFG, CFG→CNF, CYK, PDA, CFL pumping, **DPDA, DCFL, DCFG & canonical LR(1) parser generator with conflict diagnostics (3e §2.4)** | [02 — Context-Free Languages](docs/02-context-free.md) | [ch02.rs](src/ch02.rs) | [ch02.py](python/ch02.py) |
| 3 | Single-tape TM ($0^{2^n}$), multitape TM ($w\#w$), NTM BFS simulation, enumerators, Church–Turing thesis | [03 — Church–Turing Thesis](docs/03-turing.md) | [ch03.rs](src/ch03.rs) | [ch03.py](python/ch03.py) |
| 4 | Deciders ($A_{\text{DFA}}, E_{\text{DFA}}, EQ_{\text{DFA}}, A_{\text{CFG}}, E_{\text{CFG}}$), diagonalization, $A_{\text{TM}}$, co-recognizability | [04 — Decidability](docs/04-decidability.md) | [ch04.rs](src/ch04.rs) | [ch04.py](python/ch04.py) |
| 5 | Standard & computation-history reductions, $A_{\text{LBA}}$, PCP, mapping reductions ($\le_m$), **Rice's theorem (2e)** | [05 — Reducibility](docs/05-reductions.md) | [ch05.rs](src/ch05.rs) | [ch05.py](python/ch05.py) |
| 6 | Recursion theorem (`SELF`), $\text{Th}(\mathbb{N},+)$ vs $\text{Th}(\mathbb{N},+,\times)$, Gödel incompleteness, oracles, Kolmogorov complexity | [06 — Advanced Computability](docs/06-advanced-computability.md) | [ch06.rs](src/ch06.rs) | [ch06.py](python/ch06.py) |
| 7 | P (2-SAT via Tarjan SCC), NP verifiers, Cook–Levin tableau, NP-completeness reductions (`3SAT → CLIQUE → VC`, `3SAT → SUBSET-SUM`) | [07 — Time Complexity](docs/07-time.md) | [ch07.rs](src/ch07.rs) | [ch07.py](python/ch07.py) |
| 8 | Savitch's theorem (`CAN-YIELD`), PSPACE & `TQBF`, L/NL, **Immerman–Szelepcsényi ($\text{NL} = \text{coNL}$)** | [08 — Space Complexity](docs/08-space.md) | [ch08.rs](src/ch08.rs) | [ch08.py](python/ch08.py) |
| 9 | Space/Time Hierarchy theorems, Relativization (Baker–Gill–Solovay), Boolean circuits & Shannon counting | [09 — Intractability](docs/09-intractability.md) | [ch09.rs](src/ch09.rs) | [ch09.py](python/ch09.py) |
| 10 | 2-approx Vertex Cover, BPP & **repaired Miller–Rabin primality (2e)**, ROBPs, PH, IP (`GNI`), NC (`parallel_prefix_sum`), RSA | [10 — Advanced Complexity](docs/10-advanced-complexity.md) | [ch10.rs](src/ch10.rs) | [ch10.py](python/ch10.py) |

**Edition audits:** See [docs/edition-map.md](docs/edition-map.md) and [review/edition-coverage.md](review/edition-coverage.md) for the verified 1st/2nd/3rd edition change matrix and implementation checklist.

## Run All Tests (Rust + Python)

**Rust** (stable toolchain, zero external crate dependencies):
```sh
cargo test --all-targets
```

**Python** (Python 3.10+, standard library only):
```sh
python3 -m unittest discover -s tests -v
```

### Quick Example in Both Languages

**Python:**
```python
from python.ch01 import regex_to_nfa
from python.ch02 import Cfg, Production, Lr1Parser, var, term

min_dfa = regex_to_nfa("(0|1)*01(0|1)*").to_dfa().minimize()
assert min_dfa.num_states == 3  # Myhill–Nerode minimal DFA
assert min_dfa.accepts("11010")
```

**Rust:**
```rust
use theory::ch01::regex_to_nfa;

let min_dfa = regex_to_nfa("(0|1)*01(0|1)*").unwrap().to_dfa().minimize();
assert_eq!(min_dfa.num_states, 3); // Myhill–Nerode minimal DFA
assert!(min_dfa.accepts("11010"));
```

## How to Study for Your Final Exam Using This Repo

1. **Read the chapter notes** on the [GitHub Pages site](https://hevangel.github.io/introduction_to_the_theory_of_computation/) or directly in `docs/`.
2. **Memorize the Proof Blueprints** (Pumping Lemma adversary game, Subset Construction, CNF conversion, $A_{\text{TM}}$ diagonalization, standard reductions, Rice's Theorem, Cook–Levin tableau, Savitch's `CAN-YIELD`, Immerman–Szelepcsényi inductive counting).
3. **Run and trace the Rust/Python implementations** to build concrete intuition for each construction.
4. **Attempt the Practice Exam Questions** at the end of every chapter before reading the solution outlines.

## Coverage Honesty

Executable code illuminates algorithms, reductions, and finite models; running code can never prove undecidability or complexity lower bounds. Bounded simulators (such as `TuringMachine::run` with a step budget and `bounded_pcp`) explicitly bound execution and do **not** decide undecidable problems.

## Source and Publication Notes

Original editions: first (1997, ISBN 0-534-94728-X), second (2006, ISBN 0-534-95097-3), third (released 2012; ©2013, ISBN 978-1-13318-779-0). Independent educational companion, not affiliated with the author or publisher. Repository MIT license governs this companion's original material only.
