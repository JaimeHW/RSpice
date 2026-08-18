//! Mockup-owned testbench configuration-set manager and subordinate workflows.

use egui::{Align, Context, Frame, Layout, Margin, TextEdit, Ui, vec2};
use sha2::{Digest as _, Sha256};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    CellViewRef, ConfigurationBlackBoxPolicy, ConfigurationCloneScope, ConfigurationModelProfile,
    ConfigurationPlatform, ConfigurationSetCatalog, ConfigurationSetDefinition, ConfigurationSetId,
    ConfigurationSetOverride, InstancePath, InstancePathPattern, UnresolvedBindingPolicy, ViewType,
};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::design_system::{
    WorkbenchIcon, property_row, property_row_toned, section_header,
};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::review_primitives::{
    input_field, purpose_line, read_only_field,
};
use crate::workbench::app_state::AppState;

const MANAGER_TITLE: &str = "Testbench configuration sets";
const MANAGER_EYEBROW: &str = "DESIGN · HIERARCHY · VIEW BINDING";
const MANAGER_DESCRIPTION: &str = "Own exact testbench roots, DUT paths, ordered executable views, model profiles, and scoped hierarchy overrides.";
const MANAGER_PRIMARY: &str = "Save configuration set";
const NEW_TITLE: &str = "Create configuration set";
const NEW_EYEBROW: &str = "DESIGN · HIERARCHY BINDING";
const NEW_DESCRIPTION: &str = "Create a stable configuration for cell/view resolution, model sections, environment, and netlisting.";
const CLONE_TITLE: &str = "Clone configuration set";
const CLONE_EYEBROW: &str = "DESIGN · CONFIGURATION LINEAGE";
const CLONE_DESCRIPTION: &str = "Clone view-search, stop-view, model, environment, and netlisting policy with explicit lineage.";
const BINDING_TITLE: &str = "Hierarchy binding table";
const BINDING_EYEBROW: &str = "DESIGN · CONFIGURATION RESOLUTION";
const BINDING_DESCRIPTION: &str = "Edit ordered view search, explicit materialized stop boundaries, model sections, and platform eligibility by hierarchy pattern.";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ConfigurationDialogPage {
    #[default]
    Manager,
    New,
    Clone,
    Binding,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ConfigurationTemplate {
    #[default]
    AnalogSchematic,
    PostLayoutExtracted,
    MixedSignal,
    Rf,
}

impl ConfigurationTemplate {
    const ALL: [Self; 4] = [
        Self::AnalogSchematic,
        Self::PostLayoutExtracted,
        Self::MixedSignal,
        Self::Rf,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::AnalogSchematic => "Analog schematic",
            Self::PostLayoutExtracted => "Post-layout extracted",
            Self::MixedSignal => "Mixed-signal",
            Self::Rf => "RF",
        }
    }

    fn policy(self) -> (Vec<String>, Vec<String>) {
        let views: &[&str] = match self {
            Self::AnalogSchematic => &["schematic", "extracted", "spice"],
            Self::PostLayoutExtracted => &["extracted", "schematic", "spice"],
            Self::MixedSignal => &["schematic", "veriloga", "spice"],
            Self::Rf => &["schematic", "spice"],
        };
        let stops: &[&str] = match self {
            Self::AnalogSchematic | Self::PostLayoutExtracted => &["extracted", "spice"],
            Self::MixedSignal => &["veriloga", "spice"],
            Self::Rf => &["spice"],
        };
        (
            views.iter().map(|value| (*value).to_owned()).collect(),
            stops.iter().map(|value| (*value).to_owned()).collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ConfigurationSetsDialogState {
    pub(crate) open: bool,
    page: ConfigurationDialogPage,
    query: String,
    selected_id: Option<ConfigurationSetId>,
    draft: Option<ConfigurationSetDefinition>,
    original_revision: Option<u64>,
    new_name: String,
    new_root: Option<CellViewRef>,
    new_template: ConfigurationTemplate,
    clone_source: Option<ConfigurationSetId>,
    clone_scope: ConfigurationCloneScope,
    error: Option<String>,
    discard_confirmation: bool,
    receipt_cache: Option<ConfigurationReceiptCache>,
}

impl Default for ConfigurationSetsDialogState {
    fn default() -> Self {
        Self {
            open: false,
            page: ConfigurationDialogPage::Manager,
            query: String::new(),
            selected_id: None,
            draft: None,
            original_revision: None,
            new_name: String::new(),
            new_root: None,
            new_template: ConfigurationTemplate::AnalogSchematic,
            clone_source: None,
            clone_scope: ConfigurationCloneScope::AllBindings,
            error: None,
            discard_confirmation: false,
            receipt_cache: None,
        }
    }
}

#[derive(Debug, Clone)]
struct ConfigurationReceiptCache {
    key: String,
    value: ConfigurationReceiptView,
}

impl ConfigurationSetsDialogState {
    fn open(&mut self, catalog: &ConfigurationSetCatalog, root: CellViewRef) {
        *self = Self {
            open: true,
            selected_id: catalog.active_configuration_id().or_else(|| {
                catalog
                    .configurations()
                    .first()
                    .map(|configuration| configuration.id())
            }),
            new_root: Some(root),
            ..Self::default()
        };
        self.load_selected(catalog);
    }

    fn load_selected(&mut self, catalog: &ConfigurationSetCatalog) {
        let selected = self.selected_id.and_then(|id| catalog.find(id));
        self.draft = selected.map(|configuration| configuration.definition().clone());
        self.original_revision = selected.map(|configuration| configuration.revision());
        self.error = None;
        self.discard_confirmation = false;
    }

    fn dirty(&self, catalog: &ConfigurationSetCatalog) -> bool {
        self.selected_id
            .and_then(|id| catalog.find(id))
            .zip(self.draft.as_ref())
            .is_some_and(|(configuration, draft)| configuration.definition() != draft)
    }

    fn close(&mut self) {
        *self = Self::default();
    }
}

pub(crate) fn open_configuration_sets_dialog(state: &mut AppState) {
    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::warning(
            "Configuration sets require an open project.".to_owned(),
        ));
        return;
    }
    let catalog = state.workspace.configuration_sets.clone();
    let root = state.workspace.simulation_root_reference();
    state.dialogs.configuration_sets.open(&catalog, root);
}

pub(crate) fn open_configuration_binding_dialog(state: &mut AppState) {
    open_configuration_sets_dialog(state);
    if !state.dialogs.configuration_sets.open {
        return;
    }
    if state.dialogs.configuration_sets.draft.is_some() {
        state.dialogs.configuration_sets.page = ConfigurationDialogPage::Binding;
    } else {
        state.dialogs.configuration_sets.error =
            Some("Create a configuration set before editing hierarchy bindings.".to_owned());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BodyAction {
    None,
    Select(ConfigurationSetId),
    New,
    Clone,
    Binding,
    AddOverride,
    RemoveOverride(usize),
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_configuration_sets_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.configuration_sets.open {
            return;
        }
        let page = self.state.dialogs.configuration_sets.page;
        let dirty = self
            .state
            .dialogs
            .configuration_sets
            .dirty(&self.state.workspace.configuration_sets);
        let (eyebrow, title, primary, description) = match page {
            ConfigurationDialogPage::Manager => (
                MANAGER_EYEBROW,
                MANAGER_TITLE,
                MANAGER_PRIMARY,
                MANAGER_DESCRIPTION,
            ),
            ConfigurationDialogPage::New => (
                NEW_EYEBROW,
                NEW_TITLE,
                "Create configuration",
                NEW_DESCRIPTION,
            ),
            ConfigurationDialogPage::Clone => (
                CLONE_EYEBROW,
                CLONE_TITLE,
                "Create configuration",
                CLONE_DESCRIPTION,
            ),
            ConfigurationDialogPage::Binding => (
                BINDING_EYEBROW,
                BINDING_TITLE,
                "Apply reviewed bindings",
                BINDING_DESCRIPTION,
            ),
        };
        let write_allowed = !self.state.workbench.safe_mode.project_read_only();
        let primary_enabled = write_allowed
            && match page {
                ConfigurationDialogPage::Manager => {
                    dirty
                        || self.state.dialogs.configuration_sets.selected_id
                            != self
                                .state
                                .workspace
                                .configuration_sets
                                .active_configuration_id()
                }
                ConfigurationDialogPage::New => {
                    !self
                        .state
                        .dialogs
                        .configuration_sets
                        .new_name
                        .trim()
                        .is_empty()
                        && self
                            .state
                            .dialogs
                            .configuration_sets
                            .new_root
                            .as_ref()
                            .and_then(|root| {
                                default_dut_path_for_root(
                                    &self.state.workspace,
                                    &self.state.schematic,
                                    root,
                                )
                            })
                            .is_some()
                }
                ConfigurationDialogPage::Clone => {
                    !self
                        .state
                        .dialogs
                        .configuration_sets
                        .new_name
                        .trim()
                        .is_empty()
                        && self.state.dialogs.configuration_sets.clone_source.is_some()
                }
                ConfigurationDialogPage::Binding => {
                    dirty
                        || self.state.dialogs.configuration_sets.selected_id
                            != self
                                .state
                                .workspace
                                .configuration_sets
                                .active_configuration_id()
                }
            };
        let discard = page == ConfigurationDialogPage::Manager
            && self.state.dialogs.configuration_sets.discard_confirmation;
        let transaction_error = self.state.dialogs.configuration_sets.error.clone();
        let mut dialog = Dialog::new(eyebrow, title, primary)
            .description(description)
            .size(DialogSize::WideWorkflow)
            .initial_height(620.0)
            .ghost(if discard {
                "Discard changes"
            } else if page == ConfigurationDialogPage::Manager {
                "Close"
            } else {
                "Cancel"
            })
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if page == ConfigurationDialogPage::Manager {
            dialog = dialog.flush_body();
        }
        if discard {
            dialog = dialog.transaction_state(
                crate::ui::widgets::DialogTransactionTone::Error,
                "Discard unsaved configuration edits?",
                "The persisted configuration remains unchanged. This cannot affect retained run evidence.",
            );
        } else if let Some(error) = transaction_error.as_deref() {
            dialog = dialog.transaction_state(
                crate::ui::widgets::DialogTransactionTone::Error,
                "Configuration transaction cannot continue",
                error,
            );
        }

        let mut action = BodyAction::None;
        let choice = dialog.show(ctx, |ui| {
            action = configuration_body(
                ui,
                &mut self.state.dialogs.configuration_sets,
                &self.state.workspace,
                &self.state.library_manager,
                &self.state.model_library_manager,
                &self.state.schematic,
                write_allowed,
            );
        });
        self.handle_configuration_body_action(action);
        match choice {
            DialogChoice::Primary => self.commit_configuration_dialog_page(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if page != ConfigurationDialogPage::Manager {
                    self.state.dialogs.configuration_sets.page = ConfigurationDialogPage::Manager;
                    self.state.dialogs.configuration_sets.error = None;
                    self.state.dialogs.configuration_sets.discard_confirmation = false;
                    if page != ConfigurationDialogPage::Binding {
                        self.state
                            .dialogs
                            .configuration_sets
                            .load_selected(&self.state.workspace.configuration_sets);
                    }
                } else if dirty && !discard {
                    self.state.dialogs.configuration_sets.discard_confirmation = true;
                } else {
                    self.state.dialogs.configuration_sets.close();
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_configuration_body_action(&mut self, action: BodyAction) {
        match action {
            BodyAction::None => {}
            BodyAction::Select(id) => {
                if self.state.dialogs.configuration_sets.selected_id == Some(id) {
                    return;
                }
                if self
                    .state
                    .dialogs
                    .configuration_sets
                    .dirty(&self.state.workspace.configuration_sets)
                    && self.state.dialogs.configuration_sets.selected_id != Some(id)
                {
                    self.state.dialogs.configuration_sets.error = Some(
                        "Save or discard the current configuration edits before selecting another configuration."
                            .to_owned(),
                    );
                    return;
                }
                self.state.dialogs.configuration_sets.selected_id = Some(id);
                self.state
                    .dialogs
                    .configuration_sets
                    .load_selected(&self.state.workspace.configuration_sets);
            }
            BodyAction::New => {
                if self
                    .state
                    .dialogs
                    .configuration_sets
                    .dirty(&self.state.workspace.configuration_sets)
                {
                    self.state.dialogs.configuration_sets.error = Some(
                        "Save or discard the current configuration edits before creating another configuration."
                            .to_owned(),
                    );
                    return;
                }
                let default_name = unique_configuration_name(
                    &self.state.workspace.configuration_sets,
                    "Lab characterization",
                );
                let dialog = &mut self.state.dialogs.configuration_sets;
                dialog.page = ConfigurationDialogPage::New;
                dialog.new_name = default_name;
                dialog.new_root = Some(self.state.workspace.simulation_root_reference());
                dialog.new_template = ConfigurationTemplate::AnalogSchematic;
                dialog.error = None;
            }
            BodyAction::Clone => {
                if self
                    .state
                    .dialogs
                    .configuration_sets
                    .dirty(&self.state.workspace.configuration_sets)
                {
                    self.state.dialogs.configuration_sets.error = Some(
                        "Save or discard the current configuration edits before cloning a committed revision."
                            .to_owned(),
                    );
                    return;
                }
                let dialog = &mut self.state.dialogs.configuration_sets;
                dialog.page = ConfigurationDialogPage::Clone;
                dialog.clone_source = dialog.selected_id;
                dialog.clone_scope = ConfigurationCloneScope::AllBindings;
                let stem = dialog
                    .selected_id
                    .and_then(|id| self.state.workspace.configuration_sets.find(id))
                    .map_or("Configuration copy", |configuration| configuration.name());
                dialog.new_name = unique_configuration_name(
                    &self.state.workspace.configuration_sets,
                    &format!("{stem} copy"),
                );
                dialog.error = None;
            }
            BodyAction::Binding => {
                self.state.dialogs.configuration_sets.page = ConfigurationDialogPage::Binding;
                self.state.dialogs.configuration_sets.error = None;
            }
            BodyAction::AddOverride => {
                if let Some(draft) = self.state.dialogs.configuration_sets.draft.as_mut() {
                    let next = draft.overrides.len() + 1;
                    draft.overrides.push(ConfigurationSetOverride {
                        instance_path: new_override_scope(next),
                        executable_views: draft.executable_view_policy.clone(),
                        stop_view: draft.stop_views.first().cloned(),
                        model_section: None,
                        eligible_platforms: vec![ConfigurationPlatform::current()],
                    });
                }
            }
            BodyAction::RemoveOverride(index) => {
                if let Some(draft) = self.state.dialogs.configuration_sets.draft.as_mut()
                    && index < draft.overrides.len()
                {
                    draft.overrides.remove(index);
                }
            }
        }
    }

    fn commit_configuration_dialog_page(&mut self) {
        let page = self.state.dialogs.configuration_sets.page;
        let result = match page {
            ConfigurationDialogPage::Manager | ConfigurationDialogPage::Binding => {
                self.commit_configuration_update()
            }
            ConfigurationDialogPage::New => self.commit_new_configuration(),
            ConfigurationDialogPage::Clone => self.commit_cloned_configuration(),
        };
        if let Err(error) = result {
            self.state.dialogs.configuration_sets.error = Some(error);
        }
    }

    fn publish_configuration_catalog(
        &mut self,
        candidate: ConfigurationSetCatalog,
        selected: ConfigurationSetId,
        message: String,
    ) -> Result<(), String> {
        let mut projected = self.state.workspace.clone();
        projected.configuration_sets = candidate.clone();
        let projection = projected
            .configuration_execution_projection(
                &self.state.library_manager,
                &self.state.workspace.active_view,
                &self.state.schematic,
            )
            .map_err(|error| format!("Configuration cannot be published: {error}"))?;
        let root = projection.root_schematic().ok_or_else(|| {
            "Configuration cannot be published without its root schematic.".to_owned()
        })?;
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
            &self.state.library_manager,
            &projection,
        );
        let generated =
            crate::simulation::netlist_gen::generate_netlist_hierarchical(root, &[], &hierarchy);
        if !generated.errors.is_empty() {
            return Err(format!(
                "Configuration cannot be published because exact netlist generation failed: {}",
                generated.errors.join("; ")
            ));
        }
        let generated = projected.bind_generated_netlist_provenance(generated.netlist);
        crate::simulation::controller::prepared_run::expand_generated_dependencies(
            &generated,
            root.current_file.as_deref(),
            &self.state.model_library_manager,
        )
        .map_err(|error| {
            format!("Configuration cannot be published because source sealing failed: {error}")
        })?;
        if candidate == self.state.workspace.configuration_sets {
            let dialog = &mut self.state.dialogs.configuration_sets;
            dialog.page = ConfigurationDialogPage::Manager;
            dialog.selected_id = Some(selected);
            dialog.load_selected(&self.state.workspace.configuration_sets);
            return Ok(());
        }
        self.state
            .workspace
            .replace_configuration_sets(candidate)
            .map_err(|error| error.to_string())?;
        self.invalidate_simulation_preflight();
        self.state.ui.netlist.current_generation_input_digest = None;
        self.state.push_user_message(ConsoleMessage::info(message));
        let dialog = &mut self.state.dialogs.configuration_sets;
        dialog.page = ConfigurationDialogPage::Manager;
        dialog.selected_id = Some(selected);
        dialog.load_selected(&self.state.workspace.configuration_sets);
        Ok(())
    }

    fn commit_configuration_update(&mut self) -> Result<(), String> {
        let dialog = &self.state.dialogs.configuration_sets;
        let id = dialog
            .selected_id
            .ok_or_else(|| "Select a configuration before saving.".to_owned())?;
        let revision = dialog
            .original_revision
            .ok_or_else(|| "The selected configuration has no revision authority.".to_owned())?;
        let definition = dialog
            .draft
            .clone()
            .ok_or_else(|| "The selected configuration draft is unavailable.".to_owned())?;
        let mut candidate = self.state.workspace.configuration_sets.clone();
        let committed = if candidate
            .find(id)
            .is_some_and(|configuration| configuration.definition() == &definition)
        {
            revision
        } else {
            match candidate.update(id, revision, definition) {
                Ok(committed) => committed,
                Err(crate::state::ConfigurationSetError::NoChanges(_)) => revision,
                Err(error) => return Err(error.to_string()),
            }
        };
        candidate.activate(id).map_err(|error| error.to_string())?;
        self.publish_configuration_catalog(
            candidate,
            id,
            format!("Saved configuration set {id} at revision {committed}."),
        )
    }

    fn commit_new_configuration(&mut self) -> Result<(), String> {
        let dialog = &self.state.dialogs.configuration_sets;
        let root = dialog
            .new_root
            .clone()
            .ok_or_else(|| "Select a root testbench.".to_owned())?;
        let definition = definition_from_template(
            dialog.new_name.clone(),
            root,
            dialog.new_template,
            &self.state.workspace,
            &self.state.schematic,
        )?;
        let mut candidate = self.state.workspace.configuration_sets.clone();
        let id = candidate
            .create(definition)
            .map_err(|error| error.to_string())?;
        candidate.activate(id).map_err(|error| error.to_string())?;
        self.publish_configuration_catalog(
            candidate,
            id,
            format!("Created and activated configuration set {id}."),
        )
    }

    fn commit_cloned_configuration(&mut self) -> Result<(), String> {
        let dialog = &self.state.dialogs.configuration_sets;
        let source_id = dialog
            .clone_source
            .ok_or_else(|| "Select a source configuration.".to_owned())?;
        let source = self
            .state
            .workspace
            .configuration_sets
            .find(source_id)
            .ok_or_else(|| "The selected source configuration no longer exists.".to_owned())?;
        let defaults = definition_from_template(
            dialog.new_name.clone(),
            source.root().clone(),
            ConfigurationTemplate::AnalogSchematic,
            &self.state.workspace,
            &self.state.schematic,
        )?;
        let mut candidate = self.state.workspace.configuration_sets.clone();
        let id = candidate
            .clone_configuration_scoped(
                source_id,
                source.revision(),
                dialog.new_name.clone(),
                dialog.clone_scope,
                defaults,
            )
            .map_err(|error| error.to_string())?;
        candidate.activate(id).map_err(|error| error.to_string())?;
        self.publish_configuration_catalog(
            candidate,
            id,
            format!("Cloned configuration {source_id} into active configuration {id}."),
        )
    }
}

fn configuration_body(
    ui: &mut Ui,
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    model_libraries: &crate::state::ModelLibraryManager,
    active_schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> BodyAction {
    let description = match dialog.page {
        ConfigurationDialogPage::Manager => MANAGER_DESCRIPTION,
        ConfigurationDialogPage::New => NEW_DESCRIPTION,
        ConfigurationDialogPage::Clone => CLONE_DESCRIPTION,
        ConfigurationDialogPage::Binding => BINDING_DESCRIPTION,
    };
    purpose_line(ui, description);
    match dialog.page {
        ConfigurationDialogPage::Manager => manager_body(
            ui,
            dialog,
            workspace,
            libraries,
            model_libraries,
            active_schematic,
            write_allowed,
        ),
        ConfigurationDialogPage::New => {
            new_configuration_body(
                ui,
                dialog,
                workspace,
                libraries,
                active_schematic,
                write_allowed,
            );
            BodyAction::None
        }
        ConfigurationDialogPage::Clone => {
            clone_configuration_body(ui, dialog, workspace, write_allowed);
            BodyAction::None
        }
        ConfigurationDialogPage::Binding => binding_body(
            ui,
            dialog,
            workspace,
            libraries,
            model_libraries,
            active_schematic,
            write_allowed,
        ),
    }
}

fn manager_body(
    ui: &mut Ui,
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    model_libraries: &crate::state::ModelLibraryManager,
    active_schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> BodyAction {
    let mut action = BodyAction::None;
    let t = Tokens::get(ui.ctx());
    let dirty = dialog.dirty(&workspace.configuration_sets);
    let selected_receipt = selected_configuration_receipt(
        dialog,
        workspace,
        libraries,
        model_libraries,
        active_schematic,
    );
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let available = (ui.available_width() - 190.0).max(120.0);
                let response = ui.add_sized(
                    vec2(available, t.metrics.ctl_h),
                    TextEdit::singleline(&mut dialog.query)
                        .hint_text("Configuration, cell, view, or owner…")
                        .margin(Margin {
                            left: 29,
                            right: 8,
                            top: 5,
                            bottom: 5,
                        }),
                );
                WorkbenchIcon::Search.paint(
                    ui.painter(),
                    egui::Rect::from_center_size(
                        egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                        vec2(16.0, 16.0),
                    ),
                    t.color.text_faint,
                );
                if Button::new("Clone")
                    .enabled(dialog.selected_id.is_some() && write_allowed && !dirty)
                    .show(ui)
                    .clicked()
                {
                    action = BodyAction::Clone;
                }
                if Button::new("New configuration")
                    .accent()
                    .enabled(write_allowed && !dirty)
                    .show(ui)
                    .clicked()
                {
                    action = BodyAction::New;
                }
            });
        });

    configuration_table_header(ui);
    let query = dialog.query.trim().to_lowercase();
    let mut visible = 0usize;
    for configuration in workspace.configuration_sets.configurations() {
        let definition = configuration.definition();
        let searchable = format!(
            "{} {} {} {} {}",
            definition.name,
            definition.root.key(),
            definition.dut_path,
            definition.executable_view_policy.join(" "),
            definition.owner
        )
        .to_lowercase();
        if !query.is_empty() && !searchable.contains(&query) {
            continue;
        }
        visible += 1;
        let selected = dialog.selected_id == Some(configuration.id());
        let status = selected_receipt.as_ref().filter(|_| selected).map_or_else(
            || {
                if workspace.configuration_sets.active_configuration_id()
                    == Some(configuration.id())
                {
                    "Active"
                } else {
                    "Stored"
                }
            },
            |receipt| receipt.status.as_str(),
        );
        if configuration_table_row(
            ui,
            configuration,
            selected.then_some(dialog.draft.as_ref()).flatten(),
            selected,
            status,
        )
        .clicked()
            && !selected
        {
            action = BodyAction::Select(configuration.id());
        }
    }
    if visible == 0 {
        empty_row(
            ui,
            if workspace.configuration_sets.configurations().is_empty() {
                "No configuration sets exist. Create the first exact testbench binding."
            } else {
                "No configuration matches the current filter."
            },
        );
    }

    if dialog.selected_id.is_some()
        && let Some(receipt) = selected_receipt.as_ref()
    {
        let stacked = ui.available_width() < 720.0;
        if stacked {
            configuration_ownership_form(
                ui,
                dialog,
                workspace,
                libraries,
                active_schematic,
                write_allowed,
            );
            binding_validation_panel(ui, dialog, receipt, &mut action);
        } else {
            Frame::NONE
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.columns(2, |columns| {
                        Frame::NONE
                            .inner_margin(Margin::same(10))
                            .show(&mut columns[0], |ui| {
                                configuration_ownership_form(
                                    ui,
                                    dialog,
                                    workspace,
                                    libraries,
                                    active_schematic,
                                    write_allowed,
                                );
                            });
                        Frame::NONE
                            .inner_margin(Margin::same(10))
                            .show(&mut columns[1], |ui| {
                                binding_validation_panel(ui, dialog, receipt, &mut action);
                            });
                        let divider = columns[1].min_rect();
                        columns[1].painter().vline(
                            divider.left(),
                            divider.y_range(),
                            egui::Stroke::new(1.0, t.color.border),
                        );
                    });
                });
        }
    }
    action
}

#[derive(Debug, Clone)]
struct ConfigurationReceiptView {
    resolved: String,
    overrides: usize,
    incompatible: usize,
    netlist_digest: String,
    status: String,
    diagnostic: Option<String>,
    bindings: Vec<crate::state::ResolvedHierarchyBinding>,
}

fn selected_configuration_receipt(
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    model_libraries: &crate::state::ModelLibraryManager,
    active_schematic: &crate::state::SchematicState,
) -> Option<ConfigurationReceiptView> {
    let id = dialog.selected_id?;
    let key = configuration_receipt_cache_key(
        workspace,
        libraries,
        model_libraries,
        active_schematic,
        id,
        dialog.draft.as_ref(),
    );
    if let Some(cached) = dialog.receipt_cache.as_ref()
        && cached.key == key
    {
        return Some(cached.value.clone());
    }
    let value = configuration_receipt(
        workspace,
        libraries,
        model_libraries,
        active_schematic,
        id,
        dialog.draft.as_ref(),
    );
    dialog.receipt_cache = Some(ConfigurationReceiptCache {
        key,
        value: value.clone(),
    });
    Some(value)
}

fn configuration_receipt_cache_key(
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    model_libraries: &crate::state::ModelLibraryManager,
    active_schematic: &crate::state::SchematicState,
    id: ConfigurationSetId,
    draft: Option<&ConfigurationSetDefinition>,
) -> String {
    let mut digest = Sha256::new();
    digest.update(id.to_string());
    if let Some(configuration) = workspace.configuration_sets.find(id) {
        digest.update(configuration.revision().to_le_bytes());
        digest.update(configuration.semantic_digest().to_string());
    }
    if let Some(draft) = draft
        && let Ok(bytes) = serde_json::to_vec(draft)
    {
        digest.update(bytes);
    }
    digest.update(workspace.active_view.key());
    digest.update(active_schematic.topology_version().to_le_bytes());
    digest.update(libraries.revision().to_le_bytes());
    if let Ok(bytes) = serde_json::to_vec(model_libraries) {
        digest.update(bytes);
    }
    let mut buffer_versions = workspace
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key.as_str(), schematic.topology_version()))
        .collect::<Vec<_>>();
    buffer_versions.sort_unstable_by(|left, right| left.0.cmp(right.0));
    for (key, version) in buffer_versions {
        digest.update(key);
        digest.update(version.to_le_bytes());
    }
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn configuration_receipt(
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    model_libraries: &crate::state::ModelLibraryManager,
    active_schematic: &crate::state::SchematicState,
    id: ConfigurationSetId,
    draft: Option<&ConfigurationSetDefinition>,
) -> ConfigurationReceiptView {
    let projected = match projected_configuration_workspace(workspace, id, draft) {
        Ok(projected) => projected,
        Err(error) => {
            return ConfigurationReceiptView {
                resolved: "0 / 0".to_owned(),
                overrides: draft.map_or(0, |value| value.overrides.len()),
                incompatible: 0,
                netlist_digest: "generation blocked".to_owned(),
                status: "generation blocked".to_owned(),
                diagnostic: Some(error),
                bindings: Vec::new(),
            };
        }
    };
    let resolution = projected.resolve_hierarchy_with_active(
        libraries,
        &workspace.active_view,
        active_schematic,
    );
    let configuration = projected.configuration_sets.find(id);
    let overrides = configuration.map_or(0, |value| value.overrides().len());
    let fallback = resolution
        .bindings
        .iter()
        .filter(|binding| binding.used_review_fallback)
        .count();
    let incompatible = resolution
        .bindings
        .iter()
        .filter(|binding| {
            binding.diagnostic.as_deref().is_some_and(|diagnostic| {
                diagnostic.contains("unavailable in this browser session")
                    || diagnostic.contains("not supported by this execution target")
            })
        })
        .map(|binding| binding.instance_count)
        .sum();
    let unresolved = resolution.unresolved_instances();
    let mut status = if unresolved > 0 {
        format!("{unresolved} unresolved")
    } else if fallback > 0 {
        format!("{fallback} fallback review")
    } else {
        format!(
            "{} / {} bound",
            resolution.resolved_instances, resolution.total_instances
        )
    };
    let (netlist_digest, diagnostic) = if resolution.is_valid() {
        match configuration_netlist_digest(&projected, libraries, active_schematic, model_libraries)
        {
            Ok(digest) => (digest, None),
            Err(error) => {
                status = "generation blocked".to_owned();
                ("generation blocked".to_owned(), Some(error))
            }
        }
    } else {
        (
            "generation blocked".to_owned(),
            resolution
                .bindings
                .iter()
                .find_map(|binding| binding.diagnostic.clone()),
        )
    };
    let bindings = resolution.bindings.clone();
    ConfigurationReceiptView {
        resolved: format!(
            "{} / {}",
            resolution.resolved_instances, resolution.total_instances
        ),
        overrides,
        incompatible,
        netlist_digest,
        status,
        diagnostic,
        bindings,
    }
}

