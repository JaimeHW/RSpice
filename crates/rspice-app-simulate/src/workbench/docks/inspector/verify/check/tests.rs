//! The inspector states each kind of selected check by its own facts.

use egui::{Pos2, Rect, Vec2};

use super::*;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::simulate::test_host::SimulateTestHost;
use crate::workbench::surfaces::verify::checks::order_rows;

/// The SOA fixture's run, read as the frame reads it, with the Checks view's
/// rows in order and the first row of `kind` selected.
fn selecting(kind: fn(&Check) -> bool) -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    crate::workbench::surfaces::verify::raster::seed_soa(&mut harness.model);
    let checks = harness.model.simulation.verification_checks();
    let session = &mut harness.state.verification;
    order_rows(
        &mut session.checks_rows,
        checks,
        harness.model.simulation.verification_revision(),
        false,
    );
    let row = session
        .checks_rows
        .rows
        .iter()
        .position(|index| kind(&checks.rows[*index].check))
        .expect("the fixture holds the kind");
    session.checks_table.select(row, None);
    harness
}

/// Every property row the inspector drew, as (label, value).
fn rows(harness: &FeatureHarness<SimulateFeature>) -> Vec<(String, String)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut rows = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(300.0, 900.0))),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    check(ui, &harness.model, &harness.state.verification);
                });
            },
        );
        rows = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
            .collect();
    }
    rows
}

fn has(rows: &[(String, String)], label: &str, value: &str) -> bool {
    rows.iter()
        .any(|(row_label, row_value)| row_label == label && row_value == value)
}

#[test]
fn an_soa_rule_states_its_limits_and_worst_point() {
    let rows = rows(&selecting(|check| matches!(check, Check::Soa(_))));
    for (label, expected) in [
        ("Limit", value(3.3, "V")),
        ("Warning at", "90 % of limit".to_owned()),
        ("Critical at", "120 % of limit".to_owned()),
        ("Observed worst", value(3.61, "V")),
        ("Margin", "-9.4 %".to_owned()),
        ("When", value(63.0e-9, "s")),
    ] {
        assert!(
            has(&rows, label, &expected),
            "{label} {expected} in {rows:?}"
        );
    }
}

#[test]
fn an_assertion_states_its_card_and_every_stretch() {
    let rows = rows(&selecting(|check| matches!(check, Check::Assertion(_))));
    for (label, expected) in [
        ("Checks", "V(out)".to_owned()),
        ("Analysis", "Transient".to_owned()),
        ("Level", "Error".to_owned()),
        ("Message", "out left its band".to_owned()),
        ("Written at", "line 12".to_owned()),
        (
            "1",
            format!(
                "{} \u{2013} {} \u{00b7} {} at {}",
                value(1.2e-7, "s"),
                value(1.5e-7, "s"),
                value(1.9, ""),
                value(1.35e-7, "s")
            ),
        ),
    ] {
        assert!(
            has(&rows, label, &expected),
            "{label} {expected} in {rows:?}"
        );
    }
    assert!(
        rows.iter().any(|(label, _)| label == "2"),
        "the second stretch is listed too: {rows:?}"
    );
}

#[test]
fn an_hdl_record_states_its_words_time_and_origin() {
    let rows = rows(&selecting(|check| matches!(check, Check::Hdl { .. })));
    for (label, expected) in [
        ("Severity", "Warning".to_owned()),
        ("Text", "bias settled late".to_owned()),
        ("Time", value(2.5e-7, "s")),
        ("Scope", "xtb.ubias".to_owned()),
        ("Source", "tb.v:41".to_owned()),
    ] {
        assert!(
            has(&rows, label, &expected),
            "{label} {expected} in {rows:?}"
        );
    }
}

#[test]
fn nothing_selected_states_nothing_of_a_check() {
    let mut harness = selecting(|check| matches!(check, Check::Soa(_)));
    harness.state.verification.checks_table.selection = None;
    assert_eq!(rows(&harness), Vec::<(String, String)>::new());
}
