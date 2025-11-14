<!--
Sync Impact Report:
Version: null → 1.0.0 (Initial constitution for rustcla migration project)
Modified principles: N/A (initial creation)
Added sections: Core Principles, Technology Stack, Development Workflow, Governance
Removed sections: N/A
Templates requiring updates:
  ✅ plan-template.md - Constitution Check section references constitution
  ✅ spec-template.md - No direct references, compatible
  ✅ tasks-template.md - No direct references, compatible
Follow-up TODOs: None
-->

# RustCLA Constitution

## Core Principles

### I. Rust Language Excellence (NON-NEGOTIABLE)
All Rust code MUST follow the official Rust Style Guide with `cargo fmt` formatting and `cargo clippy` linting. Code MUST pass all clippy lints with no warnings. Use idiomatic Rust patterns: ownership, borrowing, error handling with Result types, and appropriate use of traits. Code MUST be memory-safe and leverage Rust's type system for correctness. Rationale: Ensures maintainable, performant, and safe code that leverages Rust's strengths.

### II. Test-First Development (NON-NEGOTIABLE)
All code changes require comprehensive testing before implementation. Features are developed using TDD principles: write tests first, ensure they fail, then implement functionality. All tests MUST pass before PR submission, including unit tests, integration tests, and property-based tests where applicable. No regressions permitted - existing functionality must remain intact. Test coverage MUST be maintained at high levels (target: >90%). Rationale: Ensures correctness, prevents regressions, and maintains confidence during the Python-to-Rust migration.

### III. Numerical Accuracy & Correctness
All numerical computations MUST maintain accuracy parity with the Python reference implementation (cvxcla). Floating-point operations MUST use appropriate precision and handle edge cases (NaN, infinity, near-zero values). Algorithm implementations MUST be verified against known test cases and reference implementations. Performance optimizations MUST NOT compromise numerical correctness. Rationale: Scientific computing requires precise results; accuracy is non-negotiable for portfolio optimization algorithms.

### IV. Migration Fidelity
The Rust implementation MUST maintain functional parity with the Python cvxcla implementation. API surface MUST be equivalent or improved, maintaining backward compatibility where possible. All features from the Python version MUST be preserved, including efficient frontier computation, turning point calculation, and visualization capabilities. Migration decisions (API changes, performance improvements) MUST be documented and justified. Rationale: Ensures smooth transition for existing users and maintains the proven functionality of the original implementation.

### V. Documentation Discipline
All public APIs MUST have comprehensive documentation with examples. Code MUST include doc comments for all public items following Rust documentation conventions. User-facing changes MUST update README.md and relevant documentation. API changes MUST update API documentation. Documentation MUST maintain accuracy with code - outdated docs are treated as bugs. Rationale: Enables adoption, reduces maintenance burden, and supports the open-source community.

### VI. Performance & Efficiency
The Rust implementation MUST demonstrate performance improvements over the Python version where applicable. Performance-critical paths MUST be profiled and optimized. Memory usage MUST be reasonable and allocations minimized in hot paths. Benchmarking MUST be included for critical algorithms. Rationale: Performance is a key motivation for the Rust migration; the implementation should leverage Rust's zero-cost abstractions.

## Technology Stack

**Primary Language**: Rust (stable toolchain, minimum supported version documented in Cargo.toml)  
**Build System**: Cargo  
**Testing**: cargo test (unit), integration tests, property-based testing with proptest where applicable  
**Code Quality**: cargo fmt, cargo clippy, cargo audit (security)  
**Documentation**: rustdoc, mdBook for user documentation  
**CI/CD**: GitHub Actions with matrix testing across Rust versions  
**Dependencies**: Minimize external dependencies; prefer standard library and well-maintained crates from crates.io  
**Numerical Computing**: Use appropriate crates (e.g., ndarray for matrices, appropriate BLAS/LAPACK bindings if needed)

## Development Workflow

**Branching**: Feature branches from main, descriptive branch names  
**Commits**: Clear, descriptive commit messages following conventional commits  
**PR Requirements**: All tests passing, clippy clean, documentation updated, migration impact assessed  
**Code Review**: At least one approval required; focus on correctness, performance, and Rust idioms  
**Release Process**: Semantic versioning (MAJOR.MINOR.PATCH), changelog updates, crate publication to crates.io  
**Testing Strategy**: Unit tests for individual functions, integration tests for algorithm correctness, property tests for mathematical properties, benchmark tests for performance regression detection

## Governance

This constitution supersedes all other development practices and guidelines. Amendments require:
- Documentation of the proposed change
- Justification against existing principles
- Approval from project maintainers
- Migration plan if the change affects existing code

All PRs and code reviews MUST verify compliance with this constitution. Complexity must be justified against simpler alternatives. Violations of principles MUST be explicitly documented with rationale in the Complexity Tracking section of implementation plans.

**Version**: 1.0.0 | **Ratified**: 2025-11-13 | **Last Amended**: 2025-11-13
