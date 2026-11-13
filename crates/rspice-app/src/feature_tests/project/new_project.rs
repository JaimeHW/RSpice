//! Creating a project handed to the shell, against creating it in place.

use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::project::new_project::{
    NewProjectDialogState, open_new_project_dialog,
};
use crate::workbench::project::commands::ProjectRequest;

use super::with_cx;

/// New project open on `name`, over the session it would replace.
fn naming(name: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    open_new_project_dialog(&mut app.state);
    app.state.project.new_project.name = name.to_owned();
    app
}

/// What a creation leaves showing: the dialog, the status bar, the
/// Console, and the project's root library, top cell and open view.
fn left(
    app: &RSpiceApp,
) -> (
    NewProjectDialogState,
    Option<String>,
    Vec<String>,
    (String, String, String),
) {
    let content = &app.state.model.workspace.content;
    (
        app.state.project.new_project.clone(),
        app.state.prompt_text().map(str::to_owned),
        app.state
            .model
            .log_buffer
            .entries()
            .map(|entry| entry.message.clone())
            .collect(),
        (
            content.project.root_library.clone(),
            content.project.top_cell.clone(),
            content.active_view.display_path(),
        ),
    )
}

/// A creation handed to the shell ends where the creation carried out in
/// place did, and the dialog held open for it takes the outcome: closed
/// once the project is created, the refusal stated when it is not.
#[test]
fn a_project_created_by_the_shell_ends_where_the_creation_in_place_did() {
    for name in ["Bandgap reference", "  "] {
        let mut in_place = naming(name);
        let params = in_place.state.project.new_project.params();
        match in_place.state.create_new_project_with(&params) {
            Ok(()) => in_place.state.project.new_project = NewProjectDialogState::default(),
            Err(error) => in_place.state.project.new_project.error = Some(error),
        }

        let mut handed = naming(name);
        with_cx(&mut handed, |cx| {
            cx.wb.outbox.push(ProjectRequest::CreateProject);
        });
        handed.after_contribution();

        assert_eq!(left(&handed), left(&in_place), "{name:?}");
        assert_eq!(
            handed.state.project.new_project.open,
            name.trim().is_empty(),
            "{name:?}"
        );
    }
}
