"""Chapter 2: deterministic stack behavior and CYK parsing in CNF."""
def balanced_parentheses(word: str) -> bool:
    depth = 0
    for c in word:
        if c == "(":
            depth += 1
        elif c == ")" and depth:
            depth -= 1
        else:
            return False
    return depth == 0

def anbn(word: str) -> bool:
    n = len(word) - len(word.lstrip("a"))
    return word == "a" * n + "b" * n

def cyk(word: str, terminals: list[tuple[str, str]],
        binary: list[tuple[str, str, str]], start: str) -> bool:
    """Nonempty-word membership. Handle S->epsilon separately if required."""
    n = len(word)
    if not n:
        return False
    table = [[set() for _ in range(n)] for _ in range(n)]
    for i, letter in enumerate(word):
        table[i][i].update(v for v, t in terminals if letter == t)
    for span in range(2, n + 1):
        for i in range(n - span + 1):
            j = i + span - 1
            for k in range(i, j):
                for parent, left, right in binary:
                    if left in table[i][k] and right in table[k + 1][j]:
                        table[i][j].add(parent)
    return start in table[0][n - 1]
