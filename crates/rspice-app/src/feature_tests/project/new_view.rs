//! Creating a cell view through the whole application: New view drawn in the
//! feature's context lent from RSpiceApp, the view it creates there, with
//! what the creation raised answered as the dialog step answers it, and the
//! first commit of a generated symbol through the application's editor.

use egui::Context;

use crate::state::{
    Cell, CellViewRef, ComponentType, Library, NetNamingPolicy, Point, PortDirection, PortSpec,
    ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner, SchematicState, SymbolDocument,
    SymbolPin, View, ViewType,
};
use crate::workbench::app::dialogs::library::LIBRARY_CATALOG_STALE_MESSAGE;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::library_dialogs::new_view::{
    CREATABLE_VIEW_TYPES, NEW_VIEW_NAME_MISSING, NameLine, create_new_view, new_view_name_line,
    seeded_symbol_view, view_type_label,
};
use crate::workbench::{ChoicePreference, RSpiceApp};

use super::{in_front, published, shown, with_cx};

/// Create the view the dialog names in the feature's context, with
/// what it raised answered as the dialog step answers it, and whether
/// it was created.
fn create(app: &mut RSpiceApp) -> bool {
    let created = with_cx(app, create_new_view);
    app.after_contribution();
    created
}

fn schematic_with_ports(names: &[&str]) -> SchematicState {
    let mut schematic = SchematicState::default();
    for (index, name) in names.iter().enumerate() {
        let id = schematic.add_component(ComponentType::Port, Point::new(index as i32 * 20, 0));
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the placed interface port")
            .value = (*name).to_owned();
    }
    schematic
}

/// New view is a command form: a 420 pt card with Library and Cell on
/// its label/value grid, not a 760 pt transaction with its two facts
/// pushed to the far edge; and an empty name says what Create waits for.
#[test]
fn new_view_is_a_command_form_that_says_what_create_waits_for() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("form_test");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .project
        .open_new_view_dialog(&app.state.model, "form_test", "amp", None)
        .expect("writable target");
    app.state.project.new_view_name.clear();

    // Access updates are incremental: keep the latest of every node.
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                with_cx(&mut app, |cx| {
                    ProjectFeature::process_new_view_dialog(cx, ctx)
                })
            },
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    // The modal's own node is the whole screen. The surface is centred
    // on it, and Create ends 12 pt in from the surface's right edge.
    let create = nodes
        .values()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some("Create"))
        .and_then(egui::accesskit::Node::bounds)
        .expect("Create has bounds");
    let width = 2.0 * (create.x1 + 12.0) - 1440.0;
    assert!(width <= 524.0, "New view is {width} pt wide");
    // The types are one row of equal segments: none wraps to a line of
    // its own, and no two are spaced apart unevenly.
    // Each reads in sentence case; the view keeps its lowercase name.
    assert_eq!(
        CREATABLE_VIEW_TYPES.map(view_type_label),
        ["Schematic", "Testbench", "Symbol", "Layout", "Verilog-A"]
    );
    let types = CREATABLE_VIEW_TYPES.map(|view_type| {
        nodes
            .values()
            .find(|node| node.label() == Some(view_type_label(view_type)))
            .and_then(egui::accesskit::Node::bounds)
            .unwrap_or_else(|| panic!("{} is published", view_type_label(view_type)))
    });
    for pair in types.windows(2) {
        assert!(
            (pair[0].y0 - pair[1].y0).abs() < 0.5,
            "{types:?} are not one row"
        );
        assert!(
            ((pair[0].x1 - pair[0].x0) - (pair[1].x1 - pair[1].x0)).abs() < 0.5,
            "{types:?} are not equal segments"
        );
    }
    assert!(nodes.values().any(|node| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("View name")
    }));
    assert!(
        nodes.values().any(|node| {
            node.label() == Some(NEW_VIEW_NAME_MISSING)
                || node.value() == Some(NEW_VIEW_NAME_MISSING)
        }),
        "Create does not say what it waits for"
    );
    // Library, Cell, View name and Type keep one pitch: no blank line
    // stands between the name and the types. Measured at row bottoms, as
    // the kit lays the first fact row out shorter at its top.
    let row = |label: &str| {
        nodes
            .values()
            .filter(|node| node.label() == Some(label))
            .filter_map(egui::accesskit::Node::bounds)
            .map(|bounds| bounds.y1)
            .next()
            .unwrap_or_else(|| panic!("{label} is published"))
    };
    // The kit paints row labels; the Type row stands where its segments do.
    let [library, cell, name] = ["Library", "Cell", "View name"].map(row);
    let bottoms = [library, cell, name, types[0].y1];
    for pair in bottoms.windows(2) {
        assert!(
            ((pair[1] - pair[0]) - (bottoms[1] - bottoms[0])).abs() < 1.0,
            "rows are not evenly pitched: {bottoms:?}"
        );
    }
    assert!(matches!(
        new_view_name_line("bad name"),
        Some(NameLine::Refused(_))
    ));
    assert!(new_view_name_line("symbol").is_none());
}

