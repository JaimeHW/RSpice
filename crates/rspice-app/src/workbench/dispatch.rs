//! The one dispatcher every command goes through, whoever raises it: a menu
//! row, a toolbar button, the palette, a shortcut or another command.
//!
//! A command runs behind one prologue, always in the same order: a project
//! operation in flight holds it back, a command that needs a project needs
//! one open, an active source editor takes the editing commands it handles,
//! a read-only schematic refuses an edit, a pending inspector edit commits
//! first, and a repeatable command is recorded for Repeat. Only then does the
//! command's owner carry it out: the feature that declares it, in its
//! context, or for a key no feature declares the shell's legacy table
//! [`Command::run_legacy_arm`]. A command a feature's command raised runs as
//! soon as that one returns, behind the same prologue.
//!
//! The dispatcher is handed a command value or a key. A key names a command
//! when the registry lists it, or when a feature declares it and leaves it
//! out of the registry for the one container that holds its key; no menu,
//! shortcut profile or Repeat reaches one of those.

use crate::schematic::view::{DesignSurfaceRequest, SelectProbedTrace};
use crate::state::command_catalog::CommandKey;
use crate::workbench::app::HardcopyWorkflow;
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app_state::PlatformState;
use crate::workbench::code::commands::{CodeRequest, NetlistEditorRequest};
#[cfg(test)]
use crate::workbench::commands::dispatch_probe;
use crate::workbench::commands::vocabulary::{Command, legacy_command};
use crate::workbench::commands::{
    active_schematic_editor, answer_code_request, answer_netlist_editor_request,
    answer_project_request, answer_surface_request, answer_void_veriloga_evidence,
    command_edits_schematic, file_action, run_project_file_action, select_probed_trace,
};
use crate::workbench::design::inspector::CommitSheetSection;
use crate::workbench::features;
use crate::workbench::framework::command_registry::{
    CommandAvailability, CommandFlags, CommandSpec, ids,
};
use crate::workbench::framework::feature::{Execute, held_back};
use crate::workbench::framework::host::{Host, InspectorScope};
use crate::workbench::framework::requests::{
    self, ApplyWaveformDefaults, ArmPartPlacement, BindComponentModel, ClearSelectedModel,
    CloseForLiveMirror, ConfirmPackAttach, ExportDataset, FollowSchematicGrid, MarkFailureSites,
    OpenInDesign, OpenPropertyEditor, OpenStimulusDefinition, OpenSymbolAuthoring, PrintActiveView,
    ProjectFileAction, PromptAfterCommands, RevealExecutedDeck, RevealModel,
    SelectAnalysisInstance, SetFamilyFilter, ShowProjectModels, ShowSchematicAnnotations,
    ShowWaveforms, SourceEditorRequest, StageComponentTuning, SubcircuitSymbolSource,
    VoidVerilogAEvidence,
};
use crate::workbench::hardcopy;
use crate::workbench::menu_bar::FileMenuAction;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::state::Workspace;
use crate::workbench::{RSpiceApp, WorkbenchSession};

/// The dispatcher by key: where a key becomes the command it names, a
/// feature's first and the shell's legacy table's otherwise.
impl RSpiceApp {
    /// Carry out the command `key` names behind the prologue every command
    /// shares. A key no command answers to does nothing.
    pub(crate) fn execute_command_key(&mut self, key: CommandKey) {
        if let Some(spec) = features::spec(key) {
            self.dispatch_feature_command(spec);
        } else if let Some(command) = legacy_command(key) {
            command.dispatch(self);
        }
    }

    /// Whether the command `key` names can run here, and why not: the
    /// shell's own guards first, then the command's owner, except that a
    /// command its owner does not offer here is hidden before any guard asks.
    /// `Hidden` for a key no command answers to.
    pub(crate) fn command_availability(&self, key: CommandKey) -> CommandAvailability {
        let Some(spec) = features::spec(key) else {
            return legacy_command(key).map_or(CommandAvailability::Hidden, |command| {
                command.availability(self)
            });
        };
        if !features::offered(self, key) {
            return CommandAvailability::Hidden;
        }
        if let Some(reason) = held_back(spec.flags, &self.state.platform) {
            return CommandAvailability::Disabled(reason);
        }
        if self.netlist_takes_run(key) {
            return self.netlist_run_availability();
        }
        features::availability(self, key).unwrap_or(CommandAvailability::Hidden)
    }

