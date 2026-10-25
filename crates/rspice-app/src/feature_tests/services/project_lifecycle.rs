//! The project lifecycle observed through a whole session: open, revert and
//! close, which run session operations beside the lifecycle, and the saves
//! whose content a feature's slot holds.
//!
//! Each case pins a scope boundary: revert must not race the active document,
//! and a close is presentation-only.

use crate::io::ProjectSnapshot;
#[cfg(not(target_arch = "wasm32"))]
use crate::simulation::plan::AnalysisKind;
use crate::state::{CellViewRef, ComponentType, Point, ViewType};
use crate::workbench::app_state::{
    AppState,
    services_access::{
        accept_loaded_project, close_active_document, confirm_revert_active_document,
        initialize_from_session, mark_project_closed, reset_for_new_project, revert_document,
    },
};
use crate::workbench::lifecycle::project_lifecycle::*;
use rspice_project::SnapshotContent;
use std::path::{Path, PathBuf};

/// The registry's pass counters, under the name the lifecycle's own tests
/// read them by.
mod registry {
    pub(super) use rspice_project::registry::result_fingerprint::RESULT_FINGERPRINT_PASSES;
    pub(super) use rspice_project::registry::{FINGERPRINT_PASSES, content_digest};
}

mod durable_content;
mod fingerprint_cache;
#[cfg(not(target_arch = "wasm32"))]
mod model_evidence;
mod result_cache;

#[cfg(not(target_arch = "wasm32"))]
fn insert_ac_analysis(state: &mut AppState) -> crate::product::AnalysisInstanceId {
    state
        .model
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC analysis inserts")
        .0
}

#[cfg(not(target_arch = "wasm32"))]
fn has_ac_analysis(setup: &rspice_simulation_contract::setup_state::SimulationSetup) -> bool {
    setup
        .stable_analysis_plan()
        .expect("current project owns a stable plan")
        .instances()
        .iter()
        .any(|instance| instance.kind() == AnalysisKind::Ac)
}

#[cfg(target_arch = "wasm32")]
#[test]
fn browser_save_active_and_revert_preserve_exact_configuration_catalog() {
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Project);
    let baseline = snapshot(&state).expect("baseline");
    state.open_project_session();
    state
        .project_lifecycle
        .accept_project(baseline.clone(), None);
    insert_configuration_root(
        &mut state,
        "Browser release",
        CellViewRef::new("user", "top", "schematic"),
    );

    let mut prepared = prepare_browser_save(
        &mut state,
        SaveScope::ActiveDocument,
        false,
        "browser-project.rspice".to_owned(),
    )
    .expect("prepare browser active save");
    assert_eq!(
        prepared.save.saved_document(),
        &ProjectDocumentId::ProjectConfiguration
    );
    assert_eq!(
        prepared.save.candidate().workspace.configuration_sets,
        state.workspace.content.configuration_sets
    );
    let bytes = prepared.save.take_bytes();
    let staged_text = std::str::from_utf8(&bytes).expect("UTF-8 project bytes");
    let decoded = crate::io::project_io::load_project_text(staged_text, None)
        .expect("decode staged browser project");
    assert_eq!(
        decoded.file.workspace.configuration_sets,
        state.workspace.content.configuration_sets
    );

    state.project_lifecycle.authority.cancel_transaction();
    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration).expect("browser revert");
    assert_eq!(
        state.workspace.content.configuration_sets,
        baseline.file.workspace.configuration_sets
    );
}

/// Rewrite the root of the project's Automation workspace, the code-workspace
/// source every new project carries.
fn edit_automation_source(state: &mut AppState, content: &str) {
    state
        .model
        .workspace
        .content
        .replace_project_source(
            crate::state::ProjectSourceLanguage::RSpiceAutomation,
            content.to_owned(),
        )
        .unwrap();
}

