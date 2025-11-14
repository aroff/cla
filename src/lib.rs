//! RustCLA - Critical Line Algorithm for Portfolio Optimization
//!
//! This library implements the Critical Line Algorithm (CLA) for computing
//! the efficient frontier in portfolio optimization problems.
//!
//! # Example
//!
//! ```rust,no_run
//! use rustcla::Cla;
//! use ndarray::array;
//!
//! // Define portfolio problem
//! let mean = array![0.1, 0.2, 0.15];
//! let covariance = array![[0.01, 0.002, 0.003],
//!                         [0.002, 0.04, 0.005],
//!                         [0.003, 0.005, 0.009]];
//! let lower_bounds = array![0.0, 0.0, 0.0];
//! let upper_bounds = array![1.0, 1.0, 1.0];
//!
//! // Create CLA instance and compute frontier
//! let cla = Cla::new(
//!     mean,
//!     covariance,
//!     lower_bounds,
//!     upper_bounds,
//!     array![[1.0, 1.0, 1.0]],
//!     array![1.0],
//! ).unwrap();
//!
//! let frontier = cla.frontier();
//! ```

pub mod cla;
pub mod error;
pub mod first;
pub mod optimize;
pub mod types;

// Re-export main types
pub use cla::Cla;
pub use error::{validate_bounds, validate_covariance_matrix, validate_dimensions, ClaError};
pub use first::init_algo;
pub use optimize::minimize;
pub use types::{Frontier, FrontierPoint, TurningPoint};

/// Numerical tolerance constants
pub const TOL_WEIGHTS: f64 = 1e-5;
pub const TOL_RETURNS: f64 = 1e-6;

#[allow(dead_code)]
const _TOL_RETURNS_USED: f64 = TOL_RETURNS; // Suppress unused warning for now
