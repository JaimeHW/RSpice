//! RSpice UI - Commercial-Grade Circuit Simulator Interface
//!
//! A high-performance GUI for the RSpice circuit simulation engine,
//! built with egui for GPU-accelerated desktop deployment.
//!
//! # Architecture
//!
//! This crate is the RSpice application, deliberately kept whole. The
//! simulation engine lives in `rspice-core` and `rspice-veriloga`; everything
//! the application itself owns — persisted design state, project I/O, run
//! orchestration, viewer mathematics, and chrome — stays here. Modules that
//! never mention `egui` are therefore expected, not misplaced: they are the
//! application layer, not the presentation layer.
//!
//! Because there is no crate boundary to lean on, the module layering is
//! enforced by `tests/module_layering.rs` instead. A module may reference
//! any module below it and none at or above it. Lowest layer first:
//!
//! | Layer | Modules | Owns |
//! |-------|---------|------|
//! | 0 | `product`, `quantity` | Framework-independent contracts, typed identities, unit-safe presentation policy |
//! | 1 | `results`, `ui` | Versioned result documents; the design system (tokens, palette, widgets, plot engine) |
//! | 2 | `hardcopy` | Persisted page setup, print mappings, and source-set records |
//! | 3 | `state` | The persisted design, library, and project model |
//! | 4 | `analysis`, `automation_workflow`, `diagnostics`, `io` | Viewer mathematics, the CI workflow language, console/log model, file formats |
//! | 5 | `services` | DRC, licensing, and the per-analysis engine adapters |
//! | 6 | `simulation` | Analysis plans, netlist generation, run orchestration |
//! | 7 | `properties` | Component property editing |
//! | 8 | `schematic` | The schematic document engine |
//! | 9 | `workbench` | The application shell: [`RSpiceApp`], state, dialogs, chrome, surfaces, commands, and the workflows that mutate them |
//!
//! `workbench` is about half the crate, so one position in this table does not
//! describe it. Its own submodules are ordered by that test's
//! `WORKBENCH_LAYERS`, on the same rules.
//!
//! Known departures from both orders are recorded, counted, and ratcheted
//! down in the `ALLOWED_VIOLATIONS` and `ALLOWED_WORKBENCH_VIOLATIONS` tables.
//! Adding to either is not a way to unblock new code — a fresh violation
//! means the code is in the wrong module.

// Temporary allowance for existing external/SPICE naming conventions.
#![allow(non_snake_case)]
// A rendering or transaction entry point takes one parameter per thing the
// caller independently varies: the `Ui`, the state it may mutate, the layout
// it must respect, the identity it acts on. Fifty call sites had already
// reached that conclusion one `#[allow]` at a time; this states it once. It
// is not licence to grow a signature that could take a struct.
#![allow(clippy::too_many_arguments)]
// NOTE: this crate previously carried a blanket `#![allow(deprecated)]`,
// hiding the egui 0.34 migration entirely. It is left off on purpose.
//
// Everything migratable on 0.34 has been migrated: the panel constructors
// (`TopBottomPanel`/`SidePanel` -> `Panel::top`/`left`), `SelectableLabel` ->
// `Button::selectable`, `Ui::set_enabled` -> `disable()`,
// `Context::screen_rect` -> `content_rect`, `Ui::allocate_ui_at_rect` ->
// `scope_builder`, and `popup_below_widget` -> the `Popup` builder. Each was
// verified against egui's own body first — every one of those forwards to its
// replacement with identical arguments, so none of them can move a pixel.
//
// What remains is one family: `Panel::show(ctx)` and `CentralPanel::show(ctx)`.
// Those cannot be migrated on 0.34. `show_inside` takes `&mut Ui`, and the
// root `Ui` that `show(ctx)` builds for itself needs `Context::pass_state_mut`
// and `PassState::allocate_central_panel`, both `pub(crate)` in egui. The
// supported way to obtain that root `Ui` arrives with eframe 0.35, which
// replaces `App::update(&mut self, ctx, frame)` with
// `App::ui(&mut self, ui, frame)`. So these warnings are not deferred cleanup
// — they are the visible edge of an eframe 0.35 upgrade, and they should be
// resolved by that upgrade rather than by hand-rolling egui internals here.
// NOTE: closing the public surface (see the visibility note below) turned 192
// items into `dead_code` warnings. They were never reachable — the compiler
// simply could not say so while their modules were `pub`. All 192 are dead on
// native, on wasm32, and with tests compiled; `--lib` alone is not enough,
// because it hides anything only a `#[cfg(test)]` block or a browser-only
// path calls.
//
// They are deliberately not swept. `workbench::simulation_analysis_tabs` is
// the reason: 26 of its items are unreachable, but it is one coherent catalog
// of 25 analysis tabs whose two index tables happen to have no reader.
// Deleting the unreferenced half would leave a catalog that no longer
// describes the product. Retire these per module — decide whether each thing
// is finished-but-unwired or genuinely abandoned — not with a bulk delete.
//
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

