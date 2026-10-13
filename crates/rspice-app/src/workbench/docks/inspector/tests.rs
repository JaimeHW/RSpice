//! The inspector edits a binding, never a copy of one.
//!
//! The axis-range field's text and parser are pinned beside them, in
//! `documents/result_document/axis_range.rs`. Scroll memory is per subject and
//! bounded, so two schematic subjects cannot inherit each other's offset.

use super::*;

fn catalog_test_diode(params: &str) -> (RSpiceApp, u64) {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    let mut library = ModelLibrary::new("models");
    library.add_model(DeviceModel::new("junction", ModelType::Diode));
    app.state.model_library_manager.add_library(library);
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::origin());
    app.state.schematic.document_mut_for_test().components[0].params = params.to_owned();
    app.state.schematic.clear_undo_history();
    app.state.schematic.session.is_dirty = false;
    (app, id)
}

#[test]
fn catalog_binding_preserves_raw_parameters_and_refuses_malformed_text_atomically() {
    let original = r#"off note="a  b" expr={a + f(b,c)}"#;
    let (mut app, id) = catalog_test_diode(original);
    bind_component_model_from_catalog(&mut app, id, "models", "junction").unwrap();
    assert_eq!(
        app.state.schematic.document().components[0].params,
        format!("{original} model=junction model_library=models")
    );
    assert!(app.state.schematic.undo());
    assert_eq!(
        app.state.schematic.document().components[0].params,
        original
    );
    assert!(!app.state.schematic.can_undo());

    let (mut app, id) = catalog_test_diode("note='unterminated");
    let before = app.state.schematic.document().components[0].clone();
    assert!(
        bind_component_model_from_catalog(&mut app, id, "models", "junction")
            .unwrap_err()
            .contains("unterminated")
    );
    assert_eq!(app.state.schematic.document().components[0], before);
    assert!(!app.state.schematic.session.is_dirty);
    assert!(!app.state.schematic.can_undo());
}

#[test]
fn catalog_binding_resolves_the_pending_inspector_draft_before_model_publication() {
    use crate::workbench::state::{InlineEditField, InlineEditSession};
    let (mut app, id) = catalog_test_diode(r#"note="a  b""#);
    let expected = app.state.schematic.document().components[0].clone();
    let mut candidate = expected.clone();
    candidate.name = "D42".to_owned();
    let authority = app.state.inline_edit_authority();
    app.state.workbench.inline_edit.begin(InlineEditSession {
        expected: expected.clone(),
        field: InlineEditField::Instance,
        authority,
        description: "rename instance".to_owned(),
        buffer: "invalid name".to_owned(),
        candidate: None,
        error: Some("Invalid name".to_owned()),
        widget: None,
    });
    assert!(bind_component_model_from_catalog(&mut app, id, "models", "junction").is_err());
    assert_eq!(app.state.schematic.document().components[0], expected);
    app.state
        .workbench
        .inline_edit
        .set_draft("D42".to_owned(), Ok(candidate));
    bind_component_model_from_catalog(&mut app, id, "models", "junction").unwrap();
    assert_eq!(app.state.schematic.document().components[0].name, "D42");
    assert!(
        app.state.schematic.document().components[0]
            .params
            .starts_with(r#"note="a  b" "#)
    );
    assert!(app.state.workbench.inline_edit.session().is_none());
}

#[test]
fn catalog_binding_uses_the_resolved_project_global_provider() {
    use crate::state::model_library::{DeviceModel, ModelConsumerScope, ModelLibrary, ModelType};

    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    for library_name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(library_name);
        library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
        app.state.model_library_manager.add_library(library);
    }
    app.state
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "shared_diode",
            "beta",
            "Test selects the executable provider.",
        )
        .expect("provider decision");
    let component_id = 9_001;
    app.state.schematic.document_mut_for_test().components.push(
        Component::new(
            component_id,
            ComponentType::Diode,
            crate::state::Point::origin(),
        )
        .with_name_value("D9001", ""),
    );

    bind_component_model_from_catalog(&mut app, component_id, "beta", "shared_diode")
        .expect("resolved provider binds");
    let params = crate::state::parse_params_string(
        &app.state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .expect("component")
            .params,
    );
    assert_eq!(
        params.get("model").map(String::as_str),
        Some("shared_diode")
    );
    assert_eq!(
        params.get("model_library").map(String::as_str),
        Some("beta")
    );

    let error = bind_component_model_from_catalog(&mut app, component_id, "alpha", "shared_diode")
        .expect_err("losing provider cannot bind");
    assert!(error.contains("project-global provider 'beta'"));
}

