//! Distribution selection and publication through the actual worker and renderer.

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, MonteCarloVariableMetadata,
    SimulationRunLifecycle, SimulationRunProvenance,
};
use crate::workbench::ResultViewer;
use crate::workbench::app_state::AppState;
use crate::workbench::hardcopy_adapters::sources::{
    HardcopySemanticDocument, SourceReads, prepare_retained_hardcopy_resolution,
};
use crate::workbench::state::WorkspaceDocumentId;
use rspice_hardcopy_contract::{
    BackgroundMode, ColorMapping, FontPolicy, HardcopyScope, HardcopySetup, OutputFormat,
    RenderSetup, RenderTarget,
};

pub(crate) fn distribution() -> (AppState, String) {
    let mut state = AppState::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 17,
                runs_requested: 5,
                runs_completed: 5,
                failures: 0,
                all_converged: true,
                member_measurements: Vec::new(),
                variables: vec![
                    MonteCarloVariableMetadata {
                        mean_confidence: None,
                        name: "offset".to_owned(),
                        samples: vec![99.0; 5],
                        mean: 99.0,
                        std_dev: 0.0,
                        min: 99.0,
                        max: 99.0,
                    },
                    MonteCarloVariableMetadata {
                        mean_confidence: None,
                        name: "gain".to_owned(),
                        samples: vec![-2.0, -1.0, 0.0, 1.0, 2.0],
                        mean: 0.0,
                        std_dev: 2.5_f64.sqrt(),
                        min: -2.0,
                        max: 2.0,
                    },
                ],
            },
        ),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    let dataset = run.dataset_id;
    state.model.simulation.complete_run();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset));
    state.results.analysis.histogram_state.selected = Some("gain".to_owned());
    state.results.analysis.histogram_state.bin_count = 5;
    state.results.results.session.viewer = ResultViewer::Hist;
    let key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        dataset
    );
    (state, key)
}

pub(crate) fn setup(
    format: OutputFormat,
    mapping: &rspice_hardcopy_contract::PrintMappingTable,
) -> HardcopySetup {
    let base = HardcopySetup::default();
    HardcopySetup::try_new_with_schematic(
        base.physical_page().clone(),
        base.scale(),
        base.tiling(),
        RenderSetup::try_new(
            if format == OutputFormat::BrowserPrintDocument {
                RenderTarget::BrowserPrintDialog
            } else {
                RenderTarget::ExportArtifact
            },
            format,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(format.is_vector(), format.is_vector()),
            true,
        )
        .unwrap(),
        base.decorations().clone(),
        base.schematic(),
        mapping.clone(),
    )
    .unwrap()
}

#[test]
fn histogram_publication_keeps_the_named_measurement_after_reorder_and_rejects_removal() {
    let (mut state, key) = distribution();
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
        state.model.simulation.retained.runs[0].analyses[0]
            .family_metadata
            .as_mut()
    else {
        unreachable!()
    };
    variables.reverse();
    let prepared = prepare_retained_hardcopy_resolution(
        SourceReads::of(&state),
        &key,
        HardcopyScope::ActivePlotDocument,
    )
    .unwrap();
    let source = prepared.resolve_owned().unwrap();
    let HardcopySemanticDocument::Plot(plot) = source.semantic_document() else {
        unreachable!()
    };
    assert_eq!(plot.traces[0].label, "gain");
    let onscreen =
        crate::workbench::documents::result_document::active_histogram(state.results_reads())
            .unwrap();
    assert_eq!(onscreen.name, "gain");
    assert_eq!(onscreen.data_max, 2.0);
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
        state.model.simulation.retained.runs[0].analyses[0]
            .family_metadata
            .as_mut()
    else {
        unreachable!()
    };
    variables.remove(0);
    assert!(
        crate::workbench::documents::result_document::active_histogram(state.results_reads())
            .is_none()
    );
    assert!(
        prepare_retained_hardcopy_resolution(
            SourceReads::of(&state),
            &key,
            HardcopyScope::ActivePlotDocument
        )
        .is_err()
    );
}