fn projected_configuration_workspace(
    workspace: &crate::state::ProjectWorkspace,
    id: ConfigurationSetId,
    draft: Option<&ConfigurationSetDefinition>,
) -> Result<crate::state::ProjectWorkspace, String> {
    let mut projected = workspace.clone();
    if let Some(draft) = draft {
        let configuration = projected
            .configuration_sets
            .find(id)
            .ok_or_else(|| "configuration no longer exists".to_owned())?;
        if configuration.definition() != draft {
            let revision = configuration.revision();
            match projected
                .configuration_sets
                .update(id, revision, draft.clone())
            {
                Ok(_) | Err(crate::state::ConfigurationSetError::NoChanges(_)) => {}
                Err(error) => return Err(format!("invalid draft: {error}")),
            }
        }
    }
    projected
        .configuration_sets
        .activate(id)
        .map_err(|error| format!("invalid identity: {error}"))?;
    Ok(projected)
}

fn configuration_netlist_digest(
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    active_schematic: &crate::state::SchematicState,
    model_libraries: &crate::state::ModelLibraryManager,
) -> Result<String, String> {
    let projection = workspace
        .configuration_execution_projection(libraries, &workspace.active_view, active_schematic)
        .map_err(|error| error.to_string())?;
    let root = projection
        .root_schematic()
        .ok_or_else(|| "configuration root schematic is unavailable".to_owned())?;
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
        libraries,
        &projection,
    );
    let generated =
        crate::simulation::netlist_gen::generate_netlist_hierarchical(root, &[], &hierarchy);
    if !generated.errors.is_empty() {
        return Err(generated.errors.join("; "));
    }
    let source = workspace.bind_generated_netlist_provenance(generated.netlist);
    let (source, _) = crate::simulation::controller::prepared_run::expand_generated_dependencies(
        &source,
        root.current_file.as_deref(),
        model_libraries,
    )
    .map_err(|error| error.to_string())?;
    let digest = Sha256::digest(source.as_bytes());
    let text = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(format!("{}…{}", &text[..8], &text[text.len() - 4..]))
}

