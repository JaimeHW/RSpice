//! Automation workflow tests.

use crate::product::{AnalysisInstanceId, ObjectRevision, ProjectId};
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, ProjectSourceRegistry,
    SimulationPlanPayload, WaveformData,
};

use super::*;

#[test]
fn source_workspace_runtime_transitions_have_no_production_panic_shortcuts() {
    // Every module that runs a source-workspace transaction belongs here, not
    // just its entry file: splitting a module must not silently narrow what
    // this guard scans. Add a submodule the same day you create it.
    for (name, source) in [
        ("Automation runtime", include_str!("../automation.rs")),
        ("Automation debugger", include_str!("debugger.rs")),
        ("Automation evidence", include_str!("evidence.rs")),
        ("Automation host calls", include_str!("host_calls.rs")),
        ("Code workspace state", include_str!("../page.rs")),
        ("Verilog-A runtime", include_str!("../veriloga.rs")),
        (
            "Netlist document runtime",
            include_str!("../../netlist_document.rs"),
        ),
        (
            "Netlist surface",
            include_str!("../../../surfaces/netlist.rs"),
        ),
        (
            "Netlist generation",
            include_str!("../../../surfaces/netlist/generation.rs"),
        ),
        (
            "Netlist ownership",
            include_str!("../../../surfaces/netlist/ownership.rs"),
        ),
        (
            "Netlist revision",
            include_str!("../../../surfaces/netlist/revision.rs"),
        ),
        (
            "Netlist search",
            include_str!("../../../surfaces/netlist/search.rs"),
        ),
        (
            "Netlist toolbar",
            include_str!("../../../surfaces/netlist/toolbar.rs"),
        ),
        (
            "Netlist transfer",
            include_str!("../../../surfaces/netlist/transfer.rs"),
        ),
    ] {
        let production = crate::source_guard::production_source(source);
        for forbidden in [
            ".expect(",
            ".unwrap(",
            "panic!(",
            "unreachable!(",
            "todo!(",
            "unimplemented!(",
        ] {
            assert!(
                !production.contains(forbidden),
                "{name} contains production panic shortcut {forbidden}"
            );
        }
    }
}

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn result(
    instance_id: AnalysisInstanceId,
    measurement: Option<rspice_core::MeasureResult>,
) -> AnalysisResult {
    let provenance =
        AnalysisResultProvenance::new(instance_id, ObjectRevision::INITIAL, digest(1), Vec::new())
            .expect("valid provenance");
    AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_provenance(provenance)
        .with_measurements(measurement.into_iter().collect())
}

