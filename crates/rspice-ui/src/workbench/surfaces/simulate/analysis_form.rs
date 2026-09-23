//! Per-analysis configuration forms for the Simulate right panel.
//!
//! Each form edits one typed [`AnalysisDraft`] owned by a stable analysis
//! instance. A form says nothing about what its kind *is* — the reader picked
//! the kind — so the only prose here belongs to a field: a constraint that
//! would refuse the run, or a consequence of the current configuration that
//! the controls do not state. Both are painted beside the control they are
//! about. Validation is rendered by the caller.

mod ac;
mod ac_data;
mod dc_mismatch;
mod dc_sweep;
mod disto;
mod envelope;
mod fourier;
mod harmonic_balance;
mod monte_carlo;
pub(super) use monte_carlo::checkpoint_sources;
mod noise;
mod operating_point;
mod optimization;
/// Kept a module of its own rather than a branch of [`form`]: an analysis's
/// advanced options are the same fields in the same grid, but which ones an
/// analysis owns is a question about the option catalogue, not about the kind's
/// own parameters. It is `pub(super)` so the route can draw them after the
/// form's own fields; the primitives it draws with stay private here.
pub(super) mod options;
mod pac;
mod periodic_network;
mod pnoise;
mod pole_zero;
mod pss;
mod pstb;
mod pxf;
mod quasi_periodic;
mod recorded_fft;
mod reliability;
mod run_space;
mod s_parameter;
mod sensitivity;
mod soa;
mod stb;
mod stb_probe;
mod sweep_point_label;
mod transfer_function;
mod transient;
mod transient_noise;

pub(super) use run_space::RunSpaceContext;
use sweep_point_label::{
    SWEEP_KINDS, SWEEP_POINT_NEUTRAL_LABEL, noise_point_field_label, sweep_point_field_label,
};

use egui::{Align, Layout, Rect, Response, Ui, UiBuilder, vec2};

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::services::simulation_runner::TfRunConfig;
use crate::simulation::plan::{AnalysisDraft, FrequencySweepDraft};
use crate::state::format_engineering;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, choice_row as inspector_choice_row, input_row as inspector_input_row, mono_input,
    select, select_mono_with_response, select_with_disabled, switch_row as inspector_switch_row,
};
use crate::workbench::design_system::property_row as inspector_property_row;

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

const XF_ENABLED_CHOICES: &[&str] = &["Enabled", "Disabled"];
/// The carrier positions the engine's `FROM=` keyword has, in its order.
///
/// Held here rather than built from [`PeriodicCarrier`] at paint time because
/// `choice_row` takes `&[&str]`, and one static list is what keeps the three
/// periodic small-signal forms offering the same words in the same order.
/// `the_periodic_carrier_row_offers_the_carriers_the_engine_has` pins it
/// against the enum.
const PERIODIC_CARRIER_CHOICES: &[&str] = &[
    "preceding solve",
    "periodic steady state",
    "harmonic balance",
];
const ENVELOPE_DECLARED_SOURCES_CHOICE: &str = "Declared list...";
const ENVELOPE_INLINE_CONTROL_GAP: f32 = 6.0;
const NOISE_SWEEP_CONTROL_COUNT: usize = 2;
const FIELD_COLUMN_GAP: f32 = 14.0;
const FIELD_ROW_GAP: f32 = 10.0;
const FIELD_LABEL_HEIGHT: f32 = 15.0;
/// Clear space between a field's caption and the helper on its right.
const FIELD_CAPTION_GAP: f32 = 8.0;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct OpContextAvailability {
    pub previous_state: bool,
    pub compatible_previous_state: bool,
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
    paint_field_caption(&cell, label_rect, label, helper);
    add_control(&mut cell)
}

