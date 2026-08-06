//! Supporting drawing-sheet managers authored by the schematic mockup.
//!
//! Page Setup edits one authored sheet. These surfaces handle document-wide
//! reconciliation and title-field ownership without mixing either operation
//! with print media, export settings, netlisting, or simulation state.

use std::collections::BTreeMap;

use egui::{
    Align, ComboBox, Context, Frame, Grid, Layout, Margin, RichText, ScrollArea, Stroke, TextEdit,
    Ui, vec2,
};
use egui_extras::{Column, TableBuilder};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    DrawingSheetDocumentControl, DrawingSheetInheritance, DrawingSheetReleaseStatus,
    DrawingSheetTitleFieldId, DrawingSheetTitleFieldState, DrawingSheetTitleFieldValueAuthority,
    DrawingSheetTransactionKind, DrawingSheetTransactionReceipt, DrawingSheetTransactionSkip,
    MAX_DRAWING_SHEET_CHANGE_REFERENCE_BYTES, MAX_DRAWING_SHEET_REVISION_BYTES,
    SchematicSheetFormat, SheetId,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};

use super::drawing_sheet_preview::{
    DrawingSheetPreviewContent, drawing_sheet_preview, drawing_sheet_preview_at_common_scale,
};
#[cfg(test)]
use super::drawing_sheet_setup::open_drawing_sheet_setup_for_state;
use super::drawing_sheet_setup::{
    DrawingSheetAuthority, GovernedDrawingSheetAuthority, validate_drawing_sheet_authority,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SheetManagerFormatSource {
    #[default]
    ActiveSheet,
    ProjectDefault,
    PageSetup,
}

#[derive(Debug, Clone)]
pub(crate) struct SheetFormatManagerRow {
    pub(crate) id: SheetId,
    pub(crate) revision: u64,
    pub(crate) name: String,
    pub(crate) page: usize,
    pub(crate) page_count: usize,
    pub(crate) format: SchematicSheetFormat,
    pub(crate) selected: bool,
    pub(crate) active: bool,
    pub(crate) has_geometry: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SheetFormatManagerState {
    pub(crate) open: bool,
    pub(crate) edit: Option<crate::workbench::app::SchematicEditAuthority>,
    pub(crate) owner_key: String,
    pub(crate) design_management_revision: u64,
    pub(crate) catalog_revision: u64,
    pub(crate) rows: Vec<SheetFormatManagerRow>,
    pub(crate) active_format: Option<SchematicSheetFormat>,
    pub(crate) project_default: Option<SchematicSheetFormat>,
    preview_content: DrawingSheetPreviewContent,
    pub(crate) source: SheetManagerFormatSource,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TitleBlockFieldsState {
    pub(crate) open: bool,
    /// True when values are being staged into the open Page Setup transaction
    /// rather than written independently to the sheet catalog.
    pub(crate) staged_page_setup: bool,
    pub(crate) authority: Option<DrawingSheetAuthority>,
    pub(crate) sheet_name: String,
    pub(crate) format: Option<SchematicSheetFormat>,
    pub(crate) baseline: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
    pub(crate) draft: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
    pub(crate) baseline_project: BTreeMap<DrawingSheetTitleFieldId, String>,
    pub(crate) draft_project: BTreeMap<DrawingSheetTitleFieldId, String>,
    /// Saved project authority, the value at nested-dialog open, and the
    /// editable value are distinct so reopening Page Setup can also remove a
    /// previously staged document-control change.
    pub(crate) saved_document_control: DrawingSheetDocumentControl,
    pub(crate) baseline_document_control: DrawingSheetDocumentControl,
    pub(crate) draft_document_control: DrawingSheetDocumentControl,
    pub(crate) automatic_values: BTreeMap<DrawingSheetTitleFieldId, String>,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DrawingSheetSupportState {
    pub(crate) manager: SheetFormatManagerState,
    pub(crate) title_fields: TitleBlockFieldsState,
}

impl DrawingSheetSupportState {
    pub(crate) fn any_open(&self) -> bool {
        self.manager.open || self.title_fields.open
    }
}

pub(crate) fn open_sheet_format_manager(state: &mut AppState) -> Result<(), String> {
    let edit = crate::workbench::app::SchematicEditAuthority::capture(state);
    edit.validate(state, "Sheet Format Manager")?;
    let preview_content = DrawingSheetPreviewContent::from_state(state);
    let owner_key = state.workspace.active_key();
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog(&owner_key)
        .ok_or_else(|| {
            "Create the governed drawing sheet in Page Setup before managing document formats."
                .to_owned()
        })?;
    let active_id = catalog
        .active_sheet_id()
        .ok_or_else(|| "The governed schematic has no active sheet.".to_owned())?;
    let page_count = catalog.sheets().len();
    let object_assignments = catalog.object_assignments();
    let rows = catalog
        .sheets()
        .iter()
        .enumerate()
        .map(|(index, sheet)| SheetFormatManagerRow {
            id: sheet.id(),
            revision: sheet.revision(),
            name: sheet.name().to_owned(),
            page: index + 1,
            page_count,
            format: sheet.page_format().clone(),
            selected: sheet.id() == active_id,
            active: sheet.id() == active_id,
            has_geometry: object_assignments
                .values()
                .any(|assigned| *assigned == sheet.id())
                || (sheet.id() == active_id && !preview_content.is_empty()),
        })
        .collect::<Vec<_>>();
    let active_format = catalog
        .find(active_id)
        .map(|sheet| sheet.page_format().clone())
        .ok_or_else(|| "The governed active sheet is unavailable.".to_owned())?;
    state.dialogs.drawing_sheet_support.manager = SheetFormatManagerState {
        open: true,
        edit: Some(edit),
        owner_key,
        design_management_revision: state.workspace.design_management.revision(),
        catalog_revision: catalog.revision(),
        rows,
        active_format: Some(active_format),
        project_default: Some(
            state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .default_format
                .clone(),
        ),
        preview_content,
        ..SheetFormatManagerState::default()
    };
    Ok(())
}

pub(crate) fn open_title_block_fields(state: &mut AppState) -> Result<(), String> {
    // A nested editor must participate in Page Setup's transaction. Writing
    // through to the catalog here would leave Page Setup holding a stale copy
    // that could overwrite the accepted field values on its eventual apply.
    if state.dialogs.drawing_sheet_setup.open {
        return open_staged_title_block_fields(state);
    }
    let owner_key = state.workspace.active_key();
    let catalog = state.workspace.design_management.sheet_catalog(&owner_key);
    if catalog.is_none() {
        return open_staged_title_block_fields(state);
    }
    let catalog = catalog.expect("checked governed catalog");
    let sheet = catalog
        .active()
        .ok_or_else(|| "The governed schematic has no active sheet.".to_owned())?;
    let (page, page_count) = catalog
        .page_number_and_count(sheet.id())
        .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)));
    let format = sheet.page_format().clone();
    let mut fields = format.title_block.fields.clone();
    if let Some(field) = fields.get_mut(&DrawingSheetTitleFieldId::SheetTitle)
        && field.value.trim().is_empty()
    {
        field.value = sheet.name().to_owned();
    }
    let authority = DrawingSheetAuthority {
        edit: crate::workbench::app::SchematicEditAuthority::capture(state),
        cell_view_key: owner_key.clone(),
        design_management_revision: state.workspace.design_management.revision(),
        personal_preferences_digest: None,
        governed: Some(GovernedDrawingSheetAuthority {
            cell_view_key: owner_key,
            catalog_revision: catalog.revision(),
            sheet_id: sheet.id(),
            sheet_revision: sheet.revision(),
        }),
    };
    let settings = state.workspace.design_management.drawing_sheet_settings();
    let document_control = settings.document_control.clone();
    let automatic_values = title_field_automatic_values(
        state,
        sheet.name(),
        page,
        page_count,
        &format,
        &document_control,
    );
    let mut project_fields = settings.title_block_field_values.clone();
    // Preserve values authored by builds that predate project-owned title
    // fields. Saving this dialog performs the one-time authority migration.
    for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
        let project_value = project_fields.entry(id).or_default();
        if project_value.is_empty()
            && let Some(legacy) = fields.get(&id)
            && !legacy.value.trim().is_empty()
        {
            *project_value = legacy.value.clone();
        }
    }
    state.dialogs.drawing_sheet_support.title_fields = TitleBlockFieldsState {
        open: true,
        staged_page_setup: false,
        authority: Some(authority),
        sheet_name: sheet.name().to_owned(),
        format: Some(format),
        baseline: fields.clone(),
        draft: fields,
        baseline_project: project_fields.clone(),
        draft_project: project_fields,
        saved_document_control: document_control.clone(),
        baseline_document_control: document_control.clone(),
        draft_document_control: document_control,
        automatic_values,
        error: None,
    };
    Ok(())
}

