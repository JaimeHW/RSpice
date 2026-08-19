//! Tests for open, save, revert, and close.
//!
//! Each case pins a scope boundary: saving a document must not dirty the
//! project configuration, revert must not race the active document, and a
//! close is presentation-only.

use super::*;
#[cfg(not(target_arch = "wasm32"))]
use crate::simulation::plan::AnalysisKind;
use crate::state::{ComponentType, Point};

#[cfg(not(target_arch = "wasm32"))]
fn insert_ac_analysis(state: &mut AppState) -> crate::product::AnalysisInstanceId {
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan")
        .insert(AnalysisKind::Ac)
        .expect("AC analysis inserts")
        .0
}

#[cfg(not(target_arch = "wasm32"))]
fn has_ac_analysis(setup: &crate::workbench::app_state::SimSetupState) -> bool {
    setup
        .stable_analysis_plan()
        .expect("current project owns a stable plan")
        .instances()
        .iter()
        .any(|instance| instance.kind() == AnalysisKind::Ac)
}

#[test]
fn browser_completion_context_rejects_every_authority_change() {
    let digest = persistence::digest_bytes(b"accepted browser bytes");
    let receipt = BrowserBindingReceipt {
        binding_id: uuid::Uuid::from_u128(0xf20c_f308_17a1_4fc4_8b0d_8f09_eab7_35c2),
        project_id: "logical-project".to_owned(),
        accepted_generation: 4,
        accepted_digest: digest,
        backend: persistence::BrowserBindingBackend::Opfs,
    };
    let context = BrowserOperationContext {
        epoch: 11,
        operation_generation: 3,
        project_id: receipt.project_id.clone(),
        binding_receipt: Some(receipt.clone()),
        accepted_generation: 9,
    };

    assert!(operation_context_matches(
        &context,
        11,
        3,
        "logical-project",
        Some(&receipt),
        9,
    ));
    assert!(!operation_context_matches(
        &context,
        12,
        3,
        "logical-project",
        Some(&receipt),
        9,
    ));
    assert!(!operation_context_matches(
        &context,
        11,
        3,
        "replacement-project",
        Some(&receipt),
        9,
    ));
    assert!(!operation_context_matches(
        &context,
        11,
        3,
        "logical-project",
        None,
        9,
    ));
    assert!(!operation_context_matches(
        &context,
        11,
        3,
        "logical-project",
        Some(&receipt),
        10,
    ));
    assert!(!operation_context_matches(
        &context,
        11,
        4,
        "logical-project",
        Some(&receipt),
        9,
    ));
}

#[cfg(target_arch = "wasm32")]
#[test]
fn browser_save_active_and_revert_preserve_exact_configuration_catalog() {
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Project);
    let baseline = snapshot(&state).expect("baseline");
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline: baseline.clone(),
        binding: None,
    });
    insert_configuration_root(
        &mut state,
        "Browser release",
        CellViewRef::new("user", "top", "schematic"),
    );

    let prepared = prepare_browser_save(
        &mut state,
        SaveScope::ActiveDocument,
        false,
        "browser-project.rspice".to_owned(),
    )
    .expect("prepare browser active save");
    assert_eq!(
        prepared.saved_document,
        ProjectDocumentId::ProjectConfiguration
    );
    assert_eq!(
        prepared.candidate.workspace.configuration_sets,
        state.workspace.configuration_sets
    );
    let staged_text = std::str::from_utf8(&prepared.bytes).expect("UTF-8 project bytes");
    let decoded = crate::io::project_io::load_project_text(staged_text, None)
        .expect("decode staged browser project");
    assert_eq!(
        decoded.workspace.configuration_sets,
        state.workspace.configuration_sets
    );

    state.project_lifecycle.transaction = None;
    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration).expect("browser revert");
    assert_eq!(
        state.workspace.configuration_sets,
        baseline.workspace.configuration_sets
    );
}

fn ensure_veriloga_source(state: &mut AppState, content: &str) {
    if state
        .workspace
        .project_sources
        .get(crate::state::ProjectSourceLanguage::VerilogA)
        .is_none()
    {
        state
            .workspace
            .project_sources
            .insert(
                crate::state::ProjectSourceDocument::try_new(
                    "sensor_bridge.va",
                    crate::state::ProjectSourceLanguage::VerilogA,
                    content,
                )
                .unwrap(),
            )
            .unwrap();
    }
}

