//! Reconciling the formats of every sheet in a schematic.
//!
//! The sheet panel edits the open sheet. This surface applies one format to a
//! selected set of the schematic's sheets, without mixing that operation with
//! print media, export settings, netlisting, or simulation state.

use egui::{
    Align, Context, Frame, Layout, Margin, Rect, RichText, ScrollArea, Stroke, Ui, pos2, vec2,
};
use egui_extras::{Column, TableBuilder};

use crate::state::{
    DrawingSheetInheritance, DrawingSheetTransactionKind, DrawingSheetTransactionReceipt,
    DrawingSheetTransactionSkip, SchematicSheetFormat, SheetId,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};

use super::drawing_sheet_preview::{
    DrawingSheetPreviewContent, drawing_sheet_preview_at_common_scale,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SheetManagerFormatSource {
    #[default]
    ActiveSheet,
    ProjectDefault,
    /// Leave the manager for the open sheet's own properties.
    SheetProperties,
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
pub(crate) struct DrawingSheetSupportState {
    pub(crate) manager: SheetFormatManagerState,
}

impl DrawingSheetSupportState {
    pub(crate) fn any_open(&self) -> bool {
        self.manager.open
    }
}

pub(crate) fn open_sheet_format_manager(state: &mut AppState) -> Result<(), String> {
    let designs = &state.workspace.content.design_management;
    let edit = crate::workbench::app::SchematicEditAuthority::capture(state);
    edit.validate(state, "Sheet Format Manager")?;
    let preview_content = DrawingSheetPreviewContent::from_state(state);
    let owner_key = state.workspace.content.active_key();
    let catalog = designs.sheet_catalog(&owner_key).ok_or_else(|| {
        "Give the open sheet a property in Sheet properties before managing document formats."
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
        design_management_revision: designs.revision(),
        catalog_revision: catalog.revision(),
        rows,
        active_format: Some(active_format),
        project_default: Some(designs.drawing_sheet_settings().default_format.clone()),
        preview_content,
        ..SheetFormatManagerState::default()
    };
    Ok(())
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_support_dialogs(&mut self, ctx: &Context) {
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
        let choice = Dialog::new(
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
                    == SheetManagerFormatSource::SheetProperties =>
            {
                self.state.dialogs.drawing_sheet_support.manager = Default::default();
                crate::workbench::app::reveal_sheet_properties(&mut self.state);
            }
            DialogChoice::Primary => match apply_sheet_format_manager(self) {
                Ok(message) => {
                    self.state.prompt(message);
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
}

fn sheet_format_manager_body(
    ui: &mut Ui,
    state: &mut SheetFormatManagerState,
    authority_error: Option<&str>,
) {
    let t = Tokens::get(ui.ctx());
    if manager_thumbnails(ui, &state.rows, &state.preview_content) {
        ui.add_space(11.0);
    }
    manager_format_table(ui, &mut state.rows, &t);
    ui.add_space(11.0);
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(11))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            if sheet_format_manager_controls_stack(ui.available_width()) {
                manager_format_selector(ui, state, &t);
                ui.add_space(MANAGER_APPLY_GAP);
                manager_apply_notes(ui);
            } else {
                // The reference splits this row 0.6fr / 1fr, not in half: the
                // selector is one line of format text, the notes beside it are
                // two paragraphs, and an even split left the selector padded
                // with empty panel while the notes wrapped early.
                let total = ui.available_width();
                let selector_width =
                    ((total - MANAGER_APPLY_GAP) * MANAGER_SELECTOR_SHARE).max(220.0);
                let notes_width = (total - MANAGER_APPLY_GAP - selector_width).max(1.0);
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = MANAGER_APPLY_GAP;
                    ui.allocate_ui_with_layout(
                        vec2(selector_width, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| manager_format_selector(ui, state, &t),
                    );
                    ui.allocate_ui_with_layout(
                        vec2(notes_width, 0.0),
                        Layout::top_down(Align::Min),
                        manager_apply_notes,
                    );
                });
            }
        });
    // Allocated only when there is something to say. A reserved band held an
    // empty strip of dialog above the footer on every well-formed selection.
    let error = authority_error.or(state.error.as_deref());
    if let Some(error) = error {
        ui.add_space(9.0);
        ui.colored_label(t.color.err, error);
    } else if !state.rows.iter().any(|row| row.selected) {
        ui.add_space(9.0);
        ui.colored_label(t.color.warn, "Select at least one sheet.");
    }
}

fn sheet_format_manager_controls_stack(available_width: f32) -> bool {
    available_width < 620.0
}

/// The lead between the selector and the notes beside it, and the selector's
/// share of the row — the reference's `0.6fr / 1fr` split with a 13 pt gap.
const MANAGER_APPLY_GAP: f32 = 13.0;
const MANAGER_SELECTOR_SHARE: f32 = 0.375;

/// Below this the dialog body scrolls sideways rather than shortening a sheet
/// name or an engineering dimension to fit.
const MANAGER_TABLE_MIN_WIDTH: f32 = 960.0;
const MANAGER_HEADER_HEIGHT: f32 = 27.0;
/// The reference's cell gutter, carried by the cell rather than by a gap
/// between cells so the header band and a selected row read as one bar.
const MANAGER_CELL_PAD: f32 = 8.0;
/// Column shares read off the reference table, each with the width its own
/// content needs. The last column takes the remainder instead of a share, which
/// is what carries the header band and a selected row's wash all the way to the
/// right edge instead of stopping at the last cell that holds text.
const MANAGER_COLUMNS: [(&str, f32, f32); 8] = [
    ("Sheet", 0.164, 150.0),
    ("Page", 0.082, 74.0),
    ("Format", 0.129, 116.0),
    ("Dimensions", 0.148, 140.0),
    ("Zones", 0.095, 84.0),
    ("Title block", 0.135, 130.0),
    ("Source", 0.136, 140.0),
    ("State", 0.0, 120.0),
];

/// Every sheet of the document, one row each, across the full width of the
/// dialog. Column widths are stated as shares of that width rather than left to
/// the widest cell: content-sized columns left the whole table hugging the left
/// half of the surface with its rules and row bands stopping in mid-air.
fn manager_format_table(ui: &mut Ui, rows: &mut [SheetFormatManagerRow], t: &Tokens) {
    let table_width = ui.available_width().max(MANAGER_TABLE_MIN_WIDTH);
    ScrollArea::horizontal()
        .id_salt("sheet-format-manager-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(table_width);
            // Cells butt against each other and carry their own gutter, as the
            // reference's do. Inheriting the dialog's item spacing gutter-ed the
            // columns apart and pushed the header band past the body inset.
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let mut table = TableBuilder::new(ui)
                .id_salt("sheet-format-manager")
                .vscroll(false)
                .cell_layout(Layout::left_to_right(Align::Center));
            let (last, leading) = MANAGER_COLUMNS.split_last().expect("eight fixed columns");
            for (_, share, minimum) in leading {
                table = table.column(
                    Column::initial((table_width * share).max(*minimum)).at_least(*minimum),
                );
            }
            table
                .column(Column::remainder().at_least(last.2))
                .header(MANAGER_HEADER_HEIGHT, |mut header| {
                    for (heading, _, _) in MANAGER_COLUMNS {
                        header.col(|ui| {
                            // The band is what separates the headings from the
                            // first row: eight labels alone above the rows read
                            // as a ninth row of data.
                            let band = ui.max_rect();
                            ui.painter().rect_filled(band, 0.0, t.color.bg_panel_2);
                            ui.painter().hline(
                                band.x_range(),
                                band.bottom() - 0.5,
                                Stroke::new(1.0, t.color.border),
                            );
                            ui.add_space(MANAGER_CELL_PAD);
                            ui.label(
                                RichText::new(heading.to_uppercase())
                                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                    .color(t.color.text_dim),
                            );
                        });
                    }
                })
                .body(|mut body| {
                    for row in rows {
                        body.row(t.metrics.row_h, |mut cells| {
                            let selected = row.selected;
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, true);
                                ui.checkbox(&mut row.selected, "");
                                ui.add_space(MANAGER_CELL_PAD);
                                ui.label(&row.name);
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                ui.monospace(format!("{} / {}", row.page, row.page_count));
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                ui.label(row.format.authored_size.label());
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                let (width, height) = row.format.oriented_dimensions_um();
                                ui.monospace(row.format.display_unit.format_size_um(width, height));
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                ui.monospace(
                                    row.format
                                        .geometry()
                                        .ok()
                                        .and_then(|geometry| geometry.zones)
                                        .map_or_else(
                                            || "\u{2014}".to_owned(),
                                            |zones| {
                                                format!("{} \u{00d7} {}", zones.columns, zones.rows)
                                            },
                                        ),
                                );
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                ui.label(
                                    row.format.title_block_template_label(
                                        row.format.title_block.template,
                                    ),
                                );
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                ui.label(inheritance_label(row.format.inheritance));
                            });
                            cells.col(|ui| {
                                manager_cell(ui, t, selected, false);
                                let (state, color) = if format_is_organization_managed(&row.format)
                                {
                                    ("managed \u{00b7} skipped", t.color.warn)
                                } else if row.active {
                                    ("open", t.color.ok)
                                } else if row.has_geometry {
                                    ("drawn", t.color.text_dim)
                                } else {
                                    ("no geometry yet", t.color.text_faint)
                                };
                                ui.label(RichText::new(state).color(color));
                            });
                        });
                    }
                });
        });
}