fn open_staged_title_block_fields(state: &mut AppState) -> Result<(), String> {
    let setup = &state.dialogs.drawing_sheet_setup;
    let authority = setup.authority.clone().ok_or_else(|| {
        "Open Page Setup before editing title-block fields in its transaction.".to_owned()
    })?;
    if !setup.open {
        return Err(
            "Open Page Setup before editing title-block fields in its transaction.".to_owned(),
        );
    }
    let format = setup
        .draft
        .validate()
        .map_err(|_| {
            "Resolve the highlighted Page Setup values before editing title-block fields."
                .to_owned()
        })?
        .page_format;
    let sheet_name = setup.sheet_name.clone();
    let mut fields = format.title_block.fields.clone();
    if let Some(field) = fields.get_mut(&DrawingSheetTitleFieldId::SheetTitle)
        && field.value.trim().is_empty()
    {
        field.value.clone_from(&sheet_name);
    }
    let mut project_fields = setup
        .staged_project_title_values
        .clone()
        .unwrap_or_else(|| {
            state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .title_block_field_values
                .clone()
        });
    for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
        let project_value = project_fields.entry(id).or_default();
        if project_value.is_empty()
            && let Some(legacy) = fields.get(&id)
            && !legacy.value.trim().is_empty()
        {
            project_value.clone_from(&legacy.value);
        }
    }
    let saved_document_control = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .document_control
        .clone();
    let document_control = setup
        .staged_document_control
        .clone()
        .unwrap_or_else(|| saved_document_control.clone());
    let (page, page_count) = authority.governed.as_ref().map_or((1, 1), |governed| {
        state
            .workspace
            .design_management
            .sheet_catalog(&governed.cell_view_key)
            .map_or((1, 1), |catalog| {
                catalog
                    .page_number_and_count(governed.sheet_id)
                    .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)))
            })
    });
    let automatic_values = title_field_automatic_values(
        state,
        &sheet_name,
        page,
        page_count,
        &format,
        &document_control,
    );
    state.dialogs.drawing_sheet_support.title_fields = TitleBlockFieldsState {
        open: true,
        staged_page_setup: true,
        authority: Some(authority),
        sheet_name,
        format: Some(format),
        baseline: fields.clone(),
        draft: fields,
        baseline_project: project_fields.clone(),
        draft_project: project_fields,
        saved_document_control,
        baseline_document_control: document_control.clone(),
        draft_document_control: document_control,
        automatic_values,
        error: None,
    };
    Ok(())
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_support_dialogs(&mut self, ctx: &Context) {
        self.render_sheet_format_manager(ctx);
        self.render_title_block_fields(ctx);
    }

    fn render_sheet_format_manager(&mut self, ctx: &Context) {
        if !self.state.dialogs.drawing_sheet_support.manager.open {
            return;
        }
        let authority_error = validate_manager_authority(
            &self.state,
            &self.state.dialogs.drawing_sheet_support.manager,
        )
        .err();
        let selected = self
            .state
            .dialogs
            .drawing_sheet_support
            .manager
            .rows
            .iter()
            .any(|row| row.selected);
        let enabled = authority_error.is_none() && selected;
        let eyebrow = format!(
            "DRAWING SHEETS \u{00b7} {}",
            self.state
                .dialogs
                .drawing_sheet_support
                .manager
                .owner_key
                .to_uppercase()
        );
        let choice = Dialog::new(
            &eyebrow,
            "Sheet formats",
            "Apply to selected sheets",
        )
        .description("Compare and reconcile the authored drawing sheet of every sheet in this schematic. Page numbering, print sets and exports all read these formats.")
        .size(DialogSize::DrawingSheetWorkflow)
        .ghost("Close")
        .primary_enabled(enabled)
        .show(ctx, |ui| {
            sheet_format_manager_body(
                ui,
                &mut self.state.dialogs.drawing_sheet_support.manager,
                authority_error.as_deref(),
            );
        });
        match choice {
            DialogChoice::Primary
                if self.state.dialogs.drawing_sheet_support.manager.source
                    == SheetManagerFormatSource::PageSetup =>
            {
                self.state.dialogs.drawing_sheet_support.manager = Default::default();
                crate::workbench::app::open_drawing_sheet_setup(self);
            }
            DialogChoice::Primary => match apply_sheet_format_manager(self) {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                    self.state.dialogs.drawing_sheet_support.manager = Default::default();
                }
                Err(error) => self.state.dialogs.drawing_sheet_support.manager.error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_support.manager = Default::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn render_title_block_fields(&mut self, ctx: &Context) {
        if !self.state.dialogs.drawing_sheet_support.title_fields.open {
            return;
        }
        let authority_error = self
            .state
            .dialogs
            .drawing_sheet_support
            .title_fields
            .authority
            .as_ref()
            .ok_or_else(|| "Title-block Fields has no active sheet authority.".to_owned())
            .and_then(|authority| validate_drawing_sheet_authority(&self.state, authority))
            .err();
        let dirty = {
            let state = &self.state.dialogs.drawing_sheet_support.title_fields;
            title_block_fields_changed(state)
        };
        let required_field_error =
            required_title_field_error(&self.state.dialogs.drawing_sheet_support.title_fields);
        let eyebrow = format!(
            "DRAWING SHEET \u{00b7} {} \u{00b7} FIELD OWNERSHIP",
            self.state
                .dialogs
                .drawing_sheet_support
                .title_fields
                .sheet_name
                .to_uppercase()
        );
        let choice = Dialog::new(
            &eyebrow,
            "Title-block fields",
            "Save title-block fields",
        )
        .description("Edit the identity values printed in the current sheet's title block. Automatic values remain linked to their owning project or document source.")
        .size(DialogSize::DrawingSheetWorkflow)
        // The body owns its own section rules; the dialog's default inset put a
        // band of elevated background around the whole page on top of them.
        .flush_body()
        .ghost(if dirty { "Discard changes" } else { "Cancel" })
        .primary_enabled(authority_error.is_none() && required_field_error.is_none() && dirty)
        .show(ctx, |ui| {
            let validation_error = authority_error
                .as_deref()
                .or(required_field_error.as_deref());
            title_block_fields_body(
                ui,
                &mut self
                    .state
                    .dialogs
                    .drawing_sheet_support
                    .title_fields,
                validation_error,
            );
        });
        match choice {
            DialogChoice::Primary => match apply_title_block_fields(self) {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                    self.state.dialogs.drawing_sheet_support.title_fields = Default::default();
                }
                Err(error) => {
                    self.state.dialogs.drawing_sheet_support.title_fields.error = Some(error);
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_support.title_fields = Default::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn sheet_format_manager_body(
    ui: &mut Ui,
    state: &mut SheetFormatManagerState,
    authority_error: Option<&str>,
) {
    let t = Tokens::get(ui.ctx());
    manager_thumbnails(ui, &state.rows, &state.preview_content);
    ui.add_space(10.0);
    ScrollArea::horizontal()
        .id_salt("sheet-format-manager-table")
        .show(ui, |ui| {
            Grid::new("sheet-format-manager-grid")
                .striped(true)
                .min_col_width(82.0)
                .spacing(vec2(14.0, 8.0))
                .show(ui, |ui| {
                    for heading in [
                        "Sheet",
                        "Page",
                        "Format",
                        "Dimensions",
                        "Zones",
                        "Title block",
                        "Source",
                        "State",
                    ] {
                        ui.label(
                            RichText::new(heading)
                                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                .color(t.color.text_dim),
                        );
                    }
                    ui.end_row();
                    for row in &mut state.rows {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut row.selected, "");
                            ui.label(&row.name);
                        });
                        ui.monospace(format!("{} / {}", row.page, row.page_count));
                        ui.label(row.format.authored_size.label());
                        let (width, height) = row.format.oriented_dimensions_um();
                        ui.monospace(row.format.display_unit.format_size_um(width, height));
                        ui.monospace(
                            row.format
                                .geometry()
                                .ok()
                                .and_then(|geometry| geometry.zones)
                                .map_or_else(
                                    || "\u{2014}".to_owned(),
                                    |zones| format!("{} \u{00d7} {}", zones.columns, zones.rows),
                                ),
                        );
                        ui.label(title_template_label(row.format.title_block.template));
                        ui.label(inheritance_label(row.format.inheritance));
                        ui.label(if format_is_organization_managed(&row.format) {
                            "managed · skipped"
                        } else if row.active {
                            "open"
                        } else if !row.has_geometry {
                            "no geometry yet"
                        } else {
                            "drawn"
                        });
                        ui.end_row();
                    }
                });
        });
    ui.add_space(14.0);
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(11))
        .show(ui, |ui| {
            if sheet_format_manager_controls_stack(ui.available_width()) {
                manager_format_selector(ui, state, &t);
                ui.add_space(12.0);
                manager_apply_notes(ui);
            } else {
                ui.columns(2, |columns| {
                    manager_format_selector(&mut columns[0], state, &t);
                    manager_apply_notes(&mut columns[1]);
                });
            }
        });
    ui.add_space(10.0);
    let error = authority_error.or(state.error.as_deref());
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 38.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            if let Some(error) = error {
                ui.colored_label(t.color.err, error);
            } else if !state.rows.iter().any(|row| row.selected) {
                ui.colored_label(t.color.warn, "Select at least one sheet.");
            }
        },
    );
}

