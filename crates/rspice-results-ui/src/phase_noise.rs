//! Retained phase-noise spectrum, exact spot marker and evidence-backed inspector.

use crate::presentation::{PlotView, angle_readout, frequency_readout, plain_readout, well_hint};
use crate::strip::{self, LegendChip};
use crate::waveform::WaveformData;
use crate::waves::ReadoutPolicy;
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::phase_noise::{
    exact_retained_value_at, retained_device_noise_shares, retained_measurement,
};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale};
use rspice_ui_kit::tokens::Tokens;

/// A retained phase-noise trace qualified by the host's source gate.
pub struct PhaseNoiseTrace<'a> {
    pub label: &'a str,
    pub source: &'a str,
    pub carrier_frequency_hz: f64,
    pub offset_hz: &'a [f64],
    pub level_dbc_per_hz: &'a [f64],
    pub cache_key: u64,
}

/// Retained measurements from the same analysis as the displayed trace.
pub struct PhaseNoiseInspector<'a> {
    pub trace: PhaseNoiseTrace<'a>,
    pub analysis: &'a AnalysisResult<WaveformData>,
}

/// Requests applied by the host to the same viewport that produced this frame.
pub struct PhaseNoiseResponse {
    pub fit: bool,
    pub plot: Option<plot::PlotResponse>,
}

/// Explain the host's reason for having no phase-noise source.
pub fn show_absent(ui: &mut Ui, selected_periodic_noise_without_phase_trace: bool) {
    well_hint(
        ui,
        if selected_periodic_noise_without_phase_trace {
            "The selected PNOISE result retains no trace explicitly identified as phase noise"
        } else {
            "No retained phase-noise spectrum in the active dataset"
        },
    );
}

/// Draw the retained spectrum and request navigation without mutating host
/// state; its cursor reads at the reader's significant digits and by the
/// reader's cursor interpolation.
pub fn show(
    ui: &mut Ui,
    model: &PhaseNoiseTrace<'_>,
    view: PlotView,
    level_range: Option<(f64, f64)>,
    quantities: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    cache: &mut plot::DecimationCache,
) -> PhaseNoiseResponse {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let colors = Tokens::get(ui.ctx()).color;
    let legend = [LegendChip {
        name: "L(f) dBc/Hz",
        color: colors.traces[0],
        on: true,
    }];
    let header = strip::StripHeader::new(
        "PHASE NOISE",
        &format!("{} · {} · retained L(f)", model.label, model.source),
        &legend,
    )
    .zoomed(view.is_zoomed())
    .show(ui);
    let requested = PhaseNoiseResponse {
        fit: header.fit_clicked,
        plot: None,
    };

    let x0 = *model.offset_hz.first().unwrap_or(&1.0);
    let x1 = *model.offset_hz.last().unwrap_or(&1.0);
    if !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        well_hint(ui, "Degenerate retained offset-frequency axis");
        return requested;
    }
    let Some((level_min, level_max)) = level_range else {
        well_hint(
            ui,
            "The retained phase-noise trace contains no finite levels",
        );
        return requested;
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
        Trace::new(model.offset_hz, model.level_dbc_per_hz, colors.traces[0])
            .cache_key(model.cache_key),
    );
    if let Some(level) = exact_retained_value_at(model.offset_hz, model.level_dbc_per_hz, 1.0e6) {
        spec.markers.push(plot::Marker {
            x: 1.0e6,
            y: level,
            color: colors.accent,
            label: format!("1 MHz {level:.1} dBc/Hz"),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }

    let readout = |offset| {
        vec![
            (
                "offset".to_owned(),
                frequency_readout(quantities, offset, digits),
            ),
            (
                "L(f)".to_owned(),
                plain_readout(
                    presentation.cursor_sample(model.offset_hz, model.level_dbc_per_hz, offset),
                    "dBc/Hz",
                    digits,
                ),
            ),
        ]
    };
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    PhaseNoiseResponse {
        plot: Some(response),
        ..requested
    }
}

/// Display only the inspector fields supported by the bound retained
/// evidence, at the reader's `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    input: Option<PhaseNoiseInspector<'_>>,
    quantities: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let digits = significant_digits;
    section_header(ui, "Phase noise", None);
    let Some(input) = input else {
        crate::presentation::panel_note(ui, "No phase-noise trace in this run.");
        return;
    };

    let PhaseNoiseInspector {
        trace: model,
        analysis,
    } = input;
    // The carrier names which oscillation L(f) is referred to; the offset
    // extent is the plot's own X axis, and the trace is in its legend.
    let mut rows = vec![(
        "Carrier",
        frequency_readout(quantities, model.carrier_frequency_hz, digits),
        false,
    )];
    if let Some(radians) = retained_measurement(analysis, "phase_error_rms_rad") {
        rows.push((
            "Integrated phase error",
            format!("{} rms", angle_readout(quantities, radians, digits)),
            true,
        ));
    }
    if let Some(seconds) = retained_measurement(analysis, "timing_jitter_rms_s") {
        rows.push((
            "Integrated timing jitter",
            rspice_ui_kit::plot::fmt_si_readout(seconds, "s rms", digits),
            true,
        ));
    }
    if let Some(level) = exact_retained_value_at(model.offset_hz, model.level_dbc_per_hz, 1.0e6) {
        rows.push((
            "Spot L(f) at 1 MHz",
            plain_readout(level, "dBc/Hz", digits),
            true,
        ));
    }
    crate::presentation::stat_table(ui, &rows);
    let shares: Vec<_> = retained_device_noise_shares(&analysis.measurements).collect();
    if shares.is_empty() {
        return;
    }
    // The fold sits on the rows' inset rather than on the panel edge.
    rspice_ui_kit::panels::panel_body(ui, |ui| {
        ui.collapsing("Device noise shares", |ui| {
            egui::ScrollArea::vertical().max_height(240.0).show_rows(
                ui,
                30.0,
                shares.len(),
                |ui, range| {
                    for index in range {
                        crate::presentation::stat_table(
                            ui,
                            &[(shares[index].0, format!("{:.1} %", shares[index].1), false)],
                        );
                    }
                },
            );
        })
    });
}
