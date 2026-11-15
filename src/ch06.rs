//! Ch 6: effective enumeration and a finite compression proxy.
pub fn binary_strings_up_to(max_len: usize) -> Vec<String> {
    assert!(max_len<=16);
    let mut out = Vec::new();
    for len in 0..=max_len {
        for value in 0..(1usize<<len) {
            out.push(format!("{:0width$b}",value,width=len));
        }
    }
    // format width=0 on zero prints "0", not epsilon; fix the empty word.
    out[0] = String::new();
    out
}
/// Simple run length encoding, NOT Kolmogorov complexity.
pub fn run_length_encode(s: &str) -> Vec<(char,usize)> {
    let mut out: Vec<(char,usize)> = Vec::new();
    for c in s.chars() {
        if let Some((last,count)) = out.last_mut() {
            if *last==c { *count+=1; continue; }
        }
        out.push((c,1));
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn enumerate() { assert_eq!(&binary_strings_up_to(2), &["","0","1","00","01","10","11"]); }
    #[test] fn encode() { assert_eq!(run_length_encode("aaabb"),vec![('a',3),('b',2)]); }
}
