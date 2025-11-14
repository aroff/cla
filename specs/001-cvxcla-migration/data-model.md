# Data Model: CVXCLA to RustCLA Migration

**Feature**: CVXCLA to RustCLA Migration  
**Date**: 2025-11-13  
**Status**: Design Phase

## Overview

This document defines the core data structures for the Rust implementation of the Critical Line Algorithm. The data model maintains functional parity with the Python cvxcla implementation while leveraging Rust's type system for safety and correctness.

## Core Entities

### Portfolio Problem (Input Data)

Represents the input parameters for portfolio optimization.

**Fields**:
- `mean`: Vector of expected returns for each asset (f64 array, length n)
- `covariance`: Covariance matrix of asset returns (f64 matrix, n×n, must be positive definite)
- `lower_bounds`: Vector of lower bounds for asset weights (f64 array, length n)
- `upper_bounds`: Vector of upper bounds for asset weights (f64 array, length n)
- `equality_constraints`: Optional matrix A for linear equality constraints Ax = b (f64 matrix, m×n)
- `equality_values`: Optional vector b for linear equality constraints Ax = b (f64 array, length m)

**Validation Rules**:
- All arrays must have consistent dimensions (n assets)
- `lower_bounds[i] <= upper_bounds[i]` for all i
- `covariance` must be symmetric and positive definite
- Sum of `lower_bounds` must be <= 1.0 (feasibility)
- Sum of `upper_bounds` must be >= 1.0 (feasibility)
- If equality constraints provided, must be consistent (m <= n, rank(A) = m)

**Rust Representation**:
```rust
pub struct PortfolioProblem {
    pub mean: Array1<f64>,
    pub covariance: Array2<f64>,
    pub lower_bounds: Array1<f64>,
    pub upper_bounds: Array1<f64>,
    pub equality_constraints: Option<Array2<f64>>,
    pub equality_values: Option<Array1<f64>>,
}
```

---

### FrontierPoint

Represents a single portfolio on the efficient frontier.

**Fields**:
- `weights`: Vector of portfolio weights for each asset (f64 array, length n)

**Invariants**:
- Sum of weights must equal 1.0 within tolerance (1e-5)
- All weights must be within [lower_bound, upper_bound] for corresponding asset

**Methods**:
- `mean(&self, mean: &Array1<f64>) -> f64`: Compute expected return
- `variance(&self, covariance: &Array2<f64>) -> f64`: Compute portfolio variance

**Rust Representation**:
```rust
#[derive(Debug, Clone)]
pub struct FrontierPoint {
    weights: Array1<f64>,
}
```

---

### TurningPoint

Represents a corner of the efficient frontier where the set of free assets changes.

**Fields**:
- `weights`: Vector of portfolio weights (inherited from FrontierPoint)
- `lambda`: Risk aversion parameter value at this turning point (f64)
- `free`: Boolean vector indicating which assets are free (not at bounds) (bool array, length n)

**Invariants**:
- At least one asset must be marked as free (prevents singular matrix)
- Assets at upper bound: `weights[i] == upper_bounds[i]` and `free[i] == false`
- Assets at lower bound: `weights[i] == lower_bounds[i]` and `free[i] == false`
- Free assets: `lower_bounds[i] < weights[i] < upper_bounds[i]` and `free[i] == true`

**Methods**:
- `free_indices(&self) -> Vec<usize>`: Returns indices of free assets
- `blocked_indices(&self) -> Vec<usize>`: Returns indices of blocked assets
- Inherits `mean()` and `variance()` from FrontierPoint

**Rust Representation**:
```rust
#[derive(Debug, Clone)]
pub struct TurningPoint {
    weights: Array1<f64>,
    lambda: f64,
    free: Array1<bool>,
}
```

---

### Frontier

Represents the entire efficient frontier as a collection of frontier points.

**Fields**:
- `mean`: Vector of expected returns (reference data, length n)
- `covariance`: Covariance matrix (reference data, n×n)
- `points`: List of FrontierPoints representing the frontier

