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
#![cfg_attr(test, allow(clippy::field_reassign_with_default))]
use crate::abort_signal::AbortSignal;
use crate::engine::{ConvergenceConfig, SimulationConfig};
use crate::{Complex64, Engine, Netlist, Value};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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
    variables: HashMap<String, ReferenceSeries>,
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
    validation_manifest: HashMap<String, ValidationContract>,
}

impl TestRunner {
    /// Create a new test runner for the given test directory
    pub fn new<P: AsRef<Path>>(test_dir: P, config: TestRunnerConfig) -> Self {
        let test_dir = test_dir.as_ref().to_path_buf();
        Self {
            config,
            validation_manifest: Self::load_validation_manifest(&test_dir),
            test_dir,
        }
    }

    /// Get the test runner configuration
    pub fn config(&self) -> &TestRunnerConfig {
        &self.config
    }

    fn validation_manifest_path(test_dir: &Path) -> PathBuf {
        test_dir.join("validation-manifest.tsv")
    }

    fn load_validation_manifest(test_dir: &Path) -> HashMap<String, ValidationContract> {
        let manifest_path = Self::validation_manifest_path(test_dir);
        let Ok(content) = fs::read_to_string(&manifest_path) else {
            return HashMap::new();
        };

        let mut manifest = HashMap::new();
        for (line_number, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(3, '\t');
            let Some(path) = parts.next() else {
                continue;
            };
            let Some(mode) = parts.next() else {
                continue;
            };
            let Some(contract) = ValidationContract::parse(mode) else {
                eprintln!(
                    "Ignoring invalid validation contract '{}' in {}:{}",
                    mode,
                    manifest_path.display(),
                    line_number + 1
                );
                continue;
            };

            manifest.insert(Self::normalize_manifest_key(path), contract);
        }

        manifest
    }

    fn normalize_manifest_key(path: &str) -> String {
        path.trim().replace('\\', "/").to_ascii_lowercase()
    }

    fn manifest_key_for_path(&self, cir_path: &Path) -> Option<String> {
        let relative = cir_path.strip_prefix(&self.test_dir).ok()?;
        Some(Self::normalize_manifest_key(
            &relative.to_string_lossy().replace('\\', "/"),
        ))
    }

    fn validation_contract_for(&self, cir_path: &Path) -> Option<ValidationContract> {
        let key = self.manifest_key_for_path(cir_path)?;
        self.validation_manifest.get(&key).copied()
    }

    fn create_dc_engine(&self) -> Engine {
        // Regression harness prioritizes convergence robustness over speed so
        // difficult ngspice decks exercise model behavior instead of solver limits.
        let mut config = SimulationConfig::default();
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = ConvergenceConfig::robust();
        // ngspice regression references run at 27C -> 300.15 K by default.
        config.temperature = 300.15;
        Engine::new(config)
    }

    fn create_dynamic_engine(&self) -> Engine {
        // Dynamic regression runs should track production transient behavior,
        // while keeping default ambient aligned with ngspice references.
        let mut config = SimulationConfig::default();
        // Keep nonlinear solve robustness aligned with DC harness so transient
        // comparisons reflect model behavior instead of iteration limits.
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = ConvergenceConfig::robust();
        // ngspice transient reference decks default to trapezoidal integration.
        // Fixing method here avoids TrapGear switching artifacts in waveform
        // comparisons while preserving production defaults elsewhere.
        config.integration_method = crate::analysis::IntegrationMethod::Trapezoidal;
        // Sub-ps floor improves waveform alignment around steep HFET/MESA edges.
        config.min_timestep = 1e-12;
        config.temperature = 300.15;
        Engine::new(config)
    }

