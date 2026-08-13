//! Tests for the models surface's evidence rules.
//!
//! The central assertion is that evidence is never invented: a non-project
//! model receives no synthetic qualification evidence, and a configured
//! correlation requires current approved evidence before it counts.

use egui::Rect;

use super::*;
use crate::workbench::state::ModelsPage;

#[test]
fn model_tabs_match_the_mockup_taxonomy() {
    assert_eq!(
        ModelsPage::ALL.map(ModelsPage::label),
        [
            "Models",
            "Symbols & CDF",
            "Corners & sections",
            "Bins & geometry",
            "Include graph",
            "Qualification",
        ]
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn complete_models_surface_keeps_action_pages_inside_the_title_band() {
    for (page, label) in [
        (ModelsPage::Symbols, "Create symbol"),
        (ModelsPage::Include, "Export manifest"),
    ] {
        for width in [1_431.0, 820.0, 720.0, 561.0] {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut app = RSpiceApp::test_instance();
            app.state.workbench.models_page = page;
            app.state.library_manager.clear();
            app.state.model_library_manager.clear();

            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 560.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| show(ui, &mut app));
                },
            );

            let nodes = output
                .platform_output
                .accesskit_update
                .expect("models accessibility tree")
                .nodes;
            let bounds = nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                })
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing {label} action"));
            assert!(
                bounds.y1 <= 150.0,
                "{label} escaped the models title band on {page:?} at {width}: {bounds:?}"
            );
            assert!(
                bounds.y0 >= 70.0,
                "{label} overlapped the manager toolbar or page tabs on {page:?} at {width}: {bounds:?}"
            );
            assert!(
                !output.shapes.is_empty(),
                "{page:?} produced no visible body composition at {width}"
            );
        }
    }
}

#[test]
fn qualification_tab_uses_the_mockup_contract_label() {
    assert_eq!(ModelsPage::Qualification.label(), "Qualification");
    assert_eq!(
        Command::ModelsPage(ModelsPage::Qualification).stable_id(),
        "model-qualification"
    );
}

#[test]
fn qualification_evidence_set_digest_is_order_independent_and_suite_qualified() {
    let first = crate::product::ContentDigest::from_bytes([0x11; 32]);
    let second = crate::product::ContentDigest::from_bytes([0x22; 32]);
    let mut one = vec![("dc".to_owned(), 3, first)];
    let one_label = qualification_evidence_contract_digest(&mut one).expect("single digest label");
    assert!(one_label.starts_with("dc@3 · "));

    let mut forward = vec![
        ("transient".to_owned(), 4, second),
        ("dc".to_owned(), 3, first),
    ];
    let mut reverse = forward.iter().cloned().rev().collect::<Vec<_>>();
    let forward_label =
        qualification_evidence_contract_digest(&mut forward).expect("aggregate digest");
    let reverse_label =
        qualification_evidence_contract_digest(&mut reverse).expect("aggregate digest");
    assert_eq!(forward_label, reverse_label);
    assert!(forward_label.starts_with("2 suites · "));
}

#[test]
fn qualification_domain_projection_never_invents_oracle_provenance() {
    let mut quantities = BTreeSet::new();
    quantities.insert("v(out)".to_owned());
    let domains = qualification_domain_summaries(BTreeMap::from([(
        QualificationDomain::Ac,
        QualificationDomainAccumulator {
            vectors: 2,
            references: 2,
            quantities,
            tolerance_contracts: BTreeMap::from([(
                qualification_tolerance_key(1.0e-6, 0.005),
                qualification_tolerance_label(1.0e-6, 0.005),
            )]),
            evidenced_vectors: 1,
            passing_vectors: 1,
            open_dispositions: 0,
        },
    )]));

    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0].domain, QualificationDomain::Ac);
    assert_eq!(domains[0].reference_coverage, "2 refs · 1 quantity");
    assert_eq!(domains[0].disposition, "1 without evidence");
    assert!(
        !domains[0]
            .reference_coverage
            .to_ascii_lowercase()
            .contains("vendor")
    );
    assert!(
        !domains[0]
            .reference_coverage
            .to_ascii_lowercase()
            .contains("oracle")
    );
}

#[test]
fn qualification_domain_projection_preserves_distinct_tolerance_contracts() {
    let domains = qualification_domain_summaries(BTreeMap::from([(
        QualificationDomain::Dc,
        QualificationDomainAccumulator {
            vectors: 2,
            references: 2,
            quantities: BTreeSet::from(["v(out)".to_owned()]),
            tolerance_contracts: BTreeMap::from([
                (
                    qualification_tolerance_key(1.0001e-6, 0.0),
                    qualification_tolerance_label(1.0001e-6, 0.0),
                ),
                (
                    qualification_tolerance_key(1.0002e-6, 0.0),
                    qualification_tolerance_label(1.0002e-6, 0.0),
                ),
            ]),
            evidenced_vectors: 0,
            passing_vectors: 0,
            open_dispositions: 0,
        },
    )]));

    assert_eq!(
        qualification_tolerance_label(1.0001e-6, 0.0),
        qualification_tolerance_label(1.0002e-6, 0.0)
    );
    assert_eq!(domains[0].tolerance, "2 declared contracts · varies");
}

