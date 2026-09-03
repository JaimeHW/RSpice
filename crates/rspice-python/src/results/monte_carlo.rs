//! Monte Carlo results and their summary statistics.
//!
//! `MonteCarloResult` holds the per-run outcomes; `VariableStatistics`
//! summarizes one measured quantity across them. Statistics are computed over
//! the runs that actually converged, and the count of those is exposed, so a
//! yield figure can never be quoted without its sample size.

use super::*;

/// Statistics for a single output variable from Monte Carlo analysis
///
/// Example:
///     >>> stats = result.get_variable("V(out)")
///     >>> print(f"Mean: {stats.mean:.3f}, Std: {stats.std_dev:.3f}")
///     >>> print(f"3σ range: {stats.three_sigma_range}")
#[pyclass(name = "VariableStatistics", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyVariableStatistics {
    /// Variable name
    #[pyo3(get)]
    pub name: String,
    /// Computed mean
    #[pyo3(get)]
    pub mean: f64,
    /// Computed standard deviation
    #[pyo3(get)]
    pub std_dev: f64,
    /// Minimum value
    #[pyo3(get)]
    pub min: f64,
    /// Maximum value
    #[pyo3(get)]
    pub max: f64,
    /// All sampled values
    samples: Vec<f64>,
    /// Histogram bin counts
    histogram: Vec<usize>,
    /// Histogram bin edges
    bin_edges: Vec<f64>,
}

impl PyVariableStatistics {
    pub fn from_core(stats: &rspice_core::analysis::VariableStatistics) -> Self {
        Self {
            name: stats.name.clone(),
            mean: stats.mean,
            std_dev: stats.std_dev,
            min: stats.min,
            max: stats.max,
            samples: stats.samples.clone(),
            histogram: stats.histogram.clone(),
            bin_edges: stats.bin_edges.clone(),
        }
    }
}

#[pymethods]
impl PyVariableStatistics {
    /// Get all sampled values as NumPy array
    #[getter]
    fn samples<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.samples.to_pyarray(py)
    }

    /// Get histogram bin counts
    #[getter]
    fn histogram(&self) -> Vec<usize> {
        self.histogram.clone()
    }

    /// Get histogram bin edges
    #[getter]
    fn bin_edges<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.bin_edges.to_pyarray(py)
    }

    /// Get a specific percentile value (0-100, linear interpolation).
    ///
    /// Raises ValueError for a non-finite/out-of-range percentile or when no
    /// successful samples are available.
    fn percentile(&self, pct: f64) -> PyResult<f64> {
        if !pct.is_finite() || !(0.0..=100.0).contains(&pct) {
            return Err(crate::errors::value_error(format!(
                "percentile must be a finite number from 0 to 100, got {pct}"
            )));
        }
        if self.samples.is_empty() {
            return Err(crate::errors::value_error(
                "cannot compute a percentile without successful samples",
            ));
        }
        let mut sorted = self.samples.clone();
        if sorted.iter().any(|sample| !sample.is_finite()) {
            return Err(crate::errors::value_error(
                "cannot compute a percentile from non-finite samples",
            ));
        }
        sorted.sort_by(f64::total_cmp);
        let rank = (pct / 100.0) * (sorted.len() - 1) as f64;
        let lo = rank.floor() as usize;
        let hi = rank.ceil() as usize;
        let (Some(&low), Some(&high)) = (sorted.get(lo), sorted.get(hi)) else {
            return Err(crate::errors::value_error(
                "percentile rank fell outside the sample",
            ));
        };
        if lo == hi {
            Ok(low)
        } else {
            let frac = rank - lo as f64;
            Ok(low * (1.0 - frac) + high * frac)
        }
    }

    /// Get 3-sigma range (mean ± 3*std_dev) as tuple
    #[getter]
    fn three_sigma_range(&self) -> (f64, f64) {
        (
            self.mean - 3.0 * self.std_dev,
            self.mean + 3.0 * self.std_dev,
        )
    }

    /// Get coefficient of variation (std_dev / |mean|) as percentage.
    ///
    /// Returns None when the mean is exactly zero because the coefficient is
    /// undefined rather than zero.
    #[getter]
    fn cv_percent(&self) -> Option<f64> {
        if self.mean != 0.0 {
            Some((self.std_dev / self.mean.abs()) * 100.0)
        } else {
            None
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "VariableStatistics({}: mean={:.4e}, std={:.4e}, range=[{:.4e}, {:.4e}])",
            self.name, self.mean, self.std_dev, self.min, self.max
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        name: String,
        moments: [f64; 4],
        samples: Vec<f64>,
        histogram: Vec<usize>,
        bin_edges: Vec<f64>,
    ) -> Self {
        Self {
            name,
            mean: moments[0],
            std_dev: moments[1],
            min: moments[2],
            max: moments[3],
            samples,
            histogram,
            bin_edges,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (String, [f64; 4], Vec<f64>, Vec<usize>, Vec<f64>),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.name.clone(),
                [self.mean, self.std_dev, self.min, self.max],
                self.samples.clone(),
                self.histogram.clone(),
                self.bin_edges.clone(),
            ),
        ))
    }
}

