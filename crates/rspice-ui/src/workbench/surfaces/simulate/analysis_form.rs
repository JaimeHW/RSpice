//! Per-analysis configuration forms for the Simulate right panel.
//!
//! Each form edits one typed [`AnalysisDraft`] owned by a stable analysis
//! instance. The form returns a one-line note describing what the analysis
//! does; validation is rendered by the caller.

mod dc_sweep;
mod pss;
mod run_space;
mod stb_probe;
mod sweep_point_label;

pub(super) use run_space::RunSpaceContext;
use sweep_point_label::{
    SWEEP_KINDS, SWEEP_POINT_NEUTRAL_LABEL, noise_point_field_label, sweep_point_field_label,
};

use egui::{Align, Layout, Rect, Response, Ui, UiBuilder, vec2};

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::services::simulation_runner::TfRunConfig;
use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};

use crate::simulation::plan::{
    AnalysisDraft, FrequencySweepDraft, NetworkPortDraft, PeriodicNetworkDraft,
};
use crate::state::format_engineering;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, choice_row as inspector_choice_row, input_row as inspector_input_row, mono_input,
    select, select_mono_with_response, select_with_disabled, switch_row as inspector_switch_row,
};
use crate::workbench::design_system::property_row as inspector_property_row;

const NOISE_FIELD_LABELS: [&str; 8] = [
    "Sweep",
    // The point field names its own units, so the frozen entry is the ungraded
    // spelling and the rendered one re-resolves. See
    // `the_sweep_point_label_names_what_a_point_is_in_each_mode`.
    SWEEP_POINT_NEUTRAL_LABEL,
    "Start frequency",
    "Stop frequency",
    "Output node",
    "Input source",
    "Contribution detail",
    "Integrated noise",
];
const NOISE_SWEEP_CHOICES: [&str; 4] = NoiseSweepType::OPTIONS;
const NOISE_OUTPUT_CUSTOM_CHOICE: &str = "Exact expression\u{2026}";
const NOISE_INPUT_CUSTOM_CHOICE: &str = "Exact source name\u{2026}";
/// How many elaborated names one noise row offers as presets.
///
/// The select paints one row per option with no scroll of its own, so a design
/// with thousands of nodes would open a popup taller than the screen. The
/// presets are a shortcut, not the domain: the exact-entry field accepts
/// anything the design contains, and the row states how much of the design it
/// is showing whenever it shows less than all of it.
const NOISE_DOMAIN_PRESET_LIMIT: usize = 64;
const NOISE_CONTRIBUTION_CHOICES: [&str; 4] = NoiseContributionDetail::OPTIONS;
const NOISE_INTEGRATION_CHOICES: [&str; 3] = NoiseIntegrationMode::OPTIONS;
const OP_FIELD_LABELS: [&str; 8] = [
    "Temperature",
    "Initial guess",
    "Node initialization",
    "Homotopy strategy",
    "Annotate schematic",
    "Device detail",
    "Save device OP",
    "Accuracy preset",
];
const OP_TEMPERATURE_CHOICES: [&str; 4] = [
    "PVT run set",
    "Nominal temperature \u{00b7} 27 \u{00b0}C",
    "Explicit temperature\u{2026}",
    "Inherit active run-set axis",
];
const OP_INITIAL_GUESS_CHOICES: [&str; 4] = [
    "Automatic",
    "Previous converged solution",
    "User node voltages",
    "Zero state",
];
const OP_NODE_INITIALIZATION_CHOICES: [&str; 4] = [
    "Use IC / nodeset",
    "Ignore IC and nodeset",
    "Force .ic values",
    "Validate initialization only",
];
const OP_HOMOTOPY_CHOICES: [&str; 5] = [
    "Adaptive",
    "Source stepping",
    "Gmin stepping",
    "Pseudo-transient",
    "None",
];
const OP_ANNOTATION_CHOICES: [&str; 4] = [
    "Voltages + currents",
    "Voltages only",
    "Voltages + device OP",
    "None",
];
const OP_DEVICE_DETAIL_CHOICES: [&str; 4] = [
    "Selected + violations",
    "All devices",
    "Violations only",
    "None",
];
const OP_SAVE_DEVICE_CHOICES: [&str; 3] = ["Enabled", "Disabled", "Final point only"];
const OP_ACCURACY_CHOICES: [&str; 4] = ["Fast", "Balanced", "Accurate", "Robust"];
const PSS_FIELD_LABELS: [&str; 9] = [
    "Mode",
    "Fundamental",
    "Tones",
    "Stabilization cycles",
    "Shooting points",
    "Period tolerance",
    "Autonomous oscillator",
    "Oscillator node",
    "Save harmonics",
];
const PSS_MODE_CHOICES: [&str; 1] = ["Driven shooting"];
const OP_STARTUP_CONFLICT: &str =
    "This initial-guess and node-initialization combination is not executable";

fn op_initial_guess_disabled(
    node_initialization_idx: usize,
    previous_state_available: bool,
) -> Vec<(usize, &'static str)> {
    let mut disabled = Vec::new();
    for initial_guess_idx in 0..OP_INITIAL_GUESS_CHOICES.len() {
        if !op_startup_indices_compatible(initial_guess_idx, node_initialization_idx) {
            disabled.push((initial_guess_idx, OP_STARTUP_CONFLICT));
        } else if initial_guess_idx == 1 && !previous_state_available {
            disabled.push((
                initial_guess_idx,
                "Run and retain a source-compatible OP state before selecting this policy",
            ));
        }
    }
    disabled
}

