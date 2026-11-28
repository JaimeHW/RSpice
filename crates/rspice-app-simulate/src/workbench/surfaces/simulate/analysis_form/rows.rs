//! The rows every analysis form is built from: the two-column field grid, input,
//! quantity, choice and switch rows, sub-headers, notes and action lines.

use super::*;

const XF_ENABLED_CHOICES: &[&str] = &["Enabled", "Disabled"];
const FIELD_COLUMN_GAP: f32 = 14.0;
const FIELD_LABEL_HEIGHT: f32 = 15.0;
/// Clear space between a field's caption and the helper on its right.
const FIELD_CAPTION_GAP: f32 = 8.0;

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

pub(super) fn clear_pending_cell(ui: &mut Ui) {
    let id = pending_cell_id(ui);
    ui.data_mut(|data| {
        data.remove_temp::<PendingCell>(id);
    });
}

pub(super) fn uses_two_column_fields(ui: &Ui) -> bool {
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

pub(super) fn field_cell<R>(
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

pub(super) fn full_width_field<R>(
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

/// The kit's mono well, under the Simulate commit rule: it holds what is
/// typed while it has the keyboard and commits when it lets go.
pub(super) fn mono_input(ui: &mut Ui, label: &str, value: &mut String, width: f32) -> Response {
    crate::workbench::surfaces::simulate::drafts::text_field(ui, label, |ui| {
        crate::ui::widgets::mono_input(ui, label, value, width)
    })
}

/// The kit's inspector row, its well under the Simulate commit rule.
pub(super) fn inspector_input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    crate::workbench::surfaces::simulate::drafts::text_field(ui, label, |ui| {
        crate::ui::widgets::input_row(ui, label, value)
    })
}

pub(super) fn input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, None, |ui| {
        mono_input(ui, label, value, ui.available_width())
    })
}

pub(super) fn input_row_enabled(
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
/// they take, and [`input_row`] says exactly that. A few have a
/// rule the label cannot carry and the value alone does not imply — a window
/// that overrides the count above it, a factor with a closed range, a control
/// only one mode reads — and for those the rule is worth more in the slot than
/// the notation is. Both accept the same spellings either way.
pub(super) fn hinted_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    hint: &str,
) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, Some(hint), |ui| {
        mono_input(ui, label, value, ui.available_width())
    })
}

