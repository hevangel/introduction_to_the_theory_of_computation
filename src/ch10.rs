//! Ch 10: Advanced complexity — 2-approximation for Vertex Cover, repaired probabilistic
//! primality (Miller–Rabin, 2nd ed. Victor Shoup repair), Read-Once Branching Programs,
//! Interactive Proofs (GNI round), Parallel Prefix Sum (NC), and RSA toy roundtrip.

use std::collections::HashSet;

/// Factor-2 approximation for Vertex Cover (Sipser §10.1):
/// Repeatedly pick an uncovered edge (u, v) and add BOTH endpoints.
/// Never exceeds 2 * OPT because the chosen edges form a maximal matching.
pub fn two_approx_vertex_cover(edges: &[(usize, usize)]) -> Vec<usize> {
    let mut covered = HashSet::new();
    let mut cover = Vec::new();
    for &(u, v) in edges {
        if !covered.contains(&u) && !covered.contains(&v) {
            covered.insert(u);
            covered.insert(v);
            cover.push(u);
            cover.push(v);
        }
    }
    cover
}

/// Fast modular exponentiation a^e mod m in O(log e) multiplications.
pub fn mod_pow(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut res = 1u128;
    let mut cur = (base as u128) % (modulus as u128);
    let m = modulus as u128;
    while exponent > 0 {
        if exponent & 1 == 1 {
            res = (res * cur) % m;
        }
        cur = (cur * cur) % m;
        exponent >>= 1;
    }
    res as u64
}

/// Monte Carlo estimate of π using a deterministic LCG (educational sampling demo).
pub fn pi_estimate(samples: usize, mut seed: u64) -> f64 {
    let mut hits = 0usize;
    for _ in 0..samples {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = ((seed >> 33) as f64) / ((1u64 << 31) as f64);
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y = ((seed >> 33) as f64) / ((1u64 << 31) as f64);
        if x * x + y * y <= 1.0 {
            hits += 1;
        }
    }
    4.0 * hits as f64 / samples as f64
}

/// Miller–Rabin probabilistic primality test (Sipser §10.2, incorporating the
/// repaired analysis from the 2nd edition).
/// For any odd composite n, at least 3/4 of bases a ∈ {2, ..., n-2} are strong witnesses;
/// testing k independent bases gives false-positive probability <= 4^(-k).
/// For all 64-bit integers, the fixed deterministic base set below is exact.
pub fn is_probably_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    // Write n - 1 = d * 2^s with d odd
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'witness: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        for _ in 1..s {
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false; // `a` is a witness to compositeness
    }
    true
}

/// Read-Once Branching Program (Sipser §10.2):
/// A DAG where each internal node queries a single input variable `var_idx` at most once
/// along any root-to-sink path, branching to `low` (if 0) or `high` (if 1).
#[derive(Debug, Clone)]
pub enum RobpNode {
    Branch { var_idx: usize, low: usize, high: usize },
    Sink(bool),
}

pub fn eval_robp(nodes: &[RobpNode], root: usize, inputs: &[bool]) -> bool {
    let mut cur = root;
    loop {
        match nodes[cur] {
            RobpNode::Sink(val) => return val,
            RobpNode::Branch { var_idx, low, high } => {
                cur = if inputs[var_idx] { high } else { low };
            }
        }
    }
}

/// Parallel Prefix Sum (Blelloch / Hillis–Steele style) in O(log n) parallel rounds,
/// illustrating why prefix sums and binary addition lie in NC (Sipser §10.5).
/// Returns `(prefix_sums, parallel_rounds)`.
pub fn parallel_prefix_sum(values: &[i64]) -> (Vec<i64>, usize) {
    let n = values.len();
    let mut cur = values.to_vec();
    let mut rounds = 0usize;
    let mut offset = 1usize;
    while offset < n {
        let prev = cur.clone();
        // All indices i >= offset can execute simultaneously in one parallel step!
        for i in offset..n {
            cur[i] = prev[i] + prev[i - offset];
        }
        rounds += 1;
        offset <<= 1;
    }
    (cur, rounds)
}

