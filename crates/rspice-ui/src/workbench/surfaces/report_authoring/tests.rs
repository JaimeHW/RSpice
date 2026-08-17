//! Report authoring gates every mutation on exact authority.
//!
//! Opening the surface mutates nothing; a plan creates the outline once;
//! page settings and order commit revision-checked transactions and fail
//! closed at the document boundaries. The layout cases hold each internal seam
//! to exactly one pane, which is what keeps a resize from splitting a block
//! across two.

use super::*;

#[test]
fn responsive_report_builder_matches_mockup_breakpoints() {
    assert_eq!(OUTLINE_DESKTOP_WIDTH, 250.0);
    assert_eq!(OUTLINE_TABLET_WIDTH, 180.0);
    assert_eq!(INSPECTOR_WIDTH, 300.0);
    assert_eq!(
        ComposerLayout::resolve(1_280.0),
        ComposerLayout::ThreeColumn
    );
    assert_eq!(
        ComposerLayout::resolve(1_020.0),
        ComposerLayout::TwoColumnInspectorBelow
    );
    assert_eq!(
        ComposerLayout::resolve(821.0),
        ComposerLayout::TwoColumnInspectorBelow
    );
    assert_eq!(ComposerLayout::resolve(820.0), ComposerLayout::Stacked);
    assert_eq!(ComposerLayout::resolve(390.0), ComposerLayout::Stacked);
}

#[test]
fn report_commands_open_for_review_and_gate_mutations_by_exact_authority() {
    let mut app = RSpiceApp::test_instance();
    assert!(crate::workbench::surface_availability(SurfaceId::ReportAuthoring).can_open());
    assert!(
        crate::workbench::commands::vocabulary::command_catalog()
            .any(|command| command == Command::ReportAuthoring)
    );
    assert_eq!(Command::ReportAuthoring.stable_id(), "report-page-editor");
    assert_eq!(
        Command::ReportAuthoring.spec().label,
        "Report page and datasheet editor…"
    );
    for contextual_command in [
        Command::SaveReportDocument,
        Command::AddReportPage,
        Command::ReportPageProperties,
    ] {
        assert!(
            !crate::workbench::commands::vocabulary::command_catalog()
                .any(|command| command == contextual_command)
        );
    }
    assert!(Command::ReportAuthoring.is_enabled(&app));
    assert!(!Command::SaveReportDocument.is_enabled(&app));
    assert!(!Command::AddReportPage.is_enabled(&app));
    assert!(!Command::ReportPageProperties.is_enabled(&app));

    Command::ReportAuthoring.execute(&mut app);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::ReportAuthoring
    );

    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    assert!(Command::SaveReportDocument.is_enabled(&app));
    assert!(Command::AddReportPage.is_enabled(&app));
    assert!(Command::ReportPageProperties.is_enabled(&app));

    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "report command authority test".to_owned(),
    );
    assert!(Command::ReportAuthoring.is_enabled(&app));
    assert!(!Command::SaveReportDocument.is_enabled(&app));
    assert!(!Command::AddReportPage.is_enabled(&app));
    assert!(!Command::ReportPageProperties.is_enabled(&app));
}

#[test]
fn every_report_layout_assigns_each_internal_seam_to_one_pane() {
    assert_eq!(
        ComposerLayout::ThreeColumn.separators(),
        [
            PaneSeparators {
                right: true,
                ..PaneSeparators::default()
            },
            PaneSeparators {
                right: true,
                ..PaneSeparators::default()
            },
            PaneSeparators::default(),
        ]
    );
    assert_eq!(
        ComposerLayout::TwoColumnInspectorBelow.separators(),
        [
            PaneSeparators {
                right: true,
                ..PaneSeparators::default()
            },
            PaneSeparators::default(),
            PaneSeparators {
                top: true,
                ..PaneSeparators::default()
            },
        ]
    );
    assert_eq!(
        ComposerLayout::Stacked.separators(),
        [
            PaneSeparators {
                bottom: true,
                ..PaneSeparators::default()
            },
            PaneSeparators {
                bottom: true,
                ..PaneSeparators::default()
            },
            PaneSeparators::default(),
        ]
    );
}

