//! Resolve drawing-sheet project inputs and coordinate current overflow navigation.

use super::viewport::Viewport;
use super::{SchematicSymbolContext, schematic_symbol_context};
use crate::state::{DrawingSheetScale, DrawingSheetTitleFieldId};
use crate::workbench::app_state::AppState;
use egui::{Painter, Rect};
use rspice_schematic_editor::view::drawing_sheet::{self as sheet_view, DrawingSheetCanvas};
pub(crate) use rspice_schematic_editor::view::drawing_sheet::{
    ActiveDrawingSheet, DRAWING_SHEET_ORIGIN, DRAWING_SHEET_UNITS_PER_MM, DrawingSheetGeometry,
    DrawingSheetOverflowItem, DrawingSheetOverflowSummary, DrawingSheetOverflowTarget,
    DrawingSheetPrintablePreview, DrawingSheetPrintablePreviewKind, FIT_SCREEN_INSET,
};

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
            .revision
            .clone(),
        DrawingSheetTitleFieldId::Format => sheet.format_label(),
        DrawingSheetTitleFieldId::Scale => match sheet.format.title_block.scale {
            DrawingSheetScale::NotToScale => "NTS".to_owned(),
            DrawingSheetScale::Ratio {
                drawing_units,
                reality_units,
            } => format!("{drawing_units}:{reality_units}"),
        },
        DrawingSheetTitleFieldId::Date => state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .display_date()
            .to_owned(),
        DrawingSheetTitleFieldId::Organization
        | DrawingSheetTitleFieldId::DocumentId
        | DrawingSheetTitleFieldId::Classification => state
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
        | DrawingSheetTitleFieldId::ApprovedBy => String::new(),
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
            pan: state.schematic.session.editor.pan,
            zoom: state.schematic.session.editor.zoom,
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
    sheet_view::draw_overflow_advisories(
        painter,
        available,
        viewport,
        &super::schematic_design_view(state),
        symbol_context,
        sheet,
    );
}

pub(crate) fn drawing_sheet_overflow_summary(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    sheet: &ActiveDrawingSheet,
) -> DrawingSheetOverflowSummary {
    sheet_view::drawing_sheet_overflow_summary(
        &super::schematic_design_view(state),
        symbol_context,
        sheet,
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Point, Wire};
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
}
