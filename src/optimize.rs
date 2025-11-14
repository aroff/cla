//! 1D optimization helper functions

use crate::error::ClaError;

/// Result of optimization
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub x: f64,
    pub fun: f64,
    pub success: bool,
    pub nit: usize,
}

/// Minimize a scalar function using golden section search
///
/// This function implements a 1D line search optimization algorithm using
/// the golden section search method. It finds the minimum of a scalar function
/// within optional bounds.
///
/// # Arguments
///
/// * `fun` - The objective function to minimize: f(x) -> f64
/// * `x0` - Initial guess for the minimum
/// * `bounds` - Optional bounds for the search interval (lower, upper)
/// * `tol` - Tolerance for convergence (default: 1e-8)
/// * `max_iter` - Maximum number of iterations (default: 200)
///
/// # Returns
///
/// An `OptimizationResult` containing the optimal x value, function value,
/// success status, and number of iterations.
///
/// # Errors
///
/// Returns `ClaError` if optimization fails or bounds are invalid.
pub fn minimize<F>(
    fun: F,
    x0: f64,
    bounds: Option<(f64, f64)>,
    tol: f64,
    max_iter: usize,
) -> Result<OptimizationResult, ClaError>
where
    F: Fn(f64) -> f64,
{
    // Golden ratio constant: (sqrt(5) - 1) / 2
    const GOLDEN_RATIO: f64 = 0.6180339887498949;

    // Set default bounds if not provided
    let (lower, upper) = if let Some((l, u)) = bounds {
        if l > u {
            return Err(ClaError::InvalidInput(format!(
                "Lower bound {} must be <= upper bound {}",
                l, u
            )));
        }
        (l, u)
    } else {
        (f64::NEG_INFINITY, f64::INFINITY)
    };

    // Ensure initial guess is within bounds
    let x = x0.max(lower).min(upper);

    // Initialize search interval
    let (mut a, mut b) = if lower.is_finite() && upper.is_finite() {
        (lower, upper)
    } else {
        // If bounds are infinite, start with a small interval around x0
        let mut a_init = x - 1.0;
        let mut b_init = x + 1.0;
        let f_x = fun(x);

        // Set a reasonable limit for expansion to avoid overflow
        let max_expansion = 100.0;
        let min_bound = if lower.is_finite() {
            lower
        } else {
            x - max_expansion
        };
        let max_bound = if upper.is_finite() {
            upper
        } else {
            x + max_expansion
        };

        // Expand to the left to bracket a minimum
        let mut step = 1.0;
        while a_init > min_bound {
            let f_a = fun(a_init);
            if f_a < f_x {
                // Found a point with lower function value, expand further
                step *= 2.0;
                a_init = (a_init - step).max(min_bound);
            } else {
                break;
            }
        }
        a_init = a_init.max(min_bound);

        // Expand to the right to bracket a minimum
        step = 1.0;
        while b_init < max_bound {
            let f_b = fun(b_init);
            if f_b < f_x {
                // Found a point with lower function value, expand further
                step *= 2.0;
                b_init = (b_init + step).min(max_bound);
            } else {
                break;
            }
        }
        b_init = b_init.min(max_bound);

        // Ensure we have a valid interval
        if b_init <= a_init {
            // If expansion failed, use a default interval
            a_init = x - 5.0;
            b_init = x + 5.0;
        }

        (a_init, b_init)
    };

    // Golden section search
    let mut c = b - GOLDEN_RATIO * (b - a);
    let mut d = a + GOLDEN_RATIO * (b - a);
    let mut fc = fun(c);
    let mut fd = fun(d);

    let mut iter_count = 0;
    while (b - a).abs() > tol && iter_count < max_iter {
        if fc < fd {
            b = d;
            d = c;
            c = b - GOLDEN_RATIO * (b - a);
            fd = fc;
            fc = fun(c);
        } else {
            a = c;
            c = d;
            d = a + GOLDEN_RATIO * (b - a);
            fc = fd;
            fd = fun(d);
        }
        iter_count += 1;
    }

    // Final solution is the midpoint of the interval
    let x_min = (a + b) / 2.0;
    let f_min = fun(x_min);

    Ok(OptimizationResult {
        x: x_min,
        fun: f_min,
        success: iter_count < max_iter,
        nit: iter_count,
    })
}
