//! Testing utilities for RSpice validation
//!
//! This module provides tools for automated testing and validation
//! of the simulator against reference implementations.

mod ngspice_runner;
mod spectre_correlation;

pub use ngspice_runner::{
    AcSweepType, AnalysisSpec, TestResult, TestRunner, TestRunnerConfig, TestStatistics,
    ValueMismatch,
};
pub use spectre_correlation::{
    CorrelationTolerancePolicy, ScalarComparison, WaveformComparison, compare_scalar,
    compare_waveform,
};
