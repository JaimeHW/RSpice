//! Pointer-to-schematic snap resolution.
//!
//! Placement and editing gestures must enter the snap system as raw schematic
//! coordinates. Pre-quantizing the pointer makes the `Free` grid mode
//! impossible and makes electrical target acquisition vary with zoom.

use egui::Pos2;

use super::design_view::DesignView;
use super::drawing::{WireScreenHit, nearest_wire_screen_hit};
use crate::session::snap::{SnapEngine, SnapResult};

use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::symbol_context::SchematicSymbolContext;
use super::viewport::Viewport;
use rspice_design::schematic::wire::Wire;
use rspice_design_model::Point;

/// Electrical targets are acquired within this stable on-screen radius.
const TARGET_ACQUISITION_RADIUS_POINTS: f32 = 6.0;

/// Convert the stable device-space acquisition radius to schematic units.
///
/// `ceil` deliberately includes targets whose rendered distance is exactly the
/// contract radius. A one-unit floor keeps acquisition usable at extreme zoom.
pub fn target_acquisition_radius(viewport: &Viewport) -> i32 {
    (TARGET_ACQUISITION_RADIUS_POINTS / viewport.zoom.max(0.1))
        .ceil()
        .max(1.0) as i32
}

/// The pitch a visual conductor attachment is quantized to along the
/// conductor, or `None` when the pointer's exact schematic coordinate is the
/// contract: snapping disabled, or the `Free` grid mode. Only the coordinate
/// along the conductor is quantized; the conductor's own position fixes the
/// other one, so the attachment always stays on its body.
pub fn conductor_attachment_pitch(engine: &SnapEngine, grid_size: i32) -> Option<i32> {
    (engine.enabled && engine.snap_to_grid).then_some(grid_size)
}

/// Resolve a visual conductor acquisition using the same active-sheet order,
/// screen-space radius and attachment pitch for route clicks and previews.
pub fn nearest_active_wire_screen_hit(
    view: &DesignView<'_>,
    engine: &SnapEngine,
    viewport: &Viewport,
    pointer: Pos2,
) -> Option<WireScreenHit> {
    let wires = view.objects_on_active_sheet(&view.document.wires, |wire| wire.id);
    nearest_wire_screen_hit(
        viewport,
        wires.as_ref(),
        pointer,
        TARGET_ACQUISITION_RADIUS_POINTS,
        conductor_attachment_pitch(engine, view.document.grid_size),
    )
}

/// Resolve a pointer for grid-governed placement and geometry editing.
///
/// The master snap switch is authoritative: when disabled, the raw integer
/// schematic coordinate is returned. Grid intersections are independently
/// optional, which is how the `Free` mode remains genuinely unsnapped while
/// preserving the existing exact grid projection when enabled.
pub fn resolve_grid_pointer(
    engine: &SnapEngine,
    grid_size: i32,
    viewport: &Viewport,
    screen_position: Pos2,
) -> SnapResult {
    let raw = screen_to_schematic(viewport, screen_position);
    if !engine.enabled || !engine.snap_to_grid {
        return SnapResult::no_snap(raw);
    }

    SnapResult::grid_only(screen_to_grid(viewport, grid_size, screen_position), raw)
}

/// The square around the pointer that holds every point within the
/// acquisition radius of it.
#[derive(Clone, Copy)]
struct AcquisitionBox {
    min: Point,
    max: Point,
}

impl AcquisitionBox {
    fn around(centre: Point, radius: i32) -> Self {
        Self {
            min: Point::new(
                centre.x.saturating_sub(radius),
                centre.y.saturating_sub(radius),
            ),
            max: Point::new(
                centre.x.saturating_add(radius),
                centre.y.saturating_add(radius),
            ),
        }
    }

    fn contains(self, point: Point) -> bool {
        (self.min.x..=self.max.x).contains(&point.x) && (self.min.y..=self.max.y).contains(&point.y)
    }

