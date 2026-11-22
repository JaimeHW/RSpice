//! Universal engineering-table view manager.

use egui::{Context, RichText, ScrollArea, TextEdit, Ui, Vec2};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    EngineeringDataset, EngineeringFilterGrammar, EngineeringSortRule, EngineeringTableView,
    EngineeringViewScope, SortDirection,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, CommandForm, Dialog, DialogChoice, DialogInitialFocus, DialogSize, IconButton,
    TableColumn, select,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::workflows::export_workflow::ObservedExportDestination;
use crate::workbench::workflows::export_workflow::{
    ExportWorkflowIo, NativeExportWorkflowIo, SaveDialogConfig,
};
use symbol_pins::{active_dataset, is_pin_table};

use crate::state::engineering_table::EngineeringTableViewStore;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::WorkbenchSession;

mod object_cells;
mod symbol_pins;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum EngineeringTableDialogPage {
    #[default]
    Manager,
    SaveView,
    SavedViews,
    Export,
    RowDetails,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EngineeringTableExportScope {
    #[default]
    CurrentView,
    SelectedRows,
    CompleteDataset,
}

impl EngineeringTableExportScope {
    pub(crate) const ALL: [Self; 3] =
        [Self::CurrentView, Self::SelectedRows, Self::CompleteDataset];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CurrentView => "Current view · filters, sort, visible columns",
            Self::SelectedRows => "Selected rows · current visible columns",
            Self::CompleteDataset => "Complete dataset · all logical rows and columns",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EngineeringTableExportFormat {
    #[default]
    CsvSchema,
    Tsv,
    Xlsx,
    Parquet,
}

impl EngineeringTableExportFormat {
    pub(crate) const ALL: [Self; 4] = [Self::CsvSchema, Self::Tsv, Self::Xlsx, Self::Parquet];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CsvSchema => "CSV + schema JSON",
            Self::Tsv => "TSV",
            Self::Xlsx => "XLSX · frozen headers and filters",
            Self::Parquet => "Parquet",
        }
    }
}

/// Transactional universal data-grid manager. The active dataset is rebuilt
/// from its owning source at open/commit time; the dialog retains only a view
/// definition and source revision, never a duplicate engineering dataset.
#[derive(Debug, Clone, Default)]
pub struct EngineeringTableDialogState {
    pub open: bool,
    pub page: EngineeringTableDialogPage,
    pub(crate) draft: Option<crate::state::EngineeringTableView>,
    pub(crate) source_revision: u64,
    pub(crate) selected_saved_id: Option<String>,
    pub save_name: String,
    pub save_scope: crate::state::EngineeringViewScope,
    pub(crate) save_as_default: bool,
    pub(crate) saved_query: String,
    pub(crate) export_scope: EngineeringTableExportScope,
    pub(crate) export_format: EngineeringTableExportFormat,
    pub(crate) export_headers: bool,
    pub(crate) export_units: bool,
    pub(crate) export_metadata: bool,
    pub(crate) export_hidden_columns: bool,
    pub(crate) selected_row_ids: std::collections::BTreeSet<String>,
    pub(crate) active_cell: Option<(usize, usize)>,
    pub(crate) focus_cell: Option<(usize, usize)>,
    pub row_details_id: Option<String>,
    /// The pin table's cell being edited, if any.
    pub pin_edit: Option<symbol_pins::PinCellEdit>,
    /// The Object table's cell being edited, if any.
    pub object_edit: Option<object_cells::ObjectCellEdit>,
    /// The columns, sort and filter are unfolded above the rows. They open
    /// folded, so the rows take the height.
    pub(crate) setup_open: bool,
    /// The pin table's row a Shift+click selects the run from.
    pub(crate) pin_anchor: Option<usize>,
    pub(crate) saved_edit_id: Option<String>,
    pub(crate) saved_edit_name: String,
    #[cfg(target_arch = "wasm32")]
    pub(crate) saved_import_token: Option<crate::workbench::browser::file_import::TextImportToken>,
    pub(crate) error: Option<String>,
}

impl EngineeringTableDialogState {
    /// Back from a page to the manager it was opened from.
    fn leave_page(&mut self) {
        self.page = EngineeringTableDialogPage::Manager;
        self.error = None;
    }

    pub fn open(&mut self, view: crate::state::EngineeringTableView, source_revision: u64) {
        *self = Self {
            open: true,
            draft: Some(view),
            source_revision,
            export_headers: true,
            export_units: true,
            export_metadata: true,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(token) = self.saved_import_token.take() {
            let _ = crate::workbench::browser::file_import::finish_text_import(token);
        }
        *self = Self::default();
    }
}

#[derive(Debug, Clone)]
enum BodyAction {
    Copy,
    Export,
    Save,
    Manage,
    UseSaved(String, EngineeringViewScope),
    DeleteSaved(String, EngineeringViewScope),
    MakeDefault(String, EngineeringViewScope),
    OpenDetails(String),
    ImportSaved,
    ExportSaved(String, EngineeringViewScope),
    BeginRename(String, String),
    CommitRename(String, EngineeringViewScope),
    DuplicateSaved(String, EngineeringViewScope, String),
    /// Set one column of the pins these rows stand for.
    EditPins {
        rows: Vec<String>,
        column: rspice_schematic_editor::symbol_editor::pin_table::PinTableColumn,
        text: String,
    },
    /// Move the selected pins one netlist place.
    MovePins {
        later: bool,
    },
    /// Set one cell of the parts these rows stand for.
    EditObjects(object_cells::ObjectCellEdit),
}

/// The file the browser picker returned for one import, keyed to the token
/// the dialog issued so a stale pick cannot be applied to a later import.
#[cfg(target_arch = "wasm32")]
type BrowserSavedViewImport = Option<(
    crate::workbench::browser::file_import::TextImportToken,
    Result<Option<crate::workbench::browser::file_import::PickedTextFile>, String>,
)>;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_SAVED_VIEW_IMPORT: std::cell::RefCell<BrowserSavedViewImport> =
        const { std::cell::RefCell::new(None) };
}

pub fn open_engineering_table_dialog(
    dialog: &mut EngineeringTableDialogState,
    views: &EngineeringTableViewStore,
    model: &Model,
) -> bool {
    if dialog.open {
        return false;
    }
    let dataset = active_dataset(model);
    let mut view = views
        .working
        .get(&dataset.id)
        .cloned()
        .or_else(|| {
            model
                .workspace
                .content
                .engineering_table_views
                .saved
                .iter()
                .find(|saved| saved.definition.grid_id == dataset.id && saved.is_default)
                .map(|saved| saved.definition.clone())
        })
        .or_else(|| {
            views
                .saved
                .iter()
                .find(|saved| saved.definition.grid_id == dataset.id && saved.is_default)
                .map(|saved| saved.definition.clone())
        })
        .unwrap_or_else(|| EngineeringTableView::for_dataset(&dataset));
    view.normalize_for(&dataset);
    dialog.open(view, dataset.source_revision);
    true
}

/// Save view: the current view's name, where it is kept, and whether
/// the grid opens on it.
fn save_view_form(state: &mut EngineeringTableDialogState, ctx: &Context) -> DialogChoice {
    let scopes = EngineeringViewScope::ALL.map(|scope| scope.label().to_owned());
    let named = !state.save_name.trim().is_empty();
    let error = state.error.clone();
    let status = match (error.as_deref(), named) {
        (Some(error), _) => Err(error),
        (None, false) => Err("Enter a name for the view."),
        (None, true) => Ok(None),
    };
    CommandForm::new("Save engineering table view", "Save view")
        .describe("Keep the table's columns, sort and filters as a named view of this grid.")
        .primary_enabled(named)
        .status(status)
        .show(ctx, |rows| {
            let name = egui::Id::new("engineering-table-save-name");
            if rows.text("Name", name, &mut state.save_name, "").changed() {
                state.error = None;
            }
            if let Some(index) = rows.select(
                "Scope",
                "engineering-table-save-scope",
                state.save_scope.label(),
                &scopes,
            ) {
                state.save_scope = EngineeringViewScope::ALL[index];
                state.error = None;
            }
            rows.derived(match state.save_scope {
                EngineeringViewScope::Personal => "Kept on this device only.",
                EngineeringViewScope::Project => "Saved with the project.",
            });
            rows.check_columns(
                "Default",
                &mut [("The grid opens on this view", &mut state.save_as_default)],
            );
            Some(name)
        })
}

