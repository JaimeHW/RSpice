//! Every Verilog-A source is a cell view.
//!
//! A project used to carry a project-wide Verilog-A bundle beside its cell
//! views, seeded with a demonstration module. These pins drive what replaced
//! it: a project still holding one moves it into a cell view when it loads, a
//! deck that named it keeps resolving, File > Import makes cell views of its
//! own, and a deck's reference to a module opens that module's tab.

use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, frame, open_in_design, open_project_app, themed_context,
};
use super::*;
use crate::io::ProjectSnapshot;
use crate::state::{
    Cell, CellViewRef, Library, PortDirection, PortSpec, ProjectSourceBundle,
    ProjectSourceDocument, ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    SymbolDocument, View, ViewType,
};
use crate::workbench::documents::code_workspace;
use crate::workbench::state::Workspace;

/// The demonstration module every project used to be created with, exactly.
const RETIRED_DEMO: &str = "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule";

/// The same module after its author changed it.
const EDITED_MODULE: &str = "`include \"constants.vams\"\n`include \"disciplines.vams\"\nmodule sensor_bridge(out, inp, inn);\n  inout out, inp, inn;\n  electrical out, inp, inn;\n  parameter real gain = 50.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule\n";

fn code_workspace_owner() -> ProjectSourceOwner {
    ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA)
}

/// The text of a project whose project-wide Verilog-A bundle holds `source`
/// under `sensor_bridge.va`, compiled, after `prepare` has had its say.
fn legacy_project_text(source: &str, prepare: impl FnOnce(&mut AppState)) -> String {
    legacy_project(source, prepare).0
}

/// [`legacy_project_text`], and the identity its Verilog-A bundle persisted.
fn legacy_project(
    source: &str,
    prepare: impl FnOnce(&mut AppState),
) -> (String, crate::state::ProjectSourceId) {
    legacy_project_at("sensor_bridge.va", source, prepare)
}

/// [`legacy_project`] with the project-wide source's root file at `root`.
fn legacy_project_at(
    root: &str,
    source: &str,
    prepare: impl FnOnce(&mut AppState),
) -> (String, crate::state::ProjectSourceId) {
    let mut state = AppState::default();
    state
        .workspace
        .content
        .project_sources
        .insert(
            ProjectSourceDocument::try_new(root, ProjectSourceLanguage::VerilogA, source)
                .expect("a valid legacy source"),
        )
        .expect("one project-wide Verilog-A source");
    state
        .workspace
        .content
        .project_sources
        .mark_validated(ProjectSourceLanguage::VerilogA)
        .expect("the legacy source was compiled");
    let id = state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&code_workspace_owner())
        .map(ProjectSourceBundle::id)
        .expect("the legacy bundle");
    prepare(&mut state);
    let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("the legacy project snapshots");
    (
        crate::io::project_io::serialize_project_file(&snapshot).expect("the legacy project saves"),
        id,
    )
}

fn load(text: &str) -> ProjectSnapshot {
    crate::io::project_io::load_project_text(text, None).expect("the project loads")
}

/// A project's persisted content as a value, so two loads compare by what
/// they hold rather than by the order a hash map happened to write it in.
fn project_content(project: &ProjectSnapshot) -> serde_json::Value {
    serde_json::to_value(&project.file).expect("the project encodes")
}

fn veriloga_bundles(sources: &ProjectSourceRegistry) -> Vec<&ProjectSourceBundle> {
    sources
        .iter_bundles()
        .filter(|bundle| bundle.language() == ProjectSourceLanguage::VerilogA)
        .collect()
}

/// An application over a loaded project, the way opening it installs one.
fn install(project: ProjectSnapshot) -> RSpiceApp {
    let mut app = open_project_app();
    app.state.library_manager = project.file.libraries;
    app.state.workspace = crate::state::ProjectWorkspace::from_parts(
        project.file.workspace,
        project.workspace_session,
    );
    app
}

