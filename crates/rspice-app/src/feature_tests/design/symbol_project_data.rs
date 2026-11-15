//! A symbol is project data: saved, reviewed, reverted and checked with the
//! project, as a schematic is.
//!
//! Each case runs through a whole application on a project saved to a file,
//! because what is claimed is what the file holds and what the project's own
//! commands do with the symbol in front.

use std::path::{Path, PathBuf};

use crate::state::{
    CellViewRef, ComponentType, Point, PortSpec, SchematicState, SymbolDocument,
    SymbolEditorMetadata, SymbolInterface, View, ViewType,
};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::lifecycle::project_lifecycle::{
    ProjectDocumentId, dirty_documents, has_unsaved_changes,
};
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::state::Workspace;
use rspice_schematic_editor::symbol_editor::surface::canvas_check_lines;

fn unique_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rspice-symbol-{label}-{}-{}.rspiceproj",
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

/// A schematic declaring `ports` by name, one port component each.
fn declaring_schematic(ports: &[&str]) -> SchematicState {
    let mut schematic = SchematicState::default();
    for (index, name) in ports.iter().enumerate() {
        let port = schematic.add_component(ComponentType::Port, Point::new(0, index as i32 * 20));
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == port)
            .expect("the port was placed")
            .value = (*name).to_owned();
    }
    schematic
}

struct Fixture {
    app: RSpiceApp,
    path: PathBuf,
    symbol: CellViewRef,
}

impl Fixture {
    /// The cell `amp` in the project library with a symbol drawn from
    /// `pins`, and, when `schematic` names its ports, a schematic declaring
    /// them. The project is saved to a file and the symbol opened in front.
    fn new(label: &str, schematic: Option<&[&str]>, pins: &[&str]) -> Self {
        let mut app = RSpiceApp::test_instance();
        let symbol = CellViewRef::new("user", "amp", "symbol");
        let mut cell = crate::state::Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        let ports: Vec<PortSpec> = pins
            .iter()
            .map(|name| PortSpec {
                name: (*name).to_owned(),
                direction: crate::state::PortDirection::InOut,
            })
            .collect();
        SymbolDocument::generated_from_ports(&ports)
            .store_in_view(&mut symbol_view)
            .expect("the symbol stores");
        cell.add_view(symbol_view);
        if schematic.is_some() {
            cell.add_view(View::new("schematic", ViewType::Schematic));
        }
        app.state
            .model
            .library_manager
            .get_library_mut("user")
            .expect("the project library")
            .add_cell(cell);
        if let Some(ports) = schematic {
            app.state.model.workspace.insert_schematic_editor(
                CellViewRef::new("user", "amp", "schematic").key(),
                declaring_schematic(ports),
            );
        }
        let path = unique_path(label);
        assert!(app.state.save_project_to_path(&path), "the project saves");
        app.state.open_workspace_view(symbol.clone());
        app.state.workbench.activate(Workspace::Design);
        Self { app, path, symbol }
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

    /// The symbol as the project file on disk holds it.
    fn saved(&self) -> (SymbolDocument, SymbolEditorMetadata) {
        let persisted = crate::io::load_project_file(&self.path).expect("the project reloads");
        let view = persisted
            .file
            .libraries
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("the file holds the symbol")
            .clone();
        let document = SymbolDocument::load_from_view(&view).expect("the saved symbol loads");
        let metadata =
            SymbolEditorMetadata::load_from_view(&view, &document).expect("its metadata loads");
        (document, metadata)
    }

    /// Move the pin `name` by `by`, as one edit of the symbol in front.
    fn move_pin(&mut self, name: &str, by: Point) {
        let before = self.document();
        let metadata = self.metadata();
        let mut edited = before.clone();
        let pin = edited
            .pins
            .iter_mut()
            .find(|pin| pin.name == name)
            .expect("the pin is drawn");
        pin.position = pin.position.map(|at| at + by);
        crate::workbench::app::record_symbol_edit(
            &mut self.app.state.design.symbol,
            &self.app.state.model,
            &before,
            "Move pin",
        );
        self.app
            .state
            .store_active_symbol_editor_bundle(&edited, &metadata)
            .expect("the edit stores");
    }

    fn pin(&self, name: &str) -> Option<Point> {
        self.document().pin(name).and_then(|pin| pin.position)
    }

    fn save(&mut self) {
        self.app.execute_command_key(ProjectCommand::Save.key());
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        remove_project_artifacts(&self.path);
    }
}

/// Save with the symbol in front writes the symbol with the project, asks
/// nothing, and leaves it as the file holds it after a reopen. The save that
/// wrote changed artwork recorded a revision, dated, with no note; a save that
/// wrote nothing new recorded none.
#[test]
fn saving_the_project_writes_the_symbol_and_records_its_revision() {
    let mut fixture = Fixture::new("save", Some(&["IN", "OUT"]), &["IN", "OUT"]);
    let first = fixture.metadata();
    assert_eq!(
        first.revision, 1,
        "the first save of a project records each symbol it writes"
    );

    fixture.move_pin("OUT", Point::new(0, 20));
    let moved = fixture.pin("OUT");
    assert!(
        has_unsaved_changes(&fixture.app.state),
        "a symbol edit is unsaved project work"
    );
    assert!(
        fixture
            .app
            .command_availability(ProjectCommand::Save.key())
            .is_available(),
        "Save is offered with the symbol in front"
    );

    fixture.save();

    assert!(
        !has_unsaved_changes(&fixture.app.state),
        "the save wrote the symbol, with nothing asked on the way"
    );
    let (saved, metadata) = fixture.saved();
    assert_eq!(saved.pin("OUT").and_then(|pin| pin.position), moved);
    assert_eq!(
        metadata.revision, 2,
        "the save that wrote the edit recorded it"
    );
    let record = metadata.revisions.last().expect("the revision is recorded");
    assert_eq!(record.revision, 2);
    assert!(record.saved_unix_ms > 0, "the record is dated");

    fixture.save();
    assert_eq!(
        fixture.saved().1.revision,
        2,
        "a save that wrote no change to the symbol records nothing"
    );
}

/// The project's dirty state sees a symbol edit as it sees a schematic edit:
/// the close review lists it, and closing the symbol's tab keeps the edit in
/// the project, unsaved, without a question.
#[test]
fn symbol_edits_are_unsaved_project_work_until_the_project_is_saved() {
    let mut fixture = Fixture::new("dirty", Some(&["IN", "OUT"]), &["IN", "OUT"]);
    fixture.move_pin("IN", Point::new(0, -20));
    let moved = fixture.pin("IN");
    assert!(
        dirty_documents(&fixture.app.state)
            .contains(&ProjectDocumentId::CellView(fixture.symbol.clone())),
        "the symbol is a document the save and close reviews list"
    );

    // A second tab, so the symbol's is not the last one presented.
    fixture
        .app
        .state
        .open_workspace_view(CellViewRef::new("user", "amp", "schematic"));
    fixture
        .app
        .state
        .open_workspace_view(fixture.symbol.clone());
    Command::CloseActiveDocument.execute(&mut fixture.app);
    assert!(
        !fixture
            .app
            .state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == fixture.symbol),
        "the symbol's tab closed"
    );
    assert_eq!(fixture.pin("IN"), moved, "the project kept the edit");
    assert!(has_unsaved_changes(&fixture.app.state));