/// The same field, offered or withheld by the mode that reads it.
pub(super) fn hinted_input_row_enabled(
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

pub(super) fn quantity_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Response {
    let response = input_row(ui, label, value);
    normalize_quantity_on_focus_loss(&response, value, kind, policy, locale);
    response
}

/// A quantity row whose helper states something the label cannot.
///
/// The ordinary row leaves its helper slot empty. A field where *leaving it
/// empty* selects a
/// behaviour has a fact the reader cannot infer from the label, and this is
/// where that fact goes: the caption row's right edge, beside the name of the
/// thing it qualifies, rather than as a note under the control where it would
/// read as advice about the form.
pub(super) fn hinted_quantity_input_row(
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
pub(super) fn hinted_quantity_input_row_enabled(
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
pub(super) fn choice_row_enabled(
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

pub(super) fn quantity_input_row_enabled(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
    enabled: bool,
) -> Response {
    let response = input_row_enabled(ui, label, value, enabled);
    if enabled {
        normalize_quantity_on_focus_loss(&response, value, kind, policy, locale);
    }
    response
}

pub(super) fn choice_row(ui: &mut Ui, label: &str, options: &[&str], value: &mut usize) -> bool {
    if !uses_two_column_fields(ui) {
        return inspector_choice_row(ui, label, options, value);
    }
    field_cell(ui, label, None, |ui| {
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
/// A tolerance well whose emptiness selects the plan's own policy.
///
/// A blank well here is not an omission, it is the selection "whatever the
/// Solver options channel states deck-wide", and the empty well says `default`.
/// A written value is this analysis's own and needs no word beside it.
pub(super) fn plan_policy_tolerance_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, label, value);
    }
    field_cell(ui, label, None, |ui| {
        defaulted_input(ui, label, value, ui.available_width())
    })
}

/// What an empty well that takes a resolved value says, faint, inside itself.
const EMPTY_WELL_PLACEHOLDER: &str = "default";

/// A mono well that reads [`EMPTY_WELL_PLACEHOLDER`] while it is empty.
///
/// Emptiness is a setting on these wells: the value the plan or the engine
/// resolves applies. So the empty well names that setting, and a well holding
/// a value shows only the value, with no tag beside it.
pub(super) fn defaulted_input(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    width: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    crate::workbench::surfaces::simulate::drafts::text_field(ui, label, |ui| {
        let response = ui.add_sized(
            vec2(width, t.metrics.ctl_h),
            egui::TextEdit::singleline(value)
                .font(theme::field_mono())
                .margin(egui::Margin::symmetric(8, 4))
                .hint_text(
                    egui::RichText::new(EMPTY_WELL_PLACEHOLDER)
                        .font(theme::field_mono())
                        .color(t.color.text_faint),
                ),
        );
        crate::ui::widgets::name_control(ui, &response, label);
        response
    })
}

pub(super) fn enabled_choice_row(ui: &mut Ui, label: &str, enabled: &mut bool) -> bool {
    let mut selected = usize::from(!*enabled);
    let changed = choice_row(ui, label, XF_ENABLED_CHOICES, &mut selected);
    if changed {
        *enabled = selected == 0;
    }
    changed
}

pub(super) fn choice_row_with_disabled(
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
        return full_width_field(ui, label, None, Tokens::get(ui.ctx()).metrics.ctl_h, |ui| {
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
        });
    }
    field_cell(ui, label, None, |ui| {
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

pub(super) fn mono_input_with_suffix(
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
pub(super) fn normalize_quantity_on_focus_loss(
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
pub(crate) fn normalize_quantity(
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
/// [`crate::quantity::spice_value::parse_spice_value_checked`] on most paths
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
    crate::quantity::spice_value::parse_spice_value_checked(text).is_ok_and(agrees)
        && crate::simulation::dialog::parse_si_value(text).is_ok_and(agrees)
}

pub(super) fn switch_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    if !uses_two_column_fields(ui) {
        return inspector_switch_row(ui, label, value);
    }
    field_cell(ui, label, None, |ui| {
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
/// [`switch_row`] leaves that slot empty. A switch whose whole point is an output the reader has to know where to look for —
/// the solver trace, which appears in the Console and nowhere else — says that
/// there instead.
pub(super) fn hinted_switch_row(ui: &mut Ui, label: &str, value: &mut bool, hint: &str) -> bool {
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

pub(super) fn property_row(ui: &mut Ui, label: &str, value: &str) {
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

/// Mono sub-header inside a form ("Tone 2", "Port 1"), as written.
pub(super) fn sub_header(ui: &mut Ui, text: &str) {
    clear_pending_cell(ui);
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.add(egui::Label::new(
        egui::RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    ));
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

/// A full-width advisory: something here will refuse the run. Its warning
/// mark says so; the words stay neutral.
pub(super) fn field_advisory(ui: &mut Ui, text: &str) {
    clear_pending_cell(ui);
    ui.add_space(2.0);
    super::page_kit::note_line(ui, text, super::page_kit::Tone::Warn);
}

/// A full-width note: something the design holds, which the reader may take or
/// leave.
///
/// Not [`field_advisory`], and the colour is the difference. Painting a fact in
/// the warning colour teaches a reader to stop believing the warning colour.
pub(super) fn field_note(ui: &mut Ui, text: &str) {
    field_prose(ui, text, Tokens::get(ui.ctx()).color.text_dim);
}

/// A full-width ghost add/remove action line. Returns `true` on click.
pub(super) fn action_line(ui: &mut Ui, label: &str) -> bool {
    action_line_enabled(ui, label, true)
}

/// The same line, offered or withheld.
///
/// A disabled line still announces itself, so the reason it is disabled has to
/// be painted beside it rather than left to the greying.
pub(super) fn action_line_enabled(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    clear_pending_cell(ui);
    Button::new(label)
        .ghost()
        .enabled(enabled)
        .min_width(ui.available_width())
        .show(ui)
        .clicked()
}

pub(super) fn frequency_sweep_fields(
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
