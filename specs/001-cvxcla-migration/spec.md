# Feature Specification: CVXCLA to RustCLA Migration

**Feature Branch**: `001-cvxcla-migration`  
**Created**: 2025-11-13  
**Status**: Draft  
**Input**: User description: "Please, create a migration proposal from codebase cvxcla to rustcla mirgate. Basically we should migrate all tests and features and examples so code runs properly."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Core CLA Algorithm Implementation (Priority: P1)

A developer needs to compute the efficient frontier for portfolio optimization using the Critical Line Algorithm in Rust, with the same functionality and numerical accuracy as the Python cvxcla implementation.

**Why this priority**: This is the core functionality that provides value to users. Without the CLA algorithm working correctly, the migration has no value. All other features depend on this foundation.

**Independent Test**: Can be fully tested by creating a CLA instance with portfolio data (mean returns, covariance matrix, bounds) and verifying that the efficient frontier is computed correctly with the same turning points and numerical results as the Python reference implementation.

**Acceptance Scenarios**:

1. **Given** a portfolio with n assets, expected returns, covariance matrix, lower/upper bounds, and equality constraints, **When** a CLA instance is created, **Then** the efficient frontier is automatically computed with all turning points identified
2. **Given** a CLA instance with computed frontier, **When** accessing the frontier property, **Then** a Frontier object is returned containing all turning points with correct weights, expected returns, and variances
3. **Given** portfolio data identical to Python cvxcla test cases, **When** computing the efficient frontier in Rust, **Then** the results match Python output within numerical tolerance (1e-5 for weights, 1e-6 for returns/variances)

---

### User Story 2 - Type System and Data Structures (Priority: P1)

A developer needs to work with portfolio data structures (FrontierPoint, TurningPoint, Frontier) that provide the same interface and behavior as the Python implementation.

**Why this priority**: These types are fundamental to the API and must be available for the core algorithm to function. They define the contract for how users interact with the library.

**Independent Test**: Can be fully tested by creating instances of each type with known data and verifying all methods (mean, variance, interpolation, max_sharpe, plot) produce correct results matching Python behavior.

**Acceptance Scenarios**:

1. **Given** portfolio weights that sum to 1.0, **When** creating a FrontierPoint, **Then** the point is created successfully and validates weight sum
2. **Given** a FrontierPoint with weights and mean returns, **When** calling the mean method, **Then** the expected return is computed correctly
3. **Given** a FrontierPoint with weights and covariance matrix, **When** calling the variance method, **Then** the portfolio variance is computed correctly
4. **Given** a Frontier with multiple points, **When** calling interpolate, **Then** additional points are generated between existing points with correct interpolation
5. **Given** a Frontier with computed points, **When** accessing max_sharpe property, **Then** the maximum Sharpe ratio portfolio is identified with correct weights and ratio value

---

### User Story 3 - First Turning Point Computation (Priority: P1)

A developer needs the init_algo function to compute the first turning point (highest expected return portfolio) that satisfies all constraints, matching Python behavior exactly.

**Why this priority**: The first turning point is the starting point for the CLA algorithm. Without it, the algorithm cannot proceed. This is a critical dependency for the core algorithm.

**Independent Test**: Can be fully tested independently by providing mean returns and bounds, and verifying the first turning point matches Python output with correct weights and free asset identification.

**Acceptance Scenarios**:

1. **Given** mean returns and upper/lower bounds, **When** calling init_algo, **Then** the first turning point is computed with weights summing to 1.0 and at least one free asset
2. **Given** mean returns where all assets have the same return, **When** calling init_algo, **Then** the algorithm handles the degenerate case correctly without errors
3. **Given** bounds where sum of upper bounds equals 1.0, **When** calling init_algo, **Then** all assets are set to upper bounds with one marked as free

---

### User Story 4 - Test Suite Migration and Validation (Priority: P2)

A developer needs comprehensive test coverage in Rust that validates all functionality matches the Python implementation, ensuring migration correctness.

