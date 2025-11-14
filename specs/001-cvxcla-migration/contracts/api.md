# API Contracts: RustCLA Public Interface

**Feature**: CVXCLA to RustCLA Migration  
**Date**: 2025-11-13  
**Status**: Design Phase

## Overview

This document defines the public API contracts for the RustCLA library. The API maintains logical parity with Python cvxcla while following Rust idioms and best practices.

## Module Structure

```rust
pub mod cla;      // CLA algorithm
pub mod types;    // FrontierPoint, TurningPoint, Frontier
pub mod first;    // init_algo function
pub mod optimize; // minimize function
pub mod error;    // Error types
```

## Core API Contracts

### CLA (Critical Line Algorithm)

**Constructor**:
```rust
impl Cla {
    pub fn new(
        mean: Array1<f64>,
        covariance: Array2<f64>,
        lower_bounds: Array1<f64>,
        upper_bounds: Array1<f64>,
        equality_constraints: Array2<f64>,
        equality_values: Array1<f64>,
    ) -> Result<Self, ClaError>
```

**Contract**:
- **Input**: All arrays must have consistent dimensions
- **Preconditions**: 
  - `mean.len() == n` where n is number of assets
  - `covariance` is n×n symmetric positive definite matrix
  - `lower_bounds.len() == n` and `upper_bounds.len() == n`
  - `equality_constraints` is m×n matrix, `equality_values.len() == m`
  - Bounds valid: `lower_bounds[i] <= upper_bounds[i]` for all i
  - Problem feasible: `sum(lower_bounds) <= 1.0 <= sum(upper_bounds)`
- **Postconditions**:
  - Efficient frontier computed and stored in `turning_points`
  - At least one turning point exists
  - All turning points satisfy constraints
- **Errors**: Returns `ClaError` if validation fails or algorithm cannot proceed

**Methods**:
```rust
impl Cla {
    pub fn frontier(&self) -> Frontier;
    pub fn len(&self) -> usize;
    pub fn turning_points(&self) -> &[TurningPoint];
}
```

---

### FrontierPoint

**Constructor**:
```rust
impl FrontierPoint {
    pub fn new(weights: Array1<f64>) -> Result<Self, ClaError>
}
```

**Contract**:
- **Input**: `weights` vector of length n
- **Preconditions**: 
  - `weights.len() == n` (matches problem dimension)
  - `sum(weights)` is approximately 1.0 (within 1e-5 tolerance)
- **Postconditions**: Valid FrontierPoint created
- **Errors**: Returns `ClaError::InvalidInput` if weights don't sum to 1.0

**Methods**:
```rust
impl FrontierPoint {
    pub fn mean(&self, mean: &Array1<f64>) -> Result<f64, ClaError>;
    pub fn variance(&self, covariance: &Array2<f64>) -> Result<f64, ClaError>;
    pub fn weights(&self) -> &Array1<f64>;
}
```

**Contract**:
- `mean()`: Computes `mean^T @ weights`, returns expected return
- `variance()`: Computes `weights^T @ covariance @ weights`, returns portfolio variance
- Both methods validate input dimensions match weights dimension

---

### TurningPoint

**Constructor**:
```rust
impl TurningPoint {
    pub fn new(
        weights: Array1<f64>,
        lambda: f64,
        free: Array1<bool>,
    ) -> Result<Self, ClaError>
}
```

**Contract**:
- **Input**: weights, lambda value, free asset boolean vector
- **Preconditions**:
  - All vectors have same length n
  - At least one element in `free` is `true` (prevents singular matrix)
  - Weights satisfy bounds based on free/blocked status
- **Postconditions**: Valid TurningPoint created
- **Errors**: Returns error if invariants violated

**Methods**:
```rust
impl TurningPoint {
    pub fn free_indices(&self) -> Vec<usize>;
    pub fn blocked_indices(&self) -> Vec<usize>;
    pub fn lambda(&self) -> f64;
    // Inherits mean() and variance() from FrontierPoint
}
```

---

### Frontier

**Constructor**:
```rust
impl Frontier {
    pub fn new(
        mean: Array1<f64>,
        covariance: Array2<f64>,
        points: Vec<FrontierPoint>,
    ) -> Self
}
```

**Contract**:
- **Input**: Reference data (mean, covariance) and list of frontier points
- **Preconditions**: All points have weights matching mean/covariance dimensions
- **Postconditions**: Valid Frontier created

**Methods**:
```rust
impl Frontier {
    pub fn interpolate(&self, num: usize) -> Result<Frontier, ClaError>;
    pub fn max_sharpe(&self) -> Result<(f64, Array1<f64>), ClaError>;
    pub fn returns(&self) -> Array1<f64>;
    pub fn variance(&self) -> Array1<f64>;
    pub fn volatility(&self) -> Array1<f64>;
    pub fn sharpe_ratio(&self) -> Array1<f64>;
    pub fn plot(&self, volatility: bool, markers: bool) -> Result<(), ClaError>;
    pub fn len(&self) -> usize;
    pub fn iter(&self) -> impl Iterator<Item = &FrontierPoint>;
}
```