/// The demonstration module carried nothing its author wrote: it goes, and
/// nothing is created in its place.
#[test]
fn an_untouched_demo_loads_as_no_bundle_and_no_cell() {
    let text = legacy_project_text(RETIRED_DEMO, |_| {});
    let original = load(&text);
    let cells_before = AppState::default()
        .library_manager
        .libraries_by_key()
        .map(|(_, library)| library.cell_count())
        .sum::<usize>();

    assert!(veriloga_bundles(&original.file.workspace.project_sources).is_empty());
    assert_eq!(
        original
            .file
            .libraries
            .libraries_by_key()
            .map(|(_, library)| library.cell_count())
            .sum::<usize>(),
        cells_before,
        "no cell was created for the demonstration module"
    );
    assert!(
        original
            .file
            .workspace_migration_warning
            .as_deref()
            .is_none_or(|warning| !warning.contains("Verilog-A")),
        "dropping the demonstration module is silent"
    );
}

/// An edited project-wide source is its author's: it becomes the cell view
/// its top module names, owning the same bundle, with the module and ports
/// New View would have written, and the load says so once. Loading the same
/// file again does the same thing, and loading the saved result does nothing.
#[test]
fn an_edited_project_wide_source_becomes_a_cell_view_once() {
    let (text, legacy_id) = legacy_project(EDITED_MODULE, |_| {});

    let loaded = load(&text);
    let library = loaded.file.workspace.project.root_library.clone();
    let reference = CellViewRef::new(&library, "sensor_bridge", "veriloga");
    let bundles = veriloga_bundles(&loaded.file.workspace.project_sources);
    assert_eq!(bundles.len(), 1);
    assert_eq!(bundles[0].id(), legacy_id);
    assert_eq!(
        bundles[0].owner(),
        &ProjectSourceOwner::cell_view(reference.clone())
    );
    assert_eq!(bundles[0].root().content(), EDITED_MODULE);
    assert!(
        bundles[0].validation_is_current(),
        "the compile it carried still describes the same bytes"
    );
    assert!(
        loaded
            .file
            .workspace
            .project_sources
            .bundle_for_owner(&code_workspace_owner())
            .is_none()
    );

    let view = loaded
        .file
        .libraries
        .get_library(&library)
        .and_then(|library| library.get_cell("sensor_bridge"))
        .and_then(|cell| cell.get_view("veriloga"))
        .expect("the module's cell view exists");
    assert_eq!(view.view_type, ViewType::VerilogA);
    assert_eq!(
        view.metadata.get("veriloga.module").map(String::as_str),
        Some("sensor_bridge")
    );
    assert_eq!(
        view.metadata.get("veriloga.ports").map(String::as_str),
        Some(r#"["out","inp","inn"]"#)
    );

    let notice = loaded
        .file
        .workspace_migration_warning
        .clone()
        .expect("the load says what it created");
    assert_eq!(
        notice.matches(&reference.display_path()).count(),
        1,
        "one notice names the cell view: {notice}"
    );

    let again = load(&text);
    assert_eq!(
        project_content(&again),
        project_content(&loaded),
        "loading the same file twice migrates it the same way"
    );

    let saved = crate::io::project_io::serialize_project_file(&loaded).unwrap();
    let reloaded = load(&saved);
    assert!(
        reloaded
            .file
            .workspace_migration_warning
            .as_deref()
            .is_none_or(|warning| !warning.contains("Verilog-A")),
        "the saved project is already in the new form"
    );
    assert_eq!(
        project_content(&reloaded),
        project_content(&loaded),
        "loading the migrated project again changes nothing"
    );

    // The migrated view is a Verilog-A cell view like any other: its module
    // binding resolves, and it opens in Design over the bundle it owns.
    let mut app = install(loaded);
    let binding = crate::state::workspace::project_veriloga_binding_for_view(
        &app.state.workspace,
        &app.state.library_manager,
        &reference,
    )
    .expect("the view's module lookup resolves");
    assert_eq!(binding.selected_module(), "sensor_bridge");
    open_in_design(&mut app.state, &reference);
    let selected = code_workspace::selected_veriloga_source(&app).expect("the tab has a source");
    assert_eq!(selected.bundle().id(), legacy_id);
}

/// A deck that named the project-wide source by its key keeps running after
/// the move: the key it was written with names the same bytes.
#[test]
fn a_deck_written_against_the_project_wide_source_still_prepares_after_migration() {
    let text = legacy_project_text(EDITED_MODULE, |state| {
        let bundle = state
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&code_workspace_owner())
            .expect("the legacy bundle");
        let key = crate::state::project_sources::project_veriloga_bundle_source_key(
            state.workspace.content.project.id(),
            bundle,
            "sensor_bridge",
        )
        .expect("the legacy key");
        let directive = rspice_simulation::netlist_preparation::project_veriloga_directive(
            &key,
            "sensor_bridge",
        );
        state.workspace.content.netlist_source = Some(format!(
            "bridge deck\nV1 inp 0 DC 1\nV2 inn 0 DC 0\nR1 out 0 1k\nX1 out inp inn sensor_bridge\n{directive}\n.op\n.end\n"
        ));
    });
    let deck = load(&text)
        .file
        .workspace
        .netlist_source
        .expect("the deck is kept as written");
    assert!(
        deck.contains("__rspice_project__/"),
        "the deck still names the source by the key it was written with"
    );

    let app = install(load(&text));
    let mut controller = crate::simulation::SimulationController::new();
    controller
        .validate_manual_deck_document(&app.state)
        .expect("the deck prepares against the migrated cell view's bundle");
}

