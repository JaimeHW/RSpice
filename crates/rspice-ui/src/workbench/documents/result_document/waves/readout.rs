//! The cursor readout: what the cursors and markers currently say.
//!
//! Every number here is read from the sample the cursor is actually on, and a
//! value that would require interpolating between samples is reported under
//! the strip's declared interpolation policy rather than silently computed.
//! Deltas and slopes are shown only when both cursors sit on the same strip,
//! because a difference across two domains is not a measurement.

use super::*;

/// Height of the cursor readout strip's header row.
pub(super) const READOUT_HEADER_H: f32 = 26.0;
/// Height of one per-trace row in the readout strip.
pub(super) const READOUT_ROW_H: f32 = 20.0;
/// Tallest scroll viewport owned by the dock body.
pub(super) const READOUT_BODY_MAX_H: f32 = 212.0;
/// Tallest complete dock: the fixed header plus its independently capped body.
pub(super) const READOUT_MAX_H: f32 = READOUT_HEADER_H + READOUT_BODY_MAX_H;
pub(super) const READOUT_PAD_X: f32 = 10.0;
/// Width of a trace row's leading line-swatch gutter.
const READOUT_SWATCH_W: f32 = 16.0;
const READOUT_DESKTOP_SPLIT_MIN_W: f32 = 680.0;
const READOUT_COLUMN_SEAM: f32 = 1.0;
const CURSOR_TABLE_MIN_W: f32 = 660.0;
const MARKER_EMPTY_H: f32 = 32.0;

/// Traces the readout strip will report for the cursor's strip.
pub(super) fn readout_trace_count(state: &AppState) -> usize {
    let Some(index) = state.ui.results.cursor_strip else {
        return 0;
    };
    state
        .simulation
        .active_run()
        .and_then(|run| run.analyses.get(index))
        .map_or(0, |analysis| {
            analysis
                .waveforms
                .iter()
                .filter(|waveform| waveform.visible)
                .count()
        })
}

/// Height of one marker row.
pub(super) const MARKER_ROW_H: f32 = 22.0;

/// Analysis indices whose strips are on screen right now.
///
/// A marker on a closed or un-maximized strip has nothing to point at, so
/// it must not hold the readout strip open.
pub(super) fn on_screen_strips(state: &AppState) -> Vec<AnalysisPresentationKey> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let results = &state.ui.results;
    let present = |key: AnalysisPresentationKey| key.resolve(run).is_some();
    match results.maximized_strip {
        Some(max_key) if present(max_key) => vec![max_key],
        _ => run
            .analyses
            .iter()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
            .filter(|key| !results.hidden_strips.contains(key))
            .collect(),
    }
}

/// Markers the strip will list, in placement order.
pub(super) fn visible_markers(state: &AppState) -> Vec<&ResultMarker> {
    let strips = on_screen_strips(state);
    state
        .ui
        .results
        .markers
        .iter()
        .filter(|marker| strips.contains(&marker.analysis))
        .collect()
}

/// Exact height the readout strip needs, or zero when it stands down.
///
/// The dock is content-fit up to a 212 px body, then its one body viewport
/// scrolls. Expanded cursor/marker content, marker-only, collapsed-header,
/// and no-strip states remain distinct.
fn cursor_target_available(state: &AppState) -> bool {
    state.ui.results.cursor_readout_active()
        && state
            .ui
            .results
            .cursor_strip
            .and_then(|index| {
                state
                    .simulation
                    .active_run()
                    .and_then(|run| run.analyses.get(index))
            })
            .is_some()
}

fn cursor_body_height(state: &AppState) -> f32 {
    if !cursor_target_available(state) {
        return 0.0;
    }
    // One column-header row, one X-domain row, then every visible trace row.
    (2 + readout_trace_count(state)) as f32 * READOUT_ROW_H
}

pub(super) fn marker_body_height(state: &AppState) -> f32 {
    let markers = visible_markers(state).len();
    if markers > 0 {
        markers as f32 * MARKER_ROW_H
    } else if cursor_target_available(state) {
        MARKER_EMPTY_H
    } else {
        0.0
    }
}

fn readout_body_content_height(state: &AppState) -> f32 {
    cursor_body_height(state).max(marker_body_height(state))
}