fn op_node_initialization_disabled(initial_guess_idx: usize) -> Vec<(usize, &'static str)> {
    (0..OP_NODE_INITIALIZATION_CHOICES.len())
        .filter(|node_idx| !op_startup_indices_compatible(initial_guess_idx, *node_idx))
        .map(|node_idx| (node_idx, OP_STARTUP_CONFLICT))
        .collect()
}

const fn op_startup_indices_compatible(
    initial_guess_idx: usize,
    node_initialization_idx: usize,
) -> bool {
    match initial_guess_idx {
        0 => true,
        1 | 3 => matches!(node_initialization_idx, 1 | 3),
        2 => matches!(node_initialization_idx, 0 | 2),
        _ => false,
    }
}
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
/// The action that fills the two fields above it from the design's own deck.
const XF_INFER_LABEL: &str = "Infer from deck";
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
const ENVELOPE_EXTRACTION_PATH: &str = "Least-squares projection";
const ENVELOPE_HARMONIC_ORDER_HELPER: &str = "positive integer";
const ENVELOPE_INLINE_CONTROL_GAP: f32 = 6.0;
const NOISE_SWEEP_CONTROL_COUNT: usize = 2;
const FIELD_COLUMN_GAP: f32 = 14.0;
const FIELD_ROW_GAP: f32 = 10.0;
const FIELD_LABEL_HEIGHT: f32 = 15.0;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct OpContextAvailability {
    pub previous_state: bool,
    pub soa_violations: bool,
}

/// The elaborated vocabulary the noise form offers as presets.
///
/// Noise refers a result to an excitation by name: the engine resolves the
/// output against the elaborated node map and the input against the
/// elaborated independent sources, so those two collections are the only
/// honest preset lists this form has. A name the design does not carry is not
/// a suggestion, it is a run that fails at validation.
///
/// A design with no nodes and a design that could not be elaborated are
/// different facts, and [`noise_domain_hint`] states which one the row has.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct NoiseDomain<'a> {
    /// Elaborated node names, in the order the row should offer them.
    pub nodes: &'a [String],
    /// Elaborated independent voltage and current sources.
    pub sources: &'a [String],
    /// Why the design could not be elaborated, when it could not be; neither
    /// list was measured in that case, and it is not the same as both being
    /// empty. The reason travels with the fact, because a form that says a
    /// vocabulary is unavailable and not why leaves the reader with nothing to
    /// go and fix.
    pub unavailable: Option<&'a str>,
}

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

fn full_width_field<R>(
    ui: &mut Ui,
    label: &str,
    helper: Option<&str>,
    control_height: f32,
    add_control: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let row_height = FIELD_LABEL_HEIGHT + 5.0 + control_height;
    let (rect, _) =
        ui.allocate_exact_size(vec2(ui.available_width(), row_height), egui::Sense::hover());
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
        mono_input(ui, label, value, ui.available_width())
    })
}

fn input_row_enabled(ui: &mut Ui, label: &str, value: &mut String, enabled: bool) -> Response {
    if !uses_two_column_fields(ui) {
        return ui
            .add_enabled_ui(enabled, |ui| inspector_input_row(ui, label, value))
            .inner;
    }
    field_cell(ui, label, None, |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            mono_input(ui, label, value, ui.available_width())
        })
        .inner
    })
}