/// A module lands in the cell named after it, unless that name is taken by a
/// cell that already has a Verilog-A view or declares a different interface;
/// then the first free `<module>_va`, `<module>_va2`, ... A cell of that name
/// that declares the module's interface and has no Verilog-A view takes it.
#[test]
fn a_module_takes_its_own_cell_name_or_the_first_free_suffixed_one() {
    let mut libraries = crate::state::LibraryManager::default();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("veriloga", ViewType::VerilogA));
    library.add_cell(amp);
    library.add_cell(Cell::new("amp_va"));
    for (name, pins) in [("buf", ["in", "out"]), ("inv", ["a", "y"])] {
        let mut cell = Cell::new(name);
        let mut symbol = View::new("symbol", ViewType::Symbol);
        SymbolDocument::generated_from_ports(
            &pins
                .iter()
                .map(|pin| PortSpec {
                    name: (*pin).to_owned(),
                    direction: PortDirection::InOut,
                })
                .collect::<Vec<_>>(),
        )
        .store_in_view(&mut symbol)
        .unwrap();
        cell.add_view(symbol);
        library.add_cell(cell);
    }
    libraries.add_library(library);
    let no_schematics = |_: &CellViewRef| None;
    let interface_of = |cell: &Cell, reference: &CellViewRef| {
        crate::state::workspace::declared_cell_interface(&no_schematics, cell, &reference.library)
    };
    let module = |name: &str, ports: [&str; 2]| crate::state::workspace::DeclaredModule {
        name: name.to_owned(),
        ports: ports.iter().map(|port| (*port).to_owned()).collect(),
    };
    let place = |name: &str, ports: [&str; 2], reserved: &[String]| {
        crate::state::workspace::veriloga_cell_for_module(
            &libraries,
            &interface_of,
            "work",
            &module(name, ports),
            reserved,
        )
    };

    assert_eq!(
        place("fresh", ["p", "n"], &[]),
        (CellViewRef::new("work", "fresh", "veriloga"), true)
    );
    assert_eq!(
        place("amp", ["p", "n"], &[]),
        (CellViewRef::new("work", "amp_va2", "veriloga"), true),
        "amp has a Verilog-A view and amp_va is taken"
    );
    assert_eq!(
        place("buf", ["in", "out"], &[]),
        (CellViewRef::new("work", "buf", "veriloga"), false),
        "buf declares the module's interface and has no Verilog-A view"
    );
    assert_eq!(
        place("inv", ["in", "out"], &[]),
        (CellViewRef::new("work", "inv_va", "veriloga"), true),
        "inv declares a different interface"
    );
    assert_eq!(
        place("fresh", ["p", "n"], &["fresh".to_owned()]),
        (CellViewRef::new("work", "fresh_va", "veriloga"), true),
        "a name the same transaction claimed is taken"
    );
    assert_eq!(
        place("AMP", ["p", "n"], &[]),
        (CellViewRef::new("work", "AMP_va2", "veriloga"), true),
        "a name taken in another case is taken"
    );
    assert_eq!(
        place("étage", ["p", "n"], &["ÉTAGE".to_owned()]),
        (CellViewRef::new("work", "étage_va", "veriloga"), true),
        "a name claimed in another Unicode case is taken"
    );
}

