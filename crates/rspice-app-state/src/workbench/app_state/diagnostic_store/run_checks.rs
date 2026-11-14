//! What a run's checks reported, as records beside its failures.
//!
//! A testbench's `$error` is not a failure of the simulator: the analysis
//! completed, its results stand, and the design is what the check found
//! wanting. So each record a run's checks left is a finding of its own, with
//! what Xcelium, Questa and Spectre state for one: the severity, the
//! simulation time, the scope that reported it, and the source line it was
//! written on. One row per HDL severity-task record, per `.ASSERT` that was
//! violated or could not be evaluated, and for a `$fatal` or `$stop` that
//! ended the run before its stop point.
//!
//! The rows stand in the run's own batch, as its notices do, and are
//! superseded by the next run of the design the same way.

use rspice_core::config::{HdlOrigin, HdlPoint, HdlSeverity};
use rspice_core::netlist::{AssertAnalysis, AssertLevel};
use rspice_core::{ModelFinish, ModelFinishPoint, ModelFinishTask};
use rspice_results::analysis_result::{HdlDiagnostic, RunChecks, samples_after_stop};

use crate::quantity::engineering::{EngineeringPrecision, format_engineering_display_with};
use crate::state::{PreflightRemediation, ProjectSourceOwner};

use super::super::model::{Model, SlotFacts};
use super::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};

/// The HDL's severity tasks and the tasks that end a run.
const HDL_PRODUCER: &str = "rspice.engine.hdl";
/// The deck's `.ASSERT` evaluation.
const ASSERT_PRODUCER: &str = "rspice.engine.assert";

pub const HDL_INFO_CODE: &str = "HDL-INFO";
pub const HDL_WARNING_CODE: &str = "HDL-WARNING";
pub const HDL_ERROR_CODE: &str = "HDL-ERROR";
pub const HDL_FATAL_CODE: &str = "HDL-FATAL";
pub const HDL_STOP_CODE: &str = "HDL-STOP";
pub const ASSERT_VIOLATED_CODE: &str = "ASSERT-VIOLATED";
pub const ASSERT_UNEVALUATED_CODE: &str = "ASSERT-NOT-EVALUATED";
pub const ASSERT_AFTER_STOP_CODE: &str = "ASSERT-AFTER-STOP";

/// One row a run's checks give, before its identity is minted.
struct CheckRow {
    key: FindingKey,
    level: DiagnosticLevel,
    object: String,
    source_line: Option<String>,
    message: String,
    details: String,
    route: Option<DiagnosticRoute>,
}

impl Model {
    /// File the rows one analysis's checks gave in its run's batch.
    ///
    /// Called once per analysis that reported checks; the rows are added to
    /// what the run's batch already holds, so a run of several analyses lists
    /// each one's checks under it. Nothing is filed for checks that reported
    /// nothing worth a row: a `$finish`, or an `.ASSERT` that held.
    pub fn publish_run_checks(
        &mut self,
        analysis: &str,
        checks: &RunChecks,
        run_sequence: u64,
        facts: SlotFacts,
    ) {
        let rows = self.check_rows(analysis, checks, run_sequence);
        if rows.is_empty() {
            return;
        }
        let inputs = DiagnosticInputs::Run { run_sequence };
        let opened = self.diagnostics.batch(&inputs);
        let dispatched = opened
            .map(|batch| batch.input_digest)
            .or_else(|| self.diagnostics.run_baselines.get(&run_sequence).copied());
        let mut records: Vec<DiagnosticRecord> = opened
            .map(|batch| batch.records.clone())
            .unwrap_or_default();
        let keys: Vec<FindingKey> = records
            .iter()
            .map(|record| record.key.clone())
            .chain(rows.iter().map(|row| row.key.clone()))
            .collect();
        let canonicals = mint_canonical(
            &keys,
            run_sequence,
            &format!("run:{run_sequence}:{analysis}:checks"),
        );
        let project = self.workspace.content.project.id();
        let filed = records.len();
        records.extend(
            rows.into_iter()
                .zip(canonicals.into_iter().skip(filed))
                .map(|(row, canonical)| {
                    DiagnosticRecord::new(
                        canonical,
                        row.level,
                        DiagnosticSubject::new(project, row.object).on_source_line(row.source_line),
                        row.message,
                    )
                    .keyed(row.key)
                    .with_details(row.details)
                    .with_route(row.route)
                }),
        );
        let published = match dispatched {
            Some(digest) => self.diagnostics.insert(digest, inputs, records, None),
            None => self.publish_diagnostics(inputs, records, facts),
        };
        if let Err(error) = published {
            self.record(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Studio,
                format!("Findings could not be recorded: {error}"),
            );
        }
    }

