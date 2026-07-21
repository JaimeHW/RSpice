//! Per-analysis configuration forms for the Simulate right panel.
//!
//! Each form edits one typed [`AnalysisDraft`] owned by a stable analysis
//! instance. The form returns a one-line note describing what the analysis
//! does; validation is rendered by the caller.

use egui::{Align, Layout, Rect, Response, Ui, UiBuilder, vec2};

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::simulation::plan::{
    AnalysisDraft, FrequencySweepDraft, NetworkPortDraft, PeriodicNetworkDraft,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, check_row as inspector_check_row, choice_row as inspector_choice_row,
    input_row as inspector_input_row, mono_input, select, select_mono_with_response,
};
use crate::workbench::design_system::property_row as inspector_property_row;

const SWEEP_KINDS: &[&str] = &["dec", "oct", "lin"];
const XF_FIELD_LABELS: [&str; 8] = [
    "Input source",
    "Output expression",
    "Solve point",
    "Transfer gain",
    "Input resistance",
    "Output resistance",
    "Normalize",
    "Accuracy",
];
const XF_SOLVE_POINT: &str = "DC operating point";
const XF_ENABLED_CHOICES: &[&str] = &["Enabled", "Disabled"];
const XF_NORMALIZATION_CHOICES: &[&str] = &["Disabled", "Relative to nominal", "Per source unit"];
const XF_ACCURACY_CHOICES: &[&str] = &["Fast", "Balanced", "Accurate", "Robust"];
const ENVELOPE_FIELD_LABELS: [&str; 8] = [
    "Carrier tones",
    "Envelope stop",
    "Envelope step",
    "Harmonic order",
    "Modulation sources",
    "Initial periodic solve",
    "Output schedule",
    "Extraction path",
];
const ENVELOPE_INITIAL_SOLVE_CHOICES: &[&str] = &["HB", "PSS", "Transient spectral estimate"];
const ENVELOPE_ADAPTIVE_CHOICES: &[&str] = &[
    "Adaptive solver samples",
    "Fixed envelope step",
    "Event-aligned only",
];
const ENVELOPE_DECLARED_SOURCES_CHOICE: &str = "Declared list...";
const ENVELOPE_EXTRACTION_PATH: &str = "Preview";
const ENVELOPE_HARMONIC_ORDER_HELPER: &str = "positive integer";
const ENVELOPE_INLINE_CONTROL_GAP: f32 = 6.0;
const FIELD_COLUMN_GAP: f32 = 14.0;
const FIELD_ROW_GAP: f32 = 10.0;
const FIELD_LABEL_HEIGHT: f32 = 15.0;

#[derive(Clone, Copy)]
struct PendingCell(Rect);

impl Default for PendingCell {
    fn default() -> Self {
        Self(Rect::NOTHING)
    }
}

fn pending_cell_id(ui: &Ui) -> egui::Id {
    ui.id().with("analysis-form.pending-cell")
}

fn clear_pending_cell(ui: &mut Ui) {
    let id = pending_cell_id(ui);
    ui.data_mut(|data| {
        data.remove_temp::<PendingCell>(id);
    });
}

fn uses_two_column_fields(ui: &Ui) -> bool {
    ui.available_width() >= 420.0
}

fn next_field_cell(ui: &mut Ui) -> Rect {
    let id = pending_cell_id(ui);
    if let Some(PendingCell(rect)) = ui.data_mut(|data| data.remove_temp::<PendingCell>(id)) {
        return rect;
    }
    let t = Tokens::get(ui.ctx());
    let row_height = FIELD_LABEL_HEIGHT + 5.0 + t.metrics.ctl_h;
    let (row, _) =
        ui.allocate_exact_size(vec2(ui.available_width(), row_height), egui::Sense::hover());
    let cell_width = ((row.width() - FIELD_COLUMN_GAP) * 0.5).max(1.0);
    let left = Rect::from_min_size(row.min, vec2(cell_width, row.height()));
    let right = Rect::from_min_max(
        egui::pos2(left.right() + FIELD_COLUMN_GAP, row.top()),
        row.max,
    );
    ui.data_mut(|data| data.insert_temp(id, PendingCell(right)));
    left
}

fn field_cell<R>(
    ui: &mut Ui,
    label: &str,
    helper: Option<&str>,
    add_control: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let rect = next_field_cell(ui);
    let mut cell = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
    );
    cell.set_clip_rect(rect.intersect(ui.clip_rect()));
    cell.spacing_mut().item_spacing.y = 5.0;
    let (label_rect, _) = cell.allocate_exact_size(
        vec2(cell.available_width(), FIELD_LABEL_HEIGHT),
        egui::Sense::hover(),
    );
    cell.painter().text(
        label_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    if let Some(helper) = helper {
        cell.painter().text(
            label_rect.right_center(),
            egui::Align2::RIGHT_CENTER,
            helper,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    add_control(&mut cell)
}

fn input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, None, |ui| {
        mono_input(ui, value, ui.available_width())
    })
}

fn input_row_enabled(ui: &mut Ui, label: &str, value: &mut String, enabled: bool) -> Response {
    if !uses_two_column_fields(ui) {
        return ui
            .add_enabled_ui(enabled, |ui| inspector_input_row(ui, label, value))
            .inner;
    }
    field_cell(ui, label, None, |ui| {
        ui.add_enabled_ui(enabled, |ui| mono_input(ui, value, ui.available_width()))
            .inner
    })
}