/// One table cell's own furniture, painted before its content so every mark
/// stays under the text: the row rule that separates it from the next row, the
/// accent wash the reference gives a selected row, the 2 pt accent rule down
/// that row's leading edge, and the cell's leading gutter.
fn manager_cell(ui: &mut Ui, t: &Tokens, selected: bool, leading: bool) {
    let rect = ui.max_rect();
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        if leading {
            ui.painter().rect_filled(
                Rect::from_min_max(rect.min, pos2(rect.left() + 2.0, rect.bottom())),
                0.0,
                t.color.accent,
            );
        }
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    ui.add_space(MANAGER_CELL_PAD);
}

fn manager_format_selector(ui: &mut Ui, state: &mut SheetFormatManagerState, t: &Tokens) {
    // A field label, set as every other field label on these surfaces is. The
    // reference does not shout it: a mono all-caps kicker over one select reads
    // as a section heading for a section that is a single control.
    ui.label(
        RichText::new("Format to apply")
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
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
                SheetManagerFormatSource::SheetProperties,
                SHEET_PROPERTIES_INSTEAD,
            );
        });
}

/// The selector's way out to the open sheet's own properties.
const SHEET_PROPERTIES_INSTEAD: &str = "Edit the open sheet's properties instead\u{2026}";

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

