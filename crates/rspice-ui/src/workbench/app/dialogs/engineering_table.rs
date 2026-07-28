//! Universal engineering-table view manager.

use egui::{ComboBox, Context, Grid, RichText, ScrollArea, TextEdit, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    EngineeringDataset, EngineeringFilterGrammar, EngineeringSortRule, EngineeringTableView,
    EngineeringViewScope, EngineeringVirtualizationPolicy, FrozenIdentifierPolicy, SortDirection,
};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::workflows::export_workflow::{
    ExportWorkflowIo, NativeExportWorkflowIo, SaveDialogConfig,
};

use crate::workbench::app::{EngineeringTableDialogPage, EngineeringTableExportFormat, EngineeringTableExportScope, RSpiceApp};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "VIEW \u{00b7} UNIVERSAL DATA-GRID CONTRACT";

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

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_SAVED_VIEW_IMPORT: std::cell::RefCell<Option<(
        crate::workbench::browser::file_import::TextImportToken,
        Result<Option<crate::workbench::browser::file_import::PickedTextFile>, String>,
    )>> = const { std::cell::RefCell::new(None) };
}

pub(crate) fn open_engineering_table_dialog(state: &mut AppState) -> bool {
    if state.dialogs.engineering_table.open {
        return false;
    }
    let dataset = EngineeringDataset::active_schematic(&state.schematic);
    let mut view = state
        .ui
        .engineering_table_views
        .working
        .get(&dataset.id)
        .cloned()
        .or_else(|| {
            state
                .workspace
                .engineering_table_views
                .saved
                .iter()
                .find(|saved| saved.definition.grid_id == dataset.id && saved.is_default)
                .map(|saved| saved.definition.clone())
        })
        .or_else(|| {
            state
                .ui
                .engineering_table_views
                .saved
                .iter()
                .find(|saved| saved.definition.grid_id == dataset.id && saved.is_default)
                .map(|saved| saved.definition.clone())
        })
        .unwrap_or_else(|| EngineeringTableView::for_dataset(&dataset));
    view.normalize_for(&dataset);
    state
        .dialogs
        .engineering_table
        .open(view, dataset.source_revision);
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_engineering_table_dialog(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        self.consume_engineering_view_import();
        if !self.state.dialogs.engineering_table.open {
            return;
        }

        let dataset = EngineeringDataset::active_schematic(&self.state.schematic);
        if self.state.dialogs.engineering_table.source_revision != dataset.source_revision {
            if let Some(draft) = self.state.dialogs.engineering_table.draft.as_mut() {
                draft.normalize_for(&dataset);
            }
            self.state.dialogs.engineering_table.source_revision = dataset.source_revision;
        }

        let page = self.state.dialogs.engineering_table.page;
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
        let choice = Dialog::new(EYEBROW, title, primary)
            .description("Manage exact table columns, sort, typed filters, rendering policy, named views, clipboard data, and exported artifacts.")
            .size(DialogSize::WideWorkflow)
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
                        &mut self.state.dialogs.engineering_table,
                        &self.state.ui.engineering_table_views.saved,
                        &self.state.workspace.engineering_table_views.saved,
                    ),
                    EngineeringTableDialogPage::SaveView => {
                        save_view_body(ui, &mut self.state.dialogs.engineering_table)
                    }
                    EngineeringTableDialogPage::SavedViews => saved_views_body(
                        ui,
                        &dataset,
                        &mut self.state.dialogs.engineering_table,
                        &self.state.ui.engineering_table_views.saved,
                        &self.state.workspace.engineering_table_views.saved,
                    ),
                    EngineeringTableDialogPage::Export => {
                        export_body(ui, &dataset, &mut self.state.dialogs.engineering_table)
                    }
                    EngineeringTableDialogPage::RowDetails => row_details_body(
                        ui,
                        &dataset,
                        &mut self.state.dialogs.engineering_table,
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
                    self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::Manager;
                }
                EngineeringTableDialogPage::Export => self.export_engineering_table(&dataset),
                EngineeringTableDialogPage::RowDetails => {
                    self.cross_probe_engineering_row();
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if page == EngineeringTableDialogPage::Manager {
                    self.state.dialogs.engineering_table.close();
                } else {
                    self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::Manager;
                    self.state.dialogs.engineering_table.error = None;
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
                self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::Export;
                self.state.dialogs.engineering_table.error = None;
            }
            BodyAction::Save => {
                self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::SaveView;
                self.state.dialogs.engineering_table.error = None;
            }
            BodyAction::Manage => {
                self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::SavedViews;
                self.state.dialogs.engineering_table.error = None;
            }
            BodyAction::UseSaved(id, scope) => {
                let saved = match scope {
                    EngineeringViewScope::Personal => self
                        .state
                        .ui
                        .engineering_table_views
                        .saved
                        .iter()
                        .find(|saved| saved.id == id),
                    EngineeringViewScope::Project => self
                        .state
                        .workspace
                        .engineering_table_views
                        .saved
                        .iter()
                        .find(|saved| saved.id == id),
                };
                if let Some(saved) = saved {
                    let mut definition = saved.definition.clone();
                    definition.normalize_for(dataset);
                    self.state.dialogs.engineering_table.draft = Some(definition);
                    self.state.dialogs.engineering_table.selected_saved_id = Some(saved.id.clone());
                    self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::Manager;
                }
            }
            BodyAction::DeleteSaved(id, scope) => {
                let deleted = match scope {
                    EngineeringViewScope::Personal => {
                        self.state.ui.engineering_table_views.delete(&id)
                    }
                    EngineeringViewScope::Project => {
                        let deleted = self.state.workspace.engineering_table_views.delete(&id);
                        if deleted {
                            self.state.workspace.project_metadata_dirty = true;
                        }
                        deleted
                    }
                };
                if deleted {
                    self.state.push_user_message(ConsoleMessage::info(
                        "Saved engineering table view deleted.",
                    ));
                }
            }
            BodyAction::MakeDefault(id, scope) => {
                let changed = match scope {
                    EngineeringViewScope::Personal => {
                        self.state.ui.engineering_table_views.make_default(&id)
                    }
                    EngineeringViewScope::Project => {
                        let changed = self
                            .state
                            .workspace
                            .engineering_table_views
                            .make_default(&id);
                        if changed {
                            self.state.workspace.project_metadata_dirty = true;
                        }
                        changed
                    }
                };
                if changed {
                    self.state.push_user_message(ConsoleMessage::info(
                        "Default engineering table view updated.",
                    ));
                }
            }
            BodyAction::OpenDetails(id) => {
                self.state.dialogs.engineering_table.row_details_id = Some(id);
                self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::RowDetails;
            }
            BodyAction::ImportSaved => self.import_engineering_saved_view(),
            BodyAction::ExportSaved(id, scope) => self.export_engineering_saved_view(&id, scope),
            BodyAction::BeginRename(id, name) => {
                self.state.dialogs.engineering_table.saved_edit_id = Some(id);
                self.state.dialogs.engineering_table.saved_edit_name = name;
                self.state.dialogs.engineering_table.error = None;
            }
            BodyAction::CommitRename(id, scope) => {
                let name = self.state.dialogs.engineering_table.saved_edit_name.clone();
                let result = match scope {
                    EngineeringViewScope::Personal => {
                        self.state.ui.engineering_table_views.rename(&id, &name)
                    }
                    EngineeringViewScope::Project => {
                        let result = self
                            .state
                            .workspace
                            .engineering_table_views
                            .rename(&id, &name);
                        if result.is_ok() {
                            self.state.workspace.project_metadata_dirty = true;
                        }
                        result
                    }
                };
                match result {
                    Ok(()) => {
                        self.state.dialogs.engineering_table.saved_edit_id = None;
                        self.state.dialogs.engineering_table.saved_edit_name.clear();
                    }
                    Err(error) => self.state.dialogs.engineering_table.error = Some(error),
                }
            }
            BodyAction::DuplicateSaved(id, scope, source_name) => {
                let store = match scope {
                    EngineeringViewScope::Personal => &mut self.state.ui.engineering_table_views,
                    EngineeringViewScope::Project => {
                        &mut self.state.workspace.engineering_table_views
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
                            self.state.workspace.project_metadata_dirty = true;
                        }
                    }
                    Err(error) => self.state.dialogs.engineering_table.error = Some(error),
                }
            }
        }
    }

    fn apply_engineering_table_view(&mut self, dataset: &EngineeringDataset) {
        let Some(mut view) = self.state.dialogs.engineering_table.draft.clone() else {
            return;
        };
        view.normalize_for(dataset);
        self.state
            .ui
            .engineering_table_views
            .set_working(view, dataset);
        self.state.dialogs.engineering_table.close();
        self.state.push_user_message(ConsoleMessage::info(
            "Engineering table view applied to this device session; project engineering data was unchanged.",
        ));
    }

    fn copy_engineering_table(&mut self, dataset: &EngineeringDataset) {
        let Some(view) = self.state.dialogs.engineering_table.draft.as_ref() else {
            return;
        };
        let selected = (!self
            .state
            .dialogs
            .engineering_table
            .selected_row_ids
            .is_empty())
        .then_some(&self.state.dialogs.engineering_table.selected_row_ids);
        match crate::state::engineering_table::delimited_text_selected(
            dataset, view, b'\t', true, true, false, selected,
        ) {
            Ok(text) => {
                let rows = dataset.project_selected(view, false, selected).rows.len();
                self.state.ui.clipboard_text_request = Some(text);
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "{rows} engineering table row{} copied with headers and displayed units.",
                    if rows == 1 { "" } else { "s" }
                )));
            }
            Err(error) => self.state.dialogs.engineering_table.error = Some(error),
        }
    }

    fn save_engineering_table_view(&mut self, dataset: &EngineeringDataset) {
        let Some(view) = self.state.dialogs.engineering_table.draft.clone() else {
            return;
        };
        let name = self.state.dialogs.engineering_table.save_name.clone();
        let scope = self.state.dialogs.engineering_table.save_scope;
        let make_default = self.state.dialogs.engineering_table.save_as_default;
        let result = match scope {
            EngineeringViewScope::Personal => self.state.ui.engineering_table_views.save(
                &name,
                scope,
                view,
                make_default,
                dataset,
            ),
            EngineeringViewScope::Project => {
                if !self.state.project_lifecycle.project_open {
                    Err("Open a project before saving a project-scoped table view.".to_owned())
                } else {
                    let result = self.state.workspace.engineering_table_views.save(
                        &name,
                        scope,
                        view,
                        make_default,
                        dataset,
                    );
                    if result.is_ok() {
                        self.state.workspace.project_metadata_dirty = true;
                    }
                    result
                }
            }
        };
        match result {
            Ok(id) => {
                self.state.dialogs.engineering_table.selected_saved_id = Some(id);
                self.state.dialogs.engineering_table.page = EngineeringTableDialogPage::Manager;
                self.state.dialogs.engineering_table.save_name.clear();
                self.state.dialogs.engineering_table.error = None;
                self.state
                    .push_user_message(ConsoleMessage::info("Engineering table view saved."));
            }
            Err(error) => self.state.dialogs.engineering_table.error = Some(error),
        }
    }

    fn export_engineering_table(&mut self, dataset: &EngineeringDataset) {
        let Some(current) = self.state.dialogs.engineering_table.draft.as_ref() else {
            return;
        };
        let mut view = current.clone();
        let selected_rows = match self.state.dialogs.engineering_table.export_scope {
            EngineeringTableExportScope::CurrentView => None,
            EngineeringTableExportScope::SelectedRows => {
                if self
                    .state
                    .dialogs
                    .engineering_table
                    .selected_row_ids
                    .is_empty()
                {
                    self.state.dialogs.engineering_table.error =
                        Some("Select at least one engineering table row first.".to_owned());
                    return;
                }
                Some(&self.state.dialogs.engineering_table.selected_row_ids)
            }
            EngineeringTableExportScope::CompleteDataset => {
                view.filters.clear();
                None
            }
        };
        let format = self.state.dialogs.engineering_table.export_format;
        let include_headers = self.state.dialogs.engineering_table.export_headers;
        let include_units = self.state.dialogs.engineering_table.export_units;
        let include_metadata = self.state.dialogs.engineering_table.export_metadata;
        let include_hidden = self.state.dialogs.engineering_table.export_hidden_columns;
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
                self.state.dialogs.engineering_table.error = Some(error);
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
                            crate::workbench::workflows::export_workflow::deterministic_stored_zip(&[
                                ("table.csv", text.as_bytes()),
                                ("schema.json", schema.as_bytes()),
                            ])?;
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
                                ])?;
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
                self.state.dialogs.engineering_table.close();
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "Engineering table exported to {}.",
                    destination.display()
                )));
            }
            Err(error) => self.state.dialogs.engineering_table.error = Some(error),
        }
    }

    fn cross_probe_engineering_row(&mut self) {
        let Some(stable_id) = self
            .state
            .dialogs
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
        let schematic = &self.state.schematic;
        let target = parse_id("component-")
            .filter(|id| schematic.components.iter().any(|object| object.id == *id))
            .map(CrossProbeTarget::Component)
            .or_else(|| {
                parse_id("wire-")
                    .filter(|id| schematic.wires.iter().any(|object| object.id == *id))
                    .map(CrossProbeTarget::Wire)
            })
            .or_else(|| {
                parse_id("net-label-")
                    .filter(|id| schematic.net_labels.iter().any(|object| object.id == *id))
                    .map(CrossProbeTarget::NetLabel)
            })
            .or_else(|| {
                parse_id("bus-")
                    .filter(|id| schematic.buses.iter().any(|object| object.id == *id))
                    .map(CrossProbeTarget::Bus)
            })
            .or_else(|| {
                parse_id("bus-tap-")
                    .filter(|id| schematic.bus_taps.iter().any(|object| object.id == *id))
                    .map(CrossProbeTarget::BusTap)
            })
            .or_else(|| {
                parse_id("junction-").and_then(|id| {
                    schematic
                        .junctions
                        .iter()
                        .find(|object| object.id == id)
                        .map(|object| CrossProbeTarget::Junction(object.pos))
                })
            });
        let selection = &mut self.state.schematic.selection;
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
            self.state.dialogs.engineering_table.close();
            self.state.workbench.inspector_visible = true;
            self.state.push_user_message(ConsoleMessage::info(
                "The source schematic object was selected and opened in the Inspector.",
            ));
        } else {
            self.state.dialogs.engineering_table.error =
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
                Err(error) => self.state.dialogs.engineering_table.error = Some(error),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            if self
                .state
                .dialogs
                .engineering_table
                .saved_import_token
                .is_some()
            {
                self.state.dialogs.engineering_table.error =
                    Some("An engineering table view picker is already open.".to_owned());
                return;
            }
            let token = match crate::workbench::browser::file_import::try_begin_text_import(
                crate::workbench::browser::file_import::BrowserTextImportKind::EngineeringTableView,
            ) {
                Ok(token) => token,
                Err(error) => {
                    self.state.dialogs.engineering_table.error = Some(error);
                    return;
                }
            };
            self.state.dialogs.engineering_table.saved_import_token = Some(token);
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
        if self.state.dialogs.engineering_table.saved_import_token != Some(token)
            || !crate::workbench::browser::file_import::finish_text_import(token)
        {
            return;
        }
        self.state.dialogs.engineering_table.saved_import_token = None;
        match result {
            Ok(Some(file)) => self.publish_imported_engineering_view(&file.contents),
            Ok(None) => {}
            Err(error) => self.state.dialogs.engineering_table.error = Some(error),
        }
    }

    fn publish_imported_engineering_view(&mut self, source: &str) {
        let dataset = EngineeringDataset::active_schematic(&self.state.schematic);
        let scope = self.state.dialogs.engineering_table.save_scope;
        let result = match scope {
            EngineeringViewScope::Personal => self
                .state
                .ui
                .engineering_table_views
                .import_view(source, scope, &dataset),
            EngineeringViewScope::Project => {
                if !self.state.project_lifecycle.project_open {
                    Err("Open a project before importing a project-scoped view.".to_owned())
                } else {
                    let result = self
                        .state
                        .workspace
                        .engineering_table_views
                        .import_view(source, scope, &dataset);
                    if result.is_ok() {
                        self.state.workspace.project_metadata_dirty = true;
                    }
                    result
                }
            }
        };
        match result {
            Ok(_) => {
                self.state.dialogs.engineering_table.error = None;
                self.state.push_user_message(ConsoleMessage::info(
                    "Engineering table view imported and validated.",
                ));
            }
            Err(error) => self.state.dialogs.engineering_table.error = Some(error),
        }
    }

    fn export_engineering_saved_view(&mut self, id: &str, scope: EngineeringViewScope) {
        let result = match scope {
            EngineeringViewScope::Personal => self.state.ui.engineering_table_views.export_view(id),
            EngineeringViewScope::Project => {
                self.state.workspace.engineering_table_views.export_view(id)
            }
        };
        let source = match result {
            Ok(source) => source,
            Err(error) => {
                self.state.dialogs.engineering_table.error = Some(error);
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
                self.state.dialogs.engineering_table.error = Some(error);
                return;
            }
        };
        let result = io
            .observe_destination(&path)
            .and_then(|destination| io.write_text_file_observed(&destination, &source));
        match result {
            Ok(()) => self.state.push_user_message(ConsoleMessage::info(format!(
                "Saved engineering table view exported to {}.",
                path.display()
            ))),
            Err(error) => self.state.dialogs.engineering_table.error = Some(error),
        }
    }
}