#[test]
fn catalog_binding_rejects_an_incompatible_primitive_without_mutation() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    let mut library = ModelLibrary::new("models");
    library.add_model(DeviceModel::new("junction", ModelType::Diode));
    app.state.model_library_manager.add_library(library);
    let component_id = 9_002;
    app.state.schematic.document_mut_for_test().components.push(
        Component::new(
            component_id,
            ComponentType::Resistor,
            crate::state::Point::origin(),
        )
        .with_name_value("R9002", "1k"),
    );
    let topology_before = app.state.schematic.topology_version();
    let params_before = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .expect("test resistor")
        .params
        .clone();

    let error = bind_component_model_from_catalog(&mut app, component_id, "models", "junction")
        .expect_err("a diode model cannot bind to a resistor");

    assert!(error.contains("incompatible with the selected Resistor instance"));
    assert_eq!(app.state.schematic.topology_version(), topology_before);
    assert_eq!(
        app.state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .expect("test resistor")
            .params,
        params_before
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inspector_header_exposes_its_workspace_heading() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);

    let nodes = ctx
        .run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(312.0);
                header(ui, &mut app);
            });
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Heading
            && node.label() == Some("Inspector")
            && node.level() == Some(2)
    }));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn convergence_inspector_exposes_unknown_and_forced_acceptance_status_accessibly() {
    use crate::state::{AnalysisResult, AnalysisType, TransientConvergenceEvidence};
    // Each status is a row on the section's label/value grid — a fact with a
    // name — never a sentence added to the panel body on its own edge.
    for (label, expected) in [
        ("Solver evidence", "not retained"),
        ("LTE acceptances", "none forced"),
        ("LTE acceptances", "1 forced"),
    ] {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient");
        if expected != "not retained" {
            let mut quality = rspice_core::diagnostics::ConvergenceQuality::default();
            if expected == "1 forced" {
                quality.record_force_accept(1);
            }
            analysis.convergence = Some(std::sync::Arc::new(
                TransientConvergenceEvidence::capture(
                    quality,
                    &[0.0, 1.0],
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap(),
            ));
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(312.0);
                result_authority::result_convergence(ui, &analysis);
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        assert!(
            nodes
                .iter()
                .any(|(_, node)| { node.label() == Some(label) && node.value() == Some(expected) }),
            "Missing accessible quality row: {label} = {expected}"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dc_inspector_exposes_exact_secondary_coordinates_and_primary_traversal() {
    use crate::simulation::controller::dc_history_tests::{nested_config, retain, solve};
    let mut config = nested_config();
    config.start = 1.0;
    config.stop = 0.0;
    config.step = -0.5;
    let analysis = retain(solve(config));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(312.0);
            result_authority::result_dc_sweep(ui, &analysis);
        });
    });
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    for expected in ["V1", "V2", "Descending", "3", "2.9999999999999999e-7"] {
        assert!(
            nodes.iter().any(|(_, n)| n.value() == Some(expected)),
            "Missing DC evidence: {expected}"
        );
    }
}

fn result_app_with_current_out_map(split: bool) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.split_with_results = split;
    app.state.workbench.activate(Workspace::Results);
    let project_revision = app.state.workspace.content.project.revision();
    let analysis_id = crate::product::AnalysisInstanceId::new();
    let receipt = crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
        project_revision,
        prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([0x11; 32]),
        source_content_digest: crate::product::ContentDigest::from_bytes([0x22; 32]),
        source_check_receipt: crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            crate::product::ContentDigest::from_bytes([0x33; 32]),
        ),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            crate::state::PreparedRunTaskReceipt::new(
                analysis_id,
                project_revision,
                Vec::new(),
                1,
                crate::product::ContentDigest::from_bytes([0x44; 32]),
            )
            .expect("valid prepared task"),
        ],
    })
    .expect("valid prepared run receipt");
    app.state
        .simulation
        .start_prepared_run(receipt)
        .unwrap()
        .add_analysis(
            crate::state::AnalysisResult::new(
                1,
                crate::state::AnalysisType::Transient,
                "retained TRAN",
            )
            .with_provenance(
                crate::state::AnalysisResultProvenance::new_with_source_domain(
                    crate::state::AnalysisResultSourceDomain::SimulationPlan,
                    analysis_id,
                    project_revision,
                    crate::product::ContentDigest::from_bytes([0x44; 32]),
                    Vec::new(),
                )
                .expect("valid prepared analysis provenance"),
            )
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#ffbd2e",
            )]),
        );
    app.state.ui.results.session.selected_trace = Some(
        rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
            app.state.simulation.active_run().expect("retained run"),
            0,
            0,
        )
        .expect("selected retained trace"),
    );
    let a = crate::state::Point::new(0, 0);
    let b = crate::state::Point::new(40, 0);
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::new(91, vec![a, b]));
    app.state.simulation.source.cross_probe.update(
        app.state.workspace.content.active_view.clone(),
        std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
        std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
        std::collections::HashMap::new(),
        app.state.schematic.topology_version(),
    );
    app
}

#[test]
fn one_step_frames_every_inspector_section_body() {
    assert_eq!(INSPECTOR_SECTION_PADDING, 8.0);
}

#[test]
fn result_trace_statistics_ignore_non_finite_samples() {
    let statistics = finite_trace_statistics(&[f64::NAN, -2.0, 1.0, f64::INFINITY, 4.0])
        .expect("three finite samples");

    assert_eq!(statistics.minimum, -2.0);
    assert_eq!(statistics.maximum, 4.0);
    assert_eq!(statistics.mean, 1.0);
    assert!(finite_trace_statistics(&[f64::NAN, f64::INFINITY]).is_none());
}

#[test]
fn result_qualification_rows_never_infer_unretained_release_authority() {
    assert_eq!(
        result_authority::RESULT_QUALIFICATION_GAPS,
        [
            ("Qualification receipt", "not retained"),
            ("Requirements mapping", "not retained"),
            ("Release gates", "not assessed"),
            ("Sign-off eligibility", "not assessed"),
            ("Approval authority", "not retained"),
        ]
    );
}

#[test]
fn schematic_subjects_do_not_share_inspector_scroll_state() {
    let mut app = RSpiceApp::test_instance();
    let sheet = inspector_scroll_identity(&app);
    let component_id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, crate::state::Point::new(10, 10));
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    let component = inspector_scroll_identity(&app);

    assert_ne!(sheet, component);
    app.state.schematic.session.editor.selection.clear();
    assert_eq!(sheet, inspector_scroll_identity(&app));
}

