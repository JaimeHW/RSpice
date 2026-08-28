//! The inspector edits a binding, never a copy of one.
//!
//! A typed interval must survive the field that shows it, in the separators an
//! engineer would actually paste, and a reversed or degenerate one is refused
//! rather than quietly sorted. Scroll memory is per subject and bounded, so
//! two schematic subjects cannot inherit each other's offset.

use super::*;

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
    app.state.schematic.components.push(
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
    app.state.schematic.components.push(
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
            .components
            .iter()
            .find(|component| component.id == component_id)
            .expect("test resistor")
            .params,
        params_before
    );
}

#[test]
fn a_typed_interval_round_trips_through_the_field_that_shows_it() {
    for range in [
        (0.0, 5.0),
        (-1.5e-3, 2.25e-3),
        (1.0e6, 1.0e9),
        (-1.0e-15, 1.0e-14),
    ] {
        let text = format_axis_range(range);
        let parsed = parse_axis_range(&text)
            .unwrap_or_else(|| panic!("{text} did not read back as an interval"));
        assert!(
            (parsed.0 - range.0).abs() <= range.0.abs() * 1.0e-6
                && (parsed.1 - range.1).abs() <= range.1.abs() * 1.0e-6,
            "{range:?} rendered as {text} and read back as {parsed:?}"
        );
    }
}

#[test]
fn an_interval_parses_from_the_separators_an_engineer_would_paste() {
    for text in ["1m … 5m", "1m..5m", "1m,5m", "1m 5m", "  1m   5m  "] {
        assert_eq!(
            parse_axis_range(text),
            Some((1.0e-3, 5.0e-3)),
            "failed on {text:?}"
        );
    }
    assert_eq!(parse_axis_range("-2.5 -1.5"), Some((-2.5, -1.5)));
}

#[test]
fn a_reversed_or_degenerate_interval_is_refused_rather_than_sorted() {
    // On a log axis "5m … 1m" is not the same request as "1m … 5m", and
    // quietly swapping them would hide the typo behind a correct-looking
    // plot.
    assert_eq!(parse_axis_range("5m … 1m"), None);
    assert_eq!(parse_axis_range("1m … 1m"), None);
    assert_eq!(parse_axis_range("1m"), None);
    assert_eq!(parse_axis_range("1m … 2m … 3m"), None);
    assert_eq!(parse_axis_range(""), None);
    assert_eq!(parse_axis_range("wide open"), None);
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

fn result_app_with_current_out_map(split: bool) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.split_with_results = split;
    app.state.workbench.activate(Workspace::Results);
    let project_revision = app.state.workspace.project.revision();
    let analysis_id = crate::product::AnalysisInstanceId::new();
    let receipt = crate::state::PreparedRunReceipt::new(
        crate::state::AnalysisResultSourceDomain::SimulationPlan,
        Some(crate::product::SimulationPlanId::new()),
        project_revision,
        crate::product::ContentDigest::from_bytes([0x11; 32]),
        crate::product::ContentDigest::from_bytes([0x22; 32]),
        crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            crate::product::ContentDigest::from_bytes([0x33; 32]),
        ),
        vec![
            crate::state::PreparedRunTaskReceipt::new(
                analysis_id,
                project_revision,
                Vec::new(),
                1,
                crate::product::ContentDigest::from_bytes([0x44; 32]),
            )
            .expect("valid prepared task"),
        ],
    )
    .expect("valid prepared run receipt");
    app.state
        .simulation
        .start_prepared_run(receipt)
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
    app.state.ui.results.selected_trace = Some(
        crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
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
        .wires
        .push(crate::state::Wire::new(91, vec![a, b]));
    app.state.simulation.cross_probe.update(
        app.state.workspace.active_view.clone(),
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
        .selection
        .select_only_component(component_id);
    let component = inspector_scroll_identity(&app);

    assert_ne!(sheet, component);
    app.state.schematic.selection.clear();
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
    assert!(app.state.schematic.selection.wires.contains(&91));
    assert!(app.state.schematic.net_highlight.is_wire_highlighted(91));
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
        app.state.project_lifecycle.project_open,
        app.state.simulation.has_retained_result_dataset(),
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
    assert!(app.state.schematic.selection.wires.is_empty());
}

#[test]
fn historical_result_revision_cannot_cross_probe_current_geometry() {
    let mut app = result_app_with_current_out_map(false);
    let stale_revision = app
        .state
        .workspace
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
    let receipt = crate::state::PreparedRunReceipt::new(
        crate::state::AnalysisResultSourceDomain::SimulationPlan,
        Some(crate::product::SimulationPlanId::new()),
        stale_revision,
        crate::product::ContentDigest::from_bytes([0x51; 32]),
        crate::product::ContentDigest::from_bytes([0x52; 32]),
        crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            crate::product::ContentDigest::from_bytes([0x53; 32]),
        ),
        vec![
            crate::state::PreparedRunTaskReceipt::new(
                analysis_id,
                stale_revision,
                Vec::new(),
                1,
                crate::product::ContentDigest::from_bytes([0x54; 32]),
            )
            .expect("valid prepared task"),
        ],
    )
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
    app.state.simulation.runs[0] = stale_run;
    app.state.ui.results.selected_trace = Some(
        crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
            &app.state.simulation.runs[0],
            0,
            0,
        )
        .expect("selected historical trace"),
    );

    let error = cross_probe_trace_to_design(&mut app, "V(out)")
        .expect_err("historical result must fail closed");

    assert!(error.contains("different project revision"));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(app.state.schematic.selection.wires.is_empty());
}

#[test]
fn cross_probe_names_the_descend_target_for_out_of_scope_traces() {
    let mut app = result_app_with_current_out_map(false);
    let view = app.state.workspace.active_view.clone();
    let view_type = app.state.workspace.active_view_type();
    app.state.workspace.open_as_root(view.clone(), view_type);

    let error = cross_probe_trace_to_design(&mut app, "V(x1.out)")
        .expect_err("a trace read inside X1 has no conductor on the root sheet");
    assert_eq!(error, "Descend to /x1 to cross-probe it.");
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(app.state.schematic.selection.wires.is_empty());

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
    assert!(app.state.schematic.selection.wires.is_empty());
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
    state.simulation.netlist_content = "owned\n.end\n".to_owned();
    let project_revision = state.workspace.project.revision().get();
    let digest = crate::workbench::documents::netlist_document::source_content_digest(
        &state.simulation.netlist_content,
    );
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
    state.workspace.netlist_source_dirty = true;
    assert_eq!(
        owned_source_state(&state, digest),
        "externally synchronized · validated · project modified"
    );
    state.simulation.netlist_content.push_str("* edit\n");
    let edited = crate::workbench::documents::netlist_document::source_content_digest(
        &state.simulation.netlist_content,
    );
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
        !shipped.contains("ManifestViewModel::from_run"),
        "the inspector rebuilds the manifest projection, digest and all, every frame"
    );
    assert!(
        !shipped.contains("validate_retained_evidence()"),
        "the inspector walks the retained evidence itself instead of resolving the \
         one memo that owns the verdict"
    );
}
