//! TABLE — the retained samples of one analysis, as rows.
//!
//! The table reads the same [`StripModel`](super::waves) the WAVES stage
//! plots, so a column can never name a trace the plot does not draw, or
//! report a value in a different unit than the curve above it. What the
//! viewer adds is the part a plot cannot give: exact retained samples, and
//! honest control over which of them you are looking at.
//!
//! Three controls shape the row set — a stride, an optional window centred
//! on cursor A, and the column selection — and the footer always states the
//! resulting count against the retained total, so "showing 200" can never
//! be mistaken for "there are 200".

use egui::Ui;

use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, measurement_table, section_header};
use crate::workbench::AppState;

use super::waves::{StripModel, cached_models};
use super::{
    AnalysisPresentationKey, ResultsState, TracePresentationKey, exact_result_artifact_text,
    well_hint,
};

/// Width of the leading row-index column.
const INDEX_W: f32 = 68.0;
/// Width of the X column.
const X_W: f32 = 128.0;
/// Width of one value column.
const VALUE_W: f32 = 118.0;
/// Most value columns the table will lay out.
pub const TABLE_MAX_COLUMNS: usize = 8;
const HEADER_H: f32 = 26.0;
const ROW_H: f32 = 22.0;
const CELL_INSET: f32 = 10.0;

struct TypedOpRow {
    source: String,
    quantity: String,
    value: String,
    unit: String,
    status: String,
}

fn operating_point_row_count(analysis: &AnalysisResult) -> usize {
    let dc_rows = analysis.dc_op.as_ref().map_or(0, |op| {
        op.node_voltages.len() + op.branch_currents.len() + op.power_dissipation.len()
    });
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
            mna_solution,
            ..
        }) => {
            1 + selected_devices.len()
                + violation_devices.len()
                + usize::from(dc_rows == 0) * mna_solution.len()
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
    if analysis.dc_op.is_some() || index >= mna_solution.len() {
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

fn show_operating_point_table(ui: &mut Ui, state: &mut AppState) -> bool {
    let Some(analysis) = state.simulation.active_analysis() else {
        return false;
    };
    if analysis.analysis_type != AnalysisType::DcOp {
        return false;
    }
    let row_count = operating_point_row_count(analysis);
    if row_count == 0 {
        return false;
    }
    let t = Tokens::get(ui.ctx());
    egui_extras::TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .sense(egui::Sense::click())
        .column(egui_extras::Column::initial(120.0).at_least(72.0))
        .column(egui_extras::Column::remainder().at_least(180.0))
        .column(egui_extras::Column::initial(160.0).at_least(100.0))
        .column(egui_extras::Column::initial(72.0).at_least(52.0))
        .column(egui_extras::Column::initial(150.0).at_least(96.0))
        .header(HEADER_H, |mut header| {
            for label in ["SOURCE", "QUANTITY", "EXACT VALUE", "UNIT", "STATUS"] {
                header.col(|ui| {
                    ui.label(
                        egui::RichText::new(label)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                });
            }
        })
        .body(|body| {
            body.rows(ROW_H, row_count, |mut row| {
                let Some(value) = operating_point_row_at(analysis, row.index()) else {
                    return;
                };
                let exact_row = format!(
                    "{}\t{}\t{}\t{}\t{}",
                    value.source, value.quantity, value.value, value.unit, value.status
                );
                for text in [
                    value.source,
                    value.quantity,
                    value.value,
                    value.unit,
                    value.status,
                ] {
                    row.col(|ui| {
                        ui.label(
                            egui::RichText::new(text)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text),
                        );
                    });
                }
                row.response().context_menu(|ui| {
                    if ui.button("Copy exact row").clicked() {
                        ui.ctx().copy_text(exact_row.clone());
                        ui.close();
                    }
                });
            });
        });
    true
}

/// Render the exact shared adapter used by clipboard and file export for a
/// selected typed Data Browser artifact. The evidence is line-virtualized and
/// intentionally not coerced into waveform samples or silently resampled.
fn show_selected_result_artifact_table(ui: &mut Ui, state: &AppState) -> bool {
    let Some(key) = state.ui.results.selected_result_artifact.clone() else {
        return false;
    };
    let exact = match exact_result_artifact_text(&key, &state.simulation.runs) {
        Ok(exact) => exact,
        Err(message) => {
            well_hint(ui, &message);
            return true;
        }
    };
    let lines = exact.lines().collect::<Vec<_>>();
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(key.canonical_name())
                .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        );
        chip(ui, &format!("{} exact records", lines.len()), false);
    });
    ui.add_space(6.0);
    egui_extras::TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .sense(egui::Sense::click())
        .column(egui_extras::Column::initial(INDEX_W).at_least(52.0))
        .column(egui_extras::Column::remainder().at_least(240.0))
        .header(HEADER_H, |mut header| {
            for label in ["LINE", "EXACT RETAINED EVIDENCE"] {
                header.col(|ui| {
                    ui.label(
                        egui::RichText::new(label)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                });
            }
        })
        .body(|body| {
            body.rows(ROW_H, lines.len(), |mut row| {
                let line_number = row.index() + 1;
                let line = lines[row.index()];
                row.col(|ui| {
                    ui.label(
                        egui::RichText::new(line_number.to_string())
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                });
                row.col(|ui| {
                    ui.label(
                        egui::RichText::new(line)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text),
                    );
                });
                row.response().context_menu(|ui| {
                    if ui.button("Copy exact line").clicked() {
                        ui.ctx().copy_text(line.to_owned());
                        ui.close();
                    }
                    if ui.button("Copy all exact evidence").clicked() {
                        ui.ctx().copy_text(exact.clone());
                        ui.close();
                    }
                });
            });
        });
    true
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
    /// Stable analysis the table reads. `None` follows the first available
    /// analysis until the user explicitly chooses one.
    pub analysis: Option<AnalysisPresentationKey>,
    /// Show every `stride`-th retained row.
    pub stride: usize,
    /// Selected trace indices (into the model's active-run prefix). Empty
    /// means "every visible signal", capped at [`TABLE_MAX_COLUMNS`].
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
        } else if self.columns.len() < TABLE_MAX_COLUMNS {
            self.columns.push(trace);
        }
        if self.columns.is_empty() {
            self.columns = automatic.to_vec();
        }
    }
}

