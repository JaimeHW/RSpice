//! Creating a library cell through the whole application: New cell drawn in
//! the feature's context lent from RSpiceApp, and the cell it creates there,
//! with what the creation raised answered as the dialog step answers it.

use crate::state::{
    CellViewRef, ComponentType, Library, NetNamingPolicy, Point, SchematicGridPitch, SymbolDocument,
};
use crate::workbench::app::dialogs::library::LIBRARY_CATALOG_STALE_MESSAGE;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::library_dialogs::new_cell::create_new_cell;
use crate::workbench::{ChoicePreference, RSpiceApp};

use super::{in_front, published, shown, with_cx};

/// Create the cell the dialog names in the feature's context, with
/// what it raised answered as the dialog step answers it, and whether
/// it was created.
fn create(app: &mut RSpiceApp) -> bool {
    let created = with_cx(app, create_new_cell);
    app.after_contribution();
    created
}

/// New cell is the command form with option columns: the four views
/// are checks on one line of the grid under one label, each in an equal
/// share, with no kicker over them and no switches.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_new_cell_form_puts_its_views_on_the_grid() {
    let mut app = RSpiceApp::test_instance();
    app.state.project.new_cell_dialog = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    let painted = crate::ui::widgets::painted_runs::painted_runs(
        egui::vec2(1440.0, 900.0),
        crate::ui::tokens::Mode::Light,
        3,
        |ctx| {
            with_cx(&mut app, |cx| {
                ProjectFeature::process_new_cell_dialog(cx, ctx)
            })
        },
    );
    assert_eq!(painted.surface.width(), 520.0);
    assert!(painted.rect_of("VIEWS TO CREATE").is_none());
    let views = painted.rect_of("Views").expect("the Views row");
    let library = painted.rect_of("Library").expect("the Library row");
    assert!((views.left() - library.left()).abs() < 0.5);
    // One line, and the views' words level with the row's label.
    for view in ["Schematic", "Symbol", "Testbench", "Verilog-A"] {
        let rect = painted.rect_of(view).expect(view);
        assert!(
            (rect.center().y - views.center().y).abs() < 1.0,
            "{view} is not on the Views line"
        );
    }
    assert!(painted.rect_of("Enter a name for the cell.").is_some());
}

fn cell_symbol_view(app: &RSpiceApp, library: &str, cell: &str) -> crate::state::View {
    app.state
        .model
        .library_manager
        .get_library(library)
        .and_then(|library| library.get_cell(cell))
        .and_then(|cell| cell.get_view("symbol"))
        .cloned()
        .expect("the created cell owns a symbol view")
}

#[test]
fn creating_cell_seeds_every_schematic_like_view_at_creation_time() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .library_manager
        .add_library(Library::new("policy_test"));
    app.state
        .model
        .preferences
        .set_choice(ChoicePreference::SchematicGrid, 2)
        .unwrap();
    app.state
        .model
        .preferences
        .set_choice(ChoicePreference::NetNamingPolicy, 1)
        .unwrap();
    app.state.project.new_cell_library = "policy_test".to_owned();
    app.state.project.new_cell_name = "amp".to_owned();
    app.state.project.new_cell_create_schematic = true;
    app.state.project.new_cell_create_testbench = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    let project_revision_before = app.state.model.workspace.content.project.revision().get();

    let created = create(&mut app);

    assert!(created);
    assert_eq!(
        app.state.model.workspace.content.project.revision().get(),
        project_revision_before + 1
    );
    assert!(matches!(
        app.state
            .model.workspace
            .content.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::CreateCell { library, cell })
            if library == "policy_test" && cell == "amp"
    ));
    assert!(app.state.model.workspace.content.project_metadata_dirty);
    for view in ["schematic", "testbench"] {
        let key = crate::state::CellViewRef::new("policy_test", "amp", view).key();
        let document = app
            .state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&key)
            .expect("document seeded when view is created");
        assert_eq!(
            document.document().document_policy.grid_pitch,
            SchematicGridPitch::Metric
        );
        assert_eq!(
            document.document().grid_size,
            SchematicGridPitch::Metric.canvas_grid_size()
        );
        assert_eq!(
            document.document().document_policy.net_naming,
            NetNamingPolicy::SpiceCompatibleRelaxed
        );
    }
}