/// Export: which rows, in which format, with what beside the values.
fn export_form(
    state: &mut EngineeringTableDialogState,
    ctx: &Context,
    dataset: &EngineeringDataset,
) -> DialogChoice {
    let scopes = EngineeringTableExportScope::ALL.map(|scope| scope.label().to_owned());
    let formats = EngineeringTableExportFormat::ALL.map(|format| format.label().to_owned());
    let rows_out = match state.export_scope {
        EngineeringTableExportScope::CurrentView => state
            .draft
            .as_ref()
            .map(|view| dataset.project(view).rows.len())
            .unwrap_or_default(),
        EngineeringTableExportScope::SelectedRows => state.selected_row_ids.len(),
        EngineeringTableExportScope::CompleteDataset => dataset.rows.len(),
    };
    let refusal = (state.export_scope == EngineeringTableExportScope::SelectedRows
        && state.selected_row_ids.is_empty())
    .then_some("Select rows in the table first.");
    let error = state.error.clone();
    let status = match (error.as_deref(), refusal) {
        (Some(error), _) | (None, Some(error)) => Err(error),
        (None, None) => Ok(None),
    };
    let counted = format!("{rows_out} row{}", if rows_out == 1 { "" } else { "s" });
    CommandForm::new("Export engineering table", "Export table")
        .wide()
        .describe("Write the table's rows to a file, with the headers, units and view details chosen here.")
        .primary_enabled(refusal.is_none())
        .status(status)
        .show(ctx, |rows| {
            rows.value_mono("Grid", &dataset.id);
            let scope = rows.select_with_response(
                "Scope",
                "engineering-table-export-scope",
                state.export_scope.label(),
                &scopes,
            );
            if let Some(index) = scope.picked {
                state.export_scope = EngineeringTableExportScope::ALL[index];
                state.error = None;
            }
            rows.derived(&counted);
            if let Some(index) = rows.select(
                "Format",
                "engineering-table-export-format",
                state.export_format.label(),
                &formats,
            ) {
                state.export_format = EngineeringTableExportFormat::ALL[index];
            }
            rows.checks(
                "Include",
                &mut [
                    ("Column headers", &mut state.export_headers),
                    ("Units in headers", &mut state.export_units),
                    ("Filter, sort and manifest", &mut state.export_metadata),
                    ("Hidden columns", &mut state.export_hidden_columns),
                ],
            );
            Some(scope.response.id)
        })
}

/// Row details: every field of one row, and the way to its source
/// object.
fn row_details_form(
    state: &EngineeringTableDialogState,
    ctx: &Context,
    dataset: &EngineeringDataset,
) -> DialogChoice {
    let source = state.row_details_id.clone().unwrap_or_default();
    let fields = dataset
        .rows
        .iter()
        .find(|row| row.stable_id == source)
        .map(|row| {
            dataset
                .columns
                .iter()
                .map(|column| {
                    vec![
                        column.label.clone(),
                        row.cells
                            .get(&column.id)
                            .map(|cell| cell.display.clone())
                            .unwrap_or_default(),
                        column.unit.clone().unwrap_or_default(),
                    ]
                })
                .collect::<Vec<_>>()
        });
    let error = state.error.clone();
    let status = match (error.as_deref(), fields.is_some()) {
        (Some(error), _) => Err(error),
        (None, false) => Err("The row is no longer in the table."),
        (None, true) => Ok(None),
    };
    let revision = dataset.source_revision.to_string();
    CommandForm::new("Engineering table row details", "Cross-probe source object")
        .wide()
        .describe(
            "Read every field of one table row, and select its source object in the schematic.",
        )
        .primary_enabled(fields.is_some())
        .status(status)
        .show(ctx, |rows| {
            rows.value_mono("Source", &source);
            rows.value("Source revision", &revision);
            if let Some(fields) = &fields {
                rows.table(&ROW_DETAIL_COLUMNS, fields);
            }
            None
        })
}

/// What the engineering table draws over and changes: its draft, the views
/// saved on this device, the model, the symbol editor whose pins it edits,
/// the platform's project lifecycle and the session.
pub struct EngineeringTable<'a> {
    pub(in crate::workbench) dialog: &'a mut EngineeringTableDialogState,
    pub(in crate::workbench) views: &'a mut EngineeringTableViewStore,
    pub(in crate::workbench) model: &'a mut Model,
    pub(in crate::workbench) symbol:
        &'a mut crate::workbench::app::actions::workspace::symbol_document::SymbolUiState,
    pub(in crate::workbench) platform: &'a mut PlatformState,
    pub(in crate::workbench) workbench: &'a mut WorkbenchSession,
    /// The property sheets a cell's value is checked against, as the
    /// inspector checks it.
    pub(in crate::workbench) registry: &'a crate::properties::PropertyEditorSchema,
}

