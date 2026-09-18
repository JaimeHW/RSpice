//! Which options a form offers, what their hints say, and what an edit does.
//!
//! The sibling file judges the rows themselves — what one analysis resolves
//! every option to. This judges the partition and the commit: which of those
//! rows become fields on which kind's form, what the hint slot says about each,
//! and what a field edit asks the plan for.
//!
//! The partition is the part worth pinning hardest. A global solver bound is
//! the plan's policy and belongs to the Solver page; the same number resolved
//! per analysis is not a feature but a way for one deck to disagree with
//! itself. The one exception is legacy data, and it is an exception precisely
//! because dropping it would change what a saved project resolves to.

use super::*;

use crate::simulation::plan::{AnalysisKind, NumericOverrideOption as O, SolverOwnership};
use crate::workbench::state::SimulationPage;

/// The test instance, with one analysis of `kind` selected on the Analyses
/// route.
fn studio(kind: AnalysisKind) -> (RSpiceApp, AnalysisInstanceId) {
    let mut app = RSpiceApp::test_instance();
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test instance holds a stable plan");
    let instance = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == kind)
        .map(|instance| instance.id())
        .unwrap_or_else(|| {
            plan.insert(kind)
                .map(|(id, _)| id)
                .unwrap_or_else(|error| panic!("{kind:?} inserts into the default plan: {error}"))
        });
    app.state.workbench.simulation_page = SimulationPage::Analyses;
    app.state.workbench.active_analysis_instance = Some(instance);
    (app, instance)
}

/// Every option field one analysis's form offers, as `(option, origin hint)`.
fn offered(app: &RSpiceApp, instance: AnalysisInstanceId) -> Vec<(O, &'static str)> {
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance holds a stable plan");
    let target = plan
        .instance(instance)
        .expect("the instance is in the plan");
    form_rows(
        target.kind(),
        target.draft(),
        target.numeric_override(),
        &app.state.sim_setup.options,
    )
    .into_iter()
    .flat_map(|section| section.rows)
    .map(|row| (row.option, origin_hint(&row)))
    .collect()
}

/// One option's row on one analysis, whether or not a form offers it.
fn row_of(app: &RSpiceApp, instance: AnalysisInstanceId, option: O) -> AdvancedOptionRow {
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance holds a stable plan");
    let target = plan
        .instance(instance)
        .expect("the instance is in the plan");
    sections(
        target.kind(),
        target.draft(),
        target.numeric_override(),
        &app.state.sim_setup.options,
    )
    .into_iter()
    .flat_map(|section| section.rows)
    .find(|row| row.option == option)
    .unwrap_or_else(|| panic!("{} earns a row", option.key()))
}

/// The authored bound one analysis currently carries for one option.
fn authored(app: &RSpiceApp, instance: AnalysisInstanceId, option: O) -> Option<String> {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| plan.instance(instance))
        .and_then(|target| target.numeric_override())
        .and_then(|record| record.value(option))
}

/// The revision the plan currently stands at.
fn revision(app: &RSpiceApp) -> crate::product::ObjectRevision {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance holds a stable plan")
        .revision()
}

/// Every global solver bound the plan owns and no analysis form offers.
const GLOBAL: [O; 11] = [
    O::Reltol,
    O::Abstol,
    O::Vntol,
    O::ResidualReltol,
    O::Gmin,
    O::Pivrel,
    O::Pivtol,
    O::Solver,
    O::Bypass,
    O::BypassReltol,
    O::BypassAbstol,
];

