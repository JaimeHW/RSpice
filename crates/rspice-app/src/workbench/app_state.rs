//! The application data.
//!
//! [`AppState`] is everything a session owns and everything a workflow reads:
//! the design, the simulation, the document engines' session state, and the
//! dialogs' visibility. It is deliberately separate from
//! [`RSpiceApp`](crate::workbench::app::RSpiceApp), which owns the frame loop
//! and drives this state — a module that needs to read or mutate the session
//! should depend on the data, not on the application root.
//!
//! Behaviour that queries a specific concern lives with that concern, as an
//! `impl AppState` block in the owning module (netlist run gating in
//! `documents::netlist_document`, modal arbitration in
//! `workflows::project_workflow`). Only accessors over this struct's own
//! fields are defined here.

pub(in crate::workbench) mod cell_usage;
pub(in crate::workbench) mod design_checks;
pub(in crate::workbench) mod design_history;
pub(crate) mod diagnostic_store;
mod full_screen;
pub(in crate::workbench) mod interaction_state;
#[cfg(test)]
mod modal_gate_tests;
mod model;
mod model_access;
pub(in crate::workbench) mod problems_view;
mod rule_deck;
mod run_access;
pub(in crate::workbench) mod run_identity;
pub(crate) mod run_preflight;
pub(in crate::workbench) mod schematic_preferences;
pub(crate) mod services_access;
pub(in crate::workbench) mod session;
pub(in crate::workbench) mod sim_setup;
#[cfg(test)]
mod stack_budget;
#[cfg(test)]
mod technology_demand_tests;
mod transaction;
pub(in crate::workbench) mod viewer_capabilities;
pub(in crate::workbench) mod waivers;

use crate::services::recent_files::{RecentFile, RecentKind};
use crate::state::notices::NoticeKind;
use crate::workbench::slots::{
    CloudState, CodeState, DesignState, HardcopyState, ModelsState, ProjectState, ResultsState,
    RunsState, SettingsState, SheetsState, ShellState, SimulateState,
};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

pub(crate) use design_checks::{DesignCheckStatus, model_design_view, model_location_anchor};
pub(crate) use design_history::{
    DesignManagementHistoryEntry, SymbolDefinitionFixtureDelta, VerilogABuildSettingsHistoryEntry,
    VerilogAModuleBinding, publish_symbol_definition_candidate,
    publish_symbol_definition_candidate_with_fixture,
};
pub(crate) use interaction_state::SchematicKeyboardFocus;
pub use interaction_state::{ContextTarget, DragType, InteractionState};
pub(crate) use model::{Model, SlotFacts};
// The platform services' own state, which they name beside the model.
pub(in crate::workbench) use crate::workbench::slots::PlatformState;
pub use rspice_simulation_contract::plan_catalog::SimulationPlanImportDocument;
pub(crate) use session::shortcuts::{
    accessibility_shortcut_summary, report_engineering_canvas_focus, runtime_command_platform,
};
pub(crate) use session::state_init::{
    default_model_library_manager, restore_session_model_library_manager,
};
pub use sim_setup::plan_catalog::{
    SimulationPlanCloneOptions, SimulationPlanLineage, SimulationPlanName,
};
pub use sim_setup::{
    AcSetup, DcSetup, ReferencePvtPoint, SimSetupState, SimulationSavePolicy, TranSetup,
};

/// Main application state container
#[derive(Clone)]
pub struct AppState {
    /// The shared project model: the design, its simulation and checks, the
    /// libraries, and the evidence two or more features read. Inline, so the
    /// state moves at the size it had before these fields were gathered.
    pub(crate) model: Model,
    /// Where the reader is standing in the model's diagnostics: the Problems
    /// grid's order, filters and groups. A reading position over the evidence, never
    /// evidence itself, and never persisted.
    pub(crate) problems_view: problems_view::ProblemsView,
    /// The shell's own dialogs. Inline: a few flags and a filter.
    pub(crate) shell: ShellState,
    /// The platform services' runtime state.
    pub(crate) platform: Box<PlatformState>,
    /// What each feature keeps open between frames: its dialogs and the
    /// drafts they edit. Boxed, so a moved state carries a pointer per
    /// feature rather than every feature's dialogs.
    pub(crate) design: Box<DesignState>,
    pub(crate) code: Box<CodeState>,
    pub(crate) simulate: Box<SimulateState>,
    pub(crate) project: Box<ProjectState>,
    pub(crate) results: Box<ResultsState>,
    pub(crate) hardcopy: Box<HardcopyState>,
    pub(crate) settings: Box<SettingsState>,
    pub(crate) cloud: Box<CloudState>,
    pub(crate) sheets: Box<SheetsState>,
    pub(crate) models: Box<ModelsState>,
    /// What watching the simulations in flight keeps between frames.
    pub(crate) runs: Box<RunsState>,
    /// Canonical workbench navigation and responsive layout state.
    pub(crate) workbench: Box<crate::workbench::WorkbenchSession>,
    /// Runtime-only chord/prefix authority. Partial sequences are never
    /// serialized or restored across application sessions.
    pub(crate) shortcut_resolver: session::shortcuts::ShortcutResolverState,
}

impl Default for AppState {
    fn default() -> Self {
        session::state_init::default_app_state()
    }
}

impl AppState {
    /// The model's copy of whether a project is open agrees with the project
    /// lifecycle, in debug builds. The frame checks this before it does
    /// anything else, and tests check it after each lifecycle transition.
    pub(crate) fn assert_project_open_fact(&self) {
        debug_assert_eq!(
            self.model.project_open.is_open(),
            self.platform.project_lifecycle.is_open(),
            "the model's project-open fact disagrees with the project lifecycle"
        );
    }

    /// The reader's result presentation, with the significant digits of the
    /// project result document the Results workspace is showing, if any.
    #[must_use]
    pub(crate) fn result_presentation_policy(
        &self,
    ) -> crate::workbench::preferences::ResultPresentationPolicy {
        let policy = self.model.preferences.result_presentation_policy();
        self.results
            .results
            .session
            .document_presentation
            .map_or(policy, |document| {
                policy.with_significant_digits(document.significant_digits)
            })
    }

    /// Reserve the next project revision before a library membership
    /// transaction publishes any live state.
    pub(crate) fn preflight_project_library_mutation(
        &self,
        mutation: crate::state::ProjectLibraryMutation,
    ) -> Result<crate::state::PreparedProjectLibraryMutation, String> {
        let operation = mutation.operation_label();
        if self.model.safe_mode.project_read_only() {
            return Err(format!(
                "{operation} is unavailable because the project is open read-only."
            ));
        }
        self.model
            .library_edit_locks
            .require_mutation_authority(
                self.model.workspace.content.project.id(),
                self.model.workspace.content.project.revision(),
                self.model.library_manager.revision(),
                &mutation,
            )
            .map_err(|error| format!("Could not reserve {operation}: {error}"))?;
        self.model
            .workspace
            .content
            .project
            .prepare_library_mutation(mutation, self.model.library_manager.revision())
            .map_err(|error| format!("Could not reserve {operation}: {error}"))
    }

