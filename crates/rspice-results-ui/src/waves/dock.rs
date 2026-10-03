//! The cursor readout: what the cursors and markers currently say.
//!
//! Every number here is read from the sample the cursor is actually on, and a
//! value that would require interpolating between samples is reported under
//! the strip's declared interpolation policy rather than silently computed.
//! Deltas and slopes are shown only when both cursors sit on the same strip,
//! because a difference across two domains is not a measurement.

use super::readout::{READOUT_PAD_X, READOUT_ROW_H, readout_rows, x_separation};
use super::{ReadoutPolicy, StripModel, anchor_key, cursor_interpolation};
use crate::session::{MarkerSelector, MarkerView, ResultViewerState};
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, MarkerKind, WaveformPresentationKey,
};
use rspice_ui_kit::plot::{fmt_si_significant, sample::sample_at_with_shape};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// Borrowed projections selected and validated by the host for this readout.
#[derive(Clone, Copy)]
pub struct ReadoutInput<'a> {
    pub models: &'a [StripModel],
    pub on_screen_strips: &'a [AnalysisPresentationKey],
    pub presentation: ReadoutPolicy,
    pub quantity_policy: QuantityPresentationPolicy,
}

/// Source-bound marker requests. The host applies removal before opening an edit.
#[derive(Default)]
pub struct MarkerActions {
    pub remove: Option<MarkerSelector>,
    pub edit: Option<MarkerSelector>,
}

/// Height of the cursor readout strip's header row.
pub const READOUT_HEADER_H: f32 = 26.0;
/// Tallest scroll viewport owned by the dock body.
pub const READOUT_BODY_MAX_H: f32 = 212.0;
/// Tallest complete dock: the fixed header plus its independently capped body.
pub const READOUT_MAX_H: f32 = READOUT_HEADER_H + READOUT_BODY_MAX_H;
const READOUT_DESKTOP_SPLIT_MIN_W: f32 = 680.0;
const READOUT_COLUMN_SEAM: f32 = 1.0;
const CURSOR_TABLE_MIN_W: f32 = 660.0;
const MARKER_EMPTY_H: f32 = 32.0;

/// Rows the readout strip will report for the cursor's strip.
///
/// The count comes from the projection the strip actually draws, not from the
/// retained waveform list: a run overlay, a real/imaginary split, a corner
/// family and a sweep that turns around each put more rows on the table than
/// there are retained waveforms, and the band was sized for the smaller
/// number and then scrolled. The same walk answers the opposite case — a
/// viewer whose projection drops the cursor's strip entirely reserved rows
/// nothing could fill.
pub fn readout_row_count(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> usize {
    let Some(index) = state.cursor_strip else {
        return 0;
    };
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let cursors = state.cursors;
    let models = source.models;
    models
        .iter()
        .find(|model| model.analysis_index == index)
        .map_or(0, |model| {
            readout_rows(model, cursors, presentation, quantity_policy).len()
        })
}

/// Height of one marker row.
pub const MARKER_ROW_H: f32 = 22.0;

/// Kind owns the marker's colour: a spec limit reads as a bound to meet,
/// a peak as a called-out feature, a note as neutral annotation.
pub fn marker_color(kind: MarkerKind, t: &Tokens) -> egui::Color32 {
    match kind {
        MarkerKind::Note => t.color.text,
        MarkerKind::Peak => t.color.accent,
        MarkerKind::Spec => t.color.warn,
    }
}

/// Tag text: the id always, the note only when the user wrote one.
pub fn marker_label(marker: MarkerView<'_>) -> String {
    let id = marker.display_id();
    let note = marker.note().trim();
    if note.is_empty() {
        id
    } else {
        format!("{id} · {note}")
    }
}

/// Markers the strip will list, in placement order, across both stores.
///
/// A persistent pane's retained markers are listed here beside the dataset's
/// quick markers so the reader has exactly one marker list, not one per owner.
pub fn visible_markers<'a>(
    state: &'a ResultViewerState,
    source: ReadoutInput<'_>,
) -> Vec<MarkerView<'a>> {
    source
        .on_screen_strips
        .iter()
        .copied()
        .flat_map(|analysis| state.strip_markers(analysis))
        .collect()
}

