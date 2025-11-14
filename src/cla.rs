//! Critical Line Algorithm implementation

use crate::error::ClaError;
use crate::first::init_algo;
use crate::types::{Frontier, FrontierPoint, TurningPoint};
use crate::{validate_bounds, validate_covariance_matrix, validate_dimensions, TOL_WEIGHTS};
use ndarray::{s, Array1, Array2};

/// Critical Line Algorithm for portfolio optimization
#[derive(Debug, Clone)]
pub struct Cla {
    mean: Array1<f64>,
    covariance: Array2<f64>,
    lower_bounds: Array1<f64>,
    upper_bounds: Array1<f64>,
    equality_constraints: Array2<f64>,
    equality_values: Array1<f64>,
    turning_points: Vec<TurningPoint>,
    tol: f64,
}

impl Cla {
    /// Create a new CLA instance and compute the efficient frontier
    ///
    /// This constructor validates inputs and automatically computes the entire
    /// efficient frontier by finding all turning points.
    pub fn new(
        mean: Array1<f64>,
        covariance: Array2<f64>,
        lower_bounds: Array1<f64>,
        upper_bounds: Array1<f64>,
        equality_constraints: Array2<f64>,
        equality_values: Array1<f64>,
    ) -> Result<Self, ClaError> {
        // Validate inputs
        let n = mean.len();
        validate_dimensions(n, covariance.dim(), lower_bounds.len(), upper_bounds.len())?;
        validate_bounds(
            lower_bounds.as_slice().unwrap(),
            upper_bounds.as_slice().unwrap(),
        )?;
        validate_covariance_matrix(&covariance)?;

        // Check for NaN or infinity in inputs
        for (i, &m) in mean.iter().enumerate() {
            if !m.is_finite() {
                return Err(ClaError::InvalidInput(format!(
                    "Mean return for asset {} is not finite: {}",
                    i, m
                )));
            }
        }

        for i in 0..lower_bounds.len() {
            if !lower_bounds[i].is_finite() {
                return Err(ClaError::InvalidInput(format!(
                    "Lower bound for asset {} is not finite: {}",
                    i, lower_bounds[i]
                )));
            }
            if !upper_bounds[i].is_finite() {
                return Err(ClaError::InvalidInput(format!(
                    "Upper bound for asset {} is not finite: {}",
                    i, upper_bounds[i]
                )));
            }
        }

        if equality_constraints.nrows() != equality_values.len() {
            return Err(ClaError::InvalidInput(format!(
                "Equality constraints matrix rows {} must match values length {}",
                equality_constraints.nrows(),
                equality_values.len()
            )));
        }

        if equality_constraints.ncols() != n {
            return Err(ClaError::InvalidInput(format!(
                "Equality constraints matrix cols {} must match mean length {}",
                equality_constraints.ncols(),
                n
            )));
        }

        let mut cla = Self {
            mean,
            covariance,
            lower_bounds,
            upper_bounds,
            equality_constraints,
            equality_values,
            turning_points: Vec::new(),
            tol: TOL_WEIGHTS,
        };

        // Compute efficient frontier
        cla.compute_frontier()?;

        Ok(cla)
    }

