//! The Dependencies page through whole application frames: each library has
//! one status, and the rail, the table and the inspector beside the page say
//! the same thing about it, in every state a library can be in.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::DependencyLibrary;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::Library;
use rspice_app_state::state::model_library::{ModelLibrary, ModelSourceAuthority};
use rspice_app_state::workbench::workspace::Workspace;

use super::recovery_frames::{app_frame, offscreen, painted};

/// What one place on the page says about a library: the status's word, and
/// the severity a screen reader is told in front of it, if any.
type Said = (String, Option<String>);

/// A severity the kit announces in front of a marked line, and the words
/// after it.
fn severity(announced: &str) -> (Option<String>, &str) {
    for severity in ["Warning", "Error"] {
        if let Some(rest) = announced.strip_prefix(&format!("{severity}: ")) {
            return (Some(severity.to_owned()), rest);
        }
    }
    (None, announced)
}

/// The AccessKit nodes one frame published, by label, value and
/// description.
fn announced(output: &egui::FullOutput) -> Vec<(String, Option<String>, Option<String>, bool)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| {
            Some((
                node.label()?.to_owned(),
                node.value().map(str::to_owned),
                node.description().map(str::to_owned),
                node.toggled() == Some(egui::accesskit::Toggled::True),
            ))
        })
        .collect()
}

/// The rail's row for `name`: its name and its status, nothing else.
fn rail_row<'a>(
    nodes: &'a [(String, Option<String>, Option<String>, bool)],
    name: &str,
) -> Vec<&'a (String, Option<String>, Option<String>, bool)> {
    nodes
        .iter()
        .filter(|(label, ..)| {
            let (_, words) = severity(label);
            let cells: Vec<_> = words.split(", ").collect();
            cells.len() == 2 && cells[0] == name
        })
        .collect()
}

/// The table's row for `name` of `kind`: every column, the status last.
fn table_row<'a>(
    nodes: &'a [(String, Option<String>, Option<String>, bool)],
    name: &str,
    kind: &str,
) -> Vec<&'a (String, Option<String>, Option<String>, bool)> {
    nodes
        .iter()
        .filter(|(label, ..)| {
            let (_, words) = severity(label);
            let cells: Vec<_> = words.split(", ").collect();
            cells.len() >= 5 && cells[0] == name && cells[1] == kind
        })
        .collect()
}

/// What the rail, the table and the inspector say about `library`, in the
/// frame after it was selected.
fn said_about(app: &mut RSpiceApp, library: &DependencyLibrary) -> [Said; 3] {
    let kind = match library.kind {
        rspice_app_project::workbench::project::DependencyKind::Design => "Design library",
        rspice_app_project::workbench::project::DependencyKind::Model => "Model library",
    };
    app.state.project.project_dependency_selection = Some(library.clone());
    let ctx = offscreen();
    app_frame(&ctx, app, Vec::new());
    let output = app_frame(&ctx, app, Vec::new());
    let nodes = announced(&output);

    let rail = rail_row(&nodes, &library.name);
    assert_eq!(rail.len(), 1, "one rail row names {library:?}: {nodes:#?}");
    let (rail_severity, words) = severity(&rail[0].0);
    let rail = (
        words.split(", ").nth(1).expect("a status").to_owned(),
        rail_severity,
    );

    let table = table_row(&nodes, &library.name, kind);
    assert_eq!(
        table.len(),
        1,
        "one table row names {library:?}: {nodes:#?}"
    );
    let (table_severity, words) = severity(&table[0].0);
    let table = (
        words.rsplit(", ").next().expect("a status").to_owned(),
        table_severity,
    );

    let statuses: Vec<_> = nodes
        .iter()
        .filter(|(label, ..)| label == "Status")
        .collect();
    assert_eq!(statuses.len(), 1, "the inspector states one status");
    let inspector = (
        statuses[0].1.clone().expect("the status row has a value"),
        statuses[0].2.clone(),
    );
    [rail, table, inspector]
}

