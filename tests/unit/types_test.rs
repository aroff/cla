//! Unit tests for type system (FrontierPoint, TurningPoint, Frontier)

use rustcla::{FrontierPoint, TurningPoint, Frontier, TOL_WEIGHTS};
use ndarray::array;

#[test]
fn test_frontier_point_new() {
    // Valid weights that sum to 1.0
    let weights = array![0.5, 0.3, 0.2];
    let result = FrontierPoint::new(weights);
    assert!(result.is_ok(), "FrontierPoint should be created successfully");
}

#[test]
fn test_frontier_point_weights_sum() {
    // Weights that don't sum to 1.0
    let weights = array![0.5, 0.3, 0.1]; // Sum = 0.9
    let result = FrontierPoint::new(weights);
    assert!(result.is_err(), "FrontierPoint should reject weights that don't sum to 1.0");
}

#[test]
fn test_frontier_point_mean() {
    let weights = array![0.5, 0.3, 0.2];
    let point = FrontierPoint::new(weights).unwrap();
    let mean = array![0.1, 0.2, 0.15];
    
    let expected_return = point.mean(&mean).unwrap();
    let expected = 0.5 * 0.1 + 0.3 * 0.2 + 0.2 * 0.15; // 0.05 + 0.06 + 0.03 = 0.14
    assert!((expected_return - expected).abs() < 1e-10, "Expected return should be 0.14");
}

#[test]
fn test_frontier_point_variance() {
    let weights = array![0.5, 0.3, 0.2];
    let point = FrontierPoint::new(weights).unwrap();
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    
    let variance = point.variance(&covariance).unwrap();
    // Manual calculation: w^T @ cov @ w
    // Should be positive
    assert!(variance > 0.0, "Variance should be positive");
}

#[test]
fn test_turning_point_new() {
    let weights = array![0.5, 0.3, 0.2];
    let free = array![true, false, false];
    let lambda = 1.5;
    
    let result = TurningPoint::new(weights, lambda, free);
    assert!(result.is_ok(), "TurningPoint should be created successfully");
}

#[test]
fn test_turning_point_free_indices() {
    let weights = array![0.5, 0.3, 0.2];
    let free = array![true, false, true];
    let lambda = 1.5;
    
    let tp = TurningPoint::new(weights, lambda, free).unwrap();
    let free_indices = tp.free_indices();
    
    assert_eq!(free_indices.len(), 2, "Should have 2 free assets");
    assert!(free_indices.contains(&0), "Asset 0 should be free");
    assert!(free_indices.contains(&2), "Asset 2 should be free");
    assert!(!free_indices.contains(&1), "Asset 1 should not be free");
}

#[test]
fn test_turning_point_blocked_indices() {
    let weights = array![0.5, 0.3, 0.2];
    let free = array![true, false, true];
    let lambda = 1.5;
    
    let tp = TurningPoint::new(weights, lambda, free).unwrap();
    let blocked_indices = tp.blocked_indices();
    
    assert_eq!(blocked_indices.len(), 1, "Should have 1 blocked asset");
    assert_eq!(blocked_indices[0], 1, "Asset 1 should be blocked");
}

#[test]
fn test_turning_point_no_free_asset() {
    // Should fail if no free asset
    let weights = array![0.5, 0.3, 0.2];
    let free = array![false, false, false]; // No free assets
    let lambda = 1.5;
    
    let result = TurningPoint::new(weights, lambda, free);
    assert!(result.is_err(), "TurningPoint should fail if no free asset");
}

#[test]
fn test_frontier_interpolate() {
    let mean = array![0.1, 0.2, 0.15];
    let covariance = array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0, 0.0]).unwrap();
    let point2 = FrontierPoint::new(array![0.0, 1.0, 0.0]).unwrap();
    let point3 = FrontierPoint::new(array![0.0, 0.0, 1.0]).unwrap();
    
    let frontier = Frontier::new(
        mean.clone(),
        covariance.clone(),
        vec![point1, point2, point3],
    );
    
    let interpolated = frontier.interpolate(3);
    
    // Should have more points than original
    assert!(interpolated.len() > frontier.len(), "Interpolated frontier should have more points");
    
    // Check that all interpolated points are valid
    for point in interpolated.points() {
        let sum: f64 = point.weights().iter().sum();
        assert!((sum - 1.0).abs() < TOL_WEIGHTS, "Interpolated point weights should sum to 1.0");
    }
}