fn validate_manager_authority(
    app: &AppState,
    state: &SheetFormatManagerState,
) -> Result<(), String> {
    let designs = &app.workspace.content.design_management;
    state
        .edit
        .as_ref()
        .ok_or_else(|| "Sheet Format Manager has no edit authority.".to_owned())?
        .validate(app, "Sheet Format Manager")?;
    if app.workspace.content.active_key() != state.owner_key {
        return Err("The active cell/view changed. Close and reopen Sheet Formats.".to_owned());
    }
    if designs.revision() != state.design_management_revision {
        return Err(
            "Drawing-sheet project policy changed. Close and reopen Sheet Formats.".to_owned(),
        );
    }
    let catalog = designs
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
    let (format, inheritance) = match transaction.source {
        SheetManagerFormatSource::ActiveSheet => (
            transaction
                .active_format
                .clone()
                .ok_or_else(|| "The open sheet format is unavailable.".to_owned())?,
            DrawingSheetInheritance::Explicit,
        ),
        SheetManagerFormatSource::ProjectDefault => (
            transaction
                .project_default
                .clone()
                .ok_or_else(|| "The project drawing-sheet default is unavailable.".to_owned())?,
            DrawingSheetInheritance::ProjectDefault,
        ),
        SheetManagerFormatSource::SheetProperties => {
            return Err("Edit the open sheet in Sheet properties.".to_owned());
        }
    };
    let format = format
        .try_update(|draft| draft.inheritance = inheritance)
        .map_err(|error| error.to_string())?
        .without_project_owned_title_values();
    let before = (*app.state.workspace.content.design_management).clone();
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
    if changed == 0 && skipped.is_empty() {
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
    if changed == 0 {
        return Ok(format!(
            "No selected sheets changed. Skipped sheets were recorded: {skipped_summary}."
        ));
    }
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
    format.title_block.template == crate::state::DrawingSheetTitleBlockTemplate::OrganizationManaged
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
    let owner = app.state.workspace.content.active_schematic_reference();
    let committed_revision = app
        .state
        .workspace
        .content
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    app.state
        .apply_design_management_schematic_transaction(&schematic_tx);
    let after = (*app.state.workspace.content.design_management).clone();
    app.state
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: description.to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            references: schematic_tx.references,
            committed_revision,
        });
    Ok(())
}

