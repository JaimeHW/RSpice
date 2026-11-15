//! The symbol canvas's context menu: what it was opened on and what its
//! rows ask for. It is drawn by the schematic canvas's own menu surface.

use super::interaction::SymbolRequestSource;
use super::session::{SymbolEditorSession, SymbolSelection, SymbolTool, selection_subject};
use super::{SymbolViewport, hit_label, hit_pin, hit_shape};
use crate::view::context_menu::{MenuAction, MenuBinding};
use egui::Pos2;
use rspice_design::symbol::{SymbolDocument, SymbolEditorMetadata};
use rspice_design_model::Point;

/// What the menu was opened on, which decides the rows it shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolMenuTarget {
    /// The selection: the click landed on something selected, or on
    /// something it selected.
    Selection,
    /// The canvas, where nothing was under the click.
    Canvas,
    /// The shape the armed tool is drawing.
    Drawing,
}

/// The symbol, selection and tool the menu was opened against; a menu
/// whose binding no longer holds closes rather than act.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolMenuBinding {
    pub source: SymbolRequestSource,
    pub selection: SymbolSelection,
    pub tool: SymbolTool,
    pub target: SymbolMenuTarget,
    /// Where the menu was opened, in symbol coordinates: where Paste here
    /// puts the clipboard.
    pub click_pos: Point,
}

impl MenuBinding for SymbolMenuBinding {
    fn title(&self) -> &'static str {
        match self.target {
            SymbolMenuTarget::Drawing => "Drawing",
            SymbolMenuTarget::Selection => "Symbol selection",
            SymbolMenuTarget::Canvas => "Symbol",
        }
    }
}

/// What a row of the menu runs: each is the command the menus and keys
/// run, or the key the shape being drawn answers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolMenuAction {
    Properties,
    Rotate,
    Mirror,
    Cut,
    Copy,
    /// Paste the clipboard where the menu was opened.
    Paste,
    /// Paste a copy of the selection where the menu was opened.
    Duplicate,
    Delete,
    /// Arm a drawing tool.
    Tool(SymbolTool),
    Fit,
    /// Show or hide the grid.
    Grid,
    /// Finish the line or polygon being drawn, as Enter does.
    Finish,
    /// Take back the last point placed, as Backspace does.
    RemoveLastPoint,
    /// Let go of the shape being drawn, as Escape does.
    Cancel,
}

impl MenuAction for SymbolMenuAction {}

/// What a right-click at `pos` opens the menu on. While a shape is being
/// drawn it is that shape, and the click neither finishes nor cancels it.
/// Otherwise an object under the click that is not selected becomes the
/// selection, as a click would make it, so the menu acts on what was
/// clicked; one already selected keeps the selection it is part of.
pub fn pointer_target(
    session: &mut SymbolEditorSession,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    viewport: SymbolViewport,
    pos: Pos2,
) -> SymbolMenuTarget {
    if session.tool.draws() && session.has_draft() {
        return SymbolMenuTarget::Drawing;
    }
    let selection = session.effective_selection();
    let hit = if let Some(pin) = hit_pin(document, viewport, pos) {
        Some((
            selection.pins.contains(&pin),
            SymbolSelection::single_pin(pin),
        ))
    } else if let Some(kind) = hit_label(editor, viewport, pos) {
        Some((
            selection.attributes.contains(&kind),
            SymbolSelection::single_attribute(kind),
        ))
    } else {
        hit_shape(document, viewport, pos).map(|index| {
            (
                selection.shapes.contains(&index),
                SymbolSelection::single_shape(index),
            )
        })
    };
    match hit {
        Some((true, _)) => SymbolMenuTarget::Selection,
        Some((false, picked)) => {
            session.set_selection(picked);
            SymbolMenuTarget::Selection
        }
        None => SymbolMenuTarget::Canvas,
    }
}

/// What Shift+F10 opens the menu on: the shape being drawn, else the
/// selection, else the canvas.
pub fn keyboard_target(session: &SymbolEditorSession) -> SymbolMenuTarget {
    if session.tool.draws() && session.has_draft() {
        SymbolMenuTarget::Drawing
    } else if session.effective_selection().is_empty() {
        SymbolMenuTarget::Canvas
    } else {
        SymbolMenuTarget::Selection
    }
}

/// What the menu's header names under its title: the selected object or how
/// many there are, or the tool drawing. The bare canvas has no header.
pub fn summary(document: &SymbolDocument, binding: &SymbolMenuBinding) -> Option<String> {
    match binding.target {
        SymbolMenuTarget::Selection => Some(selection_subject(document, &binding.selection)),
        SymbolMenuTarget::Drawing => Some(binding.tool.label().to_owned()),
        SymbolMenuTarget::Canvas => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::symbol::SymbolShape;

    fn viewport() -> SymbolViewport {
        SymbolViewport::new(
            egui::Rect::from_min_size(Pos2::ZERO, egui::vec2(400.0, 400.0)),
            1.0,
            egui::Vec2::ZERO,
        )
    }

    fn document() -> SymbolDocument {
        SymbolDocument {
            body: vec![
                SymbolShape::Circle {
                    center: Point::origin(),
                    radius: 20,
                },
                SymbolShape::Circle {
                    center: Point::new(100, 0),
                    radius: 20,
                },
            ],
            ..SymbolDocument::default()
        }
    }

    /// A right-click on an unselected shape selects it; on a selected one it
    /// keeps the whole selection; on nothing it opens the canvas menu and
    /// leaves the selection alone.
    #[test]
    fn a_right_click_opens_the_menu_on_what_it_lands_on() {
        let document = document();
        let editor = SymbolEditorMetadata::for_document(&document);
        let at = |point: Point| viewport().world_to_screen(point);
        let mut session = SymbolEditorSession::default();

        let target = pointer_target(
            &mut session,
            &document,
            &editor,
            viewport(),
            at(Point::new(20, 0)),
        );
        assert_eq!(target, SymbolMenuTarget::Selection);
        assert_eq!(
            session.effective_selection(),
            SymbolSelection::single_shape(0)
        );

        let both = SymbolSelection {
            shapes: [0, 1].into_iter().collect(),
            ..SymbolSelection::default()
        };
        session.set_selection(both.clone());
        pointer_target(
            &mut session,
            &document,
            &editor,
            viewport(),
            at(Point::new(120, 0)),
        );
        assert_eq!(session.effective_selection(), both);

        let target = pointer_target(
            &mut session,
            &document,
            &editor,
            viewport(),
            at(Point::new(60, 80)),
        );
        assert_eq!(target, SymbolMenuTarget::Canvas);
        assert_eq!(session.effective_selection(), both);
    }

    /// While a line is being drawn the menu is the drawing's, wherever the
    /// click lands, and the click selects nothing.
    #[test]
    fn a_right_click_while_drawing_opens_the_drawing_menu() {
        let document = document();
        let editor = SymbolEditorMetadata::for_document(&document);
        let mut session = SymbolEditorSession {
            tool: SymbolTool::Line,
            pending_polyline: vec![Point::new(0, 40)],
            ..SymbolEditorSession::default()
        };
        let target = pointer_target(
            &mut session,
            &document,
            &editor,
            viewport(),
            viewport().world_to_screen(Point::new(20, 0)),
        );
        assert_eq!(target, SymbolMenuTarget::Drawing);
        assert!(session.effective_selection().is_empty());
        assert_eq!(session.pending_polyline, vec![Point::new(0, 40)]);
        assert_eq!(keyboard_target(&session), SymbolMenuTarget::Drawing);
    }
}
