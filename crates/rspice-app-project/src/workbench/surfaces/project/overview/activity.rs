//! The Overview's Recent operations card, and the words its rows are made of:
//! run titles, and how long ago each operation happened.

use egui::{Sense, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::page::paint_elided;
use super::snapshot::OverviewSnapshot;
use super::{OverviewIntent, Tone, overview_card, paint_tone_mark};

const ACTIVITY_ROW_HEIGHT: f32 = 30.0;

/// Describe the operations register without letting its own limit pass for the
/// project's history: five newest rows out of thirty is never "5 retained".
pub(super) fn operations_meta(shown: usize, total: usize) -> String {
    if total > shown {
        format!("{shown} of {total} retained")
    } else {
        format!("{total} retained")
    }
}

pub(super) fn operations_card(ui: &mut Ui, snapshot: &OverviewSnapshot) -> Option<OverviewIntent> {
    overview_card(
        ui,
        "Recent operations",
        &operations_meta(snapshot.operations.len(), snapshot.operation_total),
        |ui| operations_register(ui, snapshot),
    )
}

fn operations_register(ui: &mut Ui, snapshot: &OverviewSnapshot) -> Option<OverviewIntent> {
    ui.add_space(7.0);
    let mut selected = None;
    if snapshot.operations.is_empty() {
        activity_row(
            ui,
            "now",
            "No retained project operations",
            "Validated revisions and simulation runs will appear here",
            Tone::Neutral,
            false,
        );
        ui.add_space(7.0);
        activity_row(
            ui,
            "rev",
            &format!("Working revision {}", snapshot.project_revision),
            if snapshot.dirty_documents == 0 {
                "No modified open documents"
            } else {
                "Working changes are not yet saved"
            },
            if snapshot.dirty_documents == 0 {
                Tone::Neutral
            } else {
                Tone::Warn
            },
            false,
        );
    } else {
        for (index, operation) in snapshot.operations.iter().enumerate() {
            let response = activity_row(
                ui,
                &operation.when,
                &operation.event,
                &operation.detail,
                operation.tone,
                operation.intent.is_some(),
            );
            if response.clicked() {
                selected.clone_from(&operation.intent);
            }
            if index + 1 < snapshot.operations.len() {
                ui.add_space(7.0);
            }
        }
    }
    selected
}

fn activity_row(
    ui: &mut Ui,
    time: &str,
    event: &str,
    detail: &str,
    tone: Tone,
    interactive: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, ACTIVITY_ROW_HEIGHT),
        if interactive {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if interactive && (response.hovered() || response.has_focus()) {
        ui.painter()
            .rect_filled(rect.shrink2(vec2(8.0, 1.0)), 4.0, t.color.bg_panel_2);
    }
    let time_width = 54.0_f32.min(width * 0.22);
    paint_elided(
        ui,
        pos2(rect.left() + 14.0, rect.top() + 3.0),
        time,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        (time_width - 14.0).max(1.0),
    );
    // A run that failed carries the same words as one that succeeded until the
    // reader gets to "with failures" at the end of the line, so it takes a
    // mark in front of it — the same rule the design register applies to its
    // detail column.
    let event_left = rect.left() + time_width + 14.0;
    let mark = paint_tone_mark(ui, tone, event_left, rect.top() + 2.0 + tokens::FS_0 * 0.5);
    paint_elided(
        ui,
        pos2(event_left + mark, rect.top() + 2.0),
        event,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text,
        (width - time_width - 24.0 - mark).max(1.0),
    );
    paint_elided(
        ui,
        pos2(rect.left() + time_width + 14.0, rect.top() + 16.0),
        detail,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        (width - time_width - 24.0).max(1.0),
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    // Painted columns again, so the announced label is assembled here.
    let row_label = format!("{time}, {event}, {detail}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            row_label.clone(),
        )
    });
    response.on_hover_text(format!("{event}\n{detail}"))
}

pub(super) fn run_title(sequence: u64, label: &str) -> String {
    let prefix = format!("Run {sequence}");
    if label.starts_with(&prefix) {
        label.to_owned()
    } else {
        format!("{prefix} · {label}")
    }
}

pub(super) fn unix_seconds_to_millis(seconds: f64) -> Option<u64> {
    if !seconds.is_finite() || seconds < 0.0 || seconds > u64::MAX as f64 / 1_000.0 {
        return None;
    }
    Some((seconds * 1_000.0).round() as u64)
}

pub(super) fn relative_time(timestamp_ms: u64, now_ms: u64) -> String {
    if timestamp_ms > now_ms {
        return "time n/a".to_owned();
    }
    match now_ms - timestamp_ms {
        0..=59_999 => "now".to_owned(),
        60_000..=3_599_999 => format!("{} min", (now_ms - timestamp_ms) / 60_000),
        3_600_000..=86_399_999 => format!("{} h", (now_ms - timestamp_ms) / 3_600_000),
        _ => format!("{} d", (now_ms - timestamp_ms) / 86_400_000),
    }
}
