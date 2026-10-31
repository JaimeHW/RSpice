//! The models feature: the Models & PDKs workspace, the device-model editor,
//! measurement correlation and unsigned PDK authoring.
//!
//! What the workspace keeps between frames is declared here, beneath the
//! session aggregate that holds the models slot and the wire that persists
//! it, so both name it without reaching up into the surfaces.

pub(crate) mod commands;
pub(crate) mod model_editor_workflow;
#[cfg(target_arch = "wasm32")]
pub(in crate::workbench) mod pack_persistence;
#[cfg(target_arch = "wasm32")]
pub(in crate::workbench) mod pdk_persistence;
pub(crate) mod pdk_workflow;
pub(in crate::workbench) mod qualification;
pub(crate) mod workspace_state;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::ModelsPage;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::documents::model_editor::{
    ModelEditorParts, ModelEditorReads, create_editable_project_copy_and_open, open_project_model,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureNavigator, FeatureRegistry,
    FeatureSurface, NoHandles, Reads, held_back,
};
use crate::workbench::slots::ModelsState;
use crate::workbench::state::{
    ModelsWorkbenchDialog, PackReleaseConfirmation, WorkbenchSession, Workspace,
};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use commands::ModelsCommand;
use model_editor_workflow::{
    ModelEditorWorkflow, model_library_rescan_diagnostic, request_model_editor_workflow,
    selected_model_for_project_copy, selected_project_model_for_editor,
};
use rspice_app_workbench::workbench::app::dialogs::model_browser::render_model_browser;

/// The models feature.
pub(crate) struct ModelsFeature;

impl Feature for ModelsFeature {
    type State = ModelsState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<ModelsCommand>();
        registry.dialogs(DIALOGS);
        registry.surface(FeatureSurface {
            show: ModelsFeature::show_workspace,
        });
        registry.navigator(FeatureNavigator {
            show: show_navigator,
        });
        registry.inspector(FeatureInspector {
            show: show_inspector,
            // The section only reports: it holds nothing typed to commit.
            commit_pending_edit: |_, _| {},
        });
    }
}

/// The dialogs the feature draws, each under the key the shell lists it by.
/// The dialog order draws the PDK settings and the model browser at their
/// steps, the PDK settings step taking the model operations that finished
/// since the last frame whether or not the dialog is open. The other two
/// hold a modal gate the shell asks of them, and each is drawn where its
/// work is: the correlation dialog by the measurement correlation route,
/// and the Models workspace's own dialogs by that workspace, which carries
/// out what they ask.
const DIALOGS: &[FeatureDialog<ModelsFeature>] = &[
    FeatureDialog {
        key: DialogKey("models.pdk_settings"),
        is_open: |models| models.pdk_settings_dialog.open,
        render: ModelsFeature::process_pdk_settings_dialog,
    },
    FeatureDialog {
        key: DialogKey("models.model_browser"),
        is_open: |models| models.model_browser_state.open,
        render: render_model_browser_dialog,
    },
    FeatureDialog {
        key: DialogKey("models.model_correlation"),
        is_open: |models| models.model_correlation.dialog_open(),
        render: ModelsFeature::render_correlation_dialog,
    },
    FeatureDialog {
        key: DialogKey("models.models_view"),
        is_open: |models| models.models_view.dialog.is_some(),
        // The Models workspace draws them inside its stage, where it
        // carries out what they ask.
        render: |_, _| {},
    },
];

#[cfg(target_arch = "wasm32")]
impl ModelsFeature {
    /// Take what browser PDK storage settled since the last frame: a
    /// restored or published configuration goes live and drops the run
    /// preflight, and a failure is logged.
    pub(in crate::workbench) fn poll_browser_pdk_persistence(
        cx: &mut Cx<'_, ModelsFeature>,
        ctx: &egui::Context,
    ) {
        use crate::workbench::models::pdk_persistence::{
            BrowserPdkParts, poll_browser_pdk_persistence,
        };
        poll_browser_pdk_persistence(
            &mut BrowserPdkParts {
                model: cx.model,
                workbench: cx.wb,
                platform: cx.platform,
                preflight: &mut cx.runs.preflight,
                run_controller: cx.handles.run_controller,
            },
            ctx,
        );
    }

    /// Take the model packs this browser kept, on the frame their restore
    /// lands: the hub's store takes them, and a pack that does not prove
    /// goes on the workspace's operation ladder.
    pub(in crate::workbench) fn poll_browser_pack_restore(
        cx: &mut Cx<'_, ModelsFeature>,
        ctx: &egui::Context,
    ) {
        use crate::workbench::models::pack_persistence::{
            PackRestoreParts, poll_browser_pack_restore,
        };
        poll_browser_pack_restore(
            &mut PackRestoreParts {
                models_view: &mut cx.state.models_view,
                model: cx.model,
                model_hub: cx.handles.model_hub,
            },
            ctx,
        );
    }
}

