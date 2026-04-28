//! Corner Comparison View
//!
//! Side-by-side comparison of simulation results across process corners,
//! matching Cadence's corner analysis workflow.
//!
//! # Features
//!
//! - Matrix view of metrics across corners
//! - Highlight worst-case values
//! - Statistical summary (mean, min, max, std dev)
//! - Pass/fail status per corner

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Corner Definition
// =============================================================================

/// A process corner definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corner {
    /// Corner name (e.g., "TT", "FF", "SS", "SF", "FS")
    pub name: String,
    /// Description
    pub description: String,
    /// Temperature in Celsius
    pub temperature: i32,
    /// Supply voltage
    pub vdd: f64,
    /// NMOS corner (e.g., "typical", "fast", "slow")
    pub nmos: String,
    /// PMOS corner
    pub pmos: String,
    /// Whether this is the nominal/typical corner
    pub is_nominal: bool,
}

impl Default for Corner {
    fn default() -> Self {
        Self {
            name: "TT".to_string(),
            description: "Typical-Typical".to_string(),
            temperature: 27,
            vdd: 1.8,
            nmos: "typical".to_string(),
            pmos: "typical".to_string(),
            is_nominal: true,
        }
    }
}

impl Corner {
    /// Create a new corner
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Standard process corners
    pub fn standard_corners(vdd: f64) -> Vec<Corner> {
        vec![
            Corner {
                name: "TT".to_string(),
                description: "Typical-Typical".to_string(),
                temperature: 27,
                vdd,
                nmos: "typical".to_string(),
                pmos: "typical".to_string(),
                is_nominal: true,
            },
            Corner {
                name: "FF".to_string(),
                description: "Fast-Fast".to_string(),
                temperature: -40,
                vdd: vdd * 1.1,
                nmos: "fast".to_string(),
                pmos: "fast".to_string(),
                is_nominal: false,
            },
            Corner {
                name: "SS".to_string(),
                description: "Slow-Slow".to_string(),
                temperature: 125,
                vdd: vdd * 0.9,
                nmos: "slow".to_string(),
                pmos: "slow".to_string(),
                is_nominal: false,
            },
            Corner {
                name: "SF".to_string(),
                description: "Slow-Fast".to_string(),
                temperature: 27,
                vdd,
                nmos: "slow".to_string(),
                pmos: "fast".to_string(),
                is_nominal: false,
            },
            Corner {
                name: "FS".to_string(),
                description: "Fast-Slow".to_string(),
                temperature: 27,
                vdd,
                nmos: "fast".to_string(),
                pmos: "slow".to_string(),
                is_nominal: false,
            },
        ]
    }
}

// =============================================================================
// Corner Result
// =============================================================================

/// Result for a single metric in a single corner
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CornerResult {
    /// Measured value
    pub value: Option<f64>,
    /// Whether specification is met
    pub spec_pass: Option<bool>,
    /// Whether this is the worst-case corner for this metric
    pub is_worst: bool,
    /// Whether this is the best-case corner for this metric
    pub is_best: bool,
}

impl CornerResult {
    /// Create a result with a value
    pub fn with_value(value: f64) -> Self {
        Self {
            value: Some(value),
            ..Default::default()
        }
    }
}

// =============================================================================
// Corner Comparison Table
// =============================================================================

/// A comparison table row (one metric across all corners)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComparisonRow {
    /// Metric name
    pub metric_name: String,
    /// Unit string
    pub unit: String,
    /// Minimum specification
    pub spec_min: Option<f64>,
    /// Maximum specification
    pub spec_max: Option<f64>,
    /// Results per corner (keyed by corner name)
    pub results: HashMap<String, CornerResult>,
    /// Statistical min across corners
    pub stat_min: Option<f64>,
    /// Statistical max across corners
    pub stat_max: Option<f64>,
    /// Statistical mean across corners
    pub stat_mean: Option<f64>,
    /// Statistical standard deviation
    pub stat_std: Option<f64>,
}

impl ComparisonRow {
    /// Create a new comparison row
    pub fn new(metric_name: impl Into<String>, unit: impl Into<String>) -> Self {
        Self {
            metric_name: metric_name.into(),
            unit: unit.into(),
            ..Default::default()
        }
    }

    /// Add a corner result
    pub fn add_result(&mut self, corner: &str, value: f64) {
        self.results
            .insert(corner.to_string(), CornerResult::with_value(value));
    }

