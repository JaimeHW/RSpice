//! Quick-view plots resolved from what a Results sheet is showing.
//!
//! Every builder here is a function of the retained analysis samples and the
//! pane's own declared presentation, and of nothing else.  None of them reads
//! a window, a framebuffer, or a transient viewer cache, so the same dataset
//! under the same presentation always resolves the same page.

use super::*;
use rspice_hardcopy_contract::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
use rspice_results::noise_spectrum::{
    RetainedNoiseReference, retained_noise_contributor, retained_noise_reference,
    retained_noise_waveform_is_renderable,
};
use rspice_results::result_digest::ResultDigestEncoding;
use rspice_results::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    family_metadata::AnalysisResultFamilyMetadata, run::SimulationRun, waveform::RetainedWaveform,
};
use rspice_vocabulary::product::{DatasetId, ObjectRevision, ProjectId, RunId};
use uuid::Uuid;

mod plane;
mod presentation;
use plane::{quick_nyquist_plot, quick_polar_plot, quick_smith_plot};
pub use presentation::{
    QuickFftSettings, QuickHistogramSettings, QuickSmithSettings, ResultsQuickViewPresentation,
};

/// A successful retained analysis borrowed from its terminal owning run.
/// This input grants no publication or execution authority.
pub struct RetainedQuickViewSource<'a, W> {
    dataset_id: DatasetId,
    run_id: RunId,
    /// The run's name as the sheet's bar shows it.
    run_label: &'a str,
    analysis: &'a AnalysisResult<W>,
    is_visible: fn(&W) -> bool,
}

impl<'a, W: AsRef<RetainedWaveform>> RetainedQuickViewSource<'a, W> {
    pub fn try_new<A: AsRef<AnalysisResult<W>>>(
        run: &'a SimulationRun<A>,
        analysis_id: u64,
        is_visible: fn(&W) -> bool,
    ) -> Result<Self, HardcopySourceError> {
        if !run.lifecycle.is_terminal() {
            return Err(HardcopySourceError::UnretainedResult(format!(
                "active dataset {} belongs to a non-terminal run",
                run.dataset_id,
            )));
        }
        let mut matches = run
            .analyses
            .iter()
            .map(AsRef::as_ref)
            .filter(|analysis| analysis.id == analysis_id);
        let analysis = matches.next().ok_or_else(|| {
            HardcopySourceError::UnretainedResult(format!(
                "analysis {analysis_id} is not retained in dataset {}",
                run.dataset_id,
            ))
        })?;
        if matches.next().is_some() {
            return Err(HardcopySourceError::AmbiguousRetainedAnalysis(analysis_id));
        }
        if !analysis.success {
            return Err(HardcopySourceError::UnretainedResult(format!(
                "active analysis {} did not complete successfully",
                analysis.id,
            )));
        }
        analysis
            .validate_retained_evidence()
            .map_err(HardcopySourceError::InvalidVisualizationSource)?;
        Ok(Self {
            dataset_id: run.dataset_id,
            run_id: run.run_id,
            run_label: &run.label,
            analysis,
            is_visible,
        })
    }
}