impl ModelsFeature {
    /// Put up the confirmation of attaching the model pack release
    /// `pack_id` `version` for its part `part_id`, or say on the status bar
    /// that the model hub no longer describes that release. Returns whether
    /// the confirmation is up, for the caller to show the catalog it sits on.
    pub(in crate::workbench) fn confirm_pack_attach(
        cx: &mut Cx<'_, ModelsFeature>,
        part_id: String,
        pack_id: String,
        version: String,
    ) -> bool {
        let Some(release) = PackReleaseConfirmation::for_release(
            cx.handles.model_hub,
            &pack_id,
            &version,
            Some(part_id.clone()),
        ) else {
            cx.wb.prompt(format!(
                "The model hub no longer describes {pack_id} {version}, so '{part_id}' cannot be \
                 added."
            ));
            return false;
        };
        cx.state.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
            pack_id,
            attach: true,
            release: Some(Box::new(release)),
        });
        true
    }

    /// Show the catalog model `model` of `library` on the Models workspace's
    /// catalog, saying on the status bar when the project no longer holds
    /// the library.
    pub(in crate::workbench) fn reveal_model(
        cx: &mut Cx<'_, ModelsFeature>,
        library: &str,
        model: String,
    ) {
        workspace_state::select_library_or_prompt(cx.model, cx.wb, library);
        cx.state.selected_model = Some(model);
        cx.state.models_page = ModelsPage::Models;
        cx.wb.activate(Workspace::Models);
    }
}

/// The Models workspace's navigator: its pages and the loaded model
/// libraries.
fn show_navigator(cx: &mut Cx<'_, ModelsFeature>, ui: &mut egui::Ui) {
    let models = &mut *cx.state;
    crate::workbench::docks::navigator::models::models(
        ui,
        &mut models.models_page,
        &mut models.selected_model,
        cx.model,
        cx.wb,
    );
}

/// What the inspector says about the model library a reader has selected.
fn show_inspector(cx: &mut Cx<'_, ModelsFeature>, ui: &mut egui::Ui) {
    crate::workbench::docks::inspector::models::show(
        ui,
        cx.state.selected_model.as_deref(),
        cx.model,
    );
}

/// What the model commands' rules read: the feature's state, the model and
/// the platform state, whether the shell asks or the editor does as it draws.
struct CommandReads<'a> {
    state: &'a ModelsState,
    model: &'a Model,
    platform: &'a PlatformState,
}

/// What the model commands' selection rules read.
fn model_editor_reads<'a>(reads: &CommandReads<'a>) -> ModelEditorReads<'a> {
    reads.state.model_editor_reads(reads.model, reads.platform)
}

/// Whether `command` can run here, and why not, as the shell answers it for
/// a command the feature declares: the shell's own guards on what the
/// command declares, then the feature's rule.
pub(crate) fn command_availability(
    command: ModelsCommand,
    cx: &Cx<'_, ModelsFeature>,
) -> CommandAvailability {
    if let Some(reason) = held_back(command.declared_spec().flags, cx.platform) {
        return CommandAvailability::Disabled(reason);
    }
    rule_availability(
        command,
        &CommandReads {
            state: cx.state,
            model: cx.model,
            platform: cx.platform,
        },
    )
}

/// Whether `command` can run, and why not, once the shell's guards have
/// passed.
fn rule_availability(command: ModelsCommand, reads: &CommandReads<'_>) -> CommandAvailability {
    if command_enabled(command, reads) {
        CommandAvailability::Available
    } else {
        CommandAvailability::Disabled(disabled_reason(command, reads))
    }
}

/// What opening a model in the editor writes, from the feature's context.
fn model_editor_parts<'a>(cx: &'a mut Cx<'_, ModelsFeature>) -> ModelEditorParts<'a> {
    cx.state.model_editor_parts(cx.model, cx.wb, cx.platform)
}