    /// Publish the single project revision reserved immediately before a
    /// successful library membership transaction.
    pub(crate) fn publish_project_library_mutation(
        &mut self,
        prepared: crate::state::PreparedProjectLibraryMutation,
    ) {
        let observed_library_revision = self.model.library_manager.revision();
        assert!(
            observed_library_revision >= prepared.minimum_library_revision(),
            "library mutation did not advance the library revision"
        );
        self.model
            .workspace
            .content
            .project
            .publish_library_mutation(prepared, observed_library_revision)
            .expect("the library mutation receipt was fully preflighted");
        self.model.workspace.content.project_metadata_dirty = true;
        self.model.design_execution_epoch = self.model.design_execution_epoch.wrapping_add(1);
        self.model.netlist_artifacts.current_generation_input_digest = None;
        self.model.clear_project_design_history();
    }

    /// [`Model::technology_inputs`].
    pub(crate) fn technology_inputs(
        &self,
    ) -> rspice_simulation::project_technology::ProjectTechnologyInputs<'_> {
        self.model.technology_inputs()
    }

    /// [`Model::include_search_chain`].
    pub(crate) fn include_search_chain(
        &self,
    ) -> rspice_simulation::netlist_preparation::IncludeSearchChain {
        self.model.include_search_chain()
    }

    /// [`Model::library_folders`].
    pub(crate) fn library_folders(&self) -> &[std::path::PathBuf] {
        self.model.library_folders()
    }

    /// Resolve the project-owned technology contract against both mutable
    /// execution catalogs. No simulation or governed save may infer authority
    /// from a display label, an active registry package, or model paths alone.
    pub(crate) fn validate_project_technology_contract(&self) -> Result<(), String> {
        self.technology_inputs()
            .validate_project_technology_contract()
    }

    /// Resolve the exact currently trusted signed package pinned by this
    /// project for read-only workspace overlays.
    pub(crate) fn project_signed_technology_package(
        &self,
    ) -> Result<Option<&crate::state::pdk_config::ValidatedPdkTechnologyPackage>, String> {
        self.model
            .pdk_config
            .technology_registry
            .project_signed_technology_package(
                self.model.workspace.content.project.technology_binding(),
            )
    }

    /// Whether an attached project technology is in effect: bound, and bound
    /// through checkpoint-backed authority receipts. Execution paths that
    /// prefer project-sealed model sources over the plain model library key
    /// off this, not off the binding alone.
    pub(crate) fn project_technology_in_effect(&self) -> bool {
        self.technology_inputs().project_technology_in_effect()
    }

    /// What the authored plan needs a project technology for, computed before
    /// any netlist exists.
    #[cfg(test)]
    pub(crate) fn technology_demand(&self) -> rspice_simulation::preparation::TechnologyDemand {
        self.technology_inputs().technology_demand()
    }

    /// The single technology gate: a project need not have a technology; if it
    /// has one it must be valid; if the plan needs one it must have one.
    ///
    /// A present binding is resolved against the full contract, which already
    /// yields the exact reattach message for a binding without receipts. An
    /// absent binding blocks only what the plan actually demands.
    pub(crate) fn technology_gate_block_reason(&self) -> Result<(), String> {
        self.technology_inputs().technology_gate_block_reason()
    }

    /// Resolve the exact signed project pin into the physical-layout unit and
    /// stack authority a new or imported Layout document must retain.
    pub(crate) fn exact_project_layout_technology_binding(
        &self,
    ) -> Result<crate::state::LayoutTechnologyBinding, String> {
        let package = self.exact_project_pdk_package()?;
        let manifest = package.manifest();
        let database_unit =
            crate::quantity::LayoutDatabaseUnit::from_metres(manifest.database_unit_meters)
                .map_err(|error| format!("Signed project PDK database unit is invalid: {error}"))?;
        crate::state::LayoutTechnologyBinding::try_new(
            manifest.package_id.clone(),
            manifest.revision.clone(),
            package.manifest_digest(),
            package.archive_digest(),
            manifest.technology_name.clone(),
            manifest.stack_name.clone(),
            database_unit,
        )
        .map_err(|error| format!("Signed project PDK layout authority is invalid: {error}"))
    }

    /// Resolve only the currently validated package selected by the exact
    /// signed project pin. Administrative activation is deliberately ignored.
    pub(crate) fn exact_project_pdk_package(
        &self,
    ) -> Result<&crate::state::pdk_config::ValidatedPdkTechnologyPackage, String> {
        self.validate_project_technology_contract()?;
        let signed_pin = self
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .and_then(crate::state::ProjectTechnologyBinding::signed_package)
            .expect("validated project technology has an exact signed package");
        self.model
            .pdk_config
            .technology_registry
            .validated_packages()
            .iter()
            .find(|package| {
                let manifest = package.manifest();
                manifest
                    .package_id
                    .eq_ignore_ascii_case(signed_pin.package_id())
                    && manifest.revision == signed_pin.revision()
                    && package.manifest_digest() == signed_pin.manifest_digest()
                    && package.archive_digest() == signed_pin.archive_digest()
            })
            .ok_or_else(|| {
                "The signed PDK package the project is pinned to is no longer available".to_owned()
            })
    }

    /// Initialize authoritative physical content for an existing Layout view.
    /// This is the shared entry point future editors and stream importers must
    /// use; it refuses catalog labels that do not resolve to an exact editable
    /// Layout view and never invents a technology or database unit.
    pub(crate) fn initialize_physical_layout_document(
        &mut self,
        owner: crate::state::CellViewRef,
    ) -> Result<crate::product::ObjectRevision, String> {
        if self.model.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only; physical layout cannot be created".to_owned(),
            );
        }
        owner
            .validate_name_segments()
            .map_err(|error| format!("Layout owner is invalid: {error}"))?;
        let library = self
            .model
            .library_manager
            .get_library(&owner.library)
            .ok_or_else(|| format!("Layout owner library '{}' does not exist", owner.library))?;
        if library.read_only {
            return Err(format!(
                "Layout owner library '{}' is read-only",
                owner.library
            ));
        }
        let view = library
            .get_cell(&owner.cell)
            .and_then(|cell| cell.get_view(&owner.view))
            .ok_or_else(|| format!("Layout owner '{}' does not exist", owner.display_path()))?;
        if view.view_type != crate::state::ViewType::Layout {
            return Err(format!(
                "Layout owner '{}' is a {} view, not a Layout view",
                owner.display_path(),
                view.view_type.display_name()
            ));
        }
        if self
            .model
            .workspace
            .content
            .physical_layout_document(&owner)
            .is_some()
        {
            return Err(format!(
                "Layout owner '{}' already has an authoritative physical document",
                owner.display_path()
            ));
        }
        let technology = self.exact_project_layout_technology_binding()?;
        let document = crate::state::PhysicalLayoutDocument::try_new(owner.clone(), technology)
            .map_err(|error| format!("Physical layout could not be initialized: {error}"))?;
        let revision = document.revision();
        self.model
            .workspace
            .content
            .commit_physical_layout_document(document)
            .map_err(|error| format!("Physical layout was not committed: {error}"))?;
        if let Some(mut library) = self.model.library_manager.edit_library(&owner.library) {
            library.mark_view_modified(&owner.cell, &owner.view);
        }
        if let Some(open) = self
            .model
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == owner)
        {
            open.dirty = true;
        }
        self.model.design_execution_epoch = self.model.design_execution_epoch.wrapping_add(1);
        self.model.clear_project_design_history();
        Ok(revision)
    }

    /// Apply one atomic, revision-checked edit batch to an authoritative
    /// physical-layout cellview and publish no partial state on failure.
    pub(crate) fn apply_physical_layout_transaction(
        &mut self,
        owner: &crate::state::CellViewRef,
        expected_revision: crate::product::ObjectRevision,
        edits: &[crate::state::LayoutEdit],
    ) -> Result<crate::product::ObjectRevision, String> {
        if self.model.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only; physical layout cannot be edited".to_owned(),
            );
        }
        let library = self
            .model
            .library_manager
            .get_library(&owner.library)
            .ok_or_else(|| format!("Layout owner library '{}' does not exist", owner.library))?;
        if library.read_only {
            return Err(format!(
                "Layout owner library '{}' is read-only",
                owner.library
            ));
        }
        let view = library
            .get_cell(&owner.cell)
            .and_then(|cell| cell.get_view(&owner.view))
            .ok_or_else(|| format!("Layout owner '{}' does not exist", owner.display_path()))?;
        if view.view_type != crate::state::ViewType::Layout {
            return Err(format!(
                "Layout owner '{}' is a {} view, not a Layout view",
                owner.display_path(),
                view.view_type.display_name()
            ));
        }
        let exact_technology = self.exact_project_layout_technology_binding()?;
        let allowed_layer_purposes = self
            .exact_project_pdk_package()?
            .manifest()
            .layers
            .iter()
            .flat_map(|layer| {
                layer.purposes.iter().map(move |purpose| {
                    (
                        layer.name.to_ascii_lowercase(),
                        purpose.to_ascii_lowercase(),
                    )
                })
            })
            .collect::<std::collections::BTreeSet<_>>();
        let mut document = self
            .model
            .workspace
            .content
            .physical_layout_document(owner)
            .ok_or_else(|| {
                format!(
                    "Layout owner '{}' has no authoritative physical document",
                    owner.display_path()
                )
            })?
            .clone();
        if document.technology() != &exact_technology {
            return Err(format!(
                "Layout owner '{}' is bound to a different signed technology than the project",
                owner.display_path()
            ));
        }
        let revision = document
            .apply_transaction(expected_revision, edits)
            .map_err(|error| format!("Physical-layout transaction failed: {error}"))?;
        for (kind, id, layer, purpose) in document
            .shapes()
            .iter()
            .map(|(id, shape)| {
                (
                    "shape",
                    id.to_string(),
                    shape.layer_purpose.layer.as_str(),
                    shape.layer_purpose.purpose.as_str(),
                )
            })
            .chain(document.texts().iter().map(|(id, text)| {
                (
                    "text",
                    id.to_string(),
                    text.layer_purpose.layer.as_str(),
                    text.layer_purpose.purpose.as_str(),
                )
            }))
        {
            if !allowed_layer_purposes
                .contains(&(layer.to_ascii_lowercase(), purpose.to_ascii_lowercase()))
            {
                return Err(format!(
                    "Physical-layout edit failed: {kind} {id} uses layer/purpose '{layer}/{purpose}', which the project's signed PDK does not define"
                ));
            }
        }
        self.model
            .workspace
            .content
            .commit_physical_layout_document(document)
            .map_err(|error| format!("Physical-layout transaction was not committed: {error}"))?;
        if let Some(mut library) = self.model.library_manager.edit_library(&owner.library) {
            library.mark_view_modified(&owner.cell, &owner.view);
        }
        if let Some(open) = self
            .model
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == *owner)
        {
            open.dirty = true;
        }
        self.model.design_execution_epoch = self.model.design_execution_epoch.wrapping_add(1);
        self.model.clear_project_design_history();
        Ok(revision)
    }

    /// Seal the complete model authority selected by the project.
    ///
    /// The project-owned model-library closure and its exact signed PDK
    /// package are merged into one in-memory resolver. This is the only model
    /// source snapshot simulation and inspection may consume.
    pub(crate) fn seal_project_execution_model_sources(
        &self,
    ) -> Result<crate::state::model_library::SealedModelExecutionSources, String> {
        self.technology_inputs()
            .seal_project_execution_model_sources()
    }

    /// Execute one signed PDK callback for the exact package pinned by the
    /// project and commit its evidence as one project revision transaction.
    ///
    /// Inputs are the validated active simulation-plan variables normalized to
    /// finite SI values. Administrative activation is never consulted. Guest
    /// output remains derived metadata inside the immutable receipt; it does
    /// not silently mutate models, layout, or simulation configuration.
    pub(crate) fn execute_project_pdk_callback(
        &mut self,
        callback_id: &str,
        authority: &crate::state::pdk_config::PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<crate::state::pdk_config::ProjectPdkCallbackReceipt, String> {
        for (field, value, maximum) in [
            ("actor ID", authority.actor_id.as_str(), 256_usize),
            ("authority ID", authority.authority_id.as_str(), 256_usize),
            ("reason", reason, 2_048_usize),
        ] {
            if value.is_empty()
                || value != value.trim()
                || value.len() > maximum
                || value.chars().any(char::is_control)
            {
                return Err(format!(
                    "Project callback {field} must be nonempty, trimmed, control-free, and at most {maximum} bytes"
                ));
            }
        }
        self.validate_project_technology_contract()?;

        let project_binding = self
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .expect("validated project technology has an exact binding");
        let signed_pin = project_binding
            .signed_package()
            .expect("validated project technology has an exact signed package");
        let package_binding = crate::state::pdk_config::PdkTechnologyBinding {
            package_id: signed_pin.package_id().to_owned(),
            revision: signed_pin.revision().to_owned(),
            manifest_digest: signed_pin.manifest_digest(),
        };
        let plan = self
            .model
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| {
                format!(
                    "Project callback requires an authoritative active simulation plan: {error}"
                )
            })?;
        let plan_id = plan.id();
        let plan_revision = plan.revision();
        let mut project_parameters = std::collections::BTreeMap::new();
        if let Some(payload) = self.model.workspace.content.plan_data(plan_id) {
            for (index, variable) in payload.design_variables.iter().enumerate() {
                variable.validate().map_err(|error| {
                    format!("Active-plan design variable[{index}] is invalid: {error}")
                })?;
                let value = variable.resolved_value_si().map_err(|error| {
                    format!(
                        "Active-plan design variable '{}' is unresolved: {error}",
                        variable.name
                    )
                })?;
                if project_parameters
                    .insert(variable.name.clone(), format!("{value:.17e}"))
                    .is_some()
                {
                    return Err(format!(
                        "Active plan repeats project callback parameter '{}'",
                        variable.name
                    ));
                }
            }
        }
        let input = crate::state::pdk_config::PdkCallbackExecutionInput { project_parameters };
        let execution = self
            .model
            .pdk_config
            .technology_registry
            .execute_callback_for_binding(
                &package_binding,
                signed_pin.archive_digest(),
                callback_id,
                &input,
            )
            .map_err(|error| format!("Signed PDK callback execution was rejected: {error}"))?;
        let receipt = self
            .model
            .workspace
            .content
            .commit_pdk_callback_execution(
                plan_id,
                plan_revision,
                authority,
                reason,
                input,
                execution,
            )
            .map_err(|error| format!("Signed PDK callback result was not recorded: {error}"))?;

        self.model.workspace.content.project_metadata_dirty = true;
        self.model.design_execution_epoch = self.model.design_execution_epoch.wrapping_add(1);
        self.model.netlist_artifacts.current_generation_input_digest = None;
        self.model.clear_project_design_history();
        Ok(receipt)
    }

    /// Provision one fully authenticated model-source and signed-PDK contract
    /// for tests whose subject requires a successful governed simulation.
    ///
    /// Tests for missing, stale, revoked, or tampered authority must not call
    /// this helper. Keeping it opt-in preserves the production fail-closed
    /// default while avoiding hand-built partial authority in unrelated
    /// simulation fixtures.
    #[cfg(test)]
    pub(crate) fn provision_test_project_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_technology_test_fixture(),
        );
    }

    /// The same contract, with `models` as the signed package's model library.
    #[cfg(test)]
    pub(crate) fn provision_test_project_technology_with_models(&mut self, models: Vec<u8>) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_technology_test_fixture_with_models(models),
        );
    }

    #[cfg(test)]
    pub(crate) fn provision_test_project_veriloga_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_veriloga_technology_test_fixture(),
        );
    }

    #[cfg(test)]
    pub(crate) fn provision_test_project_symbol_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_symbol_technology_test_fixture(),
        );
    }

    #[cfg(test)]
    fn provision_test_project_technology_fixture(
        &mut self,
        (archive, trust, administrative_authority): (
            Vec<u8>,
            crate::state::pdk_config::PdkPublisherTrustStore,
            crate::state::pdk_config::PdkAdministrativeAuthority,
        ),
    ) {
        self.model.pdk_config.publisher_trust_store = trust.clone();
        self.model
            .pdk_config
            .technology_registry
            .install_archive_bytes(
                &archive,
                &trust,
                &administrative_authority,
                "Install deterministic test technology",
            )
            .expect("signed test technology installs");
        let package = self
            .model
            .pdk_config
            .technology_registry
            .validated_packages()[0]
            .clone();

        let library_name = self
            .model
            .model_library_manager
            .load_library_bytes(
                "demo180-test.lib",
                b".model nmos_project_demo nmos level=1 vto=0.55".to_vec(),
                None,
            )
            .expect("authenticated retained test model imports");
        let library = self
            .model
            .model_library_manager
            .get_library(&library_name)
            .expect("imported test model remains in the catalog");
        let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
            .and_then(|binding| binding.with_signed_package_metadata(package.metadata()))
            .expect("model and signed package form one exact test binding");
        let authority = crate::state::ProjectTechnologyChangeAuthority::new(
            "simulation-test@rspice.invalid",
            "test:governed-simulation",
            "Provision deterministic signed simulation authority",
        )
        .expect("test authority is valid");
        let context = crate::state::ProjectTechnologyChangeContext::new(
            authority,
            uuid::Uuid::new_v4(),
            self.model.workspace.content.project.revision(),
            1,
            crate::product::ContentDigest::from_bytes([0x7a; 32]),
            1,
        )
        .expect("test checkpoint evidence is valid");
        self.model
            .workspace
            .content
            .project
            .attach_technology_audited(binding, context)
            .expect("test technology attachment commits with an audit receipt");
        self.validate_project_technology_contract()
            .expect("provisioned test technology resolves exactly");
    }

    /// Whether any retained application workflow owns exclusive keyboard and
    /// pointer intent for this frame: anything modal the shell lists is open.
    pub(crate) fn application_modal_open(&self) -> bool {
        crate::workbench::framework::dialogs::application_modal_open(self)
    }

    pub fn open_project_launcher(&mut self) {
        let route = SurfaceRoute::surface(SurfaceId::ProjectLauncher);
        if self.workbench.current_route() != route
            && let Err(error) = self.workbench.navigate(route, RouteTransitionSource::User)
        {
            self.workbench.record_route_diagnostic(format!(
                "The Project Launcher could not be opened: {error}"
            ));
            return;
        }
        self.project.project_launcher_open = true;
        self.project.project_launcher_page = crate::workbench::state::ProjectLauncherPage::Projects;
        self.platform.recovery_catalog.request_refresh();
        self.workbench
            .outbox
            .push(crate::workbench::framework::requests::FocusProjectLauncherSearch);
    }

    /// Arm the analysis catalogue, from wherever the reader asked for it.
    ///
    /// Beside [`Self::application_modal_open`] because `palette_open` is one of
    /// its terms: setting it withdraws the application's keyboard, and the
    /// three routes that do so — the navigator's creating action, the empty
    /// analysis rail and `Command::AddAnalysis` — each used to write some of
    /// these four fields themselves. A route that forgot the query left the
    /// catalogue filtered by whatever the last search had been.
    pub(crate) fn open_analysis_catalog(&mut self) {
        // On tablet and phone the creating action is hosted by the navigator
        // drawer. Close that transient layer first, so the modal is never
        // obscured by its own invoker.
        self.workbench.close_drawer();
        self.model.sim_setup.session.palette_open = true;
        self.model.sim_setup.session.palette_query.clear();
        self.model.sim_setup.session.palette_active = 0;
        self.model.sim_setup.session.palette_scroll_to_active = true;
    }

    /// Whether static authored state permits a run. Interactive Run commands
    /// intentionally remain available while this is false so the complete
    /// preflight can explain every blocker and offer its remediation route.
    pub fn can_run_simulation(&self) -> bool {
        self.simulation_run_block_reason().is_none()
    }

    /// User-facing reason the Run command is currently blocked.
    pub fn simulation_run_block_reason(&self) -> Option<String> {
        if self.model.simulation.execution.has_active_execution() {
            return Some("A simulation execution is already active".to_string());
        }
        self.simulation_run_preflight_block_reason()
    }

    /// User-facing preflight reason a new run cannot start, excluding the
    /// transient "already running" state so queued re-runs can share it.
    pub fn simulation_run_preflight_block_reason(&self) -> Option<String> {
        run_preflight::run_preflight_block_reason(
            &self.model.sim_setup,
            &self.model.workspace,
            &self.model.schematic,
            &self.model.model_library_manager,
            self.current_blocking_drc_result(),
        )
        .or_else(|| self.technology_gate_block_reason().err())
    }

    /// User-facing reason the Netlist workspace cannot run the current deck.
    pub fn manual_deck_run_block_reason(&self) -> Option<String> {
        if self.model.simulation.execution.has_active_execution() {
            return Some("A simulation execution is already active".to_string());
        }
        let active_document =
            crate::workbench::documents::netlist_document::effective_active_document(self);
        if self.code.netlist.active_dependency_identity.is_some() {
            return Some(
                self.workbench
                    .messages()
                    .text(crate::workbench::MessageId::NetlistDependencyCannotRun),
            );
        }
        let source = crate::workbench::documents::netlist_document::working_deck_source(self);
        if source.trim().is_empty() {
            return Some("Enter a netlist before running".to_string());
        }
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        {
            return Some("Generated comparison documents cannot be executed".to_owned());
        }
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::RunSnapshot
        {
            return Some(
                "The deck a completed run used is sealed with that run; return to the working deck to run it again."
                    .to_owned(),
            );
        }
        let current_digest = crate::state::content_digest(source);
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
            && (self.model.netlist_artifacts.generation_error.is_some()
                || self.model.netlist_artifacts.generated_input_digest
                    != self.model.netlist_artifacts.current_generation_input_digest)
        {
            return Some(
                self.model
                    .netlist_artifacts
                    .generation_error
                    .clone()
                    .unwrap_or_else(|| "Regenerate the stale netlist before running".to_owned()),
            );
        }
        let publication =
            crate::workbench::documents::netlist_document::owned_netlist_publication_state(
                self,
                current_digest,
            );
        if !publication.validated {
            return Some(
                "Validate the current source and project revision before running".to_owned(),
            );
        }
        debug_assert!(publication.run_eligible);
        None
    }

    /// Request a Netlist workspace run, queuing one re-run if the engine is busy.
    pub(crate) fn request_netlist_manual_deck_run(&mut self) {
        self.model.simulation.execution.run_intent = crate::state::SimulationRunIntent::ManualDeck;
        if self.model.simulation.execution.has_active_execution() {
            self.code.netlist.rerun_queued = true;
        } else {
            self.model.simulation.execution.request_manual_deck_run();
        }
    }

    /// Current error-level schematic DRC result that blocks generated runs.
    /// Only an exact, current receipt for the active cell/view can block. The
    /// legacy dialog projection is presentation state and is never execution
    /// authority. Manual deck runs use `manual_deck_run_block_reason` instead.
    pub fn current_blocking_drc_result(&self) -> Option<&rspice_design::drc::DrcResult> {
        match self.active_design_check_status() {
            DesignCheckStatus::Current(receipt) if receipt.result.has_errors() => {
                Some(&receipt.result)
            }
            DesignCheckStatus::NotRun
            | DesignCheckStatus::Current(_)
            | DesignCheckStatus::Stale(_)
            | DesignCheckStatus::Unavailable { .. } => None,
        }
    }

    /// Whether this project is intentionally owned by an authored or imported
    /// netlist-first SPICE deck and has no schematic-derived baseline.
    ///
    /// A pristine bootstrap buffer remains present for legacy document/save
    /// invariants, but it is not a user-visible schematic and must never be
    /// advertised or checked as one.
    pub(crate) fn is_netlist_first_without_schematic(&self) -> bool {
        let netlist_first_deck_owns_the_project = self
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .is_some_and(|document| document.generated_artifact().is_none());
        if !netlist_first_deck_owns_the_project
            || schematic_has_authored_content(&self.model.schematic)
        {
            return false;
        }

        self.model.workspace.content.schematic_buffers.len() <= 1
            && self
                .model
                .workspace
                .content
                .schematic_buffers
                .values()
                .all(|schematic| !schematic_has_authored_content(schematic))
    }

    /// Request a run from the Simulate workspace run set.
    pub fn request_run_set_simulation(&mut self) {
        self.code.netlist.rerun_queued = false;
        self.model.simulation.execution.request_simulate_run_set();
    }

    /// Answer the gesture just made, on the status bar.
    ///
    /// For a refusal or transient feedback: "The schematic is read-only.",
    /// "Select a retained result first." It is shown until the next prompt
    /// replaces it or it expires, and it is never logged or notified, because
    /// it is about the gesture, not an event in the session.
    pub fn prompt(&mut self, text: impl Into<String>) {
        self.workbench.prompt = Some(crate::workbench::state::StatusPrompt {
            text: text.into(),
            issued: crate::time_compat::Instant::now(),
        });
    }

    /// Close a route-owned transient surface without stranding its canonical
    /// URL. Ordinary opens return to their exact source; a direct deep link
    /// replaces itself with the currently projected primary workspace.
    pub(crate) fn close_route_owned_transient(
        &mut self,
        surface: crate::workbench::SurfaceId,
        label: &str,
    ) {
        if self.workbench.current_route().surface_id() != surface {
            return;
        }
        if self
            .workbench
            .navigate_back(crate::workbench::RouteTransitionSource::User)
            .is_some()
        {
            return;
        }
        let fallback = crate::workbench::SurfaceRoute::surface(
            crate::workbench::SurfaceId::from_workspace(self.workbench.workspace),
        );
        if let Err(error) = self
            .workbench
            .replace_route(fallback, crate::workbench::RouteTransitionSource::User)
        {
            self.prompt(format!("Could not close {label}: {error}"));
        }
    }

    /// The prompt on the status bar, if one is showing.
    #[cfg(test)]
    pub(crate) fn prompt_text(&self) -> Option<&str> {
        self.workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.as_str())
    }

    /// Record an event in the Console and return its log id.
    ///
    /// Error: something the user asked for, or a background job, failed.
    /// Warning: it completed, but the reader must know something. Info: a
    /// record of what happened. A refusal is never recorded; it is a
    /// [`Self::prompt`].
    pub fn record(
        &mut self,
        severity: crate::diagnostics::LogSeverity,
        source: crate::diagnostics::LogSource,
        text: impl Into<String>,
    ) -> u64 {
        self.model.record(severity, source, text)
    }

    /// Report a failure of something the user asked for: a save, an open, an
    /// export. The Console keeps the line; the toast says it now and offers
    /// that line, so the mirrored copy of it is the same notice, not another.
    pub(crate) fn report_failure(&mut self, title: impl Into<String>, detail: impl Into<String>) {
        self.model.report_failure(title, detail);
    }

    /// [`Self::report_failure`] for something that completed, but not the way
    /// the user asked: saved as a download, not kept for the next session.
    pub(crate) fn report_warning(&mut self, title: impl Into<String>, detail: impl Into<String>) {
        self.model.report_warning(title, detail);
    }

    /// Bring the reader to the findings a pass or a check just filed: the
    /// Problems page, listing them. A filter left from earlier reading could
    /// hide the very findings that stopped a run, or that a check the reader
    /// asked for found, so the filters are cleared.
    // A workbench write the checks ask for; it becomes a request.
    pub(crate) fn show_problem_findings(&mut self) {
        self.workbench.console_visible = true;
        self.workbench.console_page = crate::workbench::state::ConsolePage::Problems;
        self.problems_view.clear_filters();
    }

    /// Raise every reported notice as a toast, in the order reported.
    ///
    /// The one way a notice reaches the toasts. The shell calls it at the start
    /// of the frame and before anything that shows the toasts or reads what
    /// they hold, so a notice is seen in the frame it would have been seen had
    /// it been toasted when reported; a test that asserts a toast after a
    /// report calls it the same way. Anything drawn earlier in the frame that
    /// counts the toasts, the bell's unread count, catches up on the next.
    pub(crate) fn drain_notices(&mut self, ctx: &egui::Context) {
        use crate::ui::widgets::{NotificationAction, NotificationCategory, ToastKind};

        let notices = self.model.notices.take();
        if !notices.is_empty() {
            ctx.request_repaint();
        }
        for notice in notices {
            let kind = match notice.kind {
                NoticeKind::Error => ToastKind::Error,
                NoticeKind::Warning => ToastKind::Warn,
            };
            self.workbench.toasts.notify_with_action(
                ctx,
                NotificationCategory::System,
                kind,
                notice.title,
                notice.detail,
                NotificationAction::ShowInConsole {
                    log_id: notice.log_id,
                },
            );
        }
    }

    /// Announce a committed plan command on the lifecycle status line and
    /// record it in the Console.
    ///
    /// One funnel for every announcing site, because a plan command's outcome
    /// has two readers with different jobs: the status line states the latest
    /// outcome, and the Console is the session's log of what was commanded,
    /// which is where an operator reconstructs an hour of plan edits
    /// afterwards. A receipt used to reach only the first of those, so a
    /// committed plan edit left no durable record anywhere.
    ///
    /// [`crate::diagnostics::LogSource::Plan`] is what keeps the second reader
    /// from becoming noise: the activity mirror never lifts a `Plan` entry, so
    /// recording a receipt cannot turn a routine registry edit into a
    /// notification.
    ///
    /// Recorded only when the outcome changes. Three announcing sites sit on
    /// the render path and restate a standing refusal every frame, and the
    /// price of not writing that refusal sixty times a second is that two
    /// consecutive identical receipts leave one record.
    pub(crate) fn record_plan_receipt(&mut self, message: impl Into<String>) {
        let message = message.into();
        if self
            .simulate
            .analysis_lifecycle_status
            .record_receipt(message.as_str())
        {
            self.model.log_buffer.log(
                crate::diagnostics::LogSeverity::Info,
                crate::diagnostics::LogSource::Plan,
                message,
            );
        }
    }

    /// Announce a refused plan command, as [`Self::record_plan_receipt`] does,
    /// at warning severity, and answer the gesture on the status bar.
    ///
    /// The lifecycle status line is drawn only on the Analyses route, and a
    /// plan command can be refused from the plan manager in any workspace, so
    /// the prompt is what answers the user wherever they are. It is issued
    /// with the record, when the outcome changes, because a standing refusal
    /// restated every frame would otherwise hold the prompt up forever.
    pub(crate) fn record_plan_refusal(&mut self, message: impl Into<String>) {
        let message = message.into();
        if self
            .simulate
            .analysis_lifecycle_status
            .record_refusal(message.as_str())
        {
            self.prompt(message.clone());
            self.model.log_buffer.log(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Plan,
                message,
            );
        }
    }

    /// Select a model library, reporting a name the project no longer holds.
    ///
    /// One reporter for a refusal every caller would otherwise have to spell
    /// itself. Returns whether the selection took, so a caller that was about
    /// to write a selection *inside* that library can decline to.
    pub(crate) fn select_model_library(&mut self, name: &str) -> bool {
        match self.model.model_library_manager.select_library(name) {
            Ok(()) => true,
            Err(refusal) => {
                self.prompt(refusal);
                false
            }
        }
    }

    pub fn clear_primary_log(&mut self) {
        self.model.log_buffer.clear();
    }

    /// Record a file in the recent-files list (most recent first, deduped,
    /// capped at Projects and files ▸ Recent projects listed).
    pub(crate) fn remember_recent_file(&mut self, kind: RecentKind, path: &std::path::Path) {
        let previous = self
            .platform
            .recent_files
            .iter()
            .find(|recent| recent.path == path)
            .cloned();
        let entry = RecentFile {
            kind,
            path: path.to_path_buf(),
            opened_at_unix_ms: crate::time_compat::unix_epoch()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX),
            pinned: previous.as_ref().is_some_and(|recent| recent.pinned),
            owner: previous.as_ref().and_then(|recent| recent.owner.clone()),
            tags: previous.map_or_else(Vec::new, |recent| recent.tags),
        };
        self.platform
            .recent_files
            .retain(|recent| recent.path != entry.path);
        self.platform.recent_files.insert(0, entry);
        self.trim_recent_files();
    }

    /// Drop the oldest recent projects beyond what Projects and files ▸
    /// Recent projects listed keeps, so lowering it shortens the start page's
    /// list at once. The count is of projects, the only kind that list shows:
    /// schematics are capped at the same count on their own, so opening them
    /// never pushes a project off the page.
    pub(crate) fn trim_recent_files(&mut self) {
        let listed = self.model.preferences.recent_files_listed();
        let (mut projects, mut schematics) = (0, 0);
        self.platform.recent_files.retain(|recent| {
            let kept = match recent.kind {
                RecentKind::Project => &mut projects,
                RecentKind::Schematic => &mut schematics,
            };
            *kept += 1;
            *kept <= listed
        });
    }

    /// Change a project pin without changing its recency or file identity.
    #[cfg(test)]
    pub(crate) fn set_recent_project_pinned(
        &mut self,
        path: &std::path::Path,
        pinned: bool,
    ) -> bool {
        let Some(recent) = self
            .platform
            .recent_files
            .iter_mut()
            .find(|recent| recent.kind == RecentKind::Project && recent.path == path)
        else {
            return false;
        };
        recent.pinned = pinned;
        true
    }

    /// Where a paste should land, snapped to the schematic grid: under the
    /// cursor when it hovers the canvas, otherwise the center of the visible
    /// canvas (menu-driven paste), otherwise a sane fixed spot.
    pub(crate) fn schematic_paste_anchor(&self) -> crate::state::Point {
        let grid = self.model.schematic.document().grid_size.max(1);
        let (x, y) = self
            .design
            .canvas_hover
            .or(self.design.canvas_view_center)
            .unwrap_or((20.0, 20.0));
        crate::state::Point::new((x.round() as i32) * grid, (y.round() as i32) * grid)
    }

    /// Whether a schematic edit is under way: an undo step still open, as
    /// while a drag is held, or a copy floating under the pointer. History
    /// waits for either to land or be cancelled.
    pub(crate) fn schematic_edit_in_progress(&self) -> bool {
        self.model.schematic.has_pending_operation() || self.design.floating_paste.is_some()
    }
}

