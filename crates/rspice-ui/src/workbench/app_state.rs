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

pub(in crate::workbench) mod active_viewer;
pub(in crate::workbench) mod design_checks;
pub(in crate::workbench) mod design_history;
pub(in crate::workbench) mod interaction_state;
pub(crate) mod run_preflight;
pub(in crate::workbench) mod session;
pub(in crate::workbench) mod sim_setup;
pub(in crate::workbench) mod viewer_capabilities;

use crate::diagnostics::{ConsoleLevel, ConsoleMessage};
use crate::product::{AnalysisInstanceId, DatasetId};
use crate::state::{SchematicState, SimulationState};
use crate::workbench::app::DialogState;

pub use active_viewer::ActiveViewer;
pub(crate) use design_checks::{DesignCheckOrigin, DesignCheckRuntime, DesignCheckStatus};
pub(crate) use design_history::{
    DesignManagementHistoryEntry, SymbolDefinitionFixtureDelta,
    publish_symbol_definition_candidate, publish_symbol_definition_candidate_with_fixture,
};
pub(crate) use interaction_state::SchematicKeyboardFocus;
pub use interaction_state::{ContextTarget, DragType, InteractionState};
pub(crate) use session::shortcuts::{
    accessibility_shortcut_summary, report_engineering_canvas_focus, runtime_command_platform,
};
pub(crate) use session::state_init::default_model_library_manager;
pub use sim_setup::plan_catalog::{
    SimulationPlanCloneOptions, SimulationPlanLineage, SimulationPlanName, StoredSimulationPlan,
};
pub use sim_setup::{AcSetup, DcSetup, NoiseSetup, ReferencePvtPoint, SimSetupState, TranSetup};

/// Exact owner of one mutable specialized-viewer cache.
///
/// Current results use their prepared analysis-instance identity. Migrated
/// result history that predates prepared-task provenance falls back to the
/// retained run-local analysis ID; paired with the immutable dataset ID this
/// is still an exact identity and never aliases another dataset's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpecializedViewerCacheProvenance {
    pub(crate) dataset_id: DatasetId,
    pub(crate) analysis_identity: SpecializedViewerAnalysisIdentity,
}

impl SpecializedViewerCacheProvenance {
    pub(crate) fn for_analysis(
        dataset_id: DatasetId,
        analysis: &crate::state::AnalysisResult,
    ) -> Self {
        let analysis_identity = analysis.provenance().map_or(
            SpecializedViewerAnalysisIdentity::LegacyResultId(analysis.id),
            |provenance| {
                SpecializedViewerAnalysisIdentity::Prepared(provenance.source_instance_id())
            },
        );
        Self {
            dataset_id,
            analysis_identity,
        }
    }

