//! Ngspice Regression Test Runner
//!
//! Automated test runner that validates RSpice against ngspice
//! reference outputs. Supports DC, AC, and transient analysis with comprehensive
//! result comparison.
//!
//! # Architecture
//!
//! ```text
//! TestRunner
//!     â”œâ”€â”€ discover_tests()     - Find .cir files
//!     â”œâ”€â”€ run_test()           - Execute single test
//!     â”‚   â”œâ”€â”€ parse_analyses() - Detect .op, .dc, .tran, .ac
//!     â”‚   â”œâ”€â”€ execute_analysis() - Run appropriate simulation
//!     â”‚   â””â”€â”€ compare_results() - Validate against .out
//!     â””â”€â”€ run_suite()          - Run all tests in directory
//! ```

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use crate::abort_signal::AbortSignal;
use crate::engine::{ConvergenceConfig, SimulationConfig};
use crate::{Complex64, Engine, Netlist, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

mod dc_analyses;
mod directives;
mod discovery;
mod engines;
mod execution;
mod frequency_analyses;
mod manifest;
mod reference;
mod suite;
mod time_analyses;
mod validation;

// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
// Analysis Types
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

/// Parsed analysis directive from netlist
#[derive(Debug, Clone)]
pub enum AnalysisSpec {
    /// DC operating point (.op)
    DcOp,
    /// DC sweep (.dc source start stop step)
    DcSweep {
        source: String,
        start: Value,
        stop: Value,
        step: Value,
    },
    /// Two-dimensional DC sweep (.dc src1 s1 e1 st1 src2 s2 e2 st2)
    /// `inner_*` is the fast sweep axis (x-axis in ngspice tabular output).
    DcSweep2 {
        inner_source: String,
        inner_start: Value,
        inner_stop: Value,
        inner_step: Value,
        outer_source: String,
        outer_start: Value,
        outer_stop: Value,
        outer_step: Value,
    },
    /// Transient analysis (.tran tstep tstop [tstart] [tmax])
    Transient {
        tstep: Value,
        tstop: Value,
        tstart: Value,
        tmax: Option<Value>,
    },
    /// AC analysis (.ac dec|oct|lin points fstart fstop)
    Ac {
        sweep_type: AcSweepType,
        points: usize,
        fstart: Value,
        fstop: Value,
    },
    /// Pole-zero analysis (.pz in+ in- out+ out- cur|vol pol|zer|pz)
    PoleZero {
        input_pos: String,
        input_neg: Option<String>,
        output_pos: String,
        output_neg: Option<String>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
    },
    /// Noise analysis (.noise v(out[,ref]) input dec|oct|lin points fstart fstop)
    Noise {
        output_pos: String,
        output_neg: Option<String>,
        input_source: String,
        sweep_type: AcSweepType,
        points: usize,
        fstart: Value,
        fstop: Value,
    },
    /// Sensitivity analysis (.sens v(out[,ref]) [ac dec|oct|lin points fstart fstop])
    Sensitivity {
        output_pos: String,
        output_neg: Option<String>,
        sweep: Option<(AcSweepType, usize, Value, Value)>,
    },
    /// Transfer-function analysis (.tf v(out[,ref])|i(dev) source)
    TransferFunction {
        output: String,
        input_source: String,
    },
    /// Explicitly fail when a deck requests an analysis the regression harness
    /// does not know how to execute.
    Unsupported { directive: String },
}

/// AC sweep type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AcSweepType {
    Dec,
    Oct,
    Lin,
}

// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
// Test Results
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

/// Test result for a single circuit
#[derive(Debug)]
pub struct TestResult {
    /// Test name (filename without extension)
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Value mismatches found
    pub mismatches: Vec<ValueMismatch>,
    /// Execution time in milliseconds
    pub duration_ms: u128,
    /// Analysis type that was run
    pub analysis_type: Option<String>,
}

/// A mismatch between expected and actual values
#[derive(Debug, Clone)]
pub struct ValueMismatch {
    /// Time point (for transient) or frequency (for AC)
    pub x_value: f64,
    /// Node or variable name
    pub node: String,
    /// Expected value from reference
    pub expected: f64,
    /// Actual value from simulation
    pub actual: f64,
    /// Relative error (0.0 to 1.0)
    pub relative_error: f64,
}

#[derive(Debug, Clone, Default)]
struct ReferenceSeries {
    x: Vec<f64>,
    y: Vec<f64>,
}