/// Strip geometry: the reference's 9 pt gutter, 140 pt minimum track and 7 pt
/// card padding, with the page drawn between these two heights.
const MANAGER_THUMB_GAP: f32 = 9.0;
const MANAGER_THUMB_MIN_WIDTH: f32 = 140.0;
const MANAGER_THUMB_MIN_PREVIEW: f32 = 120.0;
const MANAGER_THUMB_MAX_PREVIEW: f32 = 280.0;
const MANAGER_THUMB_PADDING: i8 = 7;
/// What the preview itself spends on desk margin and its caption track, held
/// here so a card can be sized to the page it will actually draw.
const MANAGER_THUMB_PREVIEW_INSET_X: f32 = 20.0;
const MANAGER_THUMB_PREVIEW_INSET_Y: f32 = 18.0;
const MANAGER_THUMB_CAPTION: f32 = 20.0;

/// How many cards the strip lays across one row.
///
/// The reference track is `auto-fit`, which collapses the tracks a short
/// document does not fill, so three sheets are three cards across the strip.
/// Sizing every card to the 140 pt minimum instead left the cards bunched at
/// the left edge of an otherwise empty panel.
fn manager_thumbnail_columns(available_width: f32, sheets: usize) -> usize {
    let fits = ((available_width + MANAGER_THUMB_GAP)
        / (MANAGER_THUMB_MIN_WIDTH + MANAGER_THUMB_GAP))
        .floor()
        .max(1.0) as usize;
    fits.min(sheets.max(1))
}

/// The height a card of this width is drawn at, for a set of pages whose
/// bounding ruler has the given aspect (height over width).
///
/// The card is sized to its page rather than the page fitted into whatever card
/// the row hands out: a card taller than the page it carries is desk, and a
/// strip of desk is what the dialog is here to avoid. The ceiling stops a
/// two-sheet document from spending a third of the dialog on its previews.
fn manager_thumbnail_preview_height(item_width: f32, ruler_aspect: f32) -> f32 {
    let page_width =
        (item_width - 2.0 * f32::from(MANAGER_THUMB_PADDING) - MANAGER_THUMB_PREVIEW_INSET_X)
            .max(1.0);
    let page_height = page_width * ruler_aspect;
    (page_height + MANAGER_THUMB_PREVIEW_INSET_Y + MANAGER_THUMB_CAPTION)
        .clamp(MANAGER_THUMB_MIN_PREVIEW, MANAGER_THUMB_MAX_PREVIEW)
}

