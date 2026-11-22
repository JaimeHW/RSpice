//! The Cursors page's table.
//!
//! One column per vertical cursor or marker, its head the column's name over
//! its X, which is typed in place to move it. A difference column follows
//! for each other column against the base, and a pair adds its slope. Every
//! strip on screen that shares the columns' X has a band of rows, one per
//! trace with its swatch. Cursors and markers on strips with another X make
//! a table of their own below, as a difference across two domains is not a
//! measurement.
//!
//! The horizontal cursors follow: each pane's levels, and where each of its
//! traces crosses them.

use super::cursor_values::{marker_position, x_delta, x_reciprocal};
use super::dock::{MarkerActions, MarkerMove, ReadoutInput, marker_color, visible_markers};
use super::pane::SwatchStyle;
use super::readout::{
    READOUT_ABSENT, READOUT_PAD_X, READOUT_ROW_H, ReadingLayout, TableRow, readout_branch_note,
    readout_traces, trace_table_rows,
};
use super::{StripModel, anchor_key};
use crate::session::{HorizontalCursors, ReadingColumnId, ResultViewerState};
use egui::Ui;
use rspice_results::result_presentation::{WavePanePresentationKey, WaveformPresentationKey};
use rspice_ui_kit::plot::{CursorId, CursorPair, TraceLook, XScale, paint_trace_swatch};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// The space either side of a cell's text.
const CELL_PAD: f32 = 10.0;
/// The narrowest a value column is drawn.
const VALUE_COLUMN_MIN_W: f32 = 72.0;
/// The swatch leading a trace's row.
const SWATCH_W: f32 = 22.0;
/// The widest a crossings cell is drawn; its count leads, so what the clip
/// hides is still counted.
const CROSSINGS_MAX_W: f32 = 420.0;

/// One vertical reading of the table: a cursor or a marker, and where it
/// stands.
pub struct ReadingColumn {
    pub id: ReadingColumnId,
    /// What its head says: A, B, or the marker's id.
    pub name: String,
    /// What its head says on hover.
    pub hover: String,
    pub color: egui::Color32,
    /// The strip it was placed on, by its place in the source's models.
    pub model: usize,
    /// Where it stands, or why it stands nowhere: a search that finds
    /// nothing there.
    pub x: Result<f64, String>,
    /// The trace it rides, whose samples it snaps to.
    pub anchor: Option<WaveformPresentationKey>,
}

/// One strip's rows, under the band that names the strip.
pub struct TableBand {
    pub title: String,
    pub rows: Vec<TableRow>,
}

/// The cursors and markers that read one X, and every strip on screen
/// that has it.
pub struct CursorTable {
    /// The X quantity, as the strip's axis names it.
    pub x_label: String,
    pub columns: Vec<ReadingColumn>,
    /// The column every difference is taken from.
    pub base: usize,
    /// The columns read against the base.
    pub deltas: Vec<usize>,
    /// Exactly two columns: their difference has a slope, and their
    /// separation a reciprocal.
    pub pair: bool,
    /// The head's second row: each column's X, each difference's Δx, then
    /// the pair's 1/Δx over its slope.
    pub x_cells: Vec<String>,
    pub bands: Vec<TableBand>,
}

impl CursorTable {
    /// The head's first row: each column's name, each difference named by
    /// the two columns it subtracts, then the pair's slope.
    pub fn heads(&self) -> Vec<String> {
        let base = &self.columns[self.base].name;
        let mut heads: Vec<String> = self
            .columns
            .iter()
            .map(|column| column.name.clone())
            .collect();
        heads.extend(
            self.deltas
                .iter()
                .map(|&column| format!("{}-{base}", self.columns[column].name)),
        );
        if self.pair {
            heads.push("Slope".to_owned());
        }
        heads
    }

    /// The table as tab-separated text, as it pastes into a spreadsheet:
    /// the heads, the X row, then each band's title and rows.
    pub fn tsv(&self) -> String {
        let mut lines = vec![
            std::iter::once("Trace".to_owned())
                .chain(self.heads())
                .collect::<Vec<_>>()
                .join("\t"),
            std::iter::once(self.x_label.clone())
                .chain(self.x_cells.iter().cloned())
                .collect::<Vec<_>>()
                .join("\t"),
        ];
        for band in &self.bands {
            lines.push(band.title.clone());
            for row in &band.rows {
                lines.push(
                    std::iter::once(row.name.clone())
                        .chain(row.cells.iter().cloned())
                        .collect::<Vec<_>>()
                        .join("\t"),
                );
            }
        }
        lines.join("\n")
    }

    /// The table, spoken: one string, as the table is one painted grid.
    fn spoken(&self) -> String {
        let heads = self.heads();
        let mut text = format!("Cursor table. {}:", self.x_label);
        for (head, x) in heads.iter().zip(&self.x_cells) {
            text.push_str(&format!(" {head} {x},"));
        }
        for band in &self.bands {
            text.push_str(&format!(" {}.", band.title));
            for row in &band.rows {
                text.push_str(&format!(" {}:", row.name));
                for (head, cell) in heads.iter().zip(&row.cells) {
                    text.push_str(&format!(" {head} {cell},"));
                }
            }
        }
        text
    }
}