impl EngineeringTable<'_> {
    pub fn render_engineering_table_dialog(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        self.consume_engineering_view_import();
        if !self.dialog.open {
            return;
        }

        let dataset = active_dataset(self.model);
        if self.dialog.source_revision != dataset.source_revision
            || self
                .dialog
                .draft
                .as_ref()
                .is_some_and(|draft| draft.grid_id != dataset.id)
        {
            if let Some(draft) = self.dialog.draft.as_mut() {
                draft.normalize_for(&dataset);
            }
            self.dialog.source_revision = dataset.source_revision;
        }
        let pin_table = is_pin_table(&dataset);
        if pin_table {
            self.take_canvas_pin_selection(&dataset);
        }
        let selected_before = self.dialog.selected_row_ids.clone();

        let page = self.dialog.page;
        // What a page asks before it acts is a command form of its own;
        // the manager and its saved views are the two surfaces with lists.
        let table = &mut *self.dialog;
        let form = match page {
            EngineeringTableDialogPage::SaveView => Some(save_view_form(table, ctx)),
            EngineeringTableDialogPage::Export => Some(export_form(table, ctx, &dataset)),
            EngineeringTableDialogPage::RowDetails => Some(row_details_form(table, ctx, &dataset)),
            EngineeringTableDialogPage::Manager | EngineeringTableDialogPage::SavedViews => None,
        };
        if let Some(choice) = form {
            match choice {
                DialogChoice::Primary => match page {
                    EngineeringTableDialogPage::SaveView => {
                        self.save_engineering_table_view(&dataset);
                    }
                    EngineeringTableDialogPage::Export => self.export_engineering_table(&dataset),
                    _ => self.cross_probe_engineering_row(),
                },
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    self.dialog.leave_page();
                }
                DialogChoice::Secondary | DialogChoice::None => {}
            }
            return;
        }
        let manager = page == EngineeringTableDialogPage::Manager;
        let (title, primary) = match (manager, pin_table) {
            // A cell edit is kept as it is made, one undo step each, so a
            // table has nothing to apply or cancel but its view, which Done
            // keeps. The Object table's name is the menu row's.
            (true, false) => ("Object table", "Done"),
            (true, true) => ("Pin table", "Done"),
            (false, _) => ("Saved table views", "Done"),
        };
        let description = if pin_table {
            "Edit the symbol's pins: select rows, then edit a cell to set it on every selected \
             pin. Choose the columns and their order, and sort and filter the rows."
        } else {
            "Edit the parts' names, values, parameters and purchasing fields: select rows, then \
             edit a cell to set it on every selected part. Choose, sort and filter the columns."
        };
        // Why no cell of the schematic can be edited, when none can.
        let read_only = (!pin_table
            && (self.model.schematic_edit_read_only() || self.model.safe_mode.project_read_only()))
        .then(|| crate::workbench::app::actions::workspace::read_only_master_message(self.model));
        let mut action = None;
        let mut dialog = Dialog::new(title, primary)
            .description(description)
            .size(DialogSize::WideWorkflow)
            .overlay_scrollbar()
            .initial_focus(DialogInitialFocus::BodyControl)
            // Enter in a table edits a cell or keeps the value typed in it.
            .primary_on_enter(!manager);
        if manager {
            // The rows take whatever height the surface has and scroll inside
            // it, under set-up that folds away.
            dialog = dialog.fill_available_height().manual_body_scroll();
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = match page {
                EngineeringTableDialogPage::Manager => manager_body(
                    ui,
                    &dataset,
                    &mut *self.dialog,
                    &self.views.saved,
                    &self.model.workspace.content.engineering_table_views.saved,
                    read_only.as_deref(),
                ),
                EngineeringTableDialogPage::SavedViews => saved_views_body(
                    ui,
                    &dataset,
                    &mut *self.dialog,
                    &self.views.saved,
                    &self.model.workspace.content.engineering_table_views.saved,
                ),
                EngineeringTableDialogPage::SaveView
                | EngineeringTableDialogPage::Export
                | EngineeringTableDialogPage::RowDetails => None,
            };
            if manager {
                // The body is the surface's whole track, so the footer stays
                // at the surface's foot however tall the rows come out.
                ui.expand_to_include_rect(ui.max_rect());
            }
            Some(ui.make_persistent_id(("engineering-table-first", page)))
        });

        if pin_table {
            self.give_canvas_pin_selection(&dataset, &selected_before);
        }
        if let Some(action) = action {
            self.handle_engineering_table_body_action(action, &dataset);
        }

        match choice {
            // Escape while a cell is edited drops the edit, not the table.
            DialogChoice::Cancelled
                if self.dialog.pin_edit.is_some() || self.dialog.object_edit.is_some() =>
            {
                self.dialog.pin_edit = None;
                if self.dialog.object_edit.take().is_some() {
                    self.dialog.error = None;
                }
                self.dialog.focus_cell = self.dialog.active_cell;
            }
            DialogChoice::Primary if page == EngineeringTableDialogPage::Manager => {
                self.apply_engineering_table_view(&dataset);
            }
            DialogChoice::Ghost | DialogChoice::Cancelled
                if page == EngineeringTableDialogPage::Manager =>
            {
                self.dialog.close();
            }
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.dialog.leave_page();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn handle_engineering_table_body_action(
        &mut self,
        action: BodyAction,
        dataset: &EngineeringDataset,
    ) {
        match action {
            BodyAction::Copy => self.copy_engineering_table(dataset),
            BodyAction::Export => {
                self.dialog.page = EngineeringTableDialogPage::Export;
                self.dialog.error = None;
            }
            BodyAction::Save => {
                self.dialog.page = EngineeringTableDialogPage::SaveView;
                self.dialog.error = None;
            }
            BodyAction::Manage => {
                self.dialog.page = EngineeringTableDialogPage::SavedViews;
                self.dialog.error = None;
            }
            BodyAction::UseSaved(id, scope) => {
                let saved = match scope {
                    EngineeringViewScope::Personal => {
                        self.views.saved.iter().find(|saved| saved.id == id)
                    }
                    EngineeringViewScope::Project => self
                        .model
                        .workspace
                        .content
                        .engineering_table_views
                        .saved
                        .iter()
                        .find(|saved| saved.id == id),
                };
                if let Some(saved) = saved {
                    let mut definition = saved.definition.clone();
                    definition.normalize_for(dataset);
                    self.dialog.draft = Some(definition);
                    self.dialog.selected_saved_id = Some(saved.id.clone());
                    self.dialog.page = EngineeringTableDialogPage::Manager;
                }
            }
            BodyAction::DeleteSaved(id, scope) => {
                let deleted = match scope {
                    EngineeringViewScope::Personal => self.views.delete(&id),
                    EngineeringViewScope::Project => {
                        let deleted = self
                            .model
                            .workspace
                            .content
                            .engineering_table_views
                            .delete(&id);
                        if deleted {
                            self.model.workspace.content.project_metadata_dirty = true;
                        }
                        deleted
                    }
                };
                if deleted {
                    self.workbench
                        .prompt("Saved engineering table view deleted.");
                }
            }
            BodyAction::MakeDefault(id, scope) => {
                let changed = match scope {
                    EngineeringViewScope::Personal => self.views.make_default(&id),
                    EngineeringViewScope::Project => {
                        let changed = self
                            .model
                            .workspace
                            .content
                            .engineering_table_views
                            .make_default(&id);
                        if changed {
                            self.model.workspace.content.project_metadata_dirty = true;
                        }
                        changed
                    }
                };
                if changed {
                    self.workbench
                        .prompt("Default engineering table view updated.");
                }
            }
            BodyAction::OpenDetails(id) => {
                self.dialog.row_details_id = Some(id);
                self.dialog.page = EngineeringTableDialogPage::RowDetails;
            }
            BodyAction::EditPins { rows, column, text } => {
                self.edit_symbol_pins(&rows, column, &text);
            }
            BodyAction::MovePins { later } => self.move_symbol_pins(later),
            BodyAction::EditObjects(edit) => self.edit_object_cells(edit),
            BodyAction::ImportSaved => self.import_engineering_saved_view(),
            BodyAction::ExportSaved(id, scope) => self.export_engineering_saved_view(&id, scope),
            BodyAction::BeginRename(id, name) => {
                self.dialog.saved_edit_id = Some(id);
                self.dialog.saved_edit_name = name;
                self.dialog.error = None;
            }
            BodyAction::CommitRename(id, scope) => {
                let name = self.dialog.saved_edit_name.clone();
                let result = match scope {
                    EngineeringViewScope::Personal => self.views.rename(&id, &name),
                    EngineeringViewScope::Project => {
                        let result = self
                            .model
                            .workspace
                            .content
                            .engineering_table_views
                            .rename(&id, &name);
                        if result.is_ok() {
                            self.model.workspace.content.project_metadata_dirty = true;
                        }
                        result
                    }
                };
                match result {
                    Ok(()) => {
                        self.dialog.saved_edit_id = None;
                        self.dialog.saved_edit_name.clear();
                    }
                    Err(error) => self.dialog.error = Some(error),
                }
            }
            BodyAction::DuplicateSaved(id, scope, source_name) => {
                let store = match scope {
                    EngineeringViewScope::Personal => &mut *self.views,
                    EngineeringViewScope::Project => {
                        &mut self.model.workspace.content.engineering_table_views
                    }
                };
                let mut suffix = 1usize;
                let result = loop {
                    let candidate = if suffix == 1 {
                        format!("{source_name} copy")
                    } else {
                        format!("{source_name} copy {suffix}")
                    };
                    match store.duplicate(&id, &candidate, scope, dataset) {
                        Err(error) if error.contains("already exists") => suffix += 1,
                        result => break result,
                    }
                };
                match result {
                    Ok(_) => {
                        if scope == EngineeringViewScope::Project {
                            self.model.workspace.content.project_metadata_dirty = true;
                        }
                    }
                    Err(error) => self.dialog.error = Some(error),
                }
            }
        }
    }

    fn apply_engineering_table_view(&mut self, dataset: &EngineeringDataset) {
        let Some(mut view) = self.dialog.draft.clone() else {
            return;
        };
        view.normalize_for(dataset);
        self.views.set_working(view, dataset);
        self.dialog.close();
    }

    fn copy_engineering_table(&mut self, dataset: &EngineeringDataset) {
        let Some(view) = self.dialog.draft.as_ref() else {
            return;
        };
        let selected =
            (!self.dialog.selected_row_ids.is_empty()).then_some(&self.dialog.selected_row_ids);
        match crate::state::engineering_table::delimited_text_selected(
            dataset, view, b'\t', true, true, false, selected,
        ) {
            Ok(text) => {
                let rows = dataset.project_selected(view, false, selected).rows.len();
                self.workbench
                    .outbox
                    .push(crate::workbench::framework::requests::ClipboardText(text));
                self.workbench.prompt(format!(
                    "{rows} row{} copied with headers and displayed units.",
                    if rows == 1 { "" } else { "s" }
                ));
            }
            Err(error) => self.dialog.error = Some(error),
        }
    }

    pub fn save_engineering_table_view(&mut self, dataset: &EngineeringDataset) {
        let Some(view) = self.dialog.draft.clone() else {
            return;
        };
        let name = self.dialog.save_name.clone();
        let scope = self.dialog.save_scope;
        let make_default = self.dialog.save_as_default;
        let result = match scope {
            EngineeringViewScope::Personal => {
                self.views.save(&name, scope, view, make_default, dataset)
            }
            EngineeringViewScope::Project => {
                if !self.platform.project_lifecycle.is_open() {
                    Err("Open a project before saving a project-scoped table view.".to_owned())
                } else {
                    let result = self.model.workspace.content.engineering_table_views.save(
                        &name,
                        scope,
                        view,
                        make_default,
                        dataset,
                    );
                    if result.is_ok() {
                        self.model.workspace.content.project_metadata_dirty = true;
                    }
                    result
                }
            }
        };
        match result {
            Ok(id) => {
                self.dialog.selected_saved_id = Some(id);
                self.dialog.page = EngineeringTableDialogPage::Manager;
                self.dialog.save_name.clear();
                self.dialog.error = None;
                self.workbench.prompt("Engineering table view saved.");
            }
            Err(error) => self.dialog.error = Some(error),
        }
    }

    fn export_engineering_table(&mut self, dataset: &EngineeringDataset) {
        let Some(current) = self.dialog.draft.as_ref() else {
            return;
        };
        let mut view = current.clone();
        let selected_rows = match self.dialog.export_scope {
            EngineeringTableExportScope::CurrentView => None,
            EngineeringTableExportScope::SelectedRows => {
                if self.dialog.selected_row_ids.is_empty() {
                    self.dialog.error =
                        Some("Select at least one engineering table row first.".to_owned());
                    return;
                }
                Some(&self.dialog.selected_row_ids)
            }
            EngineeringTableExportScope::CompleteDataset => {
                view.filters.clear();
                None
            }
        };
        let format = self.dialog.export_format;
        let include_headers = self.dialog.export_headers;
        let include_units = self.dialog.export_units;
        let include_metadata = self.dialog.export_metadata;
        let include_hidden = self.dialog.export_hidden_columns;
        let (extension, filter_name) = match format {
            EngineeringTableExportFormat::CsvSchema if cfg!(target_arch = "wasm32") => {
                ("zip", "CSV + schema package")
            }
            EngineeringTableExportFormat::CsvSchema => ("csv", "CSV table"),
            EngineeringTableExportFormat::Tsv
                if cfg!(target_arch = "wasm32") && include_metadata =>
            {
                ("zip", "TSV + metadata package")
            }
            EngineeringTableExportFormat::Tsv => ("tsv", "TSV table"),
            EngineeringTableExportFormat::Xlsx => ("xlsx", "Excel workbook"),
            EngineeringTableExportFormat::Parquet => ("parquet", "Apache Parquet table"),
        };
        let io = NativeExportWorkflowIo;
        let default_name = format!("rspice-active-schematic-table.{extension}");
        let extensions = [extension];
        let destination = match io.show_save_dialog(SaveDialogConfig {
            title: "Export engineering table",
            default_name: &default_name,
            filter_name,
            filter_extensions: &extensions,
        }) {
            Ok(Some(path)) => path,
            Ok(None) => return,
            Err(error) => {
                self.dialog.error = Some(error);
                return;
            }
        };
        let result = (|| -> Result<(), String> {
            let observed = io.observe_destination(&destination)?;
            match format {
                EngineeringTableExportFormat::CsvSchema => {
                    let text = crate::state::engineering_table::delimited_text_selected(
                        dataset,
                        &view,
                        b',',
                        include_headers,
                        include_units,
                        include_hidden,
                        selected_rows,
                    )?;
                    let schema = crate::state::engineering_table::schema_json(
                        dataset,
                        &view,
                        include_hidden,
                        include_metadata,
                    )?;
                    #[cfg(target_arch = "wasm32")]
                    {
                        let bytes =
                            crate::workbench::workflows::export_workflow::deterministic_stored_zip(
                                &[
                                    ("table.csv", text.as_bytes()),
                                    ("schema.json", schema.as_bytes()),
                                ],
                            )
                            .map_err(|error| error.to_string())?;
                        io.write_bytes_file_observed(&observed, &bytes, "application/zip")
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let schema_path = destination.with_extension("schema.json");
                        publish_with_companion(&io, &observed, &schema_path, &schema, &text)
                    }
                }
                EngineeringTableExportFormat::Tsv => {
                    let text = crate::state::engineering_table::delimited_text_selected(
                        dataset,
                        &view,
                        b'\t',
                        include_headers,
                        include_units,
                        include_hidden,
                        selected_rows,
                    )?;
                    if include_metadata {
                        let metadata = crate::state::engineering_table::schema_json(
                            dataset,
                            &view,
                            include_hidden,
                            true,
                        )?;
                        #[cfg(target_arch = "wasm32")]
                        {
                            let bytes =
                                crate::workbench::workflows::export_workflow::deterministic_stored_zip(&[
                                    ("table.tsv", text.as_bytes()),
                                    ("metadata.json", metadata.as_bytes()),
                                ]).map_err(|error| error.to_string())?;
                            io.write_bytes_file_observed(&observed, &bytes, "application/zip")
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let metadata_path = destination.with_extension("metadata.json");
                            publish_with_companion(&io, &observed, &metadata_path, &metadata, &text)
                        }
                    } else {
                        io.write_text_file_observed(&observed, &text)
                    }
                }
                EngineeringTableExportFormat::Xlsx => {
                    let bytes = crate::state::engineering_table::xlsx_bytes(
                        dataset,
                        &view,
                        include_headers,
                        include_units,
                        include_metadata,
                        include_hidden,
                        selected_rows,
                    )?;
                    io.write_bytes_file_observed(
                        &observed,
                        &bytes,
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    )
                }
                EngineeringTableExportFormat::Parquet => {
                    let bytes = crate::state::engineering_table::parquet_bytes(
                        dataset,
                        &view,
                        include_metadata,
                        include_hidden,
                        selected_rows,
                    )?;
                    io.write_bytes_file_observed(
                        &observed,
                        &bytes,
                        "application/vnd.apache.parquet",
                    )
                }
            }
        })();
        match result {
            Ok(()) => {
                self.dialog.close();
                self.model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Engineering table exported to {}.", destination.display()),
                );
            }
            Err(error) => self.dialog.error = Some(error),
        }
    }

    fn cross_probe_engineering_row(&mut self) {
        let Some(stable_id) = self.dialog.row_details_id.as_deref() else {
            return;
        };
        enum CrossProbeTarget {
            Component(u64),
            Wire(u64),
            NetLabel(u64),
            Bus(u64),
            BusTap(u64),
            Junction(crate::state::Point),
        }

        let parse_id = |prefix: &str| {
            stable_id
                .strip_prefix(prefix)
                .and_then(|id| id.parse::<u64>().ok())
        };
        let schematic = &self.model.schematic;
        let target = parse_id("component-")
            .filter(|id| {
                schematic
                    .document()
                    .components
                    .iter()
                    .any(|object| object.id == *id)
            })
            .map(CrossProbeTarget::Component)
            .or_else(|| {
                parse_id("wire-")
                    .filter(|id| {
                        schematic
                            .document()
                            .wires
                            .iter()
                            .any(|object| object.id == *id)
                    })
                    .map(CrossProbeTarget::Wire)
            })
            .or_else(|| {
                parse_id("net-label-")
                    .filter(|id| {
                        schematic
                            .document()
                            .net_labels
                            .iter()
                            .any(|object| object.id == *id)
                    })
                    .map(CrossProbeTarget::NetLabel)
            })
            .or_else(|| {
                parse_id("bus-")
                    .filter(|id| {
                        schematic
                            .document()
                            .buses
                            .iter()
                            .any(|object| object.id == *id)
                    })
                    .map(CrossProbeTarget::Bus)
            })
            .or_else(|| {
                parse_id("bus-tap-")
                    .filter(|id| {
                        schematic
                            .document()
                            .bus_taps
                            .iter()
                            .any(|object| object.id == *id)
                    })
                    .map(CrossProbeTarget::BusTap)
            })
            .or_else(|| {
                parse_id("junction-").and_then(|id| {
                    schematic
                        .document()
                        .junctions
                        .iter()
                        .find(|object| object.id == id)
                        .map(|object| CrossProbeTarget::Junction(object.pos))
                })
            });
        let selection = &mut self.model.schematic.session.editor.selection;
        selection.clear();
        let selected = target.is_some();
        match target {
            Some(CrossProbeTarget::Component(id)) => {
                selection.components.insert(id);
            }
            Some(CrossProbeTarget::Wire(id)) => {
                selection.wires.insert(id);
            }
            Some(CrossProbeTarget::NetLabel(id)) => {
                selection.net_labels.insert(id);
            }
            Some(CrossProbeTarget::Bus(id)) => {
                selection.buses.insert(id);
            }
            Some(CrossProbeTarget::BusTap(id)) => {
                selection.bus_taps.insert(id);
            }
            Some(CrossProbeTarget::Junction(pos)) => {
                selection
                    .junctions
                    .push(crate::state::JunctionSelection::new(pos));
            }
            None => {}
        }
        if selected {
            self.dialog.close();
            self.workbench.inspector_visible = true;
            self.workbench
                .prompt("The source schematic object was selected and opened in the Inspector.");
        } else {
            self.dialog.error =
                Some("The source object no longer exists in the active schematic.".to_owned());
        }
    }

    fn import_engineering_saved_view(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(path) = rfd::FileDialog::new()
                .add_filter(
                    "RSpice engineering table view",
                    &["json", "rspice-table-view"],
                )
                .pick_file()
            else {
                return;
            };
            let result = std::fs::metadata(&path)
                .map_err(|error| error.to_string())
                .and_then(|metadata| {
                    if metadata.len() > 512 * 1024 {
                        Err("Saved-view import exceeds the 512 KiB limit.".to_owned())
                    } else {
                        std::fs::read_to_string(&path).map_err(|error| error.to_string())
                    }
                });
            match result {
                Ok(source) => self.publish_imported_engineering_view(&source),
                Err(error) => self.dialog.error = Some(error),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            if self.dialog.saved_import_token.is_some() {
                self.dialog.error =
                    Some("An engineering table view picker is already open.".to_owned());
                return;
            }
            let token = match crate::workbench::browser::file_import::try_begin_text_import(
                crate::workbench::browser::file_import::BrowserTextImportKind::EngineeringTableView,
            ) {
                Ok(token) => token,
                Err(error) => {
                    self.dialog.error = Some(error);
                    return;
                }
            };
            self.dialog.saved_import_token = Some(token);
            crate::workbench::browser::file_import::pick_text_file(
                "RSpice engineering table view",
                &["json", "rspice-table-view"],
                move |result| {
                    if crate::workbench::browser::file_import::text_import_is_current(token) {
                        BROWSER_SAVED_VIEW_IMPORT.with(|slot| {
                            *slot.borrow_mut() = Some((token, result));
                        });
                    }
                },
            );
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn consume_engineering_view_import(&mut self) {
        let Some((token, result)) = BROWSER_SAVED_VIEW_IMPORT.with(|slot| slot.borrow_mut().take())
        else {
            return;
        };
        if self.dialog.saved_import_token != Some(token)
            || !crate::workbench::browser::file_import::finish_text_import(token)
        {
            return;
        }
        self.dialog.saved_import_token = None;
        match result {
            Ok(Some(file)) => self.publish_imported_engineering_view(&file.contents),
            Ok(None) => {}
            Err(error) => self.dialog.error = Some(error),
        }
    }

    fn publish_imported_engineering_view(&mut self, source: &str) {
        let dataset = active_dataset(self.model);
        let scope = self.dialog.save_scope;
        let result = match scope {
            EngineeringViewScope::Personal => self.views.import_view(source, scope, &dataset),
            EngineeringViewScope::Project => {
                if !self.platform.project_lifecycle.is_open() {
                    Err("Open a project before importing a project-scoped view.".to_owned())
                } else {
                    let result = self
                        .model
                        .workspace
                        .content
                        .engineering_table_views
                        .import_view(source, scope, &dataset);
                    if result.is_ok() {
                        self.model.workspace.content.project_metadata_dirty = true;
                    }
                    result
                }
            }
        };
        match result {
            Ok(_) => {
                self.dialog.error = None;
                self.model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    "Imported an engineering table view.",
                );
            }
            Err(error) => self.dialog.error = Some(error),
        }
    }

    fn export_engineering_saved_view(&mut self, id: &str, scope: EngineeringViewScope) {
        let result = match scope {
            EngineeringViewScope::Personal => self.views.export_view(id),
            EngineeringViewScope::Project => self
                .model
                .workspace
                .content
                .engineering_table_views
                .export_view(id),
        };
        let source = match result {
            Ok(source) => source,
            Err(error) => {
                self.dialog.error = Some(error);
                return;
            }
        };
        let io = NativeExportWorkflowIo;
        let path = match io.show_save_dialog(SaveDialogConfig {
            title: "Export engineering table view",
            default_name: "rspice-engineering-table-view.json",
            filter_name: "RSpice engineering table view",
            filter_extensions: &["json", "rspice-table-view"],
        }) {
            Ok(Some(path)) => path,
            Ok(None) => return,
            Err(error) => {
                self.dialog.error = Some(error);
                return;
            }
        };
        let result = io
            .observe_destination(&path)
            .and_then(|destination| io.write_text_file_observed(&destination, &source));
        match result {
            Ok(()) => {
                self.model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!(
                        "Saved engineering table view exported to {}.",
                        path.display()
                    ),
                );
            }
            Err(error) => self.dialog.error = Some(error),
        }
    }
}

