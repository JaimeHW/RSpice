//! Library operations as steps of the one Undo: each create, copy, rename
//! and delete, at library, cell and view level, undoes as one step that puts
//! the saved project, the tabs and the drawing in front back exactly as
//! they stood, redoes to exactly what it made, and leaves the steps taken
//! before it undoable.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::ProjectFeature;
use rspice_app_project::workbench::project::library_dialogs::new_cell::create_new_cell;
use rspice_app_project::workbench::project::library_dialogs::new_view::create_new_view;
use rspice_app_project::workbench::project::workspace_state::{
    DeletionInstanceResolution, LibraryDeletionTarget,
};
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, OpenCellView, Point,
    SchematicState, View, ViewType,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

use super::with_cx;

/// A project whose top places a master from a library of parts, with that
/// master's drawing placing another, three drawings open in tabs, a
/// configuration rooted on the master and a Verilog-A source it owns, and a
/// library of its own that nothing else uses.
fn app_with_parts() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let state = &mut app.state;

    let mut parts = Library::new("parts");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    amp.add_view(View::new("model", ViewType::VerilogA));
    parts.add_cell(amp);
    for name in ["leaf", "spare"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        parts.add_cell(cell);
    }
    state.model.library_manager.add_library(parts);
    let mut scratch = Library::new("scratch");
    let mut note = Cell::new("note");
    note.add_view(View::new("schematic", ViewType::Schematic));
    scratch.add_cell(note);
    state.model.library_manager.add_library(scratch);

    let top = CellViewRef::new("user", "top", "schematic");
    let amp = CellViewRef::new("parts", "amp", "schematic");
    let leaf = CellViewRef::new("parts", "leaf", "schematic");
    let spare = CellViewRef::new("parts", "spare", "schematic");
    let note = CellViewRef::new("scratch", "note", "schematic");

    let mut top_drawing = SchematicState::default();
    top_drawing.add_library_cell_component(
        Point::new(10, 10),
        LibraryCellInstance::new("parts", "amp", "schematic"),
    );
    let mut amp_drawing = SchematicState::default();
    amp_drawing.add_component(ComponentType::Resistor, Point::new(0, 0));
    amp_drawing.add_library_cell_component(
        Point::new(40, 0),
        LibraryCellInstance::new("parts", "leaf", "schematic"),
    );
    let mut leaf_drawing = SchematicState::default();
    leaf_drawing.add_component(ComponentType::Resistor, Point::new(20, 20));
    let workspace = &mut state.model.workspace;
    workspace.insert_schematic_editor(top.key(), top_drawing.clone());
    workspace.insert_schematic_editor(amp.key(), amp_drawing);
    workspace.insert_schematic_editor(leaf.key(), leaf_drawing);
    workspace.insert_schematic_editor(spare.key(), SchematicState::default());
    workspace.insert_schematic_editor(note.key(), SchematicState::default());
    workspace.content.open_views = [&top, &amp, &leaf, &note]
        .into_iter()
        .map(|reference| OpenCellView::new(reference.clone(), ViewType::Schematic))
        .collect();
    workspace.content.active_view = top;
    workspace.content.reroot_active_occurrence();
    state.model.schematic = top_drawing;

    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(rspice_app_state::state::ConfigurationSetDefinition {
            name: "amp bench".to_owned(),
            root: amp,
            dut_path: "/".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: rspice_app_state::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                rspice_app_state::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: rspice_app_state::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Library undo test".to_owned(),
        })
        .expect("a valid configuration root");
    let bundle = rspice_app_state::state::ProjectSourceBundle::try_new(
        rspice_app_state::state::ProjectSourceOwner::cell_view(CellViewRef::new(
            "parts", "amp", "model",
        )),
        rspice_app_state::state::ProjectSourceLanguage::VerilogA,
        "model.va".to_owned(),
        "module model(p, n); inout p, n; endmodule".to_owned(),
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("a valid source bundle");
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("a unique source owner");

    // As just saved: no drawing differs from the project file.
    state.model.workspace.mark_all_clean();
    state.model.schematic.session.is_dirty = false;
    state.open_project_session();
    state.workbench.activate(Workspace::Project);
    app
}

/// What a library operation and its Undo are held to: the project as Save
/// writes it, every drawing it holds, the tabs with the one in front, and
/// the drawing the editor shows. Which tabs are marked unsaved is held
/// apart: a drawing an Undo puts back is unsaved, since the file may hold
/// the version the operation left.
struct Standing {
    saved: Value,
    drawings: BTreeMap<String, Value>,
    tabs: Value,
    open: BTreeSet<String>,
    unsaved: BTreeSet<String>,
    editor: Value,
}