/// The same rule over Unicode: a cell that differs from the module's name
/// only by case or composition is that name, for File > Import and for the
/// migration alike, so neither creates a cell beside one it aliases.
#[test]
fn a_module_never_lands_beside_a_cell_its_name_aliases() {
    let mut libraries = crate::state::LibraryManager::default();
    let mut library = Library::new("work");
    let mut upper = Cell::new("ÉTAGE");
    upper.add_view(View::new("veriloga", ViewType::VerilogA));
    library.add_cell(upper);
    // "é" spelled as "e" and a combining acute accent.
    let mut decomposed = Cell::new("cafe\u{301}");
    decomposed.add_view(View::new("veriloga", ViewType::VerilogA));
    library.add_cell(decomposed);
    libraries.add_library(library);
    let no_interface = |_: &Cell, _: &CellViewRef| Vec::new();
    let place = |name: &str| {
        crate::state::workspace::veriloga_cell_for_module(
            &libraries,
            &no_interface,
            "work",
            &crate::state::workspace::DeclaredModule {
                name: name.to_owned(),
                ports: Vec::new(),
            },
            &[],
        )
    };

    assert_eq!(
        place("étage"),
        (CellViewRef::new("work", "étage_va", "veriloga"), true)
    );
    assert_eq!(
        place("caf\u{e9}"),
        (CellViewRef::new("work", "caf\u{e9}_va", "veriloga"), true)
    );
}

/// A project-wide source moves into a cell view under the rule above. Its
/// root's stem names the cell when the source does not parse, so an accented
/// stem whose cell exists in another case lands at the suffixed name rather
/// than failing to move.
#[test]
fn a_migrated_source_whose_name_is_taken_in_another_case_lands_at_the_suffixed_cell() {
    let (text, legacy_id) = legacy_project_at("étage.va", "module broken(p, n;\n", |state| {
        let library = state.workspace.content.project.root_library.clone();
        let mut cell = Cell::new("ÉTAGE");
        cell.add_view(View::new("veriloga", ViewType::VerilogA));
        state
            .library_manager
            .get_library_mut(&library)
            .expect("the design library")
            .add_cell(cell);
        state
            .workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(CellViewRef::new(&library, "ÉTAGE", "veriloga")),
                    ProjectSourceLanguage::VerilogA,
                    "existing.va",
                    "module existing(p, n); inout p, n; electrical p, n; endmodule\n",
                    [],
                    [],
                )
                .expect("the occupying view's own source"),
            )
            .expect("one bundle per view");
    });
    let loaded = load(&text);
    let library = loaded.file.workspace.project.root_library.clone();
    let moved = loaded
        .file
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(CellViewRef::new(
            &library,
            "étage_va",
            "veriloga",
        )))
        .expect("the source moved to the suffixed cell");
    assert_eq!(moved.id(), legacy_id);
    assert!(
        loaded
            .file
            .libraries
            .get_library(&library)
            .is_some_and(|library| library.get_cell("étage").is_none()),
        "no cell was created beside the one its name aliases"
    );
}

/// Rename the `logical_path` fields of a persisted project that read `from`.
fn rename_logical_path(value: &mut serde_json::Value, from: &str, to: &str) -> usize {
    match value {
        serde_json::Value::Object(fields) => fields
            .iter_mut()
            .map(|(key, field)| {
                if key == "logical_path" && field.as_str() == Some(from) {
                    *field = serde_json::Value::from(to);
                    1
                } else {
                    rename_logical_path(field, from, to)
                }
            })
            .sum(),
        serde_json::Value::Array(items) => items
            .iter_mut()
            .map(|item| rename_logical_path(item, from, to))
            .sum(),
        _ => 0,
    }
}

