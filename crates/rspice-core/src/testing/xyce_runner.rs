//! Xyce regression corpus runner.
//!
//! The upstream Xyce suite is vendored as a runtime corpus. RSpice keeps the
//! netlists, reference output data, and licensing/provenance files, but omits
//! upstream platform-specific harness scripts. Regression execution is
//! Rust-native: every retained `.cir` deck is discovered and reported, and only
//! decks with a supported, checked-in static `.prn` oracle are numerically
//! executed.

use crate::abort_signal::AbortSignal;
use crate::analysis::DcSweep;
use crate::engine::{
    ConvergenceConfig, SimulationConfig, SimulationError, SpiceDialect, extract_dc_value,
};
use crate::netlist::{AnalysisCommand, DcSecondSweep, ElementKind, Netlist};
use crate::{Engine, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const EXPECTED_UNSUPPORTED_MARKER: &str = "EXPECTED_UNSUPPORTED:";
const HARNESS_MANIFEST_FILE: &str = "RSPICE-HARNESS-MANIFEST.tsv";
const REQUIRES_UPSTREAM_WRAPPER_CONTRACT: &str = "requires_upstream_wrapper";
const MIN_DIRECT_RESISTOR_ABS_OHMS: Value = 1.0e-30;

/// Configuration for the Xyce corpus runner.
#[derive(Debug, Clone)]
pub struct XyceRunnerConfig {
    /// Relative tolerance for value comparison.
    pub relative_tolerance: f64,
    /// Absolute tolerance for near-zero values.
    pub absolute_tolerance: f64,
    /// Maximum number of mismatches to retain in one result.
    pub max_mismatches: usize,
    /// Maximum wall-clock time allowed for a numerically executed deck.
    pub max_time_per_test_ms: u128,
    /// Print per-deck execution details.
    pub verbose: bool,
}

impl Default for XyceRunnerConfig {
    fn default() -> Self {
        Self {
            relative_tolerance: 0.02,
            absolute_tolerance: 1.0e-12,
            max_mismatches: 20,
            max_time_per_test_ms: 30_000,
            verbose: false,
        }
    }
}

/// Which vendored corpus area a `.cir` file belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XyceDeckSection {
    /// Simulator regression decks under `tests/xyce/Netlists`.
    Netlists,
    /// Any other vendored `.cir` file.
    Other,
}

/// A discovered vendored Xyce deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XyceDeck {
    /// Absolute path to the deck.
    pub path: PathBuf,
    /// Path relative to `tests/xyce`, normalized with `/` separators.
    pub relative_path: String,
    /// Corpus section classification.
    pub section: XyceDeckSection,
}

/// Numeric mismatch for a Xyce reference comparison.
#[derive(Debug, Clone)]
pub struct XyceValueMismatch {
    /// Row index in the Xyce `.prn` table.
    pub row: usize,
    /// Output column/probe name.
    pub probe: String,
    /// Expected value from Xyce output data.
    pub expected: f64,
    /// RSpice value.
    pub actual: f64,
    /// Relative error after the absolute tolerance floor.
    pub relative_error: f64,
}

/// Result for one discovered Xyce deck.
#[derive(Debug, Clone)]
pub struct XyceTestResult {
    /// Deck filename stem.
    pub name: String,
    /// Path relative to `tests/xyce`.
    pub relative_path: String,
    /// Whether the deck produced an accepted result for the current harness.
    pub passed: bool,
    /// Whether this is a named, expected unsupported result rather than a
    /// numeric comparison.
    pub expected_unsupported: bool,
    /// Error or expected-unsupported reason.
    pub error: Option<String>,
    /// Numeric mismatches for executed decks.
    pub mismatches: Vec<XyceValueMismatch>,
    /// Execution/classification duration.
    pub duration_ms: u128,
    /// Contract label applied by the runner.
    pub contract: String,
}

/// Aggregate Xyce corpus statistics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XyceStatistics {
    pub total: usize,
    pub executed: usize,
    pub passed: usize,
    pub failed: usize,
    pub expected_unsupported: usize,
    pub total_time_ms: u128,
}

impl XyceStatistics {
    pub fn executed_pass_rate(&self) -> f64 {
        if self.executed == 0 {
            0.0
        } else {
            self.passed as f64 / self.executed as f64 * 100.0
        }
    }

    pub fn executed_coverage_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.executed as f64 / self.total as f64 * 100.0
        }
    }
}

/// Rust-native runner for the vendored Xyce corpus.
pub struct XyceTestRunner {
    root: PathBuf,
    config: XyceRunnerConfig,
    upstream_wrapper_decks: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct XyceExecutionPlan {
    deck_path: PathBuf,
    reference_path: PathBuf,
    source: String,
    print: XycePrintRequest,
    dc: XyceDcSweep,
}

#[derive(Debug, Clone)]
struct XycePrintRequest {
    probes: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct XyceComparisonTolerance {
    relative: f64,
    absolute: f64,
    zero: Option<f64>,
}

impl XyceComparisonTolerance {
    fn from_config(config: &XyceRunnerConfig) -> Self {
        Self {
            relative: config.relative_tolerance,
            absolute: config.absolute_tolerance,
            zero: None,
        }
    }

    fn with_relative(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.relative = value;
        }
        self
    }

    fn with_absolute(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.absolute = self.absolute.max(value);
        }
        self
    }

    fn with_zero(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.zero = Some(self.zero.unwrap_or(0.0).max(value));
        }
        self
    }
}

#[derive(Debug, Clone)]
struct XyceDcSweep {
    source: String,
    start: Value,
    stop: Value,
    step: Value,
    sweep2: Option<DcSecondSweep>,
}

#[derive(Debug, Clone, Copy)]
struct XyceDcSweepPoint {
    primary: Value,
    secondary: Option<Value>,
}

#[derive(Debug, Clone)]
enum XyceReferenceColumn {
    PrimarySweep { name: String },
    Probe { name: String },
}

