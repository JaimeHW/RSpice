//! The project feature's behaviour, observed through a whole application.

mod builtin_models;
mod cellview_manager;
mod check_and_save;
mod check_and_save_validation;
mod configuration_dialogs;
mod configuration_inspector;
mod configuration_page;
mod configuration_transactions;
mod design_management;
mod examples;
mod file_workflow;
mod landing;
mod launcher;
mod library_changes;
mod library_deletion;
mod library_inspector;
mod library_page;
mod new_cell;
mod new_project;
mod new_project_fields;
mod new_view;
mod overview;
mod project_workflow;
mod publication;
mod rails;
mod steps;
mod surface;

use crate::workbench::app::RSpiceApp;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::feature::{Cx, LowerHandles, Peers};
use crate::workbench::project::ProjectFeature;
use crate::workbench::surfaces::project::library_publication;

/// Runs `call` in the project feature's context over `app`, lending it the
/// application's parts as the frame does.
fn with_cx<R>(app: &mut RSpiceApp, call: impl FnOnce(&mut Cx<'_, ProjectFeature>) -> R) -> R {
    let handles = LowerHandles {
        export_io: app.export_workflow_io.as_ref(),
        run_controller: &mut app.simulation_controller,
        model_hub: &mut app.model_hub,
        symbols: app.symbol_library.as_ref(),
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
            Some(&mut *state.design),
            Some(&*state.cloud),
            Some(&*state.hardcopy),
            Some(&mut *state.results),
            Some(&*state.simulate),
            Some(&*state.code),
            Some(&*state.models),
        ),
        command_registry(),
    ))
}

/// What the status bar, the Console and the drawing show: a catalog change
/// handed to the shell ends showing what the change carried out in place
/// did.
fn shown(app: &crate::workbench::RSpiceApp) -> (Option<String>, Vec<String>, String) {
    (
        app.state.prompt_text().map(str::to_owned),
        app.state
            .model
            .log_buffer
            .entries()
            .map(|entry| entry.message.clone())
            .collect(),
        app.state.model.workspace.content.active_view.display_path(),
    )
}

/// What a catalog change published: the project's library receipts, the
/// catalog's revision and the design's execution epoch.
fn published(
    app: &crate::workbench::RSpiceApp,
) -> (Vec<(crate::state::ProjectLibraryMutation, u64)>, u64, u64) {
    let model = &app.state.model;
    (
        model
            .workspace
            .content
            .project
            .library_mutation_audit()
            .iter()
            .map(|receipt| {
                (
                    receipt.mutation().clone(),
                    receipt.target_library_revision(),
                )
            })
            .collect(),
        model.library_manager.revision(),
        model.design_execution_epoch,
    )
}

/// Beside what [`shown`] holds, the workspace in front, whether the
/// Verilog-A receipt is kept, and the documents a creation seeded.
fn in_front(
    app: &crate::workbench::RSpiceApp,
) -> (crate::workbench::state::Workspace, bool, Vec<String>) {
    let mut buffers = app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    buffers.sort();
    (
        app.state.workbench.workspace,
        app.state.code.code_workspace.veriloga.receipt.is_some(),
        buffers,
    )
}
