//! The columns of a details-pane table: every row of a column printed at one
//! precision, the sheet's resolution, with the column's unit and decade
//! stated once in its head, and each column as wide as its widest words, so
//! no value is ever cut short.

use egui::Ui;
use rspice_results::number_format::unit_takes_prefix;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens;
use rspice_ui_kit::widgets::table::{Align, CELL_PAD, Column, SORT_MARK_ROOM, TABLE_EDGE_INSET};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

/// A level in a details table to the hundredth of a decibel, an analyser's
/// marker resolution, and a phase to the hundredth of a degree.
pub(crate) const HUNDREDTH: f64 = 0.01;

/// The SI prefixes, yocto to yotta, three decades apart.
const PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

/// The decimals an `f64` holds: no step finer than this prints a digit that
/// means anything.
const MOST_DECIMALS: usize = 15;

/// One column's values as printed, and the unit its head states.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Printed {
    /// The unit in the decade the column is printed in: `kHz`, `dBc`.
    pub unit: String,
    /// Each value at the column's one precision; `None` where the row has
    /// no value to print.
    pub cells: Vec<Option<String>>,
}

/// The decade step at or under `resolution`: the place a value known to
/// `resolution` is printed to, 10 Hz for a 50 Hz bin.
pub(crate) fn decade_step(resolution: f64) -> f64 {
    if resolution.is_finite() && resolution > 0.0 {
        10f64.powf(resolution.log10().floor())
    } else {
        f64::NAN
    }
}

/// The step that prints `digits` significant digits of the largest of
/// `values`: the resolution of a column whose values carry no resolution of
/// their own, a magnitude read off a coefficient.
pub(crate) fn significant_step(values: &[Option<f64>], digits: usize) -> f64 {
    let largest = largest(values);
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let digits = digits.max(1) as i32;
    if largest > 0.0 {
        10f64.powi(largest.log10().floor() as i32 - digits + 1)
    } else {
        f64::NAN
    }
}

fn largest(values: &[Option<f64>]) -> f64 {
    values
        .iter()
        .flatten()
        .filter(|value| value.is_finite())
        .fold(0.0_f64, |largest, value| largest.max(value.abs()))
}

/// `values` in `unit` printed to `step`: in the decade of the largest of
/// them when the unit takes a prefix, each rounded to the step and printed
/// at the fewest decimals that print every value of the column at it, so a
/// column of 2000, 3000 and 4000 Hz to 10 Hz reads 2, 3 and 4 kHz and one of
/// 2100, 3150 and 4200 Hz reads 2.10, 3.15 and 4.20 kHz.
pub(crate) fn at_resolution(values: &[Option<f64>], step: f64, unit: &str) -> Printed {
    let largest = largest(values);
    let (scale, prefix) = if unit_takes_prefix(unit) && largest > 0.0 {
        #[allow(clippy::cast_possible_truncation)]
        let exponent = ((largest.log10() / 3.0).floor() as i32).clamp(-8, 8);
        #[allow(clippy::cast_sign_loss)]
        (10f64.powi(3 * exponent), PREFIXES[(exponent + 8) as usize])
    } else {
        (1.0, "")
    };
    let step = step / scale;
    let rounded = values
        .iter()
        .map(|value| {
            value.filter(|value| value.is_finite()).map(|value| {
                if step.is_finite() && step > 0.0 {
                    (value / scale / step).round() * step
                } else {
                    value / scale
                }
            })
        })
        .collect::<Vec<_>>();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let most = if step.is_finite() && step > 0.0 {
        ((-step.log10()).ceil().max(0.0) as usize).min(MOST_DECIMALS)
    } else {
        MOST_DECIMALS
    };
    let tolerance = if step.is_finite() && step > 0.0 {
        step * 1.0e-6
    } else {
        0.0
    };
    let decimals = (0..=most)
        .find(|&decimals| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let place = 10f64.powi(decimals as i32);
            rounded
                .iter()
                .flatten()
                .all(|value| ((value * place).round() / place - value).abs() <= tolerance)
        })
        .unwrap_or(most);
    Printed {
        unit: format!("{prefix}{unit}"),
        cells: rounded
            .into_iter()
            .map(|value| value.map(|value| plain(value, decimals)))
            .collect(),
    }
}