#[test]
fn tablet_and_stacked_pane_heights_follow_local_space_and_document_content() {
    let tablet_short = composer_pane_heights(
        ComposerLayout::TwoColumnInspectorBelow,
        640.0,
        INITIAL_PAGES.len(),
        true,
    );
    let tablet_tall = composer_pane_heights(
        ComposerLayout::TwoColumnInspectorBelow,
        1_600.0,
        INITIAL_PAGES.len(),
        true,
    );
    assert!(tablet_tall.preview > tablet_short.preview);
    assert!(tablet_tall.inspector > tablet_short.inspector);
    assert!(tablet_short.preview + tablet_short.inspector + 0.01 >= 640.0);
    assert!(tablet_tall.preview + tablet_tall.inspector + 0.01 >= 1_600.0);

    let compact_seven =
        composer_pane_heights(ComposerLayout::Stacked, 720.0, INITIAL_PAGES.len(), true);
    let compact_twelve = composer_pane_heights(ComposerLayout::Stacked, 720.0, 12, true);
    assert!(compact_twelve.outline > compact_seven.outline);
    assert_eq!(compact_seven.preview, PREVIEW_MIN_HEIGHT);
    assert!(compact_seven.inspector > 300.0);
}

#[test]
fn contextual_report_commands_stay_out_of_the_searchable_registry() {
    assert!(
        crate::workbench::commands::vocabulary::COMMAND_REGISTRY
            .contains(&Command::ReportAuthoring)
    );
    // The three mutating commands act on the active document and page, so
    // they stay bound to the surface's own affordances rather than the
    // palette, where they would have no exact subject to resolve against.
    for command in [
        Command::SaveReportDocument,
        Command::AddReportPage,
        Command::ReportPageProperties,
    ] {
        assert!(!crate::workbench::commands::vocabulary::COMMAND_REGISTRY.contains(&command));
    }
}

#[test]
fn opening_report_authoring_never_mutates_an_empty_project() {
    let mut state = AppState::default();
    assert!(state.workspace.report_documents.is_empty());
    assert!(!state.workspace.report_documents_dirty);

    synchronize_report_selection(&mut state);

    assert!(state.workspace.report_documents.is_empty());
    assert!(!state.workspace.report_documents_dirty);
    assert_eq!(state.workbench.report_authoring.selected_document, None);
    assert_eq!(state.workbench.report_authoring.selected_page, None);
}

#[test]
fn explicit_report_plan_creates_the_exact_mockup_outline_once() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.preview_block_page = 7;
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    app.state
        .workbench
        .report_authoring
        .create_document_template = report_template_index(ReportTemplate::ReleaseVerification42);

    commit_create_document(&mut app);

    let document = active_document(&app.state).expect("active report");
    assert_eq!(document.pages().len(), INITIAL_PAGES.len());
    for (page, (_, expected)) in document.pages().iter().zip(INITIAL_PAGES) {
        assert_eq!(page.title(), expected);
    }
    assert!(app.state.workspace.report_documents_dirty);
    assert_eq!(app.state.workbench.report_authoring.preview_block_page, 0);
    let document_id = app.state.workbench.report_authoring.selected_document;
    let page_id = app.state.workbench.report_authoring.selected_page;
    synchronize_report_selection(&mut app.state);
    synchronize_report_selection(&mut app.state);
    assert_eq!(app.state.workspace.report_documents.len(), 1);
    assert_eq!(
        app.state.workbench.report_authoring.selected_document,
        document_id
    );
    assert_eq!(app.state.workbench.report_authoring.selected_page, page_id);
}

#[test]
fn report_plan_binds_initial_pages_to_the_active_immutable_dataset() {
    let mut app = RSpiceApp::test_instance();
    let mut run = crate::state::SimulationRun::new(41);
    run.add_analysis(crate::state::AnalysisResult::new(
        1,
        crate::state::AnalysisType::Transient,
        "retained transient",
    ));
    let expected_binding =
        crate::product::DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
    app.state.simulation.runs = vec![run];
    app.state.simulation.active_run_idx = Some(0);
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();

    commit_create_document(&mut app);

    let document = active_document(&app.state).expect("active report");
    assert!(document.pages().iter().all(|page| {
        page.evidence_binding()
            == ReportPageEvidenceBinding::ExactDataset {
                binding: expected_binding,
            }
    }));
    assert_eq!(
        document
            .revision_history()
            .records()
            .last()
            .expect("binding revision")
            .revision_note(),
        "Bind initial report pages to active result dataset"
    );
}