fn insert_cell_veriloga_source(
    state: &mut AppState,
    view_name: &str,
    content: &str,
) -> (CellViewRef, crate::state::ProjectSourceId) {
    let reference = CellViewRef::new(
        state.model.workspace.content.active_view.library.clone(),
        state.model.workspace.content.active_view.cell.clone(),
        view_name,
    );
    state
        .model
        .library_manager
        .get_library_mut(&reference.library)
        .and_then(|library| library.get_cell_mut(&reference.cell))
        .expect("active cell exists")
        .add_view(crate::state::View::new(
            view_name,
            crate::state::ViewType::VerilogA,
        ));
    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(reference.clone()),
        crate::state::ProjectSourceLanguage::VerilogA,
        format!("{view_name}.va"),
        content,
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid cell source bundle");
    let id = bundle.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("unique cell source owner");
    (reference, id)
}

fn insert_configuration_root(
    state: &mut AppState,
    name: &str,
    root: CellViewRef,
) -> crate::state::ConfigurationSetId {
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: name.to_owned(),
            root,
            dut_path: "/top".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Lifecycle test".to_owned(),
        })
        .expect("valid configuration")
}

#[test]
fn project_configuration_overlay_and_revert_own_exact_configuration_catalog() {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    let baseline = snapshot(&state).expect("baseline");
    state.open_project_session();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline.clone(), None);
    let id = insert_configuration_root(
        &mut state,
        "Release",
        CellViewRef::new("user", "top", "schematic"),
    );
    state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view("user/top/schematic", "Main", [1])
        .expect("design-management catalog");
    let authority = crate::state::pdk_config::PdkAdministrativeAuthority {
        actor_id: "callback-operator@rspice.invalid".to_owned(),
        authority_id: "test:lifecycle-callback".to_owned(),
    };
    let callback_receipt = state
        .execute_project_pdk_callback(
            "derive-device",
            &authority,
            "Verify callback lifecycle ownership",
        )
        .expect("execute exact project callback");
    let edited = snapshot(&state).expect("edited");
    let mut accepted = baseline.clone();

    accepted
        .file
        .overlay_document(&edited.file, &ProjectDocumentId::ProjectConfiguration)
        .expect("overlay project configuration");
    assert_eq!(
        accepted.file.workspace.configuration_sets,
        edited.file.workspace.configuration_sets
    );
    assert_eq!(
        accepted.file.workspace.design_management,
        edited.file.workspace.design_management
    );
    assert_eq!(
        accepted.file.workspace.pdk_callback_receipts(),
        [callback_receipt.clone()]
    );
    assert_eq!(
        accepted
            .file
            .workspace
            .configuration_sets
            .find(id)
            .expect("configuration persisted")
            .semantic_digest(),
        edited
            .file
            .workspace
            .configuration_sets
            .find(id)
            .unwrap()
            .semantic_digest()
    );

    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration).expect("revert catalog");
    assert_eq!(
        state.model.workspace.content.configuration_sets,
        baseline.file.workspace.configuration_sets
    );
    assert_eq!(
        state.model.workspace.content.design_management,
        baseline.file.workspace.design_management
    );
    assert!(
        state
            .model
            .workspace
            .content
            .pdk_callback_receipts()
            .is_empty()
    );
    snapshot(&state).expect("reverted state remains valid");
}

#[test]
fn cell_veriloga_view_and_source_are_one_lifecycle_document() {
    let mut state = AppState::default();
    let (reference, source_id) = insert_cell_veriloga_source(
        &mut state,
        "behavior",
        "module behavior(p, n); inout p, n; endmodule",
    );
    state.model.workspace.content.active_view = reference.clone();
    state
        .model
        .workspace
        .content
        .open_views
        .push(crate::state::OpenCellView::new(
            reference.clone(),
            crate::state::ViewType::VerilogA,
        ));
    // A Verilog-A view is edited as a Design document.
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    let baseline = snapshot(&state).expect("baseline");
    state.open_project_session();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline.clone(), None);

    assert_eq!(
        active_document(&state),
        ProjectDocumentId::CellView(reference.clone())
    );
    state
        .model
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(
            source_id,
            "behavior.va",
            "module behavior(p, n); inout p, n; analog V(p,n) <+ 1; endmodule".to_owned(),
        )
        .expect("edit cell source");
    let edited = snapshot(&state).expect("edited");
    let mut target = baseline.clone();

    target
        .file
        .overlay_document(
            &edited.file,
            &ProjectDocumentId::CellView(reference.clone()),
        )
        .expect("overlay cell view");
    assert_eq!(
        target.file.workspace.project_sources.get_bundle(source_id),
        edited.file.workspace.project_sources.get_bundle(source_id)
    );

    revert_document(&mut state, ProjectDocumentId::CellView(reference.clone()))
        .expect("revert cell source");
    assert_eq!(
        state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(source_id),
        baseline
            .file
            .workspace
            .project_sources
            .get_bundle(source_id)
    );
    assert!(
        state
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some()
    );
}

