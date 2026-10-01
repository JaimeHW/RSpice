//! Resolve pointer hits against the app's active-sheet and visibility policy.

use super::super::design_notes::design_note_at;
use super::super::documentation_shapes::documentation_shape_at;
use super::super::drawing::{bus_tap_at, nearest_bus_hit, probe_at_screen};
use super::super::net_labels::net_label_at;
use super::super::scene::visible_design_notes;
use super::super::sheet_visibility::{active_junction_at, active_wire_at, objects_on_active_sheet};
use super::super::{SchematicSymbolContext, viewport::Viewport};
use crate::state::Point;
use crate::workbench::app_state::AppState;
use rspice_design::schematic::design_note::DesignNoteRenderContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::schematic::view) enum PointerTarget {
    Component(u64),
    DesignNote(u64),
    DocumentationShape(u64),
    Probe(u64),
    NetLabel(u64),
    BusTap(u64),
    Junction(Point),
    Bus(u64),
    Wire(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::schematic::view) struct PointerHit {
    pub(in crate::schematic::view) grid: Point,
    pub(in crate::schematic::view) schematic: Point,
}

impl PointerHit {
    pub(in crate::schematic::view) const fn new(grid: Point, schematic: Point) -> Self {
        Self { grid, schematic }
    }
}

pub(in crate::schematic::view) fn pointer_target(
    state: &AppState,
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
        state.ui.schematic_selection_filter,
    )
}

pub(super) fn pointer_target_with_filter(
    state: &AppState,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
    filter: crate::state::SchematicSelectionFilter,
) -> Option<PointerTarget> {
    let notes = visible_design_notes(state);
    let labels = objects_on_active_sheet(state, &state.schematic.document().net_labels, |item| {
        item.id
    });
    let components =
        objects_on_active_sheet(state, &state.schematic.document().components, |item| {
            item.id
        });
    let taps = objects_on_active_sheet(state, &state.schematic.document().bus_taps, |item| item.id);
    let buses = objects_on_active_sheet(state, &state.schematic.document().buses, |item| item.id);
    let shapes = objects_on_active_sheet(
        state,
        &state.schematic.document().documentation_shapes,
        |item| item.id,
    );
    let probes = objects_on_active_sheet(state, &state.schematic.document().probes, |item| item.id);
    if filter.annotations
        && let Some(id) = probe_at_screen(viewport, probes.as_ref(), pointer_pos)
    {
        return Some(PointerTarget::Probe(id));
    }
    if filter.annotations
        && let Some(id) = design_note_at(
            ctx,
            viewport,
            notes.as_ref(),
            &DesignNoteRenderContext::for_document(
                state.schematic.document(),
                &state.workspace.content.active_view.display_path(),
            ),
            pointer_pos,
        )
    {
        return Some(PointerTarget::DesignNote(id));
    }
    if filter.labels
        && let Some(id) = net_label_at(ctx, viewport, labels.as_ref(), pointer_pos)
    {
        return Some(PointerTarget::NetLabel(id));
    }
    if filter.instances
        && let Some(id) = symbol_context.component_at_resolved_symbol(components.as_ref(), hit.grid)
    {
        return Some(PointerTarget::Component(id));
    }
    if filter.wires {
        if let Some(id) = bus_tap_at(taps.as_ref(), hit.schematic, hit_radius) {
            return Some(PointerTarget::BusTap(id));
        }
        if active_junction_at(state, hit.grid).is_some() {
            return Some(PointerTarget::Junction(hit.grid));
        }
        if let Some(hit) = nearest_bus_hit(buses.as_ref(), hit.schematic, hit_radius) {
            return Some(PointerTarget::Bus(hit.bus_id));
        }
        if let Some(id) = active_wire_at(state, hit.grid) {
            return Some(PointerTarget::Wire(id));
        }
    }
    if filter.annotations
        && let Some(id) = documentation_shape_at(viewport, shapes.as_ref(), pointer_pos)
    {
        return Some(PointerTarget::DocumentationShape(id));
    }
    None
}
