//! Contract checks use explicitly synthetic data, never a shipping calibration.

use super::*;
use crate::simulation::dialog::ReliabilityDialogState;
use crate::simulation::dialog::reliability::{ReliabilityConfig, ReliabilityStudyDraft};
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;

pub(crate) fn fixture() -> ReliabilityStudy {
    serde_json::from_value(serde_json::json!({
        "model_pack": {
            "schema_version": 1, "id": "synthetic-contract-fixture", "process": "test only",
            "qualification": "user_characterized", "source": "analytical unit test",
            "license": "test fixture", "characterization": "not a physical calibration",
            "models": [{
                "id": "nbti", "mechanism": "nbti", "applicability": "synthetic PMOS",
                "validity": {
                    "gate_source_v": {"min": -2.0, "max": 2.0},
                    "drain_source_v": {"min": -2.0, "max": 2.0},
                    "temperature_k": {"min": 200.0, "max": 500.0},
                    "current_density_a_per_m2": {"min": 0.0, "max": 1e12},
                    "max_equivalent_seconds": 1e10
                },
                "law": {
                    "kind": "equivalent_time_power", "reference_time_s": 100.0,
                    "reference_gate_magnitude_v": 1.0, "reference_drain_magnitude_v": 1.0,
                    "reference_temperature_k": 300.0, "gate_polarity": "negative",
                    "clock_gate_exponent": 2.0, "clock_drain_exponent": 0.0,
                    "clock_activation_energy_ev": 0.1, "time_exponent": 0.25,
                    "parameters": [{"parameter": "VTO", "update": "additive", "scale_at_reference_time": -0.01}]
                }
            }]
        },
        "bindings": [{"device": "M1", "compact_model": "PM", "aging_models": ["nbti"], "conductor_area_m2": 1e-14}],
        "mission": [
            {"name": "active", "duration_s": 3600.0, "temperature_c": 80.0, "parameters": {"VDD": 1.2}},
            {"name": "idle", "duration_s": 1800.0, "temperature_c": 27.0, "parameters": {"VDD": 0.5}}
        ],
        "repeat_mission": true,
        "transient_stress": {"step_s": 1e-9, "stop_s": 1e-6, "start_s": 2e-7, "max_step_s": 2e-9, "use_initial_conditions": true}
    })).unwrap()
}

pub(crate) fn spec(study: Option<ReliabilityStudy>) -> AnalysisSpec {
    AnalysisSpec::Reliability {
        study,
        target_years: vec![1.0000000123, 10.0],
        enable_hci: false,
        enable_nbti: true,
        enable_em: false,
        min_stress_voltage: 0.0,
    }
}

#[test]
fn reliability_study_survives_draft_project_and_worker_round_trips() {
    let study = fixture();
    study.validate(&[1.0, 10.0], false, true, false).unwrap();
    let config = ReliabilityConfig {
        study: Some(study.clone()),
        target_years: vec![1.0000000123, 1.0000000123001, 10.0],
        enable_hci: false,
        enable_nbti: true,
        enable_em: false,
        min_stress_voltage: 0.0,
    };
    let draft = ReliabilityDialogState::from_config(&config);
    let mut restored: ReliabilityDialogState =
        serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
    // Project loading initializes persisted fields without replacing them.
    restored.initialized = true;
    assert_eq!(restored.to_config().unwrap(), config);
    let spec = spec(Some(study));
    spec.validate().unwrap();
    let wire = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let wire =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&wire).unwrap()).unwrap();
    assert_eq!(AnalysisSpec::from(wire), spec);
    let legacy = serde_json::json!({"Reliability": {
        "target_years": [1.0], "enable_hci": true, "enable_nbti": true,
        "enable_em": false, "min_stress_voltage": 0.1
    }});
    let legacy: WorkerAnalysisSpec = serde_json::from_value(legacy).unwrap();
    assert!(matches!(
        AnalysisSpec::from(legacy),
        AnalysisSpec::Reliability { study: None, .. }
    ));
}

#[test]
fn reliability_study_refuses_unbound_mechanisms_incomplete_missions_and_bad_imports() {
    let study = fixture();
    assert!(
        study
            .validate(&[1.0], true, true, false)
            .unwrap_err()
            .contains("HCI")
    );
    let mut short = study.clone();
    short.repeat_mission = false;
    assert!(
        short
            .validate(&[1.0], false, true, false)
            .unwrap_err()
            .contains("cover")
    );
    let mut duplicates = study.clone();
    duplicates.bindings.push(study.bindings[0].clone());
    assert!(
        duplicates
            .validate(&[1.0], false, true, false)
            .unwrap_err()
            .contains("Duplicate")
    );
    let mut draft = ReliabilityStudyDraft::from_study(Some(&study));
    let original = draft.model_pack_json.clone();
    assert!(draft.import_model_pack("{}").is_err());
    assert_eq!(draft.model_pack_json, original);
    draft.mission[0].parameters = "VDD=1.2, vdd=1.3".into();
    assert!(
        draft
            .to_study()
            .unwrap()
            .unwrap()
            .validate(&[1.0], false, true, false)
            .is_err()
    );
    draft.model_pack_json.clear();
    assert!(
        draft.to_study().is_err(),
        "a configured mission cannot silently become legacy"
    );
    assert_eq!(ReliabilityStudyDraft::default().to_study().unwrap(), None);
}
