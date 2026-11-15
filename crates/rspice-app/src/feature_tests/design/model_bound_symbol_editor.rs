//! A model-bound symbol in the symbol editor, through a whole application:
//! created by the dialog, its pins are the model's terminals with their types
//! and sides; the editor states the model as its interface wherever it states
//! one; editing the drawing keeps the binding; and it is saved, checked,
//! reverted and undone as a drawn symbol is.

use std::path::{Path, PathBuf};

use crate::diagnostics::LogAnchor;
use crate::feature_tests::design::create_model_bound_symbol::{
    open_create_model_bound_symbol_dialog, state_with_bound_mos_model,
};
use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, Point, PortDirection, SymbolDocument,
    SymbolEditorMetadata, SymbolElectricalType, SymbolPin, SymbolPinSide,
};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app::dialogs::create_model_bound_symbol::state::PRIMARY;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame_with, themed_context};
use crate::workbench::app_state::diagnostic_store::{DiagnosticInputs, DiagnosticRoute};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::lifecycle::project_lifecycle::{
    DestinationAuthority, SaveScope, has_unsaved_changes,
};
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::state::Workspace;

fn unique_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rspice-model-bound-{label}-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4()
    ))
}

fn remove_project_artifacts(path: &Path) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(PathBuf::from(lock));
}

/// The AccessKit labels and values (a plain label's text is its value) of
/// the last of two whole-application frames.
fn labels(ctx: &egui::Context, app: &mut RSpiceApp) -> Vec<String> {
    frame_with(ctx, app, DESKTOP, Vec::new());
    frame_with(ctx, app, DESKTOP, Vec::new())
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .map(str::to_owned)
        .collect()
}

/// Press the button labelled `name` the way assistive technology does.
fn press(ctx: &egui::Context, app: &mut RSpiceApp, name: &str) {
    use egui::accesskit::{Action, ActionRequest, Role, TreeId};

    frame_with(ctx, app, DESKTOP, Vec::new());
    let output = frame_with(ctx, app, DESKTOP, Vec::new());
    let button = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .find(|(_, node)| node.role() == Role::Button && node.label() == Some(name))
        .map(|(id, _)| *id)
        .unwrap_or_else(|| panic!("the page offers {name}"));
    frame_with(
        ctx,
        app,
        DESKTOP,
        vec![egui::Event::AccessKitActionRequest(ActionRequest {
            action: Action::Click,
            target_tree: TreeId::ROOT,
            target_node: button,
            data: None,
        })],
    );
    frame_with(ctx, app, DESKTOP, Vec::new());
}

/// The bound MOS model in a project a file can hold: the project keeps its
/// own root library, the symbol goes to `analog_blocks`, and the model's
/// library is an external one, whose source the project names by its path.
fn saveable_bound_mos_model() -> AppState {
    let bound = state_with_bound_mos_model();
    let mut state = AppState::default();
    state.open_project_session();
    state
        .model
        .library_manager
        .add_library(crate::state::Library::new("analog_blocks"));
    state.model.library_manager.selected_library = Some("analog_blocks".to_owned());
    state.model.model_library_manager = bound.model.model_library_manager;
    state.models.selected_model = bound.models.selected_model;
    let library = state
        .model
        .model_library_manager
        .get_library_mut("vendor_cmos")
        .expect("the model's library");
    library.source_authority = crate::state::model_library::ModelSourceAuthority::External;
    library.root_path = Some(PathBuf::from(r"C:\models"));
    state
}

struct Fixture {
    ctx: egui::Context,
    app: RSpiceApp,
    path: PathBuf,
    symbol: CellViewRef,
}

impl Fixture {
    /// The `nmos_core` model's symbol, created by the dialog's primary in a
    /// project saved to a file, and open in the symbol editor.
    fn new(label: &str) -> Self {
        let ctx = themed_context();
        let mut app = RSpiceApp::test_instance();
        app.state = saveable_bound_mos_model();
        app.state.workbench.activate(Workspace::Design);
        let path = unique_path(label);
        app.state
            .save_project_scope_to(SaveScope::AllDocuments, &path, DestinationAuthority::UserSelected)
            .expect("the project saves");
        open_create_model_bound_symbol_dialog(&mut app.state);
        app.state
            .design
            .create_model_bound_symbol
            .pin_contract_reviewed = true;
        press(&ctx, &mut app, PRIMARY);
        let symbol = CellViewRef::new("analog_blocks", "nmos_core", "symbol");
        assert_eq!(
            app.state.model.workspace.content.active_view,
            symbol,
            "creating the symbol opens it in the symbol editor: {:?}",
            app.state.design.create_model_bound_symbol.validation_error
        );
        app.state
            .save_project_scope_to(SaveScope::AllDocuments, &path, DestinationAuthority::UserSelected)
            .expect("the project saves");
        Self {
            ctx,
            app,
            path,
            symbol,
        }
    }

