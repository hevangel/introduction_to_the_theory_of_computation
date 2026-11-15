//! Ch 0: sets, relations, graphs, construction and induction.
use std::collections::VecDeque;

/// Construct the set of vertices reachable from start in a finite graph.
pub fn reachable(graph: &[Vec<usize>], start: usize) -> Vec<bool> {
    assert!(start < graph.len());
    let mut seen = vec![false; graph.len()];
    let mut queue = VecDeque::from([start]);
    seen[start] = true;
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            assert!(v < graph.len());
            if !seen[v] {
                seen[v] = true;
                queue.push_back(v);
            }
        }
    }
    seen
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finite_graph() {
        assert_eq!(reachable(&[vec![1], vec![2], vec![], vec![]], 0),
                   vec![true, true, true, false]);
    }
}
