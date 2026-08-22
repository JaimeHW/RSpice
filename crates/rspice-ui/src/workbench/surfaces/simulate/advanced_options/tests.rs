//! The panel's rows are judged, not painted.
//!
//! Everything worth pinning about this surface is in [`super::sections`]: what
//! it reports for an untouched analysis, for an authored one, and for an
//! option the kind cannot carry. The rendering is a thin read of those rows.

use super::*;

use crate::simulation::accuracy::AnalysisAccuracy;
use crate::simulation::dialog::OpHomotopy;
use crate::simulation::plan::NumericOverrideOption as O;

fn rows_for(
    kind: AnalysisKind,
    record: Option<&AnalysisNumericOverride>,
) -> Vec<(O, String, &'static str)> {
    rows_with(
        kind,
        &AnalysisDraft::for_kind(kind),
        record,
        &SimulationOptions::default(),
    )
}

/// An operating-point draft carrying the two controls that own solver options.
///
/// Built rather than injected: the panel is handed a draft and derives the
/// ownership from it, so a test that supplied the ownership directly would be
/// exercising one step less than the surface does.
fn op_draft(accuracy: AnalysisAccuracy, homotopy: OpHomotopy) -> AnalysisDraft {
    let mut draft = AnalysisDraft::for_kind(AnalysisKind::OperatingPoint);
    let AnalysisDraft::OperatingPoint(setup) = &mut draft else {
        unreachable!("for_kind returns the draft of the kind it was given");
    };
    setup.accuracy_idx = AnalysisAccuracy::ALL
        .iter()
        .position(|tier| *tier == accuracy)
        .expect("every tier is in ALL");
    setup.homotopy_idx = OpHomotopy::ALL
        .iter()
        .position(|choice| *choice == homotopy)
        .expect("every homotopy choice is in ALL");
    draft
}

fn rows_with(
    kind: AnalysisKind,
    draft: &AnalysisDraft,
    record: Option<&AnalysisNumericOverride>,
    options: &SimulationOptions,
) -> Vec<(O, String, &'static str)> {
    sections(kind, draft, record, options)
        .into_iter()
        .flat_map(|section| section.rows)
        .map(|row| (row.option, row.effective, row.origin))
        .collect()
}

