//! A printed trace is drawn at its device's resolution.
//!
//! One test in its own binary, so the allocator it counts with sees only
//! this print.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use rspice_hardcopy::render::{HardcopyRenderer, HardcopySceneMetadata};
use rspice_hardcopy::sources::{
    HardcopySemanticDocument, ResolvedHardcopyDocument, ResultsQuickViewPresentation,
    RetainedQuickViewSource, resolve_results_quick_view_parts,
};
use rspice_hardcopy_contract::{
    BackgroundMode, Bleed, ColorMapping, DecorationSetup, FontPolicy, HardcopyPlan, HardcopyPlanId,
    HardcopyScope, HardcopySetup, Length, Orientation, OutputFormat, PageMargins, PaperSize,
    PhysicalPageSetup, RenderSetup, RenderTarget, ScaleMode, StandardPaper, TilingMode,
    TilingSetup, Watermark,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::run::{SimulationRun, SimulationRunLifecycle};
use rspice_results::waveform::RetainedWaveform;
use rspice_vocabulary::product::{DatasetId, ProjectId, RunId};

struct Counting;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            PEAK.fetch_max(live, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let moved = unsafe { System.realloc(pointer, layout, size) };
        if !moved.is_null() {
            if size >= layout.size() {
                let live =
                    LIVE.fetch_add(size - layout.size(), Ordering::Relaxed) + size - layout.size();
                PEAK.fetch_max(live, Ordering::Relaxed);
            } else {
                LIVE.fetch_sub(layout.size() - size, Ordering::Relaxed);
            }
        }
        moved
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// The most memory held at once since the last call, above what was held
/// when it was made.
fn peak_since(baseline: usize) -> usize {
    PEAK.load(Ordering::Relaxed).saturating_sub(baseline)
}

fn reset_peak() -> usize {
    let live = LIVE.load(Ordering::Relaxed);
    PEAK.store(live, Ordering::Relaxed);
    live
}

const POINTS: usize = 1_000_000;
const MEGABYTE: usize = 1 << 20;

fn million_point_run() -> SimulationRun {
    let time = (0..POINTS)
        .map(|index| index as f64 * 1.0e-9)
        .collect::<Vec<_>>();
    // A carrier with a fast ripple on it: every column holds a real extreme.
    let value = time
        .iter()
        .map(|t| (t * 2.0e5).sin() + 0.05 * (t * 7.3e8).sin())
        .collect::<Vec<_>>();
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0)
        .with_waveforms(vec![RetainedWaveform::new("V(out)", time, value)]);
    let mut run: SimulationRun = SimulationRun::from_persisted_identity(
        1,
        "Run 1".to_owned(),
        0.0,
        RunId::new(),
        DatasetId::new(),
        None,
        None,
    );
    run.analyses.push(analysis);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run
}

fn waves() -> ResultsQuickViewPresentation {
    serde_json::from_str(
        r#"{
        "viewer": "Waves", "fft_selected_source": null,
        "fft_normalization": "peak", "fft_window": "hanning",
        "fft_input_fidelity": "reference", "fft_time_window_auto": true,
        "fft_time_window_start": 0.0, "fft_time_window_end": 1.0,
        "fft_sample_count_auto": true, "fft_sample_count": 4096,
        "histogram_selected": 0, "histogram_bin_count": 20,
        "histogram_custom_range": false, "histogram_custom_min": 0.0,
        "histogram_custom_max": 1.0, "histogram_mode": "count"
    }"#,
    )
    .expect("the presentation is well formed")
}

fn plan(source: &ResolvedHardcopyDocument, format: OutputFormat) -> HardcopyPlan {
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::FitPrintableArea,
        TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            format,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(format.is_vector(), format.is_vector()),
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(true, true, true, Watermark::None).unwrap(),
        source.default_print_mapping().clone(),
    )
    .unwrap();
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(uuid::Uuid::from_u128(4)).unwrap(),
        source.authority().clone(),
        setup,
        source.content_extent(),
    )
    .unwrap()
}

