//! Persisted run checks, introduced in result schema v45: the HDL's
//! severity-task records and the deck's `.ASSERT` reports, saved with the
//! analysis they were reported after.

use super::*;
use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};
use rspice_results::analysis_result::{HdlDiagnostic, RunChecks};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectRunChecks {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hdl: Vec<HdlDiagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assertions: Vec<ProjectAssertionReport>,
}

/// One `.ASSERT` report for one checked instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectAssertionReport {
    pub name: String,
    /// `op`, `dc` or `tran`.
    pub analysis: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// `notice`, `warning` or `error`.
    pub level: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The deck file the card was written in; absent for an in-memory deck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    pub line: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub violations: Vec<ProjectAssertViolation>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub unknown_stretches: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// One violation: absent `start` and `end` at an operating point, an absent
/// `end` for a transient violation still open at the stop time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectAssertViolation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extreme: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<f64>,
}

fn is_zero(value: &usize) -> bool {
    *value == 0
}

impl From<&RunChecks> for ProjectRunChecks {
    fn from(checks: &RunChecks) -> Self {
        Self {
            hdl: checks.hdl.clone(),
            assertions: checks
                .assertions
                .iter()
                .map(ProjectAssertionReport::from)
                .collect(),
        }
    }
}

impl From<&AssertionReport> for ProjectAssertionReport {
    fn from(report: &AssertionReport) -> Self {
        Self {
            name: report.name.clone(),
            analysis: report.analysis.to_string(),
            target: report.target.clone(),
            instance: report.instance.clone(),
            level: report.level.to_string(),
            message: report.message.clone(),
            file: report
                .origin
                .path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            line: report.origin.line,
            violations: report
                .violations
                .iter()
                .map(|violation| ProjectAssertViolation {
                    start: violation.start,
                    end: violation.end,
                    extreme: violation.extreme,
                    at: violation.at,
                })
                .collect(),
            unknown_stretches: report.unknown_stretches,
            error: report.error.clone(),
        }
    }
}

impl ProjectRunChecks {
    pub(super) fn into_checks(self) -> Result<RunChecks, String> {
        Ok(RunChecks {
            hdl: self.hdl,
            assertions: self
                .assertions
                .into_iter()
                .map(ProjectAssertionReport::into_report)
                .collect::<Result<_, _>>()?,
        })
    }

    pub(super) fn validate(&self, prefix: &str) -> Result<(), String> {
        if self.hdl.is_empty() && self.assertions.is_empty() {
            return Err(format!(
                "{prefix} is empty; a result without checks omits the field"
            ));
        }
        for (index, record) in self.hdl.iter().enumerate() {
            let at = format!("{prefix}.hdl[{index}]");
            match record.point {
                rspice_core::config::HdlPoint::DcSweep { value } => {
                    require_finite(value, &format!("{at}.point"))?;
                }
                rspice_core::config::HdlPoint::Transient { time } => {
                    require_finite(time, &format!("{at}.point"))?;
                }
                rspice_core::config::HdlPoint::Initialization
                | rspice_core::config::HdlPoint::OperatingPoint => {}
            }
        }
        for (index, report) in self.assertions.iter().enumerate() {
            let at = format!("{prefix}.assertions[{index}]");
            analysis(&report.analysis).map_err(|error| format!("{at}: {error}"))?;
            level(&report.level).map_err(|error| format!("{at}: {error}"))?;
            for (number, violation) in report.violations.iter().enumerate() {
                let field = |name: &str| format!("{at}.violations[{number}].{name}");
                require_optional_finite(violation.start, &field("start"))?;
                require_optional_finite(violation.end, &field("end"))?;
                require_optional_finite(violation.extreme, &field("extreme"))?;
                require_optional_finite(violation.at, &field("at"))?;
            }
        }
        Ok(())
    }
}

impl ProjectAssertionReport {
    fn into_report(self) -> Result<AssertionReport, String> {
        Ok(AssertionReport {
            analysis: analysis(&self.analysis)?,
            level: level(&self.level)?,
            name: self.name,
            target: self.target,
            instance: self.instance,
            message: self.message,
            origin: NetlistSourceLocation {
                path: self.file.map(std::path::PathBuf::from),
                line: self.line,
            },
            violations: self
                .violations
                .into_iter()
                .map(|violation| AssertViolation {
                    start: violation.start,
                    end: violation.end,
                    extreme: violation.extreme,
                    at: violation.at,
                })
                .collect(),
            unknown_stretches: self.unknown_stretches,
            error: self.error,
        })
    }
}

fn analysis(text: &str) -> Result<AssertAnalysis, String> {
    match text {
        "op" => Ok(AssertAnalysis::Op),
        "dc" => Ok(AssertAnalysis::Dc),
        "tran" => Ok(AssertAnalysis::Tran),
        other => Err(format!("unknown assertion analysis '{other}'")),
    }
}

fn level(text: &str) -> Result<AssertLevel, String> {
    match text {
        "notice" => Ok(AssertLevel::Notice),
        "warning" => Ok(AssertLevel::Warning),
        "error" => Ok(AssertLevel::Error),
        other => Err(format!("unknown assertion level '{other}'")),
    }
}