/// Interactive Proof illustration for Graph Non-Isomorphism (GNI, Sipser §10.4):
/// Verifier picks b ∈ {0, 1}, permutes graph G_b by random permutation `perm`,
/// sends H to Prover; an honest unbounded Prover identifies b iff G_0 and G_1 are non-isomorphic.
/// Here we verify whether a given permutation `perm` maps `g_orig` identically to `h_edges`.
pub fn verify_graph_isomorphism(
    n: usize,
    g_edges: &[(usize, usize)],
    h_edges: &[(usize, usize)],
    perm: &[usize],
) -> bool {
    if perm.len() != n {
        return false;
    }
    let perm_set: HashSet<usize> = perm.iter().copied().collect();
    if perm_set.len() != n || perm_set.iter().any(|&x| x >= n) {
        return false;
    }
    let norm = |e: &[(usize, usize)]| -> HashSet<(usize, usize)> {
        e.iter()
            .map(|&(u, v)| if u < v { (u, v) } else { (v, u) })
            .collect()
    };
    let mapped: Vec<(usize, usize)> = g_edges.iter().map(|&(u, v)| (perm[u], perm[v])).collect();
    norm(&mapped) == norm(h_edges)
}

/// Toy RSA encrypt/decrypt roundtrip illustrating trapdoor one-way functions (Sipser §10.6).
/// Given primes `p, q` and public exponent `e` coprime to φ(n) = (p-1)(q-1),
/// encrypts `msg < p*q` and decrypts back using private exponent `d = e^(-1) mod φ(n)`.
pub fn rsa_roundtrip(p: u64, q: u64, e: u64, msg: u64) -> Option<(u64, u64)> {
    fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x, y) = egcd(b, a % b);
            (g, y, x - (a / b) * y)
        }
    }
    let n = p.checked_mul(q)?;
    let phi = (p - 1) * (q - 1);
    let (g, x, _) = egcd(e as i128, phi as i128);
    if g != 1 || msg >= n {
        return None;
    }
    let d = ((x % phi as i128 + phi as i128) % phi as i128) as u64;
    let cipher = mod_pow(msg, e, n);
    let plain = mod_pow(cipher, d, n);
    Some((cipher, plain))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch07::{minimum_vertex_cover, verifies_vertex_cover};

    #[test]
    fn approximation() {
        let edges = [(0, 1), (1, 2), (2, 3)];
        let cover = two_approx_vertex_cover(&edges);
        assert!(verifies_vertex_cover(4, &edges, &cover));
        assert!(cover.len() <= 2 * minimum_vertex_cover(4, &edges).len());
    }

    #[test]
    fn arithmetic() {
        assert_eq!(mod_pow(2, 10, 17), 4);
        let pi = pi_estimate(5000, 42);
        assert!((2.8..3.5).contains(&pi));
    }

    #[test]
    fn primality_robp_nc_gni_rsa() {
        assert!(is_probably_prime(2));
        assert!(is_probably_prime(1_000_000_007));
        assert!(!is_probably_prime(561)); // Carmichael number — fools Fermat test, caught by Miller–Rabin!
        assert!(!is_probably_prime(1729));

        // 2-bit parity ROBP
        let nodes = vec![
            RobpNode::Branch { var_idx: 0, low: 1, high: 2 },
            RobpNode::Branch { var_idx: 1, low: 3, high: 4 },
            RobpNode::Branch { var_idx: 1, low: 4, high: 3 },
            RobpNode::Sink(false),
            RobpNode::Sink(true),
        ];
        assert!(!eval_robp(&nodes, 0, &[false, false]));
        assert!(eval_robp(&nodes, 0, &[true, false]));
        assert!(!eval_robp(&nodes, 0, &[true, true]));

        // Parallel prefix sum in O(log n) rounds
        let (sums, rounds) = parallel_prefix_sum(&[1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(sums, vec![1, 3, 6, 10, 15, 21, 28, 36]);
        assert_eq!(rounds, 3);

        // Graph isomorphism check
        assert!(verify_graph_isomorphism(3, &[(0, 1), (1, 2)], &[(2, 0), (0, 1)], &[2, 0, 1]));

        // RSA roundtrip
        let (cipher, plain) = rsa_roundtrip(61, 53, 17, 65).unwrap();
        assert_eq!(cipher, 2790);
        assert_eq!(plain, 65);
    }
}
