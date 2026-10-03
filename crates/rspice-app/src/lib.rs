//! Desktop and browser application composition.
//!
//! Domain data, execution, hardcopy rendering and reusable editor/viewer
//! presentation belong to the sibling crates. This crate binds those owners
//! to host services, document lifecycles, commands, navigation and workbench UI.
//!
//! Internal layers are checked by `tests/module_layering.rs`; package dependency
//! boundaries are checked by `tools/ci/check_app_crate_dependencies.py`.

// Temporary allowance for existing external/SPICE naming conventions.
#![allow(non_snake_case)]
// Desktop-only workbench paths remain unreachable in the browser build.
// Native and test builds continue to diagnose ordinary dead code.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
// Rendering and transaction entry points may take independent source, state,
// layout and authority inputs. Group parameters when they form one contract.
#![allow(clippy::too_many_arguments)]
// The desktop build detaches from its console on Windows and the browser
// build has no stderr at all, so anything printed is a diagnostic nobody
// will ever read. Route it through `log` and the application log buffer.
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_constants,
        clippy::bool_assert_comparison,
        clippy::cloned_ref_to_slice_refs,
        clippy::default_constructed_unit_structs,
        clippy::expect_fun_call,
        clippy::field_reassign_with_default,
        clippy::len_zero,
        clippy::manual_range_contains,
        clippy::manual_repeat_n,
        clippy::needless_range_loop,
        clippy::unnecessary_cast,
        clippy::unnecessary_get_then_check,
        clippy::unnecessary_unwrap,
        clippy::useless_vec
    )
)]

// =============================================================================
// Domain Modules (Organized by Feature)
// =============================================================================

/// Calculator adaptation over retained application results.
pub(crate) mod analysis;

/// Application binding for schematic and symbol editors.
pub(crate) mod schematic;

/// Application simulation setup, source preparation and dispatch coordination.
pub(crate) mod simulation;

/// Project-bound property editing workflows.
pub(crate) mod properties;

/// Shared UI kit exports used by application presentation.
pub(crate) mod ui;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

/// Result document exports and application integration fixtures.
pub(crate) mod results;

/// Shared product identity and outcome vocabulary.
pub(crate) mod product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
#[cfg(not(target_arch = "wasm32"))]
mod automation_runtime;
#[cfg(target_arch = "wasm32")]
#[path = "automation_runtime_browser.rs"]
mod automation_runtime;
pub(crate) mod automation_workflow;

// =============================================================================
// Core Infrastructure
// =============================================================================

/// Application service adapters and platform integration.
pub(crate) mod services;

/// Application file workflows and platform I/O adapters.
pub(crate) mod io;

/// Application-held domain owners and session adapters.
pub(crate) mod state;

/// Unit-safe user presentation and UI quantity-input policy. Values entering
/// or leaving this module are always expressed in their documented SI base
/// units; deck dialect and PDK database-unit semantics live elsewhere.
pub(crate) mod quantity;

/// Diagnostics the application reports about itself: the console message
/// model and the structured, filterable application log.
pub(crate) mod diagnostics;

pub(crate) mod source_revision;
/// Clock shims for the browser build. `std::time::{Instant, SystemTime}` trap
/// at runtime on wasm32-unknown-unknown, so every layer uses these instead.
pub(crate) mod time_compat;

/// Locating the production half of a source file that inspects itself, for the
/// guards that assert their own shipped code takes no panic shortcuts.
#[cfg(test)]
mod source_guard;

/// The real on-disk identity of the temporary directory, for fixtures that
/// hand paths to a subject which records them through `std::fs::canonicalize`.
#[cfg(test)]
mod fixture_root;

// =============================================================================
// The crate's entire external surface
// =============================================================================
//
// Keep implementation modules private so the compiler can diagnose unused
// code. Expose entrypoint and integration contracts explicitly below.

/// The application root, constructed by both the desktop and browser entry
/// points.
pub use workbench::RSpiceApp;

/// Offline organization drawing-sheet publisher contract. Private keys are
/// accepted only by the separate native publisher binary; the GUI exposes
/// package construction, inspection, and verification primitives.
pub use workbench::{
    DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding, DrawingSheetPackageInspection,
    DrawingSheetPackageVerification, PublishedDrawingSheetPackage,
    drawing_sheet_publisher_public_key, inspect_drawing_sheet_package,
    publish_organization_drawing_sheet_package, verify_published_drawing_sheet_package,
};

