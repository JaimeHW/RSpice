//! The cursor readout: what the cursors and markers currently say.
//!
//! Every number here is read from the sample the cursor is actually on, and a
//! value that would require interpolating between samples is reported under
//! the strip's declared interpolation policy rather than silently computed.
//! Deltas and slopes are shown only when both cursors sit on the same strip,
//! because a difference across two domains is not a measurement.

use super::cursor_values::{marker_position, x_separation};
use super::readout::{READOUT_PAD_X, READOUT_ROW_H, readout_rows};
use super::{ReadoutPolicy, StripModel, anchor_key, cursor_interpolation};
use crate::session::{HorizontalCursors, MarkerSelector, MarkerView, ResultViewerState};
use egui::Ui;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, MarkerKind, WavePanePresentationKey,
};
use rspice_ui_kit::plot::{CursorPair, fmt_si_significant, sample::sample_at_with_shape};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

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
                + strip_lane_rows(state, model, cursors, presentation).len()
        })
}

/// The cursor table's rows for the strip's logic, bus and `E()` lanes, read
/// from their exact event histories.
fn strip_lane_rows(
    state: &mut ResultViewerState,
    model: &StripModel,
    cursors: CursorPair,
    presentation: ReadoutPolicy,
) -> Vec<super::readout::ReadoutRow> {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let specs = super::lanes::lane_specs(model, state.event_lanes.get(&model.analysis_key));
    let user = super::lanes::user_bus_lanes(&mut state.derived, model, &specs);
    super::lanes::lane_readout_rows(model, &specs, &user, cursors, digits)
}

/// Height of one marker row.
pub const MARKER_ROW_H: f32 = 22.0;

