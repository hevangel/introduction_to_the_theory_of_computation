//! Ch 7: NP certificates, exhaustive search and dynamic programming.
use std::collections::HashSet;
pub fn verifies_vertex_cover(n:usize, edges:&[(usize,usize)], cover:&[usize]) -> bool {
    let selected: HashSet<usize>=cover.iter().copied().collect();
    selected.iter().all(|&v| v<n) &&
        edges.iter().all(|&(u,v)| u<n && v<n && (selected.contains(&u)||selected.contains(&v)))
}
pub fn minimum_vertex_cover(n:usize, edges:&[(usize,usize)]) -> Vec<usize> {
    assert!(n<=20);
    let mut best: Vec<usize>=(0..n).collect();
    for mask in 0..(1usize<<n) {
        if mask.count_ones() as usize >= best.len() { continue; }
        let c:Vec<usize>=(0..n).filter(|&v| (mask>>v)&1==1).collect();
        if verifies_vertex_cover(n,edges,&c) { best=c; }
    }
    best
}
pub fn subset_sum_nonnegative(values:&[usize], target:usize) -> bool {
    let mut reachable=vec![false;target+1];
    reachable[0]=true;
    for &v in values {
        if v>target {continue;}
        for t in (v..=target).rev() { if reachable[t-v] {reachable[t]=true;} }
    }
    reachable[target]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn cover() {
        let edges=[(0,1),(1,2)];
        assert!(verifies_vertex_cover(3,&edges,&[1]));
        assert_eq!(minimum_vertex_cover(3,&edges).len(),1);
        assert!(!verifies_vertex_cover(3,&edges,&[0]));
    }
    #[test] fn subset() { assert!(subset_sum_nonnegative(&[3,5,7],8)); assert!(!subset_sum_nonnegative(&[3,5,7],1)); }
}