fn insert_cell_veriloga_source(
    state: &mut AppState,
    view_name: &str,
    content: &str,
) -> (CellViewRef, crate::state::ProjectSourceId) {
    let reference = CellViewRef::new(
        state.workspace.active_view.library.clone(),
        state.workspace.active_view.cell.clone(),
        view_name,
    );
    state
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
        .workspace
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
        .workspace
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
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline: baseline.clone(),
        binding: None,
    });
    let id = insert_configuration_root(
        &mut state,
        "Release",
        CellViewRef::new("user", "top", "schematic"),
    );
    state
        .workspace
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

    overlay_document(
        &mut accepted,
        &edited,
        &ProjectDocumentId::ProjectConfiguration,
    )
    .expect("overlay project configuration");
    assert_eq!(
        accepted.workspace.configuration_sets,
        edited.workspace.configuration_sets
    );
    assert_eq!(
        accepted.workspace.design_management,
        edited.workspace.design_management
    );
    assert_eq!(
        accepted.workspace.pdk_callback_receipts(),
        [callback_receipt.clone()]
    );
    assert_eq!(
        accepted
            .workspace
            .configuration_sets
            .find(id)
            .expect("configuration persisted")
            .semantic_digest(),
        edited
            .workspace
            .configuration_sets
            .find(id)
            .unwrap()
            .semantic_digest()
    );

    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration).expect("revert catalog");
    assert_eq!(
        state.workspace.configuration_sets,
        baseline.workspace.configuration_sets
    );
    assert_eq!(
        state.workspace.design_management,
        baseline.workspace.design_management
    );
    assert!(state.workspace.pdk_callback_receipts().is_empty());
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
    state.workspace.active_view = reference.clone();
    state
        .workspace
        .open_views
        .push(crate::state::OpenCellView::new(
            reference.clone(),
            crate::state::ViewType::VerilogA,
        ));
    state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    let baseline = snapshot(&state).expect("baseline");
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline: baseline.clone(),
        binding: None,
    });

    assert_eq!(
        active_document(&state),
        ProjectDocumentId::CellView(reference.clone())
    );
    state
        .workspace
        .project_sources
        .replace_bundle_file_content(
            source_id,
            "behavior.va",
            "module behavior(p, n); inout p, n; analog V(p,n) <+ 1; endmodule".to_owned(),
        )
        .expect("edit cell source");
    let edited = snapshot(&state).expect("edited");
    let mut target = baseline.clone();

    overlay_document(
        &mut target,
        &edited,
        &ProjectDocumentId::CellView(reference.clone()),
    )
    .expect("overlay cell view");
    assert_eq!(
        target.workspace.project_sources.get_bundle(source_id),
        edited.workspace.project_sources.get_bundle(source_id)
    );

    revert_document(&mut state, ProjectDocumentId::CellView(reference.clone()))
        .expect("revert cell source");
    assert_eq!(
        state.workspace.project_sources.get_bundle(source_id),
        baseline.workspace.project_sources.get_bundle(source_id)
    );
    assert!(
        state
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

    overlay_document(
        &mut target,
        &working,
        &ProjectDocumentId::ProjectConfiguration,
    )
    .expect("overlay configuration");

    assert!(
        target
            .libraries
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_none(),
        "project configuration must not accept an unrelated view document"
    );
    assert!(
        target
            .workspace
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );

    working_state.project_lifecycle.project_open = true;
    working_state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline,
        binding: None,
    });
    revert_document(&mut working_state, ProjectDocumentId::ProjectConfiguration)
        .expect("revert configuration");
    assert!(
        working_state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some(),
        "reverting configuration must preserve a view in an accepted cell"
    );
    assert!(
        working_state
            .workspace
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
        .workspace
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    insert_configuration_root(
        &mut state,
        "New cell root",
        CellViewRef::new(&reference.library, &reference.cell, "schematic"),
    );
    state.workspace.active_view = reference.clone();
    state
        .workspace
        .open_views
        .push(crate::state::OpenCellView::new(
            reference.clone(),
            crate::state::ViewType::VerilogA,
        ));
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline,
        binding: None,
    });

    revert_document(&mut state, ProjectDocumentId::ProjectConfiguration)
        .expect("revert new cell configuration");

    assert!(
        state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .is_none()
    );
    assert!(
        state
            .workspace
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );
    assert_ne!(state.workspace.active_view, reference);
    assert!(
        state
            .library_manager
            .get_library(&state.workspace.active_view.library)
            .and_then(|library| library.get_cell(&state.workspace.active_view.cell))
            .and_then(|cell| cell.get_view(&state.workspace.active_view.view))
            .is_some()
    );
    assert!(
        state
            .workspace
            .configuration_sets
            .configurations()
            .is_empty()
    );
    snapshot(&state).expect("atomic configuration revert remains valid");
}

