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
        crate::netlist::DcSweepSpec::linear(self.start, self.stop, self.step).points()
    }
}
