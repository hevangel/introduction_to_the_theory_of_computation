"""Chapter 1: DFA, NFA; assumes binary or a/b alphabets respectively."""
def contains_01(word: str) -> bool:
    transition = ((1, 0), (1, 2), (2, 2))
    state = 0
    for c in word:
        if c not in "01":
            return False
        state = transition[state][int(c)]
    return state == 2

def ends_ab_nfa(word: str) -> bool:
    active = {0}
    for c in word:
        nxt = set()
        for state in active:
            if state == 0 and c == "a":
                nxt.update((0, 1))
            elif state == 0 and c == "b":
                nxt.add(0)
            elif state == 1 and c == "b":
                nxt.add(2)
        active = nxt
    return 2 in active