fn effective_of(rows: &[(O, String, &'static str)], option: O) -> String {
    rows.iter()
        .find(|(candidate, _, _)| *candidate == option)
        .map(|(_, effective, _)| effective.clone())
        .unwrap_or_else(|| panic!("{} must earn a row", option.key()))
}

fn origin_of(rows: &[(O, String, &'static str)], option: O) -> &'static str {
    rows.iter()
        .find(|(candidate, _, _)| *candidate == option)
        .map(|(_, _, origin)| *origin)
        .unwrap_or_else(|| panic!("{} must earn a row", option.key()))
}

/// Every option earns a row, whether or not the analysis touched it.
///
/// A panel that listed only authored options would answer "what did I
/// change", which the ledger above it already answers. This one answers "what
/// will this analysis actually use", so an untouched option is exactly as
/// interesting as a changed one.
#[test]
fn every_catalog_option_earns_a_row_for_every_kind() {
    for kind in AnalysisKind::ALL {
        let rows = rows_for(kind, None);
        assert_eq!(
            rows.len(),
            NumericOverrideOption::all().count(),
            "{} must state every option, including the ones it cannot carry",
            kind.label()
        );
        for (_, effective, origin) in &rows {
            assert!(
                !effective.is_empty(),
                "{} left a row with no effective value",
                kind.label()
            );
            assert!(!origin.is_empty());
        }
    }
}

/// An untouched analysis resolves to the plan, and says so.
#[test]
fn an_untouched_analysis_reports_the_plan_as_the_owner() {
    let rows = rows_for(AnalysisKind::Ac, None);
    assert_eq!(origin_of(&rows, O::Reltol), PLAN_ORIGIN);
    assert_eq!(origin_of(&rows, O::Pivtol), PLAN_ORIGIN);
    assert_eq!(origin_of(&rows, O::Damping), PLAN_ORIGIN);

    // The plan states no LTE bound by default, so the engine's own dialect
    // default stands and the row says that rather than inventing a number.
    // Asked of a kind that actually steps: on an AC sweep the same row is a
    // refusal, which the refusal test covers.
    let stepping = rows_for(AnalysisKind::Fourier, None);
    assert_eq!(origin_of(&stepping, O::LteReltol), ENGINE_ORIGIN);
}

/// An authored option reports itself as the owner and shows its own value.
#[test]
fn an_authored_option_outranks_the_plan_in_its_own_row() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(AnalysisKind::Ac, SolverOwnership::NONE, O::Reltol, "4e-9")
        .expect("every kind carries an update bound");

    let rows = rows_for(AnalysisKind::Ac, Some(&record));
    assert_eq!(origin_of(&rows, O::Reltol), OVERRIDE_ORIGIN);
    assert_eq!(
        rows.iter()
            .find(|(option, _, _)| *option == O::Reltol)
            .map(|(_, effective, _)| effective.as_str()),
        Some("4n"),
        "the row must show the authored value, not the plan's"
    );
    // Its neighbours are untouched and still follow the plan.
    assert_eq!(origin_of(&rows, O::Vntol), PLAN_ORIGIN);
}

/// A refused option states who owns it instead, in place.
///
/// This is the tier-ownership row the Solver page's ledger also carries: the
/// operating point's Newton budget belongs to its accuracy tier, which is
/// applied after the deck's options, so an ITL1 authored here would be
/// overwritten before the first Newton step.
#[test]
fn a_refused_option_states_its_owner_rather_than_disappearing() {
    let rows = rows_for(AnalysisKind::OperatingPoint, None);
    assert_eq!(
        origin_of(&rows, O::Itl1),
        NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS,
        "the accuracy tier's ownership must be stated where a reader looks for ITL1"
    );

    // And a time-integration bound on a kind that never steps.
    let refusal = O::Chgtol
        .refusal_for(AnalysisKind::OperatingPoint)
        .expect("an operating point never advances time");
    assert_eq!(origin_of(&rows, O::Chgtol), refusal);
}

/// A restored record holding an option its kind cannot carry reports the
/// refusal, not the stored number.
///
/// A project can be edited into this state — an analysis authored under one
/// kind and then re-pointed — and the solve ignores the value. Showing it as
/// the effective value would be the exact lie the record exists to prevent.
#[test]
fn a_stored_but_refused_value_is_never_reported_as_effective() {
    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Transient,
            SolverOwnership::NONE,
            O::Itl4,
            "12",
        )
        .expect("a transient takes timesteps");

    let rows = rows_for(AnalysisKind::Ac, Some(&record));
    let (_, effective, origin) = rows
        .iter()
        .find(|(option, _, _)| *option == O::Itl4)
        .expect("ITL4 still earns a row");
    assert_ne!(effective, "12", "the solve does not use this value");
    assert_eq!(
        *origin,
        O::Itl4
            .refusal_for(AnalysisKind::Ac)
            .expect("an AC sweep never steps")
    );
}

/// A continuation aid the instance's own tier assigns states the tier's value.
///
/// The plan preset is exactly the number the solve is about to discard, so a
/// panel that showed it would be stating a policy no run resolves to.
#[test]
fn an_owned_continuation_aid_states_the_owner_value_not_the_plan_preset() {
    let draft = op_draft(AnalysisAccuracy::Robust, OpHomotopy::Adaptive);
    let robust = draft.solver_ownership();
    let rows = rows_with(
        AnalysisKind::OperatingPoint,
        &draft,
        None,
        &SimulationOptions::default(),
    );
    for option in [
        O::GminStepping,
        O::SourceStepping,
        O::PseudoTransient,
        O::ArcLength,
    ] {
        assert_eq!(
            origin_of(&rows, option),
            O::GminStepping
                .refusal_for_instance(AnalysisKind::OperatingPoint, robust)
                .expect("Robust owns every aid"),
            "{} must name the tier that assigns it",
            option.key()
        );
        assert_eq!(
            effective_of(&rows, option),
            "on",
            "{} is what Robust leaves in the configuration the engine is built from",
            option.key()
        );
    }
    assert_eq!(effective_of(&rows, O::Damping), "Combined");

    // The homotopy control is applied after the tier, so it is the owner the
    // refusal names — and gmin stepping is the aid it selected, not all four.
    let draft = op_draft(AnalysisAccuracy::Robust, OpHomotopy::GminStepping);
    let rows = rows_with(
        AnalysisKind::OperatingPoint,
        &draft,
        None,
        &SimulationOptions::default(),
    );
    assert_eq!(effective_of(&rows, O::GminStepping), "on");
    assert_eq!(effective_of(&rows, O::SourceStepping), "off");
    assert_eq!(effective_of(&rows, O::ArcLength), "off");
    // Damping is still the tier's: no homotopy choice touches it.
    assert_eq!(effective_of(&rows, O::Damping), "Combined");
}

/// A tier that inherits owns nothing, so the aids stay authorable.
#[test]
fn a_balanced_adaptive_instance_may_still_author_its_continuation_aids() {
    let draft = op_draft(AnalysisAccuracy::Balanced, OpHomotopy::Adaptive);
    let inheriting = draft.solver_ownership();
    for option in [
        O::GminStepping,
        O::SourceStepping,
        O::PseudoTransient,
        O::ArcLength,
        O::Damping,
    ] {
        assert_eq!(
            option.refusal_for_instance(AnalysisKind::OperatingPoint, inheriting),
            None,
            "{} is not overwritten under Balanced/Adaptive, so it must stay authorable",
            option.key()
        );
    }
    let rows = rows_with(
        AnalysisKind::OperatingPoint,
        &draft,
        None,
        &SimulationOptions::default(),
    );
    let origin = origin_of(&rows, O::GminStepping);
    assert!(
        origin == PLAN_ORIGIN || origin == ENGINE_ORIGIN,
        "an inheriting instance's aid follows the resolved policy, not an owner: {origin}"
    );
}

/// The sections arrive in catalog order and none of them is empty.
#[test]
fn the_sections_are_ordered_and_populated() {
    let options = SimulationOptions::default();
    let built = sections(
        AnalysisKind::Transient,
        &AnalysisDraft::for_kind(AnalysisKind::Transient),
        None,
        &options,
    );
    let titles: Vec<&str> = built
        .iter()
        .map(|section| section.section.title())
        .collect();
    assert_eq!(
        titles,
        vec![
            "Convergence",
            "Charge",
            "Integration",
            "Matrix",
            "Device bypass"
        ]
    );
    for section in &built {
        assert!(
            !section.rows.is_empty(),
            "{} arrived empty",
            section.section.title()
        );
    }
}

/// The panel's own count of departures matches the record.
#[test]
fn the_departure_count_follows_the_authored_options() {
    let mut record = AnalysisNumericOverride::default();
    for (option, authored) in [(O::Reltol, "1e-5"), (O::Pivtol, "2e-14"), (O::Gmin, "0")] {
        record
            .set_for_instance(AnalysisKind::Ac, SolverOwnership::NONE, option, authored)
            .unwrap_or_else(|error| panic!("{} is authorable: {error}", option.key()));
    }
    let options = SimulationOptions::default();
    let authored = sections(
        AnalysisKind::Ac,
        &AnalysisDraft::for_kind(AnalysisKind::Ac),
        Some(&record),
        &options,
    )
    .into_iter()
    .flat_map(|section| section.rows)
    .filter(|row| row.authored.is_some())
    .count();
    assert_eq!(authored, 3);
}

/// A refused ITL1 row states the tier's budget, not the plan's ITL1.
///
/// The tier replaces `max_iterations` after the deck resolves, so the plan's
/// number is exactly the one the solve will not use. The value is the same
/// string the Solver page's resolution ledger prints for the same analysis.
#[test]
fn a_refused_iteration_budget_states_the_tier_that_owns_it() {
    let mut draft = AnalysisDraft::for_kind(AnalysisKind::OperatingPoint);
    let AnalysisDraft::OperatingPoint(setup) = &mut draft else {
        panic!("expected an operating-point draft");
    };
    setup.accuracy_idx = crate::simulation::accuracy::AnalysisAccuracy::ALL
        .iter()
        .position(|tier| *tier == crate::simulation::accuracy::AnalysisAccuracy::Robust)
        .expect("Robust is a tier");
    let options = SimulationOptions::default();
    let rows = rows_with(AnalysisKind::OperatingPoint, &draft, None, &options);

    assert_eq!(
        origin_of(&rows, O::Itl1),
        NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS
    );
    assert_eq!(effective_of(&rows, O::Itl1), "500 \u{00b7} Robust");
    assert_ne!(
        effective_of(&rows, O::Itl1),
        options.itl1.to_string(),
        "the plan's ITL1 is the one number this solve will not use"
    );
}

/// A refusal with no owner to name shows an em dash rather than a number.
#[test]
fn a_refused_option_with_no_owner_states_no_value() {
    let rows = rows_for(AnalysisKind::OperatingPoint, None);
    assert_eq!(
        effective_of(&rows, O::Chgtol),
        "\u{2014}",
        "an operating point never advances time, so no charge tolerance runs"
    );
}

/// A refused step ceiling on a transient reads the transient's own form.
///
/// The refusal says the transient owns the control; the value column has to
/// agree, and has to compose the form's value with the plan's the way the
/// engine does (`rspice-core/src/engine/transient.rs:1995-2011`).
#[test]
fn a_refused_step_ceiling_reads_the_transient_form_it_names() {
    let options = SimulationOptions {
        max_timestep: 1.0e-6,
        ..SimulationOptions::default()
    };

    let mut draft = AnalysisDraft::for_kind(AnalysisKind::Transient);
    let AnalysisDraft::Transient(setup) = &mut draft else {
        panic!("expected a transient draft");
    };
    setup.max_step = "auto".to_owned();
    let step_time = setup.step.clone();
    let inherited = rows_with(AnalysisKind::Transient, &draft, None, &options);
    // Not the plan's ceiling. With `auto` the bridge writes no `.tran`
    // max-step, so the deck carries this analysis's own output step time in
    // that field and the engine mins it with `MAXTIMESTEP`: the run steps at
    // the tighter of the two, which for the stock 10 ns transient under a 1 µs
    // plan ceiling is 10 ns. This cell printed the 1 µs.
    assert_eq!(
        effective_of(&inherited, O::MaximumTimestep),
        step_time,
        "an `auto` transient steps at its own step time where that is the tighter bound"
    );
    assert_ne!(
        effective_of(&inherited, O::MaximumTimestep),
        super::super::page_solver::plan_preset_value(O::MaximumTimestep, &options),
        "the plan preset is not what an `auto` transient steps at"
    );

    // And where the step time is the looser of the two, the plan's ceiling is
    // what runs — the same `min`, read from the other side.
    let AnalysisDraft::Transient(setup) = &mut draft else {
        panic!("expected a transient draft");
    };
    setup.step = "1m".to_owned();
    let plan_bound = rows_with(AnalysisKind::Transient, &draft, None, &options);
    assert_eq!(
        effective_of(&plan_bound, O::MaximumTimestep),
        super::super::page_solver::plan_preset_value(O::MaximumTimestep, &options),
    );
    let AnalysisDraft::Transient(setup) = &mut draft else {
        panic!("expected a transient draft");
    };
    setup.step = step_time;

    let AnalysisDraft::Transient(setup) = &mut draft else {
        panic!("expected a transient draft");
    };
    setup.max_step = "1m".to_owned();
    let looser = rows_with(AnalysisKind::Transient, &draft, None, &options);
    assert_eq!(
        effective_of(&looser, O::MaximumTimestep),
        super::super::page_solver::plan_preset_value(O::MaximumTimestep, &options),
        "the engine takes the tighter of the two, so a looser form value never runs"
    );

    let AnalysisDraft::Transient(setup) = &mut draft else {
        panic!("expected a transient draft");
    };
    setup.max_step = "1n".to_owned();
    let tighter = rows_with(AnalysisKind::Transient, &draft, None, &options);
    assert_eq!(effective_of(&tighter, O::MaximumTimestep), "1n");
}

/// An authored step ceiling looser than the plan's is not what the run uses.
///
/// `DELMAX` and `MAXTIMESTEP` reach the engine as two fields and the transient
/// clamps against both, so the panel reports the tighter one and names the plan
/// as the owner — the same answer the Solver page's ledger gives.
#[test]
fn an_authored_step_ceiling_is_reported_after_the_plan_clamps_it() {
    let options = SimulationOptions {
        max_timestep: 1.0e-9,
        ..SimulationOptions::default()
    };

    let mut record = AnalysisNumericOverride::default();
    record
        .set_for_instance(
            AnalysisKind::Pss,
            SolverOwnership::NONE,
            O::MaximumTimestep,
            "1u",
        )
        .expect("a PSS solve advances time and carries its own ceiling");

    let rows = rows_with(
        AnalysisKind::Pss,
        &AnalysisDraft::for_kind(AnalysisKind::Pss),
        Some(&record),
        &options,
    );
    assert_eq!(
        effective_of(&rows, O::MaximumTimestep),
        super::super::page_solver::plan_preset_value(O::MaximumTimestep, &options),
        "the plan's ceiling is tighter, so it is what the run steps at"
    );
    assert_eq!(
        origin_of(&rows, O::MaximumTimestep),
        "plan preset \u{00b7} tighter than the override"
    );

    // The other direction: an override that actually tightens is honoured.
    let mut tighter = AnalysisNumericOverride::default();
    tighter
        .set_for_instance(
            AnalysisKind::Pss,
            SolverOwnership::NONE,
            O::MaximumTimestep,
            "1p",
        )
        .expect("a PSS solve carries its own ceiling");
    let rows = rows_with(
        AnalysisKind::Pss,
        &AnalysisDraft::for_kind(AnalysisKind::Pss),
        Some(&tighter),
        &options,
    );
    assert_eq!(origin_of(&rows, O::MaximumTimestep), OVERRIDE_ORIGIN);
    assert_eq!(effective_of(&rows, O::MaximumTimestep), "1p");
}
