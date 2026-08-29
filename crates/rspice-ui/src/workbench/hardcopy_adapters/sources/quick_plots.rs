//! Quick-view plots resolved from what a Results sheet is showing.
//!
//! Every builder here is a function of the retained analysis samples and the
//! pane's own declared presentation, and of nothing else.  None of them reads
//! a window, a framebuffer, or a transient viewer cache, so the same dataset
//! under the same presentation always resolves the same page.

use super::*;

pub(super) fn resolve_results_quick_view_parts(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    active: ActiveQuickResult<'_>,
    presentation: &ResultsQuickViewPresentation,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
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
        ResultViewer::Bode => HardcopySemanticDocument::Plot(quick_bode_plot(
            active,
            &presentation.overlay.for_analysis(active.analysis.id),
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
        ResultViewer::Nyquist => {
            HardcopySemanticDocument::Plot(quick_complex_plot(active, ResultViewer::Nyquist)?)
        }
        ResultViewer::Smith => {
            HardcopySemanticDocument::Plot(quick_complex_plot(active, ResultViewer::Smith)?)
        }
        ResultViewer::NoiseContrib => HardcopySemanticDocument::Plot(quick_noise_spectrum_plot(
            active,
            &presentation.overlay.for_analysis(active.analysis.id),
        )?),
        ResultViewer::Op
        | ResultViewer::Contribution
        | ResultViewer::TransferFunction
        | ResultViewer::Specs
        | ResultViewer::Table
        | ResultViewer::Soa
        | ResultViewer::Reliability
        | ResultViewer::Optimization
        | ResultViewer::Events
        | ResultViewer::PoleZero => HardcopySemanticDocument::ResultSummary(Box::new(
            semantic_result_summary(viewer, active.analysis)?,
        )),
        ResultViewer::Manifest => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                "dataset-native Manifest must resolve from its owning run".to_owned(),
            ));
        }
    };
    let digest = canonical_digest(
        b"rspice-hardcopy-results-quick-view-v2",
        &(
            active.run.dataset_id,
            active.run.run_id,
            active.analysis.id,
            active.analysis.result_data_digest(),
            viewer,
            &semantic_document,
        ),
    )?;
    let identity =
        results_quick_view_identity(&source_key, project_id, viewer, active.run, active.analysis)?;
    let bounds = match &semantic_document {
        HardcopySemanticDocument::Plot(_) => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
        _ => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    };
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        bounds,
    )
}

#[derive(Debug)]
pub(super) struct QuickResultSeries {
    pub(super) identity: String,
    pub(super) label: String,
    pub(super) points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ActiveQuickResult<'a> {
    pub(super) run: &'a SimulationRun,
    pub(super) analysis: &'a AnalysisResult,
}

#[cfg(test)]
pub(super) fn active_quick_result(
    state: &AppState,
    viewer: ResultViewer,
) -> Result<ActiveQuickResult<'_>, HardcopySourceError> {
    let run = active_terminal_run(state)?;
    let analysis_index = quick_result_analysis_index(state, run, viewer).ok_or_else(|| {
        HardcopySourceError::UnretainedResult(format!(
            "no retained analysis can provide exact evidence for {}",
            viewer.label()
        ))
    })?;
    let analysis = run.analyses.get(analysis_index).ok_or_else(|| {
        HardcopySourceError::UnretainedResult(format!(
            "active analysis index {analysis_index} is not retained in dataset {}",
            run.dataset_id
        ))
    })?;
    if !analysis.success {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "active analysis {} did not complete successfully",
            analysis.id
        )));
    }
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    Ok(ActiveQuickResult { run, analysis })
}