#[test]
fn report_page_settings_commit_canonical_revision_checked_transactions() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    let page_id = app
        .state
        .workbench
        .report_authoring
        .selected_page
        .expect("selected page");
    let initial_revision = active_document(&app.state)
        .expect("active report")
        .revision();
    app.state.workspace.report_documents_dirty = false;

    commit_page_setting(
        &mut app,
        page_id,
        PageSettingEdit::Title("Decision and release summary".to_owned()),
    );
    commit_page_setting(
        &mut app,
        page_id,
        PageSettingEdit::Inclusion(ReportPageInclusion::AppendixOnly),
    );
    commit_page_setting(
        &mut app,
        page_id,
        PageSettingEdit::EvidenceBinding(ReportPageEvidenceBinding::LatestAcceptedRun),
    );
    commit_page_setting(
        &mut app,
        page_id,
        PageSettingEdit::BlockedGateText(ReportBlockedGateTextPolicy::SummarizeWithLink),
    );

    let document = active_document(&app.state).expect("active report");
    let page = document.page(page_id).expect("selected page");
    assert_eq!(page.title(), "Decision and release summary");
    assert_eq!(page.inclusion(), ReportPageInclusion::AppendixOnly);
    assert_eq!(
        page.evidence_binding(),
        ReportPageEvidenceBinding::LatestAcceptedRun
    );
    assert_eq!(
        page.blocked_gate_text_policy(),
        ReportBlockedGateTextPolicy::SummarizeWithLink
    );
    assert_eq!(document.revision().get(), initial_revision.get() + 4);
    assert!(app.state.workspace.report_documents_dirty);
    assert!(
        app.state
            .workbench
            .report_authoring
            .transaction_error
            .is_none()
    );
}

#[test]
fn report_page_order_controls_commit_revision_checked_moves() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    let page_ids = active_document(&app.state)
        .expect("active report")
        .pages()
        .iter()
        .map(|page| page.id())
        .collect::<Vec<_>>();
    let page_to_move = page_ids[1];
    app.state.workbench.report_authoring.selected_page = Some(page_to_move);
    app.state.workspace.report_documents_dirty = false;

    assert!(can_move_selected_page(
        &app.state,
        PageMoveDirection::Earlier
    ));
    assert!(can_move_selected_page(&app.state, PageMoveDirection::Later));
    move_selected_page(&mut app, PageMoveDirection::Earlier);

    let document = active_document(&app.state).expect("active report");
    assert_eq!(document.pages()[0].id(), page_to_move);
    assert_eq!(document.pages()[1].id(), page_ids[0]);
    assert_eq!(
        document
            .revision_history()
            .records()
            .last()
            .expect("move revision")
            .revision_note(),
        "Move report page earlier"
    );
    assert!(app.state.workspace.report_documents_dirty);
    assert_eq!(
        app.state.workbench.report_authoring.selected_page,
        Some(page_to_move)
    );

    app.state.workspace.report_documents_dirty = false;
    move_selected_page(&mut app, PageMoveDirection::Later);
    let document = active_document(&app.state).expect("active report");
    assert_eq!(
        document
            .pages()
            .iter()
            .map(|page| page.id())
            .collect::<Vec<_>>(),
        page_ids
    );
    assert_eq!(
        document
            .revision_history()
            .records()
            .last()
            .expect("move revision")
            .revision_note(),
        "Move report page later"
    );
    assert!(app.state.workspace.report_documents_dirty);
}

#[test]
fn report_page_order_controls_fail_closed_at_document_boundaries() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    let first_page = active_document(&app.state).expect("active report").pages()[0].id();
    app.state.workbench.report_authoring.selected_page = Some(first_page);
    app.state.workspace.report_documents_dirty = false;
    let revision = active_document(&app.state)
        .expect("active report")
        .revision();

    assert!(!can_move_selected_page(
        &app.state,
        PageMoveDirection::Earlier
    ));
    move_selected_page(&mut app, PageMoveDirection::Earlier);

    assert_eq!(
        active_document(&app.state)
            .expect("active report")
            .revision(),
        revision
    );
    assert!(!app.state.workspace.report_documents_dirty);
}

