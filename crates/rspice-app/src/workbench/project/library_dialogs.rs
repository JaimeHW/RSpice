//! The Library/Cellview Manager's dialogs the project feature draws in its
//! own context: what each opens over the feature's state and the model, and
//! what each commits through the model's library catalog.

mod cell_ops;
mod library_ops;
mod new_cell;
mod rename_view;

/// What the status bar, the Console and the drawing show: a catalog change
/// handed to the shell ends showing what the change carried out in place
/// did.
#[cfg(test)]
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
#[cfg(test)]
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
