//! Type definitions for portfolio optimization

use crate::error::ClaError;
use crate::TOL_WEIGHTS;
use ndarray::{Array1, Array2};

/// Represents a single portfolio on the efficient frontier
#[derive(Debug, Clone)]
pub struct FrontierPoint {
    weights: Array1<f64>,
}

impl FrontierPoint {
    /// Create a new FrontierPoint with validation
    pub fn new(weights: Array1<f64>) -> Result<Self, ClaError> {
        let sum: f64 = weights.iter().sum();
        if (sum - 1.0).abs() > TOL_WEIGHTS {
            return Err(ClaError::InvalidInput(format!(
                "Weights must sum to 1.0, got {}",
                sum
            )));
        }
        Ok(Self { weights })
    }

    /// Get the weights vector
    pub fn weights(&self) -> &Array1<f64> {
        &self.weights
    }

    /// Compute expected return given mean returns vector
    pub fn mean(&self, mean: &Array1<f64>) -> Result<f64, ClaError> {
        if mean.len() != self.weights.len() {
            return Err(ClaError::InvalidInput(format!(
                "Mean returns length {} does not match weights length {}",
                mean.len(),
                self.weights.len()
            )));
        }
        Ok(mean.dot(&self.weights))
    }

    /// Compute portfolio variance given covariance matrix
    pub fn variance(&self, covariance: &Array2<f64>) -> Result<f64, ClaError> {
        if covariance.nrows() != self.weights.len() || covariance.ncols() != self.weights.len() {
            return Err(ClaError::InvalidInput(format!(
                "Covariance matrix dimensions do not match weights length {}",
                self.weights.len()
            )));
        }
        // variance = weights^T @ covariance @ weights
        let temp = covariance.dot(&self.weights);
        Ok(self.weights.dot(&temp))
    }
}

/// Represents a corner of the efficient frontier where the set of free assets changes
#[derive(Debug, Clone)]
pub struct TurningPoint {
    weights: Array1<f64>,
    lambda: f64,
    free: Array1<bool>,
}

impl TurningPoint {
    /// Create a new TurningPoint with validation
    pub fn new(weights: Array1<f64>, lambda: f64, free: Array1<bool>) -> Result<Self, ClaError> {
        if weights.len() != free.len() {
            return Err(ClaError::InvalidInput(
                "Weights and free vectors must have same length".to_string(),
            ));
        }
        if !free.iter().any(|&f| f) {
            return Err(ClaError::SingularMatrix(
                "At least one asset must be marked as free".to_string(),
            ));
        }
        // Validate weights sum
        let sum: f64 = weights.iter().sum();
        if (sum - 1.0).abs() > TOL_WEIGHTS {
            return Err(ClaError::InvalidInput(format!(
                "Weights must sum to 1.0, got {}",
                sum
            )));
        }
        Ok(Self {
            weights,
            lambda,
            free,
        })
    }

    /// Get the weights vector
    pub fn weights(&self) -> &Array1<f64> {
        &self.weights
    }

    /// Get the lambda value
    pub fn lambda(&self) -> f64 {
        self.lambda
    }

    /// Get the free asset boolean vector
    pub fn free(&self) -> &Array1<bool> {
        &self.free
    }

    /// Returns indices of free assets
    pub fn free_indices(&self) -> Vec<usize> {
        self.free
            .iter()
            .enumerate()
            .filter_map(|(i, &is_free)| if is_free { Some(i) } else { None })
            .collect()
    }

    /// Returns indices of blocked assets
    pub fn blocked_indices(&self) -> Vec<usize> {
        self.free
            .iter()
            .enumerate()
            .filter_map(|(i, &is_free)| if !is_free { Some(i) } else { None })
            .collect()
    }

    /// Compute expected return (delegates to FrontierPoint logic)
    pub fn mean(&self, mean: &Array1<f64>) -> Result<f64, ClaError> {
        if mean.len() != self.weights.len() {
            return Err(ClaError::InvalidInput(format!(
                "Mean returns length {} does not match weights length {}",
                mean.len(),
                self.weights.len()
            )));
        }
        Ok(mean.dot(&self.weights))
    }

    /// Compute portfolio variance (delegates to FrontierPoint logic)
    pub fn variance(&self, covariance: &Array2<f64>) -> Result<f64, ClaError> {
        if covariance.nrows() != self.weights.len() || covariance.ncols() != self.weights.len() {
            return Err(ClaError::InvalidInput(format!(
                "Covariance matrix dimensions do not match weights length {}",
                self.weights.len()
            )));
        }
        let temp = covariance.dot(&self.weights);
        Ok(self.weights.dot(&temp))
    }
}

/// Represents the entire efficient frontier
#[derive(Debug, Clone)]
pub struct Frontier {
    mean: Array1<f64>,
    covariance: Array2<f64>,
    points: Vec<FrontierPoint>,
}

impl Frontier {
    /// Create a new Frontier
    pub fn new(mean: Array1<f64>, covariance: Array2<f64>, points: Vec<FrontierPoint>) -> Self {
        Self {
            mean,
            covariance,
            points,
        }
    }

