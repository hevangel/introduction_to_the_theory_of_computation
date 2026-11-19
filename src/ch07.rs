//! Ch 7: Time complexity — P (2-SAT via Tarjan SCC), NP verifiers (VERTEX-COVER, CLIQUE, HAMPATH),
//! pseudo-polynomial DP (SUBSET-SUM), and polynomial-time reductions
//! (CLIQUE <=_P VERTEX-COVER, 3SAT <=_P SUBSET-SUM).

use std::collections::HashSet;

/// Polynomial-time verifier for VERTEX-COVER.
pub fn verifies_vertex_cover(n: usize, edges: &[(usize, usize)], cover: &[usize]) -> bool {
    let set: HashSet<usize> = cover.iter().copied().collect();
    set.iter().all(|&v| v < n) && edges.iter().all(|&(u, v)| set.contains(&u) || set.contains(&v))
}

/// Exact minimum vertex cover by brute force (exponential time, small graphs only).
pub fn minimum_vertex_cover(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut best: Vec<usize> = (0..n).collect();
    for mask in 0..(1u64 << n) {
        let cover: Vec<usize> = (0..n).filter(|&v| (mask >> v) & 1 == 1).collect();
        if cover.len() < best.len() && verifies_vertex_cover(n, edges, &cover) {
            best = cover;
        }
    }
    best
}

/// Dynamic programming for SUBSET-SUM on nonnegative integers.
/// Runs in O(n * target) time — PSEUDO-POLYNOMIAL (exponential in log(target), the bit-length!).
pub fn subset_sum_nonnegative(values: &[usize], target: usize) -> bool {
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &x in values {
        for sum in (x..=target).rev() {
            dp[sum] |= dp[sum - x];
        }
    }
    dp[target]
}

/// Polynomial-time verifier for CLIQUE: checks that `clique` has `k` distinct vertices
/// and every pair is connected by an edge in `edges`.
pub fn verifies_clique(n: usize, edges: &[(usize, usize)], clique: &[usize], k: usize) -> bool {
    if clique.len() != k {
        return false;
    }
    let set: HashSet<usize> = clique.iter().copied().collect();
    if set.len() != k || set.iter().any(|&v| v >= n) {
        return false;
    }
    let edge_set: HashSet<(usize, usize)> = edges
        .iter()
        .flat_map(|&(u, v)| [(u, v), (v, u)])
        .collect();
    for i in 0..k {
        for j in (i + 1)..k {
            if !edge_set.contains(&(clique[i], clique[j])) {
                return false;
            }
        }
    }
    true
}

/// Polynomial-time verifier for directed HAMPATH from `s` to `t`.
pub fn verifies_hampath(
    n: usize,
    edges: &[(usize, usize)],
    s: usize,
    t: usize,
    path: &[usize],
) -> bool {
    if path.len() != n || path.first() != Some(&s) || path.last() != Some(&t) {
        return false;
    }
    let unique: HashSet<usize> = path.iter().copied().collect();
    if unique.len() != n || unique.iter().any(|&v| v >= n) {
        return false;
    }
    let edge_set: HashSet<(usize, usize)> = edges.iter().copied().collect();
    path.windows(2).all(|w| edge_set.contains(&(w[0], w[1])))
}

/// Polynomial-time 2-SAT solver (2SAT ∈ P!) using implication graph + Tarjan's SCC algorithm.
/// Clauses are pairs `(a, b)` with 1-indexed signed literals (`+v` = x_v, `-v` = ¬x_v).
/// Returns `Some(assignment)` (0-indexed bools for variables 1..=num_vars) if satisfiable.
pub fn two_sat_solve(num_vars: usize, clauses: &[(i32, i32)]) -> Option<Vec<bool>> {
    let lit_to_node = |lit: i32| -> usize {
        let v = (lit.unsigned_abs() as usize) - 1;
        if lit > 0 { 2 * v } else { 2 * v + 1 }
    };
    let neg_node = |u: usize| -> usize { u ^ 1 };

    let total = 2 * num_vars;
    let mut adj = vec![Vec::<usize>::new(); total];
    for &(a, b) in clauses {
        let na = lit_to_node(a);
        let nb = lit_to_node(b);
        // (a ∨ b) ≡ (¬a → b) ∧ (¬b → a)
        adj[neg_node(na)].push(nb);
        adj[neg_node(nb)].push(na);
    }

    // Tarjan's Strongly Connected Components
    let mut index = 0usize;
    let mut indices = vec![usize::MAX; total];
    let mut lowlink = vec![0usize; total];
    let mut on_stack = vec![false; total];
    let mut stack = Vec::new();
    let mut comp = vec![0usize; total];
    let mut comp_count = 0usize;

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        u: usize,
        adj: &[Vec<usize>],
        index: &mut usize,
        indices: &mut [usize],
        lowlink: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<usize>,
        comp: &mut [usize],
        comp_count: &mut usize,
    ) {
        indices[u] = *index;
        lowlink[u] = *index;
        *index += 1;
        stack.push(u);
        on_stack[u] = true;

        for &v in &adj[u] {
            if indices[v] == usize::MAX {
                dfs(v, adj, index, indices, lowlink, on_stack, stack, comp, comp_count);
                lowlink[u] = lowlink[u].min(lowlink[v]);
            } else if on_stack[v] {
                lowlink[u] = lowlink[u].min(indices[v]);
            }
        }

        if lowlink[u] == indices[u] {
            while let Some(w) = stack.pop() {
                on_stack[w] = false;
                comp[w] = *comp_count;
                if w == u {
                    break;
                }
            }
            *comp_count += 1;
        }
    }

    for u in 0..total {
        if indices[u] == usize::MAX {
            dfs(
                u,
                &adj,
                &mut index,
                &mut indices,
                &mut lowlink,
                &mut on_stack,
                &mut stack,
                &mut comp,
                &mut comp_count,
            );
        }
    }

    let mut assignment = vec![false; num_vars];
    for v in 0..num_vars {
        if comp[2 * v] == comp[2 * v + 1] {
            return None; // x_v and ¬x_v in same SCC => unsatisfiable
        }
        // Tarjan assigns reverse topological order: smaller component ID = later in topological order
        assignment[v] = comp[2 * v] < comp[2 * v + 1];
    }
    Some(assignment)
}

