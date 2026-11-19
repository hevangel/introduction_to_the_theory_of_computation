//! Ch 3: Turing machines — general single-tape TM, classic textbook TMs (0^{2^n} and w#w),
//! and a 2-tape multitape TM simulator illustrating equivalence and time overhead.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Accept,
    Reject,
    Timeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
    Stay,
}

/// Append a single '1' to a unary string of '1's using a bounded TM step budget.
pub fn unary_successor(input: &str, budget: usize) -> (Outcome, String) {
    if !input.chars().all(|c| c == '1') {
        return (Outcome::Reject, input.to_string());
    }
    let mut tape: Vec<char> = input.chars().collect();
    tape.push('_');
    let mut head = 0usize;
    let mut steps = 0usize;
    while steps < budget {
        steps += 1;
        if tape[head] == '1' {
            head += 1;
        } else {
            tape[head] = '1';
            return (Outcome::Accept, tape.into_iter().collect());
        }
    }
    (Outcome::Timeout, tape.into_iter().collect())
}

/// General deterministic single-tape Turing Machine (Q, Σ, Γ, δ, q0, q_accept, q_reject).
#[derive(Debug, Clone)]
pub struct TuringMachine {
    pub start: usize,
    pub accept: usize,
    pub reject: usize,
    pub blank: char,
    pub transitions: BTreeMap<(usize, char), (usize, char, Move)>,
}

impl TuringMachine {
    /// Execute up to `budget` steps. Returns `(Outcome, final_trimmed_tape, steps_used)`.
    pub fn run(&self, input: &str, budget: usize) -> (Outcome, String, usize) {
        let mut tape: Vec<char> = if input.is_empty() {
            vec![self.blank]
        } else {
            input.chars().collect()
        };
        let mut state = self.start;
        let mut head: usize = 0;

        for step in 0..budget {
            if state == self.accept {
                return (Outcome::Accept, self.trim(&tape), step);
            }
            if state == self.reject {
                return (Outcome::Reject, self.trim(&tape), step);
            }
            let sym = tape[head];
            match self.transitions.get(&(state, sym)) {
                Some(&(next_state, write_sym, dir)) => {
                    tape[head] = write_sym;
                    state = next_state;
                    match dir {
                        Move::Left => {
                            if head > 0 {
                                head -= 1;
                            }
                        }
                        Move::Right => {
                            head += 1;
                            if head == tape.len() {
                                tape.push(self.blank);
                            }
                        }
                        Move::Stay => {}
                    }
                }
                None => return (Outcome::Reject, self.trim(&tape), step + 1),
            }
        }
        if state == self.accept {
            (Outcome::Accept, self.trim(&tape), budget)
        } else if state == self.reject {
            (Outcome::Reject, self.trim(&tape), budget)
        } else {
            (Outcome::Timeout, self.trim(&tape), budget)
        }
    }

    fn trim(&self, tape: &[char]) -> String {
        let s: String = tape.iter().collect();
        s.trim_end_matches(self.blank).to_string()
    }
}

