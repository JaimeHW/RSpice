//! Editing an instance's symbol in place, on the schematic that places it.
//!
//! The symbol view opens as the document, so every edit is the symbol
//! editor's own and lands on the symbol master. The parent stays open behind
//! it, drawn dimmed through the instance's transform and taking no input.
//! Leaving keeps every edit, as leaving any other view does: the symbol is
//! project data and is saved with the project.

use crate::state::{CellViewRef, ComponentType, ViewType};
use crate::workbench::app_state::Model;

use super::symbol_document::SymbolUiState;
use super::{NavigationParts, commit_pending_inspector_edit};
use rspice_schematic_editor::symbol_editor::placement::SymbolPlacement;

/// A symbol edited in place on the schematic that places one of its
/// instances.
///
/// The symbol view is the open document, so its tools, inspector, undo and
/// save are the symbol editor's own. This records what the canvas needs to
/// draw the symbol where the instance stands, over its parent, and what it
/// takes to hand the author back.
#[derive(Debug, Clone)]
pub struct SymbolInPlace {
    /// The symbol view every edit goes to.
    pub symbol: crate::state::CellViewRef,
    /// The schematic the instance stands on.
    pub parent: crate::state::CellViewRef,
    pub instance_id: u64,
    pub instance_name: String,
    /// The symbol origin when the session began. The instance is drawn from
    /// it for the whole session, so dragging the origin cannot move the frame
    /// the pointer is read in.
    pub origin: crate::state::Point,
    /// The in-place view's pan and zoom, which start at the parent canvas's.
    pub pan: (f64, f64),
    pub zoom: f64,
    /// The parent canvas's zoom and pan when the session began, given back
    /// to it on exit however the in-place view moved meanwhile.
    pub sheet_view: (f64, (f64, f64)),
    /// A zoom a command asked for, carried out on the canvas's next frame.
    pub zoom_request: Option<rspice_schematic_editor::view::navigation::ZoomRequest>,
    /// Whether the canvas's next frame is to bring the instance into view
    /// when the camera leaves any of it out, and otherwise keep the camera
    /// exactly where it is: set as the session begins, and again when a
    /// finding about one of the symbol's pins is followed.
    pub entering: bool,
    /// The views the session showed before this one, for Zoom previous.
    pub view_history: rspice_schematic_editor::view::navigation::ViewHistory<(f64, (f64, f64))>,
    /// Whether the symbol had a tab of its own before the session opened it.
    pub symbol_was_open: bool,
    /// The parent is drawn with nothing selected or hovered: it is context
    /// for the edit, not a drawing being edited.
    pub parent_editor: rspice_schematic_editor::session::EditorSession,
}

/// Where leaving an in-place symbol edit lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolInPlaceExit {
    /// The parent schematic, as the session found it.
    Parent,
    /// One level of the parent's occurrence, chosen on the breadcrumb.
    Level(usize),
}

/// Why the selection cannot have its symbol edited in place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolInPlaceRefusal {
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

/// The selected instance and the symbol view it would be edited in place
/// through, or why it cannot be.
pub fn selected_symbol_in_place_target(
    model: &Model,
) -> Result<(u64, CellViewRef), SymbolInPlaceRefusal> {
    if !matches!(
        model.workspace.content.active_view_type(),
        Some(ViewType::Schematic | ViewType::Testbench)
    ) {
        return Err(SymbolInPlaceRefusal::NotSchematic);
    }
    let id = model
        .schematic
        .session
        .editor
        .selection
        .single_component()
        .ok_or(SymbolInPlaceRefusal::NoInstance)?;
    symbol_in_place_target(model, id).map(|symbol| (id, symbol))
}

/// The symbol view one instance on the active schematic draws, when
/// that view can be edited.
fn symbol_in_place_target(
    model: &Model,
    component_id: u64,
) -> Result<CellViewRef, SymbolInPlaceRefusal> {
    use SymbolInPlaceRefusal as Refusal;
    let component = model
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
    let library = model
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
            .is_some_and(|view| view.view_type.is_hdl_source());
        return Err(if bound_to_module {
            Refusal::ModuleWithoutSymbol
        } else {
            Refusal::NoSymbolView
        });
    };
    if model.safe_mode.project_read_only() {
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

/// The in-place session the open document of `model` belongs to.
pub fn in_place_session<'a>(
    in_place: &'a Option<Box<SymbolInPlace>>,
    model: &Model,
) -> Option<&'a SymbolInPlace> {
    in_place
        .as_deref()
        .filter(|session| session.symbol == model.workspace.content.active_view)
}

/// Note on `model` the edit `in_place` holds, or that none is held: a
/// layout saved during the edit reads it.
pub(crate) fn sync_in_place_layout(in_place: &Option<Box<SymbolInPlace>>, model: &mut Model) {
    model.symbol_in_place_layout =
        in_place.as_deref().map(
            |session| crate::workbench::app_state::model::InPlaceLayout {
                symbol: session.symbol.clone(),
                parent: session.parent.clone(),
                symbol_was_open: session.symbol_was_open,
            },
        );
}

/// [`in_place_session`], to write to.
pub fn in_place_session_mut<'a>(
    in_place: &'a mut Option<Box<SymbolInPlace>>,
    model: &Model,
) -> Option<&'a mut SymbolInPlace> {
    let active = &model.workspace.content.active_view;
    in_place
        .as_deref_mut()
        .filter(|session| &session.symbol == active)
}