pub fn resolve_results_quick_view_parts<W: AsRef<RetainedWaveform>>(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    active: &RetainedQuickViewSource<'_, W>,
    presentation: &ResultsQuickViewPresentation,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    presentation.validate()?;
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let viewer = presentation.viewer;
    let semantic_document = match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => {
            HardcopySemanticDocument::Plot(quick_waveform_plot(
                active,
                viewer,
                &presentation.overlay.for_analysis(active.analysis.id),
            )?)
        }
        ResultViewer::SParameters => HardcopySemanticDocument::Plot(quick_network_plot(
            active,
            &presentation.overlay.for_analysis(active.analysis.id),
        )?),
        ResultViewer::Bode => HardcopySemanticDocument::Plot(quick_bode_plot(
            active,
            &presentation.overlay.for_analysis(active.analysis.id),
            presentation.bode_signal.as_deref(),
        )?),
        ResultViewer::Fft => HardcopySemanticDocument::Plot(quick_fft_plot(presentation, active)?),
        ResultViewer::HarmonicBalance => {
            HardcopySemanticDocument::Plot(quick_harmonic_balance_plot(active)?)
        }
        ResultViewer::PhaseNoise => HardcopySemanticDocument::Plot(quick_phase_noise_plot(active)?),
        ResultViewer::Eye => HardcopySemanticDocument::Plot(quick_eye_plot(presentation, active)?),
        ResultViewer::Hist => {
            HardcopySemanticDocument::Plot(quick_histogram_plot(presentation, active)?)
        }
        ResultViewer::Nyquist => HardcopySemanticDocument::Plot(quick_nyquist_plot(
            active,
            &presentation.overlay.for_analysis(active.analysis.id),
        )?),
        ResultViewer::Smith => {
            HardcopySemanticDocument::Plot(quick_smith_plot(active, &presentation.smith)?)
        }
        // The polar sheet's evidence is the same retained complex locus; the
        // printed figure states it on the coefficient plane, ringed at the
        // magnitudes its axis rules, where a page has no reader to rotate the
        // ruling for.
        ResultViewer::Polar => HardcopySemanticDocument::Plot(quick_polar_plot(active)?),
        ResultViewer::NoiseContrib => HardcopySemanticDocument::Plot(quick_noise_spectrum_plot(
            active,
            &presentation.overlay.for_analysis(active.analysis.id),
        )?),
        ResultViewer::Op
        | ResultViewer::NetworkMatrix
        | ResultViewer::Contribution
        | ResultViewer::TransferFunction
        | ResultViewer::Table
        | ResultViewer::Soa
        | ResultViewer::Optimization
        | ResultViewer::Sweep
        | ResultViewer::Events
        | ResultViewer::Scatter
        | ResultViewer::BoxViolin
        | ResultViewer::PoleZero
        | ResultViewer::Floquet => HardcopySemanticDocument::ResultSummary(Box::new(
            semantic_result_summary(viewer, active.analysis)?,
        )),
    };
    let digest = canonical_digest(
        b"rspice-hardcopy-results-quick-view-v2",
        &(
            active.dataset_id,
            active.run_id,
            active.analysis.id,
            active
                .analysis
                .result_data_ref()
                .digest(ResultDigestEncoding::CURRENT),
            viewer,
            &semantic_document,
        ),
    )?;
    let identity = quick_view_identity(
        &source_key,
        project_id,
        viewer,
        active.dataset_id,
        active.run_id,
        active.analysis,
        active.run_label,
    )?;
    let bounds = match &semantic_document {
        HardcopySemanticDocument::Plot(plot) => drawn_bounds(plot)?,
        // A summary takes as many pages as its tables run on to.
        HardcopySemanticDocument::ResultSummary(summary) => {
            use super::result_summary_layout::{SUMMARY_PAGE_WIDTH_UM, lay_out_summary};
            let height = lay_out_summary(summary).height();
            SemanticBounds::try_new(
                SemanticPoint::new(0, 0),
                SemanticPoint::new(
                    i64::try_from(SUMMARY_PAGE_WIDTH_UM)
                        .map_err(|_| HardcopySourceError::CoordinateOverflow)?,
                    i64::try_from(height).map_err(|_| HardcopySourceError::CoordinateOverflow)?,
                ),
            )?
        }
        _ => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    };
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        bounds,
    )
}

/// What a plot draws, so that its strips take the page: from its left edge,
/// where its values stand, to just past its frames and every word beside
/// them. A plot of no frames, a plane, keeps its whole square box.
fn drawn_bounds(plot: &SemanticPlot) -> Result<SemanticBounds, HardcopySourceError> {
    if plot.frames.is_empty() {
        return SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        );
    }
    let frames = plot.frames.iter();
    let captions = plot.captions.iter().map(|caption| caption.position.y_um);
    let top = frames
        .clone()
        .map(|frame| frame.minimum.y_um - 4_000)
        .chain(captions.clone().map(|y| y - 3_000))
        .min()
        .unwrap_or(0)
        .max(0);
    let bottom = frames
        .clone()
        .map(|frame| frame.maximum.y_um + 4_500)
        .chain(captions.map(|y| y + 1_500))
        .max()
        .unwrap_or(PLOT_HEIGHT_UM)
        .min(PLOT_HEIGHT_UM);
    let right = frames
        .map(|frame| frame.maximum.x_um + 3_000)
        .max()
        .unwrap_or(PLOT_WIDTH_UM)
        .min(PLOT_WIDTH_UM);
    SemanticBounds::try_new(
        SemanticPoint::new(0, top),
        SemanticPoint::new(right, bottom),
    )
}

fn quick_waveform_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    viewer: ResultViewer,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    let abscissa = waveform_abscissa_scale(active.analysis.analysis_type);
    let series = active
        .analysis
        .waveforms
        .iter()
        // The reader's per-trace override, not the dataset's flag alone: a
        // trace hidden on the sheet was still printed.
        .filter(|waveform| {
            overlay.trace_is_visible(&waveform.as_ref().name, (active.is_visible)(waveform))
        })
        .map(AsRef::as_ref)
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}",
                active.dataset_id, active.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            unit: printed_unit(waveform, active.analysis.analysis_type),
            points: printed_points(&waveform.x, &waveform.y, abscissa, overlay),
        })
        .collect();
    quick_plot_from_scaled_series(
        viewer,
        "Results",
        0,
        series,
        Some(overlay),
        &analysis_abscissa(active.analysis, abscissa),
        AxisScale::Linear,
    )
}

/// What the analysis's abscissa measures, ruled on `scale`: its quantity,
/// and the unit its coordinates are retained in.
fn analysis_abscissa<W>(analysis: &AnalysisResult<W>, scale: AxisScale) -> QuickAbscissa {
    let (quantity, unit, _, _) = analysis.analysis_type.axis_info();
    QuickAbscissa::new(
        quantity,
        analysis.waveform_coordinate_unit().unwrap_or(unit),
        scale,
    )
}

