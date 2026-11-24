//! The Dependencies page through whole application frames: each library has
//! one status, which the inspector beside the page states and the table
//! states only when it is a problem, in the same words, in every state a
//! library can be in; each row counts what its library provides; and each
//! row of the table can be opened, shown, read again, changed or removed
//! from its menu, the bar or a key.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::app::dialogs::project::{RELINK_PICK, SHOWN_FOLDERS};
use rspice_app_project::workbench::project::DependencyLibrary;
use rspice_app_project::workbench::project::library_dialogs::library_files::answer_next_folder_pick;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::Library;
use rspice_app_state::state::model_library::{ModelLibrary, ModelSourceAuthority};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

use super::recovery_frames::{app_frame, click_on, offscreen, painted, saved_project};

/// What one place on the page says about a library: the status's word, and
/// the severity a screen reader is told in front of it, if any.
pub(super) type Said = (String, Option<String>);

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

/// The table's row for `name` of `kind`, which a screen reader is told cell
/// by cell under its head.
fn table_row<'a>(
    nodes: &'a [(String, Option<String>, Option<String>, bool)],
    name: &str,
    kind: &str,
) -> Vec<&'a (String, Option<String>, Option<String>, bool)> {
    nodes
        .iter()
        .filter(|(label, ..)| names_row(label, name, kind))
        .collect()
}

/// Whether an announced table row is the one for `name` of `kind`: its name
/// first, behind the severity of a problem's mark when it has one, its kind
/// next.
fn names_row(label: &str, name: &str, kind: &str) -> bool {
    label.strip_prefix("Name: ").is_some_and(|rest| {
        severity(rest).1.starts_with(&format!("{name}, "))
            && rest.contains(&format!("Kind: {kind}, "))
    })
}

/// What a table row says of its dependency's status: for a problem, the
/// severity its name is marked with and the words at the right end of its
/// location; nothing for a sound one.
fn row_said(label: &str) -> Option<Said> {
    let severity = label
        .strip_prefix("Name: ")
        .and_then(|rest| severity(rest).0);
    let location = label.split_once("Location: ").map_or("", |(_, rest)| rest);
    let location = location
        .split_once(", Access: ")
        .map_or(location, |(location, _)| location);
    match location.rsplit_once(", ") {
        Some((_, state)) => Some((state.to_owned(), severity)),
        None => severity.map(|severity| (String::new(), Some(severity))),
    }
}

/// What the table and the inspector say about a library whose status is
/// `label`, marked `mark`: the inspector always, the table only a problem.
pub(super) fn expected_said(label: &str, mark: Option<&str>) -> (Option<Said>, Said) {
    let said: Said = (label.to_owned(), mark.map(str::to_owned));
    (mark.map(|_| said.clone()), said)
}

