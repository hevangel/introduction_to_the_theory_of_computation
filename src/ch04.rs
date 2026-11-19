//! Ch 4: Decidable languages — A_DFA, E_DFA, EQ_DFA, E_CFG, and A_CFG.
//! (Undecidable languages like A_TM and HALT_TM cannot be decided by any algorithm.)

use crate::ch01::Dfa;
use crate::ch02::{Cfg, Symbol};
use std::collections::{BTreeSet, VecDeque};

/// Decide equivalence of two binary-alphabet complete DFAs starting at state 0
/// via product-automaton reachability of disagreeing states.
pub fn dfa_equivalent(a: &[[usize; 2]], af: &[bool], b: &[[usize; 2]], bf: &[bool]) -> bool {
    let mut seen = BTreeSet::from([(0usize, 0usize)]);
    let mut q = VecDeque::from([(0usize, 0usize)]);
    while let Some((x, y)) = q.pop_front() {
        if af[x] != bf[y] {
            return false;
        }
        for sym in 0..2 {
            let next = (a[x][sym], b[y][sym]);
            if seen.insert(next) {
                q.push_back(next);
            }
        }
    }
    true
}

/// Decider for A_DFA = { <B, w> | B is a DFA that accepts input string w } (Theorem 4.1).
pub fn a_dfa(dfa: &Dfa, word: &str) -> bool {
    dfa.accepts(word)
}

/// Decider for E_DFA = { <A> | A is a DFA and L(A) = ∅ } (Theorem 4.4).
/// Returns true iff no accept state is reachable from the start state.
pub fn e_dfa(dfa: &Dfa) -> bool {
    let reach = dfa.reachable_states();
    reach.iter().all(|s| !dfa.accept.contains(s))
}

/// Decider for EQ_DFA = { <A, B> | A and B are DFAs and L(A) = L(B) } (Theorem 4.5).
/// Constructs the symmetric difference C = (L(A) ∩ L(B)^c) ∪ (L(A)^c ∩ L(B)) and tests E_DFA(C).
pub fn eq_dfa(a: &Dfa, b: &Dfa) -> bool {
    let a_minus_b = a.product(&b.complement(), true);
    let b_minus_a = a.complement().product(b, true);
    let sym_diff = a_minus_b.product(&b_minus_a, false);
    e_dfa(&sym_diff)
}

/// Decider for E_CFG = { <G> | G is a CFG and L(G) = ∅ } (Theorem 4.8).
/// Marks generating variables bottom-up; returns true iff the start variable is NOT marked.
pub fn e_cfg(cfg: &Cfg) -> bool {
    let mut generating: BTreeSet<String> = BTreeSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for p in &cfg.productions {
            if generating.contains(&p.lhs) {
                continue;
            }
            let rhs_ok = p.rhs.iter().all(|sym| match sym {
                Symbol::Term(_) => true,
                Symbol::Var(v) => generating.contains(v),
            });
            if rhs_ok {
                generating.insert(p.lhs.clone());
                changed = true;
            }
        }
    }
    !generating.contains(&cfg.start)
}

/// Decider for A_CFG = { <G, w> | G is a CFG that generates w } (Theorem 4.7).
/// Converts G to Chomsky Normal Form and runs CYK.
pub fn a_cfg(cfg: &Cfg, tokens: &[&str]) -> bool {
    cfg.to_cnf().accepts(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch01::regex_to_nfa;
    use crate::ch02::Production;

    #[test]
    fn equivalence() {
        let parity = [[0, 1], [1, 0]];
        assert!(dfa_equivalent(&parity, &[true, false], &parity, &[true, false]));
        assert!(!dfa_equivalent(&parity, &[true, false], &[[0, 0]], &[true]));
    }

    #[test]
    fn dfa_deciders() {
        let d1 = regex_to_nfa("(a|b)*ab").unwrap().to_dfa();
        let d2 = d1.minimize();
        assert!(a_dfa(&d1, "bab"));
        assert!(!e_dfa(&d1));
        assert!(eq_dfa(&d1, &d2));
        let empty = d1.product(&d1.complement(), true);
        assert!(e_dfa(&empty));
    }

    #[test]
    fn cfg_deciders() {
        // Unproductive grammar: S -> a S (never terminates!)
        let empty_cfg = Cfg {
            start: "S".into(),
            productions: vec![Production {
                lhs: "S".into(),
                rhs: vec![Symbol::Term("a".into()), Symbol::Var("S".into())],
            }],
        };
        assert!(e_cfg(&empty_cfg));

        // Productive grammar: S -> a S b | ε
        let anbn_cfg = Cfg {
            start: "S".into(),
            productions: vec![
                Production {
                    lhs: "S".into(),
                    rhs: vec![
                        Symbol::Term("a".into()),
                        Symbol::Var("S".into()),
                        Symbol::Term("b".into()),
                    ],
                },
                Production { lhs: "S".into(), rhs: vec![] },
            ],
        };
        assert!(!e_cfg(&anbn_cfg));
        assert!(a_cfg(&anbn_cfg, &["a", "a", "b", "b"]));
        assert!(!a_cfg(&anbn_cfg, &["a", "b", "b"]));
    }
}
