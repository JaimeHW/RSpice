//! What the open sheet holds outside its authored drawing area, and what it
//! prints.
//!
//! The canvas paints the overflow advisories, the sheet's inspector counts
//! them and its review dialog lists and reaches them. The advisories are
//! held while the drawing and the sheet stand still, so all three report one
//! answer and only one of them pays for it.

use std::sync::Arc;

use super::Model;
use crate::state::overflow_memo::OverflowKey;
use crate::state::transform_preview_memo::DrawingInputs;
use crate::workbench::app_state::model_design_view;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_schematic_editor::view::drawing_sheet::{
    self as sheet_view, ActiveDrawingSheet, DrawingSheetOverflowSummary,
    DrawingSheetOverflowTarget, DrawingSheetPrintablePreview,
};
use rspice_schematic_editor::view::symbol_context::SchematicSymbolContext;

impl Model {
    /// What a canvas derivation reads from the open drawing besides its own
    /// inputs.
    pub(crate) fn drawing_inputs(&self, symbol_context: &SchematicSymbolContext) -> DrawingInputs {
        let catalog = self.active_sheet_catalog();
        DrawingInputs {
            topology_version: self.schematic.topology_version(),
            content_version: self.schematic.content_version(),
            symbol_revision: symbol_context.revision(),
            active_sheet: catalog.and_then(crate::state::SheetCatalog::active_sheet_id),
            sheet_catalog_revision: catalog.map(crate::state::SheetCatalog::revision),
        }
    }

    /// What the held advisories answer for: the drawing as it stands and the
    /// sheet's geometry.
    pub(crate) fn overflow_key(
        &self,
        symbol_context: &SchematicSymbolContext,
        sheet: &ActiveDrawingSheet,
    ) -> OverflowKey {
        OverflowKey {
            drawing: self.drawing_inputs(symbol_context),
            geometry: sheet.geometry.clone(),
        }
    }

    /// The advisories held for the drawing and sheet as they stand, or found
    /// afresh, through the drawing as it shows `review_markers`, when nothing
    /// is held for them.
    pub(crate) fn held_overflow_summary(
        &self,
        review_markers: SchematicReviewMarkerVisibility,
        symbol_context: &SchematicSymbolContext,
        sheet: &ActiveDrawingSheet,
    ) -> Arc<DrawingSheetOverflowSummary> {
        self.schematic
            .session
            .overflow_summary
            .current(&self.overflow_key(symbol_context, sheet))
            .unwrap_or_else(|| {
                Arc::new(sheet_view::drawing_sheet_overflow_summary(
                    &model_design_view(self, review_markers),
                    symbol_context,
                    sheet,
                ))
            })
    }

    /// Hold the advisories for the drawing and sheet as they stand, finding
    /// them, through the drawing as it shows `review_markers`, only if
    /// nothing is held for exactly these inputs.
    pub(crate) fn hold_overflow_summary(
        &mut self,
        review_markers: SchematicReviewMarkerVisibility,
        symbol_context: &SchematicSymbolContext,
        sheet: &ActiveDrawingSheet,
    ) {
        let key = self.overflow_key(symbol_context, sheet);
        if self
            .schematic
            .session
            .overflow_summary
            .current(&key)
            .is_some()
        {
            return;
        }
        let summary = Arc::new(sheet_view::drawing_sheet_overflow_summary(
            &model_design_view(self, review_markers),
            symbol_context,
            sheet,
        ));
        self.schematic.session.overflow_summary.store(key, summary);
    }

    /// The sheet's overflow advisories: the ones the canvas holds while the
    /// drawing and sheet stand still, so the inspector and the canvas report
    /// one answer and only one of them pays for it. Read through the drawing
    /// as it shows `review_markers`.
    pub(crate) fn overflow_summary(
        &self,
        review_markers: SchematicReviewMarkerVisibility,
        symbol_context: &SchematicSymbolContext,
        sheet: &ActiveDrawingSheet,
    ) -> DrawingSheetOverflowSummary {
        self.held_overflow_summary(review_markers, symbol_context, sheet)
            .as_ref()
            .clone()
    }

    /// What the open sheet prints, read through the drawing as it shows
    /// `review_markers`.
    pub(crate) fn drawing_sheet_printable_preview(
        &self,
        review_markers: SchematicReviewMarkerVisibility,
    ) -> Vec<DrawingSheetPrintablePreview> {
        sheet_view::drawing_sheet_printable_preview(
            &model_design_view(self, review_markers),
            &self.schematic_symbol_context(),
        )
    }

    /// Recompute at invocation time before selecting and centering a current
    /// item, as the drawing shows `review_markers`.
    pub(crate) fn show_first_drawing_sheet_overflow(
        &mut self,
        review_markers: SchematicReviewMarkerVisibility,
    ) -> bool {
        let first_target = {
            let symbol_context = self.schematic_symbol_context();
            let sheet = self.active_drawing_sheet();
            self.overflow_summary(review_markers, &symbol_context, &sheet)
                .first_target
        };
        first_target.is_some_and(|target| self.show_overflow_target(review_markers, target))
    }

    /// Select `target` and center the canvas on it, as the advisories the
    /// drawing shows with `review_markers` place it.
    pub(crate) fn show_overflow_target(
        &mut self,
        review_markers: SchematicReviewMarkerVisibility,
        target: DrawingSheetOverflowTarget,
    ) -> bool {
        let center = {
            let symbol_context = self.schematic_symbol_context();
            let sheet = self.active_drawing_sheet();
            self.overflow_summary(review_markers, &symbol_context, &sheet)
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
                self.schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
            }
            DrawingSheetOverflowTarget::Wire(id) => {
                self.schematic.session.editor.selection.select_only_wire(id);
            }
            DrawingSheetOverflowTarget::Bus(id) => {
                self.schematic.session.editor.selection.select_only_bus(id);
            }
            DrawingSheetOverflowTarget::BusTap(id) => {
                self.schematic
                    .session
                    .editor
                    .selection
                    .select_only_bus_tap(id);
            }
            DrawingSheetOverflowTarget::Junction(id) => {
                let Some(position) = self
                    .schematic
                    .document()
                    .junctions
                    .iter()
                    .find(|junction| junction.id == id)
                    .map(|junction| junction.pos)
                else {
                    return false;
                };
                self.schematic
                    .session
                    .editor
                    .selection
                    .select_only_junction(position);
            }
            DrawingSheetOverflowTarget::NetLabel(id) => {
                self.schematic
                    .session
                    .editor
                    .selection
                    .select_only_net_label(id);
            }
            DrawingSheetOverflowTarget::DesignNote(id) => {
                self.schematic
                    .session
                    .editor
                    .selection
                    .select_only_design_note(id);
            }
            DrawingSheetOverflowTarget::DocumentationShape(id) => {
                self.schematic
                    .session
                    .editor
                    .selection
                    .select_only_documentation_shape(id);
            }
        }
        self.schematic.session.editor.center_request = Some(center);
        true
    }
}
