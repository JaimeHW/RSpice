//! The Checks view over a run with every kind of check: its rows, their
//! order, the filter, and where a row leads.

use super::*;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;

/// A run of five SOA rules, an `.ASSERT` broken twice, one that held and an
/// HDL `$warning`, read as the frame reads it.
fn checked_run() -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    super::super::raster::seed_soa(&mut harness.model);
    harness
}

/// The shown rows as (target, check, status), in the order the view shows
/// them.
fn shown(harness: &FeatureHarness<SimulateFeature>) -> Vec<(String, String, &'static str)> {
    let checks = harness.model.simulation.verification_checks();
    harness
        .state
        .verification
        .checks_rows
        .rows
        .iter()
        .map(|index| {
            let row = &checks.rows[*index];
            let cells = row_cells(checks, row, false);
            (
                cells[0].text.clone(),
                cells[1].text.clone(),
                status_word(row.status),
            )
        })
        .collect()
}

/// One frame of the Checks view at the review size.
fn draw(harness: &mut FeatureHarness<SimulateFeature>) {
    use super::super::raster::{page_input, page_pass};
    harness
        .wb
        .activate(crate::workbench::state::Workspace::Verify);
    harness.state.verification_page = VerificationPage::Checks;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = page_pass(&ctx, page_input(Vec::new()), |ui| {
        super::super::show(ui, &mut harness.cx());
    });
}

