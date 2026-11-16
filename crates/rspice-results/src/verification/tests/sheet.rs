//! Rows, cells and points: each test checks the rows against the judge.

use super::*;
use crate::provenance::{AnalysisResultSweptAxis, AnalysisResultSweptValue};
use crate::specification::SpecificationWaiver;
use crate::verification::{Acceptance, CellStatus, LimitsSinceRun, PointAxis, StatusCounts};

fn cell_statuses(sheet: &Sheet, row: usize) -> Vec<CellStatus> {
    sheet.rows[row]
        .cells
        .iter()
        .map(|cell| cell.status)
        .collect()
}

/// The fixture the corner views once gave four different answers for: the
/// sheet's rows are the judge's, and the near ones are near.
#[test]
fn the_corner_fixture_reads_as_the_judge_reads_it() {
    let specifications = corner_limits();
    let policy = SpecificationPolicy::default();
    let run = history(vec![corners()]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    let statuses = sheet.rows.iter().map(|row| row.status).collect::<Vec<_>>();
    assert_eq!(
        statuses,
        [
            Some(RowStatus::Fail),
            Some(RowStatus::Pass),
            Some(RowStatus::Near),
            Some(RowStatus::Near),
        ]
    );
    // One point per corner, headed by the coordinates that vary.
    assert_eq!(sheet.points.len(), CORNERS.len());
    assert_eq!(
        sheet.axes,
        [
            PointAxis::Process,
            PointAxis::Supply,
            PointAxis::Temperature
        ]
    );
    assert_eq!(sheet.points[1].label, "SS 1.62 V 125 \u{00b0}C");
    assert_eq!(sheet.points[2].label, "SS 1.62 V \u{2212}40 \u{00b0}C");
    assert_eq!(sheet.points[1].process.as_deref(), Some("SS"));
    // Gain fails at SS hot and nowhere else; that is its worst point.
    use CellStatus::{Near, Pass};
    assert_eq!(
        cell_statuses(&sheet, 0),
        [Pass, CellStatus::Fail, Pass, Pass, Pass, Pass, Pass]
    );
    assert_eq!(
        sheet.rows[0].worst.as_ref().and_then(|worst| worst.point),
        Some(1)
    );
    assert_eq!(
        cell_statuses(&sheet, 2),
        [Pass, Pass, Pass, Near, Pass, Pass, Pass]
    );
    assert_eq!(
        cell_statuses(&sheet, 3),
        [Pass, Pass, Pass, Near, Pass, Pass, Pass]
    );
    assert_eq!(
        sheet.summary.counts,
        StatusCounts {
            pass: 1,
            near: 2,
            fail: 1,
            ..StatusCounts::default()
        }
    );
    assert_eq!(
        sheet.summary.acceptance,
        Some(Acceptance {
            blocked: true,
            blocking: vec![0],
        })
    );
    assert_eq!(sheet.summary.limits, LimitsSinceRun::NotSealed);
}

#[test]
fn the_nominal_point_heads_the_sheet() {
    let specifications = vec![limit("gain", Some(40.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let run = history(vec![
        measured(
            AnalysisType::Ac,
            Some(at("SS", 1.62, 125.0, false)),
            &[("gain", 39.0)],
        ),
        measured(
            AnalysisType::Ac,
            Some(at("TT", 1.8, 27.0, true)),
            &[("gain", 44.0)],
        ),
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert!(sheet.points[0].nominal);
    assert_eq!(sheet.points[0].process.as_deref(), Some("TT"));
    let row = &sheet.rows[0];
    assert_eq!(row.nominal_value, Some(44.0));
    assert_eq!(
        cell_statuses(&sheet, 0),
        [CellStatus::Pass, CellStatus::Fail]
    );
    assert_eq!(row.worst.as_ref().and_then(|worst| worst.point), Some(1));
}

/// An AC and a transient solved at one point are one column, and the cell
/// states the worse of their values.
#[test]
fn analyses_solved_at_one_point_share_its_column() {
    let specifications = vec![limit("gain", Some(40.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let typical = at("TT", 1.8, 27.0, true);
    let run = history(vec![
        measured(AnalysisType::Ac, Some(typical.clone()), &[("gain", 44.0)]),
        measured(
            AnalysisType::Transient,
            Some(typical),
            &[("gain", 43.0), ("slew", 5.0)],
        ),
        measured(
            AnalysisType::Ac,
            Some(at("SS", 1.62, 125.0, false)),
            &[("gain", 45.0)],
        ),
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert_eq!(sheet.points.len(), 2);
    let cell = &sheet.rows[0].cells[0];
    assert_eq!(cell.evidence_count, 2);
    assert_eq!(cell.value, Some(43.0));
    assert_eq!(sheet.rows[0].cells[1].evidence_count, 1);
}

/// A run set that swept a design variable gives one column per value,
/// though every value ran at the same corner.
#[test]
fn a_run_set_sweep_gives_a_point_per_value() {
    let swept = |value: &str| {
        at("TT", 1.8, 27.0, false)
            .with_swept_values(vec![AnalysisResultSweptValue {
                axis: AnalysisResultSweptAxis::Parameter,
                name: "CLOAD".to_owned(),
                value: value.to_owned(),
            }])
            .expect("a valid swept value")
    };
    let specifications = vec![limit("pm", Some(45.0), None, "deg")];
    let policy = SpecificationPolicy::default();
    let run = history(vec![
        measured(AnalysisType::Ac, Some(swept("2p")), &[("pm", 61.0)]),
        measured(AnalysisType::Ac, Some(swept("20p")), &[("pm", 44.0)]),
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert_eq!(
        sheet.axes,
        [PointAxis::Swept {
            axis: AnalysisResultSweptAxis::Parameter,
            name: "CLOAD".to_owned(),
        }]
    );
    let labels = sheet
        .points
        .iter()
        .map(|point| point.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(labels, ["CLOAD = 2p", "CLOAD = 20p"]);
    assert_eq!(
        cell_statuses(&sheet, 0),
        [CellStatus::Pass, CellStatus::Fail]
    );
}

/// A sweep solved inside one analysis gives its members as points: a
/// temperature sweep, a temperature each.
#[test]
fn a_sweep_family_gives_its_members_as_points() {
    let members = [(0, 25.0, 12.0), (1, 75.0, 7.25), (2, 125.0, 11.0)]
        .into_iter()
        .map(|(index, celsius, gain)| {
            member(
                FamilyMemberId::SweepPoint {
                    index,
                    value: celsius,
                },
                &[("gain", Some(gain))],
            )
        })
        .collect();
    let sweep = AnalysisResult::new(1, AnalysisType::Parametric, "Parametric", 0.0)
        .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
            target: "TEMP".to_owned(),
            sweep_values: vec![25.0, 75.0, 125.0],
            failed_points: 0,
            member_measurements: members,
        })
        .with_provenance(provenance(None));
    let specifications = vec![limit("gain", Some(10.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let run = history(vec![sweep]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert_eq!(sheet.axes, [PointAxis::Temperature]);
    let temperatures = sheet
        .points
        .iter()
        .map(|point| point.temperature_celsius)
        .collect::<Vec<_>>();
    assert_eq!(temperatures, [Some(25.0), Some(75.0), Some(125.0)]);
    assert_eq!(sheet.points[1].label, "75 \u{00b0}C");
    assert_eq!(
        cell_statuses(&sheet, 0),
        [CellStatus::Pass, CellStatus::Fail, CellStatus::Pass]
    );
    assert_eq!(
        sheet.rows[0].worst.as_ref().and_then(|worst| worst.point),
        Some(1)
    );
}

/// A point the specification's scope leaves out is excluded from it, and a
/// value there that would break the limit fails nothing.
#[test]
fn a_point_outside_the_scope_is_excluded_never_failed() {
    let mut nominal_only = entry("gain", Some(40.0), None, "dB");
    nominal_only.scope = SpecPointScope::Nominal;
    let specifications = vec![PreparedSpecification::new(nominal_only).expect("a valid limit")];
    let policy = SpecificationPolicy::default();
    let run = history(vec![
        measured(
            AnalysisType::Ac,
            Some(at("TT", 1.8, 27.0, true)),
            &[("gain", 44.0)],
        ),
        measured(
            AnalysisType::Ac,
            Some(at("SS", 1.62, 125.0, false)),
            &[("gain", 30.0)],
        ),
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert_eq!(
        cell_statuses(&sheet, 0),
        [CellStatus::Pass, CellStatus::Excluded]
    );
    assert_eq!(sheet.rows[0].status, Some(RowStatus::Pass));
}

#[test]
fn a_guard_band_turns_a_narrow_pass_into_a_fail() {
    let gain = entry("gain", Some(40.0), None, "dB");
    let specifications = vec![
        limit("gain", Some(40.0), None, "dB"),
        governed(&gain, |definition| definition.guard_band = Some(1.0)),
    ];
    let policy = SpecificationPolicy::default();
    let run = history(vec![measured(
        AnalysisType::Ac,
        Some(at("TT", 1.8, 27.0, true)),
        &[("gain", 40.5)],
    )]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert_eq!(sheet.rows[0].status, Some(RowStatus::Near));
    assert_eq!(sheet.rows[1].status, Some(RowStatus::Fail));
    assert_eq!(sheet.rows[1].guard_band, 1.0);
    assert_eq!(cell_statuses(&sheet, 1), [CellStatus::Fail]);
    assert_eq!(
        sheet
            .summary
            .acceptance
            .as_ref()
            .map(|acceptance| acceptance.blocking.clone()),
        Some(vec![1])
    );
}

#[test]
fn a_failed_measurement_is_an_error() {
    let specifications = vec![limit("gain", Some(40.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let failed = AnalysisResult::new(1, AnalysisType::Ac, "Ac", 0.0)
        .with_measurements(vec![rspice_core::MeasureResult::failed(
            "gain",
            "no unity-gain crossing",
        )])
        .with_provenance(provenance(Some(at("SS", 1.62, 125.0, false))));
    let run = history(vec![
        measured(
            AnalysisType::Ac,
            Some(at("TT", 1.8, 27.0, true)),
            &[("gain", 44.0)],
        ),
        failed,
    ]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert_eq!(
        cell_statuses(&sheet, 0),
        [CellStatus::Pass, CellStatus::Error]
    );
    assert_eq!(sheet.rows[0].status, Some(RowStatus::Error));
}

/// A waived failure is stated as waived and does not block; another row's
/// failure still does, and the summary names that row.
#[test]
fn a_waived_failure_does_not_block_acceptance() {
    let gain = entry("gain", Some(40.0), None, "dB");
    let specifications = vec![
        governed(&gain, |definition| {
            definition.waiver = Some(SpecificationWaiver {
                reference: "WVR-7".to_owned(),
                owner: "Analog lead".to_owned(),
                rationale: "Accepted at the design review".to_owned(),
            });
        }),
        limit("pm", Some(45.0), None, "deg"),
    ];
    let policy = SpecificationPolicy::default();
    let run = history(vec![measured(
        AnalysisType::Ac,
        Some(at("TT", 1.8, 27.0, true)),
        &[("gain", 38.0), ("pm", 60.0)],
    )]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);
    assert_eq!(sheet.rows[0].status, Some(RowStatus::Waived));
    assert_eq!(
        sheet.summary.acceptance,
        Some(Acceptance {
            blocked: false,
            blocking: Vec::new(),
        })
    );

    let failing = history(vec![measured(
        AnalysisType::Ac,
        Some(at("TT", 1.8, 27.0, true)),
        &[("gain", 38.0), ("pm", 40.0)],
    )]);
    let sheet = build(&specifications, &policy, &failing, None);
    assert_rows_agree(&sheet, &specifications, &policy, &failing);
    assert_eq!(
        sheet.summary.acceptance,
        Some(Acceptance {
            blocked: true,
            blocking: vec![1],
        })
    );
    assert_eq!(sheet.summary.counts.waived, 1);
}

/// A requirement with no limit records its values and is not judged.
#[test]
fn a_tracked_requirement_has_no_verdict() {
    let specifications = vec![governed(&entry("idd", None, None, "A"), |_| {})];
    let policy = SpecificationPolicy::default();
    let run = history(vec![measured(
        AnalysisType::DcOp,
        Some(at("TT", 1.8, 27.0, true)),
        &[("idd", 0.8e-3)],
    )]);
    let sheet = build(&specifications, &policy, &run, None);
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    let row = &sheet.rows[0];
    assert_eq!(row.status, Some(RowStatus::Tracked));
    assert_eq!(row.cells[0].value, Some(0.8e-3));
    assert_eq!(sheet.summary.counts.tracked, 1);
    assert_eq!(sheet.summary.counts.judged(), 0);
    assert_eq!(
        sheet
            .summary
            .acceptance
            .as_ref()
            .map(|acceptance| acceptance.blocked),
        Some(false)
    );
}

/// Another plan's run shows its values and no verdict: its limits are not
/// this plan's to judge it by.
#[test]
fn a_foreign_plans_run_gives_values_and_no_verdict() {
    let specifications = vec![limit("gain", Some(40.0), None, "dB")];
    let policy = SpecificationPolicy::default();
    let run = sealed(
        SimulationPlanId::new(),
        &specifications,
        &policy,
        vec![measured(
            AnalysisType::Ac,
            Some(at("TT", 1.8, 27.0, true)),
            &[("gain", 39.0)],
        )],
    );
    let sheet = build(
        &specifications,
        &policy,
        &run,
        Some(SimulationPlanId::new()),
    );
    assert_rows_agree(&sheet, &specifications, &policy, &run);

    assert!(!sheet.is_judged());
    assert!(sheet.refusal().is_some());
    let row = &sheet.rows[0];
    assert_eq!(row.cells[0].value, Some(39.0));
    assert_eq!(row.cells[0].status, CellStatus::Pass, "no limit judges it");
    assert_eq!(sheet.summary.counts, StatusCounts::default());
    assert_eq!(sheet.summary.limits, LimitsSinceRun::NotSealed);
}

#[test]
fn limits_edited_since_the_run_are_detected() {
    let plan = SimulationPlanId::new();
    let gain = entry("gain", Some(40.0), None, "dB");
    let specifications = vec![governed(&gain, |_| {})];
    let policy = SpecificationPolicy::default();
    let run = sealed(
        plan,
        &specifications,
        &policy,
        vec![measured(
            AnalysisType::Ac,
            Some(at("TT", 1.8, 27.0, true)),
            &[("gain", 44.0)],
        )],
    );
    let sheet = build(&specifications, &policy, &run, Some(plan));
    assert_rows_agree(&sheet, &specifications, &policy, &run);
    assert_eq!(sheet.summary.limits, LimitsSinceRun::Unchanged);

    let mut raised = specifications[0]
        .definition()
        .expect("a governed requirement")
        .clone();
    raised.comparison = SpecificationComparison::Minimum { limit: 45.0 };
    let edited = vec![PreparedSpecification::from_definition(raised).expect("a valid requirement")];
    let sheet = build(&edited, &policy, &run, Some(plan));
    assert_rows_agree(&sheet, &edited, &policy, &run);
    assert_eq!(sheet.summary.limits, LimitsSinceRun::Edited);
    assert_eq!(
        sheet.rows[0].status,
        Some(RowStatus::Fail),
        "judged by the plan's limit"
    );

    let looser = SpecificationPolicy {
        missing_measurement: crate::specification::MissingMeasurementPolicy::ReportUnmapped,
        ..SpecificationPolicy::default()
    };
    let sheet = build(&specifications, &looser, &run, Some(plan));
    assert_eq!(
        sheet.summary.limits,
        LimitsSinceRun::Edited,
        "a policy edit"
    );
}