**Why this priority**: Tests provide confidence that the migration is correct and prevent regressions. They serve as the validation mechanism for numerical accuracy and functional parity.

**Independent Test**: Can be fully tested by running the Rust test suite and verifying all tests pass, with results matching Python test outputs where applicable.

**Acceptance Scenarios**:

1. **Given** the Rust test suite, **When** running all tests, **Then** all tests pass with no failures
2. **Given** Python test cases with known inputs and expected outputs, **When** running equivalent Rust tests, **Then** Rust tests produce identical results within numerical tolerance
3. **Given** edge cases from Python tests (degenerate matrices, boundary conditions), **When** running Rust tests, **Then** all edge cases are handled correctly
4. **Given** property-based tests for mathematical properties, **When** running tests, **Then** properties hold for all generated test cases

---

### User Story 5 - Optimization Helper Functions (Priority: P2)

A developer needs the minimize function for 1D line search optimization used in computing maximum Sharpe ratio, with the same interface and behavior as Python.

**Why this priority**: The minimize function is used by the Frontier.max_sharpe computation. While not user-facing directly, it's required for complete feature parity.

**Independent Test**: Can be fully tested independently by providing objective functions and bounds, and verifying optimization results match Python output.

**Acceptance Scenarios**:

1. **Given** a scalar objective function and initial guess, **When** calling minimize, **Then** the function finds the minimum within specified tolerance
2. **Given** optimization with bounds, **When** calling minimize, **Then** the solution respects the bounds
3. **Given** the same objective function used in Python max_sharpe computation, **When** calling minimize in Rust, **Then** results match Python output

---

### User Story 6 - Examples and Experiments Migration (Priority: P3)

A developer needs working examples and experiments that demonstrate library usage, migrated from Python to Rust with equivalent functionality.

**Why this priority**: Examples help users understand how to use the library and validate that real-world use cases work correctly. Lower priority than core functionality but important for adoption.

**Independent Test**: Can be fully tested by running each example/experiment and verifying they execute successfully and produce expected outputs.

**Acceptance Scenarios**:

1. **Given** Python example scripts (minvar.py, unconstrained.py), **When** running equivalent Rust examples, **Then** all examples execute successfully. A basic_usage.rs example demonstrates core CLA functionality.
2. **Given** example scripts with specific inputs, **When** running Rust versions, **Then** outputs match Python outputs within numerical tolerance
3. **Given** the Marimo notebook example, **When** creating equivalent Rust demonstration, **Then** the demonstration works and produces similar visualizations

---

### User Story 7 - API Parity and Documentation (Priority: P3)

A developer needs the Rust API to match Python API structure where possible, with comprehensive documentation enabling smooth transition for existing users.

**Why this priority**: API parity reduces learning curve for existing Python users. Documentation is essential but can be refined after core functionality is working.

**Independent Test**: Can be fully tested by comparing Rust API surface to Python API and verifying documentation completeness and accuracy.

**Acceptance Scenarios**:

1. **Given** the Python CLA class interface, **When** examining Rust API, **Then** equivalent functionality is available with similar method names and signatures
2. **Given** Rust documentation, **When** a developer reads the docs, **Then** all public APIs are documented with examples
3. **Given** Python usage examples, **When** creating Rust equivalents, **Then** Rust examples follow similar patterns and produce equivalent results

---

### Edge Cases