#[test]
fn creating_schematic_view_freezes_current_defaults_into_its_buffer() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("view_policy_test");
    library.add_cell(Cell::new("filter"));
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .preferences
        .set_choice(ChoicePreference::NetNamingPolicy, 1)
        .unwrap();
    app.state.project.new_view_library = "view_policy_test".to_owned();
    app.state.project.new_view_cell = "filter".to_owned();
    app.state.project.new_view_name = "schematic".to_owned();
    app.state.project.new_view_type = crate::state::ViewType::Schematic;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
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
        Some(crate::state::ProjectLibraryMutation::CreateView {
            library,
            cell,
            view,
        }) if library == "view_policy_test" && cell == "filter" && view == "schematic"
    ));
    let key = crate::state::CellViewRef::new("view_policy_test", "filter", "schematic").key();
    let document = app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&key)
        .expect("new schematic view owns an explicit document buffer");
    assert_eq!(
        document.document().document_policy.net_naming,
        NetNamingPolicy::SpiceCompatibleRelaxed
    );
}

#[test]
fn new_symbol_view_reflects_the_cells_current_schematic_interface() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("interface_test");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state.model.workspace.insert_schematic_editor(
        CellViewRef::new("interface_test", "amp", "schematic").key(),
        schematic_with_ports(&["IN", "OUT"]),
    );
    app.state.project.new_view_library = "interface_test".to_owned();
    app.state.project.new_view_cell = "amp".to_owned();
    app.state.project.new_view_name = "symbol".to_owned();
    app.state.project.new_view_type = ViewType::Symbol;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

    let created = create(&mut app);

    assert!(created, "{:?}", app.state.project.new_view_error);
    let view = app
        .state
        .model
        .library_manager
        .get_library("interface_test")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("the created symbol view");
    assert_eq!(
        view.metadata.get("generated").map(String::as_str),
        Some("ports")
    );
    assert_eq!(
        view.metadata.get("ports").map(String::as_str),
        Some("IN:inout OUT:inout")
    );
    assert_eq!(
        SymbolDocument::load_from_view(view)
            .expect("the seeded symbol document parses")
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        ["IN", "OUT"],
        "a symbol created against an existing schematic starts on its ports"
    );
}

#[test]
fn first_symbol_editor_commit_takes_the_view_off_the_generated_marker() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("marker_test");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    cell.add_view(
        seeded_symbol_view(
            "symbol",
            &[PortSpec {
                name: "IN".to_owned(),
                direction: PortDirection::In,
            }],
        )
        .expect("the generated symbol seeds"),
    );
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("marker_test", "amp", "symbol"));
    let document = app
        .state
        .model
        .load_active_symbol_document()
        .expect("the generated symbol loads into the editor");

    app.state
        .store_active_symbol_document(&document)
        .expect("the editor commits the symbol");

    let view = app
        .state
        .model
        .library_manager
        .get_library("marker_test")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("the committed symbol view");
    assert!(!view.metadata.contains_key("generated"));
    assert!(!view.metadata.contains_key("ports"));
}

#[test]
fn unsupported_view_type_is_rejected_before_project_mutation() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("view_policy_test");
    library.add_cell(Cell::new("filter"));
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "view_policy_test".to_owned();
    app.state.project.new_view_cell = "filter".to_owned();
    app.state.project.new_view_name = "doc".to_owned();
    app.state.project.new_view_type = crate::state::ViewType::Document;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

    let created = create(&mut app);

    assert!(!created);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("view_policy_test")
            .and_then(|library| library.get_cell("filter"))
            .is_some_and(|cell| cell.get_view("doc").is_none())
    );
    assert!(
        app.state
            .project
            .new_view_error
            .as_deref()
            .is_some_and(|error| error.contains("production editor"))
    );
}

