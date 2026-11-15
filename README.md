# Theory of Computation — Rust + Python, all three Sipser editions

**An original, open educational companion** to Michael Sipser's *Introduction to the Theory of Computation* (1st, 2nd, and 3rd editions). This repository **does not reproduce the textbook, its diagrams, or its exercises and solutions**. It follows the union of the documented topic outlines, gives its own explanations and thought experiments, and pairs selected ideas with small implementations in **both Rust and Python**.

> **What this is / isn't:** All 11 chapter *topic areas* are mapped and explained; executable examples illuminate selected algorithms. This is not a line-by-line reconstruction, a complete proof of every textbook theorem, or a claim that running code can establish undecidability or complexity lower bounds. Sections without a faithful executable analogue are explained mathematically.

## Read in order

| Book chapter | What you learn | Original lesson | Rust | Python |
|---|---|---|---|---|
| 0 | Proofs, sets, languages, graphs | [Introduction](docs/00-foundations.md) | [ch00.rs](src/ch00.rs) | [ch00.py](python/ch00.py) |
| 1 | DFA, NFA, regex, pumping, Myhill–Nerode | [Regular languages](docs/01-regular.md) | [ch01.rs](src/ch01.rs) | [ch01.py](python/ch01.py) |
| 2 | CFG, PDA, CYK, **DPDA / LR parsing (3rd edition)** | [Context-free](docs/02-context-free.md) | [ch02.rs](src/ch02.rs) | [ch02.py](python/ch02.py) |
| 3 | Turing machines, Church–Turing thesis | [Turing machines](docs/03-turing.md) | [ch03.rs](src/ch03.rs) | [ch03.py](python/ch03.py) |
| 4 | Decidability, diagonalization, HALT | [Decidability](docs/04-decidability.md) | [ch04.rs](src/ch04.rs) | [ch04.py](python/ch04.py) |
| 5 | Computation histories, mapping reductions, **Rice's theorem** | [Reducibility](docs/05-reductions.md) | [ch05.rs](src/ch05.rs) | [ch05.py](python/ch05.py) |
| 6 | Self-reference, logic, Turing reductions, information | [Advanced computability](docs/06-advanced-computability.md) | [ch06.rs](src/ch06.rs) | [ch06.py](python/ch06.py) |
| 7 | P, NP, Cook–Levin, NP-completeness | [Time](docs/07-time.md) | [ch07.rs](src/ch07.rs) | [ch07.py](python/ch07.py) |
| 8 | PSPACE, Savitch, NL/coNL | [Space](docs/08-space.md) | [ch08.rs](src/ch08.rs) | [ch08.py](python/ch08.py) |
| 9 | Hierarchies, relativization, circuit complexity | [Intractability](docs/09-intractability.md) | [ch09.rs](src/ch09.rs) | [ch09.py](python/ch09.py) |
| 10 | Approximation, BPP, PH, IP, NC, crypto | [Advanced complexity](docs/10-advanced-complexity.md) | [ch10.rs](src/ch10.rs) | [ch10.py](python/ch10.py) |

**Start with the [edition change and retention matrix](docs/edition-map.md).** It records verified additions and explains why material appearing only in solved problems must not be skipped. No major chapter was confirmed deleted across the three published editions; undocumented fine-grained removals are explicitly **not** inferred from a missing headline.

## Run everything

Rust stable (no dependencies):
\`\`\`sh
cargo test
\`\`\`

Python 3.10+ (standard library only):
\`\`\`sh
python -m unittest discover -s tests -v
\`\`\`

For example, try the same concept in both languages:

\`\`\`python
from python.ch02 import balanced_parentheses, anbn
assert balanced_parentheses("(()())")
assert anbn("aaabbb")
\`\`\`

Rust library usage (inside the crate or a Rust integration test):

\`\`\`rust
assert!(theory::ch02::balanced_parentheses("(()())"));
assert!(theory::ch02::anbn("aaabbb"));
\`\`\`

## How to study an example

1. State the **mathematical language or problem**, including its alphabet, inputs and outputs.
2. Predict the result for positive, negative and boundary cases.
3. Run both implementations; identify the **invariant** or certificate.
4. Prove correctness independently. A finite test suite is never a general proof.
5. Analyze asymptotic resource use on the specified computational model.
6. Read the **edition-specific note** before moving on.

## Coverage honesty

Examples are small, runnable **illustrations**, not production recognizers or solvers. The machine in chapter 3 is a single fixed TM; its timeout does **not** solve the halting problem. The chapter 6 compressor is **not** Kolmogorov complexity. Counting Boolean functions in chapter 9 is **not** a circuit lower-bound proof. The chapter 10 random sampler is **not** a BPP algorithm for an arbitrary decision problem. LR(k) parsing theory is taught in the chapter 2 notes, but a full canonical LR(k) parser generator is not yet implemented.

## Source and publication notes

Original editions: first (1997, ISBN 0-534-94728-X), second (2006, ISBN 0-534-95097-3), third (released 2012; ©2013, ISBN 978-1-13318-779-0). The 1996 **preliminary edition** is separate and is **not** counted as a fourth published edition here. Source citations and caveats are in [docs/edition-map.md](docs/edition-map.md).

Independent work inspired by the topics of the textbook, not affiliated with the author or publisher. Repository license governs this companion's original material only.