/// Whether the cursor has a strip on this sheet to report about.
///
/// "On this sheet" is the operative half: the retained analysis surviving is
/// not enough, because switching viewers reprojects the stack and can leave
/// the cursor pointing at a strip the sheet no longer draws. The band was
/// still reserved for it, and the table it opened for drew nothing.
fn cursor_target_available(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> bool {
    if !state.cursor_readout_active() {
        return false;
    }
    let Some(index) = state.cursor_strip else {
        return false;
    };
    let models = source.models;
    models.iter().any(|model| model.analysis_index == index)
}

fn cursor_body_height(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> f32 {
    if !cursor_target_available(state, source) {
        return 0.0;
    }
    // One column-header row, one X-domain row, then every projected row.
    (2 + readout_row_count(state, source)) as f32 * READOUT_ROW_H
}

pub fn marker_body_height(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> f32 {
    let markers = visible_markers(state, source).len();
    if markers > 0 {
        markers as f32 * MARKER_ROW_H
    } else if cursor_target_available(state, source) {
        MARKER_EMPTY_H
    } else {
        0.0
    }
}

fn readout_body_content_height(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> f32 {
    cursor_body_height(state, source).max(marker_body_height(state, source))
}

fn readout_columns_side_by_side(width: f32, cursor: bool, markers: bool) -> bool {
    cursor && markers && width >= READOUT_DESKTOP_SPLIT_MIN_W
}

/// Exact height the readout strip needs, or zero when it stands down.
///
/// The dock is content-fit up to a 212 px body, then its one body viewport
/// scrolls. Expanded cursor/marker content, marker-only, collapsed-header,
/// and no-strip states remain distinct.
///
/// Row counts and painting use the same host-provided strip projections.
pub fn readout_strip_height(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> f32 {
    let cursor = state.cursor_readout_active();
    let markers = !visible_markers(state, source).is_empty();
    if !cursor && !markers {
        return 0.0;
    }
    if state.readout_collapsed {
        return READOUT_HEADER_H;
    }
    (READOUT_HEADER_H + readout_body_content_height(state, source)).min(READOUT_MAX_H)
}

/// The cursor readout: one X row naming A, B and Δ, then the value each
/// visible trace takes at those cursors.
///
/// This is the single home for the cursor readout. The inspector reports
/// window statistics the strip does not carry, and never repeats these
/// numbers one panel away.
pub fn readout_strip(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    height: f32,
) -> MarkerActions {
    let mut actions = MarkerActions::default();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter()
        .hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, c.border));

    let header = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(
            rect.right(),
            (rect.top() + READOUT_HEADER_H).min(rect.bottom()),
        ),
    );
    readout_header(ui, state, source, header);
    if state.readout_collapsed || rect.bottom() <= header.bottom() {
        return actions;
    }

    let body = egui::Rect::from_min_max(header.left_bottom(), rect.right_bottom());
    ui.painter()
        .hline(body.x_range(), body.top(), egui::Stroke::new(1.0, c.border));
    let mut body_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("results-readout-body")
            .max_rect(body)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    body_ui.set_clip_rect(body);
    egui::ScrollArea::vertical()
        .id_salt("rspice.results.readout")
        .auto_shrink([false, false])
        .show(&mut body_ui, |ui| {
            readout_body(ui, state, source, &mut actions)
        });
    actions
}

fn readout_header(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    rect: egui::Rect,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let cursor = state.cursor_readout_active();
    let trace_count = if cursor {
        readout_row_count(state, source)
    } else {
        0
    };
    let marker_count = visible_markers(state, source).len();
    let title = if cursor {
        "Cursors & markers"
    } else {
        "Markers"
    };
    let count = if cursor {
        format!(
            "{trace_count} row{} · {marker_count} marker{}",
            if trace_count == 1 { "" } else { "s" },
            if marker_count == 1 { "" } else { "s" }
        )
    } else {
        format!(
            "{marker_count} marker{}",
            if marker_count == 1 { "" } else { "s" }
        )
    };
    let mut header_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("results-readout-header")
            .max_rect(rect.shrink2(egui::vec2(8.0, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    header_ui.set_clip_rect(rect);
    header_ui.spacing_mut().item_spacing.x = 6.0;
    header_ui.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_1, FontWeight::Medium))
            .color(c.text),
    );
    header_ui.label(
        egui::RichText::new(count)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(c.text_faint)
            .background_color(c.bg_inset),
    );
    header_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let collapsed = state.readout_collapsed;
        let label = if collapsed {
            "Expand readout"
        } else {
            "Collapse readout"
        };
        if ui
            .add_sized(
                egui::vec2(24.0, 22.0),
                egui::Button::new(if collapsed { "▴" } else { "▾" }).frame(false),
            )
            .on_hover_text(label)
            .clicked()
        {
            state.readout_collapsed = !collapsed;
        }
    });
}

