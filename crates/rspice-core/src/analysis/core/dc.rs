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
        let mut points = Vec::new();
        let mut v = self.start;

        if self.step > 0.0 {
            while v <= self.stop {
                points.push(v);
                v += self.step;
            }
        } else if self.step < 0.0 {
            while v >= self.stop {
                points.push(v);
                v += self.step;
            }
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
}