fn configuration_table_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let labels = [
        "Configuration",
        "Testbench",
        "DUT",
        "View policy",
        "Overrides",
        "Owner",
        "Status",
    ];
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 27.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
    paint_configuration_columns(
        ui,
        rect,
        &labels.map(str::to_uppercase),
        crate::ui::theme::sans(tokens::FS_0, crate::ui::theme::FontWeight::SemiBold),
        t.color.text_dim,
        t.color.text_dim,
    );
}

fn configuration_table_row(
    ui: &mut Ui,
    configuration: &crate::state::ConfigurationSet,
    draft: Option<&ConfigurationSetDefinition>,
    selected: bool,
    status: &str,
) -> egui::Response {
    let definition = draft.unwrap_or_else(|| configuration.definition());
    let values = [
        definition.name.clone(),
        definition.root.cell.clone(),
        definition.dut_path.clone(),
        definition.executable_view_policy.join(" → "),
        definition.overrides.len().to_string(),
        definition.owner.clone(),
        status.to_owned(),
    ];
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 29.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("Select configuration set {}", definition.name),
        )
    });
    let fill = if selected {
        t.color.accent_dim
    } else if response.hovered() {
        t.color.bg_elevated
    } else {
        t.color.bg_panel
    };
    ui.painter().rect_filled(rect, 0.0, fill);
    if selected {
        ui.painter().vline(
            rect.left() + 1.0,
            rect.y_range(),
            egui::Stroke::new(2.0, t.color.accent),
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let status_color = if status.contains("unresolved")
        || status.contains("invalid")
        || status.contains("blocked")
    {
        t.color.err
    } else if status.contains("fallback") || status.contains("Select") {
        t.color.warn
    } else if matches!(status, "Active" | "Stored") {
        t.color.text_dim
    } else {
        t.color.ok
    };
    paint_configuration_columns(
        ui,
        rect,
        &values,
        crate::ui::theme::sans(tokens::FS_0, crate::ui::theme::FontWeight::Regular),
        t.color.text,
        status_color,
    );
    crate::ui::theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(format!(
        "{} · {} · revision {} · {}",
        definition.root.display_path(),
        definition.dut_path,
        configuration.revision(),
        status
    ))
}

fn paint_configuration_columns(
    ui: &Ui,
    rect: egui::Rect,
    values: &[String; 7],
    font: egui::FontId,
    color: egui::Color32,
    status_color: egui::Color32,
) {
    const FRACTIONS: [f32; 7] = [0.18, 0.11, 0.13, 0.20, 0.08, 0.13, 0.17];
    let inner = rect.shrink2(vec2(8.0, 0.0));
    let mut left = inner.left();
    for (index, (value, fraction)) in values.iter().zip(FRACTIONS).enumerate() {
        let right = if index + 1 == values.len() {
            inner.right()
        } else {
            left + inner.width() * fraction
        };
        let cell = egui::Rect::from_min_max(
            egui::pos2(left, inner.top()),
            egui::pos2((right - 8.0).max(left), inner.bottom()),
        );
        ui.painter().with_clip_rect(cell).text(
            cell.left_center(),
            egui::Align2::LEFT_CENTER,
            value,
            font.clone(),
            if index + 1 == values.len() {
                status_color
            } else {
                color
            },
        );
        left = right;
    }
}

fn empty_row(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    ui.label(egui::RichText::new(message).color(t.color.text_dim));
    ui.add_space(12.0);
}

fn configuration_ownership_form(
    ui: &mut Ui,
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    active_schematic: &crate::state::SchematicState,
    write_allowed: bool,
) {
    section_header(ui, "Configuration ownership", None);
    let Some(draft) = dialog.draft.as_mut() else {
        empty_row(ui, "Select a configuration to inspect its ownership.");
        return;
    };
    ui.add_enabled_ui(write_allowed, |ui| {
        input_field(
            ui,
            "Name",
            &mut draft.name,
            "Configuration name",
            None,
            "Unique project-owned configuration name",
        );
        root_selector(
            ui,
            "Root testbench",
            &mut draft.root,
            workspace,
            libraries,
            active_schematic,
        );
        let dut_rejection = instance_path_rejection(&draft.dut_path);
        input_field(
            ui,
            "DUT path",
            &mut draft.dut_path,
            "/XAFE",
            dut_rejection.as_deref(),
            "Exact absolute instance path of the design under test",
        );
        read_only_field(
            ui,
            "Model profile",
            draft.model_profile.label(),
            "Model and environment profile bound by this configuration",
        );
    });
}

fn binding_validation_panel(
    ui: &mut Ui,
    dialog: &ConfigurationSetsDialogState,
    receipt: &ConfigurationReceiptView,
    action: &mut BodyAction,
) {
    section_header(ui, "Binding validation", None);
    property_row(ui, "Resolved instances", &receipt.resolved);
    property_row(ui, "Scoped overrides", &receipt.overrides.to_string());
    property_row(
        ui,
        "Platform-incompatible views",
        &receipt.incompatible.to_string(),
    );
    property_row(ui, "Generated netlist digest", &receipt.netlist_digest);
    if let Some(diagnostic) = receipt.diagnostic.as_deref() {
        property_row_toned(
            ui,
            "Blocking diagnostic",
            diagnostic,
            Tokens::get(ui.ctx()).color.err,
        );
    }
    ui.add_space(7.0);
    if Button::new("Open hierarchy binding table")
        .enabled(dialog.selected_id.is_some())
        .show(ui)
        .clicked()
    {
        *action = BodyAction::Binding;
    }
}

fn new_configuration_body(
    ui: &mut Ui,
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    active_schematic: &crate::state::SchematicState,
    write_allowed: bool,
) {
    section_header(ui, "Configuration identity", None);
    ui.add_enabled_ui(write_allowed, |ui| {
        input_field(
            ui,
            "Name",
            &mut dialog.new_name,
            "afe_lab_rf",
            None,
            "Unique project-owned configuration name",
        );
        if let Some(root) = dialog.new_root.as_mut() {
            root_selector(ui, "Root", root, workspace, libraries, active_schematic);
        } else if let Some(first) = executable_roots(workspace, libraries, active_schematic)
            .into_iter()
            .next()
        {
            dialog.new_root = Some(first);
        } else {
            read_only_field(
                ui,
                "Root",
                "No executable schematic/testbench views",
                "Create an executable root before this workflow can commit",
            );
        }
        enum_combo(
            ui,
            "Template",
            &mut dialog.new_template,
            ConfigurationTemplate::ALL,
            ConfigurationTemplate::label,
        );
    });
    section_header(ui, "Template contract", None);
    let (views, stops) = dialog.new_template.policy();
    property_row(ui, "Ordered view search", &views.join(" → "));
    property_row(ui, "Stop views", &stops.join(" · "));
    property_row(ui, "Unresolved cells", "Block netlist");
    property_row(ui, "Lineage", "New independent configuration");
    if let Some(root) = dialog.new_root.as_ref() {
        if let Some(dut) = default_dut_path_for_root(workspace, active_schematic, root) {
            property_row(ui, "Design under test", &dut);
        } else {
            property_row_toned(
                ui,
                "Design under test",
                "No hierarchical instance exists in the selected root",
                Tokens::get(ui.ctx()).color.err,
            );
        }
    }
}

fn clone_configuration_body(
    ui: &mut Ui,
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    write_allowed: bool,
) {
    {
        section_header(ui, "Configuration lineage", None);
        ui.add_enabled_ui(write_allowed, |ui| {
            input_field(
                ui,
                "New name",
                &mut dialog.new_name,
                "afe_lab_postlayout",
                None,
                "Unique name for the cloned configuration",
            );
            configuration_selector(
                ui,
                "Source",
                &mut dialog.clone_source,
                &workspace.configuration_sets,
            );
            enum_combo(
                ui,
                "Copy",
                &mut dialog.clone_scope,
                ConfigurationCloneScope::ALL,
                ConfigurationCloneScope::label,
            );
        });
        section_header(ui, "Copy contract", None);
        let detail = match dialog.clone_scope {
            ConfigurationCloneScope::AllBindings => {
                "Root, DUT, ordered views, stop views, overrides, model profile, and policy"
            }
            ConfigurationCloneScope::BindingsOnly => {
                "Root, DUT, ordered views, stop views, overrides, and unresolved policy"
            }
            ConfigurationCloneScope::EnvironmentOnly => {
                "Model profile into a fresh analog-schematic binding on the source root"
            }
        };
        property_row(ui, "Copied semantics", detail);
        let lineage = dialog
            .clone_source
            .and_then(|id| workspace.configuration_sets.find(id))
            .map_or_else(
                || "No source selected".to_owned(),
                |configuration| {
                    format!(
                        "{} · config-{}",
                        configuration.name(),
                        configuration.revision()
                    )
                },
            );
        property_row(ui, "Retained lineage", &lineage);
    }
}

fn binding_body(
    ui: &mut Ui,
    dialog: &mut ConfigurationSetsDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    model_libraries: &crate::state::ModelLibraryManager,
    active_schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> BodyAction {
    let mut action = BodyAction::None;
    {
        section_header(ui, "Binding authority", None);
        let previous_selection = dialog.selected_id;
        let mut requested_selection = previous_selection;
        configuration_selector(
            ui,
            "Configuration",
            &mut requested_selection,
            &workspace.configuration_sets,
        );
        if requested_selection != previous_selection
            && let Some(id) = requested_selection
        {
            action = BodyAction::Select(id);
            return action;
        }
        let receipt = selected_configuration_receipt(
            dialog,
            workspace,
            libraries,
            model_libraries,
            active_schematic,
        );
        if let Some(receipt) = receipt.as_ref() {
            property_row(ui, "Validation", &receipt.status);
        }
        let Some(draft) = dialog.draft.as_mut() else {
            empty_row(ui, "Select a configuration before editing bindings.");
            return action;
        };
        let library_views = project_view_names(libraries);
        let ordered_hint = view_name_hint(&library_views, "schematic, extracted, spice");
        let ordered_rejection = view_policy_rejection(&draft.executable_view_policy);
        let stops_rejection = view_policy_rejection(&draft.stop_views);
        ui.add_enabled_ui(write_allowed, |ui| {
            enum_combo(
                ui,
                "Unresolved policy",
                &mut draft.unresolved_policy,
                UnresolvedBindingPolicy::ALL,
                UnresolvedBindingPolicy::label,
            );
            comma_list_field(
                ui,
                "Ordered view search",
                &mut draft.executable_view_policy,
                &ordered_hint,
                ordered_rejection.as_deref(),
            );
            comma_list_field(
                ui,
                "Stop views",
                &mut draft.stop_views,
                &ordered_hint,
                stops_rejection.as_deref(),
            );
        });
        library_view_row(ui, &library_views);
        absent_view_row(
            ui,
            &library_views,
            [
                draft.executable_view_policy.as_slice(),
                draft.stop_views.as_slice(),
            ],
        );
        property_row(ui, "Black-box boundary", draft.black_box_policy.label());
        property_row(
            ui,
            "Platform authority",
            &format!(
                "{} current · scoped target allowlists",
                ConfigurationPlatform::current().label()
            ),
        );

        section_header(
            ui,
            "Scoped hierarchy overrides",
            Some(&format!("{} configured", draft.overrides.len())),
        );
        if draft.overrides.is_empty() {
            empty_row(
                ui,
                "No scoped overrides. Every instance uses the global ordered policy.",
            );
        }
        let compact = ui.available_width() < 760.0;
        for (index, scoped) in draft.overrides.iter_mut().enumerate() {
            Frame::NONE
                .stroke(egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border))
                .inner_margin(Margin::same(8))
                .show(ui, |ui| {
                    let scope_rejection = instance_pattern_rejection(&scoped.instance_path);
                    let views_rejection = view_policy_rejection(&scoped.executable_views);
                    let stop_rejection = scoped.stop_view.as_deref().and_then(view_name_rejection);
                    if compact {
                        input_field(
                            ui,
                            "Instance path or pattern",
                            &mut scoped.instance_path,
                            "/XAFE/*",
                            scope_rejection.as_deref(),
                            "Case-insensitive bounded hierarchy pattern",
                        );
                        comma_list_field(
                            ui,
                            "Executable views",
                            &mut scoped.executable_views,
                            &ordered_hint,
                            views_rejection.as_deref(),
                        );
                        optional_text_field(
                            ui,
                            "Stop view",
                            &mut scoped.stop_view,
                            "spice",
                            stop_rejection.as_deref(),
                        );
                        optional_text_field(
                            ui,
                            "Model section",
                            &mut scoped.model_section,
                            "tt",
                            None,
                        );
                    } else {
                        ui.columns(2, |columns| {
                            input_field(
                                &mut columns[0],
                                "Instance path or pattern",
                                &mut scoped.instance_path,
                                "/XAFE/*",
                                scope_rejection.as_deref(),
                                "Case-insensitive bounded hierarchy pattern",
                            );
                            comma_list_field(
                                &mut columns[1],
                                "Executable views",
                                &mut scoped.executable_views,
                                &ordered_hint,
                                views_rejection.as_deref(),
                            );
                        });
                        ui.columns(2, |columns| {
                            optional_text_field(
                                &mut columns[0],
                                "Stop view",
                                &mut scoped.stop_view,
                                "spice",
                                stop_rejection.as_deref(),
                            );
                            optional_text_field(
                                &mut columns[1],
                                "Model section",
                                &mut scoped.model_section,
                                "tt",
                                None,
                            );
                        });
                    }
                    platform_eligibility_field(ui, &mut scoped.eligible_platforms, write_allowed);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if Button::new("Remove")
                            .enabled(write_allowed)
                            .show(ui)
                            .clicked()
                        {
                            action = BodyAction::RemoveOverride(index);
                        }
                    });
                });
            ui.add_space(6.0);
        }
        if Button::new("Add scoped override")
            .enabled(write_allowed)
            .show(ui)
            .clicked()
        {
            action = BodyAction::AddOverride;
        }

        section_header(ui, "Resolved hierarchy", None);
        if let Some(receipt) = receipt.as_ref() {
            for binding in &receipt.bindings {
                let value = format!(
                    "{} · {} · {}",
                    binding.view_search_order.join(" → "),
                    binding.stop_view.as_deref().unwrap_or("—"),
                    binding.model_section
                );
                let label = format!(
                    "{} · {}",
                    binding.instance_paths.join(", "),
                    binding.reference.display_path()
                );
                if binding.status.is_resolved() {
                    property_row(ui, &label, &value);
                } else {
                    property_row_toned(
                        ui,
                        &label,
                        binding.diagnostic.as_deref().unwrap_or(&value),
                        Tokens::get(ui.ctx()).color.err,
                    );
                }
                // A warning belongs to the row it qualifies: the binding is
                // netlisted either way, and the author is reading this list to
                // find out how.
                for warning in &binding.warnings {
                    property_row_toned(ui, "", warning, Tokens::get(ui.ctx()).color.warn);
                }
            }
        }
    }
    action
}