    fn overlaps(self, min: Point, max: Point) -> bool {
        min.x <= self.max.x && max.x >= self.min.x && min.y <= self.max.y && max.y >= self.min.y
    }
}

/// The active sheet's wires whose extent reaches into `reach`, in document
/// order, read from the canvas cache's spatial bins when it is current.
fn wires_within(view: &DesignView<'_>, reach: AcquisitionBox) -> Vec<Wire> {
    let wire_reaches = |wire: &Wire| {
        let mut points = wire.points.iter();
        let Some(first) = points.next() else {
            return false;
        };
        let (min, max) = points.fold((*first, *first), |(min, max), point| {
            (
                Point::new(min.x.min(point.x), min.y.min(point.y)),
                Point::new(max.x.max(point.x), max.y.max(point.y)),
            )
        });
        reach.overlaps(min, max)
    };
    let candidates = view.canvas_cache.map_or_else(
        || (0..view.document.wires.len()).collect(),
        |cache| {
            cache.wire_indices_in_world_rect(
                reach.min.x as f32,
                reach.min.y as f32,
                reach.max.x as f32,
                reach.max.y as f32,
            )
        },
    );
    candidates
        .into_iter()
        .filter_map(|index| view.document.wires.get(index))
        .filter(|wire| view.object_is_visible(wire.id) && wire_reaches(wire))
        .cloned()
        .collect()
}

/// Resolve a pointer against active-sheet electrical targets, then fall back
/// to the current grid/free policy.
pub fn resolve_target_pointer(
    view: &DesignView<'_>,
    engine: &SnapEngine,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    screen_position: Pos2,
) -> SnapResult {
    let raw = screen_to_schematic(viewport, screen_position);
    if !engine.enabled {
        return SnapResult::no_snap(raw);
    }

    let radius = target_acquisition_radius(viewport);
    // Only an object with something inside the acquisition radius can be
    // acquired, so only those objects, in document order, are offered to the
    // snap engine. Offering it every object on the sheet, every hover frame,
    // made it build and name a target for every pin, junction and segment.
    let reach = AcquisitionBox::around(raw, radius);
    let components = view
        .document
        .components
        .iter()
        .filter(|component| view.object_is_visible(component.id))
        .filter(|component| {
            component
                .terminal_positions_resolved(symbol_context.resolved_symbol(component))
                .iter()
                .any(|(_, terminal)| reach.contains(*terminal))
        })
        .cloned()
        .collect::<Vec<_>>();
    let wires = wires_within(view, reach);
    let junctions = view
        .document
        .junctions
        .iter()
        .filter(|junction| reach.contains(junction.pos) && view.object_is_visible(junction.id))
        .cloned()
        .collect::<Vec<_>>();
    let mut target_engine = engine.clone();
    target_engine.snap_radius = radius;
    // A non-electrical grid fallback is resolved from the original screen
    // coordinate below. This keeps its rounding contract identical to
    // `screen_to_grid`, including negative coordinates, while preserving the
    // configured grid/free behavior used for wire-segment acquisition.
    let target = target_engine.find_snap_target_resolved(
        raw,
        &components,
        &wires,
        &junctions,
        |component| symbol_context.resolved_symbol(component),
    );
    if target.show_indicator {
        target
    } else {
        resolve_grid_pointer(engine, view.document.grid_size, viewport, screen_position)
    }
}

#[cfg(test)]
mod tests {
    use egui::{Pos2, Rect, Vec2};

    use super::*;
    use crate::session::snap::SnapTargetType;
    use rspice_design::schematic::net_label::Junction;
    use rspice_design_model::Point;

