//! Ch 9: Intractability — Boolean circuits, Shannon's counting argument,
//! and oracle-machine relativization models.

/// Number of distinct n-input Boolean functions: 2^(2^n).
/// Returns `None` if n > 6 (exceeds u128).
pub fn boolean_function_count(n: u32) -> Option<u128> {
    if n > 6 {
        return None;
    }
    Some(1u128 << (1u32 << n))
}

/// Truth table for 2-input XOR: (0,0)->0, (0,1)->1, (1,0)->1, (1,1)->0.
pub fn xor_truth_table() -> [bool; 4] {
    [false, true, true, false]
}

/// Gate in an acyclic Boolean circuit (Section 9.3).
/// Wires 0..num_inputs are the circuit inputs; gate `i` writes to wire `num_inputs + i`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    And(usize, usize),
    Or(usize, usize),
    Not(usize),
}

/// Acyclic Boolean circuit over {AND, OR, NOT}.
#[derive(Debug, Clone)]
pub struct BooleanCircuit {
    pub num_inputs: usize,
    pub gates: Vec<Gate>,
    pub output_wire: usize,
}

impl BooleanCircuit {
    /// Evaluate the circuit on a boolean input vector.
    pub fn eval(&self, inputs: &[bool]) -> bool {
        assert_eq!(inputs.len(), self.num_inputs);
        let mut wires = Vec::with_capacity(self.num_inputs + self.gates.len());
        wires.extend_from_slice(inputs);
        for &gate in &self.gates {
            let val = match gate {
                Gate::And(a, b) => wires[a] && wires[b],
                Gate::Or(a, b) => wires[a] || wires[b],
                Gate::Not(a) => !wires[a],
            };
            wires.push(val);
        }
        wires[self.output_wire]
    }

    /// Compute the full 2^n truth table of this circuit.
    pub fn truth_table(&self) -> Vec<bool> {
        let total = 1usize << self.num_inputs;
        let mut table = Vec::with_capacity(total);
        for mask in 0..total {
            let inputs: Vec<bool> = (0..self.num_inputs)
                .map(|i| (mask >> (self.num_inputs - 1 - i)) & 1 == 1)
                .collect();
            table.push(self.eval(&inputs));
        }
        table
    }
}

/// Build an {AND, OR, NOT} circuit computing 2-input XOR: (x0 ∨ x1) ∧ ¬(x0 ∧ x1).
pub fn xor_circuit() -> BooleanCircuit {
    // Wire 0: x0, Wire 1: x1
    // Wire 2: OR(0, 1)
    // Wire 3: AND(0, 1)
    // Wire 4: NOT(3)
    // Wire 5: AND(2, 4)
    BooleanCircuit {
        num_inputs: 2,
        gates: vec![
            Gate::Or(0, 1),
            Gate::And(0, 1),
            Gate::Not(3),
            Gate::And(2, 4),
        ],
        output_wire: 5,
    }
}

/// Illustrative oracle-query decision procedure (Section 9.2 Relativization):
/// Given an oracle `O: &str -> bool`, decides whether ANY string of length `len`
/// over `{0, 1}` is in `O` (the unary language U_O = { 1^n | ∃w ∈ {0,1}^n, w ∈ O }
/// used in the Baker–Gill–Solovay theorem separating P^B and NP^B).
pub fn oracle_exists_of_length<F>(len: usize, oracle: F) -> bool
where
    F: Fn(&str) -> bool,
{
    for mask in 0..(1usize << len) {
        let mut s = String::with_capacity(len);
        for bit in (0..len).rev() {
            s.push(if (mask >> bit) & 1 == 1 { '1' } else { '0' });
        }
        if oracle(&s) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting() {
        assert_eq!(boolean_function_count(3), Some(256));
        assert_eq!(xor_truth_table(), [false, true, true, false]);
    }

    #[test]
    fn circuit_and_oracle() {
        let c = xor_circuit();
        assert_eq!(c.truth_table(), vec![false, true, true, false]);
        assert!(oracle_exists_of_length(3, |w| w == "101"));
        assert!(!oracle_exists_of_length(3, |w| w == "1010"));
    }
}