fn engineering_input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, Some("engineering notation"), |ui| {
        mono_input(ui, label, value, ui.available_width())
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
        ui.add_enabled_ui(enabled, |ui| {
            mono_input(ui, label, value, ui.available_width())
        })
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
    normalize_quantity_on_focus_loss(&response, value, kind, policy, locale);
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
    if enabled {
        normalize_quantity_on_focus_loss(&response, value, kind, policy, locale);
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

fn noise_enum_choice_row(
    ui: &mut Ui,
    label: &str,
    options: &[&str],
    selected: usize,
) -> Option<usize> {
    let mut next = selected;
    choice_row(ui, label, options, &mut next).then_some(next)
}

/// What a noise domain row is offering, stated in the row itself.
///
/// `offered` is what the select paints, `total` what the elaborated design
/// carries. Saying "design nodes" while showing 64 of 812 of them would be the
/// same fabrication in a smaller font, so a truncated list says so.
fn noise_domain_hint(kind: &str, offered: usize, total: usize, unavailable: bool) -> String {
    if unavailable {
        // The reason is not here: this caption is painted right-aligned on the
        // field's label row and clipped to the cell, so an elaboration
        // diagnostic put in it would be cut off or land on the label. It goes
        // under the two rows it explains, in `field_advisory`.
        format!("design {kind} unavailable")
    } else if total == 0 {
        format!("no design {kind}")
    } else if offered < total {
        format!("{offered} of {total} {kind}")
    } else {
        format!("design {kind}")
    }
}

fn noise_domain_control(
    ui: &mut Ui,
    label: &str,
    id_namespace: &str,
    names: &[String],
    custom_choice: &str,
    value: &mut String,
) {
    let preset = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(value.trim()));
    let custom_index = names.len();
    let selected = preset.unwrap_or(custom_index);
    let mut custom_selected = selected == custom_index;
    let mut options = names.to_vec();
    options.push(custom_choice.to_owned());
    let current = options.get(selected).map_or(custom_choice, String::as_str);
    let width = ui.available_width();
    let (selector_width, editor_width) = noise_sweep_control_widths(width);
    ui.allocate_ui_with_layout(
        vec2(width, Tokens::get(ui.ctx()).metrics.ctl_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = ENVELOPE_INLINE_CONTROL_GAP;
            let salt = format!("analysis-noise-{id_namespace}-{}", ui.id().value());
            if let Some(index) =
                select_mono_with_response(ui, &salt, label, current, &options, selector_width)
                    .picked
            {
                if let Some(name) = names.get(index) {
                    *value = name.clone();
                    custom_selected = false;
                } else if selected != custom_index {
                    value.clear();
                    custom_selected = true;
                }
            }
            ui.add_enabled_ui(custom_selected, |ui| {
                mono_input(ui, label, value, editor_width);
            });
        },
    );
}

fn noise_domain_row(
    ui: &mut Ui,
    label: &str,
    id_namespace: &str,
    hint: &str,
    names: &[String],
    custom_choice: &str,
    value: &mut String,
) {
    if uses_two_column_fields(ui) {
        field_cell(ui, label, Some(hint), |ui| {
            noise_domain_control(ui, label, id_namespace, names, custom_choice, value);
        });
    } else {
        full_width_field(
            ui,
            label,
            Some(hint),
            Tokens::get(ui.ctx()).metrics.ctl_h,
            |ui| noise_domain_control(ui, label, id_namespace, names, custom_choice, value),
        );
    }
}

fn noise_sweep_control(ui: &mut Ui, sweep: &mut NoiseSweepType, explicit_frequencies: &mut String) {
    ui.spacing_mut().item_spacing.x = ENVELOPE_INLINE_CONTROL_GAP;
    let selected = sweep.selection_index();
    let current = selected
        .and_then(|index| NOISE_SWEEP_CHOICES.get(index))
        .copied()
        .unwrap_or("Schema unavailable");
    let options = NOISE_SWEEP_CHOICES.map(str::to_owned);
    let width = ui.available_width();
    let explicit = matches!(sweep, NoiseSweepType::ExplicitFrequencyList);
    let (selector_width, editor_width) = noise_sweep_control_widths(width);
    let salt = format!("analysis-noise-sweep-{}", ui.id().value());
    if let Some(index) = select_mono_with_response(
        ui,
        &salt,
        NOISE_FIELD_LABELS[0],
        current,
        &options,
        selector_width,
    )
    .picked
    {
        *sweep = NoiseSweepType::from_selection_index(index);
    }
    ui.add_enabled_ui(explicit, |ui| {
        mono_input(
            ui,
            NOISE_FIELD_LABELS[0],
            explicit_frequencies,
            editor_width,
        )
        .on_hover_text(if explicit {
            "Comma- or space-separated frequencies in Hz"
        } else {
            "Select Explicit frequency list to edit this retained axis"
        });
    });
}

fn noise_sweep_control_widths(available_width: f32) -> (f32, f32) {
    let content_width =
        (available_width - ENVELOPE_INLINE_CONTROL_GAP).max(NOISE_SWEEP_CONTROL_COUNT as f32);
    let selector_width = content_width * 0.44;
    (selector_width, content_width - selector_width)
}

fn noise_sweep_row(ui: &mut Ui, sweep: &mut NoiseSweepType, explicit_frequencies: &mut String) {
    if uses_two_column_fields(ui) {
        field_cell(
            ui,
            NOISE_FIELD_LABELS[0],
            Some("domain constrained"),
            |ui| {
                ui.horizontal(|ui| noise_sweep_control(ui, sweep, explicit_frequencies));
            },
        );
    } else {
        full_width_field(
            ui,
            NOISE_FIELD_LABELS[0],
            Some("domain constrained"),
            Tokens::get(ui.ctx()).metrics.ctl_h,
            |ui| ui.horizontal(|ui| noise_sweep_control(ui, sweep, explicit_frequencies)),
        );
    }
}

fn choice_row_with_disabled(
    ui: &mut Ui,
    label: &str,
    options: &[&str],
    value: &mut usize,
    disabled: &[(usize, &'static str)],
) -> bool {
    if !uses_two_column_fields(ui) {
        let options = options
            .iter()
            .map(|option| (*option).to_owned())
            .collect::<Vec<_>>();
        let current = options
            .get(*value)
            .map_or("Schema unavailable", String::as_str);
        let salt = format!(
            "analysis-field-disabled-stacked-{}-{label}",
            ui.id().value()
        );
        return full_width_field(
            ui,
            label,
            Some("domain constrained"),
            Tokens::get(ui.ctx()).metrics.ctl_h,
            |ui| {
                if let Some(index) = select_with_disabled(
                    ui,
                    &salt,
                    label,
                    current,
                    &options,
                    disabled,
                    ui.available_width(),
                ) {
                    *value = index;
                    true
                } else {
                    false
                }
            },
        );
    }
    field_cell(ui, label, Some("domain constrained"), |ui| {
        let options = options
            .iter()
            .map(|option| (*option).to_owned())
            .collect::<Vec<_>>();
        let current = options
            .get(*value)
            .map_or("Schema unavailable", String::as_str);
        let salt = format!("analysis-field-disabled-{}-{label}", ui.id().value());
        if let Some(index) = select_with_disabled(
            ui,
            &salt,
            label,
            current,
            &options,
            disabled,
            ui.available_width(),
        ) {
            *value = index;
            true
        } else {
            false
        }
    })
}

fn op_temperature_row(ui: &mut Ui, setup: &mut crate::simulation::dialog::OpDialogState) {
    if !uses_two_column_fields(ui) {
        let control_height = Tokens::get(ui.ctx()).metrics.ctl_h * 2.0 + 6.0;
        full_width_field(
            ui,
            OP_FIELD_LABELS[0],
            Some("Celsius"),
            control_height,
            |ui| {
                let current = OP_TEMPERATURE_CHOICES
                    .get(setup.temperature_mode_idx)
                    .copied()
                    .unwrap_or("Schema unavailable");
                if let Some(index) = select(
                    ui,
                    "op-temperature-mode-stacked",
                    OP_FIELD_LABELS[0],
                    current,
                    &OP_TEMPERATURE_CHOICES.map(|value| value.to_owned()),
                    ui.available_width(),
                ) {
                    setup.temperature_mode_idx = index;
                    if index == 1 {
                        setup.temperature = "27".to_owned();
                    }
                }
                ui.add_enabled_ui(setup.temperature_mode_idx == 2, |ui| {
                    mono_input(
                        ui,
                        OP_FIELD_LABELS[0],
                        &mut setup.temperature,
                        ui.available_width(),
                    )
                });
            },
        );
        return;
    }
    field_cell(ui, OP_FIELD_LABELS[0], Some("Celsius"), |ui| {
        let input_width = 92.0;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let select_width = (ui.available_width() - input_width - 6.0).max(1.0);
            let current = OP_TEMPERATURE_CHOICES
                .get(setup.temperature_mode_idx)
                .copied()
                .unwrap_or("Schema unavailable");
            if let Some(index) = select(
                ui,
                "op-temperature-mode",
                OP_FIELD_LABELS[0],
                current,
                &OP_TEMPERATURE_CHOICES.map(|value| value.to_owned()),
                select_width,
            ) {
                setup.temperature_mode_idx = index;
                if index == 1 {
                    setup.temperature = "27".to_owned();
                }
            }
            ui.add_enabled_ui(setup.temperature_mode_idx == 2, |ui| {
                mono_input(ui, OP_FIELD_LABELS[0], &mut setup.temperature, input_width)
            });
        });
    });
}

fn mono_input_with_suffix(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    suffix: &'static str,
) -> Response {
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
            let response = mono_input(ui, label, value, input_width);
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

/// Rewrite a quantity field on focus loss, but only where the operator's
/// spelling and the schema's reading of it are two different numbers.
///
/// The one rule for every quantity row on this form. A field holds the draft
/// text the deck is generated from, so what it says has to mean the same thing
/// to `parse_si_value` — which is what the controller reads it back with — as
/// it did to the reader who typed it. Where it already does, and `5ms`, `2u`
/// and `1e-3` all do, the spelling belongs to the operator and nothing here
/// touches it.
///
/// Where it does not, the field is rewritten, because a field that reads one
/// value and runs another is the worse failure. Three spellings this form
/// accepts are not deck spellings: a comma decimal separator under a locale
/// that allows one, a temperature carrying its unit (`25 °C`, `77 °F` — the
/// draft is bare Celsius), and an angular frequency in `rad/s`. Those are
/// written back through [`format_engineering`], the deck's own formatter.
///
/// The rule used to be "rewrite always", so typing `5ms` into Stop time and
/// pressing Tab left `5.00000000000000010e-3` behind while `2u` beside it was
/// untouched: a field that could not be relied on to hold what was typed into
/// it, in the one notation the row's own helper text advertises.
fn normalize_quantity_on_focus_loss(
    response: &Response,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    if response.lost_focus() {
        normalize_quantity(value, kind, policy, locale);
    }
}

/// The rule itself, without the focus it is applied on.
///
/// Separated so it can be asked directly: whether a spelling survives is a
/// question about two parsers, and the cases worth pinning — a unit the deck
/// cannot read, a value it can — are not all reachable by typing into a
/// rendered field. Returns whether `value` was rewritten.
pub(super) fn normalize_quantity(
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> bool {
    let Ok(parsed) = parse_ui_quantity(value, kind, policy, locale) else {
        // A field that does not parse keeps what was typed. The form states the
        // refusal beside it, and rewriting the text would take away the thing
        // the reader has to correct.
        return false;
    };
    let schema_value = if kind == QuantityInputKind::Temperature {
        parsed - 273.15
    } else {
        parsed
    };
    if schema_reads(value, schema_value) {
        return false;
    }
    *value = format_engineering(schema_value);
    true
}

/// Whether a draft reader would get `schema_value` back out of `text`.
///
/// Two readers, because two of them are in the controller: an analysis's
/// numeric drafts reach the engine through
/// [`crate::simulation::spice_value::parse_spice_value_checked`] on most paths
/// and through [`crate::simulation::dialog::parse_si_value`] on the rest, and
/// the two do not accept exactly the same spellings — `x` and `µ` are SI-side
/// only, `gig` and `tera` are SPICE-side. Asking both is the fail-closed
/// reading: a spelling is kept only where nothing downstream could read it as
/// something else.
///
/// Compared within a relative tolerance rather than by bits: each parser
/// reaches the number by its own arithmetic, and a field rewritten because two
/// of them rounded differently in the last place would be exactly the churn
/// this exists to stop.
fn schema_reads(text: &str, schema_value: f64) -> bool {
    let agrees = |read: f64| {
        read == schema_value || {
            let scale = read.abs().max(schema_value.abs());
            (read - schema_value).abs() <= scale * 1e-12
        }
    };
    crate::simulation::spice_value::parse_spice_value_checked(text).is_ok_and(agrees)
        && crate::simulation::dialog::parse_si_value(text).is_ok_and(agrees)
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

fn envelope_harmonic_order_row(ui: &mut Ui, value: &mut String) -> Response {
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

fn named_periodic_source_row(
    ui: &mut Ui,
    label: &str,
    id_namespace: &str,
    value: &mut String,
    circuit_sources: &[String],
) {
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
                let salt = format!("analysis-{id_namespace}-source-{}", ui.id().value());
                if let Some(index) =
                    select_mono_with_response(ui, &salt, label, current, &options, selector_width)
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
                    mono_input(ui, label, value, editor_width);
                } else {
                    ui.allocate_exact_size(vec2(editor_width, 1.0), egui::Sense::hover());
                }
            },
        );
    };

    if uses_two_column_fields(ui) {
        field_cell(ui, label, Some("domain constrained"), add_control);
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
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                color,
            );
            add_control(ui);
        },
    );
}

