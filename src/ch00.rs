//! Ch 0: Mathematical foundations — sets, relations, graphs, and diagonalization.

use std::collections::{BTreeSet, VecDeque};

/// Breadth-first reachability in a directed graph represented as adjacency lists.
pub fn reachable(graph: &[Vec<usize>], start: usize) -> Vec<bool> {
    let mut seen = vec![false; graph.len()];
    if start >= graph.len() {
        return seen;
    }
    let mut queue = VecDeque::from([start]);
    seen[start] = true;
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if v < graph.len() && !seen[v] {
                seen[v] = true;
                queue.push_back(v);
            }
        }
    }
    seen
}

/// Enumerate the power set P(A) of a finite slice (size 2^|A|).
pub fn power_set<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let n = items.len();
    let total = 1usize << n;
    let mut result = Vec::with_capacity(total);
    for mask in 0..total {
        let mut subset = Vec::new();
        for (i, item) in items.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                subset.push(item.clone());
            }
        }
        result.push(subset);
    }
    result
}

/// Check whether a binary relation on {0, ..., n-1} is an equivalence relation
/// (reflexive, symmetric, and transitive).
pub fn is_equivalence_relation(n: usize, pairs: &[(usize, usize)]) -> bool {
    let mut mat = vec![vec![false; n]; n];
    for &(a, b) in pairs {
        if a >= n || b >= n {
            return false;
        }
        mat[a][b] = true;
    }
    // Reflexive: (i, i) in R for all i
    for i in 0..n {
        if !mat[i][i] {
            return false;
        }
    }
    // Symmetric: (i, j) in R => (j, i) in R
    for i in 0..n {
        for j in 0..n {
            if mat[i][j] && !mat[j][i] {
                return false;
            }
        }
    }
    // Transitive: (i, j) in R and (j, k) in R => (i, k) in R
    for i in 0..n {
        for j in 0..n {
            if mat[i][j] {
                for k in 0..n {
                    if mat[j][k] && !mat[i][k] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

/// Partition {0, ..., n-1} into equivalence classes if `pairs` is an equivalence relation.
pub fn equivalence_classes(n: usize, pairs: &[(usize, usize)]) -> Option<Vec<Vec<usize>>> {
    if !is_equivalence_relation(n, pairs) {
        return None;
    }
    let mut adj = vec![BTreeSet::new(); n];
    for &(a, b) in pairs {
        adj[a].insert(b);
    }
    let mut visited = vec![false; n];
    let mut classes = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let class: Vec<usize> = adj[i].iter().copied().collect();
            for &x in &class {
                visited[x] = true;
            }
            classes.push(class);
        }
    }
    Some(classes)
}

/// Finite illustration of Cantor's diagonalization:
/// Given n binary strings each of length >= n, return an n-bit string differing
/// from the i-th string at index i, so it cannot equal any string in the list.
pub fn cantor_diagonal_bitstring(rows: &[&str]) -> String {
    let mut diag = String::with_capacity(rows.len());
    for (i, row) in rows.iter().enumerate() {
        let ch = row.as_bytes().get(i).copied().unwrap_or(b'0');
        diag.push(if ch == b'0' { '1' } else { '0' });
    }
    diag
}

/// Detect whether a directed graph contains a cycle using 3-color DFS
/// (0 = unvisited, 1 = on recursion stack, 2 = finished).
pub fn has_cycle(graph: &[Vec<usize>]) -> bool {
    fn dfs(u: usize, graph: &[Vec<usize>], color: &mut [u8]) -> bool {
        color[u] = 1;
        for &v in &graph[u] {
            if v < graph.len() {
                if color[v] == 1 || (color[v] == 0 && dfs(v, graph, color)) {
                    return true;
                }
            }
        }
        color[u] = 2;
        false
    }
    let mut color = vec![0u8; graph.len()];
    for u in 0..graph.len() {
        if color[u] == 0 && dfs(u, graph, &mut color) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_graph() {
        let g = vec![vec![1], vec![2], vec![], vec![]];
        assert_eq!(reachable(&g, 0), vec![true, true, true, false]);
        assert!(!has_cycle(&g));
        let cyclic = vec![vec![1], vec![2], vec![0]];
        assert!(has_cycle(&cyclic));
    }

    #[test]
    fn sets_and_relations() {
        assert_eq!(power_set(&[1, 2, 3]).len(), 8);
        // Congruence mod 2 on {0, 1, 2, 3}
        let mod2 = vec![
            (0, 0), (0, 2), (2, 0), (2, 2),
            (1, 1), (1, 3), (3, 1), (3, 3),
        ];
        assert!(is_equivalence_relation(4, &mod2));
        assert_eq!(equivalence_classes(4, &mod2), Some(vec![vec![0, 2], vec![1, 3]]));
        // Missing symmetry
        assert!(!is_equivalence_relation(2, &[(0, 0), (1, 1), (0, 1)]));
    }

    #[test]
    fn diagonalization() {
        let rows = ["010", "111", "001"];
        let d = cantor_diagonal_bitstring(&rows);
        assert_eq!(d, "100");
        assert!(!rows.contains(&d.as_str()));
    }
}
