//! One analysis's advanced options, drawn as fields of its own form.
//!
//! They used to be a table: twenty-five rows of option, effective value and
//! origin, with a press to open a row and three commands to close it. A table
//! is the wrong shape for a form. An analysis's stop time and its truncation
//! bound are the same kind of thing — a value this analysis states — and they
//! now read the same way, in the same two-column grid, with the same label over
//! the same well.
//!
//! A field shows the value the solve will use and nothing beside it. A well
//! the plan states nothing for is empty and reads `default`, faint, inside
//! itself; emptying a well is how an override returns to that value.
//!
//! Which options appear is [`super::super::advanced_options::form_rows`]: the
//! ones this analysis owns, never the plan's global policy. What a field edit
//! asks for is decided there too, so this file is presentation and nothing
//! else — it collects edits and returns them, and the frame that drew it
//! commits them after it has closed.

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisDraft, NumericOverrideOption, OverrideValueKind};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::select;
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
    let plan = app.state.sim_setup.stable_analysis_plan().ok();
    let record = plan
        .and_then(|plan| plan.instance(instance))
        .and_then(|target| target.numeric_override());
    let ownership = plan.map_or_else(
        || draft.solver_ownership_with_options(record),
        |plan| plan.solver_ownership_for_draft(draft, record),
    );
    let offered = if ownership.study_inherited_options.is_some() {
        advanced_options::form_rows_with_ownership(
            draft.kind(),
            draft,
            record,
            &app.state.sim_setup.options,
            ownership,
        )
    } else {
        advanced_options::form_rows(draft.kind(), draft, record, &app.state.sim_setup.options)
    };
    if ownership.study_inherited_options.is_some() {
        super::field_note(
            ui,
            "These defaults apply to study stages that inherit them. Settings on the selected base analysis and its prerequisites take precedence.",
        );
    }
    if matches!(draft, AnalysisDraft::HarmonicBalance(_))
        && ownership.time_integration == Some(true)
    {
        super::field_note(
            ui,
            "Time-integration settings control the transient used to initialize harmonic balance.",
        );
    }
    if draft.kind().inherits_periodic_solver_options() {
        super::field_note(
            ui,
            "Shared solver options come from the bound periodic analysis. Configure its solver options there; response-specific settings remain on this form.",
        );
    }
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