    /// Get the number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if frontier is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get reference to points
    pub fn points(&self) -> &[FrontierPoint] {
        &self.points
    }

    /// Get reference to mean returns
    pub fn mean(&self) -> &Array1<f64> {
        &self.mean
    }

    /// Get reference to covariance matrix
    pub fn covariance(&self) -> &Array2<f64> {
        &self.covariance
    }

    /// Returns a matrix of weights, one row per point
    pub fn weights(&self) -> Array2<f64> {
        if self.points.is_empty() {
            return Array2::zeros((0, 0));
        }
        let n_points = self.points.len();
        let n_assets = self.points[0].weights().len();
        let mut weights_matrix = Array2::zeros((n_points, n_assets));
        for (i, point) in self.points.iter().enumerate() {
            for (j, &w) in point.weights().iter().enumerate() {
                weights_matrix[(i, j)] = w;
            }
        }
        weights_matrix
    }

    /// Returns a vector of expected returns for all points
    pub fn returns(&self) -> Result<Array1<f64>, ClaError> {
        let mut returns = Vec::with_capacity(self.points.len());
        for point in &self.points {
            returns.push(point.mean(&self.mean)?);
        }
        Ok(Array1::from_vec(returns))
    }

    /// Returns a vector of expected variances for all points
    pub fn variance(&self) -> Result<Array1<f64>, ClaError> {
        let mut variances = Vec::with_capacity(self.points.len());
        for point in &self.points {
            variances.push(point.variance(&self.covariance)?);
        }
        Ok(Array1::from_vec(variances))
    }

    /// Returns a vector of expected volatilities (standard deviations) for all points
    pub fn volatility(&self) -> Result<Array1<f64>, ClaError> {
        let variances = self.variance()?;
        Ok(variances.mapv(|v| v.sqrt()))
    }

    /// Returns a vector of Sharpe ratios for all points
    pub fn sharpe_ratio(&self) -> Result<Array1<f64>, ClaError> {
        let returns = self.returns()?;
        let volatilities = self.volatility()?;
        Ok(returns / volatilities)
    }

    /// Interpolate the frontier with additional points between existing points
    ///
    /// Creates a new Frontier with additional points interpolated between each pair
    /// of adjacent existing points using linear interpolation.
    ///
    /// # Arguments
    ///
    /// * `num` - Number of interpolation points between each pair of existing points
    ///
    /// # Returns
    ///
    /// A new Frontier with interpolated points
    pub fn interpolate(&self, num: usize) -> Self {
        if self.points.len() < 2 {
            return self.clone();
        }

        let mut interpolated_points = Vec::new();

        // Add first point
        if let Some(first) = self.points.first() {
            interpolated_points.push(first.clone());
        }

        // Interpolate between adjacent points
        for i in 0..(self.points.len() - 1) {
            let w_left = self.points[i].weights();
            let w_right = self.points[i + 1].weights();

            // Generate num-1 interpolated points between w_right and w_left
            for j in 1..num {
                let lambda = j as f64 / num as f64;
                // Interpolate: lambda * w_left + (1 - lambda) * w_right
                let interpolated_weights = w_right * (1.0 - lambda) + w_left * lambda;

                if let Ok(point) = FrontierPoint::new(interpolated_weights) {
                    interpolated_points.push(point);
                }
            }

            // Add the right point
            interpolated_points.push(self.points[i + 1].clone());
        }

        Self {
            mean: self.mean.clone(),
            covariance: self.covariance.clone(),
            points: interpolated_points,
        }
    }

