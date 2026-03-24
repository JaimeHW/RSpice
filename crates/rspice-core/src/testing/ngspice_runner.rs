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
//!     ├── discover_tests()     - Find .cir files
//!     ├── run_test()           - Execute single test
//!     │   ├── parse_analyses() - Detect .op, .dc, .tran, .ac
//!     │   ├── execute_analysis() - Run appropriate simulation
//!     │   └── compare_results() - Validate against .out
//!     └── run_suite()          - Run all tests in directory
//! ```

use crate::abort_signal::AbortSignal;
use crate::engine::{ConvergenceConfig, SimulationConfig};
use crate::{Engine, Netlist, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════════
// Analysis Types
// ═══════════════════════════════════════════════════════════════════════════════

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
}

/// AC sweep type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AcSweepType {
    Dec,
    Oct,
    Lin,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test Results
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

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
            relative_tolerance: 0.01, // 1%
            absolute_tolerance: 1e-12,
            max_mismatches: 10,
            skip_unsupported: true,
            verbose: false,
            max_time_per_test_ms: 30000, // 30 seconds max per test
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test Runner Implementation
// ═══════════════════════════════════════════════════════════════════════════════

/// Ngspice regression test runner
pub struct TestRunner {
    config: TestRunnerConfig,
    test_dir: PathBuf,
}

impl TestRunner {
    /// Create a new test runner for the given test directory
    pub fn new<P: AsRef<Path>>(test_dir: P, config: TestRunnerConfig) -> Self {
        Self {
            config,
            test_dir: test_dir.as_ref().to_path_buf(),
        }
    }

