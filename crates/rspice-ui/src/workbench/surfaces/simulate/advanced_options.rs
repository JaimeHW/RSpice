//! One analysis's advanced options, by section.
//!
//! The resolution ledger above this panel answers "which analyses depart from
//! the plan, and where". This one answers the other question: for *this*
//! analysis, what does every advanced option actually resolve to, and who
//! decided it.
//!
//! So every option the catalog knows earns a row, not only the authored ones.
//! A row an analysis has not touched still states the value its solve will
//! use and names the plan as the owner; a row its kind cannot carry states the
//! refusal in place rather than vanishing, because an option that is simply
//! absent reads as an oversight and sends a reader looking for it elsewhere.
//!
//! The origin column is the whole point of the panel and has four values:
//!
//! - **plan policy** — the analysis states nothing, so the plan's own
//!   `.OPTIONS` block decides.
//! - **analysis override** — this analysis states it, and its card comes
//!   second in the deck, so it wins.
//! - **engine default** — neither the plan nor the analysis states a value and
//!   the engine's own dialect default stands.
//! - a **refusal sentence** — the kind cannot carry the option, and the
//!   sentence says who owns it instead. The accuracy tier's ownership of the
//!   Newton budget is the one a reader meets most often.

use std::cell::{Cell, RefCell};

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::dialog::SimulationOptions;
use crate::simulation::plan::{
    AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, OverrideSection,
};
use crate::ui::widgets::{Button, mono_input};
use crate::workbench::RSpiceApp;
use crate::workbench::state::AdvancedOptionsEditor;

use super::page_kit::{Tone, card_note, card_with_head, ledger_head, ledger_row};

/// Analysis · Option · Effective value · Origin, and the editor's own cell.
const COLUMNS: [f32; 4] = [0.30, 0.22, 0.30, 0.18];

pub(super) const PLAN_ORIGIN: &str = "plan policy";
pub(super) const OVERRIDE_ORIGIN: &str = "analysis override";
pub(super) const ENGINE_ORIGIN: &str = "engine default";

/// One option, as this analysis resolves it.
pub(super) struct AdvancedOptionRow {
    pub(super) option: NumericOverrideOption,
    /// What the solve will use.
    pub(super) effective: String,
    pub(super) origin: &'static str,
    /// The value this analysis states, when it states one. `None` means the
    /// row is inherited and "Clear" has nothing to do.
    pub(super) authored: Option<String>,
    /// Set when the kind cannot carry the option at all.
    pub(super) refused: bool,
}

/// One section's rows, in catalog order.
pub(super) struct AdvancedOptionSection {
    pub(super) section: OverrideSection,
    pub(super) rows: Vec<AdvancedOptionRow>,
}

/// Every option for one analysis, grouped into its sections.
///
/// Derived fresh from the plan each frame rather than cached: an option's
/// effective value depends on the plan's options block and on the analysis's
/// own record, and both are edited from elsewhere on this page.
pub(super) fn sections(
    kind: AnalysisKind,
    record: Option<&AnalysisNumericOverride>,
    options: &SimulationOptions,
) -> Vec<AdvancedOptionSection> {
    OverrideSection::ALL
        .into_iter()
        .map(|section| AdvancedOptionSection {
            section,
            rows: NumericOverrideOption::all()
                .filter(|option| option.section() == section)
                .map(|option| row(option, kind, record, options))
                .collect(),
        })
        .filter(|section| !section.rows.is_empty())
        .collect()
}

fn row(
    option: NumericOverrideOption,
    kind: AnalysisKind,
    record: Option<&AnalysisNumericOverride>,
    options: &SimulationOptions,
) -> AdvancedOptionRow {
    let preset = super::page_solver::plan_preset_value(option, options);
    // A refusal outranks an authored value on purpose. A record restored from
    // an older project can hold an option its kind stopped accepting, and the
    // honest report is that the solve ignores it — not the number it holds.
    if let Some(reason) = option.refusal_for(kind) {
        return AdvancedOptionRow {
            option,
            effective: preset,
            origin: reason,
            authored: None,
            refused: true,
        };
    }
    match record.and_then(|record| record.value(option)) {
        Some(authored) => AdvancedOptionRow {
            option,
            effective: authored.clone(),
            origin: OVERRIDE_ORIGIN,
            authored: Some(authored),
            refused: false,
        },
        None => AdvancedOptionRow {
            option,
            // `plan_preset_value` says so itself when the plan states nothing.
            origin: if preset == "engine default" {
                ENGINE_ORIGIN
            } else {
                PLAN_ORIGIN
            },
            effective: preset,
            authored: None,
            refused: false,
        },
    }
}

