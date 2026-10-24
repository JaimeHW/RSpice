//! The drawing sheet the open cell view is drawn on, resolved from the
//! project's sheet catalog and drawing-sheet settings alone, so the canvas,
//! the sheet's own inspector and the printed sheet read one answer.

use super::Model;
use crate::state::DrawingSheetTitleFieldId;
use rspice_schematic_editor::view::drawing_sheet::{ActiveDrawingSheet, DrawingSheetGeometry};

impl Model {
    /// The sheet the active cell view's catalog has active, with the format
    /// it prints on; the project's default format, named for the cell, where
    /// the cell view has no catalog.
    pub(crate) fn active_drawing_sheet(&self) -> ActiveDrawingSheet {
        let key = self.workspace.content.active_schematic_reference().key();
        if let Some(catalog) = self.workspace.content.design_management.sheet_catalog(&key)
            && let Some(sheet) = catalog.active()
        {
            let (page, page_count) = catalog
                .page_number_and_count(sheet.id())
                .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)));
            let format = match sheet.page_format().inheritance {
                crate::state::DrawingSheetInheritance::ProjectDefault => self
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

        let format = self
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .default_format
            .clone();
        ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: self.workspace.content.active_view.cell.clone(),
            page_label: "1 of 1".to_owned(),
        }
    }

    /// Every title-block field `sheet` letters, with the value the project
    /// supplies where the sheet types none of its own.
    pub(crate) fn title_block_fields(
        &self,
        sheet: &ActiveDrawingSheet,
    ) -> Vec<crate::state::ResolvedDrawingSheetTitleField> {
        let authority_values = DrawingSheetTitleFieldId::ALL
            .into_iter()
            .map(|id| (id, self.title_field_value(id, sheet)))
            .collect();
        crate::state::resolve_drawing_sheet_title_fields(&sheet.format, &authority_values)
    }

    fn title_field_value(
        &self,
        field: DrawingSheetTitleFieldId,
        sheet: &ActiveDrawingSheet,
    ) -> String {
        match field {
            DrawingSheetTitleFieldId::Project => self.workspace.content.project.name().to_owned(),
            DrawingSheetTitleFieldId::CellView => {
                self.workspace.content.active_view.display_path().to_owned()
            }
            DrawingSheetTitleFieldId::SheetTitle => sheet.sheet_name.clone(),
            DrawingSheetTitleFieldId::Page => sheet.page_label.clone(),
            DrawingSheetTitleFieldId::Revision => self
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .document_control
                .revision_index()
                .to_owned(),
            DrawingSheetTitleFieldId::PaperSize => {
                sheet.format.authored_size.paper_size().to_owned()
            }
            // Retired: never present in a normalized field set, so never resolved.
            DrawingSheetTitleFieldId::Scale => String::new(),
            // The approval date reads the document's date of issue while the
            // sheet gives none of its own.
            DrawingSheetTitleFieldId::Date | DrawingSheetTitleFieldId::ApprovedDate => self
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .document_control
                .revision_date_utc
                .clone(),
            DrawingSheetTitleFieldId::DocumentStatus => self
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
            | DrawingSheetTitleFieldId::ContractNumber => self
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
}