fn platform_eligibility_field(
    ui: &mut Ui,
    eligible_platforms: &mut Vec<ConfigurationPlatform>,
    enabled: bool,
) {
    ui.label(
        egui::RichText::new("Eligible platforms")
            .size(11.0)
            .color(Tokens::get(ui.ctx()).color.text_dim),
    );
    ui.add_enabled_ui(enabled, |ui| {
        ui.horizontal(|ui| {
            for platform in ConfigurationPlatform::ALL {
                let mut selected = eligible_platforms.contains(&platform);
                if ui.checkbox(&mut selected, platform.label()).changed() {
                    if selected {
                        eligible_platforms.push(platform);
                        eligible_platforms.sort();
                        eligible_platforms.dedup();
                    } else {
                        eligible_platforms.retain(|candidate| *candidate != platform);
                    }
                }
            }
        });
    });
}

fn root_selector(
    ui: &mut Ui,
    label: &str,
    value: &mut CellViewRef,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    active_schematic: &crate::state::SchematicState,
) {
    ui.label(label);
    egui::ComboBox::from_id_salt(("configuration-root", label))
        .selected_text(value.display_path())
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for candidate in executable_roots(workspace, libraries, active_schematic) {
                let text = candidate.display_path();
                ui.selectable_value(value, candidate, text);
            }
        });
}