pub(super) fn quick_waveform_plot(
    active: ActiveQuickResult<'_>,
    viewer: ResultViewer,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        // The reader's per-trace override, not the dataset's flag alone: a
        // trace hidden on the sheet was still printed.
        .filter(|waveform| overlay.trace_is_visible(&waveform.name, waveform.visible))
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: waveform
                .x
                .iter()
                .copied()
                .zip(waveform.y.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_scaled_series(
        viewer,
        "Results",
        0,
        series,
        Some(overlay),
        waveform_abscissa_scale(active.analysis.analysis_type),
        AxisScale::Linear,
    )
}

/// How a retained sweep's abscissa is ruled.
///
/// One question, asked of the analysis rather than of the viewer, and
/// answered the way `waves::build_models` answers it for the sheet: every
/// frequency family is a decade axis, everything else is linear.
pub(super) const fn waveform_abscissa_scale(analysis: AnalysisType) -> AxisScale {
    if analysis.is_bode_response()
        || analysis.is_raw_frequency_curve()
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

fn quick_bode_plot(
    active: ActiveQuickResult<'_>,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    let Some(summary) = crate::state::ac_bode_summary_for_analysis(active.analysis, 0) else {
        if active.analysis.analysis_type.is_raw_frequency_curve() {
            return quick_waveform_plot(active, ResultViewer::Bode, overlay);
        }
        return Err(HardcopySourceError::MissingViewerEvidence(
            "frequency response",
        ));
    };
    let mut series = vec![QuickResultSeries {
        identity: format!(
            "{}:{}:{}:{}:magnitude-db",
            active.run.dataset_id, active.run.run_id, active.analysis.id, summary.signal
        ),
        label: format!("|{}| (dB)", summary.signal),
        points: summary
            .frequency
            .iter()
            .copied()
            .zip(summary.gain_db.iter().copied())
            .collect(),
    }];
    if let Some(phase) = summary.phase_deg {
        series.push(QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:phase-deg",
                active.run.dataset_id, active.run.run_id, active.analysis.id, summary.signal
            ),
            label: format!("phase({}) (°)", summary.signal),
            points: summary
                .frequency
                .iter()
                .copied()
                .zip(phase.iter().copied())
                .collect(),
        });
    }
    // Frequency is ruled in decades on the sheet, so the page rules it in
    // decades too. The ordinate is left linear: the magnitude series is in
    // decibels and the phase series in degrees, and one axis cannot honestly
    // claim to be either while it carries both.
    quick_plot_from_scaled_series(
        ResultViewer::Bode,
        "Results",
        0,
        series,
        Some(overlay),
        AxisScale::Logarithmic,
        AxisScale::Linear,
    )
}

fn quick_noise_spectrum_plot(
    active: ActiveQuickResult<'_>,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    if !ordinary_noise_spectrum_is_renderable(active.analysis) {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "ordinary noise spectrum",
        ));
    }

    let input = active
        .analysis
        .waveforms
        .iter()
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
            .enumerate()
            .filter(|(_, waveform)| {
                retained_noise_reference(&waveform.name) != Some(RetainedNoiseReference::Input)
                    && (retained_noise_reference(&waveform.name)
                        == Some(RetainedNoiseReference::Output)
                        || retained_noise_contributor(&waveform.name))
                    && retained_noise_waveform_is_renderable(waveform)
                    && waveform.x.as_slice() == anchor.x.as_slice()
            })
            .collect()
    };
    let series = source_waveforms
        .into_iter()
        .map(|(waveform_index, waveform)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:noise-amplitude-density:{waveform_index}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: waveform
                .x
                .iter()
                .copied()
                .zip(waveform.y.iter().map(|density| density.sqrt() * 1.0e9))
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
        AxisScale::Logarithmic,
        AxisScale::Linear,
    )
}

fn quick_harmonic_balance_plot(
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    if !crate::workbench::documents::result_document::harmonic_balance_analysis_is_renderable(
        active.analysis,
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
            waveform.visible
                && crate::workbench::documents::result_document::harmonic_balance_waveform_is_renderable(
                    waveform,
                )
        })
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:hb-coefficients",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform
                .complex
                .as_ref()
                .map_or_else(|| waveform.name.clone(), |complex| complex.source_name.clone()),
            points: waveform
                .x
                .iter()
                .copied()
                .zip(waveform.y.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(ResultViewer::HarmonicBalance, "Results", 0, series, None)
}

