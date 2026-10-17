//! Wiring acceptance: whole edits driven through the calls the canvas and
//! the commands make, judged against what the netlister reads.
//!
//! The judge is never the canvas. A step's nets come from the extraction of
//! the document design management projects for a run, with every sheet laid
//! apart; its dots are the junctions the canvas paints, read sheet by sheet.

use std::collections::{BTreeMap, BTreeSet};

use super::RSpiceApp;
use crate::schematic::view::gesture_commits;
use crate::schematic::view::schematic_symbol_context;
use crate::schematic::view::selection_transform::{
    SelectionTransform, transform_selection_on_active_sheet,
};
use crate::schematic::view::sheet_visibility::finish_wire_on_active_sheet;
use crate::state::{
    Cell, CellViewRef, ComponentType, Library, MoveSelectionMode, Point, Rotation, Selection,
    SheetDefinition, SheetId, SheetPortPolicy, SheetTemplate, View, ViewType,
};
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;
use rspice_design::projection::ProjectionSource;
use rspice_design::schematic::document::SchematicDocument;

mod regressions;
mod scenario;

/// The library every bench cell lives in.
const LIBRARY: &str = "work";

/// An application with cells `top` and `parts`, `top` open with two sheets
/// and Sheet 1 active, driven one frame at a time.
pub(super) struct Bench {
    pub(super) app: RSpiceApp,
    ctx: egui::Context,
    pub(super) sheets: [SheetId; 2],
}

/// Multi-terminal nets, each the set of `name.pin` terminals the deck joins.
pub(super) type Nets = BTreeSet<BTreeSet<String>>;

/// Junction dots as `(sheet index, x, y)`.
pub(super) type Dots = BTreeSet<(usize, i32, i32)>;

fn cell_view(cell: &str) -> CellViewRef {
    CellViewRef::new(LIBRARY, cell, "schematic")
}

impl Bench {
    /// `top` open and empty but for `parts` placed on Sheet 1, which a
    /// second, empty sheet follows. Nothing is on the undo history.
    pub(super) fn new(parts: &[(&str, ComponentType, Point, Rotation)]) -> Self {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        let mut library = Library::new(LIBRARY);
        for name in ["top", "parts"] {
            let mut cell = Cell::new(name);
            cell.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(cell);
        }
        app.state.library_manager.add_library(library);
        app.state.open_workspace_view(cell_view("top"));
        app.state.workbench.activate(Workspace::Design);
        let state = &mut app.state;
        let mut ids = Vec::new();
        for (name, kind, at, rotation) in parts {
            state.schematic.session.editor.preview_rotation = *rotation;
            let id = state.schematic.add_component(*kind, *at);
            let document = state.schematic.document_mut_for_test();
            let part = document
                .components
                .iter_mut()
                .find(|part| part.id == id)
                .expect("placed part");
            part.name = (*name).to_owned();
            ids.push(id);
        }
        state.schematic.session.editor.preview_rotation = Rotation::R0;
        state.schematic.recalculate_runtime_state();
        let key = cell_view("top").key();
        let first = state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", ids)
            .expect("first sheet");
        let catalog = state
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .expect("second sheet");
        catalog.set_active(first).expect("active sheet");
        state.schematic.clear_undo_history();
        state.sync_active_schematic_to_workspace();
        Self {
            app,
            ctx: egui::Context::default(),
            sheets: [first, second],
        }
    }

    pub(super) fn state(&self) -> &AppState {
        &self.app.state
    }

    pub(super) fn state_mut(&mut self) -> &mut AppState {
        &mut self.app.state
    }

    /// What the end of every frame does: the live buffer goes back into the
    /// workspace, and objects authored since take the active sheet.
    fn end_frame(&mut self) {
        self.app.state.sync_active_schematic_to_workspace();
    }

