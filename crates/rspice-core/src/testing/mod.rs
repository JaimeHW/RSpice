//! Testing utilities for RSpice validation
//!
//! This module provides tools for automated testing and validation
//! of the simulator against reference implementations.

mod ngspice_runner;
mod result_codec;
mod xyce_runner;

pub use ngspice_runner::{
    AcSweepType, AnalysisSpec, TestResult, TestRunner, TestRunnerConfig, TestStatistics,
    ValueMismatch,
};
pub use result_codec::{decode_test_result, encode_test_result};
pub use xyce_runner::{
    XyceDeck, XyceDeckSection, XyceRunnerConfig, XyceStatistics, XyceTestResult, XyceTestRunner,
    XyceValueMismatch,
};
