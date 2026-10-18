//! Editing an instance's symbol in place, on the schematic that places it.
//!
//! The symbol view opens as the document, so every edit is the symbol
//! editor's own and lands on the symbol master. The parent stays open behind
//! it, drawn dimmed through the instance's transform and taking no input.
//! Leaving with unsaved edits asks whether to save them first.

#[cfg(test)]
mod tests;

use crate::state::{CellViewRef, ComponentType, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::session::{SymbolInPlace, SymbolInPlaceExit};
use rspice_schematic_editor::symbol_editor::placement::SymbolPlacement;

/// Why the selection cannot have its symbol edited in place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SymbolInPlaceRefusal {
    NotSchematic,
    NoInstance,
    BuiltInSymbol,
    BuiltInXspiceSymbol,
    CellNotLoaded,
    ModuleWithoutSymbol,
    NoSymbolView,
    SafeMode,
    ReadOnlyLibrary,
    UnloadableSymbol,
}

impl SymbolInPlaceRefusal {
    /// The reason as a command's unavailable state completes it.
    pub(crate) const fn reason(self) -> &'static str {
        match self {
            Self::NotSchematic => "open the schematic that places the instance",
            Self::NoInstance => "select one instance",
            Self::BuiltInSymbol => "a primitive's symbol is built in",
            Self::BuiltInXspiceSymbol => "the instance draws a built-in XSPICE symbol",
            Self::CellNotLoaded => "the instance's library cell is not loaded",
            Self::ModuleWithoutSymbol => "the Verilog-A module has no symbol view",
            Self::NoSymbolView => "the cell has no symbol view",
            Self::SafeMode => "safe mode opened this project read-only",
            Self::ReadOnlyLibrary => "the symbol's library is read-only",
            Self::UnloadableSymbol => "the symbol view does not load",
        }
    }

    /// The reason as a sentence of its own, for a menu row's tooltip.
    pub(crate) const fn sentence(self) -> &'static str {
        match self {
            Self::NotSchematic => "Open the schematic that places the instance",
            Self::NoInstance => "Select one instance",
            Self::BuiltInSymbol => "A primitive's symbol is built in",
            Self::BuiltInXspiceSymbol => "The instance draws a built-in XSPICE symbol",
            Self::CellNotLoaded => "The instance's library cell is not loaded",
            Self::ModuleWithoutSymbol => "The Verilog-A module has no symbol view",
            Self::NoSymbolView => "The cell has no symbol view",
            Self::SafeMode => "Safe mode opened this project read-only",
            Self::ReadOnlyLibrary => "The symbol's library is read-only",
            Self::UnloadableSymbol => "The symbol view does not load",
        }
    }
}

