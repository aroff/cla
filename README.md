# RustCLA - Critical Line Algorithm for Portfolio Optimization

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

RustCLA is a Rust implementation of the Critical Line Algorithm (CLA) for portfolio optimization. This library provides efficient computation of the efficient frontier for portfolio optimization problems with linear constraints and bounds on asset weights.

## Features

- ✅ Efficient computation of the entire efficient frontier
- ✅ Support for linear constraints and bounds on portfolio weights
- ✅ Numerical accuracy matching Python cvxcla implementation
- ✅ Comprehensive test coverage
- ✅ High-performance Rust implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustcla = "0.1.0"
ndarray = "0.15"
ndarray-linalg = { version = "0.16", features = ["openblas"] }
```

## Quick Start

```rust
use rustcla::{Cla, Frontier};
use ndarray::Array1;

// Define your portfolio problem
let n = 10; // Number of assets
let mean = Array1::from_iter((0..n).map(|i| 0.1 + (i as f64) * 0.01));

// Generate a positive definite covariance matrix
let mut covariance = ndarray::Array2::<f64>::zeros((n, n));
for i in 0..n {
    covariance[(i, i)] = 0.01 + (i as f64) * 0.001;
    for j in (i + 1)..n {
        let cov = 0.001 * (1.0 - (i as f64 - j as f64).abs() / n as f64);
        covariance[(i, j)] = cov;
        covariance[(j, i)] = cov;
    }
}

let lower_bounds = Array1::zeros(n); // No short selling
let upper_bounds = Array1::ones(n);  // No leverage

// Fully invested constraint: sum(weights) = 1.0
let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
let b = Array1::from_vec(vec![1.0]);

// Create CLA instance and compute efficient frontier
let cla = Cla::new(mean.clone(), covariance.clone(), lower_bounds, upper_bounds, a, b)?;
let frontier: Frontier = cla.frontier();

// Get the maximum Sharpe ratio portfolio
let (max_sharpe_ratio, max_sharpe_weights) = frontier.max_sharpe()?;
println!("Maximum Sharpe ratio: {:.6}", max_sharpe_ratio);
println!("Number of turning points: {}", frontier.len());
```

## Examples

See the [examples](examples/) directory for more detailed usage examples:

- **basic_usage**: Basic CLA usage matching Python README example
- **minvar**: Finding the minimum variance portfolio
- **unconstrained**: Portfolio selection along the efficient frontier
- **plot_frontier**: Visualization of the efficient frontier (requires `plotting` feature)

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example plot_frontier --features plotting
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Migration from Python cvxcla

This library maintains functional parity with the Python [cvxcla](https://github.com/cvxgrp/cvxcla) implementation. See the [migration guide](docs/migration_guide.md) for details on transitioning from Python to Rust.