/// The comparison strip, drawn only when there is a comparison to make.
///
/// One sheet has nothing to compare itself against: the strip became a band of
/// empty desk around a thumbnail of the page the table below already states in
/// full. Returns whether anything was drawn so the caller can skip its lead.
fn manager_thumbnails(
    ui: &mut Ui,
    rows: &[SheetFormatManagerRow],
    preview_content: &DrawingSheetPreviewContent,
) -> bool {
    if rows.len() < 2 {
        return false;
    }
    let t = Tokens::get(ui.ctx());
    // The ruler every card in the strip is drawn against: the widest page in the
    // set by the tallest, so a mixed set is compared rather than each sheet
    // being drawn to its own card.
    let ruler = rows
        .iter()
        .map(|row| row.format.oriented_dimensions_um())
        .fold((1_u64, 1_u64), |(width, height), (row_w, row_h)| {
            (width.max(row_w), height.max(row_h))
        });
    let ruler_aspect = ruler.1 as f32 / ruler.0.max(1) as f32;
    let gap = MANAGER_THUMB_GAP;
    Frame::NONE
        .fill(t.color.canvas_bg)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::same(11))
        .show(ui, |ui| {
            // Without this the strip shrinks to its cards and the panel around
            // them stops short of the table below it.
            ui.set_min_width(ui.available_width());
            let available = ui.available_width().max(MANAGER_THUMB_MIN_WIDTH);
            let columns = manager_thumbnail_columns(available, rows.len());
            // Floored, because a card wider than its share by even a fraction of
            // a point wraps the last card of the row onto a row of its own.
            let item_width = ((available - gap * (columns as f32 - 1.0)) / columns as f32)
                .floor()
                .max(MANAGER_THUMB_MIN_WIDTH);
            let preview_height = manager_thumbnail_preview_height(item_width, ruler_aspect);
            let item_height = preview_height + 2.0 * f32::from(MANAGER_THUMB_PADDING);
            ui.spacing_mut().item_spacing = vec2(gap, gap);
            ui.horizontal_wrapped(|ui| {
                for row in rows {
                    ui.allocate_ui_with_layout(
                        vec2(item_width, item_height),
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
                                .inner_margin(Margin::same(MANAGER_THUMB_PADDING))
                                .show(ui, |ui| {
                                    drawing_sheet_preview_at_common_scale(
                                        ui,
                                        &row.format,
                                        preview_height,
                                        &format!("{} \u{00b7} {}", row.page, row.name),
                                        row.active.then_some(preview_content),
                                        ruler,
                                    );
                                });
                        },
                    );
                }
            });
        });
    true
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
        SheetManagerFormatSource::SheetProperties => SHEET_PROPERTIES_INSTEAD.to_owned(),
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

