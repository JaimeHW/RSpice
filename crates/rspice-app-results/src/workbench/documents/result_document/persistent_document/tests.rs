//! Lifecycle, projection, marker-ownership and Latest-tracking tests for
//! the project-owned Results document projection.

use super::*;
use crate::state::{AnalysisResultProvenance, AnalysisResultSourceDomain, SimulationRunLifecycle};
use crate::workbench::documents::result_document::test_fixtures::{completed_prepared_run, digest};
use crate::workbench::documents::result_document::test_host::ResultsTestHost;

// -----------------------------------------------------------------
// marker ownership
// -----------------------------------------------------------------

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
    assert_eq!(release_targets.len(), 21);
    for viewer in release_targets {
        let native = ResultViewer::from_viewer_document_id(viewer.id);
        assert!(
            native.is_some(),
            "release-target viewer {} has no native Results renderer",
            viewer.id
        );
        assert_eq!(
            native.map(ResultViewer::viewer_document_id),
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
        ResultViewer::Table,
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::PoleZero,
        ResultViewer::Events,
        ResultViewer::Soa,
        ResultViewer::Optimization,
    ] {
        let document_id = viewer.viewer_document_id();
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

// -----------------------------------------------------------------
// event lanes
// -----------------------------------------------------------------

/// A net ticked after the lanes were drawn has no row yet. The document
/// lists it after the rows, or the view, which plots history-only nets as
/// the document lists them, takes it off before it is ever drawn.
#[test]
fn a_net_ticked_after_the_lanes_were_drawn_stays_on_the_plot() {
    use crate::results::visualization_document::{DocumentEdit, EventLaneRef};
    use rspice_results_ui::waves::lanes::{EventLaneLayout, LaneKey};

    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        crate::workbench::documents::result_document::test_fixtures::events_analysis(),
    );
    let run = host.model.simulation.active_run().expect("a run");
    let analysis = &run.analyses[0];
    let names = lanes::EventNames::of(analysis, None).expect("an event history");
    let source = crate::workbench::documents::result_document::create_document::source_reference(
        run,
        analysis,
        digest(0x52),
    )
    .expect("a source");
    let document = VisualizationDocument::new("Waves", vec![source]).expect("a document");
    let pane = document.panes()[0].id;
    let drawn = EventLaneLayout {
        order: vec![LaneKey::Signal("D(clk)".to_owned())],
        ..EventLaneLayout::default()
    };
    let edits = lanes::lane_edits(&document, pane, &names, &drawn, &["D(clk)", "D(addr[0])"]);
    let order = edits
        .iter()
        .find_map(|edit| match edit {
            DocumentEdit::SetEventLaneOrder(order) => Some(&order.lanes),
            _ => None,
        })
        .expect("the lanes' order is written");
    let net = |name: &str| EventLaneRef::Net {
        name: name.to_owned(),
    };
    assert_eq!(order, &vec![net("D(clk)"), net("D(addr[0])")]);
}
