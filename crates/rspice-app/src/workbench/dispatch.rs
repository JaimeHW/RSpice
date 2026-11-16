//! The one dispatcher every command goes through, whoever raises it: a menu
//! row, a toolbar button, the palette, a shortcut or another command.
//!
//! A command runs behind one prologue, always in the same order: a project
//! operation in flight holds it back, a command that needs a project needs
//! one open, an active source editor takes the editing commands it handles,
//! a read-only schematic refuses an edit, a pending inspector edit commits
//! first, and a repeatable command is recorded for Repeat. Only then does the
//! command's owner carry it out: the feature that declares it, in its
//! context; for one of the workbench's routed commands, the part the shell's
//! edit target sends it to; or for one of the shell's own commands, the
//! shell ([`ShellCommand::carry_out`]). A command a feature's command
//! raised runs as soon as that one returns, behind the same prologue.
//!
//! The dispatcher is handed a key. A key names a command when the registry
//! lists it, or when a feature declares it and leaves it out of the registry
//! for the one container that holds its key; no menu, shortcut profile or
//! Repeat reaches one of those.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::ShellCommand;
use crate::workbench::commands::{
    active_schematic_editor, answer_project_request, answer_surface_request,
    carry_out_code_hand_off, file_action, open_connectivity_source_map, run_project_file_action,
    select_probed_trace, source_editor_redirect,
};
use crate::workbench::features;
use crate::workbench::menu_bar::FileMenuAction;
use rspice_app_code::workbench::code::CodeFeature;
use rspice_app_code::workbench::code::commands::{
    CodeRequest, NetlistEditorRequest, VerilogAStripRequest,
};
use rspice_app_code::workbench::code::requests as code_requests;
use rspice_app_design::schematic::view::{DesignSurfaceRequest, SelectProbedTrace};
use rspice_app_design::workbench::app::actions::workspace::HistoryDirection;
use rspice_app_design::workbench::app::dialogs::design_request::DesignRequest;
use rspice_app_design::workbench::design::DesignFeature;
use rspice_app_design::workbench::design::inspector::CommitSheetSection;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use rspice_app_hardcopy::workbench::hardcopy;
use rspice_app_models::workbench::models::ModelsFeature;
use rspice_app_project::workbench::project::commands::ProjectRequest;
use rspice_app_results::workbench::results::ResultsFeature;
use rspice_app_simulate::workbench::simulate::SimulateFeature;
use rspice_app_state::state::command_catalog::CommandKey;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::{
    CommandAvailability, CommandFlags, CommandSpec, WorkbenchCommand, ids,
};
use rspice_app_workbench::workbench::framework::feature::{Execute, held_back};
use rspice_app_workbench::workbench::framework::host::{Host, InspectorScope};
use rspice_app_workbench::workbench::framework::requests::{
    self, ApplyWaveformDefaults, ArmPartPlacement, BindComponentModel, ClearSelectedModel,
    CloseForLiveMirror, ConfirmPackAttach, ExportDataset, FollowSchematicGrid, MarkFailureSites,
    OpenInDesign, OpenPropertyEditor, OpenStimulusDefinition, OpenSymbolAuthoring, PrintActiveView,
    ProjectFileAction, PromptAfterCommands, RevealExecutedDeck, RevealModel,
    SelectAnalysisInstance, SetFamilyFilter, ShowProjectModels, ShowSchematicAnnotations,
    ShowWaveforms, VoidVerilogAEvidence,
};

/// Who carries out a command once the prologue has let it through.
#[derive(Debug, Clone, Copy)]
enum Owner {
    /// The feature that declares it, in its context.
    Feature,
    /// The part the shell's edit target sends it to.
    Workbench(WorkbenchCommand),
    /// The shell itself.
    Shell(ShellCommand),
}

/// The dispatcher by key: where a key becomes the command it names, a
/// feature's first, then a routed command's, and the shell's own otherwise.
impl RSpiceApp {
    /// Carry out the command `key` names behind the prologue every command
    /// shares. A key no command answers to does nothing.
    pub(crate) fn execute_command_key(&mut self, key: CommandKey) {
        if let Some(spec) = features::spec(key) {
            self.dispatch(*spec, Owner::Feature);
        } else if let Some(command) = WorkbenchCommand::from_key(key) {
            self.dispatch(command.declared_spec(), Owner::Workbench(command));
        } else if let Some(command) = ShellCommand::from_key(key) {
            self.dispatch(command.declared_spec(), Owner::Shell(command));
        }
    }