/// One model file written to `folder`, imported as the Models workspace
/// imports a file from disk, and the library's name.
fn import_file(app: &mut RSpiceApp, folder: &Path, file: &str, card: &str) -> (String, PathBuf) {
    let path = folder.join(file);
    std::fs::write(&path, format!(".model {card} nmos level=1 vto=0.5\n"))
        .expect("the model file writes");
    let name = app
        .state
        .model
        .model_library_manager
        .load_library_file(&path, None)
        .expect("the model file imports");
    (name, path)
}

/// A project holding a library in every state, open on Dependencies.
struct Libraries {
    app: RSpiceApp,
    folder: PathBuf,
    built_in: String,
    found: String,
    changed: String,
    missing: String,
    unread: String,
    kept: String,
}

fn libraries() -> Libraries {
    let folder = unique_temp_dir("dependency-status");
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    let model = &mut app.state.model;
    model.library_manager.add_library(Library::new("cells"));
    let built_in = model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| matches!(library.source_authority, ModelSourceAuthority::BuiltIn))
        .map(|library| library.name.clone())
        .expect("a new project has the built-in library");

    let (found, _) = import_file(&mut app, &folder, "found.lib", "nfound");
    let (changed, changed_path) = import_file(&mut app, &folder, "changed.lib", "nchanged");
    let (missing, missing_path) = import_file(&mut app, &folder, "missing.lib", "nmissing");
    // Edited and deleted after they were attached, before the page opens.
    std::fs::write(&changed_path, ".model nchanged nmos level=1 vto=0.7\n")
        .expect("the model file rewrites");
    std::fs::remove_file(&missing_path).expect("the model file is deleted");

    let model = &mut app.state.model;
    let mut unread = ModelLibrary::new("unread");
    unread.source_authority = ModelSourceAuthority::External;
    unread.root_path = Some(folder.join("unread.lib"));
    model.model_library_manager.add_library(unread);
    let kept = model
        .model_library_manager
        .load_library_bytes(
            "kept.lib",
            b".model nkept nmos level=1 vto=0.5\n".to_vec(),
            None,
        )
        .expect("the model bytes import");
    Libraries {
        app,
        folder,
        built_in,
        found,
        changed,
        missing,
        unread: "unread".to_owned(),
        kept,
    }
}

fn assert_said(app: &mut RSpiceApp, library: DependencyLibrary, status: &str, mark: Option<&str>) {
    let expected: Said = (status.to_owned(), mark.map(str::to_owned));
    let said = said_about(app, &library);
    assert_eq!(
        said,
        [expected.clone(), expected.clone(), expected],
        "the rail, the table and the inspector on {library:?}"
    );
}

#[test]
fn a_design_library_is_in_the_project_in_every_place() {
    let mut libraries = libraries();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::design("cells"),
        "In project",
        None,
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// The library RSpice ships simulates from its own models: it is built in,
/// and nowhere says it cannot simulate.
#[test]
fn the_built_in_library_is_built_in_in_every_place() {
    let mut libraries = libraries();
    let name = libraries.built_in.clone();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "Built in",
        None,
    );
    let ctx = offscreen();
    let text = painted(&app_frame(&ctx, &mut libraries.app, Vec::new()));
    assert!(!text.contains("cannot simulate"), "{text}");
    let _ = std::fs::remove_dir_all(libraries.folder);
}

