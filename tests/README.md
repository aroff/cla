# Test Suite

This directory contains the comprehensive test suite for `rustcla`.

## Test Organization

Tests are organized into several categories:

### Unit Tests (`tests/unit_*.rs`)
- **`unit_first_test.rs`**: Tests for `init_algo` function
- **`unit_types_test.rs`**: Tests for `FrontierPoint`, `TurningPoint`, and `Frontier` types
- **`unit_optimize_test.rs`**: Tests for `minimize` optimization function
- **`unit_cla_test.rs`**: Tests for `Cla` struct and its components

### Integration Tests (`tests/integration_*.rs`)
- **`integration_first_test.rs`**: Integration tests for `init_algo`
- **`integration_types_test.rs`**: Integration tests for types
- **`integration_optimize_test.rs`**: Integration tests for `minimize`
- **`integration_cla_test.rs`**: Full CLA algorithm integration tests
- **`integration_accuracy_test.rs`**: Accuracy validation against Python reference
- **`integration_edge_cases_test.rs`**: Edge case handling tests
- **`integration_performance_test.rs`**: Performance validation tests

### Property-Based Tests (`tests/property_*.rs`)
- **`property_mathematical_properties_test.rs`**: Property-based tests for mathematical invariants

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test File
```bash
cargo test --test unit_first_test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Ignored Tests
```bash
cargo test -- --ignored
```

### Run Tests in Parallel (default)
Tests run in parallel by default. To run sequentially:
```bash
cargo test -- --test-threads=1
```

## Test Coverage

The test suite aims for >90% code coverage. Current coverage includes:

- ✅ **init_algo**: 7 tests covering basic functionality, edge cases, and Python parity
- ✅ **Types**: 16 tests covering all FrontierPoint, TurningPoint, and Frontier methods
- ✅ **minimize**: 5 tests covering optimization with various bounds and convergence
- ✅ **CLA Algorithm**: 5 tests covering full algorithm execution and edge cases
- ✅ **Edge Cases**: 10 tests covering various edge cases (NaN, singular matrices, infeasible problems, etc.)
- ✅ **Property-Based**: 3 tests verifying mathematical invariants
- ✅ **Performance**: 2 benchmarks for 10 and 100 assets

## Test Data

Test data is generated programmatically in each test file. For consistency with Python tests, test cases use:
- Deterministic random number generation (where applicable)
- Standard portfolio sizes (3, 10, 100 assets)
- Positive definite covariance matrices
- Valid bounds (lower <= upper, sum constraints satisfied)

## Python Parity Tests

Tests in `integration_accuracy_test.rs` compare Rust outputs with Python `cvxcla` outputs to ensure numerical accuracy:
- Weights within 1e-5 tolerance
- Returns/variances within 1e-6 tolerance

## Performance Tests

Performance tests in `integration_performance_test.rs` validate:
- 100 assets: <10 seconds
- 1000 assets: <60 seconds (may require optimized linear solver)

Note: The 1000-asset test is marked `#[ignore]` by default as it may exceed the time limit with the basic Gaussian elimination solver. Performance will improve significantly with `ndarray-linalg` and LAPACK.

## Writing New Tests

When adding new tests:

1. **Unit Tests**: Place in appropriate `tests/unit_*.rs` file
2. **Integration Tests**: Place in appropriate `tests/integration_*.rs` file
3. **Property Tests**: Use `proptest` for property-based testing
4. **Follow Naming**: Use `test_` prefix for test functions
5. **Documentation**: Add doc comments explaining what the test validates
6. **Edge Cases**: Consider edge cases and error conditions

## Test Utilities

Common test utilities are available in `tests/common/mod.rs` (currently minimal, can be extended as needed).

## Continuous Integration

All tests must pass before code is merged. The CI pipeline runs:
- `cargo test` - All tests
- `cargo test --all-features` - Tests with all features enabled
- `cargo clippy` - Linting
- `cargo fmt --check` - Formatting check