    /// The rows one analysis's checks give, in the order they were reported:
    /// the HDL's records, then the assertions, then the task that ended the
    /// run where it is not already one of the records.
    fn check_rows(&self, analysis: &str, checks: &RunChecks, run_sequence: u64) -> Vec<CheckRow> {
        let scope = super::producers::run_scope(self, run_sequence);
        let manual_deck = self.ran_a_manual_deck(run_sequence);
        let deck_path = self
            .workspace
            .content
            .netlist_source_path
            .as_deref()
            .filter(|_| manual_deck);
        let place = |kind: &str, object: &str, file: &str, line: u32| {
            format!("{analysis} · {kind} · {object} · {file}:{line}")
        };
        let mut rows: Vec<CheckRow> = checks
            .hdl
            .iter()
            .map(|record| {
                let HdlDiagnostic {
                    severity,
                    text,
                    point,
                    origin,
                } = record;
                let (level, code) = match severity {
                    HdlSeverity::Info => (DiagnosticLevel::Info, HDL_INFO_CODE),
                    HdlSeverity::Warning => (DiagnosticLevel::Warning, HDL_WARNING_CODE),
                    HdlSeverity::Error => (DiagnosticLevel::Error, HDL_ERROR_CODE),
                    HdlSeverity::Fatal => (DiagnosticLevel::Error, HDL_FATAL_CODE),
                };
                let task = format!("${}", severity.label().to_ascii_lowercase());
                CheckRow {
                    key: FindingKey::new(
                        HDL_PRODUCER,
                        code,
                        scope.clone(),
                        place("hdl", &origin.scope, &origin.file, origin.line),
                    ),
                    level,
                    object: scope_name(&origin.scope, analysis),
                    source_line: source_line(&origin.file, origin.line),
                    message: format!(
                        "{task} {}: {}",
                        hdl_point(point),
                        text.trim_end_matches(['\r', '\n'])
                    ),
                    details: format!("Reported by {task} during {analysis}"),
                    route: self.hdl_source_route(&origin.file, origin.line, manual_deck),
                }
            })
            .collect();
        rows.extend(checks.assertions.iter().filter_map(|report| {
            let violated = !report.violations.is_empty();
            let (level, code) = match (&report.error, report.level, violated) {
                (Some(_), _, _) => (DiagnosticLevel::Error, ASSERT_UNEVALUATED_CODE),
                (None, _, false) if samples_after_stop(report).is_some() => {
                    (DiagnosticLevel::Warning, ASSERT_AFTER_STOP_CODE)
                }
                (None, _, false) => return None,
                (None, AssertLevel::Error, true) => (DiagnosticLevel::Error, ASSERT_VIOLATED_CODE),
                (None, AssertLevel::Warning, true) => {
                    (DiagnosticLevel::Warning, ASSERT_VIOLATED_CODE)
                }
                (None, AssertLevel::Notice, true) => (DiagnosticLevel::Info, ASSERT_VIOLATED_CODE),
            };
            let name = match &report.instance {
                Some(instance) => format!("{}[{instance}]", report.name),
                None => report.name.clone(),
            };
            let file = report
                .origin
                .path
                .as_ref()
                .map_or_else(|| "deck".to_owned(), |path| path.display().to_string());
            let line = u32::try_from(report.origin.line).unwrap_or(u32::MAX);
            // The card is a line of the deck the run read, so it is repaired
            // there; a card read from an included file names a line the deck
            // does not have, and is offered no jump.
            let in_deck = report
                .origin
                .path
                .as_deref()
                .is_none_or(|path| deck_path == Some(path));
            let route = in_deck.then_some(DiagnosticRoute::Repair(
                PreflightRemediation::NetlistSource {
                    line: Some(report.origin.line).filter(|line| *line > 0),
                },
            ));
            Some(CheckRow {
                key: FindingKey::new(
                    ASSERT_PRODUCER,
                    code,
                    scope.clone(),
                    place("assert", &name, &file, line),
                ),
                level,
                object: name,
                source_line: report.origin.path.as_ref().map_or_else(
                    || (report.origin.line > 0).then(|| format!("line {}", report.origin.line)),
                    |path| source_line(&path.display().to_string(), line),
                ),
                message: assertion_message(report),
                details: format!(
                    "{} assertion on {}, checked after {}",
                    report.level, report.target, report.analysis
                ),
                route,
            })
        }));
        if let Some(finish) = checks
            .finish
            .as_ref()
            .filter(|finish| !finish_is_listed(finish, &checks.hdl))
        {
            let (level, code) = match finish.task {
                ModelFinishTask::Fatal => (DiagnosticLevel::Error, HDL_FATAL_CODE),
                ModelFinishTask::Stop => (DiagnosticLevel::Info, HDL_STOP_CODE),
                ModelFinishTask::Finish => return rows,
            };
            let (file, line) = finish
                .source
                .as_ref()
                .map_or(("", 0), |source| (source.file.as_str(), source.line));
            let message = match &finish.message {
                Some(message) => format!(
                    "{} {}: {message}",
                    finish.task.name(),
                    finish_point(finish.point)
                ),
                None => format!(
                    "{} {} ended the run before its stop point",
                    finish.task.name(),
                    finish_point(finish.point)
                ),
            };
            rows.push(CheckRow {
                key: FindingKey::new(
                    HDL_PRODUCER,
                    code,
                    scope.clone(),
                    place("finish", &finish.instance, file, line),
                ),
                level,
                object: scope_name(&finish.instance, analysis),
                source_line: source_line(file, line),
                message,
                details: format!(
                    "{} in model {} ended {analysis}",
                    finish.task.name(),
                    finish.model
                ),
                route: self.hdl_source_route(file, line, manual_deck),
            });
        }
        rows
    }

