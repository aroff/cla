//! First turning point computation

use crate::error::ClaError;
use crate::types::TurningPoint;
use crate::TOL_WEIGHTS;
use ndarray::{Array1, ArrayView1};

/// Compute the first turning point on the efficient frontier.
///
/// The first turning point is the portfolio with the highest expected return
/// that satisfies all constraints. This is computed by:
/// 1. Sorting assets by expected return (descending)
/// 2. Setting all weights to lower bounds initially
/// 3. Moving weights from lower to upper bounds until sum >= 1.0
/// 4. The last asset moved becomes the free asset
///
/// # Arguments
///
/// * `mean` - Vector of expected returns for each asset
/// * `lower_bounds` - Vector of lower bounds for asset weights
/// * `upper_bounds` - Vector of upper bounds for asset weights
///
/// # Returns
///
/// A `TurningPoint` representing the first turning point with:
/// - Weights summing to 1.0
/// - At least one free asset
/// - Lambda set to infinity (highest return point)
///
/// # Errors
///
/// Returns `ClaError::InvalidInput` if bounds are invalid or problem is infeasible.
pub fn init_algo(
    mean: ArrayView1<f64>,
    lower_bounds: ArrayView1<f64>,
    upper_bounds: ArrayView1<f64>,
) -> Result<TurningPoint, ClaError> {
    let n = mean.len();

    // Validate bounds
    if n != lower_bounds.len() || n != upper_bounds.len() {
        return Err(ClaError::InvalidInput(format!(
            "All arrays must have same length: mean={}, lower={}, upper={}",
            n,
            lower_bounds.len(),
            upper_bounds.len()
        )));
    }

    // Check for invalid bounds
    for (i, (lower, upper)) in lower_bounds.iter().zip(upper_bounds.iter()).enumerate() {
        if lower > upper {
            return Err(ClaError::InvalidInput(format!(
                "Lower bound {} > upper bound {} for asset {}",
                lower, upper, i
            )));
        }
    }

    // Initialize weights to lower bounds
    let mut weights = lower_bounds.to_owned();
    let mut free = Array1::from_elem(n, false);

    // Sort indices by mean return (descending)
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| {
        mean[b]
            .partial_cmp(&mean[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Move weights from lower to upper bound until sum >= 1.0
    for &index in &indices {
        let remaining = 1.0 - weights.sum();
        if remaining <= TOL_WEIGHTS {
            break;
        }

        let available = upper_bounds[index] - lower_bounds[index];
        let to_add = remaining.min(available);
        weights[index] += to_add;

        if weights.sum() >= 1.0 - TOL_WEIGHTS {
            free[index] = true;
            break;
        }
    }

    // Normalize weights to exactly sum to 1.0
    let sum: f64 = weights.sum();
    if sum < 1.0 - TOL_WEIGHTS {
        return Err(ClaError::InfeasibleProblem(format!(
            "Could not construct a fully invested portfolio: sum of weights = {}",
            sum
        )));
    }

    // Adjust the free asset weight to make sum exactly 1.0
    if let Some(free_idx) = free.iter().position(|&f| f) {
        weights[free_idx] += 1.0 - sum;
    } else {
        // If no free asset was set, we need to ensure at least one
        // This handles the case where sum of upper bounds exactly equals 1.0
        if (sum - 1.0).abs() > TOL_WEIGHTS {
            return Err(ClaError::InfeasibleProblem(
                "Could not construct a fully invested portfolio".to_string(),
            ));
        }
        // Mark the last asset as free if all are at bounds
        if let Some(&last_idx) = indices.last() {
            free[last_idx] = true;
        }
    }

    // Ensure at least one free asset
    if !free.iter().any(|&f| f) {
        return Err(ClaError::SingularMatrix(
            "At least one asset must be marked as free".to_string(),
        ));
    }

    // Create turning point with lambda = infinity (highest return)
    TurningPoint::new(weights, f64::INFINITY, free)
}
