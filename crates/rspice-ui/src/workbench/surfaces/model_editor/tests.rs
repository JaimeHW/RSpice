//! Tests for the model editor surface's authoring behaviour.
//!
//! The cases pin that normalizing a selection never resizes the form, that
//! fallbacks are deterministic, that cancelling authoring discards every
//! partial field rather than leaving some behind, and that every count the
//! surface states is painted by the element that owns it.

use super::*;

/// A project-owned candidate with enough declared parameters that the counts
/// the rail and the section headers state are not all zero.
#[cfg(not(target_arch = "wasm32"))]
fn app_with_open_candidate() -> RSpiceApp {
    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    app.state
        .model_library_manager
        .create_project_model(
            "owned-models",
            &ProjectModelDefinition {
                name: "nch_owned".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Project model".to_owned(),
                numeric_parameters: BTreeMap::from([
                    ("vto".to_owned(), 0.7),
                    ("kp".to_owned(), 120e-6),
                    ("gamma".to_owned(), 0.45),
                ]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create project model");
    let revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "owned-models",
            "nch_owned",
            revision,
        )
        .expect("open model candidate");
    app
}

/// Every string the surface paints, with the colour it was laid out in, at the
/// workspace width the surface is designed against.
#[cfg(not(target_arch = "wasm32"))]
fn painted_surface(app: &mut RSpiceApp, section: ModelEditorSection) -> Vec<(String, Color32)> {
    fn walk(shape: &egui::epaint::Shape, into: &mut Vec<(String, Color32)>) {
        match shape {
            egui::epaint::Shape::Text(painted) => into.push((
                painted.galley.job.text.clone(),
                painted
                    .galley
                    .job
                    .sections
                    .first()
                    .map_or(painted.fallback_color, |section| section.format.color),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }

    app.state.workbench.model_editor.active_section = section;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let size = Vec2::new(1_180.0, 900.0);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show(ui, app));
        },
    );
    let mut painted = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut painted);
    }
    painted
}

#[test]
fn section_contract_matches_the_six_mockup_tabs() {
    assert_eq!(SECTION_SPECS.len(), 6);
    assert_eq!(
        SECTION_SPECS.map(|spec| spec.section),
        ModelEditorSection::ALL
    );
    assert_eq!(
        SECTION_SPECS[0].title,
        "Typed parameters, inheritance, units, and bounds"
    );
    assert_eq!(SECTION_SPECS[4].action_label, Some("Qualification plan…"));
    assert_eq!(SECTION_SPECS[5].action_label, None);
}

#[test]
fn column_projection_is_stable_and_exactly_fills_the_row() {
    let columns = [("A", 0.16), ("B", 0.19), ("C", 0.65)];
    let rect = Rect::from_min_size(Pos2::new(11.0, 7.0), Vec2::new(913.0, 28.0));
    let cells = split_columns(rect, &columns);
    assert_eq!(cells.len(), columns.len());
    assert_eq!(cells.first().unwrap().left(), rect.left());
    assert_eq!(cells.last().unwrap().right(), rect.right());
    assert!(
        cells
            .windows(2)
            .all(|pair| pair[0].right() == pair[1].left())
    );
}

#[test]
fn fixed_workspace_geometry_does_not_depend_on_selected_section() {
    for section in ModelEditorSection::ALL {
        assert_eq!(section_spec(section).section, section);
        assert_eq!(NAV_ROW_H, 36.0);
        assert_eq!(SECTION_HEADER_H, 70.0);
        assert_eq!(TABLE_HEADER_H, 27.0);
    }
}

#[test]
fn specialist_header_preserves_the_mockup_ownership_breakpoint() {
    assert_eq!(SPECIALIST_HEADER_H, 82.0);
    assert_eq!(SPECIALIST_HEADER_MEDIUM_H, 124.0);
    assert_eq!(specialist_header_height(820.0), SPECIALIST_HEADER_H);
    assert_eq!(specialist_header_height(821.0), SPECIALIST_HEADER_MEDIUM_H);
    assert_eq!(
        specialist_header_height(1_120.0),
        SPECIALIST_HEADER_MEDIUM_H
    );
    assert_eq!(specialist_header_height(1_121.0), SPECIALIST_HEADER_H);
}

