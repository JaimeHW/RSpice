//! Universal engineering-table view manager.

use egui::{ComboBox, Context, Grid, RichText, ScrollArea, TextEdit, Ui, Vec2};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::engineering_table::active_schematic_dataset;
use crate::state::{
    EngineeringDataset, EngineeringFilterGrammar, EngineeringSortRule, EngineeringTableView,
    EngineeringViewScope, SortDirection,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, IconButton, select,
};
use crate::workbench::workflows::export_workflow::{
    ExportWorkflowIo, NativeExportWorkflowIo, SaveDialogConfig,
};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum EngineeringTableDialogPage {
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
pub(crate) struct EngineeringTableDialogState {
    pub(crate) open: bool,
    pub(crate) page: EngineeringTableDialogPage,
    pub(crate) draft: Option<crate::state::EngineeringTableView>,
    pub(crate) source_revision: u64,
    pub(crate) selected_saved_id: Option<String>,
    pub(crate) save_name: String,
    pub(crate) save_scope: crate::state::EngineeringViewScope,
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
    pub(crate) row_details_id: Option<String>,
    pub(crate) saved_edit_id: Option<String>,
    pub(crate) saved_edit_name: String,
    #[cfg(target_arch = "wasm32")]
    pub(crate) saved_import_token: Option<crate::workbench::browser::file_import::TextImportToken>,
    pub(crate) error: Option<String>,
}

impl EngineeringTableDialogState {
    pub(crate) fn open(&mut self, view: crate::state::EngineeringTableView, source_revision: u64) {
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

pub(crate) fn open_engineering_table_dialog(state: &mut AppState) -> bool {
    if state.design.engineering_table.open {
        return false;
    }
    let dataset = active_schematic_dataset(&state.model.schematic);
    let mut view = state
        .design
        .engineering_table_views
        .working
        .get(&dataset.id)
        .cloned()
        .or_else(|| {
            state
                .model
                .workspace
                .content
                .engineering_table_views
                .saved
                .iter()
                .find(|saved| saved.definition.grid_id == dataset.id && saved.is_default)
                .map(|saved| saved.definition.clone())
        })
        .or_else(|| {
            state
                .design
                .engineering_table_views
                .saved
                .iter()
                .find(|saved| saved.definition.grid_id == dataset.id && saved.is_default)
                .map(|saved| saved.definition.clone())
        })
        .unwrap_or_else(|| EngineeringTableView::for_dataset(&dataset));
    view.normalize_for(&dataset);
    state
        .design
        .engineering_table
        .open(view, dataset.source_revision);
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_engineering_table_dialog(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        self.consume_engineering_view_import();
        if !self.state.design.engineering_table.open {
            return;
        }

        let dataset = active_schematic_dataset(&self.state.model.schematic);
        if self.state.design.engineering_table.source_revision != dataset.source_revision {
            if let Some(draft) = self.state.design.engineering_table.draft.as_mut() {
                draft.normalize_for(&dataset);
            }
            self.state.design.engineering_table.source_revision = dataset.source_revision;
        }

        let page = self.state.design.engineering_table.page;
        let (title, primary) = match page {
            EngineeringTableDialogPage::Manager => ("Engineering table view", "Apply table view"),
            EngineeringTableDialogPage::SaveView => ("Save engineering table view", "Save view"),
            EngineeringTableDialogPage::SavedViews => ("Saved engineering table views", "Close"),
            EngineeringTableDialogPage::Export => ("Export engineering table", "Export table"),
            EngineeringTableDialogPage::RowDetails => {
                ("Engineering table row details", "Cross-probe source object")
            }
        };
        let mut action = None;
        let choice = Dialog::new(title, primary)
            .description("Choose the table's columns and their order, sort and filter its rows, and copy or export them.")
            .size(DialogSize::WideWorkflow)
            .overlay_scrollbar()
            .ghost(if page == EngineeringTableDialogPage::Manager {
                "Cancel"
            } else {
                "Back"
            })
            .initial_focus(DialogInitialFocus::BodyControl)
            .show_with_initial_body_focus(ctx, |ui| {
                action = match page {
                    EngineeringTableDialogPage::Manager => manager_body(
                        ui,
                        &dataset,
                        &mut self.state.design.engineering_table,
                        &self.state.design.engineering_table_views.saved,
                        &self.state.model.workspace.content.engineering_table_views.saved,
                    ),
                    EngineeringTableDialogPage::SaveView => {
                        save_view_body(ui, &mut self.state.design.engineering_table)
                    }
                    EngineeringTableDialogPage::SavedViews => saved_views_body(
                        ui,
                        &dataset,
                        &mut self.state.design.engineering_table,
                        &self.state.design.engineering_table_views.saved,
                        &self.state.model.workspace.content.engineering_table_views.saved,
                    ),
                    EngineeringTableDialogPage::Export => {
                        export_body(ui, &dataset, &mut self.state.design.engineering_table)
                    }
                    EngineeringTableDialogPage::RowDetails => row_details_body(
                        ui,
                        &dataset,
                        &mut self.state.design.engineering_table,
                    ),
                };
                Some(ui.make_persistent_id(("engineering-table-first", page)))
            });

        if let Some(action) = action {
            self.handle_engineering_table_body_action(action, &dataset);
        }

        match choice {
            DialogChoice::Primary => match page {
                EngineeringTableDialogPage::Manager => self.apply_engineering_table_view(&dataset),
                EngineeringTableDialogPage::SaveView => self.save_engineering_table_view(&dataset),
                EngineeringTableDialogPage::SavedViews => {
                    self.state.design.engineering_table.page = EngineeringTableDialogPage::Manager;
                }
                EngineeringTableDialogPage::Export => self.export_engineering_table(&dataset),
                EngineeringTableDialogPage::RowDetails => {
                    self.cross_probe_engineering_row();
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if page == EngineeringTableDialogPage::Manager {
                    self.state.design.engineering_table.close();
                } else {
                    self.state.design.engineering_table.page = EngineeringTableDialogPage::Manager;
                    self.state.design.engineering_table.error = None;
                }
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
                self.state.design.engineering_table.page = EngineeringTableDialogPage::Export;
                self.state.design.engineering_table.error = None;
            }
            BodyAction::Save => {
                self.state.design.engineering_table.page = EngineeringTableDialogPage::SaveView;
                self.state.design.engineering_table.error = None;
            }
            BodyAction::Manage => {
                self.state.design.engineering_table.page = EngineeringTableDialogPage::SavedViews;
                self.state.design.engineering_table.error = None;
            }
            BodyAction::UseSaved(id, scope) => {
                let saved = match scope {
                    EngineeringViewScope::Personal => self
                        .state
                        .design
                        .engineering_table_views
                        .saved
                        .iter()
                        .find(|saved| saved.id == id),
                    EngineeringViewScope::Project => self
                        .state
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
                    self.state.design.engineering_table.draft = Some(definition);
                    self.state.design.engineering_table.selected_saved_id = Some(saved.id.clone());
                    self.state.design.engineering_table.page = EngineeringTableDialogPage::Manager;
                }
            }
            BodyAction::DeleteSaved(id, scope) => {
                let deleted = match scope {
                    EngineeringViewScope::Personal => {
                        self.state.design.engineering_table_views.delete(&id)
                    }
                    EngineeringViewScope::Project => {
                        let deleted = self
                            .state
                            .model
                            .workspace
                            .content
                            .engineering_table_views
                            .delete(&id);
                        if deleted {
                            self.state.model.workspace.content.project_metadata_dirty = true;
                        }
                        deleted
                    }
                };
                if deleted {
                    self.state.prompt("Saved engineering table view deleted.");
                }
            }
            BodyAction::MakeDefault(id, scope) => {
                let changed = match scope {
                    EngineeringViewScope::Personal => {
                        self.state.design.engineering_table_views.make_default(&id)
                    }
                    EngineeringViewScope::Project => {
                        let changed = self
                            .state
                            .model
                            .workspace
                            .content
                            .engineering_table_views
                            .make_default(&id);
                        if changed {
                            self.state.model.workspace.content.project_metadata_dirty = true;
                        }
                        changed
                    }
                };
                if changed {
                    self.state.prompt("Default engineering table view updated.");
                }
            }
            BodyAction::OpenDetails(id) => {
                self.state.design.engineering_table.row_details_id = Some(id);
                self.state.design.engineering_table.page = EngineeringTableDialogPage::RowDetails;
            }
            BodyAction::ImportSaved => self.import_engineering_saved_view(),
            BodyAction::ExportSaved(id, scope) => self.export_engineering_saved_view(&id, scope),
            BodyAction::BeginRename(id, name) => {
                self.state.design.engineering_table.saved_edit_id = Some(id);
                self.state.design.engineering_table.saved_edit_name = name;
                self.state.design.engineering_table.error = None;
            }
            BodyAction::CommitRename(id, scope) => {
                let name = self.state.design.engineering_table.saved_edit_name.clone();
                let result = match scope {
                    EngineeringViewScope::Personal => {
                        self.state.design.engineering_table_views.rename(&id, &name)
                    }
                    EngineeringViewScope::Project => {
                        let result = self
                            .state
                            .model
                            .workspace
                            .content
                            .engineering_table_views
                            .rename(&id, &name);
                        if result.is_ok() {
                            self.state.model.workspace.content.project_metadata_dirty = true;
                        }
                        result
                    }
                };
                match result {
                    Ok(()) => {
                        self.state.design.engineering_table.saved_edit_id = None;
                        self.state.design.engineering_table.saved_edit_name.clear();
                    }
                    Err(error) => self.state.design.engineering_table.error = Some(error),
                }
            }
            BodyAction::DuplicateSaved(id, scope, source_name) => {
                let store = match scope {
                    EngineeringViewScope::Personal => {
                        &mut self.state.design.engineering_table_views
                    }
                    EngineeringViewScope::Project => {
                        &mut self.state.model.workspace.content.engineering_table_views
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
                            self.state.model.workspace.content.project_metadata_dirty = true;
                        }
                    }
                    Err(error) => self.state.design.engineering_table.error = Some(error),
                }
            }
        }
    }

    fn apply_engineering_table_view(&mut self, dataset: &EngineeringDataset) {
        let Some(mut view) = self.state.design.engineering_table.draft.clone() else {
            return;
        };
        view.normalize_for(dataset);
        self.state
            .design
            .engineering_table_views
            .set_working(view, dataset);
        self.state.design.engineering_table.close();
        self.state
            .prompt("Engineering table view applied on this device; the project is unchanged.");
    }

    fn copy_engineering_table(&mut self, dataset: &EngineeringDataset) {
        let Some(view) = self.state.design.engineering_table.draft.as_ref() else {
            return;
        };
        let selected = (!self
            .state
            .design
            .engineering_table
            .selected_row_ids
            .is_empty())
        .then_some(&self.state.design.engineering_table.selected_row_ids);
        match crate::state::engineering_table::delimited_text_selected(
            dataset, view, b'\t', true, true, false, selected,
        ) {
            Ok(text) => {
                let rows = dataset.project_selected(view, false, selected).rows.len();
                self.state
                    .workbench
                    .outbox
                    .push(crate::workbench::framework::requests::ClipboardText(text));
                self.state.prompt(format!(
                    "{rows} engineering table row{} copied with headers and displayed units.",
                    if rows == 1 { "" } else { "s" }
                ));
            }
            Err(error) => self.state.design.engineering_table.error = Some(error),
        }
    }

    fn save_engineering_table_view(&mut self, dataset: &EngineeringDataset) {
        let Some(view) = self.state.design.engineering_table.draft.clone() else {
            return;
        };
        let name = self.state.design.engineering_table.save_name.clone();
        let scope = self.state.design.engineering_table.save_scope;
        let make_default = self.state.design.engineering_table.save_as_default;
        let result = match scope {
            EngineeringViewScope::Personal => self.state.design.engineering_table_views.save(
                &name,
                scope,
                view,
                make_default,
                dataset,
            ),
            EngineeringViewScope::Project => {
                if !self.state.platform.project_lifecycle.is_open() {
                    Err("Open a project before saving a project-scoped table view.".to_owned())
                } else {
                    let result = self
                        .state
                        .model
                        .workspace
                        .content
                        .engineering_table_views
                        .save(&name, scope, view, make_default, dataset);
                    if result.is_ok() {
                        self.state.model.workspace.content.project_metadata_dirty = true;
                    }
                    result
                }
            }
        };
        match result {
            Ok(id) => {
                self.state.design.engineering_table.selected_saved_id = Some(id);
                self.state.design.engineering_table.page = EngineeringTableDialogPage::Manager;
                self.state.design.engineering_table.save_name.clear();
                self.state.design.engineering_table.error = None;
                self.state.prompt("Engineering table view saved.");
            }
            Err(error) => self.state.design.engineering_table.error = Some(error),
        }
    }

    fn export_engineering_table(&mut self, dataset: &EngineeringDataset) {
        let Some(current) = self.state.design.engineering_table.draft.as_ref() else {
            return;
        };
        let mut view = current.clone();
        let selected_rows = match self.state.design.engineering_table.export_scope {
            EngineeringTableExportScope::CurrentView => None,
            EngineeringTableExportScope::SelectedRows => {
                if self
                    .state
                    .design
                    .engineering_table
                    .selected_row_ids
                    .is_empty()
                {
                    self.state.design.engineering_table.error =
                        Some("Select at least one engineering table row first.".to_owned());
                    return;
                }
                Some(&self.state.design.engineering_table.selected_row_ids)
            }
            EngineeringTableExportScope::CompleteDataset => {
                view.filters.clear();
                None
            }
        };
        let format = self.state.design.engineering_table.export_format;
        let include_headers = self.state.design.engineering_table.export_headers;
        let include_units = self.state.design.engineering_table.export_units;
        let include_metadata = self.state.design.engineering_table.export_metadata;
        let include_hidden = self.state.design.engineering_table.export_hidden_columns;
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
                self.state.design.engineering_table.error = Some(error);
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
                        if schema_path.exists() {
                            return Err(format!(
                                "Schema destination {} already exists; choose another CSV destination.",
                                schema_path.display()
                            ));
                        }
                        // Publish the companion first and the user-selected
                        // CSV last. The primary destination is the completion
                        // marker: a failed companion can never leave a CSV
                        // that appears to be a complete CSV+schema export.
                        io.write_new_text_file(&schema_path, &schema)?;
                        io.write_text_file_observed(&observed, &text)
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
                            if metadata_path.exists() {
                                return Err(format!(
                                    "Metadata destination {} already exists; choose another TSV destination.",
                                    metadata_path.display()
                                ));
                            }
                            // As with CSV+schema, the selected TSV is
                            // published only after its requested metadata
                            // companion.
                            io.write_new_text_file(&metadata_path, &metadata)?;
                            io.write_text_file_observed(&observed, &text)
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
                self.state.design.engineering_table.close();
                self.state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Engineering table exported to {}.", destination.display()),
                );
            }
            Err(error) => self.state.design.engineering_table.error = Some(error),
        }
    }

    fn cross_probe_engineering_row(&mut self) {
        let Some(stable_id) = self
            .state
            .design
            .engineering_table
            .row_details_id
            .as_deref()
        else {
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
        let schematic = &self.state.model.schematic;
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
        let selection = &mut self.state.model.schematic.session.editor.selection;
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
            self.state.design.engineering_table.close();
            self.state.workbench.inspector_visible = true;
            self.state
                .prompt("The source schematic object was selected and opened in the Inspector.");
        } else {
            self.state.design.engineering_table.error =
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
                Err(error) => self.state.design.engineering_table.error = Some(error),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            if self
                .state
                .design
                .engineering_table
                .saved_import_token
                .is_some()
            {
                self.state.design.engineering_table.error =
                    Some("An engineering table view picker is already open.".to_owned());
                return;
            }
            let token = match crate::workbench::browser::file_import::try_begin_text_import(
                crate::workbench::browser::file_import::BrowserTextImportKind::EngineeringTableView,
            ) {
                Ok(token) => token,
                Err(error) => {
                    self.state.design.engineering_table.error = Some(error);
                    return;
                }
            };
            self.state.design.engineering_table.saved_import_token = Some(token);
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
        if self.state.design.engineering_table.saved_import_token != Some(token)
            || !crate::workbench::browser::file_import::finish_text_import(token)
        {
            return;
        }
        self.state.design.engineering_table.saved_import_token = None;
        match result {
            Ok(Some(file)) => self.publish_imported_engineering_view(&file.contents),
            Ok(None) => {}
            Err(error) => self.state.design.engineering_table.error = Some(error),
        }
    }

    fn publish_imported_engineering_view(&mut self, source: &str) {
        let dataset = active_schematic_dataset(&self.state.model.schematic);
        let scope = self.state.design.engineering_table.save_scope;
        let result = match scope {
            EngineeringViewScope::Personal => self
                .state
                .design
                .engineering_table_views
                .import_view(source, scope, &dataset),
            EngineeringViewScope::Project => {
                if !self.state.platform.project_lifecycle.is_open() {
                    Err("Open a project before importing a project-scoped view.".to_owned())
                } else {
                    let result = self
                        .state
                        .model
                        .workspace
                        .content
                        .engineering_table_views
                        .import_view(source, scope, &dataset);
                    if result.is_ok() {
                        self.state.model.workspace.content.project_metadata_dirty = true;
                    }
                    result
                }
            }
        };
        match result {
            Ok(_) => {
                self.state.design.engineering_table.error = None;
                self.state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    "Imported an engineering table view.",
                );
            }
            Err(error) => self.state.design.engineering_table.error = Some(error),
        }
    }

    fn export_engineering_saved_view(&mut self, id: &str, scope: EngineeringViewScope) {
        let result = match scope {
            EngineeringViewScope::Personal => {
                self.state.design.engineering_table_views.export_view(id)
            }
            EngineeringViewScope::Project => self
                .state
                .model
                .workspace
                .content
                .engineering_table_views
                .export_view(id),
        };
        let source = match result {
            Ok(source) => source,
            Err(error) => {
                self.state.design.engineering_table.error = Some(error);
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
                self.state.design.engineering_table.error = Some(error);
                return;
            }
        };
        let result = io
            .observe_destination(&path)
            .and_then(|destination| io.write_text_file_observed(&destination, &source));
        match result {
            Ok(()) => {
                self.state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!(
                        "Saved engineering table view exported to {}.",
                        path.display()
                    ),
                );
            }
            Err(error) => self.state.design.engineering_table.error = Some(error),
        }
    }
}