pub(super) fn readout_columns_side_by_side(width: f32, cursor: bool, markers: bool) -> bool {
    cursor && markers && width >= READOUT_DESKTOP_SPLIT_MIN_W
}

pub fn readout_strip_height(state: &AppState) -> f32 {
    let cursor = state.ui.results.cursor_readout_active();
    let markers = !visible_markers(state).is_empty();
    if !cursor && !markers {
        return 0.0;
    }
    if state.ui.results.readout_collapsed {
        return READOUT_HEADER_H;
    }
    (READOUT_HEADER_H + readout_body_content_height(state)).min(READOUT_MAX_H)
}

/// The cursor readout: one X row naming A, B and Δ, then the value each
/// visible trace takes at those cursors.
///
/// This is the single home for the cursor readout. The inspector reports
/// window statistics the strip does not carry, and never repeats these
/// numbers one panel away.
pub fn readout_strip(ui: &mut Ui, state: &mut AppState, height: f32) {
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
    if state.ui.results.readout_collapsed || rect.bottom() <= header.bottom() {
        return;
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
        .show(&mut body_ui, |ui| readout_body(ui, state));
}

fn readout_header(ui: &mut Ui, state: &mut AppState, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let cursor = state.ui.results.cursor_readout_active();
    let trace_count = if cursor {
        readout_trace_count(state)
    } else {
        0
    };
    let marker_count = visible_markers(state).len();
    let title = if cursor {
        "Cursors & markers"
    } else {
        "Markers"
    };
    let count = if cursor {
        format!(
            "{trace_count} trace{} · {marker_count} marker{}",
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
        let collapsed = state.ui.results.readout_collapsed;
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
            state.ui.results.readout_collapsed = !collapsed;
        }
    });
}

fn readout_body(ui: &mut Ui, state: &mut AppState) {
    let cursor = cursor_target_available(state);
    // Cursor mode keeps an explicit markers panel even when it is empty.
    let markers = !visible_markers(state).is_empty() || cursor;
    let width = ui.available_width();
    if readout_columns_side_by_side(width, cursor, markers) {
        let marker_width = (width * 0.42).clamp(240.0, 720.0).min(width);
        let cursor_width = (width - marker_width - READOUT_COLUMN_SEAM).max(0.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(
                egui::vec2(cursor_width, cursor_body_height(state)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| cursor_readout_section(ui, state),
            );
            let (seam, _) = ui.allocate_exact_size(
                egui::vec2(READOUT_COLUMN_SEAM, readout_body_content_height(state)),
                egui::Sense::hover(),
            );
            ui.painter()
                .rect_filled(seam, 0.0, Tokens::get(ui.ctx()).color.border);
            ui.allocate_ui_with_layout(
                egui::vec2(marker_width, marker_body_height(state)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| marker_section(ui, state),
            );
        });
    } else {
        if cursor {
            ui.allocate_ui_with_layout(
                egui::vec2(width, cursor_body_height(state)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| cursor_readout_section(ui, state),
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
                egui::vec2(width, marker_body_height(state)),
                egui::Layout::top_down(egui::Align::Min),
                |ui| marker_section(ui, state),
            );
        }
    }
}

/// The instrument bar's inline `A · B · Δ`, for the collapsed readout.
///
/// Collapsing the strip hides the register that owns these three numbers, so
/// the bar states them until it is expanded again — never both at once, which
/// is the duplication the results de-duplication pass removed.
pub(crate) fn inline_cursor_readout(state: &mut AppState, t: &Tokens) -> Option<String> {
    if !state.ui.results.readout_collapsed || !state.ui.results.cursor_readout_active() {
        return None;
    }
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let model = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;
    let cursors = state.ui.results.cursors;
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
pub(super) fn cursor_readout_section(ui: &mut Ui, state: &mut AppState) {
    let table_width = ui.available_width().max(CURSOR_TABLE_MIN_W);
    let table_height = cursor_body_height(state);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.cursor-readout-horizontal")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(table_width, table_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_min_height(table_height);
                    cursor_readout_table(ui, state);
                },
            );
        });
}