#[test]
fn navigation_scroll_extent_always_contains_all_tabs_and_evidence() {
    const NOTE_H: f32 = 119.0;
    let required = SECTION_SPECS.len() as f32 * NAV_ROW_H + NOTE_H;
    assert_eq!(required, 335.0);
    assert!(
        required > 220.0,
        "short workspaces must scroll, not clip tabs"
    );
}

#[test]
fn identity_elision_preserves_both_collision_significant_ends() {
    assert_eq!(short_identity("short"), "short");
    assert_eq!(
        short_identity("0123456789abcdef0123456789abcdef"),
        "01234567…abcdef"
    );
}

#[test]
fn qualification_progress_combines_suite_and_vector_progress() {
    assert_eq!(qualification_progress_fraction(0, 0, 0, 0), 0.0);
    assert_eq!(qualification_progress_fraction(0, 2, 1, 4), 0.125);
    assert_eq!(qualification_progress_fraction(1, 2, 0, 4), 0.5);
    assert_eq!(qualification_progress_fraction(1, 2, 4, 4), 1.0);
    assert_eq!(qualification_progress_fraction(4, 2, 0, 0), 1.0);
}

#[test]
fn qualification_form_geometry_is_independent_of_combo_choices() {
    assert_eq!(QUALIFICATION_FIELD_ROW_H, 54.0);
    assert_eq!(QUALIFICATION_MULTILINE_ROW_H, 118.0);
    assert_eq!(QUALIFICATION_STATUS_H, 86.0);
    assert_eq!(QUALIFICATION_ERROR_H, 50.0);
}

#[test]
fn qualification_authoring_exposes_every_solver_analysis_domain() {
    let analyses = [
        QualificationAuthoringAnalysis::DcOperatingPoint,
        QualificationAuthoringAnalysis::DcSweep,
        QualificationAuthoringAnalysis::AcSweep,
        QualificationAuthoringAnalysis::Noise,
        QualificationAuthoringAnalysis::Transient,
    ];
    for analysis in analyses {
        assert!(!qualification_authoring_probe_options(analysis).is_empty());
        assert!(!qualification_authoring_sample_options(analysis).is_empty());
    }
    assert_eq!(
        qualification_authoring_probe_options(QualificationAuthoringAnalysis::AcSweep).len(),
        10
    );
    assert_eq!(
        qualification_authoring_probe_options(QualificationAuthoringAnalysis::Noise).len(),
        5
    );
    assert_eq!(
        qualification_authoring_probe_options(QualificationAuthoringAnalysis::Transient).len(),
        3
    );
}

#[test]
fn qualification_authoring_normalizes_probe_and_sample_without_resizing_form() {
    let mut fields = crate::workbench::documents::model_editor::QualificationAuthoringDraft {
        analysis: QualificationAuthoringAnalysis::Noise,
        probe: QualificationAuthoringProbe::NodeVoltage,
        sample: QualificationAuthoringSample::OperatingPoint,
        ..Default::default()
    };
    qualification_normalize_authoring_domains(&mut fields);
    assert_eq!(fields.probe, QualificationAuthoringProbe::FrequencyValue);
    assert_eq!(
        fields.sample,
        QualificationAuthoringSample::FirstFrequencyPoint
    );
    assert!(!qualification_probe_requires_target(fields.probe));
    assert!(!qualification_sample_requires_index(fields.sample));
    assert!(qualification_sample_requires_index(
        QualificationAuthoringSample::FrequencyPoint
    ));
}