#[test]
fn project_configuration_never_accepts_or_discards_unsaved_cell_views() {
    let state = AppState::default();
    let baseline = snapshot(&state).expect("baseline");
    let mut working_state = state;
    let (reference, source_id) = insert_cell_veriloga_source(
        &mut working_state,
        "behavior",
        "module behavior(p, n); inout p, n; endmodule",
    );
    let working = snapshot(&working_state).expect("working");
    let mut target = baseline.clone();

    target
        .file
        .overlay_document(&working.file, &ProjectDocumentId::ProjectConfiguration)
        .expect("overlay configuration");

    assert!(
        target
            .file
            .libraries
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_none(),
        "project configuration must not accept an unrelated view document"
    );
    assert!(
        target
            .file
            .workspace
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );

    working_state.open_project_session();
    working_state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    revert_document(&mut working_state, ProjectDocumentId::ProjectConfiguration)
        .expect("revert configuration");
    assert!(
        working_state
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some(),
        "reverting configuration must preserve a view in an accepted cell"
    );
    assert!(
        working_state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(source_id)
            .is_some()
    );
}

#[test]
fn reverting_new_cell_configuration_removes_orphan_sources_and_restores_focus() {
    let mut state = AppState::default();
    let baseline = snapshot(&state).expect("baseline");
    let reference = CellViewRef::new("user", "new_behavioral_cell", "behavior");
    let mut cell = crate::state::Cell::new(reference.cell.as_str());
    cell.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    cell.add_view(crate::state::View::new(
        reference.view.as_str(),
        crate::state::ViewType::VerilogA,
    ));
    state
        .model
        .library_manager
        .get_library_mut(&reference.library)
        .expect("project library")
        .add_cell(cell);
    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(reference.clone()),
        crate::state::ProjectSourceLanguage::VerilogA,
        "behavior.va",
        "module behavior(p, n); inout p, n; endmodule",
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid source bundle");
    let source_id = bundle.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    insert_configuration_root(
        &mut state,
        "New cell root",
        CellViewRef::new(&reference.library, &reference.cell, "schematic"),
    );
    state.model.workspace.content.active_view = reference.clone();
    state
        .model
        .workspace
        .content
        .open_views
        .push(crate::state::OpenCellView::new(
            reference.clone(),
            crate::state::ViewType::VerilogA,
        ));
    state.open_project_session();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);

    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration)
        .expect("revert new cell configuration");

    assert!(
        state
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .is_none()
    );
    assert!(
        state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );
    assert_ne!(state.model.workspace.content.active_view, reference);
    assert!(
        state
            .model
            .library_manager
            .get_library(&state.model.workspace.content.active_view.library)
            .and_then(|library| library.get_cell(&state.model.workspace.content.active_view.cell))
            .and_then(|cell| cell.get_view(&state.model.workspace.content.active_view.view))
            .is_some()
    );
    assert!(
        state
            .model
            .workspace
            .content
            .configuration_sets
            .configurations()
            .is_empty()
    );
    snapshot(&state).expect("atomic configuration revert remains valid");
}

