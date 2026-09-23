//! What one analysis's advanced options resolve to, and who decided each.
//!
//! In Spectre and ADS an analysis's options belong to the analysis, so they are
//! drawn as fields on the analysis's own form — the same two-column grid that
//! carries a transient's stop, step and start times, with each field's hint
//! slot naming where its value came from. This file is the half that decides
//! *what* those fields say; `analysis_form/options.rs` draws them.
//!
//! Every option the catalogue knows earns a row here, including the ones no
//! form offers: the Solver page's resolution ledger reports the plan whole, and
//! it reads these same rows. [`form_rows`] is what narrows them to the ones one
//! analysis owns.
//!
//! The origin has four values:
//!
//! - **plan policy** — the analysis states nothing, so the plan's own
//!   `.OPTIONS` block decides.
//! - **analysis override** — this analysis states it, and its card comes
//!   second in the deck, so it wins.
//! - **engine default** — neither the plan nor the analysis states a value and
//!   the engine's own dialect default stands.
//! - a **refusal sentence** — the option cannot be carried, and the sentence
//!   says who owns it instead. The accuracy tier's ownership of the Newton
//!   budget is the one a reader meets most often. A refused option is given no
//!   field: a control that authors a value the solve discards is worse than no
//!   control, and the ledger has the room to say who owns it.
//!
//! A refusal is not always a property of the kind. Five of these options land
//! on fields the analysis's *own* accuracy tier and homotopy control assign,
//! and both are applied after the deck's `.OPTIONS` are resolved, so whether
//! such an option reaches the solve depends on which tier and which homotopy
//! this instance carries. Those rows are refused per instance, and their
//! effective value is the owner's, read back out of the same two functions the
//! solve applies — never the plan preset, which is precisely the number the
//! solve is about to discard.

use crate::product::AnalysisInstanceId;
use crate::simulation::dialog::SimulationOptions;
use crate::simulation::plan::{
    AnalysisDraft, AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, OverrideSection,
    SolverOwnership,
};
use crate::workbench::RSpiceApp;
use crate::workbench::state::WorkbenchState;

pub(super) const PLAN_ORIGIN: &str = "plan policy";
pub(super) const OVERRIDE_ORIGIN: &str = "analysis override";
pub(crate) const ENGINE_ORIGIN: &str = "engine default";

/// One option, as this analysis resolves it.
pub(super) struct AdvancedOptionRow {
    pub(super) option: NumericOverrideOption,
    /// What the solve will use.
    pub(super) effective: String,
    /// What the plan's own block resolves this option to, which is the policy
    /// the analysis departs *from*. Carried beside the effective value because
    /// a two-state control has no empty state to clear through, so choosing the
    /// plan's own setting is how such a field returns to it.
    pub(super) preset: String,
    /// Who decided the effective value: one of the three origin constants
    /// above, or the sentence naming the owner of an option this instance
    /// cannot carry. [`form_rows`] gives such a row no field, so a form only
    /// ever meets the first three.
    pub(super) origin: &'static str,
    /// The value this analysis states, when it states one. `None` means the
    /// row is inherited and there is nothing to clear.
    pub(super) authored: Option<String>,
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
            preset,
            origin: reason,
            authored: None,
        };
    }
    // A bound the plan states no policy for rests at the engine's own value,
    // and for a two-state or chooser control that value has to be stated: a
    // switch has no empty position to clear through, so the setting it returns
    // to is the setting it has to be able to show.
    let plan_states_nothing = preset == ENGINE_ORIGIN;
    let preset = if plan_states_nothing {
        engine_rest_value(option).map_or(preset, str::to_owned)
    } else {
        preset
    };
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
                preset,
                origin,
                authored: Some(authored),
            }
        }
        None => AdvancedOptionRow {
            option,
            // `plan_preset_value` says so itself when the plan states nothing.
            origin: if plan_states_nothing {
                ENGINE_ORIGIN
            } else {
                PLAN_ORIGIN
            },
            effective: preset.clone(),
            preset,
            authored: None,
        },
    }
}

