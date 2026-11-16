//! The sheet and the checks, held to the judges they restate.

mod sheet;

use rspice_vocabulary::product::{
    AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId,
};

use super::{RowStatus, Sheet};
use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::family_measurements::{
    FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
};
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::provenance::{
    AnalysisResultProvenance, AnalysisResultPvtPoint, AnalysisResultSourceDomain,
};
use crate::run::{ExecutionTarget, SimulationRun};
use crate::run_receipt::{
    PreparedRunReceipt, PreparedRunReceiptInput, PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
};
use crate::specification::{
    PreparedSpecification, PreparedSpecificationPolicy, SpecEntry, SpecPointScope,
    SpecificationComparison, SpecificationDefinition, SpecificationPolicy,
};
use crate::specification_verdict::{
    SpecificationVerdict, SpecificationVerdictStatus, acceptance_is_blocked,
};
use crate::waveform::RetainedWaveform;

/// A point the run set expanded a task to.
fn at(process: &str, volts: f64, celsius: f64, nominal: bool) -> AnalysisResultPvtPoint {
    AnalysisResultPvtPoint::new(process, Some(volts), celsius, None, nominal)
        .expect("a valid point")
}

fn provenance(point: Option<AnalysisResultPvtPoint>) -> AnalysisResultProvenance {
    AnalysisResultProvenance::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x5a; 32]),
        Vec::new(),
    )
    .expect("valid prepared provenance")
    .with_pvt_point(point)
}

/// An analysis that measured each `(name, value)`, solved at `point`.
fn measured(
    analysis_type: AnalysisType,
    point: Option<AnalysisResultPvtPoint>,
    values: &[(&str, f64)],
) -> AnalysisResult {
    AnalysisResult::new(1, analysis_type, format!("{analysis_type:?}"), 0.0)
        .with_measurements(
            values
                .iter()
                .map(|(name, value)| rspice_core::MeasureResult::success(name, *value))
                .collect(),
        )
        .with_provenance(provenance(point))
}

/// One family member's measurements; `None` is a measurement that ran and
/// gave no number, as a trial that did not converge does.
fn member(id: FamilyMemberId, values: &[(&str, Option<f64>)]) -> FamilyMemberMeasurements {
    FamilyMemberMeasurements::new(
        id,
        values
            .iter()
            .map(|(name, value)| FamilyMeasurementEvidence {
                unit: None,
                name: (*name).to_owned(),
                value: *value,
                passed: value.is_some(),
                error: value.is_none().then(|| "did not converge".to_owned()),
            })
            .collect(),
    )
}

/// The corners of the corner fixture: process, supply, temperature.
const CORNERS: [(&str, f64, f64); 7] = [
    ("TT", 1.8, 27.0),
    ("SS", 1.62, 125.0),
    ("SS", 1.62, -40.0),
    ("FF", 1.98, 125.0),
    ("FF", 1.98, -40.0),
    ("SF", 1.8, 27.0),
    ("FS", 1.8, 27.0),
];

/// The corner fixture: one corner analysis whose members measured four
/// outputs, labelled as the executor labels them.
fn corners() -> AnalysisResult {
    let outputs: [(&str, [f64; 7]); 4] = [
        ("gain", [42.3, 39.4, 42.6, 43.0, 44.2, 42.4, 42.9]),
        (
            "bw",
            [18.2e6, 11.9e6, 14.0e6, 24.1e6, 26.5e6, 16.0e6, 17.1e6],
        ),
        ("pm", [62.0, 58.1, 60.4, 47.0, 52.3, 61.0, 63.2]),
        (
            "idd",
            [
                0.82e-3, 0.71e-3, 0.66e-3, 1.16e-3, 0.95e-3, 0.80e-3, 0.84e-3,
            ],
        ),
    ];
    let labels = CORNERS
        .iter()
        .map(|(process, volts, celsius)| format!("{process}_{volts:.6}V_{celsius:.6}C"))
        .collect::<Vec<_>>();
    let members = labels
        .iter()
        .enumerate()
        .map(|(index, label)| {
            member(
                FamilyMemberId::Corner {
                    index,
                    label: label.clone(),
                },
                &outputs
                    .iter()
                    .map(|(name, values)| (*name, Some(values[index])))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    AnalysisResult::new(1, AnalysisType::Corner, "Corners", 0.0)
        .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
            x_values: (0..CORNERS.len()).map(|index| index as f64).collect(),
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: CORNERS.iter().map(|corner| corner.2).collect(),
            corner_labels: labels,
            failed_corners: 0,
            member_measurements: members,
        })
        .with_provenance(provenance(None))
}

/// The limits the corner fixture is judged by.
fn corner_limits() -> Vec<PreparedSpecification> {
    vec![
        limit("gain", Some(40.0), None, "dB"),
        limit("bw", Some(10.0e6), None, "Hz"),
        limit("pm", Some(45.0), None, "deg"),
        limit("idd", None, Some(1.2e-3), "A"),
    ]
}

fn entry(measurement: &str, min: Option<f64>, max: Option<f64>, unit: &str) -> SpecEntry {
    SpecEntry {
        measurement: measurement.to_owned(),
        expression: String::new(),
        min,
        max,
        unit: unit.to_owned(),
        scope: SpecPointScope::AllPoints,
    }
}

/// A limit as projects stated them before requirements were governed.
fn limit(
    measurement: &str,
    min: Option<f64>,
    max: Option<f64>,
    unit: &str,
) -> PreparedSpecification {
    PreparedSpecification::new(entry(measurement, min, max, unit)).expect("a valid limit")
}

/// A governed requirement over `entry`, as `edit` leaves it.
fn governed(
    entry: &SpecEntry,
    edit: impl FnOnce(&mut SpecificationDefinition),
) -> PreparedSpecification {
    let mut definition = SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, entry);
    edit(&mut definition);
    PreparedSpecification::from_definition(definition).expect("a valid requirement")
}