/// Frequencies in hertz through the reader's frequency presentation, to
/// `step` hertz: [`at_resolution`] in the unit the plot's axis reads.
pub(crate) fn frequencies(
    values: &[f64],
    step: f64,
    policy: &QuantityPresentationPolicy,
) -> Printed {
    let (scale, offset, unit) = policy.frequency_axis_transform();
    let shown = values
        .iter()
        .map(|value| Some(value * scale + offset))
        .collect::<Vec<_>>();
    at_resolution(&shown, step * scale.abs(), unit)
}

/// `value` at `decimals`, with a true minus, and unsigned when it rounds to
/// zero.
fn plain(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    match text.strip_prefix('-') {
        Some(magnitude) if magnitude.chars().all(|c| c == '0' || c == '.') => magnitude.to_owned(),
        Some(magnitude) => format!("\u{2212}{magnitude}"),
        None => text,
    }
}

/// One column of a details table before it is sized: its head, what the
/// head says on hover, where its words sit, and every cell's words with
/// whether they are a number, set in mono.
pub(crate) struct Fitting {
    pub head: String,
    pub tooltip: Option<String>,
    pub align: Align,
    pub cells: Vec<(String, bool)>,
}

/// The columns, each as wide as its widest words, its head with room for a
/// sort mark or its widest cell, padded as the table pads it: the outer
/// columns to the panel inset, the rest to the cell pad either side. Each
/// takes a share of the width the pane leaves over, so the table runs the
/// pane's width and no column is cut short.
pub(crate) fn fitted(ui: &Ui, columns: Vec<Fitting>) -> Vec<Column> {
    let width = |text: &str, font: egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font, egui::Color32::PLACEHOLDER)
            .size()
            .x
    };
    let count = columns.len();
    columns
        .into_iter()
        .enumerate()
        .map(|(index, column)| {
            let head =
                width(&column.head, theme::sans(tokens::FS_0, FontWeight::Medium)) + SORT_MARK_ROOM;
            let widest = column
                .cells
                .iter()
                .map(|(text, number)| {
                    width(
                        text,
                        if *number {
                            theme::mono(tokens::FS_0, FontWeight::Regular)
                        } else {
                            theme::sans(tokens::FS_0, FontWeight::Regular)
                        },
                    )
                })
                .fold(head, f32::max);
            let left = if index == 0 {
                TABLE_EDGE_INSET
            } else {
                CELL_PAD
            };
            let right = if index + 1 == count {
                TABLE_EDGE_INSET
            } else {
                CELL_PAD
            };
            let fitted =
                Column::name(column.head, widest.ceil() + left + right).align(column.align);
            match column.tooltip {
                Some(tooltip) => fitted.tooltip(tooltip),
                None => fitted,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every row of a column prints at one precision, the fewest decimals
    /// that print each value to the step, in the decade of the largest.
    #[test]
    fn a_column_prints_every_row_at_one_precision() {
        let column = |values: &[f64], step, unit| {
            at_resolution(
                &values.iter().copied().map(Some).collect::<Vec<_>>(),
                step,
                unit,
            )
        };
        let harmonics = column(&[2000.0003, 3000.0, 4000.0], 10.0, "Hz");
        assert_eq!(harmonics.unit, "kHz");
        assert_eq!(
            harmonics.cells,
            [Some("2".into()), Some("3".into()), Some("4".into())]
        );
        let between = column(&[2100.0, 3150.0, 4200.0], 10.0, "Hz");
        assert_eq!(
            between.cells,
            [
                Some("2.10".into()),
                Some("3.15".into()),
                Some("4.20".into())
            ]
        );
        let levels = column(&[-40.000_01, -52.041_2, -184.831_7], HUNDREDTH, "dBc");
        assert_eq!(levels.unit, "dBc");
        assert_eq!(
            levels.cells,
            [
                Some("\u{2212}40.00".into()),
                Some("\u{2212}52.04".into()),
                Some("\u{2212}184.83".into())
            ]
        );
        let whole = column(&[-6.0, 0.0, -0.001], HUNDREDTH, "dBV");
        assert_eq!(
            whole.cells,
            [Some("\u{2212}6".into()), Some("0".into()), Some("0".into())]
        );
        assert_eq!(decade_step(50.0), 10.0);
        assert_eq!(decade_step(128.2), 100.0);
    }
}
