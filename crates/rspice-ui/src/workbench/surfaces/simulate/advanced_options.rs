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
//! - a **refusal sentence** — the option cannot be carried, and the sentence
//!   says who owns it instead. The accuracy tier's ownership of the Newton
//!   budget is the one a reader meets most often.
//!
//! A refusal is not always a property of the kind. Five of these options land
//! on fields the analysis's *own* accuracy tier and homotopy control assign,
//! and both are applied after the deck's `.OPTIONS` are resolved, so whether
//! such an option reaches the solve depends on which tier and which homotopy
//! this instance carries. Those rows are refused per instance, and their
//! effective cell states the owner's value read back out of the same two
//! functions the solve applies — never the plan preset, which is precisely
//! the number the solve is about to discard.

use std::cell::{Cell, RefCell};

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::dialog::SimulationOptions;
use crate::simulation::plan::{
    AnalysisDraft, AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, OverrideSection,
    OverrideValueKind, SolverOwnership,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
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
    draft: &AnalysisDraft,
    record: Option<&AnalysisNumericOverride>,
    options: &SimulationOptions,
) -> Vec<AdvancedOptionSection> {
    OverrideSection::ALL
        .into_iter()
        .map(|section| AdvancedOptionSection {
            section,
            rows: NumericOverrideOption::all()
                .filter(|option| option.section() == section)
                .map(|option| row(option, kind, draft, record, options))
                .collect(),
        })
        .filter(|section| !section.rows.is_empty())
        .collect()
}

/// The value an instance's own controls assign, when they assign one.
///
/// `None` when nothing on this analysis's form owns the option, which is the
/// case a kind-level refusal produces: the kind forbids the key rather than
/// assigning the field, so there is no owner's number to print.
///
/// Read back out of the same two functions the solve applies —
/// [`crate::simulation::accuracy::AccuracyPolicy::apply`] and
/// [`crate::simulation::dialog::OpHomotopy::apply`] — rather than restated
/// here, so this cell cannot name a value the engine is not given. The base is
/// a default configuration on purpose: an owned field is assigned
/// unconditionally, so nothing the deck resolved to reaches it.
fn owned_solver_value(option: NumericOverrideOption, ownership: SolverOwnership) -> Option<String> {
    use NumericOverrideOption as O;

    let owner = match option {
        O::GminStepping | O::SourceStepping | O::PseudoTransient | O::ArcLength => {
            ownership.continuation_aid_owner()
        }
        O::Damping => ownership.damping_owner(),
        _ => None,
    };
    owner?;

    let mut config = rspice_core::SimulationConfig::default();
    if let Some(accuracy) = ownership.accuracy {
        accuracy.solver_policy().apply(&mut config);
    }
    if let Some(homotopy) = ownership.homotopy {
        homotopy.apply(&mut config);
    }
    let convergence = &config.convergence_config;
    // The same two words the record itself renders a flag with, so a reader
    // moving between an authored row and an owned one reads one vocabulary.
    let flag = |value: bool| if value { "on" } else { "off" }.to_owned();
    Some(match option {
        O::GminStepping => flag(convergence.gmin_stepping),
        O::SourceStepping => flag(convergence.source_stepping),
        O::PseudoTransient => flag(convergence.pseudo_transient),
        O::ArcLength => flag(convergence.arc_length),
        O::Damping => {
            crate::simulation::dialog::DampingStrategy::from_core(convergence.damping_strategy)
                .display_name()
                .to_owned()
        }
        _ => return None,
    })
}

