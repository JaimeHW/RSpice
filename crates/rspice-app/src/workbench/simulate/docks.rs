//! The Simulate and Verify workspaces' navigator and inspector, over what
//! the feature lends the docks that draw them.

use crate::workbench::docks::inspector::simulation::{
    SimulateInspector, simulate as simulate_inspector,
};
use crate::workbench::docks::inspector::verify::{VerifyInspector, verify as verify_inspector};
use crate::workbench::docks::navigator::simulate::{
    SimulateNavigator, simulate as simulate_navigator,
};
use crate::workbench::docks::navigator::verify::{VerifyNavigator, verify as verify_navigator};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::state::Workspace;

use super::SimulateFeature;
use super::commands::SimulateCommand;

/// The navigator of the workspace in front: the Verify flows, or the
/// Simulate setup routes. What the reader asked of it is carried out once it
/// has drawn: a route opens in place, and Add analysis is raised as the
/// command, which the shell answers before the next point draws.
pub(super) fn show_navigator(cx: &mut Cx<'_, SimulateFeature>, ui: &mut egui::Ui) {
    if cx.wb.workspace == Workspace::Verify {
        let opened = verify_navigator(
            ui,
            &VerifyNavigator {
                model: cx.model,
                verification: &cx.state.verification,
                page: cx.state.verification_page,
                filter: cx.wb.navigator_filter(),
            },
        );
        if let Some(page) = opened {
            cx.state.verification_page = page;
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
        verify_inspector(
            ui,
            &VerifyInspector {
                model: cx.model,
                page: cx.state.verification_page,
                verification: &cx.state.verification,
            },
        );
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
