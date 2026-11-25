//! TABLE — the retained samples of one analysis, as rows.
//!
//! The table reads the same [`StripModel`](crate::waves::StripModel) the WAVES stage
//! plots, so a column can never name a trace the plot does not draw, or
//! report a value in a different unit than the curve above it. What the
//! viewer adds is the part a plot cannot give: exact retained samples, and
//! honest control over which of them you are looking at.
//!
//! Three controls shape the row set — a stride, an optional window centred
//! on cursor A, and the column selection — and the footer always states the
//! resulting count against the retained total, so "showing 200" can never
//! be mistaken for "there are 200". Any number of signals are columns; a
//! head sorts by its column, a search keeps the rows that contain what was
//! typed, and Go to selects the row nearest an X value.

use crate::{
    fft::view::columns::{Fitting, fitted, fitted_to},
    presentation::{table_row_menu, well_hint, with_table_state},
    waves::StripModel,
};
use egui::Ui;
use rspice_results::{
    analysis_payload::AnalysisResultPayload,
    result_presentation::{AnalysisPresentationKey, TracePresentationKey},
    waveform::RetainedTrace,
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{
        chip,
        table::{Align, Cell, DataTable, Selection, SortKey},
    },
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::borrow::Cow;
type AnalysisResult = rspice_results::analysis_result::AnalysisResult<RetainedTrace>;

pub struct SampleTable<'a> {
    pub model: &'a StripModel,
    pub cursor_a: Option<f64>,
    pub cursor_b: Option<f64>,
    pub significant_digits: usize,
    pub quantity_policy: QuantityPresentationPolicy,
}

pub struct TableResponse {
    pub clicked_sample: Option<f64>,
    pub status: String,
}

pub struct TableControls<'a> {
    /// The analysis the data browser selected, as the table draws it.
    pub model: Option<&'a StripModel>,
    pub cursor_strip: Option<usize>,
    pub cursor_a: Option<f64>,
    pub status: Option<&'a str>,
}

/// Rows read from each end of the selection to size the columns: a value
/// column is as wide as the widest of them and its head.
const SIZING_ROWS: usize = 32;

struct TypedOpRow {
    source: String,
    quantity: String,
    value: String,
    unit: String,
    status: String,
}

fn dc_op_row_count(analysis: &AnalysisResult) -> usize {
    analysis.dc_op.as_ref().map_or(0, |op| {
        op.node_voltages.len() + op.branch_currents.len() + op.power_dissipation.len()
    })
}

/// Rows the retained solver state contributes to the typed table.
///
/// It is a fallback: the DC block is the authority on the operating point, and
/// the raw MNA vector is offered only when that block yielded nothing to show.
/// Beyond that, a solution entry no retained name can address is not a row —
/// the producer has no quantity to put in the row's second column.
///
/// The count and the producer decided both clauses independently, and
/// disagreed on both. The count dropped the solver state when the DC block was
/// *empty*, the producer dropped it when the block was merely *present*, and
/// neither bounded the solution by the names addressing it. An analysis
/// retaining an empty `dc_op` therefore counted rows nothing could produce,
/// and the table drew that many blank lines under the last real one.
fn mna_solution_row_count(
    dc_rows: usize,
    mna_node_names: &[String],
    mna_branch_names: &[String],
    mna_solution: &[f64],
) -> usize {
    if dc_rows > 0 {
        return 0;
    }
    mna_solution
        .len()
        .min(mna_node_names.len().saturating_add(mna_branch_names.len()))
}

pub fn operating_point_row_count(analysis: &AnalysisResult) -> usize {
    let dc_rows = dc_op_row_count(analysis);
    let device_rows = analysis.device_op.as_ref().map_or(0, |report| {
        report
            .entries
            .iter()
            .map(|entry| 1 + entry.params.len())
            .sum()
    });
    let payload_rows = match analysis.result_payload.as_ref() {
        Some(AnalysisResultPayload::OperatingPoint {
            selected_devices,
            violation_devices,
            mna_node_names,
            mna_branch_names,
            mna_solution,
            previous_state,
            ..
        }) => {
            1 + usize::from(previous_state.is_some()) * 4
                + selected_devices.len()
                + violation_devices.len()
                + mna_solution_row_count(dc_rows, mna_node_names, mna_branch_names, mna_solution)
        }
        _ => 0,
    };
    dc_rows + device_rows + payload_rows
}

