//! The configuration dialogs: New configuration, Clone, Rename and Delete,
//! and the question asked before edits that are not applied would be lost.
//!
//! Everything else about a configuration is edited on its page. These four
//! are the transactions that add one to the project, name one, or take one
//! away; each commits through the configuration transactions.

use egui::{Context, Id, TextEdit, Ui};

use crate::services::recent_files::RecentKind;
use crate::state::{
    CellViewRef, ConfigurationBlackBoxPolicy, ConfigurationModelProfile, ConfigurationSetCatalog,
    ConfigurationSetDefinition, ConfigurationSetId, DEFAULT_STOP_VIEW_TYPES, InstancePath,
    UnresolvedBindingPolicy, ViewType, default_view_search_order,
};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, field_label,
    kv_row, select, switch_row,
};
use crate::workbench::app::dialogs::project::configuration::{
    ConfigurationTransaction, commit_configuration_transaction, delete_refusal,
};
use crate::workbench::app::dialogs::review_primitives::text_edit_field;
use crate::workbench::app::{ConfirmationAction, RSpiceApp};
use crate::workbench::app_state::AppState;
use crate::workbench::project::configuration_state::{ConfigurationSelection, used_for_runs};
use crate::workbench::project::workspace_state::ProjectPage;

/// Where a new configuration's view list and stop list come from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum StartFrom {
    /// The built-in order, written out as a configuration's lists.
    #[default]
    Default,
    AnalogSchematic,
    PostLayoutExtracted,
    MixedSignal,
    Rf,
}

impl StartFrom {
    const ALL: [Self; 5] = [
        Self::Default,
        Self::AnalogSchematic,
        Self::PostLayoutExtracted,
        Self::MixedSignal,
        Self::Rf,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::AnalogSchematic => "Analog schematic",
            Self::PostLayoutExtracted => "Post-layout extracted",
            Self::MixedSignal => "Mixed-signal",
            Self::Rf => "RF",
        }
    }

    /// The view list and stop list a configuration started from here holds.
    fn lists(self) -> (Vec<String>, Vec<String>) {
        let owned = |views: &[&str]| views.iter().map(|view| (*view).to_owned()).collect();
        match self {
            Self::Default => {
                // Default searches an instance's own view first, which one
                // list cannot say. Every view Default stops at follows the
                // schematic order, so a cell placed by its Verilog-A or
                // Verilog view still binds, and the stop list is Default's.
                let mut views = default_view_search_order("schematic", false);
                let stops: Vec<String> = DEFAULT_STOP_VIEW_TYPES
                    .iter()
                    .map(|view| view.display_name().to_owned())
                    .collect();
                for stop in &stops {
                    if !views.iter().any(|view| view.eq_ignore_ascii_case(stop)) {
                        views.push(stop.clone());
                    }
                }
                (views, stops)
            }
            Self::AnalogSchematic => (
                owned(&["schematic", "extracted", "spice"]),
                owned(&["extracted", "spice"]),
            ),
            Self::PostLayoutExtracted => (
                owned(&["extracted", "schematic", "spice"]),
                owned(&["extracted", "spice"]),
            ),
            Self::MixedSignal => (
                owned(&["schematic", "veriloga", "spice"]),
                owned(&["veriloga", "spice"]),
            ),
            Self::Rf => (owned(&["schematic", "spice"]), owned(&["spice"])),
        }
    }
}

/// The dialog that is open, with what has been entered in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConfigurationDialog {
    New {
        name: String,
        start: StartFrom,
        top: CellViewRef,
        use_for_runs: bool,
    },
    Clone {
        source: ConfigurationSetId,
        name: String,
        use_for_runs: bool,
    },
    Rename {
        id: ConfigurationSetId,
        name: String,
    },
    Delete(ConfigurationSetId),
}

/// A request that takes the open project away, held while the configuration
/// in front has edits that are not applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Leaving {
    pub(crate) action: ConfirmationAction,
    /// The recent project an Open recent asked for.
    pub(crate) recent: Option<(std::path::PathBuf, RecentKind)>,
}

/// The configuration dialogs' state, held by the project feature.
#[derive(Debug, Clone, Default)]
pub(crate) struct ConfigurationDialogState {
    pub(crate) open: Option<ConfigurationDialog>,
    /// Why the last commit was refused; the dialog stays open to say so.
    refusal: Option<String>,
    /// Opening, creating or importing another project, closing this one, or
    /// exiting, waiting for the edits not applied to be applied or discarded.
    pub(crate) leaving: Option<Leaving>,
}

