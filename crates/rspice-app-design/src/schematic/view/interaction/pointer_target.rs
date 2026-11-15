//! Compose visible app-owned design inputs for the editor's pointer queries.

use super::super::parts::CanvasParts;
use super::super::{SchematicSymbolContext, viewport::Viewport};
pub(in crate::schematic::view) use pointer_target::{PointerHit, PointerTarget};
use rspice_schematic_editor::view::pointer_target::{self, PointerQuery};

pub(in crate::schematic::view) fn pointer_target(
    state: &CanvasParts<'_>,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) -> Option<PointerTarget> {
    pointer_target_with_filter(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
        state.design.schematic_selection_filter,
    )
}

pub(super) fn pointer_target_with_filter(
    state: &CanvasParts<'_>,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
    filter: crate::state::SchematicSelectionFilter,
) -> Option<PointerTarget> {
    pointer_target::pointer_target(
        &state.design_view(),
        PointerQuery {
            hit,
            radius: hit_radius,
            position: pointer_pos,
        },
        filter,
        symbol_context,
        ctx,
        viewport,
        || state.model.workspace.content.active_view.display_path(),
    )
}
