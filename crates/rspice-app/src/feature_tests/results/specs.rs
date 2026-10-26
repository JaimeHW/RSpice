//! The specification table over the limit the studio carried into it: a
//! hop from the Requirements page selects one limit in the simulate
//! workspace, and the table the results workspace draws marks it and
//! scrolls it into view.

use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SpecEntry};
#[cfg(not(target_arch = "wasm32"))]
use crate::state::{SimulationRunLifecycle, SpecPointScope};
use crate::workbench::AppState;
use crate::workbench::documents::result_document::specs::show;
#[cfg(not(target_arch = "wasm32"))]
use rspice_results_ui::specs::test_support::painted_spans;

/// A hop from the studio's Requirements page names one limit. This table
/// and that page read the same selection, so arriving here has to mark
/// the row the reader asked for rather than leaving them to find it.
#[test]
fn the_table_marks_the_limit_a_hop_carried_into_it() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "ac").with_measurements(vec![
            rspice_core::MeasureResult::success("gain_dc", 44.0),
            rspice_core::MeasureResult::success("bandwidth_3db", 1.0e6),
        ]),
    );
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.workspace.content.specs = vec![
        SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: "db20(V(out))".to_owned(),
            min: Some(40.0),
            max: None,
            unit: "dB".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        SpecEntry {
            measurement: "bandwidth_3db".to_owned(),
            expression: "bw(V(out))".to_owned(),
            min: Some(1.0),
            max: None,
            unit: "Hz".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
    ];
    // Matched the way the studio matches: a measurement name is one
    // identity however it was typed.
    state.simulate.selected_specification = Some("BANDWIDTH_3DB".to_owned());

    let selected_specification = state.simulate.selected_specification.clone();
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let nodes = ctx
        .run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(
                        ui,
                        &mut state.results_parts(),
                        selected_specification.as_deref(),
                    );
                });
            },
        )
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    let selected: Vec<&str> = nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Row && node.is_selected() == Some(true)
        })
        .filter_map(|(_, node)| node.label())
        .collect();
    assert_eq!(
        selected.len(),
        1,
        "exactly one row is the one the hop carried"
    );
    assert!(
        selected[0].starts_with("Measurement bandwidth_3db;"),
        "and it is the limit the studio named, not the first row: {selected:?}"
    );
}

/// A workspace holding one long requirement contract and two runs that both
/// measured every limit in it, with the last limit carried in from the studio.
#[cfg(not(target_arch = "wasm32"))]
fn two_runs_over_a_long_contract() -> AppState {
    let names: Vec<String> = (0..60).map(|index| format!("m{index:02}")).collect();
    let mut state = AppState::default();
    for run_number in 1..=2 {
        let mut run = SimulationRun::new(run_number);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "ac").with_measurements(
                names
                    .iter()
                    .enumerate()
                    .map(|(index, name)| {
                        rspice_core::MeasureResult::success(name, 1.0 + index as f64)
                    })
                    .collect(),
            ),
        );
        state.model.simulation.retained.runs.push(run);
    }
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.workspace.content.specs = names
        .iter()
        .map(|name| SpecEntry {
            measurement: name.clone(),
            expression: format!("max V({name})"),
            min: Some(0.0),
            max: None,
            unit: "V".to_owned(),
            scope: SpecPointScope::AllPoints,
        })
        .collect();
    state.simulate.selected_specification = Some("m59".to_owned());
    state
}

/// Where the carried row is painted, after the table has settled.
///
/// Forty frames because the scroll area animates towards a requested target
/// rather than jumping to it, and this is the position the reader ends up
/// looking at.
#[cfg(not(target_arch = "wasm32"))]
fn carried_row_position(
    ctx: &egui::Context,
    state: &mut AppState,
    screen: egui::Rect,
) -> egui::Rect {
    let selected_specification = state.simulate.selected_specification.clone();
    let mut spans = Vec::new();
    for _ in 0..40 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        show(
                            ui,
                            &mut state.results_parts(),
                            selected_specification.as_deref(),
                        );
                    });
            },
        );
        spans = painted_spans(&output);
    }
    spans
        .into_iter()
        .find(|(text, _, _)| text == "m59")
        .map(|(_, rect, _)| rect)
        .expect("the carried requirement is one of the rows the table paints")
}

/// Hopping to another run scrolls the carried limit into view again.
///
/// The table remembered only which measurement it had last scrolled to, so
/// after a hop carried `m59` into run #1 and the reader selected run #2 — a
/// different dataset, a table drawn from the top — the memo still named `m59`
/// and the request was suppressed. The selected row was marked sixty rows
/// below the fold, in a table the reader had no reason to think had scrolled.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_carried_limit_is_scrolled_to_again_in_the_next_run() {
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1400.0, 420.0));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = two_runs_over_a_long_contract();

    let first = carried_row_position(&ctx, &mut state, screen);
    assert!(
        screen.contains(first.center()),
        "the hop into the first run puts its limit on screen: {first:?}"
    );

    state.model.simulation.view.active_run_idx = Some(1);
    let second = carried_row_position(&ctx, &mut state, screen);

    assert!(
        screen.contains(second.center()),
        "the same limit in the next run is {:.0} points below the fold, because the \
         table's scroll memo was never told the dataset changed: {second:?}",
        second.center().y - screen.bottom()
    );
}
