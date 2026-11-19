"""Ch 9: Intractability — Boolean circuits, Shannon's counting argument,
and oracle-machine relativization models."""
from dataclasses import dataclass
from typing import Callable


def boolean_function_count(inputs: int) -> int:
    """Number of distinct n-input Boolean functions: 2^(2^n)."""
    return 1 << (1 << inputs)


def xor_truth_table() -> tuple[bool, ...]:
    """Truth table for 2-input XOR."""
    return (False, True, True, False)


@dataclass(frozen=True)
class Gate:
    kind: str  # "and", "or", "not"
    a: int
    b: int = -1


@dataclass
class BooleanCircuit:
    """Acyclic Boolean circuit over {AND, OR, NOT} (Sipser §9.3)."""
    num_inputs: int
    gates: list[Gate]
    output_wire: int

    def eval(self, inputs: list[bool]) -> bool:
        assert len(inputs) == self.num_inputs
        wires = list(inputs)
        for g in self.gates:
            if g.kind == "and":
                wires.append(wires[g.a] and wires[g.b])
            elif g.kind == "or":
                wires.append(wires[g.a] or wires[g.b])
            elif g.kind == "not":
                wires.append(not wires[g.a])
            else:
                raise ValueError(f"Unknown gate: {g.kind}")
        return wires[self.output_wire]

    def truth_table(self) -> list[bool]:
        table: list[bool] = []
        for mask in range(1 << self.num_inputs):
            inp = [bool((mask >> (self.num_inputs - 1 - i)) & 1) for i in range(self.num_inputs)]
            table.append(self.eval(inp))
        return table


def xor_circuit() -> BooleanCircuit:
    """{AND, OR, NOT} circuit for 2-input XOR: (x0 ∨ x1) ∧ ¬(x0 ∧ x1)."""
    return BooleanCircuit(
        num_inputs=2,
        gates=[
            Gate("or", 0, 1),   # wire 2
            Gate("and", 0, 1),  # wire 3
            Gate("not", 3),     # wire 4
            Gate("and", 2, 4),  # wire 5
        ],
        output_wire=5,
    )


def oracle_exists_of_length(length: int, oracle: Callable[[str], bool]) -> bool:
    """Unary oracle language U_O = { 1^n | ∃w ∈ {0,1}^n, w ∈ O } (Baker–Gill–Solovay §9.2)."""
    for mask in range(1 << length):
        s = f"{mask:0{length}b}" if length > 0 else ""
        if oracle(s):
            return True
    return False
