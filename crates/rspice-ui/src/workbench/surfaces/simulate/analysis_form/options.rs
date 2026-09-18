//! One analysis's advanced options, drawn as fields of its own form.
//!
//! They used to be a table: twenty-five rows of option, effective value and
//! origin, with a press to open a row and three commands to close it. A table
//! is the wrong shape for a form. An analysis's stop time and its truncation
//! bound are the same kind of thing — a value this analysis states — and they
//! now read the same way, in the same two-column grid, with the same label over
//! the same well.
//!
//! The field's hint slot is where the table's Origin column went. Start time
//! says "engineering notation" there; an option says `plan policy`, `override`
//! or `engine default`, which is the one fact a table of identical numbers
//! existed to carry.
//!
//! Which options appear is [`super::super::advanced_options::form_rows`]: the
//! ones this analysis owns, never the plan's global policy. What a field edit
//! asks for is decided there too, so this file is presentation and nothing
//! else — it collects edits and returns them, and the frame that drew it
//! commits them after it has closed.

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisDraft, OverrideValueKind};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{mono_input, select};
use crate::workbench::RSpiceApp;

use super::super::advanced_options::{self, AdvancedOptionRow, AdvancedOptionSection, OptionEdit};

/// Draw every advanced option this analysis owns, and report what was edited.
///
/// The record is the committed one and the options are the plan's committed
/// block, not the drafts either is edited through: an option field reports what
/// a run would resolve to, and an uncommitted edit is not part of any run yet.
/// The draft is still what carries the instance's tier and homotopy, which is
/// what decides whether an option reaches the solve at all.
pub(in crate::workbench::surfaces::simulate) fn fields(
    ui: &mut Ui,
    app: &RSpiceApp,
    draft: &AnalysisDraft,
    instance: AnalysisInstanceId,
) -> Vec<OptionEdit> {
    let record = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| plan.instance(instance))
        .and_then(|target| target.numeric_override());
    let offered =
        advanced_options::form_rows(draft.kind(), draft, record, &app.state.sim_setup.options);
    let mut edits = Vec::new();
    for (heading, rows) in grouped(&offered) {
        super::sub_header(ui, heading);
        for row in rows {
            if let Some(edit) = field(ui, row) {
                edits.push(edit);
            }
        }
    }
    edits
}

/// The rows a form draws, under the headings it draws them under.
///
/// The catalogue's sections are what the table used, and most kinds land almost
/// all of their options in one of them: a transient's charge floor and per-step
/// iteration budget sit in Charge and Convergence while the other five sit in
/// Integration. A heading over a single field says less than the field's own
/// label does, so a lone section joins the next one that has more than one, and
/// the merged group takes that section's name. A kind with one group gets one
/// heading; a kind with none gets nothing at all.
fn grouped(sections: &[AdvancedOptionSection]) -> Vec<(&'static str, Vec<&AdvancedOptionRow>)> {
    let mut groups: Vec<(&'static str, Vec<&AdvancedOptionRow>)> = Vec::new();
    let mut pending: Vec<&AdvancedOptionRow> = Vec::new();
    for section in sections {
        let mut held: Vec<&AdvancedOptionRow> = section.rows.iter().collect();
        if held.len() < 2 {
            pending.append(&mut held);
            continue;
        }
        let mut merged = std::mem::take(&mut pending);
        merged.append(&mut held);
        groups.push((section.section.title(), merged));
    }
    if let Some(first) = pending.first() {
        let title = first.option.section().title();
        match groups.last_mut() {
            Some((_, last)) => last.append(&mut pending),
            None => groups.push((title, pending)),
        }
    }
    groups
}

/// One option as a field: its label, its origin, and the control its value kind
/// asks for.
fn field(ui: &mut Ui, row: &AdvancedOptionRow) -> Option<OptionEdit> {
    let label = row.option.label();
    let origin = advanced_options::origin_hint(row);
    match row.option.value_kind() {
        OverrideValueKind::Flag => option_field(ui, label, origin, |ui| {
            let mut setting = row.effective.trim().eq_ignore_ascii_case("on");
            let row_size = egui::vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h);
            let changed = ui
                .allocate_ui_with_layout(
                    row_size,
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        // Bare, because the field has already painted the
                        // caption over it, and compact at the leading edge: the
                        // cell owns the whole grid column but a switch keeps its
                        // natural width, the way the form's own booleans do.
                        super::super::page_kit::switch_cell(ui, label, &mut setting).changed()
                    },
                )
                .inner;
            changed.then_some(if setting { "on" } else { "off" })
        })
        .and_then(|chosen| advanced_options::setting_edit(row, chosen)),
        OverrideValueKind::Method
        | OverrideValueKind::Damping
        | OverrideValueKind::Solver
        | OverrideValueKind::TimeDomainMode => {
            let choices = settings_of(row.option);
            // What the solve will use, shown verbatim. A value the chooser does
            // not offer can only be one the plan resolved and no analysis may
            // author — the automatic matrix backend is the one — and stating it
            // is more use than replacing it with a placeholder.
            let current = row.effective.clone();
            option_field(ui, label, origin, |ui| {
                let salt = format!("analysis-option-{}", row.option.key());
                select(ui, &salt, label, &current, &choices, ui.available_width())
                    .and_then(|index| choices.get(index).cloned())
            })
            .and_then(|chosen| advanced_options::setting_edit(row, &chosen))
        }
        _ => {
            let mut text = well_text(ui, row);
            let response = option_field(ui, label, origin, |ui| {
                // The hint slot states the origin, so the shape a value has to
                // take is the hover — and with it the engine field the option
                // lands on and the site that reads it, which is what tells a
                // reader the bound they are typing reaches a solve at all.
                mono_input(ui, label, &mut text, ui.available_width()).on_hover_text(format!(
                    "{}\n\n{} resolves onto {}, read at {}.",
                    row.option.value_hint(),
                    row.option.key(),
                    row.option.config_field(),
                    row.option.consumer()
                ))
            });
            retain_well_text(ui, row, &text, &response);
            // Committed when the well is let go of, not per keystroke: one
            // would produce a configuration receipt for every character and
            // invalidate preflight mid-number.
            let released = response.lost_focus() || (response.changed() && !response.has_focus());
            released
                .then(|| advanced_options::well_edit(row, &text))
                .flatten()
        }
    }
}