/// The migration moves a bundle it read, and reading one refuses two files
/// that differ only by Unicode case: such a project never reaches the move.
#[test]
fn a_legacy_source_holding_unicode_case_alias_paths_is_refused_at_load() {
    let text = legacy_project_text(EDITED_MODULE, |state| {
        let id = state
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&code_workspace_owner())
            .map(ProjectSourceBundle::id)
            .expect("the legacy bundle");
        for path in ["models/étage.vams", "models/other.vams"] {
            state
                .workspace
                .content
                .add_project_source_bundle_file(
                    id,
                    "sensor_bridge.va",
                    crate::state::ProjectSourceFile::try_new(path, "// stage\n")
                        .expect("a valid dependency"),
                )
                .expect("the dependency is added");
        }
    });
    load(&text);

    let mut value = serde_json::from_str::<serde_json::Value>(&text).expect("the project is JSON");
    assert_eq!(
        rename_logical_path(&mut value, "models/other.vams", "MODELS/ÉTAGE.VAMS"),
        1
    );
    let aliased = serde_json::to_string_pretty(&value).expect("the project encodes");
    let Err(error) = crate::io::project_io::load_project_text(&aliased, None) else {
        panic!("a source holding two spellings of one path loaded");
    };
    let error = format!("{error:?}");
    assert!(error.contains("case-insensitively"), "{error}");
}

/// A project-wide source whose cell name is taken lands at the suffixed name.
#[test]
fn a_migrated_source_whose_name_is_taken_lands_at_the_suffixed_cell() {
    let text = legacy_project_text(EDITED_MODULE, |state| {
        let library = state.workspace.content.project.root_library.clone();
        let mut cell = Cell::new("sensor_bridge");
        cell.add_view(View::new("veriloga", ViewType::VerilogA));
        state
            .library_manager
            .get_library_mut(&library)
            .expect("the design library")
            .add_cell(cell);
        state
            .workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(CellViewRef::new(
                        &library,
                        "sensor_bridge",
                        "veriloga",
                    )),
                    ProjectSourceLanguage::VerilogA,
                    "existing.va",
                    "module existing(p, n); inout p, n; electrical p, n; endmodule\n",
                    [],
                    [],
                )
                .expect("the occupying view's own source"),
            )
            .expect("one bundle per view");
    });
    let loaded = load(&text);
    let library = loaded.file.workspace.project.root_library.clone();
    assert!(
        loaded
            .file
            .workspace
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(CellViewRef::new(
                &library,
                "sensor_bridge_va",
                "veriloga",
            )))
            .is_some()
    );
}

const TWO_TOP_MODULES: &str = "`include \"disciplines.vams\"\nmodule child(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\nmodule first(a, b);\n  inout a, b;\n  electrical a, b;\n  child c1(a, b);\nendmodule\nmodule second(x, y);\n  inout x, y;\n  electrical x, y;\n  analog I(x, y) <+ V(x, y) / 1k;\nendmodule\n";

/// With no Verilog-A tab in front, an imported file is read into the form,
/// and committing it makes one cell view per top module, each owning a
/// bundle of the file, and opens the first in Design. A module the file only
/// instantiates is not a top and gets no cell.
#[test]
fn import_with_no_veriloga_tab_creates_the_cells_and_opens_the_first() {
    let mut app = open_project_app();
    let library = app.state.workspace.content.active_view.library.clone();
    code_workspace::import_dropped_veriloga_source(
        &mut app,
        "pair.va".to_owned(),
        TWO_TOP_MODULES.to_owned(),
    )
    .expect("the file is read");
    let form = app
        .state
        .ui
        .code_workspace
        .veriloga
        .import_form
        .clone()
        .expect("the import form opened");
    assert_eq!(form.library, library, "the active design's library");
    assert_eq!(
        form.modules.as_ref().map(|modules| modules
            .iter()
            .map(|module| module.name.as_str())
            .collect::<Vec<_>>()),
        Ok(vec!["first", "second"])
    );

    let first = app.commit_veriloga_import().expect("the cells are created");

    assert_eq!(first, CellViewRef::new(&library, "first", "veriloga"));
    for (cell, module) in [("first", "first"), ("second", "second")] {
        let reference = CellViewRef::new(&library, cell, "veriloga");
        let bundle = app
            .state
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
            .expect("each module's view owns a bundle");
        assert_eq!(bundle.root().logical_path(), "pair.va");
        assert_eq!(bundle.root().content(), TWO_TOP_MODULES);
        let binding = crate::state::workspace::project_veriloga_binding_for_view(
            &app.state.workspace,
            &app.state.library_manager,
            &reference,
        )
        .expect("each view binds its module");
        assert_eq!(binding.selected_module(), module);
    }
    assert!(
        app.state
            .library_manager
            .get_library(&library)
            .and_then(|library| library.get_cell("child"))
            .is_none(),
        "an instantiated module is not a top"
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.workspace.content.active_view, first);
    assert!(app.state.ui.code_workspace.veriloga.import_form.is_none());
}

