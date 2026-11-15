"""Chapter 5: 3-colorability <=p SAT, plus tiny brute-force SAT."""
from itertools import product

def three_color_to_cnf(n: int, edges: list[tuple[int, int]]) -> list[list[int]]:
    def var(v: int, c: int) -> int:
        return 3 * v + c + 1
    cnf = []
    for v in range(n):
        cnf.append([var(v, c) for c in range(3)])
        for c in range(3):
            for d in range(c + 1, 3):
                cnf.append([-var(v, c), -var(v, d)])
    for u, v in edges:
        assert 0 <= u < n and 0 <= v < n
        for c in range(3):
            cnf.append([-var(u, c), -var(v, c)])
    return cnf

def brute_sat(cnf: list[list[int]], variables: int) -> bool:
    assert 0 <= variables <= 20
    for bits in product((False, True), repeat=variables):
        if all(any(bits[abs(lit) - 1] == (lit > 0) for lit in clause)
               for clause in cnf):
            return True
    return False