/// The unit a retained series reads in, as the sheet names it: the unit the
/// producer stated, else the unit its name's accessor reads (`I(R1)` is
/// amperes, `phase(V(out))` degrees, however wrapped), else the analysis's
/// own quantity.
fn printed_unit(waveform: &RetainedWaveform, analysis: AnalysisType) -> String {
    if let Some(unit) = &waveform.unit {
        return unit.clone();
    }
    let accessor = |name: &str, accessor: &str| {
        name.get(..accessor.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(accessor))
    };
    let mut name = waveform.name.trim_start();
    if accessor(name, "phase(") {
        return "°".to_owned();
    }
    for wrapper in ["re(", "im(", "mag(", "abs("] {
        if accessor(name, wrapper) {
            name = name[wrapper.len()..].trim_start();
        }
    }
    let name = name.strip_prefix('|').unwrap_or(name);
    if accessor(name, "i(") {
        "A"
    } else if accessor(name, "v(") {
        "V"
    } else if accessor(name, "p(") {
        "W"
    } else if analysis == AnalysisType::Pstb {
        ""
    } else if analysis.is_bode_response() {
        "dB"
    } else if matches!(
        analysis,
        AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise
    ) {
        "V^2/Hz"
    } else {
        "V"
    }
    .to_owned()
}

/// `y` over `x` as the page reads it for a sheet ruled on `scale`: over the
/// reader's window at the page's columns, as the sheet reads what it draws.
fn printed_points(
    x: &rspice_results::waveform::WaveformValues,
    y: &rspice_results::waveform::WaveformValues,
    scale: AxisScale,
    overlay: &RetainedQuickViewOverlay,
) -> Vec<(f64, f64)> {
    print_read(
        x,
        y,
        scale,
        overlay.window_x(),
        print_columns(PLOT_WIDTH_UM - 2 * PLOT_INSET_UM),
    )
}

/// The S-parameter sheet on paper: each coefficient it draws, in dB against
/// a decade frequency axis, as ADS prints dB(S(i,j)). The phase panes and
/// any other column are left off, since one ordinate cannot carry decibels
/// and degrees or ohms at once.
fn quick_network_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            overlay.trace_is_visible(&waveform.as_ref().name, (active.is_visible)(waveform))
        })
        .map(AsRef::as_ref)
        .filter_map(|waveform| {
            let coefficient = &waveform.complex.as_ref()?.source_name;
            rspice_results::network_matrix::trace_identity(coefficient)?;
            Some(QuickResultSeries {
                identity: format!(
                    "{}:{}:{}:{}:magnitude-db",
                    active.dataset_id, active.run_id, active.analysis.id, waveform.name
                ),
                label: format!("|{coefficient}| (dB)"),
                unit: "dB".to_owned(),
                points: printed_points(&waveform.x, &waveform.y, AxisScale::Logarithmic, overlay)
                    .into_iter()
                    .map(|(frequency, magnitude)| (frequency, 20.0 * magnitude.log10()))
                    .collect(),
            })
        })
        .collect::<Vec<_>>();
    if series.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "scattering coefficients",
        ));
    }
    quick_plot_from_scaled_series(
        ResultViewer::SParameters,
        "Results",
        0,
        series,
        Some(overlay),
        &analysis_abscissa(active.analysis, AxisScale::Logarithmic),
        AxisScale::Linear,
    )
}

/// How a retained sweep's abscissa is ruled.
///
/// One question, asked of the analysis rather than of the viewer, and
/// answered the way `waves::build_models` answers it for the sheet: every
/// frequency family is a decade axis, everything else is linear.
const fn waveform_abscissa_scale(analysis: AnalysisType) -> AxisScale {
    if analysis.is_frequency_sweep()
        || matches!(
            analysis,
            AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise
        )
    {
        AxisScale::Logarithmic
    } else {
        AxisScale::Linear
    }
}

