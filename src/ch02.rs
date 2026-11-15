//! Ch 2: deterministic stack machine and a CNF grammar recognizer.
use std::collections::HashSet;
/// One-type balanced parentheses: a deterministic pushdown language.
pub fn balanced_parentheses(word: &str) -> bool {
    let mut depth = 0usize;
    for c in word.chars() {
        match c {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            _ => return false,
        }
    }
    depth == 0
}
/// Recognize a^n b^n for n>=0 with deterministic bookkeeping.
pub fn anbn(word: &str) -> bool {
    let bytes = word.as_bytes();
    let n = bytes.iter().take_while(|&&b| b == b'a').count();
    bytes[n..].iter().all(|&b| b == b'b') && bytes.len() - n == n
}
/// CYK membership for a grammar in Chomsky normal form.
/// terminals: (nonterminal, terminal); binary: (parent, left, right).
/// For epsilon, extend the interface with a separate start->epsilon flag.
pub fn cyk(word: &str, terminals: &[(char, char)], binary: &[(char, char, char)], start: char) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    if n == 0 { return false; }
    let mut dp = vec![vec![HashSet::<char>::new(); n]; n];
    for (i, &letter) in chars.iter().enumerate() {
        for &(variable, terminal) in terminals {
            if letter == terminal { dp[i][i].insert(variable); }
        }
    }
    for span in 2..=n {
        for i in 0..=n-span {
            let j = i + span - 1;
            for k in i..j {
                for &(a, b, c) in binary {
                    if dp[i][k].contains(&b) && dp[k+1][j].contains(&c) {
                        dp[i][j].insert(a);
                    }
                }
            }
        }
    }
    dp[0][n-1].contains(&start)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn stack() { assert!(balanced_parentheses("(()())")); assert!(!balanced_parentheses("())(")); assert!(anbn("aaabbb")); assert!(!anbn("aab")); }
    #[test] fn grammar() {
        let t = [('A','a'), ('B','b')];
        let rules = [('S','A','B')];
        assert!(cyk("ab",&t,&rules,'S'));
        assert!(!cyk("aa",&t,&rules,'S'));
    }
}