/// The columns of a row's details: each field, its value, and its unit.
/// Publish a table export that has a companion file beside it: the
/// companion first, then the table the save dialog named, so a companion
/// that fails can never leave a table that looks like a complete export.
///
/// The companion is overwritten along with the table. It is named for the
/// table and holds that table's schema, and the save dialog has already
/// asked whether to replace the table, so a second question, or a refusal,
/// would only stand between the reader and the usual export-edit-export
/// loop.
#[cfg(not(target_arch = "wasm32"))]
fn publish_with_companion(
    io: &impl ExportWorkflowIo,
    table: &ObservedExportDestination,
    companion: &std::path::Path,
    companion_text: &str,
    table_text: &str,
) -> Result<(), String> {
    let companion = io.observe_destination(companion)?;
    io.write_text_file_observed(&companion, companion_text)?;
    io.write_text_file_observed(table, table_text)
}

const ROW_DETAIL_COLUMNS: [TableColumn<'static>; 3] = [
    TableColumn {
        header: "Field",
        mono: false,
    },
    TableColumn {
        header: "Value",
        mono: true,
    },
    TableColumn {
        header: "Unit",
        mono: false,
    },
];

/// Width of the label column on the manager's label/value rows.
const LABEL_WIDTH: f32 = 84.0;