fn quick_bode_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    overlay: &RetainedQuickViewOverlay,
    measured: Option<&str>,
) -> Result<SemanticPlot, HardcopySourceError> {
    use rspice_results::bode::retained::{bode_responses, response_signal};
    // Which traces make the response, from their names alone: the page reads
    // only the window it draws of them, never the whole sweep. It is the
    // response the card measures: the reader's choice while the analysis
    // still has it, else the one the sheet shows by default.
    let analysis_type = active.analysis.analysis_type;
    let waveforms = &active.analysis.waveforms;
    let measured = measured.filter(|signal| {
        bode_responses(analysis_type, waveforms)
            .iter()
            .any(|candidate| candidate == signal)
    });
    let Some(shape) = rspice_results::bode::retained::ac_bode_shape_for_analysis(
        analysis_type,
        waveforms,
        0,
        |waveform: &W| match measured {
            Some(signal) => {
                response_signal(analysis_type, &waveform.as_ref().name).as_deref() == Some(signal)
            }
            None => (active.is_visible)(waveform),
        },
    ) else {
        if active.analysis.analysis_type.is_raw_frequency_curve() {
            return quick_waveform_plot(active, ResultViewer::Bode, overlay);
        }
        return Err(HardcopySourceError::MissingViewerEvidence(
            "frequency response",
        ));
    };
    let magnitude = waveforms[shape.mag_index].as_ref();
    let signal = shape.signal;
    // A stability analysis retains its loop gain in decibels; every other
    // response retains a linear magnitude.
    let decibels = analysis_type == AnalysisType::Stb;
    let mut series = vec![QuickResultSeries {
        identity: format!(
            "{}:{}:{}:{signal}:magnitude-db",
            active.dataset_id, active.run_id, active.analysis.id
        ),
        label: format!("|{signal}| (dB)"),
        unit: "dB".to_owned(),
        points: printed_points(&magnitude.x, &magnitude.y, AxisScale::Logarithmic, overlay)
            .into_iter()
            .map(|(frequency, gain)| (frequency, if decibels { gain } else { 20.0 * gain.log10() }))
            .collect(),
    }];
    if let Some(phase) = shape
        .phase_index
        .and_then(|index| waveforms.get(index))
        .map(AsRef::as_ref)
    {
        series.push(QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{signal}:phase-deg",
                active.dataset_id, active.run_id, active.analysis.id
            ),
            label: format!("phase({signal}) (°)"),
            unit: "°".to_owned(),
            points: printed_points(&phase.x, &phase.y, AxisScale::Logarithmic, overlay),
        });
    }
    // Frequency is ruled in decades on the sheet, so the page rules it in
    // decades too. Magnitude in decibels and phase in degrees each get their
    // own strip and axis, as the sheet's two panes do.
    let keys = [
        (format!("|{signal}| (dB)"), format!("|{signal}|")),
        (
            format!("phase({signal}) (\u{b0})"),
            format!("phase({signal})"),
        ),
    ];
    let mut plot = quick_plot_from_scaled_series(
        ResultViewer::Bode,
        "Results",
        0,
        series,
        Some(overlay),
        &analysis_abscissa(active.analysis, AxisScale::Logarithmic),
        AxisScale::Linear,
    )?;
    keyed_as_the_sheet(&mut plot, &keys);
    Ok(plot)
}

/// Name each trace in the key as the sheet's legend names it, the unit
/// left to the axis: `keys` pairs a trace's label with its key.
fn keyed_as_the_sheet(plot: &mut SemanticPlot, keys: &[(String, String)]) {
    for trace in &mut plot.traces {
        trace.key = keys
            .iter()
            .find(|(label, _)| *label == trace.label)
            .map(|(_, key)| key.clone());
    }
}

fn quick_noise_spectrum_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    if active.analysis.success
        && rspice_results::noise_spectrum::qpnoise_is_renderable(active.analysis)
        && active.analysis.validate_retained_evidence().is_ok()
    {
        return quick_waveform_plot(active, ResultViewer::NoiseContrib, overlay);
    }
    if !rspice_results::noise_spectrum::ordinary_noise_spectrum_is_renderable(
        active.analysis.success,
        active.analysis.analysis_type,
        &active.analysis.waveforms,
        || {},
    ) {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "ordinary noise spectrum",
        ));
    }

    let input = active
        .analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .enumerate()
        .find(|(_, waveform)| {
            retained_noise_reference(&waveform.name) == Some(RetainedNoiseReference::Input)
                && retained_noise_waveform_is_renderable(waveform)
        });
    let (reference, anchor_index, anchor) = if let Some((index, waveform)) = input {
        (RetainedNoiseReference::Input, index, waveform)
    } else {
        let (index, waveform) = active
            .analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .enumerate()
            .find(|(_, waveform)| {
                retained_noise_reference(&waveform.name) == Some(RetainedNoiseReference::Output)
                    && retained_noise_waveform_is_renderable(waveform)
            })
            .ok_or(HardcopySourceError::MissingViewerEvidence(
                "ordinary noise spectrum",
            ))?;
        (RetainedNoiseReference::Output, index, waveform)
    };

    let source_waveforms = if reference == RetainedNoiseReference::Input {
        vec![(anchor_index, anchor)]
    } else {
        active
            .analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .enumerate()
            .filter(|(_, waveform)| {
                retained_noise_reference(&waveform.name) != Some(RetainedNoiseReference::Input)
                    && (retained_noise_reference(&waveform.name)
                        == Some(RetainedNoiseReference::Output)
                        || retained_noise_contributor(&waveform.name))
                    && retained_noise_waveform_is_renderable(waveform)
                    && waveform.x == anchor.x
            })
            .collect()
    };
    let series = source_waveforms
        .into_iter()
        .map(|(waveform_index, waveform)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:noise-amplitude-density:{waveform_index}",
                active.dataset_id, active.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            unit: match waveform.unit.as_deref() {
                Some("A²/Hz" | "A^2/Hz") => "nA/√Hz",
                Some("s²/Hz" | "s^2/Hz") => "ns/√Hz",
                _ => "nV/√Hz",
            }
            .to_owned(),
            points: printed_points(&waveform.x, &waveform.y, AxisScale::Logarithmic, overlay)
                .into_iter()
                .map(|(frequency, density)| (frequency, density.sqrt() * 1.0e9))
                .collect(),
        })
        .collect();
    // The ordinary-noise sheet sweeps frequency in decades like every other
    // frequency instrument. The density itself is plotted linearly there.
    quick_plot_from_scaled_series(
        ResultViewer::NoiseContrib,
        "Results",
        0,
        series,
        Some(overlay),
        &analysis_abscissa(active.analysis, AxisScale::Logarithmic),
        AxisScale::Linear,
    )
}