/// Build Sipser's classic Turing Machine M2 deciding A = { 0^(2^n) | n >= 0 }.
/// Strategy: repeatedly sweep left-to-right crossing off every other '0' with 'x'.
/// - If a sweep sees exactly one '0', accept.
/// - If a sweep sees an odd number > 1 of '0's, reject.
/// - Otherwise return head to left end and repeat.
pub fn tm_power_of_two_zeros() -> TuringMachine {
    // States:
    // 0 (q1): start — expect first '0', mark it blank '_' (left sentinel) and move right
    // 1 (q2): seen 1st '0' of sweep (odd count so far) — scan for next '0' or blank
    // 2 (q3): crossed off a '0' (even count > 0) — scan for next '0' or blank
    // 3 (q4): saw another '0' (odd count > 1) — cross off if another '0' exists, else reject
    // 4 (q5): return head left to left sentinel '_'
    // 5: q_accept, 6: q_reject
    let mut t = BTreeMap::new();
    // q1 (0):
    t.insert((0, '0'), (1, '_', Move::Right));
    t.insert((0, '_'), (6, '_', Move::Right));
    t.insert((0, 'x'), (6, 'x', Move::Right));
    // q2 (1):
    t.insert((1, 'x'), (1, 'x', Move::Right));
    t.insert((1, '_'), (5, '_', Move::Right)); // only 1 zero remained -> ACCEPT!
    t.insert((1, '0'), (2, 'x', Move::Right)); // 2nd zero -> cross off
    // q3 (2): even number of zeros seen in this sweep
    t.insert((2, 'x'), (2, 'x', Move::Right));
    t.insert((2, '_'), (4, '_', Move::Left)); // even count -> rewind!
    t.insert((2, '0'), (3, '0', Move::Right)); // 3rd (odd) zero -> leave unmarked
    // q4 (3): odd (>1) number of zeros seen
    t.insert((3, 'x'), (3, 'x', Move::Right));
    t.insert((3, '0'), (2, 'x', Move::Right)); // pair it off -> back to even
    t.insert((3, '_'), (6, '_', Move::Right)); // odd > 1 -> REJECT!
    // q5 (4): rewind to left sentinel '_'
    t.insert((4, '0'), (4, '0', Move::Left));
    t.insert((4, 'x'), (4, 'x', Move::Left));
    t.insert((4, '_'), (1, '_', Move::Right));

    TuringMachine {
        start: 0,
        accept: 5,
        reject: 6,
        blank: '_',
        transitions: t,
    }
}

/// Decide B = { w#w | w ∈ {0, 1}* } using a 2-tape Turing Machine simulation.
/// Tape 1 holds input `w#w`; Tape 2 copies the first `w`, then both heads compare in O(n) steps.
pub fn two_tape_ww_hash(input: &str) -> (Outcome, usize) {
    let tape1: Vec<char> = input.chars().collect();
    let mut tape2: Vec<char> = Vec::new();
    let mut h1 = 0usize;
    let mut steps = 0usize;

    // Phase 1: copy w before '#' onto tape 2
    while h1 < tape1.len() && tape1[h1] != '#' {
        let c = tape1[h1];
        if c != '0' && c != '1' {
            return (Outcome::Reject, steps + 1);
        }
        tape2.push(c);
        h1 += 1;
        steps += 1;
    }
    if h1 == tape1.len() || tape1[h1] != '#' {
        return (Outcome::Reject, steps + 1);
    }
    h1 += 1; // skip '#'
    steps += 1;

    // Phase 2: rewind tape 2 head to 0 and compare symbol-by-symbol
    let mut h2 = 0usize;
    steps += tape2.len(); // rewind cost on tape 2
    while h1 < tape1.len() && h2 < tape2.len() {
        steps += 1;
        if tape1[h1] != tape2[h2] {
            return (Outcome::Reject, steps);
        }
        h1 += 1;
        h2 += 1;
    }
    if h1 == tape1.len() && h2 == tape2.len() {
        (Outcome::Accept, steps + 1)
    } else {
        (Outcome::Reject, steps + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successor() {
        assert_eq!(unary_successor("111", 4), (Outcome::Accept, "1111".into()));
        assert_eq!(unary_successor("111", 3).0, Outcome::Timeout);
        assert_eq!(unary_successor("10", 5).0, Outcome::Reject);
    }

    #[test]
    fn power_of_two_tm() {
        let m2 = tm_power_of_two_zeros();
        assert_eq!(m2.run("0", 50).0, Outcome::Accept);
        assert_eq!(m2.run("00", 50).0, Outcome::Accept);
        assert_eq!(m2.run("0000", 100).0, Outcome::Accept);
        assert_eq!(m2.run("00000000", 300).0, Outcome::Accept);
        assert_eq!(m2.run("", 50).0, Outcome::Reject);
        assert_eq!(m2.run("000", 100).0, Outcome::Reject);
        assert_eq!(m2.run("00000", 200).0, Outcome::Reject);
    }

    #[test]
    fn multitape_ww_hash() {
        assert_eq!(two_tape_ww_hash("0110#0110").0, Outcome::Accept);
        assert_eq!(two_tape_ww_hash("#").0, Outcome::Accept);
        assert_eq!(two_tape_ww_hash("01#10").0, Outcome::Reject);
        assert_eq!(two_tape_ww_hash("011").0, Outcome::Reject);
    }
}
