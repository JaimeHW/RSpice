//! Noise-figure spectrum, source reference, and exact sampled values.

use egui::Ui;
use rspice_results::noise::NoiseFigureEvidence;
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_si_readout;
use rspice_ui_kit::widgets::measurement_table;
use std::sync::Arc;

/// The temperature a noise figure is referred to unless the run says
/// otherwise. Stating 290 K twice tells the reader nothing.
const STANDARD_REFERENCE_KELVIN: f64 = 290.0;

/// Which port the noise figure is referred to.
///
/// The SSB curve itself is a retained waveform ("Noise figure (SSB)") the
/// waveform sheet draws at full width, so the inspector does not draw a
/// second 190 px copy of it beside the rows, and the exact samples belong to
/// the table viewer and the CSV export.
pub(super) fn show(ui: &mut Ui, figure: &Arc<NoiseFigureEvidence>) {
    section_header(ui, "Noise figure", Some("SSB · dB"));
    let mut rows = vec![
        ("Input generator", figure.input_source.clone()),
        ("Source resistor", figure.source_resistor.clone()),
        (
            "Effective resistance",
            fmt_si_readout(figure.source_resistance_ohm, "Ω", 4),
        ),
    ];
    if (figure.source_temperature_kelvin - STANDARD_REFERENCE_KELVIN).abs() > f64::EPSILON
        || (figure.reference_temperature_kelvin - STANDARD_REFERENCE_KELVIN).abs() > f64::EPSILON
    {
        rows.push((
            "Source temperature",
            format!("{} K", figure.source_temperature_kelvin),
        ));
        rows.push((
            "Reference temperature",
            format!("{} K", figure.reference_temperature_kelvin),
        ));
    }
    measurement_table(
        ui,
        &rows
            .iter()
            .map(|(name, value)| (*name, value.as_str()))
            .collect::<Vec<_>>(),
    );
}
