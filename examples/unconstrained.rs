//! Portfolio Selection Along Efficient Frontier Example
//!
//! This example demonstrates how different risk preferences affect portfolio
//! selection along the efficient frontier. We compute the efficient frontier
//! and then show how different points correspond to different risk-return trade-offs.
//!
//! Run with: `cargo run --example unconstrained`

use ndarray::Array1;
use rustcla::{Cla, Frontier};

fn main() {
    // Set up a small test problem with 3 assets
    let n = 3;

    // Create a positive definite covariance matrix
    let mut covariance = ndarray::Array2::<f64>::zeros((n, n));

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

    // Expected returns
    let mean = Array1::from_vec(vec![0.1, 0.12, 0.15]);

    let lower_bounds = Array1::zeros(n);
    let upper_bounds = Array1::ones(n);

    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    // Compute efficient frontier
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

    let returns = frontier.returns().expect("Failed to compute returns");
    let variances = frontier.variance().expect("Failed to compute variance");
    let weights_matrix = frontier.weights();

    println!("Portfolio Selection Along Efficient Frontier");
    println!("{}", "=".repeat(60));
    println!();

    // Show different points along the frontier
    let num_points = frontier.len().min(5);
    let step = if frontier.len() > 1 {
        (frontier.len() - 1) / (num_points - 1).max(1)
    } else {
        0
    };

    for i in (0..frontier.len()).step_by(step.max(1)) {
        let ret = returns[i];
        let var = variances[i];
        let vol = var.sqrt();
        let weights = weights_matrix.row(i).to_owned();

        println!("Point {} on Efficient Frontier:", i);
        println!("  Expected Return: {:.6}", ret);
        println!("  Variance: {:.6}", var);
        println!("  Volatility (std dev): {:.6}", vol);
        println!("  Sharpe Ratio: {:.6}", ret / vol);
        println!(
            "  Weights: [{:.6}, {:.6}, {:.6}]",
            weights[0], weights[1], weights[2]
        );
        println!();
    }

    // Show maximum return portfolio (first point)
    println!("Maximum Return Portfolio (Risk-Seeking):");
    let max_ret_idx = returns
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap_or(0);
    let max_ret_weights = weights_matrix.row(max_ret_idx).to_owned();
    println!("  Return: {:.6}", returns[max_ret_idx]);
    println!("  Volatility: {:.6}", variances[max_ret_idx].sqrt());
    println!(
        "  Weights: [{:.6}, {:.6}, {:.6}]",
        max_ret_weights[0], max_ret_weights[1], max_ret_weights[2]
    );
    println!();

    // Show minimum variance portfolio (last point)
    println!("Minimum Variance Portfolio (Risk-Averse):");
    let min_var_idx = variances
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap_or(0);
    let min_var_weights = weights_matrix.row(min_var_idx).to_owned();
    println!("  Return: {:.6}", returns[min_var_idx]);
    println!("  Volatility: {:.6}", variances[min_var_idx].sqrt());
    println!(
        "  Weights: [{:.6}, {:.6}, {:.6}]",
        min_var_weights[0], min_var_weights[1], min_var_weights[2]
    );
    println!();

    // Show maximum Sharpe ratio portfolio
    if let Ok((max_sharpe, max_sharpe_weights)) = frontier.max_sharpe() {
        println!("Maximum Sharpe Ratio Portfolio (Optimal Risk-Adjusted Return):");
        println!("  Sharpe Ratio: {:.6}", max_sharpe);
        let max_sharpe_var = max_sharpe_weights.dot(&covariance.dot(&max_sharpe_weights));
        let max_sharpe_ret = mean.dot(&max_sharpe_weights);
        println!("  Return: {:.6}", max_sharpe_ret);
        println!("  Volatility: {:.6}", max_sharpe_var.sqrt());
        println!(
            "  Weights: [{:.6}, {:.6}, {:.6}]",
            max_sharpe_weights[0], max_sharpe_weights[1], max_sharpe_weights[2]
        );
    }
}
