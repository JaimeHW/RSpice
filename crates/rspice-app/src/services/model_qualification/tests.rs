//! Solver integration tests for cooperative qualification sessions.

use super::*;
use rspice_app_types::product::{ContentDigest, ObjectRevision};
use sha2::{Digest as _, Sha256};

fn digest_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn execute_current_platform(
    suite: &QualificationSuite,
    source: &ModelSourceEvidenceBinding,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<QualificationPlatformRun, QualificationExecutionError> {
    let mut session = QualificationExecutionSession::try_new(suite, source)?;
    loop {
        if let QualificationExecutionStep::Complete { run, .. } = session.step(abort)? {
            return Ok(run);
        }
    }
}

fn model_source(byte: u8) -> Vec<u8> {
    format!("* retained qualification model\n.model demo180_nch NMOS (VTO=0.{byte})\n").into_bytes()
}

fn source(byte: u8) -> ModelSourceEvidenceBinding {
    ModelSourceEvidenceBinding::try_new_project_bound(
        "demo180_nch",
        "11111111-1111-4111-8111-111111111111".parse().unwrap(),
        digest_bytes(&model_source(byte)),
        ObjectRevision::new(7).unwrap(),
    )
    .unwrap()
}

fn vector(id: &str, name: &str, byte: u8) -> QualificationVector {
    vector_for_source(id, name, byte, 7)
}

fn vector_for_source(
    id: &str,
    name: &str,
    input_variant: u8,
    source_variant: u8,
) -> QualificationVector {
    let retained_model = model_source(source_variant);
    let input = format!(
            "qualification {input_variant}\n{}V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n",
            String::from_utf8(retained_model.clone()).unwrap()
        )
        .into_bytes();
    QualificationVector::try_new(
        rspice_model_library::qualification::QualificationVectorInput {
            id: id.into(),
            name: name.into(),
            source: source(source_variant),
            model_section: None,
            execution_model_source: retained_model.clone(),
            model_source: retained_model,
            executable_input: input,
            analysis: QualificationAnalysis::DcOperatingPoint,
            outputs: vec![
                QualificationOutputDefinition::try_new(
                    "drain_current",
                    QualificationProbe::NodeVoltage {
                        node: "output".into(),
                    },
                    QualificationSample::OperatingPoint,
                )
                .unwrap(),
            ],
            references: vec![
                QualificationReference::try_new("drain_current", 1.0, 0.01, 0.005).unwrap(),
            ],
        },
    )
    .unwrap()
}

fn suite() -> QualificationSuite {
    QualificationSuite::try_new(
        "dc-iv",
        "DC IV",
        ObjectRevision::new(3).unwrap(),
        vec![
            vector("dc-002", "Output sweep", 2),
            vector("dc-001", "Transfer sweep", 1),
        ],
    )
    .unwrap()
}

fn executable_vector(
    id: &str,
    name: &str,
    testbench: &str,
    analysis: QualificationAnalysis,
    outputs: Vec<QualificationOutputDefinition>,
    references: Vec<QualificationReference>,
) -> QualificationVector {
    let retained_model = model_source(7);
    let input = format!(
        "qualification {id}\n{}{testbench}",
        String::from_utf8(retained_model.clone()).unwrap()
    )
    .into_bytes();
    QualificationVector::try_new(
        rspice_model_library::qualification::QualificationVectorInput {
            id: id.into(),
            name: name.into(),
            source: source(7),
            model_section: None,
            execution_model_source: retained_model.clone(),
            model_source: retained_model,
            executable_input: input,
            analysis,
            outputs,
            references,
        },
    )
    .unwrap()
}

#[test]
fn native_execution_measures_outputs_and_applies_declared_tolerances() {
    let source = source(7);
    let passing_suite = suite();
    let run = execute_current_platform(&passing_suite, &source, &rspice_core::NoAbort).unwrap();
    assert_eq!(run.platform, QualificationPlatform::Desktop);
    assert!(run.passed);
    assert_eq!(run.vector_outcomes.len(), passing_suite.vectors.len());
    let reference = &run.vector_outcomes[0].outcome.references[0];
    assert_eq!(reference.expected_value, FiniteValue::new(1.0).unwrap());
    assert_eq!(reference.observed_value, FiniteValue::new(1.0).unwrap());

    let mut failing_suite = passing_suite.clone();
    for vector in &mut failing_suite.vectors {
        vector.references[0].expected = FiniteValue::new(2.0).unwrap();
    }
    let run = execute_current_platform(&failing_suite, &source, &rspice_core::NoAbort).unwrap();
    assert!(!run.passed);
    assert!(
        run.vector_outcomes
            .iter()
            .all(|value| !value.outcome.passed)
    );
}

#[test]
fn ac_cv_noise_and_transient_vectors_execute_real_solver_results() {
    let ac_vector = executable_vector(
        "ac-cv",
        "AC and effective capacitance",
        "V1 output 0 DC 0 AC 1\nM1 output output 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::AcSweep {
            frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
        },
        vec![
            QualificationOutputDefinition::try_new(
                "capacitance",
                QualificationProbe::AcEffectiveCapacitance {
                    branch: "V1".to_owned(),
                    excitation_magnitude: FiniteValue::new(1.0).unwrap(),
                },
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
            QualificationOutputDefinition::try_new(
                "voltage-magnitude",
                QualificationProbe::AcNodeVoltageMagnitude {
                    node: "output".to_owned(),
                },
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
        ],
        vec![
            QualificationReference::try_new("capacitance", 0.0, 1.0, 0.0).unwrap(),
            QualificationReference::try_new("voltage-magnitude", 1.0, 1.0e-10, 1.0e-10).unwrap(),
        ],
    );
    let noise_vector = executable_vector(
        "noise",
        "Output noise",
        "V1 input 0 DC 0 AC 1\nR1 input output 1k\nR2 output 0 1k\nM1 output input 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::Noise {
            output_node: "output".to_owned(),
            output_reference: None,
            input_source: "V1".to_owned(),
            frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
            temperature_kelvin: FiniteValue::new(300.15).unwrap(),
        },
        vec![
            QualificationOutputDefinition::try_new(
                "output-noise-density",
                QualificationProbe::NoiseOutputDensity,
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("output-noise-density", 0.0, 1.0, 0.0).unwrap()],
    );
    let transient_vector = executable_vector(
        "transient",
        "Transient voltage",
        "V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::Transient {
            stop_time: FiniteValue::new(1.0e-6).unwrap(),
            max_step: FiniteValue::new(1.0e-7).unwrap(),
        },
        vec![
            QualificationOutputDefinition::try_new(
                "output-voltage",
                QualificationProbe::TransientNodeVoltage {
                    node: "output".to_owned(),
                },
                QualificationSample::LastTimePoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("output-voltage", 1.0, 1.0e-8, 1.0e-8).unwrap()],
    );
    let suite = QualificationSuite::try_new(
        "advanced",
        "Advanced qualification",
        ObjectRevision::INITIAL,
        vec![ac_vector, noise_vector, transient_vector],
    )
    .unwrap();

    let run = execute_current_platform(&suite, &source(7), &rspice_core::NoAbort).unwrap();
    assert!(run.passed, "{run:#?}");
    for outcome in &run.vector_outcomes {
        assert!(outcome.outcome.failure.is_none(), "{outcome:#?}");
        assert!(
            outcome
                .outcome
                .references
                .iter()
                .all(|reference| reference.observed_value.get().is_finite()),
            "{outcome:#?}"
        );
    }
    let ac = run
        .vector_outcomes
        .iter()
        .find(|outcome| outcome.vector_id == "ac-cv")
        .unwrap();
    let voltage = ac
        .outcome
        .references
        .iter()
        .find(|reference| reference.quantity == "voltage-magnitude")
        .unwrap();
    assert!((voltage.observed_value.get() - 1.0).abs() <= 1.0e-10);
}

#[test]
fn execution_failures_are_retained_as_failing_platform_outcomes() {
    let source = source(7);
    let mut suite = suite();
    suite.vectors[0].outputs[0].probe = QualificationProbe::NodeVoltage {
        node: "missing-node".into(),
    };
    let run = execute_current_platform(&suite, &source, &rspice_core::NoAbort).unwrap();
    let failed = run
        .vector_outcomes
        .iter()
        .find(|value| value.vector_id == "dc-001")
        .unwrap();
    assert!(!failed.outcome.passed);
    assert_eq!(
        failed.outcome.failure.as_ref().unwrap().stage,
        QualificationFailureStage::Measurement
    );
    assert!(failed.outcome.references.is_empty());
    assert!(run.validate_bound(&suite, &source).is_ok());
}

#[test]
fn missing_runtime_parity_cannot_be_promoted_to_evidence() {
    let source = source(7);
    let suite = suite();
    let desktop = execute_current_platform(&suite, &source, &rspice_core::NoAbort).unwrap();
    let error =
        QualificationEvidence::assemble_platform_runs("evidence", &suite, &source, vec![desktop])
            .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::EvidenceCoverageMismatch);
}

#[test]
fn cancellation_publishes_no_partial_platform_run() {
    let result = execute_current_platform(
        &suite(),
        &source(7),
        &rspice_core::abort_signal::ImmediateAbort,
    );
    assert!(matches!(
        result,
        Err(QualificationExecutionError::Cancelled)
    ));
}

#[test]
fn cooperative_session_runs_one_vector_per_step_and_cancels_atomically() {
    let source = source(7);
    let suite = suite();
    let mut session = QualificationExecutionSession::try_new(&suite, &source).unwrap();
    assert_eq!(session.progress().completed_vectors, 0);
    let step = session.step(&rspice_core::NoAbort).unwrap();
    let QualificationExecutionStep::InProgress(progress) = step else {
        panic!("a two-vector suite must not publish a run after one step");
    };
    assert_eq!(progress.completed_vectors, 1);
    assert!(!session.is_finished());
    session.cancel();
    assert!(session.is_cancelled());
    assert!(matches!(
        session.step(&rspice_core::NoAbort),
        Err(QualificationExecutionError::Cancelled)
    ));
}

#[test]
fn cooperative_session_publishes_only_the_terminal_validated_run() {
    let source = source(7);
    let suite = suite();
    let mut session = QualificationExecutionSession::try_new(&suite, &source).unwrap();
    assert!(matches!(
        session.step(&rspice_core::NoAbort).unwrap(),
        QualificationExecutionStep::InProgress(_)
    ));
    let terminal = session.step(&rspice_core::NoAbort).unwrap();
    let QualificationExecutionStep::Complete { progress, run } = terminal else {
        panic!("the final vector must publish the complete platform run");
    };
    assert_eq!(progress.completed_vectors, suite.vectors.len());
    run.validate_bound(&suite, &source).unwrap();
    assert!(session.is_finished());
    assert!(matches!(
        session.step(&rspice_core::NoAbort),
        Err(QualificationExecutionError::SessionFinished)
    ));
}

#[test]
fn assembled_evidence_is_deterministic_and_requires_real_run_records() {
    let source = source(7);
    let suite = suite();
    let desktop = execute_current_platform(&suite, &source, &rspice_core::NoAbort).unwrap();
    let wasm_outcomes = desktop
        .vector_outcomes
        .iter()
        .map(|value| {
            let mut value = value.clone();
            value.outcome.platform = QualificationPlatform::WebAssembly;
            value
        })
        .collect();
    // The session identifies its actual runtime; imported runs use the same
    // bound-run validator.
    let wasm = QualificationPlatformRun::try_new(
        QualificationPlatform::WebAssembly,
        source.clone(),
        &suite,
        wasm_outcomes,
    )
    .unwrap();
    let first = QualificationEvidence::assemble_platform_runs(
        "evidence",
        &suite,
        &source,
        vec![desktop.clone(), wasm.clone()],
    )
    .unwrap();
    let second = QualificationEvidence::assemble_platform_runs(
        "evidence",
        &suite,
        &source,
        vec![wasm, desktop],
    )
    .unwrap();
    assert_eq!(first, second);
    assert_eq!(
        serde_json::to_string(&first).unwrap(),
        serde_json::to_string(&second).unwrap()
    );
}