#[test]
fn reverting_new_cell_view_removes_its_source_without_touching_code_workspace() {
    let mut state = AppState::default();
    ensure_veriloga_source(&mut state, "module workspace_model; endmodule");
    let code_workspace = state.workspace.project_sources.clone();
    let baseline = snapshot(&state).expect("baseline");
    let (reference, source_id) = insert_cell_veriloga_source(
        &mut state,
        "behavior",
        "module behavior(p, n); inout p, n; endmodule",
    );
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline,
        binding: None,
    });

    revert_document(&mut state, ProjectDocumentId::CellView(reference.clone()))
        .expect("revert new view");

    assert!(
        state
            .workspace
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );
    assert_eq!(
        state
            .workspace
            .project_sources
            .get(crate::state::ProjectSourceLanguage::VerilogA),
        code_workspace.get(crate::state::ProjectSourceLanguage::VerilogA)
    );
    assert!(
        state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_none()
    );
}

#[test]
fn code_document_overlay_copies_project_sources_atomically() {
    let mut state = AppState::default();
    ensure_veriloga_source(&mut state, "module sensor_bridge; endmodule");
    let mut target = snapshot(&state).unwrap();
    state
        .workspace
        .replace_project_source(
            crate::state::ProjectSourceLanguage::VerilogA,
            "module sensor_bridge; analog begin end endmodule".to_owned(),
        )
        .unwrap();
    let working = snapshot(&state).unwrap();

    overlay_document(&mut target, &working, &ProjectDocumentId::NetlistSource).unwrap();

    assert_eq!(
        target.workspace.project_sources,
        working.workspace.project_sources
    );
    assert_eq!(
        target.workspace.netlist_source,
        working.workspace.netlist_source
    );
}

#[test]
fn code_sources_change_generated_netlist_and_execution_identity() {
    let mut state = AppState::default();
    ensure_veriloga_source(&mut state, "module sensor_bridge; endmodule");
    let before = generated_netlist_input_digest(&state).unwrap();

    state
        .workspace
        .replace_project_source(
            crate::state::ProjectSourceLanguage::VerilogA,
            "module sensor_bridge; analog begin end endmodule".to_owned(),
        )
        .unwrap();

    assert_ne!(generated_netlist_input_digest(&state).unwrap(), before);
}

#[test]
fn reverting_the_code_document_restores_sources_and_clears_dirty_state() {
    let mut state = AppState::default();
    ensure_veriloga_source(&mut state, "module sensor_bridge; endmodule");
    let baseline = snapshot(&state).unwrap();
    state.project_lifecycle.project_open = true;
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline: baseline.clone(),
        binding: None,
    });
    state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    state
        .workspace
        .replace_project_source(
            crate::state::ProjectSourceLanguage::VerilogA,
            "module sensor_bridge; analog begin end endmodule".to_owned(),
        )
        .unwrap();
    assert!(state.workspace.project_sources_dirty);

    revert_document(&mut state, ProjectDocumentId::NetlistSource).unwrap();

    assert_eq!(
        state.workspace.project_sources,
        baseline.workspace.project_sources
    );
    assert!(!state.workspace.project_sources_dirty);
    assert!(!state.workspace.netlist_source_dirty);
}

#[test]
fn lifecycle_epoch_advances_across_new_and_close() {
    let mut state = AppState::default();
    let receipt = NativeBindingReceipt {
        canonical_path: PathBuf::from("accepted.rspiceproj"),
        project_id: state.workspace.project.id().to_string(),
        accepted_digest: crate::product::ContentDigest::from_bytes([0x44; 32]),
    };
    state.native_project_binding_receipt = Some(receipt.clone());
    let initial = state.project_lifecycle.epoch;
    reset_for_new_project(&mut state);
    let after_new = state.project_lifecycle.epoch;
    assert!(state.native_project_binding_receipt.is_none());
    state.native_project_binding_receipt = Some(receipt);
    mark_project_closed(&mut state);
    let after_close = state.project_lifecycle.epoch;

    assert!(after_new > initial);
    assert!(after_close > after_new);
    assert!(state.native_project_binding_receipt.is_none());
}

