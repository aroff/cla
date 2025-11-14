//! Integration tests for minimize comparing with Python cvxcla

use rustcla::optimize::minimize;

#[test]
fn test_minimize_python_parity() {
    // Test case that should match Python cvxcla minimize behavior
    // Minimize a simple quadratic: f(x) = (x - 1.5)^2
    let result = minimize(|x| (x - 1.5).powi(2), 0.0, None, 1e-8, 200);

    assert!(result.is_ok(), "minimize should succeed");
    let opt_result = result.unwrap();
    assert!(opt_result.success, "Optimization should succeed");
    assert!(
        (opt_result.x - 1.5).abs() < 1e-6,
        "Should find minimum at x = 1.5"
    );
    assert!(
        opt_result.fun < 1e-10,
        "Function value should be near zero at minimum"
    );
}