/// Width of a select on the manager's rows.
const SELECT_WIDTH: f32 = 220.0;

/// Air between the manager's sections.
const SECTION_GAP: f32 = 16.0;

/// Width of the columns list: a column's name and its order controls.
const COLUMNS_WIDTH: f32 = 280.0;

/// The manager is the table under what a reader configures about it: which
/// view it starts from, then, folded until opened, which columns it shows in
/// which order and how its rows sort and filter. The rows take the rest of
/// the height. The table was six rows tall under the whole set-up. It used
/// to state the grid's internal id and render window, a pixel width per
/// column, a pin that nothing reads, the renderer's virtualization, overscan
/// and frozen identifier policies, and a line of keyboard shortcuts; its
/// verbs trailed under a table clipped by a sideways scroll. `read_only` is
/// why the schematic's cells cannot be edited, when they cannot.
fn manager_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut EngineeringTableDialogState,
    personal: &[crate::state::SavedEngineeringTableView],
    project: &[crate::state::SavedEngineeringTableView],
    read_only: Option<&str>,
) -> Option<BodyAction> {
    let tokens = Tokens::get(ui.ctx());
    let mut action = None;

    let saved = personal
        .iter()
        .chain(project)
        .filter(|saved| saved.definition.grid_id == dataset.id)
        .collect::<Vec<_>>();
    ui.horizontal(|ui| {
        row_label(ui, "Saved view");
        let mut options = vec!["Current working view".to_owned()];
        options.extend(
            saved
                .iter()
                .map(|saved| format!("{} \u{00b7} {}", saved.name, saved.scope.label())),
        );
        let selected = state
            .selected_saved_id
            .as_deref()
            .and_then(|id| saved.iter().position(|saved| saved.id == id))
            .map_or(0, |index| index + 1);
        if let Some(index) = select(
            ui,
            "engineering-table-saved",
            "Saved view",
            &options[selected],
            &options,
            SELECT_WIDTH,
        ) {
            match index.checked_sub(1).and_then(|index| saved.get(index)) {
                Some(saved) => {
                    state.selected_saved_id = Some(saved.id.clone());
                    action = Some(BodyAction::UseSaved(saved.id.clone(), saved.scope));
                }
                None => state.selected_saved_id = None,
            }
        }
        if Button::new("Save view\u{2026}").show(ui).clicked() {
            action = Some(BodyAction::Save);
        }
        if Button::new("Saved views\u{2026}").show(ui).clicked() {
            action = Some(BodyAction::Manage);
        }
    });
    let summary = setup_summary(dataset, state.draft.as_ref()?);
    ui.add_space(SECTION_GAP / 2.0);
    setup_fold(ui, &mut state.setup_open, &summary);
    let view = state.draft.as_mut()?;

    let mut move_column = None;
    let mut remove_sort = None;
    if state.setup_open {
        // However many columns there are, the set-up never takes more than
        // half the height from the rows.
        ScrollArea::vertical()
            .id_salt("engineering-table-setup")
            .max_height(ui.available_height() * 0.5)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(COLUMNS_WIDTH);
                        let reset = heading_row(ui, "Columns", |ui| {
                            Button::new("Reset").compact().show(ui).clicked()
                        });
                        if reset {
                            *view = EngineeringTableView::for_dataset(dataset);
                        }
                        let count = view.columns.len();
                        for index in 0..count {
                            let label = dataset
                                .columns
                                .iter()
                                .find(|column| column.id == view.columns[index].column_id)
                                .map(|column| column.label.as_str())
                                .unwrap_or("Removed column");
                            ui.horizontal(|ui| {
                                ui.set_min_height(tokens.metrics.ctl_h);
                                ui.checkbox(&mut view.columns[index].visible, label);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.spacing_mut().item_spacing.x = 2.0;
                                        if IconButton::new(Icon::ChevronDown)
                                            .enabled(index + 1 < count)
                                            .side(24.0)
                                            .tooltip("Move column later")
                                            .show(ui)
                                            .clicked()
                                        {
                                            move_column = Some((index, index + 1));
                                        }
                                        if IconButton::new(Icon::ChevronUp)
                                            .enabled(index > 0)
                                            .side(24.0)
                                            .tooltip("Move column earlier")
                                            .show(ui)
                                            .clicked()
                                        {
                                            move_column = Some((index, index - 1));
                                        }
                                    },
                                );
                            });
                        }
                    });
                    ui.add_space(SECTION_GAP * 2.0);
                    ui.vertical(|ui| {
                        heading_row(ui, "Sort", |_| ());
                        let labels = dataset
                            .columns
                            .iter()
                            .map(|column| column.label.clone())
                            .collect::<Vec<_>>();
                        let directions = ["Ascending".to_owned(), "Descending".to_owned()];
                        for index in 0..view.sort.len() {
                            ui.horizontal(|ui| {
                                row_label(ui, if index == 0 { "Sort by" } else { "Then by" });
                                let current = dataset
                                    .columns
                                    .iter()
                                    .position(|column| column.id == view.sort[index].column_id);
                                if let Some(picked) = select(
                                    ui,
                                    &format!("engineering-sort-column-{index}"),
                                    "Sort column",
                                    current.map_or("Column", |current| labels[current].as_str()),
                                    &labels,
                                    SELECT_WIDTH * 0.6,
                                ) {
                                    view.sort[index].column_id = dataset.columns[picked].id.clone();
                                }
                                let direction = usize::from(
                                    view.sort[index].direction == SortDirection::Descending,
                                );
                                if let Some(picked) = select(
                                    ui,
                                    &format!("engineering-sort-direction-{index}"),
                                    "Sort direction",
                                    &directions[direction],
                                    &directions,
                                    SELECT_WIDTH * 0.5,
                                ) {
                                    view.sort[index].direction = if picked == 0 {
                                        SortDirection::Ascending
                                    } else {
                                        SortDirection::Descending
                                    };
                                }
                                if Button::new("Remove").compact().show(ui).clicked() {
                                    remove_sort = Some(index);
                                }
                            });
                        }
                        if view.sort.len() < dataset.columns.len() {
                            ui.horizontal(|ui| {
                                row_label(ui, if view.sort.is_empty() { "Sort by" } else { "" });
                                if Button::new("Add sort column").show(ui).clicked()
                                    && let Some(column) = dataset.columns.iter().find(|column| {
                                        !view.sort.iter().any(|rule| rule.column_id == column.id)
                                    })
                                {
                                    view.sort.push(EngineeringSortRule {
                                        column_id: column.id.clone(),
                                        direction: SortDirection::Ascending,
                                    });
                                }
                            });
                        }

                        ui.add_space(SECTION_GAP);
                        heading_row(ui, "Filter", |_| ());
                        ui.horizontal(|ui| {
                            ui.set_min_height(tokens.metrics.ctl_h);
                            ui.checkbox(&mut view.show_filter_row, "Filter rows by column");
                        });
                        if view.show_filter_row {
                            ui.horizontal(|ui| {
                                row_label(ui, "Match");
                                let grammars = EngineeringFilterGrammar::ALL
                                    .map(|grammar| grammar.label().to_owned());
                                let current = EngineeringFilterGrammar::ALL
                                    .iter()
                                    .position(|grammar| *grammar == view.filter_grammar)
                                    .unwrap_or(0);
                                if let Some(picked) = select(
                                    ui,
                                    "engineering-filter-grammar",
                                    "Match",
                                    &grammars[current],
                                    &grammars,
                                    SELECT_WIDTH,
                                ) {
                                    view.filter_grammar = EngineeringFilterGrammar::ALL[picked];
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.add_space(LABEL_WIDTH + ui.spacing().item_spacing.x);
                                ui.label(
                    RichText::new(match view.filter_grammar {
                        EngineeringFilterGrammar::EngineeringValues => {
                            "Compare with engineering suffixes: >= 10k, 1p..5p."
                        }
                        EngineeringFilterGrammar::TextMatching => {
                            "Contains by default; =value is exact, /pattern/ a regular expression."
                        }
                    })
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(tokens.color.text_dim),
                );
                            });
                            for column in &dataset.columns {
                                ui.horizontal(|ui| {
                                    row_label(ui, &column.label);
                                    ui.add_sized(
                                        [SELECT_WIDTH, tokens.metrics.ctl_h],
                                        TextEdit::singleline(
                                            view.filters.entry(column.id.clone()).or_default(),
                                        ),
                                    );
                                });
                            }
                        }
                    });
                });
            });
    }
    if let Some((from, to)) = move_column {
        let column = view.columns.remove(from);
        view.columns.insert(to, column);
    }
    if let Some(index) = remove_sort {
        view.sort.remove(index);
    }

    ui.add_space(SECTION_GAP);
    let projection = dataset.project(view);
    let rows = match projection.rows.len() {
        1 => "1 row".to_owned(),
        count => format!("{count} rows"),
    };
    let pin_table = is_pin_table(dataset);
    let selected = !state.selected_row_ids.is_empty();
    let verbs = heading_row(ui, &rows, |ui| {
        let export = Button::new("Export\u{2026}").show(ui).clicked();
        let copy = Button::new("Copy")
            .show(ui)
            .on_hover_text("Copy the selected rows, or every row, with headings and units")
            .clicked();
        let reorder = pin_table
            .then(|| symbol_pins::reorder_verbs(ui, selected))
            .flatten();
        (export, copy, reorder)
    });
    match verbs {
        (true, _, _) => action = Some(BodyAction::Export),
        (_, true, _) => action = Some(BodyAction::Copy),
        (_, _, Some(reorder)) => action = Some(reorder),
        _ => {}
    }
    // The rows take the height left, less a line for a refusal under them.
    let error_line = if state.error.is_some() {
        tokens.metrics.ctl_h + ui.spacing().item_spacing.y
    } else {
        0.0
    };
    let height = ui.available_height() - error_line;
    let grid_action = if pin_table {
        symbol_pins::pin_grid(ui, &projection, state, height)
    } else {
        object_cells::object_grid(ui, &projection, state, read_only, height)
    };
    if let Some(error) = &state.error {
        ui.colored_label(tokens.color.err, error);
    }
    grid_action.or(action)
}