#[test]
fn report_element_catalog_commits_every_non_plot_block_kind() {
    let mut app = RSpiceApp::test_instance();
    let mut run = crate::state::SimulationRun::new(83);
    run.add_analysis(crate::state::AnalysisResult::new(
        1,
        crate::state::AnalysisType::Transient,
        "retained transient",
    ));
    app.state.simulation.runs = vec![run];
    app.state.simulation.active_run_idx = Some(0);
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);

    for kind_index in 0..=6 {
        reset_add_report_element_kind(&mut app, kind_index);
        assert!(valid_add_report_element_draft(&app.state, true));
        commit_add_report_element(&mut app);
        assert!(
            app.state
                .workbench
                .report_authoring
                .transaction_error
                .is_none()
        );
    }

    let document = active_document(&app.state).expect("active report");
    let page_id = app
        .state
        .workbench
        .report_authoring
        .selected_page
        .expect("selected page");
    let page = document.page(page_id).expect("selected page");
    assert_eq!(page.sections().len(), 1);
    let blocks = page.sections()[0].blocks();
    assert_eq!(blocks.len(), 7);
    assert!(matches!(blocks[0].kind(), ReportBlockKind::Prose(_)));
    assert!(matches!(blocks[1].kind(), ReportBlockKind::DataTable(_)));
    assert!(matches!(blocks[2].kind(), ReportBlockKind::Datasheet(_)));
    assert!(matches!(blocks[3].kind(), ReportBlockKind::Requirements(_)));
    assert!(matches!(
        blocks[4].kind(),
        ReportBlockKind::Specifications(_)
    ));
    assert!(matches!(blocks[5].kind(), ReportBlockKind::ReviewNote(_)));
    assert!(matches!(blocks[6].kind(), ReportBlockKind::Evidence(_)));
    assert!(blocks.iter().all(|block| block.enabled()));
}

#[test]
fn page_element_toggle_and_remove_are_canonical_transactions() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    reset_add_report_element_kind(&mut app, 0);
    commit_add_report_element(&mut app);
    let document_id = active_document(&app.state).expect("active report").id();
    let block_id = app
        .state
        .workbench
        .report_authoring
        .selected_report_block
        .expect("new block selected");

    set_report_block_enabled(&mut app, document_id, block_id, false);
    assert!(
        !active_document(&app.state)
            .expect("active report")
            .block(block_id)
            .expect("block")
            .enabled()
    );
    app.state
        .workbench
        .report_authoring
        .remove_report_block_open = true;
    commit_remove_report_block(&mut app);
    let document = active_document(&app.state).expect("active report");
    assert!(document.block(block_id).is_none());
    assert!(
        document
            .tombstones()
            .iter()
            .any(|tombstone| tombstone.entity == ReportEntityRef::Block(block_id))
    );
    assert_eq!(
        app.state.workbench.report_authoring.selected_report_block,
        None
    );
}

#[test]
fn publication_inspector_edits_are_canonical_document_transactions() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    let document_id = active_document(&app.state).expect("active report").id();
    let initial_revision = active_document(&app.state)
        .expect("active report")
        .revision();
    let output_formats = ReportOutputFormats {
        pdf_a: true,
        html_bundle: false,
        canonical_json: true,
        selected_csv: true,
    };
    commit_document_publication_setting(
        &mut app,
        document_id,
        DocumentPublicationEdit::OutputFormats(output_formats),
    );
    let publication_profile = ReportPublicationProfile {
        template: ReportPublicationTemplate::CustomerDatasheet,
        page_size: ReportPublicationPageSize::A3Landscape,
        draft_marking: ReportDraftMarking::NeverWatermark,
        numbering: ReportPageNumbering::ContinuousPageNumbers,
        table_precision: ReportTablePrecision::FullStoredF64,
    };
    commit_document_publication_setting(
        &mut app,
        document_id,
        DocumentPublicationEdit::PublicationProfile(publication_profile),
    );

    let document = active_document(&app.state).expect("active report");
    assert_eq!(document.output_formats(), output_formats);
    assert_eq!(document.publication_profile(), publication_profile);
    assert_eq!(document.revision().get(), initial_revision.get() + 2);
    assert!(app.state.workspace.report_documents_dirty);
    assert!(
        app.state
            .workbench
            .report_authoring
            .transaction_error
            .is_none()
    );
}

