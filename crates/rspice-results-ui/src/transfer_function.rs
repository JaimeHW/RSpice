//! The transfer-function sheet: a DC transfer function's outputs in one dense
//! table, each with its unit, and how they were measured in the inspector.
//!
//! The rows are the outputs a `.TF` reports, in the order it reports them —
//! the gain from the input source to the output, the resistance seen at the
//! input source, the resistance seen at the output — and, for a gain made
//! relative to the operating point, the nominal values it was made relative
//! to. Each reads at the reader's significant digits.

use crate::presentation::{stat_table, well_hint, with_table_state};
use egui::Ui;
use rspice_results::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_column_reading;
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable};

/// Borrowed retained values selected and validated by the host.
pub struct TransferFunctionView<'a> {
    pub input_source: &'a str,
    pub output_expression: &'a str,
    pub input_quantity: TransferFunctionQuantityEvidence,
    pub output_quantity: TransferFunctionQuantityEvidence,
    pub input_unit: &'a str,
    pub output_unit: &'a str,
    pub normalization: TransferFunctionNormalizationEvidence,
    pub accuracy: TransferFunctionAccuracyEvidence,
    pub gain: Option<TransferFunctionScalarEvidence>,
    pub input_resistance: Option<TransferFunctionScalarEvidence>,
    pub output_resistance: Option<TransferFunctionScalarEvidence>,
    pub nominal_input: Option<f64>,
    pub nominal_output: Option<f64>,
}

/// What an output the run was not asked for says when the pointer rests on
/// its dash.
const NOT_REQUESTED: &str = "Not requested by this run";

pub fn show(ui: &mut Ui, view: Option<TransferFunctionView<'_>>, significant_digits: usize) {
    let Some(view) = view else {
        well_hint(ui, "Pick a transfer-function analysis in the data browser");
        return;
    };
    let rows = output_rows(&view, significant_digits);
    // The value stands beside what it is, and the signal it is read at takes
    // what the two leave, so a wide sheet does not part a name from its
    // number. The rows keep the order a transfer function reports them in.
    let columns = [
        Column::words("Output", 168.0).unsortable(),
        Column::number("Value", 168.0).unsortable(),
        Column::name("Signal", 168.0).unsortable(),
    ];
    with_table_state(ui, "transfer-function", |ui, state| {
        DataTable::new("rspice.results.transfer-function", &columns, rows.len())
            .state(state)
            .show(ui, |index| rows[index].clone())
    });
}

/// The sheet's rows: each output, its value with its unit, and the signal it
/// is read at.
fn output_rows(view: &TransferFunctionView<'_>, significant_digits: usize) -> Vec<Vec<Cell>> {
    let relative = view.normalization == TransferFunctionNormalizationEvidence::RelativeToNominal;
    let mut rows = vec![
        vec![
            Cell::text(if relative {
                "Relative gain"
            } else {
                "Transfer gain"
            }),
            scalar_cell(view.gain, gain_unit(view), significant_digits),
            Cell::text(format!(
                "{} / {}",
                view.output_expression, view.input_source
            )),
        ],
        vec![
            Cell::text("Input resistance"),
            scalar_cell(view.input_resistance, "Ω", significant_digits),
            Cell::text(view.input_source),
        ],
        vec![
            Cell::text("Output resistance"),
            scalar_cell(view.output_resistance, "Ω", significant_digits),
            Cell::text(view.output_expression),
        ],
    ];
    if relative {
        rows.push(vec![
            Cell::text("Nominal input"),
            nominal_cell(view.nominal_input, view.input_unit, significant_digits),
            Cell::text(view.input_source),
        ]);
        rows.push(vec![
            Cell::text("Nominal output"),
            nominal_cell(view.nominal_output, view.output_unit, significant_digits),
            Cell::text(view.output_expression),
        ]);
    }
    rows
}

fn scalar_cell(
    value: Option<TransferFunctionScalarEvidence>,
    unit: &str,
    significant_digits: usize,
) -> Cell {
    match value {
        Some(value) => Cell::quantity(scalar_text(value, unit, significant_digits)),
        None => Cell::absent().hint(NOT_REQUESTED),
    }
}

fn nominal_cell(value: Option<f64>, unit: &str, significant_digits: usize) -> Cell {
    match value {
        Some(value) => Cell::quantity(fmt_column_reading(value, unit, significant_digits)),
        None => Cell::absent(),
    }
}

/// A retained scalar at `significant_digits` with its unit. A ratio of like
/// quantities takes no SI prefix — a gain of a half reads "0.5 V/V", not
/// "500 mV/V" — and a relative gain is a bare number.
fn scalar_text(
    value: TransferFunctionScalarEvidence,
    unit: &str,
    significant_digits: usize,
) -> String {
    let with_unit = |number: String| {
        if unit == "1" {
            number
        } else {
            format!("{number} {unit}")
        }
    };
    match value {
        TransferFunctionScalarEvidence::Finite(value) => match unit {
            "1" | "V/V" | "A/A" => with_unit(fmt_column_reading(value, "", significant_digits)),
            _ => fmt_column_reading(value, unit, significant_digits),
        },
        TransferFunctionScalarEvidence::PositiveInfinity => with_unit("+∞".to_owned()),
        TransferFunctionScalarEvidence::NegativeInfinity => with_unit("−∞".to_owned()),
    }
}