fn engineering_input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, Some("engineering notation"), |ui| {
        mono_input(ui, value, ui.available_width())
    })
}

fn engineering_input_row_enabled(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    enabled: bool,
) -> Response {
    if !uses_two_column_fields(ui) {
        return ui
            .add_enabled_ui(enabled, |ui| inspector_input_row(ui, label, value))
            .inner;
    }
    field_cell(ui, label, Some("engineering notation"), |ui| {
        ui.add_enabled_ui(enabled, |ui| mono_input(ui, value, ui.available_width()))
            .inner
    })
}

fn quantity_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Response {
    let response = engineering_input_row(ui, label, value);
    if response.lost_focus()
        && let Ok(parsed) = parse_ui_quantity(value, kind, policy, locale)
    {
        let schema_value = if kind == QuantityInputKind::Temperature {
            parsed - 273.15
        } else {
            parsed
        };
        *value = format!("{schema_value:.17e}");
    }
    response
}

fn quantity_input_row_enabled(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
    enabled: bool,
) -> Response {
    let response = engineering_input_row_enabled(ui, label, value, enabled);
    if enabled
        && response.lost_focus()
        && let Ok(parsed) = parse_ui_quantity(value, kind, policy, locale)
    {
        let schema_value = if kind == QuantityInputKind::Temperature {
            parsed - 273.15
        } else {
            parsed
        };
        *value = format!("{schema_value:.17e}");
    }
    response
}

fn choice_row(ui: &mut Ui, label: &str, options: &[&str], value: &mut usize) -> bool {
    if !uses_two_column_fields(ui) {
        return inspector_choice_row(ui, label, options, value);
    }
    field_cell(ui, label, Some("domain constrained"), |ui| {
        let options = options
            .iter()
            .map(|option| (*option).to_owned())
            .collect::<Vec<_>>();
        let current = options
            .get(*value)
            .map_or("Schema unavailable", String::as_str);
        let salt = format!("analysis-field-{}-{label}", ui.id().value());
        if let Some(index) = select(ui, &salt, label, current, &options, ui.available_width()) {
            *value = index;
            true
        } else {
            false
        }
    })
}

fn enabled_choice_row(ui: &mut Ui, label: &str, enabled: &mut bool) -> bool {
    let mut selected = usize::from(!*enabled);
    let changed = choice_row(ui, label, XF_ENABLED_CHOICES, &mut selected);
    if changed {
        *enabled = selected == 0;
    }
    changed
}

fn mono_input_with_suffix(ui: &mut Ui, value: &mut String, suffix: &'static str) -> Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let suffix_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let suffix_width = ui
        .painter()
        .layout_no_wrap(suffix.to_owned(), suffix_font.clone(), t.color.text_dim)
        .size()
        .x;
    ui.allocate_ui_with_layout(
        vec2(width, t.metrics.ctl_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = ENVELOPE_INLINE_CONTROL_GAP;
            let input_width = (width - suffix_width - ENVELOPE_INLINE_CONTROL_GAP).max(1.0);
            let response = mono_input(ui, value, input_width);
            let (suffix_rect, _) =
                ui.allocate_exact_size(vec2(suffix_width, t.metrics.ctl_h), egui::Sense::hover());
            ui.painter().text(
                suffix_rect.center(),
                egui::Align2::CENTER_CENTER,
                suffix,
                suffix_font,
                t.color.text_dim,
            );
            response
        },
    )
    .inner
}

fn normalize_quantity_on_focus_loss(
    response: &Response,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    if response.lost_focus()
        && let Ok(parsed) = parse_ui_quantity(value, kind, policy, locale)
    {
        let schema_value = if kind == QuantityInputKind::Temperature {
            parsed - 273.15
        } else {
            parsed
        };
        *value = format!("{schema_value:.17e}");
    }
}

fn envelope_time_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Response {
    let response = if uses_two_column_fields(ui) {
        field_cell(ui, label, Some("engineering notation"), |ui| {
            mono_input_with_suffix(ui, value, "s")
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
                mono_input_with_suffix(ui, value, "s")
            },
        )
        .inner
    };
    normalize_quantity_on_focus_loss(&response, value, QuantityInputKind::Time, policy, locale);
    response
}

fn envelope_harmonic_order_row(ui: &mut Ui, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, ENVELOPE_FIELD_LABELS[3], value);
    }
    field_cell(
        ui,
        ENVELOPE_FIELD_LABELS[3],
        Some(ENVELOPE_HARMONIC_ORDER_HELPER),
        |ui| mono_input(ui, value, ui.available_width()),
    )
}