fn manager_apply_note(ui: &mut Ui, heading: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(heading)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text),
    );
    // Stated, because the note sits in a column the caller sized: without it the
    // detail line lays out against the panel and runs under the footer.
    ui.add(
        egui::Label::new(
            RichText::new(body)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
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
        DrawingSheetStandard, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
        SchematicPageOrientation, SheetDefinition, SheetPortPolicy, SheetTemplate,
    };

    #[test]
    fn sheet_format_manager_controls_stack_at_phone_width() {
        assert!(sheet_format_manager_controls_stack(366.0));
        assert!(sheet_format_manager_controls_stack(619.0));
        assert!(!sheet_format_manager_controls_stack(620.0));
        assert!(!sheet_format_manager_controls_stack(760.0));
    }

    #[test]
    fn thumbnail_strip_collapses_the_tracks_a_short_document_does_not_fill() {
        // Three sheets are three cards across the strip, exactly as the
        // reference's `auto-fit` track does, rather than three 140 pt tiles
        // bunched against the left edge of an empty panel.
        assert_eq!(manager_thumbnail_columns(1_136.0, 3), 3);
        // Once the document is longer than the row, the row is the limit.
        assert_eq!(manager_thumbnail_columns(1_136.0, 12), 7);
        assert_eq!(manager_thumbnail_columns(300.0, 12), 2);
        // A phone-width strip still lays one readable card.
        assert_eq!(manager_thumbnail_columns(120.0, 4), 1);
    }

    #[test]
    fn thumbnail_card_is_sized_to_the_page_it_draws() {
        // A4 landscape: the card a three-sheet strip hands out is spent on the
        // page, with only the desk margin and the caption left over.
        let landscape = 210.0 / 297.0;
        let card = 300.0;
        let height = manager_thumbnail_preview_height(card, landscape);
        assert!(
            height < MANAGER_THUMB_MAX_PREVIEW,
            "{height} hit the ceiling"
        );
        let page_height = height - MANAGER_THUMB_PREVIEW_INSET_Y - MANAGER_THUMB_CAPTION;
        let page_width = page_height / landscape;
        let card_inner = card - 2.0 * f32::from(MANAGER_THUMB_PADDING);
        assert!(
            page_width >= card_inner - MANAGER_THUMB_PREVIEW_INSET_X - 1.0,
            "the page should fill its card: {page_width} in {card_inner}"
        );
        // Both bounds hold: a two-sheet strip stops at the ceiling and a full
        // row of narrow cards never draws a page too small to read.
        assert_eq!(
            manager_thumbnail_preview_height(900.0, landscape),
            MANAGER_THUMB_MAX_PREVIEW
        );
        assert_eq!(
            manager_thumbnail_preview_height(140.0, landscape),
            MANAGER_THUMB_MIN_PREVIEW
        );
        // A portrait set is taller for the same card, which is the whole point
        // of ruling the strip on the set's own bounding page.
        assert!(
            manager_thumbnail_preview_height(372.0, 297.0 / 210.0)
                > manager_thumbnail_preview_height(372.0, landscape)
        );
    }

    #[test]
    fn manager_columns_spend_the_whole_table_width() {
        // The body width of the drawing-sheet dialog, which is the width the
        // reference table's column shares were read at.
        const REFERENCE_WIDTH: f32 = 1_136.0;
        let (last, leading) = MANAGER_COLUMNS.split_last().unwrap();
        let shares: f32 = leading.iter().map(|(_, share, _)| share).sum();
        // The seven stated columns leave the eighth a remainder wide enough for
        // its longest value, so the header band and a selected row's wash reach
        // the right edge of the table instead of stopping mid-surface.
        assert!(shares < 1.0, "the stated shares must leave a remainder");
        let stated: f32 = leading
            .iter()
            .map(|(_, share, minimum)| (REFERENCE_WIDTH * share).max(*minimum))
            .sum();
        let remainder = REFERENCE_WIDTH - stated;
        assert!(
            remainder >= last.2,
            "the last column is {remainder}, under its own {} minimum",
            last.2
        );
        // Every column can still state its content at the width below which the
        // body scrolls sideways rather than shortening a value.
        let minima: f32 = MANAGER_COLUMNS.iter().map(|(_, _, minimum)| minimum).sum();
        assert!(
            minima <= MANAGER_TABLE_MIN_WIDTH,
            "{minima} exceeds the {MANAGER_TABLE_MIN_WIDTH} scroll floor"
        );
    }

    #[test]
    fn manager_refuses_to_open_without_schematic_edit_authority() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.content.active_key();
        app.state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        app.state.schematic.session.read_only = true;

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
        let key = app.state.workspace.content.active_key();
        let first = app
            .state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        let (managed, ordinary) = {
            let catalog = app
                .state
                .workspace
                .content
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
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
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
                .content
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
            .content
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
            .content
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

    /// The manager's way out to the open sheet's own properties closes it and
    /// shows the sheet panel, rather than applying anything.
    #[test]
    fn the_manager_hands_the_open_sheet_to_sheet_properties() {
        let mut app = RSpiceApp::test_instance();
        let key = app.state.workspace.content.active_key();
        app.state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [])
            .unwrap();
        open_sheet_format_manager(&mut app.state).unwrap();
        app.state.dialogs.drawing_sheet_support.manager.source =
            SheetManagerFormatSource::SheetProperties;
        assert_eq!(
            apply_sheet_format_manager(&mut app).unwrap_err(),
            "Edit the open sheet in Sheet properties."
        );
        assert!(!app.state.can_undo_project_design());
    }
}
