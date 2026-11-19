"""Ch 3: Turing machines — general single-tape TM, classic textbook TMs (0^{2^n} and w#w),
and a 2-tape multitape TM simulator."""
from dataclasses import dataclass
from enum import Enum


class Outcome(Enum):
    ACCEPT = "accept"
    REJECT = "reject"
    TIMEOUT = "timeout"


class Move(Enum):
    LEFT = -1
    STAY = 0
    RIGHT = 1


def unary_successor(word: str, budget: int) -> tuple[Outcome, str]:
    """Append a '1' to a unary string of '1's with a step budget."""
    if any(ch != "1" for ch in word):
        return Outcome.REJECT, word
    tape = list(word) + ["_"]
    head = 0
    for _ in range(budget):
        if tape[head] == "1":
            head += 1
        else:
            tape[head] = "1"
            return Outcome.ACCEPT, "".join(tape)
    return Outcome.TIMEOUT, "".join(tape)


@dataclass
class TuringMachine:
    """General deterministic single-tape Turing Machine (Q, Σ, Γ, δ, q0, q_accept, q_reject)."""
    start: int
    accept: int
    reject: int
    transitions: dict[tuple[int, str], tuple[int, str, Move]]
    blank: str = "_"

    def run(self, word: str, budget: int = 1000) -> tuple[Outcome, str, int]:
        tape = list(word) if word else [self.blank]
        state = self.start
        head = 0
        for step in range(budget):
            if state == self.accept:
                return Outcome.ACCEPT, "".join(tape).rstrip(self.blank), step
            if state == self.reject:
                return Outcome.REJECT, "".join(tape).rstrip(self.blank), step
            action = self.transitions.get((state, tape[head]))
            if action is None:
                return Outcome.REJECT, "".join(tape).rstrip(self.blank), step + 1
            next_state, write_sym, move = action
            tape[head] = write_sym
            state = next_state
            if move is Move.LEFT:
                if head > 0:
                    head -= 1
            elif move is Move.RIGHT:
                head += 1
                if head == len(tape):
                    tape.append(self.blank)
        out = "".join(tape).rstrip(self.blank)
        if state == self.accept:
            return Outcome.ACCEPT, out, budget
        if state == self.reject:
            return Outcome.REJECT, out, budget
        return Outcome.TIMEOUT, out, budget


def tm_power_of_two_zeros() -> TuringMachine:
    """Sipser's classic Turing Machine M2 deciding A = { 0^(2^n) | n >= 0 }."""
    t = {
        (0, "0"): (1, "_", Move.RIGHT),
        (0, "_"): (6, "_", Move.RIGHT),
        (0, "x"): (6, "x", Move.RIGHT),
        (1, "x"): (1, "x", Move.RIGHT),
        (1, "_"): (5, "_", Move.RIGHT),
        (1, "0"): (2, "x", Move.RIGHT),
        (2, "x"): (2, "x", Move.RIGHT),
        (2, "_"): (4, "_", Move.LEFT),
        (2, "0"): (3, "0", Move.RIGHT),
        (3, "x"): (3, "x", Move.RIGHT),
        (3, "0"): (2, "x", Move.RIGHT),
        (3, "_"): (6, "_", Move.RIGHT),
        (4, "0"): (4, "0", Move.LEFT),
        (4, "x"): (4, "x", Move.LEFT),
        (4, "_"): (1, "_", Move.RIGHT),
    }
    return TuringMachine(start=0, accept=5, reject=6, transitions=t)


def two_tape_ww_hash(word: str) -> tuple[Outcome, int]:
    """2-tape TM deciding B = { w#w | w ∈ {0, 1}* } in O(n) steps."""
    tape2: list[str] = []
    h1 = 0
    steps = 0
    while h1 < len(word) and word[h1] != "#":
        if word[h1] not in ("0", "1"):
            return Outcome.REJECT, steps + 1
        tape2.append(word[h1])
        h1 += 1
        steps += 1
    if h1 == len(word) or word[h1] != "#":
        return Outcome.REJECT, steps + 1
    h1 += 1
    steps += 1 + len(tape2)  # skip '#' + rewind tape 2
    h2 = 0
    while h1 < len(word) and h2 < len(tape2):
        steps += 1
        if word[h1] != tape2[h2]:
            return Outcome.REJECT, steps
        h1 += 1
        h2 += 1
    if h1 == len(word) and h2 == len(tape2):
        return Outcome.ACCEPT, steps + 1
    return Outcome.REJECT, steps + 1
