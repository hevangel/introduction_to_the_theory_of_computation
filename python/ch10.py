"""Chapter 10: matching-based approximation, randomness and modexp."""
from random import Random

def two_approx_vertex_cover(edges: list[tuple[int, int]]) -> list[int]:
    chosen = set()
    for u, v in edges:
        if u not in chosen and v not in chosen:
            chosen.update((u, v))
    return sorted(chosen)

def mod_pow(base: int, exponent: int, modulus: int) -> int:
    assert modulus > 0 and exponent >= 0
    result = 1 % modulus
    base %= modulus
    while exponent:
        if exponent & 1:
            result = result * base % modulus
        base = base * base % modulus
        exponent >>= 1
    return result

def pi_estimate(samples: int, seed: int = 42) -> float:
    """Sampling illustration; does not establish a BPP error bound."""
    assert samples > 0
    rng = Random(seed)
    hits = sum(rng.random() ** 2 + rng.random() ** 2 <= 1
               for _ in range(samples))
    return 4 * hits / samples