/// What the engine resolves one option to when neither the plan nor the
/// analysis states it.
///
/// Only the two-state and chooser controls need one. A well returns to the
/// plan by being emptied, so a bound the plan does not state opens empty and
/// says `engine default` in its hint slot. A switch has no empty position: if
/// it could not show the setting the run actually uses, it would sit at `off`
/// over a solve that enforces device convergence, and flipping it to the value
/// already in force would author an override instead of clearing one.
///
/// Each arm is the same `unwrap_or` the engine applies, cited beside it, so a
/// default changed in the engine and not here is a one-line correction rather
/// than a hunt.
fn engine_rest_value(option: NumericOverrideOption) -> Option<&'static str> {
    use NumericOverrideOption as O;

    Some(match option {
        // engine/transient.rs:4621 reads `unwrap_or(true)`: Xyce's option
        // metadata advertises zero, but its runtime default is on.
        O::TransientDeviceConvergence => "on",
        // engine/transient.rs:1475 reads `unwrap_or(false)`, which selects the
        // damped transient solver.
        O::TransientNoxSolver => "off",
        // engine/transient.rs:1292 reads `unwrap_or(false)`.
        O::RetainEverySignal => "off",
        // engine/hb.rs:1150 leaves `HbInitialStateStrategy::DefaultDcSeed` in
        // force when no mode is stated, which is not one of the three modes
        // `TAHB` can name — so the chooser offers it as its own position and
        // selecting it clears the option.
        O::HbInitialState => HB_DEFAULT_INITIAL_STATE,
        _ => return None,
    })
}

/// The harmonic-balance initial state an unstated `TAHB` leaves in force.
///
/// A chooser position rather than a fourth [`crate::simulation::dialog::HbTimeDomainMode`]
/// variant: the engine's resting behaviour is the *absence* of the key, and a
/// variant for it would have to emit something.
pub(super) const HB_DEFAULT_INITIAL_STATE: &str = "Engine default (DC seed)";

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

/// What a press on the section asked the application to do.
///
/// Returned rather than performed, because [`section`] reads the application
/// and never writes it. The split is the same one `participation_action` makes
/// one level up (`simulate.rs`): a command that moves focus off a form field
/// must be dispatched before the editor compares its draft, or pressing it
/// would commit a half-typed field on the way out.
/// One change an analysis's advanced-option fields asked for.
///
/// Returned rather than performed, because the fields are drawn from a shared
/// reference and never write through it. The split is the one the form already
/// makes for its own values: an edit is collected while the frame is drawing
/// and committed after it has closed, so a control that moves focus off a
/// half-typed field cannot commit that field on the way out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum OptionEdit {
    /// Author, or re-author, one option at the value its field now holds.
    Set(NumericOverrideOption, String),
    /// Return one option to the plan.
    Clear(NumericOverrideOption),
}

/// Which advanced options an analysis's own form offers as fields.
///
/// Not all of them. A global solver bound — the Newton update bound, the
/// floors, the pivot thresholds, the matrix backend, device bypass — is the
/// plan's policy and is owned by the Solver page: the same number resolved
/// twenty ways per analysis is not a feature, it is twenty ways for one deck to
/// disagree with itself. What an analysis's form offers is what belongs to that
/// analysis:
///
/// * a kind that advances time owns how time advances — the integration method,
///   the truncation bounds, the step floor, the charge floor and the per-step
///   iteration budget;
/// * the operating point and the DC sweep own how their solve recovers — the
///   Newton budget and the four continuation aids with the damping strategy.
///
/// Two exclusions on top of that. The transient's step ceiling is its own `Max
/// step` field and is not repeated here. And an option the instance cannot
/// carry earns no field at all: a control that authors a value the solve
/// discards is worse than no control, and the Solver page's ledger has the room
/// to say who owns it instead.
///
/// The one exception is legacy data. A project saved before this partition can
/// hold a per-analysis override of a global option, and dropping it silently
/// would change what the run resolves to. So a global option earns a field
/// exactly while the analysis states one — visible, readable, and clearable —
/// and there is no path that authors a new one.
pub(super) fn form_rows(
    kind: AnalysisKind,
    draft: &AnalysisDraft,
    record: Option<&AnalysisNumericOverride>,
    options: &SimulationOptions,
) -> Vec<AdvancedOptionSection> {
    // The catalogue's own answer to what this instance may carry, rather than a
    // second reading of the refusals behind it: the gate that decides whether a
    // value survives to the solve is the gate that decides whether it earns a
    // control.
    let authorable = NumericOverrideOption::applicable_to_instance(kind, draft.solver_ownership());
    // Whether this analysis has already chosen the shape of its output
    // schedule. Read once, because the two keys that answer it are the two
    // rows that depend on the answer.
    let schedule_stated = record.is_some_and(|record| {
        record
            .value(NumericOverrideOption::StrobeInterval)
            .is_some()
            || record
                .value(NumericOverrideOption::OutputTimePoints)
                .is_some()
    });
    sections(kind, draft, record, options)
        .into_iter()
        .map(|mut section| {
            section.rows.retain(|row| {
                authorable.contains(&row.option)
                    && offered_on_the_form(
                        row.option,
                        kind,
                        row.authored.is_some(),
                        schedule_stated,
                    )
            });
            section
        })
        .filter(|section| !section.rows.is_empty())
        .collect()
}