fn spec(name: &str) -> SpecEntry {
    SpecEntry {
        measurement: name.to_owned(),
        expression: String::new(),
        min: Some(0.5),
        max: Some(1.5),
        unit: "V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    }
}

fn prepared_run(
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    plan_revision: ObjectRevision,
) -> SimulationRun {
    let instance_id = AnalysisInstanceId::new();
    let snapshot_digest = digest(0x31);
    let task = PreparedRunTaskReceipt::new(instance_id, plan_revision, Vec::new(), 5, digest(0x32))
        .expect("valid task");
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project_revision,
        snapshot_digest,
        digest(0x33),
        PreparedSourceCheckReceipt::SchematicDrc(digest(0x34)),
        vec![task],
    )
    .expect("valid receipt");
    let provenance =
        AnalysisResultProvenance::new(instance_id, plan_revision, snapshot_digest, Vec::new())
            .expect("valid provenance");
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_provenance(provenance)
        .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)])
        .with_waveforms(vec![WaveformData::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![1.0, 2.0],
            "#ffffff",
        )]);
    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(analysis);
    run
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn environment_authority_is_one_exact_multi_name_scope() {
    let app = RSpiceApp::test_instance();
    let documents = [
        AutomationSourceDocument {
            path: "characterize.py",
            source: crate::state::DEFAULT_AUTOMATION_PYTHON,
        },
        AutomationSourceDocument {
            path: "runplan.rspice.yaml",
            source: crate::state::DEFAULT_AUTOMATION_RUN_PLAN,
        },
        AutomationSourceDocument {
            path: "requirements.lock",
            source: crate::state::DEFAULT_ENVIRONMENT_LOCK,
        },
        AutomationSourceDocument {
            path: "permissions.toml",
            source: crate::state::DEFAULT_AUTOMATION_PERMISSIONS,
        },
    ];
    let roles = [
        AutomationRoleBinding {
            path: "runplan.rspice.yaml",
            role: AutomationSourceRole::RunPlan,
        },
        AutomationRoleBinding {
            path: "requirements.lock",
            role: AutomationSourceRole::EnvironmentLock,
        },
        AutomationRoleBinding {
            path: "permissions.toml",
            role: AutomationSourceRole::PermissionManifest,
        },
    ];
    let (_, mut manifest) = compile_automation_documents("characterize.py", &documents, &roles)
        .expect("valid workspace");
    manifest.environment = vec!["RSPICE_ALPHA".to_owned(), "RSPICE_BETA".to_owned()];

    let grants = automation_capabilities(&app, &manifest).expect("capability scope");
    let environment = grants
        .iter()
        .filter(|grant| {
            grant.capability == rspice_automation_protocol::CapabilityKind::EnvironmentRead
        })
        .collect::<Vec<_>>();
    assert_eq!(environment.len(), 1);
    assert_eq!(
        serde_json::from_str::<Vec<String>>(&environment[0].scope).unwrap(),
        manifest.environment
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn unscoped_network_and_process_permissions_fail_closed() {
    let mut app = RSpiceApp::test_instance();
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .unwrap();
    let bundle_id = bundle.id();
    let permissions_path = bundle
        .paths_for_role(ProjectSourceRole::AutomationPermissionManifest)
        .next()
        .unwrap()
        .to_owned();
    let permissions = bundle
        .file_content(&permissions_path)
        .unwrap()
        .replace("network = \"deny\"", "network = \"allow\"")
        .replace("process_spawn = \"deny\"", "process_spawn = \"allow\"");
    app.state
        .workspace
        .replace_project_source_bundle_file(bundle_id, &permissions_path, permissions)
        .unwrap();

    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(bundle_id)
        .unwrap();
    let diagnostics = compile_automation_bundle(bundle).unwrap_err();
    assert_eq!(diagnostics.len(), 2);
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.canonical.code.as_ref() == "RSPAUTO-PERM-NETWORK-SCOPE")
    );
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.canonical.code.as_ref() == "RSPAUTO-PERM-PROCESS-SCOPE")
    );

    let documents = [
        AutomationSourceDocument {
            path: "characterize.py",
            source: crate::state::DEFAULT_AUTOMATION_PYTHON,
        },
        AutomationSourceDocument {
            path: "runplan.rspice.yaml",
            source: crate::state::DEFAULT_AUTOMATION_RUN_PLAN,
        },
        AutomationSourceDocument {
            path: "requirements.lock",
            source: crate::state::DEFAULT_ENVIRONMENT_LOCK,
        },
        AutomationSourceDocument {
            path: "permissions.toml",
            source: crate::state::DEFAULT_AUTOMATION_PERMISSIONS,
        },
    ];
    let roles = [
        AutomationRoleBinding {
            path: "runplan.rspice.yaml",
            role: AutomationSourceRole::RunPlan,
        },
        AutomationRoleBinding {
            path: "requirements.lock",
            role: AutomationSourceRole::EnvironmentLock,
        },
        AutomationRoleBinding {
            path: "permissions.toml",
            role: AutomationSourceRole::PermissionManifest,
        },
    ];
    let (_, mut manifest) = compile_automation_documents("characterize.py", &documents, &roles)
        .expect("valid baseline workspace");
    manifest.network = "allow".to_owned();
    manifest.process_spawn = "allow".to_owned();
    let grants = automation_capabilities(&app, &manifest).expect("capability scope");
    assert!(grants.iter().all(|grant| {
        !matches!(
            grant.capability,
            rspice_automation_protocol::CapabilityKind::Network
                | rspice_automation_protocol::CapabilityKind::ProcessSpawn
        )
    }));
}

#[test]
fn ambient_external_write_and_clipboard_authorities_are_never_granted() {
    let app = RSpiceApp::test_instance();
    let documents = [
        AutomationSourceDocument {
            path: "characterize.py",
            source: crate::state::DEFAULT_AUTOMATION_PYTHON,
        },
        AutomationSourceDocument {
            path: "runplan.rspice.yaml",
            source: crate::state::DEFAULT_AUTOMATION_RUN_PLAN,
        },
        AutomationSourceDocument {
            path: "requirements.lock",
            source: crate::state::DEFAULT_ENVIRONMENT_LOCK,
        },
        AutomationSourceDocument {
            path: "permissions.toml",
            source: crate::state::DEFAULT_AUTOMATION_PERMISSIONS,
        },
    ];
    let roles = [
        AutomationRoleBinding {
            path: "runplan.rspice.yaml",
            role: AutomationSourceRole::RunPlan,
        },
        AutomationRoleBinding {
            path: "requirements.lock",
            role: AutomationSourceRole::EnvironmentLock,
        },
        AutomationRoleBinding {
            path: "permissions.toml",
            role: AutomationSourceRole::PermissionManifest,
        },
    ];
    let (_, manifest) = compile_automation_documents("characterize.py", &documents, &roles)
        .expect("valid governed workspace");
    let grants = automation_capabilities(&app, &manifest).expect("capability scope");

    assert!(grants.iter().all(|grant| {
        !matches!(
            grant.capability,
            rspice_automation_protocol::CapabilityKind::ProjectWrite
                | rspice_automation_protocol::CapabilityKind::ExternalFileRead
                | rspice_automation_protocol::CapabilityKind::ExternalFileWrite
                | rspice_automation_protocol::CapabilityKind::Network
                | rspice_automation_protocol::CapabilityKind::ProcessSpawn
                | rspice_automation_protocol::CapabilityKind::ClipboardRead
                | rspice_automation_protocol::CapabilityKind::ClipboardWrite
        )
    }));
}