#[test]
fn inspector_scroll_memory_restores_subject_offsets_with_one_stable_widget() {
    let mut memory = InspectorScrollMemory::default();

    assert_eq!(memory.begin_subject("sheet"), Some(0.0));
    memory.record("sheet".to_owned(), 320.0);
    assert_eq!(memory.begin_subject("component:R1"), Some(0.0));
    memory.record("component:R1".to_owned(), 48.0);
    assert_eq!(memory.begin_subject("sheet"), Some(320.0));
    assert_eq!(memory.begin_subject("sheet"), None);
}

#[test]
fn inspector_scroll_memory_is_bounded_and_clamps_invalid_offsets() {
    let mut memory = InspectorScrollMemory::default();
    for index in 0..=INSPECTOR_SCROLL_HISTORY_LIMIT {
        memory.record(format!("subject:{index}"), index as f32);
    }
    memory.record("negative".to_owned(), -50.0);

    assert_eq!(memory.offsets.len(), INSPECTOR_SCROLL_HISTORY_LIMIT);
    assert_eq!(memory.begin_subject("negative"), Some(0.0));
    assert!(
        memory
            .offsets
            .iter()
            .all(|(_, offset)| offset.is_finite() && *offset >= 0.0)
    );
}

#[test]
fn full_results_cross_probe_navigates_to_design_after_exact_resolution() {
    let mut app = result_app_with_current_out_map(false);

    assert_eq!(
        cross_probe_trace_to_design(&mut app, "V(out)"),
        Ok("OUT".to_owned())
    );

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(!app.state.workbench.split_with_results);
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&91)
    );
    assert!(
        app.state
            .schematic
            .session
            .editor
            .net_highlight
            .is_wire_highlighted(91)
    );
}

#[test]
fn split_cross_probe_keeps_the_canonical_result_document_beside_design() {
    let mut app = result_app_with_current_out_map(true);

    assert_eq!(
        cross_probe_trace_to_design(&mut app, "V(out)"),
        Ok("OUT".to_owned())
    );

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(app.state.workbench.results_split_visible(
        app.state.project_lifecycle.is_open(),
        app.state.simulation.retained.has_retained_result_dataset(),
    ));
}

#[test]
fn stale_cross_probe_map_fails_without_leaving_results() {
    let mut app = result_app_with_current_out_map(true);
    app.state.schematic.add_wire(vec![
        crate::state::Point::new(80, 0),
        crate::state::Point::new(120, 0),
    ]);

    let error = cross_probe_trace_to_design(&mut app, "V(out)")
        .expect_err("topology mismatch must fail closed");

    assert!(error.contains("changed since this result"));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
}

#[test]
fn historical_result_revision_cannot_cross_probe_current_geometry() {
    let mut app = result_app_with_current_out_map(false);
    let stale_revision = app
        .state
        .workspace
        .content
        .project
        .revision()
        .next()
        .expect("next project revision");
    let run = app
        .state
        .simulation
        .active_run()
        .expect("prepared retained run");
    let analysis_id = run.analyses[0]
        .provenance()
        .expect("prepared analysis provenance")
        .source_instance_id();
    let receipt = crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
        project_revision: stale_revision,
        prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([0x51; 32]),
        source_content_digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
        source_check_receipt: crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            crate::product::ContentDigest::from_bytes([0x53; 32]),
        ),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            crate::state::PreparedRunTaskReceipt::new(
                analysis_id,
                stale_revision,
                Vec::new(),
                1,
                crate::product::ContentDigest::from_bytes([0x54; 32]),
            )
            .expect("valid prepared task"),
        ],
    })
    .expect("valid stale receipt");
    let mut stale_run = crate::state::SimulationRun::new_prepared(1, receipt);
    stale_run.add_analysis(
        crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "historical TRAN",
        )
        .with_provenance(
            crate::state::AnalysisResultProvenance::new_with_source_domain(
                crate::state::AnalysisResultSourceDomain::SimulationPlan,
                analysis_id,
                stale_revision,
                crate::product::ContentDigest::from_bytes([0x54; 32]),
                Vec::new(),
            )
            .expect("valid historical analysis provenance"),
        )
        .with_waveforms(vec![crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "#ffbd2e",
        )]),
    );
    app.state.simulation.retained.runs[0] = stale_run;
    app.state.ui.results.session.selected_trace = Some(
        rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
            &app.state.simulation.retained.runs[0],
            0,
            0,
        )
        .expect("selected historical trace"),
    );

    let error = cross_probe_trace_to_design(&mut app, "V(out)")
        .expect_err("historical result must fail closed");

    assert!(error.contains("different project revision"));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
}

#[test]
fn cross_probe_names_the_descend_target_for_out_of_scope_traces() {
    let mut app = result_app_with_current_out_map(false);
    let view = app.state.workspace.content.active_view.clone();
    let view_type = app.state.workspace.content.active_view_type();
    app.state.workspace.open_as_root(view.clone(), view_type);

    let error = cross_probe_trace_to_design(&mut app, "V(x1.out)")
        .expect_err("a trace read inside X1 has no conductor on the root sheet");
    assert_eq!(error, "Descend to /x1 to cross-probe it.");
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );

    // One level down, the root-scoped trace is the one out of scope — even
    // though the open sheet happens to carry a conductor of that name.
    let parent = crate::state::CellViewRef::new(&view.library, "tb", &view.view);
    app.state.workspace.open_as_root(parent, view_type);
    app.state
        .workspace
        .descend_into("X1".to_owned(), view, view_type);

    let error = cross_probe_trace_to_design(&mut app, "V(out)")
        .expect_err("a root-scoped trace does not name a conductor inside X1");
    assert_eq!(error, "Ascend to / to cross-probe it.");
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
}