fn operating_point_row_at(analysis: &AnalysisResult, mut index: usize) -> Option<TypedOpRow> {
    if let Some(op) = &analysis.dc_op {
        for (source, values) in [
            ("Node", op.node_voltages.as_slice()),
            ("Branch", op.branch_currents.as_slice()),
            ("Device", op.power_dissipation.as_slice()),
        ] {
            if index < values.len() {
                let value = &values[index];
                return Some(TypedOpRow {
                    source: source.to_owned(),
                    quantity: value.name.clone(),
                    value: format!("{:.17e}", value.value),
                    unit: value.unit.clone(),
                    status: "retained".to_owned(),
                });
            }
            index -= values.len();
        }
    }
    if let Some(report) = &analysis.device_op {
        for device in &report.entries {
            if index == 0 {
                return Some(TypedOpRow {
                    source: device.device_kind.to_owned(),
                    quantity: device.name.clone(),
                    value: device.region.unwrap_or("—").to_owned(),
                    unit: String::new(),
                    status: "device operating point".to_owned(),
                });
            }
            index -= 1;
            if index < device.params.len() {
                let (name, value) = device.params[index];
                return Some(TypedOpRow {
                    source: device.name.clone(),
                    quantity: name.to_owned(),
                    value: format!("{value:.17e}"),
                    unit: String::new(),
                    status: "retained".to_owned(),
                });
            }
            index -= device.params.len();
        }
    }
    let AnalysisResultPayload::OperatingPoint {
        temperature_celsius,
        selected_devices,
        violation_devices,
        mna_node_names,
        mna_branch_names,
        mna_solution,
        initial_guess,
        previous_state,
        ..
    } = analysis.result_payload.as_ref()?
    else {
        return None;
    };
    if index == 0 {
        return Some(TypedOpRow {
            source: "Analysis".to_owned(),
            quantity: "Temperature".to_owned(),
            value: format!("{temperature_celsius:.17e}"),
            unit: "°C".to_owned(),
            status: "retained".to_owned(),
        });
    }
    index -= 1;
    if let Some(previous) = previous_state {
        if index < 4 {
            let (quantity, value, unit) = match index {
                0 => (
                    "Initial guess",
                    match initial_guess {
                        rspice_results::operating_point::OperatingPointInitialGuessEvidence::PreviousCompatible => {
                            "Previous solution, compatible circuit".to_owned()
                        }
                        _ => "Previous converged solution".to_owned(),
                    },
                    "",
                ),
                1 => (
                    "Previous source",
                    previous.source_content_digest.to_string(),
                    "sha256",
                ),
                2 => (
                    "Previous snapshot",
                    previous.producer_snapshot_digest.to_string(),
                    "sha256",
                ),
                _ => (
                    "Previous result",
                    previous.producer_result_digest.to_string(),
                    "sha256",
                ),
            };
            return Some(TypedOpRow {
                source: "Startup".to_owned(),
                quantity: quantity.to_owned(),
                value,
                unit: unit.to_owned(),
                status: "initial guess; solved again".to_owned(),
            });
        }
        index -= 4;
    }
    if index < selected_devices.len() {
        return Some(TypedOpRow {
            source: "Selection".to_owned(),
            quantity: selected_devices[index].clone(),
            value: "selected".to_owned(),
            unit: String::new(),
            status: "device detail".to_owned(),
        });
    }
    index -= selected_devices.len();
    if index < violation_devices.len() {
        return Some(TypedOpRow {
            source: "SOA".to_owned(),
            quantity: violation_devices[index].clone(),
            value: "violation".to_owned(),
            unit: String::new(),
            status: "attention".to_owned(),
        });
    }
    index -= violation_devices.len();
    if index
        >= mna_solution_row_count(
            dc_op_row_count(analysis),
            mna_node_names,
            mna_branch_names,
            mna_solution,
        )
    {
        return None;
    }
    let (source, quantity) = if index < mna_node_names.len() {
        ("MNA node", mna_node_names[index].clone())
    } else {
        let branch = index.checked_sub(mna_node_names.len())?;
        ("MNA branch", mna_branch_names.get(branch)?.clone())
    };
    Some(TypedOpRow {
        source: source.to_owned(),
        quantity,
        value: format!("{:.17e}", mna_solution[index]),
        unit: String::new(),
        status: "retained solver state".to_owned(),
    })
}

/// One operating-point row as "Copy row" puts it on the clipboard: the value
/// with its unit in the format Copied values chose, at full precision.
fn copied_operating_point_row(
    row: &TypedOpRow,
    quantity_policy: &QuantityPresentationPolicy,
) -> String {
    // A value without a unit stays as it is: an engineering prefix with no
    // unit after it would read as a unit of its own.
    let value = match row.value.parse::<f64>() {
        Ok(value) if value.is_finite() && !row.unit.is_empty() => {
            quantity_policy.copy_si_value(value, &row.unit)
        }
        _ => format!("{} {}", row.value, row.unit).trim_end().to_owned(),
    };
    format!("{}\t{}\t{value}\t{}", row.source, row.quantity, row.status)
}

pub fn show_operating_point_table(
    ui: &mut Ui,
    analysis: &AnalysisResult,
    row_count: usize,
    quantity_policy: &QuantityPresentationPolicy,
) {
    let rows: Vec<Vec<Cell>> = (0..row_count)
        .map(|row| {
            let Some(value) = operating_point_row_at(analysis, row) else {
                return vec![Cell::absent(); 5];
            };
            let exact = match value.value.parse::<f64>() {
                Ok(number) if number.is_finite() => {
                    Cell::number(value.value).sort_by(SortKey::Number(number))
                }
                _ => Cell::number(value.value),
            };
            vec![
                Cell::number(value.source),
                Cell::number(value.quantity),
                exact,
                Cell::number(value.unit),
                Cell::number(value.status),
            ]
        })
        .collect();
    let columns = fitted_to(
        ui,
        &[
            ("Source", Align::Left, true),
            ("Quantity", Align::Left, true),
            ("Exact value", Align::Right, true),
            ("Unit", Align::Left, true),
            ("Status", Align::Left, true),
        ],
        &rows,
    );
    let revision = egui::Id::new((
        "rspice.results.table.operating-point",
        analysis.id,
        row_count,
    ))
    .value();
    let response = with_table_state(ui, "operating-point", |ui, state| {
        DataTable::new("rspice.results.table.operating-point", &columns, row_count)
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(ui.available_height())
            .show(ui, |row| rows[row].clone())
    });
    table_row_menu(ui, "operating-point", response.context_menu, |ui, row| {
        if ui.button("Copy exact row").clicked() {
            if let Some(value) = operating_point_row_at(analysis, row) {
                ui.ctx()
                    .copy_text(copied_operating_point_row(&value, quantity_policy));
            }
            ui.close();
        }
    });
}

pub fn show_artifact(
    ui: &mut Ui,
    title: &str,
    text: &Result<String, String>,
    lines: &[(usize, usize)],
) {
    let exact = match text.as_ref() {
        Ok(exact) => exact,
        Err(message) => {
            well_hint(ui, message);
            return;
        }
    };
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        );
        chip(ui, &format!("{} exact records", lines.len()), false);
    });
    ui.add_space(6.0);
    let row_cells = |row: usize| -> Vec<Cell> {
        let (start, end) = lines[row];
        #[allow(clippy::cast_precision_loss)]
        let number = Cell::number((row + 1).to_string()).sort_by(SortKey::Number((row + 1) as f64));
        vec![number, Cell::number(&exact[start..end])]
    };
    // The evidence is in mono, so its longest line is its widest: the
    // column is sized from it and from the lines at either end.
    let longest = lines
        .iter()
        .enumerate()
        .max_by_key(|(_, (start, end))| end - start)
        .map(|(row, _)| row);
    let sizing: Vec<Vec<Cell>> = (0..lines.len().min(SIZING_ROWS))
        .chain(lines.len().saturating_sub(SIZING_ROWS)..lines.len())
        .chain(longest)
        .map(row_cells)
        .collect();
    let columns = fitted_to(
        ui,
        &[
            ("Line", Align::Right, true),
            ("Evidence", Align::Left, true),
        ],
        &sizing,
    );
    let revision = egui::Id::new(("rspice.results.table.artifact", title, lines.len())).value();
    let response = with_table_state(ui, "artifact", |ui, state| {
        DataTable::new("rspice.results.table.artifact", &columns, lines.len())
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(ui.available_height())
            .show(ui, row_cells)
    });
    table_row_menu(ui, "artifact", response.context_menu, |ui, row| {
        if ui.button("Copy exact line").clicked() {
            if let Some(&(start, end)) = lines.get(row) {
                ui.ctx().copy_text(exact[start..end].to_owned());
            }
            ui.close();
        }
        if ui.button("Copy all exact evidence").clicked() {
            ui.ctx().copy_text(exact.clone());
            ui.close();
        }
    });
}

