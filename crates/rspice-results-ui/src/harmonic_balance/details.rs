//! What the details pane reads off a coefficient spectrum: every line of
//! every signal in one table, the THD of each one-tone signal, and the
//! intermodulation and intercept of each two-tone one.

use super::HarmonicBalanceModel;
use crate::presentation::{frequency_readout, with_table_state};
use egui::Ui;
use rspice_results::harmonic_spectrum::lines::{
    CoefficientLine, Intermodulation, carrier, harmonic_order, single_tone_fundamental,
    total_harmonic_distortion,
};
use rspice_results::harmonic_spectrum::settings::{CoefficientQuantity, CoefficientSettings};
use rspice_ui_kit::panels::{section_header, section_header_action};
use rspice_ui_kit::plot::fmt_si_significant;
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable, SortKey};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

/// One row of the line table.
struct Row<'a> {
    signal: &'a str,
    line: String,
    order: f64,
    frequency: f64,
    level: f64,
    unit: &'static str,
    dbc: Option<f64>,
    phase: f64,
}

/// A line's name: `DC`, `H3` on one tone's grid, or its frequency's place
/// on the grid of several tones.
fn line_name(line: &CoefficientLine, fundamental: Option<f64>) -> (String, f64) {
    if line.frequency == 0.0 {
        return ("DC".to_owned(), 0.0);
    }
    match fundamental {
        Some(fundamental) => {
            let order = harmonic_order(line.frequency, fundamental);
            #[allow(clippy::cast_precision_loss)]
            (format!("H{order}"), order as f64)
        }
        None => ("Mixing".to_owned(), line.frequency),
    }
}

fn rows<'a>(model: &'a HarmonicBalanceModel, settings: &CoefficientSettings) -> Vec<Row<'a>> {
    let mut rows = Vec::new();
    for trace in &model.traces {
        let quantity = trace.quantity();
        let fundamental = single_tone_fundamental(&trace.lines);
        let carrier = carrier(&trace.lines);
        for line in trace.lines.iter() {
            let (name, order) = line_name(line, fundamental);
            rows.push(Row {
                signal: &trace.name,
                line: name,
                order,
                frequency: line.frequency,
                level: settings.level(line.magnitude(), line.frequency, quantity, carrier),
                unit: settings.unit(quantity),
                dbc: (line.frequency > 0.0)
                    .then(|| carrier.map(|carrier| 20.0 * (line.magnitude() / carrier).log10()))
                    .flatten(),
                phase: line.phase_degrees(),
            });
        }
    }
    rows
}

