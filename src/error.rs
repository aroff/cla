//! Error types for CLA operations

use thiserror::Error;

/// Errors that can occur during CLA operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ClaError {
    /// Invalid input data (dimensions mismatch, invalid bounds, etc.)
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Problem is infeasible (e.g., sum of lower bounds > 1.0)
    #[error("Infeasible problem: {0}")]
    InfeasibleProblem(String),

    /// Matrix is singular or near-singular
    #[error("Singular matrix: {0}")]
    SingularMatrix(String),

    /// Numerical error (NaN, infinity, etc.)
    #[error("Numerical error: {0}")]
    NumericalError(String),
}

/// Validate that all arrays have consistent dimensions
pub fn validate_dimensions(
    mean_len: usize,
    covariance_shape: (usize, usize),
    lower_bounds_len: usize,
    upper_bounds_len: usize,
) -> Result<(), ClaError> {
    let n = mean_len;
    if covariance_shape.0 != n || covariance_shape.1 != n {
        return Err(ClaError::InvalidInput(format!(
            "Covariance matrix must be {}x{}, got {}x{}",
            n, n, covariance_shape.0, covariance_shape.1
        )));
    }
    if lower_bounds_len != n {
        return Err(ClaError::InvalidInput(format!(
            "Lower bounds length must be {}, got {}",
            n, lower_bounds_len
        )));
    }
    if upper_bounds_len != n {
        return Err(ClaError::InvalidInput(format!(
            "Upper bounds length must be {}, got {}",
            n, upper_bounds_len
        )));
    }
    Ok(())
}

/// Validate that bounds are valid (lower <= upper for all assets)
pub fn validate_bounds(lower_bounds: &[f64], upper_bounds: &[f64]) -> Result<(), ClaError> {
    for (i, (lower, upper)) in lower_bounds.iter().zip(upper_bounds.iter()).enumerate() {
        if lower > upper {
            return Err(ClaError::InvalidInput(format!(
                "Invalid bounds for asset {}: lower ({}) > upper ({})",
                i, lower, upper
            )));
        }
    }
    Ok(())
}

/// Validate that covariance matrix is symmetric and positive semi-definite
pub fn validate_covariance_matrix(covariance: &ndarray::Array2<f64>) -> Result<(), ClaError> {
    let n = covariance.nrows();
    if n != covariance.ncols() {
        return Err(ClaError::InvalidInput(
            "Covariance matrix must be square".to_string(),
        ));
    }

    // Check symmetry (within numerical tolerance)
    for i in 0..n {
        for j in 0..n {
            if (covariance[(i, j)] - covariance[(j, i)]).abs() > 1e-10 {
                return Err(ClaError::InvalidInput(format!(
                    "Covariance matrix is not symmetric: [{},{}] = {}, [{},{}] = {}",
                    i,
                    j,
                    covariance[(i, j)],
                    j,
                    i,
                    covariance[(j, i)]
                )));
            }
        }
    }

    // Check for NaN or infinity
    for i in 0..n {
        for j in 0..n {
            let val = covariance[(i, j)];
            if val.is_nan() {
                return Err(ClaError::NumericalError(format!(
                    "NaN found in covariance matrix at [{},{}]",
                    i, j
                )));
            }
            if val.is_infinite() {
                return Err(ClaError::NumericalError(format!(
                    "Infinity found in covariance matrix at [{},{}]",
                    i, j
                )));
            }
        }
    }

    Ok(())
}
