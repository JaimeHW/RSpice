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

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, measurement_table, section_header};
use crate::workbench::AppState;

use super::waves::{StripModel, cached_models};
use super::{AnalysisPresentationKey, TracePresentationKey, well_hint};

/// Width of the leading row-index column.
const INDEX_W: f32 = 68.0;
/// Width of the X column.
const X_W: f32 = 128.0;
/// Width of one value column.
const VALUE_W: f32 = 118.0;
/// Most value columns the table will lay out.
pub const TABLE_MAX_COLUMNS: usize = 8;
const ROW_H: f32 = 22.0;
const CELL_INSET: f32 = 10.0;

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

/// The rows the current controls select, as indices into the sample grid.
///
/// Returned alongside the retained total so the footer can state both
/// without recomputing either.
fn selected_rows(model: &StripModel, view: &TableView, cursor: Option<f64>) -> (Vec<usize>, usize) {
    let Some(grid) = model.sample_grid() else {
        return (Vec::new(), 0);
    };
    let retained = grid.len();
    let stride = view.stride.max(1);
    // The window is taken first and the stride applied inside it, so
    // "around A, every 10th" means 40 rows either side of A, not the whole
    // run decimated and then cropped.
    let (start, end) = match cursor.filter(|_| view.around_cursor) {
        Some(x) => {
            let centre = grid
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| (*a - x).abs().total_cmp(&(*b - x).abs()))
                .map_or(0, |(index, _)| index);
            (
                centre.saturating_sub(CURSOR_SPAN),
                (centre + CURSOR_SPAN + 1).min(retained),
            )
        }
        None => (0, retained),
    };
    ((start..end).step_by(stride).collect(), retained)
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
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
    let columns: Vec<usize> = if view.columns.is_empty() {
        automatic
            .iter()
            .filter_map(|key| model.trace_index_for_key(key))
            .collect()
    } else {
        view.columns
            .iter()
            .filter_map(|key| model.trace_index_for_key(key))
            .collect()
    };
    if columns.is_empty() {
        well_hint(ui, "No columns — enable a signal from the Columns menu");
        return;
    }

    // Around-cursor mode follows cursor A only when it belongs to this
    // analysis; a cursor on another strip is not a window into this one.
    let cursor = (state.ui.results.cursor_strip == Some(model.analysis_index()))
        .then(|| state.ui.results.cursors.a)
        .flatten();
    let (rows, retained) = selected_rows(model, &view, cursor);
    if rows.is_empty() {
        well_hint(ui, "This analysis retained no samples");
        return;
    }

    let table_w = INDEX_W + X_W + columns.len() as f32 * VALUE_W;
    let viewport_width = ui.available_width().max(1.0);
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

    egui::ScrollArea::both()
        .id_salt("rspice.results.table")
        .auto_shrink([false, false])
        .show_rows(ui, ROW_H, rows.len(), |ui, visible| {
            let width = viewport_width.max(table_w);
            // The header scrolls with the rows: a frozen header would have
            // to be painted outside the virtualized area, and the row set
            // is short enough that scrolling back is cheap.
            if visible.start == 0 {
                let (header, _) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
                ui.painter().hline(
                    header.x_range(),
                    header.bottom() - 0.5,
                    egui::Stroke::new(1.0, c.border),
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
            }

            for row_position in visible {
                let Some(&sample) = rows.get(row_position) else {
                    continue;
                };
                let (row, _) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
                if row_position % 2 == 1 {
                    ui.painter().rect_filled(row, 0.0, c.bg_panel);
                }
                cell(
                    ui,
                    egui::Rect::from_min_max(
                        row.min,
                        egui::pos2(row.left() + INDEX_W, row.bottom()),
                    ),
                    &format!("{sample}"),
                    c.text_faint,
                    egui::Align2::LEFT_CENTER,
                );
                cell(
                    ui,
                    egui::Rect::from_min_max(
                        egui::pos2(row.left() + INDEX_W, row.top()),
                        egui::pos2(row.left() + INDEX_W + X_W, row.bottom()),
                    ),
                    &model.format_x_at(sample, significant_digits, quantity_policy),
                    c.text_dim,
                    egui::Align2::LEFT_CENTER,
                );
                for (position, &trace_index) in columns.iter().enumerate() {
                    // A trace shorter than the grid says so rather than
                    // borrowing a neighbouring sample's value.
                    let text = model
                        .format_sample(trace_index, sample, significant_digits, quantity_policy)
                        .unwrap_or_else(|| "—".to_owned());
                    cell(
                        ui,
                        column_rect(row, position),
                        &text,
                        c.text,
                        egui::Align2::RIGHT_CENTER,
                    );
                }
            }
        });

    state.ui.results.table.analysis = Some(model.analysis_key());
    state.ui.results.table_status = Some(status_line(rows.len(), retained, &view, cursor));
}

/// Docbar controls: the three things that decide which samples are shown.
pub fn inline_actions(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
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
        parts.push(format!("every {}th", view.stride));
    }
    if view.around_cursor {
        parts.push(if cursor.is_some() {
            format!("±{CURSOR_SPAN} around A")
        } else {
            "around A — no cursor placed".to_owned()
        });
    }
    parts.join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, WaveformData};

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
        selected_rows(&models[0], &view, cursor)
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
        assert!(line.contains("every 10th"), "{line}");
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
}