/// Which X a strip reads: its quantity, unit and scale.
fn domain(model: &StripModel) -> (String, String, bool) {
    (
        model.x_label().to_owned(),
        model.x_unit.clone(),
        matches!(model.x_scale, XScale::Log10),
    )
}

/// The table's columns: A and B on the strip the cursors stand on, then
/// every marker on screen, in placement order.
pub fn reading_columns(
    state: &ResultViewerState,
    source: ReadoutInput<'_>,
    t: &Tokens,
) -> Vec<ReadingColumn> {
    let models = source.models;
    let mut columns = Vec::new();
    let cursor_model = state
        .cursor_strip
        .filter(|_| state.cursor_readout_active())
        .and_then(|index| {
            models
                .iter()
                .position(|model| model.analysis_index == index)
        });
    if let Some(model) = cursor_model {
        for (id, color, anchor) in [
            (
                CursorId::A,
                t.color.canvas_cursor_a,
                state.cursor_a_anchor.clone(),
            ),
            (CursorId::B, t.color.canvas_cursor_b, None),
        ] {
            let Some(x) = state.cursors.get(id) else {
                continue;
            };
            columns.push(ReadingColumn {
                id: ReadingColumnId::Cursor(id),
                name: id.letter().to_string(),
                hover: format!(
                    "Cursor {}. Click to move it with the arrow keys; right-click to make it \
                     the base, snap or delete it. Click its X to type where it stands.",
                    id.letter()
                ),
                color,
                model,
                x: Ok(x),
                anchor,
            });
        }
    }
    for marker in visible_markers(state, source) {
        let Some(model) = models
            .iter()
            .position(|model| model.analysis_key == marker.analysis())
        else {
            continue;
        };
        let kind = marker.kind();
        let x = marker_position(&models[model], marker.anchor(), marker.x(), kind)
            .map_err(|_| format!("no {} here", kind.label()));
        let note = marker.note().trim();
        let mut hover = format!("{} · {}", marker.display_id(), kind.label());
        if kind.rides_a_trace() {
            hover.push_str(&format!(" on {}", marker.trace_name()));
        }
        if !note.is_empty() {
            hover.push_str(&format!(" · {note}"));
        }
        hover.push_str(
            ". Right-click to make it the base, snap, rename or delete it. Click its X to type \
             where it stands.",
        );
        columns.push(ReadingColumn {
            id: ReadingColumnId::Marker(marker.selector()),
            name: marker.display_id(),
            hover,
            color: marker_color(kind, t),
            model,
            x,
            anchor: Some(marker.anchor().clone()),
        });
    }
    columns
}

/// Every table the page draws: one per X the columns read, the cursors'
/// first.
pub fn cursor_tables(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    t: &Tokens,
) -> Vec<CursorTable> {
    let models = source.models;
    let mut groups: Vec<((String, String, bool), Vec<ReadingColumn>)> = Vec::new();
    for column in reading_columns(state, source, t) {
        let key = domain(&models[column.model]);
        match groups.iter_mut().find(|(domain, _)| *domain == key) {
            Some((_, columns)) => columns.push(column),
            None => groups.push((key, vec![column])),
        }
    }
    groups
        .into_iter()
        .map(|(key, columns)| build_table(state, source, &key, columns))
        .collect()
}