#[test]
fn reverting_new_cell_view_removes_its_source_without_touching_code_workspace() {
    let mut state = AppState::default();
    edit_automation_source(&mut state, "print('workspace')\n");
    let code_workspace = state.model.workspace.content.project_sources.clone();
    let baseline = snapshot(&state).expect("baseline");
    let (reference, source_id) = insert_cell_veriloga_source(
        &mut state,
        "behavior",
        "module behavior(p, n); inout p, n; endmodule",
    );
    state.open_project_session();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);

    revert_document(&mut state, ProjectDocumentId::CellView(reference.clone()))
        .expect("revert new view");

    assert!(
        state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .project_sources
            .get(crate::state::ProjectSourceLanguage::RSpiceAutomation),
        code_workspace.get(crate::state::ProjectSourceLanguage::RSpiceAutomation)
    );
    assert!(
        state
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_none()
    );
}

#[test]
fn reverting_the_code_document_restores_sources_and_clears_dirty_state() {
    let mut state = AppState::default();
    let baseline = snapshot(&state).unwrap();
    state.open_project_session();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline.clone(), None);
    state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    edit_automation_source(&mut state, "print('edited')\n");
    assert!(state.model.workspace.content.project_sources_dirty);

    revert_document(&mut state, ProjectDocumentId::NetlistSource).unwrap();

    assert_eq!(
        state.model.workspace.content.project_sources,
        baseline.file.workspace.project_sources
    );
    assert!(!state.model.workspace.content.project_sources_dirty);
    assert!(!state.model.workspace.content.netlist_source_dirty);
}

#[test]
fn lifecycle_epoch_advances_across_new_and_close() {
    let mut state = AppState::default();
    let receipt = NativeBindingReceipt {
        canonical_path: PathBuf::from("accepted.rspiceproj"),
        project_id: state.model.workspace.content.project.id().to_string(),
        accepted_digest: crate::product::ContentDigest::from_bytes([0x44; 32]),
    };
    state.platform.native_project_binding_receipt = Some(receipt.clone());
    let initial = state.platform.project_lifecycle.authority.epoch();
    reset_for_new_project(&mut state);
    state.assert_project_open_fact();
    let after_new = state.platform.project_lifecycle.authority.epoch();
    assert!(state.platform.native_project_binding_receipt.is_none());
    state.platform.native_project_binding_receipt = Some(receipt);
    mark_project_closed(&mut state);
    state.assert_project_open_fact();
    let after_close = state.platform.project_lifecycle.authority.epoch();

    assert!(after_new > initial);
    assert!(after_close > after_new);
    assert!(state.platform.native_project_binding_receipt.is_none());
}

/// Model code reads whether a project is open from its own copy, so every
/// transition that opens or closes the project has to carry the copy with it.
#[test]
fn the_model_reads_the_open_project_every_transition_leaves() {
    let mut state = AppState::default();
    assert!(state.model.project_open.is_open());
    state.assert_project_open_fact();
    let baseline = snapshot(&state).expect("baseline");

    mark_project_closed(&mut state);
    assert!(!state.model.project_open.is_open());
    state.assert_project_open_fact();

    accept_loaded_project(&mut state, baseline, None);
    assert!(state.model.project_open.is_open());
    state.assert_project_open_fact();

    mark_project_closed(&mut state);
    reset_for_new_project(&mut state);
    assert!(state.model.project_open.is_open());
    state.assert_project_open_fact();

    state
        .platform
        .project_lifecycle
        .close_restored_session(&mut state.model.project_open);
    assert!(!state.model.project_open.is_open());
    state.assert_project_open_fact();

    state.open_project_session();
    assert!(state.model.project_open.is_open());
    state.close_project_session();
    assert!(!state.model.project_open.is_open());
    state.assert_project_open_fact();
}