fn envelope_choice_row(ui: &mut Ui, label: &str, options: &[&str], value: &mut usize) -> bool {
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

fn envelope_modulation_control_widths(available_width: f32) -> (f32, f32) {
    let content_width = (available_width - ENVELOPE_INLINE_CONTROL_GAP).max(2.0);
    let selector_width = content_width * 0.58;
    (selector_width, content_width - selector_width)
}

fn envelope_modulation_source_row(ui: &mut Ui, value: &mut String, circuit_sources: &[String]) {
    let catalog_selection = circuit_sources
        .iter()
        .position(|source| source.eq_ignore_ascii_case(value.trim()));
    let declared_index = circuit_sources.len();
    let mut selected = catalog_selection.unwrap_or(declared_index);
    let mut add_control = |ui: &mut Ui| {
        let mut options = circuit_sources.to_vec();
        options.push(ENVELOPE_DECLARED_SOURCES_CHOICE.to_owned());
        let current = options
            .get(selected)
            .map_or(ENVELOPE_DECLARED_SOURCES_CHOICE, String::as_str);
        let width = ui.available_width();
        let (selector_width, editor_width) = envelope_modulation_control_widths(width);
        ui.allocate_ui_with_layout(
            vec2(width, Tokens::get(ui.ctx()).metrics.ctl_h),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = ENVELOPE_INLINE_CONTROL_GAP;
                let salt = format!("analysis-envelope-modulation-source-{}", ui.id().value());
                if let Some(index) = select_mono_with_response(
                    ui,
                    &salt,
                    "Modulation sources",
                    current,
                    &options,
                    selector_width,
                )
                .picked
                {
                    selected = index;
                    if let Some(source) = circuit_sources.get(index) {
                        *value = source.clone();
                    } else if catalog_selection.is_some() {
                        value.clear();
                    }
                }
                if selected == declared_index {
                    mono_input(ui, value, editor_width);
                } else {
                    ui.allocate_exact_size(vec2(editor_width, 1.0), egui::Sense::hover());
                }
            },
        );
    };

    if uses_two_column_fields(ui) {
        field_cell(
            ui,
            ENVELOPE_FIELD_LABELS[4],
            Some("domain constrained"),
            add_control,
        );
        return;
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
                ENVELOPE_FIELD_LABELS[4],
                theme::sans(tokens::FS_1, FontWeight::Regular),
                color,
            );
            add_control(ui);
        },
    );
}

fn check_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    if !uses_two_column_fields(ui) {
        return inspector_check_row(ui, label, value);
    }
    field_cell(ui, label, Some("domain constrained"), |ui| {
        let row_size = vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h);
        ui.allocate_ui_with_layout(row_size, Layout::left_to_right(Align::Center), |ui| {
            // The cell owns the full grid column, but the checkbox keeps its
            // natural compact width at the leading edge. `add_sized` would
            // center its contents across an oversized half-column.
            ui.add(egui::Checkbox::new(
                value,
                if *value { "Enabled" } else { "Disabled" },
            ))
            .changed()
        })
        .inner
    })
}

fn property_row(ui: &mut Ui, label: &str, value: &str) {
    if !uses_two_column_fields(ui) {
        inspector_property_row(ui, label, value);
        return;
    }
    let t = Tokens::get(ui.ctx());
    field_cell(ui, label, None, |ui| {
        let (rect, _) = ui.allocate_exact_size(
            vec2(ui.available_width(), t.metrics.ctl_h),
            egui::Sense::hover(),
        );
        ui.painter().rect(
            rect,
            2.0,
            t.color.bg_inset,
            egui::Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            rect.left_center() + vec2(8.0, 0.0),
            egui::Align2::LEFT_CENTER,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text,
        );
    });
}

/// Mono sub-header inside a form ("TONE 2", "PORT 1").
fn sub_header(ui: &mut Ui, text: &str) {
    clear_pending_cell(ui);
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_faint,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
}

/// A full-width ghost add/remove action line. Returns `true` on click.
fn action_line(ui: &mut Ui, label: &str) -> bool {
    clear_pending_cell(ui);
    Button::new(label)
        .ghost()
        .min_width(ui.available_width())
        .show(ui)
        .clicked()
}

fn frequency_sweep_fields(
    ui: &mut Ui,
    sweep: &mut FrequencySweepDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Start",
        &mut sweep.start,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Stop",
        &mut sweep.stop,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(ui, "Points", &mut sweep.points);
    choice_row(ui, "Sweep", SWEEP_KINDS, &mut sweep.sweep);
}

fn periodic_network_fields(
    ui: &mut Ui,
    setup: &mut PeriodicNetworkDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    input_row(ui, "Max sideband", &mut setup.max_sideband);
    check_row(ui, "Mixed-mode matrix", &mut setup.mixed_mode);
    check_row(ui, "Noise parameters", &mut setup.noise_parameters);
    let port_count = setup.ports.len();
    let mut remove = None;
    for (index, port) in setup.ports.iter_mut().enumerate() {
        network_port_fields(ui, index, port);
        if port_count > 1 && action_line(ui, "Remove port") {
            remove = Some(index);
        }
    }
    if let Some(index) = remove {
        setup.ports.remove(index);
    }
    if action_line(ui, "+ Add port") {
        setup.ports.push(NetworkPortDraft::default());
    }
}

fn network_port_fields(ui: &mut Ui, index: usize, port: &mut NetworkPortDraft) {
    sub_header(ui, &format!("Port {}", index + 1));
    input_row(ui, "Node +", &mut port.node_pos);
    input_row(ui, "Node −", &mut port.node_neg);
    input_row(ui, "Reference Z0", &mut port.z0);
}