#[test]
fn test_frontier_weights() {
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04]];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0]).unwrap();
    let point2 = FrontierPoint::new(array![0.0, 1.0]).unwrap();
    
    let frontier = Frontier::new(mean, covariance, vec![point1, point2]);
    let weights_matrix = frontier.weights();
    
    assert_eq!(weights_matrix.shape(), &[2, 2], "Weights matrix should be 2x2");
    assert_eq!(weights_matrix[(0, 0)], 1.0, "First point, first asset should be 1.0");
    assert_eq!(weights_matrix[(1, 1)], 1.0, "Second point, second asset should be 1.0");
}

#[test]
fn test_frontier_returns() {
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04]];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0]).unwrap(); // Return = 0.1
    let point2 = FrontierPoint::new(array![0.0, 1.0]).unwrap(); // Return = 0.2
    
    let frontier = Frontier::new(mean, covariance, vec![point1, point2]);
    let returns = frontier.returns().unwrap();
    
    assert_eq!(returns.len(), 2, "Should have 2 returns");
    assert!((returns[0] - 0.1).abs() < 1e-10, "First return should be 0.1");
    assert!((returns[1] - 0.2).abs() < 1e-10, "Second return should be 0.2");
}

#[test]
fn test_frontier_variance() {
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04]];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0]).unwrap(); // Variance = 0.01
    let point2 = FrontierPoint::new(array![0.0, 1.0]).unwrap(); // Variance = 0.04
    
    let frontier = Frontier::new(mean, covariance, vec![point1, point2]);
    let variances = frontier.variance().unwrap();
    
    assert_eq!(variances.len(), 2, "Should have 2 variances");
    assert!((variances[0] - 0.01).abs() < 1e-10, "First variance should be 0.01");
    assert!((variances[1] - 0.04).abs() < 1e-10, "Second variance should be 0.04");
}

#[test]
fn test_frontier_volatility() {
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04]];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0]).unwrap();
    let point2 = FrontierPoint::new(array![0.0, 1.0]).unwrap();
    
    let frontier = Frontier::new(mean, covariance, vec![point1, point2]);
    let volatilities = frontier.volatility().unwrap();
    
    assert_eq!(volatilities.len(), 2, "Should have 2 volatilities");
    assert!((volatilities[0] - 0.1).abs() < 1e-10, "First volatility should be 0.1 (sqrt(0.01))");
    assert!((volatilities[1] - 0.2).abs() < 1e-10, "Second volatility should be 0.2 (sqrt(0.04))");
}

#[test]
fn test_frontier_sharpe_ratio() {
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04]];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0]).unwrap(); // Return=0.1, Vol=0.1, SR=1.0
    let point2 = FrontierPoint::new(array![0.0, 1.0]).unwrap(); // Return=0.2, Vol=0.2, SR=1.0
    
    let frontier = Frontier::new(mean, covariance, vec![point1, point2]);
    let sharpe_ratios = frontier.sharpe_ratio().unwrap();
    
    assert_eq!(sharpe_ratios.len(), 2, "Should have 2 Sharpe ratios");
    assert!((sharpe_ratios[0] - 1.0).abs() < 1e-10, "First Sharpe ratio should be 1.0");
    assert!((sharpe_ratios[1] - 1.0).abs() < 1e-10, "Second Sharpe ratio should be 1.0");
}

#[test]
fn test_frontier_max_sharpe() {
    let mean = array![0.1, 0.2];
    let covariance = array![[0.01, 0.002], [0.002, 0.04]];
    
    let point1 = FrontierPoint::new(array![1.0, 0.0]).unwrap();
    let point2 = FrontierPoint::new(array![0.0, 1.0]).unwrap();
    
    let frontier = Frontier::new(mean, covariance, vec![point1, point2]);
    let (max_sharpe, weights) = frontier.max_sharpe().unwrap();
    
    assert!(max_sharpe > 0.0, "Max Sharpe ratio should be positive");
    assert_eq!(weights.len(), 2, "Weights should have length 2");
    let sum: f64 = weights.iter().sum();
    assert!((sum - 1.0).abs() < TOL_WEIGHTS, "Max Sharpe weights should sum to 1.0");
}