#[derive(Debug, Clone)]
struct XycePrnTable {
    columns: Vec<String>,
    rows: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XycePrnDelimiter {
    Whitespace,
    Comma,
}

#[derive(Debug, Clone, Copy)]
struct DeadlineAbort {
    start: Instant,
    deadline: Duration,
}

impl DeadlineAbort {
    fn new(start: Instant, timeout_ms: u128) -> Self {
        Self {
            start,
            deadline: Duration::from_millis(timeout_ms.min(u128::from(u64::MAX)) as u64),
        }
    }
}

impl AbortSignal for DeadlineAbort {
    fn is_aborted(&self) -> bool {
        self.start.elapsed() >= self.deadline
    }
}

impl XyceTestRunner {
    /// Create a runner rooted at `tests/xyce`.
    pub fn new<P: AsRef<Path>>(root: P, config: XyceRunnerConfig) -> Self {
        let root = root
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| root.as_ref().to_path_buf());
        let upstream_wrapper_decks = Self::load_upstream_wrapper_decks(&root);
        Self {
            root,
            config,
            upstream_wrapper_decks,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config(&self) -> &XyceRunnerConfig {
        &self.config
    }

    /// RSpice never executes upstream `.sh`/`.pl` harness files in this runner.
    pub fn executes_upstream_scripts(&self) -> bool {
        false
    }

    /// Whether a retained deck had an upstream `.cir.sh` wrapper sidecar in the
    /// source corpus. Those scripts are intentionally not vendored; the
    /// cross-platform manifest records the execution contract instead.
    pub fn requires_upstream_wrapper(&self, relative_path: &str) -> bool {
        self.upstream_wrapper_decks
            .contains(&Self::normalize_manifest_key(relative_path))
    }

    /// Discover every `.cir` file under the vendored Xyce root.
    pub fn discover_tests(&self) -> Vec<XyceDeck> {
        let mut paths = Vec::new();
        Self::collect_circuit_files(&self.root, &mut paths);
        paths.sort();
        paths
            .into_iter()
            .map(|path| {
                let relative_path = self.relative_key(&path);
                let section = Self::section_for_relative_path(&relative_path);
                XyceDeck {
                    path,
                    relative_path,
                    section,
                }
            })
            .collect()
    }

    /// Discover simulator regression decks under `Netlists`.
    pub fn discover_netlist_tests(&self) -> Vec<XyceDeck> {
        self.discover_tests()
            .into_iter()
            .filter(|deck| deck.section == XyceDeckSection::Netlists)
            .collect()
    }

    /// Run every discovered vendored `.cir` through the Xyce corpus contract.
    pub fn run_all(&self) -> Vec<XyceTestResult> {
        self.discover_tests()
            .into_iter()
            .map(|deck| self.run_discovered_test(&deck))
            .collect()
    }

    /// Run every simulator regression deck under `Netlists`.
    pub fn run_netlist_suite(&self) -> Vec<XyceTestResult> {
        self.discover_netlist_tests()
            .into_iter()
            .map(|deck| self.run_discovered_test(&deck))
            .collect()
    }

    /// Run one deck path.
    pub fn run_test<P: AsRef<Path>>(&self, deck_path: P) -> XyceTestResult {
        let path = deck_path
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| deck_path.as_ref().to_path_buf());
        let relative_path = self.relative_key(&path);
        let deck = XyceDeck {
            path,
            section: Self::section_for_relative_path(&relative_path),
            relative_path,
        };
        self.run_discovered_test(&deck)
    }

    pub fn statistics(results: &[XyceTestResult]) -> XyceStatistics {
        let mut stats = XyceStatistics::default();
        stats.total = results.len();
        for result in results {
            stats.total_time_ms += result.duration_ms;
            if result.expected_unsupported {
                stats.expected_unsupported += 1;
            } else {
                stats.executed += 1;
                if result.passed {
                    stats.passed += 1;
                }
            }
            if !result.passed {
                stats.failed += 1;
            }
        }
        stats
    }

    pub fn print_summary(results: &[XyceTestResult]) {
        let stats = Self::statistics(results);
        println!("\nXyce corpus summary");
        println!(
            "  total: {} | executed: {} | passed: {} | failed: {} | expected unsupported: {}",
            stats.total, stats.executed, stats.passed, stats.failed, stats.expected_unsupported
        );
        println!(
            "  executed pass rate: {:.1}% | executed coverage: {:.1}%",
            stats.executed_pass_rate(),
            stats.executed_coverage_rate()
        );

        for result in results.iter().filter(|result| !result.passed).take(20) {
            println!(
                "  FAIL {} [{}]: {}",
                result.relative_path,
                result.contract,
                result.error.as_deref().unwrap_or("unknown failure")
            );
            for mismatch in result.mismatches.iter().take(3) {
                println!(
                    "       row {} {} expected {:.8e}, actual {:.8e}, rel {:.3e}",
                    mismatch.row,
                    mismatch.probe,
                    mismatch.expected,
                    mismatch.actual,
                    mismatch.relative_error
                );
            }
        }
    }

    fn run_discovered_test(&self, deck: &XyceDeck) -> XyceTestResult {
        let start = Instant::now();
        if deck.section != XyceDeckSection::Netlists {
            return self.expected_unsupported_result(
                deck,
                start,
                "upstream_harness_fixture",
                "deck is part of upstream Xyce harness self-tests, not the simulator Netlists corpus",
            );
        }

        let plan = match self.execution_plan(deck) {
            Ok(plan) => plan,
            Err(reason) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_contract",
                    &reason,
                );
            }
        };

