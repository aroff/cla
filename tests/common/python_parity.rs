//! Utilities for comparing Rust outputs with Python reference outputs

use ndarray::Array1;
use rustcla::TOL_WEIGHTS;
use rustcla::TOL_RETURNS;

/// Compare two weight vectors within tolerance
///
/// Returns true if all weights are within TOL_WEIGHTS (1e-5)
pub fn compare_weights(rust_weights: &Array1<f64>, python_weights: &Array1<f64>) -> bool {
    if rust_weights.len() != python_weights.len() {
        return false;
    }
    
    rust_weights
        .iter()
        .zip(python_weights.iter())
        .all(|(&r, &p)| (r - p).abs() <= TOL_WEIGHTS)
}

/// Compare two return values within tolerance
///
/// Returns true if values are within TOL_RETURNS (1e-6)
pub fn compare_return(rust_return: f64, python_return: f64) -> bool {
    (rust_return - python_return).abs() <= TOL_RETURNS
}

/// Compare two variance values within tolerance
///
/// Returns true if values are within TOL_RETURNS (1e-6)
pub fn compare_variance(rust_variance: f64, python_variance: f64) -> bool {
    (rust_variance - python_variance).abs() <= TOL_RETURNS
}

/// Compare arrays of returns within tolerance
pub fn compare_returns(rust_returns: &Array1<f64>, python_returns: &Array1<f64>) -> bool {
    if rust_returns.len() != python_returns.len() {
        return false;
    }
    
    rust_returns
        .iter()
        .zip(python_returns.iter())
        .all(|(&r, &p)| compare_return(r, p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn test_compare_weights() {
        let rust = Array1::from_vec(vec![0.5, 0.3, 0.2]);
        let python = Array1::from_vec(vec![0.500001, 0.299999, 0.2]);
        assert!(compare_weights(&rust, &python));
    }

    #[test]
    fn test_compare_return() {
        assert!(compare_return(0.1, 0.1000001));
        assert!(!compare_return(0.1, 0.2));
    }
}