impl AppState {
    /// The selected instance and the symbol view it would be edited in place
    /// through, or why it cannot be.
    pub(crate) fn selected_symbol_in_place_target(
        &self,
    ) -> Result<(u64, CellViewRef), SymbolInPlaceRefusal> {
        if !matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return Err(SymbolInPlaceRefusal::NotSchematic);
        }
        let id = self
            .schematic
            .session
            .editor
            .selection
            .single_component()
            .ok_or(SymbolInPlaceRefusal::NoInstance)?;
        self.symbol_in_place_target(id).map(|symbol| (id, symbol))
    }

    /// The symbol view one instance on the active schematic draws, when
    /// that view can be edited.
    pub(crate) fn symbol_in_place_target(
        &self,
        component_id: u64,
    ) -> Result<CellViewRef, SymbolInPlaceRefusal> {
        use SymbolInPlaceRefusal as Refusal;
        let component = self
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .ok_or(Refusal::NoInstance)?;
        if component.kind != ComponentType::CellInstance {
            return Err(Refusal::BuiltInSymbol);
        }
        let binding = component
            .library_cell
            .as_ref()
            .ok_or(Refusal::CellNotLoaded)?;
        if binding.builtin_xspice.is_some() {
            return Err(Refusal::BuiltInXspiceSymbol);
        }
        let library = self
            .library_manager
            .get_library(&binding.library)
            .ok_or(Refusal::CellNotLoaded)?;
        let cell = library
            .get_cell(&binding.cell)
            .ok_or(Refusal::CellNotLoaded)?;
        // The view the canvas resolves the instance's artwork from.
        let symbol = cell
            .get_view(&binding.view)
            .filter(|view| view.view_type == ViewType::Symbol)
            .or_else(|| {
                cell.get_view("symbol")
                    .filter(|view| view.view_type == ViewType::Symbol)
            })
            .or_else(|| {
                cell.views_sorted()
                    .into_iter()
                    .find(|view| view.view_type == ViewType::Symbol)
            });
        let Some(symbol) = symbol else {
            let bound_to_module = cell
                .get_view(&binding.view)
                .is_some_and(|view| view.view_type == ViewType::VerilogA);
            return Err(if bound_to_module {
                Refusal::ModuleWithoutSymbol
            } else {
                Refusal::NoSymbolView
            });
        };
        if self.workbench.safe_mode.project_read_only() {
            return Err(Refusal::SafeMode);
        }
        if library.read_only {
            return Err(Refusal::ReadOnlyLibrary);
        }
        if crate::state::SymbolDocument::load_from_view(symbol).is_err() {
            return Err(Refusal::UnloadableSymbol);
        }
        Ok(CellViewRef::new(
            &binding.library,
            &binding.cell,
            &symbol.name,
        ))
    }

    /// The in-place session the open document belongs to.
    pub(crate) fn symbol_in_place(&self) -> Option<&SymbolInPlace> {
        self.ui
            .symbol
            .in_place
            .as_deref()
            .filter(|session| session.symbol == self.workspace.content.active_view)
    }

    pub(crate) fn symbol_in_place_mut(&mut self) -> Option<&mut SymbolInPlace> {
        let active = &self.workspace.content.active_view;
        self.ui
            .symbol
            .in_place
            .as_deref_mut()
            .filter(|session| &session.symbol == active)
    }

    /// Where the edited instance puts its symbol on the parent, read from
    /// the parent as it stands. `None` once the parent or the instance is
    /// gone, which ends the session.
    pub(crate) fn symbol_in_place_placement(&self) -> Option<SymbolPlacement> {
        let session = self.symbol_in_place()?;
        let parent = self
            .workspace
            .content
            .schematic_buffers
            .get(&session.parent.key())?;
        parent
            .document()
            .components
            .iter()
            .find(|component| component.id == session.instance_id)
            .map(|instance| SymbolPlacement::of_instance(instance, session.origin))
    }

    /// The parent-sheet rectangle the edited symbol covers, which Fit frames.
    pub(crate) fn symbol_in_place_bounds(&self) -> Option<(f64, f64, f64, f64)> {
        const MARGIN: f64 = 20.0;
        let placement = self.symbol_in_place_placement()?;
        let document = self.load_active_symbol_document().ok()?;
        let body = document.drawn_body_bounds();
        let points = document
            .pins
            .iter()
            .filter_map(|pin| pin.position)
            .chain(body.into_iter().flat_map(|(min, max)| [min, max]))
            .chain([document.origin])
            .map(|point| placement.place(point));
        let (low, high) = points.fold(
            ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)),
            |(low, high), point| {
                let (x, y) = (f64::from(point.x), f64::from(point.y));
                ((low.0.min(x), low.1.min(y)), (high.0.max(x), high.1.max(y)))
            },
        );
        Some((
            low.0 - MARGIN,
            low.1 - MARGIN,
            high.0 + MARGIN,
            high.1 + MARGIN,
        ))
    }

    /// Open the selected instance's symbol where the instance stands.
    pub(crate) fn edit_selected_symbol_in_place(&mut self) -> bool {
        match self.selected_symbol_in_place_target() {
            Ok((instance, symbol)) => self.edit_symbol_in_place(instance, symbol),
            Err(refusal) => {
                self.prompt(format!(
                    "Edit symbol in place is unavailable: {}.",
                    refusal.reason()
                ));
                false
            }
        }
    }

    fn edit_symbol_in_place(&mut self, instance_id: u64, symbol: CellViewRef) -> bool {
        let Some(instance_name) = self
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == instance_id)
            .map(|component| component.name.clone())
        else {
            return false;
        };
        let baseline = match self.load_symbol_document(&symbol).and_then(|document| {
            let metadata = self.load_symbol_editor_metadata(&symbol, &document)?;
            Ok((document, metadata))
        }) {
            Ok(baseline) => baseline,
            Err(error) => {
                self.prompt(error);
                return false;
            }
        };
        let parent = self.workspace.content.active_view.clone();
        let (pan, zoom) = (
            self.schematic.session.editor.pan,
            self.schematic.session.editor.zoom,
        );
        let symbol_was_open = self
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == symbol);
        self.open_workspace_view(symbol.clone());
        if self.workspace.content.active_view != symbol {
            // An inspector edit that could not be committed kept the parent.
            return false;
        }
        self.ui.symbol.editor.tool = crate::workbench::SymbolTool::Select;
        self.ui.symbol.editor.clear_selection();
        self.ui.symbol.in_place = Some(Box::new(SymbolInPlace {
            origin: baseline.0.origin,
            baseline_depth: self.ui.symbol.history.undo_depth(&symbol.key()),
            symbol: symbol.clone(),
            parent: parent.clone(),
            instance_id,
            instance_name: instance_name.clone(),
            pan,
            zoom,
            zoom_request: None,
            symbol_was_open,
            baseline,
            leaving: None,
            exit_after_save: None,
            parent_editor: Default::default(),
        }));
        self.prompt(format!(
            "Editing the {} symbol in place on {instance_name}. Esc or Return goes back to {}.",
            symbol.cell, parent.cell
        ));
        true
    }

    /// Whether the session holds edits the symbol has not been saved with.
    pub(crate) fn symbol_in_place_has_unsaved_edits(&self) -> bool {
        let Some(session) = self.symbol_in_place() else {
            return false;
        };
        if !self.ui.symbol.history.is_dirty(&session.symbol.key()) {
            return false;
        }
        let Ok(document) = self.load_symbol_document(&session.symbol) else {
            return false;
        };
        let metadata = self.load_symbol_editor_metadata(&session.symbol, &document);
        document != session.baseline.0 || metadata.ok().as_ref() != Some(&session.baseline.1)
    }

    /// Whether Esc has nothing of the canvas's own to cancel first, so it
    /// leaves the in-place edit instead.
    pub(crate) fn symbol_in_place_canvas_idle(&self) -> bool {
        let editor = &self.ui.symbol.editor;
        self.symbol_in_place().is_some()
            && editor.tool == crate::workbench::SymbolTool::Select
            && editor.pending_polyline.is_empty()
            && editor.shape_start.is_none()
            && editor.marquee_start.is_none()
            && editor.dragging_pin.is_none()
            && editor.dragging_shape.is_none()
            && editor.dragging_label.is_none()
            && editor.dragging_group.is_none()
            && !editor.dragging_origin
    }

    /// Leave the in-place edit for `exit`, asking first when the session
    /// holds unsaved edits.
    pub(crate) fn request_symbol_in_place_exit(&mut self, exit: SymbolInPlaceExit) {
        if self.symbol_in_place().is_none() || !self.commit_pending_inspector_edit() {
            return;
        }
        if self.symbol_in_place_has_unsaved_edits() {
            if let Some(session) = self.symbol_in_place_mut() {
                session.leaving = Some(exit);
            }
        } else {
            self.finish_symbol_in_place(exit);
        }
    }

    /// Save, then leave: the symbol editor's own save transaction runs, and
    /// the session ends when it publishes.
    pub(crate) fn save_and_leave_symbol_in_place(&mut self) {
        let Some(exit) = self
            .symbol_in_place_mut()
            .and_then(|session| session.leaving.take())
        else {
            return;
        };
        if let Some(refusal) = self.refuse_unsavable_active_symbol() {
            self.prompt(refusal);
            return;
        }
        if let Some(session) = self.symbol_in_place_mut() {
            session.exit_after_save = Some(exit);
        }
        self.ui.symbol.editor.save_dialog_open = true;
        self.ui.symbol.editor.save_error = None;
    }

    /// Put the symbol back as the session found it, or as it was last saved
    /// in it, then leave.
    pub(crate) fn discard_and_leave_symbol_in_place(&mut self) {
        let Some(exit) = self
            .symbol_in_place_mut()
            .and_then(|session| session.leaving.take())
        else {
            return;
        };
        match self.discard_symbol_in_place_edits() {
            Ok(()) => self.finish_symbol_in_place(exit),
            Err(error) => self.prompt(format!("The symbol edits were not discarded: {error}")),
        }
    }

    /// Stay in the session: the question is withdrawn and nothing changes.
    pub(crate) fn stay_in_symbol_in_place(&mut self) {
        if let Some(session) = self.symbol_in_place_mut() {
            session.leaving = None;
        }
    }

    /// The save dialog published the symbol: what it published is the new
    /// baseline, and a save asked for on the way out finishes leaving.
    pub(crate) fn symbol_in_place_saved(&mut self) {
        let Some(symbol) = self.symbol_in_place().map(|session| session.symbol.clone()) else {
            return;
        };
        let depth = self.ui.symbol.history.undo_depth(&symbol.key());
        let baseline = self.load_symbol_document(&symbol).and_then(|document| {
            let metadata = self.load_symbol_editor_metadata(&symbol, &document)?;
            Ok((document, metadata))
        });
        let Some(session) = self.symbol_in_place_mut() else {
            return;
        };
        if let Ok(baseline) = baseline {
            session.baseline = baseline;
            session.baseline_depth = depth;
        }
        if let Some(exit) = session.exit_after_save.take() {
            self.finish_symbol_in_place(exit);
        }
    }

    /// The save dialog closed without publishing: the author stays.
    pub(crate) fn symbol_in_place_save_cancelled(&mut self) {
        if let Some(session) = self.symbol_in_place_mut() {
            session.exit_after_save = None;
        }
    }

    fn discard_symbol_in_place_edits(&mut self) -> Result<(), String> {
        let Some(session) = self.symbol_in_place() else {
            return Ok(());
        };
        let key = session.symbol.key();
        let depth = session.baseline_depth;
        let baseline = session.baseline.clone();
        while self.ui.symbol.history.undo_depth(&key) > depth {
            if !self.undo_active_symbol_document()? {
                break;
            }
        }
        while self.ui.symbol.history.undo_depth(&key) < depth {
            if !self.redo_active_symbol_document()? {
                break;
            }
        }
        // The edits are gone, not waiting to be redone.
        self.ui.symbol.history.discard_redo(&key);
        // A stack trimmed to its cap cannot walk back that far; the
        // baseline itself is what the symbol returns to.
        let document = self.load_active_symbol_document()?;
        let metadata = self.load_active_symbol_editor_metadata(&document)?;
        if (document, metadata) != baseline {
            self.store_active_symbol_editor_bundle(&baseline.0, &baseline.1)?;
        }
        Ok(())
    }

    /// End the session and land where `exit` says, the parent reopening with
    /// the instance selected and its canvas where the session left it.
    fn finish_symbol_in_place(&mut self, exit: SymbolInPlaceExit) {
        let Some(session) = self.ui.symbol.in_place.take() else {
            return;
        };
        self.open_workspace_view(session.parent.clone());
        if self.workspace.content.active_view != session.parent {
            self.ui.symbol.in_place = Some(session);
            return;
        }
        if !session.symbol_was_open {
            self.workspace.content.close_view(&session.symbol);
        }
        let editor = &mut self.schematic.session.editor;
        editor.pan = session.pan;
        editor.zoom = session.zoom;
        editor.selection.select_only_component(session.instance_id);
        match exit {
            SymbolInPlaceExit::Level(level)
                if level + 1 < self.workspace.content.occurrence_depth() =>
            {
                self.focus_workspace_breadcrumb(level);
            }
            SymbolInPlaceExit::Level(_) | SymbolInPlaceExit::Parent => {
                self.prompt(format!(
                    "Back on {} with {} selected.",
                    session.parent.cell, session.instance_name
                ));
            }
        }
    }
}
