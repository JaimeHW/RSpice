//! The pane legend on the SAR converter's run (C4), judged through the
//! frame the reader sees: every control it draws says what it is.

use super::conversion::add_expression;
use super::lanes::{FRAME, sar_strip, theme_context};
use super::*;
use egui::accesskit::Role;
use rspice_results::visualization_document::BusRadix;

/// Every control the settled frame publishes, by its role and its name.
fn controls(ctx: &egui::Context, state: &mut ResultsTestHost) -> Vec<(Role, Option<String>)> {
    ctx.enable_accesskit();
    let mut output = None;
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
            ..Default::default()
        };
        output = Some(ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
        }));
    }
    output
        .and_then(|output| output.platform_output.accesskit_update)
        .map(|update| {
            update
                .nodes
                .iter()
                .map(|(_, node)| (node.role(), node.label().map(str::to_owned)))
                .collect()
        })
        .unwrap_or_default()
}

/// The `analog()` staircase is a plain number, and its pane names no unit:
/// the axis goes unmarked, and the header draws no tag with nothing on it.
#[test]
fn a_plain_numbers_pane_draws_no_blank_control() {
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, "analog(D(code))");
    let ctx = theme_context();
    let controls = controls(&ctx, &mut state);
    let blank: Vec<_> = controls
        .iter()
        .filter(|(role, label)| {
            *role == Role::Button && label.as_deref().is_none_or(|label| label.trim().is_empty())
        })
        .collect();
    assert!(
        controls
            .iter()
            .any(|(_, label)| label.as_deref() == Some("V")),
        "the volts pane keeps its tag: {controls:?}"
    );
    assert!(blank.is_empty(), "a button says nothing: {blank:?}");
}