#[test]
fn failure_row_reserves_a_non_overlapping_margin_column_at_drawer_width() {
    let row_width = 228.0;
    let columns = failure_row_columns(row_width, 48.0);
    let target_right = FAILURE_ROW_PADDING_X + columns.target_width;
    let margin_left = row_width - FAILURE_ROW_PADDING_X - columns.margin_width;

    assert!(target_right + FAILURE_ROW_COLUMN_GAP <= margin_left + f32::EPSILON);
    assert_eq!(columns.margin_width, 48.0);
}

#[test]
fn failure_row_height_grows_from_wrapped_target_copy_at_drawer_width() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(228.0, 400.0))),
        ..egui::RawInput::default()
    };
    let failure = FailedSample {
        target: "an intentionally long production specification target that must wrap".to_owned(),
        unit: "V".to_owned(),
        sample_index: 16,
        value: 0.912_345,
        normalized_margin: -0.1834,
    };
    let mut row_height = 0.0;

    let _ = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show(ctx, |ui| {
                row_height = failure_row(ui, &failure).rect.height();
            });
    });

    assert!(row_height > 58.0, "wrapped row height was {row_height}");
}

#[test]
fn netlist_diagnostic_locations_are_one_based_and_exact() {
    let mut diagnostic = crate::workbench::documents::netlist_document::Diagnostic::current(
        "rspice.test",
        "TEST-ADVISORY",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
        "Maximum transient step is implicit",
    );
    diagnostic.source_line = Some(127);
    diagnostic.line = Some(127);
    diagnostic.column = Some(8);

    assert_eq!(diagnostic_location(&diagnostic), "line 128 · column 9");
}

#[test]
fn generated_provenance_never_claims_source_mapping_without_evidence() {
    let mut state = AppState::default();
    state.ui.netlist.generated_source = "generated\n.end\n".to_owned();
    let input = crate::product::ContentDigest::from_bytes([0x31; 32]);
    state.ui.netlist.generated_input_digest = Some(input);
    state.ui.netlist.current_generation_input_digest = Some(input);

    assert_eq!(generated_state(&state), "generated · current");
    assert!(!generated_state(&state).contains("source mapped"));
}

#[test]
fn owned_provenance_requires_exact_saved_and_validated_bytes() {
    let mut state = AppState::default();
    state.simulation.source.netlist_content = "owned\n.end\n".to_owned();
    let project_revision = state.workspace.content.project.revision().get();
    let digest = crate::state::content_digest(&state.simulation.source.netlist_content);
    state.ui.netlist.externally_saved_content_digest = Some(digest);
    state.ui.netlist.validation = Some(
        crate::workbench::documents::netlist_document::NetlistValidationReceipt {
            visible_content_digest: digest,
            executable_source_digest: digest,
            prepared_snapshot_digest: digest,
            project_revision,
            task_count: 1,
            advisory_count: 0,
        },
    );

    assert_eq!(
        owned_source_state(&state, digest),
        "externally synchronized · validated"
    );
    state.workspace.content.netlist_source_dirty = true;
    assert_eq!(
        owned_source_state(&state, digest),
        "externally synchronized · validated · project modified"
    );
    state.simulation.source.netlist_content.push_str("* edit\n");
    let edited = crate::state::content_digest(&state.simulation.source.netlist_content);
    assert_eq!(
        owned_source_state(&state, edited),
        "modified · validation required"
    );
}

#[test]
fn native_component_never_claims_a_resolved_catalog_model() {
    let state = AppState::default();
    let component = Component::new(1, ComponentType::Resistor, crate::state::Point::origin());

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.tone, ModelEvidenceTone::Neutral);
    assert_eq!(evidence.model, "Not applicable");
    assert!(!evidence.status.contains("resolved"));
}

#[test]
fn missing_explicit_model_is_reported_as_unresolved_review_evidence() {
    let state = AppState::default();
    let component = Component::new(2, ComponentType::NpnBjt, crate::state::Point::origin())
        .with_name_value("Q1", "vendor_npn");

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.tone, ModelEvidenceTone::Error);
    assert_eq!(evidence.model, "vendor_npn");
    assert!(evidence.status.contains("not loaded"));
}

#[test]
fn green_model_evidence_requires_an_exact_pinned_catalog_source() {
    let mut state = AppState::default();
    let path = std::path::PathBuf::from("models/vendor.lib");
    let mut library = crate::state::model_library::ModelLibrary::new("vendor");
    library.root_path = Some(path.clone());
    library
        .source_closure
        .push(crate::state::model_library::ModelSourcePin {
            path: path.clone(),
            digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
        });
    let mut model = crate::state::model_library::DeviceModel::new(
        "vendor_npn",
        crate::state::model_library::ModelType::Npn,
    );
    model.file_path = Some(path);
    library.add_model(model);
    state.model_library_manager.add_library(library);
    let component = Component::new(3, ComponentType::NpnBjt, crate::state::Point::origin())
        .with_name_value("Q1", "vendor_npn");

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.tone, ModelEvidenceTone::Ok);
    assert_eq!(evidence.model, "vendor_npn");
    assert!(evidence.status.contains("source pinned"));
    assert!(evidence.source.contains("vendor.lib"));
}