    /// Clicks with the wire tool at each of `points`, the last one ending the
    /// route: the router adds a corner between two points off one line.
    pub(super) fn draw_wire(&mut self, points: &[Point]) {
        let (first, rest) = points.split_first().expect("a route has a start");
        let ctx = self.ctx.clone();
        let state = &mut self.app.state;
        state.schematic.arm_tool(crate::state::Tool::Wire);
        state.start_canvas_wire(&ctx, *first);
        for point in rest {
            state.schematic.extend_wire(*point);
        }
        let _ = finish_wire_on_active_sheet(state);
        state.schematic.cancel_tool();
        self.end_frame();
    }

    /// A click with the placement tool, the ghost turned to `rotation`.
    pub(super) fn place(&mut self, kind: ComponentType, at: Point, rotation: Rotation) -> u64 {
        let state = &mut self.app.state;
        let before = state
            .schematic
            .document()
            .components
            .iter()
            .map(|part| part.id)
            .collect::<BTreeSet<_>>();
        state.schematic.session.editor.preview_rotation = rotation;
        gesture_commits::place_component(state, kind, at);
        state.schematic.session.editor.preview_rotation = Rotation::R0;
        self.end_frame();
        self.app
            .state
            .schematic
            .document()
            .components
            .iter()
            .map(|part| part.id)
            .find(|id| !before.contains(id))
            .unwrap_or(0)
    }

    /// A click with the junction tool.
    pub(super) fn click_junction(&mut self, at: Point) {
        gesture_commits::click_junction(&mut self.app.state, at);
        self.end_frame();
    }

    /// Replace the selection, as a click or a window does.
    pub(super) fn select(&mut self, choose: impl FnOnce(&mut Selection)) {
        let selection = &mut self.app.state.schematic.session.editor.selection;
        selection.clear();
        choose(selection);
    }

    /// Move the selection by `delta`: arm Move, nudge the preview there and
    /// press Enter.
    pub(super) fn move_selection(&mut self, delta: Point, mode: MoveSelectionMode) {
        let state = &mut self.app.state;
        state.dialogs.move_selection.mode = mode;
        super::arm_move_selection(state);
        if state.dialogs.move_selection.armed {
            state.dialogs.move_selection.canvas.preview_delta = delta;
            gesture_commits::commit_move(state);
        }
        self.end_frame();
    }

    /// Stretch the selected segment by `delta`: arm Stretch, carry the
    /// preview there and press Enter.
    pub(super) fn stretch_selection(&mut self, delta: Point) {
        let state = &mut self.app.state;
        super::arm_stretch_selection(state);
        if state.dialogs.stretch_selection.armed {
            state.dialogs.stretch_selection.canvas.gesture.preview_delta = delta;
            gesture_commits::commit_stretch(state);
        }
        self.end_frame();
    }

    /// Rotate or flip the selection about its middle.
    pub(super) fn turn_selection(&mut self, transform: SelectionTransform) {
        let state = &mut self.app.state;
        let symbols = schematic_symbol_context(state);
        transform_selection_on_active_sheet(state, &symbols, transform);
        self.end_frame();
    }

    /// Copy the selection to the application's clipboard.
    pub(super) fn copy_selection(&mut self) {
        self.app.state.copy_active_schematic_selection();
    }

    /// Paste the clipboard and click it down at `at` without moving the
    /// pointer.
    pub(super) fn paste_at(&mut self, at: Point) {
        let state = &mut self.app.state;
        if matches!(state.paste_schematic_clipboard_at(at), Ok(true)) {
            state.place_floating_paste();
        }
        self.end_frame();
    }

    /// Open another cell of the library on the canvas.
    pub(super) fn open_cell(&mut self, cell: &str) {
        self.end_frame();
        self.app.state.open_workspace_view(cell_view(cell));
        self.app.state.workbench.activate(Workspace::Design);
    }

    /// Make sheet `index` the one being edited, as its tab does.
    pub(super) fn activate_sheet(&mut self, index: usize) {
        let _ = super::sheets::activate_sheet(&mut self.app.state, self.sheets[index]);
        self.end_frame();
    }