    assert!(fixture.app.state.request_close_project());
    assert!(
        matches!(
            fixture.app.state.workbench.project_review_dialog.request,
            Some(crate::workbench::app::ProjectReviewRequest::CloseProject(_))
        ),
        "closing the project asks about the unsaved symbol"
    );
}

/// Check and save with the symbol in front checks it, then saves it with the
/// project: what the checks find stands on the Problems page, and none of it
/// stops the save. A pin off the grid is found, and saved.
#[test]
fn check_and_save_checks_the_symbol_and_never_refuses_the_save() {
    let mut fixture = Fixture::new("findings", Some(&["IN", "OUT"]), &["IN", "OUT"]);
    fixture.move_pin("OUT", Point::new(3, 0));
    let off_grid = fixture.pin("OUT");
    assert!(
        fixture
            .app
            .command_availability(ProjectCommand::CheckAndSave.key())
            .is_available(),
        "Check and save is offered with the symbol in front"
    );

    fixture
        .app
        .execute_command_key(ProjectCommand::CheckAndSave.key());

    assert!(
        fixture
            .app
            .state
            .active_symbol_pin_finding_count()
            .is_some_and(|found| found > 0),
        "the symbol's checks ran and published what they found"
    );
    assert!(
        !fixture.app.state.project.check_and_save.open,
        "the schematic's validated save is not asked of a symbol"
    );
    assert_eq!(
        fixture.saved().0.pin("OUT").and_then(|pin| pin.position),
        off_grid,
        "the finding did not stop the save"
    );
    assert!(!has_unsaved_changes(&fixture.app.state));
}

/// A cell with no schematic is drawn symbol first: its symbol defines the
/// interface, and nothing is said about pins it has no schematic for.
#[test]
fn a_symbol_without_a_schematic_defines_its_own_interface() {
    let mut fixture = Fixture::new("symbol-first", None, &["A", "B"]);
    let interface = fixture.app.state.model.symbol_interface(&fixture.symbol);
    assert_eq!(interface, SymbolInterface::SymbolDefined);
    assert!(canvas_check_lines(fixture.document().pin_summary(&interface), "schematic").is_empty());

    fixture.app.state.run_active_symbol_pin_checks();
    assert_eq!(fixture.app.state.active_symbol_pin_finding_count(), Some(0));
}

