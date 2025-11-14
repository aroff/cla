# Research: CVXCLA to RustCLA Migration

**Feature**: CVXCLA to RustCLA Migration  
**Date**: 2025-11-13  
**Purpose**: Resolve technical decisions for Rust migration

## Research Questions

### Q1: Matrix/Vector Library Choice (ndarray vs nalgebra)

**Context**: Python cvxcla uses numpy for all matrix and vector operations. Need to choose equivalent Rust library.

**Decision**: Use `ndarray` as the primary matrix/vector library

**Rationale**:
- `ndarray` provides numpy-like semantics and API, making migration more straightforward
- Better compatibility with scientific computing ecosystem (works well with BLAS/LAPACK)
- More flexible array shapes and broadcasting support
- Active maintenance and wide adoption in Rust scientific computing
- `ndarray-linalg` provides linear algebra operations that match numpy.linalg

**Alternatives Considered**:
- `nalgebra`: More type-safe with compile-time dimensions, but less flexible for dynamic-sized matrices (portfolio size varies)
- Pure Rust implementation: Too complex, reinventing well-solved problems

**References**:
- ndarray crate: https://docs.rs/ndarray/
- ndarray-linalg: https://docs.rs/ndarray-linalg/

---

### Q2: Linear Algebra Solver

**Context**: Python uses `numpy.linalg.solve` for KKT system solving. Need equivalent in Rust.

**Decision**: Use `ndarray-linalg` with LAPACK backend (OpenBLAS or Intel MKL)

**Rationale**:
- `ndarray-linalg` provides `solve` function matching numpy.linalg.solve semantics
- LAPACK backend ensures numerical accuracy matching Python (which uses LAPACK via numpy)
- OpenBLAS is default and works cross-platform
- Intel MKL available as optional for better performance on Intel hardware
- Maintains numerical accuracy parity with Python implementation

**Alternatives Considered**:
- Pure Rust linear algebra: Not mature enough, may have accuracy issues
- nalgebra: Could work but less direct numpy compatibility

**References**:
- ndarray-linalg documentation: https://docs.rs/ndarray-linalg/

---

### Q3: Visualization Library

**Context**: Python uses Plotly for efficient frontier visualization. Need Rust equivalent.

**Decision**: Use `plotters` crate for visualization

**Rationale**:
- `plotters` is the most mature and feature-rich plotting library in Rust
- Supports multiple backends (SVG, PNG, HTML canvas)
- Can generate static images suitable for documentation
- Active maintenance and good documentation
- Sufficient for 2D plots (return vs variance/volatility)

**Alternatives Considered**:
- `plotly-rs`: Direct Plotly bindings, but adds JavaScript dependency and complexity
- `egui`: GUI-focused, overkill for library visualization needs
- No visualization: Would break feature parity requirement

**Note**: Visualization is lower priority (P3 user story), can be implemented after core functionality.

**References**:
- plotters crate: https://docs.rs/plotters/

---

### Q4: Error Handling Strategy

**Context**: Python uses exceptions and assertions. Need Rust error handling approach.

**Decision**: Use custom error types with `thiserror` and `Result` return types

**Rationale**:
- Rust's type system enforces error handling at compile time
- Custom error types provide clear error messages
- `thiserror` simplifies error type definition
- Matches Rust best practices and constitution requirement for Result types
- Allows graceful error handling vs Python's exception model

**Error Categories**:
- Input validation errors (invalid bounds, dimensions mismatch)
- Algorithm errors (infeasible problem, singular matrix)
- Numerical errors (NaN, infinity detected)

**References**:
- thiserror crate: https://docs.rs/thiserror/

---

### Q5: Testing Strategy Details

**Context**: Need comprehensive testing matching Python test coverage.

**Decision**: Multi-layered testing approach:
1. Unit tests for individual functions
2. Integration tests for algorithm correctness
3. Property-based tests with `proptest` for mathematical properties
4. Accuracy tests comparing against Python reference outputs
5. Benchmark tests with `criterion` for performance validation

**Rationale**:
- Matches constitution requirement for >90% coverage
- Property-based tests catch edge cases automatically
- Accuracy tests ensure numerical parity
- Benchmarks validate performance goals

**References**:
- proptest: https://docs.rs/proptest/
- criterion: https://docs.rs/criterion/

---

### Q6: API Design Philosophy

**Context**: Balance Rust idioms with Python API compatibility.

**Decision**: Prefer Rust idioms while maintaining logical API parity

**Rationale**:
- Rust ownership model requires different API patterns than Python
- Use builder pattern or struct initialization instead of keyword arguments
- Return `Result` types instead of raising exceptions
- Use Rust naming conventions (snake_case for functions, PascalCase for types)
- Maintain same logical structure (CLA struct, Frontier type, etc.)

**Examples**:
- Python: `CLA(mean=..., covariance=..., ...)` 
- Rust: `Cla::new(mean, covariance, ...)?` or `ClaBuilder` pattern
- Python: `frontier.max_sharpe` (property)
- Rust: `frontier.max_sharpe()` (method returning Result)

---

## Summary

All technical decisions resolved:
- ✅ Matrix library: `ndarray` with `ndarray-linalg`
- ✅ Linear solver: LAPACK via `ndarray-linalg`
- ✅ Visualization: `plotters` crate
- ✅ Error handling: Custom errors with `thiserror`
- ✅ Testing: Multi-layered approach with `proptest` and `criterion`
- ✅ API design: Rust idioms with logical parity

All NEEDS CLARIFICATION markers from Technical Context are now resolved.