fn envelope_modulation_source_row(ui: &mut Ui, value: &mut String, circuit_sources: &[String]) {
    named_periodic_source_row(
        ui,
        ENVELOPE_FIELD_LABELS[4],
        "envelope-modulation",
        value,
        circuit_sources,
    );
}

fn switch_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    if !uses_two_column_fields(ui) {
        return inspector_switch_row(ui, label, value);
    }
    field_cell(ui, label, Some("domain constrained"), |ui| {
        let row_size = vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h);
        ui.allocate_ui_with_layout(row_size, Layout::left_to_right(Align::Center), |ui| {
            // The cell owns the full grid column, but the switch keeps its
            // natural compact width at the leading edge. `add_sized` would
            // center its contents across an oversized half-column.
            //
            // Bare, because `field_cell` has already painted the caption over
            // it. What it announces is that caption and never its own state:
            // the tick box this replaced was named by the word it was showing,
            // so it said "Enabled" and a reader had no way to tell which of a
            // form's booleans they had reached.
            super::page_kit::switch_cell(ui, label, value).changed()
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
    ui.add(egui::Label::new(job));
}

/// A full-width line of prose under the rows it is about.
///
/// The two-column field grid has no cell for prose, so this leaves the grid the
/// way [`sub_header`] does and takes the whole width; the text wraps rather
/// than being clipped, because a sentence that is cut off is worse than none.
fn field_prose(ui: &mut Ui, text: &str, color: egui::Color32) {
    clear_pending_cell(ui);
    ui.add_space(2.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(color),
        )
        .wrap(),
    );
}

/// A full-width advisory: something here will refuse the run.
fn field_advisory(ui: &mut Ui, text: &str) {
    field_prose(ui, text, Tokens::get(ui.ctx()).color.warn);
}

/// A full-width note: something the design holds, which the reader may take or
/// leave.
///
/// Not [`field_advisory`], and the colour is the difference. Painting a fact in
/// the warning colour teaches a reader to stop believing the warning colour.
fn field_note(ui: &mut Ui, text: &str) {
    field_prose(ui, text, Tokens::get(ui.ctx()).color.text_dim);
}

/// A full-width ghost add/remove action line. Returns `true` on click.
fn action_line(ui: &mut Ui, label: &str) -> bool {
    action_line_enabled(ui, label, true)
}

/// The same line, offered or withheld.
///
/// A disabled line still announces itself, so the reason it is disabled has to
/// be painted beside it rather than left to the greying.
fn action_line_enabled(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    clear_pending_cell(ui);
    Button::new(label)
        .ghost()
        .enabled(enabled)
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
    input_row(ui, sweep_point_field_label(sweep.sweep), &mut sweep.points);
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
    switch_row(ui, "Mixed-mode matrix", &mut setup.mixed_mode);
    switch_row(ui, "Noise parameters", &mut setup.noise_parameters);
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

/// The S-parameter run's ports, from whichever of the two declarations owns
/// them. Returns the form's note.
///
/// A port is a Z0 plane the run drives and measures, and a design can declare
/// one in two places: `RF Port` components on the sheet, which the netlist
/// carries as `P` cards, or node pairs typed here, which the runner
/// materializes only for a deck that declares none of its own
/// (`services::simulation_runner::sparameter::resolve_ports`). Both at once is
/// not a richer setup, it is two answers to one question — so this row picks
/// the owner, and the other declaration goes quiet rather than half-applying.
fn sp_port_fields(
    ui: &mut Ui,
    setup: &mut crate::simulation::dialog::SpDialogState,
    placed: &[crate::simulation::placed_sources::PlacedRfPort],
) -> &'static str {
    use crate::simulation::dialog::SpPortSource;

    let labels: Vec<&str> = SpPortSource::ALL
        .iter()
        .map(|source| source.display_name())
        .collect();
    let resolved = setup.port_source(placed.len());
    let mut selected = resolved.index();
    choice_row(ui, "Ports", &labels, &mut selected);
    // Written back only when the reader actually moves it. Stamping the
    // resolved value every frame would record a choice on a project that never
    // made one, and pin it to whatever the sheet happened to hold the first
    // time this form was drawn.
    if selected != resolved.index() {
        setup.port_source_idx = Some(selected);
    }

    let source = setup.port_source(placed.len());
    if source == SpPortSource::Placed {
        // Read-only: these are the design's, and an editable copy of them here
        // is the second declaration this switch exists to remove.
        for port in placed {
            sub_header(ui, &format!("Port {}", port.port_number));
            property_row(ui, "Instance", &port.reference);
            property_row(ui, "Role", &port.summary());
        }
    } else {
        let mut remove: Option<usize> = None;
        let port_count = setup.ports.len();
        for (idx, port) in setup.ports.iter_mut().enumerate() {
            sub_header(ui, &format!("Port {}", idx + 1));
            input_row(ui, "Node +", &mut port.node_pos);
            switch_row(ui, "Differential", &mut port.differential);
            if port.differential {
                input_row(ui, "Node −", &mut port.node_neg);
            }
            switch_row(ui, "Z0 override", &mut port.z0_override);
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
    }

    // The same resolution dispatch performs, stated here so the reason a run
    // will be refused is visible beside the ports it is about.
    if let Some(reason) = setup.port_roster_error(placed) {
        field_advisory(ui, &reason);
    }

    match source {
        SpPortSource::Placed => {
            "Scattering parameters between the RF ports the design places. Their P cards are \
             the ports the run drives and measures; nothing is added to the netlist for it."
        }
        SpPortSource::AdHoc => {
            "Scattering parameters between the node pairs named here. Each becomes a generator \
             behind its reference impedance, which only a deck that declares no ports of its \
             own accepts."
        }
    }
}

/// The design's own answer to the transfer-function form's two ports.
///
/// A pre-fill, not a run. Pressing this writes the deck's only independent
/// source and the node that source does not connect to into the two fields
/// above, where both stay editable and the reader still presses Run — which is
/// the whole reason a positional guess at the output is acceptable here and was
/// not acceptable in the PAC, PXF and PNOISE runners that ran on one.
///
/// When the deck names no single obvious pair the action is offered and
/// refused, with the reason in its place: a button whose only answer is a
/// refusal teaches nothing, and a button that has silently vanished teaches
/// less.
///
/// The note retires itself. It says what the deck offers only while the form
/// does not already say it, so a reader who has pressed the action — or typed
/// the same two names — is not told a third time.
fn xf_inference_action(
    ui: &mut Ui,
    setup: &mut crate::simulation::dialog::XfDialogState,
    inference: Option<&Result<TfRunConfig, String>>,
) {
    // `None` is "not measured", which only a caller that painted this form
    // without resolving a design can produce. There is nothing honest to say
    // about a deck nobody read.
    let Some(inference) = inference else {
        return;
    };
    match inference {
        Ok(config) => {
            if action_line(ui, XF_INFER_LABEL) {
                setup.input_source.clone_from(&config.input_source);
                setup
                    .output_expression
                    .clone_from(&config.output_expression);
            }
            if setup.input_source != config.input_source
                || setup.output_expression != config.output_expression
            {
                field_note(
                    ui,
                    &format!(
                        "This design offers input {} and output {}.",
                        config.input_source, config.output_expression
                    ),
                );
            }
        }
        Err(reason) => {
            action_line_enabled(ui, XF_INFER_LABEL, false);
            field_advisory(ui, reason);
        }
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
    placed_loop_probes: &[String],
    // The RF ports the design places, resolved once by the caller. The
    // S-parameter form reads these rather than keeping a copy: an editable
    // second declaration of the same ports is what this list replaces.
    placed_rf_ports: &[crate::simulation::placed_sources::PlacedRfPort],
    noise_domain: NoiseDomain<'_>,
    // What the design deck offers the transfer-function form's two ports, or
    // why it offers nothing. Resolved once per design by the caller, and
    // `None` for every analysis that is not a transfer function.
    tf_inference: Option<&Result<TfRunConfig, String>>,
    op_context: OpContextAvailability,
    run_space: &run_space::RunSpaceContext<'_>,
    route: &mut Option<crate::workbench::state::SimulationPage>,
) -> &'static str {
    clear_pending_cell(ui);
    ui.spacing_mut().item_spacing.y = FIELD_ROW_GAP;
    let note = match draft {
        AnalysisDraft::OperatingPoint(setup) => {
            setup.ensure_initialized();
            op_temperature_row(ui, setup);
            let initial_guess_disabled =
                op_initial_guess_disabled(setup.node_initialization_idx, op_context.previous_state);
            choice_row_with_disabled(
                ui,
                OP_FIELD_LABELS[1],
                &OP_INITIAL_GUESS_CHOICES,
                &mut setup.initial_guess_idx,
                &initial_guess_disabled,
            );
            let node_initialization_disabled =
                op_node_initialization_disabled(setup.initial_guess_idx);
            choice_row_with_disabled(
                ui,
                OP_FIELD_LABELS[2],
                &OP_NODE_INITIALIZATION_CHOICES,
                &mut setup.node_initialization_idx,
                &node_initialization_disabled,
            );
            choice_row(
                ui,
                OP_FIELD_LABELS[3],
                &OP_HOMOTOPY_CHOICES,
                &mut setup.homotopy_idx,
            );
            choice_row(
                ui,
                OP_FIELD_LABELS[4],
                &OP_ANNOTATION_CHOICES,
                &mut setup.annotation_idx,
            );
            choice_row_with_disabled(
                ui,
                OP_FIELD_LABELS[5],
                &OP_DEVICE_DETAIL_CHOICES,
                &mut setup.device_detail_idx,
                if op_context.soa_violations {
                    &[]
                } else {
                    &[(
                        2,
                        "Run SOA checks with warning or violation evidence before selecting this policy",
                    )]
                },
            );
            choice_row(
                ui,
                OP_FIELD_LABELS[6],
                &OP_SAVE_DEVICE_CHOICES,
                &mut setup.save_device_op_idx,
            );
            choice_row(
                ui,
                OP_FIELD_LABELS[7],
                &OP_ACCURACY_CHOICES,
                &mut setup.accuracy_idx,
            );
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
            switch_row(ui, "Use initial conditions", &mut setup.uic);
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
            input_row(ui, sweep_point_field_label(setup.sweep), &mut setup.points);
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep);
            "Small-signal sweep around the operating point."
        }
        AnalysisDraft::DcSweep(setup) => dc_sweep::fields(ui, setup),
        AnalysisDraft::Noise(setup) => {
            noise_sweep_row(ui, &mut setup.sweep, &mut setup.explicit_frequencies);
            let fixed_grid = !matches!(setup.sweep, NoiseSweepType::ExplicitFrequencyList);
            input_row_enabled(
                ui,
                noise_point_field_label(setup.sweep),
                &mut setup.points,
                fixed_grid,
            );
            quantity_input_row_enabled(
                ui,
                NOISE_FIELD_LABELS[2],
                &mut setup.fstart,
                QuantityInputKind::Frequency,
                policy,
                locale,
                fixed_grid,
            );
            quantity_input_row_enabled(
                ui,
                NOISE_FIELD_LABELS[3],
                &mut setup.fstop,
                QuantityInputKind::Frequency,
                policy,
                locale,
                fixed_grid,
            );
            let offered_nodes = noise_domain
                .nodes
                .get(..NOISE_DOMAIN_PRESET_LIMIT)
                .unwrap_or(noise_domain.nodes);
            let offered_sources = noise_domain
                .sources
                .get(..NOISE_DOMAIN_PRESET_LIMIT)
                .unwrap_or(noise_domain.sources);
            let previous_output = setup.output.clone();
            noise_domain_row(
                ui,
                NOISE_FIELD_LABELS[4],
                "output",
                &noise_domain_hint(
                    "nodes",
                    offered_nodes.len(),
                    noise_domain.nodes.len(),
                    noise_domain.unavailable.is_some(),
                ),
                offered_nodes,
                NOISE_OUTPUT_CUSTOM_CHOICE,
                &mut setup.output,
            );
            if setup.output != previous_output {
                // Once the exact output-expression field is edited it owns
                // both nodes; a hidden legacy reference must not leak into it.
                setup.reference = "0".to_owned();
            }
            noise_domain_row(
                ui,
                NOISE_FIELD_LABELS[5],
                "input",
                &noise_domain_hint(
                    "sources",
                    offered_sources.len(),
                    noise_domain.sources.len(),
                    noise_domain.unavailable.is_some(),
                ),
                offered_sources,
                NOISE_INPUT_CUSTOM_CHOICE,
                &mut setup.input,
            );
            if let Some(reason) = noise_domain.unavailable {
                field_advisory(
                    ui,
                    &format!(
                        "The elaborated node and source lists are unavailable: {reason}. Both \
                         fields still take a name typed in full, and the run checks it against \
                         the design before it starts."
                    ),
                );
            }
            if let Some(selection) = noise_enum_choice_row(
                ui,
                NOISE_FIELD_LABELS[6],
                &NOISE_CONTRIBUTION_CHOICES,
                setup.contribution_detail.selection_index(),
            ) && let Some(detail) = NoiseContributionDetail::from_selection_index(selection)
            {
                setup.contribution_detail = detail;
            }
            if let Some(selection) = noise_enum_choice_row(
                ui,
                NOISE_FIELD_LABELS[7],
                &NOISE_INTEGRATION_CHOICES,
                setup.integration_mode.selection_index(),
            ) && let Some(mode) = NoiseIntegrationMode::from_selection_index(selection)
            {
                setup.integration_mode = mode;
            }
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
            "Sensitivity of the output to every parameter the circuit exposes."
        }
        AnalysisDraft::MonteCarlo(setup) => {
            use crate::simulation::dialog::McVariationSource;

            input_row(ui, "Samples", &mut setup.num_runs);
            input_row(ui, "Seed", &mut setup.seed);
            choice_row(
                ui,
                "From",
                &["parameters", "deck"],
                &mut setup.variation_source_idx,
            );
            // The spread and its shape belong to the parameter-tolerance
            // source. Under deck statistics the deck states its own spread, so
            // these two rows would be read by nothing.
            let states_spread = McVariationSource::ALL
                .get(setup.variation_source_idx)
                .copied()
                .unwrap_or_default()
                .uses_stated_spread();
            input_row_enabled(ui, "Spread %", &mut setup.variation_pct, states_spread);
            choice_row_with_disabled(
                ui,
                "Vary",
                &["gauss", "uniform", "worst"],
                &mut setup.distribution_idx,
                &if states_spread {
                    Vec::new()
                } else {
                    (0..3)
                        .map(|index| (index, "the deck states its own distribution"))
                        .collect::<Vec<_>>()
                },
            );
            if states_spread {
                "Each trial perturbs the eligible parameters and solves an operating \
                 point; the result is the distribution of the node voltages."
            } else {
                "Each trial redraws the deck's own agauss/gauss/unif expressions, model \
                 cards included, and solves an operating point."
            }
        }
        AnalysisDraft::Pss(setup) => {
            pss::fields(ui, setup, envelope_modulation_sources, policy, locale)
        }
        AnalysisDraft::Stb(setup) => {
            stb_probe::row(
                ui,
                placed_loop_probes,
                &mut setup.probe_source,
                &mut setup.probe_reference,
            );
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
            input_row(
                ui,
                sweep_point_field_label(setup.sweep_type_idx),
                &mut setup.num_points,
            );
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            switch_row(ui, "Nyquist contour", &mut setup.compute_nyquist);
            "Loop gain via the probe source. Gain margin, phase margin and \
             crossover are always extracted and reported as measurements."
        }
        AnalysisDraft::Temperature(setup) => {
            // The plan may already declare a temperature axis. This instance
            // either reads it or states its own, and says which in place —
            // because two temperature declarations that silently disagree is
            // the same defect the corner form above was built to stop.
            run_space::temperature_form(ui, setup, run_space, route, policy, locale)
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
            switch_row(ui, "Source stepping", &mut setup.source_stepping);
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
            input_row(
                ui,
                sweep_point_field_label(setup.sweep_type_idx),
                &mut setup.num_points,
            );
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Z0", &mut setup.z0);
            switch_row(ui, "Noise parameters", &mut setup.do_noise);
            switch_row(ui, "Touchstone export", &mut setup.touchstone_export);
            sp_port_fields(ui, setup, placed_rf_ports)
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
            input_row(
                ui,
                sweep_point_field_label(setup.sweep_type_idx),
                &mut setup.num_points,
            );
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Magnitude", &mut setup.pac_magnitude);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            switch_row(ui, "Include DC", &mut setup.include_dc);
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
            input_row(
                ui,
                sweep_point_field_label(setup.sweep_type_idx),
                &mut setup.num_points,
            );
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
            switch_row(ui, "Integrated noise", &mut setup.integrated_noise);
            switch_row(ui, "Noise summary", &mut setup.noise_summary);
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
            input_row(
                ui,
                sweep_point_field_label(setup.sweep_type_idx),
                &mut setup.num_points,
            );
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Out sideband", &mut setup.output_sideband);
            input_row(ui, "Input src", &mut setup.input_source);
            input_row(ui, "Max sideband", &mut setup.max_sideband);
            "Transfer functions onto a periodic steady state (needs PSS)."
        }
        AnalysisDraft::Pstb(setup) => {
            // The same element STB designates, chosen the same way. It was
            // free text here long after the stability form's stopped being
            // one, so a probe name that matched nothing on the drawing failed
            // in the solver instead of being refused by name.
            stb_probe::row(
                ui,
                placed_loop_probes,
                &mut setup.probe,
                &mut setup.probe_reference,
            );
            input_row(ui, "Harmonics", &mut setup.max_harmonics);
            input_row(ui, "Multipliers", &mut setup.num_multipliers);
            engineering_input_row(ui, "Unstable above", &mut setup.stability_threshold);
            engineering_input_row(ui, "Eigen tol", &mut setup.eigenvalue_tolerance);
            switch_row(ui, "Detect subharmonics", &mut setup.detect_subharmonics);
            "Loop stability around the periodic steady state (needs PSS). \
             Margins are always extracted from the Floquet multipliers."
        }
        AnalysisDraft::TransferFunction(setup) => {
            input_row(ui, XF_FIELD_LABELS[0], &mut setup.input_source);
            input_row(ui, XF_FIELD_LABELS[1], &mut setup.output_expression);
            xf_inference_action(ui, setup, tf_inference);
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
            // The run space has one editor — PVT, sweeps & variation — and one
            // owner, the plan. This form reads that declaration; it does not
            // keep one. A second set of axis controls here, or a second copy
            // behind them, would be a second owner of the same fact, and the
            // two would eventually disagree about how many points run.
            run_space::corner_form(ui, &mut setup.base_analysis_idx, run_space, route)
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
            switch_row(ui, "Compute THD", &mut setup.compute_thd);
            switch_row(ui, "Normalize", &mut setup.normalize);
            "Fourier components of a transient waveform window."
        }
        AnalysisDraft::Reliability(setup) => {
            input_row(ui, "Years", &mut setup.years_csv);
            input_row(ui, "Min stress V", &mut setup.min_stress_voltage);
            switch_row(ui, "Hot carrier (HCI)", &mut setup.enable_hci);
            switch_row(ui, "Bias instability (NBTI)", &mut setup.enable_nbti);
            switch_row(ui, "Electromigration", &mut setup.enable_em);
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
            switch_row(ui, "Check Vgs", &mut setup.check_vgs_max);
            input_row_enabled(ui, "Max Vgs", &mut setup.max_vgs, setup.check_vgs_max);
            switch_row(ui, "Check Vds", &mut setup.check_vds_max);
            input_row_enabled(ui, "Max Vds", &mut setup.max_vds, setup.check_vds_max);
            switch_row(ui, "Check Vbe", &mut setup.check_vbe_max);
            input_row_enabled(ui, "Max Vbe", &mut setup.max_vbe, setup.check_vbe_max);
            switch_row(ui, "Check Vce", &mut setup.check_vce_max);
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
            input_row(
                ui,
                sweep_point_field_label(setup.sweep.sweep),
                &mut setup.sweep.points,
            );
            choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep.sweep);
            input_row(ui, "f2/f1", &mut setup.f2_over_f1);
            "Volterra harmonic and intermodulation distortion from source DISTOF1/DISTOF2 excitations; f2/f1 must be between 0 and 1, or empty for single-tone."
        }
        AnalysisDraft::Qpss(setup) => {
            input_row(ui, "Tone frequencies", &mut setup.tones);
            input_row(ui, "Harmonic orders", &mut setup.harmonics);
            input_row(ui, "Max iterations", &mut setup.max_iterations);
            input_row(ui, "Relative tolerance", &mut setup.relative_tolerance);
            switch_row(ui, "Autonomous oscillator", &mut setup.autonomous);
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
            switch_row(ui, "Integrated noise", &mut setup.integrated_noise);
            switch_row(ui, "Noise figure", &mut setup.noise_figure);
            switch_row(ui, "Contributor ranking", &mut setup.contributor_ranking);
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
            switch_row(ui, "Integrated noise", &mut setup.integrated_noise);
            switch_row(ui, "Contributor ranking", &mut setup.contributor_ranking);
            "Noise folding across a multi-tone spectral lattice (needs QPSS)."
        }
        AnalysisDraft::Qpxf(setup) => {
            frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
            input_row(ui, "Input source", &mut setup.input_source);
            input_row(ui, "Output", &mut setup.output_node);
            input_row(ui, "Output ref", &mut setup.output_ref);
            input_row(ui, "Input lattice", &mut setup.input_lattice);
            input_row(ui, "Output lattice", &mut setup.output_lattice);
            switch_row(ui, "Group delay", &mut setup.group_delay);
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
            switch_row(
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
            switch_row(ui, "Process variation", &mut setup.include_process);
            switch_row(ui, "Local mismatch", &mut setup.include_mismatch);
            switch_row(
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
mod tests;