/// Where the edited instance puts its symbol on the parent, read from the
/// parent as it stands. `None` once the parent or the instance is gone,
/// which ends the session.
pub fn in_place_placement(session: &SymbolInPlace, model: &Model) -> Option<SymbolPlacement> {
    let parent = model
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
pub fn in_place_bounds(session: &SymbolInPlace, model: &Model) -> Option<(f64, f64, f64, f64)> {
    const MARGIN: f64 = 20.0;
    let placement = in_place_placement(session, model)?;
    let document = model.load_active_symbol_document().ok()?;
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

/// Whether Esc has nothing of the symbol canvas's own to cancel first, so
/// it leaves the in-place edit instead.
pub fn symbol_in_place_canvas_idle(symbol: &SymbolUiState, model: &Model) -> bool {
    let editor = &symbol.editor;
    in_place_session(&symbol.in_place, model).is_some()
        && editor.tool == crate::workbench::SymbolTool::Select
        && !editor.has_draft()
        && editor.marquee_start.is_none()
        && editor.dragging_pin.is_none()
        && editor.dragging_shape.is_none()
        && editor.dragging_label.is_none()
        && editor.dragging_group.is_none()
        && editor.dragging_handle.is_none()
        && editor.stretch.is_none()
        && !editor.dragging_origin
}

impl NavigationParts<'_> {
    /// Open the selected instance's symbol where the instance stands.
    pub(crate) fn edit_selected_symbol_in_place(&mut self) -> bool {
        match selected_symbol_in_place_target(self.model) {
            Ok((instance, symbol)) => self.edit_symbol_in_place(instance, symbol),
            Err(refusal) => {
                self.wb.prompt(format!(
                    "Edit symbol in place is unavailable: {}.",
                    refusal.reason()
                ));
                false
            }
        }
    }

    fn edit_symbol_in_place(&mut self, instance_id: u64, symbol: CellViewRef) -> bool {
        let Some(instance_name) = self
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == instance_id)
            .map(|component| component.name.clone())
        else {
            return false;
        };
        let origin = match self.model.load_symbol_document(&symbol) {
            Ok(document) => document.origin,
            Err(error) => {
                self.wb.prompt(error);
                return false;
            }
        };
        let parent = self.model.workspace.content.active_view.clone();
        let (pan, zoom) = (
            self.model.schematic.session.editor.pan,
            self.model.schematic.session.editor.zoom,
        );
        let symbol_was_open = self
            .model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == symbol);
        self.open_workspace_view(symbol.clone());
        if self.model.workspace.content.active_view != symbol {
            // An inspector edit that could not be committed kept the parent.
            return false;
        }
        self.restore.symbol.editor.tool = crate::workbench::SymbolTool::Select;
        self.restore.symbol.editor.clear_selection();
        self.restore.symbol.in_place = Some(Box::new(SymbolInPlace {
            origin,
            symbol: symbol.clone(),
            parent: parent.clone(),
            instance_id,
            instance_name: instance_name.clone(),
            pan,
            zoom,
            sheet_view: (zoom, pan),
            zoom_request: None,
            entering: true,
            view_history: Default::default(),
            symbol_was_open,
            parent_editor: Default::default(),
        }));
        sync_in_place_layout(&self.restore.symbol.in_place, self.model);
        self.wb.prompt(format!(
            "Editing the {} symbol in place on {instance_name}. Esc or Return goes back to {}.",
            symbol.cell, parent.cell
        ));
        true
    }

    /// Leave the in-place edit for `exit`. Every edit made in it stays; a
    /// shape, text or pin name still in flight holds the session, by name,
    /// rather than being dropped on the way out.
    pub fn request_symbol_in_place_exit(&mut self, exit: SymbolInPlaceExit) {
        if in_place_session(&self.restore.symbol.in_place, self.model).is_none() {
            return;
        }
        if let Some(activity) = self.restore.symbol.editor.draft_activity() {
            self.wb.prompt(format!(
                "Still editing the symbol: {activity}. Finish it, or press Esc to cancel it."
            ));
            return;
        }
        if !commit_pending_inspector_edit(self.model, self.wb) {
            return;
        }
        self.finish_symbol_in_place(exit);
    }

    /// End the session and land where `exit` says, the parent reopening with
    /// the instance selected and its canvas where it stood when the session
    /// began.
    pub(crate) fn finish_symbol_in_place(&mut self, exit: SymbolInPlaceExit) {
        let Some(session) = self.restore.symbol.in_place.take() else {
            return;
        };
        self.open_workspace_view(session.parent.clone());
        if self.model.workspace.content.active_view != session.parent {
            self.restore.symbol.in_place = Some(session);
            return;
        }
        sync_in_place_layout(&self.restore.symbol.in_place, self.model);
        if !session.symbol_was_open {
            self.model.workspace.content.close_view(&session.symbol);
        }
        let editor = &mut self.model.schematic.session.editor;
        (editor.zoom, editor.pan) = session.sheet_view;
        editor.selection.select_only_component(session.instance_id);
        match exit {
            SymbolInPlaceExit::Level(level)
                if level + 1 < self.model.workspace.content.occurrence_depth() =>
            {
                self.focus_workspace_breadcrumb(level);
            }
            SymbolInPlaceExit::Level(_) | SymbolInPlaceExit::Parent => {
                self.wb.prompt(format!(
                    "Back on {} with {} selected.",
                    session.parent.cell, session.instance_name
                ));
            }
        }
    }
}
