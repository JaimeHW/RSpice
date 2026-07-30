//! Supporting drawing-sheet managers authored by the schematic mockup.
//!
//! Page Setup edits one authored sheet. These surfaces handle document-wide
//! reconciliation and title-field ownership without mixing either operation
//! with print media, export settings, netlisting, or simulation state.

use std::collections::BTreeMap;

use egui::{Context, Frame, Grid, Margin, RichText, ScrollArea, Stroke, TextEdit, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    DrawingSheetInheritance, DrawingSheetTitleFieldId, DrawingSheetTitleFieldState,
    DrawingSheetTitleFieldValueAuthority, DrawingSheetTransactionKind,
    DrawingSheetTransactionReceipt, DrawingSheetTransactionSkip, SchematicSheetFormat, SheetId,
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
    /// True when values are being staged into a not-yet-governed Page Setup
    /// transaction rather than written to an existing sheet catalog.
    pub(crate) staged_page_setup: bool,
    pub(crate) authority: Option<DrawingSheetAuthority>,
    pub(crate) sheet_name: String,
    pub(crate) format: Option<SchematicSheetFormat>,
    pub(crate) baseline: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
    pub(crate) draft: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
    pub(crate) baseline_project: BTreeMap<DrawingSheetTitleFieldId, String>,
    pub(crate) draft_project: BTreeMap<DrawingSheetTitleFieldId, String>,
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
        edit: Some(crate::workbench::app::SchematicEditAuthority::capture(
            state,
        )),
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
    let owner_key = state.workspace.active_key();
    let catalog = state.workspace.design_management.sheet_catalog(&owner_key);
    if catalog.is_none() {
        return open_staged_title_block_fields(state);
    }
    let catalog = catalog.expect("checked governed catalog");
    let sheet = catalog
        .active()
        .ok_or_else(|| "The governed schematic has no active sheet.".to_owned())?;
    let page = catalog
        .sheets()
        .iter()
        .position(|candidate| candidate.id() == sheet.id())
        .map_or(1, |index| index + 1);
    let page_count = catalog.sheets().len();
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
    let automatic_values =
        title_field_automatic_values(state, sheet.name(), page, page_count, &format);
    let mut project_fields = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .title_block_field_values
        .clone();
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
        automatic_values,
        error: None,
    };
    Ok(())
}