#[test]
fn project_replacement_clears_open_model_editor_state() {
    use crate::state::model_library::ProjectModelDefinition;
    use std::collections::BTreeMap;

    let mut state = AppState::default();
    state
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
        .workbench
        .model_editor
        .open(
            &state.model_library_manager,
            "owned-models",
            "nch_owned",
            state.workspace.project.revision(),
        )
        .expect("open editor");
    state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("draft")
        .description = "Unsaved".to_owned();
    state.workbench.capture_authoring_recovery();
    assert!(state.workbench.model_editor.draft.is_some());
    assert!(state.workbench.model_editor_recovery.is_some());

    reset_for_new_project(&mut state);
    assert!(state.workbench.model_editor.draft.is_none());
    assert!(state.workbench.model_editor_recovery.is_none());

    state
        .workbench
        .model_editor
        .open(
            &state.model_library_manager,
            "owned-models",
            "nch_owned",
            state.workspace.project.revision(),
        )
        .expect("reopen editor");
    mark_project_closed(&mut state);
    assert!(state.workbench.model_editor.draft.is_none());
    assert!(state.workbench.model_editor_recovery.is_none());
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
fn safe_mode_read_only_policy_blocks_native_project_writes_before_publication() {
    let path = unique_path("safe-mode-read-only");
    let mut state = AppState::default();
    state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            disable_third_party_extensions: false,
            disable_gpu_acceleration: false,
            isolate_prior_documents: false,
            reset_layout: false,
            open_project_read_only: true,
        },
        "protected session".to_owned(),
    );

    let error = save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect_err("read-only safe mode must reject project writes");

    assert!(matches!(error, ProjectLifecycleError::SafeModeReadOnly));
    assert!(!path.exists());
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
        .native_project_binding_receipt
        .clone()
        .expect("successful save records receipt");
    let accepted_bytes = std::fs::read(&canonical).expect("read accepted bytes");
    let session = serde_json::to_string(state.as_ref()).expect("serialize accepted session");

    assert_eq!(receipt.canonical_path, canonical);
    assert_eq!(receipt.project_id, state.workspace.project.id().to_string());

    let mut exact = serde_json::from_str::<Box<AppState>>(&session).expect("restore exact session");
    initialize_from_session(exact.as_mut());
    assert_eq!(canonical_native_path(&exact), Some(canonical.clone()));
    assert!(exact.project_lifecycle.accepted.is_some());

    let mut same_project =
        Box::new(crate::io::load_project_file(&canonical).expect("load fixture"));
    same_project
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
    assert!(digest_conflict.project_lifecycle.accepted.is_none());
    assert!(canonical_native_path(&digest_conflict).is_none());
    assert_eq!(
        digest_conflict.native_project_binding_receipt,
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
    different.workspace.project.set_path(canonical.clone());
    let different_bytes = crate::io::project_io::serialize_project_file(different.as_ref())
        .expect("serialize different-UUID replacement");
    std::fs::write(&canonical, different_bytes.as_bytes())
        .expect("write different-UUID replacement");
    let mut identity_conflict =
        serde_json::from_str::<Box<AppState>>(&session).expect("restore identity-conflict session");
    initialize_from_session(identity_conflict.as_mut());
    assert!(identity_conflict.project_lifecycle.accepted.is_none());
    assert!(canonical_native_path(&identity_conflict).is_none());

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
    assert!(legacy.project_lifecycle.accepted.is_none());
    assert!(canonical_native_path(&legacy).is_none());

    std::fs::remove_file(&canonical).expect("remove canonical fixture");
    let mut missing =
        serde_json::from_str::<Box<AppState>>(&session).expect("restore missing session");
    initialize_from_session(missing.as_mut());
    assert!(missing.project_lifecycle.accepted.is_none());
    assert!(canonical_native_path(&missing).is_none());
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn user_selected_native_publication_rejects_late_create_and_edit() {
    let path = unique_path("picker-cas");
    let missing = persistence::observe_native_destination(&path)
        .expect("observe picker destination as missing");
    std::fs::write(&path, b"created after picker").expect("simulate late create");

    let create_conflict =
        persistence::publish_canonical_native(&path, missing, b"local project bytes")
            .expect_err("late create must block publication");
    assert!(matches!(
        create_conflict,
        persistence::PersistenceError::ExternalChange
    ));
    assert_eq!(
        std::fs::read(&path).expect("read late create"),
        b"created after picker"
    );

    let accepted =
        persistence::observe_native_destination(&path).expect("capture exact picker-time bytes");
    std::fs::write(&path, b"edited after picker").expect("simulate late edit");
    let edit_conflict =
        persistence::publish_canonical_native(&path, accepted, b"local project bytes")
            .expect_err("late edit must block publication");
    assert!(matches!(
        edit_conflict,
        persistence::PersistenceError::ExternalChange
    ));
    assert_eq!(
        std::fs::read(&path).expect("read late edit"),
        b"edited after picker"
    );
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_active_overlays_only_active_document_on_accepted_baseline() {
    let path = unique_path("active-overlay");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("first full save");

    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(4, 8));
    let ac_id = insert_ac_analysis(&mut state);
    assert!(
        save_native(
            &mut state,
            SaveScope::ActiveDocument,
            &path,
            DestinationAuthority::Canonical,
        )
        .is_ok()
    );

    let persisted = crate::io::load_project_file(&path).expect("reload saved project");
    let persisted_context = persisted.execution_context.expect("execution context");
    assert_eq!(
        persisted
            .workspace
            .schematic_buffers
            .get(&state.workspace.active_key())
            .expect("active buffer")
            .components
            .len(),
        1
    );
    assert!(
        !has_ac_analysis(&persisted_context.simulation_plan),
        "unrelated plan draft must remain outside an active-design save"
    );
    assert_eq!(
        state
            .sim_setup
            .stable_analysis_plan()
            .expect("live project owns a stable plan")
            .instance(ac_id)
            .expect("active-design save retains the exact live AC identity")
            .kind(),
        AnalysisKind::Ac
    );
    assert!(has_unsaved_changes(&state));
    assert!(!active_document_is_dirty(&state));
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
        .workspace
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
        persisted.workspace.configuration_sets,
        state.workspace.configuration_sets
    );
    assert_eq!(
        persisted.workspace.design_management,
        state.workspace.design_management
    );
    assert_eq!(
        state
            .project_lifecycle
            .accepted
            .as_ref()
            .expect("accepted save")
            .baseline
            .workspace
            .configuration_sets,
        state.workspace.configuration_sets
    );
    assert_eq!(
        state
            .project_lifecycle
            .accepted
            .as_ref()
            .expect("accepted save")
            .baseline
            .workspace
            .design_management,
        state.workspace.design_management
    );

    state
        .workspace
        .configuration_sets
        .clone_configuration(release_id, 1, "Characterization")
        .expect("unsaved catalog edit");
    state
        .workspace
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
        state.workspace.configuration_sets,
        persisted.workspace.configuration_sets
    );
    assert_eq!(
        state.workspace.design_management,
        persisted.workspace.design_management
    );
    assert_eq!(state.workspace.configuration_sets.configurations().len(), 1);
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_active_design_preserves_unpublished_live_project_descriptor() {
    let path = unique_path("active-preserves-project-draft");
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("establish baseline");
    let accepted_name = state.workspace.project.name().to_owned();

    state
        .workspace
        .project
        .rename("Unpublished descriptor draft")
        .expect("valid draft name");
    state.workspace.project.description = "not part of design Save".to_owned();
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(9, 4));

    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save active design only");

    assert_eq!(
        state.workspace.project.name(),
        "Unpublished descriptor draft",
        "successful design save must not roll back a different document's draft"
    );
    assert_eq!(
        state.workspace.project.description,
        "not part of design Save"
    );
    assert!(
        state
            .project_lifecycle
            .registry
            .is_dirty(&ProjectDocumentId::ProjectConfiguration)
    );
    assert!(!active_document_is_dirty(&state));

    let persisted = crate::io::load_project_file(&path).expect("reload active save");
    assert_eq!(persisted.workspace.project.name(), accepted_name);
    assert!(persisted.workspace.project.description.is_empty());
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn newer_edits_after_saved_snapshot_revoke_destructive_continuation() {
    let path = unique_path("continuation-guard");
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("establish accepted snapshot");
    let saved_document = active_document(&state);

    assert!(saved_snapshot_authorizes_continuation(
        &state,
        SaveScope::AllDocuments,
        &saved_document
    ));
    state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(6, 2));
    assert!(
        !saved_snapshot_authorizes_continuation(&state, SaveScope::AllDocuments, &saved_document),
        "post-snapshot project edits require another Save All before replacement"
    );
    assert!(
        !saved_snapshot_authorizes_continuation(&state, SaveScope::ActiveDocument, &saved_document),
        "post-snapshot active-document edits require another save"
    );

    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save active document");
    assert!(saved_snapshot_authorizes_continuation(
        &state,
        SaveScope::ActiveDocument,
        &saved_document
    ));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Project);
    assert!(
        !saved_snapshot_authorizes_continuation(&state, SaveScope::ActiveDocument, &saved_document),
        "an active-tab change cannot redirect an old continuation"
    );
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_all_commits_complete_working_set() {
    let path = unique_path("save-all");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("first save");
    let ac_id = insert_ac_analysis(&mut state);

    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save all");

    let persisted = crate::io::load_project_file(&path).expect("reload");
    let persisted_context = persisted.execution_context.expect("context");
    assert_eq!(
        persisted_context
            .simulation_plan
            .stable_analysis_plan()
            .expect("saved project owns a stable plan")
            .instance(ac_id)
            .expect("saved plan retains the exact AC identity")
            .kind(),
        AnalysisKind::Ac
    );
    assert!(!has_unsaved_changes(&state));
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn project_copy_does_not_rebind_or_clean_source_project() {
    let source = unique_path("copy-source");
    let copy = unique_path("copy-target");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &source,
        DestinationAuthority::UserSelected,
    )
    .expect("save source");
    let source_id = state.workspace.project.id();
    let source_receipt = state.native_project_binding_receipt.clone();
    state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(1, 2));

    save_project_copy_native(&mut state, &copy).expect("save independent copy");

    assert_eq!(state.workspace.project.id(), source_id);
    assert_eq!(state.native_project_binding_receipt, source_receipt);
    assert_eq!(
        canonical_native_path(&state),
        Some(normalize_native_path(&source).expect("normalize source"))
    );
    assert!(has_unsaved_changes(&state));
    let copied = crate::io::load_project_file(&copy).expect("load copy");
    assert_ne!(copied.workspace.project.id(), source_id);
    remove_project_artifacts(&source);
    remove_project_artifacts(&copy);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn project_copy_rejects_direct_normalized_symlink_and_hardlink_aliases() {
    let source = unique_path("copy-alias-source");
    let hardlink = unique_path("copy-alias-hardlink");
    let symlink = unique_path("copy-alias-symlink");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &source,
        DestinationAuthority::UserSelected,
    )
    .expect("save canonical source");
    state
        .schematic
        .add_component(ComponentType::Diode, Point::new(7, 9));

    let direct = save_project_copy_native(&mut state, &source)
        .expect_err("direct canonical destination must be rejected");
    assert!(matches!(
        direct,
        ProjectLifecycleError::CopyDestinationIsCanonical
    ));

    let normalized_alias = source
        .parent()
        .expect("temporary parent")
        .join(".")
        .join(source.file_name().expect("source filename"));
    let normalized = save_project_copy_native(&mut state, &normalized_alias)
        .expect_err("normalized canonical alias must be rejected");
    assert!(matches!(
        normalized,
        ProjectLifecycleError::CopyDestinationIsCanonical
    ));

    std::fs::hard_link(&source, &hardlink).expect("create hardlink alias");
    let hardlinked = save_project_copy_native(&mut state, &hardlink)
        .expect_err("hardlink alias must be rejected");
    assert!(matches!(
        hardlinked,
        ProjectLifecycleError::CopyDestinationIsCanonical
    ));

    #[cfg(unix)]
    std::os::unix::fs::symlink(&source, &symlink).expect("create symlink alias");
    #[cfg(windows)]
    let symlink_created = std::os::windows::fs::symlink_file(&source, &symlink).is_ok();
    #[cfg(unix)]
    let symlink_created = true;
    if symlink_created {
        let symlinked = save_project_copy_native(&mut state, &symlink)
            .expect_err("symlink alias must be rejected");
        assert!(matches!(
            symlinked,
            ProjectLifecycleError::CopyDestinationIsCanonical
        ));
    }

    assert!(has_unsaved_changes(&state));
    remove_project_artifacts(&hardlink);
    remove_project_artifacts(&symlink);
    remove_project_artifacts(&source);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deleted_source_allows_recovery_copy_but_unreadable_canonical_path_is_rejected() {
    let source = unique_path("deleted-copy-source");
    let recovery = unique_path("deleted-copy-recovery");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &source,
        DestinationAuthority::UserSelected,
    )
    .expect("establish canonical source");
    std::fs::remove_file(&source).expect("simulate external source deletion");
    std::fs::write(&recovery, b"picker-observed prior recovery bytes")
        .expect("create existing recovery destination");
    state
        .schematic
        .add_component(ComponentType::Diode, Point::new(12, 7));

    save_project_copy_native(&mut state, &recovery)
        .expect("missing source cannot alias an independent recovery destination");
    let recovered = crate::io::load_project_file(&recovery).expect("load recovery copy");
    assert_ne!(
        recovered.workspace.project.id(),
        state.workspace.project.id()
    );

    let unreadable = unique_path("remembered-unreadable-canonical");
    std::fs::write(&unreadable, b"not a project").expect("create unreadable canonical");
    let unreadable = normalize_native_path(&unreadable).expect("normalize unreadable path");
    let mut unreadable_state = AppState::default();
    unreadable_state.project_lifecycle.unreadable_native_binding =
        Some(persistence::UnreadableNativeBinding {
            canonical_path: unreadable.clone(),
            reason: "invalid project bytes".to_owned(),
        });
    let before = std::fs::read(&unreadable).expect("capture unreadable bytes");
    let error = save_project_copy_native(&mut unreadable_state, &unreadable)
        .expect_err("Save Copy cannot overwrite remembered unreadable authority");
    assert!(matches!(
        error,
        ProjectLifecycleError::CopyDestinationIsCanonical
    ));
    assert_eq!(std::fs::read(&unreadable).unwrap(), before);

    remove_project_artifacts(&source);
    remove_project_artifacts(&recovery);
    remove_project_artifacts(&unreadable);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_all_preserves_live_document_presentation_while_sanitizing_persisted_copy() {
    let path = unique_path("presentation-preservation");
    let view_path = unique_path("view-presentation");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("save baseline");
    let active = state.workspace.active_view.clone();
    let open_count = state.workspace.open_views.len();
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(14, 3));
    let view = state
        .library_manager
        .get_library_mut(&active.library)
        .and_then(|library| library.get_cell_mut(&active.cell))
        .and_then(|cell| cell.get_view_mut(&active.view))
        .expect("active library view");
    view.is_open = true;
    view.modified = true;
    view.file_path = Some(view_path.clone());
    view.modified_time = Some(8_675_309);
    let governed_revision = state.library_manager.revision();

    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save all");

    assert_eq!(state.workspace.active_view, active);
    assert_eq!(state.workspace.open_views.len(), open_count);
    assert_eq!(
        state.library_manager.revision(),
        governed_revision,
        "save acceptance must not manufacture a catalog revision"
    );
    let live = state
        .library_manager
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view(&active.view))
        .expect("live active view");
    assert!(live.is_open);
    assert!(!live.modified);
    assert_eq!(live.file_path.as_deref(), Some(view_path.as_path()));
    assert_eq!(live.modified_time, Some(8_675_309));

    let persisted = crate::io::load_project_file(&path).expect("reload persisted project");
    assert_eq!(
        persisted.libraries.revision(),
        governed_revision,
        "persistence sanitization must preserve the exact governed revision"
    );
    let persisted_view = persisted
        .libraries
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view(&active.view))
        .expect("persisted active view");
    assert!(!persisted_view.is_open);
    assert!(!persisted_view.modified);
    assert!(persisted_view.file_path.is_none());
    assert!(persisted_view.modified_time.is_none());
    remove_project_artifacts(&path);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn saving_active_cell_never_dirties_project_configuration() {
    let path = unique_path("active-cell-config-boundary");
    let mut state = AppState::default();
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("save baseline");
    let active = state.workspace.active_view.clone();
    state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(5, 11));
    state
        .library_manager
        .get_library_mut(&active.library)
        .and_then(|library| library.get_cell_mut(&active.cell))
        .and_then(|cell| cell.get_view_mut(&active.view))
        .expect("active view")
        .metadata
        .insert(
            "document-setting".to_owned(),
            "engineering-value".to_owned(),
        );
    let governed_revision = state.library_manager.revision();

    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save active cell");

    assert!(
        !state
            .project_lifecycle
            .registry
            .is_dirty(&ProjectDocumentId::CellView(active))
    );
    assert!(
        !state
            .project_lifecycle
            .registry
            .is_dirty(&ProjectDocumentId::ProjectConfiguration)
    );
    assert_eq!(
        state.library_manager.revision(),
        governed_revision,
        "partial-save overlay must preserve the observed catalog revision"
    );
    assert_eq!(
        state
            .project_lifecycle
            .accepted
            .as_ref()
            .expect("accepted save")
            .baseline
            .libraries
            .revision(),
        governed_revision,
        "accepted partial-save artifact must record the observed revision"
    );
    assert!(!has_unsaved_changes(&state));
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
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 2));
    let ac_id = insert_ac_analysis(&mut state);
    let scoped = prepare_revert_active_document(&state).expect("prepare scoped revert");
    confirm_revert_active_document(&mut state, &scoped).expect("confirm scoped revert");
    assert!(state.schematic.components.is_empty());
    assert_eq!(
        state
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
    assert_eq!(state.schematic.components.len(), 1);

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
        .schematic
        .add_component(ComponentType::Diode, Point::new(8, 12));
    let baseline_race_error = confirm_revert_active_document(&mut state, &baseline_race)
        .expect_err("changed accepted baseline invalidates review");
    assert!(matches!(
        baseline_race_error,
        ProjectLifecycleError::RevertReviewStale
    ));
    assert_eq!(state.schematic.components.len(), 2);
    remove_project_artifacts(&path);
}

