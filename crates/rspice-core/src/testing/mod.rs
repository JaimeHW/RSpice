//! Testing utilities for RSpice validation
//!
//! This module provides tools for automated testing and validation
//! of the simulator against reference implementations.

mod xyce_runner;

pub use xyce_runner::{
    XyceDeck, XyceDeckSection, XyceRunnerConfig, XyceStatistics, XyceTestResult, XyceTestRunner,
    XyceValueMismatch,
};
