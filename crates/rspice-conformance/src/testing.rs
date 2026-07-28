//! Ngspice conformance runner and process result protocol.

#[path = "suites/ngspice.rs"]
pub mod ngspice_runner;

pub use ngspice_runner::{
    AcSweepType, AnalysisSpec, TestResult, TestRunner, TestRunnerConfig, TestStatistics,
    ValueMismatch,
};

#[path = "suites/ngspice/codec.rs"]
mod result_codec;

pub use result_codec::{decode_test_result, encode_test_result};
