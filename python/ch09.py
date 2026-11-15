"""Chapter 9: counting functions, not proving circuit lower bounds."""
def boolean_function_count(inputs: int) -> int:
    assert inputs >= 0
    return 2 ** (2 ** inputs)

def xor_truth_table() -> tuple[bool, ...]:
    return False, True, True, False
