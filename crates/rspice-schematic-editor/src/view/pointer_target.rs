//! Ordered pointer targeting over the visible schematic design.

use super::design_notes::design_note_at;
use super::documentation_shapes::documentation_shape_at;
use super::drawing::{
    bus_tap_at, conductor_width_pt, nearest_bus_hit, nearest_wire_screen_hit, probe_at_screen,
};
use super::lod;
use super::net_labels::net_label_at;
use super::{design_view::DesignView, symbol_context::SchematicSymbolContext, viewport::Viewport};
use crate::session::selection::SchematicSelectionFilter;
use rspice_design::schematic::{design_note::DesignNoteRenderContext, wire::Wire};
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
        let slop = pick_slop(ctx);
        if let Some(position) = junction_at_screen(view, viewport, pointer_pos, slop) {
            return Some(PointerTarget::Junction(position));
        }
        if let Some(hit) = nearest_bus_hit(buses.as_ref(), hit.schematic, hit_radius) {
            return Some(PointerTarget::Bus(hit.bus_id));
        }
        if let Some(id) = wire_at_screen(view, viewport, pointer_pos, slop) {
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

/// How far outside a wire or a junction dot a click still takes it, in
/// points: a mouse is precise, a finger is not.
pub const MOUSE_PICK_SLOP: f32 = 2.5;
pub const TOUCH_PICK_SLOP: f32 = 12.0;

/// Whether the canvas is being driven by touch: the shell is laid out for a
/// finger, or the last pointer input was a touch. The latch holds until a
/// mouse moves with no touch and no button down.
pub fn canvas_touch_mode(ctx: &egui::Context) -> bool {
    let id = egui::Id::new("rspice.schematic.canvas_touch_latch");
    let (touched, hovered_by_mouse) = ctx.input(|input| {
        let touched = input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Touch { .. }));
        let moved = input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::PointerMoved(_)));
        (touched, moved && !touched && !input.pointer.any_down())
    });
    let latched = ctx.data_mut(|data| {
        let latch = data.get_temp_mut_or_default::<bool>(id);
        if touched {
            *latch = true;
        } else if hovered_by_mouse {
            *latch = false;
        }
        *latch
    });
    latched || rspice_ui_kit::tokens::Tokens::get(ctx).metrics.is_touch()
}

/// The pick slop for the current input, in points.
pub fn pick_slop(ctx: &egui::Context) -> f32 {
    if canvas_touch_mode(ctx) {
        TOUCH_PICK_SLOP
    } else {
        MOUSE_PICK_SLOP
    }
}

/// The visible junction whose dot, grown by `slop` points, holds the pointer;
/// the nearest when several do.
pub fn junction_at_screen(
    view: &DesignView<'_>,
    viewport: &Viewport,
    pointer: egui::Pos2,
    slop: f32,
) -> Option<Point> {
    let wire = conductor_width_pt(viewport.zoom, 1.0, false);
    let reach = lod::junction_radius_pt(viewport.zoom, wire) + slop;
    view.document
        .junctions
        .iter()
        .filter(|junction| view.object_is_visible(junction.id))
        .map(|junction| {
            let distance = viewport.schematic_to_screen(junction.pos).distance(pointer);
            (distance, junction.pos)
        })
        .filter(|(distance, _)| *distance <= reach)
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .map(|(_, position)| position)
}