/// A kind's form offers the options that kind owns, and nothing else.
///
/// This is the partition, asserted on both sides for both halves of it. A
/// transient owns how time advances; an operating point owns how its solve
/// recovers; neither owns the plan's global bounds, and a form that offered one
/// would be offering to resolve one number twenty ways.
#[test]
fn a_form_offers_the_options_its_own_kind_owns() {
    let (app, transient) = studio(AnalysisKind::Transient);
    let offered_on_the_transient: Vec<O> = offered(&app, transient)
        .into_iter()
        .map(|(option, _)| option)
        .collect();
    assert_eq!(
        offered_on_the_transient,
        vec![
            O::Itl4,
            O::Chgtol,
            O::Trtol,
            O::IntegrationMethod,
            O::LteReltol,
            O::LteAbstol,
            O::MinTimestep,
            O::TransientNewtonReltol,
            O::TransientNewtonAbstol,
            O::TransientNewtonUpdateBound,
            O::TransientNewtonResidualBound,
            O::TransientNewtonBudget,
            O::TransientDeviceConvergence,
            O::TransientNoxSolver,
            O::StrobeInterval,
            O::OutputTimePoints,
            O::RetainEverySignal,
        ],
        "a transient owns how time advances, how each step's solve is accepted, and which of \
         the accepted steps are reported"
    );

    // The harmonic-balance family's own package, on the family's own forms and
    // nowhere else. Its solve is where `TAHB` is read, so it is the only form
    // that can offer a control for it. Asked of the kind's own draft rather
    // than of a plan instance, because two of the three carry a prerequisite
    // and what decides this is the kind, not whether a plan admits one.
    for kind in [
        AnalysisKind::HarmonicBalance,
        AnalysisKind::Hbsp,
        AnalysisKind::Hbnoise,
    ] {
        let draft = AnalysisDraft::for_kind(kind);
        let offered: Vec<O> = form_rows(kind, &draft, None, &SimulationOptions::default())
            .into_iter()
            .flat_map(|section| section.rows)
            .map(|row| row.option)
            .collect();
        assert_eq!(
            offered,
            vec![O::HbInitialState],
            "{} owns how its harmonic-balance solve starts and nothing else",
            kind.label()
        );
    }
    assert!(
        !offered_on_the_transient.contains(&O::HbInitialState),
        "a transient runs no harmonic-balance solve"
    );

    let (app, operating_point) = studio(AnalysisKind::OperatingPoint);
    let offered_on_the_op: Vec<O> = offered(&app, operating_point)
        .into_iter()
        .map(|(option, _)| option)
        .collect();
    assert_eq!(
        offered_on_the_op,
        vec![O::Damping],
        "the operating point's own form carries the homotopy chooser, so the three ramp \
         switches would be that chooser said twice; its Newton budget belongs to its \
         accuracy tier; and arc length continues a sweep, which it is not"
    );

    let (app, dc_sweep) = studio(AnalysisKind::DcSweep);
    let offered_on_the_sweep: Vec<O> = offered(&app, dc_sweep)
        .into_iter()
        .map(|(option, _)| option)
        .collect();
    assert_eq!(
        offered_on_the_sweep,
        vec![
            O::Itl1,
            O::GminStepping,
            O::SourceStepping,
            O::PseudoTransient,
            O::ArcLength,
            O::Damping,
        ],
        "a DC sweep carries neither a tier nor a homotopy chooser, so every part of how \
         its solve recovers is its own"
    );

    for kind in [
        AnalysisKind::Transient,
        AnalysisKind::OperatingPoint,
        AnalysisKind::DcSweep,
        AnalysisKind::Ac,
    ] {
        let (app, instance) = studio(kind);
        let offered: Vec<O> = offered(&app, instance)
            .into_iter()
            .map(|(option, _)| option)
            .collect();
        for global in GLOBAL {
            assert!(
                !offered.contains(&global),
                "{kind:?} offers {}, which is the plan's own policy",
                global.key()
            );
        }
        assert!(
            !offered.contains(&O::MaximumTimestep),
            "{kind:?} would repeat its own Max step field"
        );
    }
}

/// An analysis with nothing authored states the plan as every field's origin.
#[test]
fn an_untouched_form_states_the_plan_as_every_origin() {
    let (app, transient) = studio(AnalysisKind::Transient);
    // The plan states no truncation bound of its own, and it states no policy
    // at all for the two packages the engine reads straight off the deck: an
    // output schedule and a transient Newton bound belong to one analysis, not
    // to the whole deck, so there is no plan field for them to depart from.
    // Those rows resolve to the engine's own default and the hint says so
    // rather than naming a policy the plan does not hold.
    const ENGINE_DEFAULTED: [O; 12] = [
        O::LteReltol,
        O::LteAbstol,
        O::TransientNewtonReltol,
        O::TransientNewtonAbstol,
        O::TransientNewtonUpdateBound,
        O::TransientNewtonResidualBound,
        O::TransientNewtonBudget,
        O::TransientDeviceConvergence,
        O::TransientNoxSolver,
        O::StrobeInterval,
        O::OutputTimePoints,
        O::RetainEverySignal,
    ];
    for (option, hint) in offered(&app, transient) {
        let expected = if ENGINE_DEFAULTED.contains(&option) {
            ENGINE_ORIGIN
        } else {
            PLAN_ORIGIN
        };
        assert_eq!(hint, expected, "{}", option.key());
    }
}