#[test]
fn qualification_selection_falls_back_deterministically() {
    let mut state = ModelQualificationState::default();
    state.suites.push(QualificationSuite {
        schema_version: crate::state::model_library::MODEL_QUALIFICATION_SCHEMA_VERSION,
        id: "suite-b".to_owned(),
        name: "Suite B".to_owned(),
        revision: crate::product::ObjectRevision::INITIAL,
        vectors: Vec::new(),
    });
    let mut plan = QualificationPlanUiState {
        selected_suite_id: "missing".to_owned(),
        selected_vector_id: "missing".to_owned(),
        ..Default::default()
    };
    normalize_qualification_selection(&state, &mut plan);
    assert_eq!(plan.selected_suite_id, "suite-b");
    assert!(plan.selected_vector_id.is_empty());
}

#[test]
fn retained_platform_runs_project_before_cross_platform_evidence_exists() {
    let source_digest = crate::product::ContentDigest::from_bytes([0x51; 32]);
    let source_revision = crate::product::ObjectRevision::INITIAL;
    let source_id = crate::product::ModelSourceId::from_namespace(
        uuid::Uuid::from_u128(0x795d_0456_80da_46b3_95f5_a327_aa67_323a),
        b"surface-platform-run",
    );
    let suite = QualificationSuite {
        schema_version: crate::state::model_library::MODEL_QUALIFICATION_SCHEMA_VERSION,
        id: "dc-parity".to_owned(),
        name: "DC parity".to_owned(),
        revision: crate::product::ObjectRevision::INITIAL,
        vectors: Vec::new(),
    };
    let source = crate::state::model_library::ModelSourceEvidenceBinding {
        model_id: "nch_qualified".to_owned(),
        source_id: Some(source_id),
        source_digest,
        source_revision,
    };
    let desktop = QualificationPlatformRun {
        schema_version: crate::state::model_library::MODEL_QUALIFICATION_SCHEMA_VERSION,
        platform: QualificationPlatform::Desktop,
        source,
        suite_id: suite.id.clone(),
        suite_revision: suite.revision,
        vector_outcomes: vec![
            crate::state::model_library::QualificationPlatformVectorOutcome {
                vector_id: "nominal".to_owned(),
                input_digest: source_digest,
                outcome: crate::state::model_library::PlatformQualificationOutcome {
                    platform: QualificationPlatform::Desktop,
                    references: Vec::new(),
                    failure: None,
                    passed: true,
                },
            },
        ],
        passed: true,
    };
    let mut state = ModelQualificationState::default();
    state.platform_runs.push(desktop);

    let retained_desktop = exact_platform_run(
        &state,
        &suite,
        "NCH_QUALIFIED",
        source_id,
        source_digest,
        source_revision,
        QualificationPlatform::Desktop,
    );
    let missing_wasm = exact_platform_run(
        &state,
        &suite,
        "nch_qualified",
        source_id,
        source_digest,
        source_revision,
        QualificationPlatform::WebAssembly,
    );

    assert!(state.evidence.is_empty());
    assert_eq!(platform_run_summary(retained_desktop), "1 pass");
    assert!(missing_wasm.is_none());
    assert_eq!(platform_run_summary(missing_wasm), "no run");
    assert!(
        exact_platform_run(
            &state,
            &suite,
            "nch_qualified",
            source_id,
            crate::product::ContentDigest::from_bytes([0x52; 32]),
            source_revision,
            QualificationPlatform::Desktop,
        )
        .is_none()
    );
    assert!(
        exact_platform_run(
            &state,
            &suite,
            "nch_qualified",
            crate::product::ModelSourceId::new(),
            source_digest,
            source_revision,
            QualificationPlatform::Desktop,
        )
        .is_none()
    );
}

