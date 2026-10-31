//! Report authoring in a whole application: its commands, read through the
//! product's own registry, and the document transactions they commit.

use crate::results::report_document::ReportBlockKind;
use crate::results::report_document::ReportDocument;
use crate::results::report_document::ReportEntityRef;
use crate::results::report_document::ReportFigureSourceLocator;
use crate::results::report_document::ReportPageEvidenceBinding;
use crate::results::report_document::ReportSourceId;
use crate::results::report_document::ReportTemplate;
use crate::services::model_hub::ModelHubService;
use crate::simulation::SimulationController;
use crate::workbench::SurfaceId;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::feature::{LowerHandles, Peers};
use crate::workbench::hardcopy::HardcopyFeature;
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::hardcopy::report as report_editor;
use crate::workbench::hardcopy::report_authoring::result_insert::commit_insert_result_document;
use crate::workbench::hardcopy::report_authoring::result_insert::open_insert_result_document;
use crate::workbench::hardcopy::report_authoring::*;
use crate::workbench::{AppState, RSpiceApp};
use rspice_results::report_document::{
    ReportBlockedGateTextPolicy, ReportDraftMarking, ReportOutputFormats, ReportPageInclusion,
    ReportPageNumbering, ReportPublicationPageSize, ReportPublicationProfile,
    ReportPublicationTemplate, ReportTablePrecision,
};
use rspice_results_ui::report::INITIAL_PAGES;
use rspice_results_ui::report::composer::PageMoveDirection;
use rspice_results_ui::report::composer::PageSettingEdit;
use rspice_results_ui::report::inspector::DocumentPublicationEdit;
use rspice_results_ui::report::session::report_template_index;

/// The hardcopy feature's context over `state`, built as the shell builds
/// it, with the export IO the application uses, `run_controller` and
/// `model_hub`.
fn cx<'a>(
    state: &'a mut AppState,
    run_controller: &'a mut SimulationController,
    model_hub: &'a mut ModelHubService,
) -> Cx<'a, HardcopyFeature> {
    Cx::new(
        &mut *state.hardcopy,
        &mut [],
        &mut state.model,
        &mut *state.workbench,
        &mut state.platform,
        &mut state.runs,
        LowerHandles {
            export_io:
                &rspice_app_services::workbench::workflows::export_workflow::NativeExportWorkflowIo,
            run_controller,
            model_hub,
        },
        Peers::new(
            &mut *state.design,
            Some(&*state.cloud),
            None,
            Some(&*state.results),
            &*state.simulate,
            &*state.code,
            Some(&*state.models),
        ),
        command_registry(),
    )
}

/// The report document the editor has open in `state`.
fn active_report(state: &AppState) -> Option<&ReportDocument> {
    report_editor::active_document(&state.hardcopy.report_authoring, &state.model)
}

/// Whether the shell would carry out `command` now, as the menus and the
/// report toolbar ask it.
fn enabled(app: &RSpiceApp, command: HardcopyCommand) -> bool {
    app.command_key_enabled(command.key())
}

#[test]
fn report_commands_open_for_review_and_gate_mutations_by_exact_authority() {
    let mut app = RSpiceApp::test_instance();
    assert!(
        crate::workbench::routing::availability::surface_availability(SurfaceId::ReportAuthoring)
            .can_open()
    );
    let registry = command_registry();
    let report = HardcopyCommand::ReportAuthoring.key();
    assert!(registry.order().contains(&report));
    assert_eq!(report.as_str(), "report-page-editor");
    assert_eq!(registry.label(report), "Report page and datasheet editor…");
    for contextual_command in HardcopyCommand::REPORT_TOOLBAR {
        assert!(!registry.order().contains(&contextual_command.key()));
    }
    assert!(enabled(&app, HardcopyCommand::ReportAuthoring));
    assert!(!enabled(&app, HardcopyCommand::SaveReportDocument));
    assert!(!enabled(&app, HardcopyCommand::AddReportPage));
    assert!(!enabled(&app, HardcopyCommand::ReportPageProperties));

    app.execute_command_key(report);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::ReportAuthoring
    );

    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    assert!(enabled(&app, HardcopyCommand::SaveReportDocument));
    assert!(enabled(&app, HardcopyCommand::AddReportPage));
    assert!(enabled(&app, HardcopyCommand::ReportPageProperties));

    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        "report command authority test".to_owned(),
    );
    assert!(enabled(&app, HardcopyCommand::ReportAuthoring));
    assert!(!enabled(&app, HardcopyCommand::SaveReportDocument));
    assert!(!enabled(&app, HardcopyCommand::AddReportPage));
    assert!(!enabled(&app, HardcopyCommand::ReportPageProperties));
}