/// How the transfer function was measured: where it was solved, how its gain
/// is normalized and how hard the solves worked. The sheet lists what came of
/// it.
pub fn right_panel(ui: &mut Ui, view: Option<TransferFunctionView<'_>>) {
    let Some(view) = view else {
        return;
    };
    section_header(ui, "Measurement", None);
    stat_table(
        ui,
        &[
            ("Solve point", "DC operating point".to_owned(), false),
            (
                "Normalization",
                normalization_label(view.normalization).to_owned(),
                false,
            ),
            ("Accuracy", accuracy_label(view.accuracy).to_owned(), false),
        ],
    );
}

fn normalization_label(value: TransferFunctionNormalizationEvidence) -> &'static str {
    match value {
        TransferFunctionNormalizationEvidence::None => "Disabled",
        TransferFunctionNormalizationEvidence::RelativeToNominal => "Relative to nominal",
        TransferFunctionNormalizationEvidence::PerSourceUnit => "Per source unit",
    }
}

fn accuracy_label(value: TransferFunctionAccuracyEvidence) -> &'static str {
    match value {
        TransferFunctionAccuracyEvidence::Fast => "Fast",
        TransferFunctionAccuracyEvidence::Balanced => "Balanced",
        TransferFunctionAccuracyEvidence::Accurate => "Accurate",
        TransferFunctionAccuracyEvidence::Robust => "Robust",
    }
}

pub fn gain_unit(view: &TransferFunctionView<'_>) -> &'static str {
    if view.normalization == TransferFunctionNormalizationEvidence::RelativeToNominal {
        return "1";
    }
    match (view.input_quantity, view.output_quantity) {
        (TransferFunctionQuantityEvidence::Voltage, TransferFunctionQuantityEvidence::Voltage) => {
            "V/V"
        }
        (TransferFunctionQuantityEvidence::Voltage, TransferFunctionQuantityEvidence::Current) => {
            "A/V"
        }
        (TransferFunctionQuantityEvidence::Current, TransferFunctionQuantityEvidence::Voltage) => {
            "V/A"
        }
        (TransferFunctionQuantityEvidence::Current, TransferFunctionQuantityEvidence::Current) => {
            "A/A"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TransferFunctionScalarEvidence::{Finite, NegativeInfinity, PositiveInfinity};

    fn view(normalization: TransferFunctionNormalizationEvidence) -> TransferFunctionView<'static> {
        let relative = normalization == TransferFunctionNormalizationEvidence::RelativeToNominal;
        TransferFunctionView {
            input_source: "VIN",
            output_expression: "V(out)",
            input_quantity: TransferFunctionQuantityEvidence::Voltage,
            output_quantity: TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V",
            output_unit: "V",
            normalization,
            accuracy: TransferFunctionAccuracyEvidence::Balanced,
            gain: Some(Finite(-9.999_000_1)),
            input_resistance: Some(Finite(1_000.0)),
            output_resistance: Some(PositiveInfinity),
            nominal_input: relative.then_some(2.0),
            nominal_output: relative.then_some(0.5),
        }
    }

    fn texts(rows: &[Vec<Cell>]) -> Vec<Vec<&str>> {
        rows.iter()
            .map(|row| row.iter().map(|cell| cell.text.as_str()).collect())
            .collect()
    }

    /// Every output reads at the reader's digits, zeros kept, with its unit
    /// and the signal it is read at: resistances in ohms with their prefix,
    /// a voltage gain as a plain ratio, an open input as infinite ohms.
    #[test]
    fn each_output_reads_with_its_unit_and_its_signal() {
        let rows = output_rows(&view(TransferFunctionNormalizationEvidence::None), 4);
        assert_eq!(
            texts(&rows),
            [
                ["Transfer gain", "−9.999 V/V", "V(out) / VIN"],
                ["Input resistance", "1.000 kΩ", "VIN"],
                ["Output resistance", "+∞ Ω", "V(out)"],
            ]
        );
    }

    /// A gain made relative to the operating point is a bare number, and the
    /// nominal values it was made relative to read beneath it.
    #[test]
    fn a_relative_gain_reads_with_the_nominal_values_it_divides_by() {
        let rows = output_rows(
            &view(TransferFunctionNormalizationEvidence::RelativeToNominal),
            4,
        );
        assert_eq!(texts(&rows)[0], ["Relative gain", "−9.999", "V(out) / VIN"]);
        assert_eq!(texts(&rows)[3], ["Nominal input", "2.000 V", "VIN"]);
        assert_eq!(texts(&rows)[4], ["Nominal output", "500.0 mV", "V(out)"]);
    }

    /// An output the run was not asked for reads as a dash, not as zero.
    #[test]
    fn an_output_the_run_did_not_request_reads_as_a_dash() {
        let mut view = view(TransferFunctionNormalizationEvidence::None);
        view.input_resistance = None;
        assert_eq!(output_rows(&view, 4)[1][1].text, "\u{2014}");
    }

    /// A ratio of unlike quantities keeps its prefix, a ratio of like ones
    /// does not, and an infinity keeps its sign and unit.
    #[test]
    fn a_scalar_takes_a_prefix_only_where_its_unit_does() {
        assert_eq!(scalar_text(Finite(0.5), "V/V", 4), "0.5000 V/V");
        assert_eq!(scalar_text(Finite(0.002), "A/V", 4), "2.000 mA/V");
        assert_eq!(scalar_text(Finite(1.25e4), "V/A", 4), "12.50 kV/A");
        assert_eq!(scalar_text(NegativeInfinity, "V/A", 4), "−∞ V/A");
        assert_eq!(scalar_text(PositiveInfinity, "1", 4), "+∞");
    }
}