fn owned(rows: &[(&str, &str, &'static str)]) -> Vec<(String, String, &'static str)> {
    rows.iter()
        .map(|(target, check, status)| ((*target).to_owned(), (*check).to_owned(), *status))
        .collect()
}

/// Every check the run evaluated is a row, worst first: the failures, the
/// SOA rule with the least room leading them; then the warnings; then what
/// passed. Each assertion stretch is its own row.
#[test]
fn the_checks_view_lists_every_check_worst_first() {
    let mut harness = checked_run();
    draw(&mut harness);
    assert_eq!(
        shown(&harness),
        owned(&[
            ("M3", "VDS max", "Fail"),
            ("M1", "VDS max", "Fail"),
            ("V(out)", "out_band", "Fail"),
            ("V(out)", "out_band", "Fail"),
            ("M2", "VDS max", "Warning"),
            ("xtb.ubias", "$warning", "Warning"),
            ("M1", "VGS max", "Pass"),
            ("M4", "ID max", "Pass"),
            ("V(out)", "out_floor", "Pass"),
        ])
    );
    let checks = harness.model.simulation.verification_checks();
    assert_eq!(
        checks_summary(checks),
        Summary::failing("4 violations \u{00b7} 2 warnings \u{00b7} 9 checks")
    );
}

/// A row states when the check broke and for how long: an SOA rule's worst
/// time, an assertion stretch's start and length, an HDL record's time. The
/// record's words are the hover hint on its check, and a run of one point
/// names no point.
#[test]
fn each_row_states_its_time_and_duration() {
    let mut harness = checked_run();
    draw(&mut harness);
    let checks = harness.model.simulation.verification_checks();
    assert_eq!(checks.points.len(), 1, "the fixture ran at one point");
    let cells = |target: &str, check: &str| {
        let order = &harness.state.verification.checks_rows;
        order
            .rows
            .iter()
            .map(|index| row_cells(checks, &checks.rows[*index], false))
            .filter(|cells| cells[0].text == target && cells[1].text == check)
            .collect::<Vec<_>>()
    };
    let words = |cells: &[Cell]| {
        cells
            .iter()
            .map(|cell| cell.text.clone())
            .collect::<Vec<_>>()
    };

    let stretches = cells("V(out)", "out_band");
    assert_eq!(stretches.len(), 2);
    assert_eq!(
        words(&stretches[0]),
        [
            "V(out)".to_owned(),
            "out_band".to_owned(),
            format_scalar(1.9),
            "\u{2014}".to_owned(),
            "\u{2014}".to_owned(),
            format_value(1.2e-7, "s"),
            format_value(1.5e-7 - 1.2e-7, "s"),
            "Fail".to_owned(),
        ]
    );
    assert_eq!(stretches[1][5].text, format_value(3.4e-7, "s"));

    let rule = &cells("M3", "VDS max")[0];
    assert_eq!(rule[2].text, format_value(3.61, "V"));
    assert_eq!(rule[3].text, format_value(3.3, "V"));
    assert_eq!(rule[4].text, "-9.4 %");
    assert_eq!(rule[5].text, format_value(63.0e-9, "s"));

    let record = &cells("xtb.ubias", "$warning")[0];
    assert_eq!(record[5].text, format_value(2.5e-7, "s"));
    assert!(
        format!("{:?}", record[1]).contains("hint: Some(\"bias settled late\")"),
        "the record's words are its check's hint: {:?}",
        record[1]
    );
}

/// A run of several points names each row's point in a column of its own.
#[test]
fn a_run_of_several_points_names_each_rows_point() {
    assert_eq!(check_columns(false).len(), 8);
    let columns = check_columns(true);
    assert_eq!(
        columns
            .iter()
            .map(|column| column.head.as_str())
            .collect::<Vec<_>>(),
        [
            "Target", "Check", "Observed", "Limit", "Margin", "When", "Duration", "Point", "Status"
        ]
    );
}

/// Violations only shows the checks the summary counts as violated: a warning
/// is not a violation, so it is left out with what passed.
#[test]
fn violations_only_shows_the_violated_checks() {
    let mut harness = checked_run();
    harness.state.verification.checks_violations_only = true;
    draw(&mut harness);
    let rows = shown(&harness);
    let counts = harness.model.simulation.verification_checks().counts();
    assert_eq!(rows.len(), counts.violations);
    assert!(rows.iter().all(|(_, _, status)| *status == "Fail"));
}

/// The shown order is built once per revision of the run's checks and filter:
/// a frame that changed neither orders nothing.
#[test]
fn an_idle_frame_orders_nothing() {
    let mut harness = checked_run();
    draw(&mut harness);
    let built = harness.state.verification.checks_rows.clone();
    let checks = harness.model.simulation.verification_checks();
    let revision = harness.model.simulation.verification_revision();
    let mut order = CheckRowOrder {
        rows: vec![usize::MAX],
        ..built.clone()
    };
    order_rows(&mut order, checks, revision, false);
    assert_eq!(order.rows, [usize::MAX], "nothing changed, nothing rebuilt");
    order_rows(&mut order, checks, revision.wrapping_add(1), false);
    assert_eq!(order.rows, built.rows, "a new revision is ordered again");
}

/// A clean run says how many checks it ran, neutral; one whose checks only
/// warned says so too, and how many warned, behind a warning's mark.
#[test]
fn a_clean_run_summary_is_neutral() {
    let mut harness = checked_run();
    draw(&mut harness);
    let mut checks = harness.model.simulation.verification_checks().clone();
    checks.rows.retain(|row| !row.status.is_violation());
    assert_eq!(
        checks_summary(&checks),
        Summary::warning("5 checks, none violated \u{00b7} 2 warnings")
    );
    checks
        .rows
        .retain(|row| matches!(row.status, CheckStatus::Pass));
    assert_eq!(
        checks_summary(&checks),
        Summary::plain("3 checks, none violated")
    );
}

/// An SOA row leads to its device on the schematic, any other to the deck;
/// both lead to Results.
#[test]
fn a_row_menu_leads_to_where_the_check_is() {
    let mut harness = checked_run();
    draw(&mut harness);
    let checks = harness.model.simulation.verification_checks();
    let labels = |kind: fn(&Check) -> bool| {
        let index = checks
            .rows
            .iter()
            .position(|row| kind(&row.check))
            .expect("the fixture holds the kind");
        menu_for(&checks.rows[index], index)
    };
    let soa = labels(|check| matches!(check, Check::Soa(_)));
    assert_eq!(soa[0], ("Show in schematic", Leave::Schematic("M1".into())));
    assert_eq!(soa[1].0, "Open in Results");
    let assertion = labels(|check| matches!(check, Check::Assertion(_)));
    assert_eq!(assertion[0], ("Show in deck", Leave::Deck));
    let record = labels(|check| matches!(check, Check::Hdl { .. }));
    assert_eq!(record[0], ("Show in deck", Leave::Deck));

    show_in_deck(&mut harness.cx()).expect("the run is there to open");
    let request = harness
        .wb
        .outbox
        .take::<RevealExecutedDeck>()
        .expect("the deck is asked for");
    assert_eq!(request.seq, 3);
    assert!(!request.close_manager_on_success);
}

#[test]
fn soa_cross_probe_selects_the_exact_schematic_device() {
    let mut harness = SimulateTestHost::harness();
    harness
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            crate::state::Component::new(
                42,
                crate::state::ComponentType::Nmos,
                crate::state::Point::new(10, 20),
            )
            .with_name_value("M1", "NMOS"),
        );

    cross_probe_soa_device(&mut harness.cx(), "m1")
        .expect("device identity cross-probes case-insensitively");

    assert!(
        harness
            .model
            .schematic
            .session
            .editor
            .selection
            .has_component(42)
    );
    assert_eq!(
        harness.wb.workspace,
        crate::workbench::state::Workspace::Design
    );
    assert!(cross_probe_soa_device(&mut harness.cx(), "M404").is_err());
}
