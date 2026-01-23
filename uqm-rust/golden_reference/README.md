# Golden Reference Implementation

This directory contains Python reference implementations of UQM game mechanics.

## Purpose

The Python code serves as the **"source of truth"** for validating the Rust implementation. Every Rust function must produce identical results to the corresponding Python function.

## Files

- `uqm_golden.py` - Core game mechanics reference implementation
- `validate.py` - Validation script to compare Rust and Python outputs
- `README.md` - This file

## Running Tests

```bash
# Run Python golden reference tests
python3 golden_reference/uqm_golden.py

# Expected output:
# ✅ All golden reference tests passed!
# 📖 Golden Reference Implementation Ready
```

## Validation Process

1. **Implement in Python First**: Write the expected behavior in Python
2. **Document Test Cases**: Define inputs and expected outputs
3. **Implement in Rust**: Match the Python behavior exactly
4. **Validate**: Run both implementations and compare results

## Test Categories

### Core Types
- Point distance calculations
- Vector arithmetic and normalization
- Type conversions

### Game State
- Resource management (fuel, crew, RU)
- Time progression
- Flag management

### Starmap
- Distance calculations
- Fuel cost computation
- Closest star search

### Combat
- Ship creation and stats
- Damage calculations
- Energy regeneration
- Position updates

## Adding New Tests

When implementing a new feature:

1. Write the Python version first in `uqm_golden.py`
2. Add test cases with clear inputs/outputs
3. Document any edge cases
4. Implement the Rust version to match
5. Run validation to ensure correctness

## Example Test Case

```python
# Python golden reference
def calculate_fuel_cost(distance: float) -> int:
    return int(distance * 0.5)

# Test cases
assert calculate_fuel_cost(100.0) == 50
assert calculate_fuel_cost(500.0) == 250
```

```rust
// Rust implementation must match
pub fn calculate_fuel_cost(distance: f32) -> u32 {
    (distance * 0.5) as u32
}

#[test]
fn test_fuel_cost() {
    assert_eq!(calculate_fuel_cost(100.0), 50);
    assert_eq!(calculate_fuel_cost(500.0), 250);
}
```

## Critical Rules

1. **Exact Match**: Rust must produce identical results
2. **Same Algorithms**: Use the same approach, not just same results
3. **Edge Cases**: Test boundary conditions thoroughly
4. **Floating Point**: Be aware of precision differences
5. **Documentation**: Explain any intentional differences

## Future Enhancements

- Automated validation script
- Performance comparison
- Fuzz testing with random inputs
- Visual diff tool for complex outputs
