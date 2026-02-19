//! DC Operating Point and DC Sweep Analysis

use crate::Value;
use crate::analysis::AnalysisConfig;

/// DC Analysis engine
#[derive(Debug)]
pub struct DcAnalysis {
    config: AnalysisConfig,
}

impl DcAnalysis {
    pub fn new(config: AnalysisConfig) -> Self {
        Self { config }
    }

    /// Get config
    pub fn config(&self) -> &AnalysisConfig {
        &self.config
    }
}

impl Default for DcAnalysis {
    fn default() -> Self {
        Self::new(AnalysisConfig::default())
    }
}

/// DC sweep parameters
#[derive(Debug, Clone)]
#[allow(dead_code)] // Reserved for DC sweep analysis
pub struct DcSweep {
    /// Source to sweep
    pub source_name: String,
    /// Start value
    pub start: Value,
    /// Stop value
    pub stop: Value,
    /// Step size
    pub step: Value,
}

#[allow(dead_code)] // Reserved for DC sweep analysis
impl DcSweep {
    pub fn new(source_name: String, start: Value, stop: Value, step: Value) -> Self {
        Self {
            source_name,
            start,
            stop,
            step,
        }
    }

    /// Generate sweep points
    pub fn points(&self) -> Vec<Value> {
        if !self.start.is_finite()
            || !self.stop.is_finite()
            || !self.step.is_finite()
            || self.step == 0.0
        {
            return Vec::new();
        }
        if (self.stop > self.start && self.step < 0.0)
            || (self.stop < self.start && self.step > 0.0)
        {
            return Vec::new();
        }

        let mut points = Vec::new();
        let mut value = self.start;
        let eps = (self.step.abs() * 1e-9).max(1e-18);
        let mut guard = 0usize;
        const MAX_POINTS: usize = 2_000_000;

        let done = |x: Value| -> bool {
            if self.step > 0.0 {
                x > self.stop + eps
            } else {
                x < self.stop - eps
            }
        };

        while !done(value) {
            points.push(value);
            guard += 1;
            if guard >= MAX_POINTS {
                break;
            }
            value += self.step;
        }

        if points.is_empty() {
            points.push(self.start);
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dc_sweep_points() {
        let sweep = DcSweep::new("V1".to_string(), 0.0, 5.0, 1.0);
        let points = sweep.points();

        assert_eq!(points.len(), 6);
        assert_eq!(points[0], 0.0);
        assert_eq!(points[5], 5.0);
    }

    #[test]
    fn test_dc_sweep_points_includes_fractional_stop() {
        let sweep = DcSweep::new("V1".to_string(), 0.0, 2.0, 0.05);
        let points = sweep.points();

        assert_eq!(points.len(), 41);
        assert!((points[40] - 2.0).abs() <= 1e-10);
    }

    #[test]
    fn test_dc_sweep_points_descending_fractional() {
        let sweep = DcSweep::new("V1".to_string(), 0.0, -3.0, -0.05);
        let points = sweep.points();

        assert_eq!(points.len(), 61);
        assert!((points[0] - 0.0).abs() <= 1e-12);
        assert!((points[60] + 3.0).abs() <= 1e-10);
    }

    #[test]
    fn test_dc_sweep_points_reject_invalid_step_direction() {
        let sweep = DcSweep::new("V1".to_string(), 0.0, 1.0, -0.1);
        assert!(sweep.points().is_empty());
    }
}