/// One option's field: its label, its origin in the hint slot, and its control.
///
/// Two columns where the form has room for them, and a full-width field where
/// it does not — the hint slot is the whole of the Origin column the table gave
/// up, so a narrow form keeps it rather than falling back to a row that has
/// nowhere to put it.
fn option_field<R>(
    ui: &mut Ui,
    label: &str,
    origin: &str,
    add_control: impl FnOnce(&mut Ui) -> R,
) -> R {
    if super::uses_two_column_fields(ui) {
        return super::field_cell(ui, label, Some(origin), add_control);
    }
    let height = Tokens::get(ui.ctx()).metrics.ctl_h;
    super::full_width_field(ui, label, Some(origin), height, add_control)
}

/// The settings a chooser offers for one option, in the enum's own order.
///
/// Read out of the same enumerations the record parses against, so a chooser
/// cannot offer a setting the deck could not carry. The display name is what is
/// offered and what is committed: the record accepts either spelling and
/// renders the one a reader saw.
fn settings_of(option: crate::simulation::plan::NumericOverrideOption) -> Vec<String> {
    use crate::simulation::dialog::{
        DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver,
    };

    match option.value_kind() {
        OverrideValueKind::Method => IntegrationMethod::all()
            .iter()
            .map(|method| method.spice_name().to_owned())
            .collect(),
        OverrideValueKind::Damping => DampingStrategy::all()
            .iter()
            .map(|strategy| strategy.display_name().to_owned())
            .collect(),
        // The sentence, never the digit the deck carries: `TAHB=1` says
        // nothing a reader can choose between, and the record accepts either
        // spelling back. The engine's resting behaviour opens the list because
        // it is not one of the three modes — a chooser has no empty position,
        // so inheriting has to be a position.
        OverrideValueKind::TimeDomainMode => {
            std::iter::once(advanced_options::HB_DEFAULT_INITIAL_STATE.to_owned())
                .chain(
                    HbTimeDomainMode::all()
                        .iter()
                        .map(|mode| mode.display_name().to_owned()),
                )
                .collect()
        }
        // The automatic backend emits no key at all, so an analysis cannot
        // state it: removing the override is how one returns to automatic, and
        // a chooser that offered it would offer a departure from nothing.
        OverrideValueKind::Solver => MatrixSolver::all()
            .iter()
            .filter(|solver| solver.spice_name().is_some())
            .map(|solver| solver.display_name().to_owned())
            .collect(),
        _ => Vec::new(),
    }
}

/// The text one option's well currently holds.
///
/// Seeded from the effective value on every frame the well is not being typed
/// into, which is what makes a refused edit revert: the value the solve is
/// actually using comes back rather than a number no run resolves to staying on
/// screen. While the well has focus the reader's own keystrokes are what it
/// holds, so the buffer sits beside the widget that draws it — the fields are
/// drawn from a shared reference and cannot write a keystroke into the plan.
fn well_text(ui: &Ui, row: &AdvancedOptionRow) -> String {
    ui.data(|data| data.get_temp::<String>(well_id(ui, row)))
        .unwrap_or_else(|| advanced_options::well_value(row).to_owned())
}

/// Retain what the well holds while it is focused, and drop it once it is not.
fn retain_well_text(ui: &Ui, row: &AdvancedOptionRow, text: &str, response: &egui::Response) {
    let id = well_id(ui, row);
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text.to_owned()));
    } else {
        ui.data_mut(|data| data.remove::<String>(id));
    }
}

/// Where one option's well keeps its text. Keyed by the option, under the id of
/// the form drawing it, so two analyses on one route cannot share a buffer.
fn well_id(ui: &Ui, row: &AdvancedOptionRow) -> egui::Id {
    ui.id().with(("analysis-option-well", row.option.key()))
}