impl Standing {
    fn of(app: &RSpiceApp) -> Self {
        let snapshot =
            rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&app.state)
                .expect("the project is captured");
        let text = rspice_app_services::io::project_io::serialize_project_file(&snapshot)
            .expect("the project is written");
        let mut saved = serde_json::from_str(&text).expect("the project file is JSON");
        forget_counters(&mut saved);
        let content = &app.state.model.workspace.content;
        let front = content.active_schematic_reference().key();
        let drawings = content
            .schematic_buffers
            .iter()
            .map(|(key, stored)| {
                let document = if *key == front {
                    app.state.model.schematic.document()
                } else {
                    stored.document()
                };
                let document = serde_json::to_value(document).expect("the drawing is written");
                (key.clone(), document)
            })
            .collect();
        let mut tabs = serde_json::json!({
            "open": content.open_views,
            "front": content.active_view,
        });
        if let Some(Value::Array(open)) = tabs.get_mut("open") {
            for tab in open {
                if let Value::Object(fields) = tab {
                    fields.remove("dirty");
                }
            }
        }
        Self {
            saved,
            drawings,
            tabs,
            open: content
                .open_views
                .iter()
                .map(|open| open.reference.key())
                .collect(),
            unsaved: content
                .open_views
                .iter()
                .filter(|open| open.dirty)
                .map(|open| open.reference.key())
                .collect(),
            editor: serde_json::to_value(app.state.model.schematic.document())
                .expect("the drawing in front is written"),
        }
    }

    /// The drawings that are not the same in `self` and `other`.
    fn rewritten(&self, other: &Self) -> BTreeSet<String> {
        self.drawings
            .keys()
            .chain(other.drawings.keys())
            .filter(|key| self.drawings.get(*key) != other.drawings.get(*key))
            .cloned()
            .collect()
    }

    /// Every place where `self` is not `expected`, by path.
    fn differences(&self, expected: &Self) -> Vec<String> {
        let mut found = Vec::new();
        differences(&expected.saved, &self.saved, "file", &mut found);
        differences(&expected.editor, &self.editor, "editor", &mut found);
        differences(&expected.tabs, &self.tabs, "tabs", &mut found);
        for key in self.rewritten(expected) {
            found.push(format!("drawing {key}"));
        }
        found
    }
}

/// Take out what only ever moves forward: the governed revisions an Undo
/// advances like any other change, and the library audit trail, which
/// records the Undo as a change of its own.
fn forget_counters(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            fields.retain(|key, _| {
                key != "revision" && !key.ends_with("_revision") && !key.contains("audit")
            });
            fields.values_mut().for_each(forget_counters);
        }
        Value::Array(items) => items.iter_mut().for_each(forget_counters),
        _ => {}
    }
}

fn differences(expected: &Value, found: &Value, path: &str, out: &mut Vec<String>) {
    match (expected, found) {
        (Value::Object(expected), Value::Object(found)) => {
            let keys = expected
                .keys()
                .chain(found.keys())
                .collect::<std::collections::BTreeSet<_>>();
            for key in keys {
                match (expected.get(key), found.get(key)) {
                    (Some(left), Some(right)) => {
                        differences(left, right, &format!("{path}.{key}"), out);
                    }
                    _ => out.push(format!("{path}.{key}")),
                }
            }
        }
        (Value::Array(expected), Value::Array(found)) if expected.len() == found.len() => {
            for (index, (left, right)) in expected.iter().zip(found).enumerate() {
                differences(left, right, &format!("{path}[{index}]"), out);
            }
        }
        _ if expected != found => out.push(path.to_owned()),
        _ => {}
    }
}

/// The reference process and temperature the setup states.
fn reference_pvt(app: &RSpiceApp) -> String {
    let setup = &app.state.model.sim_setup;
    format!("{:?}", (setup.reference_pvt, setup.options.temp))
}

