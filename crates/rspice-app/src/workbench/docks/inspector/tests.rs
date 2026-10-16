//! The inspector edits a binding, never a copy of one.
//!
//! The axis-range field's text and parser are pinned beside them, in
//! `documents/result_document/axis_range.rs`. Scroll memory is per subject and
//! bounded, so two schematic subjects cannot inherit each other's offset.

use super::*;
use crate::state::ComponentType;

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
fn convergence_inspector_speaks_only_of_forced_acceptances() {
    use crate::state::{AnalysisResult, AnalysisType, TransientConvergenceEvidence};
    // A forced acceptance is a row on the section's label/value grid; no
    // evidence, or evidence that forced nothing, has no section at all.
    for (evidence, forced, expected) in [
        (false, false, None),
        (true, false, None),
        (true, true, Some("1 forced")),
    ] {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient");
        if evidence {
            let mut quality = rspice_core::diagnostics::ConvergenceQuality::default();
            if forced {
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
        let row = nodes
            .iter()
            .find(|(_, node)| node.label() == Some("LTE acceptances"))
            .and_then(|(_, node)| node.value());
        assert_eq!(row, expected, "evidence {evidence}, forced {forced}");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dc_inspector_names_its_sources_and_primary_traversal() {
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
    for expected in ["V1", "V2", "Descending", "3"] {
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
fn result_trace_statistics_ignore_non_finite_samples() {
    let statistics = finite_trace_statistics(&[f64::NAN, -2.0, 1.0, f64::INFINITY, 4.0])
        .expect("three finite samples");

    assert_eq!(statistics.minimum, -2.0);
    assert_eq!(statistics.maximum, 4.0);
    assert_eq!(statistics.mean, 1.0);
    assert!(finite_trace_statistics(&[f64::NAN, f64::INFINITY]).is_none());
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

/// The engine scopes a cell drawn `amp` by the name the deck writes it
/// under, so its trace is `V(xamp.out)`. On that cell's sheet the trace is the
/// conductor the sheet draws, not a trace read elsewhere.
#[test]
fn cross_probe_finds_a_trace_scoped_by_the_written_instance_name() {
    let mut app = result_app_with_current_out_map(false);
    let view = app.state.workspace.content.active_view.clone();
    let view_type = app.state.workspace.content.active_view_type();
    let parent = crate::state::CellViewRef::new(&view.library, "tb", &view.view);
    app.state.workspace.open_as_root(parent, view_type);
    app.state
        .workspace
        .descend_into("amp".to_owned(), view, view_type);

    let net = cross_probe_trace_to_design(&mut app, "V(xamp.out)")
        .expect("the written scope is this occurrence");

    assert!(net.eq_ignore_ascii_case("out"), "{net}");
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
}

#[test]
fn failure_row_reserves_a_non_overlapping_margin_column_at_drawer_width() {
    let row_width = 228.0;
    let columns = failure_row_columns(row_width, 48.0, PANEL_INSET);
    let target_right = PANEL_INSET + columns.target_width;
    let margin_left = row_width - PANEL_INSET - columns.margin_width;

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

/// A deck that matches the schematic says nothing at all: the three states
/// left are the ones a designer has to act on.
#[test]
fn generated_provenance_speaks_only_when_the_deck_is_not_the_schematic() {
    let mut state = AppState::default();
    state.ui.netlist.generated_source = "generated\n.end\n".to_owned();
    let input = crate::product::ContentDigest::from_bytes([0x31; 32]);
    state.ui.netlist.generated_input_digest = Some(input);
    state.ui.netlist.current_generation_input_digest = Some(input);
    assert_eq!(generated_state(&state), None);

    state.ui.netlist.current_generation_input_digest =
        Some(crate::product::ContentDigest::from_bytes([0x32; 32]));
    assert_eq!(
        generated_state(&state),
        Some(("Older than the schematic", Problem::Warning))
    );

    state.ui.netlist.generated_source.clear();
    assert_eq!(
        generated_state(&state),
        Some(("Not generated yet", Problem::Warning))
    );
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

    // Saved and validated is the ordinary state, and so is a dirty project —
    // the document tab's marker owns that one.
    assert_eq!(owned_source_state(&state, digest), None);
    state.workspace.content.netlist_source_dirty = true;
    assert_eq!(owned_source_state(&state, digest), None);

    // Edited since the validation receipt was taken: the receipt no longer
    // describes these bytes, and the panel says so.
    state.simulation.source.netlist_content.push_str("* edit\n");
    let edited = crate::state::content_digest(&state.simulation.source.netlist_content);
    assert_eq!(owned_source_state(&state, edited), Some("Not checked yet"));
}

#[test]
fn native_component_never_claims_a_resolved_catalog_model() {
    let state = AppState::default();
    let component = Component::new(1, ComponentType::Resistor, crate::state::Point::origin());

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model, None, "a resistor names no model");
    assert_eq!(evidence.problem, None);
}

#[test]
fn missing_explicit_model_is_reported_as_unresolved_review_evidence() {
    let state = AppState::default();
    let component = Component::new(2, ComponentType::NpnBjt, crate::state::Point::origin())
        .with_name_value("Q1", "vendor_npn");

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model.as_deref(), Some("vendor_npn"));
    let (problem, text) = evidence.problem.expect("a missing model is a problem");
    assert_eq!(problem, Problem::Failure);
    assert_eq!(text, "Model vendor_npn not found in the loaded libraries");
}

#[test]
fn a_model_resolves_only_from_a_file_its_library_records() {
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

    assert_eq!(evidence.model.as_deref(), Some("vendor_npn"));
    assert_eq!(evidence.problem, None, "a recorded file says nothing more");

    let library = state
        .model_library_manager
        .get_library_mut("vendor")
        .expect("the fixture library");
    library.source_closure.clear();
    let evidence = component_model_evidence(&state, &component);
    let (problem, text) = evidence
        .problem
        .expect("a file the library does not record is a problem");
    assert_eq!(problem, Problem::Failure);
    assert!(text.contains("vendor.lib"), "{text}");
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

    assert_eq!(evidence.model.as_deref(), Some("OPA189_B"));
    assert_eq!(evidence.problem, None);
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

/// The Design and Verilog-A inspectors state problems with a mark and say
/// nothing about success: no colored status text, no status dots, no success
/// or neutral marks.
///
/// The editor strip is scanned with them. `veriloga_compile_status` used to be
/// carved out of this scan because the strip painted its five tones and an
/// always-on dot; it now returns neutral text and an `Option<Problem>`, so the
/// carve-out is gone and the surface that paints it is in the list.
#[test]
fn the_design_and_veriloga_inspectors_paint_no_status_color() {
    // The token path, not one binding's spelling of it. These were
    // `t.color.ok` and `t.color.info`, which let `tokens.color.info` through
    // in `inspector.rs` — the file this scan is named after and did not read.
    const BANNED: &[&str] = &[
        "colored_label",
        "status_dot(",
        "StatusMark::Success",
        "StatusMark::Neutral",
        ".color.ok",
        ".color.info",
    ];
    let files = [
        (
            "inspector.rs",
            crate::source_guard::without_test_items(include_str!("../inspector.rs")),
        ),
        (
            "design.rs",
            crate::source_guard::without_test_items(include_str!("design.rs")),
        ),
        (
            "design/component.rs",
            crate::source_guard::without_test_items(include_str!("design/component.rs")),
        ),
        (
            "design/inline_instance.rs",
            crate::source_guard::without_test_items(include_str!("design/inline_instance.rs")),
        ),
        (
            "veriloga_inspector.rs",
            crate::source_guard::without_test_items(include_str!(
                "../../documents/code_workspace/veriloga_inspector.rs"
            )),
        ),
        (
            "surfaces/veriloga.rs",
            crate::source_guard::without_test_items(include_str!("../../surfaces/veriloga.rs")),
        ),
    ];
    for (file, source) in files {
        for (number, line) in source.lines().enumerate() {
            for pattern in BANNED {
                assert!(
                    !line.contains(pattern),
                    "{file}:{} paints status with `{pattern}`: {}",
                    number + 1,
                    line.trim()
                );
            }
        }
    }
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
    // Where the panel body starts every row's label.
    let inset = super::super::super::design_system::PANEL_INSET;
    for width in [INSPECTOR_DEFAULT_WIDTH, INSPECTOR_MIN_WIDTH] {
        let mut app = waves_details_app();
        let (shapes, _) = run_inspector(&mut app, width, 2400.0);
        let lines = painted_text(&shapes);
        assert!(
            !lines.is_empty(),
            "the inspector painted no text at {width}"
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

/// The Results inspector names the run and leaves its provenance to Run
/// details…, which holds the identifiers and the executed deck the panel no
/// longer shows.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn run_provenance_lives_in_run_details_not_on_the_panel() {
    let mut app = waves_details_app();
    let (_, nodes) = run_inspector(&mut app, INSPECTOR_DEFAULT_WIDTH, 2400.0);
    let labelled = |nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], label: &str| {
        nodes.iter().any(|(_, node)| node.label() == Some(label))
    };
    assert!(
        labelled(&nodes, "Run details\u{2026}"),
        "no route to the run's details"
    );
    for gone in [
        "Dataset ID",
        "Run sequence",
        "Execution target",
        "Samples",
        "Visibility",
    ] {
        assert!(!labelled(&nodes, gone), "{gone} is still on the panel");
    }

    let run_id = app.state.simulation.active_run().expect("retained run").id;
    run_details::open(&mut app.state, run_id);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            run_details::show(ui.ctx(), &mut app);
        });
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    for present in ["Dataset ID", "Dataset digest", "Run ID"] {
        assert!(labelled(&nodes, present), "Run details lacks {present}");
    }

    run_details::open(&mut app.state, u64::MAX);
    assert_eq!(
        app.state
            .dialogs
            .run_details
            .as_ref()
            .map(|details| details.run_id),
        Some(run_id),
        "a run no longer retained opens nothing"
    );
}

/// What James read off the panel, pinned on the panel: each range field shows
/// the shortest form of its axis interval with the unit, a run that forced no
/// acceptance says nothing of convergence, and how the measurements were
/// computed is their section header's metadata.
#[test]
fn the_waves_details_panel_states_ranges_and_method_as_rows_and_meta() {
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
    assert!(
        !nodes
            .iter()
            .any(|(_, node)| node.label() == Some("LTE acceptances")),
        "a run that forced nothing has no convergence row"
    );
    // Linear interpolation between retained samples is how every
    // measurement in the product is taken, so the header does not say it.
    assert!(
        nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Heading && node.label() == Some("Measurements")
        }),
        "the measurements header is gone"
    );
}

/// A diode bound to its catalog model, selected on the Design sheet.
fn design_component_app() -> RSpiceApp {
    let (mut app, id) = catalog_test_diode("model=junction");
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    app
}

/// A Verilog-A cell view open in Design whose module failed to compile, so
/// the inspector's diagnostics section has findings to list.
fn veriloga_inspector_app() -> RSpiceApp {
    use crate::state::{
        Cell, CellViewRef, Library, ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner,
        View, ViewType,
    };
    use crate::workbench::documents::code_workspace;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let reference = CellViewRef::new("behavioral", "gain_cell", "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain_cell".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut cell = Cell::new("gain_cell");
    cell.add_view(view);
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    app.state.library_manager.add_library(library);
    let source = "// Behavioral conductance.\nmodule gain_cell(p, n);\n  inout p, n;\n  \
                  electrical p, n;\n  parameter real g = 1m;\n  analog I(p, n) <+ gain * V(p, n) \
                  + offset;\nendmodule\n";
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        "models/gain_cell.va",
        source.to_owned(),
        [],
        [],
    )
    .expect("a valid cell-view bundle");
    app.state
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle.clone())
        .expect("one bundle per cell view");
    app.state.open_workspace_view(reference);
    app.state.workbench.activate(Workspace::Design);
    let project = app.state.workspace.content.project.id();
    let Err(diagnostics) =
        code_workspace::compile_project_bundle_receipt(project, &bundle, Some("gain_cell"))
    else {
        panic!("an undeclared identifier refuses the compile");
    };
    let veriloga = &mut app.state.ui.code_workspace.veriloga;
    veriloga.last_failure = std::sync::Arc::new(
        code_workspace::CodeDiagnosticCollection::try_new(diagnostics).expect("few findings"),
    );
    veriloga.last_failure_token = Some(
        rspice_simulation::project_veriloga::VerilogASourceOperationToken::capture(
            project,
            &bundle,
            Some("gain_cell"),
        ),
    );
    app
}

/// The shipped CMOS inverter example open on the Design sheet, nothing
/// selected.
fn cmos_inverter_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    crate::workbench::examples::load_example("CMOS Inverter", &mut app.state.schematic);
    app.state.schematic.clear_undo_history();
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Design);
    app
}

/// The ID of the inverter's component named `name`.
fn inverter_component(app: &RSpiceApp, name: &str) -> u64 {
    app.state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == name)
        .unwrap_or_else(|| panic!("the inverter has no {name}"))
        .id
}