#[test]
fn bound_model_evidence_uses_the_instance_model_and_section_overrides() {
    let mut state = AppState::default();
    let mut library = crate::state::model_library::ModelLibrary::new("vendor_analog");
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_A",
        crate::state::model_library::ModelType::Other,
    ));
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_B",
        crate::state::model_library::ModelType::Other,
    ));
    state.model_library_manager.add_library(library);

    let mut binding = crate::state::LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
    binding.module_name = Some("OPA189_B".to_owned());
    binding.model_section = Some("high_accuracy".to_owned());
    let component = Component::new(
        9,
        ComponentType::CellInstance,
        crate::state::Point::origin(),
    )
    .with_library_cell(binding)
    .with_name_value("XU1", "OPA189");

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model, "OPA189_B");
    assert_eq!(evidence.section, "high_accuracy");
}

#[test]
fn wilson_interval_is_bounded_and_contains_the_observed_rate() {
    let (low, high) = wilson_interval_95(986, 1_000).expect("valid population");

    assert!((0.0..=1.0).contains(&low));
    assert!((0.0..=1.0).contains(&high));
    assert!(low < 0.986 && high > 0.986);
    assert!(wilson_interval_95(0, 0).is_none());
    assert!(wilson_interval_95(2, 1).is_none());
}

/// The inspector reads the workspace's answers, not the walks behind them.
///
/// Both were whole-dataset work on every frame the panel was open.
/// `ManifestViewModel::from_run` takes a SHA-256 over every retained sample
/// in the run to state the dataset digest, and `validate_retained_evidence`
/// walks every sample of every waveform to state the integrity row. The
/// Results workspace owns a memo for each — keyed on the dataset generation,
/// so neither can serve a stale answer — and the inspector is one more reader
/// of them rather than a second computer of the same facts.
#[test]
fn the_result_inspector_routes_through_the_workspace_memos() {
    let shipped = crate::source_guard::without_test_items(include_str!("../inspector.rs"));
    assert!(
        !shipped.contains("ManifestViewModel::from_run") && !shipped.contains("manifest_for_run("),
        "the inspector rebuilds the manifest projection, digest and all, every frame"
    );
    assert!(
        !shipped.contains("validate_retained_evidence()"),
        "the inspector walks the retained evidence itself instead of resolving the \
         one memo that owns the verdict"
    );
}

/// A finished 1 ms transient on the Waves sheet, the way James had it open:
/// `V(out)` swinging inside 4…6 V with its pane active, the trace selected,
/// and the solver's convergence evidence retained with no forced points.
fn waves_details_app() -> RSpiceApp {
    use crate::state::{AnalysisResult, AnalysisType, TransientConvergenceEvidence, WaveformData};
    let quality = rspice_core::diagnostics::ConvergenceQuality::default();
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.activate(Workspace::Results);
    let time: Vec<f64> = (0..=200).map(|step| f64::from(step) * 5.0e-6).collect();
    let volts: Vec<f64> = time
        .iter()
        .map(|t| 5.0 + 0.9 * (std::f64::consts::TAU * 2.0e3 * t).sin())
        .collect();
    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", time.clone(), volts, "#ffbd2e"),
        ]);
    analysis.convergence = Some(std::sync::Arc::new(
        TransientConvergenceEvidence::capture(quality, &time, &rspice_core::abort_signal::NoAbort)
            .expect("convergence evidence"),
    ));
    app.state.simulation.start_run().add_analysis(analysis);
    app.state.simulation.view.active_analysis_idx = Some(0);
    let run = app.state.simulation.active_run().expect("retained run");
    let analysis_key = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
        run.dataset_id,
        &run.analyses[0],
    );
    let selected = rspice_results_ui::selection::SelectedResultTrace::from_run_indices(run, 0, 0)
        .expect("selected retained trace");
    app.state.ui.results.session.selected_trace = Some(selected);
    app.state.ui.results.session.viewer = ResultViewer::Waves;
    app.state.ui.results.session.active_wave_pane = Some(
        crate::workbench::documents::result_document::WavePanePresentationKey {
            analysis: analysis_key,
            unit: "V".to_owned(),
        },
    );
    // Y held at the 4…6 V the field showed James as `4.0000000 … 6.000`.
    crate::workbench::documents::result_document::set_active_axis_range(
        &Tokens::default(),
        &mut app.state,
        PaneAxis::Y,
        Some((4.0, 6.0)),
    );
    app
}

/// The inspector's default width on a wide window and its floor in every
/// window class (`workbench::layout::inspector_width_bounds`).
const INSPECTOR_DEFAULT_WIDTH: f32 = 312.0;
const INSPECTOR_MIN_WIDTH: f32 = 278.0;

/// Run the whole inspector at `width` the way the raster harness does — three
/// passes, the last one kept — and return what it painted and published.
fn run_inspector(
    app: &mut RSpiceApp,
    width: f32,
    height: f32,
) -> (
    Vec<egui::epaint::ClippedShape>,
    Vec<(egui::accesskit::NodeId, egui::accesskit::Node)>,
) {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = || egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(width, height))),
        ..Default::default()
    };
    let mut last = None;
    for _ in 0..3 {
        last = Some(ctx.run_ui(input(), |ui| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| show(ui, app));
        }));
    }
    let output = last.expect("three passes ran");
    let nodes = output
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default();
    (output.shapes, nodes)
}

/// One run of text the inspector painted: where its layout box landed, the
/// rectangle it was clipped to, and whether anything of it is visible.
#[derive(Debug)]
struct PaintedText {
    text: String,
    rect: Rect,
    clip: Rect,
}

