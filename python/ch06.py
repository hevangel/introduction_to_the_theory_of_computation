"""Chapter 6: finite prefixes of an effective enumeration; toy compression."""
from itertools import product, groupby

def binary_strings_up_to(max_len: int) -> list[str]:
    return ["".join(bits) for n in range(max_len + 1)
            for bits in product("01", repeat=n)]

def run_length_encode(text: str) -> list[tuple[str, int]]:
    return [(ch, sum(1 for _ in group)) for ch, group in groupby(text)]