/// Native logging environment for the desktop binary, and the logger it
/// installs: stderr as before, plus the running analysis's own Console log.
#[cfg(not(target_arch = "wasm32"))]
pub use workbench::logging::{install_studio_logger, native_log_env};

/// Typed identities, for `tests/simulation_configuration_contract.rs`.
pub use product::{AnalysisInstanceId, ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};

/// Trusted in-process collaboration-connector boundary for exact,
/// revision-bound project-library edit-lock snapshots.
pub use state::library_browser::{
    ProjectLibraryEditLock, ProjectLibraryEditLockScope, ProjectLibraryLockSnapshot,
};
pub use state::workspace::ProjectLibraryPublicationReceipt;

#[cfg(not(target_arch = "wasm32"))]
pub use state::model_hub::{CloudModelHubTransport, FilesystemModelHubStore};
/// The Model Hub client runtime: signed catalog, verified installation, and
/// the unified part shelf over foundation, installed, catalog, and retained
/// parts. This is the crate's model-distribution boundary — the surfaces that
/// present it are consumers of exactly these items, not of the module's
/// interior.
pub use state::model_hub::{
    InstalledPack, ModelHub, ModelHubError, ModelHubPartRow, ModelHubStore, ModelHubTransport,
    PartProvenance, PartState, TrustAnchor, missing_capabilities,
};
pub use state::model_hub::{MemoryModelHubStore, OfflineTransport};

/// The one hierarchical path grammar.
pub use state::{
    HierarchyPathError, InstancePath, InstancePathPattern, MAX_INSTANCE_PATH_BYTES,
    MAX_INSTANCE_PATH_DEPTH, PatternSegment, ProbeTarget,
};

/// The persisted project model the configuration contract exercises.
pub use state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};

pub struct ProjectLibraryPublicationCandidate {
    draft: crate::state::workspace::ProjectLibraryPublicationDraft,
    artifact_bytes: Vec<u8>,
    source_project_revision: ObjectRevision,
}

impl ProjectLibraryPublicationCandidate {
    #[must_use]
    pub fn artifact_bytes(&self) -> &[u8] {
        &self.artifact_bytes
    }

    #[must_use]
    pub fn publication_id(&self) -> uuid::Uuid {
        self.draft.publication_id
    }

    #[must_use]
    pub fn snapshot_digest(&self) -> ContentDigest {
        self.draft.snapshot_digest
    }

    #[must_use]
    pub fn snapshot_byte_len(&self) -> u64 {
        self.draft.snapshot_byte_len
    }
}

impl RSpiceApp {
    /// Install a snapshot already authenticated by a trusted collaboration
    /// connector. RSpice validates its content digest, project identity,
    /// authority continuity, generation, and exact live revisions before the
    /// snapshot can govern any edit.
    pub fn install_project_library_lock_snapshot(
        &mut self,
        snapshot: ProjectLibraryLockSnapshot,
    ) -> Result<(), String> {
        snapshot.validate()?;
        if snapshot.project_id() != self.state.workspace.content.project.id() {
            return Err(format!(
                "project library lock snapshot belongs to project {}, not current project {}",
                snapshot.project_id(),
                self.state.workspace.content.project.id()
            ));
        }
        if snapshot.project_revision() != self.state.workspace.content.project.revision()
            || snapshot.library_revision() != self.state.library_manager.revision()
        {
            return Err(format!(
                "project library lock snapshot is stale (project {} vs {}, library {} vs {})",
                snapshot.project_revision().get(),
                self.state.workspace.content.project.revision().get(),
                snapshot.library_revision(),
                self.state.library_manager.revision()
            ));
        }
        self.state
            .library_edit_locks
            .install_authoritative(snapshot)
    }