/// Polynomial-time reduction CLIQUE <=_P VERTEX-COVER via complement graph:
/// G has a k-clique iff the complement graph Ḡ has a vertex cover of size n - k.
pub fn clique_to_vertex_cover(
    n: usize,
    edges: &[(usize, usize)],
    k: usize,
) -> (usize, Vec<(usize, usize)>, usize) {
    let edge_set: HashSet<(usize, usize)> = edges
        .iter()
        .map(|&(u, v)| if u < v { (u, v) } else { (v, u) })
        .collect();
    let mut comp_edges = Vec::new();
    for u in 0..n {
        for v in (u + 1)..n {
            if !edge_set.contains(&(u, v)) {
                comp_edges.push((u, v));
            }
        }
    }
    (n, comp_edges, n.saturating_sub(k))
}

/// Sipser's table-based polynomial-time reduction 3SAT <=_P SUBSET-SUM (Theorem 7.56).
/// Uses base-10 digits (zero carry because each column sums to at most 6 < 10) for small instances.
/// Returns `(numbers, target)`.
pub fn three_sat_to_subset_sum(num_vars: usize, clauses: &[[i32; 3]]) -> (Vec<u128>, u128) {
    let l = num_vars;
    let k = clauses.len();
    let total_cols = l + k;
    let col_pow = |col: usize| -> u128 { 10u128.pow((total_cols - 1 - col) as u32) };

    let mut numbers = Vec::with_capacity(2 * l + 2 * k);
    for i in 0..l {
        let var_id = (i + 1) as i32;
        let mut y_i = col_pow(i);
        let mut z_i = col_pow(i);
        for (j, clause) in clauses.iter().enumerate() {
            if clause.contains(&var_id) {
                y_i += col_pow(l + j);
            }
            if clause.contains(&(-var_id)) {
                z_i += col_pow(l + j);
            }
        }
        numbers.push(y_i);
        numbers.push(z_i);
    }
    // Slack numbers g_j = h_j = 10^(k-1-j) so any clause with 1, 2, or 3 true literals can reach 3
    for j in 0..k {
        let slack = col_pow(l + j);
        numbers.push(slack);
        numbers.push(slack);
    }

    let mut target = 0u128;
    for i in 0..l {
        target += col_pow(i); // digit 1 in every variable column
    }
    for j in 0..k {
        target += 3 * col_pow(l + j); // digit 3 in every clause column
    }
    (numbers, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover() {
        let edges = [(0, 1), (1, 2)];
        assert!(verifies_vertex_cover(3, &edges, &[1]));
        assert_eq!(minimum_vertex_cover(3, &edges).len(), 1);
    }

    #[test]
    fn subset() {
        assert!(subset_sum_nonnegative(&[3, 5, 7], 8));
        assert!(!subset_sum_nonnegative(&[3, 5, 7], 1));
    }

    #[test]
    fn two_sat_and_verifiers() {
        // (x1 ∨ x2) ∧ (¬x1 ∨ x2) ∧ (x1 ∨ ¬x2) => x1=true, x2=true
        let sol = two_sat_solve(2, &[(1, 2), (-1, 2), (1, -2)]).unwrap();
        assert_eq!(sol, vec![true, true]);
        // Unsatisfiable: (x1 ∨ x1) ∧ (¬x1 ∨ ¬x1)
        assert!(two_sat_solve(1, &[(1, 1), (-1, -1)]).is_none());

        assert!(verifies_clique(4, &[(0, 1), (1, 2), (0, 2), (2, 3)], &[0, 1, 2], 3));
        assert!(verifies_hampath(3, &[(0, 1), (1, 2)], 0, 2, &[0, 1, 2]));
    }

    #[test]
    fn reductions_p() {
        // 3-clique in 4-vertex graph => complement has 1-vertex cover
        let edges = [(0, 1), (1, 2), (0, 2), (2, 3)];
        let (n, comp_edges, vc_k) = clique_to_vertex_cover(4, &edges, 3);
        assert_eq!(vc_k, 1);
        assert_eq!(minimum_vertex_cover(n, &comp_edges).len(), 1);

        // 3SAT -> SUBSET-SUM: satisfiable formula (x1 ∨ x1 ∨ x2) ∧ (¬x1 ∨ ¬x2 ∨ ¬x2)
        let (nums, target) = three_sat_to_subset_sum(2, &[[1, 1, 2], [-1, -2, -2]]);
        let nums_usize: Vec<usize> = nums.iter().map(|&x| x as usize).collect();
        assert!(subset_sum_nonnegative(&nums_usize, target as usize));
    }
}
