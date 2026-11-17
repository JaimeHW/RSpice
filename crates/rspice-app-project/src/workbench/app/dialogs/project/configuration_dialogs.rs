//! The configuration dialogs: New configuration, Clone, Rename and Delete,
//! and the question asked before edits that are not applied would be lost.
//!
//! Everything else about a configuration is edited on its page. These four
//! are the transactions that add one to the project, name one, or take one
//! away; each commits through the configuration transactions.

use egui::{Context, Id, Ui};

use crate::services::recent_files::RecentKind;
use crate::state::{
    CellViewRef, ConfigurationBlackBoxPolicy, ConfigurationModelProfile, ConfigurationSetCatalog,
    ConfigurationSetDefinition, ConfigurationSetId, DEFAULT_STOP_VIEW_TYPES, InstancePath,
    UnresolvedBindingPolicy, ViewType, default_view_search_order,
};
use crate::ui::widgets::{CommandForm, Dialog, DialogChoice, DialogSize, DialogTransactionTone};
use crate::workbench::app::ConfirmationAction;
use crate::workbench::app::dialogs::project::configuration::{
    Committed, ConfigurationTransaction, commit_configuration, delete_refusal,
    finish_configuration_check,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationSelection, used_for_runs,
};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::slots::runs::RunsState;

/// Where a new configuration's view list and stop list come from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StartFrom {
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
            // Every behavioural view Default binds, after the schematic: a
            // cell placed by its Verilog or Verilog-AMS view binds here too.
            Self::MixedSignal => (
                owned(&["schematic", "veriloga", "verilogams", "verilog", "spice"]),
                owned(&["veriloga", "verilogams", "verilog", "spice"]),
            ),
            Self::Rf => (owned(&["schematic", "spice"]), owned(&["spice"])),
        }
    }
}

/// The dialog that is open, with what has been entered in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigurationDialog {
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
    /// Drop the edits not applied to the configuration.
    Revert(ConfigurationSetId),
}

/// A request that takes the open project away, held while the configuration
/// in front has edits that are not applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leaving {
    pub action: ConfirmationAction,
    /// The recent project an Open recent asked for.
    pub recent: Option<(std::path::PathBuf, RecentKind)>,
}

/// The configuration dialogs' state, held by the project feature.
#[derive(Debug, Clone, Default)]
pub struct ConfigurationDialogState {
    pub open: Option<ConfigurationDialog>,
    /// Why the last commit was refused; the dialog stays open to say so.
    refusal: Option<String>,
    /// Opening, creating or importing another project, closing this one, or
    /// exiting, waiting for the edits not applied to be applied or discarded.
    pub leaving: Option<Leaving>,
    /// The reader answered the question about edits not applied that New
    /// and Clone ask before they open; reset whenever a dialog opens.
    edits_answered: bool,
    /// What a dialog committed asks to happen once the check of the
    /// configuration runs are to use answers.
    after_check: Option<AfterCheck>,
}

/// What follows a commit a dialog asked for, once it lands or is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
enum AfterCheck {
    /// Go on leaving the project; a refusal stays on the Configuration page.
    Leave(Leaving),
    /// Put this configuration in front.
    Select(ConfigurationSelection),
    /// Show the New or Clone dialog the edits were applied for; a refusal
    /// closes it, as the edits are still there.
    ShowDialog,
    /// Close the dialog that committed; a refusal stays in it.
    CloseDialog,
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

/// Which dialog to open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationDialogRequest {
    New,
    Clone(ConfigurationSetId),
    Rename(ConfigurationSetId),
    Delete(ConfigurationSetId),
    Revert(ConfigurationSetId),
}