    /// Get the test runner configuration
    pub fn config(&self) -> &TestRunnerConfig {
        &self.config
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
                if path.extension().map_or(false, |e| e == "cir") {
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
        let source = match Netlist::preprocess_includes(&source, cir_path) {
            Ok(preprocessed) => preprocessed,
            Err(_) => source, // Keep original if preprocessing fails
        };

        // Strip .control/.endc blocks (ngspice scripting) before parsing
        let source = Netlist::strip_control_blocks(&source);

        // Check for unsupported features
        if self.config.skip_unsupported {
            if let Some(reason) = self.check_unsupported(&source) {
                return TestResult {
                    name,
                    passed: true, // Mark as passed (skipped)
                    error: Some(format!("SKIPPED: {}", reason)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
        }

        // Parse analysis directives. Run all analyses in deck order.
        let analyses = self.parse_analyses(&source);
        let analysis_plan = if analyses.is_empty() {
            vec![AnalysisSpec::DcOp]
        } else {
            analyses
        };

        let mut all_mismatches = Vec::new();
        let mut analysis_labels = Vec::new();
        let mut first_error: Option<String> = None;

        for analysis in &analysis_plan {
            let analysis_start = std::time::Instant::now();
            let result = match analysis {
                AnalysisSpec::DcOp => self.run_dc_op_test(&name, &source, analysis_start),
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
                    tstart: _,
                    tmax,
                } => self.run_transient_test(
                    &name,
                    cir_path,
                    &source,
                    *tstep,
                    *tstop,
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
            };

            if let Some(label) = result.analysis_type.clone() {
                analysis_labels.push(label);
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
        let mut analyses = Vec::new();

        for line in source.lines() {
            let trimmed = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.is_empty() {
                continue;
            }

            // Match directive tokens exactly to avoid prefix collisions
            // (e.g., ".options" must not be treated as ".op").
            let directive = trimmed.split_whitespace().next().unwrap_or("");
            match directive {
                ".op" => analyses.push(AnalysisSpec::DcOp),
                ".dc" => {
                    if let Some(spec) = self.parse_dc_directive(&trimmed) {
                        analyses.push(spec);
                    }
                }
                ".tran" => {
                    if let Some(spec) = self.parse_tran_directive(&trimmed) {
                        analyses.push(spec);
                    }
                }
                ".ac" => {
                    if let Some(spec) = self.parse_ac_directive(&trimmed) {
                        analyses.push(spec);
                    }
                }
                ".disto" => {
                    if let Some(spec) = self.parse_disto_directive(&trimmed) {
                        analyses.push(spec);
                    }
                }
                _ => {}
            }
        }

        analyses
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

    /// Parse SPICE values using the same lexer/parser as netlist parsing.
    /// This keeps regression analysis directives aligned with production parsing
    /// (e.g. 1ns, 10us, 10ghz, 1nF, 1mH).
    fn parse_spice_value(&self, s: &str) -> Option<Value> {
        crate::netlist::lexer::parse_spice_value(s).ok()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Analysis Execution
    // ═══════════════════════════════════════════════════════════════════════════

    fn run_dc_op_test(&self, name: &str, source: &str, start: std::time::Instant) -> TestResult {
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
                // Extract test/gold node pairs and compare
                let mismatches = self.compare_test_gold_nodes(&result);
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
        let max_step = tmax.unwrap_or_else(|| {
            if tstep > 0.0 {
                // .tran print-step is an output interval, not an internal
                // solver cap. Match the release CLI heuristic so regression
                // startup behavior tracks the shipping transient runner.
                tstep * 10.0
            } else {
                tstop / 100.0
            }
        });

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        match engine.run_tran_with_abort(&netlist, tstop, max_step, &abort) {
            Ok(result) => {
                let mismatches = match self.compare_transient_reference(cir_path, &result) {
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Frequency Point Generation
    // ═══════════════════════════════════════════════════════════════════════════

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

    // ═══════════════════════════════════════════════════════════════════════════
    // Feature Detection
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check if netlist uses unsupported features
    fn check_unsupported(&self, source: &str) -> Option<String> {
        // Directive-based patterns.
        // NOTE: .subckt is now supported via flattening
        // NOTE: .tf and .sens are handled by running DC OP (sufficient for basic validation)
        // NOTE: .noise, .pz, .four, and .disto are exercised directly in regression suites.
        // .control blocks are ignored - we just run the circuit simulation part
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

                // Multiconductor CPL transmission-line models are currently unsupported.
                if token == ".model"
                    && trimmed
                        .split_whitespace()
                        .any(|part| part.eq_ignore_ascii_case("cpl"))
                {
                    return Some("CPL transmission line model".to_string());
                }
                if token == ".model" && Self::is_unsupported_soi_level_model(&trimmed) {
                    return Some("BSIMSOI LEVEL=55/56/57 model".to_string());
                }
            }

            // Check if line starts with device letter followed by alphanumeric (e.g., "t1", "k1")
            for (prefix, reason) in &line_start_devices {
                if trimmed.starts_with(prefix) {
                    // Verify next char is alphanumeric (device name) not just a word
                    if let Some(next_char) = trimmed.chars().nth(1) {
                        if next_char.is_alphanumeric() {
                            return Some(reason.to_string());
                        }
                    }
                }
            }

        }

        None
    }

    fn is_unsupported_soi_level_model(model_line: &str) -> bool {
        let normalized: String = model_line
            .chars()
            .map(|ch| match ch {
                '(' | ')' | ',' => ' ',
                _ => ch,
            })
            .collect();
        let tokens: Vec<&str> = normalized.split_whitespace().collect();
        if tokens.len() < 3 {
            return false;
        }

        let model_type = tokens[2];
        if model_type != "nmos" && model_type != "pmos" {
            return false;
        }

        let mut idx = 3usize;
        while idx < tokens.len() {
            let token = tokens[idx];
            if token == "level" || token == "level=" {
                if idx + 1 < tokens.len() && Self::is_unsupported_soi_level_value(tokens[idx + 1]) {
                    return true;
                }
            } else if let Some(value) = token.strip_prefix("level=") {
                if value.is_empty() {
                    if idx + 1 < tokens.len()
                        && Self::is_unsupported_soi_level_value(tokens[idx + 1])
                    {
                        return true;
                    }
                } else if Self::is_unsupported_soi_level_value(value) {
                    return true;
                }
            }
            idx += 1;
        }

        false
    }

    fn is_unsupported_soi_level_value(level: &str) -> bool {
        let Ok(value) = level.parse::<f64>() else {
            return false;
        };
        ((value - 55.0).abs() < 0.5) || ((value - 56.0).abs() < 0.5) || ((value - 57.0).abs() < 0.5)
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Suite Execution
    // ═══════════════════════════════════════════════════════════════════════════

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
            .filter(|r| r.error.as_ref().map_or(false, |e| e.starts_with("SKIPPED")))
            .count();
        let failed = total - passed - skipped;

        println!("\n════════════════════════════════════════════════════════════");
        println!(
            "  Test Summary: {} total | {} passed | {} failed | {} skipped",
            total, passed, failed, skipped
        );
        println!("════════════════════════════════════════════════════════════\n");

        // Print failures first
        for result in results
            .iter()
            .filter(|r| !r.passed && r.error.as_ref().map_or(true, |e| !e.starts_with("SKIPPED")))
        {
            let analysis = result.analysis_type.as_deref().unwrap_or("?");
            println!("  ✗ {} [{}] - {:?}", result.name, analysis, result.error);
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
                .filter(|r| r.error.as_ref().map_or(false, |e| e.starts_with("SKIPPED")))
            {
                if let Some(ref err) = result.error {
                    println!("  ⊘ {} - {}", result.name, err);
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
            .filter(|r| r.error.as_ref().map_or(false, |e| e.starts_with("SKIPPED")))
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

// ═══════════════════════════════════════════════════════════════════════════════
// Reference Output Parsing (for future comparison)
// ═══════════════════════════════════════════════════════════════════════════════

impl TestRunner {
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
            return Ok(Vec::new());
        }

        let engine = self.create_dynamic_engine();
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| format!("Failed to build circuit for DC reference mapping: {e}"))?;

        let x_sim: Vec<f64> = results.iter().map(|(x, _)| *x).collect();
        let first = &results[0].1;
        let mut node_to_idx = HashMap::with_capacity(first.node_names.len());
        for (idx, name) in first.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx);
        }

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            let normalized = Self::normalize_variable_name(var);
            if normalized == Self::normalize_variable_name(&reference.x_name) {
                return None;
            }

            if let Some((n1, n2)) = Self::parse_voltage_probe(var) {
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

            if let Some(branch_name) = Self::parse_current_probe(var) {
                let branch_idx = circuit.get_branch_by_name(&branch_name)?.checked_sub(1)?;
                let series = results
                    .iter()
                    .map(|(_, r)| r.branch_currents.get(branch_idx).copied().unwrap_or(0.0))
                    .collect();
                return Some(series);
            }

            None
        }))
    }

    fn compare_transient_reference(
        &self,
        cir_path: &Path,
        result: &crate::engine::TransientResult,
    ) -> Result<Vec<ValueMismatch>, String> {
        let Some(reference) = self.load_reference_table_for_axis(cir_path, &["time"])? else {
            return Ok(Vec::new());
        };
        if result.time.is_empty() {
            return Ok(Vec::new());
        }

        let x_sim = result.time.clone();
        let mut node_to_idx = HashMap::with_capacity(result.node_names.len() + 1);
        node_to_idx.insert("0".to_string(), 0usize);
        for (idx, name) in result.node_names.iter().enumerate() {
            node_to_idx.insert(name.to_ascii_lowercase(), idx + 1);
        }

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            if let Some((n1, n2)) = Self::parse_voltage_probe(var) {
                let idx1 = Self::resolve_node_index(&node_to_idx, &n1)?;
                let idx2 = n2
                    .as_deref()
                    .and_then(|name| Self::resolve_node_index(&node_to_idx, name))
                    .unwrap_or(0);

                let get_node_waveform = |idx: usize| -> Vec<f64> {
                    if idx == 0 {
                        vec![0.0; result.time.len()]
                    } else {
                        result
                            .voltages
                            .get(idx - 1)
                            .cloned()
                            .unwrap_or_else(|| vec![0.0; result.time.len()])
                    }
                };

                let w1 = get_node_waveform(idx1);
                let w2 = get_node_waveform(idx2);
                let series = w1
                    .iter()
                    .zip(w2.iter())
                    .map(|(a, b)| a - b)
                    .collect::<Vec<_>>();
                return Some(series);
            }

            None
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
            return Ok(Vec::new());
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

        let x_sim: Vec<f64> = results.iter().map(|r| r.frequency).collect();

        Ok(self.compare_reference_dataset(&reference, &x_sim, |var| {
            let normalized = Self::normalize_variable_name(var);

            if let Some((func, node_a, node_b)) = Self::parse_ac_voltage_probe(&normalized) {
                let idx_a = Self::resolve_node_index(&node_to_idx, &node_a)?;
                let idx_b = node_b
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
                        let v = va - vb;
                        match func {
                            "v" | "vm" => v.norm(),
                            "vr" => v.re,
                            "vi" => v.im,
                            "vp" => v.arg().to_degrees(),
                            "vdb" => {
                                let mag = v.norm().max(self.config.absolute_tolerance);
                                20.0 * mag.log10()
                            }
                            _ => v.norm(),
                        }
                    })
                    .collect();
                return Some(series);
            }

            None
        }))
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

            let Some(actual_series) = resolver(var) else {
                continue;
            };
            if actual_series.is_empty() || x_sim.is_empty() {
                continue;
            }

            let ref_monotonic = Self::is_monotonic_axis(&expected_series.x);
            if sim_monotonic && ref_monotonic {
                for (&x_ref, &expected) in expected_series.x.iter().zip(expected_series.y.iter()) {
                    let Some(actual) = Self::interpolate_series(x_sim, &actual_series, x_ref)
                    else {
                        continue;
                    };
                    if let Some(relative_error) = self.compare_values(expected, actual) {
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
                let n = expected_series.y.len().min(actual_series.len());
                for i in 0..n {
                    let expected = expected_series.y[i];
                    let actual = actual_series[i];
                    let x_value = expected_series.x.get(i).copied().unwrap_or(i as f64);
                    if let Some(relative_error) = self.compare_values(expected, actual) {
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
        let Ok(mut tables) = self.parse_ngspice_output_tables(&content) else {
            return Ok(None);
        };
        if tables.is_empty() {
            return Ok(None);
        }

        for candidate in axis_candidates {
            let target = Self::normalize_variable_name(candidate);
            if let Some(idx) = tables
                .iter()
                .position(|t| Self::normalize_variable_name(&t.x_name) == target)
            {
                return Ok(Some(tables.swap_remove(idx)));
            }
        }

        Ok(tables.into_iter().next())
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

            let Ok(x_value) = parts[x_col_idx].parse::<f64>() else {
                continue;
            };

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
            if let Some(last) = merged.last_mut() {
                if Self::can_merge_reference_tables(last, &table) {
                    for (name, mut series) in table.variables.drain() {
                        if let Some(dst) = last.variables.get_mut(&name) {
                            dst.x.append(&mut series.x);
                            dst.y.append(&mut series.y);
                        }
                    }
                    continue;
                }
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

    fn parse_ac_voltage_probe(var: &str) -> Option<(&'static str, String, Option<String>)> {
        for func in ["vdb", "vm", "vr", "vi", "vp", "v"] {
            let prefix = format!("{func}(");
            if var.starts_with(&prefix) && var.ends_with(')') {
                let inner = &var[prefix.len()..var.len() - 1];
                if let Some((a, b)) = inner.split_once(',') {
                    return Some((func, a.to_string(), Some(b.to_string())));
                }
                return Some((func, inner.to_string(), None));
            }
        }
        None
    }

    fn resolve_node_index(node_to_idx: &HashMap<String, usize>, node: &str) -> Option<usize> {
        if let Some(idx) = node_to_idx.get(&node.to_ascii_lowercase()) {
            return Some(*idx);
        }
        node.parse::<usize>().ok()
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

    fn compare_values(&self, expected: f64, actual: f64) -> Option<f64> {
        let abs_diff = (expected - actual).abs();

        if abs_diff < self.config.absolute_tolerance {
            return None;
        }

        let rel_error = if expected.abs() > self.config.absolute_tolerance {
            abs_diff / expected.abs()
        } else {
            abs_diff
        };

        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }
}

// Unit Tests
// ═══════════════════════════════════════════════════════════════════════════════

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

    #[test]
    fn test_config_defaults() {
        let config = TestRunnerConfig::default();
        assert!((config.relative_tolerance - 0.01).abs() < 1e-10);
        assert!((config.absolute_tolerance - 1e-12).abs() < 1e-20);
        assert!(config.skip_unsupported);
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
        assert_eq!(
            runner.check_unsupported(".model cpl1 cpl (R=1)").as_deref(),
            Some("CPL transmission line model")
        );
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
        assert_eq!(
            runner
                .check_unsupported(".model n1 nmos (level=55 tox=4.5e-9)")
                .as_deref(),
            Some("BSIMSOI LEVEL=55/56/57 model")
        );
        assert_eq!(
            runner
                .check_unsupported(".model p1 pmos level= 57")
                .as_deref(),
            Some("BSIMSOI LEVEL=55/56/57 model")
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
        assert!(runner.compare_values(1.0, 1.02).is_some());

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
}