#[test]
fn contextual_report_commands_stay_out_of_the_searchable_registry() {
    let registry = command_registry();
    assert!(
        registry
            .order()
            .contains(&HardcopyCommand::ReportAuthoring.key())
    );
    // The three mutating commands act on the active document and page, so
    // they stay bound to the surface's own affordances rather than the
    // palette, where they would have no exact subject to resolve against.
    for command in HardcopyCommand::REPORT_TOOLBAR {
        assert!(!registry.order().contains(&command.key()), "{command:?}");
        assert_eq!(
            registry.resolve(command.key().as_str()),
            None,
            "{command:?}"
        );
    }
}

#[test]
fn opening_report_authoring_never_mutates_an_empty_project() {
    let mut state = AppState::default();
    assert!(state.model.workspace.content.report_documents.is_empty());
    assert!(!state.model.workspace.content.report_documents_dirty);

    synchronize_report_selection(&mut cx(
        &mut state,
        &mut SimulationController::new(),
        &mut ModelHubService::unavailable("This test runs without a model-pack store."),
    ));

    assert!(state.model.workspace.content.report_documents.is_empty());
    assert!(!state.model.workspace.content.report_documents_dirty);
    assert_eq!(state.hardcopy.report_authoring.selected_document, None);
    assert_eq!(state.hardcopy.report_authoring.selected_page, None);
}

#[test]
fn explicit_report_plan_creates_the_exact_mockup_outline_once() {
    let mut app = RSpiceApp::test_instance();
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    app.state.hardcopy.report_authoring.create_document_template =
        report_template_index(ReportTemplate::ReleaseVerification42);

    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));

    let document = active_report(&app.state).expect("active report");
    assert_eq!(document.pages().len(), INITIAL_PAGES.len());
    for (page, (_, expected)) in document.pages().iter().zip(INITIAL_PAGES) {
        assert_eq!(page.title(), expected);
    }
    assert!(app.state.model.workspace.content.report_documents_dirty);
    let document_id = app.state.hardcopy.report_authoring.selected_document;
    let page_id = app.state.hardcopy.report_authoring.selected_page;
    synchronize_report_selection(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    synchronize_report_selection(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    assert_eq!(app.state.model.workspace.content.report_documents.len(), 1);
    assert_eq!(
        app.state.hardcopy.report_authoring.selected_document,
        document_id
    );
    assert_eq!(app.state.hardcopy.report_authoring.selected_page, page_id);
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
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();

    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));

    let document = active_report(&app.state).expect("active report");
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
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    let page_id = app
        .state
        .hardcopy
        .report_authoring
        .selected_page
        .expect("selected page");
    let initial_revision = active_report(&app.state).expect("active report").revision();
    app.state.model.workspace.content.report_documents_dirty = false;

    commit_page_setting(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        page_id,
        PageSettingEdit::Title("Decision and release summary".to_owned()),
    );
    commit_page_setting(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        page_id,
        PageSettingEdit::Inclusion(ReportPageInclusion::AppendixOnly),
    );
    commit_page_setting(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        page_id,
        PageSettingEdit::EvidenceBinding(ReportPageEvidenceBinding::LatestAcceptedRun),
    );
    commit_page_setting(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        page_id,
        PageSettingEdit::BlockedGateText(ReportBlockedGateTextPolicy::SummarizeWithLink),
    );

    let document = active_report(&app.state).expect("active report");
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
    assert!(app.state.model.workspace.content.report_documents_dirty);
    assert!(
        app.state
            .hardcopy
            .report_authoring
            .transaction_error
            .is_none()
    );
}