#[derive(Debug, Clone, Default)]
struct ReferenceTable {
    x_name: String,
    variables: BTreeMap<String, ReferenceSeries>,
}

#[derive(Debug, Clone, Default)]
struct OpReference {
    node_voltages: HashMap<String, f64>,
    branch_currents: HashMap<String, f64>,
}

#[derive(Debug, Clone, Default)]
struct PzReference {
    poles: Vec<crate::analysis::pole_zero::Complex>,
    zeros: Vec<crate::analysis::pole_zero::Complex>,
    all: Vec<crate::analysis::pole_zero::Complex>,
}

#[derive(Debug, Clone, Default)]
struct TransferFunctionReference {
    transfer_function: Option<f64>,
    output_probe: Option<String>,
    input_source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AcProbe {
    Voltage {
        func: &'static str,
        node_pos: String,
        node_neg: Option<String>,
    },
    Current {
        func: &'static str,
        branch: String,
    },
}

#[derive(Debug, Clone, Copy)]
struct DeadlineAbort {
    start: Instant,
    deadline: Duration,
}

impl DeadlineAbort {
    fn new(start: Instant, timeout_ms: u128) -> Self {
        let millis = timeout_ms.min(u128::from(u64::MAX)) as u64;
        Self {
            start,
            deadline: Duration::from_millis(millis),
        }
    }
}

impl AbortSignal for DeadlineAbort {
    #[inline]
    fn is_aborted(&self) -> bool {
        self.start.elapsed() >= self.deadline
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationContract {
    Smoke,
    ScriptedControl,
}

impl ValidationContract {
    fn parse(token: &str) -> Option<Self> {
        match token.trim() {
            "smoke" => Some(Self::Smoke),
            "scripted_control" => Some(Self::ScriptedControl),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct LiveNgspiceReferenceConfig {
    source_root: PathBuf,
    ngspice_exe: PathBuf,
    timeout_ms: u128,
}

struct ReferenceOutput {
    content: String,
    description: String,
}

// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
// Configuration
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

/// Configuration for the test runner
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    /// Relative tolerance for value comparison (default: 1%)
    pub relative_tolerance: f64,
    /// Absolute tolerance for very small values (default: 1e-12)
    pub absolute_tolerance: f64,
    /// Maximum number of mismatches to report per test
    pub max_mismatches: usize,
    /// Skip tests that use unsupported features
    pub skip_unsupported: bool,
    /// Verbose output during test execution
    pub verbose: bool,
    /// Maximum time allowed per test in milliseconds (default: 5000ms = 5s)
    pub max_time_per_test_ms: u128,
}

impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
            relative_tolerance: 0.02, // 2%
            absolute_tolerance: 1e-12,
            max_mismatches: 10,
            skip_unsupported: false,
            verbose: false,
            max_time_per_test_ms: 30000, // 30 seconds max per test
        }
    }
}

// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
// Test Runner Implementation
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

/// Ngspice regression test runner
pub struct TestRunner {
    config: TestRunnerConfig,
    test_dir: PathBuf,
    validation_manifest_root: PathBuf,
    validation_manifest: HashMap<String, ValidationContract>,
    live_reference_config: Result<Option<LiveNgspiceReferenceConfig>, String>,
    live_reference_cache: Mutex<HashMap<PathBuf, Result<String, String>>>,
    live_raw_reference_cache: Mutex<HashMap<PathBuf, Result<Vec<ReferenceTable>, String>>>,
}

impl TestRunner {
    /// Create a new test runner for the given test directory
    pub fn new<P: AsRef<Path>>(test_dir: P, config: TestRunnerConfig) -> Self {
        let test_dir = test_dir
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| test_dir.as_ref().to_path_buf());
        let (validation_manifest_root, validation_manifest) =
            Self::load_validation_manifest(&test_dir);
        Self {
            config,
            validation_manifest_root,
            validation_manifest,
            test_dir,
            live_reference_config: Self::live_ngspice_reference_config_from_env(),
            live_reference_cache: Mutex::new(HashMap::new()),
            live_raw_reference_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Get the test runner configuration
    pub fn config(&self) -> &TestRunnerConfig {
        &self.config
    }
}

/// Aggregate test statistics
#[derive(Debug, Clone)]
pub struct TestStatistics {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub total_time_ms: u128,
}

impl TestStatistics {
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64 * 100.0
        }
    }
}

// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