/// A schematic that declares no pins is said to, as a warning; the symbol's
/// pins are not refused for it, and the symbol saves.
#[test]
fn a_schematic_declaring_no_pins_is_a_note_never_a_refusal() {
    let mut fixture = Fixture::new("empty-schematic", Some(&[]), &["A", "B"]);
    let interface = fixture.app.state.model.symbol_interface(&fixture.symbol);
    assert_eq!(interface, SymbolInterface::Declared(Vec::new()));
    assert_eq!(
        canvas_check_lines(fixture.document().pin_summary(&interface), "schematic"),
        ["The schematic declares no pins"]
    );

    fixture.move_pin("A", Point::new(0, 20));
    let moved = fixture.pin("A");
    fixture.save();
    assert_eq!(
        fixture.saved().0.pin("A").and_then(|pin| pin.position),
        moved
    );
    let levels = fixture
        .app
        .state
        .active_symbol_pin_finding_levels()
        .expect("the save ran the symbol's checks");
    assert!(!levels.is_empty(), "the undeclared pins are found");
    assert!(
        levels.iter().all(|level| *level
            == crate::workbench::app_state::diagnostic_store::DiagnosticLevel::Warning),
        "a pin the schematic does not declare is a warning: {levels:?}"
    );
}

/// A mismatch is stated by kind, with exact counts in the singular or the
/// plural.
#[test]
fn a_pin_mismatch_is_counted_by_kind() {
    let fixture = Fixture::new("mismatch", Some(&["IN", "OUT", "X"]), &["IN", "OUT", "Y"]);
    let interface = fixture.app.state.model.symbol_interface(&fixture.symbol);
    assert_eq!(
        canvas_check_lines(fixture.document().pin_summary(&interface), "schematic"),
        [
            "1 schematic pin has no symbol pin",
            "1 pin is not in the schematic"
        ]
    );

    let two = Fixture::new("mismatch-two", Some(&["IN", "X", "Z"]), &["IN", "P", "Q"]);
    let interface = two.app.state.model.symbol_interface(&two.symbol);
    assert_eq!(
        canvas_check_lines(two.document().pin_summary(&interface), "schematic"),
        [
            "2 schematic pins have no symbol pin",
            "2 pins are not in the schematic"
        ]
    );
}

/// Revert to saved puts the symbol back as the project last saved it, with
/// every wire a placed instance holds following the pin back.
#[test]
fn reverting_the_symbol_restores_it_and_its_placed_instances() {
    use crate::state::{Component, LibraryCellInstance, Wire, WireConnection};

    let mut fixture = Fixture::new("revert", Some(&["IN", "OUT"]), &["IN", "OUT"]);
    let saved = fixture.document();
    let out = saved
        .pin("OUT")
        .and_then(|pin| pin.position)
        .expect("OUT is placed");
    // An instance on a sibling sheet, wired at OUT.
    let parent = CellViewRef::new("user", "top", "schematic");
    let mut sheet = SchematicState::default();
    let mut binding = LibraryCellInstance::new("user", "amp", "schematic");
    binding.bind_interface(&fixture.app.state.model.symbol_ports(&fixture.symbol));
    // The instance stands at the origin unturned, so a terminal is where its
    // pin is, measured from the symbol's origin.
    let terminal = out - saved.origin;
    sheet.document_mut_for_test().components.push(
        Component::new(900, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(binding),
    );
    sheet.document_mut_for_test().wires.push(Wire::segment(
        950,
        terminal,
        terminal + Point::new(40, 0),
    ));
    sheet
        .document_mut_for_test()
        .connections
        .push(WireConnection {
            wire_id: 950,
            point_index: 0,
            component_id: 900,
            terminal_name: "OUT".to_owned(),
        });
    fixture
        .app
        .state
        .model
        .workspace
        .insert_schematic_editor(parent.key(), sheet);
    let wire_start = |fixture: &Fixture| {
        fixture
            .app
            .state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&parent.key())
            .expect("the parent sheet is held")
            .document()
            .wires[0]
            .points[0]
    };
    let before = wire_start(&fixture);
    assert!(
        !fixture
            .app
            .command_availability(Command::RevertActiveDocument.key())
            .is_available(),
        "nothing to revert before an edit"
    );

    fixture.move_pin("OUT", Point::new(0, 20));
    assert_ne!(
        wire_start(&fixture),
        before,
        "the edit carried the wire with the pin"
    );
    assert!(
        fixture
            .app
            .command_availability(Command::RevertActiveDocument.key())
            .is_available(),
        "Revert to saved is offered once the symbol has changed"
    );

    let token = crate::workbench::lifecycle::project_lifecycle::prepare_revert_active_document(
        &fixture.app.state,
    )
    .expect("the revert is reviewed");
    assert!(fixture.app.state.confirm_revert_active_document(&token));

    assert_eq!(fixture.document(), saved, "the symbol is as last saved");
    assert_eq!(
        wire_start(&fixture),
        before,
        "the wire followed the pin back"
    );
    assert!(
        !fixture.app.state.can_undo_active_symbol_document(),
        "no step of the edits it discarded applies any more"
    );
    assert!(
        !fixture
            .app
            .command_availability(Command::RevertActiveDocument.key())
            .is_available(),
        "the symbol is as saved again"
    );
}