fn build_table(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    key: &(String, String, bool),
    columns: Vec<ReadingColumn>,
) -> CursorTable {
    let models = source.models;
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let base = state
        .readout_base
        .and_then(|base| columns.iter().position(|column| column.id == base))
        .unwrap_or(0);
    let deltas: Vec<usize> = (0..columns.len()).filter(|&index| index != base).collect();
    let pair = columns.len() == 2;
    let positions: Vec<Option<f64>> = columns.iter().map(|column| column.x.clone().ok()).collect();
    let domain_model = &models[columns[base].model];

    let mut x_cells: Vec<String> = columns
        .iter()
        .map(|column| match &column.x {
            Ok(x) => models[column.model].format_x(*x, digits, quantity_policy),
            Err(why) => why.clone(),
        })
        .collect();
    let base_x = positions[base];
    for &column in &deltas {
        x_cells.push(match (base_x, positions[column]) {
            (Some(from), Some(to)) => x_delta(domain_model, from, to, digits, quantity_policy),
            _ => READOUT_ABSENT.to_owned(),
        });
    }
    if pair {
        let reciprocal = match (base_x, positions[deltas[0]]) {
            (Some(from), Some(to)) => x_reciprocal(domain_model, from, to, digits, quantity_policy),
            _ => None,
        };
        x_cells.push(reciprocal.map_or_else(
            || READOUT_ABSENT.to_owned(),
            |reciprocal| format!("1/\u{0394}x {reciprocal}"),
        ));
    }

    let layout = ReadingLayout {
        positions: &positions,
        base,
        deltas: &deltas,
        pair,
    };
    let cursor_model = columns
        .iter()
        .find(|column| matches!(column.id, ReadingColumnId::Cursor(_)))
        .map(|column| column.model);
    let mut bands = Vec::new();
    for (index, model) in models.iter().enumerate() {
        let shown =
            source.on_screen_strips.contains(&model.analysis_key) || cursor_model == Some(index);
        if !shown || domain(model) != *key {
            continue;
        }
        let mut rows = Vec::new();
        for trace in readout_traces(model) {
            let trace_index = model
                .traces
                .iter()
                .position(|candidate| std::ptr::eq(candidate, trace));
            let extent = trace_index.and_then(|trace_index| {
                super::stack::pane_trace_extremes(
                    &mut state.derived,
                    model,
                    &[trace_index],
                    None,
                    |_| {},
                )
                .range()
            });
            rows.extend(trace_table_rows(
                model,
                trace,
                &layout,
                (presentation, quantity_policy, extent),
                source.display_name,
            ));
        }
        rows.extend(lane_table_rows(state, source, model, &layout));
        let mut title = if model.subtitle.is_empty() || model.subtitle == model.kind_tag {
            model.kind_tag.clone()
        } else {
            format!("{} · {}", model.kind_tag, model.subtitle)
        };
        if let Some(note) = readout_branch_note(model) {
            title.push_str(&format!(" · {note}"));
        }
        bands.push(TableBand { title, rows });
    }
    CursorTable {
        x_label: key.0.clone(),
        columns,
        base,
        deltas,
        pair,
        x_cells,
        bands,
    }
}

/// The table's rows for a strip's logic, bus and `E()` lanes: what each
/// reads at every column, and the pair's difference and slope as the lanes
/// state them between two cursors.
fn lane_table_rows(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    model: &StripModel,
    layout: &ReadingLayout<'_>,
) -> Vec<TableRow> {
    let read = |state: &mut ResultViewerState, cursors: CursorPair| {
        super::dock::strip_lane_rows(
            state,
            model,
            cursors,
            source.presentation,
            source.display_name,
        )
    };
    let at: Vec<_> = layout
        .positions
        .iter()
        .map(|&x| read(state, CursorPair { a: x, b: None }))
        .collect();
    let between = layout.pair.then(|| {
        read(
            state,
            CursorPair {
                a: layout.positions[layout.base],
                b: layout.positions[layout.deltas[0]],
            },
        )
    });
    let Some(first) = at.first() else {
        return Vec::new();
    };
    (0..first.len())
        .map(|row| {
            let mut cells: Vec<String> = at
                .iter()
                .map(|rows| {
                    rows.get(row)
                        .map_or_else(|| READOUT_ABSENT.to_owned(), |found| found.a.clone())
                })
                .collect();
            match between.as_ref().and_then(|rows| rows.get(row)) {
                Some(found) => cells.extend([found.delta.clone(), found.slope.clone()]),
                None => cells.extend(
                    std::iter::repeat_with(|| READOUT_ABSENT.to_owned())
                        .take(layout.deltas.len() + usize::from(layout.pair)),
                ),
            }
            TableRow {
                name: first[row].name.clone(),
                swatch: first[row].swatch,
                swatch_style: first[row].swatch_style,
                cells,
            }
        })
        .collect()
}

/// Read a typed X: a number with an SI prefix, as the table prints it
/// ("10 ns", "1.5 MHz", "2.2u"), the axis' unit allowed after it. `M` is
/// mega and `m` milli, as printed; `meg` is mega too.
pub fn parse_typed_x(text: &str, unit: &str) -> Option<f64> {
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let split =
        text.char_indices()
            .find(|&(index, c)| {
                !(c.is_ascii_digit()
                    || c == '.'
                    || ((c == '-' || c == '+')
                        && (index == 0 || matches!(text[..index].chars().last(), Some('e' | 'E')))))
                    && !((c == 'e' || c == 'E')
                        && text[index + 1..].chars().next().is_some_and(|next| {
                            next.is_ascii_digit() || next == '-' || next == '+'
                        }))
            })
            .map_or(text.len(), |(index, _)| index);
    let number: f64 = text[..split].parse().ok()?;
    let suffix = &text[split..];
    let scale = if suffix.is_empty() || suffix == unit {
        1.0
    } else if suffix.to_ascii_lowercase().starts_with("meg") {
        1e6
    } else {
        match suffix.chars().next()? {
            'f' => 1e-15,
            'p' => 1e-12,
            'n' => 1e-9,
            'u' | '\u{00b5}' | '\u{03bc}' => 1e-6,
            'm' => 1e-3,
            'k' | 'K' => 1e3,
            'M' => 1e6,
            'G' => 1e9,
            'T' => 1e12,
            _ if suffix.chars().all(char::is_alphabetic) => 1.0,
            _ => return None,
        }
    };
    let value = number * scale;
    value.is_finite().then_some(value)
}

