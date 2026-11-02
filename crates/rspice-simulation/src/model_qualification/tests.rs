//! The executor in process and through the browser worker's wire.

use super::*;
use rspice_vocabulary::product::{ContentDigest, ObjectRevision};
use sha2::{Digest as _, Sha256};

const MODEL: &str = "* retained qualification model\n.model demo180_nch NMOS (VTO=0.7 KP=2e-5)\n";
const BENCH: &str = "I1 0 gate DC 100u\nM1 gate gate 0 0 demo180_nch W=10u L=1u\n.end\n";

fn vector(node: &str) -> QualificationVector {
    let model = MODEL.as_bytes().to_vec();
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        "demo180_nch",
        "11111111-1111-4111-8111-111111111111".parse().unwrap(),
        ContentDigest::from_bytes(Sha256::digest(&model).into()),
        ObjectRevision::new(7).unwrap(),
    )
    .unwrap();
    QualificationVector::try_new(QualificationVectorInput {
        id: "dc-001".into(),
        name: "Diode-connected gate voltage".into(),
        source,
        model_section: None,
        execution_model_source: model.clone(),
        model_source: model,
        executable_input: format!("qualification pin\n{MODEL}{BENCH}").into_bytes(),
        analysis: QualificationAnalysis::DcOperatingPoint,
        outputs: vec![
            QualificationOutputDefinition::try_new(
                "gate_voltage",
                QualificationProbe::NodeVoltage { node: node.into() },
                QualificationSample::OperatingPoint,
            )
            .unwrap(),
        ],
        references: vec![QualificationReference::try_new("gate_voltage", 1.7, 1e-6, 0.0).unwrap()],
    })
    .unwrap()
}

fn in_process(vector: &QualificationVector) -> PlatformQualificationOutcome {
    execute_qualification_vector(
        vector,
        QualificationPlatform::WebAssembly,
        &rspice_core::NoAbort,
    )
    .unwrap()
}

fn through_the_worker_wire(vector: &QualificationVector) -> QualificationVectorResult {
    let request =
        encode_qualification_worker_request(vector, QualificationPlatform::WebAssembly).unwrap();
    let response = run_qualification_worker_request(&request).unwrap();
    decode_qualification_worker_response(&response).unwrap()
}

#[test]
fn a_vector_measures_the_same_outcome_in_process_and_through_the_worker() {
    // 100 uA into a diode-connected device with KP*W/L = 2e-4 settles at
    // VGS = VTO + sqrt(2 * 100e-6 / 2e-4) = 1.7 V.
    let passing = vector("gate");
    let direct = in_process(&passing);
    assert!(direct.passed, "{direct:?}");
    assert_eq!(direct.platform, QualificationPlatform::WebAssembly);
    let observed = direct.references[0].observed_value.get();
    assert!((observed - 1.7).abs() < 1e-6, "observed {observed}");
    assert_eq!(through_the_worker_wire(&passing), Ok(direct));

    // A probe of a node the deck lacks fails at measurement: a retained
    // failed outcome, not an error.
    let failing = vector("absent");
    let direct = in_process(&failing);
    assert!(!direct.passed);
    let failure = direct.failure.as_ref().expect("a failed measurement");
    assert_eq!(failure.stage, QualificationFailureStage::Measurement);
    assert_eq!(failure.message, "node voltage 'absent' is unavailable");
    assert_eq!(through_the_worker_wire(&failing), Ok(direct));
}

#[test]
fn the_worker_refuses_a_request_of_another_protocol() {
    let request =
        encode_qualification_worker_request(&vector("gate"), QualificationPlatform::WebAssembly)
            .unwrap()
            .replacen(
                &format!("\"protocol_version\":{QUALIFICATION_WORKER_PROTOCOL_VERSION}"),
                "\"protocol_version\":0",
                1,
            );
    assert_eq!(
        run_qualification_worker_request(&request),
        Err("unsupported qualification request protocol 0".to_owned())
    );
}