/// A file the front end cannot read is refused in the form, and committing
/// it anyway creates nothing.
#[test]
fn import_of_an_unparseable_file_creates_nothing() {
    let mut app = open_project_app();
    let sources = app.state.workspace.content.project_sources.clone();
    let cells = app
        .state
        .library_manager
        .libraries_by_key()
        .map(|(_, library)| library.cell_count())
        .sum::<usize>();
    code_workspace::import_dropped_veriloga_source(
        &mut app,
        "broken.va".to_owned(),
        "module broken(p, n;\nendmodule\n".to_owned(),
    )
    .expect("the file is read");
    let form = app
        .state
        .ui
        .code_workspace
        .veriloga
        .import_form
        .clone()
        .expect("the form opens to say why");
    assert!(form.modules.is_err());

    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);
    let refusal = output
        .platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update.nodes.iter().any(|(_, node)| {
                node.value()
                    .is_some_and(|value| value.starts_with("broken.va cannot be read"))
            })
        });
    assert!(refusal, "the status line states the refusal");

    assert!(app.commit_veriloga_import().is_err());
    assert_eq!(app.state.workspace.content.project_sources, sources);
    assert_eq!(
        app.state
            .library_manager
            .libraries_by_key()
            .map(|(_, library)| library.cell_count())
            .sum::<usize>(),
        cells
    );
}

/// The text of the root file `reference`'s bundle holds.
fn root_text(app: &RSpiceApp, reference: &CellViewRef) -> String {
    app.state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
        .expect("the view owns a bundle")
        .root()
        .content()
        .to_owned()
}

/// File > Import with a Verilog-A tab in front makes cell views of its own,
/// exactly as with none: the form opens, and the module being edited keeps
/// every byte, before the commit and after it.
#[test]
fn import_with_a_veriloga_tab_in_front_opens_the_form_and_leaves_the_view_alone() {
    use crate::workbench::commands::vocabulary::Command;

    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let sources = app.state.workspace.content.project_sources.clone();
    let text = root_text(&app, &reference);

    Command::ImportVerilogA.execute(&mut app);
    assert!(app.state.ui.code_workspace.veriloga.import_requested);
    code_workspace::complete_veriloga_file_pick(&mut app, "pair.va", TWO_TOP_MODULES);

    let form = app
        .state
        .ui
        .code_workspace
        .veriloga
        .import_form
        .clone()
        .expect("the import form opened");
    assert_eq!(form.file_name, "pair.va");
    assert!(
        app.state
            .ui
            .code_workspace
            .veriloga
            .replace_review
            .is_none()
    );
    assert_eq!(
        app.state.workspace.content.project_sources, sources,
        "nothing changes until the form commits"
    );
    assert_eq!(root_text(&app, &reference), text);

    app.commit_veriloga_import().expect("the cells are created");
    assert_eq!(
        root_text(&app, &reference),
        text,
        "the view that was in front is not the import's"
    );
}

/// A file dropped onto the Design Verilog-A editor goes to the import form;
/// it never replaces the source under the pointer.
#[test]
fn a_file_dropped_on_the_veriloga_editor_opens_the_import_form() {
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let sources = app.state.workspace.content.project_sources.clone();
    let text = root_text(&app, &reference);

    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, DESKTOP)),
            dropped_files: vec![egui::DroppedFile {
                name: "pair.va".to_owned(),
                bytes: Some(TWO_TOP_MODULES.as_bytes().into()),
                ..Default::default()
            }],
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    );

    assert_eq!(
        app.state
            .ui
            .code_workspace
            .veriloga
            .import_form
            .as_ref()
            .map(|form| form.file_name.as_str()),
        Some("pair.va"),
        "the drop opened the import form"
    );
    assert_eq!(app.state.workspace.content.project_sources, sources);
    assert_eq!(root_text(&app, &reference), text);
}