#[test]
fn report_page_order_controls_commit_revision_checked_moves() {
    let mut app = RSpiceApp::test_instance();
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    let page_ids = active_report(&app.state)
        .expect("active report")
        .pages()
        .iter()
        .map(|page| page.id())
        .collect::<Vec<_>>();
    let page_to_move = page_ids[1];
    app.state.hardcopy.report_authoring.selected_page = Some(page_to_move);
    app.state.model.workspace.content.report_documents_dirty = false;

    assert!(can_move_selected_page(
        &cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        PageMoveDirection::Earlier
    ));
    assert!(can_move_selected_page(
        &cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        PageMoveDirection::Later
    ));
    move_selected_page(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        PageMoveDirection::Earlier,
    );

    let document = active_report(&app.state).expect("active report");
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
    assert!(app.state.model.workspace.content.report_documents_dirty);
    assert_eq!(
        app.state.hardcopy.report_authoring.selected_page,
        Some(page_to_move)
    );

    app.state.model.workspace.content.report_documents_dirty = false;
    move_selected_page(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        PageMoveDirection::Later,
    );
    let document = active_report(&app.state).expect("active report");
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
    assert!(app.state.model.workspace.content.report_documents_dirty);
}

#[test]
fn report_page_order_controls_fail_closed_at_document_boundaries() {
    let mut app = RSpiceApp::test_instance();
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    let first_page = active_report(&app.state).expect("active report").pages()[0].id();
    app.state.hardcopy.report_authoring.selected_page = Some(first_page);
    app.state.model.workspace.content.report_documents_dirty = false;
    let revision = active_report(&app.state).expect("active report").revision();

    assert!(!can_move_selected_page(
        &cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        PageMoveDirection::Earlier
    ));
    move_selected_page(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        PageMoveDirection::Earlier,
    );

    assert_eq!(
        active_report(&app.state).expect("active report").revision(),
        revision
    );
    assert!(!app.state.model.workspace.content.report_documents_dirty);
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
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));

    for kind_index in 0..=6 {
        app.state
            .hardcopy
            .report_authoring
            .reset_add_report_element_kind(kind_index);
        assert!(
            app.state
                .hardcopy
                .report_authoring
                .valid_add_report_element_draft(true)
        );
        commit_add_report_element(&mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ));
        assert!(
            app.state
                .hardcopy
                .report_authoring
                .transaction_error
                .is_none()
        );
    }

    let document = active_report(&app.state).expect("active report");
    let page_id = app
        .state
        .hardcopy
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
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    app.state
        .hardcopy
        .report_authoring
        .reset_add_report_element_kind(0);
    commit_add_report_element(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    let document_id = active_report(&app.state).expect("active report").id();
    let block_id = app
        .state
        .hardcopy
        .report_authoring
        .selected_report_block
        .expect("new block selected");

    set_report_block_enabled(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        document_id,
        block_id,
        false,
    );
    assert!(
        !active_report(&app.state)
            .expect("active report")
            .block(block_id)
            .expect("block")
            .enabled()
    );
    app.state.hardcopy.report_authoring.remove_report_block_open = true;
    commit_remove_report_block(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    let document = active_report(&app.state).expect("active report");
    assert!(document.block(block_id).is_none());
    assert!(
        document
            .tombstones()
            .iter()
            .any(|tombstone| tombstone.entity == ReportEntityRef::Block(block_id))
    );
    assert_eq!(
        app.state.hardcopy.report_authoring.selected_report_block,
        None
    );
}

#[test]
fn publication_inspector_edits_are_canonical_document_transactions() {
    let mut app = RSpiceApp::test_instance();
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    let document_id = active_report(&app.state).expect("active report").id();
    let initial_revision = active_report(&app.state).expect("active report").revision();
    let output_formats = ReportOutputFormats {
        pdf_a: true,
        html_bundle: false,
        canonical_json: true,
        selected_csv: true,
    };
    commit_document_publication_setting(
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
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
        &mut cx(
            &mut app.state,
            &mut app.simulation_controller,
            &mut app.model_hub,
        ),
        document_id,
        DocumentPublicationEdit::PublicationProfile(publication_profile),
    );

    let document = active_report(&app.state).expect("active report");
    assert_eq!(document.output_formats(), output_formats);
    assert_eq!(document.publication_profile(), publication_profile);
    assert_eq!(document.revision().get(), initial_revision.get() + 2);
    assert!(app.state.model.workspace.content.report_documents_dirty);
    assert!(
        app.state
            .hardcopy
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
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
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
    app.state
        .model
        .workspace
        .content
        .visualization_documents
        .push(source);

    open_insert_result_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    commit_insert_result_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));

    let block_id = app
        .state
        .hardcopy
        .report_authoring
        .selected_report_block
        .expect("inserted plot selected");
    let block = active_report(&app.state)
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
    assert!(report_reference_resolves(
        &app.state.model,
        &figure.reference
    ));

    fn rendered_binding_states(app: &mut RSpiceApp) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1600.0, 1500.0),
                )),
                ..Default::default()
            },
            |ui| {
                show(
                    &mut cx(
                        &mut app.state,
                        &mut app.simulation_controller,
                        &mut app.model_hub,
                    ),
                    ui,
                )
            },
        );
        let labels: Vec<_> = output
            .platform_output
            .accesskit_update
            .unwrap()
            .nodes
            .iter()
            .filter(|(_, node)| node.role() != egui::accesskit::Role::TextRun)
            .filter_map(|(_, node)| node.label().or_else(|| node.value()))
            .map(str::to_owned)
            .collect();
        let states: Vec<_> = labels
            .iter()
            .filter(|label| matches!(label.as_str(), "bound" | "source missing" | "frozen"))
            .cloned()
            .collect();
        assert!(
            !states.is_empty(),
            "report binding state is not visible: {labels:?}"
        );
        states
    }

    let reference = figure.reference.clone();
    let retained_report = serde_json::to_vec(active_report(&app.state).unwrap()).unwrap();
    assert_eq!(rendered_binding_states(&mut app), ["bound"]);
    let original_source = app.state.model.workspace.content.visualization_documents[0].clone();
    let changed_source = &mut app.state.model.workspace.content.visualization_documents[0];
    changed_source
        .transact(
            changed_source.revision(),
            vec![
                crate::results::visualization_document::DocumentEdit::Rename {
                    entity: crate::results::visualization_document::EntityRef::Page(
                        changed_source.pages()[0].id,
                    ),
                    value: "Updated source page".to_owned(),
                },
            ],
        )
        .unwrap();
    assert!(!report_reference_resolves(&app.state.model, &reference));
    assert_eq!(rendered_binding_states(&mut app), ["source missing"]);

    app.state
        .model
        .workspace
        .content
        .visualization_documents
        .clear();
    assert!(!report_reference_resolves(&app.state.model, &reference));
    assert_eq!(rendered_binding_states(&mut app), ["source missing"]);
    app.state
        .model
        .workspace
        .content
        .visualization_documents
        .push(original_source);
    assert!(report_reference_resolves(&app.state.model, &reference));
    assert_eq!(rendered_binding_states(&mut app), ["bound"]);
    assert_eq!(
        serde_json::to_vec(active_report(&app.state).unwrap()).unwrap(),
        retained_report,
        "source changes and rendering must not silently refresh the report's pinned reference"
    );
}