/// Kind owns the marker's colour: a spec limit reads as a bound to meet,
/// a peak as a called-out feature, a note as neutral annotation.
pub fn marker_color(kind: MarkerKind, t: &Tokens) -> egui::Color32 {
    match kind {
        MarkerKind::Note => t.color.text,
        MarkerKind::Peak | MarkerKind::Valley | MarkerKind::Crossing(_) => t.color.accent,
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
    let markers =
        visible_markers(state, source).len() + horizontal_cursor_rows(state, source).len();
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
    let markers = !visible_markers(state, source).is_empty()
        || !horizontal_cursor_rows(state, source).is_empty();
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
    readout_header(ui, state, header);
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

fn readout_header(ui: &mut Ui, state: &mut ResultViewerState, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let cursor = state.cursor_readout_active();
    let title = if cursor {
        "Cursors & markers"
    } else {
        "Markers"
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
    // The rows and markers are directly below; counting them is the band
    // describing what the reader can see.
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
    let markers = !visible_markers(state, source).is_empty()
        || !horizontal_cursor_rows(state, source).is_empty()
        || cursor;
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
    let lane_rows = strip_lane_rows(state, model, cursors, presentation);
    if let Some(stepping) = super::readout::cursor_readout_table(
        ui,
        model,
        cursors,
        state.stepping_cursor,
        presentation,
        quantity_policy,
        lane_rows,
    ) {
        state.stepping_cursor = stepping;
    }
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

    // The horizontal cursors lead: one row per pane that has any, its H1, H2
    // and the ΔY between them in the pane's own unit.
    let horizontal = horizontal_cursor_rows(state, source);
    let mut clear_horizontal = None;
    for (index, (pane, cursors)) in horizontal.iter().enumerate() {
        let top = rect.top() + index as f32 * MARKER_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left() + READOUT_PAD_X, top),
            egui::pos2(rect.right() - READOUT_PAD_X, top + MARKER_ROW_H),
        );
        let mut row_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(row)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        row_ui.set_clip_rect(row);
        row_ui.spacing_mut().item_spacing.x = 8.0;
        let mono = |text: String, color| {
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color)
        };
        let format =
            |value: f64| super::units::fmt_in_unit_readout(value, &pane.unit, significant_digits);
        row_ui.label(mono(horizontal_pane_name(models, pane), c.text_dim));
        for (slot, color) in [(0, c.canvas_cursor_a), (1, c.canvas_cursor_b)] {
            if let Some(value) = cursors[slot] {
                row_ui.label(mono(format!("H{}", slot + 1), color));
                row_ui.label(mono(format(value), c.text));
            }
        }
        if let [Some(h1), Some(h2)] = cursors {
            row_ui.label(mono(format!("\u{0394}Y {}", format(h2 - h1)), c.text));
        }
        if row_ui
            .add(egui::Button::new(mono("×".to_owned(), c.text_dim)).frame(false))
            .on_hover_text("Remove this pane's horizontal cursors")
            .clicked()
        {
            clear_horizontal = Some(pane.clone());
        }
    }
    if let Some(pane) = clear_horizontal {
        state.horizontal_cursors.remove(&pane);
    }
    let rect = egui::Rect::from_min_max(
        egui::pos2(
            rect.left(),
            rect.top() + horizontal.len() as f32 * MARKER_ROW_H,
        ),
        rect.max,
    );

    // Rows are derived once, up front, so a row can never describe a marker
    // the plot placed somewhere else — and so the borrow of the marker stores
    // ends before the row widgets need `state` mutably.
    struct MarkerRow {
        selector: MarkerSelector,
        display_id: String,
        kind: MarkerKind,
        analysis: AnalysisPresentationKey,
        trace_name: String,
        note: String,
        retained: bool,
        /// Where the marker is now, or where it was put when its search
        /// finds nothing there.
        position: Result<f64, f64>,
        /// The anchored trace's value there, with the strip and trace it
        /// was read from.
        value: Option<(usize, usize, f64)>,
    }
    let read = |marker: MarkerView<'_>| -> MarkerRow {
        let model_index = models
            .iter()
            .position(|model| model.analysis_key == marker.analysis());
        let position = model_index.map_or(Ok(marker.x()), |index| {
            marker_position(&models[index], marker.anchor(), marker.x(), marker.kind())
        });
        let value = model_index
            .filter(|_| marker.kind().rides_a_trace())
            .and_then(|model_index| {
                let model = &models[model_index];
                let trace_index = model.traces.iter().position(|trace| {
                    !trace.overlay && anchor_key(model, trace) == *marker.anchor()
                })?;
                let trace = &model.traces[trace_index];
                // Read through the sweep's shape: a loop has no single
                // value here, and bisecting across its turnaround puts a
                // number in the row the curve never takes. This is the
                // canvas' own fallback — the first branch that covers the
                // marker's X — so the row and the tagged point agree.
                let at = position.unwrap_or_else(|seed| seed);
                let sampled =
                    sample_at_with_shape(&trace.x, &trace.y, &trace.shape, at, interpolation);
                Some((model_index, trace_index, sampled))
            });
        MarkerRow {
            selector: marker.selector(),
            display_id: marker.display_id(),
            kind: marker.kind(),
            analysis: marker.analysis(),
            trace_name: marker.trace_name().to_owned(),
            note: marker.note().to_owned(),
            retained: matches!(marker, MarkerView::Document(_)),
            position,
            value,
        }
    };
    let shown: Vec<MarkerRow> = visible_markers(state, source)
        .into_iter()
        .map(read)
        .collect();
    // An empty marker list reserves no height to say it is empty; the +M
    // control's own hover says how one is placed.
    if shown.is_empty() {
        return;
    }
    // The reference, when it is one of these markers: every other marker on
    // its strip reads its Δx, and its Δy where the two share a unit.
    let reference = state
        .reference_marker
        .and_then(|selector| shown.iter().find(|row| row.selector == selector))
        .map(|row| (row.selector, row.analysis, row.position, row.value));

    let mut remove: Option<MarkerSelector> = None;
    let mut edit: Option<MarkerSelector> = None;
    let mut reference_toggle: Option<MarkerSelector> = None;
    for (index, entry) in shown.iter().enumerate() {
        let top = rect.top() + index as f32 * MARKER_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left() + READOUT_PAD_X, top),
            egui::pos2(rect.right() - READOUT_PAD_X, top + MARKER_ROW_H),
        );
        let selector = entry.selector;
        let kind = entry.kind;
        let model = models
            .iter()
            .find(|model| model.analysis_key == entry.analysis);
        let marker_x = entry.position.unwrap_or_else(|seed| seed);
        let position = model.map_or_else(
            || super::units::as_readout(&fmt_si_significant(marker_x, "", significant_digits)),
            |model| {
                format!(
                    "{} = {}",
                    model.x_label(),
                    model.format_x(marker_x, significant_digits, quantity_policy)
                )
            },
        );
        // A spec marker constrains the X position alone; reporting a curve
        // value against it would assert a reading it does not make. A
        // search that finds nothing says so rather than reading the curve
        // where the feature used to be.
        let value = kind
            .rides_a_trace()
            .then(|| match (entry.position, entry.value) {
                (Err(_), _) => format!("no {} here", kind.label()),
                (Ok(_), Some((model_index, trace_index, sampled))) => {
                    let model = &models[model_index];
                    model.format_trace_value(
                        &model.traces[trace_index],
                        sampled,
                        significant_digits,
                        quantity_policy,
                    )
                }
                (Ok(_), None) => "trace unavailable".to_owned(),
            });
        let is_reference = reference.is_some_and(|(chosen, ..)| chosen == selector);
        let delta = reference
            .filter(|(chosen, analysis, ..)| *chosen != selector && *analysis == entry.analysis)
            .and_then(|(_, _, reference_position, reference_value)| {
                let model = model?;
                let reference_x = reference_position.ok()?;
                let x = entry.position.ok()?;
                let mut text = format!(
                    "\u{0394}x {}",
                    x_separation(model, reference_x, x, significant_digits, quantity_policy)
                );
                if let (Some((_, trace_index, y)), Some((_, reference_trace, reference_y))) =
                    (entry.value, reference_value)
                {
                    let trace = &model.traces[trace_index];
                    if model.trace_unit(trace) == model.trace_unit(&model.traces[reference_trace]) {
                        text.push_str(&format!(
                            "  \u{0394}y {}",
                            model.format_trace_value(
                                trace,
                                y - reference_y,
                                significant_digits,
                                quantity_policy
                            )
                        ));
                    }
                }
                Some(text)
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
                "Saved in this result document"
            } else {
                "Saved in the project"
            });
        // The kind is stated, not cycled: a click that silently reclassifies
        // what a marker asserts is a decision made by accident. The id
        // already carries the marker's colour, so the kind is plain words,
        // padded to the longest kind so the controls after it line up.
        let kind_width = MarkerKind::ALL
            .iter()
            .map(|kind| kind.label().len())
            .max()
            .unwrap_or(0);
        row_ui.label(
            egui::RichText::new(format!("{:<kind_width$}", kind.label()))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(c.text_dim),
        );
        // The reference is chosen on its own row, as ViVA's reference
        // marker is: the word lights when this marker is the one the
        // others are read against.
        if row_ui
            .add(
                egui::Button::new(
                    egui::RichText::new("ref")
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(if is_reference { c.accent } else { c.text_faint }),
                )
                .frame(false),
            )
            .on_hover_text(if is_reference {
                "The other markers read \u{0394}x and \u{0394}y against this one; click to stop"
            } else {
                "Make this the reference marker: the others read \u{0394}x and \u{0394}y against it"
            })
            .clicked()
        {
            reference_toggle = Some(selector);
        }
        // Which store owns the marker is on the id's own hover above; the
        // word "retained" on the row said it a second time.
        if row_ui
            .add(
                // A word, as `ref` is: the pencil glyph is not in the mono
                // face and drew as an empty box.
                egui::Button::new(
                    egui::RichText::new("edit")
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(c.text_faint),
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
            egui::RichText::new(&entry.trace_name)
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
        if let Some(delta) = delta {
            row_ui.label(
                egui::RichText::new(delta)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text),
            );
        }
        // The note takes what is left of the row. It reads as text here and
        // is edited in the marker dialog, so a stray keystroke over the strip
        // cannot rewrite what a marker says.
        let note = &entry.note;
        // A marker without a note is the common case and says nothing.
        if !note.is_empty() {
            row_ui
                .label(
                    egui::RichText::new(note)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_dim),
                )
                .on_hover_text(note);
        }
    }
    if let Some(selector) = reference_toggle {
        state.toggle_reference_marker(selector);
    }
    actions.remove = remove;
    actions.edit = edit;
}