/// A bound the plan does not state opens on an empty well.
///
/// Its effective value is a pair of words rather than a number — the engine's
/// own dialect default stands — and the hint slot is where that belongs. Inside
/// a numeric well the same words read as a value a reader could edit, which is
/// exactly what they are not.
#[test]
fn a_bound_the_plan_does_not_state_opens_on_an_empty_well() {
    let (mut app, transient) = studio(AnalysisKind::Transient);
    let inherited = row_of(&app, transient, O::LteReltol);
    assert_eq!(origin_hint(&inherited), ENGINE_ORIGIN);
    assert_eq!(
        inherited.effective, ENGINE_ORIGIN,
        "the row states the words"
    );
    assert_eq!(well_value(&inherited), "", "the well states nothing");
    assert_eq!(
        well_edit(&inherited, ""),
        None,
        "and an empty well over an inherited bound asks for nothing"
    );

    // A number typed into it is an override like any other, and the well then
    // opens on what the solve will use.
    commit(
        &mut app,
        transient,
        &[OptionEdit::Set(O::LteReltol, "2e-3".to_owned())],
    );
    let authored = row_of(&app, transient, O::LteReltol);
    assert_eq!(origin_hint(&authored), "override");
    assert_eq!(well_value(&authored), authored.effective);
    assert!(!well_value(&authored).is_empty());
}

/// A well let go of with a new value authors an override; emptied, it clears.
///
/// Both halves of one rule, and the hint is what a reader sees of it: the field
/// reads `override` for exactly as long as the analysis states a value, and
/// `plan policy` again the moment it stops.
#[test]
fn a_well_authors_what_it_holds_and_clears_when_it_is_emptied() {
    let (mut app, transient) = studio(AnalysisKind::Transient);
    let before = revision(&app);

    let inherited = row_of(&app, transient, O::Trtol);
    assert_eq!(origin_hint(&inherited), PLAN_ORIGIN);
    let edit = well_edit(&inherited, " 4 ").expect("a new value asks to be authored");
    assert_eq!(edit, OptionEdit::Set(O::Trtol, "4".to_owned()));
    commit(&mut app, transient, &[edit]);

    assert_eq!(authored(&app, transient, O::Trtol).as_deref(), Some("4"));
    assert_ne!(
        revision(&app),
        before,
        "a committed override advances the plan, which is what invalidates preflight"
    );
    let overridden = row_of(&app, transient, O::Trtol);
    assert_eq!(origin_hint(&overridden), "override");
    assert_eq!(overridden.effective, "4");

    // Let go of unchanged, a well asks for nothing: tabbing through a form must
    // not author every field it passes.
    assert_eq!(well_edit(&overridden, "4"), None);

    let cleared = well_edit(&overridden, "   ").expect("an emptied well asks to be cleared");
    assert_eq!(cleared, OptionEdit::Clear(O::Trtol));
    commit(&mut app, transient, &[cleared]);
    assert_eq!(authored(&app, transient, O::Trtol), None);
    let inherited = row_of(&app, transient, O::Trtol);
    assert_eq!(origin_hint(&inherited), PLAN_ORIGIN);
    assert_eq!(
        inherited.effective,
        super::super::page_solver::plan_preset_value(O::Trtol, &app.state.sim_setup.options),
        "and the field states the plan's own value once more"
    );

    // An inherited well emptied asks for nothing at all, because there is
    // nothing to return to the plan.
    assert_eq!(well_edit(&inherited, ""), None);
}

