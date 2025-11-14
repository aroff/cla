//! Unit tests for minimize function

use rustcla::optimize::minimize;

#[test]
fn test_minimize_basic() {
    // Minimize a simple quadratic function: f(x) = (x - 2)^2
    // Minimum should be at x = 2
    let result = minimize(|x| (x - 2.0).powi(2), 0.0, None, 1e-8, 200);

    assert!(result.is_ok(), "minimize should succeed for basic case");
    let opt_result = result.unwrap();
    assert!(opt_result.success, "Optimization should succeed");
    assert!(
        (opt_result.x - 2.0).abs() < 1e-6,
        "Should find minimum near x = 2.0, got {}",
        opt_result.x
    );
    assert!(
        opt_result.fun < 1e-10,
        "Function value should be near zero at minimum"
    );
}

#[test]
fn test_minimize_with_bounds() {
    // Minimize f(x) = (x - 2)^2 with bounds [0, 1]
    // Minimum within bounds should be at x = 1 (boundary, closest to unconstrained minimum at 2)
    let result = minimize(|x| (x - 2.0).powi(2), 0.5, Some((0.0, 1.0)), 1e-8, 200);

    assert!(result.is_ok(), "minimize should handle bounds");
    let opt_result = result.unwrap();
    assert!(opt_result.success, "Optimization should succeed");
    assert!(
        opt_result.x >= 0.0 && opt_result.x <= 1.0,
        "Solution should respect bounds"
    );
    // Should be at or near the upper bound (1.0) since minimum is at 2.0
    assert!(
        opt_result.x > 0.9,
        "Should be near upper bound, got {}",
        opt_result.x
    );
}

#[test]
fn test_minimize_tolerance() {
    // Test that minimize respects tolerance
    let result = minimize(|x| (x - 2.0).powi(2), 0.0, None, 1e-10, 200);

    assert!(result.is_ok(), "minimize should respect tolerance");
    let opt_result = result.unwrap();
    assert!(opt_result.success, "Optimization should succeed");
    assert!(
        (opt_result.x - 2.0).abs() < 1e-8,
        "Should converge to high precision"
    );
}

#[test]
fn test_minimize_convergence() {
    // Test convergence for a simple function
    let result = minimize(|x| x * x, 10.0, None, 1e-8, 200);

    assert!(result.is_ok(), "minimize should converge");
    let opt_result = result.unwrap();
    assert!(opt_result.success, "Optimization should succeed");
    assert!(
        (opt_result.x - 0.0).abs() < 1e-6,
        "Should find minimum at x = 0"
    );
    assert!(opt_result.nit > 0, "Should perform iterations");
}