    /// Whether the command `key` names is available here, as the shortcut
    /// resolver asks it before a chord may fire.
    pub(crate) fn command_key_available(&self, key: CommandKey) -> bool {
        self.command_availability(key).is_available()
    }

    /// Whether the command `key` names would run if raised now. A feature's
    /// command runs exactly when it is available: no source editor takes
    /// one in its place.
    pub(crate) fn command_key_enabled(&self, key: CommandKey) -> bool {
        if features::spec(key).is_some() {
            return self.command_availability(key).is_available();
        }
        legacy_command(key).is_some_and(|command| command.is_enabled(self))
    }

    /// Carry out the routed command `key` in the feature that takes it where
    /// the reader is working, and say whether one did. The shell's own arm
    /// for the command asks it where that feature's turn comes.
    pub(crate) fn route_command_key(&mut self, key: CommandKey) -> bool {
        features::route(self, key)
    }

    /// Whether the routed command `key` can run in the feature that takes it
    /// where the reader is working; `None` when no feature takes it there.
    pub(crate) fn routed_command_key_enabled(&self, key: CommandKey) -> Option<bool> {
        features::routed_enabled(self, key)
    }

    /// Carry out the feature command `spec` declares behind the same
    /// prologue as [`Command::dispatch`], then every command it raised.
    fn dispatch_feature_command(&mut self, spec: &CommandSpec) {
        let flags = spec.flags;
        if held_back(flags, &self.state.platform).is_some() {
            return;
        }
        if flags.contains(CommandFlags::EDITS_ACTIVE_DOCUMENT)
            && active_schematic_editor(self)
            && self.state.schematic_edit_read_only()
        {
            self.state.deny_read_only_edit();
            return;
        }
        if flags.contains(CommandFlags::COMMITS_INSPECTOR_EDIT)
            && !self.state.commit_pending_inspector_edit()
        {
            return;
        }
        if flags.contains(CommandFlags::REPEATABLE) && self.command_key_enabled(spec.key) {
            self.state.workbench.last_repeatable_command = Some(spec.key.as_str().to_owned());
        }
        if self.netlist_takes_run(spec.key) {
            if self.manual_deck_run_block_reason().is_none() {
                self.state.request_netlist_manual_deck_run();
            }
        } else {
            features::execute(self, spec.key);
        }
        self.after_contribution();
    }

    /// Whether `key` is Run on the Netlist workspace, which runs the deck
    /// the page shows rather than the active plan. The shell answers it
    /// there, ahead of simulate.
    // Deleted when code registers its own handler for Run on the Netlist
    // workspace.
    fn netlist_takes_run(&self, key: CommandKey) -> bool {
        key == ids::RUN_SIMULATION && self.state.workbench.workspace == Workspace::Netlist
    }

    /// Whether Run on the Netlist workspace can run the deck there, and why
    /// not: a run in progress first, then whatever the deck still lacks.
    fn netlist_run_availability(&self) -> CommandAvailability {
        if self.manual_deck_run_block_reason().is_none() {
            return CommandAvailability::Available;
        }
        let execution = &self.state.model.simulation.execution;
        CommandAvailability::Disabled(
            if execution.has_active_execution() || execution.trigger_simulation {
                "a run is already in progress"
            } else {
                "the netlist on this page cannot run yet"
            },
        )
    }

    /// Open the hardcopy feature's `workflow` on the active view, in its
    /// context, for a request another part raised: a result document's
    /// figure export, or the print another feature offers as its dialog's
    /// next step.
    pub(in crate::workbench) fn open_requested_hardcopy_workflow(
        &mut self,
        workflow: HardcopyWorkflow,
    ) {
        hardcopy::open_workflow(&mut features::cx(self), workflow);
    }

