//! Retained phase-noise spectrum, exact spot marker and evidence-backed inspector.

use crate::presentation::{PlotView, well_hint};
use crate::strip::{self, LegendChip};
use crate::waveform::WaveformData;
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::phase_noise::{
    exact_retained_value_at, retained_device_noise_shares, retained_measurement,
};
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::section_header;

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
    pub offset_range: Option<(f64, f64)>,
}

/// Requests applied by the host to the same viewport that produced this frame.
pub struct PhaseNoiseResponse {
    pub fit: bool,
    pub plot: Option<plot::PlotResponse>,
}

fn format_offset_range(
    range: Option<(f64, f64)>,
    quantities: &rspice_app_types::quantity::QuantityPresentationPolicy,
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

/// Draw the retained spectrum and request navigation without mutating host state.
pub fn show(
    ui: &mut Ui,
    model: &PhaseNoiseTrace<'_>,
    view: PlotView,
    level_range: Option<(f64, f64)>,
    quantities: &QuantityPresentationPolicy,
    cache: &mut plot::DecimationCache,
) -> PhaseNoiseResponse {
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
            ("offset".to_owned(), quantities.format_frequency(offset, 2)),
            (
                "L(f)".to_owned(),
                format!(
                    "{:.3} dBc/Hz",
                    rspice_ui_kit::plot::sample_at(model.offset_hz, model.level_dbc_per_hz, offset)
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

/// Display only the inspector fields supported by the bound retained evidence.
pub fn right_panel(
    ui: &mut Ui,
    input: Option<PhaseNoiseInspector<'_>>,
    quantities: &QuantityPresentationPolicy,
) {
    section_header(ui, "Phase noise", None);
    let Some(input) = input else {
        crate::presentation::panel_note(
            ui,
            "A PNOISE/QPNOISE result needs an explicitly labelled phase-noise trace before L(f) can be shown.",
        );
        return;
    };

    let PhaseNoiseInspector {
        trace: model,
        analysis,
        offset_range,
    } = input;
    let offset_range = format_offset_range(offset_range, quantities);
    let phase = retained_measurement(analysis, "phase_error_rms_rad")
        .map(|value| format!("{value:.6e} rad RMS"))
        .unwrap_or_else(|| "Not retained".into());
    let jitter = retained_measurement(analysis, "timing_jitter_rms_s")
        .map(|value| rspice_ui_kit::plot::fmt_si(value, "s RMS", 6))
        .unwrap_or_else(|| "Not retained".into());
    let spot = exact_retained_value_at(model.offset_hz, model.level_dbc_per_hz, 1.0e6).map_or_else(
        || "Unavailable — 1 MHz sample not retained".to_owned(),
        |level| format!("{level:.3} dBc/Hz · retained sample"),
    );
    let rows = [
        ("Trace", model.source.to_owned(), false),
        (
            "Carrier",
            quantities.format_frequency(model.carrier_frequency_hz, 3),
            false,
        ),
        ("Offset range", offset_range, true),
        ("Integrated phase error", phase, true),
        ("Integrated timing jitter", jitter, true),
        ("Spot L(f) at 1 MHz", spot, true),
        (
            "Spurs",
            "Unavailable — spur evidence not retained".to_owned(),
            false,
        ),
    ];
    crate::presentation::stat_table(ui, &rows);
    ui.collapsing("Device noise shares", |ui| {
        let shares: Vec<_> = retained_device_noise_shares(&analysis.measurements).collect();
        if shares.is_empty() {
            crate::presentation::panel_note(ui, "No per-device noise shares were retained.");
        } else {
            egui::ScrollArea::vertical().max_height(240.0).show_rows(
                ui,
                30.0,
                shares.len(),
                |ui, range| {
                    for index in range {
                        crate::presentation::stat_table(
                            ui,
                            &[(shares[index].0, format!("{:.6} %", shares[index].1), false)],
                        );
                    }
                },
            );
        }
    });
    crate::presentation::panel_note(
        ui,
        "Only an explicitly labelled retained phase-noise trace is rendered; ordinary periodic-noise traces are not reinterpreted as L(f).",
    );
}