#[test]
fn close_active_document_is_presentation_only() {
    let mut state = AppState::default();
    let second = CellViewRef::new("user", "second", "schematic");
    assert!(state.library_manager.create_cell("user", "second"));
    assert!(
        state
            .library_manager
            .create_view("user", "second", "schematic", ViewType::Schematic)
    );
    state
        .workspace
        .open_view(second.clone(), ViewType::Schematic);
    state.restore_active_schematic_from_workspace();
    state
        .schematic
        .add_component(ComponentType::Inductor, Point::new(3, 5));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);

    close_active_document(&mut state).expect("close presentation");

    assert!(
        !state
            .workspace
            .open_views
            .iter()
            .any(|open| open.reference == second)
    );
    assert_eq!(
        state
            .workspace
            .schematic_buffers
            .get(&second.key())
            .expect("closed document data retained")
            .components
            .len(),
        1
    );
}

/// Publish one more sheet into a governed cell view, the way a sheet workflow
/// does: a reviewed catalog candidate through the project revision boundary.
#[cfg(not(target_arch = "wasm32"))]
fn add_sheet(state: &mut AppState, reference: &CellViewRef, name: &str) {
    let key = reference.key();
    let mut candidate = state.workspace.design_management.clone();
    let catalog = candidate
        .sheet_catalog_mut(&key)
        .expect("the cell view is governed");
    let last = catalog.sheets().last().map(crate::state::DesignSheet::id);
    catalog
        .create_sheet(
            crate::state::SheetDefinition {
                name: name.to_owned(),
                template: crate::state::SheetTemplate::AnalogSchematic,
                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: None,
            },
            last,
        )
        .expect("the new sheet is valid");
    state
        .workspace
        .replace_design_management(candidate)
        .expect("publish the reviewed catalog");
}