#[test]
fn creating_layout_view_atomically_retains_exact_pdk_document_and_opens_it() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    let mut library = Library::new("layout_designs");
    library.add_cell(Cell::new("precision_amp"));
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "layout_designs".to_owned();
    app.state.project.new_view_cell = "precision_amp".to_owned();
    app.state.project.new_view_name = "layout".to_owned();
    app.state.project.new_view_type = ViewType::Layout;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
    let project_revision_before = app.state.model.workspace.content.project.revision().get();

    let created = create(&mut app);

    assert!(created, "{:?}", app.state.project.new_view_error);
    assert_eq!(
        app.state.model.workspace.content.project.revision().get(),
        project_revision_before + 1
    );
    let reference = CellViewRef::new("layout_designs", "precision_amp", "layout");
    let view = app
        .state
        .model
        .library_manager
        .get_library("layout_designs")
        .and_then(|library| library.get_cell("precision_amp"))
        .and_then(|cell| cell.get_view("layout"))
        .expect("Layout view commits");
    assert_eq!(view.view_type, ViewType::Layout);
    let document = app
        .state
        .model
        .workspace
        .content
        .physical_layout_document(&reference)
        .expect("authoritative physical-layout document commits atomically");
    assert_eq!(document.owner(), &reference);
    assert_eq!(document.technology().package_id(), "demo180");
    assert_eq!(document.technology().stack_id(), "1P2M");
    assert_eq!(app.state.model.workspace.content.active_view, reference);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Design
    );
    crate::workbench::lifecycle::project_lifecycle::snapshot(&app.state)
        .expect("created Layout view and document validate as one project transaction");
}

#[test]
fn creating_layout_view_without_exact_project_pdk_is_transactional() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("layout_designs");
    library.add_cell(Cell::new("precision_amp"));
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "layout_designs".to_owned();
    app.state.project.new_view_cell = "precision_amp".to_owned();
    app.state.project.new_view_name = "layout".to_owned();
    app.state.project.new_view_type = ViewType::Layout;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
    let project_revision_before = app.state.model.workspace.content.project.revision();

    let created = create(&mut app);

    assert!(!created);
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        project_revision_before
    );
    let reference = CellViewRef::new("layout_designs", "precision_amp", "layout");
    assert!(
        app.state
            .model
            .library_manager
            .get_library("layout_designs")
            .and_then(|library| library.get_cell("precision_amp"))
            .is_some_and(|cell| cell.get_view("layout").is_none())
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .physical_layout_document(&reference)
            .is_none()
    );
    assert!(
        app.state
            .project
            .new_view_error
            .as_deref()
            .is_some_and(|error| error.contains("validated PDK"))
    );
}

#[test]
fn new_view_rejects_accented_canonical_collision_without_mutation() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("identity_test");
    let mut cell = Cell::new("filter");
    cell.add_view(View::new("Mod\u{e8}le", ViewType::Symbol));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "identity_test".to_owned();
    app.state.project.new_view_cell = "filter".to_owned();
    app.state.project.new_view_name = "MOD\u{c8}LE".to_owned();
    app.state.project.new_view_type = ViewType::Schematic;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
    let buffers_before = app.state.model.workspace.content.schematic_buffers.len();

    let created = create(&mut app);

    assert!(!created);
    let cell = app
        .state
        .model
        .library_manager
        .get_library("identity_test")
        .and_then(|library| library.get_cell("filter"))
        .expect("identity cell remains");
    assert_eq!(cell.view_count(), 1);
    assert!(cell.get_view("Mod\u{e8}le").is_some());
    assert_eq!(
        app.state.model.workspace.content.schematic_buffers.len(),
        buffers_before
    );
    assert!(
        app.state
            .project
            .new_view_error
            .as_deref()
            .is_some_and(|error| error.contains("clashes with an existing view"))
    );
}

