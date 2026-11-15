//! Ch 4: a genuine finite-state decision procedure, unlike the halting problem.
use std::collections::{HashSet, VecDeque};
/// Binary DFAs: transition table per state, accepting flags, start state 0.
/// BFS in the product finds a distinguishing word if languages differ.
pub fn dfa_equivalent(a: &[[usize;2]], af: &[bool], b: &[[usize;2]], bf: &[bool]) -> bool {
    assert!(!a.is_empty() && !b.is_empty() && a.len()==af.len() && b.len()==bf.len());
    let mut seen = HashSet::from([(0usize,0usize)]);
    let mut queue = VecDeque::from([(0usize,0usize)]);
    while let Some((u,v)) = queue.pop_front() {
        if af[u] != bf[v] { return false; }
        for digit in 0..2 {
            let next=(a[u][digit],b[v][digit]);
            assert!(next.0<a.len() && next.1<b.len());
            if seen.insert(next) { queue.push_back(next); }
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn equivalence() {
        let parity=[[0,1],[1,0]];
        let all=[[0,0]];
        assert!(dfa_equivalent(&parity,&[true,false],&parity,&[true,false]));
        assert!(!dfa_equivalent(&parity,&[true,false],&all,&[true]));
    }
}