/// The retained sample of `model`'s trace `anchor` nearest `x`, or of its
/// first visible trace when the column rides none.
pub fn nearest_sample_x(
    model: &StripModel,
    anchor: Option<&WaveformPresentationKey>,
    x: f64,
) -> Option<f64> {
    let trace = anchor
        .and_then(|anchor| {
            model
                .traces
                .iter()
                .find(|trace| !trace.overlay && anchor_key(model, trace) == *anchor)
        })
        .or_else(|| readout_traces(model).next())?;
    let (xs, _) = trace.held()?;
    xs.iter()
        .copied()
        .filter(|sample| sample.is_finite())
        .min_by(|left, right| (left - x).abs().total_cmp(&(right - x).abs()))
}

/// Paint the page's tables, then its horizontal cursors, and take what the
/// reader asks of them. Every section shares one name column, so their
/// readings start on one edge.
pub fn paint_page(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    tables: &[CursorTable],
) -> MarkerActions {
    let mut requests = MarkerActions::default();
    let horizontal = horizontal_sections(
        source,
        &super::dock::horizontal_cursor_rows(state, source),
        &Tokens::get(ui.ctx()),
    );
    let name_w = name_column_width(ui, tables, &horizontal);
    for (index, table) in tables.iter().enumerate() {
        if index > 0 {
            seam(ui);
        }
        egui::ScrollArea::horizontal()
            .id_salt(("rspice.results.cursor-table", index))
            .auto_shrink([false, true])
            .show(ui, |ui| {
                paint_table(ui, state, source, table, name_w, &mut requests);
            });
    }
    if !horizontal.is_empty() {
        if !tables.is_empty() {
            seam(ui);
        }
        egui::ScrollArea::horizontal()
            .id_salt("rspice.results.horizontal-cursors")
            .auto_shrink([false, true])
            .show(ui, |ui| paint_horizontal(ui, state, &horizontal, name_w));
    }
    requests
}

/// The name column every section of the page draws: as wide as its widest
/// name, within bounds.
fn name_column_width(ui: &Ui, tables: &[CursorTable], horizontal: &[HorizontalSection]) -> f32 {
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let head_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);
    let mut width = text_width(ui, "Trace", &head_font);
    for table in tables {
        width = width.max(text_width(ui, &table.x_label, &font));
        for row in table.bands.iter().flat_map(|band| &band.rows) {
            width = width.max(SWATCH_W + text_width(ui, &row.name, &font));
        }
    }
    for section in horizontal {
        width = width.max(text_width(ui, &section.name, &head_font));
        for row in &section.rows {
            width = width.max(SWATCH_W + text_width(ui, &row.name, &font));
        }
    }
    width.clamp(120.0, 360.0) + CELL_PAD
}

fn seam(ui: &mut Ui) {
    let (seam, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter()
        .rect_filled(seam, 0.0, Tokens::get(ui.ctx()).color.border);
}

fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
}

fn paint_swatch(
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
    style: Option<SwatchStyle>,
    t: &Tokens,
) {
    let look = style.map_or(
        TraceLook {
            dash: None,
            marker: None,
        },
        |style| style.look(t.color_safe_traces),
    );
    paint_trace_swatch(painter, rect, color, look, t.color.bg_panel);
}

/// The editor of one column's X while the reader types it.
fn x_edit_id() -> egui::Id {
    egui::Id::new("rspice.results.cursor-table.x-edit")
}

/// A column's X as the reader is typing it, and as it read when they began.
#[derive(Clone)]
struct XEdit {
    column: ReadingColumnId,
    text: String,
    original: String,
}

