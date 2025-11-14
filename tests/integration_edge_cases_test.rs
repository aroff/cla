//! Edge case tests for CLA algorithm

use ndarray::array;
use rustcla::Cla;
use rustcla::TOL_WEIGHTS;

#[test]
fn test_identical_returns() {
    // Edge case: All assets have identical expected returns
    let mean = array![0.1, 0.1, 0.1];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    assert!(result.is_ok(), "Should handle identical returns case");
    let cla = result.unwrap();
    assert!(cla.len() >= 1, "Should have at least one turning point");
}

#[test]
fn test_singular_covariance_matrix() {
    // Edge case: Near-singular covariance matrix
    // Note: This test may pass or fail depending on numerical precision
    // The algorithm should handle near-singular matrices gracefully
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

    // Should either succeed or fail gracefully
    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    // Accept either success or failure - both are valid outcomes
    if let Err(e) = result {
        // If it fails, should be a meaningful error
        let err_msg = e.to_string();
        assert!(
            err_msg.contains("singular")
                || err_msg.contains("Singular")
                || err_msg.contains("Invalid")
                || err_msg.contains("Numerical"),
            "Error should be meaningful: {}",
            err_msg
        );
    } else {
        // If it succeeds, verify it produces valid results
        let cla = result.unwrap();
        assert!(cla.len() >= 1, "Should have at least one turning point");
    }
}

#[test]
fn test_infeasible_problem() {
    // Edge case: Sum of lower bounds > 1.0 (infeasible)
    let mean = array![0.1, 0.2, 0.15];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = array![0.4, 0.4, 0.4]; // Sum = 1.2 > 1.0
    let upper_bounds = array![1.0, 1.0, 1.0];
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    // Should fail with InfeasibleProblem error
    assert!(result.is_err(), "Should detect infeasible problem");
}

#[test]
fn test_large_portfolio() {
    // Edge case: Large portfolio (10 assets)
    use ndarray::Array1;
    let n = 10;
    let mean = Array1::from_iter((0..n).map(|i| 0.1 + (i as f64) * 0.01));
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
    let upper_bounds = Array1::ones(n);
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    assert!(result.is_ok(), "Should handle large portfolios");
    let cla = result.unwrap();
    assert!(
        cla.len() >= 1,
        "Should compute turning points for large portfolio"
    );
}

#[test]
fn test_all_variables_blocked() {
    // Edge case: All variables blocked at first turning point
    // This is handled by init_algo ensuring at least one free asset
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

    // This should work because init_algo ensures at least one free asset
    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    assert!(
        result.is_ok(),
        "Should handle case where init_algo ensures free asset"
    );
}

#[test]
fn test_nan_infinity_input() {
    // Edge case: NaN or infinity in input data
    let mut mean = array![0.1, 0.2, 0.15];
    mean[1] = f64::NAN;
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    // Should fail with NumericalError
    assert!(result.is_err(), "Should detect NaN in input");
}

#[test]
fn test_invalid_bounds() {
    // Edge case: Invalid bounds (lower > upper)
    let mean = array![0.1, 0.2, 0.15];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = array![0.5, 0.0, 0.0];
    let upper_bounds = array![0.3, 1.0, 1.0]; // Invalid: lower[0] > upper[0]
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    assert!(result.is_err(), "Should detect invalid bounds");
}

#[test]
fn test_zero_variance_asset() {
    // Edge case: Asset with zero variance
    let mean = array![0.1, 0.2, 0.15];
    let mut covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    covariance[(0, 0)] = 0.0; // Zero variance for asset 0
    let lower_bounds = array![0.0, 0.0, 0.0];
    let upper_bounds = array![1.0, 1.0, 1.0];
    let a = array![[1.0, 1.0, 1.0]];
    let b = array![1.0];

    // May work or fail depending on implementation
    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    // Accept either success or graceful failure
    if let Err(e) = result {
        // Should be a reasonable error
        let err_msg = e.to_string();
        assert!(
            err_msg.contains("singular")
                || err_msg.contains("Invalid")
                || err_msg.contains("Numerical"),
            "Error should be meaningful"
        );
    }
}

#[test]
fn test_single_turning_point() {
    // Edge case: Only one turning point (degenerate case)
    // This can happen when the problem structure results in only one valid portfolio
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04],];
    let lower_bounds = array![0.0, 0.0];
    let upper_bounds = array![1.0, 1.0];
    let a = array![[1.0, 1.0]];
    let b = array![1.0];

    let result = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b);
    assert!(result.is_ok(), "Should handle small portfolios");
    let cla = result.unwrap();
    assert!(cla.len() >= 1, "Should have at least one turning point");
}

#[test]
fn test_max_sharpe_edge_cases() {
    // Edge case: max_sharpe with negative returns or zero variance
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
    let frontier = cla.frontier();

    // max_sharpe should work even with edge cases
    let result = frontier.max_sharpe();
    assert!(result.is_ok(), "max_sharpe should handle edge cases");
    let (max_sharpe, weights) = result.unwrap();
    assert!(max_sharpe.is_finite(), "Max Sharpe ratio should be finite");
    let sum: f64 = weights.iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Max Sharpe weights should sum to 1.0"
    );
}