    pub(crate) const fn for_prepared_analysis(
        dataset_id: DatasetId,
        source_instance_id: AnalysisInstanceId,
    ) -> Self {
        Self {
            dataset_id,
            analysis_identity: SpecializedViewerAnalysisIdentity::Prepared(source_instance_id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpecializedViewerAnalysisIdentity {
    Prepared(AnalysisInstanceId),
    LegacyResultId(u64),
}

/// Provenance for the six mutable specialized result caches.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SpecializedViewerCacheAuthority {
    pub(crate) bode: Option<SpecializedViewerCacheProvenance>,
    pub(crate) nyquist: Option<SpecializedViewerCacheProvenance>,
    pub(crate) smith: Option<SpecializedViewerCacheProvenance>,
    pub(crate) histogram: Option<SpecializedViewerCacheProvenance>,
    pub(crate) fft: Option<SpecializedViewerCacheProvenance>,
    pub(crate) eye: Option<SpecializedViewerCacheProvenance>,
}

/// Analysis viewer state grouped behind a dedicated workspace surface.
#[derive(Clone, Default)]
pub struct AnalysisWorkspaceState {
    /// Pole-Zero viewer state
    pub(crate) pole_zero_state: crate::analysis::pole_zero::PoleZeroState,
    /// Bode viewer state
    pub(crate) bode_plot_state: crate::analysis::bode::BodePlotState,
    /// Nyquist viewer state
    pub(crate) nyquist_state: crate::analysis::nyquist::NyquistState,
    /// Eye diagram viewer state
    pub(crate) eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState,
    /// FFT viewer state
    pub(crate) fft_state: crate::analysis::fft::FftState,
    /// Smith chart viewer state
    pub(crate) smith_chart_state: crate::analysis::smith_chart::SmithChartState,
    /// Histogram viewer state
    pub(crate) histogram_state: crate::analysis::histogram::HistogramState,
    /// Exact immutable result that owns each mutable viewer cache.
    pub(crate) cache_authority: SpecializedViewerCacheAuthority,
}

/// What kind of document a recent-files entry points at. `.json` is a valid
/// extension for both schematics and projects, so the kind is recorded at
/// open/save time instead of being inferred from the path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum RecentKind {
    Schematic,
    Project,
}

/// One entry in the File ▸ Open recent list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct RecentFile {
    pub kind: RecentKind,
    pub path: std::path::PathBuf,
    /// Milliseconds since the Unix epoch when this entry most recently
    /// became active. Legacy sessions deserialize to zero and retain their
    /// stored list order.
    #[serde(default)]
    pub opened_at_unix_ms: u64,
    /// User pin state for Project Launcher filtering.
    #[serde(default)]
    pub pinned: bool,
    /// Governed project owner. This is never guessed from the operating-
    /// system account when project metadata does not provide it.
    #[serde(default)]
    pub owner: Option<String>,
    /// Searchable project classifications retained by the launcher.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl RecentFile {
    #[cfg(test)]
    pub(crate) fn is_shared_project(&self) -> bool {
        self.kind == RecentKind::Project && path_is_shared(&self.path)
    }
}

#[cfg(test)]
fn path_is_shared(path: &std::path::Path) -> bool {
    let text = path.as_os_str().to_string_lossy();
    text.starts_with(r"\\") || text.starts_with("//")
}

/// Main application state container
#[derive(Clone)]
pub struct AppState {
    /// Circuit schematic state (components, wires, topology)
    pub(crate) schematic: SchematicState,
    /// Simulation results and waveforms
    pub(crate) simulation: SimulationState,
    /// Monotonic token that invalidates in-flight controller work whenever an
    /// unrelated design document replaces the active project/schematic.
    pub(crate) design_execution_epoch: u64,
    /// Monotonic identity for the active schematic buffer. Unlike the
    /// execution epoch, this advances on ordinary cell/view navigation so an
    /// A→B→A round trip cannot revive a stale editor transaction.
    pub(crate) active_schematic_epoch: u64,
    /// Runtime-only cross-document design transactions. Schematic-local undo
    /// cannot own operations that also create cells, views, or buffers.
    pub(crate) project_design_history: design_history::ProjectDesignHistory,
    /// Per-cell-view interactive check receipts, bound to exact live inputs.
    pub(crate) design_checks: DesignCheckRuntime,
    /// Dialog visibility
    pub(crate) dialogs: DialogState,
    /// Typed analysis configuration behind the Simulate view.
    pub(crate) sim_setup: SimSetupState,
    /// Structured log history buffer (ring-buffer, filterable).
    pub(crate) log_buffer: crate::diagnostics::LogBuffer,
    /// Scripting/Automation console state
    pub(crate) script_console: session::script_console::ScriptConsoleState,
    /// Library/Cell/View manager for design hierarchy
    pub(crate) library_manager: crate::state::LibraryManager,
    /// Runtime collaboration-lock authority for governed project library
    /// mutations. Local projects remain explicitly unmanaged; connected
    /// sessions install revision-bound authoritative snapshots.
    pub(crate) library_edit_locks: crate::state::ProjectLibraryLockAuthority,
    /// Project/workspace model for active design context and open LCV views.
    pub(crate) workspace: crate::state::ProjectWorkspace,
    /// Transactional document registry, accepted project baseline, and
    /// canonical persistence identity. Runtime-only; session recovery keeps
    /// the working set separately and reconstructs this boundary at startup.
    pub(crate) project_lifecycle:
        crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState,
    /// Pending cell deletion (library, cell_name)
    pub(crate) pending_delete_cell: Option<(String, String)>,
    /// Pending view deletion (library, cell, view_name)
    pub(crate) pending_delete_view: Option<(String, String, String)>,
    /// Tabbed property dialog state (commercial-grade property editing)
    pub(crate) tabbed_property_dialog: crate::properties::TabbedPropertyDialogState,
    /// Property registry (component property schemas)
    pub(crate) property_registry: crate::state::PropertyRegistry,
    /// Calculator panel state
    pub(crate) calculator_panel: session::calculator::CalculatorPanel,
    /// PDK Settings dialog state
    pub(crate) pdk_settings_dialog: session::pdk_settings::PdkSettingsDialogState,
    /// PDK model paths plus signed technology packages, trust roots, exact
    /// active binding, and administrative receipts.
    pub(crate) pdk_config: crate::state::pdk_config::PdkConfig,
    /// Model library manager (PDK models, device libraries)
    pub(crate) model_library_manager: crate::state::model_library::ModelLibraryManager,
    /// Standalone model browser state (for Tools menu access)
    pub(crate) model_browser_state: crate::properties::model_browser::ModelBrowserState,
    /// Flag to signal that application exit has been requested (after confirmation)
    pub(crate) exit_requested: bool,
    /// Recently opened/saved schematic and project files, most recent first
    /// (persisted across sessions; drives File ▸ Open recent).
    pub(crate) recent_files: Vec<RecentFile>,
    /// Browser-only suggested filename for the next schematic download.
    pub(crate) browser_schematic_save_name: Option<String>,
    /// Browser-only suggested filename for the next project download.
    pub(crate) browser_project_save_name: Option<String>,
    /// Exact native canonical-file authority saved with the working session.
    /// A restored pathname alone never grants ordinary Save authority.
    pub(crate) native_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::NativeBindingReceipt>,
    /// Exact browser canonical-binding authority saved with the working-set
    /// session.  The binding UUID is opaque and intentionally independent of
    /// the logical project UUID. Runtime file handles are never serialized.
    pub(crate) browser_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::BrowserBindingReceipt>,
    /// The activated license key as pasted (persisted; re-verified on load).
    pub(crate) license_key: Option<String>,
    /// The verified grant behind `license_key` (derived, never persisted).
    pub(crate) license: Option<crate::services::license::LicenseInfo>,
    /// Specialized analysis viewer state grouped by analysis workspace.
    pub(crate) analysis: AnalysisWorkspaceState,
    /// Document-engine and interaction session state (theme, canvas, result
    /// viewers, symbol editor, and netlist editor).
    pub(crate) ui: crate::workbench::UiSessionState,
    /// Canonical workbench navigation and responsive layout state.
    pub(crate) workbench: crate::workbench::WorkbenchState,
    /// Runtime-only chord/prefix authority. Partial sequences are never
    /// serialized or restored across application sessions.
    pub(crate) shortcut_resolver: session::shortcuts::ShortcutResolverState,
    /// Canonical device-local shortcut-library CAS authority. The recoverable
    /// eframe session copy never replaces this owner after startup.
    pub(crate) shortcut_library_persistence:
        session::shortcut_library::ShortcutLibraryPersistenceRuntime,
    /// UI transaction paired with the single browser shortcut-library CAS.
    pub(crate) shortcut_library_publication_continuation:
        Option<session::shortcut_library::ShortcutLibraryPublicationContinuation>,
}

impl Default for AppState {
    fn default() -> Self {
        session::state_init::default_app_state()
    }
}

impl AppState {
    /// Reserve the next project revision before a library membership
    /// transaction publishes any live state.
    pub(crate) fn preflight_project_library_mutation(
        &self,
        mutation: crate::state::ProjectLibraryMutation,
    ) -> Result<crate::state::PreparedProjectLibraryMutation, String> {
        let operation = mutation.operation_label();
        if self.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "{operation} is unavailable because the project is open read-only."
            ));
        }
        self.library_edit_locks
            .require_mutation_authority(
                self.workspace.project.id(),
                self.workspace.project.revision(),
                self.library_manager.revision(),
                &mutation,
            )
            .map_err(|error| format!("Could not reserve {operation}: {error}"))?;
        self.workspace
            .project
            .prepare_library_mutation(mutation, self.library_manager.revision())
            .map_err(|error| format!("Could not reserve {operation}: {error}"))
    }

    /// Publish the single project revision reserved immediately before a
    /// successful library membership transaction.
    pub(crate) fn publish_project_library_mutation(
        &mut self,
        prepared: crate::state::PreparedProjectLibraryMutation,
    ) {
        let observed_library_revision = self.library_manager.revision();
        assert!(
            observed_library_revision >= prepared.minimum_library_revision(),
            "library mutation did not advance the library revision"
        );
        self.workspace
            .project
            .publish_library_mutation(prepared, observed_library_revision)
            .expect("the library mutation receipt was fully preflighted");
        self.workspace.project_metadata_dirty = true;
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.ui.netlist.current_generation_input_digest = None;
        self.clear_project_design_history();
    }

    /// Resolve the project-owned technology contract against both mutable
    /// execution catalogs. No simulation or governed save may infer authority
    /// from a display label, an active registry package, or model paths alone.
    pub(crate) fn validate_project_technology_contract(&self) -> Result<(), String> {
        self.workspace
            .project
            .validate()
            .map_err(|error| format!("Project technology metadata is invalid: {error}"))?;
        let binding = self.workspace.project.technology_binding().ok_or_else(|| {
            "Project has no exact authenticated model-source and signed PDK binding".to_owned()
        })?;
        if self.workspace.project.technology_change_audit().is_empty() {
            return Err(
                "Project technology binding predates checkpoint-backed authority receipts; reattach it before governed saving or simulation"
                    .to_owned(),
            );
        }
        self.model_library_manager
            .validate_attached_technology(Some(binding))?;
        binding
            .validate_signed_package(&self.pdk_config.technology_registry)
            .map_err(|error| format!("Signed PDK project binding is unavailable: {error}"))?;
        let signed_pin = binding
            .signed_package()
            .expect("validated project technology has an exact signed package");
        let package = self
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
            .expect("validated signed project pin resolves to an exact package");
        let manifest = package.manifest();
        let database_unit =
            crate::quantity::LayoutDatabaseUnit::from_metres(manifest.database_unit_meters)
                .map_err(|error| format!("Signed project PDK database unit is invalid: {error}"))?;
        let expected_layout_technology = crate::state::LayoutTechnologyBinding::try_new(
            manifest.package_id.clone(),
            manifest.revision.clone(),
            package.manifest_digest(),
            package.archive_digest(),
            manifest.technology_name.clone(),
            manifest.stack_name.clone(),
            database_unit,
        )
        .map_err(|error| format!("Signed project PDK layout authority is invalid: {error}"))?;
        let allowed_layer_purposes = manifest
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
        for (key, document) in self.workspace.physical_layout_documents() {
            document
                .validate()
                .map_err(|error| format!("Physical layout '{key}' is invalid: {error}"))?;
            if document.technology() != &expected_layout_technology {
                return Err(format!(
                    "Physical layout '{key}' is bound to a different signed technology than the project"
                ));
            }
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
                        "Physical layout '{key}' {kind} {id} uses layer/purpose '{layer}/{purpose}' outside the exact signed project PDK"
                    ));
                }
            }
        }
        Ok(())
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
            .workspace
            .project
            .technology_binding()
            .and_then(crate::state::ProjectTechnologyBinding::signed_package)
            .expect("validated project technology has an exact signed package");
        self.pdk_config
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
                "Exact signed project PDK package disappeared while resolving project authority"
                    .to_owned()
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
        if self.workbench.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only; physical layout cannot be created".to_owned(),
            );
        }
        owner
            .validate_name_segments()
            .map_err(|error| format!("Layout owner is invalid: {error}"))?;
        let library = self
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
        if self.workspace.physical_layout_document(&owner).is_some() {
            return Err(format!(
                "Layout owner '{}' already has an authoritative physical document",
                owner.display_path()
            ));
        }
        let technology = self.exact_project_layout_technology_binding()?;
        let document = crate::state::PhysicalLayoutDocument::try_new(owner.clone(), technology)
            .map_err(|error| format!("Physical layout could not be initialized: {error}"))?;
        let revision = document.revision();
        self.workspace
            .commit_physical_layout_document(document)
            .map_err(|error| format!("Physical layout was not committed: {error}"))?;
        if let Some(view) = self
            .library_manager
            .get_library_mut(&owner.library)
            .and_then(|library| library.get_cell_mut(&owner.cell))
            .and_then(|cell| cell.get_view_mut(&owner.view))
        {
            view.modified = true;
        }
        if let Some(open) = self
            .workspace
            .open_views
            .iter_mut()
            .find(|open| open.reference == owner)
        {
            open.dirty = true;
        }
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.clear_project_design_history();
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
        if self.workbench.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only; physical layout cannot be edited".to_owned(),
            );
        }
        let library = self
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
            .workspace
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
                    "Physical-layout transaction failed: {kind} {id} uses layer/purpose '{layer}/{purpose}' outside the exact signed project PDK"
                ));
            }
        }
        self.workspace
            .commit_physical_layout_document(document)
            .map_err(|error| format!("Physical-layout transaction was not committed: {error}"))?;
        if let Some(view) = self
            .library_manager
            .get_library_mut(&owner.library)
            .and_then(|library| library.get_cell_mut(&owner.cell))
            .and_then(|cell| cell.get_view_mut(&owner.view))
        {
            view.modified = true;
        }
        if let Some(open) = self
            .workspace
            .open_views
            .iter_mut()
            .find(|open| open.reference == *owner)
        {
            open.dirty = true;
        }
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.clear_project_design_history();
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
        self.validate_project_technology_contract()?;
        let project_binding = self
            .workspace
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
        let sealed_pdk = self
            .pdk_config
            .technology_registry
            .seal_model_sources_for_binding(&package_binding, signed_pin.archive_digest())
            .map_err(|error| {
                format!("Signed PDK model sources cannot be sealed for project execution: {error}")
            })?;
        self.model_library_manager
            .seal_execution_sources()?
            .with_pdk_model_sources(sealed_pdk)
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
            .workspace
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
        let plan = self.sim_setup.stable_analysis_plan().map_err(|error| {
            format!("Project callback requires an authoritative active simulation plan: {error}")
        })?;
        let plan_id = plan.id();
        let plan_revision = plan.revision();
        let mut project_parameters = std::collections::BTreeMap::new();
        if let Some(payload) = self.workspace.active_plan_data(plan_id) {
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
            .workspace
            .commit_pdk_callback_execution(
                plan_id,
                plan_revision,
                authority,
                reason,
                input,
                execution,
            )
            .map_err(|error| format!("Signed PDK callback evidence was not committed: {error}"))?;

        self.workspace.project_metadata_dirty = true;
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.ui.netlist.current_generation_input_digest = None;
        self.clear_project_design_history();
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

    #[cfg(test)]
    pub(crate) fn provision_test_project_veriloga_technology_contract(&mut self) {
        self.provision_test_project_technology_fixture(
            crate::state::pdk_config::signed_veriloga_technology_test_fixture(),
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
        self.pdk_config.publisher_trust_store = trust.clone();
        self.pdk_config
            .technology_registry
            .install_archive_bytes(
                &archive,
                &trust,
                &administrative_authority,
                "Install deterministic test technology",
            )
            .expect("signed test technology installs");
        let package = self.pdk_config.technology_registry.validated_packages()[0].clone();

        let library_name = self
            .model_library_manager
            .load_library_bytes(
                "demo180-test.lib",
                b".model nmos_project_demo nmos level=1 vto=0.55".to_vec(),
                None,
            )
            .expect("authenticated retained test model imports");
        let library = self
            .model_library_manager
            .get_library(&library_name)
            .expect("imported test model remains in the catalog");
        let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
            .and_then(|binding| binding.with_signed_package(&package))
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
            self.workspace.project.revision(),
            1,
            crate::product::ContentDigest::from_bytes([0x7a; 32]),
            1,
        )
        .expect("test checkpoint evidence is valid");
        self.workspace
            .project
            .attach_technology_audited(binding, context)
            .expect("test technology attachment commits with an audit receipt");
        self.validate_project_technology_contract()
            .expect("provisioned test technology resolves exactly");
    }

    /// Whether any retained application workflow owns exclusive keyboard and
    /// pointer intent for this frame.
    pub(crate) fn application_modal_open(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        let browser_file_operation_open =
            crate::workbench::workflows::project_workflow::browser_file_operation_label(self)
                .is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let browser_file_operation_open = false;

        self.workbench.application_modal_open()
            || self.dialogs.application_modal_open()
            || (self.workbench.workspace == crate::workbench::state::Workspace::Netlist
                && self.ui.netlist.application_modal_open())
            || self.sim_setup.options_open
            || self.sim_setup.palette_open
            || self.tabbed_property_dialog.open
            || self.pdk_settings_dialog.open
            || self.model_browser_state.open
            || browser_file_operation_open
    }

    /// Whether a run can start. Every Run affordance (toolbar, run bar, menu,
    /// F5) gates on this so schematic preflight is consistent everywhere.
    pub fn can_run_simulation(&self) -> bool {
        self.simulation_run_block_reason().is_none()
    }

    /// User-facing reason the Run command is currently blocked.
    pub fn simulation_run_block_reason(&self) -> Option<String> {
        if self.simulation.is_running {
            return Some("A simulation is already running".to_string());
        }
        self.simulation_run_preflight_block_reason()
    }

    /// User-facing preflight reason a new run cannot start, excluding the
    /// transient "already running" state so queued re-runs can share it.
    pub fn simulation_run_preflight_block_reason(&self) -> Option<String> {
        run_preflight::run_preflight_block_reason(
            &self.sim_setup,
            &self.workspace,
            &self.schematic,
            &self.model_library_manager,
            self.current_blocking_drc_result(),
        )
        .or_else(|| self.validate_project_technology_contract().err())
    }

    /// User-facing reason the Netlist workspace cannot run the current deck.
    pub fn manual_deck_run_block_reason(&self) -> Option<String> {
        if self.simulation.is_running {
            return Some("A simulation is already running".to_string());
        }
        let active_document = if self.ui.netlist.active_document_initialized {
            self.ui.netlist.active_document
        } else if self.workspace.netlist_source.is_some() {
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
        } else {
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
        };
        let source = if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
        {
            self.workspace
                .netlist_source
                .as_deref()
                .unwrap_or(self.simulation.netlist_content.as_str())
        } else {
            self.simulation.netlist_content.as_str()
        };
        if source.trim().is_empty() {
            return Some("Enter a netlist before running".to_string());
        }
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        {
            return Some("Generated comparison documents cannot be executed".to_owned());
        }
        let current_digest =
            crate::workbench::documents::netlist_document::source_content_digest(source);
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
            && (self.ui.netlist.generation_error.is_some()
                || self.ui.netlist.generated_input_digest
                    != self.ui.netlist.current_generation_input_digest)
        {
            return Some(
                self.ui
                    .netlist
                    .generation_error
                    .clone()
                    .unwrap_or_else(|| "Regenerate the stale netlist before running".to_owned()),
            );
        }
        let validated = self.ui.netlist.validation.as_ref().is_some_and(|receipt| {
            receipt.visible_content_digest == current_digest
                && receipt.project_revision == self.workspace.project.revision().get()
        });
        if !validated {
            return Some(
                "Validate the exact current source and project revision before running".to_owned(),
            );
        }
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
            && self.ui.netlist.externally_saved_content_digest != Some(current_digest)
        {
            return Some("Save the validated owned source deck before running".to_owned());
        }
        None
    }

    /// Request a Netlist workspace run, queuing one re-run if the engine is busy.
    pub(crate) fn request_netlist_manual_deck_run(&mut self) {
        self.simulation.run_intent = crate::state::SimulationRunIntent::ManualDeck;
        if self.simulation.is_running {
            self.ui.netlist.rerun_queued = true;
        } else {
            self.simulation.request_manual_deck_run();
        }
    }

    /// Current error-level schematic DRC result that blocks generated runs.
    /// Stale DRC is non-blocking because it no longer describes the current
    /// topology. Manual deck runs use `manual_deck_run_block_reason` instead.
    pub fn current_blocking_drc_result(&self) -> Option<&crate::services::drc::DrcResult> {
        if self.dialogs.drc_checked_version != self.schematic.topology_version() {
            return None;
        }
        self.dialogs
            .drc_results
            .as_ref()
            .filter(|result| result.has_errors())
    }

    /// Request a run from the Simulate workspace run set.
    pub fn request_run_set_simulation(&mut self) {
        self.ui.netlist.rerun_queued = false;
        self.simulation.request_simulate_run_set();
    }

    fn log_severity_for_console(level: ConsoleLevel) -> crate::diagnostics::LogSeverity {
        match level {
            ConsoleLevel::Info => crate::diagnostics::LogSeverity::Info,
            ConsoleLevel::Warning => crate::diagnostics::LogSeverity::Warning,
            ConsoleLevel::Error => crate::diagnostics::LogSeverity::Error,
        }
    }

    /// Push a legacy console message and mirror it into the structured log.
    pub fn push_console_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::diagnostics::LogSource::System, message);
    }

    /// Push a console message with an explicit structured-log source.
    pub fn push_console_message_with_source(
        &mut self,
        source: crate::diagnostics::LogSource,
        message: ConsoleMessage,
    ) {
        let severity = Self::log_severity_for_console(message.level);
        self.log_buffer.log(severity, source, message.message, None);
    }

    pub fn push_user_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::diagnostics::LogSource::User, message);
    }

    pub fn push_sim_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::diagnostics::LogSource::Simulation, message);
    }

    pub fn clear_primary_log(&mut self) {
        self.log_buffer.clear();
    }

    /// Record a file in the recent-files list (most recent first, deduped,
    /// capped at 8 — the conventional depth for an EDA File menu).
    pub(crate) fn remember_recent_file(&mut self, kind: RecentKind, path: &std::path::Path) {
        const MAX_RECENT_FILES: usize = 8;
        let previous = self
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
        self.recent_files.retain(|recent| recent.path != entry.path);
        self.recent_files.insert(0, entry);
        self.recent_files.truncate(MAX_RECENT_FILES);
    }

    /// Change a project pin without changing its recency or file identity.
    #[cfg(test)]
    pub(crate) fn set_recent_project_pinned(
        &mut self,
        path: &std::path::Path,
        pinned: bool,
    ) -> bool {
        let Some(recent) = self
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
        let grid = self.schematic.grid_size.max(1);
        let (x, y) = self
            .ui
            .canvas_hover
            .or(self.ui.canvas_view_center)
            .unwrap_or((20.0, 20.0));
        crate::state::Point::new((x.round() as i32) * grid, (y.round() as i32) * grid)
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
        state.recent_files[0].pinned = true;
        state.recent_files[0].owner = Some("Analog Design".to_owned());
        state.recent_files[0].tags = vec!["afe".to_owned(), "release".to_owned()];
        let first_timestamp = state.recent_files[0].opened_at_unix_ms;

        state.remember_recent_file(RecentKind::Project, path);

        let recent = &state.recent_files[0];
        assert!(recent.pinned);
        assert_eq!(recent.owner.as_deref(), Some("Analog Design"));
        assert_eq!(recent.tags, ["afe", "release"]);
        assert!(recent.opened_at_unix_ms >= first_timestamp);
    }

    #[test]
    fn launcher_metadata_is_backward_compatible_and_shared_paths_are_detected() {
        let legacy = r#"{"kind":"Project","path":"C:/Engineering/afe.rspiceproj"}"#;
        let recent: RecentFile = serde_json::from_str(legacy).expect("legacy recent entry loads");
        assert_eq!(recent.opened_at_unix_ms, 0);
        assert!(!recent.pinned);
        assert!(recent.owner.is_none());
        assert!(recent.tags.is_empty());
        assert!(!recent.is_shared_project());

        let shared: RecentFile =
            serde_json::from_str(r#"{"kind":"Project","path":"\\\\server\\share\\rf.rspiceproj"}"#)
                .expect("UNC recent entry loads");
        assert!(shared.is_shared_project());
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
        assert_eq!(state.recent_files[0].path, schematic);
        assert_eq!(state.recent_files[1].path, project);
        assert!(state.recent_files[1].pinned);
    }

    #[test]
    fn exact_project_callback_execution_commits_one_validated_project_receipt() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        let plan_id = state
            .sim_setup
            .stable_analysis_plan()
            .expect("active plan")
            .id();
        state
            .workspace
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
        crate::workbench::lifecycle::project_lifecycle::accept_loaded_project(
            &mut state, baseline, None,
        );
        assert!(!crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
        let from_revision = state.workspace.project.revision();
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
            state.workspace.project.revision()
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
            state.workspace.pdk_callback_receipts(),
            [receipt.clone(), second.clone()]
        );
        state
            .workspace
            .validate_pdk_callback_receipts()
            .expect("project callback ledger validates");
        assert!(state.workspace.project_metadata_dirty);

        let encoded = serde_json::to_vec(&state.workspace).expect("serialize callback evidence");
        let restored: crate::state::ProjectWorkspace =
            serde_json::from_slice(&encoded).expect("restore callback evidence");
        assert_eq!(restored.pdk_callback_receipts(), [receipt.clone(), second]);
        restored
            .validate_pdk_callback_receipts()
            .expect("restored callback ledger validates");

        let mut tampered: serde_json::Value =
            serde_json::from_slice(&encoded).expect("decode callback evidence for tamper test");
        tampered["pdk_callback_receipts"][0]["reason"] =
            serde_json::Value::String("tampered callback reason".to_owned());
        let tampered: crate::state::ProjectWorkspace =
            serde_json::from_value(tampered).expect("structurally valid tampered workspace");
        assert!(tampered.validate_pdk_callback_receipts().is_err());
    }

    #[test]
    fn project_callback_failure_is_transactional_without_an_exact_project_pin() {
        let mut state = AppState::default();
        let revision = state.workspace.project.revision();
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
        assert_eq!(state.workspace.project.revision(), revision);
        assert!(state.workspace.pdk_callback_receipts().is_empty());
        assert!(!state.workspace.project_metadata_dirty);
    }

    #[test]
    fn physical_layout_initialization_uses_exact_signed_project_dbu_and_persists() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        let owner = crate::state::CellViewRef::new("user", "top", "layout");
        state
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
            .workspace
            .physical_layout_document(&owner)
            .expect("authoritative layout document");
        assert_eq!(document.revision(), revision);
        assert_eq!(document.technology().database_unit().metres(), 1.0e-9);
        assert_eq!(document.technology().stack_id(), "1P2M");

        let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
            .expect("layout project snapshot validates");
        let encoded = serde_json::to_vec(&snapshot).expect("serialize layout project");
        let restored: crate::io::ProjectFile =
            serde_json::from_slice(&encoded).expect("deserialize layout project");
        restored
            .validate()
            .expect("restored layout project validates");

        let mut tampered: serde_json::Value =
            serde_json::from_slice(&encoded).expect("decode layout project");
        tampered["workspace"]["physical_layout_documents"][owner.key()]["technology"]["archive_digest"] =
            serde_json::Value::String("00".repeat(32));
        let tampered: crate::io::ProjectFile =
            serde_json::from_value(tampered).expect("deserialize structurally valid tamper");
        assert!(tampered.validate().is_err());
    }

    #[test]
    fn physical_layout_edits_commit_atomically_and_reject_unsigned_layer_purposes() {
        let mut state = AppState::default();
        state.provision_test_project_technology_contract();
        let owner = crate::state::CellViewRef::new("user", "top", "layout");
        state
            .library_manager
            .get_library_mut("user")
            .expect("default project library")
            .get_or_create_cell("top")
            .add_view(crate::state::View::new(
                "layout",
                crate::state::ViewType::Layout,
            ));
        state
            .workspace
            .open_view(owner.clone(), crate::state::ViewType::Layout);
        let start = state
            .initialize_physical_layout_document(owner.clone())
            .expect("initialize exact signed-PDK physical layout");
        state
            .library_manager
            .get_library_mut("user")
            .unwrap()
            .get_cell_mut("top")
            .unwrap()
            .get_view_mut("layout")
            .unwrap()
            .modified = false;
        state
            .workspace
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
                .workspace
                .physical_layout_document(&owner)
                .unwrap()
                .shapes()
                .contains_key(&valid_shape)
        );
        assert!(
            state
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
                .workspace
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
            error.contains("outside the exact signed project PDK"),
            "{error}"
        );
        let unchanged = state.workspace.physical_layout_document(&owner).unwrap();
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
            .workspace
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
