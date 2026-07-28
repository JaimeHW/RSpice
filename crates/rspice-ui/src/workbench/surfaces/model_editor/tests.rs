//! Tests for the model editor surface's authoring behaviour.
//!
//! The cases pin that normalizing a selection never resizes the form, that
//! fallbacks are deterministic, and that cancelling authoring discards every
//! partial field rather than leaving some behind.

use super::*;

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

    cancel_qualification_authoring(&mut app);

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