/// Trace indices the table shows when the user has chosen none: every
/// visible signal in the active run, up to the column cap.
fn automatic_columns(model: &StripModel) -> Vec<TracePresentationKey> {
    model
        .visible_signal_indices()
        .filter_map(|index| model.trace_presentation_key(index))
        .take(TABLE_MAX_COLUMNS)
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

fn selected_rows(model: &StripModel, view: &TableView, cursor: Option<f64>) -> RowSelection {
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
            let centre = nearest_sample_index(grid, x).unwrap_or(0);
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

fn nearest_sample_index(grid: &[f64], x: f64) -> Option<usize> {
    grid.iter()
        .enumerate()
        .filter(|(_, value)| value.is_finite())
        .min_by(|(_, left), (_, right)| (**left - x).abs().total_cmp(&(**right - x).abs()))
        .map(|(index, _)| index)
}

const fn table_width(column_count: usize) -> f32 {
    INDEX_W + X_W + column_count as f32 * VALUE_W
}

fn table_body_height(viewport_height: f32, scrollbar_gutter: f32) -> f32 {
    (viewport_height - HEADER_H - scrollbar_gutter).max(ROW_H)
}

fn accessible_row_label(
    sample: usize,
    cursor_flag: &str,
    x_heading: &str,
    x_value: &str,
    values: &[(String, String)],
) -> String {
    let mut label = format!("Retained sample {sample}; {x_heading}: {x_value}");
    for (heading, value) in values {
        label.push_str("; ");
        label.push_str(heading);
        label.push_str(": ");
        label.push_str(value);
    }
    if !cursor_flag.is_empty() {
        label.push_str("; cursor ");
        label.push_str(cursor_flag);
    }
    label
}

fn place_table_cursor(results: &mut ResultsState, analysis_index: usize, x: f64) {
    if !results.cursor_tool.is_armed() {
        results.toggle_cursor_tool();
    }
    if results.cursor_strip != Some(analysis_index) {
        results.clear_cursors();
    }
    let placing_a = results.cursor_a_is_next();
    results.cursor_strip = Some(analysis_index);
    results.cursors.place(x);
    if placing_a {
        // A table row has no unique trace anchor; retaining the plot's
        // previous anchor would make marker placement claim the wrong
        // waveform identity.
        results.cursor_a_anchor = None;
    }
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    if show_selected_result_artifact_table(ui, state) {
        return;
    }
    if show_operating_point_table(ui, state) {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    if models.is_empty() {
        well_hint(ui, "No results yet — run a simulation to fill the table");
        return;
    }

    let view = state.ui.results.table.clone();
    let Some(model) = models
        .iter()
        .find(|model| Some(model.analysis_key()) == view.analysis)
        .or_else(|| models.first())
    else {
        well_hint(ui, "The active run has no tabular analyses");
        return;
    };

    let automatic = automatic_columns(model);
    let columns = resolved_columns(model, &view.columns, &automatic);
    if columns.is_empty() {
        well_hint(ui, "No columns — enable a signal from the Columns menu");
        return;
    }

    // Around-cursor mode follows cursor A only when it belongs to this
    // analysis; a cursor on another strip is not a window into this one.
    let cursor = (state.ui.results.cursor_strip == Some(model.analysis_index()))
        .then_some(state.ui.results.cursors.a)
        .flatten();
    let rows = selected_rows(model, &view, cursor);
    if rows.is_empty() {
        well_hint(ui, "This analysis retained no samples");
        return;
    }

    let table_w = table_width(columns.len());
    let viewport_width = ui.available_width().max(1.0);
    let viewport_height = ui.available_height().max(HEADER_H + ROW_H);
    let grid = model.sample_grid().unwrap_or_default();
    let cursor_a_row = cursor.and_then(|x| nearest_sample_index(grid, x));
    let cursor_b_row = if state.ui.results.cursor_strip == Some(model.analysis_index()) {
        state
            .ui
            .results
            .cursors
            .b
            .and_then(|x| nearest_sample_index(grid, x))
    } else {
        None
    };
    let cell =
        |ui: &Ui, rect: egui::Rect, text: &str, color: egui::Color32, align: egui::Align2| {
            let inset = rect.shrink2(egui::vec2(CELL_INSET * 0.5, 0.0));
            let pos = match align {
                egui::Align2::RIGHT_CENTER => egui::pos2(inset.right(), inset.center().y),
                _ => egui::pos2(inset.left(), inset.center().y),
            };
            ui.painter().with_clip_rect(inset).text(
                pos,
                align,
                text,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                color,
            );
        };
    let column_rect = |row: egui::Rect, index: usize| {
        let left = row.left() + INDEX_W + X_W + index as f32 * VALUE_W;
        egui::Rect::from_min_max(
            egui::pos2(left, row.top()),
            egui::pos2(left + VALUE_W, row.bottom()),
        )
    };

    let width = viewport_width.max(table_w);
    let scrollbar_gutter = ui.spacing().scroll.bar_width + ui.spacing().item_spacing.y;
    let mut clicked_sample = None;
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.table.horizontal")
        .auto_shrink([false, false])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .max_height(viewport_height)
        .show(ui, |ui| {
            ui.set_width(width);

            // The header deliberately lives outside the vertical ScrollArea.
            // It remains visible while rows are virtualized, but shares this
            // horizontal owner so headings and cells cannot drift apart.
            let (header, _) =
                ui.allocate_exact_size(egui::vec2(width, HEADER_H), egui::Sense::hover());
            ui.painter().rect_filled(header, 0.0, c.bg_panel_2);
            ui.painter().hline(
                header.x_range(),
                header.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border_strong),
            );
            cell(
                ui,
                egui::Rect::from_min_max(
                    header.min,
                    egui::pos2(header.left() + INDEX_W, header.bottom()),
                ),
                "ROW",
                c.text_faint,
                egui::Align2::LEFT_CENTER,
            );
            cell(
                ui,
                egui::Rect::from_min_max(
                    egui::pos2(header.left() + INDEX_W, header.top()),
                    egui::pos2(header.left() + INDEX_W + X_W, header.bottom()),
                ),
                &model.x_axis_heading(),
                c.text_faint,
                egui::Align2::LEFT_CENTER,
            );
            for (position, &trace_index) in columns.iter().enumerate() {
                let Some((name, color)) = model.trace_heading(trace_index) else {
                    continue;
                };
                cell(
                    ui,
                    column_rect(header, position),
                    name,
                    color,
                    egui::Align2::RIGHT_CENTER,
                );
            }

            let body_height = table_body_height(viewport_height, scrollbar_gutter);
            egui::ScrollArea::vertical()
                .id_salt("rspice.results.table.rows")
                .auto_shrink([false, false])
                .max_height(body_height)
                .show_rows(ui, ROW_H, rows.len(), |ui, visible| {
                    ui.set_width(width);
                    for row_position in visible {
                        let Some(sample) = rows.sample_at(row_position) else {
                            continue;
                        };
                        let (row, response) =
                            ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::click());
                        let at_a = cursor_a_row == Some(sample);
                        let at_b = cursor_b_row == Some(sample);
                        if at_a || at_b {
                            ui.painter().rect_filled(row, 0.0, c.accent_dim);
                            ui.painter().vline(
                                row.left() + 1.0,
                                row.y_range(),
                                egui::Stroke::new(2.0, c.accent),
                            );
                        } else if row_position % 2 == 1 {
                            ui.painter().rect_filled(row, 0.0, c.bg_panel);
                        }
                        let cursor_flag = match (at_a, at_b) {
                            (true, true) => "A/B",
                            (true, false) => "A",
                            (false, true) => "B",
                            (false, false) => "",
                        };
                        let index_text = if cursor_flag.is_empty() {
                            format!("{sample}")
                        } else {
                            format!("{cursor_flag}  {sample}")
                        };
                        let x_text = model.format_x_at(sample, significant_digits, quantity_policy);
                        cell(
                            ui,
                            egui::Rect::from_min_max(
                                row.min,
                                egui::pos2(row.left() + INDEX_W, row.bottom()),
                            ),
                            &index_text,
                            if cursor_flag.is_empty() {
                                c.text_faint
                            } else {
                                c.accent
                            },
                            egui::Align2::LEFT_CENTER,
                        );
                        cell(
                            ui,
                            egui::Rect::from_min_max(
                                egui::pos2(row.left() + INDEX_W, row.top()),
                                egui::pos2(row.left() + INDEX_W + X_W, row.bottom()),
                            ),
                            &x_text,
                            c.text_dim,
                            egui::Align2::LEFT_CENTER,
                        );
                        let mut accessible_values = Vec::with_capacity(columns.len());
                        for (position, &trace_index) in columns.iter().enumerate() {
                            // A trace shorter than the grid says so rather
                            // than borrowing a neighbouring sample's value.
                            let text = model
                                .format_sample(
                                    trace_index,
                                    sample,
                                    significant_digits,
                                    quantity_policy,
                                )
                                .unwrap_or_else(|| "—".to_owned());
                            cell(
                                ui,
                                column_rect(row, position),
                                &text,
                                c.text,
                                egui::Align2::RIGHT_CENTER,
                            );
                            let heading = model
                                .trace_heading(trace_index)
                                .map_or_else(|| "Signal".to_owned(), |(name, _)| name.to_owned());
                            accessible_values.push((heading, text));
                        }
                        let response = response.on_hover_text(
                            "Place the next A/B cursor on this exact retained sample",
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Button,
                                true,
                                at_a || at_b,
                                accessible_row_label(
                                    sample,
                                    cursor_flag,
                                    &model.x_axis_heading(),
                                    &x_text,
                                    &accessible_values,
                                ),
                            )
                        });
                        theme::paint_focus_ring(ui, &response, row);
                        if response.clicked() {
                            clicked_sample = Some(sample);
                        }
                    }
                });
        });

    if let Some(sample) = clicked_sample.and_then(|index| grid.get(index).copied()) {
        place_table_cursor(&mut state.ui.results, model.analysis_index(), sample);
    }

    state.ui.results.table.analysis = Some(model.analysis_key());
    state.ui.results.table_status = Some(status_line(rows.len(), rows.retained, &view, cursor));
}

