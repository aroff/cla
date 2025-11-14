//! Test data utilities for generating portfolio test cases

use ndarray::{Array1, Array2};
use rustcla::Cla;

/// Generate a simple test portfolio with specified number of assets
///
/// Creates a portfolio with:
/// - Deterministic expected returns (increasing with asset index)
/// - Positive definite covariance matrix
/// - Standard bounds (0 to 1, no short selling, no leverage)
pub fn generate_test_portfolio(n: usize) -> (Array1<f64>, Array2<f64>, Array1<f64>, Array1<f64>) {
    // Generate expected returns
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

/// Create a CLA instance from test portfolio data
pub fn create_test_cla(n: usize) -> Result<Cla, rustcla::ClaError> {
    let (mean, covariance, lower_bounds, upper_bounds) = generate_test_portfolio(n);
    let a = Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);
    
    Cla::new(mean, covariance, lower_bounds, upper_bounds, a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_portfolio() {
        let (mean, covariance, lower_bounds, upper_bounds) = generate_test_portfolio(5);
        assert_eq!(mean.len(), 5);
        assert_eq!(covariance.nrows(), 5);
        assert_eq!(covariance.ncols(), 5);
        assert_eq!(lower_bounds.len(), 5);
        assert_eq!(upper_bounds.len(), 5);
    }

    #[test]
    fn test_create_test_cla() {
        let cla = create_test_cla(5).unwrap();
        assert!(cla.len() >= 1);
    }
}