/// One option as a field: its label and the control its value kind asks for.
fn field(ui: &mut Ui, row: &AdvancedOptionRow) -> Option<OptionEdit> {
    let label = row.option.label();
    match row.option.value_kind() {
        OverrideValueKind::Flag => option_field(ui, label, |ui| {
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
            option_field(ui, label, |ui| {
                let salt = format!("analysis-option-{}", row.option.key());
                select(ui, &salt, label, &current, &choices, ui.available_width())
                    .and_then(|index| choices.get(index).cloned())
            })
            .and_then(|chosen| advanced_options::setting_edit(row, &chosen))
        }
        _ => {
            let mut text = well_text(ui, row);
            let response = option_field(ui, label, |ui| {
                super::defaulted_input(ui, label, &mut text, ui.available_width())
                    .on_hover_text(option_tooltip(row.option, &row.preset))
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

/// One option's field: its label over its control.
///
/// Two columns where the form has room for them, and a full-width field where
/// it does not, so a narrow form keeps the caption over the well.
fn option_field<R>(ui: &mut Ui, label: &str, add_control: impl FnOnce(&mut Ui) -> R) -> R {
    if super::uses_two_column_fields(ui) {
        return super::field_cell(ui, label, None, add_control);
    }
    let height = Tokens::get(ui.ctx()).metrics.ctl_h;
    super::full_width_field(ui, label, None, height, add_control)
}

/// What hovering an option's well says: what the option does, in a designer's
/// words, its SPICE option name, and the value an empty well resolves to.
///
/// `preset` is the plan's own value for the option, or the engine-default
/// words where the plan states none.
fn option_tooltip(option: NumericOverrideOption, preset: &str) -> String {
    let default = if preset == advanced_options::ENGINE_ORIGIN || preset.trim().is_empty() {
        "Empty uses the engine's default.".to_owned()
    } else {
        format!("Default {preset}.")
    };
    format!("{} ({}). {default}", option_meaning(option), option.key())
}

/// One line on what an option does to the solve.
fn option_meaning(option: NumericOverrideOption) -> &'static str {
    use NumericOverrideOption as O;

    match option {
        O::Reltol => "Relative tolerance on the Newton update of voltages and currents",
        O::Abstol => "Absolute current tolerance for Newton convergence",
        O::Vntol => "Absolute voltage tolerance for Newton convergence",
        O::ResidualReltol => "Relative tolerance on the KCL residual for Newton convergence",
        O::Gmin => "Conductance added across every junction; 0 adds none",
        O::Itl1 => "Maximum Newton iterations for the DC operating point",
        O::Itl4 => "Maximum Newton iterations per transient step",
        O::GminStepping => "Retry a failed operating point by stepping Gmin down",
        O::SourceStepping => "Retry a failed operating point by ramping the sources up",
        O::PseudoTransient => "Retry a failed operating point with a pseudo-transient ramp",
        O::ArcLength => "Retry a failed operating point with arc-length continuation",
        O::Damping => "How each Newton update is limited",
        O::Chgtol => "Charge floor in the truncation-error estimate",
        O::Trtol => "Truncation-error overestimate factor; larger allows longer steps",
        O::IntegrationMethod => "Integration method for time stepping",
        O::LteReltol => "Relative bound on the local truncation error per step",
        O::LteAbstol => "Absolute bound on the local truncation error per step",
        O::MinTimestep => "Smallest time step before the transient stops",
        O::MaximumTimestep => "Largest time step the transient may take",
        O::Pivrel => "Relative pivot threshold for the matrix factorization",
        O::Pivtol => "Smallest pivot the matrix factorization accepts",
        O::Solver => "Sparse matrix solver",
        O::Bypass => "Skip devices whose terminal voltages have not moved",
        O::BypassReltol => "Relative voltage change below which a device is skipped",
        O::BypassAbstol => "Voltage change below which a device is skipped",
        O::TransientNewtonReltol => "Relative tolerance on the Newton update within a step",
        O::TransientNewtonAbstol => "Absolute tolerance on the Newton update within a step",
        O::TransientNewtonUpdateBound => "Largest weighted Newton correction a step accepts",
        O::TransientNewtonResidualBound => "Largest residual norm a step accepts",
        O::TransientNewtonBudget => "Maximum Newton iterations per time step",
        O::TransientDeviceConvergence => {
            "Require every device to converge before a step is accepted"
        }
        O::TransientNoxSolver => "Solve each step's Newton iteration with NOX",
        O::StrobeInterval => "Interval between reported time points",
        O::OutputTimePoints => "Report at exactly these times, space or comma separated",
        O::RetainEverySignal => "Keep every node and branch signal, not only the saved outputs",
        O::HbInitialState => "How the first harmonic-balance iterate is built",
    }
}

/// The settings a chooser offers for one option, in the enum's own order.
///
/// Read out of the same enumerations the record parses against, so a chooser
/// cannot offer a setting the deck could not carry. The display name is what is
/// offered and what is committed: the record accepts either spelling and
/// renders the one a reader saw.
fn settings_of(option: NumericOverrideOption) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Engine internals a reader cannot act on: a source path and line, the
    /// config field an option lands on, or the resolver's own vocabulary.
    const INTERNALS: [&str; 5] = [".rs:", "resolves onto", "read at", "engine/", "_config"];

    /// An option's hover says what it does, its SPICE name and its default,
    /// and never where the engine reads it.
    ///
    /// The hover once read "iteration count — ITL4 resolves onto
    /// transient_max_iterations, read at engine/transient/step_control.rs:191".
    #[test]
    fn an_option_tooltip_states_meaning_name_and_default_without_engine_internals() {
        for option in NumericOverrideOption::all() {
            for preset in ["10", advanced_options::ENGINE_ORIGIN] {
                let tooltip = option_tooltip(option, preset);
                for internal in INTERNALS {
                    assert!(
                        !tooltip.contains(internal),
                        "{} hovers {tooltip:?}, which names {internal:?}",
                        option.key()
                    );
                }
                assert!(
                    tooltip.contains(&format!("({})", option.key())),
                    "{tooltip:?} names its option"
                );
                assert!(!tooltip.contains(option.config_field()), "{tooltip:?}");
            }
        }
        assert_eq!(
            option_tooltip(NumericOverrideOption::Itl4, "10"),
            "Maximum Newton iterations per transient step (ITL4). Default 10."
        );
    }

    /// The form builds its hovers from the catalogue's meaning, not from the
    /// engine site an option is admitted on.
    #[test]
    fn the_option_form_never_reads_the_engine_site_into_product_text() {
        let source = crate::source_guard::production_source(include_str!("options.rs"));
        for internal in [".consumer()", ".config_field()", "resolves onto"] {
            assert!(
                !source.contains(internal),
                "options.rs builds product text from {internal}"
            );
        }
    }
}