- What happens when all assets have identical expected returns? (Degenerate case handling)
- How does the system handle singular or near-singular covariance matrices?
- What happens when sum of lower bounds exceeds 1.0? (Infeasible problem)
- How does the system handle very large portfolios (1000+ assets)?
- What happens when all variables are blocked at the first turning point? (Singular matrix prevention)
- How does the system handle NaN or infinity values in input data?
- What happens when bounds are invalid (lower > upper)?
- How does the system handle zero-variance assets?
- What happens during interpolation with only one turning point?
- How does max_sharpe computation handle edge cases (negative returns, zero variance)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST implement the Critical Line Algorithm (CLA) class with the same interface and behavior as Python cvxcla.CLA
- **FR-002**: System MUST compute the efficient frontier automatically upon CLA instantiation, identifying all turning points
- **FR-003**: System MUST provide FrontierPoint, TurningPoint, and Frontier types with equivalent functionality to Python
- **FR-004**: System MUST implement init_algo function to compute the first turning point with correct free asset identification
- **FR-005**: System MUST implement minimize function for 1D optimization with golden section search algorithm
- **FR-006**: System MUST provide Frontier.interpolate method to generate additional points between turning points
- **FR-007**: System MUST provide Frontier.max_sharpe() method to compute maximum Sharpe ratio portfolio
- **FR-008**: System MUST validate portfolio weights sum to 1.0 within tolerance (1e-5)
- **FR-009**: System MUST maintain numerical accuracy matching Python implementation (weights within 1e-5, returns/variances within 1e-6)
- **FR-010**: System MUST handle linear equality constraints (Ax = b) in portfolio optimization
- **FR-011**: System MUST support lower and upper bounds on individual asset weights
- **FR-012**: System MUST compute expected return for any portfolio point given mean returns vector
- **FR-013**: System MUST compute portfolio variance for any portfolio point given covariance matrix
- **FR-014**: System MUST handle edge cases: degenerate matrices, identical returns, infeasible problems, all variables blocked
- **FR-015**: System MUST provide comprehensive test suite covering all functionality with >90% code coverage
- **FR-016**: System MUST migrate all Python test cases to Rust with equivalent test coverage
- **FR-017**: System MUST migrate example scripts (minvar, unconstrained) to working Rust equivalents, plus provide a basic_usage.rs example demonstrating core CLA functionality
- **FR-018**: System MUST provide API documentation for all public types and functions
- **FR-019**: System MUST validate input data (check bounds validity, matrix dimensions, constraint consistency)
- **FR-020**: System MUST use appropriate numerical precision (f64) for all floating-point computations
- **FR-021**: System MUST handle KKT system solving with free/blocked variable partitioning
- **FR-022**: System MUST compute Lagrange multipliers (gamma, delta) for constraint analysis
- **FR-023**: System MUST identify next turning point by finding minimum positive event ratio (lambda)
- **FR-024**: System MUST update free set correctly when transitioning between turning points

### Key Entities *(include if feature involves data)*

- **CLA**: Main algorithm class containing mean returns, covariance matrix, bounds, constraints, and computed turning points. Provides efficient frontier computation via iterative turning point identification.
- **FrontierPoint**: Represents a single portfolio on the efficient frontier with weights vector. Provides methods to compute expected return and variance.
- **TurningPoint**: Specialized FrontierPoint representing a corner of the efficient frontier where the set of free assets changes. Contains weights, lambda value, and free asset boolean vector.
- **Frontier**: Collection of FrontierPoints representing the entire efficient frontier. Provides interpolation, iteration, and analysis methods (max_sharpe, plot).
- **Portfolio Problem**: Input data structure containing mean returns vector, covariance matrix, lower/upper bounds vectors, and equality constraint matrix/vector (A, b).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All Python test cases pass when run against Rust implementation with identical inputs, producing results within numerical tolerance (weights: 1e-5, returns/variances: 1e-6)
- **SC-002**: Rust test suite achieves >90% code coverage across all migrated functionality
- **SC-003**: All example scripts (3 total: basic_usage, minvar, unconstrained) execute successfully in Rust and produce outputs matching Python within tolerance
- **SC-004**: Core CLA algorithm computes efficient frontier for portfolios with 10-1000 assets in reasonable time (<10 seconds for 100 assets, <60 seconds for 1000 assets)
- **SC-005**: API documentation covers 100% of public types, functions, and methods with usage examples
- **SC-006**: Migration maintains functional parity: all Python cvxcla features work equivalently in Rust implementation
- **SC-007**: Numerical accuracy validation: 100% of test cases produce results matching Python reference within specified tolerances
- **SC-008**: Edge case handling: all identified edge cases (degenerate matrices, identical returns, infeasible problems) are handled gracefully without panics or incorrect results
