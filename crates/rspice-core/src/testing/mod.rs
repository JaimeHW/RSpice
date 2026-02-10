//! Testing utilities for RSpice validation
//!
//! This module provides tools for automated testing and validation
//! of the simulator against reference implementations.

mod ngspice_runner;

pub use ngspice_runner::{
    AcSweepType, AnalysisSpec, TestResult, TestRunner, TestRunnerConfig, TestStatistics,
    ValueMismatch,
};