#[test]
fn specification_evidence_covers_every_task_and_spec() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(result(
        AnalysisInstanceId::new(),
        Some(rspice_core::MeasureResult::success("gain", 1.0)),
    ));
    run.add_analysis(result(
        AnalysisInstanceId::new(),
        Some(rspice_core::MeasureResult::success("gain", 1.2)),
    ));

    let checks = build_spec_evidence(&[spec("gain")], &run).expect("spec evidence");
    assert_eq!(checks.len(), 2);
    assert!(
        checks
            .iter()
            .all(|check| check.outcome() == CheckOutcome::Passed)
    );
    assert_ne!(checks[0].id(), checks[1].id());
}

#[test]
fn missing_or_duplicate_task_measurements_are_explicit_failures() {
    let id = AnalysisInstanceId::new();
    let mut missing = SimulationRun::new(1);
    missing.add_analysis(result(id, None));
    let checks = build_spec_evidence(&[spec("gain")], &missing).expect("failure evidence");
    assert_eq!(checks[0].outcome(), CheckOutcome::Failed);
    assert!(checks[0].detail().contains("retained no measurement"));

    let mut duplicate = result(id, Some(rspice_core::MeasureResult::success("gain", 1.0)));
    duplicate
        .measurements
        .push(rspice_core::MeasureResult::success("GAIN", 1.0));
    let mut duplicate_run = SimulationRun::new(2);
    duplicate_run.add_analysis(duplicate);
    let checks = build_spec_evidence(&[spec("gain")], &duplicate_run).expect("failure evidence");
    assert_eq!(checks[0].outcome(), CheckOutcome::Failed);
    assert!(checks[0].detail().contains("ambiguous measurements"));
}

#[test]
fn dispatch_matching_requires_exact_project_and_plan_revisions() {
    let plan_id = SimulationPlanId::new();
    let project_revision = ObjectRevision::INITIAL;
    let plan_revision = ObjectRevision::INITIAL;
    let run = prepared_run(plan_id, project_revision, plan_revision);
    let baseline_digest = simulation_run_digest(&run).expect("baseline digest");
    let snapshot = AutomationDispatchSnapshot {
        project_id: ProjectId::new(),
        project_revision,
        plan_id,
        plan_revision,
        prepared_snapshot_digest: run
            .prepared_receipt()
            .expect("prepared run")
            .prepared_snapshot_digest(),
        source_content_digest: run
            .prepared_receipt()
            .expect("prepared run")
            .source_content_digest(),
        plan_name: "Plan".to_owned(),
        plan_payload: SimulationPlanPayload::default(),
        project_sources: ProjectSourceRegistry::default(),
        baseline_run: Arc::new(run.clone()),
        baseline_digest,
    };
    assert!(run_matches_dispatch(&run, &snapshot));

    let mut changed = snapshot.clone();
    changed.plan_revision = ObjectRevision::new(2).expect("revision two");
    assert!(!run_matches_dispatch(&run, &changed));

    let mut changed = snapshot.clone();
    changed.prepared_snapshot_digest = digest(0x91);
    assert!(!run_matches_dispatch(&run, &changed));

    let mut changed = snapshot;
    changed.source_content_digest = digest(0x92);
    assert!(!run_matches_dispatch(&run, &changed));
}

#[test]
fn baseline_digest_changes_when_retained_evidence_changes() {
    let plan_id = SimulationPlanId::new();
    let mut baseline = prepared_run(plan_id, ObjectRevision::INITIAL, ObjectRevision::INITIAL);
    let original = simulation_run_digest(&baseline).expect("baseline digest");
    baseline.analyses[0].waveforms[0].y = Arc::new(vec![1.0, 2.001]);
    let changed = simulation_run_digest(&baseline).expect("changed digest");
    assert_ne!(original, changed);
}