/// Rows kept either side of cursor A in around-cursor mode.
const CURSOR_SPAN: usize = 40;

/// Stride values the docbar control cycles through.
pub const STRIDES: [usize; 5] = [1, 2, 5, 10, 100];

/// Which retained samples the table shows, and as what columns.
///
/// Defaults to every row of the first analysis: a table that silently
/// decimated its own rows would be the one thing this viewer exists to
/// avoid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableView {
    /// The analysis the chosen columns belong to. The data browser picks
    /// the analysis the table shows; when it picks another, the columns
    /// return to that analysis' signals.
    pub analysis: Option<AnalysisPresentationKey>,
    /// Show every `stride`-th retained row.
    pub stride: usize,
    /// Selected trace indices (into the model's active-run prefix). Empty
    /// means every visible signal.
    pub columns: Vec<TracePresentationKey>,
    /// Restrict the rows to a window centred on cursor A.
    pub around_cursor: bool,
}

impl Default for TableView {
    fn default() -> Self {
        Self {
            analysis: None,
            stride: 1,
            columns: Vec::new(),
            around_cursor: false,
        }
    }
}

impl TableView {
    /// Advance the stride control to the next value, wrapping.
    pub fn cycle_stride(&mut self) {
        let next = STRIDES
            .iter()
            .position(|&stride| stride == self.stride)
            .map_or(0, |index| (index + 1) % STRIDES.len());
        self.stride = STRIDES[next];
    }

    /// Toggle one trace's column. Selecting the last remaining column off
    /// returns to the automatic selection rather than an empty table.
    pub fn toggle_column(
        &mut self,
        trace: TracePresentationKey,
        automatic: &[TracePresentationKey],
    ) {
        if self.columns.is_empty() {
            self.columns = automatic.to_vec();
        }
        if let Some(at) = self.columns.iter().position(|key| *key == trace) {
            self.columns.remove(at);
        } else {
            self.columns.push(trace);
        }
        if self.columns.is_empty() {
            self.columns = automatic.to_vec();
        }
    }
}

/// Trace indices the table shows when the user has chosen none: every
/// visible signal in the active run.
fn automatic_columns(model: &StripModel) -> Vec<TracePresentationKey> {
    model
        .visible_signal_indices()
        .filter_map(|index| model.trace_presentation_key(index))
        .collect()
}

/// Constant-size description of the retained rows selected by the controls.
/// Visible row ordinals are translated to retained sample indices lazily, so
/// opening a multi-million-point dataset does not allocate a second index list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowSelection {
    start: usize,
    end: usize,
    stride: usize,
    retained: usize,
}

impl RowSelection {
    const fn empty() -> Self {
        Self {
            start: 0,
            end: 0,
            stride: 1,
            retained: 0,
        }
    }

    const fn len(self) -> usize {
        if self.start >= self.end {
            0
        } else {
            (self.end - self.start).div_ceil(self.stride)
        }
    }

    const fn is_empty(self) -> bool {
        self.len() == 0
    }

    fn sample_at(self, visible_row: usize) -> Option<usize> {
        let sample = self
            .start
            .checked_add(visible_row.checked_mul(self.stride)?)?;
        (sample < self.end).then_some(sample)
    }

    #[cfg(test)]
    fn indices(self) -> impl Iterator<Item = usize> {
        (0..self.len()).filter_map(move |row| self.sample_at(row))
    }
}

fn selected_rows(
    model: &StripModel,
    view: &TableView,
    cursor: Option<f64>,
    on_scan: &mut impl FnMut(),
) -> RowSelection {
    let Some(grid) = model.sample_grid() else {
        return RowSelection::empty();
    };
    let retained = grid.len();
    let stride = view.stride.max(1);
    // The window is taken first and the stride applied inside it, so
    // "around A, every 10th" means 40 rows either side of A, not the whole
    // run decimated and then cropped.
    let (start, end) = match cursor.filter(|_| view.around_cursor) {
        Some(x) => {
            let centre =
                nearest_sample_index(&grid, x, model.grid_is_ascending(), on_scan).unwrap_or(0);
            (
                centre.saturating_sub(CURSOR_SPAN),
                (centre + CURSOR_SPAN + 1).min(retained),
            )
        }
        None => (0, retained),
    };
    RowSelection {
        start,
        end,
        stride,
        retained,
    }
}

fn resolved_columns(
    model: &StripModel,
    saved: &[TracePresentationKey],
    automatic: &[TracePresentationKey],
) -> Vec<usize> {
    let selected = if saved.is_empty() { automatic } else { saved };
    let resolved = selected
        .iter()
        .filter_map(|key| model.trace_index_for_key(key))
        .collect::<Vec<_>>();
    if resolved.is_empty() && !saved.is_empty() {
        automatic
            .iter()
            .filter_map(|key| model.trace_index_for_key(key))
            .collect()
    } else {
        resolved
    }
}

/// The retained sample a cursor sits on.
///
/// A million-point transient makes this the difference between a bisection
/// and a million comparisons, three times a frame, for a cursor that has not
/// moved. `ascending` is the strip's own verdict on its grid, resolved when
/// the model was built; a grid it cannot vouch for keeps the scan, because a
/// parametric sweep is under no obligation to be monotonic.
fn nearest_sample_index(
    grid: &[f64],
    x: f64,
    ascending: bool,
    on_scan: &mut impl FnMut(),
) -> Option<usize> {
    if !ascending {
        on_scan();
        return grid
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_finite())
            .min_by(|(_, left), (_, right)| (**left - x).abs().total_cmp(&(**right - x).abs()))
            .map(|(index, _)| index);
    }
    if grid.is_empty() {
        return None;
    }
    // The first sample at or past the cursor, and the last one before it:
    // the nearest retained sample is one of the two. An event history
    // retains repeated coordinates, so each candidate is resolved to the
    // *first* index carrying its value — the scan reported the first of
    // several equally near samples, and so must this.
    let at_or_after = grid.partition_point(|value| *value < x);
    let before = at_or_after
        .checked_sub(1)
        .map(|last| grid.partition_point(|value| *value < grid[last]));
    match (before, grid.get(at_or_after)) {
        (None, Some(_)) => Some(at_or_after),
        (Some(before), None) => Some(before),
        (Some(before), Some(after)) => {
            // Ties go to the lower index, as the scan's `min_by` did.
            Some(if (x - grid[before]).abs() <= (*after - x).abs() {
                before
            } else {
                at_or_after
            })
        }
        (None, None) => None,
    }
}