    /// Compute the efficient frontier by finding all turning points
    fn compute_frontier(&mut self) -> Result<(), ClaError> {
        let m = self.equality_constraints.nrows();
        let ns = self.mean.len();

        // Compute and store the first turning point
        let first_tp = self.first_turning_point()?;
        self.append_turning_point(&first_tp)?;

        let mut lam = f64::INFINITY;
        let mut last_alpha: Option<Array1<f64>> = None;
        let mut last_free: Option<Array1<bool>> = None;

        while lam > 0.0 {
            let last = self.turning_points.last().unwrap();

            // Identify active set
            let blocked = last.free().mapv(|f| !f);

            if blocked.iter().all(|&b| b) {
                return Err(ClaError::SingularMatrix(
                    "All variables cannot be blocked".to_string(),
                ));
            }

            // Determine which blocked assets are at upper/lower bounds
            let at_upper: Array1<bool> = blocked
                .iter()
                .zip(last.weights().iter())
                .zip(self.upper_bounds.iter())
                .map(|((&b, &w), &ub)| b && (w - ub).abs() < self.tol)
                .collect();

            let at_lower: Array1<bool> = blocked
                .iter()
                .zip(last.weights().iter())
                .zip(self.lower_bounds.iter())
                .map(|((&b, &w), &lb)| b && (w - lb).abs() < self.tol)
                .collect();

            let out = &at_upper | &at_lower;
            let in_set = out.mapv(|o| !o);

            // Construct RHS for KKT system
            let mut fixed_weights = Array1::zeros(ns);
            for i in 0..ns {
                if at_upper[i] {
                    fixed_weights[i] = self.upper_bounds[i];
                } else if at_lower[i] {
                    fixed_weights[i] = self.lower_bounds[i];
                }
            }

            let mut adjusted_mean = self.mean.clone();
            for i in 0..ns {
                if out[i] {
                    adjusted_mean[i] = 0.0;
                }
            }

            // Build free mask: [in_set (ns elements), all true (m elements)]
            let mut free_mask = Vec::with_capacity(ns + m);
            free_mask.extend(in_set.iter().copied());
            free_mask.extend(std::iter::repeat(true).take(m));

            // Build RHS vectors
            let mut rhs_alpha = Vec::with_capacity(ns + m);
            rhs_alpha.extend(fixed_weights.iter().copied());
            rhs_alpha.extend(self.equality_values.iter().copied());

            let mut rhs_beta = Vec::with_capacity(ns + m);
            rhs_beta.extend(adjusted_mean.iter().copied());
            rhs_beta.extend(std::iter::repeat(0.0).take(m));

            // Solve KKT system
            let (alpha, beta) =
                Self::solve_kkt(&self.kkt_matrix(), &rhs_alpha, &rhs_beta, &free_mask)?;

            // Save alpha for final point
            last_alpha = Some(alpha.clone());
            last_free = Some(last.free().clone());

            // Compute Lagrange multipliers and directional derivatives
            let gamma = self.proj_matrix().dot(&alpha);
            let delta = self.proj_matrix().dot(&beta) - &self.mean;

            // Compute event ratios
            let r_alpha = alpha.slice(s![..ns]).to_owned();
            let r_beta = beta.slice(s![..ns]).to_owned();

            let mut l_mat = Array2::from_elem((ns, 4), f64::NEG_INFINITY);

            // Event type 0: free asset hits upper bound (r_beta < -tol)
            for i in 0..ns {
                if in_set[i] && r_beta[i] < -self.tol {
                    l_mat[(i, 0)] = (self.upper_bounds[i] - r_alpha[i]) / r_beta[i];
                }
            }

            // Event type 1: free asset hits lower bound (r_beta > +tol)
            for i in 0..ns {
                if in_set[i] && r_beta[i] > self.tol {
                    l_mat[(i, 1)] = (self.lower_bounds[i] - r_alpha[i]) / r_beta[i];
                }
            }

            // Event type 2: blocked asset at upper bound becomes free (delta < -tol)
            for i in 0..ns {
                if at_upper[i] && delta[i] < -self.tol {
                    l_mat[(i, 2)] = -gamma[i] / delta[i];
                }
            }

            // Event type 3: blocked asset at lower bound becomes free (delta > +tol)
            for i in 0..ns {
                if at_lower[i] && delta[i] > self.tol {
                    l_mat[(i, 3)] = -gamma[i] / delta[i];
                }
            }

            // Determine next event (minimum positive lambda)
            let max_val = l_mat.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            if max_val < 0.0 {
                break;
            }

            // Find position of maximum
            let mut max_pos = (0, 0);
            let mut max_val_found = f64::NEG_INFINITY;
            for i in 0..ns {
                for j in 0..4 {
                    if l_mat[(i, j)] > max_val_found {
                        max_val_found = l_mat[(i, j)];
                        max_pos = (i, j);
                    }
                }
            }

            let (secchg, dirchg) = max_pos;
            lam = l_mat[(secchg, dirchg)];

            // Update free set
            let mut free = last.free().clone();
            free[secchg] = dirchg >= 2; // boundary → IN if dirchg in {2, 3}

            // Compute new turning point
            let new_weights = &r_alpha + &r_beta * lam;
            let new_tp = TurningPoint::new(new_weights, lam, free)?;
            self.append_turning_point(&new_tp)?;
        }

        // Final point at lambda = 0 (minimum variance portfolio)
        if let (Some(alpha), Some(free)) = (last_alpha, last_free) {
            let final_weights = alpha.slice(s![..ns]).to_owned();
            let final_tp = TurningPoint::new(final_weights, 0.0, free)?;
            self.append_turning_point(&final_tp)?;
        }

        Ok(())
    }

