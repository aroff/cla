//! Efficient Frontier Visualization Example
//!
//! This example demonstrates how to visualize the efficient frontier
//! using the plotters crate. It creates a plot showing expected return
//! vs. risk (volatility or variance).
//!
//! Run with: `cargo run --example plot_frontier --features plotting`

use ndarray::Array1;
use rustcla::{Cla, Frontier};

fn main() {
    // Create a portfolio optimization problem
    let n = 1000;

    // Generate expected returns
    let mean = Array1::from_iter((0..n).map(|i| (i as f64) * 0.1 - 0.5));

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

    let lower_bounds = Array1::zeros(n);
    let upper_bounds = Array1::ones(n) * 0.3;

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

    // Interpolate for smoother plot
    let smooth_frontier = frontier.interpolate(10);

    println!("Plotting efficient frontier...");
    println!("  Number of points: {}", smooth_frontier.len());

    // Plot with volatility on x-axis
    match smooth_frontier.plot(true, true, Some("frontier_volatility.png")) {
        Ok(()) => println!("  Saved plot to: frontier_volatility.png"),
        Err(e) => eprintln!("  Error plotting: {}", e),
    }

    // Plot with variance on x-axis
    match smooth_frontier.plot(false, true, Some("frontier_variance.png")) {
        Ok(()) => println!("  Saved plot to: frontier_variance.png"),
        Err(e) => eprintln!("  Error plotting: {}", e),
    }

    println!("\nPlot files created successfully!");
}