fn executable_roots(
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    _active_schematic: &crate::state::SchematicState,
) -> Vec<CellViewRef> {
    let mut roots = Vec::new();
    for (library_key, library) in libraries.libraries_by_key() {
        for (cell_key, cell) in &library.cells {
            for (view_key, view) in &cell.views {
                if matches!(view.view_type, ViewType::Schematic | ViewType::Testbench) {
                    let reference = CellViewRef::new(library_key, cell_key, view_key);
                    let has_buffer = reference
                        .key()
                        .eq_ignore_ascii_case(&workspace.active_view.key())
                        || workspace
                            .schematic_buffers
                            .keys()
                            .any(|key| key.eq_ignore_ascii_case(&reference.key()));
                    if has_buffer {
                        roots.push(reference);
                    }
                }
            }
        }
    }
    roots.sort_by_key(|reference| reference.key().to_lowercase());
    roots
}

fn configuration_selector(
    ui: &mut Ui,
    label: &str,
    selected: &mut Option<ConfigurationSetId>,
    catalog: &ConfigurationSetCatalog,
) {
    ui.label(label);
    let selected_text = selected.and_then(|id| catalog.find(id)).map_or_else(
        || "Select configuration".to_owned(),
        |configuration| {
            format!(
                "{} · config-{}",
                configuration.name(),
                configuration.revision()
            )
        },
    );
    egui::ComboBox::from_id_salt(("configuration-selector", label))
        .selected_text(selected_text)
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for configuration in catalog.configurations() {
                ui.selectable_value(
                    selected,
                    Some(configuration.id()),
                    format!(
                        "{} · config-{}",
                        configuration.name(),
                        configuration.revision()
                    ),
                );
            }
        });
}