fn paint_table(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    table: &CursorTable,
    name_w: f32,
    requests: &mut MarkerActions,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let head_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);
    let heads = table.heads();

    // Columns as wide as what they hold, so no reading is cut short.
    let widths: Vec<f32> = (0..heads.len())
        .map(|column| {
            let mut width = text_width(ui, &heads[column], &head_font);
            if let Some(x) = table.x_cells.get(column) {
                width = width.max(text_width(ui, x, &font));
            }
            for row in table.bands.iter().flat_map(|band| &band.rows) {
                if let Some(cell) = row.cells.get(column) {
                    width = width.max(text_width(ui, cell, &font));
                }
            }
            (width + CELL_PAD * 2.0).max(VALUE_COLUMN_MIN_W)
        })
        .collect();
    let rows = 2 + table
        .bands
        .iter()
        .map(|band| 1 + band.rows.len())
        .sum::<usize>();
    let table_w = READOUT_PAD_X * 2.0 + name_w + widths.iter().sum::<f32>();
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(
            table_w.max(ui.available_width()),
            rows as f32 * READOUT_ROW_H,
        ),
        egui::Sense::hover(),
    );
    let left = rect.left() + READOUT_PAD_X;
    let mut edges = vec![left + name_w];
    for width in &widths {
        edges.push(edges.last().copied().unwrap_or(left) + width);
    }
    let row_rect = |row: usize| {
        let top = rect.top() + row as f32 * READOUT_ROW_H;
        egui::Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + READOUT_ROW_H),
        )
    };
    let cell_rect = |row: usize, column: usize| {
        let row = row_rect(row);
        egui::Rect::from_x_y_ranges(edges[column]..=edges[column + 1], row.y_range())
    };
    let painter = ui.painter().with_clip_rect(rect);
    let right_text = |row: usize, column: usize, text: &str, font: &egui::FontId, color| {
        let cell = cell_rect(row, column);
        painter.with_clip_rect(cell).text(
            egui::pos2(cell.right() - CELL_PAD, cell.center().y),
            egui::Align2::RIGHT_CENTER,
            text,
            font.clone(),
            color,
        );
    };
    let name_text = |row: usize, indent: f32, text: &str, font: &egui::FontId, color| {
        let row = row_rect(row);
        let clip = egui::Rect::from_x_y_ranges(left..=left + name_w - CELL_PAD, row.y_range());
        painter.with_clip_rect(clip).text(
            egui::pos2(left + indent, row.center().y),
            egui::Align2::LEFT_CENTER,
            text,
            font.clone(),
            color,
        );
    };

    // The heads: names over their X.
    name_text(0, 0.0, "Trace", &head_font, c.text_faint);
    name_text(1, 0.0, &table.x_label, &font, c.text_dim);
    let stepping = state.stepping_cursor;
    for (column, head) in heads.iter().enumerate() {
        let color = match table.columns.get(column).map(|column| column.id) {
            Some(ReadingColumnId::Cursor(id)) if id != stepping => c.text_faint,
            Some(_) => table.columns[column].color,
            None => c.text_faint,
        };
        right_text(0, column, head, &head_font, color);
    }
    let editing = ui.data(|data| data.get_temp::<XEdit>(x_edit_id()));
    for (column, x) in table.x_cells.iter().enumerate() {
        let is_edited = table.columns.get(column).is_some_and(|reading| {
            editing
                .as_ref()
                .is_some_and(|edit| edit.column == reading.id)
        });
        if !is_edited {
            let color = if table
                .columns
                .get(column)
                .is_some_and(|reading| reading.x.is_err())
            {
                c.text_faint
            } else {
                c.text
            };
            right_text(1, column, x, &font, color);
        }
    }

    // Each strip's band, then its rows.
    let mut row_index = 2;
    for band in &table.bands {
        let band_rect = row_rect(row_index);
        painter.rect_filled(band_rect, 0.0, c.bg_panel_2);
        name_text(row_index, 0.0, &band.title, &head_font, c.text_dim);
        row_index += 1;
        for row in &band.rows {
            let swatch = egui::Rect::from_x_y_ranges(
                left..=left + SWATCH_W - 4.0,
                row_rect(row_index).y_range(),
            );
            paint_swatch(&painter, swatch, row.swatch, row.swatch_style, &t);
            name_text(row_index, SWATCH_W, &row.name, &font, c.text_dim);
            for (column, cell) in row.cells.iter().enumerate() {
                let color = if column < table.columns.len() {
                    c.text
                } else {
                    c.text_dim
                };
                right_text(row_index, column, cell, &font, color);
            }
            row_index += 1;
        }
    }

    // The painted grid is one node to a screen reader.
    let response = ui.interact(rect, ui.id().with("cursor-table"), egui::Sense::hover());
    let spoken = table.spoken();
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, spoken.clone()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label(spoken.clone());
    });

    // A column's head: a click hands a cursor the arrow keys; the menu
    // makes it the base, snaps, renames or deletes it.
    for (column, reading) in table.columns.iter().enumerate() {
        let head = cell_rect(0, column);
        let key = (table.x_label.as_str(), column);
        let response = ui
            .interact(
                head,
                ui.id().with(("cursor-table-head", key)),
                egui::Sense::click(),
            )
            .on_hover_text(&reading.hover);
        if response.clicked()
            && let ReadingColumnId::Cursor(id) = reading.id
        {
            state.stepping_cursor = id;
        }
        response.context_menu(|ui| {
            column_menu(ui, state, source, table, column, requests);
        });

        // Its X, typed in place.
        let x_cell = cell_rect(1, column);
        match editing.as_ref() {
            Some(edit) if edit.column == reading.id => {
                let mut text = edit.text.clone();
                let unit = source.models[reading.model].x_unit.clone();
                let valid = parse_typed_x(&text, &unit).is_some();
                let field = ui.put(
                    x_cell.shrink2(egui::vec2(2.0, 1.0)),
                    egui::TextEdit::singleline(&mut text)
                        .id(ui.id().with(("cursor-table-x", key)))
                        .font(font.clone())
                        .horizontal_align(egui::Align::Max)
                        .text_color(if valid { c.text } else { c.warn }),
                );
                if !field.has_focus() && !field.lost_focus() {
                    field.request_focus();
                }
                let entered =
                    field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                let parsed = parse_typed_x(&text, &unit);
                let keep_editing = if entered {
                    // An unchanged entry leaves the column where it stands
                    // rather than at its printed rounding.
                    if text != edit.original
                        && let Some(x) = parsed
                    {
                        move_column(state, source, reading, x, requests);
                    }
                    text != edit.original && parsed.is_none()
                } else {
                    !field.lost_focus()
                };
                ui.data_mut(|data| {
                    if keep_editing {
                        data.insert_temp(
                            x_edit_id(),
                            XEdit {
                                text,
                                ..edit.clone()
                            },
                        );
                    } else {
                        data.remove::<XEdit>(x_edit_id());
                    }
                });
            }
            _ => {
                let response = ui
                    .interact(
                        x_cell,
                        ui.id().with(("cursor-table-x-cell", key)),
                        egui::Sense::click(),
                    )
                    .on_hover_text("Click to type where it stands");
                if response.clicked()
                    && let Some(text) = table.x_cells.get(column).filter(|_| reading.x.is_ok())
                {
                    ui.data_mut(|data| {
                        data.insert_temp(
                            x_edit_id(),
                            XEdit {
                                column: reading.id,
                                text: text.clone(),
                                original: text.clone(),
                            },
                        );
                    });
                }
            }
        }
    }
}