    /// Whether the command `key` names can run here, and why not: the
    /// shell's own guards first, then the command's owner, except that a
    /// command its owner does not offer here is hidden before any guard asks.
    /// `Hidden` for a key no command answers to.
    pub(crate) fn command_availability(&self, key: CommandKey) -> CommandAvailability {
        let Some(spec) = features::spec(key) else {
            if let Some(command) = WorkbenchCommand::from_key(key) {
                return self.workbench_command_availability(command);
            }
            return ShellCommand::from_key(key).map_or(CommandAvailability::Hidden, |command| {
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
        if let Some(command) = WorkbenchCommand::from_key(key) {
            return self.workbench_command_enabled(command);
        }
        ShellCommand::from_key(key).is_some_and(|command| command.is_enabled(self))
    }

    /// Whether the state the command `key` names switches is on right now;
    /// `None` for a command that acts once, and for a key no command
    /// answers to.
    pub(crate) fn command_checked(&self, key: CommandKey) -> Option<bool> {
        ShellCommand::from_key(key).map_or_else(
            || features::checked(self, key),
            |command| command.checked(self),
        )
    }

    /// Carry out the routed command `key` in the feature that takes it where
    /// the reader is working, and say whether one did. The shell's own arm
    /// for the command asks it where that feature's turn comes.
    pub(crate) fn route_command_key(&mut self, key: CommandKey) -> bool {
        features::route(self, key)
    }

    /// Carry out the routed command `key` where `F` takes it here, and say
    /// whether it did: the turn of `F` alone, for a command whose precedence
    /// the edit target states owner by owner.
    pub(crate) fn route_command_key_to<F: features::Registered>(
        &mut self,
        key: CommandKey,
    ) -> bool {
        features::route_to::<F>(self, key)
    }

    /// Whether the routed command `key` can run in the feature that takes it
    /// where the reader is working; `None` when no feature takes it there.
    pub(crate) fn routed_command_key_enabled(&self, key: CommandKey) -> Option<bool> {
        features::routed_enabled(self, key)
    }

    /// Carry out the command `spec` declares behind the prologue, in its
    /// `owner`: a routed one, in the part that takes it here; the shell's,
    /// by the shell; a feature's, then every command it raised.
    fn dispatch(&mut self, spec: CommandSpec, owner: Owner) {
        let flags = spec.flags;
        if held_back(flags, &self.state.platform).is_some() {
            return;
        }
        if let Owner::Workbench(command) = owner
            && let Some((request, accepted)) =
                source_editor_redirect(command, &self.state.workbench)
        {
            if accepted {
                self.state.workbench.outbox.push(request);
            }
            return;
        }
        if flags.contains(CommandFlags::EDITS_ACTIVE_DOCUMENT)
            && active_schematic_editor(self)
            && self.state.model.schematic_edit_read_only()
        {
            rspice_app_design::workbench::app::actions::workspace::deny_read_only_edit(
                &self.state.model,
                &mut self.state.workbench,
            );
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
        match owner {
            Owner::Workbench(command) => {
                self.carry_out_workbench_command(command);
                return;
            }
            Owner::Shell(command) => {
                command.carry_out(self);
                return;
            }
            Owner::Feature => {}
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
    /// what the Verilog-A page's interface strip asked of design once its
    /// press had done the code feature's part (the updated symbol to open,
    /// the offered device to arm), which the shell once did as the page
    /// returned; then the commands it ran next and what it said once they
    /// had run, the print workflow it asked for, the snap that follows a new schematic grid, the
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
            |app, request: VerilogAStripRequest| match request {
                VerilogAStripRequest::OpenUpdatedSymbol { symbol, pins } => {
                    rspice_app_design::workbench::design::DesignFeature::open_updated_veriloga_symbol(
                        &mut features::cx(app),
                        symbol,
                        pins,
                    );
                }
                VerilogAStripRequest::Place {
                    target,
                    binding,
                    ctx,
                } => {
                    rspice_app_design::workbench::design::DesignFeature::place_veriloga_device(
                        &mut features::cx(app),
                        target,
                        *binding,
                        &ctx,
                    );
                }
            },
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
            |app, FollowSchematicGrid| DesignFeature::follow_schematic_grid(&mut features::cx(app)),
        );
        self.state.take_shortcut_resolver_reset();
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ApplyWaveformDefaults| {
                ResultsFeature::apply_waveform_defaults(&mut features::cx(app));
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
                SimulateFeature::select_analysis_instance(&mut features::cx(app), instance);
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
                    rspice_app_simulate::workbench::simulate::close_jobs_manager(
                        &mut app.state.workbench,
                    );
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
                app.state.open_property_editor(component_id);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, OpenStimulusDefinition(definition)| {
                DesignFeature::show_stimulus_definition(&mut features::cx(app), &definition);
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
                ResultsFeature::show_waveforms(&mut features::cx(app), visibility, selected);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, SetFamilyFilter(filter)| {
                ResultsFeature::set_family_filter(&mut features::cx(app), filter);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ShowSchematicAnnotations(annotations)| {
                DesignFeature::show_schematic_annotations(&mut features::cx(app), annotations);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, MarkFailureSites(run_sequence)| {
                if rspice_app_design::workbench::app::actions::workspace::highlight_active_failure_sites(
                    &mut app.state.model,
                    &mut app.state.workbench,
                ) {
                    ResultsFeature::mark_failure_run(&mut features::cx(app), run_sequence);
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
                rspice_app_state::workbench::app::schematic::placement::arm_pack_part(
                    &mut app.state.model,
                    placement,
                );
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
                    rspice_app_design::workbench::design::inspector::bind_component_model_from_catalog(
                        &mut features::cx(app),
                        component_id,
                        &library,
                        &model,
                    )
                    .map(|()| format!("Bound selected instance to model '{library} / {model}'."));
                ModelsFeature::note_receipt(&mut features::cx(app), result);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: OpenSymbolAuthoring| {
                if let Some(result) =
                    DesignFeature::open_symbol_authoring(&mut features::cx(app), request)
                {
                    ModelsFeature::note_receipt(&mut features::cx(app), result);
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
                    .activate(rspice_app_state::workbench::workspace::Workspace::Design);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ShowProjectModels| {
                ModelsFeature::show_project_models(&mut features::cx(app));
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
                if rspice_app_models::workbench::models::ModelsFeature::confirm_pack_attach(
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
                rspice_app_models::workbench::models::ModelsFeature::reveal_model(
                    &mut features::cx(app),
                    &library,
                    model,
                );
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ClearSelectedModel| ModelsFeature::clear_selected_model(&mut features::cx(app)),
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
    if rspice_app_code::workbench::documents::netlist_document::reveal_executed_deck(
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
        rspice_app_state::state::absent_deck_reason()
    ));
    false
}

/// Have the code feature carry out what the netlist page's editor raised,
/// in its context.
fn answer_netlist_editor_request(app: &mut RSpiceApp, request: NetlistEditorRequest) {
    code_requests::answer_netlist_editor(&mut features::cx(app), request);
}

/// Have the code feature drop what the last Verilog-A compile proved, in its
/// context.
fn answer_void_veriloga_evidence(app: &mut RSpiceApp) {
    code_requests::void_veriloga_evidence(&mut features::cx(app));
}

/// Have the code feature answer what it raised, in its context, holding
/// whether an application-modal dialog is open, taken as the answer starts.
/// What reaches past the feature the shell carries out straight after.
fn answer_code_request(app: &mut RSpiceApp, request: CodeRequest) {
    let modal_open = app.state.application_modal_open();
    let hand_off = code_requests::answer(
        &mut features::cx::<CodeFeature>(app).with_application_modal(modal_open),
        request,
    );
    if let Some(hand_off) = hand_off {
        carry_out_code_hand_off(app, hand_off);
    }
}

impl RSpiceApp {
    /// Carry out what a design dialog handed over, as the dialog once did in
    /// place: in design's context, except reopening the component editor,
    /// which the shell's property form opens, and the source map, which
    /// opens the code feature's netlist page.
    pub(crate) fn answer_design_request(&mut self, request: DesignRequest) {
        match request {
            DesignRequest::Descend(choice) => {
                DesignFeature::descend(&mut features::cx(self), choice);
            }
            DesignRequest::ReopenPropertyEditor(component_id) => {
                self.state.open_property_editor(component_id);
            }
            DesignRequest::RevealConnectivity(target) => {
                DesignFeature::reveal_connectivity(&mut features::cx(self), target);
            }
            DesignRequest::OpenSourceMap => open_connectivity_source_map(&mut self.state),
            DesignRequest::ApplyConnectivityRepairs => {
                DesignFeature::apply_connectivity_repairs(&mut features::cx(self));
            }
            DesignRequest::CreateHierarchy => {
                DesignFeature::create_hierarchy(&mut features::cx(self));
            }
        }
    }

    /// Have the code feature take the netlist page's own history step
    /// `direction`, in its context.
    pub(in crate::workbench) fn take_netlist_history_step(
        &mut self,
        direction: HistoryDirection,
    ) -> Result<Option<String>, String> {
        code_requests::step_history(&mut features::cx(self), direction == HistoryDirection::Undo)
    }
}

#[cfg(test)]
mod tests {
    use crate::workbench::commands::vocabulary::{CommandRegistry, ShellCommand};
    use rspice_app_design::workbench::design::commands::DesignCommand;
    use rspice_app_state::state::command_catalog::CommandCatalog;

    /// Create pins and the seven kinds the registry lists are the design
    /// feature's, each under its own id, and none is a shell command; the id
    /// the shell once answered for a kind it did not list names nothing.
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
            assert_eq!(ShellCommand::from_stable_id(key.as_str()), None, "{key:?}");
        }
        assert_eq!(CommandRegistry.resolve("place-component"), None);
    }

    /// Run on the Netlist workspace runs the deck the page shows, and only
    /// that: the shell's arm answers there ahead of simulate's, which would
    /// queue the active plan instead. Everywhere else Run is simulate's.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn run_on_the_netlist_workspace_runs_the_deck_and_not_the_plan() {
        use crate::workbench::RSpiceApp;
        use crate::workbench::features::cx;
        use rspice_app_code::workbench::code::netlist_validation::validate_visible_netlist_source;
        use rspice_app_state::state::SimulationRunIntent;
        use rspice_app_state::workbench::workspace::Workspace;
        use rspice_app_workbench::workbench::framework::command_registry::{
            CommandAvailability, ids,
        };

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
        assert!(validate_visible_netlist_source(&mut cx(&mut app)));
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
