//! The one dispatcher every command goes through, whoever raises it: a menu
//! row, a toolbar button, the palette, a shortcut or another command.
//!
//! A command runs behind one prologue, always in the same order: a project
//! operation in flight holds it back, a command that needs a project needs
//! one open, an active source editor takes the editing commands it handles,
//! a read-only schematic refuses an edit, a feature's command that cannot
//! run here is not run (its menu row is off and says why), a pending
//! inspector edit commits first, and a repeatable command is recorded for
//! Repeat. Only then does the command's owner carry it out: the feature that
//! declares it, in its context; for one of the workbench's routed commands,
//! the part the shell's edit target sends it to; or for one of the shell's
//! own commands, the shell ([`ShellCommand::carry_out`]). A command a
//! feature's command raised runs as soon as that one returns, behind the
//! same prologue.
//!
//! A command acts on a source editor's text as typed before it. The focused
//! editor applies a frame's keys as the page draws, after the keys' own
//! command and the toolbar have been read; so a command raised while that
//! editor still has keys of the frame to apply waits until the page has
//! drawn them. Run then runs, and Save writes, the text the reader typed
//! just before pressing it, in a browser that hands the page the typing and
//! the key in one frame too. The editor's own commands (Undo, Select all,
//! Toggle comment and the like) do not wait: the editor takes them as it
//! draws those keys, as it always has; nor does the palette, which takes
//! the keystroke that opened it.
//!
//! The dispatcher is handed a key. A key names a command when the registry
//! lists it, or when a feature declares it and leaves it out of the registry
//! for the one container that holds its key; no menu, shortcut profile or
//! Repeat reaches one of those.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::ShellCommand;
use crate::workbench::commands::{
    active_schematic_editor, answer_project_request, answer_surface_request,
    carry_out_code_hand_off, file_action, open_connectivity_source_map, plot_probed_difference,
    plot_probed_trace, run_project_file_action, show_signal_in_its_occurrence,
    source_editor_redirect,
};
use crate::workbench::diagnostic_routes::open_in_verify;
use crate::workbench::features;
use crate::workbench::menu_bar::FileMenuAction;
use rspice_app_code::workbench::code::SourceFeature;
use rspice_app_code::workbench::code::commands::{
    NetlistEditorRequest, SourceCommand, SourceRequest, VerilogAStripRequest,
};
use rspice_app_code::workbench::code::requests as code_requests;
use rspice_app_design::schematic::view::{
    DesignSurfaceRequest, PlotProbedDifference, PlotProbedTrace,
};
use rspice_app_design::workbench::app::actions::workspace::HistoryDirection;
use rspice_app_design::workbench::app::dialogs::design_request::DesignRequest;
use rspice_app_design::workbench::design::DesignFeature;
use rspice_app_design::workbench::design::inspector::CommitSheetSection;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use rspice_app_hardcopy::workbench::hardcopy;
use rspice_app_models::workbench::models::ModelsFeature;
use rspice_app_project::workbench::project::ProjectFeature;
use rspice_app_project::workbench::project::commands::ProjectRequest;
use rspice_app_results::workbench::commands::schematic_cross_probe::ShowSignalOnSchematic;
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
    self, ApplyPathVariables, ApplyWaveformDefaults, ArmPartPlacement, AskNewSheetFormat,
    BindComponentModel, CellHousekeeping, ClearSelectedModel, CloseForLiveMirror, ExportDataset,
    FollowSchematicGrid, MarkFailureSites, OpenInDesign, OpenInVerify, OpenPropertyEditor,
    OpenStimulusDefinition, OpenSymbolAuthoring, PrintActiveView, ProjectFileAction,
    PromptAfterCommands, RevealExecutedDeck, RevealModel, RunPlanAnyway, SelectAnalysisInstance,
    SetFamilyFilter, ShowOnSchematic, ShowProjectDeck, ShowSchematicAnnotations, ShowWaveforms,
    VoidVerilogAEvidence,
};

