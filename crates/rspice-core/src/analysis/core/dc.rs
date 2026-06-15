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
        let eps = (self.step.abs() * 1e-9).max(1e-18);
        let mut point_index = 0usize;
        const MAX_POINTS: usize = 2_000_000;

        let done = |x: Value| -> bool {
            if self.step > 0.0 {
                x > self.stop + eps
            } else {
                x < self.stop - eps
            }
        };

        loop {
            let value = self.start + self.step * point_index as Value;
            if done(value) {
                break;
            }

            let snapped_to_stop = (value - self.stop).abs() <= eps;
            points.push(if snapped_to_stop { self.stop } else { value });
            point_index += 1;
            if snapped_to_stop || point_index >= MAX_POINTS {
                break;
            }
        }

        if points.is_empty() {
            points.push(self.start);
        }

        points
    }
}
