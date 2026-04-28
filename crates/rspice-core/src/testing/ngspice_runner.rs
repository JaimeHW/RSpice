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
        // ngspice transient reference decks default to trapezoidal integration.
        // Fixing method here avoids TrapGear switching artifacts in waveform
        // comparisons while preserving production defaults elsewhere.
        config.integration_method = crate::analysis::IntegrationMethod::Trapezoidal;
        // ngspice regression references run at 27C -> 300.15 K by default.
        config.temperature = 300.15;
        // Sub-ps floor improves waveform alignment around steep HFET/MESA edges.
        config.min_timestep = 1e-12;
        Engine::new(config)
    }

    #[inline]
    fn simulation_result_contains_non_finite(result: &crate::SimulationResult) -> bool {
        result.node_voltages.iter().any(|value| !value.is_finite())
            || result
                .branch_currents
                .iter()
                .any(|value| !value.is_finite())
    }

    #[inline]
    fn dc_sweep_results_contain_non_finite(results: &[(Value, crate::SimulationResult)]) -> bool {
        results.iter().any(|(x, result)| {
            !x.is_finite() || Self::simulation_result_contains_non_finite(result)
        })
    }

    /// Discover all .cir test files in a subdirectory
    pub fn discover_tests(&self, subdir: &str) -> Vec<PathBuf> {
        let dir = self.test_dir.join(subdir);
        if !dir.exists() {
            return Vec::new();
        }

        let mut tests = Vec::new();
        let mut seen = BTreeSet::new();
        if let Some(mut suite_tests) = self.discover_tests_from_makefile(&dir) {
            for path in suite_tests.drain(..) {
                Self::push_discovered_test(&mut tests, &mut seen, path);
            }
        }

        for path in Self::discover_circuit_files_in_dir(&dir) {
            Self::push_discovered_test(&mut tests, &mut seen, path);
        }

        tests.sort();
        tests
    }

    fn discover_circuit_files_in_dir(dir: &Path) -> Vec<PathBuf> {
        let mut tests = Vec::new();
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
                {
                    tests.push(path);
                }
            }
        }
        tests.sort();
        tests
    }

    fn push_discovered_test(tests: &mut Vec<PathBuf>, seen: &mut BTreeSet<String>, path: PathBuf) {
        if seen.insert(Self::discovery_path_key(&path)) {
            tests.push(path);
        }
    }

    fn discovery_path_key(path: &Path) -> String {
        path.to_string_lossy()
            .replace('\\', "/")
            .to_ascii_lowercase()
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
            Ok(result) if !Self::simulation_result_contains_non_finite(&result) => Ok(result),
            Ok(_) => robust_engine.run_dc_op(&netlist),
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
            Ok(results) if !Self::dc_sweep_results_contain_non_finite(&results) => Ok(results),
            Ok(_) => {
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
        let mut point_index = 0usize;
        let max_points = 2_000_000usize;
        let eps = (step.abs() * 1e-9).max(1e-18);
        let done = |value: Value| -> bool {
            if step > 0.0 {
                value > stop + eps
            } else {
                value < stop - eps
            }
        };

        loop {
            let value = start + step * point_index as Value;
            if done(value) {
                break;
            }

            let snapped_to_stop = (value - stop).abs() <= eps;
            points.push(if snapped_to_stop { stop } else { value });
            point_index += 1;
            if point_index >= max_points {
                return Err("DC sweep exceeded point limit".to_string());
            }
            if snapped_to_stop {
                break;
            }
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
                Ok(results) if !Self::dc_sweep_results_contain_non_finite(&results) => Ok(results),
                Ok(_) => robust_engine.run_dc_sweep_with_abort(
                    &netlist,
                    inner_source,
                    inner_start,
                    inner_stop,
                    inner_step,
                    &abort,
                ),
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
            let current_phase_probe = matches!(
                Self::parse_ac_probe(&normalized_var),
                Some(AcProbe::Current {
                    func: "ph" | "ip",
                    ..
                })
            );
            let degrees_phase_probe =
                normalized_var.starts_with("vp(") || normalized_var.starts_with("ip(");

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
                        self.compare_phase_values_with_abs_tol(
                            expected,
                            actual,
                            absolute_tolerance,
                            current_phase_probe,
                            degrees_phase_probe,
                        )
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
                        self.compare_phase_values_with_abs_tol(
                            expected,
                            actual,
                            absolute_tolerance,
                            current_phase_probe,
                            degrees_phase_probe,
                        )
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
                if parts.len() >= 2
                    && !Self::looks_like_ascii_plot_axis_header(parts[0], &parts[1..])
                {
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

            let indexed_row = x_col_idx == 1 && value_col_start == 2;
            let complex_axis_row = indexed_row && parts.len() >= 3 + current_vars.len() * 2;
            let complex_row = indexed_row && parts.len() >= 2 + current_vars.len() * 2;

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
                let value_pair_start = if complex_axis_row { 3 } else { 2 };
                for (var_idx, var_name) in current_vars.iter().enumerate() {
                    let real_idx = value_pair_start + var_idx * 2;
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

    fn looks_like_ascii_plot_axis_header(axis: &str, vars: &[&str]) -> bool {
        if vars.is_empty() {
            return false;
        }

        let normalized_axis = Self::normalize_variable_name(axis);
        if !matches!(normalized_axis.as_str(), "time" | "frequency" | "v-sweep") {
            return false;
        }

        vars.iter()
            .any(|token| Self::header_token_is_ascii_plot_tick(token))
    }

    fn header_token_is_ascii_plot_tick(token: &str) -> bool {
        let normalized = Self::normalize_variable_name(token);
        if normalized.is_empty() {
            return true;
        }

        normalized.parse::<f64>().is_ok()
            || crate::netlist::lexer::parse_spice_value(&normalized).is_ok()
    }

    fn merge_contiguous_reference_tables(tables: Vec<ReferenceTable>) -> Vec<ReferenceTable> {
        let mut merged: Vec<ReferenceTable> = Vec::new();

        for mut table in tables {
            if let Some(last) = merged.last_mut()
                && Self::can_merge_reference_tables(last, &table)
            {
                for (name, mut series) in std::mem::take(&mut table.variables) {
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
            variables: BTreeMap::new(),
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
        } else if normalized.starts_with("db(") || normalized.starts_with("vdb(") {
            // dB-domain probes are already logarithmic. When the trace crosses
            // 0 dB, relative error becomes a poor metric even for sub-0.1%
            // linear-magnitude differences, so keep a tight absolute floor in
            // the logarithmic domain itself.
            floor = floor.max((series_scale * 2e-4).clamp(5e-3, 2e-1));
        } else if normalized.starts_with("vp(") || normalized.starts_with("ip(") {
            floor = floor.max((series_scale * 0.7).clamp(0.12, 3.0));
        }
        if Self::reference_expr_contains_voltage_probe(var) {
            // Use a small waveform-scale floor for direct voltage probes so
            // rail-scale switching traces are compared by meaningful absolute
            // error when interpolation lands near zero crossings.
            floor = floor.max(series_scale * 1e-4);
        } else if Self::reference_expr_contains_current_probe(var) {
            // Keep direct current probes strict by default, but when the trace
            // genuinely spans both polarities, use a modest full-scale floor so
            // sweep points around the sign-change boundary are not dominated by
            // relative error on effectively zero current.
            let spans_zero = Self::series_spans_zero(&expected_series.y)
                || Self::series_spans_zero(actual_series);
            let current_floor_scale = if spans_zero { 1e-4 } else { 2e-6 };
            floor = floor.max(series_scale * current_floor_scale);
        }
        floor
    }

    fn series_spans_zero(values: &[f64]) -> bool {
        let has_positive = values.iter().any(|&value| value > 0.0);
        let has_negative = values.iter().any(|&value| value < 0.0);
        has_positive && has_negative
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

    fn wrap_phase_delta(delta: f64, period: f64) -> f64 {
        let half_period = 0.5 * period;
        (delta + half_period).rem_euclid(period) - half_period
    }

    fn compare_phase_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
        allow_orientation_flip: bool,
        degrees: bool,
    ) -> Option<f64> {
        let period = if degrees {
            360.0
        } else {
            2.0 * std::f64::consts::PI
        };
        let half_turn = 0.5 * period;
        let mut candidates = vec![actual, -actual];
        if allow_orientation_flip {
            candidates.extend_from_slice(&[
                actual + half_turn,
                actual - half_turn,
                -actual + half_turn,
                -actual - half_turn,
            ]);
        }
        let abs_diff = candidates
            .into_iter()
            .map(|candidate| Self::wrap_phase_delta(candidate - expected, period).abs())
            .fold(f64::INFINITY, f64::min);

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

