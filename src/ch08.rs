//! Ch 8: reachability by Savitch-style recursive midpoint search.
/// Does a directed path of at most steps edges lead from u to v?
/// This is intentionally exponential-time and uses recursion, not BFS memory.
pub fn reach_within(graph:&[Vec<usize>], u:usize, v:usize, steps:usize) -> bool {
    assert!(u<graph.len() && v<graph.len());
    match steps {
        0 => u==v,
        1 => u==v || graph[u].contains(&v),
        k => {
            let left=k/2;
            (0..graph.len()).any(|mid|
                reach_within(graph,u,mid,left) &&
                reach_within(graph,mid,v,k-left))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn paths() {
        let g=[vec![1],vec![2],vec![]];
        assert!(!reach_within(&g,0,2,1));
        assert!(reach_within(&g,0,2,2));
        assert!(!reach_within(&g,2,0,4));
    }
}
