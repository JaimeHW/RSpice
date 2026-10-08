//! Exact active/overlay source projection into unit-aware waveform strips.

use super::{
    AnalysisPresentationKey, ComplexNumberDisplay, FamilyTraceStyle, FamilyTraceVisibilityKey,
    SharedWaveformValues, StripModel, StripTrace, SweepShape, TraceKind, XScale,
    analysis_default_unit, run_mixed_key, stable_hash,
};
use crate::derived::DerivedSeries;
use crate::waveform::{WaveformData, waveform_color};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::executed_deck::ExecutedDeckArchive;
use rspice_results::family_projection::{
    FamilyRenderGroup, FamilyRenderPlan, SourceSampleSelection,
};
use rspice_results::run::SimulationRun;
use rspice_results::visualization_document::AccessibleColorPalette;
use rspice_ui_kit::tokens::Tokens;
use std::collections::HashSet;
use std::sync::Arc;

fn unique_analysis<'a>(
    mut candidates: impl Iterator<Item = &'a AnalysisResult<WaveformData>>,
) -> Option<&'a AnalysisResult<WaveformData>> {
    let candidate = candidates.next()?;
    candidates.next().is_none().then_some(candidate)
}

/// Resolve the result in an overlay run that was produced by the same
/// prepared analysis instance. Kind/label inference is permitted only for
/// unambiguous legacy history, where neither side has prepared provenance.
fn matching_overlay_analysis<'a, A: AsRef<AnalysisResult<WaveformData>>>(
    analysis: &AnalysisResult<WaveformData>,
    overlay_run: &'a SimulationRun<A>,
) -> Option<&'a AnalysisResult<WaveformData>> {
    if let Some(source_instance_id) = analysis
        .provenance
        .as_ref()
        .map(|provenance| provenance.source_instance_id())
    {
        return overlay_run
            .find_analysis_by_source_instance(source_instance_id)
            .map(AsRef::as_ref)
            .filter(|candidate| candidate.analysis_type == analysis.analysis_type);
    }

    let legacy_candidates = || {
        overlay_run
            .analyses
            .iter()
            .map(AsRef::as_ref)
            .filter(|candidate| {
                candidate.provenance.is_none() && candidate.analysis_type == analysis.analysis_type
            })
    };
    unique_analysis(legacy_candidates().filter(|candidate| candidate.label == analysis.label))
        .or_else(|| unique_analysis(legacy_candidates()))
}

fn selected_series_pair(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    selection: Option<&SourceSampleSelection>,
) -> Option<(SharedWaveformValues, SharedWaveformValues)> {
    let Some(selection) = selection else {
        return Some((Arc::clone(x), Arc::clone(y)));
    };
    selected_series_pair_indices(x, y, &selection.source_indices)
}

fn selected_series_pair_indices(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    source_indices: &[usize],
) -> Option<(SharedWaveformValues, SharedWaveformValues)> {
    if x.len() != y.len() || source_indices.last().is_some_and(|index| *index >= x.len()) {
        return None;
    }
    let selected_x = source_indices.iter().map(|index| x[*index]).collect();
    let selected_y = source_indices.iter().map(|index| y[*index]).collect();
    Some((Arc::new(selected_x), Arc::new(selected_y)))
}

struct FamilyProjection<'a> {
    group: Option<&'a FamilyRenderGroup>,
    x: SharedWaveformValues,
    y: SharedWaveformValues,
}

fn projected_family_series<'a>(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    selection: Option<&'a SourceSampleSelection>,
) -> Vec<FamilyProjection<'a>> {
    let Some(selection) = selection else {
        return vec![FamilyProjection {
            group: None,
            x: Arc::clone(x),
            y: Arc::clone(y),
        }];
    };
    let groups = selection
        .family_render_plan()
        .map(|plan| plan.groups())
        .filter(|groups| !groups.is_empty());
    if let Some(groups) = groups {
        return groups
            .iter()
            .filter_map(|group| {
                selected_series_pair_indices(x, y, &group.source_indices).and_then(|(_, y)| {
                    (group.x_values.len() == y.len()).then(|| FamilyProjection {
                        group: Some(group),
                        x: Arc::new(group.x_values.clone()),
                        y,
                    })
                })
            })
            .collect();
    }
    selected_series_pair(x, y, Some(selection))
        .map(|(x, y)| vec![FamilyProjection { group: None, x, y }])
        .unwrap_or_default()
}

