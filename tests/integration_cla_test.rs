//! Integration tests for CLA algorithm

use ndarray::array;
use rustcla::Cla;
use rustcla::{TOL_RETURNS, TOL_WEIGHTS};

#[test]
fn test_cla_full_algorithm() {
    // Test the full CLA algorithm with a simple 3-asset portfolio
    let mean = array![0.1, 0.2, 0.15];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    let cla = Cla::new(
        mean.clone(),
        covariance.clone(),
        lower_bounds,
        upper_bounds,
        a,
        b,
    )
    .unwrap();

    // Should have multiple turning points
    assert!(
        cla.len() >= 2,
        "Should have at least 2 turning points (first and minimum variance)"
    );

    // First turning point should have infinite lambda
    let first_tp = cla.turning_points().first().unwrap();
    assert!(
        first_tp.lambda().is_infinite(),
        "First turning point should have infinite lambda"
    );

    // Last turning point should have lambda = 0
    let last_tp = cla.turning_points().last().unwrap();
    assert!(
        (last_tp.lambda() - 0.0).abs() < 1e-10,
        "Last turning point should have lambda = 0"
    );

    // All turning points should have valid weights
    for tp in cla.turning_points() {
        let sum: f64 = tp.weights().iter().sum();
        assert!((sum - 1.0).abs() < TOL_WEIGHTS, "Weights should sum to 1.0");
    }

    // Test frontier access
    let frontier = cla.frontier();
    assert_eq!(
        frontier.len(),
        cla.len(),
        "Frontier should have same number of points as turning points"
    );

    // Test that returns are decreasing (efficient frontier property)
    let returns = frontier.returns().unwrap();
    for i in 1..returns.len() {
        assert!(
            returns[i] <= returns[i - 1] + TOL_RETURNS,
            "Returns should be non-increasing along efficient frontier"
        );
    }
}
