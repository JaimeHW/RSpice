//! Tests for the models surface's evidence rules.
//!
//! The central assertion is that evidence is never invented: a non-project
//! model receives no synthetic qualification evidence, and a configured
//! correlation requires current approved evidence before it counts.

use super::*;

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

#[test]
fn responsive_model_geometry_matches_mockup_breakpoints() {
    assert_eq!(model_tab_strip_height(false, false), 38.0);
    assert_eq!(model_tab_strip_height(false, true), 38.0);
    assert_eq!(model_tab_strip_height(true, false), 44.0);
    assert_eq!(model_tab_strip_height(true, true), 54.0);

    assert_eq!(model_catalog_min_width(561.0), 780.0);
    assert_eq!(model_catalog_min_width(560.0), 690.0);
    assert!(!model_title_actions_stack(820.0));
    assert!(!model_title_actions_stack(561.0));
    assert!(model_title_actions_stack(560.0));
}

#[test]
fn project_catalog_exposes_imported_subcircuit_interfaces_without_fabricating_models() {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    app.state
        .model_library_manager
        .load_library_bytes(
            "browser-subcircuits.lib",
            b".subckt AMP inp inn out params: GAIN=100\n\
              e1 out 0 inp inn {GAIN}\n\
              .ends AMP\n"
                .to_vec(),
            None,
        )
        .expect("subcircuit source imports");

    let records = project_catalog_records(&app);
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.definition.name(), "AMP");
    assert_eq!(record.status, ProjectCatalogStatus::Ready);
    assert!(record.qualification.is_none());
    let ProjectCatalogDefinition::Subcircuit(interface) = &record.definition else {
        panic!("pure subcircuit import must not fabricate a device model");
    };
    assert_eq!(interface.ports, ["inp", "inn", "out"]);
    assert_eq!(
        interface.parameter_defaults.get("GAIN").map(String::as_str),
        Some("100")
    );
    assert!(project_record_matches_query(record, "inn"));
    assert!(project_record_matches_query(record, "gain"));
    assert_eq!(app.state.model_library_manager.total_definition_count(), 1);
}

#[test]
fn symbol_and_corner_compositions_own_overflow_without_changing_desktop_geometry() {
    let desktop = model_table_summary_layout(egui::vec2(1_120.0, 620.0), false);
    assert!(!desktop.narrow);
    assert_eq!(desktop.table_height, 470.0);
    assert_eq!(desktop.summary_height, MODEL_WIDE_SUMMARY_H);
    assert!(!desktop.owns_vertical_scroll);

    let short_desktop = model_table_summary_layout(egui::vec2(1_120.0, 240.0), false);
    assert!(!short_desktop.narrow);
    assert_eq!(short_desktop.table_height, MODEL_TABLE_MIN_H);
    assert!(short_desktop.owns_vertical_scroll);

    let narrow = model_table_summary_layout(egui::vec2(560.0, 500.0), false);
    assert!(narrow.narrow);
    assert_eq!(narrow.table_height, 200.0);
    assert_eq!(narrow.summary_height, MODEL_STACKED_SUMMARY_H);
    assert!(!narrow.owns_vertical_scroll);

    let short_narrow = model_table_summary_layout(egui::vec2(560.0, 380.0), false);
    assert!(short_narrow.narrow);
    assert_eq!(short_narrow.table_height, MODEL_TABLE_MIN_H);
    assert!(short_narrow.owns_vertical_scroll);

    let touch = model_table_summary_layout(egui::vec2(1_120.0, 380.0), true);
    assert!(touch.narrow);
    assert!(touch.owns_vertical_scroll);
}

