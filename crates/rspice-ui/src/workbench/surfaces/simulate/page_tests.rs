//! Contract tests for the Simulation Studio setup pages.
//!
//! These check what the pages promise rather than how they paint: every route
//! renders without an egui identity clash, the page's own subject reaches the
//! screen, and the numerical controls resolve to exactly the values the run
//! will use.

use egui::{Rect, vec2};

use super::page_kit::column_rects;
use super::pages;
use crate::simulation::dialog::{OptionsDialogState, SimulationOptions};
use crate::workbench::RSpiceApp;
use crate::workbench::state::SimulationPage;

/// Render one page and collect the text it painted.
fn render(page: SimulationPage, width: f32) -> String {
    fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                rendered.push_str(&text.galley.job.text);
                rendered.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, rendered);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.simulation_page = page;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 1400.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| pages::show(ui, &mut app, page));
                });
        },
    );
    let mut rendered = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rendered);
    }
    rendered
}

#[test]
fn every_setup_route_renders_its_own_heading_without_an_identity_clash() {
    for page in SimulationPage::NAVIGATION {
        if page == SimulationPage::Analyses {
            continue;
        }
        for width in [720.0, 1200.0] {
            let rendered = render(page, width);
            assert!(
                rendered.contains(page.title()),
                "{page:?} at {width}px did not render its own title:\n{rendered}"
            );
            assert!(
                !rendered.contains("First use of")
                    && !rendered.contains("Second use of")
                    && !rendered.contains("Double use of"),
                "{page:?} rendered an egui ID-clash diagnostic at {width}px:\n{rendered}"
            );
        }
    }
}

#[test]
fn the_solver_page_states_both_acceptance_criteria_and_the_whole_ladder() {
    let rendered = render(SimulationPage::Solver, 1200.0);
    for symbol in [
        "RELTOL",
        "VNTOL",
        "ABSTOL",
        "CHGTOL",
        "RESIDUAL_RELTOL",
        "IABSTOL",
        "ITL1",
        "ITL4",
    ] {
        assert!(
            rendered.contains(symbol),
            "the solver page did not state {symbol}:\n{rendered}"
        );
    }
    for stage in [
        "Damped Newton",
        "Adaptive GMIN stepping",
        "Source stepping",
        "Pseudo-transient continuation",
        "Arc-length continuation",
    ] {
        assert!(
            rendered.contains(stage),
            "the continuation ladder omitted {stage}:\n{rendered}"
        );
    }
}

#[test]
fn a_page_never_claims_a_run_space_it_cannot_resolve() {
    let rendered = render(SimulationPage::RunSet, 1200.0);
    assert!(
        rendered.contains("single reference point") || rendered.contains("corner sweep enabled"),
        "the run-set page did not state which composition is active:\n{rendered}"
    );
}

/// Every option a page edits must survive a draft round trip.
///
/// A field with no draft buffer was silently reset to a literal on the next
/// commit, so a value the user set never reached the engine. The pivot
/// thresholds, the DC-sweep budget and arc-length continuation all behaved
/// that way before they were given buffers.
#[test]
fn every_edited_option_survives_a_draft_round_trip() {
    let options = SimulationOptions {
        pivrel: 0.125,
        pivtol: 2.5e-14,
        itl1: 77,
        itl2: 133,
        itl4: 21,
        arc_length: true,
        gmin_stepping: false,
        source_stepping: false,
        pseudo_transient: false,
        reltol: 4.5e-5,
        residual_reltol: 6.5e-5,
        vntol: 3.5e-7,
        abstol: 7.5e-13,
        iabstol: 8.5e-13,
        chgtol: 9.5e-15,
        ..SimulationOptions::default()
    };
    let round_tripped = OptionsDialogState::from_options(&options)
        .to_options()
        .expect("a draft built from valid options must parse");
    assert_eq!(
        serde_json::to_value(&round_tripped).expect("serializable"),
        serde_json::to_value(&options).expect("serializable"),
        "a draft round trip changed the effective options"
    );
}

/// Presets must be distinguishable by exact comparison, because that is how
/// the strip decides which chip is active. Two presets that serialize
/// identically would make the strip report the wrong policy.
#[test]
fn the_named_presets_are_pairwise_distinct() {
    let presets = [
        SimulationOptions::fast(),
        SimulationOptions::default(),
        SimulationOptions::accurate(),
        SimulationOptions::robust(),
    ];
    for (index, left) in presets.iter().enumerate() {
        for right in presets.iter().skip(index + 1) {
            assert_ne!(
                serde_json::to_value(left).expect("serializable"),
                serde_json::to_value(right).expect("serializable"),
                "two named presets resolve to the same option set"
            );
        }
    }
}

#[test]
fn ledger_columns_tile_their_row_without_a_gap_or_an_overhang() {
    let row = Rect::from_min_size(egui::Pos2::ZERO, vec2(640.0, 28.0));
    for fractions in [
        vec![0.36, 0.34, 0.30],
        vec![0.18, 0.52, 0.30],
        vec![0.20, 0.13, 0.28, 0.17, 0.22],
    ] {
        let columns = column_rects(row, &fractions);
        assert_eq!(columns.len(), fractions.len());
        assert_eq!(columns[0].left(), row.left());
        assert_eq!(
            columns[columns.len() - 1].right(),
            row.right(),
            "the last column must absorb the remainder so no strip of the row is unclaimed"
        );
        assert!(columns.iter().all(|column| column.width() > 0.0));
        assert!(
            columns
                .windows(2)
                .all(|pair| pair[0].right() == pair[1].left())
        );
    }
}