fn sheet_format_manager_controls_stack(available_width: f32) -> bool {
    available_width < 620.0
}

fn manager_format_selector(ui: &mut Ui, state: &mut SheetFormatManagerState, t: &Tokens) {
    ui.label(
        RichText::new("FORMAT TO APPLY")
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    ui.add_space(6.0);
    egui::ComboBox::from_id_salt("sheet-format-manager-apply")
        .width(ui.available_width())
        .selected_text(manager_source_label(state))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut state.source,
                SheetManagerFormatSource::ActiveSheet,
                format!(
                    "Match the open sheet \u{00b7} {}",
                    state
                        .active_format
                        .as_ref()
                        .map_or_else(|| "unavailable".to_owned(), format_label_with_dimensions,)
                ),
            );
            ui.selectable_value(
                &mut state.source,
                SheetManagerFormatSource::ProjectDefault,
                format!(
                    "Project default \u{00b7} {}",
                    state
                        .project_default
                        .as_ref()
                        .map_or_else(|| "unavailable".to_owned(), format_label_with_dimensions,)
                ),
            );
            ui.selectable_value(
                &mut state.source,
                SheetManagerFormatSource::PageSetup,
                "Open Page Setup instead\u{2026}",
            );
        });
}

fn manager_apply_notes(ui: &mut Ui) {
    manager_apply_note(
        ui,
        "Partial application is reported, never silent.",
        "A sheet governed by an organization template is skipped and named in the receipt; sheets that did change are not rolled back.",
    );
    ui.add_space(7.0);
    manager_apply_note(
        ui,
        "One undo entry.",
        "Applying to several sheets is one transaction: undo restores every sheet's previous format together.",
    );
}