**Methods**:
- `interpolate(&self, num: usize) -> Frontier`: Generate additional points between existing points
- `max_sharpe(&self) -> Result<(f64, Array1<f64>), Error>`: Compute maximum Sharpe ratio portfolio
- `returns(&self) -> Array1<f64>`: Vector of expected returns for all points
- `variance(&self) -> Array1<f64>`: Vector of variances for all points
- `volatility(&self) -> Array1<f64>`: Vector of volatilities (sqrt of variance)
- `sharpe_ratio(&self) -> Array1<f64>`: Vector of Sharpe ratios
- `plot(&self, volatility: bool, markers: bool) -> Result<(), Error>`: Generate visualization

**Rust Representation**:
```rust
#[derive(Debug, Clone)]
pub struct Frontier {
    mean: Array1<f64>,
    covariance: Array2<f64>,
    points: Vec<FrontierPoint>,
}
```

---

### CLA (Critical Line Algorithm)

Main algorithm struct that computes the efficient frontier.

**Fields**:
- `mean`: Vector of expected returns (f64 array, length n)
- `covariance`: Covariance matrix (f64 matrix, n×n)
- `lower_bounds`: Vector of lower bounds (f64 array, length n)
- `upper_bounds`: Vector of upper bounds (f64 array, length n)
- `equality_constraints`: Matrix A for Ax = b (f64 matrix, m×n)
- `equality_values`: Vector b for Ax = b (f64 array, length m)
- `turning_points`: List of computed turning points (computed during initialization)
- `tolerance`: Numerical tolerance for comparisons (f64, default 1e-5)

**Computed Properties** (cached):
- `proj`: Projection matrix [Σ | A^T] for Lagrange multiplier computation
- `kkt`: KKT system matrix [[Σ, A^T], [A, 0]]

**Methods**:
- `new(...) -> Result<Self, Error>`: Create CLA instance and compute frontier
- `frontier(&self) -> Frontier`: Get the efficient frontier
- `len(&self) -> usize`: Number of turning points

**Rust Representation**:
```rust
pub struct Cla {
    mean: Array1<f64>,
    covariance: Array2<f64>,
    lower_bounds: Array1<f64>,
    upper_bounds: Array1<f64>,
    equality_constraints: Array2<f64>,
    equality_values: Array1<f64>,
    turning_points: Vec<TurningPoint>,
    tolerance: f64,
}
```

---

## Relationships

```
PortfolioProblem
    ↓ (input to)
CLA
    ↓ (computes)
TurningPoint[] (turning_points)
    ↓ (converted to)
FrontierPoint[] (in Frontier)
    ↓ (contained in)
Frontier
```

## State Transitions

### CLA Initialization Flow

1. **Input Validation**: Validate PortfolioProblem dimensions and constraints
2. **First Turning Point**: Call `init_algo()` to compute first turning point (highest return)
3. **Iterative Computation**: While lambda > 0:
   - Identify active set (free vs blocked assets)
   - Construct KKT system
   - Solve for alpha and beta
   - Compute Lagrange multipliers (gamma, delta)
   - Compute event ratios (lambda values)
   - Find next turning point
   - Update free set
4. **Final Point**: Add turning point at lambda = 0 (minimum variance)

### Frontier Computation Flow

1. **Create Frontier**: Convert turning_points to FrontierPoints
2. **Access Properties**: Compute returns, variances, volatilities on demand
3. **Interpolation**: Generate additional points between turning points (optional)
4. **Analysis**: Compute max_sharpe, plot, etc.

## Error Types

```rust
pub enum ClaError {
    InvalidDimensions,
    InvalidBounds,
    InfeasibleProblem,
    SingularMatrix,
    NumericalError,
    InvalidInput(String),
}
```

## Validation Rules Summary

1. **Dimension Consistency**: All vectors must have length n (number of assets)
2. **Matrix Dimensions**: Covariance must be n×n, equality constraints must be m×n
3. **Bounds Validity**: `lower_bounds[i] <= upper_bounds[i]` for all i
4. **Feasibility**: Sum of lower_bounds <= 1.0 <= Sum of upper_bounds
5. **Positive Definite**: Covariance matrix must be positive definite
6. **Weight Sum**: All portfolio weights must sum to 1.0 within tolerance
7. **Free Asset Requirement**: At least one asset must be free at each turning point