    /// Answer what a feature's call raised for straight after it: first the
    /// sheet section's typed values, which a design inspector pass that
    /// shows something else commits; then the netlist editor's commit, undo
    /// and redo, which the editor once made
    /// while the page drew, so everything else the frame raised is answered
    /// over the committed deck; then what the design surface asked of
    /// another feature's workspace (the probed trace; the operating-point
    /// hop, the specification editor, the Verilog-A device form, the
    /// model-bound symbol, the generated deck at an instance's card and the
    /// copy-cell dialog), which the surface once did in place, so every
    /// later answer reads those workspaces as the surface left them; then
    /// the commands it ran next and what it said once they had run, the print
    /// workflow it asked for, the snap that follows a new schematic grid, the
    /// shortcut resolver's reset when a shortcut library was put in use, the
    /// open waveform view starting again from changed defaults, the dataset
    /// export, the analysis instance a results command asked for, the
    /// executed deck a results command or the run history asked for, the
    /// waveforms and the family member a hand-off into Results
    /// opens its viewer or form on, the annotations a result viewer shows on
    /// the schematic it locates something on, and the failure sites the
    /// results failure banner marks there, the close that makes room for a
    /// live session's mirror, the selected model and the Verilog-A compile
    /// evidence a history step left standing on nothing, what a code command
    /// still needs of the whole application, the descent Descend into
    /// hierarchy checked, and last what the project feature hands over and
    /// the project's file commands.
    pub(crate) fn after_contribution(&mut self) {
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, CommitSheetSection(ctx)| {
                app.commit_inspector_section(InspectorScope::Document(Workspace::Design), &ctx);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: NetlistEditorRequest| answer_netlist_editor_request(app, request),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, probed: SelectProbedTrace| select_probed_trace(&mut app.state, probed),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: DesignSurfaceRequest| answer_surface_request(&mut app.state, request),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, Execute(key)| app.execute_command_key(key),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, PromptAfterCommands(text)| app.state.workbench.prompt(text),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, PrintActiveView| {
                app.open_requested_hardcopy_workflow(HardcopyWorkflow::Print);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, FollowSchematicGrid| {
                let state = &mut app.state;
                state.design.follow_schematic_grid(&state.model);
            },
        );
        self.state.take_shortcut_resolver_reset();
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ApplyWaveformDefaults| {
                let state = &mut app.state;
                state
                    .results
                    .start_waveform_viewer_session(&state.model.preferences);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ExportDataset| file_action(app, FileMenuAction::ExportCsvWaveforms),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, SelectAnalysisInstance(instance)| {
                crate::workbench::simulate::select_analysis_instance(
                    &mut app.state.simulate,
                    instance,
                );
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             RevealExecutedDeck {
                 seq,
                 close_manager_on_success,
             }| {
                if open_task_deck(&mut app.state, seq) && close_manager_on_success {
                    crate::workbench::simulate::close_jobs_manager(&mut app.state.workbench);
                }
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             OpenPropertyEditor {
                 component_id,
                 occurrence,
             }| {
                crate::workbench::app::actions::reveal::placed_instance(
                    &mut app.state,
                    occurrence.as_ref(),
                    component_id,
                );
                crate::workbench::app::open_property_editor(&mut app.state, component_id);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, OpenStimulusDefinition(definition)| {
                crate::workbench::app::show_stimulus_definition(
                    &mut app.state.design.selected_stimulus_definition,
                    &mut app.state.workbench,
                    &definition,
                );
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, StageComponentTuning { component_id }| {
                if crate::workbench::simulate::SimulateFeature::stage_component_tuning(
                    &mut features::cx(app),
                    component_id,
                ) {
                    app.execute_command_key(ids::TUNING);
                }
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             ShowWaveforms {
                 visibility,
                 selected,
             }| {
                let results = &mut app.state.results.results;
                results.reveal_waveforms(visibility);
                results.session.selected_trace = selected;
                results.session.clear_cursors();
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, SetFamilyFilter(filter)| {
                crate::workbench::documents::result_document::set_family_filter(
                    &mut app.state.results.result_document_dialog,
                    filter,
                );
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ShowSchematicAnnotations(annotations)| {
                app.state.design.schematic_visibility.annotations = annotations;
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, MarkFailureSites(run_sequence)| {
                if app.state.highlight_active_failure_sites() {
                    app.state.results.results.marked_failure_run = run_sequence;
                }
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, CloseForLiveMirror| app.close_for_live_mirror(),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ArmPartPlacement(placement)| {
                crate::workbench::app::arm_pack_part(&mut app.state.model, placement);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             BindComponentModel {
                 component_id,
                 library,
                 model,
             }| {
                let result =
                    crate::workbench::design::inspector::bind_component_model_from_catalog(
                        &mut features::cx(app),
                        component_id,
                        &library,
                        &model,
                    )
                    .map(|()| format!("Bound selected instance to model '{library} / {model}'."));
                app.state.models.models_view.note_receipt(result);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: OpenSymbolAuthoring| match request {
                OpenSymbolAuthoring::ModelBound => {
                    crate::workbench::app::open_create_model_bound_symbol_dialog(
                        &mut app.state.design.create_model_bound_symbol,
                        &app.state.model,
                        &mut app.state.workbench,
                        app.state.models.selected_model.as_deref(),
                    );
                }
                OpenSymbolAuthoring::ParameterForm(None) => {
                    crate::workbench::app::open_symbol_parameter_form_dialog(
                        &mut app.state.design.symbol_parameter_form,
                        &app.state.model,
                        &mut app.state.workbench,
                        &app.state.platform,
                    );
                }
                OpenSymbolAuthoring::ParameterForm(Some(target)) => {
                    crate::workbench::app::open_symbol_parameter_form_dialog_for(
                        &mut app.state.design.symbol_parameter_form,
                        &app.state.model,
                        &mut app.state.workbench,
                        &app.state.platform,
                        target,
                    );
                }
                OpenSymbolAuthoring::Import(None) => {
                    crate::workbench::app::open_symbol_import_dialog(
                        &mut app.state.design.symbol_import,
                        &app.state.model,
                        &mut app.state.workbench,
                        &app.state.platform,
                    );
                }
                OpenSymbolAuthoring::Import(Some(binding_source)) => {
                    crate::workbench::app::open_symbol_import_dialog_for(
                        &mut app.state.design.symbol_import,
                        &app.state.model,
                        &mut app.state.workbench,
                        &app.state.platform,
                        Some(&binding_source),
                    );
                }
                OpenSymbolAuthoring::SubcircuitBound(source) => {
                    let SubcircuitSymbolSource {
                        library,
                        subcircuit,
                        source_path,
                        ordered_ports,
                        description,
                        section,
                        parameter_defaults,
                        opened_receipt,
                    } = *source;
                    let result = crate::workbench::app::open_create_subcircuit_bound_symbol_dialog(
                        &mut app.state.design.create_model_bound_symbol,
                        &app.state.model,
                        library,
                        subcircuit,
                        source_path,
                        ordered_ports,
                        description,
                        section,
                        parameter_defaults,
                    )
                    .map(|()| opened_receipt);
                    app.state.models.models_view.note_receipt(result);
                }
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, OpenInDesign(reference)| {
                app.state.open_workspace_view(reference);
                app.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ShowProjectModels| {
                app.state.models.models_view.catalog_scope =
                    crate::workbench::models::workspace_state::ModelsCatalogScope::Project;
                app.execute_command_key(ids::MODELS_CATALOG);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             ConfirmPackAttach {
                 part_id,
                 pack_id,
                 version,
             }| {
                if crate::workbench::models::ModelsFeature::confirm_pack_attach(
                    &mut features::cx(app),
                    part_id,
                    pack_id,
                    version,
                ) {
                    app.execute_command_key(ids::MODELS_CATALOG);
                }
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, RevealModel { library, model }| {
                crate::workbench::models::ModelsFeature::reveal_model(
                    &mut features::cx(app),
                    &library,
                    model,
                );
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ClearSelectedModel| app.state.models.clear_selected_model(),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, VoidVerilogAEvidence| answer_void_veriloga_evidence(app),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: CodeRequest| answer_code_request(app, request),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: DesignRequest| app.answer_design_request(request),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: ProjectRequest| answer_project_request(app, request),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, action: ProjectFileAction| run_project_file_action(app, action),
        );
    }
}

/// Open the source one run's engine was actually handed, or say why not.
///
/// The owner for every route to it. The Jobs manager's row reached
/// `netlist_document::reveal_executed_deck` and spelled its own refusal beside
/// it; a second caller spelling a second refusal for the same missing deck is
/// how one absence acquires two names.
pub(crate) fn open_task_deck(state: &mut crate::workbench::AppState, sequence: u64) -> bool {
    if crate::workbench::documents::netlist_document::reveal_executed_deck(
        &mut state.model,
        &mut state.code.netlist,
        &state.runs,
        &mut state.workbench,
        sequence,
        0,
    ) {
        return true;
    }
    state.prompt(format!(
        "The source Run {sequence} executed cannot be opened: {}.",
        crate::state::absent_deck_reason()
    ));
    false
}

impl Command {
    /// Carry this command out behind the prologue every command shares.
    pub(super) fn dispatch(self, app: &mut RSpiceApp) {
        #[cfg(test)]
        if dispatch_probe::intercept(self) {
            return;
        }
        // Both guards are also `is_enabled`'s, so the command is drawn
        // disabled and a shortcut for it is silent, as in any desktop app.
        if self.held_by_project_operation(&app.state.platform) {
            return;
        }
        if self.lacks_open_project(&app.state.platform) {
            return;
        }
        if let Some((request, accepted)) = self.source_editor_redirect(&app.state.workbench) {
            if accepted {
                app.state.workbench.outbox.push(request);
            }
            return;
        }
        if active_schematic_editor(app)
            && command_edits_schematic(self)
            && app.state.schematic_edit_read_only()
        {
            app.state.deny_read_only_edit();
            return;
        }
        if self.commits_inspector_edit() && !app.state.commit_pending_inspector_edit() {
            return;
        }
        if self.repeatable() && self.is_enabled(app) {
            app.state.workbench.last_repeatable_command = Some(self.stable_id().to_owned());
        }
        self.run_legacy_arm(app);
    }

    /// Whether a project operation still in progress holds this command
    /// back. Availability asks the same question, so a command the
    /// dispatcher would refuse is drawn disabled.
    pub(crate) fn held_by_project_operation(self, platform: &PlatformState) -> bool {
        crate::workbench::lifecycle::project_lifecycle::operation_in_progress(platform)
            && self.blocked_by_project_operation()
    }

    /// Whether this command needs an open project and none is open.
    pub(crate) fn lacks_open_project(self, platform: &PlatformState) -> bool {
        self.requires_open_project() && !platform.project_lifecycle.is_open()
    }

    /// The request the active source editor takes in this command's place,
    /// with whether it accepts it: a read-only editor refuses a request that
    /// would change its text. `None` when no source editor takes the command.
    pub(crate) fn source_editor_redirect(
        self,
        workbench: &WorkbenchSession,
    ) -> Option<(SourceEditorRequest, bool)> {
        let request = self.source_editor_request()?;
        let editor = workbench.active_source_editor?;
        Some((request, editor.editable || !request.mutates_source()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::state::command_catalog::CommandCatalog;
    use crate::workbench::commands::vocabulary::{COMMAND_REGISTRY, Command, CommandRegistry};
    use crate::workbench::design::commands::DesignCommand;

    fn variant(command: Command) -> String {
        let name = format!("{command:?}");
        name.split('(').next().unwrap_or_default().to_owned()
    }

    /// Every variant `enum Command` declares, read from its source, so a
    /// variant added there is seen here without being listed anywhere.
    fn declared_variants() -> BTreeSet<String> {
        const VOCABULARY: &str = include_str!("commands/vocabulary.rs");
        let body = VOCABULARY
            .split_once("pub enum Command {")
            .and_then(|(_, rest)| rest.split_once("\n}"))
            .map(|(body, _)| body)
            .expect("vocabulary.rs declares `pub enum Command`");
        body.lines()
            .filter_map(|line| line.strip_prefix("    "))
            .filter(|line| line.starts_with(|c: char| c.is_ascii_uppercase()))
            .filter_map(|line| line.split(|c: char| !c.is_ascii_alphanumeric()).next())
            .map(str::to_owned)
            .collect()
    }

    /// A command the registry leaves out is a feature's, declared in its
    /// command set; the legacy enum lists every command it declares.
    #[test]
    fn the_registry_lists_every_command_the_enum_declares() {
        let declared = declared_variants();
        // The figure is the legacy commands COMMAND_REGISTRY lists at run
        // time. The parse finds exactly that many, so it stays exact as the
        // features take the enum's commands, and a parse that broke cannot
        // pass for an enum that shrank.
        let registered: BTreeSet<String> = COMMAND_REGISTRY
            .iter()
            .map(|command| variant(*command))
            .collect();
        assert!(
            !registered.is_empty(),
            "COMMAND_REGISTRY lists no legacy command: `enum Command` goes, and this test with it"
        );
        assert_eq!(
            declared.len(),
            registered.len(),
            "`enum Command` parsed as {} variants; COMMAND_REGISTRY lists {} at run time",
            declared.len(),
            registered.len()
        );
        assert_eq!(
            registered, declared,
            "a new command goes in COMMAND_REGISTRY"
        );
    }

    /// Create pins and the seven kinds the registry lists are the design
    /// feature's, each under its own id; the id the legacy table answered for
    /// a kind it did not list names nothing.
    #[test]
    fn the_listed_placements_are_the_design_features_and_place_component_names_nothing() {
        for command in [
            DesignCommand::PlacePin,
            DesignCommand::PlaceResistor,
            DesignCommand::PlaceCapacitor,
            DesignCommand::PlaceInductor,
            DesignCommand::PlaceDiode,
            DesignCommand::PlaceGround,
            DesignCommand::PlaceVoltageSource,
            DesignCommand::PlaceCurrentSource,
        ] {
            let key = command.key();
            assert_eq!(CommandRegistry.resolve(key.as_str()), Some(key));
            assert_eq!(DesignCommand::from_key(key), Some(command));
            assert_eq!(Command::from_stable_id(key.as_str()), None, "{key:?}");
        }
        assert_eq!(CommandRegistry.resolve("place-component"), None);
    }

    /// Run on the Netlist workspace runs the deck the page shows, and only
    /// that: the shell's arm answers there ahead of simulate's, which would
    /// queue the active plan instead. Everywhere else Run is simulate's.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn run_on_the_netlist_workspace_runs_the_deck_and_not_the_plan() {
        use crate::state::SimulationRunIntent;
        use crate::workbench::RSpiceApp;
        use crate::workbench::commands::netlist_validation::validate_visible_netlist_source;
        use crate::workbench::framework::command_registry::{CommandAvailability, ids};
        use crate::workbench::state::Workspace;

        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .clear();
        app.state.workbench.workspace = Workspace::Netlist;
        assert_eq!(
            app.command_availability(ids::RUN_SIMULATION),
            CommandAvailability::Disabled("the netlist on this page cannot run yet")
        );
        app.execute_command_key(ids::RUN_SIMULATION);
        assert!(!app.state.model.simulation.execution.trigger_simulation);
        assert!(!app.state.runs.preflight.take_run_and_queue_request());

        assert!(app.state.apply_imported_netlist(
            "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
            None,
            "standalone.cir"
        ));
        assert!(validate_visible_netlist_source(&mut app));
        app.state.workbench.workspace = Workspace::Netlist;
        assert_eq!(
            app.command_availability(ids::RUN_SIMULATION),
            CommandAvailability::Available
        );
        app.execute_command_key(ids::RUN_SIMULATION);
        assert_eq!(
            app.state.model.simulation.execution.run_intent,
            SimulationRunIntent::ManualDeck
        );
        assert!(app.state.model.simulation.execution.trigger_simulation);
        assert!(
            !app.state.runs.preflight.take_run_and_queue_request(),
            "simulate's arm stays out of the Netlist workspace"
        );

        app.state.model.simulation.execution.trigger_simulation = false;
        app.state.workbench.workspace = Workspace::Simulate;
        app.execute_command_key(ids::RUN_SIMULATION);
        assert!(app.state.runs.preflight.take_run_and_queue_request());
    }
}