fn title_block_fields_body(
    ui: &mut Ui,
    state: &mut TitleBlockFieldsState,
    authority_error: Option<&str>,
) {
    let t = Tokens::get(ui.ctx());
    let managed = state.format.as_ref().is_some_and(|format| {
        format.title_block.template
            == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
    });
    let field_order = state.format.as_ref().map_or_else(
        || DrawingSheetTitleFieldId::ALL.to_vec(),
        |format| format.title_block_field_order().to_vec(),
    );
    let locked_fields = state
        .format
        .as_ref()
        .and_then(|format| format.title_block.managed_template.as_ref())
        .map_or_else(Vec::new, |snapshot| {
            field_order
                .iter()
                .copied()
                .filter(|field| snapshot.locks_field(*field))
                .collect()
        });
    // Section rhythm is the reference's: the concept banner is one 8 pt row
    // with its heading and detail side by side, the summary is a bordered
    // sheet-desk card with 11 pt padding and an 11 pt lead below it, and the
    // table and closing notes then run full width.
    ui.spacing_mut().item_spacing.y = 0.0;
    Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            let heading = if managed {
                "The organization owns the captured template, artwork, and field order."
            } else {
                "Field provenance remains visible in the printed block."
            };
            let detail = if managed {
                "Authorized sheet and project values remain editable; digest-covered artwork, managed labels, required fields, order, and organization-policy values are locked."
            } else {
                "Automatic fields update from their named source. Editable fields are saved with the sheet or project and never silently replaced by a generated value."
            };
            if title_provenance_banner_stacks(ui.available_width()) {
                ui.add(
                    egui::Label::new(
                        RichText::new(heading)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
                ui.add_space(4.0);
                ui.add(
                    egui::Label::new(
                        RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            } else {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 7.0;
                    ui.label(
                        RichText::new(heading)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.add(
                        egui::Label::new(
                            RichText::new(detail)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                });
            }
        });
    document_control_editor(ui, state);
    title_field_summary(ui, state);
    title_field_table(ui, state, managed, &field_order, &locked_fields);
    Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        title_block_field_notes(ui, authority_error.or(state.error.as_deref()));
    });
}

fn title_provenance_banner_stacks(available_width: f32) -> bool {
    available_width < 520.0
}

fn document_control_editor(ui: &mut Ui, state: &mut TitleBlockFieldsState) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(
                RichText::new("Document control")
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.add_space(7.0);
            if ui.available_width() < 520.0 {
                document_control_stacked_fields(ui, state, &t);
            } else {
                Grid::new("drawing-sheet-document-control")
                    .num_columns(2)
                    .spacing(vec2(12.0, 7.0))
                    .show(ui, |ui| {
                        ui.label("Revision");
                        document_revision_input(ui, state, &t);
                        ui.end_row();

                        ui.label("Status");
                        document_status_input(ui, state);
                        ui.end_row();

                        ui.label("Revision date (UTC)");
                        document_date_input(ui, state, &t);
                        ui.end_row();

                        ui.label("Change reference");
                        document_change_reference_input(ui, state, &t);
                        ui.end_row();
                    });
            }
            ui.add_space(7.0);
            ui.label(
                RichText::new(
                    "Revision and date are project-owned publication values. Released and obsolete documents require a dated, non-DRAFT revision.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
    ui.add_space(10.0);
}

fn document_control_stacked_fields(ui: &mut Ui, state: &mut TitleBlockFieldsState, t: &Tokens) {
    ui.label("Revision");
    document_revision_input(ui, state, t);
    ui.add_space(7.0);
    ui.label("Status");
    document_status_input(ui, state);
    ui.add_space(7.0);
    ui.label("Revision date (UTC)");
    document_date_input(ui, state, t);
    ui.add_space(7.0);
    ui.label("Change reference");
    document_change_reference_input(ui, state, t);
}

fn document_revision_input(ui: &mut Ui, state: &mut TitleBlockFieldsState, t: &Tokens) {
    ui.add_sized(
        vec2(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(&mut state.draft_document_control.revision)
            .char_limit(MAX_DRAWING_SHEET_REVISION_BYTES),
    );
}

fn document_status_input(ui: &mut Ui, state: &mut TitleBlockFieldsState) {
    ComboBox::from_id_salt("drawing-sheet-release-status")
        .width(ui.available_width().max(140.0))
        .selected_text(state.draft_document_control.status.label())
        .show_ui(ui, |ui| {
            for status in DrawingSheetReleaseStatus::ALL {
                ui.selectable_value(
                    &mut state.draft_document_control.status,
                    status,
                    status.label(),
                );
            }
        });
}

fn document_date_input(ui: &mut Ui, state: &mut TitleBlockFieldsState, t: &Tokens) {
    ui.add_sized(
        vec2(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(&mut state.draft_document_control.revision_date_utc)
            .hint_text("YYYY-MM-DD")
            .char_limit(10),
    );
}

fn document_change_reference_input(ui: &mut Ui, state: &mut TitleBlockFieldsState, t: &Tokens) {
    ui.add_sized(
        vec2(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(&mut state.draft_document_control.change_reference)
            .hint_text("ECO, ticket, or release record")
            .char_limit(MAX_DRAWING_SHEET_CHANGE_REFERENCE_BYTES),
    );
}

fn title_block_field_notes(ui: &mut Ui, error: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    if let Some(error) = error {
        ui.colored_label(t.color.err, error);
        ui.add_space(8.0);
    }
    let notes = [
        (
            "Overflow",
            "Long values keep their full saved text. The printed cell truncates with an ellipsis and Page Setup reports the exact field; RSpice never shrinks one field below the template's type size.",
        ),
        (
            "Localization",
            "Labels come from the selected title-block template. Engineering identifiers and revision values preserve their source spelling and direction.",
        ),
        (
            "Revision history",
            "Saving these values is one presentation transaction and does not change connectivity, checks, simulations, or retained results.",
        ),
    ];
    if ui.available_width() < 720.0 {
        for (index, (heading, body)) in notes.iter().enumerate() {
            note(ui, heading, body);
            if index + 1 < notes.len() {
                ui.add_space(8.0);
            }
        }
    } else {
        ui.columns(3, |columns| {
            for (column, (heading, body)) in columns.iter_mut().zip(notes) {
                note(column, heading, body);
            }
        });
    }
}

const TITLE_FIELD_TABLE_MIN_WIDTH: f32 = 760.0;
const TITLE_FIELD_HEADER_HEIGHT: f32 = 27.0;
/// The reference row is 29 pt around a 22 pt input. Ours is stated against the
/// live control height instead of a constant, so a row can never be shorter
/// than the control it holds — a row that has to grow to fit its own cell
/// leaves the table without a common rhythm.
const TITLE_FIELD_ROW_PADDING: f32 = 7.0;

fn title_field_table(
    ui: &mut Ui,
    state: &mut TitleBlockFieldsState,
    managed: bool,
    field_order: &[DrawingSheetTitleFieldId],
    locked_fields: &[DrawingSheetTitleFieldId],
) {
    let t = Tokens::get(ui.ctx());
    let table_width = ui.available_width().max(TITLE_FIELD_TABLE_MIN_WIDTH);
    ScrollArea::horizontal()
        .id_salt("title-block-fields-table-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(table_width);
            // The reference splits the table 24 / 42 / 19.8 / remainder. Giving
            // the last column the remainder is what keeps the striped rows from
            // running past the final cell into empty table.
            let row_height = t.metrics.ctl_h + TITLE_FIELD_ROW_PADDING;
            let field_width = (table_width * 0.24).max(180.0);
            let value_width = (table_width * 0.42).max(300.0);
            let owner_width = (table_width * 0.198).max(140.0);
            TableBuilder::new(ui)
                .id_salt("title-block-fields-table")
                .striped(true)
                .vscroll(false)
                .cell_layout(Layout::left_to_right(Align::Center))
                .column(Column::initial(field_width).at_least(180.0))
                .column(Column::initial(value_width).at_least(300.0))
                .column(Column::initial(owner_width).at_least(140.0))
                .column(Column::remainder().at_least(120.0))
                .header(TITLE_FIELD_HEADER_HEIGHT, |mut header| {
                    for heading in ["Field", "Value", "Owner", "State"] {
                        header.col(|ui| {
                            ui.painter()
                                .rect_filled(ui.max_rect(), 0.0, t.color.bg_panel_2);
                            ui.label(
                                RichText::new(heading)
                                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                    .color(t.color.text_dim),
                            );
                        });
                    }
                })
                .body(|mut body| {
                    for &id in field_order {
                        let policy = id.policy();
                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    let field = state.draft.entry(id).or_default();
                                    if policy.required_visible {
                                        field.visible = true;
                                        let mut visible = true;
                                        ui.add_enabled(
                                            false,
                                            egui::Checkbox::without_text(&mut visible),
                                        );
                                    } else {
                                        ui.checkbox(&mut field.visible, "");
                                    }
                                    ui.label(title_field_label(id));
                                });
                            });
                            row.col(|ui| match policy.value_authority {
                                DrawingSheetTitleFieldValueAuthority::Automatic => {
                                    automatic_field_readout(
                                        ui,
                                        state.automatic_values.entry(id).or_default(),
                                    );
                                    state.draft.entry(id).or_default().value.clear();
                                }
                                DrawingSheetTitleFieldValueAuthority::Authored => {
                                    let policy_locked = managed && locked_fields.contains(&id);
                                    let size = vec2(ui.available_width(), t.metrics.ctl_h);
                                    if id.is_project_owned() {
                                        ui.add_enabled_ui(!policy_locked, |ui| {
                                            ui.add_sized(
                                                size,
                                                TextEdit::singleline(
                                                    state.draft_project.entry(id).or_default(),
                                                )
                                                .char_limit(256),
                                            );
                                        });
                                    } else {
                                        ui.add_sized(
                                            size,
                                            TextEdit::singleline(
                                                &mut state.draft.entry(id).or_default().value,
                                            )
                                            .char_limit(256),
                                        );
                                    }
                                }
                            });
                            row.col(|ui| {
                                ui.label(title_field_owner(id));
                            });
                            row.col(|ui| {
                                let (status, color) = if policy.value_authority
                                    == DrawingSheetTitleFieldValueAuthority::Automatic
                                {
                                    ("automatic", t.color.ok)
                                } else if policy.required_visible {
                                    ("required", t.color.warn)
                                } else {
                                    ("editable", t.color.text_dim)
                                };
                                field_state_badge(ui, status, color);
                            });
                        });
                    }
                });
        });
}

fn automatic_field_readout(ui: &mut Ui, value: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(8, 5))
        .show(ui, |ui| {
            // Every Value cell is one control height. Three different heights
            // across the column made the table's rows grow by different
            // amounts and lose their common rhythm.
            ui.set_min_size(vec2(ui.available_width(), t.metrics.ctl_h - 10.0));
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(16.0, 14.0), egui::Sense::hover());
                let left = rect.center() + vec2(-3.0, 0.0);
                let right = rect.center() + vec2(3.0, 0.0);
                ui.painter()
                    .circle_stroke(left, 4.0, Stroke::new(1.0, t.color.accent));
                ui.painter()
                    .circle_stroke(right, 4.0, Stroke::new(1.0, t.color.accent));
                ui.label(
                    RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                );
            });
        });
}

/// The row's state, stated by the word and its colour alone. The reference
/// sets it as plain mono text: a filled and stroked chip around every row of a
/// fourteen-row table reads as a column of buttons.
fn field_state_badge(ui: &mut Ui, label: &str, color: egui::Color32) {
    ui.label(
        RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(color),
    );
}

fn validate_manager_authority(
    app: &AppState,
    state: &SheetFormatManagerState,
) -> Result<(), String> {
    state
        .edit
        .as_ref()
        .ok_or_else(|| "Sheet Format Manager has no edit authority.".to_owned())?
        .validate(app, "Sheet Format Manager")?;
    if app.workspace.active_key() != state.owner_key {
        return Err("The active cell/view changed. Close and reopen Sheet Formats.".to_owned());
    }
    if app.workspace.design_management.revision() != state.design_management_revision {
        return Err(
            "Drawing-sheet project policy changed. Close and reopen Sheet Formats.".to_owned(),
        );
    }
    let catalog = app
        .workspace
        .design_management
        .sheet_catalog(&state.owner_key)
        .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?;
    if catalog.revision() != state.catalog_revision {
        return Err("The sheet catalog changed. Close and reopen Sheet Formats.".to_owned());
    }
    Ok(())
}