    /// Set specification limits
    pub fn with_spec(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.spec_min = min;
        self.spec_max = max;
        self
    }

    /// Compute statistics and mark worst/best cases
    pub fn compute_statistics(&mut self) {
        let values: Vec<f64> = self.results.values().filter_map(|r| r.value).collect();

        if values.is_empty() {
            return;
        }

        // Compute stats
        let n = values.len() as f64;
        let sum: f64 = values.iter().sum();
        self.stat_mean = Some(sum / n);

        self.stat_min = values.iter().copied().reduce(f64::min);
        self.stat_max = values.iter().copied().reduce(f64::max);

        if let Some(mean) = self.stat_mean {
            let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
            self.stat_std = Some(variance.sqrt());
        }

        // Mark worst/best cases
        if let (Some(min), Some(max)) = (self.stat_min, self.stat_max) {
            for result in self.results.values_mut() {
                if let Some(v) = result.value {
                    result.is_best = (v - max).abs() < 1e-12;
                    result.is_worst = (v - min).abs() < 1e-12;
                }
            }
        }

        // Check specs
        for result in self.results.values_mut() {
            if let Some(v) = result.value {
                let meets_min = self.spec_min.is_none_or(|min| v >= min);
                let meets_max = self.spec_max.is_none_or(|max| v <= max);
                result.spec_pass = Some(meets_min && meets_max);
            }
        }
    }

    /// Check if all corners pass specification
    pub fn all_pass(&self) -> bool {
        self.results.values().all(|r| r.spec_pass.unwrap_or(true))
    }

    /// Get the worst-case corner name
    pub fn worst_corner(&self) -> Option<&str> {
        self.results
            .iter()
            .find(|(_, r)| r.is_worst)
            .map(|(name, _)| name.as_str())
    }
}

// =============================================================================
// Corner Comparison View
// =============================================================================

/// Complete corner comparison view state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CornerComparisonView {
    /// View title
    pub title: String,
    /// Corners being compared
    pub corners: Vec<Corner>,
    /// Comparison rows (metrics)
    pub rows: Vec<ComparisonRow>,
    /// Whether to show statistics columns
    pub show_statistics: bool,
    /// Whether to highlight worst-case values
    pub highlight_worst: bool,
    /// Whether to show only failing metrics
    pub show_failures_only: bool,
}

impl CornerComparisonView {
    /// Create a new comparison view
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            show_statistics: true,
            highlight_worst: true,
            ..Default::default()
        }
    }

    /// Add a corner to compare
    pub fn add_corner(&mut self, corner: Corner) {
        self.corners.push(corner);
    }

    /// Initialize with standard corners
    pub fn with_standard_corners(mut self, vdd: f64) -> Self {
        self.corners = Corner::standard_corners(vdd);
        self
    }

    /// Add a metric row
    pub fn add_row(&mut self, row: ComparisonRow) {
        self.rows.push(row);
    }

    /// Get corner names
    pub fn corner_names(&self) -> Vec<&str> {
        self.corners.iter().map(|c| c.name.as_str()).collect()
    }

    /// Compute all statistics
    pub fn compute_all_statistics(&mut self) {
        for row in &mut self.rows {
            row.compute_statistics();
        }
    }

    /// Get visible rows (respecting filters)
    pub fn visible_rows(&self) -> Vec<&ComparisonRow> {
        if self.show_failures_only {
            self.rows.iter().filter(|r| !r.all_pass()).collect()
        } else {
            self.rows.iter().collect()
        }
    }

    /// Count passing/failing metrics
    pub fn pass_fail_counts(&self) -> (usize, usize) {
        let pass = self.rows.iter().filter(|r| r.all_pass()).count();
        let fail = self.rows.len() - pass;
        (pass, fail)
    }

    /// Check if all metrics pass across all corners
    pub fn all_pass(&self) -> bool {
        self.rows.iter().all(|r| r.all_pass())
    }

    /// Get worst-case summary
    pub fn worst_case_summary(&self) -> HashMap<String, Vec<String>> {
        let mut summary: HashMap<String, Vec<String>> = HashMap::new();
        for row in &self.rows {
            if let Some(corner) = row.worst_corner() {
                summary
                    .entry(corner.to_string())
                    .or_default()
                    .push(row.metric_name.clone());
            }
        }
        summary
    }

    /// Total row count
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.rows.clear();
        self.corners.clear();
    }
}

// =============================================================================
// Tests
// =============================================================================