/// Take an earlier setup step, then `operation`. The operation is one step
/// of Undo: Undo puts back exactly what stood before it, with a catalog
/// stamp no earlier catalog had, Redo makes exactly what it made, and once
/// it is undone the earlier setup step still undoes.
fn one_step(mut app: RSpiceApp, description: &str, operation: impl FnOnce(&mut RSpiceApp)) {
    let pvt = reference_pvt(&app);
    assert!(
        rspice_app_simulate::workbench::simulate::chrome::commit_reference_pvt(
            &mut crate::workbench::features::cx(&mut app),
            rspice_app_state::product::ProcessCorner::FF,
            -40.0,
        )
        .expect("the reference PVT commits")
    );
    assert!(app.state.model.can_undo_simulation_setup());
    assert!(!app.state.model.can_undo_project_design());

    let before = Standing::of(&app);
    let mut stamps = vec![app.state.model.library_manager.content_stamp()];
    operation(&mut app);
    let after = Standing::of(&app);
    stamps.push(app.state.model.library_manager.content_stamp());
    assert!(
        !after.differences(&before).is_empty(),
        "{description} changed nothing"
    );
    assert!(
        app.state.model.can_undo_simulation_setup(),
        "{description} emptied the setup's history"
    );
    assert!(
        app.command_key_enabled(ids::UNDO),
        "{description} is not a step of Undo"
    );

    app.execute_command_key(ids::UNDO);
    let undone = app.state.model.library_manager.content_stamp();
    assert!(
        !stamps.contains(&undone),
        "the catalog {description} undid to reuses an earlier stamp"
    );
    stamps.push(undone);
    let undone = Standing::of(&app);
    let found = undone.differences(&before);
    assert!(
        found.is_empty(),
        "Undo of {description} left these unlike before: {found:#?}"
    );
    let rewritten = after.rewritten(&before);
    assert_eq!(
        undone.unsaved,
        before
            .unsaved
            .iter()
            .chain(rewritten.intersection(&before.open))
            .cloned()
            .collect::<BTreeSet<_>>(),
        "Undo of {description} marks unsaved exactly the open drawings it put back"
    );
    assert!(
        !app.state.model.can_undo_project_design(),
        "{description} took more than one step to undo"
    );
    assert!(app.command_key_enabled(ids::REDO));

    app.execute_command_key(ids::REDO);
    assert!(!stamps.contains(&app.state.model.library_manager.content_stamp()));
    let found = Standing::of(&app).differences(&after);
    assert!(
        found.is_empty(),
        "Redo of {description} left these unlike it made them: {found:#?}"
    );

    app.execute_command_key(ids::UNDO);
    let found = Standing::of(&app).differences(&before);
    assert!(
        found.is_empty(),
        "a second Undo of {description} left these unlike before: {found:#?}"
    );
    assert_eq!(
        app.state
            .model
            .undo_simulation_setup()
            .expect("the setup step is still there")
            .map(|_| ()),
        Ok(()),
        "the setup step taken before {description} no longer undoes"
    );
    assert_eq!(reference_pvt(&app), pvt);
}

/// Carry out a confirmed deletion review and what it raised, as the dialog
/// walk does.
fn delete(app: &mut RSpiceApp) {
    with_cx(app, ProjectFeature::process_pending_library_deletions);
    app.after_contribution();
}

#[test]
fn creating_a_library_undoes_as_one_step() {
    one_step(app_with_parts(), "creating a library", |app| {
        app.state
            .model
            .create_library("extra")
            .expect("the library is created");
    });
}

#[test]
fn renaming_a_library_undoes_as_one_step() {
    one_step(app_with_parts(), "renaming a library", |app| {
        app.state
            .rename_library("parts", "blocks")
            .expect("the library is renamed");
    });
}

#[test]
fn deleting_a_library_undoes_as_one_step() {
    one_step(app_with_parts(), "deleting a library", |app| {
        app.state
            .delete_library("scratch")
            .expect("the library is deleted");
    });
}

#[test]
fn creating_a_cell_undoes_as_one_step() {
    one_step(app_with_parts(), "creating a cell", |app| {
        let project = &mut app.state.project;
        project.new_cell_library = "parts".to_owned();
        project.new_cell_name = "fresh".to_owned();
        project.new_cell_create_schematic = true;
        app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
        assert!(with_cx(app, create_new_cell), "the cell is created");
        app.after_contribution();
    });
}

#[test]
fn copying_a_cell_undoes_as_one_step() {
    one_step(app_with_parts(), "copying a cell", |app| {
        app.state
            .copy_cell("parts", "amp", "scratch", "amp_copy")
            .expect("the cell is copied");
    });
}