#[test]
fn inserting_result_document_binds_exact_revision_digest_and_dataset() {
    use crate::results::visualization_document::{
        ColumnRole, SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType,
        VisualizationDocument,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    let binding = crate::product::DatasetBinding::new(
        crate::product::DatasetId::new(),
        crate::product::ContentDigest::from_bytes([0x83; 32]),
    );
    let dataset = SourceDataset::new(
        binding,
        vec![
            SourceColumn::new(
                "time",
                "Time",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("s".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "vout",
                "V(out)",
                ValueType::Real,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ],
        vec![SourceRow::new(vec![
            TypedValue::Real(0.0),
            TypedValue::Real(1.0),
        ])],
    )
    .unwrap();
    let source = VisualizationDocument::new("Nominal response", vec![dataset]).unwrap();
    let source_id = source.id();
    let source_revision = source.revision();
    let source_digest = source.content_digest().unwrap();
    let expected_page_id = source.pages()[0].id.get();
    let expected_pane_id = source.panes()[0].id.get();
    app.state.workspace.visualization_documents.push(source);

    open_insert_result_document(&mut app);
    commit_insert_result_document(&mut app);

    let block_id = app
        .state
        .workbench
        .report_authoring
        .selected_report_block
        .expect("inserted plot selected");
    let block = active_document(&app.state)
        .expect("active report")
        .block(block_id)
        .expect("inserted plot");
    let ReportBlockKind::PlotFigure(figure) = block.kind() else {
        panic!("inserted block must be a plot figure");
    };
    let snapshot = figure.reference.snapshot();
    assert_eq!(
        snapshot.source,
        ReportSourceId::VisualizationDocument {
            document_id: source_id
        }
    );
    assert_eq!(snapshot.source_revision, Some(source_revision));
    assert_eq!(snapshot.content_digest, source_digest);
    assert_eq!(snapshot.dataset_bindings, vec![binding]);
    assert_eq!(
        figure.source_locator,
        Some(ReportFigureSourceLocator {
            page_id: expected_page_id,
            pane_id: expected_pane_id,
        })
    );
    assert!(report_reference_resolves(&app.state, &figure.reference));
}

#[test]
fn report_page_markers_remain_semantically_stable_after_reordering() {
    assert_eq!(page_marker(0, "Executive summary"), "1");
    assert_eq!(page_marker(5, "Executive summary"), "1");
    assert_eq!(page_marker(0, "Run manifests"), "A");
    assert_eq!(page_marker(7, "Custom appendix"), "+");
}

#[test]
fn direct_report_creation_fails_closed_in_read_only_safe_mode() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "report authoring test".to_owned(),
    );
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();

    commit_create_document(&mut app);

    assert!(app.state.workspace.report_documents.is_empty());
    assert!(!app.state.workspace.report_documents_dirty);
    assert_eq!(
        app.state
            .workbench
            .report_authoring
            .transaction_error
            .as_deref(),
        Some("Report changes are unavailable because the active project is read-only.")
    );
}

#[test]
fn report_page_title_validation_matches_domain_limits() {
    assert!(valid_page_title("PVT and yield"));
    assert!(!valid_page_title(""));
    assert!(!valid_page_title(" leading"));
    assert!(!valid_page_title("trailing "));
    assert!(!valid_page_title("bad\nlabel"));
    assert!(!valid_page_title(&"x".repeat(513)));
}

#[test]
fn changing_active_run_does_not_mutate_the_project_report_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    app.state.simulation.runs = vec![
        crate::state::SimulationRun::new(2),
        crate::state::SimulationRun::new(1),
    ];
    let document = active_document(&app.state)
        .expect("report document")
        .clone();
    app.state.simulation.active_run_idx = Some(0);
    app.state.simulation.active_run_idx = Some(1);
    assert_eq!(active_document(&app.state), Some(&document));
}

