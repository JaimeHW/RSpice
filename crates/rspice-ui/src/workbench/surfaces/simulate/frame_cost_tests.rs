//! What one frame of a studio route costs the simulation model.
//!
//! A surface that resolves the same thing twice looks exactly like one that
//! resolves it once: both paint the same pixels, both pass every assertion
//! about what they say, and the only difference is how long the frame took.
//! The Analyses route expanded the declared space twice a frame for months and
//! the Run Set route six times, and nothing could have failed over it.
//!
//! So these count instead of reading. `simulation::cost_probe` records the
//! three derivations a frame is allowed to pay for once, under `#[cfg(test)]`,
//! and each route asserts its own count. A shipped build has no counter and no
//! branch.

use super::*;

/// The Analyses route expands the declared space once a frame.
///
/// The two columns each resolved it: the rail through the workload it prices
/// its rows from, the editor through the participation its run-points control
/// reads. Expanding a space costs the size of the space, not the size of the
/// declaration, so a plan over a few hundred points paid for the whole matrix
/// twice on every frame the route was drawn — and nothing the route painted
/// was different for it, which is why this counts rather than reads.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_analyses_route_expands_the_declared_space_once_a_frame() {
    use crate::simulation::cost_probe::{Derivation, count, reset};

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
    for dimension in &mut app.state.sim_setup.run_set.dimensions {
        dimension.enabled =
            dimension.kind == crate::simulation::run_set::RunSetDimensionKind::Temperature;
    }

    reset();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 1200.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::show(ui, &mut app));
        },
    );
    assert!(
        !output.shapes.is_empty(),
        "the frame drew the Analyses route"
    );

    assert_eq!(
        count(Derivation::SpaceExpansion),
        1,
        "the route resolves the space once and lends it to both columns"
    );
}

/// One frame of one studio route, and what it made the model derive.
///
/// The counters are per thread and the routes are drawn one at a time, so a
/// frame's cost is exactly what this returns.
#[cfg(not(target_arch = "wasm32"))]
fn derivations_for_route(page: crate::workbench::state::SimulationPage) -> (usize, usize, usize) {
    use crate::simulation::cost_probe::{Derivation, count, reset};

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.simulation_page = page;
    // A declared axis, so the space is a matrix rather than a single point and
    // an expansion actually costs something.
    for dimension in &mut app.state.sim_setup.run_set.dimensions {
        dimension.enabled =
            dimension.kind == crate::simulation::run_set::RunSetDimensionKind::Temperature;
    }

    reset();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 1200.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::show(ui, &mut app));
        },
    );
    assert!(!output.shapes.is_empty(), "the frame drew {page:?}");
    (
        count(Derivation::SpaceExpansion),
        count(Derivation::RunSetValidation),
        count(Derivation::PlacedSources),
    )
}

/// The Run Set route derives its space once and validates it once.
///
/// It expanded the declared space six times a frame and validated it seven —
/// the toolbar's status, the run-space card's worst point, the task-rate card,
/// the budgets card, the receipts card and the point table each asked, and each
/// expansion costs the size of the space rather than the size of the
/// declaration. Nothing the route paints is different for it, which is why this
/// counts rather than reads.
///
/// One resolution at the page entry, lent to every card. Not a cache: there is
/// no key and nothing survives the frame, so a route cannot serve a reader a
/// space the plan no longer declares.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_run_set_route_derives_its_space_once_a_frame() {
    let (expansions, validations, _) =
        derivations_for_route(crate::workbench::state::SimulationPage::RunSet);
    assert_eq!(
        (expansions, validations),
        (1, 1),
        "the Run Set route expanded the space {expansions} times and validated it \
         {validations} times in one frame"
    );
}

/// The two routes that read the run set without owning it pay for it once too.
///
/// Outputs prices its capture groups against the declared space and Save prices
/// its retention forecast; neither is the page that declares it, and both were
/// asking more than once.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_outputs_and_save_routes_derive_the_space_once_a_frame() {
    use crate::workbench::state::SimulationPage;

    for page in [SimulationPage::Outputs, SimulationPage::Save] {
        let (expansions, validations, _) = derivations_for_route(page);
        assert!(
            expansions <= 1 && validations <= 1,
            "the {page:?} route expanded the space {expansions} times and validated it \
             {validations} times in one frame"
        );
    }
}