fn open_staged_title_block_fields(state: &mut AppState) -> Result<(), String> {
    let setup = &state.dialogs.drawing_sheet_setup;
    let authority = setup
        .authority
        .clone()
        .filter(|authority| authority.governed.is_none())
        .ok_or_else(|| {
            "Open Page Setup before editing title-block fields on the first drawing sheet."
                .to_owned()
        })?;
    if !setup.open {
        return Err(
            "Open Page Setup before editing title-block fields on the first drawing sheet."
                .to_owned(),
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
    let mut project_fields = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .title_block_field_values
        .clone();
    for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
        let project_value = project_fields.entry(id).or_default();
        if project_value.is_empty()
            && let Some(legacy) = fields.get(&id)
            && !legacy.value.trim().is_empty()
        {
            project_value.clone_from(&legacy.value);
        }
    }
    let automatic_values = title_field_automatic_values(state, &sheet_name, 1, 1, &format);
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
            ui.columns(2, |columns| {
                let selector = &mut columns[0];
                selector.label(
                    RichText::new("FORMAT TO APPLY")
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                selector.add_space(6.0);
                egui::ComboBox::from_id_salt("sheet-format-manager-apply")
                    .width(selector.available_width())
                    .selected_text(manager_source_label(state))
                    .show_ui(selector, |ui| {
                        ui.selectable_value(
                            &mut state.source,
                            SheetManagerFormatSource::ActiveSheet,
                            format!(
                                "Match the open sheet \u{00b7} {}",
                                state.active_format.as_ref().map_or_else(
                                    || "unavailable".to_owned(),
                                    format_label_with_dimensions,
                                )
                            ),
                        );
                        ui.selectable_value(
                            &mut state.source,
                            SheetManagerFormatSource::ProjectDefault,
                            format!(
                                "Project default \u{00b7} {}",
                                state.project_default.as_ref().map_or_else(
                                    || "unavailable".to_owned(),
                                    format_label_with_dimensions,
                                )
                            ),
                        );
                        ui.selectable_value(
                            &mut state.source,
                            SheetManagerFormatSource::PageSetup,
                            "Open Page Setup instead\u{2026}",
                        );
                    });

                manager_apply_note(
                    &mut columns[1],
                    "Partial application is reported, never silent.",
                    "A sheet governed by an organization template is skipped and named in the receipt; sheets that did change are not rolled back.",
                );
                columns[1].add_space(7.0);
                manager_apply_note(
                    &mut columns[1],
                    "One undo entry.",
                    "Applying to several sheets is one transaction: undo restores every sheet's previous format together.",
                );
            });
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
    ui.group(|ui| {
        ui.label(
            RichText::new(if managed {
                "The organization owns the template and field order."
            } else {
                "Field provenance remains visible in the printed block."
            })
            .strong(),
        );
        ui.weak(if managed {
            "Authorized sheet and project values remain editable; managed labels, required fields, order, and organization-policy values are locked."
        } else {
            "Automatic fields update from their named source. Editable fields are saved with the sheet or project and never silently replaced by a generated value."
        });
    });
    ui.add_space(10.0);
    title_field_summary(ui, state);
    ui.add_space(10.0);
    Grid::new("title-block-fields-grid")
        .striped(true)
        .min_col_width(110.0)
        .spacing(vec2(14.0, 8.0))
        .show(ui, |ui| {
            for heading in ["Field", "Value", "Owner", "State"] {
                ui.label(
                    RichText::new(heading)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
            }
            ui.end_row();
            for id in DrawingSheetTitleFieldId::ALL {
                let policy = id.policy();
                ui.horizontal(|ui| {
                    let field = state.draft.entry(id).or_default();
                    if policy.required_visible {
                        field.visible = true;
                        let mut visible = true;
                        ui.add_enabled(false, egui::Checkbox::without_text(&mut visible));
                    } else {
                        ui.checkbox(&mut field.visible, "");
                    }
                    ui.label(title_field_label(id));
                });
                match policy.value_authority {
                    DrawingSheetTitleFieldValueAuthority::Automatic => {
                        automatic_field_readout(ui, state.automatic_values.entry(id).or_default());
                        state.draft.entry(id).or_default().value.clear();
                    }
                    DrawingSheetTitleFieldValueAuthority::Authored => {
                        let policy_locked =
                            managed && id == DrawingSheetTitleFieldId::Classification;
                        if id.is_project_owned() {
                            ui.add_enabled(
                                !policy_locked,
                                TextEdit::singleline(state.draft_project.entry(id).or_default())
                                    .desired_width(260.0)
                                    .char_limit(256),
                            );
                        } else {
                            ui.add(
                                TextEdit::singleline(&mut state.draft.entry(id).or_default().value)
                                    .desired_width(260.0)
                                    .char_limit(256),
                            );
                        }
                    }
                }
                ui.label(title_field_owner(id));
                let (status, color) =
                    if policy.value_authority == DrawingSheetTitleFieldValueAuthority::Automatic {
                        ("automatic", t.color.ok)
                    } else if policy.required_visible {
                        ("required", t.color.warn)
                    } else {
                        ("editable", t.color.text_dim)
                    };
                field_state_badge(ui, status, color);
                ui.end_row();
            }
        });
    ui.add_space(10.0);
    if let Some(error) = authority_error.or(state.error.as_deref()) {
        ui.colored_label(t.color.err, error);
    }
    ui.add_space(8.0);
    ui.columns(3, |columns| {
        note(
            &mut columns[0],
            "Overflow",
            "Long values keep their full saved text. The printed cell truncates with an ellipsis and Page Setup reports the exact field; RSpice never shrinks one field below the template's type size.",
        );
        note(
            &mut columns[1],
            "Localization",
            "Labels come from the selected title-block template. Engineering identifiers and revision values preserve their source spelling and direction.",
        );
        note(
            &mut columns[2],
            "Revision history",
            "Saving these values is one presentation transaction and does not change connectivity, checks, simulations, or retained results.",
        );
    });
}

fn automatic_field_readout(ui: &mut Ui, value: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.set_min_width(244.0);
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

fn field_state_badge(ui: &mut Ui, label: &str, color: egui::Color32) {
    Frame::NONE
        .fill(color.gamma_multiply(0.12))
        .stroke(Stroke::new(1.0, color.gamma_multiply(0.45)))
        .corner_radius(2.0)
        .inner_margin(Margin::symmetric(6, 3))
        .show(ui, |ui| {
            ui.label(
                RichText::new(label)
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(color),
            );
        });
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
    if transaction.format.as_ref().is_some_and(|format| {
        format.title_block.template
            == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
    }) {
        if transaction
            .draft_project
            .get(&DrawingSheetTitleFieldId::Classification)
            != transaction
                .baseline_project
                .get(&DrawingSheetTitleFieldId::Classification)
        {
            return Err(
                "The organization-managed classification value cannot be edited.".to_owned(),
            );
        }
    }
    if transaction.staged_page_setup {
        let setup = &mut app.state.dialogs.drawing_sheet_setup;
        if setup
            .authority
            .as_ref()
            .is_none_or(|authority| authority.governed.is_some())
        {
            return Err(
                "The first-sheet Page Setup transaction is no longer available.".to_owned(),
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
        setup.commit_error = None;
        return Ok(format!(
            "Title-block fields staged for {}. Apply Page Setup to save the first drawing sheet.",
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
    if project_changed {
        project_settings.title_block_field_values = transaction.draft_project;
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
    page: usize,
    page_count: usize,
    format: &SchematicSheetFormat,
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
        format!("{page} / {page_count}"),
    );
    values.insert(
        DrawingSheetTitleFieldId::Revision,
        state.workspace.project.revision().get().to_string(),
    );
    values.insert(
        DrawingSheetTitleFieldId::Format,
        format.authored_size.label().to_owned(),
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
        crate::state::automatic_drawing_sheet_date_utc(),
    );
    values
}

fn title_field_label(id: DrawingSheetTitleFieldId) -> &'static str {
    match id {
        DrawingSheetTitleFieldId::Project => "Project",
        DrawingSheetTitleFieldId::CellView => "Cell / view",
        DrawingSheetTitleFieldId::SheetTitle => "Sheet title",
        DrawingSheetTitleFieldId::Page => "Page",
        DrawingSheetTitleFieldId::Revision => "Revision",
        DrawingSheetTitleFieldId::Format => "Format",
        DrawingSheetTitleFieldId::Scale => "Scale",
        DrawingSheetTitleFieldId::DrawnBy => "Drawn by",
        DrawingSheetTitleFieldId::CheckedBy => "Checked by",
        DrawingSheetTitleFieldId::ApprovedBy => "Approved by",
        DrawingSheetTitleFieldId::Date => "Date",
        DrawingSheetTitleFieldId::Organization => "Organization",
        DrawingSheetTitleFieldId::DocumentId => "Document ID",
        DrawingSheetTitleFieldId::Classification => "Classification",
    }
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
    state.draft != state.baseline || state.draft_project != state.baseline_project
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
    let total_width = ui.available_width();
    let preview_width = total_width * (0.72 / 2.0);
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            vec2(preview_width, 176.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                drawing_sheet_preview(ui, format, 142.0, &format_label_with_dimensions(format));
            },
        );
        ui.allocate_ui_with_layout(
            vec2((total_width - preview_width - 8.0).max(280.0), 176.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.label(format!(
                    "Template  {}",
                    title_template_label(format.title_block.template)
                ));
                ui.label(format!(
                    "Placement  {}",
                    title_anchor_label(format.title_block.anchor)
                ));
                let automatic = DrawingSheetTitleFieldId::ALL
                    .into_iter()
                    .filter(|id| {
                        id.policy().value_authority
                            == DrawingSheetTitleFieldValueAuthority::Automatic
                    })
                    .count();
                ui.label(format!("Automatic fields  {automatic} linked"));
                ui.label(format!(
                    "Editable fields  {} governed values",
                    DrawingSheetTitleFieldId::ALL.len() - automatic
                ));
            },
        );
    });
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

fn note(ui: &mut Ui, heading: &str, body: &str) {
    ui.group(|ui| {
        ui.label(RichText::new(heading).strong());
        ui.weak(body);
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
        assert_eq!(catalog.find(ordinary).unwrap().page_format(), &source);
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
}
