//! What the details pane reads off a coefficient spectrum: the THD of each
//! one-tone signal, the intermodulation and intercept of each two-tone one,
//! and every line of each signal in a table of its own.

use super::{HarmonicBalanceModel, HarmonicTrace};
use crate::fft::view::columns::{self, Fitting, HUNDREDTH};
use crate::presentation::{frequency_readout, with_table_state};
use egui::Ui;
use rspice_results::harmonic_spectrum::lines::{
    CoefficientLine, Intermodulation, carrier, harmonic_order, single_tone_fundamental,
    total_harmonic_distortion,
};
use rspice_results::harmonic_spectrum::settings::{CoefficientQuantity, CoefficientSettings};
use rspice_ui_kit::panels::{section_header, section_header_action};
use rspice_ui_kit::plot::fmt_column_reading;
use rspice_ui_kit::widgets::table::{Align, Cell, DataTable, SortKey};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

/// One row of a signal's line table.
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

fn rows<'a>(trace: &'a HarmonicTrace, settings: &CoefficientSettings) -> Vec<Row<'a>> {
    let quantity = trace.quantity();
    let fundamental = single_tone_fundamental(&trace.lines);
    let carrier = carrier(&trace.lines);
    trace
        .lines
        .iter()
        .map(|line| {
            let (name, order) = line_name(line, fundamental);
            Row {
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
            }
        })
        .collect()
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
    distortion(ui, model, quantity_policy, digits);
    intermodulation(ui, model, settings, quantity_policy, digits);
    for trace in &model.traces {
        lines(ui, trace, settings, quantity_policy, digits);
    }
}

/// One signal's lines, one signal to a table so each column fits the pane:
/// on one tone's grid each harmonic by its order, the fundamental standing
/// under Distortion; on several tones' grid each line by its frequency, to
/// the decade under the grid's closest two lines; then its level in the
/// bar's scale, a decibel to its hundredth and an amplitude to the reader's
/// digits of the strongest line, and its phase to the hundredth of a
/// degree. Copy takes every column, the frequency and the level under the
/// carrier too. The pane scrolls, so the table shows every line.
fn lines(
    ui: &mut Ui,
    trace: &HarmonicTrace,
    settings: &CoefficientSettings,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) {
    let rows = rows(trace, settings);
    if section_header_action(
        ui,
        &format!("Lines · {}", trace.name),
        "Copy",
        !rows.is_empty(),
    )
    .on_hover_text(format!(
        "Copy every line of {} as tab-separated text",
        trace.name
    ))
    .clicked()
    {
        ui.ctx().copy_text(table_text(&rows, digits));
    }
    let one_tone = single_tone_fundamental(&trace.lines).is_some();
    let unit = settings.unit(trace.quantity());
    let levels = rows.iter().map(|row| Some(row.level)).collect::<Vec<_>>();
    let level_step = if rspice_results::number_format::unit_takes_prefix(unit) {
        columns::significant_step(&levels, digits)
    } else {
        HUNDREDTH
    };
    let level = columns::at_resolution(&levels, level_step, unit);
    let phase = columns::at_resolution(
        &rows.iter().map(|row| Some(row.phase)).collect::<Vec<_>>(),
        HUNDREDTH,
        "°",
    );
    let frequencies = rows.iter().map(|row| row.frequency).collect::<Vec<_>>();
    let frequency = columns::frequencies(
        &frequencies,
        columns::decade_step(closest_spacing(&frequencies)),
        quantity_policy,
    );
    let numbers = |cells: &[Option<String>]| {
        cells
            .iter()
            .map(|cell| (cell.clone().unwrap_or_default(), true))
            .collect::<Vec<_>>()
    };
    let key = if one_tone {
        Fitting {
            head: "Line".to_owned(),
            tooltip: Some("The line's place on the grid: DC or its harmonic order".to_owned()),
            align: Align::Left,
            cells: rows.iter().map(|row| (row.line.clone(), true)).collect(),
        }
    } else {
        Fitting {
            head: frequency.unit.clone(),
            tooltip: Some(format!("Frequency, in {}", frequency.unit)),
            align: Align::Right,
            cells: numbers(&frequency.cells),
        }
    };
    let columns = columns::fitted(
        ui,
        vec![
            key,
            Fitting {
                head: level.unit.clone(),
                tooltip: Some(format!("Level, in {}", level.unit)),
                align: Align::Right,
                cells: numbers(&level.cells),
            },
            Fitting {
                head: format!("Phase ({})", phase.unit),
                tooltip: Some("Phase, in degrees".to_owned()),
                align: Align::Right,
                cells: numbers(&phase.cells),
            },
        ],
    );
    let id = format!("rspice.results.harmonic-balance.lines.{}", trace.name);
    with_table_state(ui, &id, |ui, state| {
        DataTable::new(&id, &columns, rows.len())
            .state(state)
            .show(ui, |index| {
                let row = &rows[index];
                let printed = |cells: &[Option<String>]| cells[index].clone().unwrap_or_default();
                vec![
                    if one_tone {
                        Cell::number(row.line.clone()).sort_by(SortKey::Number(row.order))
                    } else {
                        Cell::number(printed(&frequency.cells))
                            .sort_by(SortKey::Number(row.frequency))
                    },
                    Cell::number(printed(&level.cells)).sort_by(SortKey::Number(row.level)),
                    Cell::number(printed(&phase.cells)).sort_by(SortKey::Number(row.phase)),
                ]
            });
    });
}

/// The closest two distinct frequencies of a grid lie apart: the resolution
/// its lines are told apart at.
fn closest_spacing(frequencies: &[f64]) -> f64 {
    let mut sorted = frequencies
        .iter()
        .copied()
        .filter(|frequency| frequency.is_finite())
        .collect::<Vec<_>>();
    sorted.sort_by(f64::total_cmp);
    sorted
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|spacing| *spacing > 0.0)
        .fold(f64::INFINITY, f64::min)
}

/// Each one-tone signal's fundamental and THD under its name, the THD in
/// percent and in dB on a row each as the spectrum sheet reads it; a THD
/// that cannot be read says why in its row in a few words, and in full on
/// hover.
fn distortion(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) {
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
        let mut head = vec![(trace.name.as_str(), String::new(), true)];
        if let Some(fundamental) = single_tone_fundamental(&trace.lines) {
            head.push((
                "Fundamental",
                frequency_readout(quantity_policy, fundamental, digits),
                false,
            ));
        }
        crate::presentation::stat_table(ui, &head);
        match thd {
            Ok(ratio) => crate::presentation::stat_table(
                ui,
                &[
                    ("THD", fmt_column_reading(100.0 * ratio, "%", digits), false),
                    (
                        "THD",
                        fmt_column_reading(20.0 * ratio.log10(), "dB", digits),
                        false,
                    ),
                ],
            ),
            Err(absence) => {
                let row = ui
                    .scope(|ui| {
                        crate::presentation::stat_table(
                            ui,
                            &[("THD", absence.short().to_owned(), false)],
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
                fmt_column_reading(product.dbc, "dBc", digits),
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
                    fmt_column_reading(settings.reference_ohms, "Ω", digits)
                ),
            )
        };
        rows.push((
            name,
            im.output_third_order_intercept(quantity, settings)
                .map_or_else(
                    || "— no third-order product kept".to_owned(),
                    |intercept| fmt_column_reading(intercept, unit, digits),
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