#[test]
fn action_title_keeps_its_button_in_the_title_band_and_leaves_body_space() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut action_rect = None;
    let mut body_rect = None;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_431.0, 560.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                surface_title(
                    ui,
                    "SYMBOLS, PINS & DEVICE FORMS",
                    "Symbol and component-definition manager",
                    "Bind graphical symbols and explicit terminal contracts.",
                    true,
                    |ui| action_rect = Some(ui.button("Create symbol").rect),
                );
                body_rect = Some(ui.label("BODY CONTENT").rect);
            });
        },
    );
    let action_rect = action_rect.expect("title action rendered");
    let body_rect = body_rect.expect("body rendered");

    assert!(
        action_rect.top() < 90.0,
        "action was pushed below title: {action_rect:?}"
    );
    assert!(
        body_rect.top() < 130.0,
        "title consumed the surface: {body_rect:?}"
    );
    assert!(body_rect.top() >= action_rect.bottom() - 1.0);
    assert!(!output.shapes.is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn complete_models_surface_keeps_action_pages_inside_the_title_band() {
    for (page, label, description, body_label) in [
        (
            ModelsPage::Symbols,
            "Create symbol",
            "Bind graphical symbols, terminals, parameter forms and model families without hiding netlist semantics.",
            "No symbol views are present in the loaded design libraries.",
        ),
        (
            ModelsPage::Bins,
            "Import bin map...",
            "Audit the parsed card ranges that drive model selection and trace placed device geometry to the winning card.",
            "No executable model source contains geometry-binned model families.",
        ),
        (
            ModelsPage::Include,
            "Collapse transitive",
            "Inspect ordered dependency resolution, captured paths, source pins, and cycle diagnostics.",
            "No loaded model libraries expose an include graph.",
        ),
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
            let description_bounds = nodes
                .iter()
                .find(|(_, node)| node.label() == Some(description))
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing {description} title description"));
            let body_bounds = nodes
                .iter()
                .find(|(_, node)| node.label() == Some(body_label))
                .and_then(|(_, node)| node.bounds())
                .unwrap_or_else(|| panic!("missing {body_label} body state"));
            assert!(
                body_bounds.y0 >= bounds.y1 - 1.0
                    && body_bounds.y0 >= description_bounds.y1 - 1.0
                    && body_bounds.y1 <= 560.0,
                "{body_label} overlaps the title or leaves the visible body on {page:?} at {width}: action={bounds:?}, description={description_bounds:?}, body={body_bounds:?}"
            );
            assert!(
                (body_bounds.y0 + body_bounds.y1) * 0.5 >= 220.0,
                "{body_label} is stranded at the top of an otherwise empty table on {page:?} at {width}: {body_bounds:?}"
            );
        }
    }
}

#[test]
fn summary_cards_reserve_exact_height_when_long_values_wrap() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut consumed = None;
    let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(825.0, 420.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let long_value = "C:/commercial/pdk/releases/current/models/process/sections/temperature-and-voltage-corner".to_owned();
                    let left = [
                        ("Resolved bindings", long_value.clone()),
                        ("Unresolved bindings", long_value.clone()),
                        ("Missing non-TT section", long_value.clone()),
                    ];
                    let right = [
                        ("Temperature", long_value.clone()),
                        ("Supply factor", long_value.clone()),
                        ("PDK search paths", long_value),
                    ];
                    let response = summary_cards(
                        ui,
                        false,
                        MODEL_WIDE_SUMMARY_H,
                        false,
                        SummaryCardSpec::new("Binding policy", &left),
                        SummaryCardSpec::new("Environment axes", &right),
                    );
                    consumed = Some(response.rect.height());
                });
            },
        );

    assert!((consumed.expect("summary rendered") - MODEL_WIDE_SUMMARY_H).abs() <= 0.5);
}

#[test]
fn parent_scrolled_summary_uses_natural_height_instead_of_nesting_scrollbars() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut consumed = None;
    // Five fixed-height property rows plus the card header exceed the nominal
    // summary height. Read-only property rows deliberately elide instead of
    // wrapping, so row count is the stable way to exercise natural expansion.
    let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(430.0, 260.0),
                )),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show(root, |ui| {
                    let long = "C:/commercial/pdk/releases/current/models/process/sections/temperature-and-voltage-corner".to_owned();
                    let left = [
                        ("Resolved bindings", long.clone()),
                        ("Unresolved bindings", long.clone()),
                        ("Missing non-TT section", long.clone()),
                        ("Unpinned sources", long.clone()),
                        ("Duplicate definitions", long.clone()),
                    ];
                    let right = [
                        ("Temperature", long.clone()),
                        ("Supply factor", long.clone()),
                        ("PDK search paths", long.clone()),
                        ("Model libraries", long.clone()),
                        ("Active sections", long),
                    ];
                    consumed = Some(
                        summary_cards(
                            ui,
                            false,
                            MODEL_WIDE_SUMMARY_H,
                            true,
                            SummaryCardSpec::new("Binding policy", &left),
                            SummaryCardSpec::new("Environment axes", &right),
                        )
                        .rect
                        .height(),
                    );
                });
            },
        );

    assert!(
        consumed.expect("summary rendered") > MODEL_WIDE_SUMMARY_H,
        "natural content must expand inside the single parent scroll owner"
    );
}

#[test]
fn qualification_tab_uses_the_mockup_contract_label() {
    assert_eq!(ModelsPage::Qualification.label(), "Qualification");
    assert_eq!(
        Command::ModelsPage(ModelsPage::Qualification).stable_id(),
        "model-qualification"
    );
    assert!(QUALIFICATION_MIN_CONTENT_H > 600.0);
    assert_eq!(
        qualification_required_content_height(MODEL_SUMMARY_BREAKPOINT),
        QUALIFICATION_STACKED_MIN_CONTENT_H
    );
    assert_eq!(
        qualification_required_content_height(MODEL_SUMMARY_BREAKPOINT + 1.0),
        QUALIFICATION_MIN_CONTENT_H
    );
}

