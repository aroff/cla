//! Property-based tests for CLA mathematical properties

use ndarray::Array1;
use rustcla::Cla;
use rustcla::TOL_WEIGHTS;

#[test]
fn test_cla_weights_sum_property() {
    // Property: All turning point weights should sum to 1.0
    let mean = Array1::from_vec(vec![0.1, 0.2, 0.15]);
    let covariance = ndarray::array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = Array1::from_vec(vec![0.0, 0.0, 0.0]);
    let upper_bounds = Array1::from_vec(vec![1.0, 1.0, 1.0]);
    let a = ndarray::array![[1.0, 1.0, 1.0]];
    let b = Array1::from_vec(vec![1.0]);

    let cla = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b).unwrap();

    // All turning point weights should sum to 1.0
    for tp in cla.turning_points() {
        let sum: f64 = tp.weights().iter().sum();
        assert!(
            (sum - 1.0).abs() < TOL_WEIGHTS,
            "Weights should sum to 1.0, got {}",
            sum
        );
    }
}

#[test]
fn test_cla_efficient_frontier_monotonicity() {
    // Property: Efficient frontier returns should be non-increasing
    let mean = Array1::from_vec(vec![0.1, 0.2, 0.15]);
    let covariance = ndarray::array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = Array1::from_vec(vec![0.0, 0.0, 0.0]);
    let upper_bounds = Array1::from_vec(vec![1.0, 1.0, 1.0]);
    let a = ndarray::array![[1.0, 1.0, 1.0]];
    let b = Array1::from_vec(vec![1.0]);

    let cla = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b).unwrap();

    let frontier = cla.frontier();
    if frontier.len() > 1 {
        let returns = frontier.returns().unwrap();
        for i in 1..returns.len() {
            assert!(returns[i] <= returns[i-1] + 1e-6,
                "Returns should be non-increasing along efficient frontier: returns[{}] = {}, returns[{}] = {}",
                i-1, returns[i-1], i, returns[i]);
        }
    }
}

#[test]
fn test_cla_sharpe_ratio_property() {
    // Property: Sharpe ratios should be finite and meaningful
    let mean = Array1::from_vec(vec![0.1, 0.2, 0.15]);
    let covariance = ndarray::array![
        [0.01, 0.002, 0.003],
        [0.002, 0.04, 0.005],
        [0.003, 0.005, 0.009],
    ];
    let lower_bounds = Array1::from_vec(vec![0.0, 0.0, 0.0]);
    let upper_bounds = Array1::from_vec(vec![1.0, 1.0, 1.0]);
    let a = ndarray::array![[1.0, 1.0, 1.0]];
    let b = Array1::from_vec(vec![1.0]);

    let cla = Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b).unwrap();

    let frontier = cla.frontier();
    let sharpe_ratios = frontier.sharpe_ratio().unwrap();

    // All Sharpe ratios should be finite
    for &sr in sharpe_ratios.iter() {
        assert!(sr.is_finite(), "Sharpe ratios should be finite, got {}", sr);
    }
}