const REPLACEMENT: &str = "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n) / 2k;\nendmodule\n";

/// "Replace from file…" in the Manage source document dialog for the open
/// `model.va`, through the picker, to the confirmation it opens.
fn pick_replacement(ctx: &egui::Context, app: &mut RSpiceApp) {
    crate::workbench::commands::vocabulary::Command::ManageSourceDocument.execute(app);
    let mut output = frame(ctx, app, DESKTOP);
    for _ in 0..12 {
        output = frame(ctx, app, DESKTOP);
    }
    let replace = node_center(&output, "Replace from file…")
        .expect("Manage source document offers Replace from file");
    click(ctx, app, replace);
    assert!(
        app.state.ui.code_workspace.veriloga.import_requested,
        "Replace from file asks for a file"
    );
    assert!(
        !app.state.ui.code_workspace.source_document_dialog,
        "the picker and the confirmation take over from the dialog"
    );
    code_workspace::complete_veriloga_file_pick(app, "replacement.va", REPLACEMENT);
    assert!(app.state.ui.code_workspace.veriloga.import_form.is_none());
}

/// The confirmation, settled: a few frames so it has faded in.
fn settled_confirmation(ctx: &egui::Context, app: &mut RSpiceApp) -> egui::FullOutput {
    for _ in 0..12 {
        frame(ctx, app, DESKTOP);
    }
    frame(ctx, app, DESKTOP)
}

/// "Replace source from file…" replaces nothing when the file arrives: the
/// confirmation names the file it replaces, Cancel leaves it byte for byte,
/// and only Replace rewrites it, keeping its previous text in the history.
#[test]
fn replace_source_from_file_replaces_only_after_its_confirmation() {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let original = root_text(&app, &reference);

    pick_replacement(&ctx, &mut app);
    assert_eq!(
        root_text(&app, &reference),
        original,
        "the pick replaced nothing"
    );
    let review = app
        .state
        .ui
        .code_workspace
        .veriloga
        .replace_review
        .clone()
        .expect("the confirmation opened");
    assert!(review.restorable, "the history keeps the replaced text");
    let output = settled_confirmation(&ctx, &mut app);
    let names_the_file = output
        .platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update.nodes.iter().any(|(_, node)| {
                node.label() == Some("Replaces") && node.value() == Some("model.va")
            })
        });
    assert!(
        names_the_file,
        "the confirmation names the file it replaces"
    );
    let cancel = node_center(&output, "Cancel").expect("the confirmation offers Cancel");
    click(&ctx, &mut app, cancel);
    assert!(
        app.state
            .ui
            .code_workspace
            .veriloga
            .replace_review
            .is_none()
    );
    assert_eq!(
        root_text(&app, &reference),
        original,
        "Cancel replaced nothing"
    );

    pick_replacement(&ctx, &mut app);
    let bundle = |app: &RSpiceApp| {
        app.state
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .expect("the bundle")
            .clone()
    };
    let previous = bundle(&app).revision();
    let output = settled_confirmation(&ctx, &mut app);
    let replace = node_center(&output, "Replace").expect("the confirmation offers Replace");
    click(&ctx, &mut app, replace);

    assert!(
        app.state
            .ui
            .code_workspace
            .veriloga
            .replace_review
            .is_none()
    );
    assert_eq!(root_text(&app, &reference), REPLACEMENT);
    assert_eq!(
        bundle(&app)
            .revision_snapshot(previous)
            .and_then(|snapshot| snapshot.file_content("model.va")),
        Some(original.as_str()),
        "the replaced text stays restorable"
    );
}

/// The bounds of the accessible node labelled `label`, in points.
fn node_center(output: &egui::FullOutput, label: &str) -> Option<egui::Pos2> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find(|(_, node)| node.label() == Some(label))
        .and_then(|(_, node)| node.bounds())
        .map(|bounds| {
            egui::pos2(
                ((bounds.x0 + bounds.x1) * 0.5) as f32,
                ((bounds.y0 + bounds.y1) * 0.5) as f32,
            )
        })
}