fn prepared(policy: &SpecificationPolicy) -> PreparedSpecificationPolicy {
    PreparedSpecificationPolicy::new(policy.clone()).expect("a valid policy")
}

/// A run from before runs recorded the plan they answer, which every plan
/// judges.
fn history(analyses: Vec<AnalysisResult>) -> SimulationRun {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses = analyses;
    run
}

/// A run `plan` dispatched, sealed with these limits and policy.
fn sealed(
    plan: SimulationPlanId,
    specifications: &[PreparedSpecification],
    policy: &SpecificationPolicy,
    analyses: Vec<AnalysisResult>,
) -> SimulationRun {
    let digest = |byte: u8| ContentDigest::from_bytes([byte; 32]);
    let receipt = PreparedRunReceipt::new(PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: digest(0x61),
        source_content_digest: digest(0x62),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x63)),
        project_model_sources: Vec::new(),
        specifications: specifications.to_vec(),
        specification_policy: prepared(policy),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                0,
                digest(0x64),
            )
            .expect("a task receipt"),
        ],
    })
    .expect("a plan receipt");
    let mut run = SimulationRun::new_prepared(1, 0.0, ExecutionTarget::LocalDesktop, receipt);
    run.analyses = analyses;
    run
}

/// The sheet `plan` reads of `run`.
fn build(
    specifications: &[PreparedSpecification],
    policy: &SpecificationPolicy,
    run: &SimulationRun,
    plan: Option<SimulationPlanId>,
) -> Sheet {
    Sheet::build::<AnalysisResult, RetainedWaveform>(
        specifications,
        &prepared(policy),
        Some(run),
        plan,
    )
}

/// Every row restates the sealed judge's verdict on the same evidence, and
/// the sheet's acceptance is the gate's.
fn assert_rows_agree(
    sheet: &Sheet,
    specifications: &[PreparedSpecification],
    policy: &SpecificationPolicy,
    run: &SimulationRun,
) {
    let verdicts = SpecificationVerdict::evaluate(specifications, &run.analyses);
    assert_eq!(sheet.rows.len(), verdicts.len());
    if !sheet.is_judged() {
        for row in &sheet.rows {
            assert_eq!(row.status, None, "{} is not judged", row.name);
            assert!(row.verdict.is_none(), "{} is not judged", row.name);
        }
        assert_eq!(sheet.summary.acceptance, None);
        return;
    }
    for (row, verdict) in sheet.rows.iter().zip(&verdicts) {
        let restated = row.verdict.as_ref().expect("a judged row has a verdict");
        assert_eq!(
            format!("{restated:?}"),
            format!("{verdict:?}"),
            "{} restates the judge",
            row.name
        );
        let worst = row.worst.as_ref();
        assert_eq!(worst.and_then(|worst| worst.value), verdict.worst_value());
        assert_eq!(
            worst.and_then(|worst| worst.member.as_ref()),
            verdict.worst_member()
        );
        let status = row.status.expect("a judged row has a status");
        let judged_as = match status {
            RowStatus::Tracked => {
                assert_eq!(row.comparison, SpecificationComparison::Tracked);
                continue;
            }
            RowStatus::Waived => {
                assert!(
                    row.waiver.is_some(),
                    "{} is waived without a waiver",
                    row.name
                );
                assert!(
                    matches!(
                        verdict.status(),
                        SpecificationVerdictStatus::BoundFailure
                            | SpecificationVerdictStatus::MeasurementFailure
                    ),
                    "{} is waived but did not fail",
                    row.name
                );
                continue;
            }
            RowStatus::Pass | RowStatus::Near => SpecificationVerdictStatus::Pass,
            RowStatus::Fail => SpecificationVerdictStatus::BoundFailure,
            RowStatus::Error => SpecificationVerdictStatus::MeasurementFailure,
            RowStatus::NotMeasured => SpecificationVerdictStatus::MissingEvidence,
        };
        assert_eq!(judged_as, verdict.status(), "{}", row.name);
    }
    let blocked = acceptance_is_blocked(specifications, policy, &verdicts, &run.analyses);
    let acceptance = sheet
        .summary
        .acceptance
        .as_ref()
        .expect("a judged sheet states its acceptance");
    assert_eq!(acceptance.blocked, blocked);
    assert_eq!(acceptance.blocked, !acceptance.blocking.is_empty());
}