/// A value the record refuses is announced once and costs the plan nothing.
///
/// Through the same funnel every refused plan command uses, so one refused
/// field is one toast and one Console line. The well reverts on its own: its
/// text is only retained while it is being typed into, so the next frame seeds
/// it from the value the solve is actually using.
#[test]
fn a_refused_value_is_announced_once_and_leaves_the_plan_alone() {
    let (mut app, transient) = studio(AnalysisKind::Transient);
    let before = revision(&app);
    let announced = app.state.workbench.analysis_lifecycle_status.sequence();

    commit(
        &mut app,
        transient,
        &[OptionEdit::Set(O::Trtol, "banana".to_owned())],
    );

    assert_eq!(authored(&app, transient, O::Trtol), None);
    assert_eq!(
        revision(&app),
        before,
        "a refused value is not a plan change"
    );
    assert!(
        app.state.workbench.analysis_lifecycle_status.is_refusal(),
        "the refusal is stated, not swallowed"
    );
    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .contains("TRTOL"),
        "and it names the option it refused: {}",
        app.state.workbench.analysis_lifecycle_status.message()
    );
    assert_eq!(
        app.state.workbench.analysis_lifecycle_status.sequence(),
        announced + 1,
        "one refused field is one announcement, not two"
    );
    assert_eq!(
        row_of(&app, transient, O::Trtol).effective,
        super::super::page_solver::plan_preset_value(O::Trtol, &app.state.sim_setup.options),
        "the field states what the solve uses, which the refusal did not change"
    );
}

/// A switch or chooser moved to the plan's own setting clears its override.
///
/// A two-state control has no empty state to clear through, so the plan's value
/// is the clear. Without it a reader who flipped a switch could never put it
/// back: the flag would read `override` for ever, over a value identical to the
/// policy it departed from.
#[test]
fn a_setting_returned_to_the_plan_clears_its_override() {
    let (mut app, operating_point) = studio(AnalysisKind::OperatingPoint);
    let inherited = row_of(&app, operating_point, O::GminStepping);
    assert_eq!(origin_hint(&inherited), PLAN_ORIGIN);
    let departed = if inherited.preset.eq_ignore_ascii_case("on") {
        "off"
    } else {
        "on"
    };

    let preset = inherited.preset.clone();
    // Choosing what the plan already resolves to asks for nothing while the
    // analysis states nothing.
    assert_eq!(setting_edit(&inherited, &preset), None);

    let edit = setting_edit(&inherited, departed).expect("a departure asks to be authored");
    assert_eq!(edit, OptionEdit::Set(O::GminStepping, departed.to_owned()));
    commit(&mut app, operating_point, &[edit]);
    let overridden = row_of(&app, operating_point, O::GminStepping);
    assert_eq!(origin_hint(&overridden), "override");

    let back = setting_edit(&overridden, &preset)
        .expect("the plan's own setting is how a switch returns to it");
    assert_eq!(back, OptionEdit::Clear(O::GminStepping));
    commit(&mut app, operating_point, &[back]);
    assert_eq!(authored(&app, operating_point, O::GminStepping), None);
    assert_eq!(
        origin_hint(&row_of(&app, operating_point, O::GminStepping)),
        PLAN_ORIGIN
    );
}

/// A legacy override of a global option stays visible until it is cleared.
///
/// A project saved before the partition can hold one, and dropping it silently
/// would change what the run resolves to. So it earns a field on the form for
/// exactly as long as the analysis states it — readable, clearable, and gone the
/// moment it is cleared — while no form offers to author a new one.
#[test]
fn a_legacy_override_of_a_global_option_stays_until_it_is_cleared() {
    let (mut app, transient) = studio(AnalysisKind::Transient);
    assert!(
        !offered(&app, transient)
            .iter()
            .any(|(option, _)| *option == O::Reltol),
        "no form offers to author the plan's update bound"
    );

    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            O::Reltol,
            "4e-9",
        )
        .expect("every kind carries an update bound");
    app.state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test instance holds a stable plan")
        .set_numeric_override(transient, Some(record))
        .expect("the plan accepts a bound the transient can use");

    assert_eq!(
        offered(&app, transient)
            .into_iter()
            .find(|(option, _)| *option == O::Reltol)
            .map(|(_, hint)| hint),
        Some("override"),
        "a stated global bound is a field, and its hint says who stated it"
    );

    commit(&mut app, transient, &[OptionEdit::Clear(O::Reltol)]);
    assert_eq!(authored(&app, transient, O::Reltol), None);
    assert!(
        !offered(&app, transient)
            .iter()
            .any(|(option, _)| *option == O::Reltol),
        "and once it is cleared the field is gone with it"
    );
}