/// The inverter with its output net selected, the way a click on the
/// output wire leaves it.
fn design_net_app() -> RSpiceApp {
    let mut app = cmos_inverter_app();
    let out = rspice_design::connectivity::summary::design_nets(&app.state.schematic)
        .into_iter()
        .find(|net| net.name.eq_ignore_ascii_case("out"))
        .expect("the inverter's output is labelled out");
    for wire in out.wire_ids {
        app.state
            .schematic
            .session
            .editor
            .selection
            .select_wire(wire);
    }
    app
}

/// The inverter with both transistors and the output stub selected.
fn design_multi_app() -> RSpiceApp {
    let mut app = cmos_inverter_app();
    let (mp, mn) = (
        inverter_component(&app, "MP"),
        inverter_component(&app, "MN"),
    );
    let selection = &mut app.state.schematic.session.editor.selection;
    selection.select_component(mp);
    selection.select_component(mn);
    let stub = app
        .state
        .schematic
        .document()
        .wires
        .last()
        .map(|wire| wire.id)
        .expect("the inverter is wired");
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_wire(stub);
    app
}

/// The inverter's pull-down transistor selected, beside a current run that
/// solved the inverter's operating point with its input held at mid-rail —
/// the device report and node voltages are the engine's own.
fn design_mos_op_app() -> RSpiceApp {
    // The shipped example's own transistors, cards and rail: the panel
    // states an operating point the design in front of the reader would
    // actually solve, so the cards are the foundation library's own text.
    let card = |name: &str| {
        rspice_core::library::foundation_card_source(name)
            .unwrap_or_else(|| panic!("{name} is a foundation card"))
    };
    let deck = format!(
        "\
* CMOS inverter at its switching point
vdd vdd 0 dc 3.3
vin in 0 dc 1.65
MP out in vdd vdd RSPICE_PMOS W=2u L=0.18u
MN out in 0 0 RSPICE_NMOS W=1u L=0.18u
{}
{}
.op
.end
",
        card("RSPICE_NMOS"),
        card("RSPICE_PMOS")
    );
    let deck = deck.as_str();
    let netlist = rspice_core::netlist::Netlist::parse(deck).expect("the op deck parses");
    let (solution, report) =
        rspice_core::engine::Engine::new(rspice_core::engine::SimulationConfig::default())
            .run_dc_op_with_report(&netlist)
            .expect("the inverter's operating point solves");
    let node_voltages = solution
        .node_names
        .iter()
        .zip(&solution.node_voltages)
        .skip(1)
        .map(
            |(name, value)| rspice_results::operating_point::OperatingPointValue {
                name: format!("V({name})"),
                value: *value,
                unit: "V".to_owned(),
            },
        )
        .collect();

    let mut app = cmos_inverter_app();
    let mn = inverter_component(&app, "MN");
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
        .expect("the run starts")
        .add_analysis(
            crate::state::AnalysisResult::new(
                1,
                crate::state::AnalysisType::DcOp,
                "Operating point",
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
            .with_dc_op(rspice_results::operating_point::DcOpResult {
                node_voltages,
                branch_currents: Vec::new(),
                power_dissipation: Vec::new(),
            })
            .with_device_op(report),
        );
    let generated = crate::simulation::netlist_gen::generate_netlist(&app.state.schematic);
    let point_to_net = generated
        .nets
        .iter()
        .flat_map(|(net, points)| points.iter().map(move |point| (*point, net.clone())))
        .collect();
    app.state.simulation.source.cross_probe.update(
        app.state.workspace.content.active_view.clone(),
        point_to_net,
        generated.nets,
        std::collections::HashMap::new(),
        app.state.schematic.topology_version(),
    );
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(mn);
    app
}

/// A compact panel row — an operating point, a node voltage, a trace's
/// statistics — holds four significant digits whatever precision the reader
/// set for the result tables, and still drops the zeros that only pad a
/// value out.
#[test]
fn a_compact_panel_readout_holds_four_significant_digits() {
    assert_eq!(compact_readout(96.86112e-6, "A"), "96.86 µA");
    assert_eq!(compact_readout(275.30774e-3, "V"), "275.3 mV");
    assert_eq!(compact_readout(310.10824e-6, "S"), "310.1 µS");
    assert_eq!(compact_readout(4.1, "V"), "4.1 V");
    assert_eq!(compact_readout(0.9, "V"), "900 mV");
}

/// One review panel: its file name and the app state it renders.
type ReviewPanel = (&'static str, fn() -> RSpiceApp);

/// The Stimulus Library inspector with two adopters, one of which the library
/// has published past.
///
/// Both states the Adopters list has to draw at once: an instance whose copy
/// is current, which states no drift and carries two verbs, and one the
/// library has moved past, which says so and offers re-adoption.
fn stimulus_adopters_app() -> RSpiceApp {
    use crate::state::stimulus_library::definition::StimulusDefinition;
    use crate::state::stimulus_library::fixtures;
    use crate::state::{Component, Point};

    const DEFINITION: &str = "bridge_cal_step";

    let mut app = RSpiceApp::test_instance();
    app.state.workspace.content.stimulus_library = fixtures::library();
    app.state.workbench.workspace = crate::workbench::state::Workspace::Stimulus;
    app.state.workbench.selected_stimulus_definition = Some(DEFINITION.to_owned());
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let adopt = |app: &mut RSpiceApp, id: u64, instance: &str| {
        let definition: StimulusDefinition = app
            .state
            .workspace
            .content
            .stimulus_library
            .get(DEFINITION)
            .cloned()
            .expect("fixture definition");
        let mut component = Component::new(id, definition.component_type(), Point::new(4, 4));
        component.name = instance.to_owned();
        definition.adopt_onto(&mut component).expect("adopt");
        app.state
            .schematic
            .document_mut_for_test()
            .components
            .push(component);
    };
    adopt(&mut app, 7, "V1");
    // Publishing between the two adoptions is what leaves the first instance
    // behind the library and the second in step with it, which is the
    // comparison this section exists to make.
    crate::workbench::app::actions::stimulus::edit_field(&mut app.state, "pw", "250u");
    crate::workbench::app::actions::stimulus::apply_draft(&mut app.state);
    adopt(&mut app, 8, "V3");
    app
}

/// The inspector's panels as PNGs, with their measured geometry beside each,
/// at the dock's default and minimum widths. `RSPICE_RASTER_DIR` names the
/// directory; the system temp directory otherwise.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_inspector_panels_for_review() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    let panels: [ReviewPanel; 8] = [
        ("design-component", design_component_app),
        ("design-net", design_net_app),
        ("design-sheet", cmos_inverter_app),
        ("design-multi", design_multi_app),
        ("design-mos-op", design_mos_op_app),
        ("veriloga-inspector", veriloga_inspector_app),
        ("waves-details", waves_details_app),
        ("stimulus-adopters", stimulus_adopters_app),
    ];
    for (name, fixture) in panels {
        for width in [INSPECTOR_DEFAULT_WIDTH, INSPECTOR_MIN_WIDTH] {
            let mut app = fixture();
            crate::ui::raster::write_panel_review(
                &directory,
                &format!("{name}-{width}"),
                crate::ui::Theme::default(),
                Vec2::new(width, 1800.0),
                10.0,
                |ui, _| {
                    // The dock's own fill, so the render reads the way the
                    // panel does.
                    let panel = Tokens::get(ui.ctx()).color.bg_panel;
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(panel))
                        .show(ui, |ui| show(ui, &mut app));
                },
            );
        }
    }
    // The data browser is the navigator's, beside the Results inspector: the
    // same finished transient, at the navigator's default and minimum widths.
    for width in [256.0, 220.0] {
        let mut app = waves_details_app();
        crate::ui::raster::write_panel_review(
            &directory,
            &format!("data-browser-{width}"),
            crate::ui::Theme::default(),
            Vec2::new(width, 900.0),
            10.0,
            |ui, _| {
                let panel = Tokens::get(ui.ctx()).color.bg_panel;
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(panel))
                    .show(ui, |ui| super::super::navigator::show(ui, &mut app));
            },
        );
    }
}
