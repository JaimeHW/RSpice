//! Following a finding to the thing it is about.
//!
//! A row on the Problems page holds the session data, not the application
//! root, so it records the route it wants and this applies it on the frame
//! boundary. That split is deliberate: a page that could run commands would
//! be a page that could do anything, and the route is the whole of what a
//! finding is allowed to ask for.
//!
//! Every destination lands on an object. A model-binding finding opens
//! Corners & sections with its library selected and its corner inspected; a
//! netlist finding opens the deck with the cursor on the line the parser
//! named. Opening the workspace and leaving the reader to find the object
//! again is what these used to do, and it is why a finding's prose used to
//! end in an instruction to go somewhere.

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::{PreflightRemediation, ProjectPage, VerificationPage, Workspace};

/// Apply the route a consumer asked for, if one is outstanding.
///
/// Also the store's frame boundary for the one withdrawal nothing else can
/// announce: a run the simulation state retired — cleared with the result
/// history, or deleted — takes its outcome with it here.
pub(crate) fn apply_pending(ctx: &Context, app: &mut RSpiceApp) {
    app.state.withdraw_retired_run_diagnostics();
    let Some(route) = app.state.diagnostics.take_pending_route() else {
        return;
    };
    match route {
        DiagnosticRoute::Anchor(anchor) => app.state.jump_to_log_anchor(anchor),
        DiagnosticRoute::NetlistDiagnostic(diagnostic_id) => {
            if let Err(error) =
                crate::workbench::documents::netlist_document::open_diagnostic_location(
                    &mut app.state,
                    diagnostic_id,
                )
            {
                app.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
        DiagnosticRoute::CodeDocument {
            bundle,
            logical_path,
            line,
        } => open_code_document(ctx, app, bundle, &logical_path, line),
        DiagnosticRoute::Repair(remediation) => apply_remediation(app, remediation),
    }
}

/// Open the editor that owns a source bundle, select the document a finding
/// is in, and put the caret on its line.
///
/// The owner is what decides the destination: a Verilog-A bundle belongs to a
/// cell view and is edited in the Design workspace showing that view, while
/// the automation bundle belongs to the project and is edited on the Netlist
/// workspace's Automation page. The view is opened before the document is
/// selected, because the selection is read against the active owner — and the
/// reveal is queued last, because the editor consumes it on the frame it
/// shows that document.
fn open_code_document(
    ctx: &Context,
    app: &mut RSpiceApp,
    bundle: crate::state::ProjectSourceId,
    logical_path: &str,
    line: Option<usize>,
) {
    use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
    use crate::workbench::documents::code_workspace;

    let owner = app
        .state
        .workspace
        .content
        .project_sources
        .get_bundle(bundle)
        .map(|bundle| bundle.owner().clone());
    let Some(owner) = owner else {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "The source bundle that reported {logical_path} is no longer part of this project."
        )));
        return;
    };
    let language = match &owner {
        ProjectSourceOwner::CellView { reference } => {
            app.state.open_workspace_view(reference.clone());
            Command::OpenWorkspace(Workspace::Design).execute(app);
            ProjectSourceLanguage::VerilogA
        }
        ProjectSourceOwner::CodeWorkspace { language } => {
            Command::OpenWorkspace(Workspace::Netlist).execute(app);
            app.state.ui.code_workspace.page = code_workspace::CodeWorkspacePage::Automation;
            *language
        }
    };
    if code_workspace::select_source_document(
        &app.state.workspace,
        &mut app.state.ui.code_workspace,
        language,
        logical_path,
    )
    .is_none()
    {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "{logical_path} is no longer the source this editor holds."
        )));
        return;
    }
    if let Some(line) = line {
        crate::workbench::documents::text_editor_commands::queue_reveal_line(
            ctx,
            code_workspace::source_editor_id(language, bundle, logical_path),
            line,
        );
    }
}

/// Open the destination a dispatch finding named for itself.
pub(in crate::workbench) fn apply_remediation(
    app: &mut RSpiceApp,
    remediation: PreflightRemediation,
) {
    match remediation {
        PreflightRemediation::DesignChecks => {
            Command::RunChecks.execute(app);
            Command::VerificationPage(VerificationPage::Yield).execute(app);
        }
        PreflightRemediation::SimulationPlan => {
            Command::OpenWorkspace(Workspace::Simulate).execute(app);
        }
        PreflightRemediation::ProjectTechnology => {
            Command::ProjectPage(ProjectPage::Dependencies).execute(app);
        }
        PreflightRemediation::Models {
            page,
            library,
            corner,
        } => {
            // Selection first: the page reads the selected library on the
            // frame it renders, so adopting it after the route would show one
            // frame of the wrong library. A library that has since gone is
            // refused rather than silently skipped, and the corner is not
            // written either — a corner name is only unique inside its
            // library, so pinning one against a selection that did not move
            // would inspect a row of whatever was already showing.
            if let Some(library) = library.as_deref()
                && app.state.select_model_library(library)
            {
                app.state.workbench.models_view.selected_corner = corner
                    .as_deref()
                    .map(|corner| format!("{library}\u{1f}{corner}"));
            }
            Command::ModelsPage(page).execute(app);
        }
        PreflightRemediation::NetlistSource { line } => {
            use crate::workbench::documents::netlist_document;

            // The generated deck is the document the workspace holds for this
            // design, so that is what opens; a project with none is told so
            // rather than being dropped on whichever source happened to be
            // active. Routing itself goes through the same resolution the
            // Problems rows use, so a netlist finding lands the same way
            // wherever it was reported from.
            if app.state.ui.netlist.generated_document.is_some() {
                netlist_document::open_generated_primary(&mut app.state);
            } else {
                app.state.push_user_message(ConsoleMessage::warning(
                    "No generated netlist is retained yet; the Netlist workspace opens on the source it holds.",
                ));
            }
            // The parser numbers the deck from 1 and the buffer indexes it
            // from 0, so a line that survived from the failure is converted
            // rather than passed through. A finding with no line opens the
            // deck and leaves the cursor alone.
            let cursor = line.and_then(|line| line.checked_sub(1));
            if let Err(error) = netlist_document::open_source_location(&mut app.state, None, cursor)
            {
                app.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
    }
}

#[cfg(test)]
mod tests;