#[test]
fn project_replacement_clears_open_model_editor_state() {
    use crate::state::model_library::ProjectModelDefinition;
    use std::collections::BTreeMap;

    let mut state = AppState::default();
    state
        .model
        .model_library_manager
        .create_project_model(
            "owned-models",
            &ProjectModelDefinition {
                name: "nch_owned".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Project model".to_owned(),
                numeric_parameters: BTreeMap::new(),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create project model");
    state
        .models
        .model_editor
        .open(
            &state.model.model_library_manager,
            "owned-models",
            "nch_owned",
            state.model.workspace.content.project.revision(),
        )
        .expect("open editor");
    state
        .models
        .model_editor
        .draft
        .as_mut()
        .expect("draft")
        .description = "Unsaved".to_owned();
    state.models.capture_authoring_recovery();
    assert!(state.models.model_editor.draft.is_some());
    assert!(state.models.model_editor_recovery.is_some());

    reset_for_new_project(&mut state);
    state.assert_project_open_fact();
    assert!(state.models.model_editor.draft.is_none());
    assert!(state.models.model_editor_recovery.is_none());

    state
        .models
        .model_editor
        .open(
            &state.model.model_library_manager,
            "owned-models",
            "nch_owned",
            state.model.workspace.content.project.revision(),
        )
        .expect("reopen editor");
    mark_project_closed(&mut state);
    state.assert_project_open_fact();
    assert!(state.models.model_editor.draft.is_none());
    assert!(state.models.model_editor_recovery.is_none());
}

#[cfg(not(target_arch = "wasm32"))]
fn unique_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rspice-lifecycle-{label}-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4()
    ))
}