    fn document(&self) -> SymbolDocument {
        self.app
            .state
            .model
            .load_symbol_document(&self.symbol)
            .expect("the symbol loads")
    }

    fn metadata(&self) -> SymbolEditorMetadata {
        self.app
            .state
            .model
            .load_symbol_editor_metadata(&self.symbol, &self.document())
            .expect("the symbol's metadata loads")
    }

    fn binding(&self) -> Option<ModelBoundSymbolDefinition> {
        self.app.state.model.symbol_model_binding(&self.symbol)
    }

    /// Replace the symbol in front with what `edit` makes of it, as one
    /// named edit.
    fn edit(&mut self, step: &str, edit: impl FnOnce(&mut SymbolDocument)) {
        let before = self.document();
        let metadata = self.metadata();
        let mut edited = before.clone();
        edit(&mut edited);
        crate::workbench::app::record_symbol_edit(
            &mut self.app.state.design.symbol,
            &self.app.state.model,
            &before,
            step,
        );
        self.app
            .state
            .store_active_symbol_editor_bundle(&edited, &metadata)
            .expect("the edit stores");
    }

    fn pin(&self, name: &str) -> Option<SymbolPin> {
        self.document().pin(name).cloned()
    }

    /// The symbol as the project file on disk holds it, with its binding.
    fn saved(&self) -> (SymbolDocument, Option<ModelBoundSymbolDefinition>) {
        let persisted = crate::io::load_project_file(&self.path).expect("the project reloads");
        let view = persisted
            .file
            .libraries
            .get_library("analog_blocks")
            .and_then(|library| library.get_cell("nmos_core"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("the file holds the symbol")
            .clone();
        (
            SymbolDocument::load_from_view(&view).expect("the saved symbol loads"),
            crate::state::load_model_bound_symbol(&view).expect("the saved binding loads"),
        )
    }

    fn console_lines(&self) -> Vec<String> {
        self.app
            .state
            .model
            .log_buffer
            .entries()
            .map(|entry| entry.message.clone())
            .collect()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        remove_project_artifacts(&self.path);
    }
}

/// The created symbol carries the model's terminals as its pins, each with
/// the electrical type, direction and side the model's contract states, and
/// its own pin check finds nothing to say about them.
#[test]
fn a_created_model_bound_symbol_carries_the_models_typed_terminals_on_their_sides() {
    let mut fixture = Fixture::new("created");
    let binding = fixture.binding().expect("the symbol is bound to its model");
    let document = fixture.document();
    let mut declared = binding.pins.clone();
    declared.sort_by_key(|pin| pin.order);
    assert_eq!(
        document
            .pins
            .iter()
            .map(|pin| (
                pin.name.clone(),
                pin.electrical_type(),
                pin.direction,
                pin.side
            ))
            .collect::<Vec<_>>(),
        declared
            .iter()
            .map(|pin| (
                pin.name.clone(),
                pin.electrical_type,
                pin.direction,
                pin.side
            ))
            .collect::<Vec<_>>()
    );
    assert!(
        document.pins.iter().all(|pin| pin.position.is_some()),
        "every terminal is placed"
    );

    fixture.app.state.run_active_symbol_pin_checks();
    assert_eq!(fixture.app.state.active_symbol_pin_finding_count(), Some(0));
    assert!(
        fixture
            .console_lines()
            .iter()
            .any(|line| line == "Symbol pin check: pins match the model."),
        "{:?}",
        fixture.console_lines()
    );
}

/// The model is the interface wherever the editor states one: a terminal the
/// drawing lost is listed with no pin, a pin the model has no terminal for
/// is said not to be in the model, and Update pins from model puts the lost
/// terminal back with the type and side the model states for it.
#[test]
fn the_symbol_editor_answers_a_bound_symbol_to_its_model() {
    let mut fixture = Fixture::new("interface");
    let bulk = fixture.pin("B").expect("the bulk terminal is drawn");
    fixture.edit("Delete pin", |document| {
        document.pins.retain(|pin| pin.name != "B");
        document.pins.push(SymbolPin::new(
            "X",
            PortDirection::InOut,
            Some(Point::new(-60, 40)),
        ));
    });

    let ctx = fixture.ctx.clone();
    let labels = labels(&ctx, &mut fixture.app);
    assert!(
        labels.iter().any(|label| label == "B, no pin, Warning"),
        "the lost terminal is listed: {labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| label == "4. X, not in model, Warning"),
        "the stray pin is named against the model: {labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| label == "1 model pin has no symbol pin"),
        "the canvas names the model: {labels:?}"
    );

    press(&ctx, &mut fixture.app, "Update pins from model");

    let restored = fixture.pin("B").expect("Update pins put the terminal back");
    assert!(restored.position.is_some(), "and placed it");
    assert_eq!(restored.electrical_type(), SymbolElectricalType::Power);
    assert_eq!(restored.electrical_type(), bulk.electrical_type());
    assert_eq!(restored.direction, bulk.direction);
    assert_eq!(restored.side, SymbolPinSide::Bottom, "on the model's side");
    assert!(fixture.binding().is_some(), "the binding stays");
}

/// Editing the drawing never loses the binding: through an edit, its undo
/// and redo, a save and the file it writes, and Revert to saved. The save
/// that wrote the edit records a revision, as a drawn symbol's does.
#[test]
fn editing_a_bound_symbol_keeps_its_binding_through_undo_save_and_revert() {
    let mut fixture = Fixture::new("binding");
    let created = fixture.document();
    let gate = fixture.pin("G").and_then(|pin| pin.position).expect("G");
    let first_revision = fixture.metadata().revision;

    fixture.edit("Move pin", |document| {
        let pin = document
            .pins
            .iter_mut()
            .find(|pin| pin.name == "G")
            .expect("G");
        pin.position = Some(gate + Point::new(0, -20));
    });
    assert!(fixture.binding().is_some(), "an edit keeps the binding");
    assert!(has_unsaved_changes(&fixture.app.state));

    let undo = fixture.app.command_availability(Command::Undo.key());
    assert!(undo.is_available(), "{undo:?}");
    fixture.app.execute_command_key(Command::Undo.key());
    assert_eq!(fixture.document(), created, "undo puts the drawing back");
    assert!(fixture.binding().is_some(), "undo keeps the binding");
    fixture.app.execute_command_key(Command::Redo.key());
    assert_eq!(
        fixture.pin("G").and_then(|pin| pin.position),
        Some(gate + Point::new(0, -20)),
        "redo moves it again"
    );

    fixture.app.execute_command_key(ProjectCommand::Save.key());
    assert!(!has_unsaved_changes(&fixture.app.state));
    let (saved, binding) = fixture.saved();
    assert_eq!(
        saved.pin("G").and_then(|pin| pin.position),
        Some(gate + Point::new(0, -20))
    );
    assert_eq!(binding, fixture.binding(), "the file keeps the binding");
    assert!(binding.is_some());
    assert_eq!(
        fixture.metadata().revision,
        first_revision + 1,
        "the save that wrote the edit recorded it"
    );

    let moved = fixture.document();
    fixture.edit("Move pin", |document| {
        let pin = document
            .pins
            .iter_mut()
            .find(|pin| pin.name == "G")
            .expect("G");
        pin.position = Some(gate + Point::new(0, 20));
    });
    let token = crate::workbench::lifecycle::project_lifecycle::prepare_revert_active_document(
        &fixture.app.state,
    )
    .expect("the revert is reviewed");
    assert!(fixture.app.state.confirm_revert_active_document(&token));
    assert_eq!(fixture.document(), moved, "the symbol is as last saved");
    assert!(fixture.binding().is_some(), "revert keeps the binding");
}

/// A new parameter form saved from the symbol inspector's form designer is
/// the next revision of the binding, and the drawing its author made since
/// the symbol was created is kept as it was drawn.
#[test]
fn saving_a_new_parameter_form_keeps_the_bound_symbols_drawing() {
    let mut fixture = Fixture::new("form");
    let gate = fixture.pin("G").and_then(|pin| pin.position).expect("G");
    fixture.edit("Move pin", |document| {
        let pin = document
            .pins
            .iter_mut()
            .find(|pin| pin.name == "G")
            .expect("G");
        pin.position = Some(gate + Point::new(0, -20));
    });
    let drawn = fixture.document();
    let form = fixture
        .binding()
        .expect("the symbol is bound to its model")
        .parameter_form
        .revision;

    let ctx = fixture.ctx.clone();
    press(&ctx, &mut fixture.app, "Edit parameter form...");
    press(&ctx, &mut fixture.app, "Open form designer");
    press(&ctx, &mut fixture.app, "Save form revision");

    assert!(
        !fixture.app.state.design.symbol_parameter_form.open,
        "the form saved: {:?}",
        fixture
            .app
            .state
            .design
            .symbol_parameter_form
            .validation_error
    );
    assert_eq!(
        fixture
            .binding()
            .map(|binding| binding.parameter_form.revision),
        Some(form + 1),
        "the binding carries the new form"
    );
    assert_eq!(
        fixture.document(),
        drawn,
        "the drawing is as its author left it"
    );
}

/// Check and save checks a bound symbol against its model and saves it: a
/// pin pushed off the grid is a finding on the Problems page whose route
/// lands on that pin in the symbol editor.
#[test]
fn check_and_save_routes_a_bound_symbols_finding_to_its_pin() {
    let mut fixture = Fixture::new("check");
    let gate = fixture.pin("G").and_then(|pin| pin.position).expect("G");
    fixture.edit("Move pin", |document| {
        let pin = document
            .pins
            .iter_mut()
            .find(|pin| pin.name == "G")
            .expect("G");
        pin.position = Some(gate + Point::new(3, 0));
    });

    fixture
        .app
        .execute_command_key(ProjectCommand::CheckAndSave.key());

    assert!(
        !has_unsaved_changes(&fixture.app.state),
        "the finding did not stop the save"
    );
    let subject = fixture.symbol.clone();
    let route = fixture
        .app
        .state
        .model
        .diagnostics
        .batches()
        .find(|batch| {
            batch.inputs
                == DiagnosticInputs::SymbolPins {
                    subject: subject.clone(),
                }
        })
        .and_then(|batch| {
            batch
                .records
                .iter()
                .find(|record| record.canonical.code.as_ref() == "SYMBOL-PIN-OFF-GRID")
        })
        .and_then(|record| record.route.clone())
        .expect("the off-grid pin is a routed finding");
    let Some(DiagnosticRoute::Anchor(anchor @ LogAnchor::Symbol { .. })) = Some(route) else {
        panic!("the finding routes to a symbol pin");
    };

    fixture.app.state.open_workspace_view(CellViewRef::new(
        "analog_blocks",
        "nmos_core",
        "parameter_form",
    ));
    fixture.app.state.jump_to_log_anchor(anchor);

    assert_eq!(
        fixture.app.state.model.workspace.content.active_view,
        subject
    );
    assert_eq!(
        fixture
            .app
            .state
            .design
            .symbol
            .editor
            .effective_selection()
            .pins,
        std::collections::BTreeSet::from(["G".to_owned()])
    );
}

/// A placed instance of the model's cell, bound to its netlist view, edits
/// the cell's symbol in place.
#[test]
fn a_placed_bound_instance_edits_its_symbol_in_place() {
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    app.state.workbench.activate(Workspace::Design);
    open_create_model_bound_symbol_dialog(&mut app.state);
    let draft = &mut app.state.design.create_model_bound_symbol;
    draft.pin_contract_reviewed = true;
    draft.simulation_test_fixture = true;
    let ctx = themed_context();
    press(&ctx, &mut app, PRIMARY);

    let fixture = CellViewRef::new("analog_blocks", "nmos_core", "testbench");
    app.state.open_workspace_view(fixture);
    let dut = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| {
            component
                .library_cell
                .as_ref()
                .is_some_and(|binding| binding.cell == "nmos_core")
        })
        .map(|component| component.id)
        .expect("the fixture places the device");
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(dut);
    assert!(
        app.command_availability(DesignCommand::EditSymbolInPlace.key())
            .is_available()
    );

    app.execute_command_key(DesignCommand::EditSymbolInPlace.key());

    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("analog_blocks", "nmos_core", "symbol")
    );
}