fn readout_body(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    actions: &mut MarkerActions,
) {
    let cursor = cursor_target_available(state, source);
    // Cursor mode keeps an explicit markers panel even when it is empty.
    let markers = !visible_markers(state, source).is_empty() || cursor;
    let width = ui.available_width();
    if readout_columns_side_by_side(width, cursor, markers) {
        let marker_width = (width * 0.42).clamp(240.0, 720.0).min(width);
        let cursor_width = (width - marker_width - READOUT_COLUMN_SEAM).max(0.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(
                egui::vec2(cursor_width, cursor_body_height(state, source)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| cursor_readout_section(ui, state, source),
            );
            let (seam, _) = ui.allocate_exact_size(
                egui::vec2(
                    READOUT_COLUMN_SEAM,
                    readout_body_content_height(state, source),
                ),
                egui::Sense::hover(),
            );
            ui.painter()
                .rect_filled(seam, 0.0, Tokens::get(ui.ctx()).color.border);
            ui.allocate_ui_with_layout(
                egui::vec2(marker_width, marker_body_height(state, source)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| marker_section(ui, state, source, actions),
            );
        });
    } else {
        if cursor {
            ui.allocate_ui_with_layout(
                egui::vec2(width, cursor_body_height(state, source)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| cursor_readout_section(ui, state, source),
            );
        }
        if markers {
            if cursor {
                let (seam, _) = ui.allocate_exact_size(
                    egui::vec2(width, READOUT_COLUMN_SEAM),
                    egui::Sense::hover(),
                );
                ui.painter()
                    .rect_filled(seam, 0.0, Tokens::get(ui.ctx()).color.border);
            }
            ui.allocate_ui_with_layout(
                egui::vec2(width, marker_body_height(state, source)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| marker_section(ui, state, source, actions),
            );
        }
    }
}

/// The instrument bar's inline `A · B · Δ`, for the collapsed readout.
///
/// Collapsing the strip hides the register that owns these three numbers, so
/// the bar states them until it is expanded again — never both at once, which
/// is the duplication the results de-duplication pass removed.
pub fn inline_cursor_readout(
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
) -> Option<String> {
    if !state.readout_collapsed || !state.cursor_readout_active() {
        return None;
    }
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = source.models;
    let model = state
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;
    let cursors = state.cursors;
    let a = cursors.a?;
    let a_text = model.format_x(a, significant_digits, quantity_policy);
    let Some(b) = cursors.b else {
        return Some(format!("A {a_text}"));
    };
    Some(format!(
        "A {a_text} · B {} · \u{0394} {}",
        model.format_x(b, significant_digits, quantity_policy),
        x_separation(model, a, b, significant_digits, quantity_policy),
    ))
}

/// The A/B table: one X row, then the value of every visible trace.
fn cursor_readout_section(ui: &mut Ui, state: &mut ResultViewerState, source: ReadoutInput<'_>) {
    let table_width = ui.available_width().max(CURSOR_TABLE_MIN_W);
    let table_height = cursor_body_height(state, source);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.cursor-readout-horizontal")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(table_width, table_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_min_height(table_height);
                    cursor_readout_table(ui, state, source);
                },
            );
        });
}

fn cursor_readout_table(ui: &mut Ui, state: &mut ResultViewerState, source: ReadoutInput<'_>) {
    if !state.cursor_readout_active() {
        return;
    }
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let models = source.models;
    let Some(model) = state
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))
    else {
        return;
    };
    let cursors = state.cursors;

    super::readout::cursor_readout_table(ui, model, cursors, presentation, quantity_policy);
}

