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
use crate::state::{DuplicateExternalNets, Point, Tool};
use crate::workbench::app::dialogs::selection::array::{
    ArraySelectionDialogState, cancel_armed_array_selection_in,
};
use crate::workbench::app::dialogs::selection::move_selection::{
    MoveSelectionDialogState, cancel_armed_move_selection_in,
};
use crate::workbench::app::dialogs::selection::stretch::{
    StretchSelectionDialogState, cancel_armed_stretch_selection_in,
};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

/// A Paste or Duplicate floating under the pointer until a click places it.
///
/// The drawing does not change while it floats: the copy is held here, bound
/// to the cell view it was asked for in and turned as R and M asked, and the
/// click pastes it there as one undo step.
#[derive(Debug, Clone)]
pub(crate) struct FloatingPaste {
    pub(crate) objects: rspice_design::schematic::clipboard::ClipboardData,
    /// Where the copy's middle sat when the pointer took hold of it.
    pub(crate) start: crate::state::Point,
    /// The grid point the pointer took hold at; until the pointer is over
    /// the canvas, nothing has, and the copy jumps to it when it arrives.
    pub(crate) grab: Option<crate::state::Point>,
    /// How far the pointer has carried the copy since.
    pub(crate) delta: crate::state::Point,
    /// The cell view and sheet the copy was bound to.
    pub(crate) view_path: String,
    pub(crate) sheet: Option<crate::state::SheetId>,
    pub(crate) origin: FloatingPasteOrigin,
    /// The drawing the placement would leave, for the position, content and
    /// orientation it was worked out for.
    pub(crate) preview: Option<FloatingPastePreview>,
}

impl FloatingPaste {
    /// Where the copy's middle sits now, on the document grid.
    pub(crate) fn at(&self) -> crate::state::Point {
        self.start + self.delta
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FloatingPastePreview {
    pub(crate) at: crate::state::Point,
    pub(crate) content_version: u64,
    /// What the placement would change, taken once when the preview is
    /// worked out rather than searched for on every repaint.
    pub(crate) diff: rspice_schematic_editor::view::preview::TransformCandidateDiff,
    pub(crate) result: Result<bool, String>,
}

/// What floated, for the line the placement answers with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatingPasteOrigin {
    Paste,
    Duplicate {
        count: usize,
        kept: usize,
        unresolved_nets: bool,
    },
}

/// What a floating Paste or Duplicate reads and writes of the design
/// feature's state: the copy it holds, the armed commands it stops, where
/// the pointer is, the clipboard Paste floats and how Duplicate treats the
/// nets it leaves.
pub(crate) struct FloatingCopyParts<'a> {
    pub(crate) floating_paste: &'a mut Option<FloatingPaste>,
    pub(crate) move_selection: &'a mut MoveSelectionDialogState,
    pub(crate) stretch_selection: &'a mut StretchSelectionDialogState,
    pub(crate) array_selection: &'a mut ArraySelectionDialogState,
    pub(crate) canvas_hover: Option<(f64, f64)>,
    pub(crate) canvas_view_center: Option<(f64, f64)>,
    pub(crate) clipboard: &'a ClipboardData,
    pub(crate) duplicate_external_nets: DuplicateExternalNets,
}

impl FloatingCopyParts<'_> {
    /// [`schematic_paste_anchor`] for this pointer.
    pub(crate) fn paste_anchor(&self, model: &Model) -> Point {
        schematic_paste_anchor(self.canvas_hover, self.canvas_view_center, model)
    }
}

/// Where a paste should land, snapped to the schematic grid: under the
/// cursor when it hovers the canvas, otherwise the center of the visible
/// canvas (menu-driven paste), otherwise a sane fixed spot.
pub(crate) fn schematic_paste_anchor(
    canvas_hover: Option<(f64, f64)>,
    canvas_view_center: Option<(f64, f64)>,
    model: &Model,
) -> Point {
    let grid = model.schematic.document().grid_size.max(1);
    let (x, y) = canvas_hover.or(canvas_view_center).unwrap_or((20.0, 20.0));
    Point::new((x.round() as i32) * grid, (y.round() as i32) * grid)
}

