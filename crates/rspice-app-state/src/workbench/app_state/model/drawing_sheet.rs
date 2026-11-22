//! The drawing sheet the open cell view is drawn on, resolved from the
//! project's sheet catalog and drawing-sheet settings alone, so the canvas,
//! the sheet's own inspector and the printed sheet read one answer.

use super::Model;
use crate::state::{DesignSheet, DrawingSheetTitleFieldId, SchematicSheetFormat, SheetId};
use rspice_schematic_editor::view::drawing_sheet::{
    ActiveDrawingSheet, DrawingSheetGeometry, zone_on_format,
};
use std::collections::HashMap;

impl Model {
    /// The sheet the active cell view's catalog has active, with the format
    /// it prints on; the project's default format where the cell view has no
    /// catalog. Either way the sheet carries the cell's name for its block.
    pub fn active_drawing_sheet(&self) -> ActiveDrawingSheet {
        let key = self.workspace.content.active_schematic_reference().key();
        if let Some(catalog) = self.workspace.content.design_management.sheet_catalog(&key)
            && let Some(sheet) = catalog.active()
        {
            let (page, page_count) = catalog
                .page_number_and_count(sheet.id())
                .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)));
            let format = self.sheet_print_format(sheet);
            return ActiveDrawingSheet {
                geometry: DrawingSheetGeometry::from_format(&format),
                format,
                design_name: self.workspace.content.active_view.cell.clone(),
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
            design_name: self.workspace.content.active_view.cell.clone(),
            page_label: "1 of 1".to_owned(),
        }
    }

    /// The format `sheet` prints on: its own, or the project's default with
    /// the sheet's own title values where it inherits the default.
    fn sheet_print_format(&self, sheet: &DesignSheet) -> SchematicSheetFormat {
        match sheet.page_format().inheritance {
            crate::state::DrawingSheetInheritance::ProjectDefault => self
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .default_format
                .with_target_sheet_title_fields(sheet.page_format()),
            crate::state::DrawingSheetInheritance::Explicit
            | crate::state::DrawingSheetInheritance::UserDefault => sheet.page_format().clone(),
        }
    }

    /// What each off-sheet connector on the active sheet letters after its
    /// tab: the page and zone of every connector of the same name on another
    /// sheet, such as `[p2 B3, p4 A1]`, so a reader can find where the signal
    /// goes. A page whose format draws no zones gives its number alone. A
    /// connector with no partner on another sheet letters nothing, and
    /// neither does a drawing of one sheet.
    pub fn off_sheet_partner_references(&self) -> HashMap<u64, String> {
        let key = self.workspace.content.active_schematic_reference().key();
        let Some(catalog) = self.workspace.content.design_management.sheet_catalog(&key) else {
            return HashMap::new();
        };
        let Some(active) = catalog.active_sheet_id() else {
            return HashMap::new();
        };
        // Two connectors are partners when the netlister would join them,
        // and it folds ASCII case.
        let mut by_name: HashMap<String, Vec<(SheetId, &crate::state::NetLabel)>> = HashMap::new();
        for connector in &self.schematic.document().net_labels {
            if connector.kind.off_sheet_direction().is_some() {
                let sheet = catalog.sheet_for_object(connector.id).unwrap_or(active);
                by_name
                    .entry(connector.name.trim().to_ascii_lowercase())
                    .or_default()
                    .push((sheet, connector));
            }
        }
        let mut formats: HashMap<SheetId, Option<SchematicSheetFormat>> = HashMap::new();
        let mut references = HashMap::new();
        for connectors in by_name.values() {
            for &(sheet, connector) in connectors {
                if sheet != active {
                    continue;
                }
                let mut partners = connectors
                    .iter()
                    .filter(|(elsewhere, _)| *elsewhere != active)
                    .filter_map(|&(elsewhere, partner)| {
                        let (page, _) = catalog.page_number_and_count(elsewhere)?;
                        let format = formats.entry(elsewhere).or_insert_with(|| {
                            catalog
                                .find(elsewhere)
                                .map(|sheet| self.sheet_print_format(sheet))
                        });
                        let zone = format
                            .as_ref()
                            .and_then(|format| zone_on_format(format, partner.pos));
                        Some((page, zone))
                    })
                    .collect::<Vec<_>>();
                if partners.is_empty() {
                    continue;
                }
                partners.sort();
                partners.dedup();
                let partners = partners
                    .into_iter()
                    .map(|(page, zone)| match zone {
                        Some(zone) => format!("p{page} {zone}"),
                        None => format!("p{page}"),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                references.insert(connector.id, format!("[{partners}]"));
            }
        }
        references
    }

    /// The ISO 639 code of the language the project's documents are written
    /// in, from its title-block Language code, if it gives one. The project
    /// keeps it, so every sheet of a document shares it.
    pub fn document_language(&self) -> Option<&str> {
        self.workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .title_block_field_values
            .get(&DrawingSheetTitleFieldId::Language)
            .map(|language| language.trim())
            .filter(|language| !language.is_empty())
    }

    /// Whether the open view's sheet is one an editor may change: a writable
    /// schematic or testbench.
    pub fn drawing_sheet_writable(&self) -> bool {
        !self.schematic_edit_read_only()
            && matches!(
                self.workspace.content.active_view_type(),
                Some(crate::state::ViewType::Schematic | crate::state::ViewType::Testbench)
            )
    }

    /// Every title-block field `sheet` letters, with the value the project
    /// supplies where the sheet types none of its own.
    pub fn title_block_fields(
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
            DrawingSheetTitleFieldId::SheetTitle => sheet.design_name.clone(),
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
            // The project's own values, and its names for the people who
            // sign, which a sheet's own names stand in for.
            DrawingSheetTitleFieldId::Organization
            | DrawingSheetTitleFieldId::DocumentId
            | DrawingSheetTitleFieldId::Classification
            | DrawingSheetTitleFieldId::Language
            | DrawingSheetTitleFieldId::CageCode
            | DrawingSheetTitleFieldId::ContractNumber
            | DrawingSheetTitleFieldId::DrawnBy
            | DrawingSheetTitleFieldId::CheckedBy
            | DrawingSheetTitleFieldId::ApprovedBy => self
                .workspace
                .content
                .design_management
                .drawing_sheet_settings()
                .title_block_field_values
                .get(&field)
                .cloned()
                .unwrap_or_default(),
            DrawingSheetTitleFieldId::Engineer
            | DrawingSheetTitleFieldId::DrawnDate
            | DrawingSheetTitleFieldId::CheckedDate
            | DrawingSheetTitleFieldId::EngineerDate
            | DrawingSheetTitleFieldId::DocumentType => String::new(),
        }
    }
}
