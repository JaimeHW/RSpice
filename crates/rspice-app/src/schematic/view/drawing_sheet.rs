//! Resolve drawing-sheet project inputs and coordinate current overflow navigation.

use super::viewport::Viewport;
use super::{AppState, SchematicSymbolContext, schematic_symbol_context};
use crate::state::DrawingSheetTitleFieldId;
use crate::state::overflow_memo::OverflowKey;
use egui::{Painter, Rect};
use rspice_schematic_editor::view::drawing_sheet::{self as sheet_view, DrawingSheetCanvas};
pub(crate) use rspice_schematic_editor::view::drawing_sheet::{
    ActiveDrawingSheet, DRAWING_SHEET_ORIGIN, DRAWING_SHEET_UNITS_PER_MM, DrawingSheetGeometry,
    DrawingSheetOverflowItem, DrawingSheetOverflowSummary, DrawingSheetOverflowTarget,
    DrawingSheetPrintablePreview, DrawingSheetPrintablePreviewKind, FIT_SCREEN_INSET,
};
use std::sync::Arc;

pub(crate) fn resolve_active_drawing_sheet(state: &AppState) -> ActiveDrawingSheet {
    let key = state.workspace.content.active_schematic_reference().key();
    if let Some(catalog) = state
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        && let Some(sheet) = catalog.active()
    {
        let (page, page_count) = catalog
            .page_number_and_count(sheet.id())
            .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)));
        let format = match sheet.page_format().inheritance {
            crate::state::DrawingSheetInheritance::ProjectDefault => state
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .default_format
                .with_target_sheet_title_fields(sheet.page_format()),
            crate::state::DrawingSheetInheritance::Explicit
            | crate::state::DrawingSheetInheritance::UserDefault => sheet.page_format().clone(),
        };
        return ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: sheet.name().to_owned(),
            page_label: format!("{page} of {page_count}"),
        };
    }

    let format = state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .default_format
        .clone();
    ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: state.workspace.content.active_view.cell.clone(),
        page_label: "1 of 1".to_owned(),
    }
}

pub(crate) fn resolved_title_block_fields(
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) -> Vec<crate::state::ResolvedDrawingSheetTitleField> {
    let authority_values = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .map(|id| (id, default_title_field_value(id, state, sheet)))
        .collect();
    crate::state::resolve_drawing_sheet_title_fields(&sheet.format, &authority_values)
}

fn default_title_field_value(
    field: DrawingSheetTitleFieldId,
    state: &AppState,
    sheet: &ActiveDrawingSheet,
) -> String {
    match field {
        DrawingSheetTitleFieldId::Project => state.workspace.content.project.name().to_owned(),
        DrawingSheetTitleFieldId::CellView => state
            .workspace
            .content
            .active_view
            .display_path()
            .to_owned(),
        DrawingSheetTitleFieldId::SheetTitle => sheet.sheet_name.clone(),
        DrawingSheetTitleFieldId::Page => sheet.page_label.clone(),
        DrawingSheetTitleFieldId::Revision => state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .revision_index()
            .to_owned(),
        DrawingSheetTitleFieldId::PaperSize => sheet.format.authored_size.paper_size().to_owned(),
        // Retired: never present in a normalized field set, so never resolved.
        DrawingSheetTitleFieldId::Scale => String::new(),
        // The approval date reads the document's date of issue while the
        // sheet gives none of its own.
        DrawingSheetTitleFieldId::Date | DrawingSheetTitleFieldId::ApprovedDate => state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .revision_date_utc
            .clone(),
        DrawingSheetTitleFieldId::DocumentStatus => state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .status
            .label()
            .to_owned(),
        DrawingSheetTitleFieldId::Organization
        | DrawingSheetTitleFieldId::DocumentId
        | DrawingSheetTitleFieldId::Classification
        | DrawingSheetTitleFieldId::Language
        | DrawingSheetTitleFieldId::CageCode
        | DrawingSheetTitleFieldId::ContractNumber => state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .title_block_field_values
            .get(&field)
            .cloned()
            .unwrap_or_default(),
        DrawingSheetTitleFieldId::DrawnBy
        | DrawingSheetTitleFieldId::CheckedBy
        | DrawingSheetTitleFieldId::ApprovedBy
        | DrawingSheetTitleFieldId::Engineer
        | DrawingSheetTitleFieldId::DrawnDate
        | DrawingSheetTitleFieldId::CheckedDate
        | DrawingSheetTitleFieldId::EngineerDate
        | DrawingSheetTitleFieldId::DocumentType => String::new(),
    }
}

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
            layers: state.ui.drawing_sheet_layers,
            grid: state.ui.grid,
            grid_size: state.schematic.document().grid_size,
        },
        sheet,
        || resolved_title_block_fields(state, sheet),
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
    let summary = held_overflow_summary(state, symbol_context, sheet);
    sheet_view::draw_overflow_advisories(painter, available, viewport, &summary);
}