/// Whether the command `key` names runs at once while the focused source
/// editor has keys of the frame to apply: the palette, which takes the
/// keystroke that opened it, and a command the editor carries out itself as
/// it draws those keys.
fn runs_among_editor_keys(key: CommandKey) -> bool {
    key == ShellCommand::CommandPalette.key()
        || WorkbenchCommand::from_key(key)
            .is_some_and(|command| command.source_editor_request().is_some())
        || SourceCommand::from_key(key)
            .is_some_and(|command| command.source_editor_request().is_some())
}

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
        if self.state.shell.editor_keys_pending && !runs_among_editor_keys(key) {
            self.state.shell.held_commands.push(key);
            return;
        }
        if let Some(spec) = features::spec(key) {
            self.dispatch(*spec, Owner::Feature);
        } else if let Some(command) = WorkbenchCommand::from_key(key) {
            self.dispatch(command.declared_spec(), Owner::Workbench(command));
        } else if let Some(command) = ShellCommand::from_key(key) {
            self.dispatch(command.declared_spec(), Owner::Shell(command));
        }
    }

    /// Note whether the focused source editor has keys of this frame still
    /// to apply: typed text, a paste, a composition or a key press that no
    /// shortcut took. Until the page has drawn them, commands wait.
    pub(crate) fn note_editor_keys(&mut self, ctx: &egui::Context) {
        let focused = self
            .state
            .workbench
            .active_source_editor
            .is_some_and(|editor| ctx.memory(|memory| memory.has_focus(editor.id)));
        self.state.shell.editor_keys_pending = focused
            && ctx.input(|input| {
                input.events.iter().any(|event| {
                    matches!(
                        event,
                        egui::Event::Text(_)
                            | egui::Event::Paste(_)
                            | egui::Event::Ime(_)
                            | egui::Event::Key { pressed: true, .. }
                    )
                })
            });
    }

    /// Carry out the commands that waited for the editor's keys, now that
    /// the page has drawn them, in the order they were raised. Whether there
    /// were any.
    pub(crate) fn carry_out_held_commands(&mut self) -> bool {
        self.state.shell.editor_keys_pending = false;
        let held = std::mem::take(&mut self.state.shell.held_commands);
        let any = !held.is_empty();
        for key in held {
            self.execute_command_key(key);
        }
        any
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
        // A feature that takes another feature's command where the reader is
        // working answers for it there; refused, it says why in the owner's
        // words.
        let owner = features::availability(self, key).unwrap_or(CommandAvailability::Hidden);
        match features::routed_enabled(self, key) {
            Some(true) => CommandAvailability::Available,
            Some(false) if owner.is_available() => {
                CommandAvailability::Disabled("command is unavailable in this context")
            }
            Some(false) | None => owner,
        }
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
        // A feature's command runs only where it will act. Raised where it
        // cannot, by a row of a feature's own menu or by another command, it
        // is not run; the row that offers it is off and says why. One that
        // states its own refusal runs to say it.
        if let Owner::Feature = owner
            && !flags.contains(CommandFlags::STATES_ITS_REFUSAL)
            && !self.command_availability(spec.key).is_available()
        {
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
        if !features::route(self, spec.key) {
            features::execute(self, spec.key);
        }
        self.after_contribution();
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
    /// press had done the source feature's part (the updated symbol to open,
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
    /// live session's mirror, the cell a navigator Masters row asked to
    /// rename, copy, move or delete, the selected model and the Verilog-A compile
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
            |app, AskNewSheetFormat| ask_new_sheet_format(&mut app.state),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, request: NetlistEditorRequest| answer_netlist_editor_request(app, request),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, probed: PlotProbedTrace| plot_probed_trace(&mut app.state, probed),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, probed: PlotProbedDifference| plot_probed_difference(&mut app.state, probed),
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, shown: ShowSignalOnSchematic| {
                show_signal_in_its_occurrence(&mut app.state, shown)
            },
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
            |app, RunPlanAnyway| run_plan_anyway(app),
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
            |app, ApplyPathVariables { ctx, variables }| {
                ModelsFeature::apply_path_variables(&mut features::cx(app), &ctx, variables);
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
             OpenInVerify {
                 page,
                 specification,
                 check,
             }| open_in_verify(app, page, specification, check),
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
                rspice_app_state::workbench::app::schematic::placement::arm_part_placement(
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
            |app, place: ShowOnSchematic| {
                app.state.navigation().show_on_schematic(place);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app, ShowProjectDeck| {
                SourceFeature::show_project_deck(&mut features::cx(app));
                app.execute_command_key(ids::NETLIST_WORKSPACE);
            },
        );
        requests::drain(
            self,
            |app| &mut app.state.workbench.outbox,
            |app,
             CellHousekeeping {
                 library,
                 cell,
                 action,
             }| {
                ProjectFeature::cell_housekeeping(&mut features::cx(app), &library, &cell, action);
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
            |app, request: SourceRequest| answer_code_request(app, request),
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
    let text = state.workbench.messages().format(
        rspice_app_workbench::workbench::localization::MessageId::NetlistRunSourceUnavailable,
        &[
            ("id", &sequence.to_string()),
            ("reason", rspice_app_state::state::absent_deck_reason()),
        ],
    );
    state.prompt(text);
    false
}

/// Open Design management on its New sheet page, the new sheet going after
/// the active one, so a project that asks for each new sheet's drawing sheet
/// asks there.
fn ask_new_sheet_format(state: &mut crate::workbench::AppState) {
    use rspice_app_project::workbench::app::dialogs::design_management::{
        DesignManagementPage, DesignManagementTab, open_design_management_on,
    };
    use rspice_app_workbench::workbench::routing::{
        navigation::RouteTransitionSource, surface_catalog::SurfaceId, surface_route::SurfaceRoute,
    };

    let route = SurfaceRoute::surface(SurfaceId::DesignManagement);
    if let Err(error) = state.workbench.navigate(route, RouteTransitionSource::User) {
        state.workbench.prompt(error.to_string());
        return;
    }
    let dialog = &mut state.project.design_management;
    open_design_management_on(
        dialog,
        &mut state.model,
        &state.platform,
        &mut state.workbench,
        DesignManagementTab::Sheets,
    );
    if dialog.open {
        let configurations = state.model.workspace.content.configuration_sets.clone();
        dialog.reset_inputs_for_page(
            DesignManagementPage::NewSheet,
            Some(&configurations),
            state.model.active_sheet_id(),
        );
    }
}

/// Have the source feature carry out what the netlist page's editor raised,
/// in its context.
fn answer_netlist_editor_request(app: &mut RSpiceApp, request: NetlistEditorRequest) {
    code_requests::answer_netlist_editor(&mut features::cx(app), request);
}

/// Have the source feature drop what the last Verilog-A compile proved, in its
/// context.
fn answer_void_veriloga_evidence(app: &mut RSpiceApp) {
    code_requests::void_veriloga_evidence(&mut features::cx(app));
}

/// Have the source feature answer what it raised, in its context, holding
/// whether an application-modal dialog is open, taken as the answer starts.
/// What reaches past the feature the shell carries out straight after.
fn answer_code_request(app: &mut RSpiceApp, request: SourceRequest) {
    let modal_open = app.state.application_modal_open();
    let hand_off = code_requests::answer(
        &mut features::cx::<SourceFeature>(app).with_application_modal(modal_open),
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
    /// opens the source feature's netlist page.
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
            DesignRequest::OpenSourceMap(net) => {
                rspice_app_code::workbench::surfaces::netlist::generation::show_generated_netlist(
                    &mut features::cx::<SourceFeature>(self),
                );
                open_connectivity_source_map(&mut self.state, &net);
            }
            DesignRequest::ApplyConnectivityRepairs => {
                DesignFeature::apply_connectivity_repairs(&mut features::cx(self));
            }
            DesignRequest::CreateHierarchy => {
                DesignFeature::create_hierarchy(&mut features::cx(self));
            }
        }
    }

    /// Have the source feature take the netlist page's own history step
    /// `direction`, in its context.
    pub(in crate::workbench) fn take_netlist_history_step(
        &mut self,
        direction: HistoryDirection,
    ) -> Result<Option<String>, String> {
        code_requests::step_history(&mut features::cx(self), direction == HistoryDirection::Undo)
    }
}

/// Run anyway: start the plan's run held for more memory than is free, as
/// Run starts it. A run that needs more memory than is free may take the
/// machine down with it: the work it could lose is kept before it
/// dispatches.
fn run_plan_anyway(app: &mut RSpiceApp) {
    if !rspice_app_runs::workbench::preflight::run_held_run_anyway(&mut app.state) {
        return;
    }
    let plan = app
        .state
        .model
        .sim_setup
        .active_plan_name()
        .as_str()
        .to_owned();
    let said = app.state.workbench.messages().format(
        rspice_app_workbench::workbench::localization::MessageId::NetlistMemoryRunningAnyway,
        &[("name", &plan)],
    );
    app.state.model.record(
        rspice_app_state::diagnostics::LogSeverity::Info,
        rspice_app_state::diagnostics::LogSource::Run,
        said,
    );
    rspice_app_services::workbench::app_state::session::autosave::write_now(&mut app.state, None);
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
}