/// The analysis this panel is open on, if any.
fn open_instance(app: &RSpiceApp) -> Option<AnalysisInstanceId> {
    let editor = app.state.workbench.advanced_options.as_ref()?;
    let plan = app.state.sim_setup.stable_analysis_plan().ok()?;
    // A plan edit elsewhere can retire the instance the panel was opened on.
    plan.instance(editor.instance).map(|_| editor.instance)
}

pub(super) fn panel(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(instance) = open_instance(app) else {
        return;
    };
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return;
    };
    let Some(target) = plan.instance(instance) else {
        return;
    };
    let kind = target.kind();
    let name = target.display_name().to_owned();
    // The committed plan options, not the page's draft: this panel reports
    // what a run would resolve to, and an uncommitted edit above it is not
    // yet part of any run.
    let sections = sections(
        kind,
        target.numeric_override(),
        &app.state.sim_setup.options,
    );
    let editing = app
        .state
        .workbench
        .advanced_options
        .as_ref()
        .and_then(|editor| editor.editing);
    let error = app
        .state
        .workbench
        .advanced_options
        .as_ref()
        .and_then(|editor| editor.error.clone());
    let departures = sections
        .iter()
        .flat_map(|section| &section.rows)
        .filter(|row| row.authored.is_some())
        .count();
    let status = if departures == 0 {
        format!("{} resolves entirely to the plan policy", kind.label())
    } else {
        format!("{departures} of this analysis's options depart from the plan")
    };

    let close = Cell::new(false);
    let picked = Cell::new(None::<NumericOverrideOption>);
    let clear = Cell::new(None::<NumericOverrideOption>);
    let apply = Cell::new(false);
    let value = RefCell::new(
        app.state
            .workbench
            .advanced_options
            .as_ref()
            .map_or_else(String::new, |editor| editor.value.clone()),
    );

    card_with_head(
        ui,
        |ui| {
            super::page_kit::card_head_row(
                ui,
                &format!("Advanced options · {name}"),
                Some((status.as_str(), Tone::Neutral)),
                |ui| {
                    close.set(Button::new("Close").show(ui).clicked());
                },
            );
        },
        |ui| {
            for section in &sections {
                ledger_head(
                    ui,
                    &COLUMNS,
                    &[section.section.title(), "Effective", "Origin", ""],
                );
                for row in &section.rows {
                    let is_editing = editing == Some(row.option);
                    if is_editing {
                        editor_row(ui, row, &value, &apply, &clear);
                        continue;
                    }
                    let response = ledger_row(
                        ui,
                        &COLUMNS,
                        &[
                            (row.option.label(), Tone::Neutral),
                            (
                                row.effective.as_str(),
                                match () {
                                    () if row.refused => Tone::Neutral,
                                    () if row.authored.is_some() => Tone::Warn,
                                    () => Tone::Accent,
                                },
                            ),
                            (row.origin, Tone::Neutral),
                            (if row.refused { "—" } else { "Edit" }, Tone::Neutral),
                        ],
                        false,
                    );
                    // The origin cell carries a whole sentence for a refused
                    // row and is the first to elide, so the hover restates it
                    // together with the engine site that reads the option.
                    let response = response.on_hover_text(format!(
                        "{}\n\n{} resolves onto {}, read at {}.",
                        row.origin,
                        row.option.key(),
                        row.option.config_field(),
                        row.option.consumer()
                    ));
                    if response.clicked() && !row.refused {
                        picked.set(Some(row.option));
                    }
                }
            }
            if let Some(error) = error.as_ref() {
                card_note(ui, error);
            }
            card_note(
                ui,
                "Every row states what this analysis's solve will use. An inherited row follows \
                 the plan; an authored one reaches the engine as a second options block in this \
                 task's own deck, so it wins for exactly the key it names. A refused row states \
                 who owns the value instead — clearing the override is how the analysis returns \
                 to the plan.",
            );
        },
    );

    if close.get() {
        app.state.workbench.advanced_options = None;
        return;
    }
    if let Some(editor) = app.state.workbench.advanced_options.as_mut() {
        editor.value = value.into_inner();
    }
    if let Some(option) = picked.get() {
        let seeded = sections
            .iter()
            .flat_map(|section| &section.rows)
            .find(|row| row.option == option)
            .map(|row| row.effective.clone())
            .unwrap_or_default();
        if let Some(editor) = app.state.workbench.advanced_options.as_mut() {
            editor.editing = Some(option);
            editor.value = seeded;
            editor.error = None;
        }
        return;
    }
    if let Some(option) = clear.get() {
        let outcome = clear_option(app, instance, option);
        if let Some(editor) = app.state.workbench.advanced_options.as_mut() {
            editor.editing = None;
            editor.error = outcome.err();
        }
        return;
    }
    if apply.get()
        && let Some(option) = editing
    {
        let authored = app
            .state
            .workbench
            .advanced_options
            .as_ref()
            .map(|editor| editor.value.trim().to_owned())
            .unwrap_or_default();
        let outcome = write_option(app, instance, option, &authored);
        if let Some(editor) = app.state.workbench.advanced_options.as_mut() {
            editor.error = outcome.as_ref().err().cloned();
            if outcome.is_ok() {
                editor.editing = None;
            }
        }
    }
}