    /// Compute the maximum Sharpe ratio portfolio
    ///
    /// This method finds the portfolio on the efficient frontier with the highest
    /// Sharpe ratio. It uses optimization to find the exact maximum between turning points.
    ///
    /// # Returns
    ///
    /// A tuple of (max_sharpe_ratio, weights) or an error if computation fails
    pub fn max_sharpe(&self) -> Result<(f64, Array1<f64>), ClaError> {
        use crate::optimize::minimize;

        if self.points.is_empty() {
            return Err(ClaError::InvalidInput("Frontier is empty".to_string()));
        }

        let sharpe_ratios = self.sharpe_ratio()?;
        let weights_matrix = self.weights();

        // Find the point with maximum Sharpe ratio
        let mut max_idx = 0;
        let mut max_sharpe = sharpe_ratios[0];
        for (i, &sr) in sharpe_ratios.iter().enumerate() {
            if sr > max_sharpe {
                max_sharpe = sr;
                max_idx = i;
            }
        }

        // Look to the left and right of the max point to find the true maximum
        let right_idx = (max_idx + 1).min(self.points.len() - 1);
        let left_idx = max_idx.saturating_sub(1);

        let mut best_sharpe = max_sharpe;
        let mut best_weights = self.points[max_idx].weights().clone();

        // Optimize to the right
        if right_idx > max_idx {
            let w_left = weights_matrix.row(max_idx).to_owned();
            let w_right = weights_matrix.row(right_idx).to_owned();

            // Negative Sharpe ratio function for minimization
            let neg_sharpe = |alpha: f64| -> f64 {
                let weight = &w_left * alpha + &w_right * (1.0 - alpha);
                let var = weight.dot(&self.covariance.dot(&weight));
                let returns = self.mean.dot(&weight);
                -returns / var.sqrt()
            };

            if let Ok(opt_result) = minimize(neg_sharpe, 0.5, Some((0.0, 1.0)), 1e-8, 200) {
                let alpha = opt_result.x;
                let w_opt = &w_left * alpha + &w_right * (1.0 - alpha);
                let var_opt = w_opt.dot(&self.covariance.dot(&w_opt));
                let returns_opt = self.mean.dot(&w_opt);
                let sharpe_opt = returns_opt / var_opt.sqrt();

                if sharpe_opt > best_sharpe {
                    best_sharpe = sharpe_opt;
                    best_weights = w_opt;
                }
            }
        }

        // Optimize to the left
        if left_idx < max_idx {
            let w_left = weights_matrix.row(left_idx).to_owned();
            let w_right = weights_matrix.row(max_idx).to_owned();

            // Negative Sharpe ratio function for minimization
            let neg_sharpe = |alpha: f64| -> f64 {
                let weight = &w_left * alpha + &w_right * (1.0 - alpha);
                let var = weight.dot(&self.covariance.dot(&weight));
                let returns = self.mean.dot(&weight);
                -returns / var.sqrt()
            };

            if let Ok(opt_result) = minimize(neg_sharpe, 0.5, Some((0.0, 1.0)), 1e-8, 200) {
                let alpha = opt_result.x;
                let w_opt = &w_left * alpha + &w_right * (1.0 - alpha);
                let var_opt = w_opt.dot(&self.covariance.dot(&w_opt));
                let returns_opt = self.mean.dot(&w_opt);
                let sharpe_opt = returns_opt / var_opt.sqrt();

                if sharpe_opt > best_sharpe {
                    best_sharpe = sharpe_opt;
                    best_weights = w_opt;
                }
            }
        }

        Ok((best_sharpe, best_weights))
    }

    /// Plot the efficient frontier
    ///
    /// Creates a line plot of the efficient frontier with expected return on the y-axis
    /// and either variance or volatility on the x-axis.
    ///
    /// # Arguments
    ///
    /// * `volatility` - If true, plot volatility (standard deviation) on x-axis, otherwise variance
    /// * `markers` - If true, show markers at each point on the frontier
    /// * `output_path` - Optional path to save the plot as PNG. If None, plot is not saved.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustcla::Frontier;
    /// # let frontier = todo!();
    /// frontier.plot(true, true, Some("frontier.png")).unwrap();
    /// ```
    #[cfg(feature = "plotting")]
    pub fn plot(
        &self,
        volatility: bool,
        markers: bool,
        output_path: Option<&str>,
    ) -> Result<(), ClaError> {
        use plotters::prelude::*;

        let returns = self.returns()?;
        let x_data = if volatility {
            self.volatility()?
        } else {
            self.variance()?
        };

        let x_axis_title = if volatility {
            "Expected Volatility"
        } else {
            "Expected Variance"
        };

        // Determine output target
        let path = output_path.ok_or_else(|| {
            ClaError::InvalidInput("output_path must be provided to save plot".to_string())
        })?;

        let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();

        root.fill(&WHITE).map_err(|e| {
            ClaError::InvalidInput(format!("Failed to fill plot background: {:?}", e))
        })?;

        let x_min = x_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = x_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = returns.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = returns.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let mut chart = ChartBuilder::on(&root)
            .caption("Efficient Frontier", ("sans-serif", 40).into_font())
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| ClaError::InvalidInput(format!("Failed to build chart: {:?}", e)))?;

        chart
            .configure_mesh()
            .x_desc(x_axis_title)
            .y_desc("Expected Return")
            .draw()
            .map_err(|e| ClaError::InvalidInput(format!("Failed to draw mesh: {:?}", e)))?;

        let style = ShapeStyle::from(&BLUE).stroke_width(2);

        chart
            .draw_series(LineSeries::new(
                x_data.iter().zip(returns.iter()).map(|(&x, &y)| (x, y)),
                style,
            ))
            .map_err(|e| ClaError::InvalidInput(format!("Failed to draw line series: {:?}", e)))?;

        if markers {
            chart
                .draw_series(
                    x_data
                        .iter()
                        .zip(returns.iter())
                        .map(|(&x, &y)| Circle::new((x, y), 3, &BLUE)),
                )
                .map_err(|e| ClaError::InvalidInput(format!("Failed to draw markers: {:?}", e)))?;
        }

        root.present()
            .map_err(|e| ClaError::InvalidInput(format!("Failed to present plot: {:?}", e)))?;
        Ok(())
    }

    /// Plot the efficient frontier (no-op when plotting feature is disabled)
    #[cfg(not(feature = "plotting"))]
    pub fn plot(
        &self,
        _volatility: bool,
        _markers: bool,
        _output_path: Option<&str>,
    ) -> Result<(), ClaError> {
        Err(ClaError::InvalidInput(
            "Plotting feature is not enabled. Enable it with: cargo build --features plotting"
                .to_string(),
        ))
    }
}