fn quick_harmonic_balance_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    if !rspice_results::harmonic_spectrum::analysis_is_renderable(
        active.analysis.success,
        active.analysis.analysis_type,
        &active.analysis.waveforms,
        || {},
    ) {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "harmonic-balance spectrum",
        ));
    }
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            (active.is_visible)(waveform)
                && rspice_results::harmonic_spectrum::spectrum_trace_is_renderable(
                    waveform.as_ref(),
                    || {},
                )
        })
        .map(AsRef::as_ref)
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:hb-coefficients",
                active.dataset_id, active.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.complex.as_ref().map_or_else(
                || waveform.name.clone(),
                |complex| complex.source_name.clone(),
            ),
            unit: printed_unit(waveform, active.analysis.analysis_type),
            points: printed_points(
                &waveform.x,
                &waveform.y,
                AxisScale::Linear,
                &RetainedQuickViewOverlay::default(),
            ),
        })
        .collect();
    quick_plot_from_series(
        ResultViewer::HarmonicBalance,
        "Results",
        0,
        series,
        None,
        &analysis_abscissa(active.analysis, AxisScale::Linear),
    )
}

fn quick_phase_noise_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    if !rspice_results::phase_noise::phase_noise_is_renderable(
        active.analysis.success,
        active.analysis.analysis_type,
        active.analysis.family_metadata.as_ref(),
        &active.analysis.waveforms,
        || {},
    ) {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "phase-noise spectrum",
        ));
    }
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            (active.is_visible)(waveform)
                && rspice_results::phase_noise::phase_noise_waveform_is_renderable(
                    waveform.as_ref(),
                    || {},
                )
        })
        .map(AsRef::as_ref)
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:phase-noise",
                active.dataset_id, active.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            unit: "dBc/Hz".to_owned(),
            points: printed_points(
                &waveform.x,
                &waveform.y,
                AxisScale::Logarithmic,
                &RetainedQuickViewOverlay::default(),
            ),
        })
        .collect();
    // Offset frequency in decades, phase noise in dBc/Hz: the two axes of
    // every phase-noise plot ever published.
    quick_plot_from_scaled_series(
        ResultViewer::PhaseNoise,
        "Results",
        0,
        series,
        None,
        &QuickAbscissa::new("Offset frequency", "Hz", AxisScale::Logarithmic),
        AxisScale::Decibels,
    )
}

fn quick_fft_plot<W: AsRef<RetainedWaveform>>(
    presentation: &ResultsQuickViewPresentation,
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft_selected_source.as_deref(),
        "FFT source waveform",
    )?;
    // Read as the spectrum sheet reads it: the record validated a block at
    // a time and only the rows of the time window held, so a long stored
    // transient prints rather than being read whole.
    let input = rspice_results::fft::pipeline::prepare_fft_input_from_columns(
        &waveform.name,
        &waveform.x,
        &waveform.y,
        presentation.fft_input_options(),
    )
    .map_err(|error| {
        HardcopySourceError::InvalidVisualizationSource(format!(
            "FFT input preparation failed: {error}"
        ))
    })?;
    let data = rspice_results::fft::data::FftData::from_time_domain_with_normalization(
        &waveform.name,
        &input.samples,
        input.sample_rate,
        presentation.fft_window(),
        presentation.fft_normalization(),
    )
    .map_err(|error| {
        HardcopySourceError::InvalidVisualizationSource(format!(
            "FFT spectrum construction failed: {error}"
        ))
    })?;
    // The spectrum sheet is a decibel instrument: it plots `magnitude_db`
    // against a reference-aware level unit, and the harmonic table beside it
    // is in dBc. The page took the linear magnitude instead, so a printed
    // spectrum showed one peak and a flat floor where the sheet showed a
    // noise floor sixty decibels down and every harmonic in it.
    let mut plot = quick_plot_from_scaled_series(
        ResultViewer::Fft,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:fft-db:{}",
                active.dataset_id, active.run_id, active.analysis.id, waveform.name, data.fft_size
            ),
            label: data.name.clone(),
            unit: spectrum_level_unit(&waveform.name, presentation.fft_normalization()).to_owned(),
            points: data
                .points
                .iter()
                .map(|point| (point.frequency, point.magnitude_db()))
                .collect(),
        }],
        None,
        &QuickAbscissa::new("Frequency", "Hz", AxisScale::Linear),
        AxisScale::Decibels,
    )?;
    plot.captions.push(SemanticPlotCaption {
        text: spectrum_settings(presentation, &data),
        position: SemanticPoint::new(PLOT_WIDTH_UM - PLOT_INSET_UM, PLOT_INSET_UM - 1_200),
        anchor: SemanticCaptionAnchor::End,
    });
    Ok(plot)
}

