//! The Simulate and Verify workspaces' navigator and inspector, over what
//! the feature lends the docks that draw them.

use crate::workbench::docks::inspector::simulation::{
    SimulateInspector, simulate as simulate_inspector,
};
use crate::workbench::docks::inspector::verify::verify as verify_inspector;
use crate::workbench::docks::navigator::simulate::{
    SimulateNavigator, simulate as simulate_navigator,
};
use crate::workbench::docks::navigator::verify::{VerifyNavigator, verify as verify_navigator};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::state::Workspace;
use crate::workbench::surfaces::verify::set_reference_run;

use super::SimulateFeature;
use super::commands::SimulateCommand;

/// The navigator of the workspace in front: the Verify flows, or the
/// Simulate setup routes. What the reader asked of it is carried out once it
/// has drawn: a route opens in place, and Add analysis is raised as the
/// command, which the shell answers before the next point draws.
pub(super) fn show_navigator(cx: &mut Cx<'_, SimulateFeature>, ui: &mut egui::Ui) {
    if cx.wb.workspace == Workspace::Verify {
        refresh_verification(cx);
        let asked = verify_navigator(
            ui,
            &VerifyNavigator {
                model: cx.model,
                simulate: cx.state,
            },
        );
        if let Some(page) = asked.open {
            cx.state.verification_page = page;
            cx.wb.close_drawer();
        }
        if let Some(index) = asked.run {
            let _ = cx.model.simulation.select_run(index);
        }
        if let Some(run) = asked.reference {
            cx.state.verification.action_problem = set_reference_run(cx, run).err();
        }
        if let Some(index) = asked.results
            && cx.model.simulation.select_run(index)
        {
            cx.execute(ids::RESULTS_WORKSPACE);
        }
        return;
    }
    let asked = simulate_navigator(
        ui,
        &SimulateNavigator {
            model: cx.model,
            page: cx.state.simulation_page,
            filter: cx.wb.navigator_filter(),
        },
    );
    if let Some(page) = asked.open {
        cx.state.simulation_page = page;
        cx.wb.close_drawer();
    }
    if asked.add_analysis {
        cx.execute(SimulateCommand::AddAnalysis.key());
    }
}

/// The inspector of the workspace in front: the Verify flow's details, or
/// the selected analysis and its last run. Opening that run's results is
/// raised as the command once the inspector has drawn.
pub(super) fn show_inspector(cx: &mut Cx<'_, SimulateFeature>, ui: &mut egui::Ui) {
    if cx.wb.workspace == Workspace::Verify {
        refresh_verification(cx);
        verify_inspector(ui, cx);
        return;
    }
    let open_results = simulate_inspector(
        ui,
        &mut SimulateInspector {
            model: cx.model,
            active_analysis: &mut cx.state.active_analysis,
            active_analysis_instance: &mut cx.state.active_analysis_instance,
        },
    );
    if open_results {
        cx.execute(ids::RESULTS_WORKSPACE);
    }
}

/// Judge the run Verify reads against the plan as it stands, before a dock
/// states anything about it: the docks may draw before the view does, and
/// the judgement is kept until the run, the plan or the project changes.
fn refresh_verification(cx: &mut Cx<'_, SimulateFeature>) {
    let model = &mut *cx.model;
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
}

#[cfg(test)]
mod tests {
    use super::show_navigator;
    use crate::state::VerificationPage;
    use crate::workbench::simulate::test_host::SimulateTestHost;
    use crate::workbench::state::Workspace;
    use crate::workbench::state::workbench_session::Drawer;

    /// Picking a view in the compact navigator drawer closes the drawer, as
    /// picking a Simulate page does, so the view is not left under it.
    #[test]
    fn picking_a_verify_view_closes_the_navigator_drawer() {
        let mut harness = SimulateTestHost::harness();
        harness.wb.activate(Workspace::Verify);
        harness.wb.drawer = Some(Drawer::Navigator);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = |events| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 640.0),
            )),
            events,
            ..Default::default()
        };
        let row = ctx
            .run_ui(input(Vec::new()), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show_navigator(&mut harness.cx(), ui));
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .into_iter()
            .find(|(_, node)| {
                node.label()
                    .is_some_and(|label| label.starts_with(VerificationPage::MonteCarlo.label()))
            })
            .and_then(|(_, node)| node.bounds())
            .expect("the Monte Carlo row");
        let at = egui::pos2(
            ((row.x0 + row.x1) / 2.0) as f32,
            ((row.y0 + row.y1) / 2.0) as f32,
        );
        let _ = ctx.run_ui(
            input(vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
            ]),
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show_navigator(&mut harness.cx(), ui));
            },
        );

        assert_eq!(
            harness.state.verification_page,
            VerificationPage::MonteCarlo
        );
        assert_eq!(harness.wb.drawer, None);
    }
}