fn painted_text(shapes: &[egui::epaint::ClippedShape]) -> Vec<PaintedText> {
    fn walk(shape: &egui::Shape, clip: Rect, out: &mut Vec<PaintedText>) {
        match shape {
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, out);
                }
            }
            egui::Shape::Text(text) => {
                let color = text.override_text_color.unwrap_or_else(|| {
                    text.galley
                        .job
                        .sections
                        .iter()
                        .map(|section| section.format.color)
                        .find(|color| *color != Color32::PLACEHOLDER)
                        .unwrap_or(text.fallback_color)
                });
                let rect = Rect::from_min_size(text.pos, text.galley.size());
                let visible = color.a() > 0
                    && text.opacity_factor > 0.0
                    && !text.galley.text().trim().is_empty()
                    && rect.intersects(clip);
                if visible {
                    out.push(PaintedText {
                        text: text.galley.text().to_owned(),
                        rect,
                        clip,
                    });
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for shape in shapes {
        walk(&shape.shape, shape.clip_rect, &mut out);
    }
    out
}

/// Every line the Waves details panel paints starts on the rows' inset and
/// ends inside it, and none of it is cut off by the box it was clipped to —
/// at the inspector's default width and at its minimum.
///
/// What James saw was two lines at x = 0 beside rows at x = 10, and a Y range
/// field whose padded text ran out of the field. The measured numbers this
/// prints are the report; the assertions keep them.
#[test]
fn the_waves_details_panel_keeps_every_line_inside_the_row_inset() {
    // Where a property row starts its label, read from the rows themselves.
    let (inset, _) =
        super::super::super::design_system::property_row_control_columns(INSPECTOR_MIN_WIDTH);
    for width in [INSPECTOR_DEFAULT_WIDTH, INSPECTOR_MIN_WIDTH] {
        let mut app = waves_details_app();
        let (shapes, _) = run_inspector(&mut app, width, 2400.0);
        let lines = painted_text(&shapes);
        assert!(
            !lines.is_empty(),
            "the inspector painted no text at {width}"
        );
        let left = lines
            .iter()
            .map(|line| line.rect.left())
            .fold(f32::INFINITY, f32::min);
        let right = lines
            .iter()
            .map(|line| line.rect.right())
            .fold(f32::NEG_INFINITY, f32::max);
        eprintln!(
            "width {width}: {} text runs; leftmost starts at {left:.1}, rightmost ends at \
             {right:.1} (right inset {:.1})",
            lines.len(),
            width - right
        );
        let mut off_inset = Vec::new();
        let mut clipped = Vec::new();
        for line in &lines {
            if line.rect.left() < inset - 0.01 || line.rect.right() > width - inset + 0.01 {
                off_inset.push(format!(
                    "{:?} spans {:.1}..{:.1}",
                    line.text,
                    line.rect.left(),
                    line.rect.right()
                ));
            }
            if line.rect.left() < line.clip.left() - 0.01
                || line.rect.right() > line.clip.right() + 0.01
            {
                clipped.push(format!(
                    "{:?} spans {:.1}..{:.1}, clipped to {:.1}..{:.1}",
                    line.text,
                    line.rect.left(),
                    line.rect.right(),
                    line.clip.left(),
                    line.clip.right()
                ));
            }
        }
        assert!(
            off_inset.is_empty(),
            "at {width} px these lines leave the {inset} px row inset:\n{}",
            off_inset.join("\n")
        );
        assert!(
            clipped.is_empty(),
            "at {width} px these lines are cut off:\n{}",
            clipped.join("\n")
        );
    }
}

/// What James read off the panel, pinned on the panel: each range field shows
/// the shortest form of its axis interval with the unit, the convergence
/// status is a row, and how the measurements were computed is their section
/// header's metadata.
#[test]
fn the_waves_details_panel_states_ranges_status_and_method_as_rows_and_meta() {
    let mut app = waves_details_app();
    let (_, nodes) = run_inspector(&mut app, INSPECTOR_DEFAULT_WIDTH, 2400.0);
    let value_of = |label: &str| {
        nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label) && node.value().is_some())
            .and_then(|(_, node)| node.value())
            .unwrap_or_else(|| panic!("no {label:?} row or field"))
            .to_owned()
    };
    assert_eq!(value_of("X range"), "0 \u{2026} 1 ms");
    assert_eq!(value_of("Y range"), "4 \u{2026} 6 V");
    assert_eq!(value_of("LTE acceptances"), "none forced");
    assert!(
        nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Heading
                && node.label() == Some("MEASUREMENTS, linear interpolation")
        }),
        "the measurements header does not carry its method as metadata"
    );
}

/// The Waves details panel as PNGs for a person to look at, at the
/// inspector's default and minimum widths. `RSPICE_RASTER_DIR` names the
/// directory; the system temp directory otherwise.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_waves_details_panel() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let height = 1400.0;
    for width in [INSPECTOR_DEFAULT_WIDTH, INSPECTOR_MIN_WIDTH] {
        let mut app = waves_details_app();
        let canvas = crate::ui::raster::render(Vec2::new(width, height), |ui, _| {
            // The dock's own fill, so the render reads the way the panel does.
            let panel = Tokens::get(ui.ctx()).color.bg_panel;
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(panel))
                .show(ui, |ui| show(ui, &mut app));
        });
        std::fs::write(
            directory.join(format!("waves-details-{width}.png")),
            canvas.png(height as usize),
        )
        .expect("write inspector render");
    }
}

