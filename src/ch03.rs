//! Ch 3: bounded simulation of a particular Turing machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome { Accept, Reject, Timeout }
/// Machine: scan unary 1s to the first blank, write one more 1, halt.
/// Budget counts transition steps, NOT a universal halting decision procedure.
pub fn unary_successor(input: &str, budget: usize) -> (Outcome, String) {
    if !input.chars().all(|c| c == '1') { return (Outcome::Reject, input.to_string()); }
    let mut tape: Vec<char> = input.chars().collect();
    let mut head = 0usize;
    for _ in 0..budget {
        if head == tape.len() {
            tape.push('1');
            return (Outcome::Accept, tape.iter().collect());
        }
        head += 1;
    }
    (Outcome::Timeout, tape.iter().collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn successor() {
        assert_eq!(unary_successor("111",4),(Outcome::Accept,"1111".into()));
        assert_eq!(unary_successor("111",3).0,Outcome::Timeout);
        assert_eq!(unary_successor("10",100).0,Outcome::Reject);
    }
}
