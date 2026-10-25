//! The drawing sheet on the schematic canvas: its frame, and the overflow
//! advisories the model holds for it, painted every frame.

use super::viewport::Viewport;
use super::{AppState, SchematicSymbolContext};
use egui::{Painter, Rect};
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_schematic_editor::view::drawing_sheet::{self as sheet_view, DrawingSheetCanvas};
pub(super) use rspice_schematic_editor::view::drawing_sheet::{
    ActiveDrawingSheet, FIT_SCREEN_INSET,
};

pub(super) fn draw_base(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) {
    sheet_view::draw_base(
        painter,
        available,
        viewport,
        DrawingSheetCanvas {
            layers: state.design.drawing_sheet_layers,
            grid: state.design.grid,
            grid_size: state.model.schematic.document().grid_size,
        },
        sheet,
        || state.model.title_block_fields(sheet),
    );
}

pub(super) fn draw_overflow_advisories(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) {
    if !sheet_view::overflow_advisories_shown(painter, viewport) {
        return;
    }
    let summary = state
        .model
        .held_overflow_summary(review_markers(state), symbol_context, sheet);
    sheet_view::draw_overflow_advisories(painter, available, viewport, &summary);
}

/// The review markers the drawing shows.
fn review_markers(state: &AppState) -> SchematicReviewMarkerVisibility {
    state.design.schematic_visibility.review_markers
}

/// Find the overflow advisories afresh only when the drawing or the sheet
/// changed, and only at a zoom that shows them. Runs once a frame, before
/// painting, where the session can hold them.
pub(super) fn refresh_overflow_summary(
    state: &mut AppState,
    painter: &Painter,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) {
    if sheet_view::overflow_advisories_shown(painter, viewport) {
        let review_markers = review_markers(state);
        state
            .model
            .hold_overflow_summary(review_markers, symbol_context, sheet);
    }
}

/// [`Model::overflow_summary`](crate::workbench::app_state::Model::overflow_summary)
/// through the drawing as `state` shows it.
#[cfg(test)]
pub(super) fn drawing_sheet_overflow_summary(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> sheet_view::DrawingSheetOverflowSummary {
    state
        .model
        .overflow_summary(review_markers(state), symbol_context, sheet)
}

#[cfg(test)]
mod tests {
    use super::super::schematic_symbol_context;
    use super::*;
    use crate::state::{Point, Wire};
    use rspice_schematic_editor::view::drawing_sheet::{
        DrawingSheetGeometry, DrawingSheetOverflowTarget,
    };
    use std::sync::Arc;

    /// Saved with 5 mm margins all round, an A4 sheet drew its frame 5 mm
    /// from the trimmed edge. It loads on ISO 5457's 20 mm filing border, so
    /// a wire 12 mm from the left edge keeps its place on the sheet and the
    /// overflow advisory reports it outside the frame.
    #[test]
    fn content_the_standard_margins_now_frame_out_is_reported() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(
                31,
                vec![Point::new(-92, 100), Point::new(-88, 100)],
            ));
        let mut saved =
            serde_json::to_value(crate::state::SchematicSheetFormat::default()).unwrap();
        saved["margins"] = serde_json::json!({
            "top_um": 5_000,
            "right_um": 5_000,
            "bottom_um": 5_000,
            "left_um": 5_000,
        });
        let format: crate::state::SchematicSheetFormat = serde_json::from_value(saved).unwrap();
        let sheet = ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: "top".to_owned(),
            page_label: "1 of 1".to_owned(),
        };
        assert_eq!(sheet.geometry.physical.drawing_area.x_um, 20_000);

        let symbol_context = schematic_symbol_context(&state);
        let report = drawing_sheet_overflow_summary(&state, &symbol_context, &sheet);

        assert_eq!(report.outside_border, 1);
        assert_eq!(report.off_paper, 0);
        assert_eq!(report.items[0].sheet_coordinates_mm, (12.0, 35.0));
        assert_eq!(
            report.first_target,
            Some(DrawingSheetOverflowTarget::Wire(31))
        );
    }

    #[test]
    fn overflow_navigation_selects_and_centers_the_exact_current_target() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(
                41,
                vec![Point::new(1_080, 120), Point::new(1_120, 120)],
            ));
        let expected_center = {
            let symbol_context = schematic_symbol_context(&state);
            let sheet = state.model.active_drawing_sheet();
            drawing_sheet_overflow_summary(&state, &symbol_context, &sheet).items[0].center()
        };

        let markers = review_markers(&state);
        assert!(state.model.show_first_drawing_sheet_overflow(markers));
        assert_eq!(
            state.model.schematic.session.editor.selection.single_wire(),
            Some(41)
        );
        assert_eq!(
            state.model.schematic.session.editor.center_request,
            Some(expected_center)
        );
        assert!(
            !state
                .model
                .show_overflow_target(markers, DrawingSheetOverflowTarget::Wire(404))
        );
    }

    /// The advisories are found once while the drawing and sheet stand still,
    /// the inspector reads the same answer, and each input changed on its own
    /// finds them afresh.
    #[test]
    fn the_overflow_summary_is_held_until_the_drawing_or_sheet_changes() {
        let overflowing = || {
            let mut state = AppState::default();
            state
                .model
                .schematic
                .document_mut_for_test()
                .wires
                .push(Wire::new(
                    41,
                    vec![Point::new(1_080, 120), Point::new(1_120, 120)],
                ));
            state
        };
        let held = |state: &AppState, sheet: &ActiveDrawingSheet| {
            let symbol_context = schematic_symbol_context(state);
            state
                .model
                .schematic
                .session
                .overflow_summary
                .current(&state.model.overflow_key(&symbol_context, sheet))
        };
        let hold = |state: &mut AppState, sheet: &ActiveDrawingSheet| {
            let symbol_context = schematic_symbol_context(state);
            let review_markers = review_markers(state);
            state
                .model
                .hold_overflow_summary(review_markers, &symbol_context, sheet);
        };

        let mut state = overflowing();
        let sheet = state.model.active_drawing_sheet();
        assert!(held(&state, &sheet).is_none());
        hold(&mut state, &sheet);
        let first = held(&state, &sheet).expect("held after a frame");
        hold(&mut state, &sheet);
        assert!(Arc::ptr_eq(
            &first,
            &held(&state, &sheet).expect("still held")
        ));
        assert_eq!(first.finding_count(), 1);
        let symbol_context = schematic_symbol_context(&state);
        assert_eq!(
            drawing_sheet_overflow_summary(&state, &symbol_context, &sheet).first_target,
            Some(DrawingSheetOverflowTarget::Wire(41))
        );

        let mut moved = sheet.clone();
        moved.geometry.content_area.max_x += 400.0;
        assert!(held(&state, &moved).is_none(), "sheet geometry");

        type Change = (&'static str, fn(&mut AppState));
        let changes: [Change; 3] = [
            ("topology", |state| {
                state.model.schematic.bump_topology_version()
            }),
            ("committed content", |state| {
                state.model.schematic.init_undo_history();
                state.model.schematic.begin_operation("move");
                state.model.schematic.document_mut_for_test().wires[0].points[1] =
                    Point::new(1_160, 120);
                assert!(state.model.schematic.end_operation());
            }),
            ("symbols", |state| {
                state
                    .model
                    .library_manager
                    .add_library(crate::state::Library::new("other"));
            }),
        ];
        for (input, change) in changes {
            let mut state = overflowing();
            let sheet = state.model.active_drawing_sheet();
            hold(&mut state, &sheet);
            change(&mut state);
            assert!(held(&state, &sheet).is_none(), "{input}");
        }
    }
}
