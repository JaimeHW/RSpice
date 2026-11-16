//! Every check a run evaluated, one row each, at the point it ran.
//!
//! Three kinds of check report beside a run's results: the safe-operating-area
//! rules, the deck's `.ASSERT` cards and the HDL's severity tasks. Each row
//! restates its own verdict — the SOA rule's, or the grade
//! [`RunChecks::verdict`](crate::analysis_result::RunChecks::verdict) gives
//! the record — so this table cannot call a run clean that its checks failed.

use rspice_core::config::{HdlOrigin, HdlPoint, HdlSeverity};
use rspice_core::netlist::{AssertAnalysis, AssertLevel};

use super::points::{Point, RunPoints};
use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::{AnalysisResult, HdlDiagnostic, samples_after_stop};
use crate::run::SimulationRun;
use crate::safety::{SoaDurationEvidence, SoaPowerDeratingEvidence};
use crate::soa_evidence::{SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence};
use crate::waveform::RetainedWaveform;

/// What one check says of the run, from best to worst.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckStatus {
    Pass,
    /// Reported for information only.
    Info,
    Warning,
    /// The check could not be evaluated.
    Error,
    Fail,
}

impl CheckStatus {
    /// Whether the check was violated: it failed, or it could not be
    /// evaluated, so the run cannot be called clean over it. A warning is not
    /// a violation.
    #[must_use]
    pub fn is_violation(self) -> bool {
        matches!(self, Self::Error | Self::Fail)
    }
}

/// How many of a run's checks were violated, how many only warned, and how
/// many there are.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CheckCounts {
    /// Checks that failed or could not be evaluated.
    pub violations: usize,
    /// Checks that warned.
    pub warnings: usize,
    /// Every check, whatever it says.
    pub total: usize,
}

/// One safe-operating-area rule, evaluated over a whole analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaCheck {
    pub device: String,
    pub parameter: SoaParameterEvidence,
    /// The worst value the device reached.
    pub observed: f64,
    pub limit: f64,
    pub unit: String,
    /// Room before the limit, as a fraction of it; negative once broken.
    /// `None` for a limit of zero.
    pub margin: Option<f64>,
    /// When the worst value was reached, in seconds.
    pub worst_time_s: f64,
    pub duration: Option<SoaDurationEvidence>,
    pub derating: Option<SoaPowerDeratingEvidence>,
    pub verdict: SoaRuleVerdictEvidence,
    pub description: String,
}

/// What an `.ASSERT` card found.
#[derive(Debug, Clone, PartialEq)]
pub enum AssertionOutcome {
    /// The check held everywhere it was evaluated.
    Held,
    /// One stretch on which the check was false.
    Violated {
        /// Where it starts: a time, or a swept value; `None` at an operating
        /// point.
        start: Option<f64>,
        /// Where it ends; `None` when it was still open at the stop time.
        end: Option<f64>,
        /// How long it lasted, for a transient stretch that closed.
        duration: Option<f64>,
        /// The checked value farthest outside its band.
        extreme: Option<f64>,
        /// Where that value was reached.
        at: Option<f64>,
    },
    /// Why the card could not be evaluated.
    NotEvaluated(String),
    /// Expected outputs a vector file states for after the stop time, which
    /// the run could not check.
    SamplesAfterStop { count: usize, first_time: f64 },
}

/// One `.ASSERT` card's finding, for one checked instance.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionCheck {
    pub name: String,
    pub analysis: AssertAnalysis,
    pub target: String,
    pub instance: Option<String>,
    pub level: AssertLevel,
    pub message: Option<String>,
    pub outcome: AssertionOutcome,
}

/// What a check row reports.
#[derive(Debug, Clone, PartialEq)]
pub enum Check {
    Soa(SoaCheck),
    Assertion(AssertionCheck),
    /// A `$warning`, `$error` or `$fatal` record.
    Hdl {
        severity: HdlSeverity,
        text: String,
        at: HdlPoint,
        origin: HdlOrigin,
    },
    /// The `$fatal` that ended the run.
    Finish(rspice_core::ModelFinish),
}

/// One check, at the point its analysis ran.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckRow {
    pub status: CheckStatus,
    /// The point the analysis ran at, in [`Checks::points`].
    pub point: Option<usize>,
    /// The position in the run of the analysis it came from.
    pub analysis: usize,
    pub check: Check,
}

/// Every check one run evaluated.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Checks {
    /// The points the run's analyses ran at, named as a sheet names them.
    pub points: Vec<Point>,
    /// In run order, each analysis's SOA rules, then its `.ASSERT` cards,
    /// then its HDL records.
    pub rows: Vec<CheckRow>,
}

