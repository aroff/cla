//! Minimum Variance Portfolio Optimization Example
//!
//! This example demonstrates how to find the minimum variance portfolio
//! using the Critical Line Algorithm. The minimum variance portfolio
//! is the portfolio on the efficient frontier with the lowest risk (variance).
//!
//! Run with: `cargo run --example minvar`

use ndarray::Array1;
use rustcla::{Cla, Frontier};

fn main() {
    // Set up a small test problem with 3 assets
    let n = 3;

    // Generate a positive definite covariance matrix
    // Using a simple approach: create a random matrix and make it positive definite
    let mut covariance = ndarray::Array2::<f64>::zeros((n, n));

    // Create a simple positive definite matrix
    // Diagonal elements
    covariance[(0, 0)] = 0.04;
    covariance[(1, 1)] = 0.05;
    covariance[(2, 2)] = 0.06;

    // Off-diagonal elements (symmetric)
    covariance[(0, 1)] = 0.01;
    covariance[(1, 0)] = 0.01;
    covariance[(0, 2)] = 0.01;
    covariance[(2, 0)] = 0.01;
    covariance[(1, 2)] = 0.02;
    covariance[(2, 1)] = 0.02;

    // Expected returns (not critical for minimum variance, but needed for CLA)
    let mean = Array1::from_vec(vec![0.1, 0.12, 0.15]);

    // No short selling
    let lower_bounds = Array1::zeros(n);

    // No leverage
    let upper_bounds = Array1::ones(n);

    // Fully invested constraint: sum of weights = 1
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    // Create CLA instance and compute efficient frontier
    let cla = Cla::new(
        mean.clone(),
        covariance.clone(),
        lower_bounds,
        upper_bounds,
        a,
        b,
    )
    .expect("Failed to create CLA instance");

    let frontier: Frontier = cla.frontier();

    // Find the minimum variance portfolio (last point on frontier)
    // The efficient frontier goes from maximum return to minimum variance
    let variance = frontier.variance().expect("Failed to compute variance");
    let min_var_idx = variance
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .expect("Frontier should have at least one point");

    let min_var = variance[min_var_idx];
    let min_var_weights = frontier.weights().row(min_var_idx).to_owned();

    println!("Minimum Variance Portfolio:");
    println!("  Variance: {:.6}", min_var);
    println!("  Volatility (std dev): {:.6}", min_var.sqrt());
    println!(
        "  Expected Return: {:.6}",
        frontier.returns().unwrap()[min_var_idx]
    );
    println!(
        "  Weights: [{:.6}, {:.6}, {:.6}]",
        min_var_weights[0], min_var_weights[1], min_var_weights[2]
    );
    println!(
        "  Sum of weights: {:.6}",
        min_var_weights.iter().sum::<f64>()
    );

    // Verify the variance calculation
    let computed_var = min_var_weights.dot(&covariance.dot(&min_var_weights));
    println!("\nVerification:");
    println!("  Computed variance from weights: {:.6}", computed_var);
    println!("  Difference: {:.2e}", (computed_var - min_var).abs());
}