/// Render the form for `draft`; returns the explanatory note.
pub(super) fn form(
    ui: &mut Ui,
    draft: &mut AnalysisDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
    envelope_modulation_sources: &[String],
) -> &'static str {
    clear_pending_cell(ui);
    ui.spacing_mut().item_spacing.y = FIELD_ROW_GAP;
    let note = match draft {
        AnalysisDraft::OperatingPoint(setup) => {
            setup.ensure_initialized();
            property_row(ui, "Temperature", "PVT run set");
            input_row(ui, "GMIN steps", &mut setup.gmin_steps);
            check_row(ui, "Source stepping", &mut setup.source_stepping);
            check_row(ui, "Save all signals", &mut setup.save_all);
            check_row(ui, "Save OP details", &mut setup.save_op_info);
            "Solves the DC operating point; device bias lands in the OP inspector."
        }
        AnalysisDraft::Transient(setup) => {
            quantity_input_row(
                ui,
                "Stop time",
                &mut setup.stop,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Step time",
                &mut setup.step,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Start time",
                &mut setup.start,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            if !setup.max_step.eq_ignore_ascii_case("auto") {
                quantity_input_row(
                    ui,
                    "Max step",
                    &mut setup.max_step,
                    QuantityInputKind::Time,
                    policy,
                    locale,
                );
            } else {
                input_row(ui, "Max step", &mut setup.max_step);
            }
            check_row(ui, "Use initial conditions", &mut setup.uic);
            "Local truncation error controls step size between limits."
        }
        AnalysisDraft::Ac(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.fstart,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.fstop,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep);
            "Small-signal sweep around the operating point."
        }
        AnalysisDraft::DcSweep(setup) => {
            input_row(ui, "Source", &mut setup.source);
            input_row(ui, "Start", &mut setup.start);
            input_row(ui, "Stop", &mut setup.stop);
            input_row(ui, "Step", &mut setup.step);
            check_row(ui, "Nested sweep", &mut setup.nested);
            // Nested-sweep enablement is a complete field group. Do not pair
            // the first secondary-sweep value with the checkbox: doing so
            // shifts every following field by one column and leaves Step 2
            // stranded on a partial final row.
            clear_pending_cell(ui);
            input_row_enabled(ui, "Source 2", &mut setup.source2, setup.nested);
            input_row_enabled(ui, "Start 2", &mut setup.start2, setup.nested);
            input_row_enabled(ui, "Stop 2", &mut setup.stop2, setup.nested);
            input_row_enabled(ui, "Step 2", &mut setup.step2, setup.nested);
            "Sweeps a source over the operating range."
        }
        AnalysisDraft::Noise(setup) => {
            input_row(ui, "Output", &mut setup.output);
            input_row(ui, "Reference", &mut setup.reference);
            input_row(ui, "Input src", &mut setup.input);
            quantity_input_row(
                ui,
                "Start",
                &mut setup.fstart,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.fstop,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep);
            "Integrated and spot noise over its independent small-signal sweep."
        }
        AnalysisDraft::PoleZero(setup) => {
            input_row(ui, "Input +", &mut setup.input_pos);
            input_row(ui, "Input −", &mut setup.input_neg);
            input_row(ui, "Output +", &mut setup.output_pos);
            input_row(ui, "Output −", &mut setup.output_neg);
            choice_row(ui, "Transfer", &["V", "I"], &mut setup.transfer_idx);
            choice_row(
                ui,
                "Roots",
                &["both", "poles", "zeros"],
                &mut setup.analysis_idx,
            );
            "Extracts poles and zeros of the small-signal transfer."
        }
        AnalysisDraft::Sensitivity(setup) => {
            input_row(ui, "Output", &mut setup.output_expr);
            choice_row(ui, "Mode", &["DC", "AC"], &mut setup.sens_type_idx);
            quantity_input_row_enabled(
                ui,
                "Frequency",
                &mut setup.ac_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
                setup.sens_type_idx == 1,
            );
            check_row(ui, "Include parameters", &mut setup.include_params);
            check_row(ui, "Include devices", &mut setup.include_devices);
            "Sensitivity of the output to every parameter."
        }
        AnalysisDraft::MonteCarlo(setup) => {
            input_row(ui, "Samples", &mut setup.num_runs);
            input_row(ui, "Seed", &mut setup.seed);
            input_row(ui, "Spread %", &mut setup.variation_pct);
            choice_row(
                ui,
                "Vary",
                &["gauss", "uniform", "worst"],
                &mut setup.distribution_idx,
            );
            choice_row(ui, "Base", &["tran", "ac", "dc", "op"], &mut setup.base_idx);
            check_row(ui, "Process variations", &mut setup.process_variations);
            check_row(ui, "Mismatch variations", &mut setup.mismatch_variations);
            check_row(ui, "Save every run", &mut setup.save_all_runs);
            "Statistical sampling around the nominal design."
        }
        AnalysisDraft::Pss(setup) => {
            quantity_input_row(
                ui,
                "Fundamental",
                &mut setup.fund_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Harmonics", &mut setup.num_harmonics);
            input_row(ui, "Max iters", &mut setup.max_iter);
            choice_row(ui, "Method", &["shooting", "HB"], &mut setup.method_idx);
            check_row(ui, "Autonomous oscillator", &mut setup.osc_mode);
            // The oscillator field is a stable member of the grid. Toggling
            // autonomous mode changes enablement, not the position of every
            // field that follows it.
            input_row_enabled(ui, "Oscillator node", &mut setup.osc_node, setup.osc_mode);
            check_row(ui, "Save harmonics", &mut setup.save_harmonics);
            "Periodic steady state of the large-signal circuit."
        }
        AnalysisDraft::Stb(setup) => {
            input_row(ui, "Probe", &mut setup.probe_source);
            quantity_input_row(
                ui,
                "Start",
                &mut setup.start_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.stop_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points/dec", &mut setup.points_per_decade);
            check_row(ui, "Gain margin", &mut setup.gain_margin);
            check_row(ui, "Phase margin", &mut setup.phase_margin);
            check_row(ui, "Crossover freq", &mut setup.crossover_freq);
            "Loop gain and margins via the probe source."
        }
        AnalysisDraft::Temperature(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.temp_start,
                QuantityInputKind::Temperature,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.temp_stop,
                QuantityInputKind::Temperature,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Step",
                &mut setup.temp_step,
                QuantityInputKind::TemperatureDelta,
                policy,
                locale,
            );
            choice_row(ui, "Base", &["op", "tran", "ac", "dc"], &mut setup.base_idx);
            check_row(ui, "Corner temps only", &mut setup.corner_temps);
            "Repeats the base analysis across temperature."
        }
        AnalysisDraft::HarmonicBalance(setup) => {
            quantity_input_row(
                ui,
                "Fundamental",
                &mut setup.fundamental,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Harmonics", &mut setup.harmonics);
            input_row(ui, "Source", &mut setup.fundamental_source);
            input_row(ui, "Oversample", &mut setup.oversample);
            input_row(ui, "Max iters", &mut setup.maxiter);
            choice_row(ui, "Solver", &["newton", "krylov"], &mut setup.solver_idx);
            check_row(ui, "Source stepping", &mut setup.source_stepping);
            let mut remove: Option<usize> = None;
            for (idx, tone) in setup.additional_tones.iter_mut().enumerate() {
                sub_header(ui, &format!("Tone {}", idx + 2));
                quantity_input_row(
                    ui,
                    "Frequency",
                    &mut tone.frequency,
                    QuantityInputKind::Frequency,
                    policy,
                    locale,
                );
                input_row(ui, "Harmonics", &mut tone.harmonics);
                input_row(ui, "Source", &mut tone.source);
                if action_line(ui, "Remove tone") {
                    remove = Some(idx);
                }
            }
            if let Some(idx) = remove {
                setup.additional_tones.remove(idx);
            }
            ui.add_space(4.0);
            if action_line(ui, "+ Add tone") {
                setup.additional_tones.push(Default::default());
            }
            "Multi-tone steady state in the frequency domain."
        }
        AnalysisDraft::SParameter(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.start_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.stop_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Z0", &mut setup.z0);
            check_row(ui, "Noise parameters", &mut setup.do_noise);
            check_row(ui, "Touchstone export", &mut setup.touchstone_export);
            let mut remove: Option<usize> = None;
            let port_count = setup.ports.len();
            for (idx, port) in setup.ports.iter_mut().enumerate() {
                sub_header(ui, &format!("Port {}", idx + 1));
                input_row(ui, "Node +", &mut port.node_pos);
                check_row(ui, "Differential", &mut port.differential);
                if port.differential {
                    input_row(ui, "Node −", &mut port.node_neg);
                }
                check_row(ui, "Z0 override", &mut port.z0_override);
                if port.z0_override {
                    input_row(ui, "Port Z0", &mut port.z0);
                }
                if port_count > 1 && action_line(ui, "Remove port") {
                    remove = Some(idx);
                }
            }
            if let Some(idx) = remove {
                setup.ports.remove(idx);
            }
            ui.add_space(4.0);
            if action_line(ui, "+ Add port") {
                setup.ports.push(Default::default());
            }
            "Scattering parameters between the defined ports."
        }
        AnalysisDraft::Pac(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.start_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.stop_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Magnitude", &mut setup.pac_magnitude);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            check_row(ui, "Include DC", &mut setup.include_dc);
            "Small-signal AC around the periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pnoise(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.start_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.stop_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            choice_row(
                ui,
                "Refer to",
                &["output", "input", "phase"],
                &mut setup.noise_ref_idx,
            );
            check_row(ui, "Integrated noise", &mut setup.integrated_noise);
            check_row(ui, "Noise summary", &mut setup.noise_summary);
            "Cyclostationary noise around the periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pxf(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.start_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.stop_freq,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.num_points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Out sideband", &mut setup.output_sideband);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            "Transfer functions onto a periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pstb(setup) => {
            input_row(ui, "Probe", &mut setup.probe);
            input_row(ui, "Harmonics", &mut setup.max_harmonics);
            input_row(ui, "Multipliers", &mut setup.num_multipliers);
            check_row(ui, "Annotate", &mut setup.annotate);
            check_row(ui, "Phase margin", &mut setup.phase_margin);
            check_row(ui, "Gain margin", &mut setup.gain_margin);
            "Loop stability around the periodic steady state (needs PSS)."
        }
        AnalysisDraft::TransferFunction(setup) => {
            input_row(ui, XF_FIELD_LABELS[0], &mut setup.input_source);
            input_row(ui, XF_FIELD_LABELS[1], &mut setup.output_expression);
            property_row(ui, XF_FIELD_LABELS[2], XF_SOLVE_POINT);
            enabled_choice_row(ui, XF_FIELD_LABELS[3], &mut setup.transfer_gain);
            enabled_choice_row(ui, XF_FIELD_LABELS[4], &mut setup.input_resistance);
            enabled_choice_row(ui, XF_FIELD_LABELS[5], &mut setup.output_resistance);
            choice_row(
                ui,
                XF_FIELD_LABELS[6],
                XF_NORMALIZATION_CHOICES,
                &mut setup.normalization_idx,
            );
            choice_row(
                ui,
                XF_FIELD_LABELS[7],
                XF_ACCURACY_CHOICES,
                &mut setup.accuracy_idx,
            );
            "DC-linearized transfer gain plus input and output resistance."
        }
        AnalysisDraft::Corner(setup) => {
            sub_header(ui, "Process");
            check_row(ui, "TT — typical", &mut setup.process_tt);
            check_row(ui, "SS — slow/slow", &mut setup.process_ss);
            check_row(ui, "FF — fast/fast", &mut setup.process_ff);
            check_row(ui, "SF — slow/fast", &mut setup.process_sf);
            check_row(ui, "FS — fast/slow", &mut setup.process_fs);
            sub_header(ui, "Supply");
            check_row(ui, "Sweep voltage", &mut setup.enable_voltage_sweep);
            input_row_enabled(
                ui,
                "Min",
                &mut setup.voltage_min,
                setup.enable_voltage_sweep,
            );
            input_row_enabled(
                ui,
                "Nominal",
                &mut setup.voltage_nom,
                setup.enable_voltage_sweep,
            );
            input_row_enabled(
                ui,
                "Max",
                &mut setup.voltage_max,
                setup.enable_voltage_sweep,
            );
            sub_header(ui, "Temperature");
            check_row(ui, "Sweep temperature", &mut setup.enable_temp_sweep);
            quantity_input_row_enabled(
                ui,
                "Cold",
                &mut setup.temp_cold,
                QuantityInputKind::Temperature,
                policy,
                locale,
                setup.enable_temp_sweep,
            );
            quantity_input_row_enabled(
                ui,
                "Room",
                &mut setup.temp_room,
                QuantityInputKind::Temperature,
                policy,
                locale,
                setup.enable_temp_sweep,
            );
            quantity_input_row_enabled(
                ui,
                "Hot",
                &mut setup.temp_hot,
                QuantityInputKind::Temperature,
                policy,
                locale,
                setup.enable_temp_sweep,
            );
            ui.add_space(4.0);
            check_row(ui, "Full matrix", &mut setup.full_matrix);
            choice_row(
                ui,
                "Base",
                &["tran", "ac", "dc", "op"],
                &mut setup.base_analysis_idx,
            );
            "Repeats the base analysis across the selected corners."
        }
        AnalysisDraft::Envelope(setup) => {
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
            "Envelope-following transient for modulated carriers."
        }
        AnalysisDraft::Fourier(setup) => {
            quantity_input_row(
                ui,
                "Fundamental",
                &mut setup.fundamental,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Harmonics", &mut setup.harmonics);
            input_row(ui, "Output", &mut setup.output_node);
            quantity_input_row(
                ui,
                "From",
                &mut setup.start_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "To",
                &mut setup.stop_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            check_row(ui, "Compute THD", &mut setup.compute_thd);
            check_row(ui, "Normalize", &mut setup.normalize);
            "Fourier components of a transient waveform window."
        }
        AnalysisDraft::Reliability(setup) => {
            input_row(ui, "Years", &mut setup.years_csv);
            input_row(ui, "Min stress V", &mut setup.min_stress_voltage);
            check_row(ui, "Hot carrier (HCI)", &mut setup.enable_hci);
            check_row(ui, "Bias instability (NBTI)", &mut setup.enable_nbti);
            check_row(ui, "Electromigration", &mut setup.enable_em);
            "Projects device aging across the lifetime points."
        }
        AnalysisDraft::Optimization(setup) => {
            input_row(ui, "Variables", &mut setup.variables_text);
            input_row(ui, "Objective", &mut setup.objective_node);
            input_row(ui, "Obj ref", &mut setup.objective_ref);
            choice_row(ui, "Goal", &["min", "max", "target"], &mut setup.goal_mode);
            input_row_enabled(ui, "Target", &mut setup.target_value, setup.goal_mode == 2);
            choice_row(
                ui,
                "Method",
                &["gradient", "pattern", "anneal"],
                &mut setup.algorithm,
            );
            input_row(ui, "Max iters", &mut setup.max_iterations);
            input_row(ui, "Tolerance", &mut setup.cost_tolerance);
            "Tunes the variables (name:min:max[:initial]) toward the goal."
        }
        AnalysisDraft::Soa(setup) => {
            quantity_input_row(
                ui,
                "Stop time",
                &mut setup.stop_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Step time",
                &mut setup.step_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            check_row(ui, "Check Vgs", &mut setup.check_vgs_max);
            input_row_enabled(ui, "Max Vgs", &mut setup.max_vgs, setup.check_vgs_max);
            check_row(ui, "Check Vds", &mut setup.check_vds_max);
            input_row_enabled(ui, "Max Vds", &mut setup.max_vds, setup.check_vds_max);
            check_row(ui, "Check Vbe", &mut setup.check_vbe_max);
            input_row_enabled(ui, "Max Vbe", &mut setup.max_vbe, setup.check_vbe_max);
            check_row(ui, "Check Vce", &mut setup.check_vce_max);
            input_row_enabled(ui, "Max Vce", &mut setup.max_vce, setup.check_vce_max);
            "Flags excursions outside the safe operating area during transient."
        }
        AnalysisDraft::Disto(setup) => {
            quantity_input_row(
                ui,
                "Start",
                &mut setup.sweep.fstart,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Stop",
                &mut setup.sweep.fstop,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Points", &mut setup.sweep.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep.sweep);
            input_row(ui, "f2/f1", &mut setup.f2_over_f1);
            "Harmonic and intermodulation distortion; empty ratio means single-tone."
        }
        AnalysisDraft::Qpss(setup) => {
            input_row(ui, "Tone frequencies", &mut setup.tones);
            input_row(ui, "Harmonic orders", &mut setup.harmonics);
            input_row(ui, "Max iterations", &mut setup.max_iterations);
            input_row(ui, "Relative tolerance", &mut setup.relative_tolerance);
            check_row(ui, "Autonomous oscillator", &mut setup.autonomous);
            input_row_enabled(
                ui,
                "Oscillator node",
                &mut setup.oscillator_node,
                setup.autonomous,
            );
            "Multi-tone spectral-lattice operating state (needs OP)."
        }
        AnalysisDraft::Hbsp(setup) => {
            periodic_network_fields(ui, setup, policy, locale);
            "Large-signal network response linearized around harmonic balance (needs HB)."
        }
        AnalysisDraft::Hbnoise(setup) => {
            frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input source", &mut setup.input_source);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            check_row(ui, "Integrated noise", &mut setup.integrated_noise);
            check_row(ui, "Noise figure", &mut setup.noise_figure);
            check_row(ui, "Contributor ranking", &mut setup.contributor_ranking);
            "Noise folding and correlation around harmonic balance (needs HB)."
        }
        AnalysisDraft::Psp(setup) => {
            periodic_network_fields(ui, setup, policy, locale);
            "Frequency-translated network response around PSS (needs PSS)."
        }
        AnalysisDraft::Qpac(setup) => {
            frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
            input_row(ui, "Input source", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input lattice", &mut setup.input_lattice);
            input_row(ui, "Output lattice", &mut setup.output_lattice);
            "Small-signal conversion matrix around QPSS (needs QPSS)."
        }
        AnalysisDraft::Qpnoise(setup) => {
            frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input source", &mut setup.input_source);
            input_row(ui, "Lattice ranges", &mut setup.lattice_products);
            check_row(ui, "Integrated noise", &mut setup.integrated_noise);
            check_row(ui, "Contributor ranking", &mut setup.contributor_ranking);
            "Noise folding across a multi-tone spectral lattice (needs QPSS)."
        }
        AnalysisDraft::Qpxf(setup) => {
            frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
            input_row(ui, "Input source", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input lattice", &mut setup.input_lattice);
            input_row(ui, "Output lattice", &mut setup.output_lattice);
            check_row(ui, "Group delay", &mut setup.group_delay);
            "Translated transfer paths indexed by lattice products (needs QPSS)."
        }
        AnalysisDraft::TransientNoise(setup) => {
            quantity_input_row(
                ui,
                "Stop time",
                &mut setup.stop_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Step time",
                &mut setup.step_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Start time",
                &mut setup.start_time,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            quantity_input_row(
                ui,
                "Max step",
                &mut setup.max_step,
                QuantityInputKind::Time,
                policy,
                locale,
            );
            input_row(ui, "Seed", &mut setup.seed);
            quantity_input_row(
                ui,
                "Noise fmax",
                &mut setup.noise_fmax,
                QuantityInputKind::Frequency,
                policy,
                locale,
            );
            input_row(ui, "Noise scale", &mut setup.scale);
            check_row(
                ui,
                "Use initial conditions",
                &mut setup.use_initial_conditions,
            );
            "Reproducible stochastic device noise in the time domain (needs TRAN)."
        }
        AnalysisDraft::DcMismatch(setup) => {
            input_row(ui, "Output expression", &mut setup.output_expression);
            input_row(ui, "Sigma multiplier", &mut setup.sigma_multiplier);
            input_row(ui, "Contributor limit", &mut setup.contributor_limit);
            check_row(ui, "Process variation", &mut setup.include_process);
            check_row(ui, "Local mismatch", &mut setup.include_mismatch);
            check_row(
                ui,
                "Normalize contributions",
                &mut setup.normalized_contributions,
            );
            "Local mismatch variance and ranked contributions around OP (needs OP)."
        }
    };
    clear_pending_cell(ui);
    note
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::dialog::{PssConfig, PssDialogState};
    use crate::simulation::plan::AnalysisKind;

    #[cfg(not(target_arch = "wasm32"))]
    fn analysis_form_height(mut draft: AnalysisDraft) -> f32 {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut height = 0.0;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(964.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        let top = ui.cursor().top();
                        form(
                            ui,
                            &mut draft,
                            QuantityPresentationPolicy::default(),
                            UiNumberLocale::default(),
                            &["VIN_AM".to_owned(), "VIN_IQ".to_owned()],
                        );
                        height = ui.cursor().top() - top;
                    });
            },
        );
        height
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pss_form_height(oscillator_mode: bool) -> f32 {
        let mut setup = PssDialogState::from_config(&PssConfig::default());
        setup.osc_mode = oscillator_mode;
        analysis_form_height(AnalysisDraft::Pss(setup))
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn pss_oscillator_toggle_preserves_form_geometry() {
        let driven_height = pss_form_height(false);
        let oscillator_height = pss_form_height(true);
        assert_eq!(driven_height, oscillator_height);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn conditional_analysis_controls_preserve_form_geometry() {
        let mut pairs = Vec::new();

        let disabled = AnalysisDraft::for_kind(AnalysisKind::DcSweep);
        let mut enabled = disabled.clone();
        if let AnalysisDraft::DcSweep(setup) = &mut enabled {
            setup.nested = true;
        }
        pairs.push((disabled, enabled));

        let disabled = AnalysisDraft::for_kind(AnalysisKind::Sensitivity);
        let mut enabled = disabled.clone();
        if let AnalysisDraft::Sensitivity(setup) = &mut enabled {
            setup.sens_type_idx = 1;
        }
        pairs.push((disabled, enabled));

        let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Corner);
        let mut enabled = disabled.clone();
        if let AnalysisDraft::Corner(setup) = &mut disabled {
            setup.enable_voltage_sweep = false;
            setup.enable_temp_sweep = false;
        }
        if let AnalysisDraft::Corner(setup) = &mut enabled {
            setup.enable_voltage_sweep = true;
            setup.enable_temp_sweep = true;
        }
        pairs.push((disabled, enabled));

        let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Optimization);
        let mut enabled = disabled.clone();
        if let AnalysisDraft::Optimization(setup) = &mut disabled {
            setup.goal_mode = 0;
        }
        if let AnalysisDraft::Optimization(setup) = &mut enabled {
            setup.goal_mode = 2;
        }
        pairs.push((disabled, enabled));

        let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Soa);
        let mut enabled = disabled.clone();
        if let AnalysisDraft::Soa(setup) = &mut disabled {
            setup.check_vgs_max = false;
            setup.check_vds_max = false;
            setup.check_vbe_max = false;
            setup.check_vce_max = false;
        }
        if let AnalysisDraft::Soa(setup) = &mut enabled {
            setup.check_vgs_max = true;
            setup.check_vds_max = true;
            setup.check_vbe_max = true;
            setup.check_vce_max = true;
        }
        pairs.push((disabled, enabled));

        let mut disabled = AnalysisDraft::for_kind(AnalysisKind::Qpss);
        let mut enabled = disabled.clone();
        if let AnalysisDraft::Qpss(setup) = &mut disabled {
            setup.autonomous = false;
        }
        if let AnalysisDraft::Qpss(setup) = &mut enabled {
            setup.autonomous = true;
        }
        pairs.push((disabled, enabled));

        for (disabled, enabled) in pairs {
            assert_eq!(
                analysis_form_height(disabled),
                analysis_form_height(enabled)
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn envelope_modulation_source_modes_preserve_form_geometry() {
        let mut named = AnalysisDraft::for_kind(AnalysisKind::Envelope);
        let mut declared = named.clone();
        if let AnalysisDraft::Envelope(setup) = &mut named {
            setup.modulation_sources = "VIN_AM".to_owned();
        }
        if let AnalysisDraft::Envelope(setup) = &mut declared {
            setup.modulation_sources = "VCTRL, VDATA".to_owned();
        }

        assert_eq!(analysis_form_height(named), analysis_form_height(declared));
    }

    #[test]
    fn envelope_modulation_source_split_is_fixed_and_fills_the_control_row() {
        for available_width in [1.0, 120.0, 320.0, 640.0] {
            let (selector, editor) = envelope_modulation_control_widths(available_width);
            assert!(selector > 0.0);
            assert!(editor > 0.0);
            assert_eq!(
                selector + editor + ENVELOPE_INLINE_CONTROL_GAP,
                available_width.max(ENVELOPE_INLINE_CONTROL_GAP + 2.0)
            );
        }
    }

    #[test]
    fn envelope_form_matches_mockup_owned_contract() {
        assert_eq!(
            ENVELOPE_FIELD_LABELS,
            [
                "Carrier tones",
                "Envelope stop",
                "Envelope step",
                "Harmonic order",
                "Modulation sources",
                "Initial periodic solve",
                "Output schedule",
                "Extraction path",
            ]
        );
        assert_eq!(
            ENVELOPE_INITIAL_SOLVE_CHOICES,
            ["HB", "PSS", "Transient spectral estimate"]
        );
        assert_eq!(
            ENVELOPE_ADAPTIVE_CHOICES,
            [
                "Adaptive solver samples",
                "Fixed envelope step",
                "Event-aligned only",
            ]
        );
        assert_eq!(ENVELOPE_DECLARED_SOURCES_CHOICE, "Declared list...");
        assert_eq!(ENVELOPE_HARMONIC_ORDER_HELPER, "positive integer");
        assert_eq!(ENVELOPE_EXTRACTION_PATH, "Preview");
    }

    #[test]
    fn transfer_function_form_matches_mockup_owned_contract() {
        assert_eq!(
            XF_FIELD_LABELS,
            [
                "Input source",
                "Output expression",
                "Solve point",
                "Transfer gain",
                "Input resistance",
                "Output resistance",
                "Normalize",
                "Accuracy",
            ]
        );
        assert_eq!(XF_SOLVE_POINT, "DC operating point");
        assert_eq!(XF_ENABLED_CHOICES, ["Enabled", "Disabled"]);
        assert_eq!(
            XF_NORMALIZATION_CHOICES,
            ["Disabled", "Relative to nominal", "Per source unit"]
        );
        assert_eq!(
            XF_ACCURACY_CHOICES,
            ["Fast", "Balanced", "Accurate", "Robust"]
        );
    }
}