pub fn family_color(style: FamilyTraceStyle, fallback: egui::Color32) -> egui::Color32 {
    let Some(color) = style.color else {
        return fallback;
    };
    let palette: &[[u8; 3]] = match color.palette {
        AccessibleColorPalette::OkabeItoCategorical => &[
            [0x00, 0x72, 0xB2],
            [0xE6, 0x9F, 0x00],
            [0x00, 0x9E, 0x73],
            [0xD5, 0x5E, 0x00],
            [0x56, 0xB4, 0xE9],
            [0xCC, 0x79, 0xA7],
        ],
        AccessibleColorPalette::TolBrightCategorical => &[
            [0x44, 0x77, 0xAA],
            [0xEE, 0x66, 0x77],
            [0x22, 0x88, 0x33],
            [0xCC, 0xBB, 0x44],
            [0x66, 0xCC, 0xEE],
            [0xAA, 0x33, 0x77],
            [0xBB, 0xBB, 0xBB],
        ],
        AccessibleColorPalette::CividisSequential => &[
            [0x00, 0x20, 0x4C],
            [0x41, 0x4D, 0x6B],
            [0x7C, 0x7B, 0x78],
            [0xBA, 0xA8, 0x63],
            [0xFF, 0xE9, 0x45],
        ],
        AccessibleColorPalette::ViridisSequential => &[
            [0x44, 0x01, 0x54],
            [0x3B, 0x52, 0x8B],
            [0x21, 0x91, 0x8C],
            [0x5E, 0xC9, 0x62],
            [0xFD, 0xE7, 0x25],
        ],
    };
    let index = match color.palette {
        AccessibleColorPalette::CividisSequential | AccessibleColorPalette::ViridisSequential
            if color.category_count > 1 =>
        {
            color.ordinal.saturating_mul(palette.len() - 1) / (color.category_count - 1)
        }
        _ => color.ordinal % palette.len(),
    };
    let [red, green, blue] = palette[index.min(palette.len() - 1)];
    egui::Color32::from_rgb(red, green, blue)
}

/// What one DC-sweep analysis actually swept, named and given its unit.
///
/// Read out of the deck the run executed rather than out of the workspace's
/// current authoring, for the reason every other frozen-run reading is: a
/// sheet over a completed run has to describe that run. `None` whenever the
/// evidence does not settle it — a run whose decks were not retained, or one
/// with several sweeps whose points do not line up with its analyses — and
/// the analysis default stands.
fn swept_source_axis<A: AsRef<AnalysisResult<WaveformData>>>(
    executed_decks: &ExecutedDeckArchive,
    run: &SimulationRun<A>,
    analysis_index: usize,
) -> Option<(String, String)> {
    let source = if let Some(rspice_results::analysis_payload::AnalysisResultPayload::DcSweep {
        evidence,
    }) = &run.analyses.get(analysis_index)?.as_ref().result_payload
    {
        evidence.source.clone()
    } else {
        let deck = executed_decks.get(run.id)?;
        deck.points
            .get(analysis_index)
            .and_then(|point| dc_card_source(&point.deck))
            .or_else(|| {
                let sources: std::collections::BTreeSet<String> = deck
                    .points
                    .iter()
                    .filter_map(|point| dc_card_source(&point.deck))
                    .collect();
                (sources.len() == 1)
                    .then(|| sources.into_iter().next())
                    .flatten()
            })?
    };
    // SPICE names a source by what it is: the leading letter is the element
    // type, so it is also the quantity being swept. Anything else is a swept
    // parameter, which has no unit the sheet is entitled to invent.
    let unit = match source
        .chars()
        .next()
        .map(|first| first.to_ascii_uppercase())
    {
        Some('V') => "V",
        Some('I') => "A",
        _ => "",
    };
    Some((source, unit.to_owned()))
}