/// Monte Carlo analysis results
///
/// Contains statistical results for all output variables from a Monte Carlo run.
///
/// Example:
///     >>> result = engine.run_monte_carlo(netlist, num_runs=1000, seed=42)
///     >>> v_out = result.get_variable("V(OUT)")
///     >>> print(f"V(out): {v_out.mean:.3f} ± {v_out.std_dev:.3f}V")
#[pyclass(name = "MonteCarloResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyMonteCarloResult {
    /// Number of runs completed
    #[pyo3(get)]
    pub num_runs: usize,
    /// Whether all runs converged
    #[pyo3(get)]
    pub all_converged: bool,
    /// Number of failed runs
    #[pyo3(get)]
    pub num_failures: usize,
    /// Statistics for all variables (internal storage)
    variables: std::collections::HashMap<String, PyVariableStatistics>,
}

impl PyMonteCarloResult {
    pub fn from_core(result: &rspice_core::analysis::MonteCarloResult) -> Self {
        let variables = result
            .variables
            .iter()
            .map(|(name, stats)| (name.clone(), PyVariableStatistics::from_core(stats)))
            .collect();

        Self {
            num_runs: result.num_runs,
            all_converged: result.all_converged,
            num_failures: result.num_failures,
            variables,
        }
    }

    /// Exact match first, then a case-insensitive scan.
    fn lookup(&self, name: &str) -> Option<PyVariableStatistics> {
        self.variables.get(name).cloned().or_else(|| {
            self.variables
                .iter()
                .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
                .map(|(_, stats)| stats.clone())
        })
    }
}

#[pymethods]
impl PyMonteCarloResult {
    /// Get statistics for a specific variable by name (case-insensitive)
    ///
    /// Raises:
    ///     KeyError: If no variable carries that name. Use `try_variable`
    ///               for a `None`-returning probe.
    fn get_variable(&self, name: &str) -> PyResult<PyVariableStatistics> {
        self.lookup(name).ok_or_else(|| {
            crate::errors::key_error(format!(
                "unknown Monte Carlo variable '{name}'; available: {}",
                self.variable_names().join(", ")
            ))
        })
    }

    /// Get statistics for a variable, or None when it was not recorded.
    fn try_variable(&self, name: &str) -> Option<PyVariableStatistics> {
        self.lookup(name)
    }

    /// Get all variable names
    #[getter]
    fn variable_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.variables.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get mean for a variable
    ///
    /// Raises:
    ///     KeyError: If no variable carries that name
    fn mean(&self, name: &str) -> PyResult<f64> {
        self.get_variable(name).map(|stats| stats.mean)
    }

    /// Get standard deviation for a variable
    ///
    /// Raises:
    ///     KeyError: If no variable carries that name
    fn std_dev(&self, name: &str) -> PyResult<f64> {
        self.get_variable(name).map(|stats| stats.std_dev)
    }

    /// Get min/max range as tuple
    ///
    /// Raises:
    ///     KeyError: If no variable carries that name
    fn range(&self, name: &str) -> PyResult<(f64, f64)> {
        self.get_variable(name).map(|stats| (stats.min, stats.max))
    }

    /// Look up a variable with `mc["V(OUT)"]`
    fn __getitem__(&self, name: &str) -> PyResult<PyVariableStatistics> {
        self.get_variable(name)
    }

    /// `"V(OUT)" in mc`
    fn __contains__(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    /// Get success rate as percentage
    #[getter]
    fn success_rate(&self) -> f64 {
        if self.num_runs > 0 {
            ((self.num_runs - self.num_failures) as f64 / self.num_runs as f64) * 100.0
        } else {
            0.0
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "MonteCarloResult(runs={}, failures={}, variables={})",
            self.num_runs,
            self.num_failures,
            self.variables.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        num_runs: usize,
        all_converged: bool,
        num_failures: usize,
        variables: Vec<(String, PyVariableStatistics)>,
    ) -> Self {
        Self {
            num_runs,
            all_converged,
            num_failures,
            variables: variables.into_iter().collect(),
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (usize, bool, usize, Vec<(String, PyVariableStatistics)>),
    )> {
        // Sorted so the payload is byte-stable between runs, which a HashMap
        // iteration order would not be.
        let mut variables: Vec<(String, PyVariableStatistics)> = self
            .variables
            .iter()
            .map(|(name, stats)| (name.clone(), stats.clone()))
            .collect();
        variables.sort_by(|left, right| left.0.cmp(&right.0));
        Ok((
            unpickler::<Self>(py)?,
            (
                self.num_runs,
                self.all_converged,
                self.num_failures,
                variables,
            ),
        ))
    }
}
