//! Histogram publication, in the case that renders every display mode the
//! results feature's histogram takes.

use crate::workbench::ResultViewer;
use crate::workbench::app_state::viewer_capabilities::histogram_publication::{
    distribution, setup,
};
use crate::workbench::hardcopy_adapters::sources::{
    HardcopySemanticDocument, PreparedRetainedHardcopyResolution, SemanticAxisKind, SourceReads,
    prepare_retained_hardcopy_resolution,
};
use rspice_hardcopy::render::{HardcopyPublicationTimestamp, HardcopyRenderer};
use rspice_hardcopy_contract::{HardcopyPlan, HardcopyScope, OutputFormat};
use rspice_results::histogram::HistogramDisplayMode;

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn histogram_publication_renders_every_mode_after_worker_transfer() {
    let (mut state, key) = distribution();
    for mode in HistogramDisplayMode::ALL {
        state.results.analysis.histogram_state.mode = mode;
        state
            .results
            .results
            .session
            .plot_view_mut(ResultViewer::Hist, 0)
            .x = Some((-1.5, 1.5));
        let prepared = prepare_retained_hardcopy_resolution(
            SourceReads::of(&state),
            &key,
            HardcopyScope::ActivePlotDocument,
        )
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
                crate::workbench::hardcopy_adapters::tests::source_metadata(&source, "RSpice")
                    .unwrap();
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