fn apply_sheet_format_manager(app: &mut RSpiceApp) -> Result<String, String> {
    let transaction = app.state.dialogs.drawing_sheet_support.manager.clone();
    validate_manager_authority(&app.state, &transaction)?;
    let selected = transaction
        .rows
        .iter()
        .filter(|row| row.selected)
        .collect::<Vec<_>>();
    if selected.is_empty() {
        return Err("Select at least one sheet.".to_owned());
    }
    let selected_sheet_ids = selected.iter().map(|row| row.id).collect::<Vec<_>>();
    let mut format = match transaction.source {
        SheetManagerFormatSource::ActiveSheet => transaction
            .active_format
            .clone()
            .ok_or_else(|| "The open sheet format is unavailable.".to_owned())?,
        SheetManagerFormatSource::ProjectDefault => transaction
            .project_default
            .clone()
            .ok_or_else(|| "The project drawing-sheet default is unavailable.".to_owned())?,
        SheetManagerFormatSource::PageSetup => {
            return Err("Open Page Setup to choose the format.".to_owned());
        }
    };
    format = format
        .try_update(|draft| {
            draft.inheritance = match transaction.source {
                SheetManagerFormatSource::ActiveSheet => DrawingSheetInheritance::Explicit,
                SheetManagerFormatSource::ProjectDefault => DrawingSheetInheritance::ProjectDefault,
                SheetManagerFormatSource::PageSetup => DrawingSheetInheritance::Explicit,
            };
        })
        .map_err(|error| error.to_string())?;
    format = format.without_project_owned_title_values();
    let before = app.state.workspace.design_management.clone();
    let mut candidate = before.clone();
    let mut changed = 0_usize;
    let mut applied_sheet_ids = Vec::new();
    let mut unchanged_sheet_ids = Vec::new();
    let mut skipped = Vec::new();
    for row in selected {
        if format_is_organization_managed(&row.format) {
            skipped.push(DrawingSheetTransactionSkip {
                sheet_id: row.id,
                sheet_name: row.name.clone(),
                reason: "Organization-managed drawing-sheet format".to_owned(),
            });
            continue;
        }
        let applied_format = format.with_target_sheet_title_fields(&row.format);
        if candidate
            .sheet_catalog(&transaction.owner_key)
            .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?
            .find(row.id)
            .is_some_and(|sheet| sheet.page_format() == &applied_format)
        {
            unchanged_sheet_ids.push(row.id);
            continue;
        }
        let mut row_candidate = candidate.clone();
        let result = row_candidate
            .sheet_catalog_mut(&transaction.owner_key)
            .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?
            .update_sheet_page_format(row.id, row.revision, applied_format);
        match result {
            Ok(_) => {
                candidate = row_candidate;
                changed += 1;
                applied_sheet_ids.push(row.id);
            }
            Err(error) => skipped.push(DrawingSheetTransactionSkip {
                sheet_id: row.id,
                sheet_name: row.name.clone(),
                reason: error.to_string(),
            }),
        }
    }
    if changed == 0 {
        if skipped.is_empty() {
            return Ok("Every selected sheet already used that format.".to_owned());
        }
        let skipped_summary = format_sheet_transaction_skips(&skipped);
        record_manager_transaction(
            &mut candidate,
            &transaction.owner_key,
            &format,
            selected_sheet_ids,
            applied_sheet_ids,
            unchanged_sheet_ids,
            skipped,
        )?;
        commit_candidate(app, "Sheet formats", before, candidate)?;
        return Ok(format!(
            "No selected sheets changed. Skipped sheets were recorded: {skipped_summary}."
        ));
    }
    let skipped_summary = format_sheet_transaction_skips(&skipped);
    record_manager_transaction(
        &mut candidate,
        &transaction.owner_key,
        &format,
        selected_sheet_ids,
        applied_sheet_ids,
        unchanged_sheet_ids,
        skipped,
    )?;
    commit_candidate(app, "Sheet formats", before, candidate)?;
    let mut message = format!(
        "Sheet format applied to {changed} {}.",
        if changed == 1 { "sheet" } else { "sheets" }
    );
    if !skipped_summary.is_empty() {
        message.push_str(&format!(" Skipped sheets: {skipped_summary}."));
    }
    Ok(message)
}

fn record_manager_transaction(
    candidate: &mut crate::state::DesignManagementCatalog,
    owner_cell_view_key: &str,
    source_format: &SchematicSheetFormat,
    selected_sheet_ids: Vec<SheetId>,
    applied_sheet_ids: Vec<SheetId>,
    unchanged_sheet_ids: Vec<SheetId>,
    skipped: Vec<DrawingSheetTransactionSkip>,
) -> Result<(), String> {
    let catalog_revision = candidate
        .revision()
        .checked_add(1)
        .ok_or_else(|| "Drawing-sheet receipt revision space is exhausted.".to_owned())?;
    let receipt = DrawingSheetTransactionReceipt {
        catalog_revision,
        kind: DrawingSheetTransactionKind::SheetFormatManager,
        owner_cell_view_key: owner_cell_view_key.to_owned(),
        source_format_digest: source_format
            .content_digest()
            .map_err(|error| error.to_string())?,
        selected_sheet_ids,
        applied_sheet_ids,
        unchanged_sheet_ids,
        skipped,
        project_default_changed: false,
        project_preset_saved: false,
        project_settings_changed: false,
    };
    candidate
        .record_drawing_sheet_transaction(candidate.revision(), receipt)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn format_sheet_transaction_skips(skipped: &[DrawingSheetTransactionSkip]) -> String {
    skipped
        .iter()
        .map(|entry| format!("{} ({})", entry.sheet_name, entry.reason))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_is_organization_managed(format: &SchematicSheetFormat) -> bool {
    format.border == crate::state::DrawingSheetBorderTemplate::OrganizationManaged
        || format.title_block.template
            == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
}

fn apply_title_block_fields(app: &mut RSpiceApp) -> Result<String, String> {
    let transaction = app.state.dialogs.drawing_sheet_support.title_fields.clone();
    if let Some(error) = required_title_field_error(&transaction) {
        return Err(error);
    }
    let authority = transaction
        .authority
        .as_ref()
        .ok_or_else(|| "Title-block Fields has no active sheet authority.".to_owned())?;
    validate_drawing_sheet_authority(&app.state, authority)?;
    if !title_block_fields_changed(&transaction) {
        return Ok("The title-block fields already matched the saved sheet.".to_owned());
    }
    if let Some(snapshot) = transaction
        .format
        .as_ref()
        .filter(|format| {
            format.title_block.template
                == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
        })
        .and_then(|format| format.title_block.managed_template.as_ref())
        && let Some(field) = transaction
            .format
            .as_ref()
            .expect("managed format was filtered above")
            .title_block_field_order()
            .iter()
            .copied()
            .find(|field| {
                snapshot.locks_field(*field)
                    && managed_title_field_value_changed(&transaction, *field)
            })
    {
        return Err(format!(
            "The organization-managed {} value cannot be edited.",
            field.display_label().to_lowercase()
        ));
    }
    if transaction.staged_page_setup {
        let setup = &mut app.state.dialogs.drawing_sheet_setup;
        let same_transaction = setup.authority.as_ref().is_some_and(|setup_authority| {
            setup_authority.cell_view_key == authority.cell_view_key
                && setup_authority
                    .governed
                    .as_ref()
                    .map(|governed| governed.sheet_id)
                    == authority
                        .governed
                        .as_ref()
                        .map(|governed| governed.sheet_id)
        });
        if !same_transaction {
            return Err(
                "The Page Setup transaction for these title fields is no longer available."
                    .to_owned(),
            );
        }
        setup.draft.title_fields = transaction.draft;
        for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
            let field = setup.draft.title_fields.entry(id).or_default();
            field.value = transaction
                .draft_project
                .get(&id)
                .cloned()
                .unwrap_or_default();
        }
        setup.staged_project_title_values = Some(transaction.draft_project);
        setup.staged_document_control = (transaction.draft_document_control
            != transaction.saved_document_control)
            .then_some(transaction.draft_document_control);
        setup.commit_error = None;
        return Ok(format!(
            "Title-block fields staged for {}. Apply Page Setup to save the drawing sheet.",
            transaction.sheet_name
        ));
    }
    let governed = authority
        .governed
        .as_ref()
        .ok_or_else(|| "Title-block fields require a governed drawing sheet.".to_owned())?;
    let mut format = transaction
        .format
        .ok_or_else(|| "The active sheet format is unavailable.".to_owned())?;
    format = format
        .try_update(|draft| {
            draft.title_block.fields = transaction.draft.clone();
            for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
                draft
                    .title_block
                    .fields
                    .entry(id)
                    .or_default()
                    .value
                    .clear();
            }
        })
        .map_err(|error| error.to_string())?;
    let before = app.state.workspace.design_management.clone();
    let mut candidate = before.clone();
    let mut project_settings = candidate.drawing_sheet_settings().clone();
    let project_changed = project_settings.title_block_field_values != transaction.draft_project;
    let document_control_changed =
        project_settings.document_control != transaction.draft_document_control;
    if project_changed {
        project_settings.title_block_field_values = transaction.draft_project;
    }
    if document_control_changed {
        project_settings.document_control = transaction.draft_document_control;
    }
    if project_changed || document_control_changed {
        candidate
            .update_drawing_sheet_settings(before.revision(), project_settings)
            .map_err(|error| error.to_string())?;
    }
    let current_format = candidate
        .sheet_catalog(&governed.cell_view_key)
        .and_then(|catalog| catalog.find(governed.sheet_id))
        .map(|sheet| sheet.page_format())
        .ok_or_else(|| "The governed sheet is unavailable.".to_owned())?;
    if current_format != &format {
        candidate
            .sheet_catalog_mut(&governed.cell_view_key)
            .ok_or_else(|| "The governed sheet catalog is unavailable.".to_owned())?
            .update_sheet_page_format(governed.sheet_id, governed.sheet_revision, format)
            .map_err(|error| error.to_string())?;
    }
    commit_candidate(app, "Title-block fields", before, candidate)?;
    Ok(format!(
        "Title-block fields saved for {}.",
        transaction.sheet_name
    ))
}

