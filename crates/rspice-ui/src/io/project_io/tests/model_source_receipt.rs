//! Which model definitions a run consumed, and under what qualification gate.
//!
//! A receipt names its models by exact revision and content digest rather than
//! by name, so a library edited after the run cannot be mistaken for the one
//! that produced these numbers. Alongside each is whether that exact revision
//! had cleared its qualification gate when the run was sealed — a fact that has
//! to survive the project file, because the gate is editable and the run is
//! not. A restored project that forgot it would present an unqualified result
//! as sign-off evidence, which is the one thing the stamp exists to prevent.

use super::*;

#[test]
fn prepared_run_receipt_round_trip_retains_exact_project_model_sources() {
    let source_id = ModelSourceId::new();
    let plan_id = SimulationPlanId::new();
    let identity = PreparedModelSourceIdentity::new(
        source_id,
        "nch_receipt",
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x51; 32]),
        crate::state::PreparedModelQualification::Unqualified,
    )
    .unwrap();
    let analysis_id = AnalysisInstanceId::new();
    let task = PreparedRunTaskReceipt::new(
        analysis_id,
        ObjectRevision::INITIAL,
        Vec::new(),
        2,
        ContentDigest::from_bytes([0x52; 32]),
    )
    .unwrap();
    let projection = crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param='gain'".to_owned(),
        min: Some(10.0),
        max: None,
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    let mut definition =
        crate::state::SpecificationDefinition::from_legacy(plan_id, 0, &projection);
    definition.requirement_key = "REQ-AMP-017".to_owned();
    definition.requirement_name = "Closed-loop gain".to_owned();
    definition.producing_analysis = Some(analysis_id);
    definition.role = crate::state::SpecificationRole::Review;
    definition.source = Some(crate::state::workspace::SpecificationSource {
        logical_path: "requirements/amplifier.csv".to_owned(),
        row: 18,
        imported_revision: "rev-c".to_owned(),
        source_digest: ContentDigest::from_bytes([0x56; 32]),
    });
    definition.waiver = Some(crate::state::workspace::SpecificationWaiver {
        reference: "WAIVER-9".to_owned(),
        owner: "Analog signoff".to_owned(),
        rationale: "Characterization-only review row".to_owned(),
    });
    let policy =
        crate::state::PreparedSpecificationPolicy::new(crate::state::SpecificationPolicy {
            nominal_failure: crate::state::NominalFailurePolicy::RecordDisposition,
            monte_carlo: crate::state::MonteCarloSpecificationGate::YieldAtLeast { percent: 99.5 },
            regression: crate::state::RegressionSpecificationPolicy::LimitOnly,
            missing_measurement: crate::state::MissingMeasurementPolicy::ReportUnmapped,
        })
        .unwrap();
    let receipt = PreparedRunReceipt::new_with_project_model_sources_specifications_and_policy(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x53; 32]),
        ContentDigest::from_bytes([0x54; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x55; 32])),
        vec![identity],
        vec![crate::state::PreparedSpecification::from_definition(definition.clone()).unwrap()],
        policy,
        vec![task],
    )
    .unwrap();

    let wire = ProjectPreparedRunReceipt::from(&receipt);
    let encoded = serde_json::to_string(&wire).unwrap();
    let decoded = serde_json::from_str::<ProjectPreparedRunReceipt>(&encoded).unwrap();
    let restored = decoded.into_receipt().unwrap();

    assert_eq!(restored.project_model_sources().len(), 1);
    let restored_source = &restored.project_model_sources()[0];
    assert_eq!(restored_source.source_id(), source_id);
    assert_eq!(restored_source.model_name(), "nch_receipt");
    assert_eq!(
        restored_source.qualification(),
        crate::state::PreparedModelQualification::Unqualified,
        "the qualification gate a run was prepared under must survive the \
         project file, or a reopened project presents an unqualified result as \
         sign-off evidence"
    );
    assert!(
        !restored.is_sign_off_eligible(),
        "a restored run with an unqualified model is still not sign-off"
    );
    assert_eq!(restored.unqualified_model_sources().len(), 1);
    assert_eq!(
        restored_source.content_digest(),
        ContentDigest::from_bytes([0x51; 32])
    );
    assert_eq!(restored.specifications().len(), 1);
    assert_eq!(restored.specifications()[0].entry().measurement, "gain");
    let restored_definition = restored.specifications()[0]
        .definition()
        .expect("governed definition survives project I/O");
    assert!(restored_definition.bitwise_eq(&definition));
    assert!(restored.specification_policy().policy().bitwise_eq(
        &crate::state::SpecificationPolicy {
            nominal_failure: crate::state::NominalFailurePolicy::RecordDisposition,
            monte_carlo: crate::state::MonteCarloSpecificationGate::YieldAtLeast { percent: 99.5 },
            regression: crate::state::RegressionSpecificationPolicy::LimitOnly,
            missing_measurement: crate::state::MissingMeasurementPolicy::ReportUnmapped,
        }
    ));
}