/// What the folded set-up holds while it is closed: how many columns show,
/// and whether the rows are sorted or filtered.
fn setup_summary(dataset: &EngineeringDataset, view: &EngineeringTableView) -> String {
    let shown = view.columns.iter().filter(|column| column.visible).count();
    let mut summary = format!("{shown} of {} columns", dataset.columns.len());
    if let Some(label) = view.sort.first().and_then(|rule| {
        dataset
            .columns
            .iter()
            .find(|column| column.id == rule.column_id)
            .map(|column| column.label.as_str())
    }) {
        summary.push_str(&format!(", sorted by {label}"));
    }
    if view.show_filter_row && view.filters.values().any(|text| !text.trim().is_empty()) {
        summary.push_str(", filtered");
    }
    summary
}

/// The fold the columns, sort and filter sit in: a row that opens and
/// closes it, naming what it holds and, beside that, what is set.
fn setup_fold(ui: &mut Ui, open: &mut bool, summary: &str) {
    const TITLE: &str = "Columns, sort and filter";
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), tokens.metrics.ctl_h),
        egui::Sense::click(),
    );
    if response.clicked() {
        *open = !*open;
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::CollapsingHeader, ui.is_enabled(), TITLE)
    });
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_expanded(*open));
    if response.hovered() {
        ui.painter().rect_filled(rect, 2.0, tokens.color.bg_hover);
    }
    let chevron = if *open {
        Icon::ChevronDown
    } else {
        Icon::ChevronRight
    };
    chevron.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 10.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        tokens.color.text_dim,
    );
    let title = ui.painter().layout_no_wrap(
        TITLE.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        tokens.color.text_faint,
    );
    let title_left = rect.left() + 24.0;
    let title_width = title.size().x;
    ui.painter().galley(
        egui::pos2(title_left, rect.center().y - title.size().y * 0.5),
        title,
        tokens.color.text_faint,
    );
    let detail = ui.painter().layout_no_wrap(
        summary.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        tokens.color.text_dim,
    );
    ui.painter().galley(
        egui::pos2(
            title_left + title_width + 12.0,
            rect.center().y - detail.size().y * 0.5,
        ),
        detail,
        tokens.color.text_dim,
    );
    theme::paint_focus_ring(ui, &response, rect);
}

/// A row's label, on the manager's one label column.
fn row_label(ui: &mut Ui, label: &str) {
    let tokens = Tokens::get(ui.ctx());
    let size = Vec2::new(LABEL_WIDTH, tokens.metrics.ctl_h);
    ui.allocate_ui_with_layout(
        size,
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_min_size(size);
            ui.add(
                egui::Label::new(
                    RichText::new(label)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(tokens.color.text_dim),
                )
                .truncate(),
            );
        },
    );
}

/// A section's heading, in the kit's subsection face, with its verbs at the
/// row's trailing end.
fn heading_row<R>(ui: &mut Ui, title: &str, verbs: impl FnOnce(&mut Ui) -> R) -> R {
    let tokens = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.set_min_height(tokens.metrics.ctl_h);
        ui.label(
            RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(tokens.color.text_faint),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), verbs)
            .inner
    })
    .inner
}

/// Room a table cell keeps between its text and each of its edges.
const TABLE_CELL_PAD: f32 = 4.0;

fn table_cell_font(ui: &Ui, kind: TableCell) -> egui::FontId {
    match kind {
        TableCell::Heading => theme::sans(tokens::FS_0, FontWeight::SemiBold),
        TableCell::Value { .. } => egui::TextStyle::Body.resolve(ui.style()),
    }
}

/// Rows a column's width is measured over. Past this a table is a list to
/// search rather than read, and measuring every row each frame would cost
/// more than the rows it saves from elision.
const MEASURED_ROWS: usize = 512;

