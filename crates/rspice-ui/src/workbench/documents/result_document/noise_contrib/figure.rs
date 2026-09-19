//! Noise-figure spectrum, source reference, and exact sampled values.

use crate::state::NoiseFigureEvidence;
use crate::ui::plot::{self, Axis, DecimationCache, PlotSpec, Trace, XScale};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{measurement_table, section_header};
use egui::Ui;
use std::sync::Arc;

#[derive(Clone)]
struct FigureRange {
    evidence: Arc<NoiseFigureEvidence>,
    bounds: (f64, f64),
}

pub(super) fn show(ui: &mut Ui, figure: &Arc<NoiseFigureEvidence>, cache: &mut DecimationCache) {
    ui.add_space(8.0);
    section_header(ui, "Noise figure", Some("SSB · dB"));
    measurement_table(
        ui,
        &[
            ("Input generator", figure.input_source.clone()),
            ("Source resistor", figure.source_resistor.clone()),
            (
                "Effective resistance",
                format!("{:.6e} Ω", figure.source_resistance_ohm),
            ),
            (
                "Source temperature",
                format!("{} K", figure.source_temperature_kelvin),
            ),
            (
                "Reference temperature",
                format!("{} K", figure.reference_temperature_kelvin),
            ),
        ]
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect::<Vec<_>>(),
    );
    super::super::panel_note(
        ui,
        "Signal is referred to input sideband zero. Folded source thermal noise is evaluated at the reference temperature; all other device noise retains its circuit temperature.",
    );
    let first = figure.frequencies[0];
    let last = *figure.frequencies.last().unwrap();
    if last > first {
        // Hold the source Arc in the memo so pointer reuse cannot make a
        // different dataset inherit this range. No trace scan on redraw.
        let id = ui.id().with("noise-figure-range");
        let bounds = ui.ctx().data_mut(|data| {
            if let Some(cached) = data.get_temp::<FigureRange>(id)
                && Arc::ptr_eq(&cached.evidence, figure)
            {
                return cached.bounds;
            }
            let minimum = figure
                .decibels
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);
            let maximum = figure
                .decibels
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let pad = ((maximum - minimum) * 0.1).max(0.1);
            let bounds = (minimum - pad, maximum + pad);
            data.insert_temp(
                id,
                FigureRange {
                    evidence: Arc::clone(figure),
                    bounds,
                },
            );
            bounds
        });
        let mut spec = PlotSpec::new(
            Axis::log_decades(first, last, "Hz"),
            XScale::Log10,
            Axis::linear_with(bounds.0, bounds.1, "dB", 5),
        )
        .accessible_name("SSB noise-figure spectrum");
        spec.traces.push(
            Trace::new(
                &figure.frequencies,
                &figure.decibels,
                Tokens::get(ui.ctx()).color.traces[0],
            )
            .cache_key((Arc::as_ptr(figure) as usize as u64) ^ 0x4E46_0000_0000_0000),
        );
        let readout = |frequency| {
            vec![
                ("Frequency".into(), format!("{frequency:.6e} Hz")),
                (
                    "Noise figure".into(),
                    format!(
                        "{:.6} dB",
                        plot::sample_at(&figure.frequencies, &figure.decibels, frequency)
                    ),
                ),
            ]
        };
        ui.allocate_ui(egui::vec2(ui.available_width(), 190.0), |ui| {
            plot::show(ui, &spec, cache, None, Some(&readout));
        });
    }
    egui::CollapsingHeader::new("Exact noise-figure samples").show(ui, |ui| {
        egui::ScrollArea::vertical().max_height(180.0).show_rows(
            ui,
            20.0,
            figure.frequencies.len(),
            |ui, rows| {
                for index in rows {
                    ui.monospace(format!(
                        "{:.9e} Hz  {:.9e} dB",
                        figure.frequencies[index], figure.decibels[index]
                    ));
                }
            },
        );
    });
}