#[cfg(not(target_arch = "wasm32"))]
fn sheet_names(project: &ProjectFile, key: &str) -> Vec<String> {
    project
        .workspace
        .design_management
        .sheet_catalog(key)
        .expect("the cell view is governed")
        .sheets()
        .iter()
        .map(|sheet| sheet.name().to_owned())
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn saving_one_cell_view_publishes_only_that_cell_views_sheets() {
    let path = unique_path("cell-view-sheets");
    let mut state = AppState::default();
    let active = state.workspace.active_schematic_reference();
    let other = CellViewRef::new(&active.library, "aux", "schematic");
    if let Some(library) = state.library_manager.get_library_mut(&other.library) {
        let mut cell = crate::state::Cell::new(&other.cell);
        cell.add_view(crate::state::View::new(&other.view, ViewType::Schematic));
        library.add_cell(cell);
    }
    state
        .workspace
        .schematic_buffers
        .insert(other.key(), crate::state::SchematicState::default());
    for reference in [&active, &other] {
        state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&reference.key(), "Main", [])
            .expect("governed sheet catalog");
    }
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("establish baseline");

    add_sheet(&mut state, &active, "Power");
    add_sheet(&mut state, &other, "Analog");
    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save the active cell view");

    let persisted = crate::io::load_project_file(&path).expect("reload the document save");
    assert_eq!(
        sheet_names(&persisted, &active.key()),
        ["Main".to_owned(), "Power".to_owned()],
        "the saved cell view's own sheets travel with it"
    );
    assert_eq!(
        sheet_names(&persisted, &other.key()),
        ["Main".to_owned()],
        "another cell view's sheet edit stays unsaved"
    );
    assert_eq!(
        persisted.workspace.design_management.revision(),
        state.workspace.design_management.revision(),
        "the merged file records the revision the session is actually at"
    );
    remove_project_artifacts(&path);
}