fn enum_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    selected: &mut T,
    values: impl IntoIterator<Item = T>,
    label_for: impl Fn(T) -> &'static str,
) {
    ui.label(label);
    egui::ComboBox::from_id_salt(("configuration-enum", label))
        .selected_text(label_for(*selected))
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for value in values {
                ui.selectable_value(selected, value, label_for(value));
            }
        });
}

fn comma_list_field(
    ui: &mut Ui,
    label: &str,
    values: &mut Vec<String>,
    hint: &str,
    rejection: Option<&str>,
) {
    let mut text = values.join(", ");
    if input_field(
        ui,
        label,
        &mut text,
        hint,
        rejection,
        "Ordered comma-separated executable view names",
    )
    .changed()
    {
        *values = comma_edit_values(&text);
    }
}

/// Every view name the project's libraries hold, in one case-insensitive list.
///
/// A configuration binds views by name, so this is the exact set an author can
/// expect to resolve; anything else is a name the libraries do not have yet.
fn project_view_names(libraries: &crate::state::LibraryManager) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for (_, library) in libraries.libraries_by_key() {
        for cell in library.cells.values() {
            for view in cell.views.values() {
                if !names
                    .iter()
                    .any(|existing| existing.eq_ignore_ascii_case(&view.name))
                {
                    names.push(view.name.clone());
                }
            }
        }
    }
    names.sort_by_key(|name| name.to_lowercase());
    names
}