/// The source named by the first `.dc` card of a deck.
fn dc_card_source(deck: &str) -> Option<String> {
    deck.lines().find_map(|line| {
        let mut tokens = line.split_whitespace();
        tokens
            .next()
            .filter(|first| first.eq_ignore_ascii_case(".dc"))?;
        tokens.next().map(str::to_owned)
    })
}

struct ProjectionContext<'a> {
    analysis_key: AnalysisPresentationKey,
    run_id: u64,
    selection_key: u64,
    sample_selection: Option<&'a SourceSampleSelection>,
    hidden_family_traces: &'a HashSet<FamilyTraceVisibilityKey>,
    phase_continuous: bool,
}

struct TraceSource<'a> {
    waveform_index: usize,
    source_waveform_name: &'a str,
    base_name: &'a str,
    source_unit: Option<&'a str>,
    signal_color: egui::Color32,
    kind: TraceKind,
    source_x: &'a SharedWaveformValues,
    source_y: &'a SharedWaveformValues,
    visible: bool,
}

fn append_projected_traces(
    traces: &mut Vec<StripTrace>,
    derived: &mut DerivedSeries,
    context: &ProjectionContext<'_>,
    source: TraceSource<'_>,
) {
    let &ProjectionContext {
        analysis_key,
        run_id,
        selection_key,
        sample_selection,
        hidden_family_traces,
        phase_continuous,
    } = context;
    let TraceSource {
        waveform_index,
        source_waveform_name,
        base_name,
        source_unit,
        signal_color,
        kind,
        source_x,
        source_y,
        visible,
    } = source;
    for projection in projected_family_series(source_x, source_y, sample_selection) {
        let presentation_key = projection.group.map_or(0, |group| group.stable_key);
        let derived_key = stable_hash(&(analysis_key, source_waveform_name, kind as u8))
            ^ selection_key
            ^ presentation_key.rotate_left(23);
        let y = match kind {
            TraceKind::MagnitudeDb => derived.db(derived_key, &projection.y),
            TraceKind::NoiseDensity => derived.noise_density_nv(derived_key, &projection.y),
            TraceKind::PhaseDeg | TraceKind::PhaseRad => displayed_phase_series(
                derived,
                derived_key,
                &projection.y,
                phase_continuous,
                kind == TraceKind::PhaseRad,
            ),
            _ => projection.y,
        };
        let family_style = projection.group.map(|group| group.style);
        let family_visibility_key = projection.group.map(|group| {
            FamilyTraceVisibilityKey::new(
                analysis_key,
                group.stable_key,
                source_waveform_name,
                kind,
            )
        });
        let name = projection.group.map_or_else(
            || base_name.to_owned(),
            |group| format!("{base_name} · {}", group.label),
        );
        let shape = derived.shape_or(
            shape_key(
                analysis_key,
                source_waveform_name,
                kind,
                presentation_key,
                run_id,
                false,
            ),
            || SweepShape::of(&projection.x),
        );
        traces.push(StripTrace {
            waveform_index,
            source_waveform_name: source_waveform_name.to_owned(),
            base_name: base_name.to_owned(),
            name,
            unit: source_unit.map(str::to_owned),
            signal_color,
            color: family_style.map_or(signal_color, |style| family_color(style, signal_color)),
            x: projection.x,
            y,
            shape,
            kind,
            visible: visible
                && family_visibility_key.is_none_or(|key| !hidden_family_traces.contains(&key)),
            run_id,
            overlay: false,
            presentation_key,
            family_group_ordinal: projection.group.map(|group| group.ordinal),
            family_style,
            family_visibility_key,
        });
    }
}

/// Local projection choices; source selection and evidence status remain host-owned.
#[derive(Clone, Copy)]
pub struct ProjectionOptions<'a> {
    pub phase_continuous: bool,
    pub complex_display: ComplexNumberDisplay,
    pub selection: Option<&'a SourceSampleSelection>,
    pub hidden_family_traces: &'a HashSet<FamilyTraceVisibilityKey>,
}