fn manager_body(
    ui: &mut Ui,
    dataset: &EngineeringDataset,
    state: &mut crate::workbench::app::EngineeringTableDialogState,
    personal: &[crate::state::SavedEngineeringTableView],
    project: &[crate::state::SavedEngineeringTableView],
) -> Option<BodyAction> {
    let tokens = Tokens::get(ui.ctx());
    let view = state.draft.as_mut()?;
    let projection = dataset.project(view);
    let mut select_saved = None;
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("Active grid");
            ui.monospace(&dataset.id);
            ui.small(format!(
                "{} logical rows · {}",
                dataset.rows.len(),
                if projection.virtualized {
                    "virtualized window"
                } else {
                    "complete render window"
                }
            ));
        });
        ui.add_space(24.0);
        ui.vertical(|ui| {
            ui.label("Saved view");
            let selected = state
                .selected_saved_id
                .as_deref()
                .and_then(|id| {
                    personal
                        .iter()
                        .chain(project)
                        .find(|view| view.id == id)
                        .map(|view| view.name.as_str())
                })
                .unwrap_or("Current working view");
            ComboBox::from_id_salt("engineering-table-saved")
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(state.selected_saved_id.is_none(), "Current working view")
                        .clicked()
                    {
                        state.selected_saved_id = None;
                    }
                    for saved in personal.iter().chain(project) {
                        if saved.definition.grid_id == dataset.id
                            && ui
                                .selectable_label(
                                    state.selected_saved_id.as_deref() == Some(&saved.id),
                                    format!("{} · {}", saved.name, saved.scope.label()),
                                )
                                .clicked()
                        {
                            state.selected_saved_id = Some(saved.id.clone());
                            select_saved =
                                Some(BodyAction::UseSaved(saved.id.clone(), saved.scope));
                        }
                    }
                });
        });
    });
    ui.separator();

    let mut move_column = None;
    let mut remove_sort = None;
    ui.columns(2, |columns| {
        columns[0].heading("Columns");
        if columns[0].button("Reset").clicked() {
            *view = EngineeringTableView::for_dataset(dataset);
        }
        ScrollArea::vertical()
            .id_salt("engineering-columns")
            .max_height(300.0)
            .show(&mut columns[0], |ui| {
                for index in 0..view.columns.len() {
                    let label = dataset
                        .columns
                        .iter()
                        .find(|column| column.id == view.columns[index].column_id)
                        .map(|column| column.label.as_str())
                        .unwrap_or("Removed column");
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut view.columns[index].visible, label);
                        ui.checkbox(&mut view.columns[index].pinned, "pinned");
                        ui.add(
                            egui::DragValue::new(&mut view.columns[index].width)
                                .range(56..=640)
                                .suffix(" px"),
                        );
                        if ui
                            .add_enabled(index > 0, egui::Button::new("↑"))
                            .on_hover_text("Move column up")
                            .clicked()
                        {
                            move_column = Some((index, index - 1));
                        }
                        if ui
                            .add_enabled(index + 1 < view.columns.len(), egui::Button::new("↓"))
                            .on_hover_text("Move column down")
                            .clicked()
                        {
                            move_column = Some((index, index + 1));
                        }
                    });
                }
            });

        columns[1].heading("Sort, filters, and scale");
        for index in 0..view.sort.len() {
            columns[1].horizontal(|ui| {
                ui.monospace(format!("{}", index + 1));
                ComboBox::from_id_salt(("sort-column", index))
                    .selected_text(
                        dataset
                            .columns
                            .iter()
                            .find(|column| column.id == view.sort[index].column_id)
                            .map(|column| column.label.as_str())
                            .unwrap_or("Column"),
                    )
                    .show_ui(ui, |ui| {
                        for column in &dataset.columns {
                            ui.selectable_value(
                                &mut view.sort[index].column_id,
                                column.id.clone(),
                                &column.label,
                            );
                        }
                    });
                ComboBox::from_id_salt(("sort-direction", index))
                    .selected_text(view.sort[index].direction.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut view.sort[index].direction,
                            SortDirection::Ascending,
                            "ascending",
                        );
                        ui.selectable_value(
                            &mut view.sort[index].direction,
                            SortDirection::Descending,
                            "descending",
                        );
                    });
                if ui.button("Remove").clicked() {
                    remove_sort = Some(index);
                }
            });
        }
        if view.sort.len() < dataset.columns.len()
            && columns[1].button("Add sort column").clicked()
            && let Some(column) = dataset
                .columns
                .iter()
                .find(|column| !view.sort.iter().any(|rule| rule.column_id == column.id))
        {
            view.sort.push(EngineeringSortRule {
                column_id: column.id.clone(),
                direction: SortDirection::Ascending,
            });
        }
        columns[1].checkbox(&mut view.show_filter_row, "Show typed filter row");
        ComboBox::from_label("Filter grammar")
            .selected_text(view.filter_grammar.label())
            .show_ui(&mut columns[1], |ui| {
                for grammar in EngineeringFilterGrammar::ALL {
                    ui.selectable_value(&mut view.filter_grammar, grammar, grammar.label());
                }
            });
        columns[1].small(match view.filter_grammar {
            EngineeringFilterGrammar::EngineeringValues => {
                "Comparisons and ranges accept engineering suffixes, for example >= 10k or 1p..5p."
            }
            EngineeringFilterGrammar::TextMatching => {
                "Contains by default; =value is exact and /pattern/ is a regular expression."
            }
        });
        if view.show_filter_row {
            Grid::new("engineering-filter-grid")
                .num_columns(2)
                .show(&mut columns[1], |ui| {
                    for column in &dataset.columns {
                        ui.label(&column.label);
                        ui.add(
                            TextEdit::singleline(
                                view.filters.entry(column.id.clone()).or_default(),
                            )
                            .hint_text("Filter…")
                            .desired_width(180.0),
                        );
                        ui.end_row();
                    }
                });
        }
        ComboBox::from_label("Large-table rendering")
            .selected_text(view.virtualization.label())
            .show_ui(&mut columns[1], |ui| {
                for policy in EngineeringVirtualizationPolicy::ALL {
                    ui.selectable_value(&mut view.virtualization, policy, policy.label());
                }
            });
        columns[1].small("Overscan 24 rows so keyboard paging never exposes an unrendered gap.");
        let previous_frozen = view.frozen_identifiers;
        ComboBox::from_label("Frozen identifiers")
            .selected_text(view.frozen_identifiers.label())
            .show_ui(&mut columns[1], |ui| {
                for policy in FrozenIdentifierPolicy::ALL {
                    ui.selectable_value(&mut view.frozen_identifiers, policy, policy.label());
                }
            });
        if view.frozen_identifiers != previous_frozen {
            for column in &mut view.columns {
                column.pinned = false;
            }
            let count = match view.frozen_identifiers {
                FrozenIdentifierPolicy::FirstVisibleIdentifier => 1,
                FrozenIdentifierPolicy::TwoLeftColumns => 2,
                FrozenIdentifierPolicy::None => 0,
            };
            for column in view
                .columns
                .iter_mut()
                .filter(|column| column.visible)
                .take(count)
            {
                column.pinned = true;
            }
        }
    });
    if let Some((from, to)) = move_column {
        let column = view.columns.remove(from);
        view.columns.insert(to, column);
    }
    if let Some(index) = remove_sort {
        view.sort.remove(index);
    }

    ui.separator();
    ui.horizontal_wrapped(|ui| {
        ui.strong("Keyboard");
        for contract in [
            "Arrow keys move cells",
            "Home/End move within row",
            "Ctrl+Home/End move table bounds",
            "Shift+Space selects row",
            "Enter opens details",
            "Ctrl+C copies selection",
        ] {
            ui.label(contract);
        }
    });
    ui.separator();
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
    ui.separator();
    let mut action = None;
    ui.horizontal_wrapped(|ui| {
        if ui.button("Export current or full dataset…").clicked() {
            action = Some(BodyAction::Export);
        }
        if ui.button("Copy with headers + units").clicked() {
            action = Some(BodyAction::Copy);
        }
        if ui.button("Save view…").clicked() {
            action = Some(BodyAction::Save);
        }
        if ui.button("Manage saved views…").clicked() {
            action = Some(BodyAction::Manage);
        }
        ui.label(
            RichText::new("Resize and reorder columns here; the exact projection is used for copy and export.")
                .color(tokens.color.text_dim),
        );
    });
    select_saved.or(action)
}

