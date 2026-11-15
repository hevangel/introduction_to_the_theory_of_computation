//! Ch 1: DFA and NFA execution, with a proof-oriented comparison.
/// DFA recognizing binary words containing the substring "01".
pub fn contains_01(word: &str) -> bool {
    let mut q = 0;
    for c in word.chars() {
        q = match (q, c) {
            (0, '0') => 1,
            (0, '1') => 0,
            (1, '0') => 1,
            (1, '1') => 2,
            (2, '0' | '1') => 2,
            _ => return false,
        };
    }
    q == 2
}
/// NFA for words ending in "ab". State 0 can guess where the suffix begins.
pub fn ends_ab_nfa(word: &str) -> bool {
    use std::collections::HashSet;
    let mut current = HashSet::from([0u8]);
    for c in word.chars() {
        let mut next = HashSet::new();
        for q in current {
            match (q, c) {
                (0, 'a') => { next.insert(0); next.insert(1); }
                (0, 'b') => { next.insert(0); }
                (1, 'b') => { next.insert(2); }
                _ => {}
            }
        }
        current = next;
    }
    current.contains(&2)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn dfa() { assert!(contains_01("11010")); assert!(!contains_01("111")); assert!(!contains_01("02")); }
    #[test] fn nfa() { assert!(ends_ab_nfa("bbaab")); assert!(!ends_ab_nfa("aba")); assert!(!ends_ab_nfa("acb")); }
}