/// Whether one option belongs on this kind's own form. See [`form_rows`].
fn offered_on_the_form(
    option: NumericOverrideOption,
    kind: AnalysisKind,
    authored: bool,
    schedule_stated: bool,
) -> bool {
    use NumericOverrideOption as O;

    match option {
        // The catalogue excludes the transient, whose own Max step field
        // writes this bound. Other time-stepped analyses can author it here.
        O::MaximumTimestep => true,
        O::IntegrationMethod
        | O::Trtol
        | O::LteReltol
        | O::LteAbstol
        | O::MinTimestep
        | O::Chgtol
        | O::Itl4 => true,
        // The transient Newton package and the output schedule belong to the
        // kind that runs the time steps, which is the only kind whose refusal
        // gate lets them through at all. Offered unconditionally there, for
        // the same reason the truncation bounds are: the form that owns how
        // time advances owns how each step's solve is bounded and which of the
        // solved steps are reported.
        O::TransientNewtonReltol
        | O::TransientNewtonAbstol
        | O::TransientNewtonUpdateBound
        | O::TransientNewtonResidualBound
        | O::TransientNewtonBudget
        | O::TransientDeviceConvergence
        | O::TransientNoxSolver
        | O::RetainEverySignal
        | O::HbInitialState => true,
        // The two output-schedule keys are one control with two shapes. Both
        // are offered until one is stated, and then only that one, because the
        // engine's parser refuses a card carrying both and a field that
        // authored the second would author a deck that cannot be read.
        O::StrobeInterval | O::OutputTimePoints => authored || !schedule_stated,
        // Three of the aids are the homotopy chooser's own intent wherever the
        // form carries one: switching a ramp on here and naming it there are
        // two editors of one fact on one form, and the chooser is the
        // analysis's own field, so it wins.
        O::GminStepping | O::SourceStepping | O::PseudoTransient => {
            (owns_the_dc_recovery(kind) && !form_owns_the_homotopy(kind)) || authored
        }
        // Arc length continues a sweep past a fold, which is what a DC sweep
        // does and an operating point does not.
        O::ArcLength => matches!(kind, AnalysisKind::DcSweep) || authored,
        O::Itl1 | O::Damping => owns_the_dc_recovery(kind) || authored,
        _ => authored,
    }
}

/// Whether this kind owns how its own DC solve recovers.
const fn owns_the_dc_recovery(kind: AnalysisKind) -> bool {
    matches!(kind, AnalysisKind::OperatingPoint | AnalysisKind::DcSweep)
}

/// Whether this kind's own form carries the homotopy chooser.
///
/// The operating point's does — `Homotopy strategy`, which names the one ramp
/// the solve is to try — so the three ramp switches are that chooser said
/// twice. Asked of the kind rather than read off the draft: what decides this
/// is which fields the form draws, not which ramp is currently selected.
const fn form_owns_the_homotopy(kind: AnalysisKind) -> bool {
    matches!(kind, AnalysisKind::OperatingPoint)
}

/// The word the field's hint slot states about where its value came from.
///
/// Three words, because three is how many origins an option can have once the
/// refused rows are gone: the plan's own block, this analysis's record, or the
/// engine's dialect default where the plan states nothing at all.
pub(super) fn origin_hint(row: &AdvancedOptionRow) -> &'static str {
    if row.authored.is_some() {
        return "override";
    }
    if row.origin == ENGINE_ORIGIN {
        return ENGINE_ORIGIN;
    }
    PLAN_ORIGIN
}