/// Where the bar keeps what the reader searched the table for.
fn search_id() -> egui::Id {
    egui::Id::new("rspice.results.table.search")
}

/// Where the bar leaves the X value the reader asked to go to, until the
/// table takes it.
fn go_to_id() -> egui::Id {
    egui::Id::new("rspice.results.table.go-to")
}

/// Where the sheet keeps the rows whose values print widest, with the rows
/// they were found among.
fn widest_id() -> egui::Id {
    egui::Id::new("rspice.results.table.widest")
}

/// The row positions whose words are the widest each value column prints:
/// its most negative and most positive values, and the values of either
/// sign nearest zero, which take the smallest prefix.
fn widest_rows(model: &StripModel, rows: RowSelection, columns: &[usize]) -> Vec<usize> {
    let mut widest = Vec::new();
    for &trace in columns {
        let Some(values) = model.sample_values(trace) else {
            continue;
        };
        // Most negative, most positive, smallest positive, negative
        // nearest zero.
        let mut picks: [Option<(usize, f64)>; 4] = [None; 4];
        for position in 0..rows.len() {
            let Some(&value) = rows
                .sample_at(position)
                .and_then(|sample| values.get(sample))
            else {
                continue;
            };
            if !value.is_finite() {
                continue;
            }
            let better = [
                picks[0].is_none_or(|(_, held)| value < held),
                picks[1].is_none_or(|(_, held)| value > held),
                value > 0.0 && picks[2].is_none_or(|(_, held)| value < held),
                value < 0.0 && picks[3].is_none_or(|(_, held)| value > held),
            ];
            for (pick, better) in picks.iter_mut().zip(better) {
                if better {
                    *pick = Some((position, value));
                }
            }
        }
        widest.extend(picks.iter().flatten().map(|(position, _)| *position));
    }
    widest.sort_unstable();
    widest.dedup();
    widest
}

/// The visible row that shows `sample`, or the nearest one the stride
/// keeps.
fn row_of_sample(rows: RowSelection, sample: usize) -> Option<usize> {
    if rows.is_empty() {
        return None;
    }
    let offset = sample.clamp(rows.start, rows.end.saturating_sub(1)) - rows.start;
    let stride = rows.stride.max(1);
    Some(((offset + stride / 2) / stride).min(rows.len() - 1))
}

pub fn show_samples(
    ui: &mut Ui,
    source: SampleTable<'_>,
    view: &TableView,
    display_name: impl for<'a> Fn(&'a str) -> Cow<'a, str>,
    mut on_scan: impl FnMut(),
) -> Option<TableResponse> {
    let c = Tokens::get(ui.ctx()).color;
    let model = source.model;
    let cursor = source.cursor_a;
    let significant_digits = source.significant_digits;
    let quantity_policy = source.quantity_policy;
    let automatic = automatic_columns(model);
    let columns = resolved_columns(model, &view.columns, &automatic);
    if columns.is_empty() {
        well_hint(ui, "No columns — enable a signal from the Columns menu");
        return None;
    }

    let rows = selected_rows(model, view, cursor, &mut on_scan);
    if rows.is_empty() {
        well_hint(ui, "This analysis retained no samples");
        return None;
    }

    let grid = model.sample_grid().unwrap_or_default();
    let ascending = model.grid_is_ascending();
    let cursor_a_row = cursor.and_then(|x| nearest_sample_index(&grid, x, ascending, &mut on_scan));
    let cursor_b_row = source
        .cursor_b
        .and_then(|x| nearest_sample_index(&grid, x, ascending, &mut on_scan));

    // A column heading names the conductor the design drew; the trace keeps
    // the engine's own name as its identity and as the key it is exported
    // under, exactly as the waveform legend does.
    let x_heading = model.x_axis_heading();
    let value_heads: Vec<String> = columns
        .iter()
        .map(|&trace| {
            model.trace_heading(trace).map_or_else(
                || "Signal".to_owned(),
                |(name, _)| display_name(name).into_owned(),
            )
        })
        .collect();
    let row_cells = |row_position: usize| -> Vec<Cell> {
        let Some(sample) = rows.sample_at(row_position) else {
            return vec![Cell::absent(); columns.len() + 2];
        };
        let at_a = cursor_a_row == Some(sample);
        let at_b = cursor_b_row == Some(sample);
        let flag = match (at_a, at_b) {
            (true, true) => "A/B  ",
            (true, false) => "A  ",
            (false, true) => "B  ",
            (false, false) => "",
        };
        #[allow(clippy::cast_precision_loss)]
        let mut cells = vec![
            Cell::number(format!("{flag}{sample}")).sort_by(SortKey::Number(sample as f64)),
            {
                let x =
                    Cell::quantity(model.format_x_at(sample, significant_digits, quantity_policy));
                match grid.get(sample) {
                    Some(&x_value) => x.sort_by(SortKey::Number(x_value)),
                    None => x,
                }
            },
        ];
        for &trace in &columns {
            // A trace shorter than the grid says so rather than borrowing a
            // neighbouring sample's value.
            let cell = match model.format_sample(trace, sample, significant_digits, quantity_policy)
            {
                Some(text) => {
                    let cell = Cell::quantity(text);
                    match model.sample_value(trace, sample) {
                        Some(value) => cell.sort_by(SortKey::Number(value)),
                        None => cell,
                    }
                }
                None => Cell::absent(),
            };
            cells.push(cell);
        }
        if at_a || at_b {
            cells = cells
                .into_iter()
                .map(|cell| cell.wash(c.accent_dim))
                .collect();
        }
        cells
    };

    // The rows' words change with the analysis, the row set, the columns,
    // the digits and the cursors.
    let revision = egui::Id::new((
        "rspice.results.table.rows",
        model.analysis_key(),
        (rows.start, rows.end, rows.stride),
        &columns,
        significant_digits,
        cursor_a_row,
        cursor_b_row,
    ))
    .value();
    // Each column is as wide as its head, its widest values and the rows at
    // either end of the selection, and the columns share what the pane
    // leaves, so the table runs the pane's width.
    let widest =
        match ui.data(|data| data.get_temp::<(u64, std::sync::Arc<Vec<usize>>)>(widest_id())) {
            Some((key, widest)) if key == revision => widest,
            _ => {
                let widest = std::sync::Arc::new(widest_rows(model, rows, &columns));
                ui.data_mut(|data| {
                    data.insert_temp(widest_id(), (revision, std::sync::Arc::clone(&widest)))
                });
                widest
            }
        };
    let sizing: Vec<usize> = (0..rows.len().min(SIZING_ROWS))
        .chain(rows.len().saturating_sub(SIZING_ROWS)..rows.len())
        .chain(widest.iter().copied())
        .collect();
    let sized: Vec<Vec<Cell>> = sizing.iter().map(|&row| row_cells(row)).collect();
    let fitting: Vec<Fitting> = std::iter::once("Row".to_owned())
        .chain(std::iter::once(x_heading.clone()))
        .chain(value_heads.iter().cloned())
        .enumerate()
        .map(|(column, head)| Fitting {
            head,
            tooltip: None,
            align: Align::Right,
            cells: sized
                .iter()
                .map(|cells| (cells[column].text.clone(), true))
                .collect(),
        })
        .collect();
    let table_columns = fitted(ui, fitting);

    let search = ui
        .data(|data| data.get_temp::<String>(search_id()))
        .unwrap_or_default();
    let go_to = ui.data_mut(|data| data.remove_temp::<f64>(go_to_id()));
    let mut clicked_sample = None;
    with_table_state(ui, "samples", |ui, state| {
        state.filter = search;
        let mut reveal = false;
        if let Some(x) = go_to
            && let Some(sample) = nearest_sample_index(&grid, x, ascending, &mut on_scan)
            && let Some(row) = row_of_sample(rows, sample)
        {
            state.selection = Some(Selection {
                row,
                key: None,
                column: None,
            });
            reveal = true;
        }
        let response = DataTable::new("rspice.results.table.samples", &table_columns, rows.len())
            .state(state)
            .freeze(2)
            .revision(revision)
            .reveal_selection(reveal)
            .max_height(ui.available_height())
            .show(ui, row_cells);
        if response.clicked.is_some() {
            clicked_sample = response.clicked.and_then(|row| rows.sample_at(row));
        }
    });

    Some(TableResponse {
        clicked_sample: clicked_sample.and_then(|index| grid.get(index).copied()),
        status: status_line(rows.len(), rows.retained, view, cursor),
    })
}

