/// Monte Carlo variable summary in UI-friendly form.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableResult {
    /// Variable name (e.g., V(out), I(V1))
    pub name: String,
    /// Exact finite sample values in engine execution order.
    pub samples: Vec<f64>,
    /// Arithmetic mean over converged runs
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum observed value
    pub min: f64,
    /// Maximum observed value
    pub max: f64,
    /// Histogram counts for post-processing/visualization
    pub histogram: Vec<usize>,
    /// Histogram bin edges (length = histogram.len() + 1)
    pub bin_edges: Vec<f64>,
}