/// The panes of the on-screen strips that have a horizontal cursor, with
/// their cursors, in strip and unit order.
pub fn horizontal_cursor_rows(
    state: &ResultViewerState,
    source: ReadoutInput<'_>,
) -> Vec<(WavePanePresentationKey, HorizontalCursors)> {
    let mut rows: Vec<_> = state
        .horizontal_cursors
        .iter()
        .filter(|(pane, cursors)| {
            cursors.iter().any(Option::is_some) && source.on_screen_strips.contains(&pane.analysis)
        })
        .map(|(pane, cursors)| (pane.clone(), *cursors))
        .collect();
    rows.sort_by(|(a, _), (b, _)| {
        let strip = |pane: &WavePanePresentationKey| {
            source
                .on_screen_strips
                .iter()
                .position(|analysis| *analysis == pane.analysis)
        };
        strip(a).cmp(&strip(b)).then_with(|| a.unit.cmp(&b.unit))
    });
    rows
}

/// How a horizontal cursor row names its pane: the unit, and the strip when
/// more than one strip is on the sheet.
fn horizontal_pane_name(models: &[StripModel], pane: &WavePanePresentationKey) -> String {
    let unit = if pane.unit.is_empty() {
        "number"
    } else {
        pane.unit.as_str()
    };
    match models
        .iter()
        .find(|model| model.analysis_key == pane.analysis)
    {
        Some(model) if models.len() > 1 => format!("{} {unit}", model.kind_tag),
        _ => format!("{unit} pane"),
    }
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