fn view_name_hint(library_views: &[String], fallback: &str) -> String {
    if library_views.is_empty() {
        fallback.to_owned()
    } else {
        library_views.join(", ")
    }
}

/// Why one typed view name is not a view name, or `None` when it is.
///
/// A configuration may bind any view its libraries can hold, so the only rule
/// is the library's own name grammar — the same one the New view dialog
/// applies when the view is created.
fn view_name_rejection(view: &str) -> Option<String> {
    crate::state::workspace::validate_cell_view_name_segment(view)
        .err()
        .map(|error| format!("View name {view:?} {error}"))
}

/// The same rule over an ordered policy. A single empty trailing entry is the
/// comma the author has just typed, not a name they have finished writing.
fn view_policy_rejection(views: &[String]) -> Option<String> {
    let checked = match views.split_last() {
        Some((last, head)) if last.is_empty() => head,
        _ => views,
    };
    checked.iter().find_map(|view| view_name_rejection(view))
}

fn library_view_row(ui: &mut Ui, library_views: &[String]) {
    if library_views.is_empty() {
        empty_row(ui, "The project libraries hold no views to bind.");
    } else {
        property_row(ui, "Views in project libraries", &library_views.join(", "));
    }
}

/// Name the configured views the libraries do not have. They are legal names
/// and stay exactly as typed — a view authored later resolves them — so this
/// states the fact rather than refusing the entry.
fn absent_view_row<const N: usize>(
    ui: &mut Ui,
    library_views: &[String],
    policies: [&[String]; N],
) {
    let mut absent: Vec<&str> = Vec::new();
    for view in policies.into_iter().flatten() {
        if view.is_empty()
            || library_views
                .iter()
                .any(|known| known.eq_ignore_ascii_case(view))
            || absent
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(view))
        {
            continue;
        }
        absent.push(view.as_str());
    }
    if absent.is_empty() {
        return;
    }
    property_row_toned(
        ui,
        "Not in library",
        &absent.join(", "),
        Tokens::get(ui.ctx()).color.warn,
    );
}

fn comma_edit_values(text: &str) -> Vec<String> {
    if text.trim().is_empty() {
        Vec::new()
    } else {
        text.split(',')
            .map(|value| value.trim().to_owned())
            .collect()
    }
}

fn optional_text_field(
    ui: &mut Ui,
    label: &str,
    value: &mut Option<String>,
    hint: &str,
    rejection: Option<&str>,
) {
    let mut text = value.clone().unwrap_or_default();
    if input_field(
        ui,
        label,
        &mut text,
        hint,
        rejection,
        "Optional exact configuration override",
    )
    .changed()
    {
        let trimmed = text.trim();
        *value = (!trimmed.is_empty()).then(|| trimmed.to_owned());
    }
}

fn definition_from_template(
    name: String,
    root: CellViewRef,
    template: ConfigurationTemplate,
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &crate::state::SchematicState,
) -> Result<ConfigurationSetDefinition, String> {
    let (executable_view_policy, stop_views) = template.policy();
    let dut_path =
        default_dut_path_for_root(workspace, active_schematic, &root).ok_or_else(|| {
            format!(
                "Root '{}' contains no hierarchical instance to bind as the design under test.",
                root.display_path()
            )
        })?;
    Ok(ConfigurationSetDefinition {
        name,
        root,
        dut_path,
        executable_view_policy,
        stop_views,
        unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Local project".to_owned(),
    })
}

/// Why a typed DUT path is not a configuration's, or `None` when it is one.
///
/// A configuration stores the canonical spelling — rooted at the implicit
/// design root and separated by `/`. The engine's `x1.x2` names the same
/// instance and parses, so it is reported with the spelling to write rather
/// than accepted and silently rewritten under the author.
fn instance_path_rejection(text: &str) -> Option<String> {
    let text = text.trim();
    match InstancePath::parse(text) {
        Err(error) => Some(error.to_string()),
        Ok(path) if path.to_string() != text => Some(format!(
            "{text:?} is the engine spelling of this instance; write it as {path}"
        )),
        Ok(_) => None,
    }
}

/// Why a typed override scope is not a hierarchy pattern, or `None` when it is.
fn instance_pattern_rejection(text: &str) -> Option<String> {
    InstancePathPattern::parse(text.trim())
        .err()
        .map(|error| error.to_string())
}

/// The scope a freshly added override starts at: one instance below the design
/// root, which is the shallowest scope an override can name.
fn new_override_scope(ordinal: usize) -> String {
    InstancePath::root()
        .child(&format!("XINSTANCE{ordinal}"))
        .expect("an ASCII instance name one level below the root is inside the grammar")
        .to_string()
}

/// The instance a new configuration binds as its design under test: the
/// lowest-named hierarchical instance in the selected root, one level below the
/// implicit design root.
fn default_dut_path_for_root(
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &crate::state::SchematicState,
    root: &CellViewRef,
) -> Option<String> {
    let root_key = root.key();
    let schematic = if root_key.eq_ignore_ascii_case(&workspace.active_view.key()) {
        Some(active_schematic)
    } else {
        workspace
            .schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&root_key))
            .map(|(_, schematic)| schematic)
    }?;
    schematic
        .components
        .iter()
        .filter(|component| component.kind == crate::state::ComponentType::CellInstance)
        .filter_map(|component| InstancePath::root().child(component.name.trim()).ok())
        .min_by_key(InstancePath::fold_key)
        .map(|path| path.to_string())
}

