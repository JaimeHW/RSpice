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
/// Most trace rows the strip will show before it stops growing.
pub(super) const READOUT_TRACE_LIMIT: usize = 4;
pub(super) const READOUT_PAD_X: f32 = 10.0;

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
                .min(READOUT_TRACE_LIMIT)
        })
}

/// Height of one marker row.
pub(super) const MARKER_ROW_H: f32 = 22.0;
/// Most marker rows the strip will show before it stops growing.
pub(super) const MARKER_ROW_LIMIT: usize = 4;

/// Analysis indices whose strips are on screen right now.
///
/// A marker on a closed or un-maximized strip has nothing to point at, so
/// it must not hold the readout strip open.
pub(super) fn on_screen_strips(state: &AppState) -> Vec<usize> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let results = &state.ui.results;
    let present = |index: usize| index < run.analyses.len();
    match results.maximized_strip {
        Some(max_index) if present(max_index) => vec![max_index],
        _ => (0..run.analyses.len())
            .filter(|index| !results.hidden_strips.contains(index))
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
        .filter(|marker| strips.contains(&marker.analysis_index))
        .collect()
}

/// Exact height the readout strip needs, or zero when it stands down.
///
/// The strip is content-fit by design: it is a readout, not a dock, so it
/// never reserves space for rows it has nothing to put in. Three states are
/// reachable — the full cursor readout, a markers-only strip when markers
/// exist without cursors, and no strip at all.
pub fn readout_strip_height(state: &AppState) -> f32 {
    let mut height = 0.0;
    if state.ui.results.cursor_readout_active() {
        height += READOUT_HEADER_H + readout_trace_count(state) as f32 * READOUT_ROW_H;
    }
    let markers = visible_markers(state).len();
    if markers > 0 {
        height += READOUT_HEADER_H + markers.min(MARKER_ROW_LIMIT) as f32 * MARKER_ROW_H;
    }
    height
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

    let cursor_bottom = cursor_readout_section(ui, state, rect);
    let marker_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left(), cursor_bottom),
        egui::pos2(rect.right(), rect.bottom()),
    );
    if marker_rect.height() > 1.0 {
        if cursor_bottom > rect.top() {
            ui.painter().hline(
                rect.x_range(),
                cursor_bottom,
                egui::Stroke::new(1.0, c.border),
            );
        }
        marker_section(ui, state, marker_rect);
    }
}

/// The A/B half of the strip. Returns the Y the marker section starts at,
/// which equals the strip top when there is no cursor readout to draw.
pub(super) fn cursor_readout_section(ui: &mut Ui, state: &mut AppState, rect: egui::Rect) -> f32 {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    if !state.ui.results.cursor_readout_active() {
        return rect.top();
    }
    // Read the reserved height from the same source as `readout_strip_height`
    // so the marker section can never start off the end of the strip.
    let section_bottom =
        rect.top() + READOUT_HEADER_H + readout_trace_count(state) as f32 * READOUT_ROW_H;
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
        return rect.top();
    };
    let cursors = state.ui.results.cursors;
    let Some(a) = cursors.a else {
        return rect.top();
    };

    // Header: the X positions and their separation.
    let header = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.right(), rect.top() + READOUT_HEADER_H),
    );
    let painter = ui.painter().with_clip_rect(header);
    let mut x = header.left() + READOUT_PAD_X;
    let mut chip = |text: String, color: egui::Color32, painter: &egui::Painter| {
        let galley =
            painter.layout_no_wrap(text, theme::mono(tokens::FS_0, FontWeight::Regular), color);
        painter.galley(
            egui::pos2(x, header.center().y - galley.size().y * 0.5),
            galley.clone(),
            color,
        );
        x += galley.size().x + 14.0;
    };
    chip("CURSORS".to_owned(), c.text_faint, &painter);
    chip(
        format!(
            "A  {} = {}",
            model.x_label(),
            model.format_x(a, significant_digits, quantity_policy)
        ),
        c.accent,
        &painter,
    );
    if let Some(b) = cursors.b {
        chip(
            format!(
                "B  {} = {}",
                model.x_label(),
                model.format_x(b, significant_digits, quantity_policy)
            ),
            c.traces[4],
            &painter,
        );
        chip(
            format!(
                "Δ  {}",
                x_separation(model, a, b, significant_digits, quantity_policy)
            ),
            c.text,
            &painter,
        );
        if let Some(slope) = slope_between(model, a, b, presentation) {
            chip(format!("slope  {slope}"), c.text_dim, &painter);
        }
    } else {
        chip("click again to place B".to_owned(), c.text_faint, &painter);
    }

    // Per-trace values at A, at B, and their difference.
    let rows = value_rows(model, a, presentation, quantity_policy);
    let b_rows = cursors
        .b
        .map(|b| value_rows(model, b, presentation, quantity_policy));
    let deltas = cursors
        .b
        .map(|b| delta_values(model, a, b, presentation, quantity_policy));
    let name_column = (rect.width() * 0.28).clamp(80.0, 220.0);
    let value_column = ((rect.width() - name_column - READOUT_PAD_X * 2.0) / 3.0).max(1.0);
    for (index, (name, a_value)) in rows.iter().take(READOUT_TRACE_LIMIT).enumerate() {
        let top = header.bottom() + index as f32 * READOUT_ROW_H;
        let row = egui::Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + READOUT_ROW_H),
        );
        let painter = ui.painter().with_clip_rect(row);
        painter.text(
            egui::pos2(row.left() + READOUT_PAD_X, row.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        );
        let mut column = row.left() + READOUT_PAD_X + name_column;
        let mut cell = |text: &str, color: egui::Color32| {
            painter.text(
                egui::pos2(column, row.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                color,
            );
            column += value_column;
        };
        cell(a_value, c.text);
        if let Some(b_rows) = b_rows.as_ref() {
            cell(
                b_rows.get(index).map_or("", |(_, value)| value.as_str()),
                c.text,
            );
        }
        if let Some(deltas) = deltas.as_ref() {
            cell(deltas.get(index).map_or("", String::as_str), c.text_dim);
        }
    }
    section_bottom
}