/// The settings a spectrum was taken with, as the sheet states them: its
/// window, with a Kaiser window's \u{3b2}, its normalisation, the record the
/// transform read and the resolution bandwidth that leaves.
fn spectrum_settings(
    presentation: &ResultsQuickViewPresentation,
    data: &rspice_results::fft::data::FftData,
) -> String {
    use rspice_results::fft::window::WindowFunction;
    use rspice_results::number_format::fmt_si_significant;
    let window = match presentation.fft_window() {
        WindowFunction::Kaiser(beta) => {
            format!("Kaiser \u{3b2} {}", fmt_si_significant(beta.get(), "", 4))
        }
        window => window.display_name().to_owned(),
    };
    let normalization = match presentation.fft_normalization() {
        rspice_results::fft::data::SpectrumNormalization::Peak => "peak",
        rspice_results::fft::data::SpectrumNormalization::Rms => "RMS",
    };
    let mut text = format!(
        "{window} \u{b7} {normalization} \u{b7} {} samples",
        data.fft_size
    );
    let bandwidth = data.resolution_bandwidth();
    if bandwidth.is_finite() && bandwidth > 0.0 {
        text.push_str(&format!(
            " \u{b7} RBW {}",
            fmt_si_significant(bandwidth, "Hz", 4)
        ));
    }
    text
}

/// The level unit a spectrum's decibels are referenced to, as the spectrum
/// sheet names it: a voltage is in dBV, a current in dB re 1 A, and anything
/// else in plain dB, at the peak or RMS the spectrum is normalised to.
fn spectrum_level_unit(
    source: &str,
    normalization: rspice_results::fft::data::SpectrumNormalization,
) -> &'static str {
    use rspice_results::fft::data::SpectrumNormalization::{Peak, Rms};
    let source = source.trim_start();
    let reads = |accessor: &str| {
        source
            .get(..accessor.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(accessor))
    };
    match (reads("v("), reads("i("), normalization) {
        (true, _, Peak) => "dBV pk",
        (true, _, Rms) => "dBV rms",
        (_, true, Peak) => "dB re 1 A pk",
        (_, true, Rms) => "dB re 1 A rms",
        (_, _, Peak) => "dB pk",
        (_, _, Rms) => "dB rms",
    }
}

fn quick_eye_plot<W: AsRef<RetainedWaveform>>(
    presentation: &ResultsQuickViewPresentation,
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft_selected_source.as_deref(),
        "eye source waveform",
    )?;
    let (time, values) = whole_pair(waveform)?;
    let bit_period = retained_eye_bit_period(&time, &values)?;
    let data = rspice_core::analysis::signal_integrity::EyeDataBuilder::new()
        .bit_period(bit_period)
        .ui_count(2)
        .skip_initial(2)
        .build(&time, &values);
    if data.traces.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence("eye diagram"));
    }
    let series = data
        .traces
        .iter()
        .enumerate()
        .map(|(index, trace)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:eye:{}:{index}",
                active.dataset_id,
                active.run_id,
                active.analysis.id,
                waveform.name,
                bit_period.to_bits()
            ),
            label: waveform.name.clone(),
            unit: printed_unit(waveform, active.analysis.analysis_type),
            points: trace
                .time
                .iter()
                .copied()
                .zip(trace.amplitude.iter().copied())
                .collect(),
        })
        .collect();
    // The folds are one acquisition, drawn in one colour and named once,
    // over the unit intervals the sheet rules the eye in.
    let mut plot = quick_plot_from_series(
        ResultViewer::Eye,
        "Results",
        0,
        series,
        None,
        &QuickAbscissa::new("Time", "UI", AxisScale::Linear),
    )?;
    for trace in &mut plot.traces {
        trace.palette_slot = Some(0);
    }
    Ok(plot)
}