impl ConfigurationDialogState {
    pub(crate) fn is_open(&self) -> bool {
        self.open.is_some()
    }

    fn close(&mut self) {
        self.open = None;
        self.refusal = None;
    }
}

/// Hold `action`, which takes the open project away, while the Configuration
/// page has edits that are not applied: the page asks to apply or discard
/// them first, then goes on with it. Returns whether it held the action.
pub(in crate::workbench) fn hold_for_configuration_edits(
    state: &mut AppState,
    action: ConfirmationAction,
    recent: Option<(std::path::PathBuf, RecentKind)>,
) -> bool {
    if !state
        .project
        .configuration
        .is_dirty(&state.model.workspace.content.configuration_sets)
    {
        return false;
    }
    state.project.configuration_dialog.leaving = Some(Leaving { action, recent });
    true
}

/// Which dialog to open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) enum ConfigurationDialogRequest {
    New,
    Clone(ConfigurationSetId),
    Rename(ConfigurationSetId),
    Delete(ConfigurationSetId),
}

/// Open the dialog `request` names, filled in from the project.
pub(in crate::workbench) fn open_configuration_dialog(
    state: &mut AppState,
    request: ConfigurationDialogRequest,
) {
    let catalog: &ConfigurationSetCatalog = &state.model.workspace.content.configuration_sets;
    let dialog = match request {
        ConfigurationDialogRequest::New => ConfigurationDialog::New {
            name: unique_name(catalog, "Configuration"),
            start: StartFrom::Default,
            top: state.model.workspace.content.simulation_root_reference(),
            // A project's first configuration is what it means to run with
            // one; with others already there, a new one waits to be chosen.
            use_for_runs: catalog.configurations().is_empty(),
        },
        ConfigurationDialogRequest::Clone(source) => {
            let Some(original) = catalog.find(source) else {
                return;
            };
            ConfigurationDialog::Clone {
                source,
                name: unique_name(catalog, &format!("{} copy", original.name())),
                use_for_runs: used_for_runs(catalog, ConfigurationSelection::Configuration(source)),
            }
        }
        ConfigurationDialogRequest::Rename(id) => {
            let Some(configuration) = catalog.find(id) else {
                return;
            };
            ConfigurationDialog::Rename {
                id,
                name: configuration.name().to_owned(),
            }
        }
        ConfigurationDialogRequest::Delete(id) => ConfigurationDialog::Delete(id),
    };
    state.project.configuration_dialog.open = Some(dialog);
    state.project.configuration_dialog.refusal = None;
}

fn name_id() -> Id {
    Id::new("rspice.configuration-dialog.name")
}

/// Bring the Project > Configuration page forward.
pub(in crate::workbench) fn show_configuration_page(state: &mut AppState) {
    if !state.platform.project_lifecycle.is_open() {
        return;
    }
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Project);
    state.project.project_page = ProjectPage::Configuration;
}