pub fn inline_actions(
    ui: &mut Ui,
    source: TableControls<'_>,
    view: &mut TableView,
    display_name: impl for<'a> Fn(&'a str) -> Cow<'a, str>,
) {
    let t = Tokens::get(ui.ctx());
    let model = source.model;

    let stride = view.stride;
    let response = chip(ui, &format!("1/{stride}"), stride > 1)
        .on_hover_text("Row stride — show every Nth retained sample");
    if response.clicked() {
        view.cycle_stride();
    }

    // Around-cursor is offered only when there is a cursor to centre on,
    // so the control can never promise a window it cannot produce.
    let has_cursor = model.is_some_and(|model| source.cursor_strip == Some(model.analysis_index()))
        && source.cursor_a.is_some();
    let around = view.around_cursor;
    let response = ui
        .add_enabled_ui(has_cursor, |ui| chip(ui, "around A", around))
        .inner
        .on_hover_text(if has_cursor {
            "Window the rows on cursor A"
        } else {
            "Place cursor A on this analysis to window the rows on it"
        });
    if response.clicked() {
        view.around_cursor = !around;
    }

    if let Some(active) = model {
        let automatic = automatic_columns(active);
        let chosen = view.columns.clone();
        let effective = if chosen.is_empty() {
            automatic.clone()
        } else {
            chosen
        };
        // The menu offers the same headings the table paints, so a bus bit is
        // picked by the notation its bus was drawn in rather than by the deck
        // spelling it is toggled under.
        rspice_ui_kit::widgets::menu_button(ui, format!("Columns ({})", effective.len()), |ui| {
            for index in active.visible_signal_indices() {
                let Some((name, _)) = active.trace_heading(index) else {
                    continue;
                };
                let Some(key) = active.trace_presentation_key(index) else {
                    continue;
                };
                let on = effective.contains(&key);
                if ui.selectable_label(on, &*display_name(name)).clicked() {
                    view.toggle_column(key, &automatic);
                }
            }
        });
    }

    // The status keeps its room: the search and Go to fold into Find when
    // the bar cannot hold them beside it.
    let status_width = source.status.map_or(0.0, |status| {
        ui.painter()
            .layout_no_wrap(
                status.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                egui::Color32::PLACEHOLDER,
            )
            .size()
            .x
    });
    search_and_go_to(ui, status_width);

    if let Some(status) = source.status {
        ui.label(
            egui::RichText::new(status)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        );
    }
}

/// Width of the bar's search field.
const SEARCH_WIDTH: f32 = 120.0;
/// Width of the bar's Go to field.
const GO_TO_WIDTH: f32 = 96.0;

/// The bar's search, which keeps the rows that contain what was typed, and
/// Go to, which selects the row nearest the X value typed: in the bar when
/// it holds them and `reserve` beside them, else behind Find.
fn search_and_go_to(ui: &mut Ui, reserve: f32) {
    let spacing = ui.spacing().item_spacing.x;
    let room = ui.clip_rect().right() - ui.cursor().min.x;
    if room >= SEARCH_WIDTH + GO_TO_WIDTH + 3.0 * spacing + reserve {
        search_fields(ui);
        return;
    }
    let find = rspice_ui_kit::widgets::Button::new("Find")
        .menu()
        .accessible_label("Search the rows or go to an X value")
        .show(ui);
    egui::Popup::menu(&find)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.horizontal(search_fields);
        });
}