#[test]
fn creating_veriloga_view_atomically_retains_contract_and_opens_exact_editor() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("behavioral_models");
    let mut cell = Cell::new("precision_amp");
    let mut symbol = View::new("symbol", ViewType::Symbol);
    let symbol_document = SymbolDocument {
        pins: vec![
            SymbolPin::new("inp", PortDirection::In, None),
            SymbolPin::new("inn", PortDirection::In, None),
            SymbolPin::new("out", PortDirection::Out, None),
            SymbolPin::new("vss", PortDirection::Supply, None),
        ],
        ..SymbolDocument::default()
    };
    symbol_document.store_in_view(&mut symbol).unwrap();
    cell.add_view(symbol);
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "behavioral_models".to_owned();
    app.state.project.new_view_cell = "precision_amp".to_owned();
    app.state.project.new_view_name = "veriloga".to_owned();
    app.state.project.new_view_type = ViewType::VerilogA;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
    let project_revision_before = app.state.model.workspace.content.project.revision().get();

    let created = create(&mut app);

    assert!(created, "{:?}", app.state.project.new_view_error);
    assert_eq!(
        app.state.model.workspace.content.project.revision().get(),
        project_revision_before + 1
    );
    let reference =
        crate::state::CellViewRef::new("behavioral_models", "precision_amp", "veriloga");
    let owner = ProjectSourceOwner::cell_view(reference.clone());
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("created view owns a sealed source bundle");
    assert_eq!(bundle.language(), ProjectSourceLanguage::VerilogA);
    assert_eq!(bundle.root().logical_path(), "precision_amp.va");
    assert!(
        bundle
            .root()
            .content()
            .contains("module precision_amp(inp, inn, out, vss);")
    );
    assert!(bundle.root().content().contains("input inp, inn;"));
    assert!(bundle.root().content().contains("output out;"));
    assert!(bundle.root().content().contains("inout vss;"));
    let view = app
        .state
        .model
        .library_manager
        .get_library("behavioral_models")
        .and_then(|library| library.get_cell("precision_amp"))
        .and_then(|cell| cell.get_view("veriloga"))
        .unwrap();
    assert_eq!(
        view.metadata.get("veriloga.module").map(String::as_str),
        Some("precision_amp")
    );
    assert_eq!(
        view.metadata.get("veriloga.ports").map(String::as_str),
        Some(r#"["inp","inn","out","vss"]"#)
    );
    assert_eq!(app.state.model.workspace.content.active_view, reference);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Design,
        "a new Verilog-A view opens as a Design document"
    );
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Design,
        "reached through navigation, not by writing the workspace"
    );
    let selected =
        crate::workbench::documents::code_workspace::selected_veriloga_source(&app).unwrap();
    assert_eq!(selected.bundle().id(), bundle.id());
    assert_eq!(selected.selected_module(), Some("precision_amp"));
    assert!(selected.bundle().validation_is_current());
    assert!(app.state.code.code_workspace.veriloga.receipt.is_some());
}

/// A cell with no schematic ports and no symbol pins has no interface yet,
/// and its module declares none. It used to be handed a `p`/`n` pair that
/// nothing had asked for, which then became the cell's interface.
#[test]
fn a_veriloga_view_of_a_cell_with_no_interface_starts_without_ports() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("behavioral_models");
    library.add_cell(Cell::new("blank"));
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "behavioral_models".to_owned();
    app.state.project.new_view_cell = "blank".to_owned();
    app.state.project.new_view_name = "veriloga".to_owned();
    app.state.project.new_view_type = ViewType::VerilogA;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

    let created = create(&mut app);

    assert!(created, "{:?}", app.state.project.new_view_error);
    let owner =
        ProjectSourceOwner::cell_view(CellViewRef::new("behavioral_models", "blank", "veriloga"));
    let source = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("the view owns its source")
        .root()
        .content()
        .to_owned();
    assert!(source.contains("module blank;\nendmodule"), "{source}");
    assert!(!source.contains("electrical"), "{source}");
    let view = app
        .state
        .model
        .library_manager
        .get_library("behavioral_models")
        .and_then(|library| library.get_cell("blank"))
        .and_then(|cell| cell.get_view("veriloga"))
        .expect("the created view");
    assert_eq!(
        view.metadata.get("veriloga.ports").map(String::as_str),
        Some("[]")
    );
}

#[test]
fn duplicate_cell_view_source_owner_rolls_back_view_creation() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("behavioral_models");
    library.add_cell(Cell::new("gain_stage"));
    app.state.model.library_manager.add_library(library);
    let reference = crate::state::CellViewRef::new("behavioral_models", "gain_stage", "veriloga");
    app.state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::cell_view(reference.clone()),
                ProjectSourceLanguage::VerilogA,
                "existing.va",
                "module existing; endmodule\n",
                [],
                [],
            )
            .unwrap(),
        )
        .unwrap();
    let sources_before = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .iter_bundles()
        .count();
    app.state.project.new_view_library = "behavioral_models".to_owned();
    app.state.project.new_view_cell = "gain_stage".to_owned();
    app.state.project.new_view_name = "veriloga".to_owned();
    app.state.project.new_view_type = ViewType::VerilogA;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();

    let created = create(&mut app);

    assert!(!created);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .project_sources
            .iter_bundles()
            .count(),
        sources_before
    );
    assert!(
        app.state
            .model
            .library_manager
            .get_library("behavioral_models")
            .and_then(|library| library.get_cell("gain_stage"))
            .is_some_and(|cell| cell.get_view("veriloga").is_none())
    );
    assert!(app.state.project.new_view_error.is_some());
}