fn engineering_grid_preview(
    ui: &mut Ui,
    view: &EngineeringTableView,
    projection: &crate::state::engineering_table::EngineeringProjection,
    selected_rows: &mut std::collections::BTreeSet<String>,
    active_cell: &mut Option<(usize, usize)>,
    focus_cell: &mut Option<(usize, usize)>,
) -> Option<BodyAction> {
    ui.heading("Current engineering-table projection");
    if projection.rows.is_empty() {
        ui.label("No rows match the current typed filters.");
        return None;
    }

    const SELECT_WIDTH: f32 = 64.0;
    const CELL_HEIGHT: f32 = 24.0;
    const ROW_STRIDE: f32 = 28.0;
    let column_widths = projection
        .columns
        .iter()
        .map(|column| {
            view.columns
                .iter()
                .find(|candidate| candidate.column_id == column.id)
                .map_or(120.0, |candidate| f32::from(candidate.width))
        })
        .collect::<Vec<_>>();
    let table_width =
        SELECT_WIDTH + column_widths.iter().sum::<f32>() + 8.0 * projection.columns.len() as f32;
    let mut action = None;
    ScrollArea::horizontal()
        .id_salt("engineering-table-preview-horizontal")
        .auto_shrink([false, false])
        .max_height(240.0)
        .show(ui, |ui| {
            ui.set_min_width(table_width);
            Grid::new("engineering-table-preview-grid")
                .striped(false)
                .min_col_width(0.0)
                .spacing(Vec2::new(8.0, 4.0))
                .show(ui, |ui| {
                    ui.add_sized(
                        [SELECT_WIDTH, CELL_HEIGHT],
                        egui::Label::new(RichText::new("Selected").strong()),
                    );
                    for (column, width) in projection.columns.iter().zip(&column_widths) {
                        let heading = column.unit.as_ref().map_or_else(
                            || column.label.clone(),
                            |unit| format!("{} [{}]", column.label, unit),
                        );
                        ui.add_sized(
                            [*width, CELL_HEIGHT],
                            egui::Label::new(RichText::new(&heading).strong()).truncate(),
                        )
                        .on_hover_text(heading);
                    }
                    ui.end_row();
                });
            ScrollArea::vertical()
                .id_salt("engineering-table-preview-vertical")
                .max_height(196.0)
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
                                        Vec2::new(SELECT_WIDTH, CELL_HEIGHT),
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| ui.checkbox(&mut selected, ""),
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
                                    let selected_cell =
                                        *active_cell == Some((row_index, column_index));
                                    let text = row
                                        .cells
                                        .get(&column.id)
                                        .map(|cell| cell.display.as_str())
                                        .unwrap_or_default();
                                    let response = ui
                                        .add_sized(
                                            [*width, CELL_HEIGHT],
                                            egui::Button::selectable(selected_cell, text),
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
                                                    (column_index + 1).min(
                                                        projection.columns.len().saturating_sub(1),
                                                    ),
                                                ))
                                            } else if input.key_pressed(egui::Key::ArrowUp) {
                                                Some((row_index.saturating_sub(1), column_index))
                                            } else if input.key_pressed(egui::Key::ArrowDown) {
                                                Some((
                                                    (row_index + 1).min(
                                                        projection.rows.len().saturating_sub(1),
                                                    ),
                                                    column_index,
                                                ))
                                            } else if input.key_pressed(egui::Key::Home) {
                                                Some((
                                                    if modifiers.ctrl { 0 } else { row_index },
                                                    0,
                                                ))
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
                                        {
                                            if !selected_rows.insert(row.stable_id.clone()) {
                                                selected_rows.remove(&row.stable_id);
                                            }
                                        }
                                        if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                                            action = Some(BodyAction::OpenDetails(
                                                row.stable_id.clone(),
                                            ));
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
        });
    if projection.virtualized {
        ui.small(format!(
            "{} matching rows · viewport rendering with overscan {}",
            projection.rows.len(),
            crate::state::engineering_table::VIRTUALIZATION_OVERSCAN
        ));
    }
    action
}

