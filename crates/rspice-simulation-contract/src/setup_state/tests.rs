//! Existing compatibility and deterministic migration cases.

use super::*;
use crate::analysis_draft::AnalysisDraft;
use crate::analysis_kind::AnalysisKind;
use crate::drafts::{DistoDraft, NoiseDraft};
use rspice_app_types::product::ProjectId;
use uuid::Uuid;

fn legacy_setup() -> SimulationSetup {
    let mut setup = SimulationSetup::new();
    setup.analysis_plan = None;
    setup.enabled.extend([0, 2]);
    setup.analysis_order = vec![0, 1, 2];
    setup.ac.points = "53".to_owned();
    setup.disto_f2_over_f1 = "0.93".to_owned();
    setup
}

#[test]
fn compatibility_projection_preserves_noise_and_disto_owned_sweeps() {
    let mut setup = SimulationSetup::new();
    let noise = AnalysisDraft::Noise(NoiseDraft {
        points: "77".to_owned(),
        sweep: crate::config::NoiseSweepType::Linear,
        ..NoiseDraft::default()
    });
    setup.apply_analysis_draft_projection(&noise);
    assert_eq!(setup.ac.points, "77");
    assert_eq!(setup.ac.sweep, 2);
    let captured = setup.legacy_analysis_draft(AnalysisKind::Noise);
    let AnalysisDraft::Noise(captured) = captured else {
        panic!("noise projection must retain its kind");
    };
    assert_eq!(captured.points, "77");

    let mut disto = DistoDraft::default();
    disto.sweep.points = "31".to_owned();
    disto.f2_over_f1 = "0.9".to_owned();
    setup.apply_analysis_draft_projection(&AnalysisDraft::Disto(disto));
    let captured = setup.legacy_analysis_draft(AnalysisKind::Disto);
    let AnalysisDraft::Disto(captured) = captured else {
        panic!("DISTO projection must retain its kind");
    };
    assert_eq!(captured.sweep.points, "31");
    assert_eq!(captured.f2_over_f1, "0.9");
}

#[test]
fn legacy_migration_is_project_scoped_reproducible_and_lossless() {
    let namespace = Uuid::from_u128(0xcb81_371c_b443_519d_88ec_0b38_a288_371f);
    let project = ProjectId::from_namespace(namespace, b"legacy-project");
    let mut first = legacy_setup();
    let mut replay = legacy_setup();

    assert!(
        first
            .migrate_legacy_analysis_plan(project)
            .expect("first migration succeeds")
    );
    assert!(
        replay
            .migrate_legacy_analysis_plan(project)
            .expect("replayed migration succeeds")
    );
    let first_json = serde_json::to_string(first.stable_analysis_plan().unwrap()).unwrap();
    let replay_json = serde_json::to_string(replay.stable_analysis_plan().unwrap()).unwrap();
    assert_eq!(first_json, replay_json);

    let plan = first.stable_analysis_plan().unwrap();
    assert_eq!(plan.instances().len(), AnalysisKind::ALL.len());
    assert_eq!(plan.instances()[0].kind(), AnalysisKind::OperatingPoint);
    assert_eq!(plan.instances()[1].kind(), AnalysisKind::Transient);
    assert_eq!(plan.instances()[2].kind(), AnalysisKind::Ac);
    let ac = &plan.instances()[2];
    assert_eq!(ac.dependencies().len(), 1);
    assert_eq!(ac.dependencies()[0].target(), plan.instances()[0].id());
    let AnalysisDraft::Ac(ac_draft) = ac.draft() else {
        panic!("migrated AC retains its exact draft");
    };
    assert_eq!(ac_draft.points, "53");
    let noise = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Noise)
        .unwrap();
    let AnalysisDraft::Noise(noise) = noise.draft() else {
        panic!("migrated Noise owns its sweep");
    };
    assert_eq!(noise.points, "53");
    let disto = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Disto)
        .unwrap();
    let AnalysisDraft::Disto(disto) = disto.draft() else {
        panic!("migrated DISTO owns its sweep");
    };
    assert_eq!(disto.sweep.points, "53");
    assert_eq!(disto.f2_over_f1, "0.93");

    let other_project = ProjectId::from_namespace(namespace, b"other-project");
    let mut other = legacy_setup();
    other
        .migrate_legacy_analysis_plan(other_project)
        .expect("other project migrates");
    assert_ne!(
        plan.id(),
        other.stable_analysis_plan().expect("other plan").id()
    );
    assert_ne!(
        plan.instances()[0].id(),
        other.stable_analysis_plan().unwrap().instances()[0].id()
    );
}

#[test]
fn malformed_legacy_order_fails_without_creating_a_plan() {
    let namespace = Uuid::from_u128(0xcb81_371c_b443_519d_88ec_0b38_a288_371f);
    let project = ProjectId::from_namespace(namespace, b"malformed-project");
    let mut setup = legacy_setup();
    setup.analysis_order = vec![1, 1, 0, 2];

    let error = setup
        .migrate_legacy_analysis_plan(project)
        .expect_err("duplicate legacy order must fail");
    assert!(error.contains("duplicate analysis index 1"));
    assert!(setup.analysis_plan.is_none());
}