#[test]
fn prose_preview_is_unicode_safe_and_bounded() {
    const MAXIMUM_CHARACTERS: usize = 4_096;
    let exact = "a".repeat(MAXIMUM_CHARACTERS);
    let (preview, truncated) = bounded_text_preview(&exact, MAXIMUM_CHARACTERS);
    assert!(!truncated);
    assert_eq!(preview, exact);

    let oversized = format!("{}é-tail", exact);
    let (preview, truncated) = bounded_text_preview(&oversized, MAXIMUM_CHARACTERS);
    assert!(truncated);
    assert_eq!(preview.chars().count(), MAXIMUM_CHARACTERS + 1);
    assert!(preview.ends_with('…'));
}

#[test]
fn invalid_report_page_selection_resets_preview_pagination() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut app);
    app.state.workbench.report_authoring.selected_page =
        Some(crate::results::report_document::ReportPageId::new());
    app.state.workbench.report_authoring.preview_block_page = 4;

    synchronize_report_selection(&mut app.state);

    assert_eq!(app.state.workbench.report_authoring.preview_block_page, 0);
    assert_eq!(
        app.state.workbench.report_authoring.selected_page,
        active_document(&app.state)
            .and_then(|document| document.pages().first())
            .map(|page| page.id())
    );
}

#[test]
fn report_joint_yield_requires_aligned_all_spec_sample_trails() {
    fn result(measurement: &str, trail: Vec<bool>) -> crate::services::yield_manager::YieldResult {
        let total_runs = trail.len();
        let pass_count = trail.iter().filter(|passed| **passed).count();
        crate::services::yield_manager::YieldResult {
            spec: crate::services::yield_manager::YieldSpec::lower(measurement, 0.0, "V"),
            total_runs,
            pass_count,
            fail_count: total_runs - pass_count,
            yield_percent: pass_count as f64 / total_runs as f64 * 100.0,
            stats: crate::services::yield_manager::DistributionStats::default(),
            trail,
            samples: vec![0.0; total_runs],
        }
    }

    let aligned = [
        result("V(out)", vec![true, true, false]),
        result("I(V1)", vec![true, false, true]),
    ];
    assert_eq!(
        ReportJointYield::from_results(&aligned),
        Some(ReportJointYield {
            passing: 1,
            total: 3,
        })
    );

    let misaligned = [
        result("V(out)", vec![true, false]),
        result("I(V1)", vec![true]),
    ];
    assert_eq!(ReportJointYield::from_results(&misaligned), None);
}

#[test]
fn report_summary_uses_verified_spec_and_exact_corner_evidence() {
    fn provenance(seed: u8) -> crate::state::AnalysisResultProvenance {
        crate::state::AnalysisResultProvenance::new(
            crate::product::AnalysisInstanceId::new(),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([seed; 32]),
            Vec::new(),
        )
        .expect("test provenance is valid")
    }

    let mut app = RSpiceApp::test_instance();
    app.state.workspace.specs = vec![
        crate::state::SpecEntry {
            measurement: "gain".to_owned(),
            expression: String::new(),
            min: Some(10.0),
            max: None,
            unit: "V/V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        crate::state::SpecEntry {
            measurement: "bandwidth".to_owned(),
            expression: String::new(),
            min: Some(1_000.0),
            max: None,
            unit: "Hz".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
    ];
    let checks = crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Ac, "AC")
        .with_measurements(vec![
            rspice_core::MeasureResult::success("gain", 12.0),
            rspice_core::MeasureResult::success("bandwidth", 900.0),
        ])
        .with_provenance(provenance(0x31));
    let corners = crate::state::AnalysisResult::new(2, crate::state::AnalysisType::Corner, "PVT")
        .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::Corner {
            x_values: vec![0.0, 1.0, 2.0],
            x_label: "corner".to_owned(),
            x_unit: String::new(),
            temperatures_c: vec![-40.0, 27.0, 125.0],
            corner_labels: vec!["ss".to_owned(), "tt".to_owned(), "ff".to_owned()],
            failed_corners: 1,
        })
        .with_provenance(provenance(0x32));
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(checks);
    run.add_analysis(corners);
    app.state.simulation.runs = vec![run];
    app.state.simulation.active_run_idx = Some(0);

    let metrics = ReportSummaryMetrics::from_state(&app.state);
    assert_eq!(metrics.checks_passing, 1);
    assert_eq!(metrics.checks_total, 2);
    assert_eq!(metrics.pvt_completed, Some(2));
    assert_eq!(metrics.pvt_total, Some(3));
}
