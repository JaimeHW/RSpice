//! The Design navigator for a Verilog-A cell view.
//!
//! A module is a view of a cell, so the panel opens with the path that names
//! it, the way a symbol's does. Below that is the bundle the view owns: its
//! module sources and build policy, the outline of the file being edited, and
//! the file operations the bundle's lifecycle governs. The rail is the same
//! one the Netlist page draws, resolving the same bundle, so the two hosts can
//! never list different files.

use super::*;

use crate::state::ProjectSourceLanguage;
use crate::workbench::MessageId;
use crate::workbench::documents::code_workspace::{self, CodeSourceFileAction};

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let reference = app.state.workspace.content.active_view.clone();
    super::symbol::hierarchy_path(ui, &reference);
    if let Err(error) = code_workspace::selected_veriloga_source(app) {
        // The editor states the same refusal; the panel must not list the
        // files of a bundle the view does not own.
        empty_navigator_row(ui, &error);
        return;
    }
    file_operations(ui, app);
    super::source_bundle::veriloga(ui, app);
}

/// New, rename and delete for the file being edited.
///
/// Each opens the source lifecycle dialog at its review, the same transaction
/// Manage Source Document offers: rewriting a bundle's file graph is reviewed
/// and committed, never applied by a single click in a list.
fn file_operations(ui: &mut Ui, app: &mut RSpiceApp) {
    let language = ProjectSourceLanguage::VerilogA;
    let Some((bundle_id, root, active)) =
        code_workspace::active_source_bundle(&app.state.workspace, language)
            .map(|bundle| (bundle.id(), bundle.root().logical_path().to_owned()))
            .zip(code_workspace::active_bundle_path(
                &app.state.workspace,
                &app.state.ui.code_workspace,
                language,
            ))
            .map(|((bundle_id, root), active)| (bundle_id, root, active))
    else {
        return;
    };
    let messages = app.state.ui.messages();
    let writable = !app.state.active_view_read_only()
        && code_workspace::source_file_mutation_block_reason(app, language).is_none();
    // The root is the closure's entry: the service refuses to delete it, so
    // the control says so before the click rather than after it.
    let root_is_active = crate::state::project_source_paths_equal(&active, &root);
    let mut requested = None;
    ui.add_space(6.0);
    ui.horizontal_wrapped(|ui| {
        ui.add_space(8.0);
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
        for (label, action) in [
            (MessageId::CodeSourceNewFile, CodeSourceFileAction::New),
            (MessageId::CodeSourceRename, CodeSourceFileAction::Rename),
            (MessageId::CodeSourceDelete, CodeSourceFileAction::Delete),
        ] {
            let root_delete = action == CodeSourceFileAction::Delete && root_is_active;
            let response = Button::new(&messages.text(label))
                .enabled(writable && !root_delete)
                .destructive(action == CodeSourceFileAction::Delete)
                .show(ui);
            let response = if root_delete {
                response
                    .on_disabled_hover_text(messages.text(MessageId::CodeSourceRootDeleteBlocked))
            } else {
                response
            };
            if response.clicked() {
                requested = Some(action);
            }
        }
    });
    ui.add_space(6.0);
    let Some(action) = requested else {
        return;
    };
    let opened = code_workspace::open_source_file_dialog_in_bundle(
        app, language, bundle_id, action, &active, &active,
    )
    .and_then(|()| crate::workbench::app::open_source_document_dialog(&mut app.state));
    if let Err(error) = opened {
        app.state.ui.code_workspace.source_file_dialog = None;
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
    }
}