#[test]
fn new_view_rejects_catalog_change_after_dialog_open_without_mutation() {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("stale_test");
    library.add_cell(Cell::new("filter"));
    app.state.model.library_manager.add_library(library);
    app.state.project.new_view_library = "stale_test".to_owned();
    app.state.project.new_view_cell = "filter".to_owned();
    app.state.project.new_view_name = "schematic".to_owned();
    app.state.project.new_view_type = ViewType::Schematic;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
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
            .and_then(|library| library.get_cell("filter"))
            .is_some_and(|cell| cell.get_view("schematic").is_none())
    );
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        project_revision_before
    );
    assert_eq!(
        app.state.project.new_view_error.as_deref(),
        Some(LIBRARY_CATALOG_STALE_MESSAGE)
    );
}

/// A cell of its own in an editable library, with New View open on it
/// to create a `view_type` view named `name`, under the project's
/// PDK when that is a Layout view.
fn creating(view_type: ViewType, name: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    if view_type == ViewType::Layout {
        app.state.provision_test_project_technology_contract();
    }
    let mut library = Library::new("designs");
    library.add_cell(Cell::new("amp"));
    app.state.model.library_manager.add_library(library);
    let project = &mut app.state.project;
    project.new_view_library = "designs".to_owned();
    project.new_view_cell = "amp".to_owned();
    project.new_view_name = name.to_owned();
    project.new_view_type = view_type;
    app.state.project.new_view_library_revision = app.state.model.library_manager.revision();
    app
}

/// Carry out what the creation handed over in the order the creation
/// once did it in place, with the new document it read before it
/// changed the catalog.
fn carry_out_in_place(app: &mut RSpiceApp, seeded: SchematicState) {
    let request = app.state.workbench.outbox.take::<ProjectRequest>();
    let state = &mut app.state;
    match request {
        Some(ProjectRequest::CellViewsCreated {
            schematics,
            mutation,
            notice,
            open,
            veriloga,
            persist_global_veriloga,
        }) => {
            assert!(veriloga.is_none(), "New View seeds no Verilog-A view here");
            assert!(
                !persist_global_veriloga,
                "the view is not the Verilog-A library's"
            );
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
        }
        Some(ProjectRequest::ViewOpened {
            mutation,
            view,
            veriloga,
            notice,
            persist_global_veriloga,
        }) => {
            assert!(
                !persist_global_veriloga,
                "the view is not the Verilog-A library's"
            );
            state.model.publish_project_library_mutation(
                mutation
                    .take()
                    .expect("the reserved revision is published once"),
            );
            state.open_workspace_view(view);
            if let Some(receipt) = veriloga {
                state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
                state.code.code_workspace.veriloga.receipt = Some(*receipt);
            }
            state.record(
                crate::diagnostics::LogSeverity::Info,
                crate::diagnostics::LogSource::Studio,
                notice,
            );
        }
        _ => panic!("the creation hands the shell what it left"),
    }
}

/// A view created in the feature's context, with what it left handed to
/// the shell, ends where the creation carried out in place did. A view
/// a schematic or symbol editor edits: the new document, the revision,
/// the creation's line in the log, then the view opened. A Layout or a
/// Verilog-A view: the revision, the view opened, the Verilog-A receipt
/// with Design in front, then the creation's line in the log.
#[test]
fn a_created_view_ends_where_the_creation_carried_out_in_place_did() {
    for (view_type, name) in [
        (ViewType::Schematic, "schematic"),
        (ViewType::Symbol, "symbol"),
        (ViewType::Layout, "layout"),
        (ViewType::VerilogA, "veriloga"),
    ] {
        let mut in_place = creating(view_type, name);
        let seeded = in_place.state.new_schematic_document();
        assert!(
            with_cx(&mut in_place, create_new_view),
            "{view_type:?}: {:?}",
            in_place.state.project.new_view_error
        );
        carry_out_in_place(&mut in_place, seeded);

        let mut handed = creating(view_type, name);
        assert!(create(&mut handed), "{view_type:?}");

        assert_eq!(shown(&handed), shown(&in_place), "{view_type:?}");
        assert_eq!(published(&handed), published(&in_place), "{view_type:?}");
        assert_eq!(in_front(&handed), in_front(&in_place), "{view_type:?}");
        assert_eq!(in_front(&handed).1, view_type == ViewType::VerilogA);
    }
}