#[test]
fn cancelling_qualification_authoring_discards_every_partial_field() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.model_editor.begin_qualification_suite();
    let fields = &mut app.state.workbench.model_editor.qualification_authoring;
    fields.suite_id = "partial-suite".to_owned();
    fields.vector_id = "partial-vector".to_owned();
    fields.executable_input = "V1 in 0 1".to_owned();
    fields.analysis = QualificationAuthoringAnalysis::DcSweep;
    fields.probe = QualificationAuthoringProbe::BranchCurrent;
    fields.sample = QualificationAuthoringSample::SweepPoint;
    fields.error = Some("partial error".to_owned());

    cancel_qualification_authoring(&mut app.state.workbench.model_editor);

    let editor = &app.state.workbench.model_editor;
    assert!(!editor.qualification_authoring_open);
    assert!(editor.qualification_authoring.suite_id.is_empty());
    assert!(editor.qualification_authoring.vector_id.is_empty());
    assert!(editor.qualification_authoring.executable_input.is_empty());
    assert_eq!(
        editor.qualification_authoring.analysis,
        QualificationAuthoringAnalysis::DcOperatingPoint
    );
    assert_eq!(
        editor.qualification_authoring.probe,
        QualificationAuthoringProbe::NodeVoltage
    );
    assert_eq!(
        editor.qualification_authoring.sample,
        QualificationAuthoringSample::OperatingPoint
    );
    assert!(editor.qualification_authoring.error.is_none());
}

/// Each rail entry states the count its own section owns, and only that: no
/// entry projects a fact another section is responsible for.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_navigation_rail_projects_only_section_owned_counts() {
    let app = app_with_open_candidate();
    let records = persisted_records(&app);
    let metadata = records.metadata.as_ref().expect("persisted schema");
    let parameters = metadata.parameters.len();
    let sections = metadata.sections.len();
    let qualification = qualification_totals(&app, &records);

    assert_eq!(
        navigation_meta(&app, &records, ModelEditorSection::Parameters),
        Some((parameters.to_string(), MetricTone::Neutral))
    );
    assert_eq!(
        navigation_meta(&app, &records, ModelEditorSection::Sections),
        Some((sections.to_string(), MetricTone::Neutral))
    );
    assert_eq!(
        navigation_meta(&app, &records, ModelEditorSection::Tests),
        Some((
            format!("{}/{}", qualification.desktop_passed, qualification.total),
            qualification.desktop_tone
        ))
    );
    assert_eq!(qualification.desktop_tone, MetricTone::Warning);
    for section in [
        ModelEditorSection::Statistics,
        ModelEditorSection::Temperature,
        ModelEditorSection::Release,
    ] {
        assert_eq!(navigation_meta(&app, &records, section), None);
    }
}

/// A rejected persisted schema tones both count entries and the parameter
/// split, because a count read off a rejected definition is exactly the fact
/// whose health is in doubt.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_rejected_schema_tones_every_count_read_off_it() {
    let app = app_with_open_candidate();
    let mut records = persisted_records(&app);
    records.metadata_error = Some("parameters[0]: out of declared bounds".to_owned());
    for section in [
        ModelEditorSection::Parameters,
        ModelEditorSection::Sections,
    ] {
        let (_, tone) = navigation_meta(&app, &records, section).expect("count");
        assert_eq!(tone, MetricTone::Error);
    }
    let (_, tone) =
        section_header_fact(&app, &records, ModelEditorSection::Parameters).expect("split");
    assert_eq!(tone, MetricTone::Error);
}

/// The parameter source split and runtime parity are section aggregates: no
/// single row can carry them, and no other section restates them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn section_headers_own_the_aggregates_no_row_can_carry() {
    let app = app_with_open_candidate();
    let records = persisted_records(&app);
    let metadata = records.metadata.as_ref().expect("persisted schema");
    let (declared, inherited, overridden) = parameter_source_split(metadata);
    let qualification = qualification_totals(&app, &records);

    assert_eq!(
        section_header_fact(&app, &records, ModelEditorSection::Parameters),
        Some((
            format!("{declared} declared · {inherited} inherited · {overridden} override"),
            MetricTone::Neutral
        ))
    );
    assert_eq!(
        section_header_fact(&app, &records, ModelEditorSection::Tests),
        Some((
            format!(
                "{} / {} WebAssembly · {}",
                qualification.wasm_passed, qualification.total, qualification.wasm_detail
            ),
            qualification.wasm_tone
        ))
    );
    // Section names, statistical variables, temperature laws, and release
    // facts are already stated row by row on their own pages.
    for section in [
        ModelEditorSection::Sections,
        ModelEditorSection::Statistics,
        ModelEditorSection::Temperature,
        ModelEditorSection::Release,
    ] {
        assert_eq!(section_header_fact(&app, &records, section), None);
    }
}