#[test]
fn new_cell_symbol_view_follows_the_schematic_interface_until_first_edited() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .library_manager
        .add_library(Library::new("tracking_test"));
    app.state.project.new_cell_library = "tracking_test".to_owned();
    app.state.project.new_cell_name = "amp".to_owned();
    app.state.project.new_cell_create_schematic = true;
    app.state.project.new_cell_create_symbol = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();

    let created = create(&mut app);

    assert!(created);
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("tracking_test", "amp", "schematic")
    );
    let created = cell_symbol_view(&app, "tracking_test", "amp");
    assert_eq!(
        created.metadata.get("generated").map(String::as_str),
        Some("ports"),
        "a symbol created with the cell is the schematic's until hand edited"
    );
    assert!(
        SymbolDocument::load_from_view(&created)
            .expect("the seeded symbol document parses")
            .pins
            .is_empty()
    );

    let port_id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Port, Point::origin());
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("the placed interface port")
        .value = "IN".to_owned();
    app.state.sync_active_schematic_to_workspace();

    let refreshed = cell_symbol_view(&app, "tracking_test", "amp");
    assert_eq!(
        refreshed.metadata.get("ports").map(String::as_str),
        Some("IN:inout")
    );
    assert_eq!(
        SymbolDocument::load_from_view(&refreshed)
            .expect("the refreshed symbol document parses")
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        ["IN"]
    );
}

/// The Verilog-A switch makes the view New Verilog-A device… makes, by
/// the same path: a starter module, its source, and the symbol its
/// compile generates, in the one cell transaction.
#[test]
fn new_cell_verilog_a_switch_creates_the_module_view_and_its_generated_symbol() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .library_manager
        .add_library(Library::new("behavioral"));
    app.state.project.new_cell_library = "behavioral".to_owned();
    app.state.project.new_cell_name = "comparator".to_owned();
    app.state.project.new_cell_create_schematic = false;
    app.state.project.new_cell_create_veriloga = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    let project_revision_before = app.state.model.workspace.content.project.revision().get();

    let created = create(&mut app);

    assert!(created, "{:?}", app.state.project.new_cell_error);
    assert_eq!(
        app.state.model.workspace.content.project.revision().get(),
        project_revision_before + 1
    );
    let reference = CellViewRef::new("behavioral", "comparator", "veriloga");
    let cell = app
        .state
        .model
        .library_manager
        .get_library("behavioral")
        .and_then(|library| library.get_cell("comparator"))
        .expect("the created cell");
    let veriloga = cell.get_view("veriloga").expect("the Verilog-A view");
    assert_eq!(
        veriloga.metadata.get("veriloga.module").map(String::as_str),
        Some("comparator")
    );
    assert_eq!(
        veriloga.metadata.get("veriloga.ports").map(String::as_str),
        Some("[]")
    );
    let symbol = cell.get_view("symbol").expect("the generated symbol");
    assert!(rspice_design::symbol::generated::is_generated_symbol(
        symbol
    ));
    assert!(
        app.state
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
                reference.clone()
            ))
            .is_some_and(|bundle| bundle.root().logical_path() == "comparator.va"
                && bundle.root().content().contains("module comparator;"))
    );
    assert_eq!(app.state.model.workspace.content.active_view, reference);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Design
    );
    assert!(app.state.code.code_workspace.veriloga.receipt.is_some());
}

#[test]
fn new_cell_rejects_accented_canonical_collision_without_mutation() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("identity_test");
    library.add_cell(crate::state::Cell::new("\u{c9}tage"));
    app.state.model.library_manager.add_library(library);
    app.state.project.new_cell_library = "identity_test".to_owned();
    app.state.project.new_cell_name = "\u{e9}TAGE".to_owned();
    app.state.project.new_cell_create_schematic = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    let buffers_before = app.state.model.workspace.content.schematic_buffers.len();

    let created = create(&mut app);

    assert!(!created);
    let library = app
        .state
        .model
        .library_manager
        .get_library("identity_test")
        .expect("identity library remains");
    assert_eq!(library.cell_count(), 1);
    assert!(library.get_cell("\u{c9}tage").is_some());
    assert_eq!(
        app.state.model.workspace.content.schematic_buffers.len(),
        buffers_before
    );
    assert!(
        app.state
            .project
            .new_cell_error
            .as_deref()
            .is_some_and(|error| error.contains("clashes with an existing cell"))
    );
}

