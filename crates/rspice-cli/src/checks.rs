//! The checks a run reports beside its results: the deck's `.ASSERT` reports
//! and the HDL's severity-task records.
//!
//! One CLI process runs one command, so every run of a multi-run deck records
//! into the same log, and the text report, the JSON summary and the exit
//! status read all of them. An assertion fails the run's verdict when it is
//! an ERROR-level assertion with a violation, or when it could not be
//! evaluated at all; a NOTICE or WARNING violation is reported only.

use crate::cli::CliError;
use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::config::{HdlOutputRecord, HdlPoint, HdlStream};
use rspice_core::netlist::{AssertAnalysis, AssertLevel};
use std::sync::{Mutex, OnceLock};

/// One assertion report and the run that published it.
#[derive(Debug, Clone)]
pub(crate) struct RunAssertion {
    /// The run's label in a multi-run deck; `None` for a single run.
    pub(crate) run: Option<String>,
    pub(crate) report: AssertionReport,
}

fn log() -> &'static Mutex<Vec<RunAssertion>> {
    static LOG: OnceLock<Mutex<Vec<RunAssertion>>> = OnceLock::new();
    LOG.get_or_init(Default::default)
}

/// Record the reports one analysis of one run published.
pub(crate) fn record(run: Option<&str>, reports: Vec<AssertionReport>) {
    if reports.is_empty() {
        return;
    }
    let mut log = log()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    log.extend(reports.into_iter().map(|report| RunAssertion {
        run: run.map(str::to_owned),
        report,
    }));
}

/// Every report recorded so far, in recording order.
pub(crate) fn assertions() -> Vec<RunAssertion> {
    log()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

/// Whether a report fails the run's verdict.
pub(crate) fn fails(report: &AssertionReport) -> bool {
    report.error.is_some() || (report.level == AssertLevel::Error && !report.violations.is_empty())
}

/// The name a row shows: the card's, with the instance a device form checked.
fn row_name(report: &AssertionReport) -> String {
    match &report.instance {
        Some(instance) => format!("{}[{instance}]", report.name),
        None => report.name.clone(),
    }
}

/// A coordinate as its analysis reads it: a time with an SI prefix, a swept
/// value as a number.
fn coordinate(analysis: AssertAnalysis, value: f64) -> String {
    match analysis {
        AssertAnalysis::Tran => crate::console::si_seconds(value),
        AssertAnalysis::Op | AssertAnalysis::Dc => crate::report::format_spice_exponent(value),
    }
}

fn first_interval(analysis: AssertAnalysis, violation: &AssertViolation) -> String {
    match (analysis, violation.start, violation.end) {
        (AssertAnalysis::Op, _, _) | (_, None, _) => "at the operating point".to_string(),
        (AssertAnalysis::Dc, Some(start), Some(end)) => format!(
            "[{}, {}]",
            coordinate(analysis, start),
            coordinate(analysis, end)
        ),
        (_, Some(start), Some(end)) => format!(
            "[{}, {})",
            coordinate(analysis, start),
            coordinate(analysis, end)
        ),
        (_, Some(start), None) => format!("[{}, open)", coordinate(analysis, start)),
    }
}

/// The text report's closing rows: one per violated or unevaluated
/// assertion, with its level, violation count, first interval and extreme.
pub(crate) fn print_rows(assertions: &[RunAssertion]) -> Result<(), CliError> {
    let rows = assertions
        .iter()
        .filter(|entry| entry.report.error.is_some() || !entry.report.violations.is_empty())
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return Ok(());
    }
    crate::console::line(format_args!("\nAssertions:"))?;
    for RunAssertion { run, report } in rows {
        let run = run
            .as_deref()
            .map(|run| format!(" [{run}]"))
            .unwrap_or_default();
        let name = row_name(report);
        let analysis = report.analysis;
        if let Some(error) = &report.error {
            crate::console::line(format_args!(
                "  {name}{run} ({}, {analysis}): not evaluated: {error}",
                report.level
            ))?;
            continue;
        }
        let first = &report.violations[0];
        let count = report.violations.len();
        let mut row = format!(
            "  {name}{run} ({}, {analysis}): {count} violation{}, first {}",
            report.level,
            if count == 1 { "" } else { "s" },
            first_interval(analysis, first)
        );
        if let Some(extreme) = first.extreme {
            row.push_str(&format!(
                ", extreme {}",
                crate::report::format_spice_exponent(extreme)
            ));
            if let Some(at) = first.at.filter(|_| analysis != AssertAnalysis::Op) {
                row.push_str(&format!(" at {}", coordinate(analysis, at)));
            }
        }
        if report.unknown_stretches > 0 {
            row.push_str(&format!(
                ", {} X/Z stretch(es) ignored",
                report.unknown_stretches
            ));
        }
        if let Some(message) = &report.message {
            row.push_str(&format!(": {message}"));
        }
        crate::console::line(format_args!("{row}"))?;
    }
    Ok(())
}

/// One assertion report as the JSON summary lists it.
pub(crate) fn assertion_json(entry: &RunAssertion) -> serde_json::Value {
    let report = &entry.report;
    serde_json::json!({
        "run": entry.run,
        "name": report.name,
        "analysis": report.analysis.to_string(),
        "target": report.target,
        "instance": report.instance,
        "level": report.level.to_string(),
        "message": report.message,
        "file": report
            .origin
            .path
            .as_ref()
            .map(|path| path.display().to_string()),
        "line": report.origin.line,
        "passed": !fails(report),
        "error": report.error,
        "unknown_stretches": report.unknown_stretches,
        "violations": report.violations.iter().map(|violation| serde_json::json!({
            "start": violation.start,
            "end": violation.end,
            "open": report.analysis == AssertAnalysis::Tran && violation.end.is_none(),
            "extreme": violation.extreme,
            "at": violation.at,
        })).collect::<Vec<_>>(),
    })
}

/// One HDL severity-task record as the JSON summary lists it.
pub(crate) fn hdl_json(record: &HdlOutputRecord) -> Option<serde_json::Value> {
    let HdlStream::Diagnostic { severity } = &record.stream else {
        return None;
    };
    let point = match record.point {
        HdlPoint::Initialization => serde_json::json!({ "kind": "initialization" }),
        HdlPoint::OperatingPoint => serde_json::json!({ "kind": "op" }),
        HdlPoint::DcSweep { value } => serde_json::json!({ "kind": "dc", "value": value }),
        HdlPoint::Transient { time } => serde_json::json!({ "kind": "tran", "time": time }),
    };
    Some(serde_json::json!({
        "severity": severity.label().to_ascii_lowercase(),
        "text": record.text,
        "time": record.seconds,
        "point": point,
        "scope": record.origin.scope,
        "file": record.origin.file,
        "line": record.origin.line,
    }))
}