fn click(ctx: &egui::Context, app: &mut RSpiceApp, at: egui::Pos2) {
    for pressed in [true, false] {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, DESKTOP)),
                events: vec![
                    egui::Event::PointerMoved(at),
                    egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                ..Default::default()
            },
            |ui| {
                app.prepare_frame(ui.ctx());
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            },
        );
    }
}

/// A deck whose `.veriloga` card names `reference`'s module by its key.
fn deck_naming(app: &RSpiceApp, reference: &CellViewRef, module: &str) -> String {
    let bundle = app
        .state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
        .expect("the view owns a bundle");
    let key = crate::state::project_sources::project_veriloga_bundle_source_key(
        app.state.workspace.content.project.id(),
        bundle,
        module,
    )
    .expect("the module's key");
    format!(
        "owned deck\nV1 p 0 DC 1\nX1 p 0 {module}\n{}\n.op\n.end\n",
        rspice_simulation::netlist_preparation::project_veriloga_directive(&key, module)
    )
}

/// The Netlist workspace's dependency row for a module names its cell view
/// and opens that view's tab in Design. There is no second editor.
#[test]
fn open_in_design_from_a_netlist_dependency_row_opens_the_tab() {
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    app.state.workspace.content.netlist_source = Some(deck_naming(&app, &reference, "gain"));
    app.state.workbench.activate(Workspace::Netlist);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);
    let row = node_center(&output, "gain \u{00b7} veriloga")
        .expect("the dependency row names the module's cell view");

    click(&ctx, &mut app, row);

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.workspace.content.active_view, reference);
    assert!(
        app.state
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == reference),
        "the module's tab is open"
    );
}

/// Renders for review: the import form, the confirmation "Replace source from
/// file…" opens, a save prompt beside them, and the Netlist workspace with
/// its two pages and a module dependency row.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_veriloga_import_and_netlist_dependency_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let mut write = |name: &str, canvas: &crate::ui::raster::Canvas, height: usize| {
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    };

    for (name, contents) in [
        ("veriloga-import-form", TWO_TOP_MODULES),
        (
            "veriloga-import-refused",
            "module broken(p, n;\nendmodule\n",
        ),
    ] {
        for (suffix, size) in [("1440x900", DESKTOP), ("390x844", egui::vec2(390.0, 844.0))] {
            let mut app = open_project_app();
            code_workspace::import_dropped_veriloga_source(
                &mut app,
                "pair.va".to_owned(),
                contents.to_owned(),
            )
            .expect("the file is read");
            let canvas = crate::ui::raster::render(size, |ui, _| {
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            });
            write(&format!("{name}-{suffix}"), &canvas, size.y as usize);
        }
    }

    for (suffix, size) in [("1440x900", DESKTOP), ("390x844", egui::vec2(390.0, 844.0))] {
        let mut app = open_project_app();
        let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
        open_in_design(&mut app.state, &reference);
        code_workspace::request_veriloga_source_replacement(&mut app, "model.va")
            .expect("the open view's file can be replaced");
        code_workspace::complete_veriloga_file_pick(&mut app, "gain_rev_b.va", REPLACEMENT);
        let canvas = crate::ui::raster::render(size, |ui, _| {
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        });
        write(
            &format!("veriloga-replace-confirm-{suffix}"),
            &canvas,
            size.y as usize,
        );
    }

    // A prompt outside this family, for the header inset it shares.
    let mut app = open_project_app();
    app.state
        .dialogs
        .confirmation_dialog
        .show(super::ConfirmationAction::CloseProject);
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| {
        app.render_frame_chrome(ui);
        app.render_frame_dialogs(ui.ctx());
    });
    write(
        "prompt-save-confirmation-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );

    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    app.state.workspace.content.netlist_source = Some(deck_naming(&app, &reference, "gain"));
    app.state.workbench.activate(Workspace::Netlist);
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| app.render_frame_chrome(ui));
    write(
        "netlist-dependency-open-in-design-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );

    let mut automation = open_project_app();
    automation.state.workbench.activate(Workspace::Netlist);
    automation.state.ui.code_workspace.page = code_workspace::CodeWorkspacePage::Automation;
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| automation.render_frame_chrome(ui));
    write(
        "netlist-workspace-automation-page-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );
}