#[test]
fn include_diagnostics_expose_every_contested_provider() {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    app.state
        .model_library_manager
        .load_library_bytes(
            "alpha.lib",
            b".model duplicated NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
            None,
        )
        .expect("alpha provider");
    app.state
        .model_library_manager
        .load_library_bytes(
            "beta.lib",
            b".model DUPLICATED NMOS (LEVEL=1 KP=2e-3)\n".to_vec(),
            None,
        )
        .expect("beta provider");

    let diagnostics = include_diagnostics(&app);
    assert_eq!(diagnostics.duplicate_definitions, 1);
    assert_eq!(diagnostics.conflicts.len(), 1);
    assert_eq!(diagnostics.conflicts[0].normalized_name, "duplicated");
    assert_eq!(diagnostics.conflicts[0].providers.len(), 2);
    assert!(
        diagnostics
            .resolution_error
            .as_deref()
            .is_some_and(|error| error.contains("Select one exact provider"))
    );

    app.state
        .model_library_manager
        .resolve_definition_conflict("duplicated", "beta", "DUPLICATED")
        .expect("exact provider selection");
    let resolved = include_diagnostics(&app);
    assert!(resolved.resolution_error.is_none());
}

#[test]
fn bins_tab_has_a_stable_command_identity() {
    assert_eq!(ModelsPage::Bins.label(), "Bins & geometry");
    assert_eq!(
        Command::ModelsPage(ModelsPage::Bins).stable_id(),
        "model-bins"
    );
    assert_eq!(
        Command::from_stable_id("model-bins"),
        Some(Command::ModelsPage(ModelsPage::Bins))
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
    assert_eq!(domains[0].tone, QualificationGate::Unqualified);
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
fn qualification_footer_reserves_wrapped_copy_before_stacking_its_action() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut heights = None;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 300.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                heights = Some((
                    qualification_gate_footer_height(ui, 460.0),
                    qualification_gate_footer_height(ui, 900.0),
                ));
            });
        },
    );
    let (stacked, wide) = heights.expect("footer heights measured");
    assert!(stacked > wide);
    assert_eq!(wide, 58.0);
    assert!(stacked >= 96.0);
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

    let summary = qualification_model_summary(&app, &library, &model);

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
    let mut summary = qualification_model_summary(&app, library, model);
    summary.gate = QualificationGate::Qualified;

    apply_correlation_qualification_contract(&mut summary, Some(&correlation), Some(&source));

    assert_eq!(summary.gate, QualificationGate::Review);
    assert_eq!(
        summary.correlation_status,
        "0/1 current suite approvals retained"
    );
    assert!(summary.correlation_evidence_digest.is_none());
}

#[test]
fn table_column_contracts_are_normalized() {
    let sum: f32 = [0.15, 0.17, 0.17, 0.14, 0.18, 0.10, 0.09].into_iter().sum();
    assert!((sum - 1.0).abs() < f32::EPSILON);
}

#[test]
fn cycle_diagnostics_report_only_nodes_remaining_after_topological_sort() {
    let a = PathBuf::from("a");
    let b = PathBuf::from("b");
    let c = PathBuf::from("c");
    assert_eq!(cyclic_node_count(&[(a.clone(), b.clone()), (b, c)]), 0);
    assert_eq!(cyclic_node_count(&[(a.clone(), a)]), 1);
}

#[test]
fn include_selection_and_definition_rows_project_exact_retained_source_state() {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    let library_name = app
        .state
        .model_library_manager
        .load_library_bytes(
            "include-fixture.lib",
            b".lib TT\n.model nch NMOS (LEVEL=54)\n.subckt AMP in out\nR1 in out 1k\n.ends AMP\n.endl TT\n"
                .to_vec(),
            Some("TT"),
        )
        .expect("fixture imports");
    let root = app
        .state
        .model_library_manager
        .get_library(&library_name)
        .and_then(|library| library.root_path.clone())
        .expect("authenticated root");
    app.state.workbench.model_include_selected_library = Some(library_name.clone());
    app.state.workbench.model_include_selected_source = Some(root.clone());

    let detail = selected_include_source_detail(&app).expect("selected detail");
    assert_eq!(detail.library, library_name);
    assert_eq!(detail.path, root);
    assert_eq!(detail.digest.len(), 64);
    assert!(detail.root);
    assert!(detail.byte_length > 0);
    assert_eq!(detail.incoming_edges, 0);
    assert_eq!(detail.outgoing_edges, 0);
    assert_eq!(detail.definitions, 2);
    assert_eq!(detail.authority, "retained import · digest checked");

    let rows = include_definition_rows(&app);
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row.resolved && !row.exception));
    assert!(
        rows.iter()
            .any(|row| { row.name == "AMP" && row.kind == "subcircuit" && row.section == "TT" })
    );
    assert!(
        rows.iter()
            .any(|row| { row.name == "nch" && row.kind == "model" && row.section == "TT" })
    );
}