/// Float `objects` with their middle at `anchor`. The pointer carries
/// the copy from there; a copy asked for with the pointer off the canvas
/// jumps to the pointer when it arrives.
pub(super) fn float_schematic_objects(
    design: &mut FloatingCopyParts<'_>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    objects: &ClipboardData,
    anchor: Point,
    origin: FloatingPasteOrigin,
) -> Result<(), String> {
    let objects = bound_to_active_cell(model, objects)?.into_owned();
    // A copy floating is the command now running; the one before it
    // stops, as a new command stops the last in every schematic editor.
    if design.move_selection.armed {
        cancel_armed_move_selection_in(design.move_selection, model);
    }
    if design.stretch_selection.armed {
        cancel_armed_stretch_selection_in(design.stretch_selection, model);
    }
    if design.array_selection.armed {
        cancel_armed_array_selection_in(design.array_selection, model);
    }
    if model.schematic.session.editor.tool != Tool::Select {
        model.schematic.cancel_tool();
    }
    let grid = model.schematic.document().grid_size;
    let count = objects.count();
    *design.floating_paste = Some(FloatingPaste {
        objects,
        start: on_grid(anchor, grid),
        grab: design
            .canvas_hover
            .map(|_| on_grid(design.paste_anchor(model), grid)),
        delta: Point::origin(),
        view_path: model.workspace.content.active_view.display_path(),
        sheet: active_sheet(model),
        origin,
        preview: None,
    });
    wb.prompt(format!(
        "Click to place {}. R rotates, M mirrors, Esc cancels.",
        object_count(count)
    ));
    Ok(())
}

/// Land the floating copy where it sits now. Returns whether anything was
/// placed.
pub(crate) fn place_floating_paste(
    floating: &mut Option<FloatingPaste>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
) -> bool {
    let Some(floating) = floating.take() else {
        return false;
    };
    if let Err(reason) = floating_scope(model, &floating) {
        wb.prompt(format!("{} cancelled: {reason}.", floating.origin.verb()));
        return false;
    }
    let pasted = super::paste_schematic_objects_at(model, wb, &floating.objects, floating.at());
    let placed = matches!(pasted, Ok(true));
    wb.prompt(match (pasted, floating.origin) {
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

/// Let the floating copy go, saying so on `wb`'s status bar. Returns whether
/// a copy was floating.
pub(crate) fn cancel_floating_paste_in(
    floating: &mut Option<FloatingPaste>,
    wb: &mut WorkbenchSession,
) -> bool {
    let Some(floating) = floating.take() else {
        return false;
    };
    wb.prompt(format!("{} cancelled.", floating.origin.verb()));
    true
}

/// Turn or flip the floating copy about its middle. Returns whether a copy
/// was floating.
pub(crate) fn turn_floating_paste(
    floating: &mut Option<FloatingPaste>,
    transform: SelectionTransform,
) -> bool {
    let Some(floating) = floating.as_mut() else {
        return false;
    };
    transform_clipboard(&mut floating.objects, transform);
    floating.preview = None;
    true
}

/// Why the floating copy can no longer land where it was asked for, if it
/// cannot: it was bound to one cell view and one sheet.
pub(crate) fn floating_paste_scope(
    floating: &Option<FloatingPaste>,
    model: &Model,
) -> Result<(), String> {
    match floating.as_ref() {
        Some(floating) => floating_scope(model, floating),
        None => Ok(()),
    }
}

fn floating_scope(model: &Model, floating: &FloatingPaste) -> Result<(), String> {
    if model.schematic_edit_read_only() {
        return Err("the schematic is read-only".to_owned());
    }
    if floating.view_path != model.workspace.content.active_view.display_path() {
        return Err("another cell view is open".to_owned());
    }
    if floating.sheet != active_sheet(model) {
        return Err("another sheet is open".to_owned());
    }
    Ok(())
}

fn active_sheet(model: &Model) -> Option<crate::state::SheetId> {
    let key = model.workspace.content.active_schematic_reference().key();
    model
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