/// Docbar controls: the three things that decide which samples are shown.
pub fn inline_actions(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    if state.simulation.active_analysis().is_some_and(|analysis| {
        analysis.analysis_type == AnalysisType::DcOp && operating_point_row_count(analysis) > 0
    }) {
        chip(ui, "Typed OP", true)
            .on_hover_text("Virtualized exact operating-point values; right-click a row to copy");
        return;
    }
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let analysis_key = state.ui.results.table.analysis;
    let model = models
        .iter()
        .find(|model| Some(model.analysis_key()) == analysis_key)
        .or_else(|| models.first());

    // Analysis picker — only when there is a choice to make.
    if models.len() > 1
        && let Some(active) = model
    {
        let label = active.table_label();
        ui.menu_button(elide_label(&label), |ui| {
            for candidate in models.iter() {
                let selected = candidate.analysis_key() == active.analysis_key();
                if ui
                    .selectable_label(selected, candidate.table_label())
                    .clicked()
                {
                    let table = &mut state.ui.results.table;
                    if table.analysis != Some(candidate.analysis_key()) {
                        // Columns index into the analysis they were chosen
                        // for; carrying them over would rename the data.
                        table.columns.clear();
                    }
                    table.analysis = Some(candidate.analysis_key());
                    ui.close();
                }
            }
        });
    }

    let stride = state.ui.results.table.stride;
    let response = chip(ui, &format!("1/{stride}"), stride > 1)
        .on_hover_text("Row stride — show every Nth retained sample");
    if response.clicked() {
        state.ui.results.table.cycle_stride();
    }

    // Around-cursor is offered only when there is a cursor to centre on,
    // so the control can never promise a window it cannot produce.
    let has_cursor = model
        .is_some_and(|model| state.ui.results.cursor_strip == Some(model.analysis_index()))
        && state.ui.results.cursors.a.is_some();
    let around = state.ui.results.table.around_cursor;
    let response = ui
        .add_enabled_ui(has_cursor, |ui| chip(ui, "around A", around))
        .inner
        .on_hover_text(if has_cursor {
            "Window the rows on cursor A"
        } else {
            "Place cursor A on this analysis to window the rows on it"
        });
    if response.clicked() {
        state.ui.results.table.around_cursor = !around;
    }

    if let Some(active) = model {
        let automatic = automatic_columns(active);
        let chosen = state.ui.results.table.columns.clone();
        let effective = if chosen.is_empty() {
            automatic.clone()
        } else {
            chosen
        };
        ui.menu_button(format!("Columns ({})", effective.len()), |ui| {
            for index in active.visible_signal_indices() {
                let Some((name, _)) = active.trace_heading(index) else {
                    continue;
                };
                let Some(key) = active.trace_presentation_key(index) else {
                    continue;
                };
                let on = effective.contains(&key);
                if ui.selectable_label(on, name).clicked() {
                    state.ui.results.table.toggle_column(key, &automatic);
                }
            }
            if effective.len() >= TABLE_MAX_COLUMNS {
                ui.label(
                    egui::RichText::new(format!("{TABLE_MAX_COLUMNS} column maximum"))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            }
        });
    }

    if let Some(status) = &state.ui.results.table_status {
        ui.label(
            egui::RichText::new(status)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        );
    }
}

fn elide_label(label: &str) -> String {
    if label.chars().count() <= 22 {
        return label.to_owned();
    }
    let kept: String = label.chars().take(21).collect();
    format!("{kept}…")
}

/// The right panel names the grid the rows come from.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    if let Some(analysis) = state.simulation.active_analysis()
        && analysis.analysis_type == AnalysisType::DcOp
        && operating_point_row_count(analysis) > 0
    {
        section_header(ui, "Typed datasheet", None);
        let owned_rows = [
            ("Analysis".to_owned(), analysis.label.clone()),
            (
                "Authority".to_owned(),
                "immutable operating-point evidence".to_owned(),
            ),
            (
                "Rows".to_owned(),
                operating_point_row_count(analysis).to_string(),
            ),
            ("Precision".to_owned(), "exact retained values".to_owned()),
        ];
        let borrowed = owned_rows
            .iter()
            .map(|(label, value)| (label.as_str(), value.as_str()))
            .collect::<Vec<_>>();
        measurement_table(ui, &borrowed);
        return;
    }
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let analysis_key = state.ui.results.table.analysis;
    let Some(model) = models
        .iter()
        .find(|model| Some(model.analysis_key()) == analysis_key)
        .or_else(|| models.first())
    else {
        return;
    };
    section_header(ui, "Sample grid", None);
    let retained = model.sample_grid().map_or(0, <[f64]>::len);
    let rows = [
        ("Analysis", model.table_label()),
        ("X axis", model.x_axis_heading()),
        ("Retained rows", format!("{retained}")),
        (
            "Signals",
            format!("{}", model.visible_signal_indices().count()),
        ),
    ];
    let rows: Vec<(&str, &str)> = rows
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect();
    measurement_table(ui, &rows);
}