#[test]
fn an_attached_model_file_that_is_present_is_found_in_every_place() {
    let mut libraries = libraries();
    let name = libraries.found.clone();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "Found",
        None,
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A model file edited since it was attached reads as changed, from the
/// drift report the Models workspace shows, not from the counts the library
/// keeps.
#[test]
fn a_model_file_edited_since_it_was_attached_is_changed_in_every_place() {
    let mut libraries = libraries();
    let name = libraries.changed.clone();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "Changed",
        Some("Warning"),
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

#[test]
fn a_model_file_deleted_since_it_was_attached_is_not_found_in_every_place() {
    let mut libraries = libraries();
    let name = libraries.missing.clone();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "Not found",
        Some("Error"),
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A library that names a model file no source was ever read from failed to
/// load, and says so in every place.
#[test]
fn a_model_library_whose_files_were_never_read_has_no_model_files_in_every_place() {
    let mut libraries = libraries();
    let name = libraries.unread.clone();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "No model files",
        Some("Error"),
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

#[test]
fn a_model_library_kept_in_the_project_is_in_the_project_in_every_place() {
    let mut libraries = libraries();
    let name = libraries.kept.clone();
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "In project",
        None,
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A project with no technology attached is ordinary, as on the Overview:
/// the page and the inspector say "Not attached" with no mark, and no
/// sentence about a missing binding.
#[test]
fn a_project_with_no_technology_attached_carries_no_mark() {
    let mut libraries = libraries();
    assert!(
        libraries
            .app
            .state
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .is_none()
    );
    let ctx = offscreen();
    app_frame(&ctx, &mut libraries.app, Vec::new());
    let output = app_frame(&ctx, &mut libraries.app, Vec::new());
    // The rows that state the technology, not the headings titled with it.
    let technology: Vec<_> = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter(|(_, node)| {
            node.role() != egui::accesskit::Role::Heading && node.label() == Some("Technology")
        })
        .map(|(_, node)| {
            (
                node.value().map(str::to_owned),
                node.description().map(str::to_owned),
            )
        })
        .collect();
    assert_eq!(
        technology.len(),
        2,
        "the page and the inspector: {technology:?}"
    );
    for (value, description) in technology {
        assert_eq!(value.as_deref(), Some("Not attached"));
        assert_eq!(description, None, "no mark on an ordinary project");
    }
    let text = painted(&output);
    assert!(!text.contains("binding"), "{text}");
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A design library and a model library of one name are two libraries: the
/// model library is the one selected, the one highlighted and the one the
/// inspector shows.
#[test]
fn a_design_and_a_model_library_of_one_name_are_selected_apart() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    app.state
        .model
        .library_manager
        .add_library(Library::new("shared"));
    app.state
        .model
        .model_library_manager
        .add_library(ModelLibrary::new("shared"));
    app.state.project.project_dependency_selection = Some(DependencyLibrary::model("shared"));
    let ctx = offscreen();
    app_frame(&ctx, &mut app, Vec::new());
    let output = app_frame(&ctx, &mut app, Vec::new());
    let nodes = announced(&output);

    let rail = rail_row(&nodes, "shared");
    assert_eq!(rail.len(), 2, "both are rail rows: {rail:?}");
    let selected: Vec<_> = rail.iter().filter(|(.., selected)| *selected).collect();
    assert_eq!(selected.len(), 1, "one rail row is selected: {rail:?}");
    assert!(selected[0].0.ends_with("Built in"), "{selected:?}");
    let design = table_row(&nodes, "shared", "Design library");
    let model = table_row(&nodes, "shared", "Model library");
    assert!(!design[0].3 && model[0].3, "{design:?} {model:?}");
    let text = painted(&output);
    assert!(text.contains("Model library"), "{text}");
    let status: Vec<_> = nodes
        .iter()
        .filter(|(label, ..)| label == "Status")
        .collect();
    assert_eq!(status[0].1.as_deref(), Some("Built in"));
}

/// The rail and the table list the same libraries in the same order under
/// the one filter, and a project that replaces this one starts with neither
/// the filter nor the selection.
#[test]
fn the_rail_and_the_table_share_one_filter_and_lose_it_with_the_project() {
    let mut libraries = libraries();
    let app = &mut libraries.app;
    app.state.workbench.set_navigator_filter("CHANGED");
    let name = libraries.changed.clone();
    app.state.project.project_dependency_selection = Some(DependencyLibrary::model(&name));
    let ctx = offscreen();
    app_frame(&ctx, app, Vec::new());
    let nodes = announced(&app_frame(&ctx, app, Vec::new()));
    let found_rail = rail_row(&nodes, &libraries.found);
    let found_table = table_row(&nodes, &libraries.found, "Model library");
    assert!(
        found_rail.is_empty() && found_table.is_empty(),
        "the filter hides it in both"
    );
    assert_eq!(rail_row(&nodes, &name).len(), 1);
    assert_eq!(table_row(&nodes, &name, "Model library").len(), 1);

    crate::workbench::app_state::services_access::reset_for_new_project(&mut app.state);
    assert_eq!(
        app.state
            .workbench
            .navigator_trees
            .filter(Workspace::Project),
        ""
    );
    assert_eq!(app.state.project.project_dependency_selection, None);
    let _ = std::fs::remove_dir_all(libraries.folder);
}
