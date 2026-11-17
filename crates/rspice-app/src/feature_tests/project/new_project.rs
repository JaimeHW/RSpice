//! Creating a project: handed to the shell against carried out in place,
//! and, on the desktop, written where the dialog says at once.

use crate::workbench::RSpiceApp;
use crate::workbench::app::project_transactions::open_new_project_dialog;
use rspice_app_project::workbench::app::dialogs::project::new_project::NewProjectDialogState;
use rspice_app_project::workbench::project::commands::ProjectRequest;

use super::with_cx;

/// New project open on `name`, over the session it would replace, making
/// its folder in `location`.
fn naming(name: &str, location: &std::path::Path) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    open_new_project_dialog(&mut app.state);
    app.state.project.new_project.name = name.to_owned();
    app.state.project.new_project.location = location.display().to_string();
    app
}

/// Hand the shell the creation, as Create does.
fn create(app: &mut RSpiceApp) {
    with_cx(app, |cx| {
        cx.wb.outbox.push(ProjectRequest::CreateProject);
    });
    app.after_contribution();
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
        let location =
            rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir("new-project");
        let mut in_place = naming(name, &location);
        let dialog = in_place.state.project.new_project.clone();
        match in_place.state.create_reviewed_project(&dialog) {
            Ok(()) => in_place.state.project.new_project = NewProjectDialogState::default(),
            Err(error) => in_place.state.project.new_project.error = Some(error),
        }
        let expected = left(&in_place);
        drop(in_place);
        let _ = std::fs::remove_dir_all(location.join(name.trim()));

        let mut handed = naming(name, &location);
        create(&mut handed);

        assert_eq!(left(&handed), expected, "{name:?}");
        assert_eq!(
            handed.state.project.new_project.open,
            name.trim().is_empty(),
            "{name:?}"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod written_at_creation {
    use super::{create, naming};
    use crate::workbench::RSpiceApp;
    use crate::workbench::app::project_transactions::open_new_project_dialog;
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        canonical_native_path, has_unsaved_changes,
    };
    use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;

    /// Create writes the project file in a folder of its own name in the
    /// location, and the window carries on in it: bound to the file, nothing
    /// unsaved, and the file opens as the project the dialog described.
    #[test]
    fn a_new_project_is_written_where_the_dialog_says_at_once() {
        let location = unique_temp_dir("new-project-written");
        let mut app = naming("Sensor bridge", &location);
        app.state.project.new_project.library = "afe".to_owned();
        app.state.project.new_project.library_edited = true;
        app.state.project.new_project.top_cell = "core".to_owned();
        let file = location
            .join("Sensor bridge")
            .join("Sensor bridge.rspiceproj");
        assert_eq!(
            app.state.project.new_project.project_file().as_deref(),
            Ok(file.as_path())
        );

        create(&mut app);

        assert!(!app.state.project.new_project.open, "the dialog closed");
        assert!(file.is_file(), "the project file is written at once");
        assert_eq!(
            canonical_native_path(&app.state.platform)
                .map(|path| std::fs::canonicalize(path).unwrap()),
            Some(std::fs::canonicalize(&file).unwrap()),
            "the window works in the file it wrote"
        );
        assert!(!has_unsaved_changes(&app.state), "nothing is unsaved");
        assert_eq!(app.state.model.safe_mode.opened_read_only, None);

        let mut reopened = RSpiceApp::test_instance();
        drop(app);
        assert!(reopened.state.load_project_from_path(&file));
        let project = &reopened.state.model.workspace.content.project;
        assert_eq!(project.name(), "Sensor bridge");
        assert_eq!(project.root_library, "afe");
        assert_eq!(project.top_cell, "core");
    }

    /// A folder that already holds files is not written into: Create is
    /// refused beside the fields, the open project stays, and nothing is
    /// written.
    #[test]
    fn a_new_project_is_refused_a_folder_that_already_holds_files() {
        let location = unique_temp_dir("new-project-occupied");
        let occupied = location.join("Amp");
        std::fs::create_dir_all(&occupied).unwrap();
        std::fs::write(occupied.join("notes.txt"), "kept").unwrap();
        let mut app = naming("Amp", &location);
        let before = app.state.model.workspace.content.project.name().to_owned();

        create(&mut app);

        let refusal = app
            .state
            .project
            .new_project
            .error
            .clone()
            .expect("the dialog states the refusal");
        assert!(
            refusal.contains("already exists and is not empty"),
            "{refusal}"
        );
        assert!(app.state.project.new_project.open, "the dialog stays open");
        assert_eq!(app.state.model.workspace.content.project.name(), before);
        assert!(!occupied.join("Amp.rspiceproj").exists());
        assert_eq!(
            std::fs::read_to_string(occupied.join("notes.txt")).unwrap(),
            "kept"
        );
    }

    /// The location starts where the last project lives: beside its own
    /// folder when it has one.
    #[test]
    fn the_location_starts_beside_the_most_recent_project() {
        use rspice_app_services::services::recent_files::{RecentFile, RecentKind};
        let recent = |path: &str, opened_at_unix_ms| RecentFile {
            kind: RecentKind::Project,
            path: std::path::PathBuf::from(path),
            opened_at_unix_ms,
            pinned: false,
            missing: false,
        };
        let mut app = RSpiceApp::test_instance();
        app.state.platform.recent_files = vec![
            recent("/work/old/old.rspiceproj", 1),
            recent("/work/designs/amp/amp.rspiceproj", 3),
            recent("/scratch/loose.rspiceproj", 2),
        ];
        open_new_project_dialog(&mut app.state);
        assert_eq!(
            std::path::PathBuf::from(&app.state.project.new_project.location),
            std::path::PathBuf::from("/work/designs")
        );

        app.state.platform.recent_files = vec![recent("/scratch/loose.rspiceproj", 2)];
        open_new_project_dialog(&mut app.state);
        assert_eq!(
            std::path::PathBuf::from(&app.state.project.new_project.location),
            std::path::PathBuf::from("/scratch")
        );
    }

    /// The technology chosen is attached before the first save, so the
    /// written project has it; only the installation's model libraries are
    /// offered, since a library the open project holds stays with it.
    #[test]
    fn a_new_project_is_created_with_the_technology_chosen() {
        let models = unique_temp_dir("new-project-models");
        std::fs::write(
            models.join("demo.lib"),
            ".model nmos_project_demo nmos level=1 vto=0.55\n",
        )
        .unwrap();
        let location = unique_temp_dir("new-project-technology");
        let mut app = RSpiceApp::test_instance();
        let model = &mut app.state.model;
        model.pdk_config = rspice_app_state::state::pdk_config::PdkConfig::default();
        model
            .pdk_config
            .add_library_path(models.to_string_lossy().into_owned());
        let (archive, authority) =
            rspice_app_state::state::pdk_config::pdk_technology_test_fixture();
        model
            .pdk_config
            .technology_registry
            .install_archive_bytes(&archive, &authority, "Install the test technology")
            .expect("the test PDK installs");
        let (catalog, _) =
            rspice_app_state::workbench::workspace::restore_session_model_library_manager(
                &mut model.pdk_config,
            )
            .expect("the configured library loads");
        model.model_library_manager = catalog;
        let installed = model
            .model_library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| {
                matches!(
                    library.source_authority,
                    rspice_app_state::state::model_library::ModelSourceAuthority::External
                )
            })
            .map(|library| library.name.clone())
            .expect("the configured library is in the catalog");

        open_new_project_dialog(&mut app.state);
        let dialog = &mut app.state.project.new_project;
        let technology = dialog
            .technologies
            .iter()
            .find(|technology| technology.model_library == installed)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "the installed library is offered: {:?}",
                    dialog.technologies
                )
            });
        dialog.technology = Some(technology.clone());
        dialog.name = "Bias cell".to_owned();
        dialog.location = location.display().to_string();

        create(&mut app);

        assert!(!app.state.project.new_project.open);
        let project = &app.state.model.workspace.content.project;
        assert_eq!(
            project
                .technology_binding()
                .map(|binding| binding.model_library().to_owned()),
            Some(technology.model_library.clone())
        );
        assert_eq!(
            project.technology.as_deref(),
            Some(technology.label.as_str())
        );
        assert!(
            !has_unsaved_changes(&app.state),
            "the technology is in the written file"
        );
    }
}