/// Analysis viewers - Bode, FFT, histogram, Nyquist, pole-zero, Smith chart, eye diagram
pub(crate) mod analysis;

/// Schematic editor - Canvas, export, toolbar, symbol library
pub(crate) mod schematic;

/// Simulation management - Controller, dialogs, netlist generation
pub(crate) mod simulation;

/// Property editing - Component properties and design variables
pub(crate) mod properties;

/// The RSpice design system - tokens, palettes, fonts, icons, widgets
pub(crate) mod ui;

/// Persisted page-setup contracts and deterministic pagination. Document
/// adapters, scene rendering, the platform print boundary, and the dialogs
/// live in `workbench::hardcopy`; this is the layer `state` can persist.
pub(crate) mod hardcopy;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

/// Versioned visualization documents, immutable dataset bindings, exact-data
/// queries, viewer compatibility, and progressive result operations.
pub(crate) mod results;

/// Canonical commercial product model, typed identities, command outcomes,
/// and fail-closed object lifecycles. This layer is UI-framework independent.
pub(crate) mod product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
pub(crate) mod automation_workflow;

// =============================================================================
// Core Infrastructure
// =============================================================================

/// Backend services (file I/O, simulation runner)
pub(crate) mod services;

/// File I/O (library parser, session, netlist, waveform)
pub(crate) mod io;

/// Application state management
pub(crate) mod state;

/// Unit-safe user presentation and UI quantity-input policy. Values entering
/// or leaving this module are always expressed in their documented SI base
/// units; deck dialect and PDK database-unit semantics live elsewhere.
pub(crate) mod quantity;

/// Diagnostics the application reports about itself: the console message
/// model and the structured, filterable application log.
pub(crate) mod diagnostics;

/// Clock shims for the browser build. `std::time::{Instant, SystemTime}` trap
/// at runtime on wasm32-unknown-unknown, so every layer uses these instead.
pub(crate) mod time_compat;

/// Shared output specification helpers for analysis/sensitivity paths
pub(crate) mod output_spec;

// =============================================================================
// The crate's entire external surface
// =============================================================================
//
// `rspice-ui` is an application, not a library. Its only consumers are the
// desktop and browser binary in `main.rs` and the integration tests --
// nothing in the workspace depends on it. Every
// module above is therefore `pub(crate)`, and everything reachable from
// outside is named here.
//
// That is not tidiness. A `pub` module is one the compiler must assume some
// unseen caller uses, so it cannot report an unreachable item inside it. The
// eight modules that used to be `pub` covered 262k lines -- 45% of the crate
// -- in which dead code could not be detected at all. Adding a `pub mod` to
// reach something from a test re-opens that hole; add a re-export here
// instead.

/// The application root, constructed by both the desktop and browser entry
/// points.
pub use workbench::RSpiceApp;

/// Native logging environment for the desktop binary.
#[cfg(not(target_arch = "wasm32"))]
pub use workbench::logging::native_log_env;

/// Typed identities, for `tests/simulation_configuration_contract.rs`.
pub use product::{AnalysisInstanceId, ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};

