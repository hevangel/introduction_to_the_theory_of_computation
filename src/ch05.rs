//! Ch 5: Reducibility — mapping reductions (3COLOR -> SAT, 3SAT -> CLIQUE),
//! Linear Bounded Automaton (A_LBA decider via cycle detection), and Bounded PCP.

use crate::ch03::{Move, TuringMachine};
use std::collections::HashSet;

pub type Cnf = Vec<Vec<i32>>;

/// Mapping reduction: 3-COLORING <=_m SAT.
/// Variable `3*v + c + 1` means vertex `v` receives color `c ∈ {0, 1, 2}`.
pub fn three_color_to_cnf(n: usize, edges: &[(usize, usize)]) -> Cnf {
    let var = |v: usize, c: usize| -> i32 { (3 * v + c + 1) as i32 };
    let mut clauses = Vec::new();
    for v in 0..n {
        clauses.push(vec![var(v, 0), var(v, 1), var(v, 2)]);
        for a in 0..3 {
            for b in (a + 1)..3 {
                clauses.push(vec![-var(v, a), -var(v, b)]);
            }
        }
    }
    for &(u, v) in edges {
        for c in 0..3 {
            clauses.push(vec![-var(u, c), -var(v, c)]);
        }
    }
    clauses
}

/// Brute-force SAT solver for small formulas (for educational demonstration only).
pub fn brute_sat(cnf: &[Vec<i32>], variables: usize) -> bool {
    for mask in 0..(1u64 << variables) {
        let ok = cnf.iter().all(|clause| {
            clause.iter().any(|&lit| {
                let bit = (mask >> (lit.unsigned_abs() as usize - 1)) & 1 == 1;
                if lit > 0 { bit } else { !bit }
            })
        });
        if ok {
            return true;
        }
    }
    false
}

/// Decider for A_LBA (Sipser Theorem 5.9): a Linear Bounded Automaton can never move
/// outside its n-cell input region, so it has at most |Q| * n * |Γ|^n configurations.
/// By tracking visited configurations, any repeated configuration proves an infinite loop -> reject!
pub fn lba_accepts(tm: &TuringMachine, input: &str) -> bool {
    let mut tape: Vec<char> = if input.is_empty() {
        vec![tm.blank]
    } else {
        input.chars().collect()
    };
    let n = tape.len();
    let mut state = tm.start;
    let mut head = 0usize;
    let mut visited: HashSet<(usize, usize, Vec<char>)> = HashSet::new();

    loop {
        if state == tm.accept {
            return true;
        }
        if state == tm.reject {
            return false;
        }
        if !visited.insert((state, head, tape.clone())) {
            // Repeated configuration => guaranteed infinite loop => reject
            return false;
        }
        let sym = tape[head];
        match tm.transitions.get(&(state, sym)) {
            Some(&(next_state, write_sym, dir)) => {
                tape[head] = write_sym;
                state = next_state;
                match dir {
                    Move::Left => {
                        if head > 0 {
                            head -= 1;
                        }
                    }
                    Move::Right => {
                        if head + 1 < n {
                            head += 1;
                        }
                    }
                    Move::Stay => {}
                }
            }
            None => return false,
        }
    }
}

/// Bounded Post Correspondence Problem (PCP) solver:
/// Given dominoes `(top, bottom)` and a maximum sequence length `max_len`,
/// returns a non-empty sequence of domino indices whose top and bottom concatenations match.
/// NOTE: Unbounded PCP is undecidable (Sipser Theorem 5.15); bounding `max_len` makes this finite.
pub fn bounded_pcp(dominoes: &[(&str, &str)], max_len: usize) -> Option<Vec<usize>> {
    fn dfs(
        dominoes: &[(&str, &str)],
        max_len: usize,
        seq: &mut Vec<usize>,
        top: &str,
        bot: &str,
    ) -> Option<Vec<usize>> {
        if !seq.is_empty() && top == bot {
            return Some(seq.clone());
        }
        if seq.len() == max_len {
            return None;
        }
        // Prefix pruning: the shorter string must be a prefix of the longer string.
        if !(top.starts_with(bot) || bot.starts_with(top)) {
            return None;
        }
        for (i, &(t, b)) in dominoes.iter().enumerate() {
            let next_top = format!("{}{}", top, t);
            let next_bot = format!("{}{}", bot, b);
            seq.push(i);
            if let Some(sol) = dfs(dominoes, max_len, seq, &next_top, &next_bot) {
                return Some(sol);
            }
            seq.pop();
        }
        None
    }
    let mut seq = Vec::new();
    dfs(dominoes, max_len, &mut seq, "", "")
}

/// Mapping reduction 3SAT <=_m CLIQUE (Sipser Theorem 7.32):
/// Each 3-literal clause becomes a triple of vertices; edges connect vertices in DIFFERENT
/// triples unless their literals are logical negations of each other (`x` vs `-x`).
/// Returns `(num_vertices, edges, k)` where `k = clauses.len()`.
pub fn three_sat_to_clique(clauses: &[[i32; 3]]) -> (usize, Vec<(usize, usize)>, usize) {
    let k = clauses.len();
    let num_vertices = 3 * k;
    let mut edges = Vec::new();
    for c1 in 0..k {
        for l1 in 0..3 {
            let u = 3 * c1 + l1;
            for c2 in (c1 + 1)..k {
                for l2 in 0..3 {
                    let v = 3 * c2 + l2;
                    if clauses[c1][l1] != -clauses[c2][l2] {
                        edges.push((u, v));
                    }
                }
            }
        }
    }
    (num_vertices, edges, k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch03::tm_power_of_two_zeros;

    #[test]
    fn coloring() {
        assert!(brute_sat(&three_color_to_cnf(3, &[(0, 1), (1, 2), (0, 2)]), 9));
        assert!(!brute_sat(
            &three_color_to_cnf(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
            12
        ));
    }

    #[test]
    fn pcp_example() {
        // Classic solvable PCP instance: [b/ca], [a/ab], [ca/a], [abc/c]
        let dominoes = [("b", "ca"), ("a", "ab"), ("ca", "a"), ("abc", "c")];
        let sol = bounded_pcp(&dominoes, 5).expect("should find match of length <= 5");
        let top: String = sol.iter().map(|&i| dominoes[i].0).collect();
        let bot: String = sol.iter().map(|&i| dominoes[i].1).collect();
        assert_eq!(top, bot);
    }

    #[test]
    fn lba_cycle_detection() {
        // A tiny LBA that loops forever on '0'
        let mut t = std::collections::BTreeMap::new();
        t.insert((0, '0'), (0, '0', Move::Stay));
        let looping_lba = TuringMachine {
            start: 0,
            accept: 1,
            reject: 2,
            blank: '_',
            transitions: t,
        };
        assert!(!lba_accepts(&looping_lba, "0"));
        // Power-of-two TM stays within input + 1 trailing blank cell
        let m2 = tm_power_of_two_zeros();
        assert!(lba_accepts(&m2, "0000_"));
        assert!(!lba_accepts(&m2, "000_"));
    }

    #[test]
    fn sat_to_clique_reduction() {
        let clauses = [[1, 1, 2], [-1, -2, -2], [-1, 2, 2]];
        let (n, edges, k) = three_sat_to_clique(&clauses);
        assert_eq!(n, 9);
        assert_eq!(k, 3);
        // Assignment x1=false, x2=true satisfies literal 2 in c0 (v2), -1 in c1 (v3), -1 in c2 (v6)
        assert!(edges.contains(&(2, 3)));
        assert!(edges.contains(&(2, 6)));
        assert!(edges.contains(&(3, 6)));
    }
}
