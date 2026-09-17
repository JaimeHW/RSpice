//! The envelope-following form: the carriers the envelope rides on, the slow
//! window it is walked over, and how the periodic solve it starts from is
//! obtained.
//!
//! Every time field here carries its unit inside the control rather than in
//! the caption, because an envelope step and a carrier period differ by orders
//! of magnitude and a bare number beside a bare number is the one place that
//! has been misread.

use egui::{Align, Layout, Response, Ui, vec2};

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::EnvelopeDialogState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::{
    QuantityPresentationPolicy, UiNumberLocale, field_cell, input_row, inspector_input_row,
    mono_input, mono_input_with_suffix, named_periodic_source_row,
    normalize_quantity_on_focus_loss, property_row, select_mono_with_response,
    uses_two_column_fields,
};

pub(super) const ENVELOPE_FIELD_LABELS: [&str; 8] = [
    "Carrier tones",
    "Envelope stop",
    "Envelope step",
    "Harmonic order",
    "Modulation sources",
    "Initial periodic solve",
    "Output schedule",
    "Extraction path",
];

pub(super) const ENVELOPE_INITIAL_SOLVE_CHOICES: &[&str] =
    &["HB", "PSS", "Transient spectral estimate"];

pub(super) const ENVELOPE_ADAPTIVE_CHOICES: &[&str] = &[
    "Adaptive solver samples",
    "Fixed envelope step",
    "Event-aligned only",
];

pub(super) const ENVELOPE_EXTRACTION_PATH: &str = "Least-squares projection";

pub(super) const ENVELOPE_HARMONIC_ORDER_HELPER: &str = "positive integer";

/// A time field whose unit is painted inside the control.
pub(super) fn envelope_time_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Response {
    let response = if uses_two_column_fields(ui) {
        field_cell(ui, label, Some("engineering notation"), |ui| {
            mono_input_with_suffix(ui, label, value, "s")
        })
    } else {
        let t = Tokens::get(ui.ctx());
        let row_h = t.metrics.row_h;
        let color = t.color.text_dim;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), row_h),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (label_rect, _) =
                    ui.allocate_exact_size(vec2(96.0, row_h), egui::Sense::hover());
                ui.painter().text(
                    label_rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    label,
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    color,
                );
                mono_input_with_suffix(ui, label, value, "s")
            },
        )
        .inner
    };
    normalize_quantity_on_focus_loss(&response, value, QuantityInputKind::Time, policy, locale);
    response
}

/// The harmonic-order field, with the domain it accepts beside its caption.
pub(super) fn envelope_harmonic_order_row(ui: &mut Ui, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, ENVELOPE_FIELD_LABELS[3], value);
    }
    field_cell(
        ui,
        ENVELOPE_FIELD_LABELS[3],
        Some(ENVELOPE_HARMONIC_ORDER_HELPER),
        |ui| mono_input(ui, ENVELOPE_FIELD_LABELS[3], value, ui.available_width()),
    )
}

/// A mono choice row, in the grid or stacked under its own label.
pub(super) fn envelope_choice_row(
    ui: &mut Ui,
    label: &str,
    options: &[&str],
    value: &mut usize,
) -> bool {
    let mut add_control = |ui: &mut Ui| {
        let options = options
            .iter()
            .map(|option| (*option).to_owned())
            .collect::<Vec<_>>();
        let current = options
            .get(*value)
            .map_or("Schema unavailable", String::as_str);
        let salt = format!("analysis-envelope-field-{}-{label}", ui.id().value());
        if let Some(index) =
            select_mono_with_response(ui, &salt, label, current, &options, ui.available_width())
                .picked
        {
            *value = index;
            true
        } else {
            false
        }
    };

    if uses_two_column_fields(ui) {
        return field_cell(ui, label, Some("domain constrained"), add_control);
    }

    let t = Tokens::get(ui.ctx());
    let row_h = t.metrics.row_h;
    let color = t.color.text_dim;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) = ui.allocate_exact_size(vec2(96.0, row_h), egui::Sense::hover());
            ui.painter().text(
                label_rect.left_center(),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                color,
            );
            add_control(ui)
        },
    )
    .inner
}

/// The modulation-source row: the design's periodic sources, or a declared list.
pub(super) fn envelope_modulation_source_row(
    ui: &mut Ui,
    value: &mut String,
    circuit_sources: &[String],
) {
    named_periodic_source_row(
        ui,
        ENVELOPE_FIELD_LABELS[4],
        "envelope-modulation",
        value,
        circuit_sources,
    );
}

/// Render the envelope-following fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut EnvelopeDialogState,
    envelope_modulation_sources: &[String],
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    input_row(ui, ENVELOPE_FIELD_LABELS[0], &mut setup.carrier_tones);
    envelope_time_input_row(
        ui,
        ENVELOPE_FIELD_LABELS[1],
        &mut setup.stop_time,
        policy,
        locale,
    );
    envelope_time_input_row(
        ui,
        ENVELOPE_FIELD_LABELS[2],
        &mut setup.envelope_step,
        policy,
        locale,
    );
    envelope_harmonic_order_row(ui, &mut setup.harmonic_order);
    envelope_modulation_source_row(
        ui,
        &mut setup.modulation_sources,
        envelope_modulation_sources,
    );
    envelope_choice_row(
        ui,
        ENVELOPE_FIELD_LABELS[5],
        ENVELOPE_INITIAL_SOLVE_CHOICES,
        &mut setup.initial_periodic_solve_idx,
    );
    envelope_choice_row(
        ui,
        ENVELOPE_FIELD_LABELS[6],
        ENVELOPE_ADAPTIVE_CHOICES,
        &mut setup.adaptive_mode_idx,
    );
    setup.extraction_path_idx = 0;
    property_row(ui, ENVELOPE_FIELD_LABELS[7], ENVELOPE_EXTRACTION_PATH);
}