impl Checks {
    /// Read every check one run evaluated.
    #[must_use]
    pub fn of<A, W>(run: &SimulationRun<A>) -> Self
    where
        A: AsRef<AnalysisResult<W>>,
        W: AsRef<RetainedWaveform>,
    {
        let analyses = run.analyses.iter().map(AsRef::as_ref).collect::<Vec<_>>();
        let points = RunPoints::collect(analyses.iter().copied(), |_| true);
        let mut rows = Vec::new();
        for (position, analysis) in analyses.iter().enumerate() {
            let point = points.of_analysis(position);
            let mut push = |status, check| {
                rows.push(CheckRow {
                    status,
                    point,
                    analysis: position,
                    check,
                });
            };
            if let Some(AnalysisResultPayload::Soa { evaluations, .. }) = &analysis.result_payload {
                for evaluation in evaluations {
                    push(
                        soa_status(evaluation.verdict),
                        Check::Soa(soa_check(evaluation)),
                    );
                }
            }
            for report in &analysis.checks.assertions {
                for (status, check) in assertion_rows(report) {
                    push(status, Check::Assertion(check));
                }
            }
            for record in &analysis.checks.hdl {
                if let Some(status) = hdl_status(record.severity) {
                    push(status, hdl_check(record));
                }
            }
            if let Some(finish) = analysis
                .checks
                .finish
                .as_ref()
                .filter(|finish| finish.failed())
            {
                push(CheckStatus::Fail, Check::Finish(finish.clone()));
            }
        }
        Self {
            points: points.points,
            rows,
        }
    }

    /// The rows worst first; rows of one status keep their run order.
    pub fn sort_worst_first(&mut self) {
        self.rows
            .sort_by(|left, right| right.status.cmp(&left.status));
    }

    /// The worst status of any row, `Pass` for a run with none.
    #[must_use]
    pub fn worst(&self) -> CheckStatus {
        self.rows
            .iter()
            .map(|row| row.status)
            .max()
            .unwrap_or(CheckStatus::Pass)
    }

    /// How many rows were violated and how many only warned, out of all of
    /// them: the one count every view of a run's checks states.
    #[must_use]
    pub fn counts(&self) -> CheckCounts {
        let mut counts = CheckCounts {
            total: self.rows.len(),
            ..CheckCounts::default()
        };
        for row in &self.rows {
            if row.status.is_violation() {
                counts.violations += 1;
            } else if row.status == CheckStatus::Warning {
                counts.warnings += 1;
            }
        }
        counts
    }
}

fn soa_status(verdict: SoaRuleVerdictEvidence) -> CheckStatus {
    match verdict {
        SoaRuleVerdictEvidence::Pass => CheckStatus::Pass,
        SoaRuleVerdictEvidence::Warning => CheckStatus::Warning,
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => CheckStatus::Fail,
    }
}

fn soa_check(evaluation: &SoaEvaluationEvidence) -> SoaCheck {
    SoaCheck {
        device: evaluation.device_id.clone(),
        parameter: evaluation.parameter,
        observed: evaluation.worst_actual_value,
        limit: evaluation.limit_value,
        unit: evaluation.unit.clone(),
        margin: (evaluation.limit_value != 0.0).then(|| {
            (evaluation.limit_value - evaluation.worst_actual_value) / evaluation.limit_value.abs()
        }),
        worst_time_s: evaluation.worst_time_s,
        duration: evaluation.duration,
        derating: evaluation.derating,
        verdict: evaluation.verdict,
        description: evaluation.description.clone(),
    }
}

/// One `.ASSERT` report's rows: one per violation at its level, or one row for
/// a report that held, could not be evaluated, or left samples unchecked.
fn assertion_rows(
    report: &rspice_core::analysis::AssertionReport,
) -> Vec<(CheckStatus, AssertionCheck)> {
    let check = |outcome| AssertionCheck {
        name: report.name.clone(),
        analysis: report.analysis,
        target: report.target.clone(),
        instance: report.instance.clone(),
        level: report.level,
        message: report.message.clone(),
        outcome,
    };
    if let Some(error) = &report.error {
        return vec![(
            CheckStatus::Error,
            check(AssertionOutcome::NotEvaluated(error.clone())),
        )];
    }
    if let (Some(count), Some(late)) = (samples_after_stop(report), report.after_stop) {
        return vec![(
            CheckStatus::Warning,
            check(AssertionOutcome::SamplesAfterStop {
                count,
                first_time: late.first_time,
            }),
        )];
    }
    if report.violations.is_empty() {
        return vec![(CheckStatus::Pass, check(AssertionOutcome::Held))];
    }
    let status = match report.level {
        AssertLevel::Notice => CheckStatus::Info,
        AssertLevel::Warning => CheckStatus::Warning,
        AssertLevel::Error => CheckStatus::Fail,
    };
    report
        .violations
        .iter()
        .map(|violation| {
            let duration = match (report.analysis, violation.start, violation.end) {
                (AssertAnalysis::Tran, Some(start), Some(end)) => Some(end - start),
                _ => None,
            };
            (
                status,
                check(AssertionOutcome::Violated {
                    start: violation.start,
                    end: violation.end,
                    duration,
                    extreme: violation.extreme,
                    at: violation.at,
                }),
            )
        })
        .collect()
}

/// An HDL record's status; `$info` is not a check.
fn hdl_status(severity: HdlSeverity) -> Option<CheckStatus> {
    match severity {
        HdlSeverity::Info => None,
        HdlSeverity::Warning => Some(CheckStatus::Warning),
        HdlSeverity::Error | HdlSeverity::Fatal => Some(CheckStatus::Fail),
    }
}

fn hdl_check(record: &HdlDiagnostic) -> Check {
    Check::Hdl {
        severity: record.severity,
        text: record.text.clone(),
        at: record.point,
        origin: record.origin.clone(),
    }
}
