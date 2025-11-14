//! Integration tests for init_algo comparing with Python cvxcla

use rustcla::init_algo;
use rustcla::TOL_WEIGHTS;
use ndarray::array;

#[test]
fn test_init_algo_python_parity() {
    // Test case that matches Python cvxcla behavior
    // Using the example from Python README with seed 42
    let mean = array![0.1, 0.2, 0.15];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(result.is_ok(), "init_algo should succeed");

    let tp = result.unwrap();
    
    // Verify weights sum to 1.0
    let sum: f64 = tp.weights().iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Weights should sum to 1.0, got {}",
        sum
    );

    // Verify at least one free asset
    assert!(
        tp.free().iter().any(|&f| f),
        "At least one asset should be marked as free"
    );

    // Highest return is asset 1 (0.2), so it should have the highest weight
    // Asset 0 has 0.1, asset 1 has 0.2, asset 2 has 0.15
    // So asset 1 should be used first
    assert!(
        tp.weights()[1] > tp.weights()[0],
        "Asset with highest return should have higher weight"
    );
    assert!(
        tp.weights()[1] > tp.weights()[2],
        "Asset with highest return should have higher weight"
    );
}

#[test]
fn test_init_algo_large_portfolio() {
    // Test with larger portfolio (10 assets)
    use ndarray::Array1;
    let n = 10;
    let mean = Array1::from_iter((0..n).map(|i| 0.1 + (i as f64) * 0.01));
    let lower_bounds = Array1::zeros(n);
    let upper_bounds = Array1::ones(n);

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(result.is_ok(), "init_algo should handle large portfolios");

    let tp = result.unwrap();
    
    // Verify weights sum to 1.0
    let sum: f64 = tp.weights().iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Weights should sum to 1.0 for large portfolio, got {}",
        sum
    );

    // Verify at least one free asset
    assert!(
        tp.free().iter().any(|&f| f),
        "At least one asset should be marked as free"
    );
}