/// The actions a configuration row offers, as menu buttons: the dialog the
/// reader picked, if one. Default offers only a new configuration; it can be
/// neither cloned (start a new one from it instead), renamed nor deleted.
pub(in crate::workbench) fn configuration_actions_menu(
    ui: &mut Ui,
    selection: ConfigurationSelection,
    write_allowed: bool,
) -> Option<ConfigurationDialogRequest> {
    let mut picked = None;
    let mut item = |ui: &mut Ui, label: &str, request: ConfigurationDialogRequest| {
        if ui
            .add_enabled(write_allowed, egui::Button::new(label))
            .clicked()
        {
            picked = Some(request);
        }
    };
    item(
        ui,
        "New configuration\u{2026}",
        ConfigurationDialogRequest::New,
    );
    if let ConfigurationSelection::Configuration(id) = selection {
        item(ui, "Clone\u{2026}", ConfigurationDialogRequest::Clone(id));
        item(ui, "Rename\u{2026}", ConfigurationDialogRequest::Rename(id));
        item(ui, "Delete", ConfigurationDialogRequest::Delete(id));
    }
    picked
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_configuration_dialog(&mut self, ctx: &Context) {
        if self.state.project.configuration_dialog.leaving.is_some() {
            let choice = edits_prompt(ctx, &self.state).unwrap_or(DialogChoice::Secondary);
            let leaving = match answer_leaving(&mut self.state, choice) {
                LeavingAnswer::Stay => None,
                LeavingAnswer::GoOn(leaving) => Some(leaving),
                LeavingAnswer::ApplyThenGoOn(leaving) => {
                    match commit_configuration_transaction(self, ConfigurationTransaction::Apply) {
                        Ok(()) => Some(leaving),
                        // A refused apply stays, on the page, saying why.
                        Err(refusal) => {
                            self.state.project.configuration.refusal = Some(refusal);
                            show_configuration_page(&mut self.state);
                            None
                        }
                    }
                }
            };
            if let Some((action, path, kind)) =
                leaving.and_then(|leaving| go_on_leaving(&mut self.state, leaving))
            {
                self.execute_pending_action(action, path, kind, None);
            }
            return;
        }
        if let Some((target, choice)) = selection_prompt(ctx, &self.state) {
            match choice {
                DialogChoice::Primary => {
                    self.state.project.configuration.pending_selection = None;
                    match commit_configuration_transaction(self, ConfigurationTransaction::Apply) {
                        Ok(()) => self.state.project.configuration.select(target),
                        Err(refusal) => {
                            self.state.project.configuration.refusal = Some(refusal);
                        }
                    }
                }
                DialogChoice::Secondary => self.state.project.configuration.select(target),
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    self.state.project.configuration.pending_selection = None;
                }
                DialogChoice::None => {}
            }
            return;
        }
        let Some(dialog) = self.state.project.configuration_dialog.open.clone() else {
            return;
        };
        let write_allowed = !self.state.model.safe_mode.project_read_only();
        let refusal = self.state.project.configuration_dialog.refusal.clone();
        let (choice, transaction) = match dialog {
            ConfigurationDialog::New { .. } => {
                new_dialog(ctx, &mut self.state, write_allowed, refusal)
            }
            ConfigurationDialog::Clone { .. } | ConfigurationDialog::Rename { .. } => {
                name_dialog(ctx, &mut self.state, write_allowed, refusal)
            }
            ConfigurationDialog::Delete(id) => {
                delete_dialog(ctx, &self.state, id, write_allowed, refusal)
            }
        };
        match choice {
            DialogChoice::Primary => {
                let Some(transaction) = transaction else {
                    return;
                };
                match commit_configuration_transaction(self, transaction) {
                    Ok(()) => self.state.project.configuration_dialog.close(),
                    Err(refusal) => {
                        self.state.project.configuration_dialog.refusal = Some(refusal);
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.project.configuration_dialog.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

/// What the answer to the question asked before leaving the project leaves
/// to do.
#[derive(Debug, PartialEq, Eq)]
enum LeavingAnswer {
    /// Stay in the project: cancelled, or not answered yet.
    Stay,
    /// Apply the edits, then go on leaving.
    ApplyThenGoOn(Leaving),
    /// The edits are discarded: go on leaving.
    GoOn(Leaving),
}

/// Take the reader's answer about the configuration edits held before
/// leaving the project. Discard drops them here; Apply is the caller's,
/// because it commits through the application.
fn answer_leaving(state: &mut AppState, choice: DialogChoice) -> LeavingAnswer {
    if choice == DialogChoice::None {
        return LeavingAnswer::Stay;
    }
    let Some(leaving) = state.project.configuration_dialog.leaving.take() else {
        return LeavingAnswer::Stay;
    };
    match choice {
        DialogChoice::Primary => LeavingAnswer::ApplyThenGoOn(leaving),
        DialogChoice::Secondary => {
            state.project.configuration.revert();
            LeavingAnswer::GoOn(leaving)
        }
        DialogChoice::Ghost | DialogChoice::Cancelled | DialogChoice::None => LeavingAnswer::Stay,
    }
}

/// Go on with `leaving` once the configuration edits are settled: ask about
/// the project's unsaved work first, as the request itself does, or hand back
/// the action to carry out now. An applied edit is unsaved work too: the
/// project changed.
fn go_on_leaving(
    state: &mut AppState,
    Leaving { action, recent }: Leaving,
) -> Option<(
    ConfirmationAction,
    Option<std::path::PathBuf>,
    Option<RecentKind>,
)> {
    if let ConfirmationAction::CloseProject(destination) = action {
        // The close review closes at once when nothing is left unsaved.
        state
            .workbench
            .project_review_dialog
            .show_close_project(destination);
        return None;
    }
    let drafts = if action == ConfirmationAction::Exit {
        state.models.has_unsaved_authoring_changes()
    } else {
        state.models.model_editor_has_unsaved_changes()
    };
    if !drafts && !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state) {
        let (path, kind) = recent.unzip();
        return Some((action, path, kind));
    }
    match recent {
        Some((path, kind)) => state.workbench.confirmation_dialog.show_recent(path, kind),
        None => state.workbench.confirmation_dialog.show(action),
    }
    None
}

fn new_dialog(
    ctx: &Context,
    state: &mut AppState,
    write_allowed: bool,
    refusal: Option<String>,
) -> (DialogChoice, Option<ConfigurationTransaction>) {
    let choices = top_cells(state);
    let Some(ConfigurationDialog::New {
        name,
        start,
        top,
        use_for_runs,
    }) = state.project.configuration_dialog.open.as_mut()
    else {
        return (DialogChoice::None, None);
    };
    let dut = default_dut_path(&state.model.workspace, &state.model.schematic, top);
    let problem = refusal.or_else(|| {
        dut.is_none().then(|| {
            format!(
                "{} places no instance to use as the DUT. Place one, or pick another top cell.",
                top.display_path()
            )
        })
    });
    let mut dialog = Dialog::new("New configuration", "Create")
        .description(
            "A configuration picks the top cell, the view list and stop list, and the \
                 bindings netlisting follows.",
        )
        .size(DialogSize::CommandForm)
        .overlay_scrollbar()
        .ghost("Cancel")
        .primary_enabled(write_allowed && !name.trim().is_empty() && dut.is_some())
        .initial_focus(DialogInitialFocus::Control(name_id()));
    if let Some(problem) = problem.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "The configuration cannot be created",
            problem,
        );
    }
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 6.0;
        name_field(ui, name, write_allowed);
        let labels = StartFrom::ALL.map(|start| start.label().to_owned());
        if let Some(index) = picker(ui, "Start from", start.label(), &labels, write_allowed) {
            *start = StartFrom::ALL[index];
        }
        let top_labels = choices
            .iter()
            .map(CellViewRef::display_path)
            .collect::<Vec<_>>();
        if let Some(index) = picker(
            ui,
            "Top cell",
            &top.display_path(),
            &top_labels,
            write_allowed,
        ) {
            *top = choices[index].clone();
        }
        let (views, stops) = start.lists();
        kv_row(ui, "View list", &views.join(", then "));
        kv_row(ui, "Stop list", &stops.join(", "));
        if let Some(dut) = &dut {
            kv_row(ui, "DUT instance", dut);
        }
        ui.add_enabled_ui(write_allowed, |ui| {
            switch_row(ui, "Use for runs", use_for_runs);
        });
        Some(name_id())
    });
    let transaction = dut.map(|dut_path| {
        let (executable_view_policy, stop_views) = start.lists();
        ConfigurationTransaction::Create {
            definition: Box::new(ConfigurationSetDefinition {
                name: name.trim().to_owned(),
                root: top.clone(),
                dut_path,
                executable_view_policy,
                stop_views,
                unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                cell_bindings: Vec::new(),
                model_profile: ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            }),
            use_for_runs: *use_for_runs,
        }
    });
    (choice, transaction)
}