fn cursor_readout_table(ui: &mut Ui, state: &mut AppState) {
    let rect = ui.available_rect_before_wrap();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    if !state.ui.results.cursor_readout_active() {
        return;
    }
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let Some(model) = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))
    else {
        return;
    };
    let cursors = state.ui.results.cursors;
    let Some(a) = cursors.a else {
        return;
    };

    // Fixed columns keep focus/hover/value-length changes from shifting
    // neighboring fields. At compact widths the table scrolls horizontally.
    let name_column = (rect.width() * 0.24).clamp(150.0, 220.0);
    let value_column = ((rect.width() - name_column - READOUT_PAD_X * 2.0) / 4.0).max(108.0);
    let columns = [
        rect.left() + READOUT_PAD_X,
        rect.left() + READOUT_PAD_X + name_column,
        rect.left() + READOUT_PAD_X + name_column + value_column,
        rect.left() + READOUT_PAD_X + name_column + value_column * 2.0,
        rect.left() + READOUT_PAD_X + name_column + value_column * 3.0,
    ];
    // A trace row leads with its own line swatch, so a reader ties the row to
    // a curve without matching names; the header and X rows keep the bare
    // column so the table still reads as one grid.
    let draw_row = |row_index: usize,
                    values: [&str; 5],
                    colors: [egui::Color32; 5],
                    font: egui::FontId,
                    swatch: Option<egui::Color32>| {
        let top = rect.top() + row_index as f32 * READOUT_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + READOUT_ROW_H),
        );
        let painter = ui.painter().with_clip_rect(row);
        if let Some(color) = swatch {
            painter.hline(
                egui::Rangef::new(columns[0], columns[0] + READOUT_SWATCH_W - 6.0),
                row.center().y,
                egui::Stroke::new(2.0, color),
            );
        }
        for (index, ((text, x), color)) in values.into_iter().zip(columns).zip(colors).enumerate() {
            let x = if index == 0 && swatch.is_some() {
                x + READOUT_SWATCH_W
            } else {
                x
            };
            painter.text(
                egui::pos2(x, row.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                font.clone(),
                color,
            );
        }
    };

    draw_row(
        0,
        ["TRACE", "A", "B", "\u{0394}", "SLOPE"],
        [c.text_faint; 5],
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        None,
    );
    let a_x = model.format_x(a, significant_digits, quantity_policy);
    let b_x = cursors
        .b
        .map(|b| model.format_x(b, significant_digits, quantity_policy))
        .unwrap_or_else(|| "Place cursor B".to_owned());
    let delta_x = cursors
        .b
        .map(|b| x_separation(model, a, b, significant_digits, quantity_policy))
        .unwrap_or_default();
    draw_row(
        1,
        [model.x_label(), &a_x, &b_x, &delta_x, ""],
        [c.text_dim, c.accent, c.traces[4], c.text, c.text_faint],
        theme::mono(tokens::FS_0, FontWeight::Regular),
        None,
    );

    // Per-trace values at A and B, their difference, and their own slope.
    let rows = value_rows(model, a, presentation, quantity_policy);
    let row_colors = readout_traces(model)
        .map(|trace| trace.color)
        .collect::<Vec<_>>();
    let b_rows = cursors
        .b
        .map(|b| value_rows(model, b, presentation, quantity_policy));
    let deltas = cursors
        .b
        .map(|b| delta_values(model, a, b, presentation, quantity_policy));
    let slopes = cursors.b.map(|b| slope_values(model, a, b, presentation));
    for (index, (name, a_value)) in rows.iter().enumerate() {
        let b_value = b_rows
            .as_ref()
            .and_then(|values| values.get(index))
            .map_or("", |(_, value)| value.as_str());
        let delta = deltas
            .as_ref()
            .and_then(|values| values.get(index))
            .map_or("", String::as_str);
        let slope = slopes
            .as_ref()
            .and_then(|values| values.get(index))
            .map_or("", String::as_str);
        draw_row(
            index + 2,
            [name, a_value, b_value, delta, slope],
            [c.text_dim, c.text, c.text, c.text_dim, c.text_dim],
            theme::mono(tokens::FS_0, FontWeight::Regular),
            row_colors.get(index).copied(),
        );
    }
}