fn quick_histogram_plot<W: AsRef<RetainedWaveform>>(
    presentation: &ResultsQuickViewPresentation,
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } =
        active.analysis.family_metadata.as_ref().ok_or(
            HardcopySourceError::MissingViewerEvidence("Monte Carlo family metadata"),
        )?
    else {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo family metadata",
        ));
    };
    let variable = rspice_results::histogram::measurement_index(
        presentation.histogram_measurement.as_deref(),
        presentation.histogram_selected,
        &variables
            .iter()
            .map(|variable| variable.name.as_str())
            .collect::<Vec<_>>(),
    )
    .and_then(|index| variables.get(index))
    .ok_or(HardcopySourceError::MissingViewerEvidence(
        "selected Monte Carlo variable",
    ))?;
    if variable.samples.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo samples",
        ));
    }
    let mut builder = rspice_results::histogram::HistogramBuilder::new()
        .name(&variable.name)
        .bin_count(presentation.histogram_bin_count.clamp(1, 1000));
    if presentation.histogram_custom_range {
        let minimum = presentation.histogram_custom_min;
        let maximum = presentation.histogram_custom_max;
        if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
            return Err(HardcopySourceError::InvalidResultRange);
        }
        builder = builder.range(minimum, maximum);
    }
    let histogram = builder.build(&variable.samples);
    let display = rspice_results::histogram::display::HistogramDisplay::new(
        &histogram,
        &variable.samples,
        presentation.histogram_mode(),
    )
    .map_err(HardcopySourceError::MissingViewerEvidence)?;
    let axis = rspice_results::histogram::display::hist_axis(&histogram);
    let (x0, x1) = presentation.histogram_x.unwrap_or((axis.x0, axis.x1));
    let (y0, y1) = presentation.histogram_y.unwrap_or((0.0, display.y_max()));
    if !x0.is_finite()
        || !x1.is_finite()
        || x0 >= x1
        || !(x1 - x0).is_finite()
        || !y0.is_finite()
        || !y1.is_finite()
        || y0 >= y1
        || !(y1 - y0).is_finite()
    {
        return Err(HardcopySourceError::InvalidResultRange);
    }
    let width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let (axis_ticks, mut captions) = plot_axes(
        AxisScale::Linear,
        AxisScale::Linear,
        &PlotFrame {
            x_minimum: x0,
            x_maximum: x1,
            y_minimum: y0,
            y_maximum: y1,
            x_span: x1 - x0,
            y_span: y1 - y0,
            plot_width: width,
            plot_height: height,
        },
    )?;
    captions.push(SemanticPlotCaption::new(
        format!(
            "{} [{}]; {} samples; {} below and {} above the bin range",
            display.mode.label(),
            display.mode.unit(),
            histogram.total_count,
            histogram.underflow,
            histogram.overflow
        ),
        SemanticPoint::new(PLOT_INSET_UM, 5_000),
    ));
    captions.push(SemanticPlotCaption {
        text: QuickAbscissa::new(&variable.name, "", AxisScale::Linear).caption(),
        position: SemanticPoint::new(PLOT_INSET_UM - 1_000, PLOT_INSET_UM + height + 2_600),
        anchor: SemanticCaptionAnchor::End,
    });
    let mut paths = Vec::new();
    for outline in display.paths(&histogram, x0, x1) {
        paths.extend(clipped_plot_paths(&outline, x0, x1, y0, y1, width, height)?);
    }
    let trace_id = stable_quick_trace_id(
        ResultViewer::Hist,
        0,
        &format!(
            "{}:{}:{}:monte-carlo:{}",
            active.dataset_id, active.run_id, active.analysis.id, variable.name,
        ),
    );
    Ok(SemanticPlot {
        viewer: ResultViewer::Hist,
        page_id: stable_page_id("Results"),
        pane_id: 0,
        x_scale: AxisScale::Linear,
        y_scale: AxisScale::Linear,
        axis_ticks,
        traces: vec![SemanticPlotTrace {
            trace_id,
            label: histogram.name.clone(),
            paths,
            source_samples: display
                .source_points(&histogram)
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
            companion: false,
            palette_slot: None,
            key: None,
        }],
        cursors: Vec::new(),
        markers: Vec::new(),
        annotations: Vec::new(),
        captions,
        frames: vec![SemanticPlotFrame {
            minimum: SemanticPoint::new(PLOT_INSET_UM, PLOT_INSET_UM),
            maximum: SemanticPoint::new(PLOT_INSET_UM + width, PLOT_INSET_UM + height),
        }],
        guides: Vec::new(),
    })
}

/// A source's two columns, each borrowed or read.
type ColumnPair<'a> = (std::borrow::Cow<'a, [f64]>, std::borrow::Cow<'a, [f64]>);

/// Both columns of an eye's source: the eye folds every sample of the record, as the eye
/// sheet does, so it reads them whole, and a record past the whole-read budget is refused.
fn whole_pair(waveform: &RetainedWaveform) -> Result<ColumnPair<'_>, HardcopySourceError> {
    let refused = |failure: rspice_results::waveform::ColumnReadFailure| {
        HardcopySourceError::InvalidVisualizationSource(format!(
            "{} could not be read: {failure}",
            waveform.name
        ))
    };
    Ok((
        waveform.x.read_whole().map_err(refused)?,
        waveform.y.read_whole().map_err(refused)?,
    ))
}