fn schematic_has_authored_content(
    schematic: &impl AsRef<rspice_design::schematic::document::SchematicDocument>,
) -> bool {
    let schematic = schematic.as_ref();
    !schematic.components.is_empty()
        || !schematic.wires.is_empty()
        || !schematic.buses.is_empty()
        || !schematic.bus_taps.is_empty()
        || !schematic.design_notes.is_empty()
        || !schematic.documentation_shapes.is_empty()
        || !schematic.probes.is_empty()
        || !schematic.net_labels.is_empty()
        || !schematic.junctions.is_empty()
        || !schematic.connections.is_empty()
        || !schematic.validated_revisions.is_empty()
}

impl crate::state::command_catalog::ShortcutContext {
    /// Whether this context is live for the workspace and document in front
    /// of the reader, as menus and the palette ask it.
    pub fn matches(self, workbench: &crate::workbench::WorkbenchSession, model: &Model) -> bool {
        use crate::state::ViewType;
        use crate::workbench::state::Workspace;

        match self {
            Self::Global | Self::ApplicationChrome | Self::RunnableProject => true,
            // Results is here for the same reason the design canvases are:
            // the magnification and copy commands are carried out on whatever
            // the reader is looking at, and a result sheet is one of those
            // surfaces. Membership is not permission — every command in this
            // context still answers `is_enabled` for the surface in front of
            // it, and the schematic-editing family answers `false` on a
            // result sheet.
            Self::EditContext => {
                workbench.workspace == Workspace::Results
                    || match model.workspace.content.active_view_type() {
                        ViewType::Schematic | ViewType::Testbench => {
                            workbench.workspace == Workspace::Design
                        }
                        ViewType::Symbol => {
                            matches!(workbench.workspace, Workspace::Design | Workspace::Models)
                        }
                        _ => false,
                    }
            }
            Self::EngineeringCanvas => {
                workbench.workspace == Workspace::Design
                    && matches!(
                        model.workspace.content.active_view_type(),
                        ViewType::Schematic | ViewType::Testbench
                    )
            }
            Self::SymbolCanvas => {
                matches!(workbench.workspace, Workspace::Design | Workspace::Models)
                    && model.workspace.content.active_view_type() == ViewType::Symbol
            }
            Self::DesignWorkspace => workbench.workspace == Workspace::Design,
            Self::SimulationWorkspace => workbench.workspace == Workspace::Simulate,
            Self::ResultsWorkspace => workbench.workspace == Workspace::Results,
            Self::VerificationWorkspace => workbench.workspace == Workspace::Verify,
            Self::ViolationNavigation => {
                matches!(workbench.workspace, Workspace::Design | Workspace::Verify)
            }
            Self::SourceEditor => workbench.active_source_editor.is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_project_refresh_preserves_launcher_metadata() {
        let mut state = AppState::default();
        let path = std::path::Path::new("C:/Engineering/afe.rspiceproj");
        state.remember_recent_file(RecentKind::Project, path);
        state.platform.recent_files[0].pinned = true;
        state.platform.recent_files[0].owner = Some("Analog Design".to_owned());
        state.platform.recent_files[0].tags = vec!["afe".to_owned(), "release".to_owned()];
        let first_timestamp = state.platform.recent_files[0].opened_at_unix_ms;

        state.remember_recent_file(RecentKind::Project, path);

        let recent = &state.platform.recent_files[0];
        assert!(recent.pinned);
        assert_eq!(recent.owner.as_deref(), Some("Analog Design"));
        assert_eq!(recent.tags, ["afe", "release"]);
        assert!(recent.opened_at_unix_ms >= first_timestamp);
    }

    #[test]
    fn pinning_is_project_only_and_does_not_reorder_recents() {
        let mut state = AppState::default();
        let project = std::path::Path::new("C:/Engineering/afe.rspiceproj");
        let schematic = std::path::Path::new("C:/Engineering/standalone.rsch");
        state.remember_recent_file(RecentKind::Project, project);
        state.remember_recent_file(RecentKind::Schematic, schematic);

        assert!(state.set_recent_project_pinned(project, true));
        assert!(!state.set_recent_project_pinned(schematic, true));
        assert_eq!(state.platform.recent_files[0].path, schematic);
        assert_eq!(state.platform.recent_files[1].path, project);
        assert!(state.platform.recent_files[1].pinned);
    }

    /// The list keeps as many projects as Recent projects listed asks,
    /// newest first, and lowering the count shortens it at once.
    #[test]
    fn the_recent_list_keeps_as_many_projects_as_preferences_ask() {
        let mut state = AppState::default();
        let path =
            |index: usize| std::path::PathBuf::from(format!("C:/Engineering/{index}.rspiceproj"));
        for index in 0..10 {
            state.remember_recent_file(RecentKind::Project, &path(index));
        }
        assert_eq!(
            state.platform.recent_files.len(),
            8,
            "the shipped count is 8"
        );

        state
            .model
            .preferences
            .set_scalar(crate::workbench::ScalarPreference::RecentFilesListed, 12)
            .expect("12 is an offered count");
        for index in 10..20 {
            state.remember_recent_file(RecentKind::Project, &path(index));
        }
        assert_eq!(state.platform.recent_files.len(), 12);

        state
            .model
            .preferences
            .set_scalar(crate::workbench::ScalarPreference::RecentFilesListed, 5)
            .expect("5 is an offered count");
        state.trim_recent_files();
        let kept = state
            .platform
            .recent_files
            .iter()
            .map(|recent| recent.path.clone())
            .collect::<Vec<_>>();
        assert_eq!(kept, (15..20).rev().map(path).collect::<Vec<_>>());
    }

    #[test]
    fn exact_project_callback_execution_commits_one_validated_project_receipt() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        let plan_id = state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("active plan")
            .id();
        state
            .model
            .workspace
            .content
            .add_design_variable(
                plan_id,
                crate::state::DesignVariable::new(
                    "gain",
                    "2.5",
                    crate::state::DesignVariableQuantity::Dimensionless,
                    crate::state::DesignVariableScope::Project,
                    "Callback input fixture",
                    None,
                    crate::state::DesignVariableSweepEligibility::FixedParameter,
                    crate::state::DesignVariableOverridePolicy::InheritOwnerOnly,
                )
                .expect("valid project variable"),
            )
            .expect("attach project variable to active plan");
        let baseline = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
            .expect("valid callback baseline");
        crate::workbench::app_state::services_access::accept_loaded_project(
            &mut state, baseline, None,
        );
        assert!(!crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
        let from_revision = state.model.workspace.content.project.revision();
        let authority = crate::state::pdk_config::PdkAdministrativeAuthority {
            actor_id: "callback-operator@rspice.invalid".to_owned(),
            authority_id: "test:project-callback".to_owned(),
        };

        let receipt = state
            .execute_project_pdk_callback(
                "derive-device",
                &authority,
                "Retain deterministic project callback evidence",
            )
            .expect("exact project callback executes and commits");

        assert_eq!(receipt.sequence, 1);
        assert_eq!(receipt.from_project_revision, from_revision);
        assert_eq!(
            receipt.to_project_revision,
            state.model.workspace.content.project.revision()
        );
        assert_eq!(receipt.execution.callback_id, "derive-device");
        assert!(receipt.execution.derived_metadata.is_empty());
        assert_eq!(
            receipt.input.project_parameters.get("gain"),
            Some(&"2.50000000000000000e0".to_owned())
        );
        assert!(crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
        state.workbench.workspace = crate::workbench::state::Workspace::Project;
        assert!(crate::workbench::lifecycle::project_lifecycle::active_document_is_dirty(&state));

        let second = state
            .execute_project_pdk_callback(
                "derive-device",
                &authority,
                "Retain a second deterministic callback receipt",
            )
            .expect("second exact project callback executes and commits");
        assert_eq!(second.sequence, 2);
        assert_eq!(second.from_project_revision, receipt.to_project_revision);
        assert_eq!(second.previous_receipt_digest, Some(receipt.receipt_digest));
        assert_eq!(
            state.model.workspace.content.pdk_callback_receipts(),
            [receipt.clone(), second.clone()]
        );
        state
            .model
            .workspace
            .content
            .validate_pdk_callback_receipts()
            .expect("project callback ledger validates");
        assert!(state.model.workspace.content.project_metadata_dirty);

        let encoded =
            serde_json::to_vec(&state.model.workspace).expect("serialize callback evidence");
        let restored: crate::state::ProjectWorkspace =
            serde_json::from_slice(&encoded).expect("restore callback evidence");
        assert_eq!(
            restored.content.pdk_callback_receipts(),
            [receipt.clone(), second]
        );
        restored
            .content
            .validate_pdk_callback_receipts()
            .expect("restored callback ledger validates");

        let mut tampered: serde_json::Value =
            serde_json::from_slice(&encoded).expect("decode callback evidence for tamper test");
        tampered["pdk_callback_receipts"][0]["reason"] =
            serde_json::Value::String("tampered callback reason".to_owned());
        let tampered: crate::state::ProjectWorkspace =
            serde_json::from_value(tampered).expect("structurally valid tampered workspace");
        assert!(tampered.content.validate_pdk_callback_receipts().is_err());
    }

    #[test]
    fn project_callback_failure_is_transactional_without_an_exact_project_pin() {
        let mut state = AppState::default();
        let revision = state.model.workspace.content.project.revision();
        let authority = crate::state::pdk_config::PdkAdministrativeAuthority {
            actor_id: "callback-operator@rspice.invalid".to_owned(),
            authority_id: "test:project-callback".to_owned(),
        };

        let error = state
            .execute_project_pdk_callback(
                "derive-device",
                &authority,
                "This must fail without an exact project PDK pin",
            )
            .expect_err("missing exact project pin must fail closed");

        assert!(error.contains("no exact authenticated"), "{error}");
        assert_eq!(state.model.workspace.content.project.revision(), revision);
        assert!(
            state
                .model
                .workspace
                .content
                .pdk_callback_receipts()
                .is_empty()
        );
        assert!(!state.model.workspace.content.project_metadata_dirty);
    }

    #[test]
    fn physical_layout_initialization_uses_exact_signed_project_dbu_and_persists() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        let owner = crate::state::CellViewRef::new("user", "top", "layout");
        state
            .model
            .library_manager
            .get_library_mut("user")
            .expect("default project library")
            .get_or_create_cell("top")
            .add_view(crate::state::View::new(
                "layout",
                crate::state::ViewType::Layout,
            ));

        let revision = state
            .initialize_physical_layout_document(owner.clone())
            .expect("initialize exact signed-PDK physical layout");
        let document = state
            .model
            .workspace
            .content
            .physical_layout_document(&owner)
            .expect("authoritative layout document");
        assert_eq!(document.revision(), revision);
        assert_eq!(document.technology().database_unit().metres(), 1.0e-9);
        assert_eq!(document.technology().stack_id(), "1P2M");

        let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
            .expect("layout project snapshot validates");
        let encoded = serde_json::to_vec(&snapshot).expect("serialize layout project");
        let restored: crate::io::ProjectSnapshot =
            serde_json::from_slice(&encoded).expect("deserialize layout project");
        restored
            .file
            .validate()
            .expect("restored layout project validates");

        let mut tampered: serde_json::Value =
            serde_json::from_slice(&encoded).expect("decode layout project");
        tampered["workspace"]["physical_layout_documents"][owner.key()]["technology"]["archive_digest"] =
            serde_json::Value::String("00".repeat(32));
        let tampered: crate::io::ProjectSnapshot =
            serde_json::from_value(tampered).expect("deserialize structurally valid tamper");
        assert!(tampered.file.validate().is_err());
    }

    #[test]
    fn physical_layout_edits_commit_atomically_and_reject_unsigned_layer_purposes() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        let owner = crate::state::CellViewRef::new("user", "top", "layout");
        state
            .model
            .library_manager
            .get_library_mut("user")
            .expect("default project library")
            .get_or_create_cell("top")
            .add_view(crate::state::View::new(
                "layout",
                crate::state::ViewType::Layout,
            ));
        state
            .model
            .workspace
            .open_view(owner.clone(), crate::state::ViewType::Layout);
        let start = state
            .initialize_physical_layout_document(owner.clone())
            .expect("initialize exact signed-PDK physical layout");
        state
            .model
            .library_manager
            .get_library_mut("user")
            .unwrap()
            .get_cell_mut("top")
            .unwrap()
            .get_view_mut("layout")
            .unwrap()
            .modified = false;
        state
            .model
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == owner)
            .unwrap()
            .dirty = false;

        let valid_shape = crate::state::LayoutObjectId::new();
        let committed = state
            .apply_physical_layout_transaction(
                &owner,
                start,
                &[crate::state::LayoutEdit::InsertShape {
                    id: valid_shape,
                    value: crate::state::LayoutShape {
                        layer_purpose: crate::state::LayoutLayerPurpose::try_new(
                            "metal1", "drawing",
                        )
                        .unwrap(),
                        geometry: crate::state::LayoutGeometry::Rectangle {
                            lower_left: crate::state::LayoutPoint::new(0, 0),
                            upper_right: crate::state::LayoutPoint::new(100, 200),
                        },
                        net: None,
                        properties: std::collections::BTreeMap::new(),
                    },
                }],
            )
            .expect("signed layer/purpose edit commits");
        assert_eq!(committed, start.next().unwrap());
        assert!(
            state
                .model
                .workspace
                .content
                .physical_layout_document(&owner)
                .unwrap()
                .shapes()
                .contains_key(&valid_shape)
        );
        assert!(
            state
                .model
                .library_manager
                .get_library("user")
                .unwrap()
                .get_cell("top")
                .unwrap()
                .get_view("layout")
                .unwrap()
                .modified
        );
        assert!(
            state
                .model
                .workspace
                .content
                .open_views
                .iter()
                .find(|open| open.reference == owner)
                .unwrap()
                .dirty
        );

        let invalid_shape = crate::state::LayoutObjectId::new();
        let error = state
            .apply_physical_layout_transaction(
                &owner,
                committed,
                &[crate::state::LayoutEdit::InsertShape {
                    id: invalid_shape,
                    value: crate::state::LayoutShape {
                        layer_purpose: crate::state::LayoutLayerPurpose::try_new(
                            "invented", "drawing",
                        )
                        .unwrap(),
                        geometry: crate::state::LayoutGeometry::Rectangle {
                            lower_left: crate::state::LayoutPoint::new(0, 0),
                            upper_right: crate::state::LayoutPoint::new(10, 10),
                        },
                        net: None,
                        properties: std::collections::BTreeMap::new(),
                    },
                }],
            )
            .expect_err("layer outside exact signed PDK must fail closed");
        assert!(
            error.contains("which the project's signed PDK does not define"),
            "{error}"
        );
        let unchanged = state
            .model
            .workspace
            .content
            .physical_layout_document(&owner)
            .unwrap();
        assert_eq!(unchanged.revision(), committed);
        assert_eq!(unchanged.shapes().len(), 1);
        assert!(!unchanged.shapes().contains_key(&invalid_shape));

        let mut imported = unchanged.clone();
        imported
            .apply_transaction(
                committed,
                &[crate::state::LayoutEdit::InsertShape {
                    id: invalid_shape,
                    value: crate::state::LayoutShape {
                        layer_purpose: crate::state::LayoutLayerPurpose::try_new(
                            "invented", "drawing",
                        )
                        .unwrap(),
                        geometry: crate::state::LayoutGeometry::Rectangle {
                            lower_left: crate::state::LayoutPoint::new(0, 0),
                            upper_right: crate::state::LayoutPoint::new(10, 10),
                        },
                        net: None,
                        properties: std::collections::BTreeMap::new(),
                    },
                }],
            )
            .expect("layout-domain import accepts syntactically valid package-foreign layer");
        state
            .model
            .workspace
            .content
            .commit_physical_layout_document(imported)
            .expect("simulate an imported physical-layout document");
        let imported_error = state
            .validate_project_technology_contract()
            .expect_err("project authority must reject imported package-foreign layers");
        assert!(
            imported_error.contains("outside the exact signed project PDK"),
            "{imported_error}"
        );
    }
}