/// The marker half of the strip: one editable row per marker.
///
/// Markers are document content, so their row is the place they are named,
/// re-kinded and removed — there is no second marker list elsewhere to
/// disagree with this one.
pub(super) fn marker_section(ui: &mut Ui, state: &mut AppState, rect: egui::Rect) {
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
        .take(MARKER_ROW_LIMIT)
        .map(|marker| marker.id)
        .collect();
    let total = visible_markers(state).len();
    if shown.is_empty() {
        return;
    }

    let header = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.right(), rect.top() + READOUT_HEADER_H),
    );
    let painter = ui.painter().with_clip_rect(header);
    painter.text(
        egui::pos2(header.left() + READOUT_PAD_X, header.center().y),
        egui::Align2::LEFT_CENTER,
        "MARKERS",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
    if total > shown.len() {
        painter.text(
            egui::pos2(header.right() - READOUT_PAD_X, header.center().y),
            egui::Align2::RIGHT_CENTER,
            format!("{} of {total}", shown.len()),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    let mut remove: Option<u32> = None;
    for (index, id) in shown.iter().copied().enumerate() {
        let top = header.bottom() + index as f32 * MARKER_ROW_H;
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
        let anchor = marker.anchor;
        let marker_x = marker.x;
        let analysis_index = marker.analysis_index;
        let trace_name = marker.trace_name.clone();
        let model = models
            .iter()
            .find(|model| model.analysis_index == analysis_index);
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
        if chip(&mut row_ui, kind.label(), false)
            .on_hover_text("Cycle marker kind: note → peak → spec")
            .clicked()
            && let Some(marker) = state.ui.results.marker_mut(id)
        {
            marker.kind = kind.next();
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
        // The note field takes what is left of the row.
        let note_width = row_ui.available_width().max(60.0);
        let mut note = state
            .ui
            .results
            .markers
            .iter()
            .find(|m| m.id == id)
            .map_or_else(String::new, |m| m.note.clone());
        let response = row_ui.add(
            egui::TextEdit::singleline(&mut note)
                .desired_width(note_width)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .hint_text("note…"),
        );
        if state.ui.results.editing_marker == Some(id) {
            response.request_focus();
            state.ui.results.editing_marker = None;
        }
        if response.changed()
            && let Some(marker) = state.ui.results.marker_mut(id)
        {
            marker.note = note;
        }
    }
    if let Some(id) = remove {
        state.ui.results.remove_marker(id);
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
        .and_then(|idx| models.iter().find(|m| m.analysis_index == idx));

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

pub(super) fn value_rows(
    model: &StripModel,
    x: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .take(6)
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
    model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .take(READOUT_TRACE_LIMIT)
        .map(|trace| {
            let dv = sample_at_with(&trace.x, &trace.y, b, interpolation)
                - sample_at_with(&trace.x, &trace.y, a, interpolation);
            model.format_trace_value(trace, dv, significant_digits, quantity_policy)
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

/// dB/decade slope of the magnitude trace between the cursors, on log-X
/// strips that carry one.
pub(super) fn slope_between(
    model: &StripModel,
    a: f64,
    b: f64,
    presentation: ResultPresentationPolicy,
) -> Option<String> {
    if model.x_scale != XScale::Log10 || a <= 0.0 || b <= 0.0 {
        return None;
    }
    let dlog = (b.log10() - a.log10()).abs();
    if dlog <= 1e-12 {
        return None;
    }
    let magnitude = model
        .traces
        .iter()
        .find(|trace| trace.kind == TraceKind::MagnitudeDb && trace.visible)?;
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let ddb = sample_at_with(&magnitude.x, &magnitude.y, b, interpolation)
        - sample_at_with(&magnitude.x, &magnitude.y, a, interpolation);
    Some(fmt_significant(
        ddb / dlog,
        usize::from(presentation.displayed_significant_digits().get()),
        " dB/dec",
    ))
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
        let fmt = |v: f64| -> String {
            match trace.kind {
                TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                    fmt_si_significant(v, model.y_unit, significant_digits)
                }
                TraceKind::MagnitudeDb => fmt_significant(v, significant_digits, " dB"),
                TraceKind::PhaseDeg => {
                    quantity_policy.format_angle(v.to_radians(), significant_digits)
                }
                TraceKind::PhaseRad => quantity_policy.format_angle(v, significant_digits),
            }
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