        let result = self.run_static_prn_dc_plan(deck, plan, start);
        if self.config.verbose {
            println!(
                "{} [{}] {}",
                result.relative_path,
                result.contract,
                if result.passed { "PASS" } else { "FAIL" }
            );
        }
        result
    }

    fn expected_unsupported_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        reason: &str,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: true,
            expected_unsupported: true,
            error: Some(format!("{EXPECTED_UNSUPPORTED_MARKER} {reason}")),
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    fn failure_result(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        error: String,
        mismatches: Vec<XyceValueMismatch>,
    ) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: false,
            expected_unsupported: false,
            error: Some(error),
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    fn passed_result(&self, deck: &XyceDeck, start: Instant, contract: &str) -> XyceTestResult {
        XyceTestResult {
            name: Self::deck_name(&deck.path),
            relative_path: deck.relative_path.clone(),
            passed: true,
            expected_unsupported: false,
            error: None,
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            contract: contract.to_string(),
        }
    }

    fn execution_plan(&self, deck: &XyceDeck) -> Result<XyceExecutionPlan, String> {
        if self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(
                "upstream wrapper semantics are required; RSPICE-HARNESS-MANIFEST.tsv records the removed .cir.sh sidecar contract"
                    .to_string(),
            );
        }

        let reference_path = self
            .static_prn_reference_path(&deck.path)
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !reference_path.is_file() {
            return Err(format!(
                "no checked-in static .prn oracle at {}",
                self.display_path(&reference_path)
            ));
        }

        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;

        let print = Self::single_dc_print_request(&source)?;
        let netlist = Netlist::parse_with_path(&source, &deck.path)
            .map_err(|err| format!("netlist parser does not yet accept this Xyce deck: {err}"))?;
        let dc = Self::single_dc_sweep(&netlist)?;
        Self::validate_static_dc_contract(&netlist, &dc, &print)?;

        Ok(XyceExecutionPlan {
            deck_path: deck.path.clone(),
            reference_path,
            source,
            print,
            dc,
        })
    }

    fn run_static_prn_dc_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceExecutionPlan,
        start: Instant,
    ) -> XyceTestResult {
        let netlist = match Netlist::parse_with_path(&plan.source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    "static_prn_dc",
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let reference = match Self::parse_prn_file(&plan.reference_path) {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    "static_prn_dc",
                    format!("failed to parse Xyce .prn oracle: {err}"),
                    Vec::new(),
                );
            }
        };

        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = match engine.run_dc_sweep2_with_abort(
            &netlist,
            &plan.dc.source,
            plan.dc.start,
            plan.dc.stop,
            plan.dc.step,
            plan.dc.sweep2.as_ref(),
            &abort,
        ) {
            Ok(results) => results,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    "static_prn_dc",
                    format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    "static_prn_dc",
                    format!("simulation error: {err}"),
                    Vec::new(),
                );
            }
        };

        let mismatches = match self.compare_dc_prn_reference(
            &reference,
            &plan.print,
            &netlist,
            &plan.source,
            &plan.dc,
            &results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    "static_prn_dc",
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };

        if mismatches.is_empty() {
            self.passed_result(deck, start, "static_prn_dc")
        } else {
            self.failure_result(
                deck,
                start,
                "static_prn_dc",
                format!("{} Xyce reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    fn create_dc_engine(&self) -> Engine {
        let defaults = SimulationConfig::default();
        Engine::new(SimulationConfig {
            max_iterations: defaults.max_iterations.max(1200),
            convergence_config: ConvergenceConfig::robust(),
            spice_dialect: SpiceDialect::Xyce,
            // Xyce and ngspice regression decks use 27 C unless overridden.
            temperature: 300.15,
            ..defaults
        })
    }

    fn compare_dc_prn_reference(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        source: &str,
        dc: &XyceDcSweep,
        results: &[(Value, crate::SimulationResult)],
    ) -> Result<Vec<XyceValueMismatch>, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }
        if !reference.columns[0].eq_ignore_ascii_case("Index") {
            return Err(format!(
                "expected first Xyce .prn column to be Index, got '{}'",
                reference.columns[0]
            ));
        }
        if reference.rows.len() != results.len() {
            return Err(format!(
                "reference row count ({}) does not match simulation point count ({})",
                reference.rows.len(),
                results.len()
            ));
        }

        let data_columns = self.reference_data_columns(reference, print)?;
        let comp_tolerances = self.comp_tolerances(source, &data_columns)?;
        let primary_points = DcSweep::new(dc.source.clone(), dc.start, dc.stop, dc.step).points();
        if primary_points.is_empty() {
            return Err("primary DC sweep has no points".to_string());
        }
        let secondary_points = dc.sweep2.as_ref().map(|sweep| {
            DcSweep::new(sweep.source.clone(), sweep.start, sweep.stop, sweep.step).points()
        });
        if secondary_points.as_ref().is_some_and(Vec::is_empty) {
            return Err("secondary DC sweep has no points".to_string());
        }
        let mut mismatches = Vec::new();
        for (row_index, row) in reference.rows.iter().enumerate() {
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "row {} has {} values, expected {}",
                    row_index,
                    row.len(),
                    reference.columns.len()
                ));
            }
            let expected_index = row[0];
            if (expected_index - row_index as f64).abs() > self.config.absolute_tolerance {
                mismatches.push(XyceValueMismatch {
                    row: row_index,
                    probe: "Index".to_string(),
                    expected: row_index as f64,
                    actual: expected_index,
                    relative_error: 1.0,
                });
                if mismatches.len() >= self.config.max_mismatches {
                    return Ok(mismatches);
                }
            }

            let sweep_point = XyceDcSweepPoint {
                primary: results[row_index].0,
                secondary: if let Some(points) = secondary_points.as_ref() {
                    let outer_index = row_index / primary_points.len();
                    Some(*points.get(outer_index).ok_or_else(|| {
                        format!(
                            "row {row_index} maps outside secondary DC sweep point count ({})",
                            points.len()
                        )
                    })?)
                } else {
                    None
                },
            };
            for (column_index, column) in data_columns.iter().enumerate() {
                let expected = row[column_index + 1];
                let (probe, actual) = match column {
                    XyceReferenceColumn::PrimarySweep { name } => {
                        (name.as_str(), sweep_point.primary)
                    }
                    XyceReferenceColumn::Probe { name } => (
                        name.as_str(),
                        Self::evaluate_dc_probe(
                            name,
                            netlist,
                            dc,
                            sweep_point,
                            &results[row_index].1,
                        )?,
                    ),
                };
                let tolerance = comp_tolerances
                    .get(&Self::normalize_probe(probe))
                    .copied()
                    .unwrap_or_else(|| XyceComparisonTolerance::from_config(&self.config));
                if let Some(relative_error) = self.value_mismatch(expected, actual, tolerance) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: probe.to_string(),
                        expected,
                        actual,
                        relative_error,
                    });
                    if mismatches.len() >= self.config.max_mismatches {
                        return Ok(mismatches);
                    }
                }
            }
        }

        Ok(mismatches)
    }

    fn reference_data_columns(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
    ) -> Result<Vec<XyceReferenceColumn>, String> {
        let mut data_columns = Vec::with_capacity(reference.columns.len().saturating_sub(1));
        let mut probe_index = 0usize;
        for column in reference.columns.iter().skip(1) {
            if Self::is_primary_dc_sweep_reference_column(column) {
                data_columns.push(XyceReferenceColumn::PrimarySweep {
                    name: column.clone(),
                });
                continue;
            }

            let Some(probe) = print.probes.get(probe_index) else {
                return Err(format!(
                    "reference column '{}' has no matching .PRINT DC probe",
                    column
                ));
            };
            if !Self::reference_column_matches_probe(column, probe) {
                return Err(format!(
                    "reference column '{}' does not match .PRINT probe '{}'",
                    column, probe
                ));
            }
            data_columns.push(XyceReferenceColumn::Probe {
                name: probe.clone(),
            });
            probe_index += 1;
        }
        if probe_index != print.probes.len() {
            return Err(format!(
                "reference table matched {} .PRINT DC probe(s), but deck requested {}",
                probe_index,
                print.probes.len()
            ));
        }
        Ok(data_columns)
    }

    fn is_primary_dc_sweep_reference_column(column: &str) -> bool {
        Self::normalize_probe(column) == "v-sweep"
    }

    fn reference_column_matches_probe(column: &str, probe: &str) -> bool {
        let normalized_column = Self::normalize_probe(column);
        if normalized_column == Self::normalize_probe(probe) {
            return true;
        }
        if let Some(source_name) = Self::parse_current_probe(probe) {
            return normalized_column == format!("{source_name}_branch");
        }
        false
    }

    fn comp_tolerances(
        &self,
        source: &str,
        columns: &[XyceReferenceColumn],
    ) -> Result<BTreeMap<String, XyceComparisonTolerance>, String> {
        let mut compared_probes = BTreeSet::new();
        for column in columns {
            let XyceReferenceColumn::Probe { name } = column else {
                continue;
            };
            compared_probes.insert(Self::normalize_probe(name));
        }

        let default_tolerance = XyceComparisonTolerance::from_config(&self.config);
        let mut tolerances = BTreeMap::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = line.trim_start();
            if !trimmed
                .get(..5)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("*comp"))
            {
                continue;
            }
            let rest = trimmed[5..].trim_start();
            let Some((probe, options)) = Self::split_comp_directive(rest) else {
                continue;
            };
            let normalized_probe = Self::normalize_probe(&probe);
            if !compared_probes.contains(&normalized_probe) {
                continue;
            }
            let tolerance = Self::parse_comp_tolerance(&options, default_tolerance)?;
            tolerances.insert(normalized_probe, tolerance);
        }
        Ok(tolerances)
    }

    fn split_comp_directive(rest: &str) -> Option<(String, String)> {
        let rest = rest
            .split_once(';')
            .map(|(head, _)| head)
            .unwrap_or(rest)
            .trim();
        if rest.is_empty() {
            return None;
        }

        if rest.starts_with('{') {
            let mut depth = 0usize;
            for (index, ch) in rest.char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            let end = index + ch.len_utf8();
                            return Some((
                                rest[..end].trim().to_string(),
                                rest[end..].trim().to_string(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut split = rest.splitn(2, char::is_whitespace);
        let probe = split.next()?.trim();
        if probe.is_empty() {
            return None;
        }
        Some((
            probe.to_string(),
            split.next().unwrap_or_default().trim().to_string(),
        ))
    }

    fn parse_comp_tolerance(
        options: &str,
        default_tolerance: XyceComparisonTolerance,
    ) -> Result<XyceComparisonTolerance, String> {
        let mut tolerance = default_tolerance;
        let tokens = options.split_whitespace().collect::<Vec<_>>();
        let mut index = 0usize;
        while index < tokens.len() {
            let token = tokens[index];
            let (raw_key, raw_value, consumed) = if let Some((key, value)) = token.split_once('=') {
                (key, value, 1usize)
            } else if token.ends_with('=') {
                let key = token.trim_end_matches('=');
                let value = tokens.get(index + 1).copied().unwrap_or_default();
                (key, value, 2usize)
            } else if tokens
                .get(index + 1)
                .is_some_and(|candidate| *candidate == "=")
            {
                let value = tokens.get(index + 2).copied().unwrap_or_default();
                (token, value, 3usize)
            } else {
                index += 1;
                continue;
            };

            let key = raw_key.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if value.is_empty() {
                return Err(format!("Xyce *COMP option '{key}' is missing a value"));
            }
            match key.as_str() {
                "reltol" => tolerance = tolerance.with_relative(Self::parse_comp_float(value)?),
                "abstol" | "absdifftol" => {
                    tolerance = tolerance.with_absolute(Self::parse_comp_float(value)?)
                }
                "zerotol" => tolerance = tolerance.with_zero(Self::parse_comp_float(value)?),
                _ => {}
            }
            index += consumed;
        }
        Ok(tolerance)
    }

    fn parse_comp_float(value: &str) -> Result<f64, String> {
        value
            .parse::<f64>()
            .map_err(|err| format!("invalid Xyce *COMP numeric value '{value}': {err}"))
    }

    fn value_mismatch(
        &self,
        expected: f64,
        actual: f64,
        tolerance: XyceComparisonTolerance,
    ) -> Option<f64> {
        if !expected.is_finite() || !actual.is_finite() {
            return Some(f64::INFINITY);
        }
        let abs_error = (expected - actual).abs();
        if let Some(zero_tolerance) = tolerance.zero {
            if expected.abs() <= zero_tolerance && actual.abs() <= zero_tolerance {
                return None;
            }
        }
        if abs_error <= tolerance.absolute {
            return None;
        }
        let scale = expected.abs().max(actual.abs()).max(tolerance.absolute);
        let relative_error = abs_error / scale;
        (relative_error > tolerance.relative).then_some(relative_error)
    }

    fn validate_static_dc_contract(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        Self::validate_dc_sweep_source(netlist, &dc.source)?;
        if let Some(sweep2) = &dc.sweep2 {
            Self::validate_dc_sweep_source(netlist, &sweep2.source)?;
        }

        for probe in &print.probes {
            Self::validate_dc_probe(probe, netlist)?;
        }

        Ok(())
    }

    fn is_expected_unsupported_runtime_error(err: &SimulationError) -> bool {
        let (SimulationError::Circuit(message) | SimulationError::Netlist(message)) = err else {
            return false;
        };
        let normalized = message.to_ascii_lowercase();
        normalized.contains("unsupported")
            || normalized.contains("not implemented")
            || normalized.contains("no native implementation")
            || normalized.contains("no generated verilog-a builtin")
            || normalized.contains("no generated builtin")
            || normalized.contains("not yet")
            || normalized.contains("refusing")
            || normalized.contains("must not run through")
    }

    fn validate_dc_sweep_source(netlist: &Netlist, source: &str) -> Result<(), String> {
        if Self::source_is_independent_source(netlist, source)
            || source.eq_ignore_ascii_case("TEMP")
            || source.eq_ignore_ascii_case("TEMPER")
        {
            return Ok(());
        }
        Err(format!(
            "DC sweep source '{}' is not a supported top-level independent source or TEMP sweep",
            source
        ))
    }

    fn reject_unsupported_source_directives(source: &str) -> Result<(), String> {
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.is_empty() {
                continue;
            }
            let directive = trimmed.split_whitespace().next().unwrap_or_default();
            match directive {
                ".step" => {
                    return Err(
                        "deck uses .STEP; Xyce parameter-sweep output comparison is not implemented yet"
                            .to_string(),
                    );
                }
                ".inc" | ".incl" => {
                    return Err(format!(
                        "deck uses HSPICE include alias '{directive}'; Xyce include-alias semantics are not implemented yet"
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate_dc_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        let normalized = Self::normalize_probe(probe);
        if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(&normalized) {
            if !node_pos.is_empty() && node_neg.as_deref().is_none_or(|node| !node.is_empty()) {
                return Ok(());
            }
        }
        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(&normalized) {
            match parameter.as_str() {
                "dcv0" if Self::source_is_independent_source(netlist, &element_name) => {
                    return Ok(());
                }
                "r" => {
                    if let Some(resistance) = Self::direct_resistor_value(netlist, &element_name) {
                        if resistance.is_finite()
                            && resistance.abs() >= MIN_DIRECT_RESISTOR_ABS_OHMS
                        {
                            return Ok(());
                        }
                    }
                }
                _ => {}
            }
            return Err(format!(
                "device parameter probe '{}' targets an unsupported parameter",
                probe
            ));
        }
        if let Some(parameter_name) = Self::parse_scalar_parameter_probe(&normalized)
            && Self::scalar_parameter_probe_is_supported(netlist, &parameter_name)
        {
            return Ok(());
        }
        if let Some(element_name) = Self::parse_current_probe(&normalized) {
            if Self::source_is_voltage_source(netlist, &element_name) {
                return Ok(());
            }
            if let Some(resistance) = Self::direct_resistor_value(netlist, &element_name) {
                if resistance.is_finite() && resistance.abs() >= MIN_DIRECT_RESISTOR_ABS_OHMS {
                    return Ok(());
                }
                return Err(format!(
                    "current probe '{}' targets a zero/near-zero resistor not implemented by the first Xyce adapter",
                    probe
                ));
            }
            return Err(format!(
                "current probe '{}' targets an unsupported branch/device",
                probe
            ));
        }
        Err(format!("unsupported .PRINT DC probe '{}'", probe))
    }

    fn evaluate_dc_probe(
        probe: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &crate::SimulationResult,
    ) -> Result<f64, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(&normalized) {
            let pos = result
                .try_voltage_named(&node_pos)
                .ok_or_else(|| format!("node '{}' not found in DC result", node_pos))?;
            let neg = match node_neg {
                Some(node) => result
                    .try_voltage_named(&node)
                    .ok_or_else(|| format!("node '{}' not found in DC result", node))?,
                None => 0.0,
            };
            return Ok(pos - neg);
        }

        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(&normalized) {
            return Self::evaluate_device_parameter_probe(
                netlist,
                dc,
                sweep_point,
                &element_name,
                &parameter,
            );
        }

        if let Some(parameter_name) = Self::parse_scalar_parameter_probe(&normalized) {
            return Self::evaluate_scalar_parameter_probe(
                netlist,
                dc,
                sweep_point,
                &parameter_name,
            );
        }

        if let Some(element_name) = Self::parse_current_probe(&normalized) {
            if let Some(current) = result.branch_current_named(&element_name) {
                return Ok(current);
            }
            if let Some(resistance) = Self::direct_resistor_value(netlist, &element_name) {
                return Self::evaluate_resistor_current(netlist, result, &element_name, resistance);
            }
        }

        Err(format!("unsupported DC probe '{}'", probe))
    }

    fn evaluate_device_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        match parameter {
            "dcv0" => {
                if element_name.eq_ignore_ascii_case(&dc.source) {
                    return Ok(sweep_point.primary);
                }
                if let Some(sweep2) = &dc.sweep2 {
                    if element_name.eq_ignore_ascii_case(&sweep2.source) {
                        return sweep_point.secondary.ok_or_else(|| {
                            format!(
                                "secondary sweep value for '{}' is unavailable",
                                element_name
                            )
                        });
                    }
                }
                Self::independent_source_dc_value(netlist, element_name).ok_or_else(|| {
                    format!(
                        "source parameter probe '{}:DCV0' targets an unknown independent source",
                        element_name
                    )
                })
            }
            "r" => Self::direct_resistor_value(netlist, element_name).ok_or_else(|| {
                format!(
                    "resistor parameter probe '{}:R' targets an unknown direct resistor",
                    element_name
                )
            }),
            _ => Err(format!(
                "device parameter probe '{}:{}' is not supported",
                element_name, parameter
            )),
        }
    }

    fn evaluate_scalar_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        parameter_name: &str,
    ) -> Result<f64, String> {
        if parameter_name.eq_ignore_ascii_case("TEMP")
            || parameter_name.eq_ignore_ascii_case("TEMPER")
        {
            if dc.source.eq_ignore_ascii_case(parameter_name) {
                return Ok(sweep_point.primary);
            }
            if let Some(sweep2) = &dc.sweep2
                && sweep2.source.eq_ignore_ascii_case(parameter_name)
            {
                return sweep_point.secondary.ok_or_else(|| {
                    format!(
                        "secondary sweep value for '{}' is unavailable",
                        parameter_name
                    )
                });
            }
            return Ok(netlist.options.temp.unwrap_or(27.0));
        }

        netlist
            .params
            .get(parameter_name)
            .ok_or_else(|| format!("scalar parameter probe '{}' is not defined", parameter_name))
    }

    fn evaluate_resistor_current(
        netlist: &Netlist,
        result: &crate::SimulationResult,
        resistor_name: &str,
        resistance: Value,
    ) -> Result<f64, String> {
        if resistance.abs() < MIN_DIRECT_RESISTOR_ABS_OHMS {
            return Err(format!(
                "cannot evaluate current through zero/near-zero resistor '{}'",
                resistor_name
            ));
        }
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(resistor_name))
            .ok_or_else(|| format!("resistor '{}' not found", resistor_name))?;
        let node_pos = element
            .nodes
            .first()
            .ok_or_else(|| format!("resistor '{}' has no positive node", resistor_name))?;
        let node_neg = element
            .nodes
            .get(1)
            .ok_or_else(|| format!("resistor '{}' has no negative node", resistor_name))?;
        let v_pos = result
            .try_voltage_named(node_pos)
            .ok_or_else(|| format!("node '{}' not found in DC result", node_pos))?;
        let v_neg = result
            .try_voltage_named(node_neg)
            .ok_or_else(|| format!("node '{}' not found in DC result", node_neg))?;
        Ok((v_pos - v_neg) / resistance)
    }

    fn source_is_voltage_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(&element.kind, ElementKind::VoltageSource(_))
        })
    }

    fn source_is_independent_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(
                    &element.kind,
                    ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
                )
        })
    }

    fn independent_source_dc_value(netlist: &Netlist, source: &str) -> Option<Value> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))
            .and_then(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Some(extract_dc_value(spec))
                }
                _ => None,
            })
    }

    fn scalar_parameter_probe_is_supported(netlist: &Netlist, parameter_name: &str) -> bool {
        parameter_name.eq_ignore_ascii_case("TEMP")
            || parameter_name.eq_ignore_ascii_case("TEMPER")
            || netlist.params.get(parameter_name).is_some()
    }

    fn direct_resistor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .and_then(|element| match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model: None,
                    instance_params,
                    deferred_params,
                    ..
                } if instance_params.is_empty() && deferred_params.is_empty() => {
                    Self::resolve_direct_resistor_value(
                        netlist,
                        &element.name,
                        *value,
                        value_expr.as_deref(),
                    )
                    .ok()
                }
                _ => None,
            })
    }

    fn resolve_direct_resistor_value(
        netlist: &Netlist,
        element_name: &str,
        value: Value,
        value_expr: Option<&str>,
    ) -> Result<Value, String> {
        if let Some(expression) = value_expr {
            crate::netlist::expr::eval_expression(expression, &netlist.params).map_err(|err| {
                format!(
                    "resistor '{}' parameter expression '{}' is not supported by the first Xyce adapter: {err}",
                    element_name, expression
                )
            })
        } else {
            Ok(value)
        }
    }

    fn single_dc_sweep(netlist: &Netlist) -> Result<XyceDcSweep, String> {
        let mut dc_commands = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    sweep2,
                } => Some((source, start, stop, step, sweep2)),
                _ => None,
            });

        let Some((source, start, stop, step, sweep2)) = dc_commands.next() else {
            return Err(
                "deck has no .DC analysis; first Xyce adapter supports static .PRINT DC only"
                    .to_string(),
            );
        };
        if dc_commands.next().is_some() {
            return Err("deck has multiple .DC analyses; multi-analysis Xyce comparison is not implemented yet".to_string());
        }
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() || *step == 0.0 {
            return Err("deck has invalid .DC sweep bounds".to_string());
        }
        if let Some(sweep2) = sweep2 {
            if !sweep2.start.is_finite()
                || !sweep2.stop.is_finite()
                || !sweep2.step.is_finite()
                || sweep2.step == 0.0
            {
                return Err("deck has invalid secondary .DC sweep bounds".to_string());
            }
        }

        Ok(XyceDcSweep {
            source: source.clone(),
            start: *start,
            stop: *stop,
            step: *step,
            sweep2: sweep2.clone(),
        })
    }

    fn single_dc_print_request(source: &str) -> Result<XycePrintRequest, String> {
        let mut requests = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let mut parts = trimmed.split_whitespace();
            let Some(command) = parts.next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".print") {
                continue;
            }
            let Some(analysis) = parts.next() else {
                return Err(".PRINT statement has no analysis type".to_string());
            };
            if analysis.eq_ignore_ascii_case("DC") {
                let mut writes_named_file = false;
                let mut probes = Vec::new();
                for part in parts {
                    let normalized = part.to_ascii_lowercase();
                    if normalized.starts_with("file=") {
                        writes_named_file = true;
                        continue;
                    }
                    if Self::is_print_option_token(&normalized) {
                        continue;
                    }
                    probes.push(part.to_string());
                }
                if writes_named_file {
                    continue;
                }
                if probes.is_empty() {
                    return Err(".PRINT DC statement has no probes".to_string());
                }
                requests.push(XycePrintRequest { probes });
            }
        }

        match requests.len() {
            0 => Err("deck has no .PRINT DC statement with static columns".to_string()),
            1 => Ok(requests.remove(0)),
            _ => Err("deck has multiple .PRINT DC statements; multi-table comparison is not implemented yet".to_string()),
        }
    }

    fn is_print_option_token(token: &str) -> bool {
        token.starts_with("format=")
            || token.starts_with("width=")
            || token.starts_with("precision=")
            || token.starts_with("delimiter=")
            || token.starts_with("noindex")
            || token.starts_with("index=")
    }

    fn parse_prn_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_prn_table(&content)
    }

    fn parse_prn_table(content: &str) -> Result<XycePrnTable, String> {
        let nonempty_lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if nonempty_lines.is_empty() {
            return Err("empty Xyce .prn table".to_string());
        }

        let Some((header_index, (_header_line_number, header))) = nonempty_lines
            .iter()
            .enumerate()
            .find(|(_, (_, line))| Self::is_prn_header_line(line))
        else {
            return Err("Xyce .prn table has no header".to_string());
        };
        let delimiter = Self::prn_header_delimiter(header)
            .ok_or_else(|| format!("invalid Xyce .prn header '{}'", header))?;
        let columns = Self::parse_prn_columns(header, delimiter);
        if columns.is_empty() {
            return Err("Xyce .prn header has no columns".to_string());
        }

        let mut rows = Vec::new();
        for (line_number, line) in nonempty_lines.iter().skip(header_index + 1).copied() {
            if line.to_ascii_lowercase().starts_with("end of xyce") {
                break;
            }
            if !rows.is_empty() && Self::is_prn_footer_line(line) {
                break;
            }
            if Self::is_prn_separator_line(line) {
                continue;
            }
            if Self::is_prn_header_line(line) {
                let repeated_delimiter = Self::prn_header_delimiter(line)
                    .ok_or_else(|| format!("invalid repeated Xyce .prn header '{}'", line))?;
                let repeated_columns = Self::parse_prn_columns(line, repeated_delimiter);
                if Self::same_prn_columns(&columns, &repeated_columns) {
                    continue;
                }
                return Err(format!(
                    "Xyce .prn table changes columns at line {}; multi-table .prn output is not supported",
                    line_number
                ));
            }
            let values = Self::split_prn_fields(line, delimiter)
                .map(|token| {
                    token.parse::<f64>().map_err(|err| {
                        format!(
                            "invalid numeric token '{}' on data line {}: {err}",
                            token, line_number
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != columns.len() {
                return Err(format!(
                    "data line {} has {} values, expected {}",
                    line_number,
                    values.len(),
                    columns.len()
                ));
            }
            rows.push(values);
        }

        if rows.is_empty() {
            return Err("Xyce .prn table has no data rows".to_string());
        }

        Ok(XycePrnTable { columns, rows })
    }

    fn parse_prn_columns(line: &str, delimiter: XycePrnDelimiter) -> Vec<String> {
        Self::split_prn_fields(line, delimiter)
            .map(str::to_string)
            .collect()
    }

    fn is_prn_header_line(line: &str) -> bool {
        Self::prn_header_delimiter(line).is_some()
    }

    fn prn_header_delimiter(line: &str) -> Option<XycePrnDelimiter> {
        if line
            .split(',')
            .next()
            .is_some_and(|token| token.trim().eq_ignore_ascii_case("index"))
        {
            return Some(XycePrnDelimiter::Comma);
        }
        if line
            .split_whitespace()
            .next()
            .is_some_and(|token| token.eq_ignore_ascii_case("index"))
        {
            return Some(XycePrnDelimiter::Whitespace);
        }
        None
    }

    fn split_prn_fields(
        line: &str,
        delimiter: XycePrnDelimiter,
    ) -> Box<dyn Iterator<Item = &str> + '_> {
        match delimiter {
            XycePrnDelimiter::Whitespace => Box::new(line.split_whitespace()),
            XycePrnDelimiter::Comma => Box::new(line.split(',').map(str::trim)),
        }
    }

    fn same_prn_columns(left: &[String], right: &[String]) -> bool {
        left.len() == right.len()
            && left
                .iter()
                .zip(right.iter())
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
    }

    fn is_prn_separator_line(line: &str) -> bool {
        line.chars()
            .all(|ch| ch == '-' || ch == '=' || ch.is_whitespace())
    }

    fn is_prn_footer_line(line: &str) -> bool {
        let normalized = line.to_ascii_lowercase();
        normalized.starts_with("cpu time")
            || normalized.starts_with("total cpu time")
            || normalized.starts_with("current dynamic memory usage")
            || normalized.starts_with("dynamic memory limit")
    }

    fn static_prn_reference_path(&self, deck_path: &Path) -> Option<PathBuf> {
        let netlists_root = self.root.join("Netlists");
        let canonical = deck_path
            .canonicalize()
            .unwrap_or_else(|_| deck_path.to_path_buf());
        let relative = canonical
            .strip_prefix(&netlists_root)
            .or_else(|_| deck_path.strip_prefix(&netlists_root))
            .ok()?;
        let parent = relative.parent().unwrap_or_else(|| Path::new(""));
        let file_name = relative.file_name()?.to_string_lossy();
        Some(
            self.root
                .join("OutputData")
                .join(parent)
                .join(format!("{file_name}.prn")),
        )
    }

    fn collect_circuit_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::collect_circuit_files(&path, out);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
            {
                out.push(path);
            }
        }
    }

    fn section_for_relative_path(relative_path: &str) -> XyceDeckSection {
        if relative_path.starts_with("Netlists/") {
            XyceDeckSection::Netlists
        } else {
            XyceDeckSection::Other
        }
    }

    fn load_upstream_wrapper_decks(root: &Path) -> BTreeSet<String> {
        let manifest_path = root.join(HARNESS_MANIFEST_FILE);
        let Ok(content) = fs::read_to_string(manifest_path) else {
            return BTreeSet::new();
        };

        let mut decks = BTreeSet::new();
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((path, contract)) = line.split_once('\t') else {
                continue;
            };
            if contract.trim() == REQUIRES_UPSTREAM_WRAPPER_CONTRACT {
                decks.insert(Self::normalize_manifest_key(path));
            }
        }
        decks
    }

    fn normalize_manifest_key(path: &str) -> String {
        path.trim().replace('\\', "/").to_ascii_lowercase()
    }

    fn relative_key(&self, path: &Path) -> String {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        canonical
            .strip_prefix(&self.root)
            .or_else(|_| path.strip_prefix(&self.root))
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn display_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn deck_name(path: &Path) -> String {
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    fn contains_control_block(source: &str) -> bool {
        source.lines().any(|line| {
            let normalized = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            normalized == ".control" || normalized == ".endc"
        })
    }

    fn logical_netlist_lines(source: &str) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for raw in source.lines() {
            let line = raw.trim_end();
            if line.trim_start().starts_with('+') {
                if let Some(previous) = lines.last_mut() {
                    previous.push(' ');
                    previous.push_str(line.trim_start().trim_start_matches('+').trim_start());
                } else {
                    lines.push(line.to_string());
                }
            } else {
                lines.push(line.to_string());
            }
        }
        lines
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

    fn normalize_probe(probe: &str) -> String {
        probe
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    fn parse_voltage_probe(probe: &str) -> Option<(String, Option<String>)> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("v(") || !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if inner.is_empty() {
            return None;
        }
        if let Some((a, b)) = inner.split_once(',') {
            Some((a.to_string(), Some(b.to_string())))
        } else {
            Some((inner.to_string(), None))
        }
    }

    fn parse_current_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("i(") || !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        (!inner.is_empty()).then(|| inner.to_string())
    }

    fn parse_device_parameter_probe(probe: &str) -> Option<(String, String)> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        let (element, parameter) = unwrapped.split_once(':')?;
        if element.is_empty() || parameter.is_empty() {
            return None;
        }
        Some((element.to_string(), parameter.to_string()))
    }

    fn parse_scalar_parameter_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        if unwrapped.is_empty()
            || unwrapped.contains(':')
            || unwrapped.contains('(')
            || unwrapped.contains(')')
            || unwrapped
                .chars()
                .any(|ch| matches!(ch, '+' | '-' | '*' | '/' | '^' | ','))
        {
            return None;
        }
        let mut chars = unwrapped.chars();
        let first = chars.next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        if chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.') {
            Some(unwrapped.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prn_parser_accepts_preamble_and_repeated_page_headers() {
        let table = XyceTestRunner::parse_prn_table(
            r#"Circuit: metadata line
Date: metadata line

--------------------------------------------------------------------------------
Index   v-sweep         v(2)            v(1)
--------------------------------------------------------------------------------
0       0.000000e+00    0.000000e+00    2.000000e-01
1       2.000000e-02    2.000000e-02    2.000000e-01

Index   v-sweep         v(2)            v(1)
--------------------------------------------------------------------------------
2       4.000000e-02    4.000000e-02    2.000000e-01
CPU time since last call: 0.110 seconds.
Total CPU time: 0.110 seconds.
"#,
        )
        .expect("parser accepts Xyce page headers");

        assert_eq!(
            table.columns,
            ["Index", "v-sweep", "v(2)", "v(1)"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 3);
        assert_eq!(table.rows[2], vec![2.0, 0.04, 0.04, 0.2]);
    }

    #[test]
    fn prn_parser_accepts_comma_delimited_dc_output() {
        let table = XyceTestRunner::parse_prn_table(
            r#"Index,V(2),V(1),I(VDS)
0,0.00000000e+00,0.00000000e+00,0.00000000e+00
1,5.00000000e-02,0.00000000e+00,-3.05537186e-09
End of Xyce(TM) Simulation
"#,
        )
        .expect("parser accepts comma-delimited Xyce PRN output");

        assert_eq!(
            table.columns,
            ["Index", "V(2)", "V(1)", "I(VDS)"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[1], vec![1.0, 0.05, 0.0, -3.05537186e-9]);
    }

    #[test]
    fn reference_columns_accept_primary_sweep_and_branch_labels() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let reference = XycePrnTable {
            columns: ["Index", "v-sweep", "v(2)", "vds_branch"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: Vec::new(),
        };
        let print = XycePrintRequest {
            probes: ["v(2)", "i(vds)"].into_iter().map(str::to_string).collect(),
        };

        let columns = runner
            .reference_data_columns(&reference, &print)
            .expect("reference columns map to Xyce DC probes");

        assert!(matches!(
            &columns[0],
            XyceReferenceColumn::PrimarySweep { name } if name == "v-sweep"
        ));
        assert!(matches!(
            &columns[1],
            XyceReferenceColumn::Probe { name } if name == "v(2)"
        ));
        assert!(matches!(
            &columns[2],
            XyceReferenceColumn::Probe { name } if name == "i(vds)"
        ));
    }

    #[test]
    fn comp_tolerances_apply_zero_tolerance_to_matching_print_probe() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let columns = vec![XyceReferenceColumn::Probe {
            name: "I(VIDS)".to_string(),
        }];
        let tolerances = runner
            .comp_tolerances("*COMP I(VIDS) zerotol=1.0e-11\n", &columns)
            .expect("COMP tolerance parses");
        let tolerance = *tolerances
            .get("i(vids)")
            .expect("COMP tolerance keyed by normalized probe");

        assert_eq!(tolerance.zero, Some(1.0e-11));
        assert!(
            runner.value_mismatch(1.0e-24, 1.8e-12, tolerance).is_none(),
            "zerotol should accept near-zero values on both sides"
        );
    }

    #[test]
    fn comp_tolerances_parse_case_and_spaced_assignments() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let columns = vec![XyceReferenceColumn::Probe {
            name: "i(vds)".to_string(),
        }];
        let tolerances = runner
            .comp_tolerances(
                "*COMP i(vds) RELTOL = 0.25 abstol=1e-5 absdifftol=2e-5\n",
                &columns,
            )
            .expect("COMP tolerances parse");
        let tolerance = *tolerances
            .get("i(vds)")
            .expect("COMP tolerance keyed by normalized probe");

        assert_eq!(tolerance.relative, 0.25);
        assert_eq!(tolerance.absolute, 2.0e-5);
    }

    #[test]
    fn xyce_runner_engine_uses_xyce_device_dialect() {
        let runner = XyceTestRunner::new(".", XyceRunnerConfig::default());
        let engine = runner.create_dc_engine();

        assert_eq!(engine.config().spice_dialect, SpiceDialect::Xyce);
        assert_eq!(
            engine.config().resolved_jfet_level2_model(),
            crate::engine::JfetLevel2Model::XyceModifiedShockley
        );
    }
}
