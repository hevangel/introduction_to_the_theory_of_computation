# Edition Coverage Audit — Sipser *Introduction to the Theory of Computation* (1st, 2nd, 3rd Editions)

This audit summarizes topic-by-topic coverage across all three published editions (1997, 2006, 2012/2013) and tracks every gap implemented in **both Rust (`src/ch*.rs`) and Python (`python/ch*.py`)** alongside the standalone undergraduate study notes (`docs/*.md`). See also [docs/edition-map.md](../docs/edition-map.md).

## Summary of Gaps Closed

| Gap Identified in Initial Audit | Resolution (Study Notes + Dual-Language Code) |
|---|---|
| **Ch 0:** Only BFS reachability existed | Added `power_set`, `is_equivalence_relation`, `equivalence_classes`, `cantor_diagonal_bitstring`, `has_cycle` in Rust & Python + full exam-ready notes in [docs/00-foundations.md](../docs/00-foundations.md) |
| **Ch 1 (2nd ed. Myhill–Nerode + core constructions):** Only hardcoded `contains_01` and `ends_ab_nfa` | Added general `Dfa` (accept, complement, product $\cup/\cap$, reachability), `Nfa` with $\varepsilon$-closure & subset construction (`to_dfa`), **Myhill–Nerode table-filling equivalence classes & DFA minimization**, and **Thompson's regex-to-NFA compiler** in Rust & Python + comprehensive notes in [docs/01-regular.md](../docs/01-regular.md) |
| **Ch 2 (3rd ed. §2.4 DPDA/DCFL/LR(k) + CFG/PDA):** Missing CFG→CNF, general PDA, general DPDA, and canonical LR(1) parser generator with conflict diagnostics | Added `Cfg::to_cnf`, `cyk_with_epsilon`, general nondeterministic `Pda`, deterministic `Dpda` with determinism verifier, and **canonical `Lr1Parser` generator with Shift/Reduce and Reduce/Reduce conflict diagnostics** in Rust & Python + full §2.1–2.4 study notes in [docs/02-context-free.md](../docs/02-context-free.md) |
| **Ch 3 (Turing Machines):** Only `unary_successor` | Added general single-tape `TuringMachine`, Sipser's classic $M_2$ (`tm_power_of_two_zeros`), and 2-tape `two_tape_ww_hash` in Rust & Python + full §3.1–3.3 study notes in [docs/03-turing.md](../docs/03-turing.md) |
| **Ch 4 (Decidability):** Only binary `dfa_equivalent` | Added general deciders `a_dfa`, `e_dfa`, `eq_dfa`, `e_cfg`, `a_cfg` in Rust & Python + full §4.1–4.2 study notes (diagonalization, $A_{\text{TM}}$, Theorem 4.22) in [docs/04-decidability.md](../docs/04-decidability.md) |
| **Ch 5 (Reducibility + 2nd ed. Rice's Theorem):** Only `three_color_to_cnf` | Added `lba_accepts` (cycle-detecting $A_{\text{LBA}}$ decider), `bounded_pcp`, and `three_sat_to_clique` in Rust & Python + full §5.1–5.3 & Rice's Theorem notes in [docs/05-reductions.md](../docs/05-reductions.md) |
| **Ch 6 (Advanced Computability):** Only shortlex enum & RLE | Added `cantor_pair`/`cantor_unpair`, `self_reproducing_string` (Recursion Theorem / `SELF`), and `eval_fo_mod` (first-order model checker over $(\mathbb{Z}_m, +)$) in Rust & Python + full §6.1–6.4 study notes in [docs/06-advanced-computability.md](../docs/06-advanced-computability.md) |
| **Ch 7 (Time Complexity):** Missing P algorithms (2-SAT), CLIQUE/HAMPATH verifiers, and classic NP-completeness reductions | Added `two_sat_solve` (Tarjan SCC), `verifies_clique`, `verifies_hampath`, `clique_to_vertex_cover`, and `three_sat_to_subset_sum` (Sipser Thm 7.56) in Rust & Python + full §7.1–7.5 study notes in [docs/07-time.md](../docs/07-time.md) |
| **Ch 8 (Space Complexity):** Only Savitch `reach_within` | Added `eval_tqbf` (PSPACE-complete) and `immerman_szelepcsenyi_non_reach` (inductive counting for $\text{NL} = \text{coNL}$) in Rust & Python + full §8.1–8.6 study notes in [docs/08-space.md](../docs/08-space.md) |
| **Ch 9 (Intractability):** Only truth-table count & XOR tuple | Added `BooleanCircuit` evaluator, `xor_circuit`, and `oracle_exists_of_length` (Baker–Gill–Solovay $U_B$) in Rust & Python + full §9.1–9.3 study notes in [docs/09-intractability.md](../docs/09-intractability.md) |
| **Ch 10 (Advanced Complexity + 2nd ed. Primality Repair):** Missing Miller–Rabin, ROBP, IP/GNI, NC, RSA | Added `is_probably_prime` (repaired Miller–Rabin analysis, catching Carmichael 561), `eval_robp`, `parallel_prefix_sum` (NC), `verify_graph_isomorphism` (GNI), and `rsa_roundtrip` in Rust & Python + full §10.1–10.6 study notes in [docs/10-advanced-complexity.md](../docs/10-advanced-complexity.md) |

## Verification Status

- **Rust:** `cargo test --all-targets` — 39 unit tests across `src/ch00.rs`–`src/ch10.rs`, zero external crate dependencies, zero compiler warnings.
- **Python:** `python3 -m unittest discover -s tests -v` — mirrored behavioral test suites across `python/ch00.py`–`python/ch10.py`, standard library only.
- **GitHub Pages:** `index.html` serves as an interactive study hub at `https://hevangel.github.io/introduction_to_the_theory_of_computation/` with in-browser rendering of all 11 chapter study guides.