/// Every projected fact is painted, in its tone, by the element that owns it.
/// Nothing above the workspace restates one: `Parity` names a fact the Tests
/// section owns, so that label anywhere is a KPI band standing over the page.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_owned_fact_is_painted_by_its_owner() {
    let mut app = app_with_open_candidate();
    let records = persisted_records(&app);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let t = Tokens::get(&ctx);

    let (parameters, _) =
        navigation_meta(&app, &records, ModelEditorSection::Parameters).expect("count");
    let (tests, tests_tone) =
        navigation_meta(&app, &records, ModelEditorSection::Tests).expect("ratio");
    let (split, _) =
        section_header_fact(&app, &records, ModelEditorSection::Parameters).expect("split");
    let (parity, parity_tone) =
        section_header_fact(&app, &records, ModelEditorSection::Tests).expect("parity");

    let parameters_page = painted_surface(&mut app, ModelEditorSection::Parameters);
    assert!(
        parameters_page.contains(&(parameters, t.color.text_dim)),
        "{parameters_page:?}"
    );
    assert!(
        parameters_page.contains(&(tests, meta_color(&t, tests_tone))),
        "{parameters_page:?}"
    );
    assert!(
        parameters_page.contains(&(split, t.color.text_dim)),
        "{parameters_page:?}"
    );

    let tests_page = painted_surface(&mut app, ModelEditorSection::Tests);
    assert!(
        tests_page.contains(&(parity, meta_color(&t, parity_tone))),
        "{tests_page:?}"
    );
    for page in [parameters_page, tests_page] {
        assert!(
            !page.iter().any(|(text, _)| text == "Parity"),
            "a Parity cell restates a fact the Tests section owns: {page:?}"
        );
    }
}

/// The workspace owns every pixel below the specialist header: the navigation
/// rail starts directly under it and reaches the bottom edge, so a band
/// inserted between the two would leave an unpainted seam here.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_workspace_body_tiles_the_surface_under_the_specialist_header() {
    fn walk(shape: &egui::epaint::Shape, fill: Color32, out: &mut Vec<Rect>) {
        match shape {
            egui::epaint::Shape::Rect(rect) if rect.fill == fill => out.push(rect.rect),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, fill, out);
                }
            }
            _ => {}
        }
    }

    let mut app = app_with_open_candidate();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let size = Vec2::new(1_180.0, 900.0);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show(ui, &mut app));
        },
    );
    let fill = Tokens::get(&ctx).color.bg_panel;
    let mut panels = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, fill, &mut panels);
    }
    let body_top = OUTER_IDENTITY_H + specialist_header_height(size.x);
    let nav_w = NAV_W.min((size.x * 0.31).max(150.0));
    assert!(
        panels.iter().any(|rect| {
            rect.left().abs() <= 1.0
                && (rect.right() - nav_w).abs() <= 1.0
                && (rect.top() - body_top).abs() <= 1.0
                && (rect.bottom() - size.y).abs() <= 1.0
        }),
        "the navigation rail must tile from {body_top} to the bottom edge: {panels:?}"
    );
}

/// Renders every model editor section to `RSPICE_RASTER_DIR` (or the system
/// temp directory); read them for layout, not wording — the rasterizer's own
/// header says why.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_section_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    for (index, section) in ModelEditorSection::ALL.into_iter().enumerate() {
        let mut app = app_with_open_candidate();
        app.state.workbench.model_editor.active_section = section;
        let canvas = crate::ui::raster::render(Vec2::new(1_180.0, 900.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| show(ui, &mut app));
        });
        let slug = section
            .label()
            .to_ascii_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>();
        let path = directory.join(format!("model-editor-{index:02}-{slug}.png"));
        let height = canvas.content_height().max(200);
        std::fs::write(&path, canvas.png(height)).expect("write png");
        writeln!(report, "wrote {}", path.display()).ok();
    }
}