#[test]
fn project_model_without_suites_is_truthfully_unqualified() {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "nch_owned".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Project-owned qualification fixture".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), 0.48),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    };
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");

    let summary = qualification_summaries(&app)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("qualification summary");

    assert!(summary.source_error.is_none());
    assert_eq!(summary.gate, QualificationGate::Unqualified);
    assert_eq!(summary.suites, 0);
    assert_eq!(summary.vectors, 0);
    assert_eq!(summary.passing_vectors, 0);
    assert!(summary.evidence_digest.is_none());

    app.state
        .model_library_manager
        .select_library(&summary.library);
    app.state.workbench.selected_model = Some(summary.model.clone());
    assert_eq!(
        qualification_action_block_reason(&app, Some(&summary), QualificationPageAction::RunSuite)
            .as_deref(),
        Some("Author at least one executable qualification suite first")
    );
    assert_eq!(
        qualification_action_block_reason(
            &app,
            Some(&summary),
            QualificationPageAction::ReviewVectors
        ),
        None
    );
    execute_qualification_action(&mut app, QualificationPageAction::ReviewVectors);
    assert_eq!(
        app.state.workbench.model_editor.active_section,
        ModelEditorSection::Tests
    );
    assert!(app.state.workbench.model_editor.qualification_plan_open);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::ModelEditor
    );

    let editor = &mut app.state.workbench.model_editor;
    editor.begin_qualification_suite();
    let authoring = &mut editor.qualification_authoring;
    authoring.suite_id = "dc-op".to_owned();
    authoring.suite_name = "DC operating point".to_owned();
    authoring.vector_id = "nominal".to_owned();
    authoring.vector_name = "Nominal bias".to_owned();
    authoring.executable_input =
        "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.op\n.end\n".to_owned();
    authoring.quantity = "v(out)".to_owned();
    authoring.probe_target = "out".to_owned();
    authoring.expected = "1".to_owned();
    authoring.absolute_tolerance = "1e-9".to_owned();
    authoring.relative_tolerance = "1e-9".to_owned();
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    let working_summary = qualification_summaries(&app)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("working qualification summary");
    assert_eq!(working_summary.suites, 1);
    assert_eq!(working_summary.vectors, 1);
    assert!(
        working_summary
            .source_revision
            .ends_with("· working qualification")
    );
    assert_eq!(
        qualification_action_block_reason(
            &app,
            Some(&working_summary),
            QualificationPageAction::RunSuite
        ),
        None
    );
}

#[test]
fn non_project_model_never_receives_synthetic_qualification_evidence() {
    let app = RSpiceApp::test_instance();
    let library = ModelLibrary::new("built-in");
    let model = DeviceModel::new(
        "builtin_resistor",
        crate::state::model_library::ModelType::Resistor,
    );

    let closure = model_editor::verify_project_library_closure(&library, &library.name);
    let summary = qualification_model_summary(&app, &library, &model, closure.as_ref());

    assert!(summary.source_error.is_some());
    assert_eq!(summary.gate, QualificationGate::Blocked);
    assert_eq!(summary.vectors, 0);
    assert_eq!(summary.passing_vectors, 0);
    assert!(summary.evidence_digest.is_none());
}

#[test]
fn configured_correlation_requires_current_approved_evidence_for_qualification() {
    use crate::state::model_library::{
        CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSuite,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "nch_correlated".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Correlation handoff fixture".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), 0.48)]),
        string_parameters: BTreeMap::new(),
    };
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .unwrap();
    let resolved = model_editor::resolve_project_model_for_editor(
        &app.state.model_library_manager,
        "owned-models",
        "nch_correlated",
    )
    .unwrap();
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        "nch_correlated",
        resolved.source_id,
        resolved.model_digest,
        resolved.model_revision,
    )
    .unwrap();
    let dataset = CorrelationDatasetRevision::try_from_csv(
        "bench",
        crate::product::ObjectRevision::INITIAL,
        "Bench",
        CorrelationDatasetClass::BenchMeasurement,
        "lab",
        "lot-1",
        "fixture-1",
        "calibration-1",
        "bench.csv",
        b"id,quantity,value,unit\np1,V(out),1,V\n".to_vec(),
        None,
    )
    .unwrap();
    let suite = CorrelationSuite::try_new(
        "bench-correlation",
        crate::product::ObjectRevision::INITIAL,
        "Bench correlation",
        "model-owner",
        source.clone(),
        vec![dataset],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let correlation = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
    let library = app
        .state
        .model_library_manager
        .get_library("owned-models")
        .unwrap();
    let model = library.models.get("nch_correlated").unwrap();
    let closure = model_editor::verify_project_library_closure(library, &library.name);
    let mut summary = qualification_model_summary(&app, library, model, closure.as_ref());
    summary.gate = QualificationGate::Qualified;

    apply_correlation_qualification_contract(&mut summary, Some(&correlation), Some(&source));

    assert_eq!(summary.gate, QualificationGate::Review);
    assert_eq!(
        summary.correlation_status,
        "0/1 current suite approvals retained"
    );
    assert!(summary.correlation_evidence_digest.is_none());
}