/// What the table and the inspector say about `library`, in the frame
/// after it was selected.
pub(super) fn said_about(app: &mut RSpiceApp, library: &DependencyLibrary) -> (Option<Said>, Said) {
    app.state.project.project_dependency_selection = Some(library.clone());
    let ctx = offscreen();
    app_frame(&ctx, app, Vec::new());
    let output = app_frame(&ctx, app, Vec::new());
    let nodes = announced(&output);

    let table = table_row(&nodes, library.shown_name(), library.kind.label());
    assert_eq!(
        table.len(),
        1,
        "one table row names {library:?}: {nodes:#?}"
    );
    let table = row_said(&table[0].0);

    let statuses: Vec<_> = nodes
        .iter()
        .filter(|(label, ..)| label == "Status")
        .collect();
    assert_eq!(statuses.len(), 1, "the inspector states one status");
    let inspector = (
        statuses[0].1.clone().expect("the status row has a value"),
        statuses[0].2.clone(),
    );
    (table, inspector)
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
    let said = said_about(app, &library);
    assert_eq!(
        said,
        expected_said(status, mark),
        "the table and the inspector on {library:?}"
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
/// the table has no Technology row, nothing speaks of a missing binding,
/// and Add offers to attach one.
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
    let page = app_frame(&ctx, &mut libraries.app, Vec::new());
    let rows = ledger_rows(&page);
    assert!(
        !rows.iter().any(|row| row.contains("Kind: Technology, ")),
        "{rows:#?}"
    );
    let text = painted(&page);
    assert!(!text.contains("binding"), "{text}");
    app_frame(&ctx, &mut libraries.app, click_on(&page, "Add"));
    let menu = app_frame(&ctx, &mut libraries.app, Vec::new());
    assert_eq!(offered(&menu, &["Technology\u{2026}"]), [Some(true)]);
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

    let design = table_row(&nodes, "shared", "Design library");
    let model = table_row(&nodes, "shared", "Model library");
    assert_eq!((design.len(), model.len()), (1, 1), "{nodes:#?}");
    assert!(!design[0].3 && model[0].3, "{design:?} {model:?}");
    let text = painted(&output);
    assert!(text.contains("Model library"), "{text}");
    let status: Vec<_> = nodes
        .iter()
        .filter(|(label, ..)| label == "Status")
        .collect();
    assert_eq!(status[0].1.as_deref(), Some("Built in"));
}

/// The bar's filter keeps the rows that say what it holds, and a project
/// that replaces this one starts with neither the filter nor the selection.
#[test]
fn the_bar_filter_keeps_matching_rows_and_is_lost_with_the_project() {
    let mut libraries = libraries();
    let app = &mut libraries.app;
    app.state.project.dependencies.filter = "CHANGED".to_owned();
    let name = libraries.changed.clone();
    app.state.project.project_dependency_selection = Some(DependencyLibrary::model(&name));
    let ctx = offscreen();
    app_frame(&ctx, app, Vec::new());
    let nodes = announced(&app_frame(&ctx, app, Vec::new()));
    assert!(
        table_row(&nodes, &libraries.found, "Model library").is_empty(),
        "the filter hides it"
    );
    assert_eq!(table_row(&nodes, &name, "Model library").len(), 1);
    assert!(
        nodes
            .iter()
            .any(|(label, value, ..)| label == "Filter dependencies"
                && value.as_deref() == Some("CHANGED")),
        "the bar's filter holds it"
    );

    crate::workbench::app_state::services_access::reset_for_new_project(&mut app.state);
    assert_eq!(app.state.project.dependencies.filter, "");
    assert_eq!(app.state.project.project_dependency_selection, None);
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// The table's rows as announced, top to bottom.
fn ledger_rows(output: &egui::FullOutput) -> Vec<String> {
    let mut rows: Vec<_> = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| {
            let label = node.label()?;
            label.starts_with("Name: ").then(|| {
                (
                    node.bounds().map_or(0.0, |bounds| bounds.y0),
                    label.to_owned(),
                )
            })
        })
        .collect();
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    rows.into_iter().map(|(_, label)| label).collect()
}

/// A secondary press and release on the table row for `name` of `kind`, as
/// a right-click asks for its menu.
fn menu_on(output: &egui::FullOutput, name: &str, kind: &str) -> Vec<egui::Event> {
    press_on(output, name, kind, egui::PointerButton::Secondary)
}

/// A press and release of `button` on the table row for `name` of `kind`.
fn press_on(
    output: &egui::FullOutput,
    name: &str,
    kind: &str,
    button: egui::PointerButton,
) -> Vec<egui::Event> {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| names_row(node.label()?, name, kind).then(|| node.bounds())?)
        .unwrap_or_else(|| panic!("the table shows {name} of {kind}"));
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    [true, false]
        .into_iter()
        .map(|pressed| egui::Event::PointerButton {
            pos: at,
            button,
            pressed,
            modifiers: egui::Modifiers::default(),
        })
        .collect()
}

/// A press of `key` with `modifiers` held.
fn key(key: egui::Key, modifiers: egui::Modifiers) -> Vec<egui::Event> {
    vec![egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }]
}

/// Whether each of `labels` is announced, and can be pressed.
pub(super) fn offered(output: &egui::FullOutput, labels: &[&str]) -> Vec<Option<bool>> {
    let nodes = &output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes;
    labels
        .iter()
        .map(|label| {
            nodes
                .iter()
                .find_map(|(_, node)| (node.label()? == *label).then(|| !node.is_disabled()))
        })
        .collect()
}