fn quick_phase_noise_plot(
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    if !crate::workbench::documents::result_document::phase_noise_analysis_is_renderable(
        active.analysis,
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
            waveform.visible
                && crate::workbench::documents::result_document::phase_noise_waveform_is_renderable(
                    waveform,
                )
        })
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:phase-noise",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: format!("{} - dBc/Hz", waveform.name),
            points: waveform
                .x
                .iter()
                .copied()
                .zip(waveform.y.iter().copied())
                .collect(),
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
        AxisScale::Logarithmic,
        AxisScale::Decibels,
    )
}

#[cfg(test)]
pub(super) fn active_terminal_run(state: &AppState) -> Result<&SimulationRun, HardcopySourceError> {
    let run = state.simulation.active_run().ok_or_else(|| {
        HardcopySourceError::UnretainedResult("no active result dataset is selected".to_owned())
    })?;
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "active dataset {} belongs to a non-terminal run",
            run.dataset_id
        )));
    }
    Ok(run)
}

pub(super) fn quick_fft_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft.selected_source.as_deref(),
        "FFT source waveform",
    )?;
    let input = crate::analysis::fft::prepare_fft_input_with_options(
        &waveform.name,
        &waveform.x,
        &waveform.y,
        presentation.fft.input_options_for_waveform(&waveform.x),
    )
    .map_err(|error| {
        HardcopySourceError::InvalidVisualizationSource(format!(
            "FFT input preparation failed: {error}"
        ))
    })?;
    let data = crate::analysis::fft::data::FftData::from_time_domain_with_normalization(
        &waveform.name,
        &input.samples,
        input.sample_rate,
        presentation.fft.window,
        presentation.fft.normalization,
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
    quick_plot_from_scaled_series(
        ResultViewer::Fft,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:fft-db:{}",
                active.run.dataset_id,
                active.run.run_id,
                active.analysis.id,
                waveform.name,
                data.fft_size
            ),
            label: data.name.clone(),
            points: data
                .points
                .iter()
                .map(|point| (point.frequency, point.magnitude_db()))
                .collect(),
        }],
        None,
        AxisScale::Linear,
        AxisScale::Decibels,
    )
}

pub(super) fn quick_eye_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft.selected_source.as_deref(),
        "eye source waveform",
    )?;
    let bit_period = retained_eye_bit_period(&waveform.x, &waveform.y)?;
    let data = crate::analysis::eye_diagram::EyeDataBuilder::new()
        .bit_period(bit_period)
        .ui_count(2)
        .skip_initial(2)
        .build(&waveform.x, &waveform.y);
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
                active.run.dataset_id,
                active.run.run_id,
                active.analysis.id,
                waveform.name,
                bit_period.to_bits()
            ),
            label: format!("Eye trace {}", index + 1),
            points: trace
                .time
                .iter()
                .copied()
                .zip(trace.amplitude.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(ResultViewer::Eye, "Results", 0, series, None)
}

pub(super) fn quick_histogram_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
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
    let variable = variables.get(presentation.histogram_selected).ok_or(
        HardcopySourceError::MissingViewerEvidence("selected Monte Carlo variable"),
    )?;
    if variable.samples.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo samples",
        ));
    }
    let mut builder = crate::analysis::HistogramBuilder::new()
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
    let ordinates = match presentation.histogram_mode {
        crate::analysis::HistogramDisplayMode::Count => histogram
            .bins
            .iter()
            .map(|bin| bin.count as f64)
            .collect::<Vec<_>>(),
        crate::analysis::HistogramDisplayMode::Pdf => histogram.pdf(),
        crate::analysis::HistogramDisplayMode::Cdf => histogram.cdf(),
        crate::analysis::HistogramDisplayMode::Percent => histogram
            .bins
            .iter()
            .map(|bin| {
                if histogram.total_count == 0 {
                    0.0
                } else {
                    bin.count as f64 * 100.0 / histogram.total_count as f64
                }
            })
            .collect(),
    };
    quick_plot_from_series(
        ResultViewer::Hist,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:monte-carlo:{}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, variable.name
            ),
            label: histogram.name.clone(),
            points: histogram
                .bins
                .iter()
                .zip(ordinates)
                .map(|(bin, ordinate)| (bin.center(), ordinate))
                .collect(),
        }],
        None,
    )
}

