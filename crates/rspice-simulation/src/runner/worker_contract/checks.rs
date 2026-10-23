//! A run's checks on the worker wire: the HDL's severity records and the
//! deck's `.ASSERT` reports, exactly as the engine published them.

use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::config::{HdlOutputRecord, HdlPoint, HdlStream};
use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};
use serde::{Deserialize, Serialize};

use crate::results::SimulationChecks;

/// What a run's checks reported. Rides the JSON envelope, as events do: a
/// few records of text and scalars, with no samples to buffer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerChecks {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    hdl: Vec<HdlOutputRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    assertions: Vec<WorkerAssertionReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerAssertionReport {
    name: String,
    analysis: WorkerAssertAnalysis,
    target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    level: WorkerAssertLevel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    /// The deck file the card was written in; absent for an in-memory deck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    file: Option<std::path::PathBuf>,
    line: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    violations: Vec<WorkerAssertViolation>,
    #[serde(default)]
    unknown_stretches: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WorkerAssertAnalysis {
    Op,
    Dc,
    Tran,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WorkerAssertLevel {
    Notice,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerAssertViolation {
    start: Option<f64>,
    end: Option<f64>,
    extreme: Option<f64>,
    at: Option<f64>,
}

impl WorkerChecks {
    pub(crate) fn is_empty(&self) -> bool {
        self.hdl.is_empty() && self.assertions.is_empty()
    }

    /// Refuse checks the JSON envelope cannot carry exactly: a non-finite
    /// number would arrive as nothing at all.
    pub(crate) fn validate(&self) -> Result<(), String> {
        for (index, record) in self.hdl.iter().enumerate() {
            let point = match record.point {
                HdlPoint::DcSweep { value } => value,
                HdlPoint::Transient { time } => time,
                HdlPoint::Initialization | HdlPoint::OperatingPoint => 0.0,
            };
            if !matches!(record.stream, HdlStream::Diagnostic { .. }) {
                return Err(format!("worker HDL check {index} is not a severity record"));
            }
            if !record.seconds.is_finite() || !point.is_finite() {
                return Err(format!("worker HDL check {index} has a non-finite time"));
            }
        }
        for (index, report) in self.assertions.iter().enumerate() {
            if report
                .violations
                .iter()
                .flat_map(|violation| {
                    [
                        violation.start,
                        violation.end,
                        violation.extreme,
                        violation.at,
                    ]
                })
                .flatten()
                .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "worker assertion report {index} ({}) has a non-finite violation",
                    report.name
                ));
            }
        }
        Ok(())
    }
}

impl From<SimulationChecks> for WorkerChecks {
    fn from(checks: SimulationChecks) -> Self {
        Self {
            hdl: checks.hdl,
            assertions: checks
                .assertions
                .into_iter()
                .map(WorkerAssertionReport::from)
                .collect(),
        }
    }
}

impl From<WorkerChecks> for SimulationChecks {
    fn from(checks: WorkerChecks) -> Self {
        Self {
            hdl: checks.hdl,
            assertions: checks
                .assertions
                .into_iter()
                .map(AssertionReport::from)
                .collect(),
        }
    }
}

impl From<AssertionReport> for WorkerAssertionReport {
    fn from(report: AssertionReport) -> Self {
        Self {
            name: report.name,
            analysis: match report.analysis {
                AssertAnalysis::Op => WorkerAssertAnalysis::Op,
                AssertAnalysis::Dc => WorkerAssertAnalysis::Dc,
                AssertAnalysis::Tran => WorkerAssertAnalysis::Tran,
            },
            target: report.target,
            instance: report.instance,
            level: match report.level {
                AssertLevel::Notice => WorkerAssertLevel::Notice,
                AssertLevel::Warning => WorkerAssertLevel::Warning,
                AssertLevel::Error => WorkerAssertLevel::Error,
            },
            message: report.message,
            file: report.origin.path,
            line: report.origin.line,
            violations: report
                .violations
                .into_iter()
                .map(|violation| WorkerAssertViolation {
                    start: violation.start,
                    end: violation.end,
                    extreme: violation.extreme,
                    at: violation.at,
                })
                .collect(),
            unknown_stretches: report.unknown_stretches,
            error: report.error,
        }
    }
}

impl From<WorkerAssertionReport> for AssertionReport {
    fn from(report: WorkerAssertionReport) -> Self {
        Self {
            name: report.name,
            analysis: match report.analysis {
                WorkerAssertAnalysis::Op => AssertAnalysis::Op,
                WorkerAssertAnalysis::Dc => AssertAnalysis::Dc,
                WorkerAssertAnalysis::Tran => AssertAnalysis::Tran,
            },
            target: report.target,
            instance: report.instance,
            level: match report.level {
                WorkerAssertLevel::Notice => AssertLevel::Notice,
                WorkerAssertLevel::Warning => AssertLevel::Warning,
                WorkerAssertLevel::Error => AssertLevel::Error,
            },
            message: report.message,
            origin: NetlistSourceLocation {
                path: report.file,
                line: report.line,
            },
            violations: report
                .violations
                .into_iter()
                .map(|violation| AssertViolation {
                    start: violation.start,
                    end: violation.end,
                    extreme: violation.extreme,
                    at: violation.at,
                })
                .collect(),
            unknown_stretches: report.unknown_stretches,
            error: report.error,
        }
    }
}