/// Clone and Rename: a name, and for Clone whether runs use the copy.
fn name_dialog(
    ctx: &Context,
    state: &mut AppState,
    write_allowed: bool,
    refusal: Option<String>,
) -> (DialogChoice, Option<ConfigurationTransaction>) {
    let catalog: &ConfigurationSetCatalog = &state.model.workspace.content.configuration_sets;
    let (title, primary, source_name) = match state.project.configuration_dialog.open.as_ref() {
        Some(ConfigurationDialog::Clone { source, .. }) => (
            "Clone configuration",
            "Clone",
            catalog.find(*source).map(|source| source.name().to_owned()),
        ),
        Some(ConfigurationDialog::Rename { id, .. }) => (
            "Rename configuration",
            "Rename",
            catalog.find(*id).map(|source| source.name().to_owned()),
        ),
        _ => return (DialogChoice::None, None),
    };
    let Some(source_name) = source_name else {
        state.project.configuration_dialog.close();
        return (DialogChoice::None, None);
    };
    let description = if primary == "Clone" {
        format!("The copy holds every binding of {source_name}.")
    } else {
        format!("Rename {source_name}. Runs and its bindings are not affected.")
    };
    let mut transaction = None;
    let dialog_state = &mut state.project.configuration_dialog;
    let name_empty = match dialog_state.open.as_ref() {
        Some(
            ConfigurationDialog::Clone { name, .. } | ConfigurationDialog::Rename { name, .. },
        ) => name.trim().is_empty(),
        _ => true,
    };
    let mut dialog = Dialog::new(title, primary)
        .description(&description)
        .size(DialogSize::CommandForm)
        .overlay_scrollbar()
        .ghost("Cancel")
        .primary_enabled(write_allowed && !name_empty)
        .initial_focus(DialogInitialFocus::Control(name_id()));
    if let Some(refusal) = refusal.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            if primary == "Clone" {
                "The configuration cannot be cloned"
            } else {
                "The configuration cannot be renamed"
            },
            refusal,
        );
    }
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 6.0;
        kv_row(ui, "Configuration", &source_name);
        match dialog_state.open.as_mut() {
            Some(ConfigurationDialog::Clone {
                source,
                name,
                use_for_runs,
            }) => {
                name_field(ui, name, write_allowed);
                ui.add_enabled_ui(write_allowed, |ui| {
                    switch_row(ui, "Use for runs", use_for_runs);
                });
                transaction = Some(ConfigurationTransaction::Clone {
                    source: *source,
                    name: name.trim().to_owned(),
                    use_for_runs: *use_for_runs,
                });
            }
            Some(ConfigurationDialog::Rename { id, name }) => {
                name_field(ui, name, write_allowed);
                transaction = Some(ConfigurationTransaction::Rename {
                    id: *id,
                    name: name.trim().to_owned(),
                });
            }
            _ => {}
        }
        Some(name_id())
    });
    (choice, transaction)
}

