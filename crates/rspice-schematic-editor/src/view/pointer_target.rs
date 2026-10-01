//! Ordered pointer targeting over the visible schematic design.

use super::design_notes::design_note_at;
use super::documentation_shapes::documentation_shape_at;
use super::drawing::{bus_tap_at, nearest_bus_hit, probe_at_screen};
use super::net_labels::net_label_at;
use super::{design_view::DesignView, symbol_context::SchematicSymbolContext, viewport::Viewport};
use crate::session::selection::SchematicSelectionFilter;
use rspice_design::schematic::design_note::DesignNoteRenderContext;
use rspice_design_model::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerTarget {
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
pub struct PointerHit {
    pub grid: Point,
    pub schematic: Point,
}

impl PointerHit {
    pub const fn new(grid: Point, schematic: Point) -> Self {
        Self { grid, schematic }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointerQuery {
    pub hit: PointerHit,
    pub radius: i32,
    pub position: egui::Pos2,
}

pub fn pointer_target(
    view: &DesignView<'_>,
    query: PointerQuery,
    filter: SchematicSelectionFilter,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    view_path: impl FnOnce() -> String,
) -> Option<PointerTarget> {
    let PointerQuery {
        hit,
        radius: hit_radius,
        position: pointer_pos,
    } = query;
    let notes = view.visible_design_notes();
    let labels = view.objects_on_active_sheet(&view.document.net_labels, |item| item.id);
    let components = view.objects_on_active_sheet(&view.document.components, |item| item.id);
    let taps = view.objects_on_active_sheet(&view.document.bus_taps, |item| item.id);
    let buses = view.objects_on_active_sheet(&view.document.buses, |item| item.id);
    let shapes = view.objects_on_active_sheet(&view.document.documentation_shapes, |item| item.id);
    let probes = view.objects_on_active_sheet(&view.document.probes, |item| item.id);
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
            &DesignNoteRenderContext::for_document(view.document, &view_path()),
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
        if view.active_junction_at(hit.grid).is_some() {
            return Some(PointerTarget::Junction(hit.grid));
        }
        if let Some(hit) = nearest_bus_hit(buses.as_ref(), hit.schematic, hit_radius) {
            return Some(PointerTarget::Bus(hit.bus_id));
        }
        if let Some(id) = view.active_wire_at(hit.grid) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerSelectionEffect {
    None,
    HighlightWire(u64),
}

/// Apply local selection; return any net-highlighting request to the app.
pub fn select_pointer_target(
    session: &mut crate::session::EditorSession,
    target: Option<PointerTarget>,
    additive: bool,
    alt_held: bool,
) -> PointerSelectionEffect {
    match target {
        Some(PointerTarget::Component(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_component(id);
            } else {
                session.selection.clear();
                session.selection.select_component(id);
            }
        }
        Some(PointerTarget::DesignNote(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_design_note(id);
            } else {
                session.selection.select_only_design_note(id);
            }
        }
        Some(PointerTarget::DocumentationShape(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_documentation_shape(id);
            } else {
                session.selection.select_only_documentation_shape(id);
            }
        }
        Some(PointerTarget::Probe(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_probe(id);
            } else {
                session.selection.select_only_probe(id);
            }
        }
        Some(PointerTarget::NetLabel(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_net_label(id);
            } else {
                session.selection.select_only_net_label(id);
            }
        }
        Some(PointerTarget::BusTap(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_bus_tap(id);
            } else {
                session.selection.select_only_bus_tap(id);
            }
        }
        Some(PointerTarget::Junction(pos)) => {
            session.net_highlight.clear();
            if additive {
                if session.selection.has_junction(pos) {
                    session.selection.deselect_junction(pos);
                } else {
                    session.selection.select_junction(pos);
                }
            } else {
                session.selection.select_only_junction(pos);
            }
        }
        Some(PointerTarget::Bus(id)) => {
            session.net_highlight.clear();
            if additive {
                session.selection.toggle_bus(id);
            } else {
                session.selection.select_only_bus(id);
            }
        }
        Some(PointerTarget::Wire(id)) => {
            if alt_held {
                session.selection.clear();
                return PointerSelectionEffect::HighlightWire(id);
            } else if additive {
                session.net_highlight.clear();
                session.selection.toggle_wire(id);
            } else {
                session.net_highlight.clear();
                session.selection.clear();
                session.selection.select_wire(id);
            }
        }
        None if !additive => {
            session.selection.clear();
            session.net_highlight.clear();
        }
        None => {}
    }
    PointerSelectionEffect::None
}