    /// Prepare the exact artifact and receipt candidate without changing live
    /// project state. A native, browser, or repository writer must durably
    /// publish `artifact_bytes()` before commit.
    pub fn prepare_project_library_publication(
        &self,
        label: impl Into<String>,
        actor_id: impl Into<String>,
        authority_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<ProjectLibraryPublicationCandidate, String> {
        use sha2::Digest as _;

        if self.state.workbench.safe_mode.project_read_only() {
            return Err(
                "project library publication is unavailable because the project is open read-only"
                    .to_owned(),
            );
        }
        if self.state.simulation.has_active_execution() {
            return Err(
                "project library publication is unavailable while a simulation is running"
                    .to_owned(),
            );
        }
        let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&self.state)
            .map_err(|error| format!("project library publication snapshot failed: {error}"))?;
        let serialized =
            crate::io::project_io::serialize_project_file(&snapshot).map_err(|error| {
                format!("project library publication serialization failed: {error}")
            })?;
        let bytes = serialized.into_bytes();
        let snapshot_byte_len = u64::try_from(bytes.len())
            .map_err(|_| "project library publication artifact is too large".to_owned())?;
        let draft = crate::state::workspace::ProjectLibraryPublicationDraft {
            publication_id: uuid::Uuid::new_v4(),
            label: label.into(),
            actor_id: actor_id.into(),
            authority_id: authority_id.into(),
            reason: reason.into(),
            created_unix_ms: crate::time_compat::unix_epoch()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX)
                .max(1),
            library_revision: self.state.library_manager.revision(),
            snapshot_digest: crate::product::ContentDigest::from_bytes(
                sha2::Sha256::digest(&bytes).into(),
            ),
            snapshot_byte_len,
        };
        let mut descriptor_preflight = self.state.workspace.content.project.clone();
        descriptor_preflight
            .publish_library_snapshot(draft.clone())
            .map_err(|error| format!("project library publication preflight failed: {error}"))?;
        Ok(ProjectLibraryPublicationCandidate {
            draft,
            artifact_bytes: bytes,
            source_project_revision: self.state.workspace.content.project.revision(),
        })
    }

    /// Commit a publication only after its exact artifact was durably
    /// accepted. Any intervening project or catalog change rejects the
    /// candidate and leaves live state untouched.
    pub fn commit_project_library_publication(
        &mut self,
        candidate: ProjectLibraryPublicationCandidate,
    ) -> Result<ProjectLibraryPublicationReceipt, String> {
        use sha2::Digest as _;

        if self.state.workbench.safe_mode.project_read_only() {
            return Err(
                "project library publication is unavailable because the project is open read-only"
                    .to_owned(),
            );
        }
        if self.state.simulation.has_active_execution() {
            return Err(
                "project library publication is unavailable while a simulation is running"
                    .to_owned(),
            );
        }
        if self.state.workspace.content.project.revision() != candidate.source_project_revision
            || self.state.library_manager.revision() != candidate.draft.library_revision
        {
            return Err(
                "project library publication candidate is stale; prepare and publish a new artifact"
                    .to_owned(),
            );
        }
        let current_snapshot =
            crate::workbench::lifecycle::project_lifecycle::snapshot(&self.state)
                .map_err(|error| format!("project library publication recheck failed: {error}"))?;
        let current_serialized = crate::io::project_io::serialize_project_file(&current_snapshot)
            .map_err(|error| {
            format!("project library publication recheck serialization failed: {error}")
        })?;
        let current_bytes = current_serialized.as_bytes();
        if current_bytes.len() as u64 != candidate.draft.snapshot_byte_len
            || crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(current_bytes).into())
                != candidate.draft.snapshot_digest
        {
            return Err(
                "project library publication content changed after the artifact was prepared"
                    .to_owned(),
            );
        }
        let receipt = self
            .state
            .workspace
            .content
            .project
            .publish_library_snapshot(candidate.draft)
            .map_err(|error| format!("project library publication failed: {error}"))?;
        self.state.workspace.content.project_metadata_dirty = true;
        self.state.design_execution_epoch = self.state.design_execution_epoch.wrapping_add(1);
        self.state.ui.netlist.current_generation_input_digest = None;
        self.state.clear_project_design_history();
        Ok(receipt)
    }

    /// Restore the exact complete project artifact named by an immutable
    /// library publication while preserving the current project identity,
    /// publication ledger, and intervening audit history. The rollback is one
    /// new revision; malformed, tampered, foreign, stale-authority, or
    /// technology-incompatible artifacts leave live state unchanged.
    pub fn rollback_project_library_publication(
        &mut self,
        publication_id: uuid::Uuid,
        artifact_bytes: &[u8],
        actor_id: impl Into<String>,
        authority_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(), String> {
        use sha2::Digest as _;

        if self.state.workbench.safe_mode.project_read_only() {
            return Err(
                "project library rollback is unavailable because the project is open read-only"
                    .to_owned(),
            );
        }
        if self.state.simulation.has_active_execution() {
            return Err(
                "project library rollback is unavailable while a simulation is running".to_owned(),
            );
        }
        let receipt = self
            .state
            .workspace
            .content
            .project
            .library_publications()
            .iter()
            .find(|receipt| receipt.publication_id() == publication_id)
            .cloned()
            .ok_or_else(|| {
                format!("project library publication {publication_id} is not retained")
            })?;
        if artifact_bytes.len() as u64 != receipt.snapshot_byte_len()
            || crate::product::ContentDigest::from_bytes(
                sha2::Sha256::digest(artifact_bytes).into(),
            ) != receipt.snapshot_digest()
        {
            return Err(
                "project library rollback artifact does not match its publication receipt"
                    .to_owned(),
            );
        }
        let artifact_text = std::str::from_utf8(artifact_bytes)
            .map_err(|error| format!("project library rollback artifact is not UTF-8: {error}"))?;
        let mut artifact = crate::io::project_io::load_project_text(artifact_text, None)
            .map_err(|error| format!("project library rollback artifact is invalid: {error}"))?;
        if artifact.file.workspace.project.id() != receipt.project_id()
            || artifact.file.workspace.project.revision() != receipt.source_project_revision()
            || artifact.file.libraries.revision() != receipt.library_revision()
        {
            return Err(
                "project library rollback artifact identity or revision does not match its receipt"
                    .to_owned(),
            );
        }
        let expected_prior_publications = usize::try_from(receipt.sequence() - 1)
            .map_err(|_| "project library publication sequence is invalid".to_owned())?;
        if artifact.file.workspace.project.library_publications().len()
            != expected_prior_publications
            || artifact
                .file
                .workspace
                .project
                .library_publications()
                .last()
                .map(ProjectLibraryPublicationReceipt::receipt_digest)
                != receipt.previous_receipt_digest()
        {
            return Err(
                "project library rollback artifact does not retain the exact publication lineage prefix"
                    .to_owned(),
            );
        }
        if artifact.file.workspace.project.technology_binding()
            != self.state.workspace.content.project.technology_binding()
        {
            return Err(
                "project library rollback cannot cross an exact technology-binding change"
                    .to_owned(),
            );
        }

        let mutation = crate::state::ProjectLibraryMutation::RollbackPublication {
            publication_id,
            publication_label: receipt.label().to_owned(),
            snapshot_digest: receipt.snapshot_digest(),
            actor_id: actor_id.into(),
            authority_id: authority_id.into(),
            reason: reason.into(),
        };
        let prepared = self.state.preflight_project_library_mutation(mutation)?;

        let project_id = artifact.file.workspace.project.id();
        let (simulation_plan, model_library_manager, execution_warnings) =
            match artifact.file.execution_context.take() {
                Some(context) => crate::io::restore_execution_context(context, project_id).map_err(|error| {
                    format!("project library rollback execution context is invalid: {error}")
                })?,
                None => (
                    crate::workbench::app_state::SimSetupState::new_with_user_preferences(
                        &self.state.ui.preferences,
                    ),
                    crate::workbench::app_state::default_model_library_manager(),
                    vec![
                        "The publication predates durable simulation plans; documented defaults were restored"
                            .to_owned(),
                    ],
                ),
            };

        let mut candidate = self.state.clone();
        let mut current_project = candidate.workspace.content.project.clone();
        current_project.root_library = artifact.file.workspace.project.root_library.clone();
        current_project.top_cell = artifact.file.workspace.project.top_cell.clone();
        artifact.file.workspace.project = current_project;
        candidate.clear_design_execution_context();
        candidate
            .library_manager
            .replace_catalog_from_snapshot(&artifact.file.libraries)?;
        candidate.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
        candidate.workspace = crate::state::ProjectWorkspace::from_parts(
            artifact.file.workspace,
            artifact.workspace_session,
        );
        candidate.sim_setup = simulation_plan;
        candidate.model_library_manager = model_library_manager;
        candidate.restore_active_schematic_from_workspace();
        candidate.simulation = crate::state::SimulationState::default();
        crate::io::restore_simulation_results(
            artifact.file.simulation_results,
            &mut candidate.simulation,
        )
        .map_err(|error| format!("project library rollback result history is invalid: {error}"))?;
        candidate.publish_project_library_mutation(prepared);
        candidate
            .workspace
            .content
            .project
            .validate()
            .map_err(|error| format!("project library rollback metadata is invalid: {error}"))?;
        self.state = candidate;
        for warning in execution_warnings {
            self.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(warning));
        }
        Ok(())
    }
}