fn required_title_field_error(state: &TitleBlockFieldsState) -> Option<String> {
    if let Err(error) = state.draft_document_control.validate() {
        return Some(error.to_string());
    }
    DrawingSheetTitleFieldId::ALL
        .into_iter()
        .find(|id| {
            let policy = id.policy();
            policy.required_visible
                && policy.value_authority == DrawingSheetTitleFieldValueAuthority::Authored
                && state
                    .draft
                    .get(id)
                    .is_none_or(|field| !field.visible || field.value.trim().is_empty())
        })
        .map(|id| format!("{} is required and cannot be empty.", title_field_label(id)))
}

fn commit_candidate(
    app: &mut RSpiceApp,
    description: &str,
    before: crate::state::DesignManagementCatalog,
    candidate: crate::state::DesignManagementCatalog,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = app
        .state
        .prepare_design_management_schematic_transaction(&candidate)?;
    let owner = app.state.workspace.active_schematic_reference();
    let committed_revision = app
        .state
        .workspace
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    app.state
        .apply_design_management_schematic_transaction(&schematic_tx);
    let after = app.state.workspace.design_management.clone();
    app.state
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: description.to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            committed_revision,
        });
    Ok(())
}

fn title_field_automatic_values(
    state: &AppState,
    sheet_name: &str,
    page: u32,
    page_count: u32,
    format: &SchematicSheetFormat,
    document_control: &DrawingSheetDocumentControl,
) -> BTreeMap<DrawingSheetTitleFieldId, String> {
    let mut values = BTreeMap::new();
    values.insert(
        DrawingSheetTitleFieldId::Project,
        state.workspace.project.display_name().to_owned(),
    );
    values.insert(
        DrawingSheetTitleFieldId::CellView,
        state.workspace.active_display_path(),
    );
    values.insert(DrawingSheetTitleFieldId::SheetTitle, sheet_name.to_owned());
    values.insert(
        DrawingSheetTitleFieldId::Page,
        format!("{page} of {page_count}"),
    );
    values.insert(
        DrawingSheetTitleFieldId::Revision,
        document_control.revision.clone(),
    );
    values.insert(
        DrawingSheetTitleFieldId::Format,
        format!(
            "{} · {}",
            format.authored_size.label(),
            format.orientation.label().to_lowercase()
        ),
    );
    values.insert(
        DrawingSheetTitleFieldId::Scale,
        match format.title_block.scale {
            crate::state::DrawingSheetScale::NotToScale => "NTS".to_owned(),
            crate::state::DrawingSheetScale::Ratio {
                drawing_units,
                reality_units,
            } => format!("{drawing_units}:{reality_units}"),
        },
    );
    values.insert(
        DrawingSheetTitleFieldId::Date,
        document_control.display_date().to_owned(),
    );
    values
}

fn title_field_label(id: DrawingSheetTitleFieldId) -> &'static str {
    id.display_label()
}

fn title_field_owner(id: DrawingSheetTitleFieldId) -> &'static str {
    match id {
        DrawingSheetTitleFieldId::Project => "project",
        DrawingSheetTitleFieldId::CellView => "design identity",
        DrawingSheetTitleFieldId::SheetTitle
        | DrawingSheetTitleFieldId::DrawnBy
        | DrawingSheetTitleFieldId::CheckedBy
        | DrawingSheetTitleFieldId::ApprovedBy => "sheet",
        DrawingSheetTitleFieldId::Scale => "drawing sheet",
        DrawingSheetTitleFieldId::Page => "document order",
        DrawingSheetTitleFieldId::Revision => "working revision",
        DrawingSheetTitleFieldId::Format => "drawing sheet",
        DrawingSheetTitleFieldId::Date => "document",
        DrawingSheetTitleFieldId::Organization | DrawingSheetTitleFieldId::DocumentId => "project",
        DrawingSheetTitleFieldId::Classification => "project policy",
    }
}

fn title_block_fields_changed(state: &TitleBlockFieldsState) -> bool {
    state.draft != state.baseline
        || state.draft_project != state.baseline_project
        || state.draft_document_control != state.baseline_document_control
}

fn managed_title_field_value_changed(
    state: &TitleBlockFieldsState,
    id: DrawingSheetTitleFieldId,
) -> bool {
    if id.is_project_owned() {
        state.draft_project.get(&id) != state.baseline_project.get(&id)
    } else {
        state.draft.get(&id).map(|field| field.value.as_str())
            != state.baseline.get(&id).map(|field| field.value.as_str())
    }
}

fn title_template_label(template: crate::state::DrawingSheetTitleBlockTemplate) -> &'static str {
    match template {
        crate::state::DrawingSheetTitleBlockTemplate::Compact => "RSpice compact",
        crate::state::DrawingSheetTitleBlockTemplate::Standard => "RSpice standard",
        crate::state::DrawingSheetTitleBlockTemplate::Wide => "RSpice wide",
        crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged => "Organization managed",
        crate::state::DrawingSheetTitleBlockTemplate::None => "None",
    }
}

fn manager_thumbnails(
    ui: &mut Ui,
    rows: &[SheetFormatManagerRow],
    preview_content: &DrawingSheetPreviewContent,
) {
    let t = Tokens::get(ui.ctx());
    let largest_edge = rows
        .iter()
        .map(|row| {
            let (width, height) = row.format.oriented_dimensions_um();
            width.max(height)
        })
        .max()
        .unwrap_or(1);
    let gap = 9.0;
    let available = ui.available_width().max(140.0);
    let columns = ((available + gap) / (140.0 + gap)).floor().max(1.0);
    let item_width = ((available - gap * (columns - 1.0)) / columns).max(140.0);
    Frame::NONE
        .fill(t.color.canvas_bg)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(11))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(gap, gap);
            ui.horizontal_wrapped(|ui| {
                for row in rows {
                    ui.allocate_ui_with_layout(
                        vec2(item_width, 150.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            let active_fill = egui::Color32::from_rgba_unmultiplied(
                                t.color.accent.r(),
                                t.color.accent.g(),
                                t.color.accent.b(),
                                if row.active { 18 } else { 0 },
                            );
                            Frame::NONE
                                .fill(active_fill)
                                .stroke(Stroke::new(
                                    1.0,
                                    if row.active {
                                        t.color.accent
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    },
                                ))
                                .corner_radius(t.radius)
                                .inner_margin(Margin::same(7))
                                .show(ui, |ui| {
                                    drawing_sheet_preview_at_common_scale(
                                        ui,
                                        &row.format,
                                        120.0,
                                        &format!("{} \u{00b7} {}", row.page, row.name),
                                        row.active.then_some(preview_content),
                                        largest_edge,
                                    );
                                });
                        },
                    );
                }
            });
        });
}

