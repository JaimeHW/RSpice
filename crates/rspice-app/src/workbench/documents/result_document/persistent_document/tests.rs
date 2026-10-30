//! Lifecycle, projection, marker-ownership and Latest-tracking tests for
//! the project-owned Results document projection.

use super::*;
use crate::state::{
    AnalysisResultProvenance, AnalysisResultSourceDomain, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunLifecycle,
    SimulationRunProvenance,
};
use crate::workbench::documents::result_document::test_host::ResultsTestHost;

pub(crate) fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

// -----------------------------------------------------------------
// marker ownership
// -----------------------------------------------------------------

pub(crate) fn completed_prepared_run(
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    source_digest: ContentDigest,
) -> SimulationRun {
    let task = PreparedRunTaskReceipt::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        Vec::new(),
        5,
        digest(0x44),
    )
    .expect("task receipt");
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan_id),
        project_revision,
        prepared_snapshot_digest: digest(0x41),
        source_content_digest: source_digest,
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![task],
    })
    .expect("run receipt");
    let mut run = SimulationRun::new(1);
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("run provenance");
    run.mark_running().expect("running lifecycle");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed lifecycle");
    run
}

// -----------------------------------------------------------------
// per-document selection and lifecycle
// -----------------------------------------------------------------

// -----------------------------------------------------------------
// per-pane viewport retention
// -----------------------------------------------------------------

// -----------------------------------------------------------------
// Links across panes
// -----------------------------------------------------------------

// -----------------------------------------------------------------
// Latest tracking
// -----------------------------------------------------------------

#[test]
fn latest_candidate_requires_completed_success_and_current_project_source_authority() {
    let plan_id = SimulationPlanId::new();
    let revision = ObjectRevision::INITIAL;
    let source = digest(0x52);
    let mut run = completed_prepared_run(plan_id, revision, source);

    assert!(run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(source)
    ));
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        ObjectRevision::new(revision.get() + 1).expect("next revision"),
        Some(source)
    ));
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(digest(0x53))
    ));
    assert!(!run_matches_current_authority(
        &run, plan_id, revision, None
    ));

    run.success = false;
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(source)
    ));
    run.success = true;
    run.lifecycle = SimulationRunLifecycle::Running;
    assert!(!run_matches_current_authority(
        &run,
        plan_id,
        revision,
        Some(source)
    ));
}

#[test]
fn latest_authored_analysis_uses_the_final_expanded_execution_identity() {
    let authored = AnalysisInstanceId::new();
    let first_execution = AnalysisInstanceId::new();
    let final_execution = AnalysisInstanceId::new();
    let mut run = SimulationRun::new(1);
    for (label, execution) in [
        ("PVT point 1/2", first_execution),
        ("PVT point 2/2", final_execution),
    ] {
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, label).with_provenance(
                AnalysisResultProvenance::new_with_authored_source_domain(
                    AnalysisResultSourceDomain::SimulationPlan,
                    execution,
                    authored,
                    ObjectRevision::INITIAL,
                    digest(0x61),
                    Vec::new(),
                )
                .expect("expanded analysis provenance"),
            ),
        );
    }

    let selected = latest_successful_authored_analysis(&run, authored)
        .expect("the authored analysis has expanded results");
    assert_eq!(selected.label, "PVT point 2/2");
    assert_eq!(analysis_identity(&run, selected), final_execution);
}

#[test]
fn exact_renderer_mapping_exposes_only_implemented_catalog_viewers() {
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-waveform"),
        Some(ResultViewer::Waves)
    );
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-table"),
        Some(ResultViewer::Table)
    );
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-phase-noise"),
        Some(ResultViewer::PhaseNoise)
    );
    assert_eq!(
        ResultViewer::from_viewer_document_id("viewer-manifest"),
        None
    );
    assert_eq!(ResultViewer::from_viewer_document_id("manifest"), None);
    assert_eq!(
        ResultViewer::from_viewer_document_id("field-viewer-3d"),
        None
    );
}