pub(super) fn quick_complex_plot(
    active: ActiveQuickResult<'_>,
    viewer: ResultViewer,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .filter_map(|waveform| waveform.complex.as_ref().map(|complex| (waveform, complex)))
        .map(|(waveform, complex)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:complex",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: complex
                .real
                .iter()
                .copied()
                .zip(complex.imag.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(viewer, "Results", 0, series, None)
}

pub(super) fn selected_retained_waveform<'a>(
    active: ActiveQuickResult<'a>,
    preferred_name: Option<&str>,
    evidence: &'static str,
) -> Result<&'a WaveformData, HardcopySourceError> {
    let mut candidates = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            waveform.x.len().min(waveform.y.len()) >= crate::analysis::fft::MIN_FFT_SAMPLES
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
        .iter()
        .take(sample_count)
        .chain(selected.y.iter().take(sample_count))
        .any(|value| !value.is_finite())
    {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            selected.name.clone(),
        ));
    }
    Ok(selected)
}

pub(super) fn derived_waveform_source_core(name: &str) -> String {
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

pub(super) fn retained_eye_bit_period(
    time: &[f64],
    values: &[f64],
) -> Result<f64, HardcopySourceError> {
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
    let edges = crate::analysis::eye_diagram::find_edges(
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

/// Build the printable plot for one sheet.
///
/// `overlay` is the reading the sheet was carrying — hidden traces already
/// applied by the caller, plus the cursors and markers this function places.
/// It is `None` for the sheets that compute their own abscissa: a marker
/// anchored in seconds has no position on a folded eye or a binned
/// distribution, and drawing it at one would be an invention.
pub(super) fn quick_plot_from_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
    overlay: Option<&RetainedQuickViewOverlay>,
) -> Result<SemanticPlot, HardcopySourceError> {
    quick_plot_from_scaled_series(
        viewer,
        page,
        pane_id,
        series,
        overlay,
        AxisScale::Linear,
        AxisScale::Linear,
    )
}

/// The same page, on axes that say how they map.
///
/// The geometry below is laid out in the axes' own space, so a decade of a
/// logarithmic sweep occupies the same width as every other decade. Retained
/// samples are untouched: they travel as the engine's own values, and only
/// the page coordinates move.
pub(super) fn quick_plot_from_scaled_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
    overlay: Option<&RetainedQuickViewOverlay>,
    x_scale: AxisScale,
    y_scale: AxisScale,
) -> Result<SemanticPlot, HardcopySourceError> {
    if series.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "visible plot series",
        ));
    }
    if series.iter().any(|series| {
        series.points.is_empty()
            || series
                .points
                .iter()
                .any(|(x, y)| !x.is_finite() || !y.is_finite())
    }) {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series".to_owned(),
        ));
    }
    // A logarithmic axis has no position for a non-positive value, so those
    // samples are dropped rather than clamped — exactly as the sheet drops
    // them. A series that is entirely non-positive on a log axis has nothing
    // the page can show.
    let projected = series
        .iter()
        .map(|series| {
            series
                .points
                .iter()
                .filter_map(|&(x, y)| Some((project(x_scale, x)?, project(y_scale, y)?)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if projected.iter().any(Vec::is_empty) {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series has no sample its axis can place".to_owned(),
        ));
    }
    let extreme = |select: fn(&(f64, f64)) -> f64, pick: fn(&f64, &f64) -> std::cmp::Ordering| {
        projected
            .iter()
            .flat_map(|points| points.iter().map(select))
            .min_by(pick)
            .ok_or(HardcopySourceError::InvalidResultRange)
    };
    let x_minimum = extreme(|point| point.0, f64::total_cmp)?;
    let x_maximum = extreme(|point| point.0, |left, right| right.total_cmp(left))?;
    let y_minimum = extreme(|point| point.1, f64::total_cmp)?;
    let y_maximum = extreme(|point| point.1, |left, right| right.total_cmp(left))?;
    // The window the reader had pinned is part of the sheet they captured,
    // exactly as the hidden traces and the placed cursors are.
    let data_bounds = (x_minimum, x_maximum, y_minimum, y_maximum);
    let (x_minimum, x_maximum, y_minimum, y_maximum) = overlay.map_or(data_bounds, |overlay| {
        overlay.framed_bounds(x_scale, y_scale, data_bounds)
    });
    let (x_minimum, x_maximum) = nondegenerate_range(x_minimum, x_maximum);
    let (y_minimum, y_maximum) = nondegenerate_range(y_minimum, y_maximum);
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let frame = PlotFrame {
        x_minimum,
        x_maximum,
        y_minimum,
        y_maximum,
        x_span: x_maximum - x_minimum,
        y_span: y_maximum - y_minimum,
        plot_width,
        plot_height,
    };
    let axis_ticks = plot_axis_ticks(x_scale, &frame)?;
    let (cursors, markers) = overlay.map_or_else(
        || Ok((Vec::new(), Vec::new())),
        |overlay| resolved_overlay_geometry(viewer, overlay, &series, x_scale, y_scale, &frame),
    )?;
    let mut trace_ids = std::collections::HashSet::new();
    let traces = series
        .iter()
        .zip(projected.iter())
        .enumerate()
        .map(|(index, (series, points))| {
            let trace_id = stable_quick_trace_id(viewer, index, &series.identity);
            if !trace_ids.insert(trace_id) {
                return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
            }
            Ok(SemanticPlotTrace {
                trace_id,
                label: series.label.clone(),
                paths: clipped_plot_paths(
                    points,
                    x_minimum,
                    x_maximum,
                    y_minimum,
                    y_maximum,
                    plot_width,
                    plot_height,
                )?,
                source_samples: series
                    .points
                    .iter()
                    .map(|(x, y)| (x.to_bits(), y.to_bits()))
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    Ok(SemanticPlot {
        viewer,
        page_id: stable_page_id(page),
        pane_id,
        x_scale,
        y_scale,
        axis_ticks,
        traces,
        cursors,
        markers,
        annotations: Vec::new(),
    })
}

pub(super) fn results_quick_view_identity(
    source_key: &str,
    project_id: ProjectId,
    viewer: ResultViewer,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = source_key.as_bytes().to_vec();
    identity_name.extend_from_slice(viewer.label().as_bytes());
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run.run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&analysis.id.to_be_bytes());
    identity_name.extend_from_slice(analysis.result_data_digest().as_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results - {}", viewer.label()),
    )
}

pub(super) fn stable_quick_trace_id(viewer: ResultViewer, index: usize, identity: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-hardcopy-results-trace-v1");
    hasher.update(viewer.label().as_bytes());
    hasher.update((index as u64).to_be_bytes());
    hasher.update(identity.as_bytes());
    let bytes: [u8; 8] = hasher.finalize()[..8]
        .try_into()
        .expect("SHA-256 prefix has fixed length");
    u64::from_be_bytes(bytes)
}

pub(super) const fn is_curve_viewer(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
            | ResultViewer::Fft
            | ResultViewer::HarmonicBalance
            | ResultViewer::PhaseNoise
            | ResultViewer::Eye
            | ResultViewer::Hist
            | ResultViewer::Nyquist
            | ResultViewer::Smith
    )
}