fn title_field_summary(ui: &mut Ui, state: &TitleBlockFieldsState) {
    let Some(format) = &state.format else {
        return;
    };
    let automatic = DrawingSheetTitleFieldId::ALL
        .into_iter()
        .filter(|id| id.policy().value_authority == DrawingSheetTitleFieldValueAuthority::Automatic)
        .count();
    Frame::NONE
        .fill(super::drawing_sheet_preview::sheet_desk_color(ui.ctx()))
        .inner_margin(Margin::same(11))
        .outer_margin(Margin {
            bottom: 11,
            ..Margin::ZERO
        })
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            let total_width = ui.available_width();
            let gap = 13.0;
            if total_width < 940.0 {
                drawing_sheet_preview(ui, format, 142.0, &format_label_with_dimensions(format));
                ui.add_space(gap);
                title_field_summary_facts(ui, format, automatic, total_width, 0.0);
            } else {
                let preview_width = (total_width - gap) * 0.36;
                let facts_width = (total_width - preview_width - gap).max(280.0);
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.allocate_ui_with_layout(
                        vec2(preview_width, 163.0),
                        Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            drawing_sheet_preview(
                                ui,
                                format,
                                142.0,
                                &format_label_with_dimensions(format),
                            );
                        },
                    );
                    title_field_summary_facts(ui, format, automatic, facts_width, 163.0);
                });
            }
        });
}

fn title_field_summary_facts(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    automatic: usize,
    width: f32,
    min_height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let facts = [
        (
            "Template",
            title_template_label(format.title_block.template).to_owned(),
        ),
        (
            "Placement",
            title_anchor_label(format.title_block.anchor).to_owned(),
        ),
        ("Automatic fields", format!("{automatic} linked to source")),
        (
            "Editable fields",
            format!(
                "{} governed values",
                DrawingSheetTitleFieldId::ALL.len() - automatic
            ),
        ),
    ];
    // The reference panel is an unbordered grid on the panel surface whose rows
    // divide its height evenly, so it stands level with the preview beside it
    // instead of hugging four lines of text in the middle of the card.
    let rows = facts.len() as f32;
    let height = min_height.max(rows * 30.0);
    let (rect, _) = ui.allocate_exact_size(vec2(width.max(1.0), height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let row_height = rect.height() / rows;
    let pad_x = 10.0;
    let label_width = (rect.width() - 2.0 * pad_x - 8.0).max(1.0) * 0.36;
    let value_x = rect.left() + pad_x + label_width + 8.0;
    for (index, (label, value)) in facts.iter().enumerate() {
        let top = rect.top() + row_height * index as f32 + 5.0;
        ui.painter().text(
            egui::pos2(rect.left() + pad_x, top),
            egui::Align2::LEFT_TOP,
            label,
            theme::sans(tokens::FS_2, FontWeight::Regular),
            t.color.text_dim,
        );
        ui.painter().text(
            egui::pos2(value_x, top + 2.0),
            egui::Align2::LEFT_TOP,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text,
        );
    }
}

fn manager_source_label(state: &SheetFormatManagerState) -> String {
    match state.source {
        SheetManagerFormatSource::ActiveSheet => format!(
            "Match the open sheet \u{00b7} {}",
            state
                .active_format
                .as_ref()
                .map_or_else(|| "unavailable".to_owned(), format_label_with_dimensions)
        ),
        SheetManagerFormatSource::ProjectDefault => format!(
            "Project default \u{00b7} {}",
            state
                .project_default
                .as_ref()
                .map_or_else(|| "unavailable".to_owned(), format_label_with_dimensions)
        ),
        SheetManagerFormatSource::PageSetup => "Open Page Setup instead\u{2026}".to_owned(),
    }
}

fn format_label_with_dimensions(format: &SchematicSheetFormat) -> String {
    let (width, height) = format.oriented_dimensions_um();
    format!(
        "{} \u{00b7} {}",
        format.authored_size.label(),
        format.display_unit.format_size_um(width, height)
    )
}

/// The closing notes, set exactly as the custom-sheet-size library sets its
/// own: plain cells on the page surface, no box drawn around each one.
fn note(ui: &mut Ui, heading: &str, body: &str) {
    Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(RichText::new(heading).strong());
        ui.add(egui::Label::new(RichText::new(body).weak()).wrap());
    });
}

fn manager_apply_note(ui: &mut Ui, heading: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(heading)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        RichText::new(body)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn title_anchor_label(anchor: crate::state::DrawingSheetTitleBlockAnchor) -> &'static str {
    match anchor {
        crate::state::DrawingSheetTitleBlockAnchor::BottomRight => "bottom right",
        crate::state::DrawingSheetTitleBlockAnchor::BottomLeft => "bottom left",
        crate::state::DrawingSheetTitleBlockAnchor::BottomStrip => "bottom strip",
        crate::state::DrawingSheetTitleBlockAnchor::TopRight => "top right",
    }
}

