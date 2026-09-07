//! Distribution selection and publication through the actual worker and renderer.

use super::AppState;
use crate::analysis::HistogramDisplayMode;
use crate::hardcopy::{
    BackgroundMode, ColorMapping, FontPolicy, HardcopyPlan, HardcopyScope, HardcopySetup,
    OutputFormat, RenderSetup, RenderTarget,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, MonteCarloVariableMetadata,
    SimulationRunLifecycle, SimulationRunProvenance,
};
use crate::workbench::ResultViewer;
use crate::workbench::hardcopy_adapters::render::{
    HardcopyPublicationTimestamp, HardcopyRenderer, HardcopySceneMetadata,
};
use crate::workbench::hardcopy_adapters::sources::{
    HardcopySemanticDocument, PreparedRetainedHardcopyResolution, SemanticAxisKind,
    prepare_retained_hardcopy_resolution,
};
use crate::workbench::state::WorkspaceDocumentId;

fn distribution() -> (AppState, String) {
    let mut state = AppState::default();
    let run = state.simulation.start_run();
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
                        name: "offset".to_owned(),
                        samples: vec![99.0; 5],
                        mean: 99.0,
                        std_dev: 0.0,
                        min: 99.0,
                        max: 99.0,
                    },
                    MonteCarloVariableMetadata {
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
    state.simulation.complete_run();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset));
    state.analysis.histogram_state.selected = Some("gain".to_owned());
    state.analysis.histogram_state.bin_count = 5;
    state.ui.results.viewer = ResultViewer::Hist;
    let key = format!(
        "project:{}:result-dataset:{}",
        state.workspace.project.id().as_uuid(),
        dataset
    );
    (state, key)
}

fn setup(format: OutputFormat, mapping: &crate::hardcopy::PrintMappingTable) -> HardcopySetup {
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
fn histogram_publication_renders_every_mode_after_worker_transfer() {
    let (mut state, key) = distribution();
    for mode in HistogramDisplayMode::ALL {
        state.analysis.histogram_state.mode = mode;
        state.ui.results.plot_view_mut(ResultViewer::Hist, 0).x = Some((-1.5, 1.5));
        let prepared =
            prepare_retained_hardcopy_resolution(&state, &key, HardcopyScope::ActivePlotDocument)
                .unwrap();
        let bytes = prepared.into_worker_snapshot_json().unwrap();
        let restored =
            PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&bytes).unwrap();
        let source = restored.resolve_owned().unwrap();
        let HardcopySemanticDocument::Plot(plot) = source.semantic_document() else {
            panic!("expected distribution")
        };
        assert_eq!(plot.traces[0].label, "gain");
        assert!(
            plot.captions
                .iter()
                .any(|caption| caption.text.contains(mode.label()))
        );
        for axis in [SemanticAxisKind::Horizontal, SemanticAxisKind::Vertical] {
            assert!(
                plot.axis_ticks
                    .iter()
                    .any(|tick| tick.axis == axis && !tick.label.is_empty())
            );
        }
        for (extension, format) in [
            ("svg", OutputFormat::SvgVector),
            ("pdf", OutputFormat::PdfVector),
            ("pdfa.pdf", OutputFormat::PdfA),
            ("png", OutputFormat::Png { dpi: 96 }),
            ("tiff", OutputFormat::Tiff { dpi: 96 }),
            ("html", OutputFormat::BrowserPrintDocument),
        ] {
            let plan = HardcopyPlan::compile(
                source.authority().clone(),
                setup(format, source.default_print_mapping()),
                source.content_extent(),
            )
            .unwrap();
            let mut metadata =
                HardcopySceneMetadata::for_resolved_source(&source, "RSpice").unwrap();
            metadata.set_publication_timestamp(
                HardcopyPublicationTimestamp::from_unix_seconds(1_788_761_600).unwrap(),
            );
            let output = HardcopyRenderer::render_resolved(&plan, &source, metadata)
                .unwrap_or_else(|error| panic!("{mode:?} {format:?}: {error}"));
            let bytes = output.single_part().unwrap().bytes();
            assert!(!bytes.is_empty());
            if format == OutputFormat::SvgVector {
                let svg = std::str::from_utf8(bytes).unwrap();
                assert!(svg.contains(mode.label()));
                assert!(svg.contains("text-anchor=\"end\""));
            }
            if matches!(format, OutputFormat::PdfVector | OutputFormat::PdfA) {
                assert_eq!(
                    lopdf::Document::load_mem(bytes).unwrap().get_pages().len(),
                    1
                );
            }
            if let Ok(directory) = std::env::var("RSPICE_UI_HISTOGRAM_ARTIFACT_DIR") {
                let path = std::path::Path::new(&directory).join(format!("{mode:?}.{extension}"));
                std::fs::write(path, bytes).unwrap();
            }
        }
    }
}

#[test]
fn histogram_publication_keeps_the_named_measurement_after_reorder_and_rejects_removal() {
    let (mut state, key) = distribution();
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    variables.reverse();
    let prepared =
        prepare_retained_hardcopy_resolution(&state, &key, HardcopyScope::ActivePlotDocument)
            .unwrap();
    let source = prepared.resolve_owned().unwrap();
    let HardcopySemanticDocument::Plot(plot) = source.semantic_document() else {
        unreachable!()
    };
    assert_eq!(plot.traces[0].label, "gain");
    let onscreen = crate::workbench::documents::result_document::active_histogram(&state).unwrap();
    assert_eq!(onscreen.name, "gain");
    assert_eq!(onscreen.data_max, 2.0);
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    variables.remove(0);
    assert!(crate::workbench::documents::result_document::active_histogram(&state).is_none());
    assert!(
        prepare_retained_hardcopy_resolution(&state, &key, HardcopyScope::ActivePlotDocument)
            .is_err()
    );
}