/// The page's menu for the row for `name` of `kind`, opened on a settled
/// page.
pub(super) fn open_menu(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    name: &str,
    kind: &str,
) -> egui::FullOutput {
    app_frame(ctx, app, Vec::new());
    let page = app_frame(ctx, app, Vec::new());
    app_frame(ctx, app, menu_on(&page, name, kind));
    app_frame(ctx, app, Vec::new())
}

/// Frames until the Console holds a line that starts with `start`.
pub(super) fn paint_until_recorded(ctx: &egui::Context, app: &mut RSpiceApp, start: &str) {
    for _ in 0..500 {
        app_frame(ctx, app, Vec::new());
        if app
            .state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with(start))
        {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let lines: Vec<_> = app
        .state
        .model
        .log_buffer
        .entries()
        .map(|entry| entry.message.clone())
        .collect();
    panic!("the Console never said `{start}`: {lines:#?}");
}

/// However long the list, a library the reader has to act on is found by
/// its problem's words in the bar's filter; a sound library's status is not
/// shown, so its words find nothing.
#[test]
fn a_problem_is_found_by_its_words_and_a_sound_status_is_not() {
    let mut libraries = libraries();
    let ctx = offscreen();
    libraries.app.state.project.dependencies.filter = "not found".to_owned();
    app_frame(&ctx, &mut libraries.app, Vec::new());
    let rows = ledger_rows(&app_frame(&ctx, &mut libraries.app, Vec::new()));
    assert_eq!(rows.len(), 1, "{rows:#?}");
    assert!(
        names_row(&rows[0], &libraries.missing, "Model library"),
        "{rows:#?}"
    );
    assert_eq!(
        row_said(&rows[0]),
        Some(("Not found".to_owned(), Some("Error".to_owned())))
    );

    libraries.app.state.project.dependencies.filter = "in project".to_owned();
    app_frame(&ctx, &mut libraries.app, Vec::new());
    let page = app_frame(&ctx, &mut libraries.app, Vec::new());
    assert!(ledger_rows(&page).is_empty(), "{:#?}", ledger_rows(&page));
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// Each library's row counts what it provides, from the catalogue that
/// holds it: a design library its cells, a model library its models. A
/// library whose file is gone has nothing to count, and says so with a
/// dash.
#[test]
fn each_library_counts_what_it_provides() {
    let mut libraries = libraries();
    let mut parts = Library::new("parts");
    for name in ["amp", "bias", "filter"] {
        parts.add_cell(rspice_app_state::state::Cell::new(name));
    }
    libraries.app.state.model.library_manager.add_library(parts);
    let ctx = offscreen();
    app_frame(&ctx, &mut libraries.app, Vec::new());
    let nodes = announced(&app_frame(&ctx, &mut libraries.app, Vec::new()));
    let contents = |name: &str, kind: &str| {
        let row = table_row(&nodes, name, kind);
        assert_eq!(row.len(), 1, "one row names {name}: {nodes:#?}");
        let label = &row[0].0;
        label
            .split_once("Contents: ")
            .and_then(|(_, rest)| rest.split(", Location: ").next())
            .unwrap_or_else(|| panic!("{label}"))
            .to_owned()
    };
    assert_eq!(contents("parts", "Design library"), "3 cells");
    assert_eq!(contents("cells", "Design library"), "0 cells");
    assert_eq!(contents(&libraries.found, "Model library"), "1 model");
    assert_eq!(contents(&libraries.missing, "Model library"), "\u{2014}");
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A model library read from a file offers, in its menu, every way to act
/// on it from here; Open in Models takes the reader to it there.
#[test]
fn a_model_library_opens_in_models_from_its_row() {
    let mut libraries = libraries();
    let name = libraries.found.clone();
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut libraries.app, &name, "Model library");
    assert_eq!(
        offered(
            &menu,
            &[
                "Open in Models",
                "Show in folder",
                "Change\u{2026}",
                "Reload",
                "Remove\u{2026}"
            ]
        ),
        [Some(true); 5]
    );
    app_frame(&ctx, &mut libraries.app, click_on(&menu, "Open in Models"));
    app_frame(&ctx, &mut libraries.app, Vec::new());
    assert_eq!(libraries.app.state.workbench.workspace, Workspace::Models);
    assert_eq!(
        libraries
            .app
            .state
            .model
            .model_library_manager
            .selected_library
            .as_deref(),
        Some(name.as_str())
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// The inspector states what a model library's row has no column for: on
/// whose authority it is read, its digest, how many models it holds and each
/// file it was read from, whole; and its access, which is read-only.
#[test]
fn a_model_library_states_its_authority_digest_models_and_files_in_the_inspector() {
    let mut libraries = libraries();
    let name = libraries.found.clone();
    libraries.app.state.project.project_dependency_selection =
        Some(DependencyLibrary::model(&name));
    let ctx = offscreen();
    app_frame(&ctx, &mut libraries.app, Vec::new());
    let output = app_frame(&ctx, &mut libraries.app, Vec::new());
    let nodes = announced(&output);
    let value = |label: &str| {
        let found: Vec<_> = nodes
            .iter()
            .filter(|(said, ..)| said == label)
            .map(|(_, value, ..)| value.clone())
            .collect();
        assert_eq!(found.len(), 1, "one {label} row: {nodes:#?}");
        found[0].clone().unwrap_or_default()
    };
    let library = libraries
        .app
        .state
        .model
        .model_library_manager
        .get_library(&name)
        .expect("the library is attached")
        .clone();
    let authority = match library.source_authority {
        ModelSourceAuthority::BuiltIn => "Ships with RSpice",
        ModelSourceAuthority::External => "Linked to its file",
        ModelSourceAuthority::RetainedImport { .. } => "Kept as imported",
        ModelSourceAuthority::ProjectOwned { .. } => "Project-owned",
    };
    assert_eq!(value("Authority"), authority);
    assert_eq!(value("Access"), "Read-only");
    let pin = library
        .source_closure
        .first()
        .expect("an imported library pins its file");
    assert_eq!(
        value("Digest"),
        pin.digest.to_string().chars().take(8).collect::<String>()
    );
    assert_eq!(value("Models"), library.models.len().to_string());
    let file = pin
        .path
        .file_name()
        .and_then(|file| file.to_str())
        .expect("a file name");
    // Plain words reach a screen reader as a value with no label, so every
    // node is read, labelled or not, with where it stands: the table's
    // Location says the same path, so the file is looked for under the
    // heading, in the heading's column.
    let words: Vec<(String, egui::accesskit::Rect)> = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| Some((node, node.bounds()?)))
        .flat_map(|(node, bounds)| {
            [node.label(), node.value()]
                .into_iter()
                .flatten()
                .map(move |text| (text.to_owned(), bounds))
        })
        .collect();
    let (_, heading) = words
        .iter()
        .find(|(text, _)| text.starts_with("Files"))
        .unwrap_or_else(|| panic!("the files have a heading of their own: {words:#?}"));
    assert!(
        words.iter().any(|(text, at)| text.ends_with(file)
            && at.y0 >= heading.y1
            && at.x0 >= heading.x0 - 1.0
            && at.x0 < heading.x1),
        "the file it was read from is stated under Files: {words:#?}"
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// The built-in library ships with RSpice: it opens in Models, there is no
/// file to show or read, and Change… and Remove… say why they are not taken.
#[test]
fn the_built_in_library_offers_only_to_open() {
    let mut libraries = libraries();
    let name = libraries.built_in.clone();
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut libraries.app, &name, "Model library");
    assert_eq!(
        offered(
            &menu,
            &[
                "Open in Models",
                "Show in folder",
                "Change\u{2026}",
                "Reload",
                "Remove\u{2026}"
            ]
        ),
        [Some(true), None, Some(false), None, Some(false)]
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A design library opens in the Library Manager, selected, and its menu's
/// Remove… opens the review that deletes one the project file holds.
#[test]
fn a_design_library_opens_on_the_library_page_and_reviews_its_deletion() {
    let mut libraries = libraries();
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut libraries.app, "cells", "Design library");
    assert_eq!(
        offered(&menu, &["Open in Library Manager", "Remove\u{2026}"]),
        [Some(true), Some(true)]
    );
    app_frame(
        &ctx,
        &mut libraries.app,
        click_on(&menu, "Open in Library Manager"),
    );
    assert_eq!(
        libraries.app.state.project.project_page,
        ProjectPage::Library
    );
    assert_eq!(
        libraries
            .app
            .state
            .model
            .library_manager
            .selected_library
            .as_deref(),
        Some("cells")
    );

    libraries.app.state.project.project_page = ProjectPage::Dependencies;
    let menu = open_menu(&ctx, &mut libraries.app, "cells", "Design library");
    app_frame(&ctx, &mut libraries.app, click_on(&menu, "Remove\u{2026}"));
    assert!(libraries.app.state.project.delete_library_dialog);
    assert!(
        libraries
            .app
            .state
            .model
            .library_manager
            .get_library("cells")
            .is_some(),
        "the review deletes it, not the menu"
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// Show in folder opens the folder the library's file is in.
#[test]
fn a_model_library_is_shown_in_its_folder() {
    let mut libraries = libraries();
    let name = libraries.found.clone();
    SHOWN_FOLDERS.with(|shown| shown.borrow_mut().clear());
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut libraries.app, &name, "Model library");
    app_frame(&ctx, &mut libraries.app, click_on(&menu, "Show in folder"));
    let shown = SHOWN_FOLDERS.with(|shown| shown.borrow().clone());
    assert_eq!(shown.len(), 1, "{shown:?}");
    assert!(
        shown[0].join("found.lib").is_file(),
        "{} holds the file",
        shown[0].display()
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A model file edited since it was attached is read again from the page:
/// the library takes what the file now says and is found again.
#[test]
fn a_changed_model_library_is_reloaded_from_its_row() {
    let mut libraries = libraries();
    let name = libraries.changed.clone();
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut libraries.app, &name, "Model library");
    app_frame(&ctx, &mut libraries.app, click_on(&menu, "Reload"));
    paint_until_recorded(
        &ctx,
        &mut libraries.app,
        &format!("Reloaded model library '{name}'"),
    );
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "Found",
        None,
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// A model library whose file is gone is pointed at the file where it now
/// is: the library keeps its name and is found there.
#[test]
fn a_missing_model_library_is_relinked_from_its_row() {
    let mut libraries = libraries();
    let name = libraries.missing.clone();
    let moved = libraries.folder.join("moved");
    std::fs::create_dir_all(&moved).expect("the new folder is made");
    let file = moved.join(format!("{name}.lib"));
    std::fs::write(&file, ".model nmissing nmos level=1 vto=0.5\n").expect("the file writes");
    RELINK_PICK.with(|pick| *pick.borrow_mut() = Some(file.clone()));
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut libraries.app, &name, "Model library");
    app_frame(&ctx, &mut libraries.app, click_on(&menu, "Change\u{2026}"));
    paint_until_recorded(
        &ctx,
        &mut libraries.app,
        &format!("Relinked model library '{name}'"),
    );
    let root = libraries
        .app
        .state
        .model
        .model_library_manager
        .get_library(&name)
        .and_then(|library| library.root_path.clone())
        .expect("the library has a root");
    assert_eq!(
        std::fs::canonicalize(root).ok(),
        std::fs::canonicalize(&file).ok()
    );
    assert_said(
        &mut libraries.app,
        DependencyLibrary::model(name),
        "Found",
        None,
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// Removing a model library from the page is one step of Undo, and the
/// project's technology library is kept until the technology is detached.
#[test]
fn a_model_library_removed_from_its_row_comes_back_with_undo() {
    let (mut app, root) = saved_project("dependency-remove");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    let (name, _) = import_file(&mut app, &root, "spare.lib", "nspare");
    let ctx = offscreen();
    let menu = open_menu(&ctx, &mut app, &name, "Model library");
    app_frame(&ctx, &mut app, click_on(&menu, "Remove\u{2026}"));
    assert!(
        app.state
            .model
            .model_library_manager
            .get_library(&name)
            .is_none()
    );
    assert!(
        app.state.model.log_buffer.entries().any(|entry| entry
            .message
            .starts_with(&format!("Removed model library '{name}'"))),
        "the Console records it"
    );
    assert!(app.command_key_enabled(ids::UNDO));
    app.execute_command_key(ids::UNDO);
    assert!(
        app.state
            .model
            .model_library_manager
            .get_library(&name)
            .is_some(),
        "Undo puts it back"
    );
    let _ = std::fs::remove_dir_all(root);
}

/// Enter on a design library's row opens it in the Library Manager, as the
/// first command of its menu does.
#[test]
fn enter_on_a_design_library_opens_it_in_the_library_manager() {
    let mut libraries = libraries();
    let ctx = offscreen();
    app_frame(&ctx, &mut libraries.app, Vec::new());
    let page = app_frame(&ctx, &mut libraries.app, Vec::new());
    let row = press_on(
        &page,
        "cells",
        "Design library",
        egui::PointerButton::Primary,
    );
    app_frame(&ctx, &mut libraries.app, row);
    app_frame(&ctx, &mut libraries.app, Vec::new());
    app_frame(
        &ctx,
        &mut libraries.app,
        key(egui::Key::Enter, egui::Modifiers::NONE),
    );
    assert_eq!(
        libraries.app.state.project.project_page,
        ProjectPage::Library
    );
    assert_eq!(
        libraries
            .app
            .state
            .model
            .library_manager
            .selected_library
            .as_deref(),
        Some("cells")
    );
    let _ = std::fs::remove_dir_all(libraries.folder);
}

/// The folders the netlister searches for an include, by their names.
fn searched(app: &RSpiceApp) -> Vec<String> {
    app.state
        .model
        .include_search_chain()
        .directories()
        .iter()
        .filter_map(|folder| Some(folder.file_name()?.to_str()?.to_owned()))
        .collect()
}

/// An include search path is added from the bar's Add, moved up with
/// Alt+Up and removed with Delete; each is one step of Undo, and each
/// changes the folders the netlister searches.
#[test]
fn an_include_path_is_added_moved_and_removed_one_undo_step_each() {
    let (mut app, root) = saved_project("dependency-include-paths");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Dependencies;
    for folder in ["first", "second"] {
        std::fs::create_dir_all(root.join(folder)).expect("the folder is made");
    }
    let ctx = offscreen();
    for folder in ["first", "second"] {
        app_frame(&ctx, &mut app, Vec::new());
        let page = app_frame(&ctx, &mut app, Vec::new());
        answer_next_folder_pick(&root.join(folder));
        app_frame(&ctx, &mut app, click_on(&page, "Add"));
        let menu = app_frame(&ctx, &mut app, Vec::new());
        app_frame(&ctx, &mut app, click_on(&menu, "Include path\u{2026}"));
    }
    assert_eq!(searched(&app), ["first", "second"]);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .project
            .include_search_paths(),
        [PathBuf::from("first"), PathBuf::from("second")],
        "a folder in the project is kept from the project folder"
    );
    app.execute_command_key(ids::UNDO);
    assert_eq!(searched(&app), ["first"], "one step takes one path out");
    app.execute_command_key(ids::REDO);
    assert_eq!(searched(&app), ["first", "second"]);

    app_frame(&ctx, &mut app, Vec::new());
    let page = app_frame(&ctx, &mut app, Vec::new());
    let row = press_on(
        &page,
        "second",
        "Include path",
        egui::PointerButton::Primary,
    );
    app_frame(&ctx, &mut app, row);
    app_frame(&ctx, &mut app, Vec::new());
    app_frame(
        &ctx,
        &mut app,
        key(egui::Key::ArrowUp, egui::Modifiers::ALT),
    );
    assert_eq!(searched(&app), ["second", "first"], "searched first now");
    app.execute_command_key(ids::UNDO);
    assert_eq!(
        searched(&app),
        ["first", "second"],
        "one step moves it back"
    );
    app.execute_command_key(ids::REDO);
    assert_eq!(searched(&app), ["second", "first"]);

    app_frame(&ctx, &mut app, Vec::new());
    app_frame(
        &ctx,
        &mut app,
        key(egui::Key::Delete, egui::Modifiers::NONE),
    );
    assert_eq!(
        searched(&app),
        ["first"],
        "Delete removes the selected path"
    );
    app.execute_command_key(ids::UNDO);
    assert_eq!(searched(&app), ["second", "first"], "one step puts it back");
    let _ = std::fs::remove_dir_all(root);
}