fn save_view_body(
    ui: &mut Ui,
    state: &mut crate::workbench::app::EngineeringTableDialogState,
) -> Option<BodyAction> {
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
    state: &mut crate::workbench::app::EngineeringTableDialogState,
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
                ui.colored_label(Tokens::get(ui.ctx()).color.ok, "default");
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
    state: &mut crate::workbench::app::EngineeringTableDialogState,
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
    state: &mut crate::workbench::app::EngineeringTableDialogState,
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
        "Source revision {} · cross-probe selects the exact authored schematic object without modifying it.",
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
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        let revision = state.schematic.topology_version();

        assert!(open_engineering_table_dialog(&mut state));

        assert!(state.dialogs.engineering_table.open);
        assert_eq!(state.dialogs.engineering_table.source_revision, revision);
        assert_eq!(
            state
                .dialogs
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
        app.state.project_lifecycle.project_open = true;
        let dataset = EngineeringDataset::active_schematic(&app.state.schematic);
        app.state.dialogs.engineering_table.open(
            EngineeringTableView::for_dataset(&dataset),
            dataset.source_revision,
        );
        app.state.dialogs.engineering_table.save_name = "Personal review".to_owned();
        app.state.dialogs.engineering_table.save_scope = EngineeringViewScope::Personal;
        app.save_engineering_table_view(&dataset);
        assert!(!app.state.workspace.project_metadata_dirty);

        app.state.dialogs.engineering_table.save_name = "Project review".to_owned();
        app.state.dialogs.engineering_table.save_scope = EngineeringViewScope::Project;
        app.save_engineering_table_view(&dataset);
        assert!(app.state.workspace.project_metadata_dirty);
    }
}