#[test]
fn every_release_target_has_an_exact_native_renderer_identity() {
    let release_targets = crate::results::viewer_catalog::VIEWER_DOCUMENTS
        .iter()
        .filter(|viewer| viewer.release == ViewerReleaseClass::ReleaseTarget)
        .collect::<Vec<_>>();
    assert_eq!(release_targets.len(), 18);
    for viewer in release_targets {
        let native = ResultViewer::from_viewer_document_id(viewer.id);
        assert!(
            native.is_some(),
            "release-target viewer {} has no native Results renderer",
            viewer.id
        );
        assert_eq!(
            native.and_then(ResultViewer::viewer_document_id),
            Some(viewer.id),
            "release-target viewer {} does not round-trip its canonical identity",
            viewer.id
        );
    }
}

#[test]
fn active_pane_identity_is_retained_only_when_it_belongs_to_the_page() {
    assert_eq!(resolved_active_pane_id(Some(7), [3_u64, 7, 11]), Some(7));
    assert_eq!(resolved_active_pane_id(Some(99), [3_u64, 7, 11]), Some(3));
    assert_eq!(resolved_active_pane_id(None, [3_u64, 7, 11]), Some(3));
    assert_eq!(resolved_active_pane_id(Some(7), []), None);
}

#[test]
fn inactive_panes_use_retained_viewers_and_never_the_global_viewer() {
    assert_eq!(
        pane_viewer(true, Some(ResultViewer::Eye), "viewer-waveform"),
        Some(ResultViewer::Eye)
    );
    assert_eq!(
        pane_viewer(false, Some(ResultViewer::Eye), "viewer-waveform"),
        Some(ResultViewer::Waves)
    );
    assert_eq!(
        pane_viewer(false, Some(ResultViewer::Eye), "viewer-manifest"),
        None
    );
}

#[test]
fn retained_frequency_document_restores_its_noise_projection() {
    let noise = AnalysisResult::new(3, AnalysisType::Noise, "noise").with_waveforms(vec![
        crate::state::RetainedTrace::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#fff"),
    ]);
    let mut state = ResultsTestHost::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(noise);
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));

    assert_eq!(
        bound_viewer_projection(state.results_reads(), ResultViewer::Bode),
        ResultViewer::NoiseContrib
    );
}

#[test]
fn interactive_result_viewers_keep_canonical_document_identity() {
    for viewer in [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::Op,
        ResultViewer::NoiseContrib,
        ResultViewer::Contribution,
        ResultViewer::TransferFunction,
        ResultViewer::Specs,
        ResultViewer::Table,
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::PoleZero,
        ResultViewer::Events,
        ResultViewer::Soa,
        ResultViewer::Optimization,
    ] {
        let document_id = viewer
            .viewer_document_id()
            .expect("interactive viewers have catalog identities");
        assert!(
            viewer_document(document_id).is_some(),
            "{viewer:?} mapped to unknown canonical document {document_id}"
        );
    }
}

#[test]
fn persistent_renderer_contract_never_substitutes_a_broader_catalog_mode() {
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "tran").with_waveforms(vec![
        crate::state::RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
    ]);
    let hb = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "hb").with_waveforms(vec![
        crate::state::RetainedTrace::new("V(out)", vec![1.0, 2.0], vec![1.0, 0.5], "#fff"),
    ]);
    let noise = AnalysisResult::new(3, AnalysisType::Noise, "noise").with_waveforms(vec![
        crate::state::RetainedTrace::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 1.0e-19], "#fff"),
    ]);
    let invalid_noise = AnalysisResult::new(4, AnalysisType::Noise, "invalid noise")
        .with_waveforms(vec![crate::state::RetainedTrace::new(
            "onoise",
            vec![1.0, 10.0],
            vec![1.0e-18, -1.0e-19],
            "#fff",
        )]);

    assert!(renderer_supports_analysis("viewer-spectrum", &transient));
    assert!(!renderer_supports_analysis("viewer-spectrum", &hb));
    assert!(renderer_supports_analysis("viewer-bode", &noise));
    assert!(!renderer_supports_analysis("viewer-bode", &invalid_noise));
    assert!(!renderer_supports_analysis("viewer-phase-noise", &noise));
}