/// The visible wire whose drawn stroke, grown by `slop` points, holds the
/// pointer; the nearest when several do. Candidates come from the canvas
/// cache's spatial bins when it is fresh.
pub fn wire_at_screen(
    view: &DesignView<'_>,
    viewport: &Viewport,
    pointer: egui::Pos2,
    slop: f32,
) -> Option<u64> {
    let reach = conductor_width_pt(viewport.zoom, 1.0, false) * 0.5 + slop;
    let world = super::coordinates::screen_to_schematic(viewport, pointer);
    let margin = reach / viewport.zoom.max(f32::EPSILON) + 1.0;
    let candidates: Vec<Wire> = match view.canvas_cache {
        Some(cache) => cache
            .wire_indices_in_world_rect(
                world.x as f32 - margin,
                world.y as f32 - margin,
                world.x as f32 + margin,
                world.y as f32 + margin,
            )
            .into_iter()
            .filter_map(|index| view.document.wires.get(index))
            .filter(|wire| view.object_is_visible(wire.id))
            .cloned()
            .collect(),
        None => view
            .objects_on_active_sheet(&view.document.wires, |wire| wire.id)
            .into_owned(),
    };
    nearest_wire_screen_hit(viewport, &candidates, pointer, reach, None).map(|hit| hit.wire_id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::visibility::SchematicReviewMarkerVisibility;
    use rspice_design::schematic::{
        component::Component, component_type::ComponentType, document::SchematicDocument,
        net_label::Junction,
    };

    fn view(document: &SchematicDocument) -> DesignView<'_> {
        DesignView {
            document,
            canvas_cache: None,
            sheet_catalog: None,
            review_markers: SchematicReviewMarkerVisibility::All,
        }
    }

    fn viewport(zoom: f32) -> Viewport {
        Viewport {
            offset: egui::Pos2::new(100.0, 100.0),
            zoom,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(800.0)),
        }
    }

    /// What a click resolves to, with the grid point the selection query
    /// would hand in: snapped to the 10-wu grid, or the raw point.
    fn click(
        document: &SchematicDocument,
        viewport: &Viewport,
        pointer: egui::Pos2,
        snapped: bool,
    ) -> Option<PointerTarget> {
        let raw = super::super::coordinates::screen_to_schematic(viewport, pointer);
        let grid = if snapped {
            Point::new(
                (raw.x as f32 / 10.0).round() as i32 * 10,
                (raw.y as f32 / 10.0).round() as i32 * 10,
            )
        } else {
            raw
        };
        pointer_target(
            &view(document),
            PointerQuery {
                hit: PointerHit::new(grid, raw),
                radius: (6.0 / viewport.zoom).ceil() as i32,
                position: pointer,
            },
            SchematicSelectionFilter::default(),
            &SchematicSymbolContext::default(),
            &egui::Context::default(),
            viewport,
            String::new,
        )
    }

    fn wire_document() -> SchematicDocument {
        SchematicDocument {
            wires: vec![Wire::segment(7, Point::new(0, 0), Point::new(200, 0))],
            ..SchematicDocument::default()
        }
    }

    /// A wire is taken within a few points of its stroke at every zoom, and
    /// not from half a grid away: at 800% half a grid is 40 px, and with
    /// snapping off it was half a world unit.
    #[test]
    fn a_wire_is_picked_within_a_screen_radius_at_every_zoom() {
        let document = wire_document();
        let zoomed_in = viewport(8.0);
        let on = zoomed_in.schematic_to_screen(Point::new(100, 0));
        assert_eq!(
            click(&document, &zoomed_in, on + egui::vec2(0.0, 3.0), true),
            Some(PointerTarget::Wire(7))
        );
        assert_eq!(
            click(&document, &zoomed_in, on + egui::vec2(0.0, 30.0), true),
            None,
            "30 px below a wire at 800% is not on it"
        );

        let unsnapped = viewport(1.0);
        let on = unsnapped.schematic_to_screen(Point::new(100, 0));
        assert_eq!(
            click(&document, &unsnapped, on + egui::vec2(0.0, 2.0), false),
            Some(PointerTarget::Wire(7)),
            "2 px off a wire with snapping off is on it"
        );
        let fit = viewport(0.25);
        let on = fit.schematic_to_screen(Point::new(100, 0));
        assert_eq!(
            click(&document, &fit, on + egui::vec2(0.0, 2.5), false),
            Some(PointerTarget::Wire(7))
        );
    }

    /// A junction dot is taken within the pick slop of its drawn edge,
    /// whether or not the pointer rounds to its grid point.
    #[test]
    fn a_junction_is_picked_within_a_screen_radius() {
        let mut document = wire_document();
        document
            .wires
            .push(Wire::segment(8, Point::new(100, 0), Point::new(100, 60)));
        document
            .junctions
            .push(Junction::new(9, Point::new(100, 0)));
        let zoom = viewport(1.0);
        let dot = zoom.schematic_to_screen(Point::new(100, 0));
        assert_eq!(
            click(&document, &zoom, dot + egui::vec2(3.0, 2.0), false),
            Some(PointerTarget::Junction(Point::new(100, 0)))
        );
        let zoomed_in = viewport(8.0);
        let dot = zoomed_in.schematic_to_screen(Point::new(100, 0));
        assert_eq!(
            click(&document, &zoomed_in, dot + egui::vec2(20.0, 30.0), true),
            None,
            "36 px from a junction at 800% does not take it"
        );
    }

    /// Instances paint in document order, so the later of two overlapping
    /// instances is drawn on top, and a click takes it.
    #[test]
    fn overlapping_instances_pick_the_topmost() {
        let mut document = SchematicDocument::default();
        document.components = vec![
            Component::new(1, ComponentType::Resistor, Point::new(0, 0)),
            Component::new(2, ComponentType::Resistor, Point::new(0, 0)),
        ];
        let zoom = viewport(1.0);
        assert_eq!(
            click(
                &document,
                &zoom,
                zoom.schematic_to_screen(Point::new(0, 0)),
                true
            ),
            Some(PointerTarget::Component(2))
        );
    }
}