    /// Where the source line an HDL call of run `run_sequence` was written
    /// on is edited, as that run's Problems rows route to it: the Console
    /// opens a line the call wrote the same way. `None` where the app holds
    /// no editor for that file.
    pub fn hdl_call_route(
        &self,
        run_sequence: u64,
        file: &str,
        line: u32,
    ) -> Option<DiagnosticRoute> {
        self.hdl_source_route(file, line, self.ran_a_manual_deck(run_sequence))
    }

    fn ran_a_manual_deck(&self, run_sequence: u64) -> bool {
        self.simulation
            .retained
            .run_by_sequence(run_sequence)
            .and_then(|run| run.prepared_receipt())
            .is_some_and(|receipt| {
                receipt.source_domain()
                    == rspice_results::provenance::AnalysisResultSourceDomain::ManualDeck
            })
    }

    /// Where an HDL call's source line is edited, where the app edits it.
    ///
    /// A run of the drawn design compiles its Verilog-A from the project's
    /// bundles, so a call names a document by its logical path in one of
    /// them, and the bundle's editor opens it on the line. A manual deck's
    /// `.hdl` or `.va` file is a file on disk the deck reads, not a document
    /// the app holds, and no route opens it: the row is stated without a jump
    /// rather than offering one to some other document.
    fn hdl_source_route(
        &self,
        file: &str,
        line: u32,
        manual_deck: bool,
    ) -> Option<DiagnosticRoute> {
        if manual_deck || file.is_empty() {
            return None;
        }
        let mut holders = self
            .workspace
            .content
            .project_sources
            .iter_bundles()
            .filter(|bundle| matches!(bundle.owner(), ProjectSourceOwner::CellView { .. }))
            .filter(|bundle| bundle.contains_file(file));
        let bundle = holders.next()?;
        // Two cell views that each hold a file of this name leave the call
        // unattributed; opening either would be a guess.
        if holders.next().is_some() {
            return None;
        }
        Some(DiagnosticRoute::CodeDocument {
            bundle: bundle.id(),
            logical_path: file.into(),
            line: usize::try_from(line).ok().filter(|line| *line > 0),
            column: None,
        })
    }
}

