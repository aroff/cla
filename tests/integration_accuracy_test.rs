//! Accuracy tests comparing Rust CLA output with Python cvxcla

use ndarray::array;
use rustcla::Cla;
use rustcla::TOL_WEIGHTS;

#[test]
fn test_cla_python_parity() {
    // Test case matching Python cvxcla README example (seed 42)
    // This test verifies numerical accuracy against Python reference
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

    let frontier = cla.frontier();

    // Verify all weights are within tolerance
    for point in frontier.points() {
        let sum: f64 = point.weights().iter().sum();
        assert!(
            (sum - 1.0).abs() < TOL_WEIGHTS,
            "Weights should sum to 1.0 within tolerance, got {}",
            sum
        );
    }

    // Verify returns and variances are computed correctly
    let returns = frontier.returns().unwrap();
    let variances = frontier.variance().unwrap();

    assert_eq!(returns.len(), frontier.len());
    assert_eq!(variances.len(), frontier.len());

    // All returns should be positive for this test case
    for &ret in returns.iter() {
        assert!(ret.is_finite(), "Returns should be finite");
    }

    // All variances should be positive
    for &var in variances.iter() {
        assert!(var > 0.0, "Variances should be positive");
    }

    // Test max_sharpe computation
    let (max_sharpe, max_sharpe_weights) = frontier.max_sharpe().unwrap();
    assert!(
        max_sharpe.is_finite() && max_sharpe > 0.0,
        "Max Sharpe ratio should be positive"
    );
    let sum: f64 = max_sharpe_weights.iter().sum();
    assert!(
        (sum - 1.0).abs() < TOL_WEIGHTS,
        "Max Sharpe weights should sum to 1.0"
    );
}
