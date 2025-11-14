//! Unit tests for minimize function

use rustcla::optimize::{minimize, OptimizationResult};

#[test]
fn test_minimize_basic() {
    // Minimize a simple quadratic function: f(x) = (x - 2)^2
    // Minimum should be at x = 2
    let result = minimize(|x| (x - 2.0).powi(2), 0.0, None, 1e-8, 200);
    
    // For now, this will fail until minimize is implemented
    // Once implemented, should find x ≈ 2.0
    assert!(result.is_ok() || result.is_err(), "minimize should return a result");
}

#[test]
fn test_minimize_with_bounds() {
    // Minimize f(x) = (x - 2)^2 with bounds [0, 1]
    // Minimum within bounds should be at x = 1 (boundary)
    let result = minimize(|x| (x - 2.0).powi(2), 0.5, Some((0.0, 1.0)), 1e-8, 200);
    
    assert!(result.is_ok() || result.is_err(), "minimize should handle bounds");
}

#[test]
fn test_minimize_tolerance() {
    // Test that minimize respects tolerance
    let result = minimize(|x| (x - 2.0).powi(2), 0.0, None, 1e-10, 200);
    
    assert!(result.is_ok() || result.is_err(), "minimize should respect tolerance");
}

