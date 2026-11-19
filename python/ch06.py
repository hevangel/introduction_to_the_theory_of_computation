"""Ch 6: Advanced computability — Recursion Theorem (quine), Cantor pairing,
first-order logic model evaluation on (Z_m, +), and descriptive complexity upper bounds."""
import math
from dataclasses import dataclass


def binary_strings_up_to(max_len: int) -> list[str]:
    """Enumerate all binary strings of length 0..=max_len in canonical shortlex order."""
    out = [""]
    for length in range(1, max_len + 1):
        out.extend(f"{mask:0{length}b}" for mask in range(1 << length))
    return out


def run_length_encode(text: str) -> list[tuple[str, int]]:
    """Run-length encoder (computable upper bound on descriptive complexity, NOT K(x))."""
    out: list[tuple[str, int]] = []
    for ch in text:
        if out and out[-1][0] == ch:
            out[-1] = (ch, out[-1][1] + 1)
        else:
            out.append((ch, 1))
    return out


def cantor_pair(x: int, y: int) -> int:
    """Cantor pairing bijection N x N -> N."""
    s = x + y
    return s * (s + 1) // 2 + y


def cantor_unpair(z: int) -> tuple[int, int]:
    """Inverse of Cantor pairing bijection N -> N x N."""
    w = (math.isqrt(8 * z + 1) - 1) // 2
    t = w * (w + 1) // 2
    y = z - t
    x = w - y
    return x, y


def self_reproducing_string() -> tuple[str, str]:
    """Constructive Recursion Theorem demo (SELF): a Python quine producing its own source."""
    s = "s = %r; print(s %% s)"
    rendered = s % s
    return rendered, rendered


@dataclass(frozen=True)
class EqSum:
    a: str
    b: str
    c: str


@dataclass(frozen=True)
class Not:
    inner: object


@dataclass(frozen=True)
class And:
    left: object
    right: object


@dataclass(frozen=True)
class Or:
    left: object
    right: object


@dataclass(frozen=True)
class ForAll:
    var: str
    body: object


@dataclass(frozen=True)
class Exists:
    var: str
    body: object


def eval_fo_mod(formula: object, modulus: int) -> bool:
    """Decide truth of a closed first-order sentence in the finite model (Z_m, +)."""
    env: dict[str, int] = {}

    def _eval(f: object) -> bool:
        if isinstance(f, EqSum):
            return (env[f.a] + env[f.b]) % modulus == env[f.c] % modulus
        if isinstance(f, Not):
            return not _eval(f.inner)
        if isinstance(f, And):
            return _eval(f.left) and _eval(f.right)
        if isinstance(f, Or):
            return _eval(f.left) or _eval(f.right)
        if isinstance(f, ForAll):
            for val in range(modulus):
                env[f.var] = val
                if not _eval(f.body):
                    return False
            return True
        if isinstance(f, Exists):
            for val in range(modulus):
                env[f.var] = val
                if _eval(f.body):
                    return True
            return False
        raise TypeError(f"Unsupported formula node: {f!r}")

    return _eval(formula)
