//! The Library/Cellview Manager's dialogs the project feature draws in its
//! own context: what each opens over the feature's state and the model, and
//! what each commits through the model's library catalog.

mod cell_ops;
mod library_ops;
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