fn search_fields(ui: &mut Ui) {
    // The fields stand at the bar's control height, their words on its centre.
    let height = crate::chrome::ResultBarMetrics::of(ui).instrument_control;
    let mut search = ui
        .data(|data| data.get_temp::<String>(search_id()))
        .unwrap_or_default();
    let field = ui.add_sized(
        [SEARCH_WIDTH, height],
        egui::TextEdit::singleline(&mut search)
            .font(theme::field_mono())
            .vertical_align(egui::Align::Center)
            .hint_text("Search…"),
    );
    if field.changed() {
        ui.data_mut(|data| data.insert_temp(search_id(), search));
    }

    let draft = ui.id().with("rspice.results.table.go-to-draft");
    let mut text = ui
        .data(|data| data.get_temp::<String>(draft))
        .unwrap_or_default();
    let field = ui.add_sized(
        [GO_TO_WIDTH, height],
        egui::TextEdit::singleline(&mut text)
            .font(theme::field_mono())
            .vertical_align(egui::Align::Center)
            .hint_text("Go to X…"),
    );
    let committed = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    if committed
        && let Ok(x) = rspice_vocabulary::quantity::parse_engineering_value(text.trim())
        && x.is_finite()
    {
        ui.data_mut(|data| data.insert_temp(go_to_id(), x));
    }
    ui.data_mut(|data| data.insert_temp(draft, text));
}

pub fn operating_point_controls(ui: &mut Ui) {
    chip(ui, "Typed OP", true)
        .on_hover_text("Virtualized exact operating-point values; right-click a row to copy");
}

