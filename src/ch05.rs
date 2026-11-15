//! Ch 5: a polynomial-time mapping reduction from graph 3-coloring to SAT.
pub type Cnf = Vec<Vec<i32>>;
fn var(vertex: usize, color: usize) -> i32 { (3*vertex+color+1) as i32 }
/// Each graph edge must connect distinct vertices.
pub fn three_color_to_cnf(n: usize, edges: &[(usize,usize)]) -> Cnf {
    assert!(n < (i32::MAX as usize)/3);
    let mut clauses = Vec::new();
    for v in 0..n {
        clauses.push((0..3).map(|c| var(v,c)).collect());
        for c in 0..3 { for d in c+1..3 { clauses.push(vec![-var(v,c),-var(v,d)]); } }
    }
    for &(u,v) in edges {
        assert!(u<n && v<n);
        for c in 0..3 { clauses.push(vec![-var(u,c),-var(v,c)]); }
    }
    clauses
}
/// Exponential SAT solver for tiny demonstrations only; max 20 variables.
pub fn brute_sat(cnf: &[Vec<i32>], variables: usize) -> bool {
    assert!(variables <= 20);
    for mask in 0..(1usize<<variables) {
        if cnf.iter().all(|clause| clause.iter().any(|&lit| {
            let i=lit.unsigned_abs() as usize;
            assert!(i>=1 && i<=variables);
            let value=((mask >> (i-1)) & 1)!=0;
            if lit>0 { value } else { !value }
        })) { return true; }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn coloring() {
        let triangle=[(0,1),(1,2),(0,2)];
        assert!(brute_sat(&three_color_to_cnf(3,&triangle),9));
        let k4=[(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)];
        assert!(!brute_sat(&three_color_to_cnf(4,&k4),12));
    }
}