**Contract**:
- `interpolate(num)`: Generates `num-1` points between each pair of adjacent points
- `max_sharpe()`: Returns tuple of (max_sharpe_ratio, optimal_weights)
- `returns()`: Returns vector of expected returns for all points
- `variance()`: Returns vector of variances for all points
- `volatility()`: Returns sqrt of variance (standard deviation)
- `sharpe_ratio()`: Returns returns / volatility for each point
- `plot()`: Generates visualization (implementation detail, may use plotters)

---

### Helper Functions

#### init_algo

```rust
pub fn init_algo(
    mean: &Array1<f64>,
    lower_bounds: &Array1<f64>,
    upper_bounds: &Array1<f64>,
) -> Result<TurningPoint, ClaError>
```

**Contract**:
- **Input**: Mean returns and bounds vectors
- **Preconditions**: All vectors have same length, bounds valid
- **Postconditions**: Returns first turning point (highest return portfolio)
- **Behavior**: 
  - Sorts assets by expected return (descending)
  - Sets weights to lower bounds initially
  - Moves weights to upper bounds until sum >= 1.0
  - Adjusts last weight to make sum exactly 1.0
  - Marks last adjusted asset as free
- **Errors**: Returns error if cannot construct fully invested portfolio

#### minimize

```rust
pub fn minimize<F>(
    fun: F,
    x0: f64,
    bounds: Option<(f64, f64)>,
    tol: f64,
    max_iter: usize,
) -> Result<MinimizeResult, ClaError>
where
    F: Fn(f64) -> f64,
```

**Contract**:
- **Input**: Objective function, initial guess, optional bounds, tolerance, max iterations
- **Preconditions**: 
  - Function is continuous and unimodal in search region
  - Bounds valid if provided (lower < upper)
  - Initial guess within bounds if provided
- **Postconditions**: Returns minimum found within tolerance
- **Algorithm**: Golden section search
- **Errors**: Returns error if convergence fails or invalid inputs

---

## Error Types

```rust
#[derive(Debug, Error)]
pub enum ClaError {
    #[error("Invalid dimensions: {0}")]
    InvalidDimensions(String),
    
    #[error("Invalid bounds: {0}")]
    InvalidBounds(String),
    
    #[error("Infeasible problem: {0}")]
    InfeasibleProblem(String),
    
    #[error("Singular matrix encountered")]
    SingularMatrix,
    
    #[error("Numerical error: {0}")]
    NumericalError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

**Contract**:
- All errors implement `std::error::Error` trait
- Error messages are descriptive and actionable
- Errors are recoverable where possible (validation errors)
- Algorithm errors indicate problem with input data or numerical issues

---

## API Parity Mapping

| Python API | Rust API | Notes |
|------------|----------|-------|
| `CLA(mean=..., cov=..., ...)` | `Cla::new(mean, cov, ...)?` | Constructor with validation |
| `cla.frontier` | `cla.frontier()` | Method instead of property |
| `frontier.max_sharpe` | `frontier.max_sharpe()?` | Returns Result |
| `frontier.interpolate(num)` | `frontier.interpolate(num)?` | Returns Result |
| `frontier.plot(...)` | `frontier.plot(...)?` | Returns Result |
| `FrontierPoint(weights)` | `FrontierPoint::new(weights)?` | Constructor with validation |
| `point.mean(mean)` | `point.mean(&mean)?` | Returns Result |
| `point.variance(cov)` | `point.variance(&cov)?` | Returns Result |
| `init_algo(mean, lb, ub)` | `init_algo(&mean, &lb, &ub)?` | Returns Result |
| `minimize(fun, x0, ...)` | `minimize(fun, x0, ...)?` | Returns Result |

**Key Differences**:
- Rust uses `Result` types instead of exceptions
- Rust uses methods instead of properties (where applicable)
- Rust uses references (`&`) for borrowed parameters
- Rust uses `?` operator for error propagation

---

## Usage Example

```rust
use rustcla::{Cla, Array1, Array2};

// Create portfolio problem
let n = 10;
let mean = Array1::from_vec(vec![...]); // expected returns
let covariance = Array2::from_shape_vec((n, n), vec![...])?; // covariance matrix
let lower_bounds = Array1::zeros(n);
let upper_bounds = Array1::ones(n);
let a = Array2::ones((1, n)); // fully invested constraint
let b = Array1::ones(1);

// Compute efficient frontier
let cla = Cla::new(
    mean.clone(),
    covariance.clone(),
    lower_bounds,
    upper_bounds,
    a,
    b,
)?;

// Access frontier
let frontier = cla.frontier();

// Get maximum Sharpe ratio portfolio
let (max_sharpe, weights) = frontier.max_sharpe()?;
println!("Max Sharpe ratio: {}", max_sharpe);

// Interpolate for smoother curve
let smooth_frontier = frontier.interpolate(100)?;

// Plot
smooth_frontier.plot(true, true)?; // volatility on x-axis, with markers
```