/// The x of every point a PDF page's paths visit, in points.
fn pdf_path_xs(bytes: &[u8]) -> Vec<f32> {
    let document = lopdf::Document::load_mem(bytes).expect("the PDF parses");
    let mut paths: Vec<Vec<f32>> = Vec::new();
    for page in document.get_pages().values() {
        let content = document.get_page_content(*page);
        let operations = lopdf::content::Content::decode(&content)
            .expect("the content decodes")
            .operations;
        for operation in operations {
            let number = |index: usize| {
                operation
                    .operands
                    .get(index)
                    .and_then(|operand| operand.as_float().ok())
            };
            match operation.operator.as_str() {
                "m" => paths.push(number(0).into_iter().collect()),
                "l" => paths
                    .last_mut()
                    .into_iter()
                    .for_each(|path| path.extend(number(0))),
                "c" => paths
                    .last_mut()
                    .into_iter()
                    .for_each(|path| path.extend(number(4))),
                _ => {}
            }
        }
    }
    // The trace is the one long path; rules and the frame are a few points.
    paths.into_iter().max_by_key(Vec::len).unwrap_or_default()
}

/// A million-point transient prints at 300 dpi in a fraction of the memory
/// its samples alone would take to draw one by one, and its PDF and SVG
/// carry a few points for each column a 600 dpi printer can draw.
#[test]
fn a_million_point_trace_prints_at_its_devices_resolution() {
    let run = million_point_run();
    let active = RetainedQuickViewSource::try_new(&run, 1, |_| true).unwrap();
    let presentation = waves();

    let baseline = reset_peak();
    let resolved = resolve_results_quick_view_parts(
        "print-resolution".to_owned(),
        ProjectId::new(),
        HardcopyScope::ActivePlotDocument,
        &active,
        &presentation,
    )
    .expect("a million-point trace resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("a transient prints as a plot");
    };
    let drawn = plot.traces[0].paths.iter().map(Vec::len).sum::<usize>();
    assert!(
        drawn <= 4 * 5_400,
        "the page keeps {drawn} of {POINTS} points, more than four for each of its columns"
    );

    let png = HardcopyRenderer::render_resolved(
        &plan(&resolved, OutputFormat::Png { dpi: 300 }),
        &resolved,
        HardcopySceneMetadata::try_new("V(out)", "RSpice").unwrap(),
    )
    .expect("the page prints at 300 dpi");
    drop(png);
    let peak = peak_since(baseline);
    eprintln!("peak while printing at 300 dpi: {} MB", peak / MEGABYTE);
    assert!(
        peak < 200 * MEGABYTE,
        "printing at 300 dpi held {} MB at once",
        peak / MEGABYTE
    );

    let pdf = HardcopyRenderer::render_resolved(
        &plan(&resolved, OutputFormat::PdfVector),
        &resolved,
        HardcopySceneMetadata::try_new("V(out)", "RSpice").unwrap(),
    )
    .expect("the page prints as a PDF");
    let xs = pdf_path_xs(pdf.single_part().unwrap().bytes());
    let span = xs.iter().copied().fold(f32::INFINITY, f32::min)
        ..xs.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    assert!(
        span.end - span.start > 500.0,
        "the trace spans the page in points: {span:?}"
    );
    let mut per_column = std::collections::BTreeMap::<i64, usize>::new();
    for x in &xs {
        *per_column
            .entry((f64::from(*x) * 600.0 / 72.0).floor() as i64)
            .or_default() += 1;
    }
    let busiest = per_column.values().copied().max().unwrap_or(0);
    assert!(
        busiest <= 8,
        "a 600 dpi column of the PDF holds {busiest} path points"
    );

    let svg = HardcopyRenderer::render_resolved(
        &plan(&resolved, OutputFormat::SvgVector),
        &resolved,
        HardcopySceneMetadata::try_new("V(out)", "RSpice").unwrap(),
    )
    .expect("the page prints as an SVG");
    let text = String::from_utf8_lossy(svg.single_part().unwrap().bytes()).into_owned();
    let points = text
        .split("points=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap_or("").split(' ').count())
        .max()
        .unwrap_or(0);
    assert!(
        points <= 4 * 5_400,
        "the SVG's longest line carries {points} points"
    );
}