/// Every column as wide as its widest heading or value, so nothing is cut
/// while the table has room: a share of the room in proportion to authored
/// widths cut the Owner column to "Active schema…" on every row while the
/// empty Parameters column kept a wide share. Room left over is spread
/// evenly; a table wider than its room gives it up in proportion to width.
fn content_column_widths(
    ui: &Ui,
    projection: &crate::state::engineering_table::EngineeringProjection,
    headings: &[String],
    room: f32,
) -> Vec<f32> {
    let measure = |text: &str, kind: TableCell| {
        ui.painter()
            .layout_no_wrap(
                text.to_owned(),
                table_cell_font(ui, kind),
                egui::Color32::PLACEHOLDER,
            )
            .size()
            .x
    };
    let content = projection
        .columns
        .iter()
        .zip(headings)
        .map(|(column, heading)| {
            let widest_value = projection
                .rows
                .iter()
                .take(MEASURED_ROWS)
                .filter_map(|row| row.cells.get(&column.id))
                .map(|cell| measure(&cell.display, TableCell::Value { active: false }))
                .fold(0.0_f32, f32::max);
            (measure(heading, TableCell::Heading).max(widest_value) + 2.0 * TABLE_CELL_PAD).ceil()
        })
        .collect::<Vec<_>>();
    let total = content.iter().sum::<f32>().max(1.0);
    if total <= room {
        let extra = (room - total) / content.len().max(1) as f32;
        content
            .iter()
            .map(|width| (width + extra).floor())
            .collect()
    } else {
        content
            .iter()
            .map(|width| (width * room / total).floor())
            .collect()
    }
}

/// What a table cell holds.
#[derive(Clone, Copy)]
enum TableCell {
    /// A column's heading.
    Heading,
    /// A row's value, `active` while it is the cell the keyboard is on.
    Value { active: bool },
}

/// One cell of the table, exactly its column's share wide: its text from
/// the column's left edge, elided to the share, as a heading or as a value
/// that can be pressed and focused. Cells were buttons that centred their
/// text and grew past the share to fit it, so the rows ran out from under
/// their headings and past the dialog's edge.
fn table_cell(ui: &mut Ui, size: Vec2, text: &str, kind: TableCell) -> egui::Response {
    const PAD: f32 = TABLE_CELL_PAD;
    let tokens = Tokens::get(ui.ctx());
    let sense = match kind {
        TableCell::Heading => egui::Sense::hover(),
        TableCell::Value { .. } => egui::Sense::click(),
    };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    let font = table_cell_font(ui, kind);
    let color = match kind {
        TableCell::Heading => tokens.color.text_dim,
        TableCell::Value { .. } => tokens.color.text,
    };
    if let TableCell::Value { active } = kind {
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), active, text)
        });
        let fill = if active {
            Some(tokens.color.accent.gamma_multiply(0.16))
        } else if response.hovered() {
            Some(tokens.color.bg_hover)
        } else {
            None
        };
        if let Some(fill) = fill {
            ui.painter().rect_filled(rect, 2.0, fill);
        }
        theme::paint_focus_ring(ui, &response, rect);
    } else {
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    }
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap = egui::text::TextWrapping::truncate_at_width((size.x - 2.0 * PAD).max(0.0));
    let galley = ui.painter().layout_job(job);
    ui.painter().galley(
        egui::pos2(rect.left() + PAD, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    response
}

/// Width of a saved view's name on the saved views list.
const SAVED_NAME_WIDTH: f32 = 240.0;

/// Width of a saved view's scope, default and revision on the list.
const SAVED_FACT_WIDTH: f32 = 88.0;

/// Saved views: this grid's views, kept on this device or in the project,
/// each with what can be done to it, and where a view file is imported to.
fn saved_views_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut EngineeringTableDialogState,
    personal: &[crate::state::SavedEngineeringTableView],
    project: &[crate::state::SavedEngineeringTableView],
) -> Option<BodyAction> {
    let tokens = Tokens::get(ui.ctx());
    let mut action = None;
    let scopes = EngineeringViewScope::ALL.map(|scope| scope.label().to_owned());
    ui.horizontal(|ui| {
        row_label(ui, "Filter");
        ui.add_sized(
            [SELECT_WIDTH, tokens.metrics.ctl_h],
            TextEdit::singleline(&mut state.saved_query)
                .font(theme::field_sans())
                .hint_text("View name"),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if Button::new("New from current\u{2026}").show(ui).clicked() {
                action = Some(BodyAction::Save);
            }
        });
    });
    ui.horizontal(|ui| {
        row_label(ui, "Import into");
        if let Some(index) = select(
            ui,
            "engineering-table-import-scope",
            "Import into",
            state.save_scope.label(),
            &scopes,
            SELECT_WIDTH,
        ) {
            state.save_scope = EngineeringViewScope::ALL[index];
        }
        if Button::new("Import\u{2026}").show(ui).clicked() {
            action = Some(BodyAction::ImportSaved);
        }
    });
    ui.add_space(SECTION_GAP);

    let query = state.saved_query.trim().to_lowercase();
    let views = personal
        .iter()
        .chain(project)
        .filter(|saved| saved.definition.grid_id == dataset.id)
        .collect::<Vec<_>>();
    let shown = views
        .iter()
        .filter(|saved| query.is_empty() || saved.name.to_lowercase().contains(&query))
        .collect::<Vec<_>>();
    if shown.is_empty() {
        ui.label(
            RichText::new(if views.is_empty() {
                "This table has no saved views yet."
            } else {
                "No saved view matches the filter."
            })
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(tokens.color.text_dim),
        );
        return action;
    }
    ui.horizontal(|ui| {
        let heading = |text: &str| {
            RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(tokens.color.text_dim)
        };
        list_cell(ui, SAVED_NAME_WIDTH, heading("Name"));
        list_cell(ui, SAVED_FACT_WIDTH, heading("Scope"));
        list_cell(ui, SAVED_FACT_WIDTH, heading("Default"));
        list_cell(ui, SAVED_FACT_WIDTH, heading("Revision"));
    });
    for saved in shown {
        ui.horizontal(|ui| {
            let fact = |text: String| {
                RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(tokens.color.text)
            };
            let editing = state.saved_edit_id.as_deref() == Some(&saved.id);
            if editing {
                ui.add_sized(
                    [SAVED_NAME_WIDTH, tokens.metrics.ctl_h],
                    TextEdit::singleline(&mut state.saved_edit_name)
                        .font(theme::field_sans())
                        .hint_text("View name"),
                );
            } else {
                list_cell(
                    ui,
                    SAVED_NAME_WIDTH,
                    RichText::new(&saved.name)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(tokens.color.text),
                );
            }
            list_cell(ui, SAVED_FACT_WIDTH, fact(saved.scope.label().to_owned()));
            list_cell(
                ui,
                SAVED_FACT_WIDTH,
                fact(if saved.is_default { "Yes" } else { "" }.to_owned()),
            );
            list_cell(ui, SAVED_FACT_WIDTH, fact(saved.revision.to_string()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if editing {
                    if Button::new("Cancel").compact().show(ui).clicked() {
                        state.saved_edit_id = None;
                        state.saved_edit_name.clear();
                    }
                    if Button::new("Save name").compact().show(ui).clicked() {
                        action = Some(BodyAction::CommitRename(saved.id.clone(), saved.scope));
                    }
                    return;
                }
                if Button::new("Delete").compact().show(ui).clicked() {
                    action = Some(BodyAction::DeleteSaved(saved.id.clone(), saved.scope));
                }
                if Button::new("Export\u{2026}").compact().show(ui).clicked() {
                    action = Some(BodyAction::ExportSaved(saved.id.clone(), saved.scope));
                }
                if Button::new("Duplicate").compact().show(ui).clicked() {
                    action = Some(BodyAction::DuplicateSaved(
                        saved.id.clone(),
                        saved.scope,
                        saved.name.clone(),
                    ));
                }
                if Button::new("Rename").compact().show(ui).clicked() {
                    action = Some(BodyAction::BeginRename(
                        saved.id.clone(),
                        saved.name.clone(),
                    ));
                }
                if Button::new("Make default")
                    .compact()
                    .enabled(!saved.is_default)
                    .show(ui)
                    .clicked()
                {
                    action = Some(BodyAction::MakeDefault(saved.id.clone(), saved.scope));
                }
                if Button::new("Use").compact().show(ui).clicked() {
                    action = Some(BodyAction::UseSaved(saved.id.clone(), saved.scope));
                }
            });
        });
    }
    action
}