/// A field's caption, and the helper the caption has to leave room for.
///
/// The helper never moves and never shortens: it is one or two words naming the
/// notation, the domain, or where the value came from, and a reader scanning a
/// column of fields needs them on one right edge. So the caption is what gives,
/// and it is elided rather than painted straight through — which is what a
/// painted caption did until an option field arrived carrying
/// `Truncation absolute bound · TIMEINT ABSTOL` into a 206-point cell and ran
/// its own text under the word beside it.
fn paint_field_caption(ui: &Ui, rect: Rect, label: &str, helper: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let mut caption_right = rect.right();
    if let Some(helper) = helper {
        let font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let width = ui
            .painter()
            .layout_no_wrap(helper.to_owned(), font.clone(), t.color.text_faint)
            .size()
            .x;
        ui.painter().text(
            rect.right_center(),
            egui::Align2::RIGHT_CENTER,
            helper,
            font,
            t.color.text_faint,
        );
        caption_right = rect.right() - width - FIELD_CAPTION_GAP;
    }
    super::page_kit::paint_text(
        ui,
        Rect::from_min_max(
            rect.min,
            egui::pos2(caption_right.max(rect.left()), rect.max.y),
        ),
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn full_width_field<R>(
    ui: &mut Ui,
    label: &str,
    helper: Option<&str>,
    control_height: f32,
    add_control: impl FnOnce(&mut Ui) -> R,
) -> R {
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
    paint_field_caption(&cell, label_rect, label, helper);
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

/// A well whose hint slot states a rule rather than the notation.
///
/// Most numeric fields have nothing to say in that slot but which notation
/// they take, and [`engineering_input_row`] says exactly that. A few have a
/// rule the label cannot carry and the value alone does not imply — a window
/// that overrides the count above it, a factor with a closed range, a control
/// only one mode reads — and for those the rule is worth more in the slot than
/// the notation is. Both accept the same spellings either way.
fn hinted_input_row(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, Some(hint), |ui| {
        mono_input(ui, label, value, ui.available_width())
    })
}

/// The same field, offered or withheld by the mode that reads it.
fn hinted_input_row_enabled(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    enabled: bool,
) -> Response {
    if !uses_two_column_fields(ui) {
        return ui
            .add_enabled_ui(enabled, |ui| inspector_input_row(ui, label, value))
            .inner;
    }
    field_cell(ui, label, Some(hint), |ui| {
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

/// A quantity row whose helper states something the label cannot.
///
/// The ordinary row spends its helper slot on `engineering notation`, which is
/// true of every quantity field on the page and is worth saying once a form
/// rather than once a row. A field where *leaving it empty* selects a
/// behaviour has a fact the reader cannot infer from the label, and this is
/// where that fact goes: the caption row's right edge, beside the name of the
/// thing it qualifies, rather than as a note under the control where it would
/// read as advice about the form.
fn hinted_quantity_input_row(
    ui: &mut Ui,
    label: &str,
    helper: &str,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Response {
    let response = if uses_two_column_fields(ui) {
        field_cell(ui, label, Some(helper), |ui| {
            mono_input(ui, label, value, ui.available_width())
        })
    } else {
        inspector_input_row(ui, label, value)
    };
    normalize_quantity_on_focus_loss(&response, value, kind, policy, locale);
    response
}

/// The same hinted row, offered or withheld by the mode that reads it.
///
/// A field whose empty value selects a behaviour keeps saying so while it is
/// greyed: the reason it is disabled and the meaning of leaving it blank are
/// two different facts, and a reader needs both.
fn hinted_quantity_input_row_enabled(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    helper: &str,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
    enabled: bool,
) -> Response {
    let response = if uses_two_column_fields(ui) {
        field_cell(ui, label, Some(helper), |ui| {
            ui.add_enabled_ui(enabled, |ui| {
                mono_input(ui, label, value, ui.available_width())
            })
            .inner
        })
    } else {
        ui.add_enabled_ui(enabled, |ui| inspector_input_row(ui, label, value))
            .inner
    };
    if enabled {
        normalize_quantity_on_focus_loss(&response, value, kind, policy, locale);
    }
    response
}

/// A domain selector, offered or withheld by the field that governs it.
fn choice_row_enabled(
    ui: &mut Ui,
    label: &str,
    options: &[&str],
    value: &mut usize,
    enabled: bool,
) -> bool {
    if !enabled {
        let mut unchanged = *value;
        return ui
            .add_enabled_ui(false, |ui| choice_row(ui, label, options, &mut unchanged))
            .inner;
    }
    choice_row(ui, label, options, value)
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

/// The carrier row the three periodic small-signal forms share.
///
/// One row, because the question is one question: which large-signal periodic
/// solve does this small signal sit on. The positions are the engine's own
/// `FROM=` vocabulary, and every one of them is selectable: each names a
/// prerequisite family the plan can bind and a runner this crate has. The
/// harmonic-balance position was painted disabled until the three runners took
/// a harmonic-balance operating point, which made the chooser state a
/// limitation of the Studio rather than of the analysis.
fn periodic_carrier_row(ui: &mut Ui, value: &mut usize) -> bool {
    choice_row(ui, "Carrier", PERIODIC_CARRIER_CHOICES, value)
}

/// A tolerance well whose emptiness selects the plan's own policy.
///
/// The hint slot is the whole of what distinguishes this from an ordinary
/// engineering field: a blank well here is not an omission, it is the
/// selection "whatever the Solver options channel states deck-wide", and the
/// slot says so in that channel's own words. A written value is this
/// analysis's, and the card then states it; the two cases read differently
/// because they are different, and a field that said `engineering notation`
/// in both would leave the reader to guess which one an empty well was.
fn plan_policy_tolerance_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    let hint = if value.trim().is_empty() {
        "plan policy"
    } else {
        "engineering notation"
    };
    hinted_input_row(ui, label, value, hint)
}

fn enabled_choice_row(ui: &mut Ui, label: &str, enabled: &mut bool) -> bool {
    let mut selected = usize::from(!*enabled);
    let changed = choice_row(ui, label, XF_ENABLED_CHOICES, &mut selected);
    if changed {
        *enabled = selected == 0;
    }
    changed
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

/// What a form says when the elaborated vocabulary could not be measured.
///
/// The reason travels with the fact: a form that says a vocabulary is
/// unavailable and not why leaves the reader with nothing to go and fix.
/// Noise and the transfer function ask for the same two quantities from the
/// same catalog, so they state its absence in the same words rather than in
/// two sentences that would drift apart.
fn noise_domain_advisory(ui: &mut Ui, reason: &str) {
    field_advisory(
        ui,
        &format!(
            "The elaborated node and source lists are unavailable: {reason}. Both \r
             fields still take a name typed in full, and the run checks it against \r
             the design before it starts."
        ),
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

fn noise_sweep_control_widths(available_width: f32) -> (f32, f32) {
    let content_width =
        (available_width - ENVELOPE_INLINE_CONTROL_GAP).max(NOISE_SWEEP_CONTROL_COUNT as f32);
    let selector_width = content_width * 0.44;
    (selector_width, content_width - selector_width)
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

fn envelope_modulation_control_widths(available_width: f32) -> (f32, f32) {
    let content_width = (available_width - ENVELOPE_INLINE_CONTROL_GAP).max(2.0);
    let selector_width = content_width * 0.58;
    (selector_width, content_width - selector_width)
}

/// Returns the selector's own response, so a caller with a constraint to state
/// about this field can hang it on the control rather than under the form.
fn named_periodic_source_row(
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

/// A switch whose hint slot says what turning it on produces.
///
/// [`switch_row`] fills that slot with "domain constrained", which is true of
/// every boolean and therefore says nothing about any particular one. A switch
/// whose whole point is an output the reader has to know where to look for —
/// the solver trace, which appears in the Console and nowhere else — says that
/// there instead.
fn hinted_switch_row(ui: &mut Ui, label: &str, value: &mut bool, hint: &str) -> bool {
    if !uses_two_column_fields(ui) {
        return inspector_switch_row(ui, label, value);
    }
    field_cell(ui, label, Some(hint), |ui| {
        let row_size = vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h);
        ui.allocate_ui_with_layout(row_size, Layout::left_to_right(Align::Center), |ui| {
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

/// Render the form for `draft`.
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
    study_bases: &[(crate::product::AnalysisInstanceId, String)],
    route: &mut Option<crate::workbench::state::SimulationPage>,
) {
    clear_pending_cell(ui);
    ui.spacing_mut().item_spacing.y = FIELD_ROW_GAP;
    match draft {
        AnalysisDraft::OperatingPoint(setup) => operating_point::fields(ui, setup, op_context),
        AnalysisDraft::Transient(setup) => transient::fields(ui, setup, policy, locale),
        AnalysisDraft::Ac(setup) => ac::fields(ui, setup, policy, locale),
        AnalysisDraft::DcSweep(setup) => dc_sweep::fields(ui, setup),
        AnalysisDraft::Noise(setup) => noise::fields(ui, setup, noise_domain, policy, locale),
        AnalysisDraft::PoleZero(setup) => pole_zero::fields(ui, setup),
        AnalysisDraft::Sensitivity(setup) => sensitivity::fields(ui, setup, policy, locale),
        AnalysisDraft::MonteCarlo(setup) => monte_carlo::fields(ui, setup, study_bases),
        AnalysisDraft::Pss(setup) => {
            pss::fields(ui, setup, envelope_modulation_sources, policy, locale)
        }
        AnalysisDraft::Stb(setup) => stb::fields(ui, setup, placed_loop_probes, policy, locale),
        // The plan may already declare a temperature axis. This instance
        // either reads it or states its own, and says which in place —
        // because two temperature declarations that silently disagree is
        // the same defect the corner form above was built to stop.
        AnalysisDraft::Temperature(setup) => {
            study_base_row(
                ui,
                &mut setup.base_analysis,
                study_bases,
                "Shared base settings (legacy)",
            );
            if setup.base_analysis.is_some() {
                field_note(
                    ui,
                    "Uses the selected analysis's settings at every temperature. Edit that analysis to change its configuration.",
                );
            }
            run_space::temperature_form(ui, setup, run_space, route, policy, locale)
        }
        AnalysisDraft::HarmonicBalance(setup) => {
            harmonic_balance::fields(ui, setup, policy, locale)
        }
        AnalysisDraft::SParameter(setup) => {
            s_parameter::fields(ui, setup, placed_rf_ports, policy, locale)
        }
        AnalysisDraft::Pac(setup) => pac::fields(ui, setup, policy, locale),
        AnalysisDraft::Pnoise(setup) => pnoise::fields(ui, setup, policy, locale),
        AnalysisDraft::Pxf(setup) => pxf::fields(ui, setup, policy, locale),
        AnalysisDraft::Pstb(setup) => pstb::fields(ui, setup, placed_loop_probes),
        AnalysisDraft::TransferFunction(setup) => {
            transfer_function::fields(ui, setup, noise_domain, tf_inference)
        }
        // The run space has one editor — PVT, sweeps & variation — and one
        // owner, the plan. This form reads that declaration; it does not
        // keep one. A second set of axis controls here, or a second copy
        // behind them, would be a second owner of the same fact, and the
        // two would eventually disagree about how many points run.
        AnalysisDraft::Corner(setup) => {
            study_base_row(
                ui,
                &mut setup.base_analysis,
                study_bases,
                "Shared base settings (legacy)",
            );
            if setup.base_analysis.is_some() {
                field_note(
                    ui,
                    "Uses the selected analysis's settings at every PVT point. Edit that analysis to change its configuration.",
                );
            }
            run_space::corner_form(ui, setup, run_space, route)
        }
        AnalysisDraft::Envelope(setup) => {
            envelope::fields(ui, setup, envelope_modulation_sources, policy, locale)
        }
        AnalysisDraft::Fourier(setup) => fourier::fields(ui, setup, policy, locale),
        AnalysisDraft::Reliability(setup) => reliability::fields(ui, setup),
        AnalysisDraft::Optimization(setup) => optimization::fields(ui, setup, study_bases),
        AnalysisDraft::Soa(setup) => soa::fields(ui, setup, policy, locale),
        AnalysisDraft::Disto(setup) => disto::fields(ui, setup, policy, locale),
        AnalysisDraft::Qpss(setup) => quasi_periodic::shooting_fields(ui, setup),
        AnalysisDraft::Hbsp(setup) => periodic_network::fields(ui, setup, policy, locale),
        AnalysisDraft::Hbnoise(setup) => periodic_network::noise_fields(ui, setup, policy, locale),
        AnalysisDraft::Psp(setup) => periodic_network::fields(ui, setup, policy, locale),
        AnalysisDraft::Qpac(setup) => quasi_periodic::ac_fields(ui, setup, policy, locale),
        AnalysisDraft::Qpnoise(setup) => quasi_periodic::noise_fields(ui, setup, policy, locale),
        AnalysisDraft::Qpxf(setup) => quasi_periodic::transfer_fields(ui, setup, policy, locale),
        AnalysisDraft::TransientNoise(setup) => transient_noise::fields(ui, setup, policy, locale),
        AnalysisDraft::DcMismatch(setup) => dc_mismatch::fields(ui, setup),
        AnalysisDraft::AcData(setup) => ac_data::fields(ui, setup),
        AnalysisDraft::Fft(setup) => recorded_fft::fields(ui, setup, policy, locale),
    }
    clear_pending_cell(ui);
}

#[cfg(test)]
mod tests;

fn study_base_row(
    ui: &mut Ui,
    selected_base: &mut Option<crate::product::AnalysisInstanceId>,
    bases: &[(crate::product::AnalysisInstanceId, String)],
    default_label: &str,
) {
    let mut choices = vec![(None, default_label.to_owned())];
    choices.extend(bases.iter().map(|(id, label)| (Some(*id), label.clone())));
    if let Some(id) = *selected_base
        && !choices.iter().any(|(candidate, _)| *candidate == Some(id))
    {
        choices.push((Some(id), format!("Unavailable analysis ({id})")));
    }
    let mut selected = choices
        .iter()
        .position(|(id, _)| *id == *selected_base)
        .unwrap_or(0);
    let labels = choices
        .iter()
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    if choice_row(ui, "Base analysis", &labels, &mut selected) {
        *selected_base = choices[selected].0;
    }
}