fn overflow_key(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> OverflowKey {
    OverflowKey {
        drawing: super::preview::drawing_inputs(state, symbol_context),
        geometry: sheet.geometry.clone(),
    }
}

/// The advisories held for the drawing and sheet as they stand, or found
/// afresh when nothing is held for them.
fn held_overflow_summary(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> Arc<DrawingSheetOverflowSummary> {
    state
        .schematic
        .session
        .overflow_summary
        .current(&overflow_key(state, symbol_context, sheet))
        .unwrap_or_else(|| {
            Arc::new(sheet_view::drawing_sheet_overflow_summary(
                &super::schematic_design_view(state),
                symbol_context,
                sheet,
            ))
        })
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
        hold_overflow_summary(state, symbol_context, sheet);
    }
}

/// Hold the advisories for the drawing and sheet as they stand, finding them
/// only if nothing is held for exactly these inputs.
fn hold_overflow_summary(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) {
    let key = overflow_key(state, symbol_context, sheet);
    if state
        .schematic
        .session
        .overflow_summary
        .current(&key)
        .is_some()
    {
        return;
    }
    let summary = Arc::new(sheet_view::drawing_sheet_overflow_summary(
        &super::schematic_design_view(state),
        symbol_context,
        sheet,
    ));
    state.schematic.session.overflow_summary.store(key, summary);
}

/// The sheet's overflow advisories: the ones the canvas holds while the
/// drawing and sheet stand still, so the inspector and the canvas report one
/// answer and only one of them pays for it.
pub(crate) fn drawing_sheet_overflow_summary(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> DrawingSheetOverflowSummary {
    held_overflow_summary(state, symbol_context, sheet)
        .as_ref()
        .clone()
}

pub(crate) fn drawing_sheet_printable_preview(
    state: &AppState,
) -> Vec<DrawingSheetPrintablePreview> {
    let symbol_context = schematic_symbol_context(state);
    sheet_view::drawing_sheet_printable_preview(
        &super::schematic_design_view(state),
        &symbol_context,
    )
}

/// Recompute at invocation time before selecting and centering a current item.
pub(crate) fn show_first_drawing_sheet_overflow(state: &mut AppState) -> bool {
    let first_target = {
        let symbol_context = schematic_symbol_context(state);
        let sheet = resolve_active_drawing_sheet(state);
        drawing_sheet_overflow_summary(state, &symbol_context, &sheet).first_target
    };
    first_target.is_some_and(|target| show_drawing_sheet_overflow_target(state, target))
}
pub(crate) fn show_drawing_sheet_overflow_target(
    state: &mut AppState,
    target: DrawingSheetOverflowTarget,
) -> bool {
    let center = {
        let symbol_context = schematic_symbol_context(state);
        let sheet = resolve_active_drawing_sheet(state);
        drawing_sheet_overflow_summary(state, &symbol_context, &sheet)
            .items
            .into_iter()
            .find(|item| item.target == target)
            .map(|item| item.center())
    };
    let Some(center) = center else {
        return false;
    };
    match target {
        DrawingSheetOverflowTarget::Component(id) => {
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_component(id);
        }
        DrawingSheetOverflowTarget::Wire(id) => {
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_wire(id);
        }
        DrawingSheetOverflowTarget::Bus(id) => {
            state.schematic.session.editor.selection.select_only_bus(id);
        }
        DrawingSheetOverflowTarget::BusTap(id) => {
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_bus_tap(id);
        }
        DrawingSheetOverflowTarget::Junction(id) => {
            let Some(position) = state
                .schematic
                .document()
                .junctions
                .iter()
                .find(|junction| junction.id == id)
                .map(|junction| junction.pos)
            else {
                return false;
            };
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_junction(position);
        }
        DrawingSheetOverflowTarget::NetLabel(id) => {
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_net_label(id);
        }
        DrawingSheetOverflowTarget::DesignNote(id) => {
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_design_note(id);
        }
        DrawingSheetOverflowTarget::DocumentationShape(id) => {
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_documentation_shape(id);
        }
    }
    state.schematic.session.editor.center_request = Some(center);
    true
}

/// What one schematic grid pitch measures on the printed sheet: "2.5 mm".
///
/// Derived from the sheet's calibration rather than spelled by hand, so that
/// every place a pitch is named agrees with the drawing it prints on.
pub(crate) fn grid_pitch_label(pitch: crate::state::SchematicGridPitch) -> String {
    let micrometres = u64::from(pitch.canvas_grid_size().unsigned_abs())
        * rspice_schematic_editor::view::drawing_sheet::DRAWING_SHEET_WORLD_UNIT_UM;
    format!("{} mm", micrometres as f64 / 1_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Point, Wire};

    /// A grid label is what the pitch measures on the sheet, so reading the
    /// label back through the sheet scale gives the pitch's own unit count.
    #[test]
    fn grid_labels_measure_the_pitch_on_the_printed_sheet() {
        use crate::state::SchematicGridPitch;
        for (pitch, label) in [
            (SchematicGridPitch::Mil50, "2.5 mm"),
            (SchematicGridPitch::Mil25, "1.25 mm"),
            (SchematicGridPitch::Metric, "1 mm"),
        ] {
            assert_eq!(grid_pitch_label(pitch), label);
            let millimetres = label
                .strip_suffix(" mm")
                .and_then(|value| value.parse::<f64>().ok())
                .expect("a grid label is millimetres");
            assert_eq!(
                millimetres * DRAWING_SHEET_UNITS_PER_MM,
                f64::from(pitch.canvas_grid_size()),
                "{label} disagrees with the sheet scale"
            );
        }
    }

    /// Saved with 5 mm margins all round, an A4 sheet drew its frame 5 mm
    /// from the trimmed edge. It loads on ISO 5457's 20 mm filing border, so
    /// a wire 12 mm from the left edge keeps its place on the sheet and the
    /// overflow advisory reports it outside the frame.
    #[test]
    fn content_the_standard_margins_now_frame_out_is_reported() {
        let mut state = AppState::default();
        state
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
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(
                41,
                vec![Point::new(1_080, 120), Point::new(1_120, 120)],
            ));
        let expected_center = {
            let symbol_context = schematic_symbol_context(&state);
            let sheet = resolve_active_drawing_sheet(&state);
            drawing_sheet_overflow_summary(&state, &symbol_context, &sheet).items[0].center()
        };

        assert!(show_first_drawing_sheet_overflow(&mut state));
        assert_eq!(
            state.schematic.session.editor.selection.single_wire(),
            Some(41)
        );
        assert_eq!(
            state.schematic.session.editor.center_request,
            Some(expected_center)
        );
        assert!(!show_drawing_sheet_overflow_target(
            &mut state,
            DrawingSheetOverflowTarget::Wire(404)
        ));
    }

    /// The advisories are found once while the drawing and sheet stand still,
    /// the inspector reads the same answer, and each input changed on its own
    /// finds them afresh.
    #[test]
    fn the_overflow_summary_is_held_until_the_drawing_or_sheet_changes() {
        let overflowing = || {
            let mut state = AppState::default();
            state
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
                .schematic
                .session
                .overflow_summary
                .current(&overflow_key(state, &symbol_context, sheet))
        };
        let hold = |state: &mut AppState, sheet: &ActiveDrawingSheet| {
            let symbol_context = schematic_symbol_context(state);
            hold_overflow_summary(state, &symbol_context, sheet);
        };

        let mut state = overflowing();
        let sheet = resolve_active_drawing_sheet(&state);
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
            ("topology", |state| state.schematic.bump_topology_version()),
            ("committed content", |state| {
                state.schematic.init_undo_history();
                state.schematic.begin_operation("move");
                state.schematic.document_mut_for_test().wires[0].points[1] = Point::new(1_160, 120);
                assert!(state.schematic.end_operation());
            }),
            ("symbols", |state| {
                state
                    .library_manager
                    .add_library(crate::state::Library::new("other"));
            }),
        ];
        for (input, change) in changes {
            let mut state = overflowing();
            let sheet = resolve_active_drawing_sheet(&state);
            hold(&mut state, &sheet);
            change(&mut state);
            assert!(held(&state, &sheet).is_none(), "{input}");
        }
    }
}
