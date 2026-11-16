//! The checks table, held to the grade the run's own checks give it.

use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::config::{HdlOrigin, HdlPoint, HdlSeverity};
use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};
use rspice_core::{ModelFinish, ModelFinishPoint, ModelFinishTask};

use super::*;
use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::{CheckVerdict, HdlDiagnostic, RunChecks};
use crate::safety::SoaThresholds;
use crate::soa_evidence::{SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence};
use crate::verification::{AssertionOutcome, Check, CheckStatus, Checks};

/// The rows from each analysis's own checks grade it as
/// [`RunChecks::verdict`] does, so the table cannot call clean a run its
/// checks failed.
fn assert_checks_agree(checks: &Checks, run: &SimulationRun) {
    for (position, analysis) in run.analyses.iter().enumerate() {
        let worst = checks
            .rows
            .iter()
            .filter(|row| row.analysis == position && !matches!(row.check, Check::Soa(_)))
            .map(|row| row.status)
            .max()
            .unwrap_or(CheckStatus::Pass);
        let graded = match worst {
            CheckStatus::Pass | CheckStatus::Info => CheckVerdict::Passed,
            CheckStatus::Warning => CheckVerdict::Warned,
            CheckStatus::Error | CheckStatus::Fail => CheckVerdict::Failed,
        };
        assert_eq!(graded, analysis.checks.verdict(), "analysis {position}");
    }
}

fn checked(point: Option<AnalysisResultPvtPoint>, checks: RunChecks) -> AnalysisResult {
    let mut analysis = measured(AnalysisType::Transient, point, &[]);
    analysis.checks = checks;
    analysis
}

fn report(level: AssertLevel, violations: Vec<AssertViolation>) -> AssertionReport {
    AssertionReport {
        name: "out_band".into(),
        analysis: AssertAnalysis::Tran,
        target: "V(out)".into(),
        instance: None,
        level,
        message: Some("out left its band".into()),
        origin: NetlistSourceLocation::in_memory(4),
        violations,
        unknown_stretches: 0,
        error: None,
        after_stop: None,
    }
}

fn violation(start: f64, end: Option<f64>) -> AssertViolation {
    AssertViolation {
        start: Some(start),
        end,
        extreme: Some(1.9),
        at: Some(start),
    }
}

fn record(severity: HdlSeverity, text: &str) -> HdlDiagnostic {
    HdlDiagnostic {
        severity,
        text: text.into(),
        point: HdlPoint::Transient { time: 2e-9 },
        origin: HdlOrigin {
            scope: "xtb".into(),
            file: "tb.v".into(),
            line: 12,
        },
    }
}

fn finish(task: ModelFinishTask) -> ModelFinish {
    ModelFinish {
        instance: "xtb".into(),
        model: "tb".into(),
        site: 1,
        point: ModelFinishPoint::Transient { time: 5e-9 },
        diagnostic_level: 1,
        task,
        source: None,
        message: (task == ModelFinishTask::Fatal).then(|| "supply collapsed".into()),
    }
}

fn soa_rule(
    device: &str,
    parameter: SoaParameterEvidence,
    worst: f64,
    limit: f64,
    verdict: SoaRuleVerdictEvidence,
) -> SoaEvaluationEvidence {
    SoaEvaluationEvidence {
        envelope: None,
        duration: None,
        thresholds: SoaThresholds::default(),
        derating: None,
        device_id: device.into(),
        parameter,
        limit_value: limit,
        worst_actual_value: worst,
        worst_time_s: 1e-6,
        sample_count: 100,
        unit: "V".into(),
        description: format!("{device} rule"),
        verdict,
    }
}

#[test]
fn an_soa_rule_is_a_check_at_its_point() {
    let soa = AnalysisResult::new(1, AnalysisType::Soa, "Soa", 0.0)
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![
                soa_rule(
                    "M1",
                    SoaParameterEvidence::GateSourceVoltage,
                    3.0,
                    5.0,
                    SoaRuleVerdictEvidence::Pass,
                ),
                soa_rule(
                    "M2",
                    SoaParameterEvidence::DrainSourceVoltage,
                    11.0,
                    10.0,
                    SoaRuleVerdictEvidence::Violation,
                ),
            ],
            violations: Vec::new(),
        })
        .with_provenance(provenance(Some(at("SS", 1.62, 125.0, false))));
    let run = history(vec![soa]);
    let checks = Checks::of(&run);
    assert_checks_agree(&checks, &run);

    let statuses = checks.rows.iter().map(|row| row.status).collect::<Vec<_>>();
    assert_eq!(statuses, [CheckStatus::Pass, CheckStatus::Fail]);
    let Check::Soa(broken) = &checks.rows[1].check else {
        panic!("an SOA row");
    };
    assert_eq!(broken.device, "M2");
    assert!((broken.margin.expect("a nonzero limit") + 0.1).abs() < 1e-12);
    assert_eq!(checks.rows[1].point, Some(0));
    assert_eq!(checks.points[0].process.as_deref(), Some("SS"));
}

