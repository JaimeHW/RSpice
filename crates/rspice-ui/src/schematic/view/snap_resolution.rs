//! Pointer-to-schematic snap resolution.
//!
//! Placement and editing gestures must enter the snap system as raw schematic
//! coordinates. Pre-quantizing the pointer makes the `Free` grid mode
//! impossible and makes electrical target acquisition vary with zoom.

use egui::Pos2;

use crate::state::SnapResult;
use crate::workbench::app_state::AppState;

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::sheet_visibility::objects_on_active_sheet;
use super::viewport::Viewport;

/// Electrical targets are acquired within this stable on-screen radius.
const TARGET_ACQUISITION_RADIUS_POINTS: f32 = 6.0;

/// Convert the stable device-space acquisition radius to schematic units.
///
/// `ceil` deliberately includes targets whose rendered distance is exactly the
/// contract radius. A one-unit floor keeps acquisition usable at extreme zoom.
pub(super) fn target_acquisition_radius(viewport: &Viewport) -> i32 {
    (TARGET_ACQUISITION_RADIUS_POINTS / viewport.zoom.max(0.1))
        .ceil()
        .max(1.0) as i32
}

/// Resolve a pointer for grid-governed placement and geometry editing.
///
/// The master snap switch is authoritative: when disabled, the raw integer
/// schematic coordinate is returned. Grid intersections are independently
/// optional, which is how the `Free` mode remains genuinely unsnapped while
/// preserving the existing exact grid projection when enabled.
pub(super) fn resolve_grid_pointer(
    state: &AppState,
    viewport: &Viewport,
    screen_position: Pos2,
) -> SnapResult {
    let raw = screen_to_schematic(viewport, screen_position);
    if !state.schematic.snap_engine.enabled || !state.schematic.snap_engine.snap_to_grid {
        return SnapResult::no_snap(raw);
    }

    SnapResult::grid_only(
        screen_to_grid(viewport, state.schematic.grid_size, screen_position),
        raw,
    )
}

/// Resolve a pointer against active-sheet electrical targets, then fall back
/// to the current grid/free policy.
pub(super) fn resolve_target_pointer(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    screen_position: Pos2,
) -> SnapResult {
    let raw = screen_to_schematic(viewport, screen_position);
    if !state.schematic.snap_engine.enabled {
        return SnapResult::no_snap(raw);
    }

    let components = objects_on_active_sheet(state, &state.schematic.components, |item| item.id);
    let wires = objects_on_active_sheet(state, &state.schematic.wires, |item| item.id);
    let junctions = objects_on_active_sheet(state, &state.schematic.junctions, |item| item.id);
    let mut target_engine = state.schematic.snap_engine.clone();
    target_engine.snap_radius = target_acquisition_radius(viewport);
    // A non-electrical grid fallback is resolved from the original screen
    // coordinate below. This keeps its rounding contract identical to
    // `screen_to_grid`, including negative coordinates, while preserving the
    // configured grid/free behavior used for wire-segment acquisition.
    let target = target_engine.find_snap_target_resolved(
        raw,
        components.as_ref(),
        wires.as_ref(),
        junctions.as_ref(),
        |component| symbol_context.resolved_symbol(component),
    );
    if target.show_indicator {
        target
    } else {
        resolve_grid_pointer(state, viewport, screen_position)
    }
}

#[cfg(test)]
mod tests {
    use egui::{Pos2, Rect, Vec2};

    use super::*;
    use crate::state::{Junction, Point, SnapEngine, SnapTargetType};

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
        let mut state = AppState::default();
        state.schematic.grid_size = 10;

        state.schematic.snap_engine.enabled = false;
        let disabled = resolve_grid_pointer(&state, &viewport, pointer);
        assert_eq!(disabled.original_position, Point::new(17, 23));
        assert_eq!(disabled.snapped_position, Point::new(17, 23));

        state.schematic.snap_engine.enabled = true;
        state.schematic.snap_engine.snap_to_grid = false;
        let free = resolve_grid_pointer(&state, &viewport, pointer);
        assert_eq!(free.original_position, Point::new(17, 23));
        assert_eq!(free.snapped_position, Point::new(17, 23));

        state.schematic.snap_engine.snap_to_grid = true;
        let grid = resolve_grid_pointer(&state, &viewport, pointer);
        assert_eq!(grid.snapped_position, Point::new(20, 20));
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