#[cfg(not(target_arch = "wasm32"))]
fn remove_project_artifacts(path: &Path) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(PathBuf::from(lock));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_session_restore_requires_exact_path_project_and_digest_receipt() {
    let path = unique_path("session-receipt");
    // Keep the several full application snapshots in this adversarial test on
    // the heap. AppState is intentionally broad, and retaining every scenario
    // in one native test must not depend on the test harness thread's small
    // default stack.
    let mut state = Box::new(AppState::default());
    save_native(
        state.as_mut(),
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("establish canonical binding");
    let canonical = normalize_native_path(&path).expect("normalize fixture");
    let receipt = state
        .platform
        .native_project_binding_receipt
        .clone()
        .expect("successful save records receipt");
    let accepted_bytes = std::fs::read(&canonical).expect("read accepted bytes");
    let session = serde_json::to_string(state.as_ref()).expect("serialize accepted session");

    assert_eq!(receipt.canonical_path, canonical);
    assert_eq!(
        receipt.project_id,
        state.model.workspace.content.project.id().to_string()
    );

    let mut exact = serde_json::from_str::<Box<AppState>>(&session).expect("restore exact session");
    initialize_from_session(exact.as_mut());
    assert_eq!(
        canonical_native_path(&exact.platform),
        Some(canonical.clone())
    );
    assert!(exact.platform.project_lifecycle.accepted().is_some());

    let mut same_project =
        Box::new(crate::io::load_project_file(&canonical).expect("load fixture"));
    same_project
        .file
        .workspace
        .project
        .rename("Externally renamed project")
        .expect("valid project name");
    let changed = crate::io::project_io::serialize_project_file(same_project.as_ref())
        .expect("serialize same-UUID replacement");
    std::fs::write(&canonical, changed).expect("write same-UUID replacement");
    let changed_bytes = std::fs::read(&canonical).expect("capture replacement bytes");

    let mut digest_conflict =
        serde_json::from_str::<Box<AppState>>(&session).expect("restore conflicted session");
    initialize_from_session(digest_conflict.as_mut());
    assert!(
        digest_conflict
            .platform
            .project_lifecycle
            .accepted()
            .is_none()
    );
    assert!(canonical_native_path(&digest_conflict.platform).is_none());
    assert_eq!(
        digest_conflict.platform.native_project_binding_receipt,
        Some(receipt.clone()),
        "conflict evidence must be retained"
    );
    assert_eq!(
        std::fs::read(&canonical).expect("read preserved replacement"),
        changed_bytes,
        "startup conflict handling must not rewrite the target"
    );

    let mut different =
        Box::new(snapshot(&AppState::default()).expect("snapshot different project"));
    different.file.workspace.project.set_path(canonical.clone());
    let different_bytes = crate::io::project_io::serialize_project_file(different.as_ref())
        .expect("serialize different-UUID replacement");
    std::fs::write(&canonical, different_bytes.as_bytes())
        .expect("write different-UUID replacement");
    let mut identity_conflict =
        serde_json::from_str::<Box<AppState>>(&session).expect("restore identity-conflict session");
    initialize_from_session(identity_conflict.as_mut());
    assert!(
        identity_conflict
            .platform
            .project_lifecycle
            .accepted()
            .is_none()
    );
    assert!(canonical_native_path(&identity_conflict.platform).is_none());

    std::fs::write(&canonical, &accepted_bytes).expect("restore exact fixture bytes");
    let mut legacy_value =
        serde_json::from_str::<serde_json::Value>(&session).expect("parse session JSON");
    legacy_value
        .as_object_mut()
        .expect("session object")
        .remove("native_project_binding_receipt");
    let mut legacy =
        serde_json::from_value::<Box<AppState>>(legacy_value).expect("restore legacy session");
    initialize_from_session(legacy.as_mut());
    assert!(legacy.platform.project_lifecycle.accepted().is_none());
    assert!(canonical_native_path(&legacy.platform).is_none());

    std::fs::remove_file(&canonical).expect("remove canonical fixture");
    let mut missing =
        serde_json::from_str::<Box<AppState>>(&session).expect("restore missing session");
    initialize_from_session(missing.as_mut());
    assert!(missing.platform.project_lifecycle.accepted().is_none());
    assert!(canonical_native_path(&missing.platform).is_none());
    remove_project_artifacts(&path);
}

/// Checks on save are a choice, and an advisory one. Nobody who did not ask
/// for them pays for a check pass, and nobody who did ask loses a save to one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn design_checks_on_save_are_off_by_default_and_never_refuse_the_save() {
    use crate::workbench::TogglePreference;

    let path = unique_path("design-checks-on-save");
    let mut state = Box::new(AppState::default());
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(10, 10));
    assert!(
        !state
            .model
            .preferences
            .toggle(TogglePreference::RunDesignChecksOnSave),
        "the preference is off until the author turns it on"
    );

    save_native(
        state.as_mut(),
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the first canonical save publishes");
    assert!(
        state.model.design_checks.drc_results.is_none(),
        "a save nobody asked to check ran no checks"
    );

    state
        .model
        .preferences
        .set_toggle(TogglePreference::RunDesignChecksOnSave, true);
    state
        .model
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(90, 10));
    save_native(
        state.as_mut(),
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("findings are advisory: the save still publishes");

    let checked = state
        .model
        .design_checks
        .drc_results
        .as_ref()
        .expect("the save ran the checks the preference asked for");
    assert!(
        !checked.violations().is_empty(),
        "two unwired instances have open pins and no ground"
    );
    assert!(
        state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains("Design checks on save")),
        "the outcome is stated where the author is looking"
    );
    assert!(
        !state.model.log_buffer.entries().any(|entry| checked
            .violations()
            .iter()
            .any(|violation| entry.message == violation.message)),
        "a finding is a record on the Problems page, not a second line in the Console"
    );
    let subject = state.model.workspace.content.active_schematic_reference();
    assert_eq!(
        state.model.design_rule_finding_identities(&subject).len(),
        checked.violations().len(),
        "every finding the save's checks raised is a published record"
    );
    assert!(
        !state.model.schematic.session.is_dirty,
        "the published revision stands: the advisory changed nothing about it"
    );

    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_save_active_and_revert_preserve_exact_configuration_catalog() {
    let path = unique_path("active-configuration-catalog");
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Project);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("establish baseline");

    let release_id = insert_configuration_root(
        &mut state,
        "Release",
        CellViewRef::new("user", "top", "schematic"),
    );
    state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view("user/top/schematic", "Main", [1])
        .expect("design-management catalog");
    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save active project configuration");

    let persisted = crate::io::load_project_file(&path).expect("reload configuration save");
    assert_eq!(
        persisted.file.workspace.configuration_sets,
        state.model.workspace.content.configuration_sets
    );
    assert_eq!(
        persisted.file.workspace.design_management,
        state.model.workspace.content.design_management
    );
    assert_eq!(
        state
            .platform
            .project_lifecycle
            .accepted()
            .expect("accepted save")
            .baseline()
            .workspace
            .configuration_sets,
        state.model.workspace.content.configuration_sets
    );
    assert_eq!(
        state
            .platform
            .project_lifecycle
            .accepted()
            .expect("accepted save")
            .baseline()
            .workspace
            .design_management,
        state.model.workspace.content.design_management
    );

    state
        .model
        .workspace
        .content
        .configuration_sets
        .clone_configuration(release_id, 1, "Characterization")
        .expect("unsaved catalog edit");
    state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut("user/top/schematic")
        .expect("sheet catalog")
        .create_sheet(
            crate::state::SheetDefinition {
                name: "Characterization".to_owned(),
                template: crate::state::SheetTemplate::AnalogSchematic,
                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            None,
        )
        .expect("unsaved sheet edit");
    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration)
        .expect("revert to exact saved catalog");
    assert_eq!(
        state.model.workspace.content.configuration_sets,
        persisted.file.workspace.configuration_sets
    );
    assert_eq!(
        state.model.workspace.content.design_management,
        persisted.file.workspace.design_management
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .configuration_sets
            .configurations()
            .len(),
        1
    );
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn revert_is_document_scoped_and_rejects_active_document_and_baseline_races() {
    let path = unique_path("revert-races");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("save baseline");

    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 2));
    let ac_id = insert_ac_analysis(&mut state);
    let scoped = prepare_revert_active_document(&state).expect("prepare scoped revert");
    confirm_revert_active_document(&mut state, &scoped).expect("confirm scoped revert");
    assert!(state.model.schematic.document().components.is_empty());
    assert_eq!(
        state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("live plan retained")
            .instance(ac_id)
            .expect("AC identity retained")
            .kind(),
        AnalysisKind::Ac
    );
    assert!(has_unsaved_changes(&state));

    state
        .model
        .schematic
        .add_component(ComponentType::Inductor, Point::new(4, 7));
    let active_race = prepare_revert_active_document(&state).expect("prepare active race");
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Project);
    let active_race_error = confirm_revert_active_document(&mut state, &active_race)
        .expect_err("changed active document invalidates review");
    assert!(matches!(
        active_race_error,
        ProjectLifecycleError::RevertReviewStale
    ));
    assert_eq!(state.model.schematic.document().components.len(), 1);

    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    let baseline_race = prepare_revert_active_document(&state).expect("prepare baseline race");
    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("advance accepted baseline");
    state
        .model
        .schematic
        .add_component(ComponentType::Diode, Point::new(8, 12));
    let baseline_race_error = confirm_revert_active_document(&mut state, &baseline_race)
        .expect_err("changed accepted baseline invalidates review");
    assert!(matches!(
        baseline_race_error,
        ProjectLifecycleError::RevertReviewStale
    ));
    assert_eq!(state.model.schematic.document().components.len(), 2);
    remove_project_artifacts(&path);
}

#[test]
fn close_active_document_is_presentation_only() {
    let mut state = AppState::default();
    let second = CellViewRef::new("user", "second", "schematic");
    let mut cell = crate::state::Cell::new("second");
    cell.add_view(crate::state::View::new("schematic", ViewType::Schematic));
    state
        .model
        .library_manager
        .get_library_mut("user")
        .expect("user library")
        .add_cell(cell);
    state
        .model
        .workspace
        .open_view(second.clone(), ViewType::Schematic);
    state.restore_active_schematic_from_workspace();
    state
        .model
        .schematic
        .add_component(ComponentType::Inductor, Point::new(3, 5));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);

    close_active_document(&mut state).expect("close presentation");

    assert!(
        !state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == second)
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&second.key())
            .expect("closed document data retained")
            .document()
            .components
            .len(),
        1
    );
}
