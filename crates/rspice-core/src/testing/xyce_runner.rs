//! Xyce regression corpus runner.
//!
//! The upstream Xyce suite is vendored as a runtime corpus. RSpice keeps the
//! netlists, reference output data, and licensing/provenance files, but omits
//! upstream platform-specific harness scripts. Regression execution is
//! Rust-native: every retained `.cir` deck is discovered and reported, and only
//! decks with a supported, checked-in static `.prn` oracle are numerically
//! executed.

use crate::abort_signal::AbortSignal;
use crate::engine::{ConvergenceConfig, SimulationConfig, SimulationError};
use crate::netlist::{AnalysisCommand, ElementKind, Netlist};
use crate::{Engine, Value};
use std::collections::BTreeSet;
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

#[derive(Debug, Clone)]
struct XyceDcSweep {
    source: String,
    start: Value,
    stop: Value,
    step: Value,
}

#[derive(Debug, Clone)]
struct XycePrnTable {
    columns: Vec<String>,
    rows: Vec<Vec<f64>>,
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
        Self::validate_linear_resistive_dc_contract(&netlist, &dc, &print)?;

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
        let results = match engine.run_dc_sweep_with_abort(
            &netlist,
            &plan.dc.source,
            plan.dc.start,
            plan.dc.stop,
            plan.dc.step,
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

        let mismatches =
            match self.compare_dc_prn_reference(&reference, &plan.print, &netlist, &results) {
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
        if reference.columns.len() != print.probes.len() + 1 {
            return Err(format!(
                "reference column count ({}) does not match .PRINT DC probe count ({}) plus Index",
                reference.columns.len(),
                print.probes.len()
            ));
        }
        if reference.rows.len() != results.len() {
            return Err(format!(
                "reference row count ({}) does not match simulation point count ({})",
                reference.rows.len(),
                results.len()
            ));
        }

        let column_to_probe = self.reference_columns_to_probes(reference, print)?;
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

            for (column_index, probe) in column_to_probe.iter().enumerate() {
                let expected = row[column_index + 1];
                let actual = Self::evaluate_dc_probe(probe, netlist, &results[row_index].1)?;
                if let Some(relative_error) = self.value_mismatch(expected, actual) {
                    mismatches.push(XyceValueMismatch {
                        row: row_index,
                        probe: probe.clone(),
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

    fn reference_columns_to_probes(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
    ) -> Result<Vec<String>, String> {
        let mut probes = Vec::with_capacity(print.probes.len());
        for (column, probe) in reference.columns.iter().skip(1).zip(print.probes.iter()) {
            if Self::normalize_probe(column) != Self::normalize_probe(probe) {
                return Err(format!(
                    "reference column '{}' does not match .PRINT probe '{}'",
                    column, probe
                ));
            }
            probes.push(probe.clone());
        }
        Ok(probes)
    }

    fn value_mismatch(&self, expected: f64, actual: f64) -> Option<f64> {
        if !expected.is_finite() || !actual.is_finite() {
            return Some(f64::INFINITY);
        }
        let abs_error = (expected - actual).abs();
        if abs_error <= self.config.absolute_tolerance {
            return None;
        }
        let scale = expected
            .abs()
            .max(actual.abs())
            .max(self.config.absolute_tolerance);
        let relative_error = abs_error / scale;
        (relative_error > self.config.relative_tolerance).then_some(relative_error)
    }

    fn validate_linear_resistive_dc_contract(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if !Self::source_is_voltage_source(netlist, &dc.source)
            && !dc.source.eq_ignore_ascii_case("TEMP")
            && !dc.source.eq_ignore_ascii_case("TEMPER")
        {
            return Err(format!(
                "DC sweep source '{}' is not a supported top-level voltage source or TEMP sweep",
                dc.source
            ));
        }

        if !netlist.models.is_empty() {
            return Err("deck contains model cards; first Xyce adapter supports direct linear R/V/I DC decks only".to_string());
        }
        if !netlist.subcircuits.is_empty() {
            return Err("deck contains subcircuits; first Xyce adapter supports direct linear R/V/I DC decks only".to_string());
        }

        for element in &netlist.elements {
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    ..
                } => {
                    if value_expr.is_some() || model.is_some() {
                        return Err(format!(
                            "resistor '{}' uses deferred/model semantics not implemented in the first Xyce adapter",
                            element.name
                        ));
                    }
                    if !value.is_finite() || *value <= 0.0 {
                        return Err(format!(
                            "resistor '{}' has non-positive resistance ({value:.3e}); negative and short-resistor semantics are not implemented in the first Xyce adapter",
                            element.name
                        ));
                    }
                    if value.abs() < MIN_DIRECT_RESISTOR_ABS_OHMS {
                        return Err(format!(
                            "resistor '{}' has zero/near-zero resistance ({value:.3e}); short-resistor semantics are not implemented in the first Xyce adapter",
                            element.name
                        ));
                    }
                }
                ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_) => {}
                _ => {
                    return Err(format!(
                        "element '{}' is outside the first Xyce adapter's linear R/V/I DC contract",
                        element.name
                    ));
                }
            }
        }

        for probe in &print.probes {
            Self::validate_dc_probe(probe, netlist)?;
        }

        Ok(())
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
        if let Some(element_name) = Self::parse_current_probe(&normalized) {
            if Self::source_is_voltage_source(netlist, &element_name) {
                return Ok(());
            }
            if let Some(resistance) = Self::direct_resistor_value(netlist, &element_name) {
                if resistance.is_finite()
                    && resistance > 0.0
                    && resistance.abs() >= MIN_DIRECT_RESISTOR_ABS_OHMS
                {
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
                && matches!(element.kind, ElementKind::VoltageSource(_))
        })
    }

    fn direct_resistor_value(netlist: &Netlist, name: &str) -> Option<Value> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .and_then(|element| match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    ..
                } => Some(*value),
                _ => None,
            })
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
        if sweep2.is_some() {
            return Err("deck uses a two-dimensional .DC sweep; first Xyce adapter supports one-dimensional DC sweeps only".to_string());
        }
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() || *step == 0.0 {
            return Err("deck has invalid .DC sweep bounds".to_string());
        }

        Ok(XyceDcSweep {
            source: source.clone(),
            start: *start,
            stop: *stop,
            step: *step,
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
                let probes = parts.map(str::to_string).collect::<Vec<_>>();
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

    fn parse_prn_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_prn_table(&content)
    }

    fn parse_prn_table(content: &str) -> Result<XycePrnTable, String> {
        let mut lines = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty());
        let header = lines
            .next()
            .ok_or_else(|| "empty Xyce .prn table".to_string())?;
        if header.to_ascii_lowercase().starts_with("end of xyce") {
            return Err("Xyce .prn table has no header".to_string());
        }
        let columns = header
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        if columns.is_empty() {
            return Err("Xyce .prn header has no columns".to_string());
        }

        let mut rows = Vec::new();
        for (line_index, line) in lines.enumerate() {
            if line.to_ascii_lowercase().starts_with("end of xyce") {
                break;
            }
            let values = line
                .split_whitespace()
                .map(|token| {
                    token.parse::<f64>().map_err(|err| {
                        format!(
                            "invalid numeric token '{}' on data line {}: {err}",
                            token,
                            line_index + 2
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != columns.len() {
                return Err(format!(
                    "data line {} has {} values, expected {}",
                    line_index + 2,
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
}