/// Calls that put text straight into whatever `Ui` they are given. In an
/// inspector body that is the panel itself, and the panel has no inset: the
/// inset belongs to the rows, so a bare label starts on the panel edge.
const BARE_TEXT_CALLS: [&str; 10] = [
    "ui.label(",
    "Label::new(",
    "ui.add(egui::Label",
    "ui.monospace(",
    "ui.small(",
    "ui.weak(",
    "ui.strong(",
    "ui.colored_label(",
    "ui.collapsing(",
    "CollapsingHeader::new(",
];

/// Where a bare text call is right, how many lines of them, and why.
///
/// The count is part of the entry: a new bare label in an allowlisted
/// function is the defect this guard exists for, not one more cell.
const BARE_TEXT_ALLOWED: [(&str, &str, usize, &str); 9] = [
    (
        "inspector/design.rs",
        "drawing_sheet_title_block_substitution",
        2,
        "inside the warning callout, whose frame margin is the row inset",
    ),
    (
        "inspector/design.rs",
        "drawing_sheet_overflow_advisory",
        2,
        "inside the warning callout, whose frame margin is the row inset",
    ),
    (
        "inspector/design/inline_instance.rs",
        "rejection_slot",
        1,
        "put into the slot shrunk by the row padding on both sides, truncated with \
         the whole reason on hover",
    ),
    (
        "inspector/stimulus.rs",
        "caption",
        1,
        "one line naming the card well under it, already on the rows' inset",
    ),
    (
        "results-ui/sensitivity.rs",
        "right_panel",
        8,
        "header and cells of the ranked table, which scrolls inside inspector_inset",
    ),
    (
        "results-ui/sensitivity/study.rs",
        "right_panel",
        10,
        "header and cells of the ranked table, which scrolls inside inspector_inset",
    ),
    (
        "results-ui/sensitivity/mismatch.rs",
        "right_panel",
        9,
        "header and cells of the ranked table, which scrolls inside inspector_inset",
    ),
    (
        "results-ui/noise_contrib/figure.rs",
        "show",
        2,
        "the folded list of exact samples, which sits inside inspector_inset",
    ),
    (
        "results-ui/phase_noise.rs",
        "right_panel",
        1,
        "the fold of device noise shares, which sits inside inspector_inset",
    ),
];