/// Move a column to `x`: a cursor at once, a marker through the view
/// that keeps it.
fn move_column(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    reading: &ReadingColumn,
    x: f64,
    requests: &mut MarkerActions,
) {
    match reading.id {
        ReadingColumnId::Cursor(id) => {
            state.cursors.set(id, x);
            if id == CursorId::A {
                state.cursor_a_anchor = reading.anchor.clone();
            }
            state.stepping_cursor = id;
        }
        ReadingColumnId::Marker(selector) => {
            requests.moved = Some(MarkerMove {
                selector,
                x,
                nearest_sample: nearest_sample_x(
                    &source.models[reading.model],
                    reading.anchor.as_ref(),
                    x,
                ),
            });
        }
    }
}

fn column_menu(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    table: &CursorTable,
    column: usize,
    requests: &mut MarkerActions,
) {
    let reading = &table.columns[column];
    if ui
        .add_enabled(column != table.base, egui::Button::new("Make base"))
        .on_hover_text("Read the other columns' differences against this one")
        .clicked()
    {
        state.readout_base = Some(reading.id);
        ui.close();
    }
    let snapped =
        reading.x.as_ref().ok().and_then(|&x| {
            nearest_sample_x(&source.models[reading.model], reading.anchor.as_ref(), x)
        });
    if ui
        .add_enabled(
            snapped.is_some(),
            egui::Button::new("Snap to nearest sample"),
        )
        .on_hover_text("Move it onto the nearest retained sample of the trace it rides")
        .clicked()
    {
        if let Some(x) = snapped {
            move_column(state, source, reading, x, requests);
        }
        ui.close();
    }
    if let ReadingColumnId::Marker(selector) = reading.id
        && ui
            .button("Rename\u{2026}")
            .on_hover_text("Edit the marker's label and kind")
            .clicked()
    {
        requests.edit = Some(selector);
        ui.close();
    }
    ui.separator();
    if ui.button("Delete").clicked() {
        match reading.id {
            ReadingColumnId::Cursor(CursorId::A) => {
                // B takes A's place, as a pair always has an A.
                match state.cursors.b.take() {
                    Some(b) => {
                        state.cursors.a = Some(b);
                        state.cursor_a_anchor = None;
                        state.stepping_cursor = CursorId::A;
                    }
                    None => state.clear_cursors(),
                }
            }
            ReadingColumnId::Cursor(CursorId::B) => {
                state.cursors.b = None;
                state.stepping_cursor = CursorId::A;
            }
            ReadingColumnId::Marker(selector) => requests.remove = Some(selector),
        }
        ui.close();
    }
}

/// Where `ys` over `xs` crosses `level`, by linear interpolation between
/// the samples either side.
pub fn crossings(xs: &[f64], ys: &[f64], level: f64) -> Vec<f64> {
    let mut found = Vec::new();
    for index in 1..xs.len().min(ys.len()) {
        let (x0, x1, y0, y1) = (
            xs[index - 1],
            xs[index],
            ys[index - 1] - level,
            ys[index] - level,
        );
        if !(x0.is_finite() && x1.is_finite() && y0.is_finite() && y1.is_finite()) {
            continue;
        }
        if y0 == 0.0 {
            if found.last() != Some(&x0) {
                found.push(x0);
            }
        } else if y0.signum() != y1.signum() && y1 != 0.0 {
            found.push(x0 + (x1 - x0) * (y0 / (y0 - y1)));
        } else if y1 == 0.0 {
            found.push(x1);
        }
    }
    found
}