#[test]
fn renaming_a_cell_undoes_as_one_step() {
    one_step(app_with_parts(), "renaming a cell", |app| {
        app.state
            .rename_cell("parts", "leaf", "twig")
            .expect("the cell is renamed");
    });
}

#[test]
fn deleting_a_cell_with_its_placements_undoes_as_one_step() {
    one_step(app_with_parts(), "deleting a cell", |app| {
        app.state.project.pending_delete_cell = Some(("parts".to_owned(), "leaf".to_owned()));
        app.state.project.library_deletion_review.resolution =
            Some(DeletionInstanceResolution::RemoveInstances);
        delete(app);
        assert!(
            app.state
                .model
                .library_manager
                .get_library("parts")
                .and_then(|library| library.get_cell("leaf"))
                .is_none(),
            "the cell is deleted"
        );
    });
}

#[test]
fn creating_a_view_undoes_as_one_step() {
    one_step(app_with_parts(), "creating a view", |app| {
        let project = &mut app.state.project;
        project.new_view_library = "parts".to_owned();
        project.new_view_cell = "spare".to_owned();
        project.new_view_name = "alternate".to_owned();
        project.new_view_type = ViewType::Schematic;
        app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
        assert!(with_cx(app, create_new_view), "the view is created");
        app.after_contribution();
    });
}

#[test]
fn renaming_a_view_undoes_as_one_step() {
    one_step(app_with_parts(), "renaming a view", |app| {
        app.state
            .rename_view("parts", "amp", "model", "behaviour")
            .expect("the view is renamed");
    });
}

#[test]
fn deleting_a_view_undoes_as_one_step() {
    one_step(app_with_parts(), "deleting a view", |app| {
        app.state.project.pending_delete_view =
            Some(("parts".to_owned(), "amp".to_owned(), "model".to_owned()));
        delete(app);
        assert!(
            app.state
                .model
                .library_manager
                .get_library("parts")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("model"))
                .is_none(),
            "the view is deleted"
        );
    });
}

/// Twelve cells chosen together are deleted by one review, and one step of
/// Undo puts every one of them back.
#[test]
fn deleting_twelve_cells_at_once_undoes_as_one_step() {
    let mut app = app_with_parts();
    let mut bulk = Library::new("bulk");
    let names = (1..=12).map(|n| format!("cell{n:02}")).collect::<Vec<_>>();
    for name in &names {
        let mut cell = Cell::new(name.as_str());
        cell.add_view(View::new("schematic", ViewType::Schematic));
        bulk.add_cell(cell);
    }
    app.state.model.library_manager.add_library(bulk);
    let targets = names
        .iter()
        .map(|cell| LibraryDeletionTarget::Cell {
            library: "bulk".to_owned(),
            cell: cell.clone(),
        })
        .collect::<Vec<_>>();
    one_step(app, "deleting twelve cells", |app| {
        let state = &mut app.state;
        state
            .project
            .open_library_deletions_review(&state.model, targets)
            .expect("one review opens on all twelve");
        assert_eq!(state.project.library_deletion_review.targets().len(), 12);
        state
            .project
            .confirm_library_deletion_review(
                &state.model,
                DeletionInstanceResolution::KeepUnresolved,
            )
            .expect("the review confirms");
        delete(app);
        assert_eq!(
            app.state
                .model
                .library_manager
                .get_library("bulk")
                .map(|library| library.cell_count()),
            Some(0),
            "every cell is deleted"
        );
    });
}

/// A step that was edited past refuses with its reason and changes nothing,
/// rather than putting back a catalog over the edit.
#[test]
fn an_operation_edited_since_refuses_to_undo() {
    let mut app = app_with_parts();
    app.state
        .rename_cell("parts", "spare", "extra")
        .expect("the cell is renamed");
    app.state
        .rename_cell("parts", "extra", "again")
        .expect("the renamed cell is renamed again");
    let edited = app
        .state
        .model
        .library_manager
        .get_library_mut("parts")
        .expect("the parts library");
    edited
        .get_cell_mut("again")
        .expect("the renamed cell")
        .add_view(View::new("notes", ViewType::Schematic));
    let standing = Standing::of(&app);

    let refusal = app
        .state
        .undo_project_design()
        .expect_err("the step's cell was edited since");

    assert!(
        refusal.contains("rename cell parts/extra to parts/again"),
        "{refusal}"
    );
    assert!(refusal.contains("edited since"), "{refusal}");
    assert!(Standing::of(&app).differences(&standing).is_empty());
}