    /// Construct the projection matrix [covariance, A^T]
    fn proj_matrix(&self) -> Array2<f64> {
        let n = self.covariance.nrows();
        let m = self.equality_constraints.nrows();
        let mut proj = Array2::zeros((n, n + m));

        // Copy covariance matrix
        proj.slice_mut(s![.., ..n]).assign(&self.covariance);

        // Copy A^T (transpose of equality constraints)
        for i in 0..n {
            for j in 0..m {
                proj[(i, n + j)] = self.equality_constraints[(j, i)];
            }
        }

        proj
    }

    /// Construct the KKT matrix [[covariance, A^T], [A, 0]]
    fn kkt_matrix(&self) -> Array2<f64> {
        let n = self.covariance.nrows();
        let m = self.equality_constraints.nrows();
        let total_size = n + m;
        let mut kkt = Array2::zeros((total_size, total_size));

        // Top-left: covariance matrix
        kkt.slice_mut(s![..n, ..n]).assign(&self.covariance);

        // Top-right: A^T
        for i in 0..n {
            for j in 0..m {
                kkt[(i, n + j)] = self.equality_constraints[(j, i)];
            }
        }

        // Bottom-left: A
        kkt.slice_mut(s![n.., ..n])
            .assign(&self.equality_constraints);

        // Bottom-right: zeros (already zero)

        kkt
    }

    /// Solve the KKT system A x = b with some variables fixed
    ///
    /// This is a simplified solver using Gaussian elimination.
    /// For production, this should use ndarray-linalg when available.
    fn solve_kkt(
        a: &Array2<f64>,
        rhs_alpha: &[f64],
        rhs_beta: &[f64],
        free_mask: &[bool],
    ) -> Result<(Array1<f64>, Array1<f64>), ClaError> {
        let n = a.nrows();
        if n != a.ncols() {
            return Err(ClaError::InvalidInput(
                "KKT matrix must be square".to_string(),
            ));
        }

        let out: Vec<bool> = free_mask.iter().map(|&f| !f).collect();
        let mut x_alpha = Array1::zeros(n);
        let mut x_beta = Array1::zeros(n);

        // Set fixed variables
        for i in 0..n {
            if out[i] {
                x_alpha[i] = rhs_alpha[i];
                x_beta[i] = rhs_beta[i];
            }
        }

        // Build reduced system for free variables
        let free_indices: Vec<usize> = (0..n).filter(|&i| !out[i]).collect();
        let out_indices: Vec<usize> = (0..n).filter(|&i| out[i]).collect();

        if free_indices.is_empty() {
            return Ok((x_alpha, x_beta));
        }

        let k = free_indices.len();
        let mut reduced_a = Array2::zeros((k, k));
        let mut reduced_b_alpha = Array1::zeros(k);
        let mut reduced_b_beta = Array1::zeros(k);

        // Build reduced system
        for (ii, &i) in free_indices.iter().enumerate() {
            // Compute RHS: b[i] - sum(A[i, j] * x[j] for j in out)
            let mut b_alpha = rhs_alpha[i];
            let mut b_beta = rhs_beta[i];
            for &j in &out_indices {
                b_alpha -= a[(i, j)] * x_alpha[j];
                b_beta -= a[(i, j)] * x_beta[j];
            }
            reduced_b_alpha[ii] = b_alpha;
            reduced_b_beta[ii] = b_beta;

            // Build reduced matrix
            for (jj, &j) in free_indices.iter().enumerate() {
                reduced_a[(ii, jj)] = a[(i, j)];
            }
        }

        // Solve reduced system using Gaussian elimination
        let sol_alpha = Self::gaussian_elimination(&reduced_a, &reduced_b_alpha)?;
        let sol_beta = Self::gaussian_elimination(&reduced_a, &reduced_b_beta)?;

        // Fill in solution
        for (ii, &i) in free_indices.iter().enumerate() {
            x_alpha[i] = sol_alpha[ii];
            x_beta[i] = sol_beta[ii];
        }

        Ok((x_alpha, x_beta))
    }

