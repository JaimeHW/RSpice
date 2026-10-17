//! Paste and Duplicate float the copy under the pointer until a click
//! places it, as Virtuoso's Copy and KiCad's Paste do.
//!
//! Nothing on the drawing changes while the copy floats. It is held bound to
//! the cell view it was asked for in, R and M turn it, Esc lets it go, and
//! the click pastes it as one undo step joined to the wiring it lands on.
//! History waits while it floats, as it waits for a drag.

use rspice_design::schematic::clipboard::ClipboardData;
use rspice_design::schematic::component_edit::{SelectionTransform, transform_clipboard};

use super::{bound_to_active_cell, named_net_connections, object_count};
use crate::state::{Point, Tool};
use crate::workbench::app::dialogs::state::{
    FloatingPaste, FloatingPasteOrigin, FloatingPastePreview,
};
use crate::workbench::app_state::AppState;

impl AppState {
    /// Float `objects` with their middle at `anchor`. The pointer carries
    /// the copy from there; a copy asked for with the pointer off the canvas
    /// jumps to the pointer when it arrives.
    pub(super) fn float_schematic_objects(
        &mut self,
        objects: &ClipboardData,
        anchor: Point,
        origin: FloatingPasteOrigin,
    ) -> Result<(), String> {
        let objects = bound_to_active_cell(self, objects)?.into_owned();
        // A copy floating is the command now running; the one before it
        // stops, as a new command stops the last in every schematic editor.
        if self.dialogs.move_selection.armed {
            crate::workbench::app::cancel_armed_move_selection(self);
        }
        if self.dialogs.stretch_selection.armed {
            crate::workbench::app::cancel_armed_stretch_selection(self);
        }
        if self.dialogs.array_selection.armed {
            crate::workbench::app::cancel_armed_array_selection(self);
        }
        if self.schematic.session.editor.tool != Tool::Select {
            self.schematic.cancel_tool();
        }
        let grid = self.schematic.document().grid_size;
        let count = objects.count();
        self.dialogs.floating_paste = Some(FloatingPaste {
            objects,
            start: on_grid(anchor, grid),
            grab: self
                .ui
                .canvas_hover
                .map(|_| on_grid(self.schematic_paste_anchor(), grid)),
            delta: Point::origin(),
            view_path: self.workspace.content.active_view.display_path(),
            sheet: active_sheet(self),
            origin,
            preview: None,
        });
        self.prompt(format!(
            "Click to place {}. R rotates, M mirrors, Esc cancels.",
            object_count(count)
        ));
        Ok(())
    }

    /// Land the floating copy where it sits now. Returns whether anything
    /// was placed.
    pub(crate) fn place_floating_paste(&mut self) -> bool {
        let Some(floating) = self.dialogs.floating_paste.take() else {
            return false;
        };
        if let Err(reason) = floating_scope(self, &floating) {
            self.prompt(format!("{} cancelled: {reason}.", floating.origin.verb()));
            return false;
        }
        let pasted = self.paste_schematic_objects_at(&floating.objects, floating.at());
        let placed = matches!(pasted, Ok(true));
        self.prompt(match (pasted, floating.origin) {
            (Ok(true), FloatingPasteOrigin::Paste) => {
                format!("Pasted {}.", object_count(floating.objects.count()))
            }
            (
                Ok(true),
                FloatingPasteOrigin::Duplicate {
                    count,
                    kept,
                    unresolved_nets,
                },
            ) => {
                if unresolved_nets {
                    format!(
                        "Duplicated {}; named-net connections could not be resolved.",
                        object_count(count)
                    )
                } else if kept == 0 {
                    format!("Duplicated {}.", object_count(count))
                } else {
                    format!(
                        "Duplicated {}; kept {}.",
                        object_count(count),
                        named_net_connections(kept)
                    )
                }
            }
            (Ok(false), origin) => format!("Nothing was {}.", origin.participle()),
            (Err(reason), _) => reason,
        });
        placed
    }
}

impl FloatingPasteOrigin {
    fn verb(self) -> &'static str {
        match self {
            Self::Paste => "Paste",
            Self::Duplicate { .. } => "Duplicate",
        }
    }

    fn participle(self) -> &'static str {
        match self {
            Self::Paste => "pasted",
            Self::Duplicate { .. } => "duplicated",
        }
    }
}

impl FloatingPaste {
    /// Carry the copy with the pointer, now over `pointer` on the grid.
    pub(crate) fn follow(&mut self, pointer: Point) {
        match self.grab {
            Some(grab) => self.delta = pointer - grab,
            None => {
                self.start = pointer;
                self.grab = Some(pointer);
                self.delta = Point::origin();
            }
        }
    }

    /// Shift the copy by `step` without the pointer, as the arrow keys do.
    pub(crate) fn nudge(&mut self, step: Point) {
        self.start = self.start + step;
    }

    /// The drawing the placement would leave, if it was worked out for the
    /// copy as it now sits on the drawing as it now stands.
    pub(crate) fn current_preview(&self, content_version: u64) -> Option<&FloatingPastePreview> {
        self.preview
            .as_ref()
            .filter(|preview| preview.at == self.at() && preview.content_version == content_version)
    }

    pub(crate) fn remember_preview(
        &mut self,
        content_version: u64,
        diff: rspice_schematic_editor::view::preview::TransformCandidateDiff,
        result: Result<bool, String>,
    ) {
        self.preview = Some(FloatingPastePreview {
            at: self.at(),
            content_version,
            diff,
            result,
        });
    }
}

/// Let the floating copy go.
pub(crate) fn cancel_floating_paste(state: &mut AppState) -> bool {
    let Some(floating) = state.dialogs.floating_paste.take() else {
        return false;
    };
    state.prompt(format!("{} cancelled.", floating.origin.verb()));
    true
}

/// Turn or flip the floating copy about its middle. Returns whether a copy
/// was floating.
pub(crate) fn turn_floating_paste(state: &mut AppState, transform: SelectionTransform) -> bool {
    let Some(floating) = state.dialogs.floating_paste.as_mut() else {
        return false;
    };
    transform_clipboard(&mut floating.objects, transform);
    floating.preview = None;
    true
}

/// Why the floating copy can no longer land where it was asked for, if it
/// cannot: it was bound to one cell view and one sheet.
pub(crate) fn floating_paste_scope(state: &AppState) -> Result<(), String> {
    match state.dialogs.floating_paste.as_ref() {
        Some(floating) => floating_scope(state, floating),
        None => Ok(()),
    }
}

fn floating_scope(state: &AppState, floating: &FloatingPaste) -> Result<(), String> {
    if state.schematic_edit_read_only() {
        return Err("the schematic is read-only".to_owned());
    }
    if floating.view_path != state.workspace.content.active_view.display_path() {
        return Err("another cell view is open".to_owned());
    }
    if floating.sheet != active_sheet(state) {
        return Err("another sheet is open".to_owned());
    }
    Ok(())
}

fn active_sheet(state: &AppState) -> Option<crate::state::SheetId> {
    let key = state.workspace.content.active_schematic_reference().key();
    state
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        .and_then(|catalog| catalog.active_sheet_id())
}

/// `point` on the nearest point of a `grid` pitch.
pub(crate) fn on_grid(point: Point, grid: i32) -> Point {
    let grid = grid.max(1);
    let snap = |value: i32| {
        let value = f64::from(value) / f64::from(grid);
        (value.round() as i32).saturating_mul(grid)
    };
    Point::new(snap(point.x), snap(point.y))
}