fn inheritance_label(inheritance: DrawingSheetInheritance) -> &'static str {
    match inheritance {
        DrawingSheetInheritance::Explicit => "this sheet",
        DrawingSheetInheritance::ProjectDefault => "project default",
        DrawingSheetInheritance::UserDefault => "personal default",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        DrawingSheetBorderTemplate, DrawingSheetStandard, SchematicPageOrientation,
        SheetDefinition, SheetPortPolicy, SheetTemplate,
    };

    #[test]
    fn title_provenance_banner_stacks_at_phone_width() {
        assert!(title_provenance_banner_stacks(374.0));
        assert!(title_provenance_banner_stacks(519.0));
        assert!(!title_provenance_banner_stacks(520.0));
        assert!(!title_provenance_banner_stacks(760.0));
    }

    #[test]
    fn sheet_format_manager_controls_stack_at_phone_width() {
        assert!(sheet_format_manager_controls_stack(366.0));
        assert!(sheet_format_manager_controls_stack(619.0));
        assert!(!sheet_format_manager_controls_stack(620.0));
        assert!(!sheet_format_manager_controls_stack(760.0));
    }

    #[test]
    fn manager_refuses_to_open_without_schematic_edit_authority() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        app.state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        app.state.schematic.read_only = true;

        let error = open_sheet_format_manager(&mut app.state)
            .expect_err("read-only documents must not start an edit transaction");

        assert_eq!(error, "The active schematic is read-only.");
        assert!(!app.state.dialogs.drawing_sheet_support.manager.open);
        assert!(
            app.state
                .dialogs
                .drawing_sheet_support
                .manager
                .edit
                .is_none()
        );
    }

    #[test]
    fn manager_partially_applies_and_names_organization_managed_skips() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        let first = app
            .state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        let (managed, ordinary) = {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let managed = catalog
                .create_sheet(
                    SheetDefinition {
                        name: "Managed".to_owned(),
                        template: SheetTemplate::AnalogSchematic,
                        port_policy: SheetPortPolicy::TypedOffSheetPorts,
                        explicit_page_number: Some(2),
                    },
                    Some(first),
                )
                .unwrap();
            let ordinary = catalog
                .create_sheet(
                    SheetDefinition {
                        name: "Ordinary".to_owned(),
                        template: SheetTemplate::AnalogSchematic,
                        port_policy: SheetPortPolicy::TypedOffSheetPorts,
                        explicit_page_number: Some(3),
                    },
                    Some(managed),
                )
                .unwrap();
            (managed, ordinary)
        };
        let source = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA3,
            SchematicPageOrientation::Landscape,
        )
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Sheet 1".to_owned();
        })
        .unwrap();
        let managed_format = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.border = DrawingSheetBorderTemplate::OrganizationManaged;
                draft.marks = DrawingSheetBorderTemplate::OrganizationManaged.default_marks();
                draft
                    .title_block
                    .fields
                    .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                    .unwrap()
                    .value = "Managed".to_owned();
            })
            .unwrap();
        {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let revision = catalog.find(first).unwrap().revision();
            catalog
                .update_sheet_page_format(first, revision, source.clone())
                .unwrap();
            let revision = catalog.find(managed).unwrap().revision();
            catalog
                .update_sheet_page_format(managed, revision, managed_format.clone())
                .unwrap();
            catalog.set_active(first).unwrap();
        }
        let topology = app.state.schematic.topology_version();
        open_sheet_format_manager(&mut app.state).unwrap();
        for row in &mut app.state.dialogs.drawing_sheet_support.manager.rows {
            row.selected = true;
        }

        let message = apply_sheet_format_manager(&mut app).unwrap();

        let catalog = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap();
        assert_eq!(
            catalog.find(managed).unwrap().page_format(),
            &managed_format
        );
        let ordinary_format = catalog.find(ordinary).unwrap().page_format();
        assert_eq!(ordinary_format.authored_size, source.authored_size);
        assert_eq!(ordinary_format.orientation, source.orientation);
        assert_eq!(
            ordinary_format.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
            "Ordinary"
        );
        assert!(message.contains("Managed"));
        let receipt = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .transaction_receipts
            .last()
            .unwrap();
        assert_eq!(
            receipt.kind,
            DrawingSheetTransactionKind::SheetFormatManager
        );
        assert_eq!(receipt.selected_sheet_ids.len(), 3);
        assert_eq!(receipt.applied_sheet_ids, vec![ordinary]);
        assert_eq!(receipt.unchanged_sheet_ids, vec![first]);
        assert_eq!(receipt.skipped[0].sheet_id, managed);
        assert!(app.state.can_undo_project_design());
        assert_eq!(app.state.schematic.topology_version(), topology);
    }

    #[test]
    fn first_sheet_title_fields_stage_into_page_setup_without_early_persistence() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        assert!(
            app.state
                .workspace
                .design_management
                .sheet_catalog(&key)
                .is_none()
        );
        assert!(open_drawing_sheet_setup_for_state(&mut app.state));
        let revision = app.state.workspace.design_management.revision();
        open_title_block_fields(&mut app.state).unwrap();
        let fields = &mut app.state.dialogs.drawing_sheet_support.title_fields;
        assert!(fields.staged_page_setup);
        fields
            .draft
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Bootstrap title".to_owned();
        fields.draft_project.insert(
            DrawingSheetTitleFieldId::Organization,
            "Example Labs".to_owned(),
        );

        apply_title_block_fields(&mut app).unwrap();

        assert_eq!(app.state.workspace.design_management.revision(), revision);
        assert!(
            app.state
                .workspace
                .design_management
                .sheet_catalog(&key)
                .is_none()
        );
        let staged = &app.state.dialogs.drawing_sheet_setup.draft.title_fields;
        assert_eq!(
            staged[&DrawingSheetTitleFieldId::SheetTitle].value,
            "Bootstrap title"
        );
        assert_eq!(
            staged[&DrawingSheetTitleFieldId::Organization].value,
            "Example Labs"
        );
    }

    #[test]
    fn managed_title_field_policy_is_enforced_at_commit_without_partial_persistence() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        let sheet_id = app
            .state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        let managed_format = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.title_block.template =
                    crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged;
                draft
                    .title_block
                    .fields
                    .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                    .expect("canonical title field")
                    .value = "Managed sheet".to_owned();
            })
            .unwrap();
        {
            let catalog = app
                .state
                .workspace
                .design_management
                .sheet_catalog_mut(&key)
                .unwrap();
            let revision = catalog.find(sheet_id).unwrap().revision();
            catalog
                .update_sheet_page_format(sheet_id, revision, managed_format)
                .unwrap();
        }
        let mut settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        settings.title_block_field_values.insert(
            DrawingSheetTitleFieldId::Classification,
            "INTERNAL".to_owned(),
        );
        let revision = app.state.workspace.design_management.revision();
        app.state
            .workspace
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .unwrap();
        let design_revision = app.state.workspace.design_management.revision();

        open_title_block_fields(&mut app.state).unwrap();
        app.state
            .dialogs
            .drawing_sheet_support
            .title_fields
            .draft_project
            .insert(
                DrawingSheetTitleFieldId::Classification,
                "PUBLIC".to_owned(),
            );

        let error = apply_title_block_fields(&mut app)
            .expect_err("commit validation must enforce the managed template lock");

        assert!(error.contains("classification"));
        assert_eq!(
            app.state.workspace.design_management.revision(),
            design_revision,
            "rejected policy edits must not advance project state"
        );
        assert_eq!(
            app.state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .title_block_field_values[&DrawingSheetTitleFieldId::Classification],
            "INTERNAL"
        );
    }

    #[test]
    fn governed_page_setup_stages_nested_title_fields_and_commits_once() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.active_key();
        let sheet_id = app
            .state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        let mut settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        settings.title_block_field_values.insert(
            DrawingSheetTitleFieldId::Organization,
            "Previous organization".to_owned(),
        );
        let revision = app.state.workspace.design_management.revision();
        app.state
            .workspace
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .unwrap();
        assert!(open_drawing_sheet_setup_for_state(&mut app.state));
        app.state.dialogs.drawing_sheet_setup.draft.margin_top = "12".to_owned();
        let design_revision = app.state.workspace.design_management.revision();
        let sheet_revision = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap()
            .find(sheet_id)
            .unwrap()
            .revision();

        open_title_block_fields(&mut app.state).unwrap();
        assert!(
            app.state
                .dialogs
                .drawing_sheet_support
                .title_fields
                .staged_page_setup
        );
        app.state
            .dialogs
            .drawing_sheet_support
            .title_fields
            .draft
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .expect("canonical title field")
            .value = "Nested accepted title".to_owned();
        app.state
            .dialogs
            .drawing_sheet_support
            .title_fields
            .draft_project
            .insert(
                DrawingSheetTitleFieldId::Organization,
                "Released organization".to_owned(),
            );
        app.state
            .dialogs
            .drawing_sheet_support
            .title_fields
            .draft_document_control = DrawingSheetDocumentControl {
            revision: "A".to_owned(),
            revision_date_utc: "2026-08-04".to_owned(),
            status: DrawingSheetReleaseStatus::Released,
            change_reference: "ECO-1042".to_owned(),
        };
        app.state.dialogs.drawing_sheet_setup.open = false;
        app.state.dialogs.drawing_sheet_setup.support_suspended = true;

        apply_title_block_fields(&mut app).unwrap();

        assert_eq!(
            app.state.workspace.design_management.revision(),
            design_revision
        );
        let unchanged_sheet = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap()
            .find(sheet_id)
            .unwrap();
        assert_eq!(unchanged_sheet.revision(), sheet_revision);
        assert_eq!(
            app.state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .document_control,
            DrawingSheetDocumentControl::default(),
            "nested acceptance must not publish document control before Page Setup applies"
        );
        assert_ne!(
            unchanged_sheet.page_format().title_block.fields[&DrawingSheetTitleFieldId::SheetTitle]
                .value,
            "Nested accepted title"
        );
        assert_eq!(
            app.state.dialogs.drawing_sheet_setup.draft.title_fields
                [&DrawingSheetTitleFieldId::SheetTitle]
                .value,
            "Nested accepted title"
        );

        app.state.dialogs.drawing_sheet_setup.open = true;
        app.state.dialogs.drawing_sheet_setup.support_suspended = false;
        crate::workbench::app::dialogs::drawing_sheet_setup::apply_drawing_sheet_setup(&mut app)
            .unwrap();

        let saved = app
            .state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap()
            .find(sheet_id)
            .unwrap()
            .page_format();
        assert_eq!(saved.margins.top_um, 12_000);
        assert_eq!(
            saved.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
            "Nested accepted title"
        );
        assert_eq!(
            app.state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .title_block_field_values[&DrawingSheetTitleFieldId::Organization],
            "Released organization"
        );
        assert_eq!(
            app.state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .document_control,
            DrawingSheetDocumentControl {
                revision: "A".to_owned(),
                revision_date_utc: "2026-08-04".to_owned(),
                status: DrawingSheetReleaseStatus::Released,
                change_reference: "ECO-1042".to_owned(),
            }
        );
        assert!(app.state.can_undo_project_design());
    }
}
