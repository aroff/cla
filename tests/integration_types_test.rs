//! Integration tests for types comparing with Python cvxcla

use ndarray::array;
use rustcla::{Frontier, FrontierPoint, TOL_RETURNS, TOL_WEIGHTS};

#[test]
fn test_types_python_parity() {
    // Test case matching Python cvxcla README example
    let mean = array![0.1, 0.2, 0.15];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];

    // Create frontier points
    let point1 = FrontierPoint::new(array![1.0, 0.0, 0.0]).unwrap();
    let point2 = FrontierPoint::new(array![0.0, 1.0, 0.0]).unwrap();
    let point3 = FrontierPoint::new(array![0.0, 0.0, 1.0]).unwrap();

    let frontier = Frontier::new(
        mean.clone(),
        covariance.clone(),
        vec![point1, point2, point3],
    );

    // Test returns
    let returns = frontier.returns().unwrap();
    assert_eq!(returns.len(), 3);
    assert!(
        (returns[0] - 0.1).abs() < TOL_RETURNS,
        "First return should match Python"
    );
    assert!(
        (returns[1] - 0.2).abs() < TOL_RETURNS,
        "Second return should match Python"
    );
    assert!(
        (returns[2] - 0.15).abs() < TOL_RETURNS,
        "Third return should match Python"
    );

    // Test variances
    let variances = frontier.variance().unwrap();
    assert_eq!(variances.len(), 3);
    assert!(variances[0] > 0.0, "Variances should be positive");
    assert!(variances[1] > 0.0, "Variances should be positive");
    assert!(variances[2] > 0.0, "Variances should be positive");

    // Test volatilities
    let volatilities = frontier.volatility().unwrap();
    assert_eq!(volatilities.len(), 3);
    for &vol in volatilities.iter() {
        assert!(vol > 0.0, "Volatilities should be positive");
    }

    // Test Sharpe ratios
    let sharpe_ratios = frontier.sharpe_ratio().unwrap();
    assert_eq!(sharpe_ratios.len(), 3);
    // All should be positive for this test case
    for &sr in sharpe_ratios.iter() {
        assert!(sr.is_finite(), "Sharpe ratios should be finite");
    }

    // Test interpolation
    let interpolated = frontier.interpolate(5);
    assert!(
        interpolated.len() > frontier.len(),
        "Interpolated should have more points"
    );

    // Verify interpolated points are valid
    for point in interpolated.points() {
        let sum: f64 = point.weights().iter().sum();
        assert!(
            (sum - 1.0).abs() < TOL_WEIGHTS,
            "Interpolated point weights should sum to 1.0"
        );
    }
}
