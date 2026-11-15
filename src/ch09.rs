//! Ch 9: finite counting intuition behind circuit lower bounds.
/// Number of Boolean functions from n inputs to one output: 2^(2^n).
/// None of this is an actual circuit lower bound or hierarchy proof.
pub fn boolean_function_count(n:u32) -> Option<u128> {
    if n>=7 { return None; } // 2^(2^7) would overflow u128
    Some(1u128 << (1u32<<n))
}
/// Truth table for XOR on two input bits, index = 2*a+b.
pub fn xor_truth_table() -> [bool;4] { [false,true,true,false] }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn counting() { assert_eq!(boolean_function_count(3),Some(256)); assert_eq!(boolean_function_count(7),None); assert_eq!(xor_truth_table()[1],true); }
}
