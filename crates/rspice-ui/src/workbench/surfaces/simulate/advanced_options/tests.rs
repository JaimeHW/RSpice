//! The panel's rows are judged, not painted.
//!
//! Everything worth pinning about this surface is in [`super::sections`]: what
//! it reports for an untouched analysis, for an authored one, and for an
//! option the kind cannot carry. The rendering is a thin read of those rows.

use super::*;

use crate::simulation::plan::NumericOverrideOption as O;

fn rows_for(
    kind: AnalysisKind,
    record: Option<&AnalysisNumericOverride>,
) -> Vec<(O, String, &'static str)> {
    let options = SimulationOptions::default();
    sections(kind, record, &options)
        .into_iter()
        .flat_map(|section| section.rows)
        .map(|row| (row.option, row.effective, row.origin))
        .collect()
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
        .set(AnalysisKind::Ac, O::Reltol, "4e-9")
        .expect("every kind carries an update bound");

    let rows = rows_for(AnalysisKind::Ac, Some(&record));
    assert_eq!(origin_of(&rows, O::Reltol), OVERRIDE_ORIGIN);
    assert_eq!(
        rows.iter()
            .find(|(option, _, _)| *option == O::Reltol)
            .map(|(_, effective, _)| effective.as_str()),
        Some("4e-9"),
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
        .set(AnalysisKind::Transient, O::Itl4, "12")
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

/// The sections arrive in catalog order and none of them is empty.
#[test]
fn the_sections_are_ordered_and_populated() {
    let options = SimulationOptions::default();
    let built = sections(AnalysisKind::Transient, None, &options);
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
            .set(AnalysisKind::Ac, option, authored)
            .unwrap_or_else(|error| panic!("{} is authorable: {error}", option.key()));
    }
    let options = SimulationOptions::default();
    let authored = sections(AnalysisKind::Ac, Some(&record), &options)
        .into_iter()
        .flat_map(|section| section.rows)
        .filter(|row| row.authored.is_some())
        .count();
    assert_eq!(authored, 3);
}