#[test]
fn direct_report_creation_fails_closed_in_read_only_safe_mode() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        "report authoring test".to_owned(),
    );
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();

    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));

    assert!(
        app.state
            .model
            .workspace
            .content
            .report_documents
            .is_empty()
    );
    assert!(!app.state.model.workspace.content.report_documents_dirty);
    assert_eq!(
        app.state
            .hardcopy
            .report_authoring
            .transaction_error
            .as_deref(),
        Some("Report changes are unavailable because the active project is read-only.")
    );
}

#[test]
fn changing_active_run_does_not_mutate_the_project_report_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    app.state.model.simulation.retained.runs = vec![
        crate::state::SimulationRun::new(2),
        crate::state::SimulationRun::new(1),
    ]
    .into();
    let document = active_report(&app.state).expect("report document").clone();
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.model.simulation.view.active_run_idx = Some(1);
    assert_eq!(active_report(&app.state), Some(&document));
}

#[test]
fn invalid_report_page_selection_chooses_a_retained_page() {
    let mut app = RSpiceApp::test_instance();
    app.state.hardcopy.report_authoring.create_document_title = "Verification report".to_owned();
    commit_create_document(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));
    app.state.hardcopy.report_authoring.selected_page =
        Some(crate::results::report_document::ReportPageId::new());

    synchronize_report_selection(&mut cx(
        &mut app.state,
        &mut app.simulation_controller,
        &mut app.model_hub,
    ));

    assert_eq!(
        app.state.hardcopy.report_authoring.selected_page,
        active_report(&app.state)
            .and_then(|document| document.pages().first())
            .map(|page| page.id())
    );
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
    app.state.model.workspace.content.specs = vec![
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
            member_measurements: Vec::new(),
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
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.view.active_run_idx = Some(0);

    let metrics = ReportSummaryMetrics::from_model(&app.state.model);
    assert_eq!(metrics.checks_passing, 1);
    assert_eq!(metrics.checks_total, 2);
    assert_eq!(metrics.pvt_completed, Some(2));
    assert_eq!(metrics.pvt_total, Some(3));
}