/// The marker half of the strip: one editable row per marker.
///
/// Markers are document content, so their row is the place they are named,
/// re-kinded and removed — there is no second marker list elsewhere to
/// disagree with this one.
pub fn marker_section(
    ui: &mut Ui,
    state: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    actions: &mut MarkerActions,
) {
    let rect = ui.available_rect_before_wrap();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let models = source.models;

    // Rows are derived once, up front, so a row can never describe a marker
    // the plot placed somewhere else — and so the borrow of the marker stores
    // ends before the row widgets need `state` mutably.
    struct MarkerRow {
        selector: MarkerSelector,
        display_id: String,
        kind: MarkerKind,
        anchor: WaveformPresentationKey,
        x: f64,
        analysis: AnalysisPresentationKey,
        trace_name: String,
        note: String,
        retained: bool,
    }
    let shown: Vec<MarkerRow> = visible_markers(state, source)
        .into_iter()
        .map(|marker| MarkerRow {
            selector: marker.selector(),
            display_id: marker.display_id(),
            kind: marker.kind(),
            anchor: marker.anchor().clone(),
            x: marker.x(),
            analysis: marker.analysis(),
            trace_name: marker.trace_name().to_owned(),
            note: marker.note().to_owned(),
            retained: matches!(marker, MarkerView::Document(_)),
        })
        .collect();
    if shown.is_empty() {
        ui.painter().text(
            egui::pos2(rect.left() + READOUT_PAD_X, rect.top() + 8.0),
            egui::Align2::LEFT_TOP,
            "No markers on this sheet. Drop one at cursor A with +M.",
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text_faint,
        );
        return;
    }

    let mut remove: Option<MarkerSelector> = None;
    let mut edit: Option<MarkerSelector> = None;
    for (index, entry) in shown.iter().enumerate() {
        let top = rect.top() + index as f32 * MARKER_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left() + READOUT_PAD_X, top),
            egui::pos2(rect.right() - READOUT_PAD_X, top + MARKER_ROW_H),
        );
        let selector = entry.selector;
        let kind = entry.kind;
        let anchor = entry.anchor.clone();
        let marker_x = entry.x;
        let analysis_key = entry.analysis;
        let trace_name = entry.trace_name.clone();
        let model = models
            .iter()
            .find(|model| model.analysis_key == analysis_key);
        let position = model.map_or_else(
            || fmt_si_significant(marker_x, "", significant_digits),
            |model| {
                format!(
                    "{} = {}",
                    model.x_label(),
                    model.format_x(marker_x, significant_digits, quantity_policy)
                )
            },
        );
        // A spec marker constrains the X position alone; reporting a curve
        // value against it would assert a reading it does not make.
        let value = kind.rides_a_trace().then(|| {
            model
                .and_then(|model| {
                    let trace = model
                        .traces
                        .iter()
                        .find(|trace| !trace.overlay && anchor_key(model, trace) == anchor)?;
                    // Read through the sweep's shape: a loop has no single
                    // value here, and bisecting across its turnaround puts a
                    // number in the row the curve never takes. This is the
                    // canvas' own fallback — the first branch that covers the
                    // marker's X — so the row and the tagged point agree.
                    let sampled = sample_at_with_shape(
                        &trace.x,
                        &trace.y,
                        &trace.shape,
                        marker_x,
                        interpolation,
                    );
                    Some(model.format_trace_value(
                        trace,
                        sampled,
                        significant_digits,
                        quantity_policy,
                    ))
                })
                .unwrap_or_else(|| "trace unavailable".to_owned())
        });

        let mut row_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(row)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        row_ui.set_clip_rect(row);
        row_ui.spacing_mut().item_spacing.x = 8.0;
        let color = marker_color(kind, &t);
        row_ui
            .label(
                egui::RichText::new(&entry.display_id)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(color),
            )
            .on_hover_text(if entry.retained {
                "Retained by this result document"
            } else {
                "Saved with the project"
            });
        // The kind is stated, not cycled: a click that silently reclassifies
        // what a marker asserts is a decision made by accident.
        row_ui.label(
            egui::RichText::new(kind.label())
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        );
        // Which store owns the marker changes what removing it means, so the
        // row states it rather than leaving the reader to infer it.
        if entry.retained {
            row_ui.label(
                egui::RichText::new("retained")
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        }
        if row_ui
            .add(
                egui::Button::new(
                    egui::RichText::new("\u{270e}")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                )
                .frame(false),
            )
            .on_hover_text("Edit this marker's label and kind")
            .clicked()
        {
            edit = Some(selector);
        }
        if row_ui
            .add(
                egui::Button::new(
                    egui::RichText::new("×")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                )
                .frame(false),
            )
            .on_hover_text(if entry.retained {
                "Remove this marker from the result document"
            } else {
                "Remove this marker"
            })
            .clicked()
        {
            remove = Some(selector);
        }
        row_ui.label(
            egui::RichText::new(trace_name)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if kind.rides_a_trace() {
                    c.text_dim
                } else {
                    c.text_faint
                }),
        );
        row_ui.label(
            egui::RichText::new(position)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(c.text),
        );
        if let Some(value) = value {
            row_ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text),
            );
        }
        // The note takes what is left of the row. It reads as text here and
        // is edited in the marker dialog, so a stray keystroke over the strip
        // cannot rewrite what a marker says.
        let note = &entry.note;
        if note.is_empty() {
            row_ui.label(
                egui::RichText::new("no label")
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        } else {
            row_ui
                .label(
                    egui::RichText::new(note)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_dim),
                )
                .on_hover_text(note);
        }
    }
    actions.remove = remove;
    actions.edit = edit;
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Window statistics over the cursor span.
///
/// The A/B/Δ readout itself lives in the stage's readout strip; repeating it
/// one panel away is what the results de-duplication pass removed.
pub fn right_panel(ui: &mut Ui, state: &mut ResultViewerState, source: ReadoutInput<'_>) {
    let presentation = source.presentation;
    let quantity_policy = source.quantity_policy;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = source.models;
    let cursor_model = state
        .cursor_strip
        .and_then(|index| models.iter().find(|m| m.analysis_index == index));

    // Statistics over the cursor window (or the full range).
    let cursors = state.cursors;
    let measured_model = cursor_model.or_else(|| models.first());
    if let Some(model) = measured_model {
        let window = match (cursors.a, cursors.b) {
            (Some(a), Some(b)) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        super::readout::measurement_panel(
            ui,
            &mut state.derived,
            model,
            window,
            significant_digits,
            quantity_policy,
        );
    }
}

#[cfg(test)]
mod tests;
