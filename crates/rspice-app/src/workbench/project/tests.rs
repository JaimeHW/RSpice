//! The project feature's own steps, run in its context.

use crate::workbench::app::RSpiceApp;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::feature::{Cx, LowerHandles, Peers};

use super::ProjectFeature;

/// Runs `call` in the project feature's context over `app`, lending it the
/// application's parts as the frame does.
pub(super) fn with_cx<R>(
    app: &mut RSpiceApp,
    call: impl FnOnce(&mut Cx<'_, ProjectFeature>) -> R,
) -> R {
    let handles = LowerHandles {
        export_io: app.export_workflow_io.as_ref(),
        run_controller: &mut app.simulation_controller,
        model_hub: &mut app.model_hub,
    };
    let state = &mut app.state;
    call(&mut Cx::new(
        &mut *state.project,
        &mut [],
        &mut state.model,
        &mut *state.workbench,
        &mut state.platform,
        &mut state.runs,
        handles,
        Peers::new(
            &mut *state.design,
            Some(&*state.cloud),
            Some(&*state.hardcopy),
            Some(&mut *state.results),
            Some(&*state.simulate),
            &*state.code,
            Some(&*state.models),
        ),
        command_registry(),
    ))
}

#[test]
fn design_management_route_hook_opens_and_dismisses_the_transient_dialog_owner() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .replace_route(
            crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::DesignManagement),
            crate::workbench::RouteTransitionSource::Restore,
        )
        .expect("Design Management route is executable");
    assert!(!app.state.project.design_management.open);

    with_cx(
        &mut app,
        ProjectFeature::synchronize_design_management_route,
    );
    assert!(app.state.project.design_management.open);

    app.state
        .workbench
        .replace_route(
            crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::Design),
            crate::workbench::RouteTransitionSource::BrowserPop,
        )
        .expect("Design workspace route is executable");
    with_cx(
        &mut app,
        ProjectFeature::synchronize_design_management_route,
    );
    assert!(!app.state.project.design_management.open);
}