/// Whether `command` can run, once the shell's guards (a project operation
/// in flight, no project open) have passed.
fn command_enabled(command: ModelsCommand, reads: &CommandReads<'_>) -> bool {
    let state = reads.state;
    match command {
        ModelsCommand::ModelCreateProjectCopy => {
            selected_model_for_project_copy(model_editor_reads(reads)).is_ok()
        }
        ModelsCommand::ModelEditor | ModelsCommand::ModelCorrelation => {
            selected_project_model_for_editor(model_editor_reads(reads)).is_ok()
        }
        ModelsCommand::ModelSaveRevision => {
            state
                .model_editor
                .draft
                .as_ref()
                .is_some_and(|draft| {
                    draft.is_dirty()
                        && !reads.model.safe_mode.project_read_only()
                        && reads.platform.project_lifecycle.is_open()
                        && state.model_editor.qualification_execution.is_none()
                })
        }
        ModelsCommand::ModelValidate => state.model_editor.draft.is_some(),
        ModelsCommand::ModelRunQualificationTests => {
            state
                .model_editor
                .draft
                .as_ref()
                .is_some_and(|draft| {
                    reads.platform.project_lifecycle.is_open()
                        && !reads.model.safe_mode.project_read_only()
                        && !draft.definition_is_dirty()
                        && draft
                            .qualification
                            .validate_for_model(&draft.model_name)
                            .is_ok()
                        && crate::state::model_library::ModelSourceEvidenceBinding::try_new_project_bound(
                            &draft.model_name,
                            draft.source_id,
                            draft.base_source_digest,
                            draft.base_source_revision,
                        )
                        .is_ok_and(|source| {
                            draft
                                .qualification
                                .exact_suites_for_source(&source)
                                .is_ok_and(|suites| !suites.is_empty())
                        })
                        && state.model_editor.validation.is_some()
                        && state.model_editor.qualification_execution.is_none()
                })
        }
        ModelsCommand::ModelCompareRelease => {
            state
                .model_editor
                .draft
                .as_ref()
                .is_some_and(|draft| {
                    !draft.definition_is_dirty()
                        && !draft.qualification.candidates.is_empty()
                        && !draft.qualification.releases.is_empty()
                        && draft
                            .qualification
                            .validate_for_model(&draft.model_name)
                            .is_ok()
                        && state.model_editor.qualification_execution.is_none()
                })
        }
        ModelsCommand::Page(_)
        | ModelsCommand::ModelBrowser
        | ModelsCommand::PdkSettings
        | ModelsCommand::RescanModelLibraries => true,
    }
}