/// Trusted in-process collaboration-connector boundary for exact,
/// revision-bound project-library edit-lock snapshots.
pub use state::library_browser::{
    ProjectLibraryEditLock, ProjectLibraryEditLockScope, ProjectLibraryLockSnapshot,
};
pub use state::workspace::ProjectLibraryPublicationReceipt;

/// Design-variable netlist emission, pinned by the configuration contract.
pub use simulation::netlist_gen::{DesignVariableNetlistContext, design_variable_parameter_lines};

/// The persisted project model the configuration contract exercises.
pub use state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_worker_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    simulation::runner::worker_contract::run_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_veriloga_compile_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    workbench::documents::code_workspace::run_veriloga_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_hardcopy_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    workbench::app::run_hardcopy_worker_request_value(value)
}

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
        if snapshot.project_id() != self.state.workspace.project.id() {
            return Err(format!(
                "project library lock snapshot belongs to project {}, not current project {}",
                snapshot.project_id(),
                self.state.workspace.project.id()
            ));
        }
        if snapshot.project_revision() != self.state.workspace.project.revision()
            || snapshot.library_revision() != self.state.library_manager.revision()
        {
            return Err(format!(
                "project library lock snapshot is stale (project {} vs {}, library {} vs {})",
                snapshot.project_revision().get(),
                self.state.workspace.project.revision().get(),
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
        if self.state.simulation.is_running {
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
        let mut descriptor_preflight = self.state.workspace.project.clone();
        descriptor_preflight
            .publish_library_snapshot(draft.clone())
            .map_err(|error| format!("project library publication preflight failed: {error}"))?;
        Ok(ProjectLibraryPublicationCandidate {
            draft,
            artifact_bytes: bytes,
            source_project_revision: self.state.workspace.project.revision(),
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
        if self.state.simulation.is_running {
            return Err(
                "project library publication is unavailable while a simulation is running"
                    .to_owned(),
            );
        }
        if self.state.workspace.project.revision() != candidate.source_project_revision
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
            .project
            .publish_library_snapshot(candidate.draft)
            .map_err(|error| format!("project library publication failed: {error}"))?;
        self.state.workspace.project_metadata_dirty = true;
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
        if self.state.simulation.is_running {
            return Err(
                "project library rollback is unavailable while a simulation is running".to_owned(),
            );
        }
        let receipt = self
            .state
            .workspace
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
        if artifact.workspace.project.id() != receipt.project_id()
            || artifact.workspace.project.revision() != receipt.source_project_revision()
            || artifact.libraries.revision() != receipt.library_revision()
        {
            return Err(
                "project library rollback artifact identity or revision does not match its receipt"
                    .to_owned(),
            );
        }
        let expected_prior_publications = usize::try_from(receipt.sequence() - 1)
            .map_err(|_| "project library publication sequence is invalid".to_owned())?;
        if artifact.workspace.project.library_publications().len() != expected_prior_publications
            || artifact
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
        if artifact.workspace.project.technology_binding()
            != self.state.workspace.project.technology_binding()
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

        let project_id = artifact.workspace.project.id();
        let (simulation_plan, model_library_manager, execution_warnings) =
            match artifact.execution_context.take() {
                Some(context) => context.into_state(project_id).map_err(|error| {
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
        let mut current_project = candidate.workspace.project.clone();
        current_project.root_library = artifact.workspace.project.root_library.clone();
        current_project.top_cell = artifact.workspace.project.top_cell.clone();
        artifact.workspace.project = current_project;
        candidate.clear_design_execution_context();
        candidate
            .library_manager
            .replace_catalog_from_snapshot(&artifact.libraries)?;
        candidate.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
        candidate.workspace = artifact.workspace;
        candidate.sim_setup = simulation_plan;
        candidate.model_library_manager = model_library_manager;
        candidate.restore_active_schematic_from_workspace();
        candidate.simulation = crate::state::SimulationState::default();
        artifact
            .simulation_results
            .apply_to_state(&mut candidate.simulation)
            .map_err(|error| {
                format!("project library rollback result history is invalid: {error}")
            })?;
        candidate.publish_project_library_mutation(prepared);
        candidate
            .workspace
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