    fn viewport(zoom: f32) -> Viewport {
        Viewport {
            offset: Pos2::ZERO,
            zoom,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(400.0)),
        }
    }

    #[test]
    fn disabled_and_free_modes_preserve_raw_pointer_coordinates() {
        let viewport = viewport(1.0);
        let pointer = Pos2::new(17.0, 23.0);
        let mut engine = SnapEngine {
            enabled: false,
            ..SnapEngine::default()
        };
        let disabled = resolve_grid_pointer(&engine, 10, &viewport, pointer);
        assert_eq!(disabled.original_position, Point::new(17, 23));
        assert_eq!(disabled.snapped_position, Point::new(17, 23));

        engine.enabled = true;
        engine.snap_to_grid = false;
        let free = resolve_grid_pointer(&engine, 10, &viewport, pointer);
        assert_eq!(free.original_position, Point::new(17, 23));
        assert_eq!(free.snapped_position, Point::new(17, 23));

        engine.snap_to_grid = true;
        let grid = resolve_grid_pointer(&engine, 10, &viewport, pointer);
        assert_eq!(grid.snapped_position, Point::new(20, 20));
    }

    /// Offering the snap engine only what lies near the pointer acquires
    /// exactly what offering it the whole drawing does, cache or no cache.
    #[test]
    fn acquiring_from_nearby_objects_matches_acquiring_from_all_of_them() {
        use crate::session::EditorSession;
        use crate::session::visibility::SchematicReviewMarkerVisibility;
        use rspice_design::schematic::{
            component::Component, component_type::ComponentType, document::SchematicDocument,
        };

        let mut document = SchematicDocument::default();
        for index in 0..6 {
            let x = index * 70;
            document.components.push(Component::new(
                u64::try_from(index + 1).expect("small"),
                if index % 2 == 0 {
                    ComponentType::Resistor
                } else {
                    ComponentType::Nmos
                },
                Point::new(x, 0),
            ));
            document.wires.push(Wire::new(
                u64::try_from(index + 100).expect("small"),
                vec![
                    Point::new(x, 50),
                    Point::new(x + 40, 50),
                    Point::new(x + 40, 90),
                ],
            ));
            document.junctions.push(Junction::new(
                u64::try_from(index + 200).expect("small"),
                Point::new(x + 40, 70),
            ));
        }
        let mut session = EditorSession::default();
        session.ensure_canvas_cache(&document, 1);
        let symbols = SchematicSymbolContext::default();
        let engine = SnapEngine::default();
        let viewport = viewport(1.0);
        for canvas_cache in [None, session.canvas_cache(1)] {
            let view = DesignView {
                document: &document,
                canvas_cache,
                sheet_catalog: None,
                review_markers: SchematicReviewMarkerVisibility::All,
            };
            for x in (-10..440).step_by(7) {
                for y in (-50..110).step_by(7) {
                    let pointer = Pos2::new(x as f32, y as f32);
                    let mut whole = engine.clone();
                    whole.snap_radius = target_acquisition_radius(&viewport);
                    let expected = whole.find_snap_target_resolved(
                        screen_to_schematic(&viewport, pointer),
                        &document.components,
                        &document.wires,
                        &document.junctions,
                        |_| None,
                    );
                    let expected = if expected.show_indicator {
                        expected
                    } else {
                        resolve_grid_pointer(&engine, document.grid_size, &viewport, pointer)
                    };
                    assert_eq!(
                        resolve_target_pointer(&view, &engine, &symbols, &viewport, pointer),
                        expected,
                        "pointer at ({x}, {y})"
                    );
                }
            }
        }
    }

    #[test]
    fn electrical_target_acquisition_is_zoom_invariant_in_screen_points() {
        let engine = SnapEngine::default();
        let junctions = [Junction::new(1, Point::origin())];

        for (zoom, query) in [(1.0, Point::new(6, 0)), (2.0, Point::new(3, 0))] {
            let result = engine
                .clone()
                .with_radius(target_acquisition_radius(&viewport(zoom)))
                .find_snap_target(query, &[], &[], &junctions);
            assert_eq!(result.target_type(), Some(&SnapTargetType::Junction));
            assert_eq!(result.snapped_position, Point::origin());
        }
    }
}
