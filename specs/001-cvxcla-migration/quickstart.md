# Quickstart Guide: RustCLA Migration

**Feature**: CVXCLA to RustCLA Migration  
**Date**: 2025-11-13  
**Status**: Design Phase

## Overview

This quickstart guide provides a step-by-step introduction to using RustCLA, the Rust implementation of the Critical Line Algorithm for portfolio optimization. It demonstrates the core workflow and common usage patterns.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
rustcla = { version = "0.1.0", path = "../rustcla" }
ndarray = "0.15"
```

Or from crates.io (when published):

```toml
[dependencies]
rustcla = "0.1.0"
```

## Basic Usage

### Step 1: Define Your Portfolio Problem

```rust
use rustcla::{Cla, Array1, Array2};

// Number of assets
let n = 10;

// Expected returns for each asset
let mean = Array1::from_vec(vec![
    0.1, 0.15, 0.12, 0.18, 0.14,
    0.16, 0.11, 0.13, 0.17, 0.19
]);

// Covariance matrix (must be positive definite)
let covariance = // ... create n×n covariance matrix

// Bounds on asset weights (no short selling, no leverage)
let lower_bounds = Array1::zeros(n);  // All zeros
let upper_bounds = Array1::ones(n);   // All ones

// Equality constraints: fully invested (sum of weights = 1)
let a = Array2::ones((1, n));  // Constraint matrix
let b = Array1::ones(1);       // Constraint value
```

### Step 2: Create CLA Instance and Compute Frontier

```rust
// Create CLA instance - this automatically computes the efficient frontier
let cla = Cla::new(
    mean.clone(),
    covariance.clone(),
    lower_bounds,
    upper_bounds,
    a,
    b,
)?;

// Access the computed frontier
let frontier = cla.frontier();
println!("Found {} turning points", frontier.len());
```

### Step 3: Analyze the Efficient Frontier

```rust
// Get expected returns and variances
let returns = frontier.returns();
let variances = frontier.variance();
let volatilities = frontier.volatility();

// Find maximum Sharpe ratio portfolio
let (max_sharpe_ratio, max_sharpe_weights) = frontier.max_sharpe()?;
println!("Maximum Sharpe ratio: {:.6}", max_sharpe_ratio);
println!("Optimal weights: {:?}", max_sharpe_weights);
```

### Step 4: Interpolate for Smooth Curve

```rust
// Generate additional points between turning points for smoother visualization
let smooth_frontier = frontier.interpolate(100)?;
println!("Interpolated frontier has {} points", smooth_frontier.len());
```

### Step 5: Visualize (Optional)

```rust
// Plot efficient frontier (volatility on x-axis, with markers)
smooth_frontier.plot(true, true)?;
```

## Complete Example

```rust
use rustcla::{Cla, Array1, Array2};
use ndarray::array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define portfolio with 3 assets
    let mean = array![0.1, 0.15, 0.12];
    
    // Covariance matrix (must be positive definite)
    let covariance = array![
        [0.04, 0.01, 0.02],
        [0.01, 0.09, 0.03],
        [0.02, 0.03, 0.06]
    ];
    
    // Bounds: no short selling, no leverage
    let lower_bounds = Array1::zeros(3);
    let upper_bounds = Array1::ones(3);
    
    // Fully invested constraint
    let a = Array2::ones((1, 3));
    let b = array![1.0];
    
    // Compute efficient frontier
    let cla = Cla::new(
        mean.clone(),
        covariance.clone(),
        lower_bounds,
        upper_bounds,
        a,
        b,
    )?;
    
    let frontier = cla.frontier();
    
    // Find maximum Sharpe ratio
    let (sharpe, weights) = frontier.max_sharpe()?;
    println!("Maximum Sharpe ratio: {:.6}", sharpe);
    println!("Optimal weights: {:?}", weights);
    
    Ok(())
}
```

## Error Handling

All operations return `Result` types. Always handle errors:

```rust
match Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b) {
    Ok(cla) => {
        // Success - use cla
    }
    Err(ClaError::InvalidDimensions(msg)) => {
        eprintln!("Dimension mismatch: {}", msg);
    }
    Err(ClaError::InfeasibleProblem(msg)) => {
        eprintln!("Problem is infeasible: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

Or use `?` operator for propagation:

```rust
fn compute_portfolio() -> Result<(), ClaError> {
    let cla = Cla::new(...)?;  // Propagates error if validation fails
    let frontier = cla.frontier();
    let (sharpe, weights) = frontier.max_sharpe()?;
    Ok(())
}
```

## Migration from Python

### Python Code
```python
import numpy as np
from cvxcla import CLA

n = 10
mean = np.random.randn(n)
cov = np.random.randn(n, n)
covariance = cov @ cov.T
lower_bounds = np.zeros(n)
upper_bounds = np.ones(n)

cla = CLA(
    mean=mean,
    covariance=covariance,
    lower_bounds=lower_bounds,
    upper_bounds=upper_bounds,
    a=np.ones((1, n)),
    b=np.ones(1)
)

frontier = cla.frontier
max_sharpe_ratio, max_sharpe_weights = frontier.max_sharpe
```

### Equivalent Rust Code
```rust
use rustcla::{Cla, Array1, Array2};
use ndarray::Array;

let n = 10;
let mean = Array1::from_shape_vec(n, /* random values */)?;
let covariance = /* create covariance matrix */;
let lower_bounds = Array1::zeros(n);
let upper_bounds = Array1::ones(n);

let cla = Cla::new(
    mean,
    covariance,
    lower_bounds,
    upper_bounds,
    Array2::ones((1, n)),
    Array1::ones(1),
)?;

let frontier = cla.frontier();
let (max_sharpe_ratio, max_sharpe_weights) = frontier.max_sharpe()?;
```

## Key Differences from Python

1. **Error Handling**: Rust uses `Result` types instead of exceptions
2. **Ownership**: Rust's ownership model requires explicit borrowing (`&`) for references
3. **Type Safety**: Rust enforces type safety at compile time
4. **Method Calls**: Some Python properties become methods in Rust
5. **Array Creation**: Use `ndarray` constructors instead of `numpy` functions

## Next Steps

- See `data-model.md` for detailed data structure documentation
- See `contracts/api.md` for complete API reference
- Check examples in `examples/` directory
- Review tests in `tests/` for usage patterns

