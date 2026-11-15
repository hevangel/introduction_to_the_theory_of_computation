"""Chapter 3: a bounded simulator of a specific unary Turing machine."""
from enum import Enum

class Outcome(Enum):
    ACCEPT = "accept"
    REJECT = "reject"
    TIMEOUT = "timeout"

def unary_successor(word: str, budget: int) -> tuple[Outcome, str]:
    if any(c != "1" for c in word):
        return Outcome.REJECT, word
    tape = list(word)
    head = 0
    for _ in range(budget):
        if head == len(tape):
            tape.append("1")
            return Outcome.ACCEPT, "".join(tape)
        head += 1
    return Outcome.TIMEOUT, "".join(tape)