/// One pane's horizontal cursors: its levels, and where its traces cross
/// them.
struct HorizontalSection {
    pane: WavePanePresentationKey,
    /// The pane, as its band names it.
    name: String,
    /// Each level, then their difference, in their colours.
    heads: Vec<(String, egui::Color32)>,
    rows: Vec<CrossingRow>,
}

/// Where one trace crosses each level of its pane.
struct CrossingRow {
    name: String,
    swatch: egui::Color32,
    swatch_style: Option<SwatchStyle>,
    /// Per level, every crossing in X order, as printed.
    crossings: Vec<Vec<String>>,
}

fn horizontal_sections(
    source: ReadoutInput<'_>,
    panes: &[(WavePanePresentationKey, HorizontalCursors)],
    t: &Tokens,
) -> Vec<HorizontalSection> {
    let c = t.color;
    let digits = usize::from(source.presentation.displayed_significant_digits().get());
    let quantity_policy = source.quantity_policy;
    let mut sections = Vec::new();
    for (pane, cursors) in panes {
        let model = source
            .models
            .iter()
            .find(|model| model.analysis_key == pane.analysis);
        let unit = model.map_or_else(
            || pane.unit.clone(),
            |model| model.pane_axis_unit(&pane.unit),
        );
        let format = |value: f64| rspice_ui_kit::plot::fmt_si_significant(value, &unit, digits);
        let levels: Vec<(usize, f64)> = cursors
            .iter()
            .enumerate()
            .filter_map(|(slot, level)| level.map(|level| (slot, level)))
            .collect();
        let mut heads: Vec<(String, egui::Color32)> = levels
            .iter()
            .map(|&(slot, level)| {
                let color = if slot == 0 {
                    c.canvas_cursor_a
                } else {
                    c.canvas_cursor_b
                };
                (format!("H{} {}", slot + 1, format(level)), color)
            })
            .collect();
        if let [Some(h1), Some(h2)] = cursors {
            heads.push((format!("\u{0394}Y {}", format(h2 - h1)), c.text));
        }
        let mut rows = Vec::new();
        if let Some(model) = model {
            let traces = model
                .unit_panes()
                .into_iter()
                .find(|unit_pane| unit_pane.unit == pane.unit)
                .map(|unit_pane| unit_pane.traces)
                .unwrap_or_default();
            for index in traces {
                let Some(trace) = model.traces.get(index).filter(|trace| trace.visible) else {
                    continue;
                };
                let Some((xs, ys)) = trace.held() else {
                    continue;
                };
                rows.push(CrossingRow {
                    name: (source.display_name)(&trace.name).into_owned(),
                    swatch: trace.color,
                    swatch_style: Some(SwatchStyle::of(trace)),
                    crossings: levels
                        .iter()
                        .map(|&(_, level)| {
                            crossings(&xs, &ys, level)
                                .into_iter()
                                .map(|x| model.format_x(x, digits, quantity_policy))
                                .collect()
                        })
                        .collect(),
                });
            }
        }
        sections.push(HorizontalSection {
            pane: pane.clone(),
            name: super::dock::horizontal_pane_name(source.models, pane),
            heads,
            rows,
        });
    }
    sections
}

/// A level's crossings as they fit in `width`: their count, then as many
/// as fit, and an ellipsis for the rest.
fn fitted_crossings(ui: &Ui, crossings: &[String], width: f32, font: &egui::FontId) -> String {
    if crossings.is_empty() {
        return "no crossing".to_owned();
    }
    let mut text = format!("{} \u{00b7}", crossings.len());
    for (index, crossing) in crossings.iter().enumerate() {
        let next = format!("{text}{} {crossing}", if index == 0 { "" } else { "," });
        let rest = index + 1 < crossings.len();
        let probe = if rest {
            format!("{next}, \u{2026}")
        } else {
            next.clone()
        };
        if text_width(ui, &probe, font) > width {
            text.push_str(if index == 0 {
                " \u{2026}"
            } else {
                ", \u{2026}"
            });
            return text;
        }
        text = next;
    }
    text
}

