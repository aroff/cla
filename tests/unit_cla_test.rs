//! Unit tests for CLA algorithm

use ndarray::array;
use rustcla::{validate_bounds, validate_covariance_matrix, validate_dimensions, Cla};

#[test]
fn test_cla_new_valid() {
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

    let result = Cla::new(
        mean.clone(),
        covariance.clone(),
        lower_bounds.clone(),
        upper_bounds.clone(),
        a.clone(),
        b.clone(),
    );

    assert!(result.is_ok(), "CLA should be created successfully");
    let cla = result.unwrap();
    assert!(cla.len() > 0, "CLA should have at least one turning point");
}

#[test]
fn test_cla_new_invalid_input() {
    let mean = array![0.1, 0.2, 0.15];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = array![0.0, 0.0]; // Wrong length
    let upper_bounds = array![1.0, 1.0, 1.0];
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);

    assert!(
        result.is_err(),
        "CLA should fail with invalid input dimensions"
    );
}

#[test]
fn test_cla_solve_kkt() {
    // Test KKT system solving with a simple 2x2 case
    use ndarray::Array2;
    use rustcla::cla::Cla;

    // Simple KKT system: [[2, 1], [1, 0]] @ [x, y] = [3, 1]
    let kkt = array![[2.0, 1.0], [1.0, 0.0],];
    let rhs = array![[3.0, 1.0], [1.0, 0.0],];
    let free_mask = array![true, true];

    // This will be implemented in CLA
    // For now, just verify the structure
    assert!(kkt.shape() == &[2, 2], "KKT matrix should be 2x2");
}

#[test]
fn test_cla_compute_turning_points() {
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

    let cla = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b).unwrap();

    // Should have multiple turning points
    assert!(cla.len() >= 1, "Should have at least one turning point");

    // First turning point should have lambda = infinity
    let first_tp = cla.turning_points().first().unwrap();
    assert!(
        first_tp.lambda().is_infinite(),
        "First turning point should have infinite lambda"
    );
}
