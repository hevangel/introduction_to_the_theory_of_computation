//! Ch 10: approximation, probabilistic sampling and modular arithmetic.
use std::collections::HashSet;
/// Endpoints of a maximal matching form a 2-approximate vertex cover.
pub fn two_approx_vertex_cover(edges:&[(usize,usize)]) -> Vec<usize> {
    let mut chosen=HashSet::new();
    for &(u,v) in edges {
        if !chosen.contains(&u) && !chosen.contains(&v) {
            chosen.insert(u); chosen.insert(v);
        }
    }
    let mut result:Vec<usize>=chosen.into_iter().collect();
    result.sort_unstable();
    result
}
/// Exact modular exponentiation using u128 intermediates.
pub fn mod_pow(base:u64, mut exponent:u64, modulus:u64) -> u64 {
    assert!(modulus>0);
    let m=modulus as u128;
    let mut b=(base as u128)%m;
    let mut result=1u128 % m;
    while exponent>0 {
        if exponent&1==1 { result=(result*b)%m; }
        b=(b*b)%m;
        exponent >>= 1;
    }
    result as u64
}
/// Repeatable Monte Carlo demonstration; not a BPP guarantee for arbitrary inputs.
pub fn pi_estimate(samples:usize, mut seed:u64) -> f64 {
    assert!(samples>0);
    let mut hits=0usize;
    for _ in 0..samples {
        seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x=((seed>>32) as u32) as f64 / (u32::MAX as f64);
        seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y=((seed>>32) as u32) as f64 / (u32::MAX as f64);
        if x*x+y*y<=1.0 { hits+=1; }
    }
    4.0*hits as f64/samples as f64
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch07::{minimum_vertex_cover,verifies_vertex_cover};
    #[test] fn approximation() {
        let e=[(0,1),(1,2),(2,3)];
        let c=two_approx_vertex_cover(&e);
        assert!(verifies_vertex_cover(4,&e,&c));
        assert!(c.len()<=2*minimum_vertex_cover(4,&e).len());
    }
    #[test] fn arithmetic() {
        assert_eq!(mod_pow(2,10,17),4);
        let pi=pi_estimate(5000,42);
        assert!(pi>2.8 && pi<3.5);
    }
}