fn unique_configuration_name(catalog: &ConfigurationSetCatalog, stem: &str) -> String {
    if catalog
        .configurations()
        .iter()
        .all(|configuration| !configuration.name().eq_ignore_ascii_case(stem))
    {
        return stem.to_owned();
    }
    (2..)
        .map(|suffix| format!("{stem} {suffix}"))
        .find(|candidate| {
            catalog
                .configurations()
                .iter()
                .all(|configuration| !configuration.name().eq_ignore_ascii_case(candidate))
        })
        .expect("configuration name space is unbounded")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(name: &str) -> ConfigurationSetDefinition {
        ConfigurationSetDefinition {
            name: name.to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/XDUT".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        }
    }

    fn valid_configuration_app() -> (RSpiceApp, ConfigurationSetId) {
        let mut app = RSpiceApp::test_instance();
        let cell_name = "configuration_test_dut";
        let mut cell = crate::state::Cell::new(cell_name);
        cell.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        if let Some(library) = app.state.library_manager.get_library_mut("work") {
            library.add_cell(cell);
        } else {
            let mut library = crate::state::Library::new("work");
            library.add_cell(cell);
            app.state.library_manager.add_library(library);
        }
        let reference = CellViewRef::new("work", cell_name, "schematic");
        let mut master = crate::state::SchematicState::default();
        for (name, position) in [
            ("a", crate::state::Point::new(0, 0)),
            ("b", crate::state::Point::new(40, 0)),
        ] {
            let id = master.add_component(crate::state::ComponentType::Port, position);
            master
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .expect("master port")
                .value = name.to_owned();
        }
        app.state
            .workspace
            .schematic_buffers
            .insert(reference.key(), master);
        let mut binding = crate::state::LibraryCellInstance::new("work", cell_name, "schematic");
        binding.terminal_order = vec!["a".to_owned(), "b".to_owned()];
        app.state
            .schematic
            .add_library_cell_component(crate::state::Point::new(20, 20), binding);
        let mut definition = definition("Release");
        definition.dut_path = "/top/X1".to_owned();
        let id = app
            .state
            .workspace
            .configuration_sets
            .create(definition)
            .expect("valid configuration");
        (app, id)
    }

    #[test]
    fn templates_only_advertise_bindable_view_names() {
        for template in ConfigurationTemplate::ALL {
            let (views, stops) = template.policy();
            assert!(views.iter().all(|view| view_name_rejection(view).is_none()));
            assert!(stops.iter().all(|stop| views.contains(stop)));
        }
    }

    #[test]
    fn a_view_policy_field_refuses_only_what_is_not_a_view_name() {
        assert_eq!(view_policy_rejection(&comma_edit_values("spice_tt,")), None);
        assert_eq!(
            view_policy_rejection(&comma_edit_values("schematic_fast, spice_tt")),
            None
        );
        assert!(
            view_policy_rejection(&comma_edit_values("fast schematic"))
                .is_some_and(|rejection| rejection.contains("fast schematic")),
            "a name the library grammar refuses is quoted back"
        );
        assert!(view_name_rejection("a/b").is_some());
        assert!(view_name_rejection("").is_some());
    }

    #[test]
    fn the_binding_page_states_which_configured_views_the_libraries_hold() {
        let (app, _) = valid_configuration_app();
        let library_views = project_view_names(&app.state.library_manager);
        assert!(library_views.iter().any(|view| view == "schematic"));
        assert!(
            library_views
                .windows(2)
                .all(|pair| pair[0].to_lowercase() <= pair[1].to_lowercase()),
            "the offered names are ordered: {library_views:?}"
        );
        assert!(!library_views.iter().any(|view| view == "spice_tt"));
    }

    #[test]
    fn generated_names_are_case_insensitively_unique() {
        let mut catalog = ConfigurationSetCatalog::default();
        catalog
            .create(definition("Release"))
            .expect("configuration");
        assert_eq!(unique_configuration_name(&catalog, "release"), "release 2");
    }

    #[test]
    fn hierarchy_binding_entry_opens_the_binding_page_and_requires_a_configuration() {
        let (mut app, _) = valid_configuration_app();
        open_configuration_binding_dialog(&mut app.state);
        assert!(app.state.dialogs.configuration_sets.open);
        assert_eq!(
            app.state.dialogs.configuration_sets.page,
            ConfigurationDialogPage::Binding
        );
        assert!(app.state.dialogs.configuration_sets.error.is_none());

        let mut empty = RSpiceApp::test_instance();
        open_configuration_binding_dialog(&mut empty.state);
        assert!(empty.state.dialogs.configuration_sets.open);
        assert_eq!(
            empty.state.dialogs.configuration_sets.page,
            ConfigurationDialogPage::Manager
        );
        assert_eq!(
            empty.state.dialogs.configuration_sets.error.as_deref(),
            Some("Create a configuration set before editing hierarchy bindings.")
        );
    }

    #[test]
    fn same_row_and_subordinate_actions_never_discard_a_dirty_draft() {
        let mut app = RSpiceApp::test_instance();
        let selected = app
            .state
            .workspace
            .configuration_sets
            .create(definition("Release"))
            .expect("configuration");
        let other = app
            .state
            .workspace
            .configuration_sets
            .create(definition("Characterization"))
            .expect("second configuration");
        open_configuration_sets_dialog(&mut app.state);
        app.state
            .dialogs
            .configuration_sets
            .draft
            .as_mut()
            .expect("draft")
            .name = "Unsaved release".to_owned();

        app.handle_configuration_body_action(BodyAction::Select(selected));
        assert_eq!(
            app.state
                .dialogs
                .configuration_sets
                .draft
                .as_ref()
                .map(|draft| draft.name.as_str()),
            Some("Unsaved release")
        );

        for action in [
            BodyAction::New,
            BodyAction::Clone,
            BodyAction::Select(other),
        ] {
            app.handle_configuration_body_action(action);
            assert_eq!(
                app.state.dialogs.configuration_sets.page,
                ConfigurationDialogPage::Manager
            );
            assert_eq!(
                app.state.dialogs.configuration_sets.selected_id,
                Some(selected)
            );
            assert_eq!(
                app.state
                    .dialogs
                    .configuration_sets
                    .draft
                    .as_ref()
                    .map(|draft| draft.name.as_str()),
                Some("Unsaved release")
            );
            assert!(app.state.dialogs.configuration_sets.error.is_some());
        }
    }

    #[test]
    fn comma_editor_preserves_an_in_progress_trailing_entry() {
        assert_eq!(
            comma_edit_values("schematic,"),
            vec!["schematic".to_owned(), String::new()]
        );
        assert_eq!(
            comma_edit_values("schematic, spice"),
            vec!["schematic".to_owned(), "spice".to_owned()]
        );
        assert!(comma_edit_values("  ").is_empty());
    }

    #[test]
    fn new_configuration_requires_a_real_dut_in_the_selected_root() {
        let mut app = RSpiceApp::test_instance();
        let root = app.state.workspace.active_view.clone();
        assert!(
            default_dut_path_for_root(&app.state.workspace, &app.state.schematic, &root).is_none()
        );

        for (id, name) in [(1u64, "XB"), (2, "XA")] {
            let mut instance = crate::state::Component::new(
                id,
                crate::state::ComponentType::CellInstance,
                crate::state::Point::new(20 * i32::try_from(id).expect("small id"), 20),
            );
            instance.name = name.to_owned();
            app.state.schematic.components.push(instance);
        }
        assert_eq!(
            default_dut_path_for_root(&app.state.workspace, &app.state.schematic, &root).as_deref(),
            Some("/XA"),
            "the lowest instance name, below the implicit design root"
        );
    }

    #[test]
    fn the_path_fields_take_the_canonical_spelling_and_report_the_text_they_refuse() {
        assert_eq!(instance_path_rejection("/XAFE"), None);
        assert_eq!(instance_path_rejection("/XAFE/XBIAS"), None);
        assert_eq!(instance_path_rejection("/"), None);
        assert!(
            instance_path_rejection("/XAFE/").is_some_and(|rejection| rejection.contains("/XAFE/")),
            "an empty trailing segment is refused and quoted back"
        );
        assert!(
            instance_path_rejection("XAFE").is_some_and(|rejection| rejection.contains("XAFE")),
            "the engine spelling is refused and quoted back"
        );

        assert_eq!(instance_pattern_rejection("/XAFE"), None);
        assert_eq!(instance_pattern_rejection("/XAFE/*"), None);
        assert!(
            instance_pattern_rejection("XAFE").is_some_and(|rejection| rejection.contains("XAFE"))
        );
        assert!(
            instance_pattern_rejection("/XAFE/")
                .is_some_and(|rejection| rejection.contains("/XAFE/"))
        );
    }

    #[test]
    fn a_new_override_starts_one_instance_below_the_design_root() {
        assert_eq!(new_override_scope(1), "/XINSTANCE1");
        assert_eq!(instance_pattern_rejection(&new_override_scope(2)), None);
    }

    #[test]
    fn normalization_only_save_revalidates_without_mutating_project_authority() {
        let (mut app, id) = valid_configuration_app();
        open_configuration_sets_dialog(&mut app.state);
        let revision = app.state.workspace.project.revision();
        app.state
            .dialogs
            .configuration_sets
            .draft
            .as_mut()
            .expect("configuration draft")
            .name = "  Release  ".to_owned();

        app.commit_configuration_update()
            .expect("normalization-only save validates and reloads");

        assert_eq!(app.state.workspace.project.revision(), revision);
        assert_eq!(
            app.state
                .workspace
                .configuration_sets
                .find(id)
                .expect("configuration remains")
                .name(),
            "Release"
        );
        assert_eq!(
            app.state
                .dialogs
                .configuration_sets
                .draft
                .as_ref()
                .map(|draft| draft.name.as_str()),
            Some("Release")
        );
    }

    #[test]
    fn invalid_save_preserves_catalog_and_project_revision() {
        let (mut app, _) = valid_configuration_app();
        open_configuration_sets_dialog(&mut app.state);
        let catalog = app.state.workspace.configuration_sets.clone();
        let revision = app.state.workspace.project.revision();
        app.state
            .dialogs
            .configuration_sets
            .draft
            .as_mut()
            .expect("configuration draft")
            .dut_path = "/top/XMISSING".to_owned();

        let error = app
            .commit_configuration_update()
            .expect_err("unresolved configured DUT must block publication");

        assert!(error.contains("does not exist"), "{error}");
        assert_eq!(app.state.workspace.configuration_sets, catalog);
        assert_eq!(app.state.workspace.project.revision(), revision);
    }
}