/// The reader picked another configuration while the one in front has edits
/// that are not applied: apply them, discard them, or stay. Returns the
/// choice and the configuration it was asked for.
fn selection_prompt(
    ctx: &Context,
    state: &AppState,
) -> Option<(ConfigurationSelection, DialogChoice)> {
    let target = state.project.configuration.pending_selection?;
    Some((
        target,
        edits_prompt(ctx, state).unwrap_or(DialogChoice::Secondary),
    ))
}

/// Apply, discard or keep the edits not applied to the configuration in
/// front. `None` when there are none left to ask about.
fn edits_prompt(ctx: &Context, state: &AppState) -> Option<DialogChoice> {
    let catalog: &ConfigurationSetCatalog = &state.model.workspace.content.configuration_sets;
    let draft = state.project.configuration.draft()?;
    let name = catalog.find(draft.id).map_or_else(
        || draft.definition.name.clone(),
        |committed| committed.name().to_owned(),
    );
    let title = format!("Apply your edits to {name}?");
    let consequence = format!(
        "{name} has edits that are not applied. Applying commits them to the project; discarding puts {name} back as it was."
    );
    let choice = Dialog::new(&title, "Apply")
        .description(&consequence)
        .size(DialogSize::Confirmation)
        .primary_enabled(!state.model.safe_mode.project_read_only())
        .secondary("Discard")
        .secondary_leading()
        .ghost("Cancel")
        .show(ctx, |ui| consequence_text(ui, &consequence));
    Some(choice)
}

/// What a confirmation does, in the dialog's body type.
fn consequence_text(ui: &mut Ui, consequence: &str) {
    let t = crate::ui::tokens::Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(consequence)
                .font(crate::ui::theme::sans(
                    crate::ui::tokens::FS_2,
                    crate::ui::theme::FontWeight::Regular,
                ))
                .color(t.color.text),
        )
        .wrap(),
    );
}

