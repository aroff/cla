//! Basic usage example for rustcla
//!
//! This example demonstrates how to use the Critical Line Algorithm
//! to compute the efficient frontier for a portfolio optimization problem.
//!
//! Run with: `cargo run --example basic_usage`

use ndarray::Array1;
use rustcla::{Cla, Frontier};

fn main() {
    // Set a seed for reproducibility (using a simple RNG seed)
    let n = 100; // Number of assets

    // Generate random expected returns
    let mean = Array1::from_iter((0..n).map(|i| {
        // Simple deterministic "random" values based on index
        (i as f64) * 0.1 - 0.5
    }));

    // Generate a positive definite covariance matrix
    let mut covariance = ndarray::Array2::<f64>::zeros((n, n));
    for i in 0..n {
        covariance[(i, i)] = 0.01 + (i as f64) * 0.001; // Diagonal
        for j in (i + 1)..n {
            let cov = 0.001 * (1.0 - (i as f64 - j as f64).abs() / n as f64);
            covariance[(i, j)] = cov;
            covariance[(j, i)] = cov; // Symmetric
        }
    }

    // No short selling (lower bound = 0)
    let lower_bounds = Array1::zeros(n);

    // No leverage (upper bound = 1)
    let upper_bounds = Array1::ones(n) * 0.3;

    // Fully invested constraint: sum of weights = 1
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    // Create a CLA instance
    let cla = Cla::new(
        mean.clone(),
        covariance.clone(),
        lower_bounds,
        upper_bounds,
        a,
        b,
    )
    .expect("Failed to create CLA instance");

    // Access the efficient frontier
    let frontier: Frontier = cla.frontier();

    // Get the maximum Sharpe ratio portfolio
    let (max_sharpe_ratio, max_sharpe_weights) = frontier
        .max_sharpe()
        .expect("Failed to compute max Sharpe ratio");

    println!("Maximum Sharpe ratio: {:.6}", max_sharpe_ratio);
    println!(
        "First 3 weights: [{:.6}, {:.6}, {:.6}]",
        max_sharpe_weights[0], max_sharpe_weights[1], max_sharpe_weights[2]
    );

    // Print some frontier statistics
    println!("\nFrontier Statistics:");
    println!("  Number of turning points: {}", frontier.len());
    let returns = frontier.returns().expect("Failed to compute returns");
    let volatility = frontier.volatility().expect("Failed to compute volatility");
    println!(
        "  Expected returns range: [{:.6}, {:.6}]",
        returns.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
        returns.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    );
    println!(
        "  Volatility range: [{:.6}, {:.6}]",
        volatility.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
        volatility.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    );
}