/// Why `command` cannot run while [`command_enabled`] says it cannot.
fn disabled_reason(command: ModelsCommand, reads: &CommandReads<'_>) -> &'static str {
    let state = reads.state;
    match command {
        ModelsCommand::ModelCreateProjectCopy => {
            selected_model_for_project_copy(model_editor_reads(reads))
                .err()
                .unwrap_or("the selected model cannot be copied into the project")
        }
        // Correlation opens on the same selected model the editor does.
        ModelsCommand::ModelEditor | ModelsCommand::ModelCorrelation => {
            selected_project_model_for_editor(model_editor_reads(reads))
                .err()
                .unwrap_or("the selected model cannot be opened for inspection")
        }
        ModelsCommand::ModelSaveRevision if reads.model.safe_mode.project_read_only() => {
            "the project is open read-only"
        }
        ModelsCommand::ModelSaveRevision
            if state.model_editor.qualification_execution.is_some() =>
        {
            "qualification execution is active"
        }
        ModelsCommand::ModelSaveRevision if state.model_editor.draft.is_none() => {
            "no project-owned model candidate is open"
        }
        ModelsCommand::ModelSaveRevision => "the open model candidate has no changes to save",
        ModelsCommand::ModelValidate => "no project-owned model candidate is open",
        ModelsCommand::ModelRunQualificationTests if reads.model.safe_mode.project_read_only() => {
            "qualification cannot run while the project is read-only"
        }
        ModelsCommand::ModelRunQualificationTests
            if state.model_editor.qualification_execution.is_some() =>
        {
            "a model qualification run is already active"
        }
        ModelsCommand::ModelRunQualificationTests => {
            "validate a clean candidate with at least one qualification suite first"
        }
        ModelsCommand::ModelCompareRelease => "a clean candidate and a released model are required",
        ModelsCommand::Page(_)
        | ModelsCommand::ModelBrowser
        | ModelsCommand::PdkSettings
        | ModelsCommand::RescanModelLibraries => "command is unavailable in this context",
    }
}

impl CommandSet for ModelsCommand {
    type Feature = ModelsFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// The pages, the model browser, the PDK paths and the rescan run
    /// whenever the shell lets them through. The editor, the project copy and
    /// correlation need one coherent selected model, and the editor's
    /// workflows need the candidate it has open in the state each one acts on.
    fn availability(self, reads: &Reads<'_, ModelsFeature>) -> CommandAvailability {
        rule_availability(
            self,
            &CommandReads {
                state: reads.state,
                model: reads.model,
                platform: reads.platform,
            },
        )
    }

    fn execute(self, cx: &mut Cx<'_, ModelsFeature>) {
        match self {
            Self::Page(page) => {
                cx.wb.activate(Workspace::Models);
                cx.state.models_page = page;
            }
            Self::ModelBrowser => cx.state.model_browser_state.open = true,
            // Disabled, with the reason, while nothing can be copied.
            Self::ModelCreateProjectCopy => {
                if let Ok((library_name, model_name)) = selected_model_for_project_copy(
                    cx.state.model_editor_reads(cx.model, cx.platform),
                ) {
                    let library_name = library_name.to_owned();
                    let model_name = model_name.to_owned();
                    match create_editable_project_copy_and_open(
                        &mut model_editor_parts(cx),
                        &library_name,
                        &model_name,
                    ) {
                        Ok((project_library, project_model)) => {
                            cx.model.record(
                                LogSeverity::Info,
                                LogSource::Studio,
                                format!(
                                    "Created editable project model '{project_library}/{project_model}'."
                                ),
                            );
                        }
                        Err(error) => cx
                            .wb
                            .prompt(format!("Cannot create editable project model: {error}")),
                    }
                }
            }
            // Disabled, with the reason, while no project model is selected.
            Self::ModelEditor => {
                if let Ok((library_name, model_name)) = selected_project_model_for_editor(
                    cx.state.model_editor_reads(cx.model, cx.platform),
                ) {
                    let library_name = library_name.to_owned();
                    let model_name = model_name.to_owned();
                    if let Err(error) =
                        open_project_model(&mut model_editor_parts(cx), &library_name, &model_name)
                    {
                        cx.wb
                            .prompt(format!("Cannot open device model editor: {error}"));
                    }
                }
            }
            Self::ModelCorrelation => {
                if selected_project_model_for_editor(
                    cx.state.model_editor_reads(cx.model, cx.platform),
                )
                .is_ok()
                    && let Err(error) = cx.wb.navigate(
                        SurfaceRoute::surface(SurfaceId::ModelCorrelation),
                        RouteTransitionSource::User,
                    )
                {
                    cx.wb
                        .prompt(format!("Cannot open measurement correlation: {error}"));
                }
            }
            // The three model workflows are disabled while no candidate is open.
            Self::ModelSaveRevision => {
                request_model_editor_workflow(
                    cx.state.model_editor_reads(cx.model, cx.platform),
                    ModelEditorWorkflow::SaveRevision,
                );
            }
            Self::ModelValidate => {
                request_model_editor_workflow(
                    cx.state.model_editor_reads(cx.model, cx.platform),
                    ModelEditorWorkflow::ValidateCandidate,
                );
            }
            Self::ModelRunQualificationTests => {
                request_model_editor_workflow(
                    cx.state.model_editor_reads(cx.model, cx.platform),
                    ModelEditorWorkflow::RunQualification,
                );
            }
            Self::ModelCompareRelease => {
                cx.state.model_editor.comparison_open = true;
            }
            Self::PdkSettings => cx
                .state
                .pdk_settings_dialog
                .open(cx.model.pdk_config.clone()),
            Self::RescanModelLibraries => {
                crate::state::pdk_config::discover_model_files(&mut cx.model.pdk_config);
                cx.model.model_library_manager.discover_spice_packs();
                // Reported here, so the toolbar, the menu and the palette all
                // say the same. A clean rescan answers the gesture; one that
                // could not read a library is worth interrupting for.
                let (message, has_errors) = model_library_rescan_diagnostic(cx.model);
                if has_errors {
                    cx.model
                        .report_warning("Model library rescan completed with warnings", message);
                } else {
                    let message = format!("Model libraries rescanned: {message}");
                    cx.model
                        .record(LogSeverity::Info, LogSource::Studio, message.clone());
                    cx.wb.prompt(message);
                }
            }
        }
    }
}

/// The standalone model browser the Models menu opens, over every loaded
/// model.
fn render_model_browser_dialog(cx: &mut Cx<'_, ModelsFeature>, ctx: &egui::Context) {
    let _ = render_model_browser(
        ctx,
        &mut cx.state.model_browser_state,
        &cx.model.model_library_manager,
    );
}

/// What the models feature's tests build from.
#[cfg(test)]
pub(crate) mod tests {
    use crate::state::pdk_config::PdkConfig;
    use crate::workbench::app_state::Model;
    use crate::workbench::framework::feature::harness::FeatureHarness;
    use crate::workbench::state::WorkbenchSession;
    use rspice_app_state::workbench::workspace::restore_model_library_manager;

    use super::ModelsFeature;

    /// The models feature on its own, over what a new session starts with:
    /// the model over the empty PDK configuration a test session reads, and a
    /// new session's workbench.
    pub(crate) fn harness() -> FeatureHarness<ModelsFeature> {
        FeatureHarness::with_model(
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
            WorkbenchSession::default(),
        )
    }
}