/// What a well opens on, which is nothing where there is no value to open on.
///
/// A bound the plan does not state resolves to the engine's own dialect
/// default, and the catalogue has no number for it — `plan_preset_value` says
/// the words instead. The hint slot already states `engine default`, and those
/// two words inside a numeric well read as a value a reader could edit in
/// place, which is precisely what they are not.
pub(super) fn well_value(row: &AdvancedOptionRow) -> &str {
    if row.authored.is_none() && row.origin == ENGINE_ORIGIN {
        return "";
    }
    &row.effective
}

/// What a well asks for once the reader has let go of it.
///
/// Emptying a well is how a bound returns to the plan, so an empty one clears
/// rather than refusing. A well let go of unchanged asks for nothing: tabbing
/// through a form must not author twenty overrides equal to the policy they
/// came from.
pub(super) fn well_edit(row: &AdvancedOptionRow, text: &str) -> Option<OptionEdit> {
    let text = text.trim();
    if text.is_empty() {
        return row
            .authored
            .is_some()
            .then_some(OptionEdit::Clear(row.option));
    }
    (text != row.effective).then(|| OptionEdit::Set(row.option, text.to_owned()))
}

/// What a switch or a chooser asks for when its setting moves.
///
/// A two-state control has no empty state to clear through, so the plan's own
/// value is the clear: choosing what the plan already resolves to returns the
/// option to the plan rather than authoring an override that departs from
/// nothing. That is what keeps the hint slot honest — a field reading `override`
/// is a field whose value actually differs — and what keeps a reader who
/// flipped a switch from being unable to put it back.
pub(super) fn setting_edit(row: &AdvancedOptionRow, chosen: &str) -> Option<OptionEdit> {
    if chosen.eq_ignore_ascii_case(&row.preset) {
        return row
            .authored
            .is_some()
            .then_some(OptionEdit::Clear(row.option));
    }
    (Some(chosen) != row.authored.as_deref())
        .then(|| OptionEdit::Set(row.option, chosen.to_owned()))
}

/// Commit what the option fields collected, after the form frame has closed.
///
/// Both arms go through the plan-configuration transaction the Solver page
/// owns rather than a second writer here. One writer per fact is what stops the
/// two surfaces that report an override from disagreeing about what one is, and
/// it is where a refusal is announced: `write_numeric_record` states a value the
/// record refuses through the same funnel every refused plan command uses, and
/// `commit_numeric_override` states one the plan refuses.
pub(super) fn commit(app: &mut RSpiceApp, instance: AnalysisInstanceId, edits: &[OptionEdit]) {
    for edit in edits {
        // The outcome is announced by the writer, not here: a second report of
        // one refusal is a second toast for one press.
        let _ = match edit {
            OptionEdit::Set(option, authored) => {
                super::page_solver::write_numeric_record(app, instance, *option, authored.trim())
            }
            OptionEdit::Clear(option) => {
                let cleared = app
                    .state
                    .sim_setup
                    .stable_analysis_plan()
                    .ok()
                    .and_then(|plan| plan.instance(instance))
                    .and_then(|target| target.numeric_override().cloned())
                    .map(|mut record| {
                        record.clear(*option);
                        record
                    });
                match cleared {
                    Some(record) => {
                        super::lifecycle::commit_numeric_override(app, instance, Some(record))
                    }
                    None => Ok(()),
                }
            }
        };
    }
}

///
/// Two surfaces send a reader here — the Solver page's resolution ledger and a
/// refused plan removal — and `page_tests/plan_removal.rs` pins this equal to
/// `workbench::app::REVEAL_BLOCKER` so the two cannot word one hop two ways.
pub(super) const REVEAL_ACTION: &str = "Open in Analyses";

/// Show one analysis on the page that edits it.
///
/// The two facts a hop needs are the route and the instance, and a caller
/// holding either of the surfaces that hop has both. Takes the workbench rather
/// than the application: moving what is on screen touches nothing else, and a
/// handler that asked for the whole application could mutate every subsystem to
/// do it.
pub(super) fn reveal_in_analyses(workbench: &mut WorkbenchState, instance: AnalysisInstanceId) {
    workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
    workbench.active_analysis_instance = Some(instance);
}

#[cfg(test)]
mod field_tests;
#[cfg(test)]
mod tests;