/// Build strip models for every plottable analysis of the active run.
/// `phase_continuous` swaps phase traces to their unwrapped series.
///
/// This is the raw dataset projection: `visible` carries only the retained
/// data flag, so `x_range` is left unresolved. Callers pair this with
/// their visibility overrides and then resolve the visible X ranges.
pub fn build_models<A: AsRef<AnalysisResult<WaveformData>>, R: AsRef<SimulationRun<A>>>(
    display_runs: &[&R],
    executed_decks: &ExecutedDeckArchive,
    derived: &mut DerivedSeries,
    tokens: &Tokens,
    options: ProjectionOptions<'_>,
    mut incomplete_reason: impl FnMut(usize) -> Option<&'static str>,
) -> Vec<StripModel> {
    let ProjectionOptions {
        phase_continuous,
        complex_display,
        selection,
        hidden_family_traces,
    } = options;
    let Some((&run, overlay_runs)) = display_runs.split_first() else {
        return Vec::new();
    };
    let run = run.as_ref();
    let mut models = Vec::new();

    for (analysis_index, analysis) in run.analyses.iter().map(AsRef::as_ref).enumerate() {
        let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        if analysis.waveforms.is_empty() {
            continue;
        }
        let displays_cartesian_complex = (analysis.analysis_type == AnalysisType::Qpnoise
            || analysis.analysis_type.uses_complex_bode_projection()
            || analysis.analysis_type.is_time_domain())
            && complex_display == ComplexNumberDisplay::RealImaginary
            && analysis
                .waveforms
                .iter()
                .any(|waveform| waveform.complex.is_some());
        let sample_selection = selection.filter(|selection| {
            selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
        });
        let (mut x_scale, mut x_dimension_key, mut x_label, mut x_unit) =
            match analysis.analysis_type {
                AnalysisType::Qpac => (XScale::Linear, "qpac-probe-offset", "Probe offset", "Hz"),
                AnalysisType::Qpnoise => (
                    XScale::Linear,
                    "qpnoise-output-frequency",
                    "Output frequency",
                    "Hz",
                ),
                AnalysisType::Qpxf => (
                    XScale::Linear,
                    "qpxf-output-frequency",
                    "Output frequency",
                    "Hz",
                ),
                kind if kind.is_bode_response() || kind.is_raw_frequency_curve() => {
                    (XScale::Log10, "frequency", "f", "Hz")
                }
                AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise => {
                    (XScale::Log10, "frequency", "f", "Hz")
                }
                kind if kind.is_time_domain() => (XScale::Linear, "time", "t", "s"),
                AnalysisType::DcSweep => (XScale::Linear, "dc-sweep", "x", "V"),
                _ => (XScale::Linear, "x", "x", ""),
            };
        // Cartesian complex parts carry the source quantity, not the
        // magnitude projection's decibels, so that one case overrides the
        // analysis default.
        let y_unit = if displays_cartesian_complex {
            ""
        } else {
            analysis_default_unit(analysis.analysis_type)
        };
        // Exact retained DC evidence names the swept source. Historical
        // results can still use their frozen executed deck when available.
        let swept_source = (analysis.analysis_type == AnalysisType::DcSweep)
            .then(|| swept_source_axis(executed_decks, run, analysis_index))
            .flatten();
        if let Some((source, unit)) = swept_source.as_ref() {
            x_label = source;
            x_unit = unit;
        }
        // Imported coordinates are source data. A generic DC analysis does
        // not make an imported current or temperature sweep a voltage.
        if let Some(coordinate) = analysis.imported_coordinate() {
            x_label = &coordinate.name;
            x_unit = coordinate.unit.as_deref().unwrap_or("");
        }
        if let Some(axis) = sample_selection
            .and_then(SourceSampleSelection::family_render_plan)
            .map(FamilyRenderPlan::x_axis)
        {
            // Family policies currently persist no logarithmic scale. Exact
            // manifest coordinates therefore use the only truthful default.
            x_scale = XScale::Linear;
            x_dimension_key = &axis.dimension_key;
            x_label = &axis.label;
            x_unit = &axis.unit;
        }

        let mut traces = Vec::new();
        let selection_key = sample_selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(17);
        let projection = ProjectionContext {
            analysis_key,
            run_id: run.id,
            selection_key,
            sample_selection,
            hidden_family_traces,
            phase_continuous,
        };
        for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
            // STB's optional complex contour feeds the dedicated Nyquist
            // viewer. Its Bode sheet is the retained loop-gain pair only;
            // presenting the contour's linear magnitude beside an already-dB
            // loop gain would mix two different projections and units.
            if analysis.analysis_type == AnalysisType::Stb
                && !rspice_results::bode::retained::is_stb_bode_trace(&waveform.name)
            {
                continue;
            }
            let color = waveform_color(waveform, waveform_index, tokens);
            let is_phase = waveform.name.starts_with("phase(");
            let is_mag = waveform.name.starts_with('|');
            let is_time_complex_phase = analysis.analysis_type.is_time_domain()
                && is_phase
                && analysis.waveforms.iter().any(|candidate| {
                    candidate.complex.as_ref().is_some_and(|complex| {
                        waveform.name == format!("phase({})", complex.source_name)
                    })
                });
            if displays_cartesian_complex {
                if let Some(complex) = &waveform.complex {
                    for (kind, name, signal_color, y) in [
                        (
                            TraceKind::Real,
                            format!("re({})", complex.source_name),
                            color,
                            &complex.real,
                        ),
                        (
                            TraceKind::Imaginary,
                            format!("im({})", complex.source_name),
                            tokens.color.traces[(waveform_index + 1) % tokens.color.traces.len()],
                            &complex.imag,
                        ),
                    ] {
                        append_projected_traces(
                            &mut traces,
                            derived,
                            &projection,
                            TraceSource {
                                waveform_index,
                                source_waveform_name: &waveform.name,
                                base_name: &name,
                                source_unit: waveform.unit.as_deref(),
                                signal_color,
                                kind,
                                source_x: &waveform.x,
                                source_y: y,
                                visible: waveform.visible,
                            },
                        );
                    }
                    continue;
                }
                if is_phase
                    && analysis.waveforms.iter().any(|candidate| {
                        candidate.complex.as_ref().is_some_and(|complex| {
                            waveform.name == format!("phase({})", complex.source_name)
                        })
                    })
                {
                    continue;
                }
            }
            let kind = if analysis.analysis_type == AnalysisType::Hbnoise
                && waveform.name == "Noise figure (SSB)"
                && waveform.unit.as_deref() == Some("dB")
            {
                // Noise figure is already in decibels and has its own unit
                // pane; only power spectra receive the square-root mapping.
                TraceKind::Value
            } else if ((analysis.analysis_type.uses_complex_bode_projection()
                || analysis.analysis_type == AnalysisType::Qpnoise)
                && is_phase)
                || is_time_complex_phase
            {
                match complex_display {
                    ComplexNumberDisplay::MagnitudePhaseRadians => TraceKind::PhaseRad,
                    _ => TraceKind::PhaseDeg,
                }
            } else if analysis.analysis_type.uses_complex_bode_projection() && is_mag {
                TraceKind::MagnitudeDb
            } else if matches!(
                analysis.analysis_type,
                AnalysisType::Noise | AnalysisType::Hbnoise
            ) {
                // Voltage and current PSDs share the square-root projection;
                // the retained unit selects nV/√Hz or nA/√Hz.
                TraceKind::NoiseDensity
            } else {
                TraceKind::Value
            };
            append_projected_traces(
                &mut traces,
                derived,
                &projection,
                TraceSource {
                    waveform_index,
                    source_waveform_name: &waveform.name,
                    base_name: &waveform.name,
                    source_unit: waveform.unit.as_deref(),
                    signal_color: color,
                    kind,
                    source_x: &waveform.x,
                    source_y: &waveform.y,
                    visible: waveform.visible,
                },
            );
        }
        let signal_trace_count = traces.len();
        let overlay_signals = traces[..signal_trace_count]
            .iter()
            .map(|trace| {
                (
                    trace.source_waveform_name.clone(),
                    trace.base_name.clone(),
                    trace.name.clone(),
                    trace.unit.clone(),
                    trace.signal_color,
                    trace.color,
                    trace.kind,
                    trace.visible,
                    trace.presentation_key,
                    trace.family_group_ordinal,
                    trace.family_style,
                )
            })
            .collect::<Vec<_>>();

        // Overlay runs: match the exact prepared analysis instance and merge
        // traces by signal name. Signal owns hue —
        // overlay traces reuse the active trace's color and visibility —
        // run owns weight (applied at draw time).
        let mut overlaid_run_count = 0usize;
        let mut rejected_overlay_count = 0usize;
        for overlay_run in overlay_runs {
            let overlay_run = (*overlay_run).as_ref();
            let overlay_analysis = matching_overlay_analysis(analysis, overlay_run);
            let Some(overlay_analysis) = overlay_analysis else {
                continue;
            };
            if (analysis.imported_coordinate().is_some()
                || overlay_analysis.imported_coordinate().is_some())
                && analysis.waveform_coordinate_unit()
                    != overlay_analysis.waveform_coordinate_unit()
            {
                rejected_overlay_count += 1;
                continue;
            }

            let overlay_family_plan = match sample_selection
                .map(|selection| {
                    selection.overlay_render_plan(
                        overlay_analysis.analysis_type,
                        overlay_analysis.family_metadata.as_ref(),
                    )
                })
                .transpose()
            {
                Ok(plan) => plan.flatten(),
                Err(_) => {
                    rejected_overlay_count += 1;
                    continue;
                }
            };

            let mut contributed = false;
            let mut projected_overlay_traces = Vec::new();
            let mut incompatible_x = false;
            for (
                source_name,
                base_name,
                signal_name,
                signal_unit,
                signal_color,
                display_color,
                signal_kind,
                signal_visible,
                presentation_key,
                family_group_ordinal,
                family_style,
            ) in &overlay_signals
            {
                let Some((overlay_index, overlay_waveform)) = overlay_analysis
                    .waveforms
                    .iter()
                    .enumerate()
                    .find(|(_, waveform)| waveform.name == *source_name)
                else {
                    continue;
                };

                let base_key = (analysis_index as u64) << 32 | overlay_index as u64;
                let derived_key = run_mixed_key(base_key, overlay_run.id, true);
                let source_y = match *signal_kind {
                    TraceKind::MagnitudeDb => derived.db(derived_key, &overlay_waveform.y),
                    TraceKind::NoiseDensity => {
                        derived.noise_density_nv(derived_key, &overlay_waveform.y)
                    }
                    TraceKind::PhaseDeg | TraceKind::PhaseRad => displayed_phase_series(
                        derived,
                        derived_key,
                        &overlay_waveform.y,
                        phase_continuous,
                        *signal_kind == TraceKind::PhaseRad,
                    ),
                    TraceKind::Real => {
                        let Some(complex) = &overlay_waveform.complex else {
                            continue;
                        };
                        Arc::clone(&complex.real)
                    }
                    TraceKind::Imaginary => {
                        let Some(complex) = &overlay_waveform.complex else {
                            continue;
                        };
                        Arc::clone(&complex.imag)
                    }
                    _ => Arc::clone(&overlay_waveform.y),
                };
                let projected = if let Some(plan) = overlay_family_plan.as_ref() {
                    let Some(group) = family_group_ordinal
                        .and_then(|ordinal| plan.groups().get(ordinal))
                        .filter(|group| group.stable_key == *presentation_key)
                    else {
                        incompatible_x = true;
                        break;
                    };
                    let Some((_, y)) = selected_series_pair_indices(
                        &overlay_waveform.x,
                        &source_y,
                        &group.source_indices,
                    ) else {
                        incompatible_x = true;
                        break;
                    };
                    if y.len() != group.x_values.len() {
                        incompatible_x = true;
                        break;
                    }
                    (Arc::new(group.x_values.clone()), y)
                } else if let Some(selection) = sample_selection {
                    let Some(series) =
                        selected_series_pair(&overlay_waveform.x, &source_y, Some(selection))
                    else {
                        incompatible_x = true;
                        break;
                    };
                    series
                } else {
                    (Arc::clone(&overlay_waveform.x), source_y)
                };
                let shape = derived.shape_or(
                    shape_key(
                        analysis_key,
                        source_name,
                        *signal_kind,
                        *presentation_key,
                        overlay_run.id,
                        true,
                    ),
                    || SweepShape::of(&projected.0),
                );
                projected_overlay_traces.push(StripTrace {
                    waveform_index: overlay_index,
                    source_waveform_name: source_name.clone(),
                    base_name: base_name.clone(),
                    name: signal_name.clone(),
                    unit: signal_unit.clone(),
                    signal_color: *signal_color,
                    color: *display_color,
                    x: projected.0,
                    y: projected.1,
                    shape,
                    kind: *signal_kind,
                    visible: *signal_visible,
                    run_id: overlay_run.id,
                    overlay: true,
                    presentation_key: *presentation_key,
                    family_group_ordinal: *family_group_ordinal,
                    family_style: *family_style,
                    family_visibility_key: None,
                });
                contributed = true;
            }
            if incompatible_x {
                rejected_overlay_count += 1;
                continue;
            }
            if contributed {
                traces.extend(projected_overlay_traces);
                overlaid_run_count += 1;
            }
        }

        let mut subtitle = analysis.label.clone();
        if overlaid_run_count > 0 {
            subtitle = format!(
                "{subtitle} · +{overlaid_run_count} run{} overlaid",
                if overlaid_run_count == 1 { "" } else { "s" }
            );
        }
        if rejected_overlay_count > 0 {
            subtitle = format!(
                "{subtitle} · {rejected_overlay_count} incompatible overlay{} hidden",
                if rejected_overlay_count == 1 { "" } else { "s" }
            );
        }

        let mut model = StripModel {
            analysis_index,
            analysis_key,
            analysis_type: analysis.analysis_type,
            kind_tag: analysis.analysis_type.short_label().to_uppercase(),
            subtitle,
            incomplete: incomplete_reason(analysis_index),
            x_scale,
            x_dimension_key: x_dimension_key.to_owned(),
            x_label: x_label.to_owned(),
            x_unit: x_unit.to_owned(),
            y_unit,
            phase_continuous,
            signal_trace_count,
            traces,
            grid_ascending: false,
            x_range: None,
        };
        model.grid_ascending = super::grid_is_ascending(&model);
        models.push(model);
    }
    models
}