/// A `$fatal` that ended the run is already one of the HDL's records, which
/// state it with its message; its finish is not listed a second time.
fn finish_is_listed(finish: &ModelFinish, hdl: &[HdlDiagnostic]) -> bool {
    finish.task == ModelFinishTask::Fatal
        && hdl.iter().any(|record| {
            record.severity == HdlSeverity::Fatal
                && finish.source.as_ref().is_none_or(|source| {
                    let HdlOrigin { file, line, .. } = &record.origin;
                    *file == source.file && *line == source.line
                })
        })
}

/// The scope that reported a record, or the analysis where it named none.
fn scope_name(scope: &str, analysis: &str) -> String {
    if scope.is_empty() {
        analysis.to_owned()
    } else {
        scope.to_owned()
    }
}

/// `tb.v:40`, or nothing where the call's source is not known.
fn source_line(file: &str, line: u32) -> Option<String> {
    (!file.is_empty() && line > 0).then(|| format!("{file}:{line}"))
}

fn seconds(value: f64) -> String {
    format!(
        "{}s",
        format_engineering_display_with(value, EngineeringPrecision::Adaptive)
    )
}

fn hdl_point(point: &HdlPoint) -> String {
    match point {
        HdlPoint::Initialization => "during initialization".to_owned(),
        HdlPoint::OperatingPoint => "at the operating point".to_owned(),
        HdlPoint::DcSweep { value } => format!(
            "at sweep value {}",
            format_engineering_display_with(*value, EngineeringPrecision::Adaptive)
        ),
        HdlPoint::Transient { time } => format!("at t = {}", seconds(*time)),
    }
}

fn finish_point(point: ModelFinishPoint) -> String {
    match point {
        ModelFinishPoint::Initialization => "during initialization".to_owned(),
        ModelFinishPoint::OperatingPoint => "at the operating point".to_owned(),
        ModelFinishPoint::DcSweep { value } => format!(
            "at sweep value {}",
            format_engineering_display_with(value, EngineeringPrecision::Adaptive)
        ),
        ModelFinishPoint::Transient { time } => format!("at t = {}", seconds(time)),
        ModelFinishPoint::Frequency { frequency } => format!(
            "at {}Hz",
            format_engineering_display_with(frequency, EngineeringPrecision::Adaptive)
        ),
    }
}

/// What a violated assertion found, as the CLI's assertion rows state it:
/// how many violations, the first one's interval, its extreme and where.
fn assertion_message(report: &rspice_core::analysis::AssertionReport) -> String {
    if let Some(error) = &report.error {
        return format!("Not evaluated: {error}");
    }
    if report.violations.is_empty() {
        // The vector file's samples after the stop time, which the
        // evaluator counts in words of its own.
        return report.message.clone().unwrap_or_default();
    }
    let analysis = report.analysis;
    let coordinate = |value: f64| match analysis {
        AssertAnalysis::Tran => seconds(value),
        AssertAnalysis::Op | AssertAnalysis::Dc => {
            format_engineering_display_with(value, EngineeringPrecision::Adaptive)
        }
    };
    let count = report.violations.len();
    let first = &report.violations[0];
    let interval = match (analysis, first.start, first.end) {
        (AssertAnalysis::Op, _, _) | (_, None, _) => "at the operating point".to_owned(),
        (AssertAnalysis::Dc, Some(start), Some(end)) => {
            format!("[{}, {}]", coordinate(start), coordinate(end))
        }
        (_, Some(start), Some(end)) => format!("[{}, {})", coordinate(start), coordinate(end)),
        (_, Some(start), None) => format!("[{}, open)", coordinate(start)),
    };
    let mut message = format!(
        "{count} violation{}, first {interval}",
        if count == 1 { "" } else { "s" }
    );
    if let Some(extreme) = first.extreme {
        message.push_str(&format!(
            ", extreme {}",
            format_engineering_display_with(extreme, EngineeringPrecision::Adaptive)
        ));
        if let Some(at) = first.at.filter(|_| analysis != AssertAnalysis::Op) {
            message.push_str(&format!(" at {}", coordinate(at)));
        }
    }
    if report.unknown_stretches > 0 {
        message.push_str(&format!(
            ", {} X/Z stretch(es) ignored",
            report.unknown_stretches
        ));
    }
    if let Some(text) = &report.message {
        message.push_str(&format!(": {text}"));
    }
    message
}
