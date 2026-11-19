"""Ch 10: Advanced complexity — 2-approximation for Vertex Cover, repaired probabilistic
primality (Miller–Rabin, 2nd ed. Victor Shoup repair), Read-Once Branching Programs,
Interactive Proofs (GNI verifier), Parallel Prefix Sum (NC), and RSA toy roundtrip."""
from dataclasses import dataclass


def two_approx_vertex_cover(edges: list[tuple[int, int]]) -> list[int]:
    """Factor-2 approximation for Vertex Cover (Sipser §10.1)."""
    covered: set[int] = set()
    cover: list[int] = []
    for u, v in edges:
        if u not in covered and v not in covered:
            covered.update((u, v))
            cover.extend((u, v))
    return cover


def mod_pow(base: int, exponent: int, modulus: int) -> int:
    """Fast modular exponentiation a^e mod m in O(log e) steps."""
    if modulus == 1:
        return 0
    result, cur = 1, base % modulus
    while exponent > 0:
        if exponent & 1:
            result = (result * cur) % modulus
        cur = (cur * cur) % modulus
        exponent >>= 1
    return result


def pi_estimate(samples: int, seed: int = 42) -> float:
    """Monte Carlo estimate of π using a deterministic LCG."""
    hits = 0
    for _ in range(samples):
        seed = (seed * 6364136223846793005 + 1) & ((1 << 64) - 1)
        x = (seed >> 33) / (1 << 31)
        seed = (seed * 6364136223846793005 + 1) & ((1 << 64) - 1)
        y = (seed >> 33) / (1 << 31)
        if x * x + y * y <= 1.0:
            hits += 1
    return 4.0 * hits / samples


def is_probably_prime(n: int) -> bool:
    """Miller–Rabin probabilistic primality test (repaired analysis, 2nd ed. §10.2).
    Exact for all 64-bit integers using the deterministic witness set below."""
    if n < 2:
        return False
    small = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in small:
        if n == p:
            return True
        if n % p == 0:
            return False
    d = n - 1
    s = 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:
        if a >= n:
            continue
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for _ in range(s - 1):
            x = (x * x) % n
            if x == n - 1:
                break
        else:
            return False
    return True


@dataclass(frozen=True)
class RobpBranch:
    var_idx: int
    low: int
    high: int


@dataclass(frozen=True)
class RobpSink:
    value: bool


def eval_robp(nodes: list[RobpBranch | RobpSink], root: int, inputs: list[bool]) -> bool:
    """Evaluate a Read-Once Branching Program (Sipser §10.2)."""
    cur = root
    while True:
        node = nodes[cur]
        if isinstance(node, RobpSink):
            return node.value
        cur = node.high if inputs[node.var_idx] else node.low


def parallel_prefix_sum(values: list[int]) -> tuple[list[int], int]:
    """O(log n)-depth parallel prefix sum illustrating the class NC (Sipser §10.5)."""
    n = len(values)
    cur = list(values)
    rounds = 0
    offset = 1
    while offset < n:
        prev = list(cur)
        for i in range(offset, n):
            cur[i] = prev[i] + prev[i - offset]
        rounds += 1
        offset <<= 1
    return cur, rounds


def verify_graph_isomorphism(
    n: int,
    g_edges: list[tuple[int, int]],
    h_edges: list[tuple[int, int]],
    perm: list[int],
) -> bool:
    """Verify whether bijection `perm` is an isomorphism from G to H (GNI §10.4)."""
    if len(perm) != n or set(perm) != set(range(n)):
        return False
    mapped = {(min(perm[u], perm[v]), max(perm[u], perm[v])) for u, v in g_edges}
    target = {(min(u, v), max(u, v)) for u, v in h_edges}
    return mapped == target


def rsa_roundtrip(p: int, q: int, e: int, msg: int) -> tuple[int, int] | None:
    """Toy RSA encrypt/decrypt roundtrip illustrating trapdoor one-way functions (§10.6)."""
    n = p * q
    phi = (p - 1) * (q - 1)
    if msg >= n:
        return None
    try:
        d = pow(e, -1, phi)
    except ValueError:
        return None
    cipher = mod_pow(msg, e, n)
    plain = mod_pow(cipher, d, n)
    return cipher, plain