    pub(super) fn active_sheet(&self) -> usize {
        let active = super::sheets::active_sheet_id(&self.app.state);
        self.sheets
            .iter()
            .position(|sheet| Some(*sheet) == active)
            .expect("a bench sheet is active")
    }

    /// Edit > Undo.
    pub(super) fn undo(&mut self) {
        self.app.action_edit_undo();
        self.end_frame();
    }

    /// Edit > Redo.
    pub(super) fn redo(&mut self) {
        self.app.action_edit_redo();
        self.end_frame();
    }

    /// The sheet index an object belongs to; an object no sheet claims is on
    /// the active sheet, as the projection places it.
    pub(super) fn sheet_of(&self, id: u64) -> usize {
        let key = self
            .app
            .state
            .workspace
            .content
            .active_schematic_reference()
            .key();
        let catalog = self
            .app
            .state
            .workspace
            .content
            .design_management
            .sheet_catalog(&key)
            .expect("sheet catalog");
        let sheet = catalog
            .sheet_for_object(id)
            .or(catalog.active_sheet_id())
            .expect("an owning sheet");
        self.sheets
            .iter()
            .position(|candidate| *candidate == sheet)
            .expect("a bench sheet")
    }

    /// The document a run reads: every sheet materialized apart.
    pub(super) fn projected(&self) -> SchematicDocument {
        let state = &self.app.state;
        let key = state.workspace.content.active_schematic_reference().key();
        state
            .schematic
            .materialize(&state.workspace.content.design_management, &key)
            .expect("the design projects")
            .document()
            .clone()
    }

    /// The nets the deck joins, as `name.pin` terminals.
    pub(super) fn nets(&self) -> Nets {
        nets_of(&self.projected())
    }

    /// The dots the canvas paints, sheet by sheet.
    pub(super) fn dots(&self) -> Dots {
        self.app
            .state
            .schematic
            .document()
            .junctions
            .iter()
            .map(|junction| (self.sheet_of(junction.id), junction.pos.x, junction.pos.y))
            .collect()
    }

    /// A readable account of the drawing, for a failure message.
    pub(super) fn describe(&self) -> String {
        let document = self.app.state.schematic.document();
        let mut out = String::new();
        for part in &document.components {
            out.push_str(&format!(
                "  part {} {} at {:?} {:?} sheet {}\n",
                part.id,
                part.name,
                part.pos,
                part.rotation,
                self.sheet_of(part.id)
            ));
        }
        for wire in &document.wires {
            out.push_str(&format!(
                "  wire {} {:?} sheet {}\n",
                wire.id,
                wire.points,
                self.sheet_of(wire.id)
            ));
        }
        out.push_str(&format!(
            "  nets {:?}\n  dots {:?}\n  prompt {:?}\n",
            self.nets(),
            self.dots(),
            self.app.state.prompt_text()
        ));
        out
    }
}

/// Multi-terminal nets of `document`, grouped by the node name the deck
/// writes for each terminal.
pub(super) fn nets_of(document: &SchematicDocument) -> Nets {
    let extracted = rspice_design::connectivity::extract(document, |_| None, |_| None);
    let names = document
        .components
        .iter()
        .map(|part| (part.id, part.name.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut nodes = BTreeMap::<&str, BTreeSet<String>>::new();
    for terminal in &extracted.terminals {
        nodes
            .entry(terminal.net_name.as_str())
            .or_default()
            .insert(format!(
                "{}.{}",
                names.get(&terminal.component_id).copied().unwrap_or("?"),
                terminal.pin
            ));
    }
    nodes
        .into_values()
        .filter(|terminals| terminals.len() > 1)
        .collect()
}

/// `nets` written as the test spells them.
pub(super) fn nets(nets: &[&[&str]]) -> Nets {
    nets.iter()
        .map(|net| net.iter().map(|terminal| (*terminal).to_owned()).collect())
        .collect()
}

/// `dots` written as the test spells them.
pub(super) fn dots(dots: &[(usize, i32, i32)]) -> Dots {
    dots.iter().copied().collect()
}