/// The horizontal cursors: each pane's band with its levels and their
/// difference, then where each of the pane's traces crosses each level.
fn paint_horizontal(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    sections: &[HorizontalSection],
    name_w: f32,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let head_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);

    // One column per level: as wide as its head or its crossings, up to a
    // bound past which the list is cut with an ellipsis.
    let levels = sections
        .iter()
        .map(|section| section.heads.len())
        .max()
        .unwrap_or(0);
    let mut widths = vec![VALUE_COLUMN_MIN_W; levels];
    for section in sections {
        for (column, (head, _)) in section.heads.iter().enumerate() {
            widths[column] = widths[column].max(text_width(ui, head, &head_font) + CELL_PAD * 2.0);
        }
        for row in &section.rows {
            for (column, listed) in row.crossings.iter().enumerate() {
                let whole = fitted_crossings(ui, listed, f32::INFINITY, &font);
                widths[column] = widths[column]
                    .max((text_width(ui, &whole, &font) + CELL_PAD * 2.0).min(CROSSINGS_MAX_W));
            }
        }
    }
    let rows: usize = sections.iter().map(|section| 1 + section.rows.len()).sum();
    let width = READOUT_PAD_X * 2.0 + name_w + widths.iter().sum::<f32>() + 24.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(width.max(ui.available_width()), rows as f32 * READOUT_ROW_H),
        egui::Sense::hover(),
    );
    let left = rect.left() + READOUT_PAD_X;
    let painter = ui.painter().with_clip_rect(rect);
    let row_rect = |row: usize| {
        let top = rect.top() + row as f32 * READOUT_ROW_H;
        egui::Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + READOUT_ROW_H),
        )
    };
    let mut clear = None;
    let mut row_index = 0;
    for section in sections {
        let band = row_rect(row_index);
        painter.rect_filled(band, 0.0, c.bg_panel_2);
        painter.text(
            egui::pos2(left, band.center().y),
            egui::Align2::LEFT_CENTER,
            &section.name,
            head_font.clone(),
            c.text_dim,
        );
        let mut x = left + name_w;
        for ((head, color), width) in section.heads.iter().zip(&widths) {
            painter.text(
                egui::pos2(x, band.center().y),
                egui::Align2::LEFT_CENTER,
                head,
                head_font.clone(),
                *color,
            );
            x += width;
        }
        let remove = egui::Rect::from_center_size(
            egui::pos2(band.right() - READOUT_PAD_X - 8.0, band.center().y),
            egui::vec2(16.0, READOUT_ROW_H),
        );
        let response = ui
            .interact(
                remove,
                ui.id().with(("horizontal-remove", &section.pane)),
                egui::Sense::click(),
            )
            .on_hover_text("Remove this pane's horizontal cursors");
        painter.text(
            remove.center(),
            egui::Align2::CENTER_CENTER,
            "\u{00d7}",
            font.clone(),
            if response.hovered() {
                c.text
            } else {
                c.text_dim
            },
        );
        if response.clicked() {
            clear = Some(section.pane.clone());
        }
        row_index += 1;
        for row in &section.rows {
            let line = row_rect(row_index);
            let swatch = egui::Rect::from_x_y_ranges(left..=left + SWATCH_W - 4.0, line.y_range());
            paint_swatch(&painter, swatch, row.swatch, row.swatch_style, &t);
            painter.text(
                egui::pos2(left + SWATCH_W, line.center().y),
                egui::Align2::LEFT_CENTER,
                &row.name,
                font.clone(),
                c.text_dim,
            );
            let mut x = left + name_w;
            for (listed, width) in row.crossings.iter().zip(&widths) {
                let text = fitted_crossings(ui, listed, width - CELL_PAD * 2.0, &font);
                painter.text(
                    egui::pos2(x, line.center().y),
                    egui::Align2::LEFT_CENTER,
                    text,
                    font.clone(),
                    c.text,
                );
                x += width;
            }
            row_index += 1;
        }
    }
    if let Some(pane) = clear {
        state.horizontal_cursors.remove(&pane);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_typed_x_reads_as_the_table_prints_it() {
        assert_eq!(parse_typed_x("10 ns", "s"), Some(10.0e-9));
        assert_eq!(parse_typed_x("1.5 MHz", "Hz"), Some(1.5e6));
        assert_eq!(parse_typed_x("2 mHz", "Hz"), Some(2.0e-3));
        assert_eq!(parse_typed_x("2.2u", "s"), Some(2.2e-6));
        assert_eq!(parse_typed_x("3 \u{00b5}s", "s"), Some(3.0e-6));
        assert_eq!(parse_typed_x("1meg", "Hz"), Some(1.0e6));
        assert_eq!(parse_typed_x("4 s", "s"), Some(4.0));
        assert_eq!(parse_typed_x("-1.25e-3", "V"), Some(-1.25e-3));
        assert_eq!(parse_typed_x("12", ""), Some(12.0));
        assert_eq!(parse_typed_x("ten", "s"), None);
        assert_eq!(parse_typed_x("", "s"), None);
    }

    #[test]
    fn crossings_are_found_between_samples_and_on_them() {
        let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
        let ys = [0.0, 1.0, 0.0, -1.0, 0.0];
        assert_eq!(crossings(&xs, &ys, 0.5), vec![0.5, 1.5]);
        assert_eq!(crossings(&xs, &ys, 0.0), vec![0.0, 2.0, 4.0]);
        assert!(crossings(&xs, &ys, 2.0).is_empty());
    }
}