    /// Simple Gaussian elimination solver
    /// TODO: Replace with ndarray-linalg when available
    fn gaussian_elimination(a: &Array2<f64>, b: &Array1<f64>) -> Result<Array1<f64>, ClaError> {
        let n = a.nrows();
        if n != a.ncols() || n != b.len() {
            return Err(ClaError::InvalidInput(
                "Matrix and vector dimensions must match".to_string(),
            ));
        }

        let mut aug = Array2::zeros((n, n + 1));
        aug.slice_mut(s![.., ..n]).assign(a);
        aug.column_mut(n).assign(b);

        // Forward elimination
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            let mut max_val = aug[(i, i)].abs();
            for k in (i + 1)..n {
                if aug[(k, i)].abs() > max_val {
                    max_val = aug[(k, i)].abs();
                    max_row = k;
                }
            }

            if max_val < 1e-10 {
                return Err(ClaError::SingularMatrix(
                    "Matrix is singular or near-singular".to_string(),
                ));
            }

            // Swap rows
            if max_row != i {
                for j in 0..=n {
                    aug.swap((i, j), (max_row, j));
                }
            }

            // Eliminate
            for k in (i + 1)..n {
                let factor = aug[(k, i)] / aug[(i, i)];
                for j in i..=n {
                    aug[(k, j)] -= factor * aug[(i, j)];
                }
            }
        }

        // Back substitution
        let mut x = Array1::zeros(n);
        for i in (0..n).rev() {
            x[i] = aug[(i, n)];
            for j in (i + 1)..n {
                x[i] -= aug[(i, j)] * x[j];
            }
            x[i] /= aug[(i, i)];
        }

        Ok(x)
    }

    /// Get the first turning point
    fn first_turning_point(&self) -> Result<TurningPoint, ClaError> {
        init_algo(
            self.mean.view(),
            self.lower_bounds.view(),
            self.upper_bounds.view(),
        )
    }

    /// Append a turning point with validation
    fn append_turning_point(&mut self, tp: &TurningPoint) -> Result<(), ClaError> {
        // Validate weights are within bounds
        for i in 0..tp.weights().len() {
            if tp.weights()[i] < self.lower_bounds[i] - self.tol {
                return Err(ClaError::InvalidInput(format!(
                    "Weight {} ({}) below lower bound {}",
                    i,
                    tp.weights()[i],
                    self.lower_bounds[i]
                )));
            }
            if tp.weights()[i] > self.upper_bounds[i] + self.tol {
                return Err(ClaError::InvalidInput(format!(
                    "Weight {} ({}) above upper bound {}",
                    i,
                    tp.weights()[i],
                    self.upper_bounds[i]
                )));
            }
        }

        // Validate weights sum to 1.0
        let sum: f64 = tp.weights().iter().sum();
        if (sum - 1.0).abs() > self.tol {
            return Err(ClaError::InvalidInput(format!(
                "Weights sum to {}, expected 1.0",
                sum
            )));
        }

        self.turning_points.push(tp.clone());
        Ok(())
    }

    /// Get the computed efficient frontier
    pub fn frontier(&self) -> Frontier {
        let points: Vec<FrontierPoint> = self
            .turning_points
            .iter()
            .map(|tp| FrontierPoint::new(tp.weights().clone()).unwrap())
            .collect();
        Frontier::new(self.mean.clone(), self.covariance.clone(), points)
    }

    /// Get the number of turning points
    pub fn len(&self) -> usize {
        self.turning_points.len()
    }

    /// Check if frontier is empty
    pub fn is_empty(&self) -> bool {
        self.turning_points.is_empty()
    }

    /// Get reference to turning points
    pub fn turning_points(&self) -> &[TurningPoint] {
        &self.turning_points
    }
}
