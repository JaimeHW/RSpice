//! PHASE NOISE — retained PNOISE/QPNOISE phase-noise spectra.
//!
//! A periodic-noise result can also contain output- or input-referred noise.
//! Those quantities are not phase noise, even when their analysis kind is
//! PNOISE.  This view therefore requires an explicitly phase-noise-labelled
//! retained trace plus typed periodic-noise quantity metadata before it
//! presents the data as `L(f)` in dBc/Hz. Integrated-jitter and spur values
//! have no typed retained representation yet, so the inspector reports their
//! absence rather than estimating them.

use std::sync::Arc;

use egui::Ui;

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, PeriodicNoiseOutputQuantity,
    SharedWaveformValues, WaveformData,
};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::{self, LegendChip};
use super::well_hint;

/// Exactly one phase-noise waveform selected from the active immutable run.
struct PhaseNoiseModel {
    analysis_index: usize,
    waveform_index: usize,
    label: String,
    source: String,
    carrier_frequency_hz: f64,
    offset_hz: SharedWaveformValues,
    level_dbc_per_hz: SharedWaveformValues,
}

fn retained_phase_noise_carrier(analysis: &AnalysisResult) -> Option<f64> {
    let metadata = analysis.family_metadata.as_ref()?;
    if metadata.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    let AnalysisResultFamilyMetadata::PeriodicNoise {
        output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
        carrier_frequency_hz: Some(carrier_frequency_hz),
    } = metadata
    else {
        return None;
    };
    Some(*carrier_frequency_hz)
}

/// A waveform name is phase-noise evidence only when it says so directly.
///
/// `onoise` and `inoise` deliberately do *not* qualify: the retained result
/// adapter uses those names for ordinary periodic noise too, and assigning a
/// dBc/Hz meaning to them would invent a carrier normalization.
fn is_explicit_phase_noise_name(name: &str) -> bool {
    let compact = name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    compact.contains("phasenoise") || compact == "lf"
}

pub(super) fn phase_noise_waveform_is_renderable(waveform: &WaveformData) -> bool {
    if !is_explicit_phase_noise_name(&waveform.name)
        || waveform.x.len() != waveform.y.len()
        || waveform.x.len() < 2
    {
        return false;
    }

    waveform
        .x
        .iter()
        .zip(waveform.y.iter())
        .try_fold(None, |previous, (&offset, &level)| {
            (offset.is_finite()
                && offset > 0.0
                && level.is_finite()
                && previous.is_none_or(|previous| offset > previous))
            .then_some(Some(offset))
        })
        .is_some()
}

/// `true` if this analysis contains a usable, explicitly-labelled retained
/// phase-noise trace.  Callers use this for viewer availability; it never
/// widens PNOISE into a phase-noise assertion by analysis type alone.
pub(super) fn phase_noise_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && matches!(
            analysis.analysis_type,
            AnalysisType::Pnoise | AnalysisType::Qpnoise
        )
        && retained_phase_noise_carrier(analysis).is_some()
        && analysis
            .waveforms
            .iter()
            .any(phase_noise_waveform_is_renderable)
}

fn selected_phase_noise_analysis_index(state: &AppState) -> Option<usize> {
    let run = state.simulation.active_run()?;
    state
        .simulation
        .active_analysis_idx
        .filter(|&index| {
            run.analyses
                .get(index)
                .is_some_and(phase_noise_is_renderable)
        })
        .or_else(|| run.analyses.iter().position(phase_noise_is_renderable))
}

fn model_from_analysis(
    analysis_index: usize,
    analysis: &AnalysisResult,
) -> Option<PhaseNoiseModel> {
    if !phase_noise_is_renderable(analysis) {
        return None;
    }
    let (waveform_index, waveform) = analysis
        .waveforms
        .iter()
        .enumerate()
        .find(|(_, waveform)| phase_noise_waveform_is_renderable(waveform))?;
    Some(PhaseNoiseModel {
        analysis_index,
        waveform_index,
        label: analysis.label.clone(),
        source: waveform.name.clone(),
        carrier_frequency_hz: retained_phase_noise_carrier(analysis)?,
        offset_hz: Arc::clone(&waveform.x),
        level_dbc_per_hz: Arc::clone(&waveform.y),
    })
}

fn build_model(state: &AppState) -> Option<PhaseNoiseModel> {
    let run = state.simulation.active_run()?;
    let analysis_index = selected_phase_noise_analysis_index(state)?;
    model_from_analysis(analysis_index, run.analyses.get(analysis_index)?)
}