/// Every inspector panel, as source: the app's own inspector modules whole,
/// and the result viewers' inspector halves by the functions the inspector
/// reaches. A viewer file that also draws a central sheet is scoped, because
/// labels inside a sheet's own layout are not on the inspector's edge.
fn inspector_panel_sources() -> Vec<(&'static str, &'static str, Option<&'static [&'static str]>)> {
    macro_rules! results_ui {
        ($path:literal) => {
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../rspice-results-ui/src/",
                $path
            ))
        };
    }
    macro_rules! adapter {
        ($path:literal) => {
            (
                concat!("result_document/", $path),
                include_str!(concat!("../../documents/result_document/", $path)),
                Some(RIGHT_PANEL),
            )
        };
    }
    const RIGHT_PANEL: &[&str] = &["right_panel"];
    fn only(names: &'static [&'static str]) -> Option<&'static [&'static str]> {
        Some(names)
    }
    let mut sources = vec![
        adapter!("bode.rs"),
        adapter!("box_violin.rs"),
        adapter!("events.rs"),
        adapter!("eye.rs"),
        adapter!("fft.rs"),
        adapter!("harmonic_balance.rs"),
        adapter!("hist.rs"),
        adapter!("noise_contrib.rs"),
        adapter!("nyquist.rs"),
        adapter!("op_inspector.rs"),
        adapter!("optimization.rs"),
        adapter!("phase_noise.rs"),
        adapter!("polar.rs"),
        adapter!("pz.rs"),
        adapter!("scatter.rs"),
        adapter!("sensitivity.rs"),
        adapter!("smith.rs"),
        adapter!("soa.rs"),
        adapter!("specs.rs"),
        adapter!("table.rs"),
        adapter!("transfer_function.rs"),
    ];
    sources.extend([
        ("inspector.rs", include_str!("../inspector.rs"), None),
        ("inspector/design.rs", include_str!("design.rs"), None),
        (
            "inspector/design/component.rs",
            include_str!("design/component.rs"),
            None,
        ),
        (
            "inspector/design/inline_instance.rs",
            include_str!("design/inline_instance.rs"),
            None,
        ),
        (
            "inspector/executed_deck.rs",
            include_str!("executed_deck.rs"),
            None,
        ),
        ("inspector/models.rs", include_str!("models.rs"), None),
        (
            "inspector/result_authority.rs",
            include_str!("result_authority.rs"),
            None,
        ),
        (
            "inspector/simulation.rs",
            include_str!("simulation.rs"),
            None,
        ),
        ("inspector/stimulus.rs", include_str!("stimulus.rs"), None),
        ("inspector/symbol.rs", include_str!("symbol.rs"), None),
        ("inspector/veriloga.rs", include_str!("veriloga.rs"), None),
        (
            "result_document.rs",
            include_str!("../../documents/result_document.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "result_document/waves/readout.rs",
            include_str!("../../documents/result_document/waves/readout.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/waves/dock.rs",
            results_ui!("waves/dock.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/waves/readout.rs",
            results_ui!("waves/readout.rs"),
            only(&["measurement_panel", "measurement_rows"]),
        ),
        (
            "results-ui/presentation.rs",
            results_ui!("presentation.rs"),
            only(&["stat_table", "panel_note"]),
        ),
        ("results-ui/qpnoise.rs", results_ui!("qpnoise.rs"), None),
        (
            "results-ui/bode/inspector.rs",
            results_ui!("bode/inspector.rs"),
            None,
        ),
        (
            "results-ui/box_violin.rs",
            results_ui!("box_violin.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/events.rs",
            results_ui!("events.rs"),
            only(&["right_panel", "selection_absent"]),
        ),
        (
            "results-ui/eye_diagram/view.rs",
            results_ui!("eye_diagram/view.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/fft/view.rs",
            results_ui!("fft/view.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/harmonic_balance.rs",
            results_ui!("harmonic_balance.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/histogram/view.rs",
            results_ui!("histogram/view.rs"),
            only(&["right_panel", "range_controls", "bound_row"]),
        ),
        (
            "results-ui/network_matrix.rs",
            results_ui!("network_matrix.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/noise_contrib.rs",
            results_ui!("noise_contrib.rs"),
            only(&["right_panel", "contributor_table"]),
        ),
        (
            "results-ui/noise_contrib/figure.rs",
            results_ui!("noise_contrib/figure.rs"),
            only(&["show"]),
        ),
        (
            "results-ui/nyquist/view.rs",
            results_ui!("nyquist/view.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/op_inspector.rs",
            results_ui!("op_inspector.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/optimization.rs",
            results_ui!("optimization.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/phase_noise.rs",
            results_ui!("phase_noise.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/polar.rs",
            results_ui!("polar.rs"),
            only(&["right_panel", "register_table"]),
        ),
        (
            "results-ui/pole_zero.rs",
            results_ui!("pole_zero.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/scatter.rs",
            results_ui!("scatter.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/sensitivity.rs",
            results_ui!("sensitivity.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/sensitivity/mismatch.rs",
            results_ui!("sensitivity/mismatch.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/sensitivity/study.rs",
            results_ui!("sensitivity/study.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/smith_chart/view.rs",
            results_ui!("smith_chart/view.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/soa.rs",
            results_ui!("soa.rs"),
            only(&["right_panel", "selection_note"]),
        ),
        (
            "results-ui/specs.rs",
            results_ui!("specs.rs"),
            Some(RIGHT_PANEL),
        ),
        (
            "results-ui/table.rs",
            results_ui!("table.rs"),
            only(&["operating_point_panel", "sample_panel"]),
        ),
        (
            "results-ui/transfer_function.rs",
            results_ui!("transfer_function.rs"),
            Some(RIGHT_PANEL),
        ),
    ]);
    sources
}

/// The function each line of `source` sits in, by the last `fn` opened above
/// it. Closures are not functions here, which is what is wanted: a label in a
/// closure is drawn by the function that wrote the closure.
fn enclosing_functions(source: &str) -> Vec<Option<String>> {
    let mut current: Option<String> = None;
    source
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            let declaration = ["pub(crate) fn ", "pub(super) fn ", "pub fn ", "fn "]
                .iter()
                .find_map(|prefix| trimmed.strip_prefix(prefix));
            if let Some(rest) = declaration {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    current = Some(name);
                }
            }
            current.clone()
        })
        .collect()
}

/// No inspector panel adds text straight to its body.
///
/// James found two lines on the Waves details panel sitting on the panel's
/// left edge — "Linear interpolation" and "No forced LTE acceptances
/// recorded" — while every row around them was inset. A row paints its own
/// inset; a bare label added to the panel body gets none. Every fact is a row,
/// every qualifier is its section header's metadata, every sentence goes
/// through the inspector's note, and a table or a fold sits inside
/// `inspector_inset`. What is left is listed above with its reason and count.
#[test]
fn no_inspector_panel_adds_text_to_its_body_without_an_inset() {
    let mut found: std::collections::BTreeMap<(&str, String), Vec<usize>> = Default::default();
    for (path, source, scope) in inspector_panel_sources() {
        let shipped = crate::source_guard::without_test_items(source);
        let functions = enclosing_functions(&shipped);
        if let Some(scope) = scope {
            for name in scope {
                assert!(
                    functions.iter().flatten().any(|function| function == name),
                    "{path} no longer declares `{name}`; the guard would scan nothing there"
                );
            }
        }
        for (index, line) in shipped.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") || !BARE_TEXT_CALLS.iter().any(|call| line.contains(call))
            {
                continue;
            }
            let function = functions[index].clone().unwrap_or_default();
            if scope.is_some_and(|scope| !scope.contains(&function.as_str())) {
                continue;
            }
            found.entry((path, function)).or_default().push(index + 1);
        }
    }
    let mut unexplained = Vec::new();
    for ((path, function), lines) in &found {
        let allowed = BARE_TEXT_ALLOWED
            .iter()
            .find(|(allowed_path, allowed_function, _, _)| {
                allowed_path == path && allowed_function == function
            })
            .map_or(0, |(_, _, count, _)| *count);
        if lines.len() != allowed {
            unexplained.push(format!(
                "{path} `{function}`: {} bare text line(s) at {lines:?}, {allowed} allowed",
                lines.len()
            ));
        }
    }
    for (path, function, count, _) in BARE_TEXT_ALLOWED {
        if count > 0 && !found.contains_key(&(path, function.to_owned())) {
            unexplained.push(format!(
                "{path} `{function}` is allowed {count} bare text line(s) and has none; \
                 drop the entry"
            ));
        }
    }
    assert!(
        unexplained.is_empty(),
        "text added straight to an inspector body starts on the panel edge; make it a \
         row, its section header's metadata, the inspector note, or inset it:\n{}",
        unexplained.join("\n")
    );
}