#[test]
fn two_assert_violations_give_two_rows() {
    let run = history(vec![checked(
        None,
        RunChecks {
            assertions: vec![report(
                AssertLevel::Error,
                vec![violation(1e-9, Some(3e-9)), violation(5e-9, None)],
            )],
            ..RunChecks::default()
        },
    )]);
    let checks = Checks::of(&run);
    assert_checks_agree(&checks, &run);

    assert_eq!(checks.rows.len(), 2);
    let outcomes = checks
        .rows
        .iter()
        .map(|row| match &row.check {
            Check::Assertion(check) => (row.status, check.outcome.clone()),
            other => panic!("an assertion row, not {other:?}"),
        })
        .collect::<Vec<_>>();
    let AssertionOutcome::Violated { duration, .. } = &outcomes[0].1 else {
        panic!("a violation");
    };
    assert!((duration.expect("a closed stretch") - 2e-9).abs() < 1e-21);
    assert_eq!(outcomes[0].0, CheckStatus::Fail);
    assert_eq!(
        outcomes[1],
        (
            CheckStatus::Fail,
            AssertionOutcome::Violated {
                start: Some(5e-9),
                end: None,
                duration: None,
                extreme: Some(1.9),
                at: Some(5e-9),
            }
        )
    );
    assert_eq!(checks.points[0].label, "Deck");
}

#[test]
fn a_passing_assert_is_one_passing_row() {
    let run = history(vec![checked(
        Some(at("TT", 1.8, 27.0, true)),
        RunChecks {
            assertions: vec![report(AssertLevel::Error, Vec::new())],
            ..RunChecks::default()
        },
    )]);
    let checks = Checks::of(&run);
    assert_checks_agree(&checks, &run);

    assert_eq!(checks.rows.len(), 1);
    assert_eq!(checks.rows[0].status, CheckStatus::Pass);
    assert!(matches!(
        &checks.rows[0].check,
        Check::Assertion(check) if check.outcome == AssertionOutcome::Held
    ));
}

/// A `$warning` is a warning; an `$info` is the run's log, not a check.
#[test]
fn an_hdl_warning_is_a_warning_row() {
    let run = history(vec![checked(
        None,
        RunChecks {
            hdl: vec![
                record(HdlSeverity::Info, "calibrating"),
                record(HdlSeverity::Warning, "trim code saturated"),
            ],
            ..RunChecks::default()
        },
    )]);
    let checks = Checks::of(&run);
    assert_checks_agree(&checks, &run);

    assert_eq!(checks.rows.len(), 1);
    assert_eq!(checks.rows[0].status, CheckStatus::Warning);
    assert!(matches!(
        &checks.rows[0].check,
        Check::Hdl { text, severity: HdlSeverity::Warning, .. } if text == "trim code saturated"
    ));
}

/// A `$fatal` that ended the run fails it; a `$finish` ends a run cleanly.
#[test]
fn a_fatal_finish_fails_the_run() {
    let run = history(vec![
        checked(
            None,
            RunChecks {
                hdl: vec![record(HdlSeverity::Fatal, "supply collapsed")],
                finish: Some(finish(ModelFinishTask::Fatal)),
                ..RunChecks::default()
            },
        ),
        checked(
            Some(at("TT", 1.8, 27.0, true)),
            RunChecks {
                finish: Some(finish(ModelFinishTask::Finish)),
                ..RunChecks::default()
            },
        ),
    ]);
    let checks = Checks::of(&run);
    assert_checks_agree(&checks, &run);

    let rows = checks
        .rows
        .iter()
        .map(|row| {
            (
                row.analysis,
                row.status,
                matches!(row.check, Check::Finish(_)),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        rows,
        [(0, CheckStatus::Fail, false), (0, CheckStatus::Fail, true)]
    );
    assert_eq!(checks.worst(), CheckStatus::Fail);
}

/// Worst first, and rows of one status keep their run order.
#[test]
fn checks_sort_worst_first() {
    let mut unevaluated = report(AssertLevel::Error, Vec::new());
    unevaluated.error = Some("V(nope) names no node".into());
    let run = history(vec![
        checked(
            None,
            RunChecks {
                hdl: vec![record(HdlSeverity::Warning, "first")],
                assertions: vec![
                    report(AssertLevel::Notice, vec![violation(1e-9, None)]),
                    report(AssertLevel::Error, Vec::new()),
                ],
                ..RunChecks::default()
            },
        ),
        checked(
            Some(at("TT", 1.8, 27.0, true)),
            RunChecks {
                hdl: vec![
                    record(HdlSeverity::Warning, "second"),
                    record(HdlSeverity::Error, "out of range"),
                ],
                assertions: vec![unevaluated],
                ..RunChecks::default()
            },
        ),
    ]);
    let mut checks = Checks::of(&run);
    assert_checks_agree(&checks, &run);
    checks.sort_worst_first();

    let order = checks
        .rows
        .iter()
        .map(|row| (row.status, row.analysis))
        .collect::<Vec<_>>();
    assert_eq!(
        order,
        [
            (CheckStatus::Fail, 1),
            (CheckStatus::Error, 1),
            (CheckStatus::Warning, 0),
            (CheckStatus::Warning, 1),
            (CheckStatus::Info, 0),
            (CheckStatus::Pass, 0),
        ]
    );
    let warnings = checks
        .rows
        .iter()
        .filter_map(|row| match &row.check {
            Check::Hdl { text, .. } if row.status == CheckStatus::Warning => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(warnings, ["first", "second"]);
    assert_eq!(checks.worst(), CheckStatus::Fail);
}