/// One cell of the saved views list, exactly `width` wide, its text elided
/// to it.
fn list_cell(ui: &mut Ui, width: f32, text: RichText) {
    let size = Vec2::new(width, Tokens::get(ui.ctx()).metrics.ctl_h);
    ui.allocate_ui_with_layout(
        size,
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_min_size(size);
            ui.add(egui::Label::new(text).truncate());
        },
    );
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests {
    use super::*;
    #[cfg(test)]
    use crate::state::{ComponentType, Point};
    #[cfg(test)]
    use crate::workbench::design::test_host::DesignTestHost;

    /// The engineering table over its parts, as its dialog step lends them.
    pub fn table<'a>(
        dialog: &'a mut EngineeringTableDialogState,
        views: &'a mut EngineeringTableViewStore,
        model: &'a mut Model,
        symbol: &'a mut crate::workbench::app::actions::workspace::symbol_document::SymbolUiState,
        platform: &'a mut PlatformState,
        workbench: &'a mut WorkbenchSession,
        registry: &'a crate::properties::PropertyEditorSchema,
    ) -> EngineeringTable<'a> {
        EngineeringTable {
            dialog,
            views,
            model,
            symbol,
            platform,
            workbench,
            registry,
        }
    }

    #[test]
    fn opening_manager_captures_truthful_active_schematic_revision() {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        let revision = state.model.schematic.topology_version();

        assert!(open_engineering_table_dialog(
            &mut state.design.engineering_table,
            &state.design.engineering_table_views,
            &state.model
        ));

        assert!(state.design.engineering_table.open);
        assert_eq!(state.design.engineering_table.source_revision, revision);
        assert_eq!(
            state
                .design
                .engineering_table
                .draft
                .as_ref()
                .unwrap()
                .grid_id,
            crate::state::engineering_table::ACTIVE_SCHEMATIC_GRID_ID
        );
    }

    /// Three 1k resistors, R1 to R3, in a host with a project open, and the
    /// table open on them.
    #[cfg(test)]
    fn three_resistors() -> DesignTestHost {
        let mut state = DesignTestHost::default();
        state.model.project_open.set_from_lifecycle(true);
        for (id, name) in [(1, "R1"), (2, "R2"), (3, "R3")] {
            let mut part = crate::state::Component::new(
                id,
                ComponentType::Resistor,
                Point::new(id as i32 * 100, 0),
            );
            part.name = name.to_owned();
            part.value = "1k".to_owned();
            state
                .model
                .schematic
                .document_mut_for_test()
                .components
                .push(part);
        }
        state.model.schematic.init_undo_history();
        assert!(open_engineering_table_dialog(
            &mut state.design.engineering_table,
            &state.design.engineering_table_views,
            &state.model
        ));
        state
    }

    /// Keep `text` in the cell of `row` and `column`, with `rows` selected,
    /// as the table does when Enter or a click away leaves the cell.
    #[cfg(test)]
    fn keep_cell(state: &mut DesignTestHost, rows: &[u64], row: u64, column: &str, text: &str) {
        let edit = object_cells::ObjectCellEdit {
            rows: rows.iter().map(|id| format!("component-{id}")).collect(),
            row: format!("component-{row}"),
            column: column.to_owned(),
            text: text.to_owned(),
            begun: true,
        };
        let mut platform = PlatformState::default();
        let design = &mut *state.design;
        table(
            &mut design.engineering_table,
            &mut design.engineering_table_views,
            &mut state.model,
            &mut design.symbol,
            &mut platform,
            &mut state.workbench,
            &design.property_registry,
        )
        .edit_object_cells(edit);
    }

    #[cfg(test)]
    fn part(state: &DesignTestHost, id: u64) -> crate::state::Component {
        state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .cloned()
            .expect("the part is there")
    }

    /// A value typed in a selected row's cell is set on every selected part,
    /// by the inspector's rules (resistor code is kept as the number it
    /// means), reads back in the inspector's field, and one Undo takes it
    /// back from every part.
    #[test]
    fn a_value_cell_sets_every_selected_part_and_undoes_in_one_step() {
        let mut state = three_resistors();
        keep_cell(&mut state, &[1, 2], 1, "value", "4k7");

        assert_eq!(state.design.engineering_table.error, None);
        for id in [1, 2] {
            assert_eq!(
                crate::workbench::design::inspector::inline_instance::field_value(
                    &part(&state, id),
                    &crate::workbench::state::InlineEditField::Value
                ),
                "4.7k"
            );
        }
        assert_eq!(part(&state, 3).value, "1k", "R3 was not selected");
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .components
                .contains(&1)
        );

        assert!(state.model.schematic.undo());
        assert_eq!(part(&state, 1).value, "1k");
        assert_eq!(part(&state, 2).value, "1k");
        assert!(!state.model.schematic.can_undo(), "the edit was one step");
    }

    /// A value the inspector refuses is refused at its cell with the
    /// inspector's own reason, the part named when several were selected;
    /// nothing is written and the cell opens again on the refused text.
    #[test]
    fn a_refused_cell_names_the_inspectors_reason_and_writes_nothing() {
        let mut state = three_resistors();
        let inspector = crate::workbench::design::inspector::inline_instance::prepare_field_edit(
            &state.design,
            &state.model,
            &state.workbench,
            &part(&state, 1),
            &crate::workbench::state::InlineEditField::Value,
            "1k+",
        )
        .expect_err("the inspector refuses 1k+");

        keep_cell(&mut state, &[1], 1, "value", "1k+");
        assert_eq!(
            state.design.engineering_table.error.as_deref(),
            Some(inspector.as_str())
        );
        let reopened = state
            .design
            .engineering_table
            .object_edit
            .clone()
            .expect("the cell opens again");
        assert_eq!(reopened.text, "1k+");
        assert_eq!(reopened.column, "value");

        keep_cell(&mut state, &[1, 2], 2, "value", "1k+");
        assert_eq!(
            state.design.engineering_table.error.as_deref(),
            Some(format!("R1: {inspector}").as_str())
        );
        assert_eq!(part(&state, 1).value, "1k");
        assert_eq!(part(&state, 2).value, "1k");
        assert!(!state.model.schematic.can_undo());
    }

    /// A parameters line is held to the part's property sheet one parameter
    /// at a time, as the inspector's rows hold it.
    #[test]
    fn a_parameters_cell_holds_each_parameter_to_its_sheet() {
        let mut state = three_resistors();
        keep_cell(&mut state, &[1], 1, "parameters", "noisy=maybe");
        let refusal = state
            .design
            .engineering_table
            .error
            .clone()
            .expect("noisy takes yes or no");
        assert!(refusal.starts_with("noisy: "), "was {refusal}");
        assert_eq!(part(&state, 1).params, "");

        let inspector = crate::workbench::design::inspector::inline_instance::prepare_field_edit(
            &state.design,
            &state.model,
            &state.workbench,
            &part(&state, 1),
            &crate::workbench::state::InlineEditField::Parameter("noisy".to_owned()),
            "yes",
        )
        .expect("the inspector takes yes");
        keep_cell(&mut state, &[1], 1, "parameters", "noisy=yes");
        assert_eq!(state.design.engineering_table.error, None);
        assert_eq!(
            part(&state, 1).params,
            inspector.params,
            "stored as the inspector stores it"
        );
    }

    /// A name is one part's: the name cell renames its own row whatever
    /// else is selected, and a name another part has is refused for the
    /// inspector's reason.
    #[test]
    fn a_name_cell_renames_its_own_part_alone() {
        let mut state = three_resistors();
        keep_cell(&mut state, &[1, 2], 1, "identifier", "R10");
        assert_eq!(state.design.engineering_table.error, None);
        assert_eq!(part(&state, 1).name, "R10");
        assert_eq!(part(&state, 2).name, "R2");

        keep_cell(&mut state, &[2], 2, "identifier", "r3");
        let refusal = state
            .design
            .engineering_table
            .error
            .clone()
            .expect("R3 is taken");
        assert!(refusal.contains("already exists"), "was {refusal}");
        assert_eq!(part(&state, 2).name, "R2");
    }

    /// The purchasing fields are columns of their own, left out of a new
    /// view until ticked, and a cell of one sets that field on every
    /// selected part.
    #[test]
    fn a_purchasing_field_is_a_column_whose_cells_set_every_selected_part() {
        let mut state = three_resistors();
        let draft = state
            .design
            .engineering_table
            .draft
            .clone()
            .expect("the table has a view");
        let mpn = draft
            .columns
            .iter()
            .find(|column| column.column_id == "bom_mpn")
            .expect("MPN is a column");
        assert!(!mpn.visible, "a purchasing field is ticked in to show");

        keep_cell(&mut state, &[2, 3], 3, "bom_mpn", " RC0603FR-074K7L ");
        assert_eq!(state.design.engineering_table.error, None);
        assert_eq!(part(&state, 1).bom.mpn, "");
        assert_eq!(part(&state, 2).bom.mpn, "RC0603FR-074K7L");
        assert_eq!(part(&state, 3).bom.mpn, "RC0603FR-074K7L");
    }

    /// Exporting the table again to the file it was exported to replaces
    /// the table and its companion both; it used to refuse because the
    /// companion was already there.
    #[cfg(all(test, not(target_arch = "wasm32")))]
    #[test]
    fn re_export_to_the_same_file_replaces_the_table_and_its_companion() {
        let folder = std::env::temp_dir().join(format!(
            "rspice-object-table-re-export-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&folder).unwrap();
        let table = folder.join("parts.csv");
        let schema = table.with_extension("schema.json");
        let io = NativeExportWorkflowIo;
        for (table_text, schema_text) in [("R1,1k\n", "{\"v\":1}"), ("R1,2k\n", "{\"v\":2}")] {
            let observed = io.observe_destination(&table).unwrap();
            publish_with_companion(&io, &observed, &schema, schema_text, table_text)
                .expect("the export is written");
            assert_eq!(std::fs::read_to_string(&table).unwrap(), table_text);
            assert_eq!(std::fs::read_to_string(&schema).unwrap(), schema_text);
        }
        std::fs::remove_dir_all(&folder).unwrap();
    }
}
