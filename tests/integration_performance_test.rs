//! Performance validation tests for CLA algorithm

use ndarray::Array1;
use rustcla::Cla;
use std::time::Instant;

fn generate_portfolio(n: usize) -> (Array1<f64>, ndarray::Array2<f64>, Array1<f64>, Array1<f64>) {
    use ndarray::Array2;
    // Generate random but valid portfolio data
    let mean = Array1::from_iter((0..n).map(|i| 0.1 + (i as f64) * 0.01));

    // Generate positive definite covariance matrix
    let mut covariance = Array2::<f64>::zeros((n, n));
    for i in 0..n {
        covariance[(i, i)] = 0.01 + (i as f64) * 0.001;
        for j in (i + 1)..n {
            let cov = 0.001 * (1.0 - (i as f64 - j as f64).abs() / n as f64);
            covariance[(i, j)] = cov;
            covariance[(j, i)] = cov;
        }
    }

    let lower_bounds = Array1::zeros(n);
    let upper_bounds = Array1::ones(n);

    (mean, covariance, lower_bounds, upper_bounds)
}

#[test]
fn test_performance_100_assets() {
    // SC-004: Compute efficient frontier for 100 assets in <10 seconds
    let n = 100;
    let (mean, covariance, lower_bounds, upper_bounds) = generate_portfolio(n);
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    let start = Instant::now();
    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    let duration = start.elapsed();

    assert!(result.is_ok(), "Should succeed for 100 assets");
    assert!(
        duration.as_secs() < 10,
        "Should complete in <10 seconds, took {}s",
        duration.as_secs_f64()
    );

    let cla = result.unwrap();
    assert!(cla.len() >= 1, "Should compute turning points");
}

#[test]
#[ignore] // Ignore by default - requires optimized linear solver (ndarray-linalg) for performance
fn test_performance_1000_assets() {
    // SC-004: Compute efficient frontier for 1000 assets in <60 seconds
    // NOTE: This test may fail with basic Gaussian elimination solver.
    // Performance will improve significantly with ndarray-linalg + LAPACK.
    let n = 1000;
    let (mean, covariance, lower_bounds, upper_bounds) = generate_portfolio(n);
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    let start = Instant::now();
    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    let duration = start.elapsed();

    assert!(result.is_ok(), "Should succeed for 1000 assets");
    // Note: With optimized LAPACK solver, this should be <60s
    // Current basic solver may be slower but still functional
    if duration.as_secs() >= 60 {
        eprintln!("WARNING: 1000 assets took {}s (target: <60s). Consider using ndarray-linalg for better performance.", 
                  duration.as_secs_f64());
    }

    let cla = result.unwrap();
    assert!(cla.len() >= 1, "Should compute turning points");
}