#[test]
fn new_cell_rejects_read_only_library_without_mutation() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("read_only_test");
    library.read_only = true;
    app.state.model.library_manager.add_library(library);
    app.state.project.new_cell_library = "read_only_test".to_owned();
    app.state.project.new_cell_name = "amp".to_owned();
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    let revision_before = app.state.model.workspace.content.project.revision();

    let created = create(&mut app);

    assert!(!created);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("read_only_test")
            .is_some_and(|library| library.get_cell("amp").is_none())
    );
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        revision_before
    );
    assert!(
        app.state
            .project
            .new_cell_error
            .as_deref()
            .is_some_and(|error| error.contains("read only"))
    );
}

#[test]
fn new_cell_rejects_catalog_change_after_dialog_open_without_mutation() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .library_manager
        .add_library(Library::new("stale_test"));
    app.state.project.new_cell_library = "stale_test".to_owned();
    app.state.project.new_cell_name = "amp".to_owned();
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    app.state
        .model
        .library_manager
        .add_library(Library::new("intervening_change"));
    let project_revision_before = app.state.model.workspace.content.project.revision();

    let created = create(&mut app);

    assert!(!created);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("stale_test")
            .is_some_and(|library| library.get_cell("amp").is_none())
    );
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        project_revision_before
    );
    assert_eq!(
        app.state.project.new_cell_error.as_deref(),
        Some(LIBRARY_CATALOG_STALE_MESSAGE)
    );
}

/// A cell in a library of its own typed into New Cell, with a Verilog-A
/// view: with a schematic and a testbench, which open first, or alone,
/// when the module is what opens.
fn creating(veriloga_only: bool) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .library_manager
        .add_library(Library::new("behavioral"));
    let project = &mut app.state.project;
    project.new_cell_library = "behavioral".to_owned();
    project.new_cell_name = "comparator".to_owned();
    project.new_cell_create_schematic = !veriloga_only;
    project.new_cell_create_testbench = !veriloga_only;
    project.new_cell_create_veriloga = true;
    app.state.project.new_cell_library_revision = app.state.model.library_manager.revision();
    app
}

/// Carry out what the creation handed over in the order the creation
/// once did it in place, with the new document it read before it
/// changed the catalog.
fn carry_out_in_place(app: &mut RSpiceApp, seeded: crate::state::SchematicState) {
    let Some(ProjectRequest::CellViewsCreated {
        schematics,
        mutation,
        notice,
        open,
        veriloga,
        persist_global_veriloga,
    }) = app.state.workbench.outbox.take::<ProjectRequest>()
    else {
        panic!("the creation hands the shell what it left");
    };
    assert!(
        !persist_global_veriloga,
        "the cell is not the Verilog-A library's"
    );
    let state = &mut app.state;
    for reference in schematics {
        state
            .model
            .workspace
            .insert_schematic_editor(reference.key(), seeded.clone());
    }
    state.model.publish_project_library_mutation(
        mutation
            .take()
            .expect("the reserved revision is published once"),
    );
    state.record(
        crate::diagnostics::LogSeverity::Info,
        crate::diagnostics::LogSource::Studio,
        notice,
    );
    if let Some(view) = open {
        state.open_workspace_view(view);
    }
    if let Some((view, receipt)) = veriloga.map(|seeded| *seeded)
        && state.model.workspace.content.active_view == view
    {
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        state.code.code_workspace.veriloga.receipt = Some(receipt);
    }
}

/// A cell created in the feature's context, with what it left handed to
/// the shell, ends where the creation carried out in place did: the new
/// documents, then the revision, then the creation's line in the log,
/// then the view it opens, and the Verilog-A receipt kept only when the
/// module is the view open.
#[test]
fn a_created_cell_ends_where_the_creation_carried_out_in_place_did() {
    for veriloga_only in [false, true] {
        let mut in_place = creating(veriloga_only);
        let seeded = in_place.state.new_schematic_document();
        assert!(with_cx(&mut in_place, create_new_cell));
        carry_out_in_place(&mut in_place, seeded);

        let mut handed = creating(veriloga_only);
        assert!(create(&mut handed));

        assert_eq!(
            shown(&handed),
            shown(&in_place),
            "Verilog-A only: {veriloga_only}"
        );
        assert_eq!(published(&handed), published(&in_place));
        assert_eq!(in_front(&handed), in_front(&in_place));
        assert_eq!(in_front(&handed).1, veriloga_only);
    }
}