/// What is on screen against what the run retained.
///
/// The count is always stated both ways: a table that showed 200 of 24,019
/// rows without saying so would read as the whole dataset.
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
    use crate::state::{
        AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue, WaveformData,
    };

    #[test]
    fn typed_operating_point_rows_preserve_exact_values_without_waveforms() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
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

    /// A transient run of `samples` retained rows, 1 µs apart.
    fn fixture(samples: usize) -> AppState {
        let x: Vec<f64> = (0..samples).map(|i| i as f64 * 1.0e-6).collect();
        let y: Vec<f64> = (0..samples).map(|i| i as f64).collect();
        let mut state = AppState::default();
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Tran")
                .with_waveforms(vec![WaveformData::new("V(out)", x, y, "#fff")]),
        );
        state
    }

    fn rows_for(state: &mut AppState, cursor: Option<f64>) -> (Vec<usize>, usize) {
        let presentation = state.ui.preferences.result_presentation_policy();
        let models = cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        let view = state.ui.results.table.clone();
        let rows = selected_rows(&models[0], &view, cursor);
        (rows.indices().collect(), rows.retained)
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
        state.ui.results.table.stride = 5;
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
        state.ui.results.table.around_cursor = true;
        let (rows, _) = rows_for(&mut state, Some(50.0e-6));
        assert_eq!(rows.first(), Some(&(50 - CURSOR_SPAN)));
        assert_eq!(rows.last(), Some(&(50 + CURSOR_SPAN)));

        // Striding applies inside the window: "around A, every 10th" must
        // not mean "the whole run decimated, then cropped".
        state.ui.results.table.stride = 10;
        let (rows, _) = rows_for(&mut state, Some(50.0e-6));
        assert_eq!(rows.first(), Some(&10));
        assert!(rows.iter().all(|row| (10..=90).contains(row)));
    }

    #[test]
    fn around_cursor_without_a_cursor_shows_the_whole_run() {
        let mut state = fixture(100);
        state.ui.results.table.around_cursor = true;
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
    fn table_width_depends_only_on_the_visible_column_contract() {
        assert_eq!(table_width(0), INDEX_W + X_W);
        assert_eq!(table_width(3), INDEX_W + X_W + 3.0 * VALUE_W);
        assert_eq!(table_width(4) - table_width(3), VALUE_W);
        assert_eq!(table_body_height(500.0, 14.0), 500.0 - HEADER_H - 14.0);
    }

    #[test]
    fn nearest_sample_uses_the_exact_retained_grid() {
        let grid = [0.0, 1.0e-6, 2.5e-6, 9.0e-6];
        assert_eq!(nearest_sample_index(&grid, 2.4e-6), Some(2));
        assert_eq!(nearest_sample_index(&grid, 8.0e-6), Some(3));
        assert_eq!(nearest_sample_index(&[], 1.0), None);
    }

    #[test]
    fn row_activation_places_a_and_b_at_exact_samples() {
        let mut state = AppState::default();
        place_table_cursor(&mut state.ui.results, 3, 1.25e-6);
        assert_eq!(state.ui.results.cursor_strip, Some(3));
        assert_eq!(state.ui.results.cursors.a, Some(1.25e-6));
        assert_eq!(state.ui.results.cursors.b, None);

        place_table_cursor(&mut state.ui.results, 3, 8.5e-6);
        assert_eq!(state.ui.results.cursors.a, Some(1.25e-6));
        assert_eq!(state.ui.results.cursors.b, Some(8.5e-6));

        place_table_cursor(&mut state.ui.results, 3, 4.0e-6);
        assert_eq!(state.ui.results.cursors.a, Some(4.0e-6));
        assert_eq!(state.ui.results.cursors.b, None);
    }

    #[test]
    fn row_activation_rearms_and_rebinds_the_cursor_without_stale_b() {
        let mut state = AppState::default();
        state.ui.results.toggle_cursor_tool();
        assert!(!state.ui.results.cursor_tool.is_armed());

        place_table_cursor(&mut state.ui.results, 1, 1.0);
        place_table_cursor(&mut state.ui.results, 1, 2.0);
        place_table_cursor(&mut state.ui.results, 2, 9.0);

        assert!(state.ui.results.cursor_tool.is_armed());
        assert_eq!(state.ui.results.cursor_strip, Some(2));
        assert_eq!(state.ui.results.cursors.a, Some(9.0));
        assert_eq!(state.ui.results.cursors.b, None);
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

    #[test]
    fn the_column_chooser_stops_at_the_layout_cap() {
        let mut view = TableView::default();
        view.columns = (0..TABLE_MAX_COLUMNS)
            .map(|index| TracePresentationKey {
                source_name: format!("V({index})"),
                kind: 0,
                family_group: 0,
            })
            .collect();
        view.toggle_column(
            TracePresentationKey {
                source_name: "V(extra)".to_owned(),
                kind: 0,
                family_group: 0,
            },
            &[TracePresentationKey {
                source_name: "V(0)".to_owned(),
                kind: 0,
                family_group: 0,
            }],
        );
        assert_eq!(
            view.columns.len(),
            TABLE_MAX_COLUMNS,
            "a column past the cap has nowhere to be laid out"
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
        let mut state = fixture(4);
        let presentation = state.ui.preferences.result_presentation_policy();
        let models = cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        let automatic = automatic_columns(&models[0]);
        let stale = TracePresentationKey {
            source_name: "V(renamed-away)".to_owned(),
            kind: 0,
            family_group: 0,
        };
        assert_eq!(resolved_columns(&models[0], &[stale], &automatic), vec![0]);
    }

    #[test]
    fn accessible_rows_name_axes_values_and_cursor_ownership() {
        let label = accessible_row_label(
            42,
            "A",
            "Time",
            "1.000 us",
            &[("V(out)".to_owned(), "1.250 V".to_owned())],
        );
        assert!(label.contains("Retained sample 42"), "{label}");
        assert!(label.contains("Time: 1.000 us"), "{label}");
        assert!(label.contains("V(out): 1.250 V"), "{label}");
        assert!(label.contains("cursor A"), "{label}");
    }
}
