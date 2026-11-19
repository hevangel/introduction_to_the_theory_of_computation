//! Ch 8: Space complexity — Savitch's theorem (CAN-YIELD), TQBF (PSPACE-complete),
//! and Immerman–Szelepcsényi inductive counting (NL = coNL).

use std::collections::HashMap;

/// Savitch's divide-and-conquer reachability `CAN-YIELD(u, v, steps)` (Sipser Theorem 8.5):
/// Decides whether `v` is reachable from `u` in at most `steps` edges using O(log(steps))
/// recursion depth — illustrating NSPACE(f(n)) ⊆ SPACE(f(n)^2).
pub fn reach_within(graph: &[Vec<usize>], u: usize, v: usize, steps: usize) -> bool {
    if u == v {
        return true;
    }
    if steps == 0 {
        return false;
    }
    if steps == 1 {
        return graph[u].contains(&v);
    }
    let half = steps / 2;
    (0..graph.len()).any(|mid| {
        reach_within(graph, u, mid, half) && reach_within(graph, mid, v, steps - half)
    })
}

/// Quantified Boolean Formula (QBF) AST for TQBF (PSPACE-complete, Sipser Theorem 8.9).
#[derive(Debug, Clone)]
pub enum Qbf {
    Var(String),
    Not(Box<Qbf>),
    And(Box<Qbf>, Box<Qbf>),
    Or(Box<Qbf>, Box<Qbf>),
    ForAll(String, Box<Qbf>),
    Exists(String, Box<Qbf>),
}

/// Decide TQBF in polynomial space (O(depth) stack frames, reusing space across branches).
pub fn eval_tqbf(formula: &Qbf) -> bool {
    fn eval(f: &Qbf, env: &mut HashMap<String, bool>) -> bool {
        match f {
            Qbf::Var(name) => env[name],
            Qbf::Not(inner) => !eval(inner, env),
            Qbf::And(l, r) => eval(l, env) && eval(r, env),
            Qbf::Or(l, r) => eval(l, env) || eval(r, env),
            Qbf::ForAll(v, body) => {
                env.insert(v.clone(), false);
                if !eval(body, env) {
                    return false;
                }
                env.insert(v.clone(), true);
                eval(body, env)
            }
            Qbf::Exists(v, body) => {
                env.insert(v.clone(), false);
                if eval(body, env) {
                    return true;
                }
                env.insert(v.clone(), true);
                eval(body, env)
            }
        }
    }
    let mut env = HashMap::new();
    eval(formula, &mut env)
}

/// Immerman–Szelepcsényi inductive counting algorithm (Sipser Theorem 8.27, NL = coNL):
/// Computes `c_i` = number of vertices reachable from `start` in <= `i` steps using ONLY
/// counters and vertex indices (no visited-set array!), then verifies `target` is NOT
/// among the `c_{n-1}` reachable vertices. Returns `true` iff `target` is UNREACHABLE.
pub fn immerman_szelepcsenyi_non_reach(graph: &[Vec<usize>], start: usize, target: usize) -> bool {
    let n = graph.len();
    if n == 0 || start >= n || target >= n {
        return true;
    }

    // Helper: check if `u` is reachable from `start` in <= `steps` steps
    // (In an actual NL machine this is guessed nondeterministically in O(log n) space;
    // here we use Savitch's reach_within so the deterministic code remains faithful to the
    // inductive counting structure without storing a global visited table.)
    let mut c_prev = 1usize; // c_0 = 1 (only `start` is reachable in 0 steps)

    for i in 1..n {
        let mut c_cur = 0usize;
        for v in 0..n {
            // Check whether v is reachable in <= i steps by verifying against all c_prev
            // vertices reachable in <= i-1 steps.
            let mut seen_prev = 0usize;
            let mut v_reachable = false;
            for u in 0..n {
                if reach_within(graph, start, u, i - 1) {
                    seen_prev += 1;
                    if u == v || graph[u].contains(&v) {
                        v_reachable = true;
                    }
                }
            }
            assert_eq!(seen_prev, c_prev, "Inductive count invariant");
            if v_reachable {
                c_cur += 1;
            }
        }
        c_prev = c_cur;
    }

    // Final verification: enumerate all c_{n-1} reachable vertices and confirm none equals `target`.
    let mut count = 0usize;
    for u in 0..n {
        if reach_within(graph, start, u, n - 1) {
            if u == target {
                return false;
            }
            count += 1;
        }
    }
    count == c_prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        let g = vec![vec![1], vec![2], vec![]];
        assert!(!reach_within(&g, 0, 2, 1));
        assert!(reach_within(&g, 0, 2, 2));
    }

    #[test]
    fn tqbf_solver() {
        // ∀x ∃y (x ↔ y) ≡ ∀x ∃y ((x ∧ y) ∨ (¬x ∧ ¬y)) — TRUE
        let f_true = Qbf::ForAll(
            "x".into(),
            Box::new(Qbf::Exists(
                "y".into(),
                Box::new(Qbf::Or(
                    Box::new(Qbf::And(Box::new(Qbf::Var("x".into())), Box::new(Qbf::Var("y".into())))),
                    Box::new(Qbf::And(
                        Box::new(Qbf::Not(Box::new(Qbf::Var("x".into())))),
                        Box::new(Qbf::Not(Box::new(Qbf::Var("y".into())))),
                    )),
                )),
            )),
        );
        assert!(eval_tqbf(&f_true));

        // ∃y ∀x (x ↔ y) — FALSE
        let f_false = Qbf::Exists(
            "y".into(),
            Box::new(Qbf::ForAll(
                "x".into(),
                Box::new(Qbf::Or(
                    Box::new(Qbf::And(Box::new(Qbf::Var("x".into())), Box::new(Qbf::Var("y".into())))),
                    Box::new(Qbf::And(
                        Box::new(Qbf::Not(Box::new(Qbf::Var("x".into())))),
                        Box::new(Qbf::Not(Box::new(Qbf::Var("y".into())))),
                    )),
                )),
            )),
        );
        assert!(!eval_tqbf(&f_false));
    }

    #[test]
    fn conll_inductive_counting() {
        let g = vec![vec![1], vec![2], vec![], vec![2]];
        assert!(!immerman_szelepcsenyi_non_reach(&g, 0, 2)); // 2 IS reachable from 0
        assert!(immerman_szelepcsenyi_non_reach(&g, 0, 3));  // 3 is NOT reachable from 0
    }
}
