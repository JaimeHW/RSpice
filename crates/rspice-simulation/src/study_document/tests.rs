use super::*;
use crate::execution::{HeadlessSourceResolver, PreparedRunAuthorization, prepare_headless_run};
use crate::results::SimulationResult;
use crate::runner::SimulationRunner;
use rspice_core::abort_signal::CountingAbort;
use rspice_core::{NoAbort, ResourceKind, ResourceLimits};
use rspice_simulation_contract::config::FftRequest;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const CIRCUIT: &str =
    "Study divider\nV1 in 0 DC 1 AC 1 SIN(0 1 1k)\nR1 in out 1k\nR2 out 0 1k\n.end\n";

fn example() -> Value {
    json!({
        "schema_version": 1,
        "id": "5001d7f3-8c60-42d0-9b18-0668251b1a0d",
        "circuit": { "path": "divider.cir" },
        "tasks": [{ "id": "bias", "analysis": "DcOp" }]
    })
}

fn decode(value: &Value) -> Result<StudyDocument, StudyDocumentError> {
    StudyDocument::from_json(&value.to_string(), 1024 * 1024, &NoAbort)
}

fn prepare(document: &StudyDocument) -> crate::execution::PreparedRunSnapshot {
    let origin = std::env::temp_dir().join("rspice-study-fixture.cir");
    let input = document
        .run_input(
            CIRCUIT,
            &origin,
            HeadlessSourceResolver::Sealed(Default::default()),
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    prepare_headless_run(input, &NoAbort).unwrap()
}

#[test]
fn study_decode_rejects_unknown_fields_duplicates_trailing_values_and_versions() {
    let mut document = example();
    document["tasks"][0]["analysis"] = json!({ "Ac": {
        "start_freq": 1.0, "stop_freq": 1000.0, "points_per_unit": 10, "sweep": "Decade",
        "pointz": 20
    }});
    let error = decode(&document).unwrap_err();
    assert!(
        matches!(&error, StudyDocumentError::UnknownField(path) if path.contains("pointz")),
        "{error}"
    );
    document = example();
    document["tasks"][0]["analysis_line"] = json!(".include unwanted.cir");
    assert!(decode(&document).is_err());
    document = example();
    document["schema_version"] = json!(2);
    assert!(matches!(
        decode(&document),
        Err(StudyDocumentError::UnsupportedVersion(2))
    ));
    document = example();
    document["id"] = json!("00000000-0000-0000-0000-000000000000");
    assert!(decode(&document).is_err());
    for source in [
        format!("{} {{}}", example()),
        example()
            .to_string()
            .replacen('{', "{\"schema_version\":1,", 1),
    ] {
        assert!(StudyDocument::from_json(&source, 1024 * 1024, &NoAbort).is_err());
    }
}

#[test]
fn study_task_identity_is_stable_across_reordering_labels_and_setting_edits() {
    let mut document = example();
    document["tasks"].as_array_mut().unwrap().push(json!({
        "id": "response", "depends_on": ["bias"], "analysis": { "Ac": {
            "start_freq": 1.0, "stop_freq": 1000.0, "points_per_unit": 10, "sweep": "Decade"
        }}
    }));
    let first = decode(&document)
        .unwrap()
        .lower_tasks(ResourceLimits::default(), &NoAbort)
        .unwrap();
    document["tasks"][0]["label"] = json!("Renamed bias plot");
    document["tasks"][1]["analysis"]["Ac"]["stop_freq"] = json!(2000.0);
    document["tasks"].as_array_mut().unwrap().reverse();
    let changed = decode(&document)
        .unwrap()
        .lower_tasks(ResourceLimits::default(), &NoAbort)
        .unwrap();
    assert_eq!(first[0].instance_id, changed[1].instance_id);
    assert_eq!(first[1].instance_id, changed[0].instance_id);
    assert_eq!(changed[0].dependencies, [changed[1].instance_id]);
    document["id"] = json!("d9d9154a-c044-47c8-9c2c-1e83e10984cf");
    let foreign = decode(&document)
        .unwrap()
        .lower_tasks(ResourceLimits::default(), &NoAbort)
        .unwrap();
    assert_ne!(foreign[0].instance_id, changed[0].instance_id);
}

#[test]
fn study_graph_limits_cancellation_and_missing_names_fail_before_dispatch() {
    let source = example().to_string();
    assert!(matches!(
        StudyDocument::from_json(&source, source.len() - 1, &NoAbort),
        Err(StudyDocumentError::InputTooLarge { .. })
    ));
    assert!(matches!(
        StudyDocument::from_json(&source, source.len(), &CountingAbort::new(0)),
        Err(StudyDocumentError::Aborted)
    ));
    let document = decode(&example()).unwrap();
    let mut limits = ResourceLimits::default();
    limits.max_batch_runs = 0;
    assert!(
        matches!(document.lower_tasks(limits, &NoAbort), Err(StudyDocumentError::ResourceLimit(error)) if error.resource == ResourceKind::BatchRuns)
    );
    assert!(matches!(
        document.lower_tasks(ResourceLimits::default(), &CountingAbort::new(0)),
        Err(StudyDocumentError::Aborted)
    ));
    for name in ["", "line\nbreak", "has spaces"] {
        let mut value = example();
        value["tasks"][0]["id"] = json!(name);
        assert!(
            decode(&value)
                .unwrap()
                .lower_tasks(ResourceLimits::default(), &NoAbort)
                .is_err()
        );
    }
    let mut value = example();
    value["tasks"][0]["depends_on"] = json!(["missing"]);
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .is_err()
    );
    value = example();
    let duplicate = value["tasks"][0].clone();
    value["tasks"].as_array_mut().unwrap().push(duplicate);
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .is_err()
    );
    value = example();
    value["tasks"][0]["depends_on"] = json!(["bias"]);
    let document = decode(&value).unwrap();
    let path = std::env::temp_dir().join("study-self-edge.cir");
    let input = document
        .run_input(
            CIRCUIT,
            &path,
            HeadlessSourceResolver::Sealed(Default::default()),
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    assert!(prepare_headless_run(input, &NoAbort).is_err());
}