fn active_periodic_noise_without_phase_trace(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    let Some(index) = state.simulation.active_analysis_idx else {
        return false;
    };
    let Some(analysis) = run.analyses.get(index) else {
        return false;
    };
    matches!(
        analysis.analysis_type,
        AnalysisType::Pnoise | AnalysisType::Qpnoise
    ) && !phase_noise_is_renderable(analysis)
}

fn finite_range(values: &[f64]) -> Option<(f64, f64)> {
    super::finite_extremes(values)
}

/// Return the exact retained value at the requested offset.  This intentionally
/// refuses interpolation: a displayed 1 MHz spot value is an assertion that
/// the analysis retained that sample, not an invented crossing.
fn exact_retained_value_at(offsets: &[f64], levels: &[f64], target_offset: f64) -> Option<f64> {
    offsets
        .iter()
        .position(|offset| *offset == target_offset)
        .and_then(|index| levels.get(index).copied())
        .filter(|level| level.is_finite())
}

fn format_offset_range(
    range: Option<(f64, f64)>,
    quantities: &crate::quantity::QuantityPresentationPolicy,
) -> String {
    range.map_or_else(
        || "Unavailable — no retained offsets".to_owned(),
        |(start, stop)| {
            format!(
                "{} – {}",
                quantities.format_frequency(start, 2),
                quantities.format_frequency(stop, 2)
            )
        },
    )
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the phase-noise spectrum with a logarithmic offset-frequency axis.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let tokens = Tokens::get(ui.ctx());
    let colors = tokens.color;
    let quantities = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state) else {
        well_hint(
            ui,
            if active_periodic_noise_without_phase_trace(state) {
                "The selected PNOISE result retains no trace explicitly identified as phase noise"
            } else {
                "No retained phase-noise spectrum in the active dataset"
            },
        );
        return;
    };

    let legend = [LegendChip {
        name: "L(f) dBc/Hz",
        color: colors.traces[0],
        on: true,
    }];
    let view = state
        .ui
        .results
        .plot_view(super::ResultViewer::PhaseNoise, 0);
    let header = strip::StripHeader::new(
        "PHASE NOISE",
        &format!("{} · {} · retained L(f)", model.label, model.source),
        &legend,
    )
    .zoomed(view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::PhaseNoise, 0);
    }

    let x0 = *model.offset_hz.first().unwrap_or(&1.0);
    let x1 = *model.offset_hz.last().unwrap_or(&1.0);
    if !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        well_hint(ui, "Degenerate retained offset-frequency axis");
        return;
    }
    let Some((level_min, level_max)) = finite_range(&model.level_dbc_per_hz) else {
        well_hint(
            ui,
            "The retained phase-noise trace contains no finite levels",
        );
        return;
    };
    let y_pad = ((level_max - level_min) * 0.1).max(3.0);
    let (x0, x1) = view.x.unwrap_or((x0, x1));
    let (y0, y1) = view.y.unwrap_or((level_min - y_pad, level_max + y_pad));
    let (frequency_scale, frequency_offset, frequency_unit) = quantities.frequency_axis_transform();
    let x_axis = Axis::log_decades(x0, x1, "Hz").with_display_transform(
        frequency_scale,
        frequency_offset,
        frequency_unit,
    );
    let y_axis = Axis::linear_with(y0, y1, "dBc/Hz", 6).with_label("L(f)");
    let mut spec = PlotSpec::new(x_axis, XScale::Log10, y_axis)
        .accessible_name("Phase-noise plot")
        .accessible_detail("Retained phase-noise trace shown as L(f) in dBc/Hz.");
    spec.traces.push(
        Trace::new(&model.offset_hz, &model.level_dbc_per_hz, colors.traces[0]).cache_key(
            0x504E_0000_u64 | ((model.analysis_index as u64) << 16) | model.waveform_index as u64,
        ),
    );
    if let Some(level) = exact_retained_value_at(&model.offset_hz, &model.level_dbc_per_hz, 1.0e6) {
        spec.markers.push(plot::Marker {
            x: 1.0e6,
            y: level,
            side: plot::YSide::Left,
            color: colors.accent,
            label: format!("1 MHz {level:.1} dBc/Hz"),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }

    let readout = |offset| {
        vec![
            ("offset".to_owned(), quantities.format_frequency(offset, 2)),
            (
                "L(f)".to_owned(),
                format!(
                    "{:.3} dBc/Hz",
                    crate::ui::plot::sample_at(&model.offset_hz, &model.level_dbc_per_hz, offset)
                ),
            ),
        ]
    };
    let response = plot::show(ui, &spec, &mut state.ui.results.cache, None, Some(&readout));
    if response.view.any() {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::PhaseNoise, 0)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Render only the Phase Noise inspector fields that the retained model can
/// substantiate.  All absent data is explicit so export, screenshot, and
/// interactive use tell the same truth.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Phase noise", None);
    let quantities = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state) else {
        super::panel_note(
            ui,
            "A PNOISE/QPNOISE result needs an explicitly labelled phase-noise trace before L(f) can be shown.",
        );
        return;
    };

    let offset_range = format_offset_range(finite_range(&model.offset_hz), &quantities);
    let spot = exact_retained_value_at(&model.offset_hz, &model.level_dbc_per_hz, 1.0e6)
        .map_or_else(
            || "Unavailable — 1 MHz sample not retained".to_owned(),
            |level| format!("{level:.3} dBc/Hz · retained sample"),
        );
    let rows = [
        ("Trace", model.source, false),
        (
            "Carrier",
            quantities.format_frequency(model.carrier_frequency_hz, 3),
            false,
        ),
        ("Offset range", offset_range, true),
        (
            "Integrated jitter",
            "Unavailable — jitter integration not retained".to_owned(),
            false,
        ),
        ("Spot L(f) at 1 MHz", spot, true),
        (
            "Spurs",
            "Unavailable — spur evidence not retained".to_owned(),
            false,
        ),
    ];
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        "Only an explicitly labelled retained phase-noise trace is rendered; ordinary periodic-noise traces are not reinterpreted as L(f).",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn waveform(name: &str) -> WaveformData {
        WaveformData::new(
            name,
            vec![1.0, 1.0e3, 1.0e6],
            vec![-72.0, -103.0, -132.0],
            "#f5b700",
        )
    }

    fn phase_result(id: u64, analysis_type: AnalysisType, label: &str) -> AnalysisResult {
        AnalysisResult::new(id, analysis_type, label)
            .with_waveforms(vec![waveform("phase_noise")])
            .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
    }

    #[test]
    fn phase_noise_model_requires_explicit_trace_evidence() {
        let ordinary = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_waveforms(vec![waveform("onoise")]);
        assert!(!phase_noise_is_renderable(&ordinary));
        assert!(model_from_analysis(0, &ordinary).is_none());

        let phase = phase_result(2, AnalysisType::Pnoise, "PNOISE phase");
        let model = model_from_analysis(3, &phase).expect("explicit phase trace renders");
        assert_eq!(model.analysis_index, 3);
        assert_eq!(model.source, "phase_noise");
        assert!(phase_noise_is_renderable(&phase));
    }

    #[test]
    fn qpnoise_and_l_of_f_are_accepted_but_other_analysis_kinds_are_not() {
        let mut qpnoise = phase_result(1, AnalysisType::Qpnoise, "QPNOISE");
        qpnoise.waveforms = vec![waveform("L(f)")];
        assert!(phase_noise_is_renderable(&qpnoise));

        let mut noise = AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_waveforms(vec![waveform("phase noise")]);
        noise.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(2.4e9),
        });
        assert!(!phase_noise_is_renderable(&noise));
    }

    #[test]
    fn failed_phase_noise_analysis_never_becomes_renderable_evidence() {
        let mut failed = phase_result(1, AnalysisType::Pnoise, "PNOISE failed");
        failed.success = false;

        assert!(!phase_noise_is_renderable(&failed));
        assert!(model_from_analysis(0, &failed).is_none());
    }

    #[test]
    fn phase_noise_requires_typed_quantity_and_retained_carrier() {
        let output_psd = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE output")
            .with_waveforms(vec![waveform("phase_noise")])
            .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
                carrier_frequency_hz: Some(2.4e9),
            });
        assert!(!phase_noise_is_renderable(&output_psd));

        let mut missing_carrier = AnalysisResult::new(2, AnalysisType::Pnoise, "PNOISE phase")
            .with_waveforms(vec![waveform("phase_noise")]);
        missing_carrier.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: None,
        });
        assert!(!phase_noise_is_renderable(&missing_carrier));
    }

    #[test]
    fn phase_noise_rejects_invalid_log_axis_and_spot_requires_retained_sample() {
        let invalid = WaveformData::new(
            "phase_noise",
            vec![1.0, 0.0, 1.0e6],
            vec![-80.0, -100.0, -130.0],
            "#f5b700",
        );
        assert!(!phase_noise_waveform_is_renderable(&invalid));

        let trace = waveform("phase_noise");
        assert_eq!(
            exact_retained_value_at(&trace.x, &trace.y, 1.0e6),
            Some(-132.0)
        );
        assert_eq!(exact_retained_value_at(&trace.x, &trace.y, 10.0), None);
    }
}
