//! Unit tests for init_algo function

use ndarray::array;
use rustcla::init_algo;
use rustcla::TOL_WEIGHTS;

#[test]
fn test_init_algo_basic() {
    // Basic case: 3 assets with different returns
    let mean = array![0.3, 0.2, 0.1];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(result.is_ok(), "init_algo should succeed for basic case");

    let tp = result.unwrap();

    // Check weights sum to 1.0
    let sum: f64 = tp.weights().iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Weights should sum to 1.0, got {}",
        sum
    );

    // Check at least one free asset
    assert!(
        tp.free().iter().any(|&f| f),
        "At least one asset should be marked as free"
    );

    // Highest return asset should be at upper bound or free
    // Asset 0 has highest return (0.3), so it should be used
    assert!(
        tp.weights()[0] > 0.0,
        "Highest return asset should have positive weight"
    );
}

#[test]
fn test_init_algo_identical_returns() {
    // Degenerate case: all assets have same return
    let mean = array![0.1, 0.1, 0.1];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(
        result.is_ok(),
        "init_algo should handle identical returns case"
    );

    let tp = result.unwrap();

    // Weights should still sum to 1.0
    let sum: f64 = tp.weights().iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Weights should sum to 1.0 even with identical returns, got {}",
        sum
    );

    // At least one asset should be free
    assert!(
        tp.free().iter().any(|&f| f),
        "At least one asset should be marked as free"
    );
}

#[test]
fn test_init_algo_bounds_sum_one() {
    // Case where sum of upper bounds equals 1.0
    let mean = array![0.3, 0.2, 0.1];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![0.5, 0.3, 0.2]; // Sum = 1.0

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(
        result.is_ok(),
        "init_algo should handle bounds sum equals one"
    );

    let tp = result.unwrap();

    // Weights should sum to 1.0
    let sum: f64 = tp.weights().iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Weights should sum to 1.0, got {}",
        sum
    );

    // All assets should be at their upper bounds or one should be free
    for i in 0..3 {
        assert!(
            tp.weights()[i] <= upper_bounds[i] + TOL_WEIGHTS,
            "Weight {} should not exceed upper bound",
            i
        );
    }
}

#[test]
fn test_init_algo_invalid_bounds() {
    // Test with invalid bounds (lower > upper)
    let mean = array![0.3, 0.2, 0.1];
    let lower_bounds = array![0.5, 0.0, 0.0];
    let upper_bounds = array![0.3, 1.0, 1.0]; // Invalid: lower[0] > upper[0]

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(result.is_err(), "init_algo should fail with invalid bounds");
}

#[test]
fn test_init_algo_infeasible() {
    // Test with infeasible problem (sum of upper bounds < 1.0)
    let mean = array![0.3, 0.2, 0.1];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![0.3, 0.3, 0.3]; // Sum = 0.9 < 1.0

    let result = init_algo(mean.view(), lower_bounds.view(), upper_bounds.view());
    assert!(
        result.is_err(),
        "init_algo should fail with infeasible problem"
    );
}