#[test]
fn periodic_settings_are_explicit_and_cannot_be_attached_to_another_kind() {
    let mut value = example();
    value["tasks"][0]["analysis"] = json!("Pac");
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .is_err()
    );
    value["tasks"][0]["periodic"] =
        serde_json::to_value(StudyPeriodicSettings::Pac(PacRunConfig::default())).unwrap();
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .unwrap()[0]
            .analysis
            .spec_options
            .pac
            .is_some()
    );
    value["tasks"][0]["analysis"] = json!("DcOp");
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .is_err()
    );
}

#[test]
fn numerical_settings_preserve_precision_precedence_and_solver_ownership() {
    use rspice_simulation_contract::numeric_override::NumericOverrideOption;
    let mut value = example();
    value["numeric"] = json!({"reltol": 1e-3, "integration_method": "Gear"});
    value["tasks"][0]["numeric"] = json!({"reltol": 1.234567891234567e-6});
    let document = decode(&value).unwrap();
    let tasks = document
        .lower_tasks(ResourceLimits::default(), &NoAbort)
        .unwrap();
    let record = tasks[0].analysis.numeric_override.as_ref().unwrap();
    assert!(
        record
            .stated(NumericOverrideOption::IntegrationMethod)
            .is_none()
    );
    assert_eq!(
        record.stated(NumericOverrideOption::Reltol),
        Some(
            rspice_simulation_contract::numeric_override::OverrideValue::Real(1.234567891234567e-6)
        )
    );
    let prepared = prepare(&document);
    assert!(prepared.digest() != prepare(&decode(&example()).unwrap()).digest());
    value["tasks"][0]["numeric"] = json!({"reltol": -1.0});
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .is_err()
    );
    value["tasks"][0]["numeric"] = json!({"integration_method": "Gear"});
    assert!(
        decode(&value)
            .unwrap()
            .lower_tasks(ResourceLimits::default(), &NoAbort)
            .is_err()
    );
}

#[test]
fn a_json_study_executes_and_retains_the_bound_transient_fft() {
    let mut value = example();
    let transient = AnalysisSpec::Transient {
        stop_time: 0.001,
        step_time: 1e-5,
        start_time: 0.0,
        max_timestep: Some(1e-5),
        uic: false,
    };
    let fft = AnalysisSpec::Fft {
        request: FftRequest {
            output: "V(out)".into(),
            start: Some(0.0),
            stop: Some(0.001),
            points: 16,
            ..Default::default()
        },
    };
    value["tasks"].as_array_mut().unwrap().extend([
        json!({ "id": "spectrum", "analysis": fft, "depends_on": ["waveform"] }),
        json!({ "id": "waveform", "analysis": transient }),
    ]);
    let snapshot = prepare(&decode(&value).unwrap());
    let mut pending = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .unwrap()
        .into_tasks();
    let mut runner = SimulationRunner::new();
    let mut artifacts = HashMap::new();
    let mut results = Vec::new();
    while let Some(task) = pending.pop_front() {
        let id = task.instance_id();
        let resolved = task.resolve_dependency_artifacts(&artifacts).unwrap();
        let producer = resolved.artifact_producer().unwrap();
        runner.start_prepared(resolved, false).unwrap();
        let deadline = Instant::now() + Duration::from_secs(20);
        let result = loop {
            if let Some(result) = runner.poll_result() {
                break result.unwrap();
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        };
        if let Some(artifact) = producer.capture(&result, &pending).unwrap() {
            artifacts.insert(id, artifact);
        }
        results.push(result);
    }
    assert_eq!(results.len(), 3);
    assert!((results[0].measurement("V(out)").unwrap() - 0.5).abs() < 1e-10);
    let SimulationResult::Transient { spectra, .. } = &results[1] else {
        panic!("transient")
    };
    let SimulationResult::Fft { spectrum, .. } = &results[2] else {
        panic!("FFT")
    };
    assert!(std::sync::Arc::ptr_eq(&spectra[0], spectrum));
}