/// The marker half of the strip: one editable row per marker.
///
/// Markers are document content, so their row is the place they are named,
/// re-kinded and removed — there is no second marker list elsewhere to
/// disagree with this one.
pub(super) fn marker_section(ui: &mut Ui, state: &mut AppState) {
    let rect = ui.available_rect_before_wrap();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );

    let shown: Vec<u32> = visible_markers(state)
        .into_iter()
        .map(|marker| marker.id)
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

    let mut remove: Option<u32> = None;
    let mut edit: Option<u32> = None;
    for (index, id) in shown.iter().copied().enumerate() {
        let top = rect.top() + index as f32 * MARKER_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left() + READOUT_PAD_X, top),
            egui::pos2(rect.right() - READOUT_PAD_X, top + MARKER_ROW_H),
        );
        // Everything the row reports is derived here, from the same model
        // the plot drew, so a row can never describe a marker the plot
        // placed somewhere else.
        let Some(marker) = state.ui.results.markers.iter().find(|m| m.id == id) else {
            continue;
        };
        let kind = marker.kind;
        let anchor = marker.anchor.clone();
        let marker_x = marker.x;
        let analysis_key = marker.analysis;
        let trace_name = marker.trace_name.clone();
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
                    let sampled = sample_at_with(&trace.x, &trace.y, marker_x, interpolation);
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
        row_ui.label(
            egui::RichText::new(format!("M{id}"))
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        );
        // The kind is stated, not cycled: a click that silently reclassifies
        // what a marker asserts is a decision made by accident.
        row_ui.label(
            egui::RichText::new(kind.label())
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        );
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
            edit = Some(id);
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
            .on_hover_text("Remove this marker")
            .clicked()
        {
            remove = Some(id);
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
        let note = state
            .ui
            .results
            .markers
            .iter()
            .find(|m| m.id == id)
            .map_or_else(String::new, |m| m.note.clone());
        if note.is_empty() {
            row_ui.label(
                egui::RichText::new("no label")
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        } else {
            row_ui
                .label(
                    egui::RichText::new(&note)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_dim),
                )
                .on_hover_text(&note);
        }
    }
    if let Some(id) = remove {
        state.ui.results.remove_marker(id);
    }
    if let Some(id) = edit {
        super::marker_dialog::open(state, id);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Window statistics over the cursor span.
///
/// The A/B/Δ readout itself lives in the stage's readout strip; repeating it
/// one panel away is what the results de-duplication pass removed.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let cursor_model = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|m| m.analysis_index == index));

    // Statistics over the cursor window (or the full range).
    let cursors = state.ui.results.cursors;
    let measured_model = cursor_model.or_else(|| models.first());
    if let Some(model) = measured_model {
        let window = match (cursors.a, cursors.b) {
            (Some(a), Some(b)) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        let title = if window.is_some() {
            "Measurements · A–B"
        } else {
            "Measurements"
        };
        section_header(ui, title, None);
        measurement_rows(
            ui,
            &mut state.ui.results.derived,
            model,
            window,
            significant_digits,
            quantity_policy,
        );
    }
}

/// The traces the readout lists, in row order.
///
/// Every per-row producer reads this one predicate, so the value, delta,
/// slope and swatch columns cannot fall out of step with each other.
pub(super) fn readout_traces(model: &StripModel) -> impl Iterator<Item = &StripTrace> {
    model.traces.iter().filter(|trace| trace.visible)
}

pub(super) fn value_rows(
    model: &StripModel,
    x: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    readout_traces(model)
        .map(|trace| {
            let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
            (
                trace.name.clone(),
                model.format_trace_value(trace, value, significant_digits, quantity_policy),
            )
        })
        .collect()
}

/// Per-trace change between the cursors, in the same order and length as
/// [`value_rows`] so the readout strip's columns stay aligned row for row.
///
/// The difference is taken in value space, never between two formatted
/// readouts, so a Δ can never disagree with the values above it.
pub(super) fn delta_values(
    model: &StripModel,
    a: f64,
    b: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<String> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    readout_traces(model)
        .map(|trace| {
            let dv = sample_at_with(&trace.x, &trace.y, b, interpolation)
                - sample_at_with(&trace.x, &trace.y, a, interpolation);
            model.format_trace_value(trace, dv, significant_digits, quantity_policy)
        })
        .collect()
}

/// Per-trace slope between the cursors. Logarithmic X domains report change
/// per decade; linear domains report change per displayed X unit.
pub(super) fn slope_values(
    model: &StripModel,
    a: f64,
    b: f64,
    presentation: ResultPresentationPolicy,
) -> Vec<String> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let denominator = match model.x_scale {
        XScale::Log10 if a > 0.0 && b > 0.0 => b.log10() - a.log10(),
        XScale::Log10 => f64::NAN,
        XScale::Linear => b - a,
    };
    readout_traces(model)
        .map(|trace| {
            if !denominator.is_finite() || denominator.abs() <= 1e-12 {
                return "—".to_owned();
            }
            let delta = sample_at_with(&trace.x, &trace.y, b, interpolation)
                - sample_at_with(&trace.x, &trace.y, a, interpolation);
            let slope = delta / denominator;
            if !slope.is_finite() {
                return "—".to_owned();
            }
            // A trace names its slope in its own unit, never the strip's: a
            // current sharing a sheet with voltages must not report mA/ms of
            // rise as volts. `trace_unit` is the one owner of that mapping.
            let y_unit = model.trace_unit(trace);
            let x_unit = match model.x_scale {
                XScale::Log10 => "dec",
                XScale::Linear if model.x_unit.is_empty() => "x",
                XScale::Linear => model.x_unit.as_str(),
            };
            let suffix = if y_unit.is_empty() {
                format!(" /{x_unit}")
            } else {
                format!(" {y_unit}/{x_unit}")
            };
            fmt_significant(slope, significant_digits, &suffix)
        })
        .collect()
}