fn row(
    option: NumericOverrideOption,
    kind: AnalysisKind,
    draft: &AnalysisDraft,
    record: Option<&AnalysisNumericOverride>,
    options: &SimulationOptions,
) -> AdvancedOptionRow {
    // The instance's own tier and homotopy decide five of these options, and
    // they decide them after the deck is read. The draft is what carries them.
    let ownership = draft.solver_ownership();
    let preset = super::page_solver::plan_preset_value(option, options);
    // A refusal outranks an authored value on purpose. A record restored from
    // an older project can hold an option this instance stopped accepting, and
    // the honest report is that the solve ignores it — not the number it holds.
    if let Some(reason) = option.refusal_for_instance(kind, ownership) {
        return AdvancedOptionRow {
            option,
            effective: refused_effective(option, draft, ownership, options),
            origin: reason,
            authored: None,
            refused: true,
        };
    }
    match record.and_then(|record| record.value(option)) {
        Some(authored) => {
            // A step ceiling composes with the plan's rather than replacing it,
            // so the authored number is not always what the run steps at.
            let (effective, origin) = if option == NumericOverrideOption::MaximumTimestep {
                super::page_solver::resolved_step_ceiling(&authored, options.max_timestep)
            } else {
                (authored.clone(), OVERRIDE_ORIGIN)
            };
            AdvancedOptionRow {
                option,
                effective,
                origin,
                authored: Some(authored),
                refused: false,
            }
        }
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

/// A refused option whose owner has no number to show.
const NO_REFUSED_VALUE: &str = "\u{2014}";

/// What a refused row's solve actually uses.
///
/// Never the plan preset. A row is refused precisely because something other
/// than the plan decides the value, so echoing the preset there stated a
/// number the run would not use. Three owners can answer, and each is read
/// through the call that actually assigns it rather than restated here, so
/// this cell and the Solver page's resolution ledger cannot disagree about one
/// analysis:
///
/// * the accuracy tier replaces ITL1 outright after the deck resolves, and
///   this prints the ledger's own string for it;
/// * the tier and the operating point's homotopy control assign the four
///   continuation aids and the damping strategy, also after the deck, which
///   [`owned_solver_value`] reads back out of the two `apply` calls the solve
///   makes;
/// * a step ceiling is authored on the transient's own form and composes with
///   the plan's by `min` rather than replacing it.
///
/// The fourth refusal — an option that only reaches a solve which advances
/// time, or one a kind forbids outright rather than assigning — has no owner
/// and no value, so it states an em dash rather than a number.
pub(super) fn refused_effective(
    option: NumericOverrideOption,
    draft: &AnalysisDraft,
    ownership: SolverOwnership,
    options: &SimulationOptions,
) -> String {
    match option {
        NumericOverrideOption::Itl1 => ownership.accuracy.map_or_else(
            || NO_REFUSED_VALUE.to_owned(),
            super::page_solver::tier_iteration_budget,
        ),
        NumericOverrideOption::GminStepping
        | NumericOverrideOption::SourceStepping
        | NumericOverrideOption::PseudoTransient
        | NumericOverrideOption::ArcLength
        | NumericOverrideOption::Damping => {
            owned_solver_value(option, ownership).unwrap_or_else(|| NO_REFUSED_VALUE.to_owned())
        }
        NumericOverrideOption::MaximumTimestep => match draft {
            AnalysisDraft::Transient(setup) => {
                let ceiling = setup.max_step.trim();
                if ceiling.is_empty() || ceiling.eq_ignore_ascii_case("auto") {
                    // Not the plan's ceiling. `auto` leaves the deck's `.tran`
                    // max-step carrying the analysis's own output step time,
                    // and the engine mins that with the plan's — so a stock
                    // transient under a 1 ms plan ceiling steps at 10 ns, and
                    // this cell used to print the 1 ms.
                    super::page_solver::inherited_step_ceiling(&setup.step, options.max_timestep)
                        .map_or_else(|| NO_REFUSED_VALUE.to_owned(), |(value, _)| value)
                } else {
                    super::page_solver::resolved_step_ceiling(ceiling, options.max_timestep).0
                }
            }
            _ => NO_REFUSED_VALUE.to_owned(),
        },
        _ => NO_REFUSED_VALUE.to_owned(),
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
        target.draft(),
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
                 task's own deck, so it wins for exactly the key it names — except the step \
                 ceiling, which the transient clamps against the plan's, so the tighter of the \
                 two is what runs. A refused row states its owner's value, or an em dash where \
                 the option never reaches the solve at all. Clearing an override is how the \
                 analysis returns to the plan.",
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
    // Both commands go through the plan transaction the Solver page already
    // owns, rather than a second writer here. One writer per fact is what
    // stops the two surfaces disagreeing about what an override is.
    if let Some(option) = clear.get() {
        let cleared = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .and_then(|plan| plan.instance(instance))
            .and_then(|target| target.numeric_override().cloned())
            .map(|mut record| {
                record.clear(option);
                record
            });
        let outcome = match cleared {
            Some(record) => super::lifecycle::commit_numeric_override(app, instance, Some(record)),
            None => Ok(()),
        };
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
        let outcome = super::page_solver::write_numeric_record(app, instance, option, &authored);
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
    let color = Tokens::get(ui.ctx()).color;
    // The two text cells of an editing row sit in the same columns a read-only
    // row paints, so they take that row's type: `ledger_row` writes every cell
    // in mono at the token base size, and a bare `label` here painted egui's
    // 13 px default beside its own mono neighbours.
    let (_, cells) = super::page_kit::ledger_row_cells(ui, &COLUMNS);
    let mut name = super::page_kit::cell_ui(ui, cells[0]);
    name.label(
        egui::RichText::new(row.option.label())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color.text_dim),
    );

    let mut input = super::page_kit::cell_ui(ui, cells[1]);
    // A boolean is not free text. Typing `on` into a well that also accepts
    // `3.25e-7` invites a spelling the record then refuses, so the two
    // settings a flag has are the control.
    if row.option.value_kind() == OverrideValueKind::Flag {
        let mut text = value.borrow_mut();
        for setting in ["on", "off"] {
            let button = Button::new(setting);
            // The accent is the current setting, so the pair reads as a state
            // rather than as two commands.
            let button = if text.trim().eq_ignore_ascii_case(setting) {
                button.accent()
            } else {
                button
            };
            if button.show(&mut input).clicked() {
                *text = setting.to_owned();
            }
        }
    } else {
        let width = input.available_width();
        let mut text = value.borrow_mut();
        mono_input(&mut input, row.option.label(), &mut text, width)
            .on_hover_text(row.option.value_hint());
    }

    // The hint sits where the origin does on a read-only row: an editing row
    // has no origin yet, and the shape a value must take is what a reader
    // needs in that instant instead. Faint rather than dim, because the origin
    // it stands in for is a fact about the row and this is guidance about the
    // well beside it.
    let mut hint = super::page_kit::cell_ui(ui, cells[2]);
    hint.label(
        egui::RichText::new(row.option.value_hint())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color.text_faint),
    );

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

/// Open the panel on one analysis.
///
/// Takes the workbench rather than the application: opening a panel is a
/// change to what is on screen and touches nothing else, and a handler that
/// asked for the whole application could mutate every subsystem to do it.
pub(super) fn open_for_analysis(
    workbench: &mut crate::workbench::state::WorkbenchState,
    instance: AnalysisInstanceId,
) {
    workbench.advanced_options = Some(AdvancedOptionsEditor {
        instance,
        editing: None,
        value: String::new(),
        error: None,
    });
}

#[cfg(test)]
mod tests;