/// The row under edit: the option's name, an input, and the two commands.
fn editor_row(
    ui: &mut Ui,
    row: &AdvancedOptionRow,
    value: &RefCell<String>,
    apply: &Cell<bool>,
    clear: &Cell<Option<NumericOverrideOption>>,
) {
    let (_, cells) = super::page_kit::ledger_row_cells(ui, &COLUMNS);
    let mut name = super::page_kit::cell_ui(ui, cells[0]);
    name.label(row.option.label());

    let mut input = super::page_kit::cell_ui(ui, cells[1]);
    let width = input.available_width();
    {
        let mut text = value.borrow_mut();
        mono_input(&mut input, &mut text, width).on_hover_text(row.option.value_hint());
    }

    // The hint sits where the origin does on a read-only row: an editing row
    // has no origin yet, and the shape a value must take is what a reader
    // needs in that instant instead.
    let mut hint = super::page_kit::cell_ui(ui, cells[2]);
    hint.label(row.option.value_hint());

    let mut actions = super::page_kit::cell_ui(ui, cells[3]);
    if Button::new("Apply").show(&mut actions).clicked() {
        apply.set(true);
    }
    if Button::new("Clear")
        .enabled(row.authored.is_some())
        .show(&mut actions)
        .clicked()
    {
        clear.set(Some(row.option));
    }
}

fn write_option(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    authored: &str,
) -> Result<(), String> {
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let target = plan
        .instance(instance)
        .ok_or_else(|| "The selected analysis is no longer in the plan.".to_owned())?;
    let kind = target.kind();
    let mut record = target.numeric_override().cloned().unwrap_or_default();
    record.set(kind, option, authored)?;
    super::lifecycle::commit_numeric_override(app, instance, Some(record))
}

fn clear_option(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
) -> Result<(), String> {
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let Some(mut record) = plan
        .instance(instance)
        .and_then(|target| target.numeric_override().cloned())
    else {
        return Ok(());
    };
    record.clear(option);
    super::lifecycle::commit_numeric_override(app, instance, Some(record))
}

/// Open the panel on one analysis.
pub(super) fn open_for_analysis(app: &mut RSpiceApp, instance: AnalysisInstanceId) {
    app.state.workbench.advanced_options = Some(AdvancedOptionsEditor {
        instance,
        editing: None,
        value: String::new(),
        error: None,
    });
}

#[cfg(test)]
mod tests;