fn selected_retained_waveform<'a, W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'a, W>,
    preferred_name: Option<&str>,
    evidence: &'static str,
) -> Result<&'a RetainedWaveform, HardcopySourceError> {
    let mut candidates = active
        .analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter(|waveform| {
            waveform.x.len().min(waveform.y.len()) >= rspice_results::fft::pipeline::MIN_FFT_SAMPLES
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.name.cmp(&right.name));
    let selected = preferred_name
        .and_then(|name| {
            candidates
                .iter()
                .copied()
                .find(|waveform| waveform.name == name || waveform.name.eq_ignore_ascii_case(name))
                .or_else(|| {
                    let preferred_core = derived_waveform_source_core(name);
                    candidates.iter().copied().find(|waveform| {
                        derived_waveform_source_core(&waveform.name) == preferred_core
                    })
                })
        })
        .or_else(|| candidates.first().copied())
        .ok_or(HardcopySourceError::MissingViewerEvidence(evidence))?;
    let sample_count = selected.x.len().min(selected.y.len());
    if selected
        .x
        .samples(..sample_count)
        .chain(selected.y.samples(..sample_count))
        .any(|value| !value.is_finite())
    {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            selected.name.clone(),
        ));
    }
    Ok(selected)
}

fn derived_waveform_source_core(name: &str) -> String {
    let trimmed = name.trim().trim_matches('|');
    trimmed
        .strip_prefix("V(")
        .and_then(|value| value.strip_suffix(')'))
        .or_else(|| {
            trimmed
                .strip_prefix("I(")
                .and_then(|value| value.strip_suffix(')'))
        })
        .unwrap_or(trimmed)
        .trim()
        .to_ascii_lowercase()
}

fn retained_eye_bit_period(time: &[f64], values: &[f64]) -> Result<f64, HardcopySourceError> {
    let sample_count = time.len().min(values.len());
    if sample_count < 8 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let minimum = values
        .iter()
        .take(sample_count)
        .copied()
        .filter(|value| value.is_finite())
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))?;
    let maximum = values
        .iter()
        .take(sample_count)
        .copied()
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))?;
    if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let threshold = (minimum + maximum) * 0.5;
    let edges = rspice_core::analysis::signal_integrity::find_edges(
        &time[..sample_count],
        &values[..sample_count],
        threshold,
    );
    if edges.len() < 3 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let mut rising_times = edges
        .iter()
        .filter(|edge| edge.rising && edge.time.is_finite())
        .map(|edge| edge.time)
        .collect::<Vec<_>>();
    rising_times.sort_by(f64::total_cmp);
    let edge_times = if rising_times.len() >= 3 {
        rising_times
    } else {
        let mut all = edges
            .iter()
            .map(|edge| edge.time)
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        all.sort_by(f64::total_cmp);
        all
    };
    if edge_times.len() < 3 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let mut intervals = edge_times
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|interval| interval.is_finite() && *interval > 0.0)
        .collect::<Vec<_>>();
    if intervals.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    intervals.sort_by(f64::total_cmp);
    let period = intervals[intervals.len() / 2];
    if period.is_finite() && period > 0.0 {
        Ok(period)
    } else {
        Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))
    }
}

pub fn results_quick_view_identity<W: AsRef<RetainedWaveform>, A>(
    source_key: &str,
    project_id: ProjectId,
    viewer: ResultViewer,
    run: &SimulationRun<A>,
    analysis: &AnalysisResult<W>,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    quick_view_identity(
        source_key,
        project_id,
        viewer,
        run.dataset_id,
        run.run_id,
        analysis,
        &run.label,
    )
}

fn quick_view_identity<W: AsRef<RetainedWaveform>>(
    source_key: &str,
    project_id: ProjectId,
    viewer: ResultViewer,
    dataset_id: DatasetId,
    run_id: RunId,
    analysis: &AnalysisResult<W>,
    run_label: &str,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = source_key.as_bytes().to_vec();
    identity_name.extend_from_slice(viewer.label().as_bytes());
    identity_name.extend_from_slice(dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&analysis.id.to_be_bytes());
    identity_name.extend_from_slice(
        analysis
            .result_data_ref()
            .digest(ResultDigestEncoding::CURRENT)
            .as_bytes(),
    );
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        quick_view_title(viewer, run_label, analysis),
    )
}

/// What a printed sheet is called: the sheet, its run and its analysis,
/// as the sheet's own bar names them, e.g. "Waves · Run 1 · Transient".
fn quick_view_title<W>(
    viewer: ResultViewer,
    run_label: &str,
    analysis: &AnalysisResult<W>,
) -> String {
    let analysis_label = if analysis.label.trim().is_empty() {
        analysis.analysis_type.short_label()
    } else {
        analysis.label.trim()
    };
    [viewer.tab_label(), run_label.trim(), analysis_label]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" · ")
}

#[cfg(test)]
mod tests;
