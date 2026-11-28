//! The rows that pick a name from the design: noise output and input presets from
//! the elaborated nodes and sources, and the periodic source a form modulates.

use super::*;

pub(super) const NOISE_OUTPUT_CUSTOM_CHOICE: &str = "Custom expression\u{2026}";
pub(super) const NOISE_INPUT_CUSTOM_CHOICE: &str = "Custom source name\u{2026}";
/// How many elaborated names one noise row offers as presets.
///
/// The select paints one row per option with no scroll of its own, so a design
/// with thousands of nodes would open a popup taller than the screen. The
/// presets are a shortcut, not the domain: the exact-entry field accepts
/// anything the design contains, and the row states how much of the design it
/// is showing whenever it shows less than all of it.
pub(super) const NOISE_DOMAIN_PRESET_LIMIT: usize = 64;

const ENVELOPE_DECLARED_SOURCES_CHOICE: &str = "Declared list...";
const NOISE_SWEEP_CONTROL_COUNT: usize = 2;

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
pub(crate) struct NoiseDomain<'a> {
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

/// What a noise domain row is offering, stated in the row itself.
///
/// `offered` is what the select paints, `total` what the elaborated design
/// carries. Saying "design nodes" while showing 64 of 812 of them would be the
/// same fabrication in a smaller font, so a truncated list says so.
pub(super) fn noise_domain_hint(
    kind: &str,
    offered: usize,
    total: usize,
    unavailable: bool,
) -> String {
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

/// What a form says when the elaborated vocabulary could not be measured.
///
/// The reason travels with the fact: a form that says a vocabulary is
/// unavailable and not why leaves the reader with nothing to go and fix.
/// Noise and the transfer function ask for the same two quantities from the
/// same catalog, so they state its absence in the same words rather than in
/// two sentences that would drift apart.
pub(super) fn noise_domain_advisory(ui: &mut Ui, reason: &str) {
    field_advisory(
        ui,
        &format!(
            "The elaborated node and source lists are unavailable: {reason}. Both \r
             fields still take a name typed in full, and the run checks it against \r
             the design before it starts."
        ),
    );
}

pub(super) fn noise_domain_row(
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

pub(super) fn noise_sweep_control_widths(available_width: f32) -> (f32, f32) {
    let content_width =
        (available_width - ENVELOPE_INLINE_CONTROL_GAP).max(NOISE_SWEEP_CONTROL_COUNT as f32);
    let selector_width = content_width * 0.44;
    (selector_width, content_width - selector_width)
}

fn envelope_modulation_control_widths(available_width: f32) -> (f32, f32) {
    let content_width = (available_width - ENVELOPE_INLINE_CONTROL_GAP).max(2.0);
    let selector_width = content_width * 0.58;
    (selector_width, content_width - selector_width)
}

/// Returns the selector's own response, so a caller with a constraint to state
/// about this field can hang it on the control rather than under the form.
pub(super) fn named_periodic_source_row(
    ui: &mut Ui,
    label: &str,
    id_namespace: &str,
    value: &mut String,
    circuit_sources: &[String],
) -> Response {
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
                let picker =
                    select_mono_with_response(ui, &salt, label, current, &options, selector_width);
                if let Some(index) = picker.picked {
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
                picker.response
            },
        )
        .inner
    };

    if uses_two_column_fields(ui) {
        return field_cell(ui, label, None, add_control);
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