/// Open the dialog `request` names, filled in from the project.
pub fn open_configuration_dialog(
    dialogs: &mut ConfigurationDialogState,
    model: &Model,
    request: ConfigurationDialogRequest,
) {
    let catalog: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    let dialog = match request {
        ConfigurationDialogRequest::New => ConfigurationDialog::New {
            name: unique_name(catalog, "Configuration"),
            start: StartFrom::Default,
            top: model.workspace.content.simulation_root_reference(),
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
        ConfigurationDialogRequest::Revert(id) => ConfigurationDialog::Revert(id),
    };
    dialogs.open = Some(dialog);
    dialogs.refusal = None;
    dialogs.edits_answered = false;
}

/// What the answer to the question New and Clone ask about edits not
/// applied leaves to do.
#[derive(Debug, PartialEq, Eq)]
pub enum EditsBeforeDialog {
    /// Show the dialog: nothing to ask, or the reader answered.
    ShowDialog,
    /// Keep asking: not answered yet.
    Ask,
    /// Apply the edits, then show the dialog.
    ApplyThenShow,
    /// The reader cancelled: the dialog closes and the edits stay.
    Cancel,
}

/// New and Clone make another configuration and put it in front, which
/// drops the edits not applied to the one there now: ask first, as picking
/// another configuration does. Discard leaves the edits until the new
/// configuration is made, so cancelling the dialog after it loses nothing.
pub fn answer_edits_before_dialog(
    dialogs: &mut ConfigurationDialogState,
    configuration: &ConfigurationPageState,
    catalog: &ConfigurationSetCatalog,
    choice: DialogChoice,
) -> EditsBeforeDialog {
    let asks = matches!(
        dialogs.open,
        Some(ConfigurationDialog::New { .. } | ConfigurationDialog::Clone { .. })
    );
    if !asks || dialogs.edits_answered || !configuration.is_dirty(catalog) {
        return EditsBeforeDialog::ShowDialog;
    }
    match choice {
        DialogChoice::None => EditsBeforeDialog::Ask,
        DialogChoice::Primary => {
            dialogs.edits_answered = true;
            EditsBeforeDialog::ApplyThenShow
        }
        DialogChoice::Secondary => {
            dialogs.edits_answered = true;
            EditsBeforeDialog::ShowDialog
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialogs.close();
            EditsBeforeDialog::Cancel
        }
    }
}

fn name_id() -> Id {
    Id::new("rspice.configuration-dialog.name")
}

/// [`show_configuration_page`] over the parts it reads and writes.
pub fn show_configuration_page_in(
    platform: &PlatformState,
    wb: &mut WorkbenchSession,
    project_page: &mut ProjectPage,
) {
    if !platform.project_lifecycle.is_open() {
        return;
    }
    wb.activate(crate::workbench::state::Workspace::Project);
    *project_page = ProjectPage::Configuration;
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

/// What the configuration dialogs draw over and commit against, lent by the
/// project feature's context for the one call: their own state, the
/// Configuration page and the Project page they bring forward, the model,
/// the session, and the run tier a committed change voids.
pub struct ConfigurationDialogParts<'a> {
    pub dialogs: &'a mut ConfigurationDialogState,
    pub configuration: &'a mut ConfigurationPageState,
    pub project_page: &'a mut ProjectPage,
    pub model: &'a mut Model,
    pub platform: &'a PlatformState,
    pub wb: &'a mut WorkbenchSession,
    pub runs: &'a mut RunsState,
    pub run_controller: &'a mut SimulationController,
}

/// Draw the question about edits not applied, asked before leaving the
/// project or picking another configuration, or the configuration dialog
/// that is open, and commit what the reader chose.
pub fn render_configuration_dialog(parts: &mut ConfigurationDialogParts<'_>, ctx: &Context) {
    // Every frame takes the answer of a check that is running, whichever
    // page is in front; a check an edit stopped takes what was to follow it.
    match finish_configuration_check(
        parts.configuration,
        parts.model,
        parts.runs,
        parts.run_controller,
        ctx,
    ) {
        Some(answer) => {
            let after = parts.dialogs.after_check.take();
            follow(parts, after, answer);
        }
        None if !parts.configuration.is_checking() => parts.dialogs.after_check = None,
        None => {}
    }
    if parts.dialogs.leaving.is_some() {
        let choice =
            edits_prompt(ctx, parts.configuration, parts.model).unwrap_or(DialogChoice::Secondary);
        match answer_leaving(parts.dialogs, parts.configuration, choice) {
            LeavingAnswer::Stay => {}
            LeavingAnswer::GoOn(leaving) => follow(parts, Some(AfterCheck::Leave(leaving)), Ok(())),
            LeavingAnswer::ApplyThenGoOn(leaving) => commit_then(
                parts,
                ConfigurationTransaction::Apply,
                AfterCheck::Leave(leaving),
            ),
        }
        return;
    }
    if let Some((target, choice)) = selection_prompt(ctx, parts.configuration, parts.model) {
        match choice {
            DialogChoice::Primary => {
                parts.configuration.pending_selection = None;
                commit_then(
                    parts,
                    ConfigurationTransaction::Apply,
                    AfterCheck::Select(target),
                );
            }
            DialogChoice::Secondary => parts.configuration.select(target),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                parts.configuration.pending_selection = None;
            }
            DialogChoice::None => {}
        }
        return;
    }
    let catalog = &parts.model.workspace.content.configuration_sets;
    let asked = answer_edits_before_dialog(
        parts.dialogs,
        parts.configuration,
        catalog,
        DialogChoice::None,
    );
    if asked == EditsBeforeDialog::Ask {
        let choice =
            edits_prompt(ctx, parts.configuration, parts.model).unwrap_or(DialogChoice::None);
        let catalog = &parts.model.workspace.content.configuration_sets;
        match answer_edits_before_dialog(parts.dialogs, parts.configuration, catalog, choice) {
            EditsBeforeDialog::ApplyThenShow => commit_then(
                parts,
                ConfigurationTransaction::Apply,
                AfterCheck::ShowDialog,
            ),
            EditsBeforeDialog::Ask | EditsBeforeDialog::ShowDialog | EditsBeforeDialog::Cancel => {}
        }
        return;
    }
    let Some(dialog) = parts.dialogs.open.clone() else {
        return;
    };
    if let ConfigurationDialog::Revert(id) = dialog {
        match revert_dialog(ctx, parts.configuration, parts.model, id) {
            DialogChoice::Primary => {
                parts.configuration.revert();
                parts.dialogs.close();
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => parts.dialogs.close(),
            DialogChoice::Secondary | DialogChoice::None => {}
        }
        return;
    }
    // A dialog commits nothing while a check runs: its commit would land
    // against the catalog the check is about to change.
    let write_allowed =
        !parts.model.safe_mode.project_read_only() && !parts.configuration.is_checking();
    let refusal = parts.dialogs.refusal.clone();
    let (choice, transaction) = match dialog {
        ConfigurationDialog::New { .. } => {
            new_dialog(ctx, parts.dialogs, parts.model, write_allowed, refusal)
        }
        ConfigurationDialog::Clone { .. } | ConfigurationDialog::Rename { .. } => {
            name_dialog(ctx, parts.dialogs, parts.model, write_allowed, refusal)
        }
        ConfigurationDialog::Delete(id) => {
            delete_dialog(ctx, parts.model, id, write_allowed, refusal)
        }
        ConfigurationDialog::Revert(_) => return,
    };
    match choice {
        DialogChoice::Primary => {
            let Some(transaction) = transaction else {
                return;
            };
            commit_then(parts, transaction, AfterCheck::CloseDialog);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            parts.dialogs.close();
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// Commit `transaction` over the parts, as the application's commit does,
/// and do what `after` asks once it lands or is refused: now, or when the
/// check of the configuration runs are to use answers.
fn commit_then(
    parts: &mut ConfigurationDialogParts<'_>,
    transaction: ConfigurationTransaction,
    after: AfterCheck,
) {
    match commit_configuration(
        parts.configuration,
        parts.model,
        parts.runs,
        parts.run_controller,
        transaction,
    ) {
        Ok(Committed::Checking) => parts.dialogs.after_check = Some(after),
        Ok(Committed::Done) => follow(parts, Some(after), Ok(())),
        Err(refusal) => follow(parts, Some(after), Err(refusal)),
    }
}

/// Do what follows a commit that landed or was refused. With nothing to
/// follow, a refusal is the Configuration page's to state.
fn follow(
    parts: &mut ConfigurationDialogParts<'_>,
    after: Option<AfterCheck>,
    answer: Result<(), String>,
) {
    match (after, answer) {
        // Going on leaving asks about the project's unsaved work or carries
        // the action out, over the whole application: the shell does it as
        // this dialog returns.
        (Some(AfterCheck::Leave(Leaving { action, recent })), Ok(())) => parts
            .wb
            .outbox
            .push(ProjectRequest::GoOnLeaving { action, recent }),
        // A refused apply stays, on the page, saying why.
        (Some(AfterCheck::Leave(_)), Err(refusal)) => {
            parts.configuration.refusal = Some(refusal);
            show_configuration_page_in(parts.platform, parts.wb, parts.project_page);
        }
        (Some(AfterCheck::Select(target)), Ok(())) => parts.configuration.select(target),
        (Some(AfterCheck::ShowDialog) | None, Ok(())) => {}
        // The edits are still there, so nothing new is made.
        (Some(AfterCheck::ShowDialog), Err(refusal)) => {
            parts.configuration.refusal = Some(refusal);
            parts.dialogs.close();
        }
        (Some(AfterCheck::CloseDialog), Ok(())) => parts.dialogs.close(),
        (Some(AfterCheck::CloseDialog), Err(refusal)) => parts.dialogs.refusal = Some(refusal),
        (Some(AfterCheck::Select(_)) | None, Err(refusal)) => {
            parts.configuration.refusal = Some(refusal);
        }
    }
}

/// What the answer to the question asked before leaving the project leaves
/// to do.
#[derive(Debug, PartialEq, Eq)]
pub enum LeavingAnswer {
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
pub fn answer_leaving(
    dialogs: &mut ConfigurationDialogState,
    configuration: &mut ConfigurationPageState,
    choice: DialogChoice,
) -> LeavingAnswer {
    if choice == DialogChoice::None {
        return LeavingAnswer::Stay;
    }
    let Some(leaving) = dialogs.leaving.take() else {
        return LeavingAnswer::Stay;
    };
    match choice {
        DialogChoice::Primary => LeavingAnswer::ApplyThenGoOn(leaving),
        DialogChoice::Secondary => {
            configuration.revert();
            LeavingAnswer::GoOn(leaving)
        }
        DialogChoice::Ghost | DialogChoice::Cancelled | DialogChoice::None => LeavingAnswer::Stay,
    }
}

fn new_dialog(
    ctx: &Context,
    dialogs: &mut ConfigurationDialogState,
    model: &Model,
    write_allowed: bool,
    refusal: Option<String>,
) -> (DialogChoice, Option<ConfigurationTransaction>) {
    let choices = top_cells(model);
    let Some(ConfigurationDialog::New {
        name,
        start,
        top,
        use_for_runs,
    }) = dialogs.open.as_mut()
    else {
        return (DialogChoice::None, None);
    };
    // A top cell that places no instance is its own DUT.
    let dut = default_dut_path(&model.workspace, &model.schematic, top)
        .unwrap_or_else(|| InstancePath::root().to_string());
    // One form down one label column in the body face: the lists a start
    // brings are words under the pick that brings them, wrapped whole.
    let choice = CommandForm::new("New configuration", "Create")
        .describe(
            "A configuration picks the top cell, the view list and stop list, and the \
                 bindings netlisting follows.",
        )
        .primary_enabled(write_allowed && !name.trim().is_empty())
        .fields_enabled(write_allowed)
        .status(refusal.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.text_sans("Name", name_id(), name, "Configuration name");
            let labels = StartFrom::ALL.map(|start| start.label().to_owned());
            if let Some(index) = rows.select(
                "Start from",
                "configuration-dialog-start-from",
                start.label(),
                &labels,
            ) {
                *start = StartFrom::ALL[index];
            }
            let (views, stops) = start.lists();
            rows.derived(&format!("View list: {}", views.join(", then ")));
            rows.derived(&format!("Stop list: {}", stops.join(", ")));
            let top_labels = choices
                .iter()
                .map(CellViewRef::display_path)
                .collect::<Vec<_>>();
            if let Some(index) = rows.select(
                "Top cell",
                "configuration-dialog-top-cell",
                &top.display_path(),
                &top_labels,
            ) {
                *top = choices[index].clone();
            }
            rows.value("DUT instance", &dut);
            rows.checks("Runs", &mut [("Use for runs", use_for_runs)]);
            Some(name_id())
        });
    let (executable_view_policy, stop_views) = start.lists();
    let transaction = ConfigurationTransaction::Create {
        definition: Box::new(ConfigurationSetDefinition {
            name: name.trim().to_owned(),
            root: top.clone(),
            dut_path: dut,
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
    };
    (choice, Some(transaction))
}

/// Clone and Rename: a name, and for Clone whether runs use the copy.
fn name_dialog(
    ctx: &Context,
    dialogs: &mut ConfigurationDialogState,
    model: &Model,
    write_allowed: bool,
    refusal: Option<String>,
) -> (DialogChoice, Option<ConfigurationTransaction>) {
    let catalog: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    let (title, primary, source_name) = match dialogs.open.as_ref() {
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
        dialogs.close();
        return (DialogChoice::None, None);
    };
    let description = if primary == "Clone" {
        format!("The copy holds every binding of {source_name}.")
    } else {
        format!("Rename {source_name}. Runs and its bindings are not affected.")
    };
    let mut transaction = None;
    let dialog_state = &mut *dialogs;
    let name_empty = match dialog_state.open.as_ref() {
        Some(
            ConfigurationDialog::Clone { name, .. } | ConfigurationDialog::Rename { name, .. },
        ) => name.trim().is_empty(),
        _ => true,
    };
    let choice = CommandForm::new(title, primary)
        .describe(&description)
        .primary_enabled(write_allowed && !name_empty)
        .fields_enabled(write_allowed)
        .status(refusal.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.value("Configuration", &source_name);
            match dialog_state.open.as_mut() {
                Some(ConfigurationDialog::Clone {
                    source,
                    name,
                    use_for_runs,
                }) => {
                    rows.text_sans("Name", name_id(), name, "Configuration name");
                    rows.checks("Runs", &mut [("Use for runs", use_for_runs)]);
                    transaction = Some(ConfigurationTransaction::Clone {
                        source: *source,
                        name: name.trim().to_owned(),
                        use_for_runs: *use_for_runs,
                    });
                }
                Some(ConfigurationDialog::Rename { id, name }) => {
                    rows.text_sans("Name", name_id(), name, "Configuration name");
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
    configuration: &ConfigurationPageState,
    model: &Model,
) -> Option<(ConfigurationSelection, DialogChoice)> {
    let target = configuration.pending_selection?;
    Some((
        target,
        edits_prompt(ctx, configuration, model).unwrap_or(DialogChoice::Secondary),
    ))
}

/// Apply, discard or keep the edits not applied to the configuration in
/// front. `None` when there are none left to ask about.
fn edits_prompt(
    ctx: &Context,
    configuration: &ConfigurationPageState,
    model: &Model,
) -> Option<DialogChoice> {
    let catalog: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    let draft = configuration.draft()?;
    let name = catalog.find(draft.id).map_or_else(
        || draft.definition.name.clone(),
        |committed| committed.name().to_owned(),
    );
    let title = format!("Apply your edits to {name}?");
    let consequence = format!(
        "{name} has edits that are not applied. Applying commits them to the project; discarding puts {name} back as it was."
    );
    let choice = Dialog::new(&title, "Apply")
        .description("Apply or discard the edits not applied to a configuration.")
        .size(DialogSize::Confirmation)
        .primary_enabled(!model.safe_mode.project_read_only())
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
    model: &Model,
    id: ConfigurationSetId,
    write_allowed: bool,
    refusal: Option<String>,
) -> (DialogChoice, Option<ConfigurationTransaction>) {
    let catalog: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    let Some(configuration) = catalog.find(id) else {
        return (DialogChoice::Cancelled, None);
    };
    let name = configuration.name().to_owned();
    let in_use = used_for_runs(catalog, ConfigurationSelection::Configuration(id));
    let title = format!("Delete {name}?");
    let count = |count: usize, what: &str| {
        format!(
            "{count} {what} binding{}",
            crate::ui::accessibility::plural_suffix(count)
        )
    };
    let consequence = format!(
        "{name}, its {} and its {} leave the project. Runs that used it before are not \
         affected, and Undo brings it back.",
        count(configuration.overrides().len(), "instance"),
        count(configuration.cell_bindings().len(), "cell"),
    );
    let mut dialog = Dialog::new(&title, "Delete")
        .description("Delete a configuration from the project.")
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

/// Drop the edits not applied to `id`, once the reader says so: they are
/// not kept anywhere once dropped.
fn revert_dialog(
    ctx: &Context,
    configuration: &ConfigurationPageState,
    model: &Model,
    id: ConfigurationSetId,
) -> DialogChoice {
    let catalog: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    if !configuration.is_dirty(catalog) {
        return DialogChoice::Cancelled;
    }
    let name = catalog
        .find(id)
        .map_or_else(String::new, |committed| committed.name().to_owned());
    let consequence = format!(
        "The edits not applied to {name} are dropped, and the page shows {name} as the project \
         holds it. This cannot be undone."
    );
    Dialog::new(format!("Revert the edits to {name}?"), "Revert")
        .description("Drop the edits not applied to a configuration.")
        .size(DialogSize::Confirmation)
        .destructive()
        .ghost("Keep editing")
        .show(ctx, |ui| consequence_text(ui, &consequence))
}

/// The schematic and testbench views a new configuration can take as its
/// top cell.
fn top_cells(model: &Model) -> Vec<CellViewRef> {
    let workspace = &model.workspace;
    let mut roots = Vec::new();
    for (library_key, library) in model.library_manager.libraries_by_key() {
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