/// Width of the label column on the manager's label/value rows.
const LABEL_WIDTH: f32 = 84.0;

/// Width of a select on the manager's rows.
const SELECT_WIDTH: f32 = 220.0;

/// Air between the manager's sections.
const SECTION_GAP: f32 = 16.0;

/// Width of the columns list: a column's name and its order controls.
const COLUMNS_WIDTH: f32 = 280.0;

/// The manager is what a reader configures about the table: which view it
/// starts from, which columns it shows in which order, how its rows sort and
/// filter, and the table itself under those choices. It used to state the
/// grid's internal id and render window, a pixel width per column, a pin
/// that nothing reads, the renderer's virtualization, overscan and frozen
/// identifier policies, and a line of keyboard shortcuts; its verbs trailed
/// under a table clipped by a sideways scroll.
fn manager_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut EngineeringTableDialogState,
    personal: &[crate::state::SavedEngineeringTableView],
    project: &[crate::state::SavedEngineeringTableView],
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
    let view = state.draft.as_mut()?;
    ui.add_space(SECTION_GAP);

    let mut move_column = None;
    let mut remove_sort = None;
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
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                    });
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
                    let direction =
                        usize::from(view.sort[index].direction == SortDirection::Descending);
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
                    let grammars =
                        EngineeringFilterGrammar::ALL.map(|grammar| grammar.label().to_owned());
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
    let verbs = heading_row(ui, &rows, |ui| {
        let export = Button::new("Export\u{2026}").show(ui).clicked();
        let copy = Button::new("Copy")
            .show(ui)
            .on_hover_text("Copy the selected rows, or every row, with headings and units")
            .clicked();
        (export, copy)
    });
    match verbs {
        (true, _) => action = Some(BodyAction::Export),
        (_, true) => action = Some(BodyAction::Copy),
        _ => {}
    }
    if let Some(preview_action) = engineering_grid_preview(
        ui,
        view,
        &projection,
        &mut state.selected_row_ids,
        &mut state.active_cell,
        &mut state.focus_cell,
    ) {
        return Some(preview_action);
    }
    if let Some(error) = &state.error {
        ui.colored_label(tokens.color.err, error);
    }
    action
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
    const PAD: f32 = 4.0;
    let tokens = Tokens::get(ui.ctx());
    let sense = match kind {
        TableCell::Heading => egui::Sense::hover(),
        TableCell::Value { .. } => egui::Sense::click(),
    };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    let (font, color) = match kind {
        TableCell::Heading => (
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            tokens.color.text_dim,
        ),
        TableCell::Value { .. } => (egui::TextStyle::Body.resolve(ui.style()), tokens.color.text),
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

fn engineering_grid_preview(
    ui: &mut Ui,
    view: &EngineeringTableView,
    projection: &crate::state::engineering_table::EngineeringProjection,
    selected_rows: &mut std::collections::BTreeSet<String>,
    active_cell: &mut Option<(usize, usize)>,
    focus_cell: &mut Option<(usize, usize)>,
) -> Option<BodyAction> {
    let tokens = Tokens::get(ui.ctx());
    if projection.rows.is_empty() {
        ui.label(RichText::new("No row matches the filters.").color(tokens.color.text_dim));
        return None;
    }

    const CHECK_WIDTH: f32 = 28.0;
    const GAP: f32 = 8.0;
    const CELL_HEIGHT: f32 = 24.0;
    const ROW_STRIDE: f32 = 28.0;
    const SHOWN_ROWS: f32 = 6.0;
    // The columns share the width the dialog gives the table, in the
    // proportions of their authored widths, with the rows' scroll bar kept
    // out of it. Taken at their authored widths they ran past the dialog's
    // edge behind a sideways scroll.
    let authored = projection
        .columns
        .iter()
        .map(|column| {
            view.columns
                .iter()
                .find(|candidate| candidate.column_id == column.id)
                .map_or(120.0, |candidate| f32::from(candidate.width))
        })
        .collect::<Vec<_>>();
    let room = (ui.available_width()
        - CHECK_WIDTH
        - GAP * authored.len() as f32
        - ui.spacing().scroll.allocated_width())
    .max(0.0);
    let total = authored.iter().sum::<f32>().max(1.0);
    let column_widths = authored
        .iter()
        .map(|width| width * room / total)
        .collect::<Vec<_>>();
    let mut action = None;
    Grid::new("engineering-table-preview-grid")
        .min_col_width(0.0)
        .spacing(Vec2::new(GAP, 4.0))
        .show(ui, |ui| {
            ui.allocate_exact_size(Vec2::new(CHECK_WIDTH, CELL_HEIGHT), egui::Sense::hover());
            for (column, width) in projection.columns.iter().zip(&column_widths) {
                let heading = column.unit.as_ref().map_or_else(
                    || column.label.clone(),
                    |unit| format!("{} [{}]", column.label, unit),
                );
                table_cell(
                    ui,
                    Vec2::new(*width, CELL_HEIGHT),
                    &heading,
                    TableCell::Heading,
                )
                .on_hover_text(heading.as_str());
            }
            ui.end_row();
        });
    ScrollArea::vertical()
        .id_salt("engineering-table-preview-vertical")
        .max_height(ROW_STRIDE * SHOWN_ROWS)
        .auto_shrink([false, false])
        .show_rows(ui, ROW_STRIDE, projection.rows.len(), |ui, visible| {
            Grid::new("engineering-table-preview-rows")
                .striped(true)
                .min_col_width(0.0)
                .spacing(Vec2::new(8.0, 4.0))
                .show(ui, |ui| {
                    for row_index in visible {
                        let row = &projection.rows[row_index];
                        let mut selected = selected_rows.contains(&row.stable_id);
                        let selected_response = ui
                            .allocate_ui_with_layout(
                                Vec2::new(CHECK_WIDTH, CELL_HEIGHT),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_min_size(Vec2::new(CHECK_WIDTH, CELL_HEIGHT));
                                    ui.checkbox(&mut selected, "")
                                },
                            )
                            .inner
                            .on_hover_text("Select row for copy or export");
                        if selected_response.changed() {
                            if selected {
                                selected_rows.insert(row.stable_id.clone());
                            } else {
                                selected_rows.remove(&row.stable_id);
                            }
                        }
                        for (column_index, (column, width)) in
                            projection.columns.iter().zip(&column_widths).enumerate()
                        {
                            let selected_cell = *active_cell == Some((row_index, column_index));
                            let text = row
                                .cells
                                .get(&column.id)
                                .map(|cell| cell.display.as_str())
                                .unwrap_or_default();
                            let response = table_cell(
                                ui,
                                Vec2::new(*width, CELL_HEIGHT),
                                text,
                                TableCell::Value {
                                    active: selected_cell,
                                },
                            )
                            .on_hover_text(text);
                            if *focus_cell == Some((row_index, column_index)) {
                                response.request_focus();
                                *focus_cell = None;
                            }
                            if response.clicked() {
                                *active_cell = Some((row_index, column_index));
                                response.request_focus();
                            }
                            if response.has_focus() {
                                let modifiers = ui.input(|input| input.modifiers);
                                let move_to = ui.input(|input| {
                                    if input.key_pressed(egui::Key::ArrowLeft) {
                                        Some((row_index, column_index.saturating_sub(1)))
                                    } else if input.key_pressed(egui::Key::ArrowRight) {
                                        Some((
                                            row_index,
                                            (column_index + 1)
                                                .min(projection.columns.len().saturating_sub(1)),
                                        ))
                                    } else if input.key_pressed(egui::Key::ArrowUp) {
                                        Some((row_index.saturating_sub(1), column_index))
                                    } else if input.key_pressed(egui::Key::ArrowDown) {
                                        Some((
                                            (row_index + 1)
                                                .min(projection.rows.len().saturating_sub(1)),
                                            column_index,
                                        ))
                                    } else if input.key_pressed(egui::Key::Home) {
                                        Some((if modifiers.ctrl { 0 } else { row_index }, 0))
                                    } else if input.key_pressed(egui::Key::End) {
                                        Some((
                                            if modifiers.ctrl {
                                                projection.rows.len().saturating_sub(1)
                                            } else {
                                                row_index
                                            },
                                            projection.columns.len().saturating_sub(1),
                                        ))
                                    } else {
                                        None
                                    }
                                });
                                if let Some(target) = move_to {
                                    *active_cell = Some(target);
                                    *focus_cell = Some(target);
                                }
                                if modifiers.shift
                                    && ui.input(|input| input.key_pressed(egui::Key::Space))
                                    && !selected_rows.insert(row.stable_id.clone())
                                {
                                    selected_rows.remove(&row.stable_id);
                                }
                                if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                                    action = Some(BodyAction::OpenDetails(row.stable_id.clone()));
                                }
                                if modifiers.command
                                    && ui.input(|input| input.key_pressed(egui::Key::C))
                                {
                                    action = Some(BodyAction::Copy);
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    action
}

fn save_view_body(ui: &mut Ui, state: &mut EngineeringTableDialogState) -> Option<BodyAction> {
    ui.label("Name");
    ui.add(
        TextEdit::singleline(&mut state.save_name)
            .desired_width(f32::INFINITY)
            .hint_text("Saved view name"),
    );
    ComboBox::from_label("Scope")
        .selected_text(state.save_scope.label())
        .show_ui(ui, |ui| {
            for scope in EngineeringViewScope::ALL {
                ui.selectable_value(&mut state.save_scope, scope, scope.label());
            }
        });
    ui.checkbox(
        &mut state.save_as_default,
        "Default for this grid in the selected scope",
    );
    ui.small(match state.save_scope {
        EngineeringViewScope::Personal => {
            "Personal views are device preferences and do not modify the project."
        }
        EngineeringViewScope::Project => {
            "Project views are versioned project metadata and participate in Save/Revert."
        }
    });
    if let Some(error) = &state.error {
        ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
    }
    None
}

fn saved_views_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut EngineeringTableDialogState,
    personal: &[crate::state::SavedEngineeringTableView],
    project: &[crate::state::SavedEngineeringTableView],
) -> Option<BodyAction> {
    let mut action = None;
    ui.horizontal(|ui| {
        if ui.button("New from current").clicked() {
            action = Some(BodyAction::Save);
        }
        if ui.button("Import…").clicked() {
            action = Some(BodyAction::ImportSaved);
        }
        let selected = state
            .selected_saved_id
            .as_deref()
            .and_then(|id| personal.iter().chain(project).find(|saved| saved.id == id));
        if ui
            .add_enabled(selected.is_some(), egui::Button::new("Export…"))
            .clicked()
            && let Some(selected) = selected
        {
            action = Some(BodyAction::ExportSaved(selected.id.clone(), selected.scope));
        }
        ComboBox::from_id_salt("saved-view-import-scope")
            .selected_text(format!("Import to {}", state.save_scope.label()))
            .show_ui(ui, |ui| {
                for scope in EngineeringViewScope::ALL {
                    ui.selectable_value(&mut state.save_scope, scope, scope.label());
                }
            });
        ui.separator();
        ui.label("Saved view");
        ui.add(
            TextEdit::singleline(&mut state.saved_query)
                .hint_text("Filter saved views…")
                .desired_width(260.0),
        );
    });
    ui.separator();
    let query = state.saved_query.trim().to_lowercase();
    for saved in personal.iter().chain(project).filter(|saved| {
        saved.definition.grid_id == dataset.id
            && (query.is_empty() || saved.name.to_lowercase().contains(&query))
    }) {
        ui.horizontal(|ui| {
            let editing = state.saved_edit_id.as_deref() == Some(&saved.id);
            if editing {
                ui.add(
                    TextEdit::singleline(&mut state.saved_edit_name)
                        .desired_width(180.0)
                        .hint_text("View name"),
                );
                if ui.button("Save name").clicked() {
                    action = Some(BodyAction::CommitRename(saved.id.clone(), saved.scope));
                }
                if ui.button("Cancel rename").clicked() {
                    state.saved_edit_id = None;
                    state.saved_edit_name.clear();
                }
            } else {
                ui.strong(&saved.name);
            }
            ui.label(saved.scope.label());
            ui.monospace(&saved.definition.grid_id);
            if saved.is_default {
                ui.colored_label(Tokens::get(ui.ctx()).color.text_dim, "default");
            }
            ui.label(format!("revision {}", saved.revision));
            if ui.button("Use").clicked() {
                action = Some(BodyAction::UseSaved(saved.id.clone(), saved.scope));
            }
            if !saved.is_default && ui.button("Make default").clicked() {
                action = Some(BodyAction::MakeDefault(saved.id.clone(), saved.scope));
            }
            if !editing && ui.button("Rename").clicked() {
                action = Some(BodyAction::BeginRename(
                    saved.id.clone(),
                    saved.name.clone(),
                ));
            }
            if ui.button("Duplicate").clicked() {
                action = Some(BodyAction::DuplicateSaved(
                    saved.id.clone(),
                    saved.scope,
                    saved.name.clone(),
                ));
            }
            if ui.button("Export").clicked() {
                action = Some(BodyAction::ExportSaved(saved.id.clone(), saved.scope));
            }
            if ui.button("Delete").clicked() {
                action = Some(BodyAction::DeleteSaved(saved.id.clone(), saved.scope));
            }
        });
        ui.separator();
    }
    if personal
        .iter()
        .chain(project)
        .all(|saved| saved.definition.grid_id != dataset.id)
    {
        ui.label("No saved views exist for this grid.");
    }
    action
}

fn export_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut EngineeringTableDialogState,
) -> Option<BodyAction> {
    ComboBox::from_label("Scope")
        .selected_text(state.export_scope.label())
        .show_ui(ui, |ui| {
            for scope in EngineeringTableExportScope::ALL {
                ui.selectable_value(&mut state.export_scope, scope, scope.label());
            }
        });
    ComboBox::from_label("Format")
        .selected_text(state.export_format.label())
        .show_ui(ui, |ui| {
            for format in EngineeringTableExportFormat::ALL {
                ui.selectable_value(&mut state.export_format, format, format.label());
            }
        });
    ui.checkbox(&mut state.export_headers, "Include column headers");
    ui.checkbox(
        &mut state.export_units,
        "Include engineering units and quantity types in headers",
    );
    ui.checkbox(
        &mut state.export_metadata,
        "Include filter, sort, source revision, and manifest metadata",
    );
    ui.checkbox(&mut state.export_hidden_columns, "Include hidden columns");
    let current_rows = state
        .draft
        .as_ref()
        .map(|view| dataset.project(view).rows.len())
        .unwrap_or_default();
    Grid::new("engineering-export-summary")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Grid");
            ui.monospace(&dataset.id);
            ui.end_row();
            ui.label("Logical rows");
            ui.strong(dataset.rows.len().to_string());
            ui.end_row();
            ui.label("Current rows");
            ui.strong(current_rows.to_string());
            ui.end_row();
            ui.label("Selected rows");
            ui.strong(state.selected_row_ids.len().to_string());
            ui.end_row();
            ui.label("Source revision");
            ui.monospace(dataset.source_revision.to_string());
            ui.end_row();
        });
    if let Some(error) = &state.error {
        ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
    }
    None
}

fn row_details_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut EngineeringTableDialogState,
) -> Option<BodyAction> {
    let row = state
        .row_details_id
        .as_deref()
        .and_then(|id| dataset.rows.iter().find(|row| row.stable_id == id));
    let Some(row) = row else {
        ui.colored_label(
            Tokens::get(ui.ctx()).color.err,
            "The source row no longer exists in the active engineering dataset.",
        );
        return None;
    };
    ui.label("Stable source identity");
    ui.monospace(&row.stable_id);
    ui.separator();
    Grid::new("engineering-row-details")
        .num_columns(3)
        .striped(true)
        .show(ui, |ui| {
            ui.strong("Field");
            ui.strong("Value");
            ui.strong("Unit");
            ui.end_row();
            for column in &dataset.columns {
                ui.label(&column.label);
                ui.monospace(
                    row.cells
                        .get(&column.id)
                        .map(|cell| cell.display.as_str())
                        .unwrap_or_default(),
                );
                ui.label(column.unit.as_deref().unwrap_or("—"));
                ui.end_row();
            }
        });
    ui.small(format!(
        "Source revision {} · cross-probe selects the matching schematic object without changing it.",
        dataset.source_revision
    ));
    if let Some(error) = &state.error {
        ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};

    #[test]
    fn opening_manager_captures_truthful_active_schematic_revision() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        let revision = state.model.schematic.topology_version();

        assert!(open_engineering_table_dialog(&mut state));

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

    #[test]
    fn project_scope_is_versioned_while_personal_scope_is_not() {
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let dataset = active_schematic_dataset(&app.state.model.schematic);
        app.state.design.engineering_table.open(
            EngineeringTableView::for_dataset(&dataset),
            dataset.source_revision,
        );
        app.state.design.engineering_table.save_name = "Personal review".to_owned();
        app.state.design.engineering_table.save_scope = EngineeringViewScope::Personal;
        app.save_engineering_table_view(&dataset);
        assert!(!app.state.model.workspace.content.project_metadata_dirty);

        app.state.design.engineering_table.save_name = "Project review".to_owned();
        app.state.design.engineering_table.save_scope = EngineeringViewScope::Project;
        app.save_engineering_table_view(&dataset);
        assert!(app.state.model.workspace.content.project_metadata_dirty);
    }

    /// The manager offers what a reader configures, and its table fits it:
    /// no grid id, render window, pixel widths, pin, renderer policies or
    /// keyboard line, and every column under its heading inside the
    /// dialog. The table's columns ran past the dialog's edge behind a
    /// sideways scroll, each value centred in a button under a heading
    /// laid out apart from it.
    #[test]
    fn the_manager_offers_what_a_reader_configures_and_its_table_fits() {
        let mut app = RSpiceApp::test_instance();
        rspice_app_state::workbench::examples::load_example(
            "RC Lowpass Filter",
            &mut app.state.model.schematic,
        );
        assert!(open_engineering_table_dialog(&mut app.state));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut shapes = Vec::new();
        for _ in 0..4 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1440.0, 900.0),
                        )),
                        ..Default::default()
                    },
                    |ui| app.render_engineering_table_dialog(ui.ctx()),
                )
                .shapes;
        }
        let texts = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        let at = |wanted: &str| {
            texts
                .iter()
                .find(|(text, _)| text == wanted)
                .map(|(_, rect)| *rect)
                .unwrap_or_else(|| panic!("{wanted:?} was not painted: {texts:?}"))
        };
        for wanted in ["Columns", "Sort", "Filter", "Saved view"] {
            let _ = at(wanted);
        }
        for (text, _) in &texts {
            for gone in [
                "px",
                "Active grid",
                "logical rows",
                "pinned",
                "Large-table rendering",
                "Overscan",
                "Frozen identifiers",
                "Keyboard",
            ] {
                assert!(!text.contains(gone), "{text:?} is still painted");
            }
        }
        let title = at("Engineering table view");
        // The footer's last button ends its padding past its label: the
        // dialog's content edge, which no cell may cross.
        let edge = at("Apply table view").right() + 12.0;
        for (text, rect) in &texts {
            assert!(
                rect.right() <= edge,
                "{text:?} ends at {}, past the dialog's content edge {edge}",
                rect.right(),
            );
        }
        // The table's heading, under the columns list's own "Identifier".
        let heading = texts
            .iter()
            .filter(|(text, _)| text == "Identifier")
            .map(|(_, rect)| *rect)
            .max_by(|a, b| a.top().total_cmp(&b.top()))
            .expect("the table is headed");
        let value = at("VIN");
        assert!(
            (heading.left() - value.left()).abs() < 0.5,
            "the heading starts at {} and its values at {}",
            heading.left(),
            value.left()
        );
        assert!(
            (at("Saved view").left() - title.left()).abs() < 0.5,
            "the first row starts at {}, the title at {}",
            at("Saved view").left(),
            title.left()
        );
    }
}