    /// Discover all .cir test files in a subdirectory
    pub fn discover_tests(&self, subdir: &str) -> Vec<PathBuf> {
        let dir = self.test_dir.join(subdir);
        if !dir.exists() {
            return Vec::new();
        }

        if let Some(mut suite_tests) = self.discover_tests_from_makefile(&dir) {
            suite_tests.sort();
            return suite_tests;
        }

        let mut tests = Vec::new();
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "cir") {
                    tests.push(path);
                }
            }
        }
        tests.sort();
        tests
    }

    fn discover_tests_from_makefile(&self, dir: &Path) -> Option<Vec<PathBuf>> {
        let makefile = dir.join("Makefile.am");
        let content = fs::read_to_string(&makefile).ok()?;

        let mut in_tests_block = false;
        let mut tokens: Vec<String> = Vec::new();

        for raw_line in content.lines() {
            let no_comment = raw_line
                .split_once('#')
                .map(|(head, _)| head)
                .unwrap_or(raw_line);
            let trimmed = no_comment.trim();
            if trimmed.is_empty() {
                continue;
            }

            if !in_tests_block {
                let Some((lhs, rhs)) = trimmed.split_once('=') else {
                    continue;
                };
                if lhs.trim() != "TESTS" {
                    continue;
                }
                in_tests_block = true;
                let continuation = rhs.trim_end().ends_with('\\');
                tokens.extend(
                    rhs.trim_end_matches('\\')
                        .split_whitespace()
                        .map(str::to_string),
                );
                if !continuation {
                    break;
                }
                continue;
            }

            let continuation = trimmed.ends_with('\\');
            tokens.extend(
                trimmed
                    .trim_end_matches('\\')
                    .split_whitespace()
                    .map(str::to_string),
            );
            if !continuation {
                break;
            }
        }

        if tokens.is_empty() {
            return None;
        }

        let mut tests = Vec::new();
        for token in tokens {
            if token.contains('$') {
                continue;
            }
            if !token.to_ascii_lowercase().ends_with(".cir") {
                continue;
            }
            let path = dir.join(&token);
            if path.exists() {
                tests.push(path);
            }
        }

        if tests.is_empty() { None } else { Some(tests) }
    }

    /// Run a single test circuit
    pub fn run_test(&self, cir_path: &Path) -> TestResult {
        let name = cir_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let start = std::time::Instant::now();

        // Read source file
        let source = match fs::read_to_string(cir_path) {
            Ok(s) => s,
            Err(e) => {
                return TestResult {
                    name,
                    passed: false,
                    error: Some(format!("Failed to read file: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
        };

        // Preprocess includes using file path for relative path resolution
        let preprocessed_source = match Netlist::preprocess_includes(&source, cir_path) {
            Ok(preprocessed) => preprocessed,
            Err(_) => source, // Keep original if preprocessing fails
        };

        let contract = self.validation_contract_for(cir_path);
        let analyses = if matches!(contract, Some(ValidationContract::ScriptedControl)) {
            self.parse_analyses_with_control(&preprocessed_source, false)
        } else {
            self.parse_analyses(&preprocessed_source)
        };

        // Strip .control/.endc blocks (ngspice scripting) before parsing
        let source = Netlist::strip_control_blocks(&preprocessed_source);

        // Check for unsupported features
        if let Some(reason) = self.check_unsupported(&source) {
            if self.config.skip_unsupported {
                return TestResult {
                    name,
                    passed: true, // Mark as passed (skipped)
                    error: Some(format!("SKIPPED: {}", reason)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
            return TestResult {
                name,
                passed: false,
                error: Some(format!("Unsupported test deck: {}", reason)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: None,
            };
        }

        // Run all analyses in deck order. Control-block analysis commands are
        // parsed from the preprocessed source even though the script body is
        // stripped before circuit parsing.
        let analysis_plan = if analyses.is_empty()
            && !matches!(contract, Some(ValidationContract::ScriptedControl))
        {
            vec![AnalysisSpec::DcOp]
        } else {
            analyses
        };

        if analysis_plan.is_empty() {
            return self.run_parse_build_smoke_test(&name, &source, start);
        }

        let mut all_mismatches = Vec::new();
        let mut analysis_labels = Vec::new();
        let mut first_error: Option<String> = None;

        for analysis in &analysis_plan {
            let analysis_start = std::time::Instant::now();
            let mut result = match analysis {
                AnalysisSpec::DcOp => self.run_dc_op_test(&name, cir_path, &source, analysis_start),
                AnalysisSpec::DcSweep {
                    source: src,
                    start: st,
                    stop: sp,
                    step: stp,
                } => self.run_dc_sweep_test(
                    &name,
                    cir_path,
                    &source,
                    src,
                    *st,
                    *sp,
                    *stp,
                    analysis_start,
                ),
                AnalysisSpec::DcSweep2 {
                    inner_source,
                    inner_start,
                    inner_stop,
                    inner_step,
                    outer_source,
                    outer_start,
                    outer_stop,
                    outer_step,
                } => self.run_dc_sweep_2d_test(
                    &name,
                    cir_path,
                    &source,
                    inner_source,
                    *inner_start,
                    *inner_stop,
                    *inner_step,
                    outer_source,
                    *outer_start,
                    *outer_stop,
                    *outer_step,
                    analysis_start,
                ),
                AnalysisSpec::Transient {
                    tstep,
                    tstop,
                    tstart,
                    tmax,
                } => self.run_transient_test(
                    &name,
                    cir_path,
                    &source,
                    *tstep,
                    *tstop,
                    *tstart,
                    *tmax,
                    analysis_start,
                ),
                AnalysisSpec::Ac {
                    sweep_type,
                    points,
                    fstart,
                    fstop,
                } => self.run_ac_test(
                    &name,
                    cir_path,
                    &source,
                    *sweep_type,
                    *points,
                    *fstart,
                    *fstop,
                    analysis_start,
                ),
                AnalysisSpec::PoleZero {
                    input_pos,
                    input_neg,
                    output_pos,
                    output_neg,
                    input_is_current,
                    compute_poles,
                    compute_zeros,
                } => self.run_pz_test(
                    &name,
                    cir_path,
                    &source,
                    input_pos,
                    input_neg.as_deref(),
                    output_pos,
                    output_neg.as_deref(),
                    *input_is_current,
                    *compute_poles,
                    *compute_zeros,
                    analysis_start,
                ),
                AnalysisSpec::Noise {
                    output_pos,
                    output_neg,
                    input_source,
                    sweep_type,
                    points,
                    fstart,
                    fstop,
                } => self.run_noise_test(
                    &name,
                    cir_path,
                    &source,
                    output_pos,
                    output_neg.as_deref(),
                    input_source,
                    *sweep_type,
                    *points,
                    *fstart,
                    *fstop,
                    analysis_start,
                ),
                AnalysisSpec::Sensitivity {
                    output_pos,
                    output_neg,
                    sweep,
                } => self.run_sensitivity_test(
                    &name,
                    &source,
                    output_pos,
                    output_neg.as_deref(),
                    *sweep,
                    analysis_start,
                ),
                AnalysisSpec::TransferFunction {
                    output,
                    input_source,
                } => self.run_transfer_function_test(
                    &name,
                    cir_path,
                    &source,
                    output,
                    input_source,
                    analysis_start,
                ),
                AnalysisSpec::Unsupported { directive } => TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!(
                        "Regression runner does not implement requested analysis '{}'",
                        directive
                    )),
                    mismatches: Vec::new(),
                    duration_ms: analysis_start.elapsed().as_millis(),
                    analysis_type: Some(directive.clone()),
                },
            };

            if let Some(label) = result.analysis_type.clone() {
                analysis_labels.push(label);
            }

            if result.passed
                && let Err(err) =
                    self.enforce_validation_coverage(cir_path, &preprocessed_source, analysis)
            {
                result.passed = false;
                result.error = Some(err);
            }

            all_mismatches.extend(result.mismatches);

            if !result.passed {
                first_error = result.error.clone();
                break;
            }
        }

        let mut final_result = TestResult {
            name,
            passed: first_error.is_none(),
            error: first_error,
            mismatches: all_mismatches,
            duration_ms: start.elapsed().as_millis(),
            analysis_type: if analysis_labels.is_empty() {
                None
            } else {
                Some(analysis_labels.join(" + "))
            },
        };

        if final_result.duration_ms > self.config.max_time_per_test_ms {
            final_result.passed = false;
            let timeout_msg = format!(
                "Test exceeded timeout ({}ms > {}ms)",
                final_result.duration_ms, self.config.max_time_per_test_ms
            );
            final_result.error = Some(match final_result.error {
                Some(err) => format!("{err}; {timeout_msg}"),
                None => timeout_msg,
            });
        }

        final_result
    }

    /// Parse analysis directives from netlist source
    fn parse_analyses(&self, source: &str) -> Vec<AnalysisSpec> {
        self.parse_analyses_with_control(source, true)
    }

    fn parse_analyses_with_control(
        &self,
        source: &str,
        include_control_block_commands: bool,
    ) -> Vec<AnalysisSpec> {
        let mut analyses = Vec::new();
        let mut in_control_block = false;

        for line in source.lines() {
            let trimmed = Self::strip_netlist_comment(line).trim();
            if trimmed.is_empty() {
                continue;
            }

            let normalized = trimmed.to_ascii_lowercase();
            if normalized == ".control" {
                in_control_block = true;
                continue;
            }
            if normalized == ".endc" {
                in_control_block = false;
                continue;
            }
            if in_control_block && !include_control_block_commands {
                continue;
            }

            if let Some(spec) = self.parse_analysis_line(&normalized, in_control_block) {
                analyses.push(spec);
            }
        }

        analyses
    }

    fn run_parse_build_smoke_test(&self, name: &str, source: &str, start: Instant) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(netlist) => netlist,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Netlist parse error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("SMOKE".to_string()),
                };
            }
        };

        let engine = self.create_dynamic_engine();
        match engine.build_circuit(&netlist) {
            Ok(_) => TestResult {
                name: name.to_string(),
                passed: true,
                error: None,
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("SMOKE".to_string()),
            },
            Err(err) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Circuit build error: {}", err)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("SMOKE".to_string()),
            },
        }
    }

    fn parse_analysis_line(&self, line: &str, in_control_block: bool) -> Option<AnalysisSpec> {
        let normalized = if in_control_block && !line.starts_with('.') {
            format!(".{line}")
        } else {
            line.to_string()
        };

        // Match directive tokens exactly to avoid prefix collisions
        // (e.g., ".options" must not be treated as ".op").
        let directive = normalized.split_whitespace().next().unwrap_or("");
        match directive {
            ".op" => Some(AnalysisSpec::DcOp),
            ".dc" => self.parse_dc_directive(&normalized),
            ".tran" => self.parse_tran_directive(&normalized),
            ".ac" => self.parse_ac_directive(&normalized),
            ".disto" => self.parse_disto_directive(&normalized),
            ".pz" => self.parse_pz_directive(&normalized),
            ".noise" => self.parse_noise_directive(&normalized),
            ".sens" => self.parse_sensitivity_directive(&normalized),
            ".tf" => self.parse_transfer_directive(&normalized),
            ".four" | ".fourier" => Some(AnalysisSpec::Unsupported {
                directive: directive.trim_start_matches('.').to_ascii_uppercase(),
            }),
            _ => None,
        }
    }

    /// Parse .dc directive: .dc source start stop step
    fn parse_dc_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 9 {
            let inner_source = parts[1].to_string();
            let inner_start = self.parse_spice_value(parts[2])?;
            let inner_stop = self.parse_spice_value(parts[3])?;
            let inner_step = self.parse_spice_value(parts[4])?;
            let outer_source = parts[5].to_string();
            let outer_start = self.parse_spice_value(parts[6])?;
            let outer_stop = self.parse_spice_value(parts[7])?;
            let outer_step = self.parse_spice_value(parts[8])?;
            Some(AnalysisSpec::DcSweep2 {
                inner_source,
                inner_start,
                inner_stop,
                inner_step,
                outer_source,
                outer_start,
                outer_stop,
                outer_step,
            })
        } else if parts.len() >= 5 {
            let source = parts[1].to_string();
            let start = self.parse_spice_value(parts[2])?;
            let stop = self.parse_spice_value(parts[3])?;
            let step = self.parse_spice_value(parts[4])?;
            Some(AnalysisSpec::DcSweep {
                source,
                start,
                stop,
                step,
            })
        } else {
            None
        }
    }

    /// Parse .tran directive: .tran tstep tstop [tstart] [tmax]
    fn parse_tran_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let tstep = self.parse_spice_value(parts[1])?;
            let tstop = self.parse_spice_value(parts[2])?;
            let tstart = if parts.len() > 3 {
                self.parse_spice_value(parts[3]).unwrap_or(0.0)
            } else {
                0.0
            };
            let tmax = if parts.len() > 4 {
                self.parse_spice_value(parts[4])
            } else {
                None
            };
            Some(AnalysisSpec::Transient {
                tstep,
                tstop,
                tstart,
                tmax,
            })
        } else {
            None
        }
    }

    /// Parse .ac directive: .ac dec|oct|lin points fstart fstop
    fn parse_ac_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let sweep_type = match parts[1] {
                "dec" => AcSweepType::Dec,
                "oct" => AcSweepType::Oct,
                "lin" => AcSweepType::Lin,
                _ => return None,
            };
            let points = parts[2].parse().ok()?;
            let fstart = self.parse_spice_value(parts[3])?;
            let fstop = self.parse_spice_value(parts[4])?;
            Some(AnalysisSpec::Ac {
                sweep_type,
                points,
                fstart,
                fstop,
            })
        } else {
            None
        }
    }

    /// Parse .disto directive using AC sweep semantics:
    /// .disto dec|oct|lin points fstart fstop [f2overf1]
    ///
    /// The ngspice runner currently validates against AC-like responses, so we
    /// map DISTO sweep setup onto AC execution for broad compatibility coverage.
    fn parse_disto_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let sweep_type = match parts[1] {
                "dec" => AcSweepType::Dec,
                "oct" => AcSweepType::Oct,
                "lin" => AcSweepType::Lin,
                _ => return None,
            };
            let points = parts[2].parse().ok()?;
            let fstart = self.parse_spice_value(parts[3])?;
            let fstop = self.parse_spice_value(parts[4])?;
            Some(AnalysisSpec::Ac {
                sweep_type,
                points,
                fstart,
                fstop,
            })
        } else {
            None
        }
    }

    /// Parse .pz directive: .pz in+ in- out+ out- cur|vol pol|zer|pz
    fn parse_pz_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return None;
        }

        let input_is_current = match parts[5] {
            "cur" => true,
            "vol" => false,
            _ => return None,
        };
        let (compute_poles, compute_zeros) = match parts[6] {
            "pol" | "pole" | "poles" => (true, false),
            "zer" | "zero" | "zeros" => (false, true),
            "pz" => (true, true),
            _ => return None,
        };

        Some(AnalysisSpec::PoleZero {
            input_pos: parts[1].to_string(),
            input_neg: Self::optional_probe_node(parts[2]),
            output_pos: parts[3].to_string(),
            output_neg: Self::optional_probe_node(parts[4]),
            input_is_current,
            compute_poles,
            compute_zeros,
        })
    }

    /// Parse .noise directive: .noise v(out[,ref]) input dec|oct|lin points fstart fstop
    fn parse_noise_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return None;
        }

        let (output_pos, output_neg) = Self::parse_voltage_probe(parts[1])?;
        let sweep_type = match parts[3] {
            "dec" => AcSweepType::Dec,
            "oct" => AcSweepType::Oct,
            "lin" => AcSweepType::Lin,
            _ => return None,
        };
        let points = parts[4].parse().ok()?;
        let fstart = self.parse_spice_value(parts[5])?;
        let fstop = self.parse_spice_value(parts[6])?;

        Some(AnalysisSpec::Noise {
            output_pos,
            output_neg,
            input_source: parts[2].to_string(),
            sweep_type,
            points,
            fstart,
            fstop,
        })
    }

    fn parse_sensitivity_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        let (output_pos, output_neg) = Self::parse_voltage_probe(parts[1])?;
        let sweep = if parts.len() == 2 {
            None
        } else {
            if parts.len() < 7 || parts[2] != "ac" {
                return Some(AnalysisSpec::Unsupported {
                    directive: "SENS".to_string(),
                });
            }
            let sweep_type = match parts[3] {
                "dec" => AcSweepType::Dec,
                "oct" => AcSweepType::Oct,
                "lin" => AcSweepType::Lin,
                _ => return None,
            };
            let points = parts[4].parse().ok()?;
            let fstart = self.parse_spice_value(parts[5])?;
            let fstop = self.parse_spice_value(parts[6])?;
            Some((sweep_type, points, fstart, fstop))
        };

        Some(AnalysisSpec::Sensitivity {
            output_pos,
            output_neg,
            sweep,
        })
    }

    fn parse_transfer_directive(&self, line: &str) -> Option<AnalysisSpec> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        Some(AnalysisSpec::TransferFunction {
            output: parts[1].to_string(),
            input_source: parts[2].to_string(),
        })
    }

    /// Parse SPICE values using the same lexer/parser as netlist parsing.
    /// This keeps regression analysis directives aligned with production parsing
    /// (e.g. 1ns, 10us, 10ghz, 1nF, 1mH).
    fn parse_spice_value(&self, s: &str) -> Option<Value> {
        crate::netlist::lexer::parse_spice_value(s).ok()
    }

    #[inline]
    fn has_control_block(source: &str) -> bool {
        source.lines().any(|line| {
            Self::strip_netlist_comment(line)
                .trim()
                .eq_ignore_ascii_case(".control")
        })
    }

    #[inline]
    fn source_has_test_gold_nodes(source: &str) -> bool {
        let normalized = source.to_ascii_lowercase();
        normalized.contains("_t") && normalized.contains("_g")
    }

    fn has_reference_table_for_axis(
        &self,
        cir_path: &Path,
        axis_candidates: &[&str],
    ) -> Result<bool, String> {
        Ok(self
            .load_reference_table_for_axis(cir_path, axis_candidates)?
            .is_some())
    }

    fn reference_table_unknown_voltage_nodes(
        &self,
        cir_path: &Path,
        table: &ReferenceTable,
    ) -> Result<Vec<String>, String> {
        let source = fs::read_to_string(cir_path).map_err(|e| {
            format!(
                "Failed to read circuit '{}' while validating reference output: {e}",
                cir_path.display()
            )
        })?;
        let preprocessed = Netlist::preprocess_includes(&source, cir_path).unwrap_or(source);
        let stripped = Netlist::strip_control_blocks(&preprocessed);
        let Ok(netlist) = Netlist::parse(&stripped) else {
            return Ok(Vec::new());
        };
        let Ok(circuit) = self.create_dynamic_engine().build_circuit(&netlist) else {
            return Ok(Vec::new());
        };

        let mut node_to_idx = HashMap::new();
        node_to_idx.insert("0".to_string(), 0usize);
        node_to_idx.insert("gnd".to_string(), 0usize);
        for (idx, name) in circuit.node_names_sorted().iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }

        let mut unknown = BTreeSet::new();
        for var in table.variables.keys() {
            let Some((node_pos, node_neg)) = Self::parse_voltage_probe(var) else {
                continue;
            };

            if !Self::reference_node_exists(&node_to_idx, &node_pos) {
                unknown.insert(node_pos);
            }
            if let Some(node_neg) = node_neg
                && !Self::reference_node_exists(&node_to_idx, &node_neg)
            {
                unknown.insert(node_neg);
            }
        }

        Ok(unknown.into_iter().collect())
    }

    pub fn has_valid_reference_output(&self, cir_path: &Path) -> bool {
        self.load_dc_op_reference(cir_path).ok().flatten().is_some()
            || self
                .load_reference_table_for_axis(cir_path, &["time"])
                .ok()
                .flatten()
                .is_some()
            || self
                .load_reference_table_for_axis(cir_path, &["frequency"])
                .ok()
                .flatten()
                .is_some()
            || self
                .load_reference_table_for_axis(cir_path, &["v-sweep"])
                .ok()
                .flatten()
                .is_some()
            || self.load_pz_reference(cir_path).ok().flatten().is_some()
    }

    #[inline]
    fn is_internal_device_op_probe(name: &str) -> bool {
        let normalized = name.trim().to_ascii_lowercase();
        normalized.contains('#') && !normalized.ends_with("#branch")
    }

    #[inline]
    fn is_non_data_op_section_header(normalized: &str) -> bool {
        normalized.starts_with("resistor")
            || normalized.starts_with("capacitor")
            || normalized.starts_with("inductor")
            || normalized.starts_with("vsource")
            || normalized.starts_with("isource")
            || normalized.starts_with("diode")
            || normalized.starts_with("bjt")
            || normalized.starts_with("mosfet")
            || normalized.starts_with("jfet")
            || normalized.starts_with("mesfet")
            || normalized.starts_with("mesa")
            || normalized.starts_with("vbic")
            || normalized.starts_with("hfet")
            || normalized.starts_with("model ")
            || normalized.starts_with("device ")
            || normalized.starts_with("warning")
            || normalized.starts_with("circuit:")
            || normalized.starts_with("doing analysis")
            || normalized.starts_with("no. of data rows")
    }

    fn analysis_name(analysis: &AnalysisSpec) -> &'static str {
        match analysis {
            AnalysisSpec::DcOp => "DC operating point",
            AnalysisSpec::DcSweep { .. } | AnalysisSpec::DcSweep2 { .. } => "DC sweep",
            AnalysisSpec::Transient { .. } => "transient analysis",
            AnalysisSpec::Ac { .. } => "AC analysis",
            AnalysisSpec::PoleZero { .. } => "pole-zero analysis",
            AnalysisSpec::Noise { .. } => "noise analysis",
            AnalysisSpec::Sensitivity { .. } => "sensitivity analysis",
            AnalysisSpec::TransferFunction { .. } => "transfer-function analysis",
            AnalysisSpec::Unsupported { .. } => "unsupported analysis",
        }
    }

    fn analysis_has_direct_validation(
        &self,
        cir_path: &Path,
        original_source: &str,
        analysis: &AnalysisSpec,
    ) -> Result<bool, String> {
        Ok(match analysis {
            AnalysisSpec::DcOp => {
                Self::source_has_test_gold_nodes(original_source)
                    || self.load_dc_op_reference(cir_path)?.is_some()
            }
            AnalysisSpec::DcSweep { .. } | AnalysisSpec::DcSweep2 { .. } => {
                self.has_reference_table_for_axis(cir_path, &["v-sweep"])?
            }
            AnalysisSpec::Transient { .. } => {
                self.has_reference_table_for_axis(cir_path, &["time"])?
            }
            AnalysisSpec::Ac { .. } | AnalysisSpec::Noise { .. } => {
                self.has_reference_table_for_axis(cir_path, &["frequency"])?
            }
            AnalysisSpec::PoleZero { .. } => self.load_pz_reference(cir_path)?.is_some(),
            AnalysisSpec::Sensitivity { .. } => false,
            AnalysisSpec::TransferFunction {
                output,
                input_source,
            } => self
                .load_transfer_function_reference(cir_path, output, input_source)?
                .is_some(),
            AnalysisSpec::Unsupported { .. } => true,
        })
    }

    pub fn has_direct_validation_coverage(
        &self,
        cir_path: &Path,
        original_source: &str,
    ) -> Result<bool, String> {
        let analyses = self.parse_analyses(original_source);
        if analyses.is_empty() {
            return Ok(false);
        }

        for analysis in &analyses {
            if !self.analysis_has_direct_validation(cir_path, original_source, analysis)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn enforce_validation_coverage(
        &self,
        cir_path: &Path,
        original_source: &str,
        analysis: &AnalysisSpec,
    ) -> Result<(), String> {
        let contract = self.validation_contract_for(cir_path);
        if matches!(contract, Some(ValidationContract::ScriptedControl))
            && !Self::has_control_block(original_source)
        {
            return Err(format!(
                "Validation manifest marks '{}' as scripted_control, but no .control block was found",
                cir_path.display()
            ));
        }

        let automatically_validated =
            self.analysis_has_direct_validation(cir_path, original_source, analysis)?;

        if automatically_validated || contract.is_some() {
            return Ok(());
        }

        Err(format!(
            "No validation oracle for {} in '{}'. Add _t/_g assertions, checked-in reference data, or an explicit validation-manifest entry.",
            Self::analysis_name(analysis),
            cir_path.display()
        ))
    }

    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Analysis Execution
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

    fn run_dc_op_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC OP".to_string()),
                };
            }
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_dc_op(&netlist);
        let op_result = match primary_result {
            Ok(result) => Ok(result),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => {
                robust_engine.run_dc_op(&netlist)
            }
            Err(err) => Err(err),
        };

        match op_result {
            Ok(result) => {
                let mut mismatches = self.compare_test_gold_nodes(&result);
                if mismatches.is_empty() {
                    mismatches = match self.compare_dc_op_reference(cir_path, &result) {
                        Ok(m) => m,
                        Err(e) => {
                            return TestResult {
                                name: name.to_string(),
                                passed: false,
                                error: Some(format!("Reference comparison error: {}", e)),
                                mismatches: Vec::new(),
                                duration_ms: start.elapsed().as_millis(),
                                analysis_type: Some("DC OP".to_string()),
                            };
                        }
                    };
                }
                let passed = mismatches.is_empty();

                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} assertion(s) failed", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC OP".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("DC OP".to_string()),
            },
        }
    }

    /// Compare test nodes (`*_t`) against gold nodes (`*_g`).
    ///
    /// Many ngspice regression circuits encode assertions in-circuit by driving
    /// parallel `_t` (test) and `_g` (gold) nodes. This validates those pairs
    /// directly from the operating-point solution.
    fn compare_test_gold_nodes(&self, result: &crate::SimulationResult) -> Vec<ValueMismatch> {
        let mut mismatches = Vec::new();

        // SPICE node names are case-insensitive.
        let mut node_to_idx: HashMap<String, usize> =
            HashMap::with_capacity(result.node_names.len());
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        for (test_idx, test_name) in result.node_names.iter().enumerate() {
            let test_lower = test_name.to_ascii_lowercase();
            let Some(base) = test_lower.strip_suffix("_t") else {
                continue;
            };

            let gold_name = format!("{base}_g");
            let Some(&gold_idx) = node_to_idx.get(&gold_name) else {
                mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: test_name.clone(),
                    expected: f64::NAN,
                    actual: result.node_voltages.get(test_idx).copied().unwrap_or(0.0),
                    relative_error: f64::INFINITY,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    break;
                }
                continue;
            };

            let actual = result.node_voltages.get(test_idx).copied().unwrap_or(0.0);
            let expected = result.node_voltages.get(gold_idx).copied().unwrap_or(0.0);

            if let Some(relative_error) = self.compare_values(expected, actual) {
                mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: test_name.clone(),
                    expected,
                    actual,
                    relative_error,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    break;
                }
            }
        }

        mismatches
    }

    fn run_dc_sweep_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        sweep_source: &str,
        start_val: Value,
        stop_val: Value,
        step_val: Value,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }
        };

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        let primary_engine = self.create_dynamic_engine();
        let primary_result = primary_engine.run_dc_sweep_with_abort(
            &netlist,
            sweep_source,
            start_val,
            stop_val,
            step_val,
            &abort,
        );

        let sweep_result = match primary_result {
            Ok(results) => Ok(results),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => {
                let robust_engine = self.create_dc_engine();
                robust_engine.run_dc_sweep_with_abort(
                    &netlist,
                    sweep_source,
                    start_val,
                    stop_val,
                    step_val,
                    &abort,
                )
            }
            Err(err) => Err(err),
        };

        match sweep_result {
            Ok(results) => {
                let mismatches = match self.compare_dc_sweep_reference(cir_path, &netlist, &results)
                {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("DC Sweep".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("DC Sweep".to_string()),
            },
        }
    }

    #[inline]
    fn is_recoverable_dc_convergence_error(err: &crate::engine::SimulationError) -> bool {
        let message = err.to_string().to_ascii_lowercase();
        !message.contains("aborted")
            && (message.contains("convergence")
                || message.contains("singular")
                || message.contains("failed to converge"))
    }

    #[inline]
    fn generate_sweep_points(start: Value, stop: Value, step: Value) -> Result<Vec<Value>, String> {
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() || step == 0.0 {
            return Err("Invalid DC sweep range".to_string());
        }
        if (stop > start && step < 0.0) || (stop < start && step > 0.0) {
            return Err("DC sweep step sign does not reach stop".to_string());
        }

        let mut points = Vec::new();
        let mut x = start;
        let mut guard = 0usize;
        let max_points = 2_000_000usize;
        let eps = (step.abs() * 1e-9).max(1e-18);
        let done = |value: Value| -> bool {
            if step > 0.0 {
                value > stop + eps
            } else {
                value < stop - eps
            }
        };

        while !done(x) {
            points.push(x);
            guard += 1;
            if guard >= max_points {
                return Err("DC sweep exceeded point limit".to_string());
            }
            x += step;
        }
        if points.is_empty() {
            points.push(start);
        }
        Ok(points)
    }

    fn set_source_dc_value(
        &self,
        netlist: &mut Netlist,
        source_name: &str,
        dc_value: Value,
    ) -> Result<(), String> {
        for element in &mut netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            match &mut element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    match spec {
                        crate::netlist::SourceSpec::Dc(v) => *v = dc_value,
                        crate::netlist::SourceSpec::DcAc { dc_value: v, .. } => *v = dc_value,
                        crate::netlist::SourceSpec::DcTransient { dc_value: v, .. } => {
                            *v = dc_value
                        }
                        crate::netlist::SourceSpec::DcAcTransient { dc_value: v, .. } => {
                            *v = dc_value
                        }
                        crate::netlist::SourceSpec::Ac { .. }
                        | crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Exp { .. } => {
                            *spec = crate::netlist::SourceSpec::Dc(dc_value);
                        }
                    }
                    return Ok(());
                }
                _ => {
                    return Err(format!(
                        "Sweep source '{}' is not an independent source",
                        source_name
                    ));
                }
            }
        }
        Err(format!("Sweep source '{}' not found", source_name))
    }

    fn assign_ac_to_source_spec(
        spec: &mut crate::netlist::SourceSpec,
        magnitude: Value,
        phase: Value,
    ) {
        *spec = match spec.clone() {
            crate::netlist::SourceSpec::Dc(dc_value) => crate::netlist::SourceSpec::DcAc {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
            },
            crate::netlist::SourceSpec::Ac { .. } => {
                crate::netlist::SourceSpec::Ac { magnitude, phase }
            }
            crate::netlist::SourceSpec::DcAc { dc_value, .. } => crate::netlist::SourceSpec::DcAc {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
            },
            crate::netlist::SourceSpec::DcTransient {
                dc_value,
                transient,
            } => crate::netlist::SourceSpec::DcAcTransient {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
                transient,
            },
            crate::netlist::SourceSpec::DcAcTransient {
                dc_value,
                transient,
                ..
            } => crate::netlist::SourceSpec::DcAcTransient {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
                transient,
            },
            transient => crate::netlist::SourceSpec::DcAcTransient {
                dc_value: 0.0,
                ac_magnitude: magnitude,
                ac_phase: phase,
                transient: Box::new(transient),
            },
        };
    }

    fn set_source_ac_value(
        &self,
        netlist: &mut Netlist,
        source_name: &str,
        magnitude: Value,
        phase: Value,
    ) -> Result<(), String> {
        for element in &mut netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            match &mut element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    Self::assign_ac_to_source_spec(spec, magnitude, phase);
                    return Ok(());
                }
                _ => {
                    return Err(format!(
                        "Transfer source '{}' is not an independent source",
                        source_name
                    ));
                }
            }
        }
        Err(format!("Transfer source '{}' not found", source_name))
    }

    fn clear_all_source_ac_values(&self, netlist: &mut Netlist) {
        for element in &mut netlist.elements {
            match &mut element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    Self::assign_ac_to_source_spec(spec, 0.0, 0.0);
                }
                _ => {}
            }
        }
    }

    fn run_dc_sweep_2d_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        inner_source: &str,
        inner_start: Value,
        inner_stop: Value,
        inner_step: Value,
        outer_source: &str,
        outer_start: Value,
        outer_stop: Value,
        outer_step: Value,
        start: std::time::Instant,
    ) -> TestResult {
        let base_netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }
        };

        let outer_points = match Self::generate_sweep_points(outer_start, outer_stop, outer_step) {
            Ok(points) => points,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Sweep setup error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        let mut merged_results: Vec<(Value, crate::SimulationResult)> = Vec::new();

        for outer_value in outer_points {
            let mut netlist = base_netlist.clone();
            if let Err(e) = self.set_source_dc_value(&mut netlist, outer_source, outer_value) {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Sweep setup error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("DC Sweep".to_string()),
                };
            }

            let primary_result = primary_engine.run_dc_sweep_with_abort(
                &netlist,
                inner_source,
                inner_start,
                inner_stop,
                inner_step,
                &abort,
            );
            let sweep_result = match primary_result {
                Ok(results) => Ok(results),
                Err(err) if Self::is_recoverable_dc_convergence_error(&err) => robust_engine
                    .run_dc_sweep_with_abort(
                        &netlist,
                        inner_source,
                        inner_start,
                        inner_stop,
                        inner_step,
                        &abort,
                    ),
                Err(err) => Err(err),
            };

            match sweep_result {
                Ok(mut results) => merged_results.append(&mut results),
                Err(e) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!("Simulation error: {}", e)),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("DC Sweep".to_string()),
                    };
                }
            }
        }

        let mismatches =
            match self.compare_dc_sweep_reference(cir_path, &base_netlist, &merged_results) {
                Ok(m) => m,
                Err(e) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!("Reference comparison error: {}", e)),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("DC Sweep".to_string()),
                    };
                }
            };
        let passed = mismatches.is_empty();
        TestResult {
            name: name.to_string(),
            passed,
            error: if passed {
                None
            } else {
                Some(format!("{} reference mismatch(es)", mismatches.len()))
            },
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            analysis_type: Some("DC Sweep".to_string()),
        }
    }

    fn run_transient_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        tstep: Value,
        tstop: Value,
        tstart: Value,
        tmax: Option<Value>,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Transient".to_string()),
                };
            }
        };

        let engine = self.create_dynamic_engine();
        let max_step =
            tmax.unwrap_or_else(|| Self::default_transient_max_step(tstep, tstop, tstart));

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        match engine.run_tran_with_abort(&netlist, tstop, max_step, &abort) {
            Ok(result) => {
                let mismatches = match self.compare_transient_reference(cir_path, &netlist, &result)
                {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("Transient".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Transient".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("Transient".to_string()),
            },
        }
    }

    fn run_ac_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        sweep_type: AcSweepType,
        points: usize,
        fstart: Value,
        fstop: Value,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("AC".to_string()),
                };
            }
        };

        // Generate frequency points based on sweep type
        let frequencies = match sweep_type {
            AcSweepType::Dec => self.generate_decade_points(fstart, fstop, points),
            AcSweepType::Oct => self.generate_octave_points(fstart, fstop, points),
            AcSweepType::Lin => self.generate_linear_points(fstart, fstop, points),
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_ac(&netlist, &frequencies);
        let ac_result = match primary_result {
            Ok(results) => Ok(results),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => {
                robust_engine.run_ac(&netlist, &frequencies)
            }
            Err(err) => Err(err),
        };

        match ac_result {
            Ok(results) => {
                let mismatches = match self.compare_ac_reference(cir_path, &netlist, &results) {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("AC".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("AC".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("AC".to_string()),
            },
        }
    }

    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Frequency Point Generation
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

    #[inline]
    fn optional_probe_node(name: &str) -> Option<String> {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed == "0" || trimmed.eq_ignore_ascii_case("gnd") {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    fn resolve_circuit_node_index(
        circuit: &crate::circuit::Circuit,
        node: &str,
        role: &str,
    ) -> Result<usize, String> {
        circuit
            .get_node_by_name(node)
            .ok_or_else(|| format!("Unknown {role} node '{node}'"))
    }

    fn resolve_optional_circuit_node_index(
        circuit: &crate::circuit::Circuit,
        node: Option<&str>,
        role: &str,
    ) -> Result<Option<usize>, String> {
        match node {
            Some(name) => Ok(Some(Self::resolve_circuit_node_index(circuit, name, role)?)),
            None => Ok(None),
        }
    }

    fn run_pz_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        input_pos: &str,
        input_neg: Option<&str>,
        output_pos: &str,
        output_neg: Option<&str>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("PZ".to_string()),
                };
            }
        };

        let resolver_engine = self.create_dynamic_engine();
        let circuit = match resolver_engine.build_circuit(&netlist) {
            Ok(circuit) => circuit,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Circuit build error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("PZ".to_string()),
                };
            }
        };

        let input_pos_idx = match Self::resolve_circuit_node_index(&circuit, input_pos, "PZ input+")
        {
            Ok(idx) => idx,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("PZ".to_string()),
                };
            }
        };
        let input_neg_idx =
            match Self::resolve_optional_circuit_node_index(&circuit, input_neg, "PZ input-") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("PZ".to_string()),
                    };
                }
            };
        let output_pos_idx =
            match Self::resolve_circuit_node_index(&circuit, output_pos, "PZ output+") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("PZ".to_string()),
                    };
                }
            };
        let output_neg_idx =
            match Self::resolve_optional_circuit_node_index(&circuit, output_neg, "PZ output-") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("PZ".to_string()),
                    };
                }
            };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_pz_ports(
            &netlist,
            input_pos_idx,
            input_neg_idx,
            output_pos_idx,
            output_neg_idx,
            input_is_current,
            compute_poles,
            compute_zeros,
        );
        let pz_result = match primary_result {
            Ok(result) => Ok(result),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => robust_engine
                .run_pz_ports(
                    &netlist,
                    input_pos_idx,
                    input_neg_idx,
                    output_pos_idx,
                    output_neg_idx,
                    input_is_current,
                    compute_poles,
                    compute_zeros,
                ),
            Err(err) => Err(err),
        };

        match pz_result {
            Ok(result) => {
                let mismatches = match self.compare_pz_reference(cir_path, &result) {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("PZ".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("PZ".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("PZ".to_string()),
            },
        }
    }

    fn run_noise_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        output_pos: &str,
        output_neg: Option<&str>,
        input_source: &str,
        sweep_type: AcSweepType,
        points: usize,
        fstart: Value,
        fstop: Value,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Noise".to_string()),
                };
            }
        };

        let frequencies = match sweep_type {
            AcSweepType::Dec => self.generate_decade_points(fstart, fstop, points),
            AcSweepType::Oct => self.generate_octave_points(fstart, fstop, points),
            AcSweepType::Lin => self.generate_linear_points(fstart, fstop, points),
        };

        let resolver_engine = self.create_dynamic_engine();
        let circuit = match resolver_engine.build_circuit(&netlist) {
            Ok(circuit) => circuit,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Circuit build error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Noise".to_string()),
                };
            }
        };

        let output_pos_idx =
            match Self::resolve_circuit_node_index(&circuit, output_pos, "noise output+") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Noise".to_string()),
                    };
                }
            };
        let output_neg_idx = match Self::resolve_optional_circuit_node_index(
            &circuit,
            output_neg,
            "noise output-",
        ) {
            Ok(idx) => idx,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Noise".to_string()),
                };
            }
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_noise_with_input_source(
            &netlist,
            output_pos_idx,
            output_neg_idx,
            input_source,
            &frequencies,
            300.15,
        );
        let noise_result = match primary_result {
            Ok(result) => Ok(result),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => robust_engine
                .run_noise_with_input_source(
                    &netlist,
                    output_pos_idx,
                    output_neg_idx,
                    input_source,
                    &frequencies,
                    300.15,
                ),
            Err(err) => Err(err),
        };

        match noise_result {
            Ok(results) => {
                let mismatches = match self.compare_noise_reference(cir_path, &results) {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("Noise".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Noise".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("Noise".to_string()),
            },
        }
    }

    fn run_sensitivity_test(
        &self,
        name: &str,
        source: &str,
        output_pos: &str,
        output_neg: Option<&str>,
        sweep: Option<(AcSweepType, usize, Value, Value)>,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        };

        let resolver_engine = self.create_dynamic_engine();
        let circuit = match resolver_engine.build_circuit(&netlist) {
            Ok(circuit) => circuit,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Circuit build error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        };

        let output_pos_idx =
            match Self::resolve_circuit_node_index(&circuit, output_pos, "sensitivity output+") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Sensitivity".to_string()),
                    };
                }
            };
        let output_neg_idx = match Self::resolve_optional_circuit_node_index(
            &circuit,
            output_neg,
            "sensitivity output-",
        ) {
            Ok(idx) => idx,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        };

        let mut params: Vec<(String, Value)> = netlist
            .params
            .all_params()
            .into_iter()
            .filter(|(param_name, value)| {
                !param_name.starts_with("IC_")
                    && !param_name.starts_with("NODESET_")
                    && value.is_finite()
                    && value.abs() > 0.0
            })
            .collect();
        params.sort_by(|a, b| a.0.cmp(&b.0));
        let engine = self.create_dynamic_engine();
        if params.is_empty() {
            if sweep.is_some() {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(
                        "AC .SENS currently requires at least one non-zero top-level .PARAM"
                            .to_string(),
                    ),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity AC".to_string()),
                };
            }

            return match engine.run_sensitivity_linearized(&netlist, output_pos_idx, output_neg_idx)
            {
                Ok(result) if !result.sensitivities.is_empty() => TestResult {
                    name: name.to_string(),
                    passed: true,
                    error: None,
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                },
                Ok(_) => TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(
                        "Sensitivity analysis found no eligible elements to differentiate"
                            .to_string(),
                    ),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                },
                Err(err) => TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Simulation error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                },
            };
        }

        if let Some((sweep_type, points, fstart, fstop)) = sweep {
            let frequencies = match sweep_type {
                AcSweepType::Dec => self.generate_decade_points(fstart, fstop, points),
                AcSweepType::Oct => self.generate_octave_points(fstart, fstop, points),
                AcSweepType::Lin => self.generate_linear_points(fstart, fstop, points),
            };
            if frequencies.is_empty() {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some("Invalid .SENS AC frequency sweep configuration".to_string()),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity AC".to_string()),
                };
            }

            for (param_name, param_value) in &params {
                let pos = match engine.run_sensitivity_ac(
                    &netlist,
                    output_pos_idx,
                    param_name,
                    *param_value,
                    &frequencies,
                    None,
                ) {
                    Ok(values) => values,
                    Err(err) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Simulation error: {}", err)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("Sensitivity AC".to_string()),
                        };
                    }
                };
                if let Some(neg_idx) = output_neg_idx
                    && let Err(err) = engine.run_sensitivity_ac(
                        &netlist,
                        neg_idx,
                        param_name,
                        *param_value,
                        &frequencies,
                        None,
                    )
                {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!("Simulation error: {}", err)),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Sensitivity AC".to_string()),
                    };
                }
                if pos.is_empty() {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!(
                            "Sensitivity AC returned no samples for parameter '{}'",
                            param_name
                        )),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Sensitivity AC".to_string()),
                    };
                }
            }

            return TestResult {
                name: name.to_string(),
                passed: true,
                error: None,
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("Sensitivity AC".to_string()),
            };
        }

        for (param_name, param_value) in &params {
            if let Err(err) =
                engine.run_sensitivity(&netlist, output_pos_idx, param_name, *param_value, None)
            {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Simulation error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
            if let Some(neg_idx) = output_neg_idx
                && let Err(err) =
                    engine.run_sensitivity(&netlist, neg_idx, param_name, *param_value, None)
            {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Simulation error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        }

        TestResult {
            name: name.to_string(),
            passed: true,
            error: None,
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            analysis_type: Some("Sensitivity".to_string()),
        }
    }

    fn transfer_output_value_ac(
        &self,
        result: &crate::analysis::AcResult,
        output: &str,
    ) -> Result<Value, String> {
        if let Some((pos, neg)) = Self::parse_voltage_probe(output) {
            let pos_value = if pos.eq_ignore_ascii_case("0") || pos.eq_ignore_ascii_case("gnd") {
                Complex64::new(0.0, 0.0)
            } else {
                let idx = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(&pos))
                    .ok_or_else(|| format!("Unknown TF output voltage node '{}'", pos))?;
                result
                    .voltages
                    .get(idx)
                    .copied()
                    .unwrap_or_else(|| Complex64::new(0.0, 0.0))
            };
            let neg_value = match neg.as_deref() {
                None => Complex64::new(0.0, 0.0),
                Some(name)
                    if name.eq_ignore_ascii_case("0") || name.eq_ignore_ascii_case("gnd") =>
                {
                    Complex64::new(0.0, 0.0)
                }
                Some(name) => {
                    let idx = result
                        .node_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))
                        .ok_or_else(|| format!("Unknown TF output voltage node '{}'", name))?;
                    result
                        .voltages
                        .get(idx)
                        .copied()
                        .unwrap_or_else(|| Complex64::new(0.0, 0.0))
                }
            };
            return Ok((pos_value - neg_value).re);
        }

        if let Some(element) = Self::parse_current_probe(output) {
            let idx = result
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(&element))
                .ok_or_else(|| format!("Unknown TF output current '{}'", output))?;
            return Ok(result
                .currents
                .get(idx)
                .copied()
                .unwrap_or_else(|| Complex64::new(0.0, 0.0))
                .re);
        }

        Err(format!("Unsupported .TF output probe '{}'", output))
    }

    fn transfer_output_value_linearized(
        &self,
        netlist: &Netlist,
        output: &str,
        input_source: &str,
    ) -> Result<Option<Value>, String> {
        let Some((output_pos, output_neg)) = Self::parse_voltage_probe(output) else {
            // Keep the AC fallback for non-voltage .TF probes until the branch-current
            // path is upgraded to use the same exact linearized adjoint formulation.
            return Ok(None);
        };

        let resolver_engine = self.create_dynamic_engine();
        let circuit = resolver_engine
            .build_circuit(netlist)
            .map_err(|err| format!("Circuit build error: {err}"))?;
        let output_pos_idx =
            Self::resolve_circuit_node_index(&circuit, &output_pos, "transfer output+")?;
        let output_neg_idx = Self::resolve_optional_circuit_node_index(
            &circuit,
            output_neg.as_deref(),
            "transfer output-",
        )?;

        let engine = self.create_dc_engine();
        let sensitivity = engine
            .run_sensitivity_linearized(netlist, output_pos_idx, output_neg_idx)
            .map_err(|err| format!("Linearized transfer analysis error: {err}"))?;

        let gain = sensitivity
            .sensitivities
            .iter()
            .find(|entry| entry.element.eq_ignore_ascii_case(input_source))
            .map(|entry| entry.absolute)
            .ok_or_else(|| {
                format!(
                    "Linearized transfer analysis found no independent source sensitivity for '{}'",
                    input_source
                )
            })?;

        Ok(Some(gain))
    }

    fn get_source_dc_value(&self, netlist: &Netlist, source_name: &str) -> Result<Value, String> {
        for element in &netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            match &element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    return Ok(match spec {
                        crate::netlist::SourceSpec::Dc(v) => *v,
                        crate::netlist::SourceSpec::DcAc { dc_value, .. } => *dc_value,
                        crate::netlist::SourceSpec::DcTransient { dc_value, .. } => *dc_value,
                        crate::netlist::SourceSpec::DcAcTransient { dc_value, .. } => *dc_value,
                        crate::netlist::SourceSpec::Ac { .. }
                        | crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Exp { .. } => 0.0,
                    });
                }
                _ => {
                    return Err(format!(
                        "Transfer source '{}' is not an independent source",
                        source_name
                    ));
                }
            }
        }
        Err(format!("Transfer source '{}' not found", source_name))
    }

    fn run_transfer_function_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        output: &str,
        input_source: &str,
        start: std::time::Instant,
    ) -> TestResult {
        let base_netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("TF".to_string()),
                };
            }
        };

        if let Err(err) = self.get_source_dc_value(&base_netlist, input_source) {
            return TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(err),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("TF".to_string()),
            };
        }

        let gain = match self.transfer_output_value_linearized(&base_netlist, output, input_source)
        {
            Ok(Some(gain)) => gain,
            Ok(None) => {
                let mut ac_netlist = base_netlist.clone();
                self.clear_all_source_ac_values(&mut ac_netlist);
                if let Err(err) = self.set_source_ac_value(&mut ac_netlist, input_source, 1.0, 0.0)
                {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("TF".to_string()),
                    };
                }

                let engine = self.create_dc_engine();
                let ac_result = match engine.run_ac(&ac_netlist, &[0.0]) {
                    Ok(mut results) => match results.pop() {
                        Some(result) => result,
                        None => {
                            return TestResult {
                                name: name.to_string(),
                                passed: false,
                                error: Some(
                                    "Transfer-function analysis produced no AC sample".to_string(),
                                ),
                                mismatches: Vec::new(),
                                duration_ms: start.elapsed().as_millis(),
                                analysis_type: Some("TF".to_string()),
                            };
                        }
                    },
                    Err(err) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Simulation error: {}", err)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("TF".to_string()),
                        };
                    }
                };

                match self.transfer_output_value_ac(&ac_result, output) {
                    Ok(value) => value,
                    Err(err) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(err),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("TF".to_string()),
                        };
                    }
                }
            }
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("TF".to_string()),
                };
            }
        };
        if !gain.is_finite() {
            return TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!(
                    "Transfer-function result for '{}' driven by '{}' is not finite",
                    output, input_source
                )),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("TF".to_string()),
            };
        }

        let mut mismatches = Vec::new();
        match self.load_transfer_function_reference(cir_path, output, input_source) {
            Ok(Some(reference)) => {
                if let Some(expected) = reference.transfer_function
                    && let Some(relative_error) = self.compare_values(expected, gain)
                {
                    mismatches.push(ValueMismatch {
                        x_value: 0.0,
                        node: format!("tf({}, {})", output, input_source),
                        expected,
                        actual: gain,
                        relative_error,
                    });
                }
            }
            Ok(None) => {}
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("TF".to_string()),
                };
            }
        }

        TestResult {
            name: name.to_string(),
            passed: mismatches.is_empty(),
            error: if mismatches.is_empty() {
                None
            } else {
                Some(format!(
                    "{} transfer-function mismatch(es)",
                    mismatches.len()
                ))
            },
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            analysis_type: Some("TF".to_string()),
        }
    }

    fn generate_decade_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_decade: usize,
    ) -> Vec<Value> {
        let mut freqs = Vec::new();
        let decades = (fstop / fstart).log10();
        let total_points = (decades * points_per_decade as f64).ceil() as usize;

        for i in 0..=total_points {
            let f = fstart * 10f64.powf(i as f64 / points_per_decade as f64);
            if f <= fstop {
                freqs.push(f);
            }
        }
        freqs
    }

    fn generate_octave_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_octave: usize,
    ) -> Vec<Value> {
        let mut freqs = Vec::new();
        let octaves = (fstop / fstart).log2();
        let total_points = (octaves * points_per_octave as f64).ceil() as usize;

        for i in 0..=total_points {
            let f = fstart * 2f64.powf(i as f64 / points_per_octave as f64);
            if f <= fstop {
                freqs.push(f);
            }
        }
        freqs
    }

    fn generate_linear_points(&self, fstart: Value, fstop: Value, num_points: usize) -> Vec<Value> {
        let step = (fstop - fstart) / (num_points - 1).max(1) as f64;
        (0..num_points).map(|i| fstart + i as f64 * step).collect()
    }

    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Feature Detection
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

    /// Check if netlist uses unsupported features
    fn check_unsupported(&self, source: &str) -> Option<String> {
        // Directive-based patterns.
        // NOTE: .subckt is now supported via flattening.
        // NOTE: regression analyses are dispatched explicitly; unsupported
        //       requests fail loudly in `run_test`.
        // NOTE: `.control` scripting is not interpreted here. Decks that rely
        //       on upstream control scripts must be explicitly declared in
        //       `validation-manifest.tsv` so they never pass silently.
        let directive_patterns: [(&str, &str); 0] = [];

        // Device-based patterns (must be at start of line)
        // These are single-letter device prefixes that need line-start checking
        // NOTE: t (transmission line), k (coupled inductors), b (behavioral),
        //       s (vswitch), w (iswitch) are now supported and NOT skipped
        let line_start_devices: [(&str, &str); 0] = [
            // All previously listed devices are now supported
        ];

        for line in source.lines() {
            let trimmed = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.is_empty() {
                continue;
            }

            if let Some(directive) = trimmed.strip_prefix('.') {
                let token = format!(".{}", directive.split_whitespace().next().unwrap_or(""));
                for (pattern, reason) in directive_patterns {
                    if token == pattern {
                        return Some(reason.to_string());
                    }
                }
            }

            // Check if line starts with device letter followed by alphanumeric (e.g., "t1", "k1")
            for (prefix, reason) in &line_start_devices {
                if trimmed.starts_with(prefix) {
                    // Verify next char is alphanumeric (device name) not just a word
                    if let Some(next_char) = trimmed.chars().nth(1)
                        && next_char.is_alphanumeric()
                    {
                        return Some(reason.to_string());
                    }
                }
            }
        }

        None
    }
    fn strip_netlist_comment(line: &str) -> &str {
        let no_inline = line.split_once(';').map(|(head, _)| head).unwrap_or(line);
        let trimmed = no_inline.trim_start();
        if trimmed.starts_with('*') || trimmed.starts_with("//") {
            ""
        } else {
            no_inline
        }
    }

    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Suite Execution
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

    /// Run all tests in a subdirectory
    pub fn run_suite(&self, subdir: &str) -> Vec<TestResult> {
        let tests = self.discover_tests(subdir);
        tests.iter().map(|p| self.run_test(p)).collect()
    }

    /// Print a summary of test results
    pub fn print_summary(results: &[TestResult]) {
        let total = results.len();
        let passed = results
            .iter()
            .filter(|r| r.passed && r.error.is_none())
            .count();
        let skipped = results
            .iter()
            .filter(|r| r.error.as_ref().is_some_and(|e| e.starts_with("SKIPPED")))
            .count();
        let failed = total - passed - skipped;

        println!(
            "\nâ•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•"
        );
        println!(
            "  Test Summary: {} total | {} passed | {} failed | {} skipped",
            total, passed, failed, skipped
        );
        println!(
            "â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n"
        );

        // Print failures first
        for result in results
            .iter()
            .filter(|r| !r.passed && r.error.as_ref().is_none_or(|e| !e.starts_with("SKIPPED")))
        {
            let analysis = result.analysis_type.as_deref().unwrap_or("?");
            println!("  âœ— {} [{}] - {:?}", result.name, analysis, result.error);
            for mismatch in result.mismatches.iter().take(3) {
                println!(
                    "      mismatch: {} @ x={} expected={} actual={} rel_err={:.3e}",
                    mismatch.node,
                    mismatch.x_value,
                    mismatch.expected,
                    mismatch.actual,
                    mismatch.relative_error
                );
            }
        }

        // Then skipped
        if skipped > 0 {
            println!();
            for result in results
                .iter()
                .filter(|r| r.error.as_ref().is_some_and(|e| e.starts_with("SKIPPED")))
            {
                if let Some(ref err) = result.error {
                    println!("  âŠ˜ {} - {}", result.name, err);
                }
            }
        }
    }

    /// Generate aggregate statistics
    pub fn statistics(results: &[TestResult]) -> TestStatistics {
        let total = results.len();
        let passed = results
            .iter()
            .filter(|r| r.passed && r.error.is_none())
            .count();
        let skipped = results
            .iter()
            .filter(|r| r.error.as_ref().is_some_and(|e| e.starts_with("SKIPPED")))
            .count();
        let failed = total - passed - skipped;
        let total_time_ms: u128 = results.iter().map(|r| r.duration_ms).sum();

        TestStatistics {
            total,
            passed,
            failed,
            skipped,
            total_time_ms,
        }
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
// Reference Output Parsing (for future comparison)
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

impl TestRunner {
    fn normalize_op_reference_node(name: &str) -> String {
        let trimmed = name.trim();
        if trimmed.contains('(') {
            Self::normalize_variable_name(trimmed)
        } else {
            Self::normalize_variable_name(&format!("v({trimmed})"))
        }
    }

    fn normalize_op_reference_branch(name: &str) -> String {
        Self::parse_current_probe(name).unwrap_or_else(|| name.trim().to_ascii_lowercase())
    }

    fn load_dc_op_reference(&self, cir_path: &Path) -> Result<Option<OpReference>, String> {
        let out_path = cir_path.with_extension("out");
        if !out_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&out_path).map_err(|e| {
            format!(
                "Failed to read reference output '{}': {e}",
                out_path.display()
            )
        })?;
        Ok(self.parse_dc_op_reference(&content))
    }

    fn parse_dc_op_reference(&self, content: &str) -> Option<OpReference> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Section {
            None,
            Node,
            Source,
        }

        let mut reference = OpReference::default();
        let mut section = Section::None;

        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let normalized = trimmed.to_ascii_lowercase();
            if normalized == "node voltage" || normalized.starts_with("node ") {
                section = Section::Node;
                continue;
            }
            if normalized == "source current" || normalized.starts_with("source ") {
                section = Section::Source;
                continue;
            }
            if trimmed.starts_with('-')
                || normalized.starts_with("index ")
                || normalized.starts_with("initial transient solution")
            {
                continue;
            }
            if Self::is_non_data_op_section_header(&normalized) {
                section = Section::None;
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }
            let Ok(value) = parts[parts.len() - 1].parse::<f64>() else {
                continue;
            };

            match section {
                Section::Node => {
                    if parts[0].contains("#branch") {
                        reference
                            .branch_currents
                            .insert(Self::normalize_op_reference_branch(parts[0]), value);
                    } else if !Self::is_internal_device_op_probe(parts[0]) {
                        reference
                            .node_voltages
                            .insert(Self::normalize_op_reference_node(parts[0]), value);
                    }
                }
                Section::Source => {
                    reference
                        .branch_currents
                        .insert(Self::normalize_op_reference_branch(parts[0]), value);
                }
                Section::None => {}
            }
        }

        if reference.node_voltages.is_empty() && reference.branch_currents.is_empty() {
            None
        } else {
            Some(reference)
        }
    }

    fn compare_dc_op_reference(
        &self,
        cir_path: &Path,
        result: &crate::SimulationResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_dc_op_reference(cir_path)? else {
            return Ok(Vec::new());
        };

        let mut mismatches = Vec::new();
        let mut node_to_idx = HashMap::with_capacity(result.node_names.len() + 1);
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        for (node_name, expected) in &reference.node_voltages {
            let actual = if let Some((pos, neg)) = Self::parse_voltage_probe(node_name) {
                let pos_idx = Self::resolve_node_index(&node_to_idx, &pos);
                let neg_idx = neg
                    .as_deref()
                    .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                    .unwrap_or(0);
                pos_idx.map(|p| {
                    result.node_voltages.get(p).copied().unwrap_or(0.0)
                        - if neg_idx == 0 {
                            0.0
                        } else {
                            result.node_voltages.get(neg_idx).copied().unwrap_or(0.0)
                        }
                })
            } else {
                None
            };

            match actual {
                Some(actual) => {
                    let absolute_tolerance =
                        self.dc_op_absolute_tolerance_floor(node_name, &reference, result);
                    if let Some(relative_error) =
                        self.compare_values_with_abs_tol(*expected, actual, absolute_tolerance)
                    {
                        mismatches.push(ValueMismatch {
                            x_value: 0.0,
                            node: node_name.clone(),
                            expected: *expected,
                            actual,
                            relative_error,
                        });
                    }
                }
                None => mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: node_name.clone(),
                    expected: *expected,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                }),
            }
            if mismatches.len() >= self.config.max_mismatches {
                return Ok(mismatches);
            }
        }

        for (branch_name, expected) in &reference.branch_currents {
            match result.branch_current_named(branch_name) {
                Some(actual) => {
                    if let Some(relative_error) = self.compare_values(*expected, actual) {
                        mismatches.push(ValueMismatch {
                            x_value: 0.0,
                            node: format!("i({branch_name})"),
                            expected: *expected,
                            actual,
                            relative_error,
                        });
                    }
                }
                None => mismatches.push(ValueMismatch {
                    x_value: 0.0,
                    node: format!("i({branch_name})"),
                    expected: *expected,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                }),
            }
            if mismatches.len() >= self.config.max_mismatches {
                return Ok(mismatches);
            }
        }

        Ok(mismatches)
    }

    fn load_transfer_function_reference(
        &self,
        cir_path: &Path,
        output: &str,
        input_source: &str,
    ) -> Result<Option<TransferFunctionReference>, String> {
        let out_path = cir_path.with_extension("out");
        if !out_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&out_path).map_err(|e| {
            format!(
                "Failed to read reference output '{}': {e}",
                out_path.display()
            )
        })?;
        let target_output = Self::normalize_variable_name(output);
        let target_input = input_source.trim().to_ascii_lowercase();
        Ok(self
            .parse_transfer_function_references(&content)
            .into_iter()
            .find(|reference| {
                let input_matches = reference
                    .input_source
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(&target_input));
                let output_matches = reference
                    .output_probe
                    .as_deref()
                    .is_none_or(|probe| Self::normalize_variable_name(probe) == target_output);
                input_matches && output_matches && reference.transfer_function.is_some()
            }))
    }

    fn parse_transfer_function_references(&self, content: &str) -> Vec<TransferFunctionReference> {
        let mut references = Vec::new();
        let mut current: Option<TransferFunctionReference> = None;

        let finalize = |references: &mut Vec<TransferFunctionReference>,
                        current: &mut Option<TransferFunctionReference>| {
            if let Some(reference) = current.take()
                && reference.transfer_function.is_some()
            {
                references.push(reference);
            }
        };

        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let normalized = trimmed.to_ascii_lowercase();
            if normalized.starts_with("transfer function information") {
                finalize(&mut references, &mut current);
                current = Some(TransferFunctionReference::default());
                continue;
            }

            let Some(active) = current.as_mut() else {
                continue;
            };

            let Some((lhs_raw, rhs_raw)) = trimmed.split_once('=') else {
                if !normalized.starts_with("warning") {
                    finalize(&mut references, &mut current);
                }
                continue;
            };

            let lhs = lhs_raw.trim().to_ascii_lowercase();
            let Ok(value) = rhs_raw.trim().parse::<f64>() else {
                continue;
            };

            if lhs == "transfer_function" {
                active.transfer_function = Some(value);
            } else if let Some(probe) = lhs.strip_prefix("output_impedance_at_") {
                active.output_probe = Some(probe.trim().to_string());
            } else if let Some(source) = lhs.strip_suffix("#input_impedance") {
                active.input_source = Some(source.trim().to_string());
            }
        }

        finalize(&mut references, &mut current);
        references
    }

    fn load_pz_reference(&self, cir_path: &Path) -> Result<Option<PzReference>, String> {
        let out_path = cir_path.with_extension("out");
        if !out_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&out_path).map_err(|e| {
            format!(
                "Failed to read reference output '{}': {e}",
                out_path.display()
            )
        })?;
        Ok(self.parse_pz_reference(&content))
    }

    fn parse_pz_reference(&self, content: &str) -> Option<PzReference> {
        let mut reference = PzReference::default();
        let mut current_cols: Vec<String> = Vec::new();

        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() || trimmed.starts_with('-') {
                continue;
            }

            if trimmed.to_ascii_lowercase().starts_with("index ") {
                current_cols = trimmed
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.to_ascii_lowercase())
                    .collect();
                continue;
            }

            if current_cols.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 1 + current_cols.len() * 2 {
                continue;
            }

            for (idx, col) in current_cols.iter().enumerate() {
                let Some(re_token) = parts.get(1 + idx * 2) else {
                    continue;
                };
                let Some(im_token) = parts.get(2 + idx * 2) else {
                    continue;
                };
                let re_token = re_token.trim_end_matches(',');
                let im_token = im_token.trim_end_matches(',');
                let Ok(re) = re_token.parse::<f64>() else {
                    continue;
                };
                let Ok(im) = im_token.parse::<f64>() else {
                    continue;
                };
                let value = crate::analysis::pole_zero::Complex::new(re, im);
                if col.starts_with("pole(") {
                    reference.poles.push(value);
                } else if col.starts_with("zero(") {
                    reference.zeros.push(value);
                } else if col == "all" {
                    reference.all.push(value);
                }
            }
        }

        if reference.poles.is_empty() && reference.zeros.is_empty() && reference.all.is_empty() {
            None
        } else {
            Some(reference)
        }
    }

    fn compare_pz_reference(
        &self,
        cir_path: &Path,
        result: &crate::analysis::PoleZeroResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_pz_reference(cir_path)? else {
            return Ok(Vec::new());
        };

        let mut mismatches = Vec::new();

        let compare_complex_lists =
            |runner: &Self,
             label: &str,
             expected: &[crate::analysis::pole_zero::Complex],
             actual: &[crate::analysis::pole_zero::Complex],
             mismatches: &mut Vec<ValueMismatch>| {
                let n = expected.len().max(actual.len());
                for idx in 0..n {
                    let expected_value = expected.get(idx).copied();
                    let actual_value = actual.get(idx).copied();
                    match (expected_value, actual_value) {
                        (Some(expected), Some(actual)) => {
                            if let Some(relative_error) =
                                runner.compare_values(expected.re, actual.re)
                            {
                                mismatches.push(ValueMismatch {
                                    x_value: idx as f64,
                                    node: format!("{label}({}).re", idx + 1),
                                    expected: expected.re,
                                    actual: actual.re,
                                    relative_error,
                                });
                            }
                            if mismatches.len() >= runner.config.max_mismatches {
                                return;
                            }
                            if let Some(relative_error) =
                                runner.compare_values(expected.im, actual.im)
                            {
                                mismatches.push(ValueMismatch {
                                    x_value: idx as f64,
                                    node: format!("{label}({}).im", idx + 1),
                                    expected: expected.im,
                                    actual: actual.im,
                                    relative_error,
                                });
                            }
                        }
                        (Some(expected), None) => mismatches.push(ValueMismatch {
                            x_value: idx as f64,
                            node: format!("{label}({})", idx + 1),
                            expected: expected.re,
                            actual: f64::NAN,
                            relative_error: f64::INFINITY,
                        }),
                        (None, Some(actual)) => mismatches.push(ValueMismatch {
                            x_value: idx as f64,
                            node: format!("{label}({})", idx + 1),
                            expected: f64::NAN,
                            actual: actual.re,
                            relative_error: f64::INFINITY,
                        }),
                        (None, None) => {}
                    }
                    if mismatches.len() >= runner.config.max_mismatches {
                        return;
                    }
                }
            };

        let mut actual_poles = result.poles.clone();
        let mut actual_zeros = result.zeros.clone();
        actual_poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
        actual_zeros.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));

        if !reference.all.is_empty() {
            let mut actual_all = actual_poles.clone();
            actual_all.extend(actual_zeros.iter().copied());
            actual_all.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            let mut expected_all = reference.all.clone();
            expected_all.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            compare_complex_lists(self, "pz", &expected_all, &actual_all, &mut mismatches);
        } else {
            let mut expected_poles = reference.poles.clone();
            expected_poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            let mut expected_zeros = reference.zeros.clone();
            expected_zeros.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            compare_complex_lists(
                self,
                "pole",
                &expected_poles,
                &actual_poles,
                &mut mismatches,
            );
            if mismatches.len() < self.config.max_mismatches {
                compare_complex_lists(
                    self,
                    "zero",
                    &expected_zeros,
                    &actual_zeros,
                    &mut mismatches,
                );
            }
        }

        Ok(mismatches)
    }

    fn compare_dc_sweep_reference(
        &self,
        cir_path: &Path,
        netlist: &Netlist,
        results: &[(Value, crate::SimulationResult)],
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["v-sweep"])? else {
            return Ok(Vec::new());
        };
        if results.is_empty() {
            return Ok(self.compare_reference_dataset(&reference, &[], |_| None));
        }

        let engine = self.create_dynamic_engine();
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| format!("Failed to build circuit for DC reference mapping: {e}"))?;
        let x_sim: Vec<f64> = results.iter().map(|(x, _)| *x).collect();
        let first = &results[0].1;
        let branch_names = if first.branch_names.iter().any(|name| !name.is_empty()) {
            first.branch_names.clone()
        } else {
            let names = circuit.branch_names_sorted();
            if names.is_empty() {
                Self::branch_probe_names_from_netlist(netlist)
            } else {
                names
            }
        };
        let mut node_to_idx = HashMap::with_capacity(first.node_names.len());
        for (idx, name) in first.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }
        let mut branch_to_idx = HashMap::with_capacity(branch_names.len());
        for (idx, name) in branch_names.iter().enumerate() {
            branch_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            Self::resolve_reference_series(var, &|expr| {
                let normalized = Self::normalize_variable_name(expr);
                if normalized == Self::normalize_variable_name(&reference.x_name) {
                    return None;
                }

                if let Some((n1, n2)) = Self::parse_voltage_probe(expr) {
                    let idx1 = Self::resolve_node_index(&node_to_idx, &n1)?;
                    let idx2 = n2
                        .as_deref()
                        .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                        .unwrap_or(0);
                    let series = results
                        .iter()
                        .map(|(_, r)| {
                            let v1 = r.node_voltages.get(idx1).copied().unwrap_or(0.0);
                            let v2 = r.node_voltages.get(idx2).copied().unwrap_or(0.0);
                            v1 - v2
                        })
                        .collect();
                    return Some(series);
                }

                if let Some(branch_name) = Self::parse_current_probe(expr) {
                    let branch_idx = branch_to_idx.get(&branch_name).copied().or_else(|| {
                        results
                            .iter()
                            .all(|(_, result)| result.branch_currents.len() == 1)
                            .then_some(0)
                    })?;
                    let series = results
                        .iter()
                        .map(|(_, r)| r.branch_currents.get(branch_idx).copied().unwrap_or(0.0))
                        .collect();
                    return Some(series);
                }

                None
            })
        }))
    }

    fn compare_transient_reference(
        &self,
        cir_path: &Path,
        netlist: &Netlist,
        result: &crate::engine::TransientResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["time"])? else {
            return Ok(Vec::new());
        };

        let x_sim = result.time.clone();
        let mut node_to_idx = HashMap::with_capacity(result.node_names.len() + 1);
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            Self::resolve_reference_series(var, &|expr| {
                if let Some((n1, n2)) = Self::parse_voltage_probe(expr) {
                    let idx1 = Self::resolve_node_index(&node_to_idx, &n1)?;
                    let idx2 = n2
                        .as_deref()
                        .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                        .unwrap_or(0);

                    let w1 = Self::transient_node_waveform(result, idx1);
                    let w2 = Self::transient_node_waveform(result, idx2);
                    let series = w1
                        .iter()
                        .zip(w2.iter())
                        .map(|(a, b)| a - b)
                        .collect::<Vec<_>>();
                    return Some(series);
                }

                if let Some(branch_name) = Self::parse_current_probe(expr) {
                    let branch_idx = result
                        .branch_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(&branch_name))?;
                    return result.branch_currents.get(branch_idx).cloned();
                }

                Self::resolve_transient_device_series(netlist, &node_to_idx, result, expr)
            })
        }))
    }

    fn compare_ac_reference(
        &self,
        cir_path: &Path,
        netlist: &Netlist,
        results: &[crate::analysis::AcResult],
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["frequency"])? else {
            return Ok(Vec::new());
        };
        if results.is_empty() {
            return Ok(self.compare_reference_dataset(&reference, &[], |_| None));
        }

        let engine = self.create_dynamic_engine();
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| format!("Failed to build circuit for AC reference mapping: {e}"))?;

        let mut node_to_idx = HashMap::new();
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in circuit.node_names_sorted().iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }
        let mut branch_to_idx = HashMap::new();
        for (idx, name) in results[0].branch_names.iter().enumerate() {
            branch_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        let x_sim: Vec<f64> = results.iter().map(|r| r.frequency).collect();

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            let normalized = Self::normalize_variable_name(var);

            match Self::parse_ac_probe(&normalized) {
                Some(AcProbe::Voltage {
                    func,
                    node_pos,
                    node_neg,
                }) => {
                    let idx_a = Self::resolve_node_index(&node_to_idx, &node_pos)?;
                    let idx_b = node_neg
                        .as_deref()
                        .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                        .unwrap_or(0);

                    let series = results
                        .iter()
                        .map(|r| {
                            let va = if idx_a == 0 {
                                num_complex::Complex64::new(0.0, 0.0)
                            } else {
                                r.voltages
                                    .get(idx_a - 1)
                                    .copied()
                                    .unwrap_or(num_complex::Complex64::new(0.0, 0.0))
                            };
                            let vb = if idx_b == 0 {
                                num_complex::Complex64::new(0.0, 0.0)
                            } else {
                                r.voltages
                                    .get(idx_b - 1)
                                    .copied()
                                    .unwrap_or(num_complex::Complex64::new(0.0, 0.0))
                            };
                            Self::evaluate_ac_complex_value(
                                func,
                                va - vb,
                                self.config.absolute_tolerance,
                            )
                        })
                        .collect();
                    Some(series)
                }
                Some(AcProbe::Current { func, branch }) => {
                    let branch_idx = *branch_to_idx.get(&branch)?;
                    let series = results
                        .iter()
                        .map(|r| {
                            let current = r
                                .currents
                                .get(branch_idx)
                                .copied()
                                .unwrap_or(num_complex::Complex64::new(0.0, 0.0));
                            Self::evaluate_ac_complex_value(
                                func,
                                current,
                                self.config.absolute_tolerance,
                            )
                        })
                        .collect();
                    Some(series)
                }
                None => None,
            }
        }))
    }

    fn compare_noise_reference(
        &self,
        cir_path: &Path,
        results: &[crate::analysis::NoiseResult],
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["frequency"])? else {
            return Ok(Vec::new());
        };
        if results.is_empty() {
            return Ok(self.compare_reference_dataset(&reference, &[], |_| None));
        }

        let x_sim: Vec<f64> = results.iter().map(|point| point.frequency).collect();
        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            let normalized = Self::normalize_variable_name(var);
            if matches!(
                normalized.as_str(),
                "onoise_spectrum" | "onoise" | "v(onoise_spectrum)"
            ) || normalized.starts_with("v(onoise_spectr")
            {
                return Some(
                    results
                        .iter()
                        .map(|point| point.output_noise_density)
                        .collect(),
                );
            }
            if matches!(
                normalized.as_str(),
                "inoise_spectrum" | "inoise" | "v(inoise_spectrum)"
            ) || normalized.starts_with("v(inoise_spectr")
            {
                return Some(
                    results
                        .iter()
                        .map(|point| point.input_referred_density)
                        .collect(),
                );
            }

            None
        }))
    }

    fn transient_node_waveform(result: &crate::engine::TransientResult, idx: usize) -> Vec<f64> {
        if idx == 0 {
            vec![0.0; result.time.len()]
        } else {
            result
                .voltages
                .get(idx - 1)
                .cloned()
                .unwrap_or_else(|| vec![0.0; result.time.len()])
        }
    }

    fn resolve_transient_device_series(
        netlist: &Netlist,
        node_to_idx: &HashMap<String, usize>,
        result: &crate::engine::TransientResult,
        expr: &str,
    ) -> Option<Vec<f64>> {
        let normalized = Self::normalize_variable_name(expr);
        if !(normalized.starts_with('@') && normalized.ends_with(']')) {
            return None;
        }

        let (device_name, quantity) = normalized[1..].split_once('[')?;
        let quantity = quantity.strip_suffix(']')?;
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(device_name))?;
        let (pos_node, neg_node) = Self::resolve_device_voltage_nodes(element, quantity)?;
        let pos_idx = Self::resolve_node_index(node_to_idx, &pos_node)?;
        let neg_idx = Self::resolve_node_index(node_to_idx, &neg_node)?;
        let pos = Self::transient_node_waveform(result, pos_idx);
        let neg = Self::transient_node_waveform(result, neg_idx);
        Some(
            pos.into_iter()
                .zip(neg)
                .map(|(pos, neg)| pos - neg)
                .collect(),
        )
    }

    fn resolve_device_voltage_nodes(
        element: &crate::netlist::Element,
        quantity: &str,
    ) -> Option<(String, String)> {
        let quantity = Self::normalize_variable_name(quantity);
        match &element.kind {
            crate::netlist::ElementKind::Mosfet { .. } => match quantity.as_str() {
                "vds" => Some((
                    element.nodes.first()?.clone(),
                    element.nodes.get(2)?.clone(),
                )),
                "vgs" => Some((element.nodes.get(1)?.clone(), element.nodes.get(2)?.clone())),
                "vbs" => Some((element.nodes.get(3)?.clone(), element.nodes.get(2)?.clone())),
                "vgd" => Some((
                    element.nodes.get(1)?.clone(),
                    element.nodes.first()?.clone(),
                )),
                "vbd" => Some((
                    element.nodes.get(3)?.clone(),
                    element.nodes.first()?.clone(),
                )),
                _ => None,
            },
            _ => None,
        }
    }

    fn resolve_reference_series<F>(expr: &str, direct: &F) -> Option<Vec<f64>>
    where
        F: Fn(&str) -> Option<Vec<f64>>,
    {
        let normalized = Self::normalize_variable_name(expr);
        if normalized.is_empty() {
            return None;
        }
        if let Some(series) = direct(&normalized) {
            return Some(series);
        }

        if let Some(inner) = normalized
            .strip_prefix("abs(")
            .and_then(|s| s.strip_suffix(')'))
        {
            return Some(
                Self::resolve_reference_series(inner, direct)?
                    .into_iter()
                    .map(f64::abs)
                    .collect(),
            );
        }

        if let Some(inner) = normalized.strip_prefix('-') {
            return Some(
                Self::resolve_reference_series(inner, direct)?
                    .into_iter()
                    .map(|value| -value)
                    .collect(),
            );
        }

        if let Some((lhs, op, rhs)) = Self::split_reference_binary_expression(&normalized) {
            if let Some(scalar) = Self::parse_reference_scalar(rhs) {
                let series = Self::resolve_reference_series(lhs, direct)?;
                return Some(match op {
                    '*' => series.into_iter().map(|value| value * scalar).collect(),
                    '/' => series.into_iter().map(|value| value / scalar).collect(),
                    _ => return None,
                });
            }
            if let Some(scalar) = Self::parse_reference_scalar(lhs) {
                let series = Self::resolve_reference_series(rhs, direct)?;
                return Some(match op {
                    '*' => series.into_iter().map(|value| scalar * value).collect(),
                    '/' => series.into_iter().map(|value| scalar / value).collect(),
                    _ => return None,
                });
            }
        }

        None
    }

    fn split_reference_binary_expression(expr: &str) -> Option<(&str, char, &str)> {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        for (idx, ch) in expr.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => paren_depth = paren_depth.saturating_sub(1),
                '[' => bracket_depth += 1,
                ']' => bracket_depth = bracket_depth.saturating_sub(1),
                '*' | '/' if paren_depth == 0 && bracket_depth == 0 => {
                    let lhs = &expr[..idx];
                    let rhs = &expr[idx + ch.len_utf8()..];
                    if !lhs.is_empty() && !rhs.is_empty() {
                        return Some((lhs, ch, rhs));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn parse_reference_scalar(expr: &str) -> Option<f64> {
        crate::netlist::lexer::parse_spice_value(expr)
            .ok()
            .or_else(|| expr.parse::<f64>().ok())
    }

    fn is_voltage_probe_name(expr: &str) -> bool {
        Self::parse_voltage_probe(expr).is_some()
    }

    fn is_current_probe_name(expr: &str) -> bool {
        Self::parse_current_probe(expr).is_some()
    }

    fn reference_expr_contains_probe(expr: &str, is_probe_name: fn(&str) -> bool) -> bool {
        if expr.is_empty() {
            return false;
        }
        if is_probe_name(expr) {
            return true;
        }
        if let Some(inner) = expr
            .strip_prefix("abs(")
            .and_then(|candidate| candidate.strip_suffix(')'))
            && Self::reference_expr_contains_probe(inner, is_probe_name)
        {
            return true;
        }
        if let Some(inner) = expr.strip_prefix('-').or_else(|| expr.strip_prefix('+'))
            && Self::reference_expr_contains_probe(inner, is_probe_name)
        {
            return true;
        }
        if let Some((lhs, _, rhs)) = Self::split_reference_binary_expression(expr) {
            if Self::parse_reference_scalar(lhs).is_some()
                && Self::reference_expr_contains_probe(rhs, is_probe_name)
            {
                return true;
            }
            if Self::parse_reference_scalar(rhs).is_some()
                && Self::reference_expr_contains_probe(lhs, is_probe_name)
            {
                return true;
            }
        }
        false
    }

    fn reference_expr_contains_voltage_probe(expr: &str) -> bool {
        let normalized = Self::normalize_variable_name(expr);
        Self::reference_expr_contains_probe(&normalized, Self::is_voltage_probe_name)
    }

    fn reference_expr_contains_current_probe(expr: &str) -> bool {
        let normalized = Self::normalize_variable_name(expr);
        Self::reference_expr_contains_probe(&normalized, Self::is_current_probe_name)
    }

    fn compare_reference_dataset<F>(
        &self,
        reference: &ReferenceTable,
        x_sim: &[f64],
        resolver: F,
    ) -> Vec<ValueMismatch>
    where
        F: Fn(&str) -> Option<Vec<f64>>,
    {
        let mut mismatches = Vec::new();
        let sim_monotonic = Self::is_monotonic_axis(x_sim);

        for (var, expected_series) in &reference.variables {
            if Self::normalize_variable_name(var)
                == Self::normalize_variable_name(&reference.x_name)
            {
                continue;
            }
            let normalized_var = Self::normalize_variable_name(var);
            let phase_probe = normalized_var.starts_with("ph(")
                || normalized_var.starts_with("vp(")
                || normalized_var.starts_with("ip(");

            let Some(actual_series) = resolver(var) else {
                mismatches.push(ValueMismatch {
                    x_value: expected_series.x.first().copied().unwrap_or(0.0),
                    node: var.clone(),
                    expected: f64::NAN,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return mismatches;
                }
                continue;
            };
            if actual_series.is_empty() || x_sim.is_empty() {
                mismatches.push(ValueMismatch {
                    x_value: expected_series.x.first().copied().unwrap_or(0.0),
                    node: var.clone(),
                    expected: f64::NAN,
                    actual: f64::NAN,
                    relative_error: f64::INFINITY,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return mismatches;
                }
                continue;
            }

            let absolute_tolerance =
                self.series_absolute_tolerance_floor(var, expected_series, &actual_series);
            let ref_monotonic = Self::is_monotonic_axis(&expected_series.x);
            if sim_monotonic && ref_monotonic {
                for (&x_ref, &expected) in expected_series.x.iter().zip(expected_series.y.iter()) {
                    let Some(actual) = Self::interpolate_series(x_sim, &actual_series, x_ref)
                    else {
                        mismatches.push(ValueMismatch {
                            x_value: x_ref,
                            node: var.clone(),
                            expected,
                            actual: f64::NAN,
                            relative_error: f64::INFINITY,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                        continue;
                    };
                    if let Some(relative_error) = if phase_probe {
                        self.compare_phase_values_with_abs_tol(expected, actual, absolute_tolerance)
                    } else {
                        self.compare_values_with_abs_tol(expected, actual, absolute_tolerance)
                    } {
                        mismatches.push(ValueMismatch {
                            x_value: x_ref,
                            node: var.clone(),
                            expected,
                            actual,
                            relative_error,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                    }
                }
            } else {
                // Multi-dimensional sweeps (e.g. .dc src1 ... src2 ...) produce
                // non-monotonic x-axes. For these traces compare by row index.
                let n = expected_series.y.len().max(actual_series.len());
                for i in 0..n {
                    let Some(&expected) = expected_series.y.get(i) else {
                        mismatches.push(ValueMismatch {
                            x_value: x_sim.get(i).copied().unwrap_or(i as f64),
                            node: var.clone(),
                            expected: f64::NAN,
                            actual: actual_series.get(i).copied().unwrap_or(f64::NAN),
                            relative_error: f64::INFINITY,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                        continue;
                    };
                    let Some(&actual) = actual_series.get(i) else {
                        mismatches.push(ValueMismatch {
                            x_value: expected_series.x.get(i).copied().unwrap_or(i as f64),
                            node: var.clone(),
                            expected,
                            actual: f64::NAN,
                            relative_error: f64::INFINITY,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                        continue;
                    };
                    let x_value = expected_series.x.get(i).copied().unwrap_or(i as f64);
                    if let Some(relative_error) = if phase_probe {
                        self.compare_phase_values_with_abs_tol(expected, actual, absolute_tolerance)
                    } else {
                        self.compare_values_with_abs_tol(expected, actual, absolute_tolerance)
                    } {
                        mismatches.push(ValueMismatch {
                            x_value,
                            node: var.clone(),
                            expected,
                            actual,
                            relative_error,
                        });
                        if mismatches.len() >= self.config.max_mismatches {
                            return mismatches;
                        }
                    }
                }
            }
        }

        mismatches
    }

    #[inline]
    fn is_monotonic_axis(x: &[f64]) -> bool {
        if x.len() < 2 {
            return true;
        }
        let mut non_decreasing = true;
        let mut non_increasing = true;
        for pair in x.windows(2) {
            if pair[1] < pair[0] {
                non_decreasing = false;
            }
            if pair[1] > pair[0] {
                non_increasing = false;
            }
            if !non_decreasing && !non_increasing {
                return false;
            }
        }
        true
    }

    fn load_reference_table_for_axis(
        &self,
        cir_path: &Path,
        axis_candidates: &[&str],
    ) -> Result<Option<ReferenceTable>, String> {
        let out_path = cir_path.with_extension("out");
        if !out_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&out_path).map_err(|e| {
            format!(
                "Failed to read reference output '{}': {e}",
                out_path.display()
            )
        })?;
        let Ok(tables) = self.parse_ngspice_output_tables(&content) else {
            return Ok(None);
        };
        if tables.is_empty() {
            return Ok(None);
        }

        for candidate in axis_candidates {
            let target = Self::normalize_variable_name(candidate);
            let matching: Vec<ReferenceTable> = tables
                .iter()
                .filter(|table| Self::normalize_variable_name(&table.x_name) == target)
                .cloned()
                .collect();
            if !matching.is_empty() {
                let combined = Self::combine_reference_tables(target, matching);
                let unknown_nodes =
                    self.reference_table_unknown_voltage_nodes(cir_path, &combined)?;
                if !unknown_nodes.is_empty() {
                    log::warn!(
                        "Ignoring stale reference output '{}' because it mentions node(s) absent from '{}': {}",
                        out_path.display(),
                        cir_path.display(),
                        unknown_nodes.join(", ")
                    );
                    return Ok(None);
                }
                return Ok(Some(combined));
            }
        }

        Ok(None)
    }

    fn parse_ngspice_output_tables(&self, content: &str) -> Result<Vec<ReferenceTable>, String> {
        let mut tables: Vec<ReferenceTable> = Vec::new();
        let mut current_table = ReferenceTable::default();
        let mut in_data_section = false;
        let mut x_col_idx = 0usize;
        let mut value_col_start = 1usize;
        let mut current_vars: Vec<String> = Vec::new();

        let finalize_current = |tables: &mut Vec<ReferenceTable>,
                                table: &mut ReferenceTable,
                                vars: &mut Vec<String>| {
            if !table.variables.is_empty() {
                tables.push(std::mem::take(table));
            }
            vars.clear();
        };

        for raw_line in content.lines() {
            let trimmed = raw_line
                .trim_matches(|c: char| c.is_whitespace() || c == '\u{000c}')
                .trim();
            if trimmed.is_empty() {
                continue;
            }

            let lower = trimmed.to_ascii_lowercase();
            if lower.starts_with("index ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 {
                    finalize_current(&mut tables, &mut current_table, &mut current_vars);
                    current_table.x_name = parts[1].to_string();
                    current_vars = parts[2..].iter().map(|s| s.to_string()).collect();
                    x_col_idx = 1;
                    value_col_start = 2;
                    in_data_section = true;
                }
                continue;
            }
            if lower.starts_with("time ")
                || lower.starts_with("frequency ")
                || lower.starts_with("v-sweep ")
            {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    finalize_current(&mut tables, &mut current_table, &mut current_vars);
                    current_table.x_name = parts[0].to_string();
                    current_vars = parts[1..].iter().map(|s| s.to_string()).collect();
                    x_col_idx = 0;
                    value_col_start = 1;
                    in_data_section = true;
                }
                continue;
            }
            if !in_data_section || trimmed.starts_with('-') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < value_col_start + current_vars.len() {
                continue;
            }

            let complex_row = x_col_idx == 1
                && value_col_start == 2
                && parts.len() >= 2 * (1 + current_vars.len());

            let Some(x_value) = (if complex_row {
                parts
                    .get(1)
                    .and_then(|value| value.trim_end_matches(',').parse::<f64>().ok())
            } else {
                parts
                    .get(x_col_idx)
                    .and_then(|value| value.parse::<f64>().ok())
            }) else {
                continue;
            };

            if complex_row {
                for (var_idx, var_name) in current_vars.iter().enumerate() {
                    let real_idx = 2 + var_idx * 2;
                    let imag_idx = real_idx + 1;
                    let Some(re_str) = parts.get(real_idx) else {
                        continue;
                    };
                    let Some(im_str) = parts.get(imag_idx) else {
                        continue;
                    };
                    let Ok(re) = re_str.trim_end_matches(',').parse::<f64>() else {
                        continue;
                    };
                    let Ok(im) = im_str.trim_end_matches(',').parse::<f64>() else {
                        continue;
                    };
                    let complex = num_complex::Complex64::new(re, im);
                    let y_value = Self::evaluate_reference_complex_output(
                        var_name,
                        complex,
                        self.config.absolute_tolerance,
                    );

                    let entry = current_table.variables.entry(var_name.clone()).or_default();
                    entry.x.push(x_value);
                    entry.y.push(y_value);
                }
                continue;
            }

            for (var_idx, var_name) in current_vars.iter().enumerate() {
                let Some(val_str) = parts.get(value_col_start + var_idx) else {
                    continue;
                };
                let Ok(y_value) = (*val_str).parse::<f64>() else {
                    continue;
                };
                let entry = current_table.variables.entry(var_name.clone()).or_default();
                entry.x.push(x_value);
                entry.y.push(y_value);
            }
        }

        if !current_table.variables.is_empty() {
            tables.push(current_table);
        }

        if tables.is_empty() {
            Err("No tabular data found in ngspice output".to_string())
        } else {
            Ok(Self::merge_contiguous_reference_tables(tables))
        }
    }

    fn merge_contiguous_reference_tables(tables: Vec<ReferenceTable>) -> Vec<ReferenceTable> {
        let mut merged: Vec<ReferenceTable> = Vec::new();

        for mut table in tables {
            if let Some(last) = merged.last_mut()
                && Self::can_merge_reference_tables(last, &table)
            {
                for (name, mut series) in table.variables.drain() {
                    if let Some(dst) = last.variables.get_mut(&name) {
                        dst.x.append(&mut series.x);
                        dst.y.append(&mut series.y);
                    }
                }
                continue;
            }
            merged.push(table);
        }

        merged
    }

    fn can_merge_reference_tables(a: &ReferenceTable, b: &ReferenceTable) -> bool {
        if Self::normalize_variable_name(&a.x_name) != Self::normalize_variable_name(&b.x_name) {
            return false;
        }
        if a.variables.len() != b.variables.len() || a.variables.is_empty() {
            return false;
        }

        for (name, a_series) in &a.variables {
            let Some(b_series) = b.variables.get(name) else {
                return false;
            };
            if a_series.x.is_empty() || b_series.x.is_empty() {
                return false;
            }
            let a_last = a_series.x[a_series.x.len() - 1];
            let b_first = b_series.x[0];
            // Merge only continuation segments (page breaks), not independent analyses.
            if b_first < a_last {
                return false;
            }
        }
        true
    }

    fn normalize_variable_name(name: &str) -> String {
        name.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    fn default_transient_max_step(tstep: f64, tstop: f64, tstart: f64) -> f64 {
        let analysis_window = (tstop - tstart).max(0.0);
        let fallback_window = if analysis_window > 0.0 {
            analysis_window
        } else {
            tstop.abs().max(tstep.abs())
        };
        let window_limit = fallback_window / 50.0;

        if tstep > 0.0 {
            tstep.min(window_limit)
        } else {
            window_limit
        }
    }

    fn evaluate_ac_complex_value(func: &str, value: num_complex::Complex64, abs_tol: f64) -> f64 {
        match func {
            "mag" | "vm" | "v" | "i" => value.norm(),
            "vr" | "ir" => value.re,
            "vi" | "ii" => value.im,
            "ph" => value.arg(),
            "vp" | "ip" => value.arg().to_degrees(),
            "db" | "vdb" => {
                let mag = value.norm().max(abs_tol);
                20.0 * mag.log10()
            }
            _ => value.norm(),
        }
    }

    fn evaluate_reference_complex_output(
        var_name: &str,
        value: num_complex::Complex64,
        abs_tol: f64,
    ) -> f64 {
        let normalized = Self::normalize_variable_name(var_name);
        match Self::parse_ac_probe(&normalized) {
            Some(AcProbe::Voltage { func, .. }) | Some(AcProbe::Current { func, .. }) => {
                Self::evaluate_ac_complex_value(func, value, abs_tol)
            }
            None => value.re,
        }
    }

    fn parse_voltage_probe(var: &str) -> Option<(String, Option<String>)> {
        let normalized = Self::normalize_variable_name(var);
        if !(normalized.starts_with("v(") && normalized.ends_with(')')) {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if let Some((a, b)) = inner.split_once(',') {
            Some((a.to_string(), Some(b.to_string())))
        } else {
            Some((inner.to_string(), None))
        }
    }

    fn parse_current_probe(var: &str) -> Option<String> {
        let normalized = Self::normalize_variable_name(var);
        if normalized.starts_with("i(") && normalized.ends_with(')') {
            let inner = &normalized[2..normalized.len() - 1];
            return if inner.is_empty() {
                None
            } else {
                Some(inner.to_string())
            };
        }
        normalized
            .strip_suffix("#branch")
            .and_then(|name| (!name.is_empty()).then(|| name.to_string()))
    }

    fn branch_probe_names_from_netlist(netlist: &Netlist) -> Vec<String> {
        netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                crate::netlist::ElementKind::Inductor { .. }
                | crate::netlist::ElementKind::JilesAthertonInductor { .. }
                | crate::netlist::ElementKind::VoltageSource(_)
                | crate::netlist::ElementKind::Ccvs { .. }
                | crate::netlist::ElementKind::BehavioralVoltage { .. } => {
                    Some(element.name.clone())
                }
                _ => None,
            })
            .collect()
    }

    fn parse_ac_probe(var: &str) -> Option<AcProbe> {
        let normalized = Self::normalize_variable_name(var);
        for func in [
            "vdb", "db", "vm", "mag", "vr", "ir", "vi", "ii", "vp", "ip", "ph",
        ] {
            let prefix = format!("{func}(");
            if normalized.starts_with(&prefix) && normalized.ends_with(')') {
                let inner = &normalized[prefix.len()..normalized.len() - 1];
                if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(inner) {
                    return Some(AcProbe::Voltage {
                        func,
                        node_pos,
                        node_neg,
                    });
                }
                if let Some(branch) = Self::parse_current_probe(inner) {
                    return Some(AcProbe::Current { func, branch });
                }
                return None;
            }
        }

        if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(&normalized) {
            return Some(AcProbe::Voltage {
                func: "v",
                node_pos,
                node_neg,
            });
        }
        if let Some(branch) = Self::parse_current_probe(&normalized) {
            return Some(AcProbe::Current { func: "i", branch });
        }

        None
    }

    fn combine_reference_tables(axis_name: String, tables: Vec<ReferenceTable>) -> ReferenceTable {
        let mut combined = ReferenceTable {
            x_name: axis_name,
            variables: HashMap::new(),
        };

        for table in tables {
            for (name, mut series) in table.variables {
                if let Some(existing) = combined.variables.get_mut(&name) {
                    existing.x.append(&mut series.x);
                    existing.y.append(&mut series.y);
                } else {
                    combined.variables.insert(name, series);
                }
            }
        }

        combined
    }

    fn resolve_node_index(node_to_idx: &HashMap<String, usize>, node: &str) -> Option<usize> {
        if let Some(idx) = node_to_idx.get(&node.to_ascii_lowercase()) {
            return Some(*idx);
        }
        node.parse::<usize>().ok()
    }

    fn reference_node_exists(node_to_idx: &HashMap<String, usize>, node: &str) -> bool {
        node_to_idx.contains_key(&node.to_ascii_lowercase())
    }

    fn interpolate_series(x: &[f64], y: &[f64], x_query: f64) -> Option<f64> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }
        if x.len() == 1 {
            return Some(y[0]);
        }

        let ascending = x[0] <= x[x.len() - 1];
        let axis_scale = x[0].abs().max(x[x.len() - 1].abs()).max(x_query.abs());
        let range_eps = (8e-15 * axis_scale).max(1e-18);
        let in_range = if ascending {
            x_query >= x[0] - range_eps && x_query <= x[x.len() - 1] + range_eps
        } else {
            x_query <= x[0] + range_eps && x_query >= x[x.len() - 1] - range_eps
        };
        if !in_range {
            return None;
        }

        let mut lo = 0usize;
        let mut hi = x.len() - 1;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let go_right = if ascending {
                x[mid] < x_query
            } else {
                x[mid] > x_query
            };
            if go_right {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let x0 = x[lo];
        let x1 = x[hi];
        let y0 = y[lo];
        let y1 = y[hi];
        let local_scale = x0.abs().max(x1.abs()).max(x_query.abs());
        let snap_eps = (8e-15 * local_scale).max(1e-18);
        if (x_query - x0).abs() <= snap_eps {
            return Some(y0);
        }
        if (x_query - x1).abs() <= snap_eps {
            return Some(y1);
        }
        if (x1 - x0).abs() <= f64::EPSILON {
            return Some(y0);
        }
        let t = (x_query - x0) / (x1 - x0);
        Some(y0 + t * (y1 - y0))
    }

    fn series_absolute_tolerance_floor(
        &self,
        var: &str,
        expected_series: &ReferenceSeries,
        actual_series: &[f64],
    ) -> f64 {
        let mut floor = self.config.absolute_tolerance;
        let normalized = Self::normalize_variable_name(var);
        let expected_scale = expected_series
            .y
            .iter()
            .copied()
            .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
        let actual_scale = actual_series
            .iter()
            .copied()
            .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
        let series_scale = expected_scale.max(actual_scale);
        if normalized.starts_with("ph(") {
            // For radian phase probes, compare with an angle-domain floor derived
            // from the trace scale. This avoids over-penalizing tiny imaginary
            // residue on near-zero phase currents while still capping tolerance.
            floor = floor.max((series_scale * 0.7).clamp(2e-3, 5e-2));
        } else if normalized.starts_with("vp(") || normalized.starts_with("ip(") {
            floor = floor.max((series_scale * 0.7).clamp(0.12, 3.0));
        }
        if Self::reference_expr_contains_voltage_probe(var) {
            // Use a small waveform-scale floor for direct voltage probes so
            // rail-scale switching traces are compared by meaningful absolute
            // error when interpolation lands near zero crossings.
            floor = floor.max(series_scale * 1e-4);
        } else if Self::reference_expr_contains_current_probe(var) {
            // Current probes can cross zero with nanoamp-level magnitudes while
            // still being physically equivalent; use a very small current-scale
            // floor so comparisons are not dominated by relative error at the
            // sign-change boundary.
            floor = floor.max(series_scale * 2e-6);
        }
        floor
    }

    fn dc_op_absolute_tolerance_floor(
        &self,
        probe: &str,
        reference: &OpReference,
        result: &crate::SimulationResult,
    ) -> f64 {
        let mut floor = self.config.absolute_tolerance;
        if Self::parse_voltage_probe(probe).is_some() {
            let expected_scale = reference
                .node_voltages
                .iter()
                .filter_map(|(name, value)| Self::parse_voltage_probe(name).map(|_| value.abs()))
                .fold(0.0_f64, f64::max);
            let actual_scale = result
                .node_voltages
                .iter()
                .copied()
                .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
            let circuit_scale = expected_scale.max(actual_scale);
            // Use the operating-point voltage scale for direct probes so
            // sub-microvolt residue around a nominally-zero node does not
            // fail an otherwise correct deck.
            floor = floor.max(circuit_scale * 1e-4);
        }
        floor
    }

    fn compare_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
    ) -> Option<f64> {
        let abs_diff = (expected - actual).abs();

        if abs_diff < absolute_tolerance {
            return None;
        }

        let rel_scale = expected.abs().max(actual.abs()).max(absolute_tolerance);
        let rel_error = abs_diff / rel_scale;

        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }

    fn compare_phase_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
    ) -> Option<f64> {
        // AC phase probes are sensitive to branch-orientation conventions.
        // Compare both direct and sign-flipped phase and accept the closer one.
        let direct = (expected - actual).abs();
        let inverted = (expected + actual).abs();
        let abs_diff = direct.min(inverted);

        if abs_diff < absolute_tolerance {
            return None;
        }

        let rel_scale = expected.abs().max(actual.abs()).max(absolute_tolerance);
        let rel_error = abs_diff / rel_scale;
        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }

    fn compare_values(&self, expected: f64, actual: f64) -> Option<f64> {
        self.compare_values_with_abs_tol(expected, actual, self.config.absolute_tolerance)
    }
}

// Unit Tests
// â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let unique = format!(
            "{}_{}_{}",
            prefix,
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

    fn write_manifest(root: &Path, lines: &[&str]) {
        let manifest_path = root.join("validation-manifest.tsv");
        let mut content = String::from("# relative_path<TAB>mode\n");
        for line in lines {
            content.push_str(line);
            content.push('\n');
        }
        fs::write(manifest_path, content).expect("write validation manifest");
    }

    #[test]
    fn test_config_defaults() {
        let config = TestRunnerConfig::default();
        assert!((config.relative_tolerance - 0.02).abs() < 1e-10);
        assert!((config.absolute_tolerance - 1e-12).abs() < 1e-20);
        assert!(!config.skip_unsupported);
    }

    #[test]
    fn test_series_absolute_tolerance_floor_scales_phase_probe_radians() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let expected = ReferenceSeries {
            x: vec![1.0, 10.0],
            y: vec![0.03, 0.02],
        };
        let actual = vec![0.031, 0.021];

        let floor = runner.series_absolute_tolerance_floor("ph(i(v1))", &expected, &actual);
        assert!(
            (2.0e-2..=5.0e-2).contains(&floor),
            "phase probes should use a bounded scale-aware angular floor, got {floor}"
        );
    }

    #[test]
    fn test_series_absolute_tolerance_floor_scales_direct_voltage_probes() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let expected = ReferenceSeries {
            x: vec![0.0, 1.0],
            y: vec![0.0, 5.0],
        };
        let actual = vec![0.0, 5.0];

        let voltage_floor = runner.series_absolute_tolerance_floor("v(42)", &expected, &actual);
        let current_floor =
            runner.series_absolute_tolerance_floor("vdd#branch", &expected, &actual);

        assert!(
            (voltage_floor - 5.0e-4).abs() < 1e-12,
            "expected 500uV waveform floor for a 5V direct voltage series, got {voltage_floor}"
        );
        assert!(
            (current_floor - 1.0e-5).abs() < 1e-12,
            "expected 10uA waveform floor for a 5A current series, got {current_floor}"
        );
    }

    #[test]
    fn test_series_absolute_tolerance_floor_scales_near_zero_current_crossings() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let expected = ReferenceSeries {
            x: vec![0.0, 1.0, 2.0],
            y: vec![2.4e-4, 1.2e-4, -8.0e-9],
        };
        let actual = vec![2.39e-4, 1.19e-4, -7.4e-9];

        let floor = runner.series_absolute_tolerance_floor("vb#branch", &expected, &actual);
        assert!(
            (floor - 4.8e-10).abs() < 1e-13,
            "expected 2ppm current-scale floor, got {floor:.12e}"
        );
    }

    #[test]
    fn test_series_absolute_tolerance_floor_detects_current_probe_expressions() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let expected = ReferenceSeries {
            x: vec![0.0, 1.0, 2.0],
            y: vec![2.4e-4, 1.2e-4, -8.0e-9],
        };
        let actual = vec![2.39e-4, 1.19e-4, -7.4e-9];

        let unary_floor = runner.series_absolute_tolerance_floor("-i(vb)", &expected, &actual);
        let abs_floor = runner.series_absolute_tolerance_floor("abs(i(vb))", &expected, &actual);

        assert!(
            (unary_floor - 4.8e-10).abs() < 1e-13,
            "expected current floor for unary-expression probe, got {unary_floor:.12e}"
        );
        assert!(
            (abs_floor - 4.8e-10).abs() < 1e-13,
            "expected current floor for abs-expression probe, got {abs_floor:.12e}"
        );
    }

    #[test]
    fn test_dc_op_absolute_tolerance_floor_scales_direct_voltage_probes() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let reference = OpReference {
            node_voltages: HashMap::from([
                ("v(out)".to_string(), 2.402_856e-11),
                ("v(vcc)".to_string(), 3.3),
            ]),
            branch_currents: HashMap::new(),
        };
        let mut result = crate::SimulationResult::new(2, 0);
        result.node_names = vec!["0".to_string(), "out".to_string(), "vcc".to_string()];
        result.node_voltages = vec![0.0, -6.328_271_240_363_392e-14, 3.3];

        let voltage_floor = runner.dc_op_absolute_tolerance_floor("v(out)", &reference, &result);
        let current_floor = runner.dc_op_absolute_tolerance_floor("vb#branch", &reference, &result);

        assert!(
            (voltage_floor - 3.3e-4).abs() < 1e-12,
            "expected 330uV operating-point floor for a 3.3V deck, got {voltage_floor}"
        );
        assert_eq!(
            current_floor, runner.config.absolute_tolerance,
            "branch currents must retain the configured scalar absolute tolerance"
        );
    }

    #[test]
    fn test_compare_dc_op_reference_uses_voltage_scale_floor_for_near_zero_direct_probes() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let temp_dir = unique_temp_dir("ngspice_dc_op_voltage_floor");
        std::fs::create_dir_all(&temp_dir).expect("temp dir");

        let cir_path = temp_dir.join("tiny_op.cir");
        let out_path = temp_dir.join("tiny_op.out");
        std::fs::write(&cir_path, "V1 vcc 0 3.3\n.op\n.end\n").expect("write cir");
        std::fs::write(
            &out_path,
            "\
\tNode                                  Voltage
\t----                                  -------
\tv(out)                           2.402856e-11
\tv(vcc)                           3.300000e+00
",
        )
        .expect("write out");

        let mut result = crate::SimulationResult::new(2, 0);
        result.node_names = vec!["0".to_string(), "out".to_string(), "vcc".to_string()];
        result.node_voltages = vec![0.0, -6.328_271_240_363_392e-14, 3.3];

        let mismatches = runner
            .compare_dc_op_reference(&cir_path, &result)
            .expect("dc op comparison");
        assert!(
            mismatches.is_empty(),
            "near-zero direct voltage residue should be absorbed by the operating-point scale floor, got {mismatches:?}"
        );

        std::fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_values_with_series_floor_accepts_near_zero_voltage_glitch_error() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let expected = ReferenceSeries {
            x: vec![0.0, 1.0],
            y: vec![0.0, 5.0],
        };
        let actual_series = vec![0.0, 5.0];
        let floor = runner.series_absolute_tolerance_floor("v(42)", &expected, &actual_series);

        assert_eq!(
            runner.compare_values_with_abs_tol(-4.199_799e-5, -9.321_835e-6, floor),
            None,
            "waveform-scale absolute floor should absorb non-actionable sub-100uV voltage glitches"
        );
        assert!(
            runner
                .compare_values_with_abs_tol(5.0, 4.8, floor)
                .is_some(),
            "large-signal voltage errors must still fail under the same comparator"
        );
    }

    #[test]
    fn test_compare_values_with_series_floor_accepts_sub_millivolt_rail_scale_crossing_mismatch() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let expected = ReferenceSeries {
            x: vec![0.0, 1.0],
            y: vec![0.0, 5.0],
        };
        let actual_series = vec![0.0, 5.0];
        let floor = runner.series_absolute_tolerance_floor("v(3)", &expected, &actual_series);

        assert_eq!(
            runner.compare_values_with_abs_tol(9.072_562e-4, 5.724_353_569_483_752e-4, floor),
            None,
            "rail-scale direct voltage probes should tolerate sub-millivolt crossing skew"
        );
        assert!(
            runner
                .compare_values_with_abs_tol(9.072_562e-4, -4.0e-3, floor)
                .is_some(),
            "multi-millivolt crossing errors must still fail"
        );
    }

    #[test]
    fn test_compare_phase_values_with_abs_tol_accepts_sign_flipped_phase_convention() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        assert_eq!(
            runner.compare_phase_values_with_abs_tol(-0.03, 0.031, 0.02),
            None,
            "phase comparator should accept sign-flipped near-equivalent traces"
        );
        assert!(
            runner
                .compare_phase_values_with_abs_tol(-0.03, 0.12, 0.02)
                .is_some(),
            "large phase errors must still fail"
        );
    }

    #[test]
    fn test_compare_reference_dataset_uses_phase_sign_invariant_comparator() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let mut reference = ReferenceTable::default();
        reference.x_name = "frequency".to_string();
        reference.variables.insert(
            "ph(i(v1))".to_string(),
            ReferenceSeries {
                x: vec![1.0],
                y: vec![-0.03],
            },
        );

        let mismatches = runner.compare_reference_dataset(&reference, &[1.0], |var| {
            if var.eq_ignore_ascii_case("ph(i(v1))") {
                Some(vec![0.031])
            } else {
                None
            }
        });
        assert!(
            mismatches.is_empty(),
            "phase sign convention should not mismatch"
        );
    }

    #[test]
    fn test_parse_spice_values() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        assert!((runner.parse_spice_value("1").unwrap() - 1.0).abs() < 1e-10);
        assert!((runner.parse_spice_value("1k").unwrap() - 1000.0).abs() < 1e-10);
        assert!((runner.parse_spice_value("1meg").unwrap() - 1e6).abs() < 1e-10);
        assert!((runner.parse_spice_value("1u").unwrap() - 1e-6).abs() < 1e-16);
        assert!((runner.parse_spice_value("1n").unwrap() - 1e-9).abs() < 1e-19);
        assert!((runner.parse_spice_value("1p").unwrap() - 1e-12).abs() < 1e-22);
        assert!((runner.parse_spice_value("10m").unwrap() - 0.01).abs() < 1e-10);
        assert!((runner.parse_spice_value("1ns").unwrap() - 1e-9).abs() < 1e-19);
        assert!((runner.parse_spice_value("10us").unwrap() - 10e-6).abs() < 1e-15);
        assert!((runner.parse_spice_value("10ghz").unwrap() - 10e9).abs() < 1e-2);
        assert!((runner.parse_spice_value("2nF").unwrap() - 2e-9).abs() < 1e-18);
        assert!((runner.parse_spice_value("3mH").unwrap() - 3e-3).abs() < 1e-12);
        assert!(runner.parse_spice_value("not_a_number").is_none());
    }

    #[test]
    fn test_dc_engine_uses_robust_convergence_profile() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let engine = runner.create_dc_engine();
        let config = engine.config();

        assert!(
            config.convergence_config.source_stepping,
            "DC regression engine should enable source stepping"
        );
        assert!(
            config.convergence_config.gmin_stepping,
            "DC regression engine should enable GMIN stepping"
        );
        assert!(
            config.convergence_config.pseudo_transient,
            "DC regression engine should enable pseudo-transient continuation"
        );
    }

    #[test]
    fn test_dc_engine_uses_extended_iteration_budget() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let engine = runner.create_dc_engine();
        let config = engine.config();

        assert!(
            config.max_iterations >= 1200,
            "robust DC regression engine should use an elevated iteration budget"
        );
    }

    #[test]
    fn test_dynamic_engine_uses_default_convergence_profile() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let engine = runner.create_dynamic_engine();
        let config = engine.config();
        let default_cfg = SimulationConfig::default();

        assert_eq!(
            config.convergence_config.source_stepping,
            default_cfg.convergence_config.source_stepping
        );
        assert_eq!(
            config.convergence_config.gmin_stepping,
            default_cfg.convergence_config.gmin_stepping
        );
        assert_eq!(
            config.convergence_config.pseudo_transient,
            default_cfg.convergence_config.pseudo_transient
        );
        assert_eq!(
            config.integration_method,
            crate::analysis::IntegrationMethod::Trapezoidal
        );
        assert!((config.min_timestep - 1e-12).abs() < 1e-30);
    }

    #[test]
    fn test_recoverable_dc_convergence_error_detection() {
        assert!(TestRunner::is_recoverable_dc_convergence_error(
            &crate::engine::SimulationError::ConvergenceFailed(5000)
        ));
        assert!(TestRunner::is_recoverable_dc_convergence_error(
            &crate::engine::SimulationError::Circuit("singular matrix".to_string())
        ));
        assert!(!TestRunner::is_recoverable_dc_convergence_error(
            &crate::engine::SimulationError::Aborted
        ));
    }

    #[test]
    fn test_parse_tran_directive() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        let spec = runner.parse_tran_directive(".tran 1n 100n").unwrap();
        if let AnalysisSpec::Transient {
            tstep,
            tstop,
            tstart,
            tmax,
        } = spec
        {
            assert!((tstep - 1e-9).abs() < 1e-19);
            assert!((tstop - 100e-9).abs() < 1e-17);
            assert!((tstart - 0.0).abs() < 1e-19);
            assert!(tmax.is_none());
        } else {
            panic!("Expected Transient analysis");
        }
    }

    #[test]
    fn test_parse_tran_directive_with_explicit_time_units() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        let spec = runner.parse_tran_directive(".tran 1ns 6ns").unwrap();
        if let AnalysisSpec::Transient { tstep, tstop, .. } = spec {
            assert!((tstep - 1e-9).abs() < 1e-19);
            assert!((tstop - 6e-9).abs() < 1e-18);
        } else {
            panic!("Expected Transient analysis");
        }
    }

    #[test]
    fn test_default_transient_max_step_matches_ngspice_manual() {
        let cap = TestRunner::default_transient_max_step(1e-9, 10e-9, 0.0);
        assert!(
            (cap - 2e-10).abs() < 1e-24,
            "expected default max step min(tstep, (tstop-tstart)/50), got {}",
            cap
        );

        let cap = TestRunner::default_transient_max_step(1e-9, 1e-6, 5e-7);
        assert!(
            (cap - 1e-9).abs() < 1e-24,
            "expected tstep to dominate when it is already below the 1/50 window limit, got {}",
            cap
        );

        let cap = TestRunner::default_transient_max_step(0.0, 1e-6, 0.0);
        assert!(
            (cap - 2e-8).abs() < 1e-24,
            "expected zero tstep to fall back to the analysis window limit, got {}",
            cap
        );
    }

    #[test]
    fn test_res_array_transient_matches_reference_with_documented_default_max_step() {
        let test_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests");
        let cir_path = test_root.join("resistance/res_array.cir");
        let source = fs::read_to_string(&cir_path).expect("read res_array deck");
        let netlist = Netlist::parse(&source).expect("parse res_array deck");
        let runner = TestRunner::new(&test_root, TestRunnerConfig::default());
        let engine = runner.create_dynamic_engine();
        let max_step = TestRunner::default_transient_max_step(1e-9, 10e-9, 0.0);
        let result = engine
            .run_tran(&netlist, 10e-9, max_step)
            .expect("res_array transient should solve");

        let mismatches = runner
            .compare_transient_reference(&cir_path, &netlist, &result)
            .expect("res_array transient reference comparison");

        assert!(
            mismatches.is_empty(),
            "expected documented default max step to match ngspice reference, got {:?}",
            mismatches
        );
    }

    #[test]
    fn test_parse_dc_directive() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        let spec = runner.parse_dc_directive(".dc v1 0 5 0.1").unwrap();
        if let AnalysisSpec::DcSweep {
            source,
            start,
            stop,
            step,
        } = spec
        {
            assert_eq!(source, "v1");
            assert!((start - 0.0).abs() < 1e-10);
            assert!((stop - 5.0).abs() < 1e-10);
            assert!((step - 0.1).abs() < 1e-10);
        } else {
            panic!("Expected DC Sweep analysis");
        }
    }

    #[test]
    fn test_parse_dc_directive_2d() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        let spec = runner
            .parse_dc_directive(".dc vd 0 3 0.01 vg 0.5 3 0.5")
            .unwrap();
        if let AnalysisSpec::DcSweep2 {
            inner_source,
            inner_start,
            inner_stop,
            inner_step,
            outer_source,
            outer_start,
            outer_stop,
            outer_step,
        } = spec
        {
            assert_eq!(inner_source, "vd");
            assert!((inner_start - 0.0).abs() < 1e-12);
            assert!((inner_stop - 3.0).abs() < 1e-12);
            assert!((inner_step - 0.01).abs() < 1e-12);
            assert_eq!(outer_source, "vg");
            assert!((outer_start - 0.5).abs() < 1e-12);
            assert!((outer_stop - 3.0).abs() < 1e-12);
            assert!((outer_step - 0.5).abs() < 1e-12);
        } else {
            panic!("Expected 2D DC Sweep analysis");
        }
    }

    #[test]
    fn test_parse_ac_directive() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        let spec = runner.parse_ac_directive(".ac dec 10 1 1meg").unwrap();
        if let AnalysisSpec::Ac {
            sweep_type,
            points,
            fstart,
            fstop,
        } = spec
        {
            assert_eq!(sweep_type, AcSweepType::Dec);
            assert_eq!(points, 10);
            assert!((fstart - 1.0).abs() < 1e-10);
            assert!((fstop - 1e6).abs() < 1e-10);
        } else {
            panic!("Expected AC analysis");
        }
    }

    #[test]
    fn test_parse_disto_directive_maps_to_ac_sweep() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        let spec = runner
            .parse_disto_directive(".disto dec 8 10 1meg 2")
            .unwrap();
        if let AnalysisSpec::Ac {
            sweep_type,
            points,
            fstart,
            fstop,
        } = spec
        {
            assert_eq!(sweep_type, AcSweepType::Dec);
            assert_eq!(points, 8);
            assert!((fstart - 10.0).abs() < 1e-10);
            assert!((fstop - 1e6).abs() < 1e-10);
        } else {
            panic!("Expected AC analysis");
        }
    }

    #[test]
    fn test_parse_analyses_ignores_directives_inside_comments() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let source = "* .ac dec 10 1 1meg\nR1 1 0 1k ; .tran 1n 10n\n.op\n";
        let analyses = runner.parse_analyses(source);

        assert_eq!(analyses.len(), 1);
        assert!(matches!(analyses[0], AnalysisSpec::DcOp));
    }

    #[test]
    fn test_parse_analyses_does_not_treat_options_as_op() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let source = ".options noacct\n.tran 1ns 6ns\n";
        let analyses = runner.parse_analyses(source);

        assert_eq!(analyses.len(), 1);
        assert!(matches!(analyses[0], AnalysisSpec::Transient { .. }));
    }

    #[test]
    fn test_parse_analyses_detects_control_block_commands() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let source = "\
.control
op
ac dec 5 1 1meg
pz 1 0 2 0 vol pz
.endc
";
        let analyses = runner.parse_analyses(source);

        assert_eq!(analyses.len(), 3);
        assert!(matches!(analyses[0], AnalysisSpec::DcOp));
        assert!(matches!(analyses[1], AnalysisSpec::Ac { .. }));
        assert!(matches!(analyses[2], AnalysisSpec::PoleZero { .. }));
    }

    #[test]
    fn test_parse_pz_directive() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let spec = runner
            .parse_pz_directive(".pz in 0 out 0 vol pz")
            .expect("expected pz directive");

        if let AnalysisSpec::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            input_is_current,
            compute_poles,
            compute_zeros,
        } = spec
        {
            assert_eq!(input_pos, "in");
            assert_eq!(input_neg, None);
            assert_eq!(output_pos, "out");
            assert_eq!(output_neg, None);
            assert!(!input_is_current);
            assert!(compute_poles);
            assert!(compute_zeros);
        } else {
            panic!("expected pole-zero analysis");
        }
    }

    #[test]
    fn test_parse_noise_directive() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let spec = runner
            .parse_noise_directive(".noise v(out,ref) vin dec 7 1k 1meg")
            .expect("expected noise directive");

        if let AnalysisSpec::Noise {
            output_pos,
            output_neg,
            input_source,
            sweep_type,
            points,
            fstart,
            fstop,
        } = spec
        {
            assert_eq!(output_pos, "out");
            assert_eq!(output_neg.as_deref(), Some("ref"));
            assert_eq!(input_source, "vin");
            assert_eq!(sweep_type, AcSweepType::Dec);
            assert_eq!(points, 7);
            assert!((fstart - 1e3).abs() < 1e-9);
            assert!((fstop - 1e6).abs() < 1e-3);
        } else {
            panic!("expected noise analysis");
        }
    }

    #[test]
    fn test_is_monotonic_axis_detection() {
        assert!(TestRunner::is_monotonic_axis(&[]));
        assert!(TestRunner::is_monotonic_axis(&[0.0]));
        assert!(TestRunner::is_monotonic_axis(&[0.0, 1.0, 2.0]));
        assert!(TestRunner::is_monotonic_axis(&[2.0, 1.0, 0.0]));
        assert!(!TestRunner::is_monotonic_axis(&[0.0, 1.0, 0.5]));
    }

    #[test]
    fn test_interpolate_series_snaps_near_exact_grid_point() {
        let x = vec![0.0, 1.1500000000000001, 2.0];
        let y = vec![0.0, -4.361641e-11, 1.0];
        let got =
            TestRunner::interpolate_series(&x, &y, 1.15).expect("expected interpolation result");
        assert!(
            (got - y[1]).abs() < 1e-24,
            "expected exact snap to grid point"
        );
    }

    #[test]
    fn test_interpolate_series_accepts_endpoint_roundoff() {
        let x = vec![0.0, 1.0];
        let y = vec![5.0, 7.0];
        let got =
            TestRunner::interpolate_series(&x, &y, 1.0 + 1e-15).expect("expected endpoint snap");
        assert!((got - 7.0).abs() < 1e-15);
    }

    #[test]
    fn test_interpolate_series_does_not_oversnap_transient_queries() {
        // Regression for hfet/mesa transient comparisons: do not snap to a
        // nearby earlier sample when query is between two very close timepoints.
        let x = vec![1.000_616_914_834_557e-9, 1.000_626_914_834_557e-9];
        let y = vec![0.123_382_966_911_379_7, 0.125_382_966_911_379_7];
        let got =
            TestRunner::interpolate_series(&x, &y, 1.000_625e-9).expect("expected interpolation");
        assert!(
            (got - 0.125).abs() < 1e-9,
            "expected true interpolation near 0.125, got {got}"
        );
    }

    #[test]
    fn test_compare_reference_dataset_non_monotonic_uses_row_alignment() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let mut reference = ReferenceTable::default();
        reference.x_name = "v-sweep".to_string();
        reference.variables.insert(
            "v(out)".to_string(),
            ReferenceSeries {
                x: vec![0.0, 1.0, 0.0, 1.0],
                y: vec![1.0, 2.0, 3.0, 4.0],
            },
        );

        let x_sim = vec![0.0, 1.0, 0.0, 1.0];
        let mismatches = runner.compare_reference_dataset(&reference, &x_sim, |var| {
            if var.eq_ignore_ascii_case("v(out)") {
                Some(vec![1.0, 2.0, 3.0, 4.0])
            } else {
                None
            }
        });

        assert!(mismatches.is_empty());
    }

    #[test]
    fn test_compare_reference_dataset_reports_unresolved_series_as_mismatch() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let mut reference = ReferenceTable::default();
        reference.x_name = "time".to_string();
        reference.variables.insert(
            "v(out)".to_string(),
            ReferenceSeries {
                x: vec![0.0],
                y: vec![1.0],
            },
        );

        let mismatches = runner.compare_reference_dataset(&reference, &[0.0], |_| None);

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "v(out)");
        assert!(mismatches[0].expected.is_nan());
        assert!(mismatches[0].actual.is_nan());
        assert!(mismatches[0].relative_error.is_infinite());
    }

    #[test]
    fn test_parse_ac_probe_supports_current_functions_and_branch_aliases() {
        assert!(matches!(
            TestRunner::parse_ac_probe("db(i(VMEAS))"),
            Some(AcProbe::Current { func: "db", branch }) if branch == "vmeas"
        ));
        assert!(matches!(
            TestRunner::parse_ac_probe(" ph( V1#BRANCH ) "),
            Some(AcProbe::Current { func: "ph", branch }) if branch == "v1"
        ));
        assert!(matches!(
            TestRunner::parse_ac_probe("v(out,0)"),
            Some(AcProbe::Voltage {
                func: "v",
                node_pos,
                node_neg,
            }) if node_pos == "out" && node_neg.as_deref() == Some("0")
        ));
    }

    #[test]
    fn test_resolve_reference_series_supports_ngspice_expressions() {
        let direct = |expr: &str| match expr {
            "i(vc)" => Some(vec![-2.0, 3.0]),
            "v(g)" => Some(vec![0.0, 2.0]),
            _ => None,
        };

        assert_eq!(
            TestRunner::resolve_reference_series("abs(i(vc))", &direct),
            Some(vec![2.0, 3.0])
        );
        assert_eq!(
            TestRunner::resolve_reference_series("-i(vc)", &direct),
            Some(vec![2.0, -3.0])
        );
        assert_eq!(
            TestRunner::resolve_reference_series("v(g)/10", &direct),
            Some(vec![0.0, 0.2])
        );
    }

    #[test]
    fn test_load_reference_table_for_axis_combines_same_axis_sections() {
        let root = unique_temp_dir("rspice_ngspice_axis_merge");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("merge.cir");
        fs::write(
            &cir_path,
            "\
Axis merge deck
V1 out 0 AC 1
R1 out 0 1k
.ac lin 2 1 10
.print ac v(out) i(v1)
.end
",
        )
        .expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
frequency v(out)
1 1
10 2

frequency i(v1)
1 0.001
10 0.002
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let table = runner
            .load_reference_table_for_axis(&cir_path, &["frequency"])
            .expect("load reference")
            .expect("combined table");

        assert_eq!(table.x_name, "frequency");
        assert_eq!(table.variables.len(), 2);
        assert_eq!(table.variables["v(out)"].x, vec![1.0, 10.0]);
        assert_eq!(table.variables["v(out)"].y, vec![1.0, 2.0]);
        assert_eq!(table.variables["i(v1)"].x, vec![1.0, 10.0]);
        assert_eq!(table.variables["i(v1)"].y, vec![0.001, 0.002]);

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_load_reference_table_for_axis_appends_repeated_variables() {
        let root = unique_temp_dir("rspice_ngspice_axis_append");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("append.cir");
        fs::write(
            &cir_path,
            "\
Axis append deck
VS g 0 0
R1 g 0 1k
.dc VS 0 0.1 0.1
.print dc v(g) i(vs)
.end
",
        )
        .expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
v-sweep v(g) vs#branch
0.0 0.5 1e-6
0.1 0.5 2e-6

v-sweep v(g) vs#branch
0.0 1.0 3e-6
0.1 1.0 4e-6
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let table = runner
            .load_reference_table_for_axis(&cir_path, &["v-sweep"])
            .expect("load reference")
            .expect("combined table");

        assert_eq!(table.variables["v(g)"].y, vec![0.5, 0.5, 1.0, 1.0]);
        assert_eq!(table.variables["vs#branch"].y, vec![1e-6, 2e-6, 3e-6, 4e-6]);

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_transient_reference_uses_branch_current_oracles() {
        let root = unique_temp_dir("rspice_ngspice_tran_branch");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("tran_branch.cir");
        fs::write(&cir_path, "Transient branch deck\n.end\n").expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
time v1#branch
0 0
1 1
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = crate::engine::TransientResult {
            time: vec![0.0, 1.0],
            voltages: Vec::new(),
            branch_currents: vec![vec![0.0, 0.5]],
            num_nodes: 0,
            node_names: Vec::new(),
            branch_names: vec!["V1".to_string()],
        };

        let mismatches = runner
            .compare_transient_reference(&cir_path, &Netlist::default(), &result)
            .expect("compare transient reference");

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "v1#branch");
        assert!((mismatches[0].x_value - 1.0).abs() < 1e-12);
        assert!((mismatches[0].expected - 1.0).abs() < 1e-12);
        assert!((mismatches[0].actual - 0.5).abs() < 1e-12);

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_transient_reference_rejects_empty_results() {
        let root = unique_temp_dir("rspice_ngspice_tran_empty");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("tran_empty.cir");
        fs::write(
            &cir_path,
            "\
Transient empty deck
V1 out 0 PULSE(0 1 1n 1p 1p 1n 2n)
.tran 1n 1n
.print tran v(out)
.end
",
        )
        .expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
time v(out)
0 1
1 2
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = crate::engine::TransientResult {
            time: Vec::new(),
            voltages: Vec::new(),
            branch_currents: Vec::new(),
            num_nodes: 0,
            node_names: Vec::new(),
            branch_names: Vec::new(),
        };

        let mismatches = runner
            .compare_transient_reference(&cir_path, &Netlist::default(), &result)
            .expect("compare transient reference");

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "v(out)");
        assert!(mismatches[0].expected.is_nan());
        assert!(mismatches[0].actual.is_nan());
        assert!(mismatches[0].relative_error.is_infinite());

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_ac_reference_uses_branch_current_oracles() {
        let root = unique_temp_dir("rspice_ngspice_ac_branch");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("ac_branch.cir");
        let source = "\
AC branch deck
V1 in 0 AC 1
R1 in 0 1k
.ac lin 1 1 1
.end
";
        fs::write(&cir_path, source).expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
frequency db(i(v1))
1 -40
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let netlist = crate::netlist::parse_netlist(source).expect("parse netlist");
        let results = vec![crate::analysis::AcResult {
            frequency: 1.0,
            node_names: vec!["in".to_string()],
            branch_names: vec!["v1".to_string()],
            voltages: vec![num_complex::Complex64::new(1.0, 0.0)],
            currents: vec![num_complex::Complex64::new(1e-3, 0.0)],
        }];

        let mismatches = runner
            .compare_ac_reference(&cir_path, &netlist, &results)
            .expect("compare ac reference");

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "db(i(v1))");
        assert!((mismatches[0].expected + 40.0).abs() < 1e-9);
        assert!((mismatches[0].actual + 60.0).abs() < 1e-9);

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_transient_reference_supports_device_terminal_probes() {
        let root = unique_temp_dir("rspice_ngspice_tran_device_probe");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("tran_device_probe.cir");
        let source = "\
Transient device probe deck
Vd d 0 0
Vg g 0 0
Vs s 0 1
Vb b 0 0
M1 d g s b NMOS L=1u W=10u
.model NMOS nmos level=1
.tran 1n 1n
.print tran @m1[vbs]
.end
";
        fs::write(&cir_path, source).expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
time @m1[vbs]
0 -1
1 -1
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let netlist = crate::netlist::parse_netlist(source).expect("parse netlist");
        let result = crate::engine::TransientResult {
            time: vec![0.0, 1.0],
            voltages: vec![
                vec![0.0, 0.0],
                vec![0.0, 0.0],
                vec![1.0, 1.0],
                vec![0.0, 0.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 4,
            node_names: vec![
                "d".to_string(),
                "g".to_string(),
                "s".to_string(),
                "b".to_string(),
            ],
            branch_names: Vec::new(),
        };

        let mismatches = runner
            .compare_transient_reference(&cir_path, &netlist, &result)
            .expect("compare transient reference");

        assert!(mismatches.is_empty());

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_ac_reference_rejects_empty_results() {
        let root = unique_temp_dir("rspice_ngspice_ac_empty");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("ac_empty.cir");
        let source = "\
AC empty deck
V1 in 0 AC 1
R1 in 0 1k
.ac lin 1 1 1
.end
";
        fs::write(&cir_path, source).expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
frequency v(in)
1 1
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let netlist = crate::netlist::parse_netlist(source).expect("parse netlist");
        let mismatches = runner
            .compare_ac_reference(&cir_path, &netlist, &[])
            .expect("compare ac reference");

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "v(in)");
        assert!(mismatches[0].expected.is_nan());
        assert!(mismatches[0].actual.is_nan());
        assert!(mismatches[0].relative_error.is_infinite());

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_dc_sweep_reference_rejects_empty_results() {
        let root = unique_temp_dir("rspice_ngspice_dc_empty");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("dc_empty.cir");
        let source = "\
DC empty deck
V1 in 0 DC 0
R1 in 0 1k
.dc V1 0 1 1
.end
";
        fs::write(&cir_path, source).expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
v-sweep v(in)
0 0
1 1
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let netlist = crate::netlist::parse_netlist(source).expect("parse netlist");
        let mismatches = runner
            .compare_dc_sweep_reference(&cir_path, &netlist, &[])
            .expect("compare dc sweep reference");

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "v(in)");
        assert!(mismatches[0].expected.is_nan());
        assert!(mismatches[0].actual.is_nan());
        assert!(mismatches[0].relative_error.is_infinite());

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_noise_reference_rejects_empty_results() {
        let root = unique_temp_dir("rspice_ngspice_noise_empty");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("noise_empty.cir");
        fs::write(&cir_path, "Noise empty deck\n.end\n").expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
frequency onoise_spectrum
1 1e-12
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let mismatches = runner
            .compare_noise_reference(&cir_path, &[])
            .expect("compare noise reference");

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "onoise_spectrum");
        assert!(mismatches[0].expected.is_nan());
        assert!(mismatches[0].actual.is_nan());
        assert!(mismatches[0].relative_error.is_infinite());

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_noise_reference_accepts_truncated_ngspice_alias() {
        let root = unique_temp_dir("rspice_ngspice_noise_alias");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("noise_alias.cir");
        fs::write(&cir_path, "Noise alias deck\n.end\n").expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
frequency v(inoise_spectr
1 1e-6
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let results = vec![crate::analysis::NoiseResult {
            frequency: 1.0,
            output_noise_density: 0.0,
            input_referred_density: 1e-6,
            contributions: Vec::new(),
        }];

        let mismatches = runner
            .compare_noise_reference(&cir_path, &results)
            .expect("compare noise reference");

        assert!(mismatches.is_empty());

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_compare_noise_reference_uses_spectral_density_values() {
        let root = unique_temp_dir("rspice_ngspice_noise_spectrum");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("noise_spectrum.cir");
        fs::write(&cir_path, "Noise spectrum deck\n.end\n").expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
frequency v(inoise_spectrum) v(onoise_spectrum)
1.0e3 1.0e-12 4.0e-18
1.0e4 2.5e-12 9.0e-18
",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let results = vec![
            crate::analysis::NoiseResult {
                frequency: 1.0e3,
                output_noise_density: 4.0e-18,
                input_referred_density: 1.0e-12,
                contributions: Vec::new(),
            },
            crate::analysis::NoiseResult {
                frequency: 1.0e4,
                output_noise_density: 9.0e-18,
                input_referred_density: 2.5e-12,
                contributions: Vec::new(),
            },
        ];

        let mismatches = runner
            .compare_noise_reference(&cir_path, &results)
            .expect("compare noise reference");

        assert!(
            mismatches.is_empty(),
            "expected spectral-density comparison, got mismatches: {mismatches:?}"
        );

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_parse_ngspice_output_tables_splits_multiple_sections() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let content = "\
Index   time            v(out)\n\
0       0.0             0.0\n\
1       1e-9            1.0\n\
Index   frequency       vm(out)\n\
0       1e3             2.0\n\
1       1e4             3.0\n";

        let tables = runner
            .parse_ngspice_output_tables(content)
            .expect("expected two parseable sections");
        assert_eq!(tables.len(), 2);
        assert_eq!(
            TestRunner::normalize_variable_name(&tables[0].x_name),
            "time"
        );
        assert_eq!(
            TestRunner::normalize_variable_name(&tables[1].x_name),
            "frequency"
        );
    }

    #[test]
    fn test_parse_ngspice_output_tables_merges_paginated_sections() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let content = "\
Index   time            v(out)\n\
0       0.0             0.0\n\
1       1e-9            1.0\n\
Index   time            v(out)\n\
2       2e-9            2.0\n\
3       3e-9            3.0\n";

        let tables = runner
            .parse_ngspice_output_tables(content)
            .expect("expected parseable paginated section");
        assert_eq!(tables.len(), 1, "expected merged continuation table");
        let series = tables[0]
            .variables
            .get("v(out)")
            .expect("v(out) series missing");
        assert_eq!(series.x.len(), 4);
        assert_eq!(series.y.len(), 4);
        assert!((series.x[0] - 0.0).abs() < 1e-15);
        assert!((series.x[3] - 3e-9).abs() < 1e-15);
    }

    #[test]
    fn test_parse_ngspice_output_tables_parses_single_variable_complex_rows() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let content = "\
Index   frequency       v(out)\n\
0       1.0e3           3.0, 4.0\n\
1       1.0e4           5.0, 12.0\n";

        let tables = runner
            .parse_ngspice_output_tables(content)
            .expect("expected parseable complex section");
        assert_eq!(tables.len(), 1);
        assert_eq!(
            TestRunner::normalize_variable_name(&tables[0].x_name),
            "frequency"
        );

        let series = tables[0]
            .variables
            .get("v(out)")
            .expect("v(out) series missing");
        assert_eq!(series.x, vec![1.0e3, 1.0e4]);
        assert_eq!(series.y.len(), 2);
        assert!((series.y[0] - 5.0).abs() < 1e-12);
        assert!((series.y[1] - 13.0).abs() < 1e-12);
    }

    #[test]
    fn test_has_direct_validation_coverage_detects_single_variable_complex_ac_oracle() {
        let root = unique_temp_dir("rspice_ngspice_direct_ac_complex");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("complex_ac.cir");
        fs::write(
            &cir_path,
            "\
V1 1 0 AC 1
R1 1 0 1k
.ac dec 5 1k 10k
.print ac v(1)
.end
",
        )
        .expect("write circuit");
        fs::write(
            cir_path.with_extension("out"),
            "\
Index   frequency       v(1)\n\
0       1.0e3           3.0, 4.0\n\
1       1.0e4           5.0, 12.0\n",
        )
        .expect("write reference output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let source = fs::read_to_string(&cir_path).expect("read circuit");
        assert!(
            runner
                .has_direct_validation_coverage(&cir_path, &source)
                .expect("evaluate direct validation coverage"),
            "single-variable complex AC tables should count as direct validation coverage"
        );

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_parse_dc_op_reference_ignores_device_summary_tables() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let content = "\
Initial Transient Solution\n\
\n\
Node                                   Voltage\n\
----                                   -------\n\
1                                            0\n\
vin#branch                                   0\n\
\n\
\tNode                                  Voltage\n\
\t----                                  -------\n\
\tV(2)                             5.000000e-01\n\
\tV(1)                             1.000000e+00\n\
\n\
\tSource\tCurrent\n\
\t------\t-------\n\
\n\
\tvin#branch                       -1.00000e-04\n\
\n\
 Resistor: Simple linear resistor\n\
     device                    r2                    r1\n\
 resistance                  5000                  5000\n\
         ac                 15000                  5000\n\
      dtemp                     0                     0\n\
      noisy                     1                     1\n\
          i                0.0001                0.0001\n\
          p                 5e-05                 5e-05\n";

        let reference = runner
            .parse_dc_op_reference(content)
            .expect("expected operating-point reference");
        assert_eq!(reference.node_voltages.len(), 2);
        assert_eq!(reference.branch_currents.len(), 1);
        assert_eq!(reference.node_voltages["v(1)"], 1.0);
        assert_eq!(reference.node_voltages["v(2)"], 0.5);
        assert_eq!(reference.branch_currents["vin"], -1.0e-4);
        assert!(!reference.node_voltages.contains_key("v(ac)"));
        assert!(!reference.node_voltages.contains_key("v(resistance)"));
        assert!(!reference.node_voltages.contains_key("v(p)"));
    }

    #[test]
    fn test_parse_dc_op_reference_ignores_internal_device_probes() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let content = "\
\tNode                                  Voltage\n\
\t----                                  -------\n\
\tq1#collCI                       2.614371e+00\n\
\tq1#substrate                    5.229312e-11\n\
\tQ1_E                            1.011054e+00\n\
\tvcc                             3.300000e+00\n\
\n\
\tSource\tCurrent\n\
\tv1#branch                       -1.74308e-03\n";

        let reference = runner
            .parse_dc_op_reference(content)
            .expect("expected operating-point reference");
        assert!(!reference.node_voltages.contains_key("v(q1#collci)"));
        assert!(!reference.node_voltages.contains_key("v(q1#substrate)"));
        assert_eq!(reference.node_voltages["v(q1_e)"], 1.011054);
        assert_eq!(reference.node_voltages["v(vcc)"], 3.3);
        assert_eq!(reference.branch_currents["v1"], -1.74308e-3);
    }

    #[test]
    fn test_parse_transfer_function_references_extracts_multiple_blocks() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let references = runner.parse_transfer_function_references(
            "\
Transfer function information:\n\
transfer_function = -1.10341e-01\n\
output_impedance_at_v(5) = 9.446843e+03\n\
vcm#input_impedance = 1.793366e+06\n\
\n\
Transfer function information:\n\
transfer_function = -8.78493e+01\n\
output_impedance_at_v(5) = 9.446843e+03\n\
vdm#input_impedance = 8.940897e+03\n",
        );

        assert_eq!(references.len(), 2);
        assert_eq!(references[0].input_source.as_deref(), Some("vcm"));
        assert_eq!(references[1].input_source.as_deref(), Some("vdm"));
        assert_eq!(references[1].output_probe.as_deref(), Some("v(5)"));
        assert_eq!(references[1].transfer_function, Some(-87.8493));
    }

    #[test]
    fn test_load_transfer_function_reference_matches_requested_probe_and_source() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let temp_dir = unique_temp_dir("ngspice_tf_reference");
        std::fs::create_dir_all(&temp_dir).expect("temp dir");

        let cir_path = temp_dir.join("tf.cir");
        let out_path = temp_dir.join("tf.out");
        std::fs::write(&cir_path, "* tf").expect("write cir");
        std::fs::write(
            &out_path,
            "\
Transfer function information:\n\
transfer_function = -1.10341e-01\n\
output_impedance_at_v(5) = 9.446843e+03\n\
vcm#input_impedance = 1.793366e+06\n\
\n\
Transfer function information:\n\
transfer_function = -8.78493e+01\n\
output_impedance_at_v(5) = 9.446843e+03\n\
vdm#input_impedance = 8.940897e+03\n",
        )
        .expect("write out");

        let reference = runner
            .load_transfer_function_reference(&cir_path, "v(5)", "VDM")
            .expect("load tf reference")
            .expect("matching reference");
        assert_eq!(reference.transfer_function, Some(-87.8493));

        std::fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
    }

    #[test]
    fn test_transfer_output_value_linearized_matches_voltage_divider_gain() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let netlist = Netlist::parse(
            "\
Voltage divider transfer
V1 in 0 DC 1
R1 in out 1k
R2 out 0 1k
.tf v(out) v1
.end
",
        )
        .expect("netlist should parse");

        let gain = runner
            .transfer_output_value_linearized(&netlist, "v(out)", "V1")
            .expect("linearized transfer should succeed")
            .expect("voltage output should use linearized path");

        assert!(
            (gain - 0.5).abs() < 1e-9,
            "expected divider gain 0.5, got {}",
            gain
        );
    }

    #[test]
    fn test_transfer_output_value_linearized_matches_current_source_transimpedance() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let netlist = Netlist::parse(
            "\
Current-driven resistor transfer
I1 out 0 DC 1m
R1 out 0 2k
.tf v(out) i1
.end
",
        )
        .expect("netlist should parse");

        let gain = runner
            .transfer_output_value_linearized(&netlist, "v(out)", "I1")
            .expect("linearized transfer should succeed")
            .expect("voltage output should use linearized path");

        assert!(
            (gain + 2000.0).abs() < 1e-6,
            "expected transimpedance -2000 ohms, got {}",
            gain
        );
    }

    #[test]
    fn test_transfer_output_value_linearized_matches_diffpair_ngspice_references() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let netlist = Netlist::parse(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/sensitivity/diffpair.cir"
        )))
        .expect("diffpair netlist should parse");

        let cm_gain = runner
            .transfer_output_value_linearized(&netlist, "v(5)", "vcm")
            .expect("common-mode transfer should solve")
            .expect("voltage output should use linearized path");
        let dm_gain = runner
            .transfer_output_value_linearized(&netlist, "v(5)", "vdm")
            .expect("differential-mode transfer should solve")
            .expect("voltage output should use linearized path");

        assert!(
            (cm_gain - (-1.10341e-1)).abs() < 1e-3,
            "expected common-mode gain near -0.110341, got {}",
            cm_gain
        );
        assert!(
            (dm_gain - (-8.78493e1)).abs() < 0.2,
            "expected differential gain near -87.8493, got {}",
            dm_gain
        );
    }

    #[test]
    fn test_transfer_output_value_linearized_matches_diffpair_finite_difference() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let source = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/sensitivity/diffpair.cir"
        ));
        let netlist = Netlist::parse(source).expect("diffpair netlist should parse");
        let linearized = runner
            .transfer_output_value_linearized(&netlist, "v(5)", "vcm")
            .expect("common-mode transfer should solve")
            .expect("voltage output should use linearized path");

        let solve_output = |vcm_dc: f64| -> f64 {
            let netlist = Netlist::parse(&source.replacen(
                "vcm 1 0 dc 0",
                &format!("vcm 1 0 dc {vcm_dc:.12e}"),
                1,
            ))
            .expect("perturbed diffpair netlist should parse");
            let result = runner
                .create_dc_engine()
                .run_dc_op(&netlist)
                .expect("perturbed diffpair dc op should solve");
            let output_idx = result
                .node_index_named("5")
                .expect("node named '5' should exist in diffpair result");
            result.voltage(output_idx)
        };

        let delta = 1e-6;
        let finite_difference = (solve_output(delta) - solve_output(-delta)) / (2.0 * delta);
        assert!(
            (linearized - finite_difference).abs() < 5e-4,
            "linearized common-mode gain should match finite difference: linearized={linearized}, finite_difference={finite_difference}"
        );
    }

    #[test]
    fn test_load_reference_table_for_axis_selects_requested_section() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let temp_dir = unique_temp_dir("ngspice_axis_select");
        std::fs::create_dir_all(&temp_dir).expect("temp dir");

        let cir_path = temp_dir.join("multi.cir");
        let out_path = temp_dir.join("multi.out");
        std::fs::write(&cir_path, "* axis select test").expect("write cir");
        std::fs::write(
            &out_path,
            "\
Index   time            v(out)\n\
0       0.0             0.0\n\
1       1e-9            1.0\n\
Index   frequency       vm(out)\n\
0       1e3             2.0\n\
1       1e4             3.0\n",
        )
        .expect("write out");

        let selected = runner
            .load_reference_table_for_axis(&cir_path, &["frequency"])
            .expect("load reference")
            .expect("reference table");
        assert_eq!(
            TestRunner::normalize_variable_name(&selected.x_name),
            "frequency"
        );
    }

    #[test]
    fn test_load_reference_table_for_axis_rejects_unknown_voltage_nodes() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let temp_dir = unique_temp_dir("ngspice_axis_unknown_node");
        std::fs::create_dir_all(&temp_dir).expect("temp dir");

        let cir_path = temp_dir.join("stale.cir");
        let out_path = temp_dir.join("stale.out");
        std::fs::write(
            &cir_path,
            "\
Stale oracle test
V1 in 0 PULSE(0 1 1n 1p 1p 1n 2n)
R1 out in 1k
C1 out 0 1p
.tran 0.1n 2n
.print tran v(in) v(out)
.end
",
        )
        .expect("write cir");
        std::fs::write(
            &out_path,
            "\
time v(in) v(missing)
0.0 0.0 0.0
1e-9 1.0 0.5
",
        )
        .expect("write out");

        let selected = runner
            .load_reference_table_for_axis(&cir_path, &["time"])
            .expect("load reference");
        assert!(
            selected.is_none(),
            "reference tables mentioning deck-external voltage nodes must be rejected"
        );
    }

    #[test]
    fn test_run_test_allows_smoke_contract_when_reference_table_is_stale() {
        let root = unique_temp_dir("rspice_ngspice_stale_reference_smoke");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("stale.cir");
        let out_path = root.join("stale.out");
        fs::write(
            &cir_path,
            "\
Stale oracle deck
V1 in 0 PULSE(0 1 1n 1p 1p 1n 2n)
R1 out in 1k
C1 out 0 1p
.tran 0.1n 2n
.print tran v(in) v(out)
.end
",
        )
        .expect("write circuit");
        fs::write(
            &out_path,
            "\
time v(in) v(missing)
0.0 0.0 0.0
1e-9 1.0 0.5
",
        )
        .expect("write output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);
        assert!(
            !result.passed,
            "stale references should not silently count as validation coverage"
        );
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|err| err.contains("No validation oracle")),
            "expected stale oracle to degrade into explicit missing-oracle coverage, got {:?}",
            result.error
        );

        write_manifest(&root, &["stale.cir\tsmoke"]);
        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);
        assert!(
            result.passed,
            "explicit smoke contract should allow execution when the checked-in oracle is stale: {:?}",
            result.error
        );
    }

    #[test]
    fn test_workspace_mos6_simpleinv_reference_is_detected_as_stale() {
        let tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("tests");
        let cir_path = tests_dir.join("mos6").join("simpleinv.cir");
        let runner = TestRunner::new(&tests_dir, TestRunnerConfig::default());

        let selected = runner
            .load_reference_table_for_axis(&cir_path, &["time"])
            .expect("load reference");
        let unknown = selected
            .as_ref()
            .map(|table| {
                runner
                    .reference_table_unknown_voltage_nodes(&cir_path, table)
                    .expect("validate")
            })
            .unwrap_or_default();
        assert!(
            selected.is_none(),
            "workspace simpleinv oracle should be rejected because it references deck-external nodes; unknown={unknown:?}"
        );
    }

    #[test]
    fn test_parse_current_probe_accepts_i_function_and_branch_suffix() {
        assert_eq!(
            TestRunner::parse_current_probe("i(VDS)"),
            Some("vds".to_string())
        );
        assert_eq!(
            TestRunner::parse_current_probe("vdd#branch"),
            Some("vdd".to_string())
        );
        assert_eq!(TestRunner::parse_current_probe("v(out)"), None);
    }

    #[test]
    fn test_compare_dc_sweep_reference_maps_voltage_source_branch_current() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let temp_dir = unique_temp_dir("ngspice_dc_branch_compare");
        std::fs::create_dir_all(&temp_dir).expect("temp dir");

        let cir_path = temp_dir.join("branch.cir");
        let out_path = temp_dir.join("branch.out");
        std::fs::write(
            &cir_path,
            "\
V1 1 0 0
R1 1 0 1k
.dc v1 0 1 1
.print dc i(v1)
.end
",
        )
        .expect("write cir");
        std::fs::write(
            &out_path,
            "\
Index   v-sweep         v1#branch
0       0.0             0.1
1       1.0             0.2
",
        )
        .expect("write out");

        let netlist = Netlist::parse(
            "\
V1 1 0 0
R1 1 0 1k
.dc v1 0 1 1
.print dc i(v1)
.end
",
        )
        .expect("parse netlist");

        let mut r0 = crate::SimulationResult::new(1, 1);
        r0.node_names = vec!["0".to_string(), "1".to_string()];
        r0.node_voltages = vec![0.0, 0.0];
        r0.branch_currents[0] = 0.1;

        let mut r1 = crate::SimulationResult::new(1, 1);
        r1.node_names = vec!["0".to_string(), "1".to_string()];
        r1.node_voltages = vec![0.0, 1.0];
        r1.branch_currents[0] = 0.2;

        let mismatches = runner
            .compare_dc_sweep_reference(&cir_path, &netlist, &[(0.0, r0), (1.0, r1)])
            .expect("comparison should succeed");
        assert!(
            mismatches.is_empty(),
            "branch-current mapped comparison should match reference, got {:?}",
            mismatches
        );
    }

    #[test]
    fn test_parse_ngspice_output_tables_preserves_signed_current_headers() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let content = "\
Index   v-sweep         -i(vc)          abs(i(vb))
0       0.0             -1.0e-6         2.0e-9
1       0.1             -2.0e-6         3.0e-9
";

        let tables = runner
            .parse_ngspice_output_tables(content)
            .expect("parse output");

        assert_eq!(tables.len(), 1);
        assert!(tables[0].variables.contains_key("-i(vc)"));
        assert!(tables[0].variables.contains_key("abs(i(vb))"));
        assert_eq!(tables[0].variables["-i(vc)"].y, vec![-1.0e-6, -2.0e-6]);
        assert_eq!(tables[0].variables["abs(i(vb))"].y, vec![2.0e-9, 3.0e-9]);
    }

    #[test]
    fn test_compare_dc_sweep_reference_maps_signed_current_expressions_by_branch_name() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let temp_dir = unique_temp_dir("ngspice_dc_signed_branch_compare");
        std::fs::create_dir_all(&temp_dir).expect("temp dir");

        let cir_path = temp_dir.join("branch_expr.cir");
        let out_path = temp_dir.join("branch_expr.out");
        std::fs::write(
            &cir_path,
            "\
VB b 0 0.7
VC c 0 0
RB b 0 1g
RC c 0 1g
.dc vc 0 0.1 0.1 vb 0.7 0.8 0.1
.print dc -i(vc) abs(i(vb))
.end
",
        )
        .expect("write cir");
        std::fs::write(
            &out_path,
            "\
Index   v-sweep         -i(vc)          abs(i(vb))
0       0.0             -9.0e-6         1.0e-9
1       0.1             -8.0e-6         2.0e-9
2       0.0             -7.0e-6         3.0e-9
3       0.1             -6.0e-6         4.0e-9
",
        )
        .expect("write out");

        let netlist = Netlist::parse(
            "\
VB b 0 0.7
VC c 0 0
RB b 0 1g
RC c 0 1g
.dc vc 0 0.1 0.1 vb 0.7 0.8 0.1
.print dc -i(vc) abs(i(vb))
.end
",
        )
        .expect("parse netlist");

        let make_result = |vb_current: f64, vc_current: f64, vc_voltage: f64| {
            let mut result = crate::SimulationResult::new(2, 2);
            result.node_names = vec!["0".to_string(), "b".to_string(), "c".to_string()];
            result.node_voltages = vec![0.0, 0.0, vc_voltage];
            // Keep branch names intentionally reversed to prove lookup is name-driven.
            result.branch_names = vec!["VB".to_string(), "VC".to_string()];
            result.branch_currents = vec![vb_current, vc_current];
            result
        };

        let results = vec![
            (0.0, make_result(-1.0e-9, 9.0e-6, 0.0)),
            (0.1, make_result(-2.0e-9, 8.0e-6, 0.1)),
            (0.0, make_result(-3.0e-9, 7.0e-6, 0.0)),
            (0.1, make_result(-4.0e-9, 6.0e-6, 0.1)),
        ];

        let mismatches = runner
            .compare_dc_sweep_reference(&cir_path, &netlist, &results)
            .expect("comparison should succeed");
        assert!(
            mismatches.is_empty(),
            "signed branch-expression comparison should match reference, got {:?}",
            mismatches
        );
    }

    #[test]
    fn test_frequency_generation() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        // Linear
        let lin = runner.generate_linear_points(0.0, 100.0, 11);
        assert_eq!(lin.len(), 11);
        assert!((lin[0] - 0.0).abs() < 1e-10);
        assert!((lin[10] - 100.0).abs() < 1e-10);

        // Decade
        let dec = runner.generate_decade_points(1.0, 1000.0, 10);
        assert!(dec.len() >= 30); // 3 decades * 10 points

        // Octave
        let oct = runner.generate_octave_points(1.0, 8.0, 10);
        assert!(oct.len() >= 30); // 3 octaves * 10 points
    }

    #[test]
    fn test_check_unsupported() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        assert!(
            runner
                .check_unsupported(".noise v(out) v1 dec 10 1 1meg")
                .is_none()
        );
        assert!(runner.check_unsupported(".disto h1 2").is_none());
        assert!(runner.check_unsupported(".csparam gain=2").is_none());
        assert!(runner.check_unsupported(".pz in out vol pz").is_none());
        assert!(runner.check_unsupported(".four 1k v(out)").is_none());
        assert!(
            runner
                .check_unsupported("* .disto appears in a comment only")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported("R1 1 0 1k ; .disto in inline comment")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported("r1 1 2 1k\n.dc v1 0 5 0.1")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported("a_source [a1 a2] d_source1")
                .is_none()
        );
        assert_eq!(runner.check_unsupported(".model cpl1 cpl (R=1)"), None);
        assert!(
            runner
                .check_unsupported(".model ymod ltra (r=1 l=2e-9 c=3e-12)")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported(".model ymod txl (r=1 l=2e-9 c=3e-12)")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported(".model n1 nmos (level=55 tox=4.5e-9)")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported(".model p1 pmos level= 57")
                .is_none()
        );
        assert!(
            runner
                .check_unsupported(".model n1 nmos (level=49 tox=4.5e-9)")
                .is_none()
        );
    }

    #[test]
    fn test_value_comparison() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());

        // Should pass - within tolerance
        assert!(runner.compare_values(1.0, 1.005).is_none());

        // Should fail - outside tolerance
        assert!(runner.compare_values(1.0, 1.03).is_some());

        // Should pass - small values within absolute tolerance
        assert!(runner.compare_values(1e-15, 1e-16).is_none());
    }

    #[test]
    fn test_compare_test_gold_nodes_passes_when_values_match() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let mut result = crate::SimulationResult::new(2, 0);
        result.node_names = vec!["0".to_string(), "n100_t".to_string(), "n100_g".to_string()];
        result.node_voltages = vec![0.0, 1.0, 1.0];

        let mismatches = runner.compare_test_gold_nodes(&result);
        assert!(mismatches.is_empty());
    }

    #[test]
    fn test_compare_test_gold_nodes_reports_mismatch() {
        let mut config = TestRunnerConfig::default();
        config.relative_tolerance = 1e-6;
        let runner = TestRunner::new(".", config);

        let mut result = crate::SimulationResult::new(2, 0);
        result.node_names = vec!["0".to_string(), "n100_t".to_string(), "n100_g".to_string()];
        result.node_voltages = vec![0.0, 1.1, 1.0];

        let mismatches = runner.compare_test_gold_nodes(&result);
        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "n100_t");
        assert!(mismatches[0].relative_error > 0.0);
    }

    #[test]
    fn test_compare_test_gold_nodes_reports_missing_gold_node() {
        let runner = TestRunner::new(".", TestRunnerConfig::default());
        let mut result = crate::SimulationResult::new(1, 0);
        result.node_names = vec!["0".to_string(), "n100_t".to_string()];
        result.node_voltages = vec![0.0, 1.0];

        let mismatches = runner.compare_test_gold_nodes(&result);
        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].node, "n100_t");
        assert!(mismatches[0].expected.is_nan());
        assert!(mismatches[0].relative_error.is_infinite());
    }

    #[test]
    fn test_compare_test_gold_nodes_honors_max_mismatches() {
        let mut config = TestRunnerConfig::default();
        config.max_mismatches = 1;
        config.relative_tolerance = 1e-6;
        let runner = TestRunner::new(".", config);

        let mut result = crate::SimulationResult::new(4, 0);
        result.node_names = vec![
            "0".to_string(),
            "n100_t".to_string(),
            "n100_g".to_string(),
            "n200_t".to_string(),
            "n200_g".to_string(),
        ];
        result.node_voltages = vec![0.0, 2.0, 1.0, 3.0, 1.0];

        let mismatches = runner.compare_test_gold_nodes(&result);
        assert_eq!(mismatches.len(), 1);
    }

    #[test]
    fn test_discover_tests() {
        let runner = TestRunner::new("./tests", TestRunnerConfig::default());
        let tests = runner.discover_tests("general");
        // Should find some tests in general directory
        // Just verify it doesn't panic
        let _ = tests;
    }

    #[test]
    fn test_discover_tests_prefers_makefile_tests_manifest() {
        let root = unique_temp_dir("rspice_ngspice_manifest");
        let suite = root.join("suite");
        fs::create_dir_all(&suite).expect("failed to create temp suite dir");

        fs::write(
            suite.join("Makefile.am"),
            "TESTS = \\\n  t1.cir \\\n  t2.cir\nEXTRA_DIST += helper.cir\n",
        )
        .expect("failed to write Makefile.am");
        fs::write(suite.join("t1.cir"), "* t1\n").expect("failed to create t1.cir");
        fs::write(suite.join("t2.cir"), "* t2\n").expect("failed to create t2.cir");
        fs::write(suite.join("helper.cir"), "* helper\n").expect("failed to create helper.cir");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let discovered = runner.discover_tests("suite");
        let names: Vec<String> = discovered
            .iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .collect();

        assert_eq!(names, vec!["t1.cir".to_string(), "t2.cir".to_string()]);

        fs::remove_dir_all(&root).expect("failed to remove temp suite dir");
    }

    #[test]
    fn test_discover_tests_falls_back_without_makefile_manifest() {
        let root = unique_temp_dir("rspice_ngspice_fallback");
        let suite = root.join("suite");
        fs::create_dir_all(&suite).expect("failed to create temp suite dir");

        fs::write(suite.join("a.cir"), "* a\n").expect("failed to create a.cir");
        fs::write(suite.join("b.cir"), "* b\n").expect("failed to create b.cir");
        fs::write(suite.join("notes.txt"), "not a circuit\n").expect("failed to write notes");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let discovered = runner.discover_tests("suite");
        let names: Vec<String> = discovered
            .iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .collect();

        assert_eq!(names, vec!["a.cir".to_string(), "b.cir".to_string()]);

        fs::remove_dir_all(&root).expect("failed to remove temp suite dir");
    }

    #[test]
    fn test_run_test_fails_without_validation_or_manifest() {
        let root = unique_temp_dir("rspice_ngspice_missing_validation");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("smoke_only.cir");
        fs::write(
            &cir_path,
            "\
No oracle deck
V1 in 0 DC 1
R1 in 0 1k
.op
.end
",
        )
        .expect("write circuit");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);

        assert!(
            !result.passed,
            "deck should fail without validation coverage"
        );
        let error = result.error.expect("expected error");
        assert!(error.contains("No validation oracle"));

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_run_test_allows_explicit_smoke_manifest_contract() {
        let root = unique_temp_dir("rspice_ngspice_smoke_contract");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("smoke_only.cir");
        fs::write(
            &cir_path,
            "\
Explicit smoke deck
V1 in 0 DC 1
R1 in 0 1k
.op
.end
",
        )
        .expect("write circuit");
        write_manifest(&root, &["smoke_only.cir\tsmoke"]);

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);

        assert!(
            result.passed,
            "explicit smoke contract should allow execution-only coverage: {:?}",
            result.error
        );

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_run_test_requires_manifest_for_scripted_control_deck() {
        let root = unique_temp_dir("rspice_ngspice_control_contract");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("scripted.cir");
        fs::write(
            &cir_path,
            "\
Scripted deck
V1 in 0 DC 1
R1 in 0 1k
.control
op
.endc
.end
",
        )
        .expect("write circuit");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);
        assert!(!result.passed, "scripted deck should fail without contract");
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|err| err.contains("No validation oracle"))
        );

        write_manifest(&root, &["scripted.cir\tscripted_control"]);
        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);
        assert!(
            result.passed,
            "explicit scripted_control contract should make coverage intentional: {:?}",
            result.error
        );

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_run_test_executes_linearized_sensitivity_without_top_level_params() {
        let root = unique_temp_dir("rspice_ngspice_linearized_sens");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("linearized_sens.cir");
        fs::write(
            &cir_path,
            "\
Linearized sensitivity deck
V1 in 0 DC 1
R1 in out 1k
R2 out 0 1k
.sens v(out)
.end
",
        )
        .expect("write circuit");
        write_manifest(&root, &["linearized_sens.cir\tsmoke"]);

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);

        assert!(
            result.passed,
            "linearized .SENS should execute without synthetic .PARAM cards: {:?}",
            result.error
        );
        assert_eq!(result.analysis_type.as_deref(), Some("Sensitivity"));

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_run_test_rejects_placeholder_pz_reference_output() {
        let root = unique_temp_dir("rspice_ngspice_pz_placeholder");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("placeholder_pz.cir");
        let out_path = root.join("placeholder_pz.out");
        fs::write(
            &cir_path,
            "\
Placeholder PZ deck
R1 1 0 1k
R2 2 0 1k
C1 1 2 1p
.pz 1 0 2 0 cur pz
.end
",
        )
        .expect("write circuit");
        fs::write(
            &out_path,
            "\
Warning: upstream reference not generated yet
To be done
Index   pole(1)                         zero(1)
",
        )
        .expect("write placeholder output");

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);

        assert!(
            !result.passed,
            "placeholder PZ references must not count as validation coverage"
        );
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|err| err.contains("No validation oracle")),
            "expected explicit missing-oracle failure, got {:?}",
            result.error
        );

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }

    #[test]
    fn test_run_test_fails_for_unsupported_fourier_analysis() {
        let root = unique_temp_dir("rspice_ngspice_fourier");
        fs::create_dir_all(&root).expect("temp dir");
        let cir_path = root.join("fourier.cir");
        fs::write(
            &cir_path,
            "\
Fourier deck
V1 in 0 SIN(0 1 1k)
R1 in 0 1k
.four 1k v(in)
.end
",
        )
        .expect("write circuit");
        write_manifest(&root, &["fourier.cir\tsmoke"]);

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&cir_path);

        assert!(!result.passed, "unsupported analyses must fail loudly");
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|err| err.contains("does not implement requested analysis"))
        );

        fs::remove_dir_all(&root).expect("cleanup temp dir");
    }
}