/// Every line as tab-separated text, its head first, for a spreadsheet.
fn table_text(rows: &[Row<'_>], digits: usize) -> String {
    let mut text = String::from("Signal\tLine\tFrequency (Hz)\tLevel\tUnit\tdBc\tPhase (°)");
    for row in rows {
        text.push('\n');
        text.push_str(&format!(
            "{}\t{}\t{}\t{:.*e}\t{}\t{}\t{:.*}",
            row.signal,
            row.line,
            row.frequency,
            digits.saturating_sub(1),
            row.level,
            row.unit,
            row.dbc.map_or_else(String::new, |dbc| format!("{dbc:.3}")),
            3,
            row.phase,
        ));
    }
    text
}

/// The pane's sections, at the reader's `digits`.
pub(super) fn show(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    settings: &CoefficientSettings,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) {
    distortion(ui, model, digits);
    intermodulation(ui, model, settings, quantity_policy, digits);

    let rows = rows(model, settings);
    if section_header_action(ui, "Lines", "Copy", !rows.is_empty())
        .on_hover_text("Copy every line as tab-separated text")
        .clicked()
    {
        ui.ctx().copy_text(table_text(&rows, digits));
    }
    let width = (ui.available_width() / 6.0).floor();
    let columns = [
        Column::name("Signal", width),
        Column::words("Line", width),
        Column::number("Frequency", width),
        Column::number("Level", width),
        Column::number("dBc", width),
        Column::number("Phase", width),
    ];
    let height = ui.available_height().max(120.0);
    with_table_state(ui, "harmonic-balance.lines", |ui, state| {
        DataTable::new(
            "rspice.results.harmonic-balance.lines",
            &columns,
            rows.len(),
        )
        .state(state)
        .max_height(height)
        .show(ui, |index| {
            let row = &rows[index];
            vec![
                Cell::text(row.signal),
                Cell::text(row.line.clone()).sort_by(SortKey::Number(row.order)),
                Cell::number(frequency_readout(quantity_policy, row.frequency, digits))
                    .sort_by(SortKey::Number(row.frequency)),
                Cell::number(fmt_si_significant(row.level, row.unit, digits))
                    .sort_by(SortKey::Number(row.level)),
                row.dbc.map_or_else(Cell::absent, |dbc| {
                    Cell::number(fmt_si_significant(dbc, "dBc", digits))
                        .sort_by(SortKey::Number(dbc))
                }),
                Cell::number(fmt_si_significant(row.phase, "°", digits))
                    .sort_by(SortKey::Number(row.phase)),
            ]
        });
    });
}

/// Each signal's THD, in percent and in dB; a THD that cannot be read says
/// why in its row in a few words, and in full on hover.
fn distortion(ui: &mut Ui, model: &HarmonicBalanceModel, digits: usize) {
    let mut shown = false;
    for trace in &model.traces {
        let thd = total_harmonic_distortion(&trace.lines);
        if matches!(
            thd,
            Err(rspice_results::harmonic_spectrum::lines::ThdAbsence::NotOneTone)
        ) {
            continue;
        }
        if !shown {
            section_header(ui, "Distortion", None);
            shown = true;
        }
        match thd {
            Ok(ratio) => crate::presentation::stat_table(
                ui,
                &[(
                    trace.name.as_str(),
                    format!(
                        "THD {} · {}",
                        fmt_si_significant(100.0 * ratio, "%", digits),
                        fmt_si_significant(20.0 * ratio.log10(), "dB", digits)
                    ),
                    true,
                )],
            ),
            Err(absence) => {
                let row = ui
                    .scope(|ui| {
                        crate::presentation::stat_table(
                            ui,
                            &[(
                                trace.name.as_str(),
                                format!("THD {}", absence.short()),
                                true,
                            )],
                        );
                    })
                    .response;
                row.on_hover_text(absence.sentence());
            }
        }
    }
}

/// Each two-tone signal's tones, products and output intercept.
fn intermodulation(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    settings: &CoefficientSettings,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) {
    let tones = model.stated_tones();
    let mut shown = false;
    for trace in &model.traces {
        let Some(im) = Intermodulation::read(&trace.lines, tones) else {
            continue;
        };
        if !shown {
            section_header(ui, "Intermodulation", None);
            shown = true;
        }
        let quantity = trace.quantity();
        let mut rows = vec![(
            trace.name.as_str().to_owned(),
            format!(
                "{} and {}",
                frequency_readout(quantity_policy, im.tones[0].frequency, digits),
                frequency_readout(quantity_policy, im.tones[1].frequency, digits)
            ),
            true,
        )];
        for product in &im.products {
            rows.push((
                format!("IM{} {}", product.order, product.name),
                fmt_si_significant(product.dbc, "dBc", digits),
                false,
            ));
        }
        let (unit, name) = if quantity == CoefficientQuantity::Other {
            ("dB", "OIP3".to_owned())
        } else {
            (
                "dBm",
                format!(
                    "OIP3 into {}",
                    fmt_si_significant(settings.reference_ohms, "Ω", digits)
                ),
            )
        };
        rows.push((
            name,
            im.output_third_order_intercept(quantity, settings)
                .map_or_else(
                    || "— no third-order product kept".to_owned(),
                    |intercept| fmt_si_significant(intercept, unit, digits),
                ),
            false,
        ));
        let rows = rows
            .iter()
            .map(|(name, value, key)| (name.as_str(), value.clone(), *key))
            .collect::<Vec<_>>();
        crate::presentation::stat_table(ui, &rows);
    }
}