/// The separation between the cursors, named the way the X axis reads: a
/// time span also reports its reciprocal, because 1/Δt is the number a
/// designer is actually after when measuring a period.
pub(super) fn x_separation(
    model: &StripModel,
    a: f64,
    b: f64,
    significant_digits: usize,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> String {
    let dx = b - a;
    match model.x_scale {
        XScale::Linear if model.x_unit == "s" => {
            let span = fmt_si_significant(dx, "s", significant_digits);
            if dx == 0.0 {
                span
            } else {
                format!(
                    "{span}  ({})",
                    quantity_policy.format_frequency(1.0 / dx.abs(), significant_digits)
                )
            }
        }
        XScale::Log10 => quantity_policy.format_frequency(dx, significant_digits),
        _ => fmt_si_significant(dx, &model.x_unit, significant_digits),
    }
}

/// min / max / rms rows per visible trace, optionally windowed to [a, b].
/// The single-pass stats are cached per (trace, window, data version), so
/// no samples are rescanned until the cursors or the data move.
pub(super) fn measurement_rows(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) {
    use crate::analysis::measurements as basic;

    // Window identity for the cache key; u64::MAX is a NaN bit pattern no
    // finite cursor can produce, marking the full-range case.
    let (a_bits, b_bits) = match window {
        Some((a, b)) => (a.to_bits(), b.to_bits()),
        None => (u64::MAX, u64::MAX),
    };

    let mut rows: Vec<(String, String)> = Vec::new();
    for trace in model.traces.iter().filter(|t| t.visible).take(4) {
        let key = (trace_key(model, trace), a_bits, b_bits);
        let stats = derived.stats_or(key, || {
            let (start, end) = match window {
                Some((a, b)) => {
                    let start = trace.x.partition_point(|&v| v < a);
                    let end = trace.x.partition_point(|&v| v <= b);
                    (start, end.max(start))
                }
                None => (0, trace.y.len()),
            };
            basic::calculate_min_max_rms(&trace.y[start..end])
        });
        let Some((min, max, rms)) = stats else {
            continue;
        };
        // One formatter for every value the instrument reports, so a
        // measurement and the cursor readout above it cannot disagree about
        // a trace's unit.
        let fmt = |v: f64| -> String {
            model.format_trace_value(trace, v, significant_digits, quantity_policy)
        };
        rows.push((format!("{} min", trace.name), fmt(min)));
        rows.push((format!("{} max", trace.name), fmt(max)));
        if matches!(
            trace.kind,
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary
        ) {
            rows.push((format!("{} rms", trace.name), fmt(rms)));
        }
    }
    let refs: Vec<(&str, &str)> = rows.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    crate::ui::widgets::measurement_table(ui, &refs);
}