fn status_line(shown: usize, retained: usize, view: &TableView, cursor: Option<f64>) -> String {
    let mut parts = vec![format!("{shown} of {retained} retained rows")];
    if view.stride > 1 {
        parts.push(format!("stride {}", view.stride));
    }
    if view.around_cursor {
        parts.push(if cursor.is_some() {
            "window around A".to_owned()
        } else {
            "around A — no cursor placed".to_owned()
        });
    }
    parts.join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::{
        analysis_type::AnalysisType,
        operating_point::{DcOpResult, OperatingPointValue},
    };

    /// A copied operating-point row carries its value in the format Copied
    /// values chose, unit included, at full precision.
    #[test]
    fn a_copied_operating_point_row_follows_copied_values() {
        use rspice_vocabulary::quantity::CopiedValueFormat;

        let row = TypedOpRow {
            source: "Node".to_owned(),
            quantity: "V(out)".to_owned(),
            value: format!("{:.17e}", 1.5e-3),
            unit: "V".to_owned(),
            status: "converged".to_owned(),
        };
        let mut policy = QuantityPresentationPolicy {
            copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
            ..QuantityPresentationPolicy::default()
        };
        let engineering = copied_operating_point_row(&row, &policy);
        assert!(
            engineering.starts_with("Node\tV(out)\t1.5") && engineering.contains(" mV\tconverged"),
            "{engineering}"
        );
        policy.copied_value_format = CopiedValueFormat::ScientificNotationWithSiUnit;
        let scientific = copied_operating_point_row(&row, &policy);
        let value = scientific.split('\t').nth(2).expect("value column");
        let (number, unit) = value.split_once(' ').expect("value and unit");
        assert_eq!(unit, "V");
        assert_eq!(number.parse::<f64>().expect("a number"), 1.5e-3);
    }
    struct Fixture {
        model: StripModel,
        view: TableView,
    }
    fn fixture(samples: usize) -> Fixture {
        let x: Vec<f64> = (0..samples).map(|i| i as f64 * 1.0e-6).collect();
        let y: Vec<f64> = (0..samples).map(|i| i as f64).collect();
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0)
            .with_waveforms(vec![RetainedTrace::new("V(out)", x, y, "#fff")]);
        let mut run = rspice_results::run::SimulationRun::new(
            1,
            0.0,
            rspice_results::run::ExecutionTarget::LocalDesktop,
        );
        run.add_analysis(analysis);
        let model = crate::waves::build_models(
            &[&run],
            &Default::default(),
            &mut Default::default(),
            &Tokens::default(),
            crate::waves::ProjectionOptions {
                phase_continuous: false,
                complex_display: Default::default(),
                ac: Default::default(),
                selection: None,
                hidden_family_traces: &Default::default(),
                waveform_visibility: &Default::default(),
                signals: None,
            },
            |_| None,
        )
        .remove(0);
        Fixture {
            model,
            view: TableView::default(),
        }
    }
    fn rows_for(state: &mut Fixture, cursor: Option<f64>) -> (Vec<usize>, usize) {
        let rows = selected_rows(&state.model, &state.view, cursor, &mut || {});
        (rows.indices().collect(), rows.retained)
    }
    fn mna_payload() -> AnalysisResultPayload {
        use rspice_results::operating_point::{
            OperatingPointAccuracyEvidence, OperatingPointAnnotationEvidence,
            OperatingPointDeviceDetailEvidence, OperatingPointHomotopyEvidence,
            OperatingPointInitialGuessEvidence, OperatingPointNodeInitializationEvidence,
            OperatingPointProcessEvidence, OperatingPointSaveDeviceEvidence,
            OperatingPointTemperatureEvidence,
        };
        AnalysisResultPayload::OperatingPoint {
            temperature_mode: OperatingPointTemperatureEvidence::Nominal27C,
            temperature_celsius: 27.0,
            initial_guess: OperatingPointInitialGuessEvidence::Automatic,
            node_initialization: OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
            homotopy: OperatingPointHomotopyEvidence::Adaptive,
            annotation: OperatingPointAnnotationEvidence::VoltagesAndCurrents,
            device_detail: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            save_device_op: OperatingPointSaveDeviceEvidence::Enabled,
            accuracy: OperatingPointAccuracyEvidence::Balanced,
            selected_devices: Vec::new(),
            violation_devices: Vec::new(),
            violation_source_content_digest: None,
            validated_startup_directives: 0,
            mna_node_names: vec!["in".to_owned(), "out".to_owned()],
            mna_branch_names: vec!["V1".to_owned()],
            mna_solution: vec![1.0, 0.5, -0.5e-3],
            effective_source_content_digest: None,
            previous_state: None,
            run_point_index: 0,
            run_point_count: 1,
            run_point_process: OperatingPointProcessEvidence::TT,
            run_point_supply_voltage: None,
            run_point_nominal_supply_voltage: None,
        }
    }

    #[test]
    fn typed_operating_point_rows_preserve_exact_values_without_waveforms() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP", 0.0);
        analysis.dc_op = Some(DcOpResult {
            node_voltages: vec![OperatingPointValue {
                name: "V(out)".to_owned(),
                value: 1.234_567_890_123_456,
                unit: "V".to_owned(),
            }],
            branch_currents: vec![OperatingPointValue {
                name: "I(V1)".to_owned(),
                value: -2.5e-3,
                unit: "A".to_owned(),
            }],
            power_dissipation: Vec::new(),
        });

        assert_eq!(operating_point_row_count(&analysis), 2);
        let voltage = operating_point_row_at(&analysis, 0).expect("voltage row");
        assert_eq!(voltage.quantity, "V(out)");
        assert_eq!(voltage.value, "1.23456789012345602e0");
        assert_eq!(voltage.unit, "V");
        let current = operating_point_row_at(&analysis, 1).expect("current row");
        assert_eq!(current.quantity, "I(V1)");
        assert_eq!(current.value, "-2.50000000000000005e-3");
    }

    #[test]
    fn the_typed_operating_point_count_and_its_rows_share_one_gate() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP", 0.0)
            .with_result_payload(mna_payload());
        analysis.dc_op = Some(DcOpResult::default());
        let Some(AnalysisResultPayload::OperatingPoint {
            initial_guess,
            previous_state,
            ..
        }) = &mut analysis.result_payload
        else {
            panic!("OP")
        };
        *initial_guess =
            rspice_results::operating_point::OperatingPointInitialGuessEvidence::PreviousCompatible;
        *previous_state = Some(
            rspice_results::operating_point::OperatingPointPreviousStateEvidence {
                source_content_digest: rspice_vocabulary::product::ContentDigest::from_bytes(
                    [1; 32],
                ),
                producer_snapshot_digest: rspice_vocabulary::product::ContentDigest::from_bytes(
                    [2; 32],
                ),
                producer_result_digest: rspice_vocabulary::product::ContentDigest::from_bytes(
                    [3; 32],
                ),
            },
        );
        assert_eq!(
            operating_point_row_at(&analysis, 1).unwrap().value,
            "Previous solution, compatible circuit"
        );
        assert_eq!(
            operating_point_row_at(&analysis, 4).unwrap().value,
            rspice_vocabulary::product::ContentDigest::from_bytes([3; 32]).to_string()
        );
        let count = operating_point_row_count(&analysis);
        for index in 0..count {
            assert!(
                operating_point_row_at(&analysis, index).is_some(),
                "row {index} of {count} is counted but cannot be produced"
            );
        }
    }

    #[test]
    fn unaddressable_solver_state_is_not_counted_as_a_row() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP", 0.0)
            .with_result_payload(mna_payload());
        let Some(AnalysisResultPayload::OperatingPoint {
            mna_branch_names, ..
        }) = analysis.result_payload.as_mut()
        else {
            panic!("the fixture retains an operating-point payload");
        };
        // Three entries in the solution, but only the two node names left to
        // address them by.
        mna_branch_names.clear();

        let count = operating_point_row_count(&analysis);
        for index in 0..count {
            assert!(
                operating_point_row_at(&analysis, index).is_some(),
                "row {index} of {count} is counted but cannot be produced"
            );
        }
    }

    #[test]
    fn the_table_lists_every_retained_row_by_default() {
        let mut state = fixture(100);
        let (rows, retained) = rows_for(&mut state, None);
        assert_eq!(retained, 100);
        assert_eq!(rows.len(), 100, "a sample table decimates nothing unasked");
        assert_eq!(rows.first(), Some(&0));
        assert_eq!(rows.last(), Some(&99));
    }

    #[test]
    fn stride_thins_the_rows_without_hiding_the_total() {
        let mut state = fixture(100);
        state.view.stride = 5;
        let (rows, retained) = rows_for(&mut state, None);
        assert_eq!(rows.len(), 20);
        assert_eq!(rows[1], 5, "stride steps the grid, it does not resample it");
        assert_eq!(
            retained, 100,
            "the retained total is reported against the thinned set"
        );
    }

    #[test]
    fn around_cursor_windows_the_rows_then_strides_inside_the_window() {
        let mut state = fixture(100);
        state.view.around_cursor = true;
        let (rows, _) = rows_for(&mut state, Some(50.0e-6));
        assert_eq!(rows.first(), Some(&(50 - CURSOR_SPAN)));
        assert_eq!(rows.last(), Some(&(50 + CURSOR_SPAN)));

        // Striding applies inside the window: "around A, every 10th" must
        // not mean "the whole run decimated, then cropped".
        state.view.stride = 10;
        let (rows, _) = rows_for(&mut state, Some(50.0e-6));
        assert_eq!(rows.first(), Some(&10));
        assert!(rows.iter().all(|row| (10..=90).contains(row)));
    }

    #[test]
    fn around_cursor_without_a_cursor_shows_the_whole_run() {
        let mut state = fixture(100);
        state.view.around_cursor = true;
        let (rows, _) = rows_for(&mut state, None);
        assert_eq!(
            rows.len(),
            100,
            "the mode cannot window on a cursor that was never placed"
        );
    }

    #[test]
    fn the_stride_control_cycles_and_returns() {
        let mut view = TableView::default();
        assert_eq!(view.stride, 1);
        for expected in STRIDES.iter().skip(1) {
            view.cycle_stride();
            assert_eq!(view.stride, *expected);
        }
        view.cycle_stride();
        assert_eq!(view.stride, 1);
    }

    #[test]
    fn nearest_sample_uses_the_exact_retained_grid() {
        let grid = [0.0, 1.0e-6, 2.5e-6, 9.0e-6];
        for ascending in [false, true] {
            assert_eq!(
                nearest_sample_index(&grid, 2.4e-6, ascending, &mut || {}),
                Some(2)
            );
            assert_eq!(
                nearest_sample_index(&grid, 8.0e-6, ascending, &mut || {}),
                Some(3)
            );
            assert_eq!(nearest_sample_index(&[], 1.0, ascending, &mut || {}), None);
        }
    }

    #[test]
    fn bisecting_the_grid_names_the_same_sample_the_scan_did() {
        let grids: [&[f64]; 5] = [
            &[0.0, 1.0e-6, 2.5e-6, 9.0e-6],
            // Ties either side: the scan kept the first minimum.
            &[0.0, 2.0, 4.0, 6.0],
            // Repeated coordinates, which an event history retains.
            &[0.0, 1.0, 1.0, 1.0, 3.0],
            &[5.0],
            &[],
        ];
        for grid in grids {
            let mut probes = vec![f64::NEG_INFINITY, -1.0, 0.5, 1.0, 3.0, 1.0e9];
            probes.extend(grid.iter().flat_map(|value| {
                [
                    *value - 1.0,
                    *value,
                    *value + 1.0,
                    value.midpoint(*value + 2.0),
                ]
            }));
            for probe in probes {
                assert_eq!(
                    nearest_sample_index(grid, probe, true, &mut || {}),
                    nearest_sample_index(grid, probe, false, &mut || {}),
                    "grid {grid:?} at {probe}"
                );
            }
        }
    }

    #[test]
    fn a_non_monotonic_grid_is_not_bisected() {
        let grid = [0.0, 9.0, 1.0, 8.0];
        assert_eq!(nearest_sample_index(&grid, 1.1, false, &mut || {}), Some(2));
    }

    #[test]
    fn the_column_chooser_never_leaves_the_table_empty() {
        let mut view = TableView::default();
        let first = TracePresentationKey {
            source_name: "V(a)".to_owned(),
            kind: 0,
            family_group: 0,
        };
        let second = TracePresentationKey {
            source_name: "V(b)".to_owned(),
            kind: 0,
            family_group: 0,
        };
        let automatic = vec![first.clone(), second.clone()];

        // Turning the last chosen column off returns to the automatic set
        // rather than rendering a table of nothing.
        view.toggle_column(first, &automatic);
        assert_eq!(view.columns, vec![second.clone()]);
        view.toggle_column(second, &automatic);
        assert_eq!(view.columns, automatic);
    }

    /// `signals` traces V(s0)… over `samples` microsecond-spaced samples,
    /// trace k holding k·1000 + i at sample i.
    fn wide_fixture(signals: usize, samples: usize) -> Fixture {
        let x: Vec<f64> = (0..samples).map(|i| i as f64 * 1.0e-6).collect();
        let traces = (0..signals)
            .map(|k| {
                let y: Vec<f64> = (0..samples).map(|i| (k * 1000 + i) as f64).collect();
                RetainedTrace::new(format!("V(s{k})"), x.clone(), y, "#fff")
            })
            .collect();
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0).with_waveforms(traces);
        let mut run = rspice_results::run::SimulationRun::new(
            1,
            0.0,
            rspice_results::run::ExecutionTarget::LocalDesktop,
        );
        run.add_analysis(analysis);
        let model = crate::waves::build_models(
            &[&run],
            &Default::default(),
            &mut Default::default(),
            &Tokens::default(),
            crate::waves::ProjectionOptions {
                phase_continuous: false,
                complex_display: Default::default(),
                ac: Default::default(),
                selection: None,
                hidden_family_traces: &Default::default(),
                waveform_visibility: &Default::default(),
                signals: None,
            },
            |_| None,
        )
        .remove(0);
        Fixture {
            model,
            view: TableView::default(),
        }
    }

    /// Sixty signals are sixty columns, each sorting by its numbers; Go to
    /// selects the row nearest the X typed, or the nearest the stride keeps;
    /// and the table draws, its heads in view, on a 1,000-point pane.
    #[test]
    fn a_sixty_signal_table_sorts_finds_and_draws_every_column() {
        let mut fixture = wide_fixture(60, 500);
        let automatic = automatic_columns(&fixture.model);
        let columns = resolved_columns(&fixture.model, &[], &automatic);
        assert_eq!(columns.len(), 60, "every signal is a column");
        let last = *columns.last().expect("sixty columns");
        assert_eq!(fixture.model.sample_value(last, 123), Some(59_123.0));

        let grid = fixture.model.sample_grid().expect("a retained grid");
        let sample = nearest_sample_index(&grid, 123.4e-6, true, &mut || {}).expect("in range");
        let rows = selected_rows(&fixture.model, &fixture.view, None, &mut || {});
        assert_eq!(row_of_sample(rows, sample), Some(123));
        fixture.view.stride = 10;
        let rows = selected_rows(&fixture.model, &fixture.view, None, &mut || {});
        assert_eq!(
            row_of_sample(rows, sample),
            Some(12),
            "row 12 is sample 120"
        );

        fixture.view.stride = 1;
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let labels: Vec<String> = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1_000.0, 700.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        show_samples(
                            ui,
                            SampleTable {
                                model: &fixture.model,
                                cursor_a: None,
                                cursor_b: None,
                                significant_digits: 4,
                                quantity_policy: QuantityPresentationPolicy::default(),
                            },
                            &fixture.view,
                            |name| Cow::Borrowed(name),
                            || {},
                        )
                        .expect("rows to show");
                    });
                },
            )
            .platform_output
            .accesskit_update
            .expect("an accessibility tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label().or(node.value()).map(str::to_owned))
            .collect();
        assert!(
            labels.iter().any(|label| label.contains("V(s0)")),
            "{labels:?}"
        );
    }

    #[test]
    fn the_footer_states_the_shown_count_against_the_retained_total() {
        let view = TableView {
            stride: 10,
            ..TableView::default()
        };
        let line = status_line(200, 24_019, &view, None);
        assert!(line.contains("200 of 24019 retained rows"), "{line}");
        assert!(line.contains("stride 10"), "{line}");
    }

    #[test]
    fn around_cursor_says_so_when_no_cursor_is_placed() {
        let view = TableView {
            around_cursor: true,
            ..TableView::default()
        };
        let line = status_line(100, 100, &view, None);
        assert!(line.contains("no cursor placed"), "{line}");
    }

    #[test]
    fn around_cursor_status_does_not_claim_a_symmetric_edge_window() {
        let view = TableView {
            around_cursor: true,
            ..TableView::default()
        };
        let line = status_line(41, 100, &view, Some(0.0));
        assert!(line.contains("41 of 100 retained rows"), "{line}");
        assert!(line.contains("window around A"), "{line}");
        assert!(!line.contains("±40"), "{line}");
    }

    #[test]
    fn stale_saved_columns_fall_back_to_current_automatic_columns() {
        let state = fixture(4);
        let automatic = automatic_columns(&state.model);
        let stale = TracePresentationKey {
            source_name: "V(renamed-away)".to_owned(),
            kind: 0,
            family_group: 0,
        };
        assert_eq!(
            resolved_columns(&state.model, &[stale], &automatic),
            vec![0]
        );
    }
}