fn displayed_phase_series(
    derived: &mut DerivedSeries,
    key: u64,
    phase_degrees: &SharedWaveformValues,
    continuous: bool,
    radians: bool,
) -> SharedWaveformValues {
    let degrees = if continuous {
        derived.unwrapped(key, phase_degrees)
    } else {
        Arc::clone(phase_degrees)
    };
    if !radians {
        return degrees;
    }
    // Radian conversion is a cached presentation series. Stored samples stay
    // in their original degree representation for reproducibility/export.
    const RADIANS_KEY_BIT: u64 = 1 << 61;
    const CONTINUOUS_KEY_BIT: u64 = 1 << 60;
    derived.get_or(
        key ^ RADIANS_KEY_BIT ^ if continuous { CONTINUOUS_KEY_BIT } else { 0 },
        || Arc::new(degrees.iter().map(|value| value.to_radians()).collect()),
    )
}

/// Identity of one trace's X column for the sweep-shape memo.
///
/// The abscissa is projected before the ordinate is: a family group selects
/// its own exact rows, and an overlay run carries its own coordinates, so the
/// key has to name the group and the run as well as the signal. It is not
/// [`super::trace_key`], because that folds in the phase wrap/unwrap choice — which
/// changes Y and leaves X exactly where it was.
fn shape_key(
    analysis_key: AnalysisPresentationKey,
    source_waveform_name: &str,
    kind: TraceKind,
    presentation_key: u64,
    run_id: u64,
    overlay: bool,
) -> u64 {
    run_mixed_key(
        stable_hash(&(
            analysis_key,
            source_waveform_name,
            kind as u8,
            presentation_key,
            "x-shape",
        )),
        run_id,
        overlay,
    )
}
