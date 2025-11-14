# Implementation Plan: CVXCLA to RustCLA Migration

**Branch**: `001-cvxcla-migration` | **Date**: 2025-11-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-cvxcla-migration/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Migrate the Python cvxcla library implementing the Critical Line Algorithm (CLA) for portfolio optimization to Rust, maintaining functional parity and numerical accuracy. The migration includes the core CLA algorithm, all data structures (FrontierPoint, TurningPoint, Frontier), helper functions (init_algo, minimize), comprehensive test suite, and example scripts. The Rust implementation will leverage Rust's performance characteristics while maintaining the proven algorithm correctness from the Python reference.

## Technical Context

**Language/Version**: Rust stable (minimum version 1.70+, target latest stable)  
**Primary Dependencies**: 
- `ndarray` for matrix/vector operations (numpy-like semantics)
- `ndarray-linalg` with LAPACK backend for linear algebra (OpenBLAS default, Intel MKL optional)
- `proptest` for property-based testing
- `plotters` for visualization (replaces Plotly)
- `approx` for floating-point comparisons in tests
- `thiserror` for error type definitions
- `criterion` for benchmarking

**Storage**: N/A (in-memory computation only)  
**Testing**: `cargo test` (unit), integration tests, `proptest` for property-based testing, benchmark tests with `criterion`  
**Target Platform**: Linux, macOS, Windows (cross-platform Rust library)  
**Project Type**: Library (single Rust crate)  
**Performance Goals**: 
- Compute efficient frontier for 100 assets in <10 seconds
- Compute efficient frontier for 1000 assets in <60 seconds
- Demonstrate performance improvement over Python implementation

**Constraints**: 
- Numerical accuracy: weights within 1e-5, returns/variances within 1e-6 of Python reference
- Memory: Reasonable allocation patterns, minimize allocations in hot paths
- API: Maintain functional parity with Python cvxcla API

**Scale/Scope**: 
- 4 main modules to migrate: cla.rs, types.rs, first.rs, optimize.rs
- ~500-800 lines of Python code (excluding tests)
- 5 example scripts to migrate
- Comprehensive test suite migration
- Support portfolios from 10 to 1000+ assets

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Rust Language Excellence
- ✅ **Gate**: All code must use `cargo fmt` and `cargo clippy` with zero warnings
- ✅ **Gate**: Use idiomatic Rust patterns (ownership, borrowing, Result types)
- **Status**: PASS - Will be enforced during implementation

### II. Test-First Development
- ✅ **Gate**: Write tests before implementation (TDD)
- ✅ **Gate**: Maintain >90% test coverage
- ✅ **Gate**: All tests must pass before PR submission
- **Status**: PASS - Test migration is a core user story (US4)

### III. Numerical Accuracy & Correctness
- ✅ **Gate**: Maintain accuracy parity with Python (weights: 1e-5, returns/variances: 1e-6)
- ✅ **Gate**: Verify against known test cases
- **Status**: PASS - Core requirement (FR-009, SC-001, SC-007)

### IV. Migration Fidelity
- ✅ **Gate**: Maintain functional parity with Python cvxcla
- ✅ **Gate**: Document any API changes with justification
- **Status**: PASS - Core requirement (FR-001, FR-006)

### V. Documentation Discipline
- ✅ **Gate**: All public APIs documented with examples
- ✅ **Gate**: Documentation accuracy maintained
- **Status**: PASS - User story US7 covers documentation

### VI. Performance & Efficiency
- ✅ **Gate**: Demonstrate performance improvements where applicable
- ✅ **Gate**: Profile and optimize critical paths
- **Status**: PASS - Performance goals defined (SC-004)

**Overall Status**: ✅ ALL GATES PASS - Ready for Phase 0 research

### Post-Phase 1 Re-evaluation

After Phase 1 design completion:

- ✅ **I. Rust Language Excellence**: Design uses idiomatic Rust patterns (Result types, ownership, references)
- ✅ **II. Test-First Development**: Test strategy defined with unit, integration, property-based, and accuracy tests
- ✅ **III. Numerical Accuracy & Correctness**: Data model enforces validation, accuracy requirements documented
- ✅ **IV. Migration Fidelity**: API contracts maintain logical parity with Python, documented differences
- ✅ **V. Documentation Discipline**: Quickstart guide and API contracts provide comprehensive documentation foundation
- ✅ **VI. Performance & Efficiency**: Performance goals defined, benchmarking strategy included

**Post-Design Status**: ✅ ALL GATES PASS - Design aligns with constitution principles

## Project Structure

### Documentation (this feature)

```text
specs/001-cvxcla-migration/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Library root, re-exports public API
├── cla.rs               # CLA struct and algorithm implementation
├── types.rs             # FrontierPoint, TurningPoint, Frontier types
├── first.rs             # init_algo function for first turning point
├── optimize.rs          # minimize function for 1D optimization
└── error.rs             # Error types for validation and algorithm errors

tests/
├── integration/
│   ├── cla_test.rs      # Integration tests for CLA algorithm
│   ├── types_test.rs    # Integration tests for types
│   └── accuracy_test.rs # Numerical accuracy validation against Python
├── unit/
│   ├── cla_test.rs      # Unit tests for CLA methods
│   ├── first_test.rs    # Unit tests for init_algo
│   └── optimize_test.rs # Unit tests for minimize
└── property/
    └── mathematical_properties_test.rs # Property-based tests

examples/
├── basic_usage.rs        # Basic CLA usage example
├── minvar.rs            # Minimum variance example (from minvar.py)
└── unconstrained.rs     # Unconstrained optimization example

benches/
└── cla_benchmark.rs     # Performance benchmarks

Cargo.toml                # Rust package configuration
README.md                 # User documentation
```

**Structure Decision**: Single Rust library crate following standard Rust project layout. Source code in `src/` with modules organized by functionality. Tests in `tests/` directory with integration, unit, and property-based test categories. Examples in `examples/` directory. Benchmarks in `benches/` directory.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations identified. All constitution principles are satisfied by the migration approach.