fn delete_dialog(
    ctx: &Context,
    state: &AppState,
    id: ConfigurationSetId,
    write_allowed: bool,
    refusal: Option<String>,
) -> (DialogChoice, Option<ConfigurationTransaction>) {
    let catalog: &ConfigurationSetCatalog = &state.model.workspace.content.configuration_sets;
    let Some(configuration) = catalog.find(id) else {
        return (DialogChoice::Cancelled, None);
    };
    let name = configuration.name().to_owned();
    let in_use = used_for_runs(catalog, ConfigurationSelection::Configuration(id));
    let title = format!("Delete {name}?");
    let consequence = format!(
        "{name} and its {} instance binding{} leave the project. Runs that used it before are \
         not affected.",
        configuration.overrides().len(),
        crate::ui::accessibility::plural_suffix(configuration.overrides().len())
    );
    let mut dialog = Dialog::new(&title, "Delete")
        .description(&consequence)
        .size(DialogSize::Confirmation)
        .destructive()
        .ghost("Cancel")
        .primary_enabled(write_allowed && !in_use);
    let problem = refusal.or_else(|| in_use.then(|| delete_refusal(&name)));
    if let Some(problem) = problem.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "The configuration cannot be deleted",
            problem,
        );
    }
    let choice = dialog.show(ctx, |ui| consequence_text(ui, &consequence));
    (choice, Some(ConfigurationTransaction::Delete(id)))
}

fn name_field(ui: &mut Ui, name: &mut String, write_allowed: bool) {
    ui.add_enabled_ui(write_allowed, |ui| {
        text_edit_field(
            ui,
            "Name",
            TextEdit::singleline(name).id(name_id()),
            None,
            "Configuration name",
        );
    });
}

/// A labelled dropdown; the picked index, if one was picked this frame.
fn picker(
    ui: &mut Ui,
    label: &str,
    current: &str,
    options: &[String],
    enabled: bool,
) -> Option<usize> {
    field_label(ui, label, |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            let width = ui.available_width();
            select(
                ui,
                &format!("configuration-dialog-{label}"),
                label,
                current,
                options,
                width,
            )
        })
        .inner
    })
}

/// The schematic and testbench views a new configuration can take as its
/// top cell.
fn top_cells(state: &AppState) -> Vec<CellViewRef> {
    let workspace = &state.model.workspace;
    let mut roots = Vec::new();
    for (library_key, library) in state.model.library_manager.libraries_by_key() {
        for (cell_key, cell) in &library.cells {
            for (view_key, view) in &cell.views {
                if !matches!(view.view_type, ViewType::Schematic | ViewType::Testbench) {
                    continue;
                }
                let reference = CellViewRef::new(library_key, cell_key, view_key);
                let open = reference
                    .key()
                    .eq_ignore_ascii_case(&workspace.content.active_view.key())
                    || workspace
                        .content
                        .schematic_buffers
                        .keys()
                        .any(|key| key.eq_ignore_ascii_case(&reference.key()));
                if open {
                    roots.push(reference);
                }
            }
        }
    }
    roots.sort_by_key(|reference| reference.key().to_lowercase());
    roots
}

/// The instance a new configuration takes as its DUT: the lowest-named
/// hierarchical instance one level below `root`.
fn default_dut_path(
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &crate::state::SchematicState,
    root: &CellViewRef,
) -> Option<String> {
    let root_key = root.key();
    let schematic = if root_key.eq_ignore_ascii_case(&workspace.content.active_view.key()) {
        Some(active_schematic.document())
    } else {
        workspace
            .content
            .schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&root_key))
            .map(|(_, schematic)| schematic.document())
    }?;
    schematic
        .components
        .iter()
        .filter(|component| component.kind == crate::state::ComponentType::CellInstance)
        .filter_map(|component| InstancePath::root().child(component.name.trim()).ok())
        .min_by_key(InstancePath::fold_key)
        .map(|path| path.to_string())
}

/// `stem`, or `stem 2`, `stem 3`, … : the first name no configuration has.
fn unique_name(catalog: &ConfigurationSetCatalog, stem: &str) -> String {
    let taken = |candidate: &str| {
        catalog
            .configurations()
            .iter()
            .any(|configuration| configuration.name().eq_ignore_ascii_case(candidate))
    };
    if !taken(stem) {
        return stem.to_owned();
    }
    (2..)
        .map(|suffix| format!("{stem} {suffix}"))
        .find(|candidate| !taken(candidate))
        .expect("the name space is unbounded")
}

#[cfg(test)]
mod tests;
