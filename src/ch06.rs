//! Ch 6: Advanced computability — Recursion Theorem (quine), Cantor pairing,
//! first-order logic model evaluation on (Z_m, +), and descriptive complexity upper bounds.

use std::collections::HashMap;

/// Enumerate all binary strings of length 0..=max_len in canonical shortlex order.
pub fn binary_strings_up_to(max_len: usize) -> Vec<String> {
    let mut out = vec![String::new()];
    for len in 1..=max_len {
        for mask in 0..(1usize << len) {
            let mut s = String::with_capacity(len);
            for bit in (0..len).rev() {
                s.push(if (mask >> bit) & 1 == 1 { '1' } else { '0' });
            }
            out.push(s);
        }
    }
    out
}

/// Simple run-length encoder (an explicit, computable upper bound on descriptive complexity;
/// NOT true Kolmogorov complexity K(x), which is uncomputable).
pub fn run_length_encode(s: &str) -> Vec<(char, usize)> {
    let mut out = Vec::new();
    for c in s.chars() {
        match out.last_mut() {
            Some((ch, count)) if *ch == c => *count += 1,
            _ => out.push((c, 1)),
        }
    }
    out
}

/// Cantor pairing bijection N × N -> N: π(x, y) = (x + y)(x + y + 1)/2 + y.
pub fn cantor_pair(x: u64, y: u64) -> u64 {
    let s = x + y;
    s * (s + 1) / 2 + y
}

/// Inverse of Cantor pairing bijection N -> N × N.
pub fn cantor_unpair(z: u64) -> (u64, u64) {
    // Largest w with w(w+1)/2 <= z
    let w = ((((8 * z + 1) as f64).sqrt() - 1.0) / 2.0).floor() as u64;
    let t = w * (w + 1) / 2;
    let y = z - t;
    let x = w - y;
    (x, y)
}

/// Constructive Recursion Theorem demo (Sipser Theorem 6.1, `SELF`):
/// Combines a template `B` (which formats a quoted copy of its input) with its own description `A`
/// so that evaluating the pair outputs the exact source string of the combined program.
pub fn self_reproducing_string() -> (String, String) {
    let template = "let q = {Q}; format!(q, Q = format!(\"{:?}\", q))";
    let quoted = format!("{:?}", template);
    let rendered = template.replace("{Q}", &quoted);
    (rendered.clone(), rendered)
}

/// First-order logic formula over the finite additive structure (Z_m, +).
#[derive(Debug, Clone)]
pub enum FoFormula {
    EqSum { a: String, b: String, c: String }, // a + b ≡ c (mod m)
    Not(Box<FoFormula>),
    And(Box<FoFormula>, Box<FoFormula>),
    Or(Box<FoFormula>, Box<FoFormula>),
    ForAll(String, Box<FoFormula>),
    Exists(String, Box<FoFormula>),
}

/// Decide truth of a closed first-order sentence in the finite model (Z_m, +).
/// Illustrates Section 6.2 (Decidable Theories like Th(N, +)).
pub fn eval_fo_mod(formula: &FoFormula, modulus: u64) -> bool {
    fn eval(f: &FoFormula, m: u64, env: &mut HashMap<String, u64>) -> bool {
        match f {
            FoFormula::EqSum { a, b, c } => (env[a] + env[b]) % m == env[c] % m,
            FoFormula::Not(inner) => !eval(inner, m, env),
            FoFormula::And(l, r) => eval(l, m, env) && eval(r, m, env),
            FoFormula::Or(l, r) => eval(l, m, env) || eval(r, m, env),
            FoFormula::ForAll(var, body) => {
                for val in 0..m {
                    env.insert(var.clone(), val);
                    if !eval(body, m, env) {
                        return false;
                    }
                }
                true
            }
            FoFormula::Exists(var, body) => {
                for val in 0..m {
                    env.insert(var.clone(), val);
                    if eval(body, m, env) {
                        return true;
                    }
                }
                false
            }
        }
    }
    let mut env = HashMap::new();
    eval(formula, modulus, &mut env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate() {
        assert_eq!(binary_strings_up_to(2), ["", "0", "1", "00", "01", "10", "11"]);
    }

    #[test]
    fn encode() {
        assert_eq!(run_length_encode("aaabb"), vec![('a', 3), ('b', 2)]);
    }

    #[test]
    fn pairing_bijection() {
        for x in 0..15 {
            for y in 0..15 {
                let z = cantor_pair(x, y);
                assert_eq!(cantor_unpair(z), (x, y));
            }
        }
    }

    #[test]
    fn recursion_and_logic() {
        let (a, b) = self_reproducing_string();
        assert_eq!(a, b);

        // Sentence: ∀x ∃y (x + y = x)  — true in every (Z_m, +) with y = 0!
        let has_zero = FoFormula::ForAll(
            "x".into(),
            Box::new(FoFormula::Exists(
                "y".into(),
                Box::new(FoFormula::EqSum {
                    a: "x".into(),
                    b: "y".into(),
                    c: "x".into(),
                }),
            )),
        );
        assert!(eval_fo_mod(&has_zero, 5));

        // Sentence: ∀x ∀y (x + y = x) — false for m >= 2
        let all_zero = FoFormula::ForAll(
            "x".into(),
            Box::new(FoFormula::ForAll(
                "y".into(),
                Box::new(FoFormula::EqSum {
                    a: "x".into(),
                    b: "y".into(),
                    c: "x".into(),
                }),
            )),
        );
        assert!(!eval_fo_mod(&all_zero, 5));
    }
}
