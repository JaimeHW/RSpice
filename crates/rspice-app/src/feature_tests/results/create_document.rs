//! Creating a result document through a whole application: the dialog's
//! family, name and viewer choices, and a commit that the project owns.

use crate::results::visualization_document::ResultDocumentTracking;
use crate::results::visualization_document::TypedValue;
use crate::state::AnalysisResult;
use crate::state::AnalysisResultFamilyMetadata;
use crate::state::AnalysisResultPayload;
use crate::state::AnalysisType;
use crate::state::DigitalEventPointEvidence;
use crate::state::DigitalEventTraceEvidence;
use crate::state::RetainedTrace;
use crate::state::SimulationRun;
use crate::state::SimulationRunLifecycle;
use crate::state::SoaEvaluationEvidence;
use crate::state::SoaParameterEvidence;
use crate::state::SoaRuleVerdictEvidence;
use crate::state::SoaViolationEvidence;
use crate::state::SoaViolationSeverityEvidence;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::create_document::*;
use crate::workbench::state::Workspace;
use crate::workbench::state::WorkspaceDocumentId;
use rspice_results::document_creation::ResultDocumentFamily;
use rspice_results::document_creation::ResultDocumentLayout;
use rspice_results_ui::create_document::{CreateDocumentHost, CreateResultDocumentDialogState};

fn retained_run_with(analysis_type: AnalysisType) -> SimulationRun {
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    let (name, x) = if analysis_type.is_bode_response() {
        ("|V(out)|", vec![1.0, 2.0, 3.0])
    } else if analysis_type.is_raw_frequency_curve() {
        ("V(out)", vec![1.0, 2.0, 3.0])
    } else {
        ("V(out)", vec![0.0, 0.5, 1.0])
    };
    run.analyses.push(
        AnalysisResult::new(11, analysis_type, "analysis").with_waveforms(vec![
            RetainedTrace::new(name, x, vec![0.0, 1.0, 0.0], "#55aaff"),
        ]),
    );
    run
}

#[test]
fn compatible_filter_uses_the_selected_dataset_not_the_active_dataset() {
    let mut app = RSpiceApp::test_instance();
    let transient = retained_run_with(AnalysisType::Transient);
    let transient_id = transient.dataset_id;
    let ac = retained_run_with(AnalysisType::Ac);
    let ac_id = ac.dataset_id;
    app.state.model.simulation.retained.runs = vec![transient, ac].into();

    assert_eq!(
        first_compatible_viewer(
            app.state.results_reads(),
            Some(transient_id),
            ResultDocumentFamily::WaveformWorksheet,
        ),
        Some("viewer-waveform")
    );
    assert_eq!(
        first_compatible_viewer(
            app.state.results_reads(),
            Some(ac_id),
            ResultDocumentFamily::FrequencyAndStability,
        ),
        Some("viewer-bode")
    );
}

#[test]
fn explicit_nondefault_viewer_choice_is_creatable_and_resolves_unchanged() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    let family = ResultDocumentFamily::WaveformWorksheet;
    assert!(viewer_is_creatable(
        app.state.results_reads(),
        Some(dataset_id),
        family,
        "eye-viewer",
    ));
    let draft = CreateResultDocumentDialogState {
        open: true,
        name: "Eye review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: family.id().to_owned(),
        viewer_id: "eye-viewer".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };

    let resolved =
        resolve_draft(app.state.results_reads(), &draft).expect("explicit viewer resolves");

    assert_eq!(resolved.viewer.id, "eye-viewer");
}

#[test]
fn untouched_default_name_follows_the_selected_family() {
    let mut app = RSpiceApp::test_instance();
    open(&mut app.state.results_parts());
    let mut draft = app.state.results.create_result_document.clone();
    let family = ResultDocumentFamily::FrequencyAndStability;

    CreateDocumentSources(app.state.results_reads()).select_family(&mut draft, family);

    assert_eq!(draft.name, format!("{} · 01", family.label()));
    assert!(!draft.name_touched);
}

#[test]
fn typed_name_survives_a_family_change() {
    let mut app = RSpiceApp::test_instance();
    open(&mut app.state.results_parts());
    let mut draft = app.state.results.create_result_document.clone();
    draft.name = "Loop gain review".to_owned();
    draft.name_touched = true;

    CreateDocumentSources(app.state.results_reads())
        .select_family(&mut draft, ResultDocumentFamily::FrequencyAndStability);

    assert_eq!(draft.name, "Loop gain review");
    assert_eq!(
        draft.family_id,
        ResultDocumentFamily::FrequencyAndStability.id()
    );
    assert!(draft.name_touched);
}

#[test]
fn commit_is_project_owned_dirty_and_stably_selected() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
        validation_error: None,
    };

    let document_id = commit(&mut app.state.results_parts()).expect("document commits");

    assert!(
        app.state
            .model
            .workspace
            .content
            .visualization_documents_dirty
    );
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("project owns created document");
    assert_eq!(document.title(), "Transient review");
    assert_eq!(document.panes().len(), 2);
    assert_eq!(document.axes().len(), 4);
    assert_eq!(document.traces().len(), 2);
    assert!(document.traces().iter().all(|trace| {
        trace.label == "V(out)"
            && trace.coordinate_key == "x"
            && trace.signal_key == "y"
            && trace.row_predicates.len() == 4
    }));
    assert_eq!(document.tracking(), ResultDocumentTracking::pinned());
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&WorkspaceDocumentId::VisualizationDocument(document_id))
    );
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Results
    );
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Waves
    );
}

#[test]
fn release_target_table_commits_from_typed_operating_point_without_waveforms() {
    let mut app = RSpiceApp::test_instance();
    let mut analysis = AnalysisResult::new(11, AnalysisType::DcOp, "Operating point");
    analysis.dc_op = Some(crate::state::DcOpResult {
        node_voltages: vec![crate::state::OperatingPointValue {
            name: "V(out)".to_owned(),
            value: 1.234_567_890_123_456,
            unit: "V".to_owned(),
        }],
        branch_currents: Vec::new(),
        power_dissipation: Vec::new(),
    });
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(analysis);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Typed OP datasheet".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::StatisticsAndYield.id().to_owned(),
        viewer_id: "viewer-table".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };

    let document_id = commit(&mut app.state.results_parts()).expect("typed OP table commits");
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("workspace owns typed table");
    assert_eq!(document.panes()[0].viewer_id, "viewer-table");
    assert_eq!(document.axes().len(), 2);
    assert!(document.traces().is_empty());
    assert!(
        app.state.model.simulation.retained.runs[0].analyses[0]
            .waveforms
            .is_empty()
    );
}

#[test]
fn typed_release_target_campaign_viewers_commit_with_embedded_source_evidence() {
    let events = AnalysisResult::new(21, AnalysisType::Transient, "TRAN events")
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: vec![DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![
                    DigitalEventPointEvidence {
                        time_s: 0.0,
                        value_code: 0,
                    },
                    DigitalEventPointEvidence {
                        time_s: 1.0e-9,
                        value_code: 1,
                    },
                ],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        });
    let soa_time = vec![0.0, 1.0e-9];
    let soa = AnalysisResult::new(22, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: soa_time.clone(),
        })
        .with_waveforms(vec![
            RetainedTrace::new(
                "SOA_VIOLATION_COUNT",
                soa_time.clone(),
                vec![0.0, 0.0],
                "#ffbd2e",
            ),
            RetainedTrace::new(
                crate::results::safety::soa_stress_waveform_name(
                    "M1",
                    crate::results::safety::SoAParameter::Vds,
                ),
                soa_time,
                vec![2.0, 3.0],
                "#00aaff",
            ),
        ])
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        });
    let optimization = AnalysisResult::new(23, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            best_objectives: Vec::new(),
            best_constraints: Vec::new(),
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        .with_waveforms(vec![
            RetainedTrace::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
            RetainedTrace::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
        ]);

    for (analysis, family, viewer_id, evidence_kind) in [
        (
            events,
            ResultDocumentFamily::DigitalAndAmsEvents,
            "viewer-digital-events",
            "analysis-result-payload",
        ),
        (
            soa,
            ResultDocumentFamily::VerificationAndOptimization,
            "viewer-soa",
            "analysis-result-payload",
        ),
        (
            optimization,
            ResultDocumentFamily::VerificationAndOptimization,
            "viewer-optimization",
            "analysis-family-metadata",
        ),
    ] {
        let mut app = RSpiceApp::test_instance();
        let mut run = SimulationRun::new(31);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(analysis);
        let dataset_id = run.dataset_id;
        app.state.model.simulation.retained.runs = vec![run].into();
        app.state.results.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: format!("{viewer_id} review"),
            name_touched: true,
            dataset_id: Some(dataset_id),
            family_id: family.id().to_owned(),
            viewer_id: viewer_id.to_owned(),
            layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
            validation_error: None,
        };

        let document_id =
            commit(&mut app.state.results_parts()).expect("typed release-target viewer commits");
        let document = app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("workspace owns typed document");
        assert_eq!(document.panes()[0].viewer_id, viewer_id);
        assert!(document.datasets()[0].rows().iter().any(|row| {
            matches!(row.values().get(2), Some(TypedValue::Text(kind)) if kind == evidence_kind)
        }));
    }
}

#[test]
fn duplicate_name_revalidation_leaves_project_unchanged() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    let draft = CreateResultDocumentDialogState {
        open: true,
        name: "Transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };
    app.state.results.create_result_document = draft.clone();
    commit(&mut app.state.results_parts()).expect("first document commits");
    app.state.results.create_result_document = draft;
    let before = app
        .state
        .model
        .workspace
        .content
        .visualization_documents
        .len();

    assert!(commit(&mut app.state.results_parts()).is_err());
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .visualization_documents
            .len(),
        before
    );
}

/// The Results workspace of a whole application, drawn frame by frame with
/// AccessKit on, over one retained transient run and an open project.
#[cfg(not(target_arch = "wasm32"))]
struct ResultsWindow {
    ctx: egui::Context,
    app: RSpiceApp,
    frame: eframe::Frame,
    time: f64,
    dataset_id: crate::product::DatasetId,
}

#[cfg(not(target_arch = "wasm32"))]
impl ResultsWindow {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let mut run = retained_run_with(AnalysisType::Transient);
        // A project saves a run only once its provenance is classified.
        run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
            .expect("the fixture run is explicitly legacy");
        let dataset_id = run.dataset_id;
        app.state.model.simulation.retained.runs = vec![run].into();
        assert!(app.state.model.simulation.select_run(0));
        app.state.workbench.activate(Workspace::Results);
        Self {
            ctx,
            app,
            frame: eframe::Frame::_new_kittest(),
            time: 0.0,
            dataset_id,
        }
    }

    /// Three frames, the first carrying `events`; the last one's nodes.
    fn frames(
        &mut self,
        mut events: Vec<egui::Event>,
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let mut nodes = Vec::new();
        for _ in 0..3 {
            self.time += 0.1;
            let (app, frame) = (&mut self.app, &mut self.frame);
            let output = self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::pos2(0.0, 0.0),
                        egui::vec2(1440.0, 900.0),
                    )),
                    time: Some(self.time),
                    focused: true,
                    events: std::mem::take(&mut events),
                    ..Default::default()
                },
                |ui| eframe::App::ui(app, ui, frame),
            );
            nodes = output
                .platform_output
                .accesskit_update
                .map(|update| update.nodes)
                .unwrap_or_default();
        }
        nodes
    }

    /// Create a two-pane waveform document named `name` over the run, the
    /// way the Create result document form commits it.
    fn create(
        &mut self,
        name: &str,
    ) -> Result<crate::product::ResultDocumentId, CreateResultDocumentError> {
        self.app.state.results.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: name.to_owned(),
            name_touched: true,
            dataset_id: Some(self.dataset_id),
            family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
            validation_error: None,
        };
        commit(&mut self.app.state.results_parts())
    }

    /// Save the whole project to a fresh file, as Save all does.
    fn save(&mut self) {
        use crate::workbench::lifecycle::project_lifecycle::{
            DestinationAuthority, SaveScope, save_native,
        };
        let folder = std::env::temp_dir().join(format!(
            "rspice-result-document-marks-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ));
        std::fs::create_dir_all(&folder).expect("create the test folder");
        save_native(
            &mut self.app.state,
            SaveScope::AllDocuments,
            &folder.join("marks.rspiceproj"),
            DestinationAuthority::UserSelected,
        )
        .expect("save the project");
        let _ = std::fs::remove_dir_all(&folder);
    }

    fn project_unsaved(&self) -> bool {
        crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&self.app.state)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn drawn(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> bool {
    nodes
        .iter()
        .any(|(_, node)| node.role() == role && node.label() == Some(label))
}

/// The document bar's tabs left to right, by where the strip drew them. Its
/// tabs are the ones that say how they close; the viewer and console tabs
/// say nothing.
#[cfg(not(target_arch = "wasm32"))]
fn document_tabs(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let mut tabs = nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Tab
                && node.description().is_some_and(|description| {
                    description.ends_with("Press Delete to close.")
                        || description == "Pinned primary document."
                        || description == "Unsaved changes."
                })
        })
        .filter_map(|(id, node)| Some((node.bounds()?.x0, *id, node.clone())))
        .collect::<Vec<_>>();
    tabs.sort_by(|left, right| left.0.total_cmp(&right.0));
    tabs.into_iter().map(|(_, id, node)| (id, node)).collect()
}

/// Each drawn document tab's label, and whether it is marked unsaved.
#[cfg(not(target_arch = "wasm32"))]
fn tab_marks(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]) -> Vec<(String, bool)> {
    document_tabs(nodes)
        .iter()
        .map(|(_, node)| {
            (
                node.label().unwrap_or_default().to_owned(),
                node.description()
                    .is_some_and(|description| description.starts_with("Unsaved changes")),
            )
        })
        .collect()
}

/// What the Results workspace draws, through whole frames of the
/// application, once a result document is created: the stage draws the
/// document, the document bar gives it a tab after the dataset's, and the
/// toolbar's document control agrees with the stage in the same frame.
/// Closing the tab keeps the saved document and puts the dataset's own viewer
/// back, and the toolbar's control goes with it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_created_document_is_drawn_and_its_tab_closes_back_to_the_dataset() {
    use egui::accesskit::{Action, ActionRequest, Role, TreeId};
    use egui::{Event, Key, Modifiers};

    // What only each surface draws: the stage's bar offers to create a
    // document over a dataset's viewer and to add a pane over a document, and
    // the toolbar's tracking control exists only for a document. No plan
    // produced this run, so the control reads Historical.
    const DATASET_STAGE: &str = "Create a dataset-bound result document";
    const DOCUMENT_STAGE: &str = "Add a pane to this result document";
    const DOCUMENT_TOOLBAR: &str = "Historical";

    let mut window = ResultsWindow::new();
    let nodes = window.frames(Vec::new());
    assert!(drawn(&nodes, Role::Button, DATASET_STAGE));
    assert!(!drawn(&nodes, Role::Button, DOCUMENT_TOOLBAR));

    let document_id = window.create("Transient review").expect("document commits");
    let nodes = window.frames(Vec::new());

    assert!(
        drawn(&nodes, Role::Button, DOCUMENT_STAGE),
        "the stage draws the created document"
    );
    assert!(
        !drawn(&nodes, Role::Button, DATASET_STAGE),
        "the stage still draws the dataset's viewer"
    );
    assert!(
        drawn(&nodes, Role::Button, DOCUMENT_TOOLBAR),
        "the toolbar's document control is missing beside the document"
    );
    let strip = document_tabs(&nodes);
    let labels = strip
        .iter()
        .map(|(_, node)| node.label().unwrap_or_default())
        .collect::<Vec<_>>();
    let dataset_label = window.app.state.model.simulation.retained.runs[0]
        .label
        .clone();
    assert_eq!(labels, [dataset_label.as_str(), "Transient review"]);
    let (document_tab, tab) = &strip[1];
    assert_eq!(
        tab.is_selected(),
        Some(true),
        "the document's tab is not the active one"
    );

    window.frames(vec![Event::AccessKitActionRequest(ActionRequest {
        action: Action::Focus,
        target_tree: TreeId::ROOT,
        target_node: *document_tab,
        data: None,
    })]);
    let nodes = window.frames(vec![Event::Key {
        key: Key::Delete,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);

    assert!(
        window
            .app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .is_some(),
        "closing the tab deleted the saved document"
    );
    assert!(
        drawn(&nodes, Role::Button, DATASET_STAGE),
        "the stage does not draw the dataset's viewer after the close"
    );
    assert!(!drawn(&nodes, Role::Button, DOCUMENT_STAGE));
    assert!(
        !drawn(&nodes, Role::Button, DOCUMENT_TOOLBAR),
        "the toolbar keeps the closed document's control"
    );
    // The dataset is the one document left, and a strip of one is not drawn.
    assert!(
        document_tabs(&nodes).is_empty(),
        "the closed document still has a tab"
    );
}

/// An edit to one of two saved documents marks that document's tab and not
/// the other's, and saving the project clears the mark.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_edit_marks_only_its_own_document_tab_until_the_project_is_saved() {
    let mut window = ResultsWindow::new();
    let first = window.create("Review A").expect("A commits");
    window.create("Review B").expect("B commits");
    window.save();
    let dataset = window.app.state.model.simulation.retained.runs[0]
        .label
        .clone();
    assert_eq!(
        tab_marks(&window.frames(Vec::new())),
        [
            (dataset.clone(), false),
            ("Review A".to_owned(), false),
            ("Review B".to_owned(), false),
        ]
    );

    let content = &mut window.app.state.model.workspace.content;
    let document = content.visualization_document(first).expect("A is saved");
    let (revision, page) = (document.revision(), document.pages()[0].id);
    content
        .transact_visualization_document(
            first,
            revision,
            vec![
                crate::results::visualization_document::DocumentEdit::Rename {
                    entity: crate::results::visualization_document::EntityRef::Page(page),
                    value: "Settling".to_owned(),
                },
            ],
        )
        .expect("A takes the edit");
    assert!(window.project_unsaved());
    assert_eq!(
        tab_marks(&window.frames(Vec::new())),
        [
            (dataset.clone(), false),
            ("Review A".to_owned(), true),
            ("Review B".to_owned(), false),
        ],
        "only the edited document's tab is marked"
    );

    window.save();
    assert!(!window.project_unsaved());
    assert_eq!(
        tab_marks(&window.frames(Vec::new())),
        [
            (dataset, false),
            ("Review A".to_owned(), false),
            ("Review B".to_owned(), false),
        ],
        "saving leaves a mark behind"
    );
}

/// An expression trace edits the results session's presentation, not a saved
/// document: no document's tab is marked, and the project still reads
/// unsaved.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_expression_trace_marks_no_document_tab_but_leaves_the_project_unsaved() {
    let mut window = ResultsWindow::new();
    window.create("Review A").expect("A commits");
    window.save();
    assert!(!window.project_unsaved());

    let state = &mut window.app.state;
    state.results.calculator_panel.expression = "V(out)*2".to_owned();
    crate::workbench::results::calculator::plot_expression(
        &mut state.results,
        &mut state.model,
        &mut state.workbench,
    );
    assert_eq!(state.prompt_text(), Some("Plotted V(out)*2"));

    assert!(window.project_unsaved(), "the plotted trace is not unsaved");
    assert!(
        tab_marks(&window.frames(Vec::new()))
            .iter()
            .all(|(_, marked)| !marked),
        "an expression trace marked a document tab"
    );
}

/// A create refused after it reached the catalog is rolled back whole: the
/// document is gone and no tab carries a mark for it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_rolled_back_create_leaves_no_mark() {
    let mut window = ResultsWindow::new();
    window.create("Review A").expect("A commits");
    window.save();

    // The Results route with another workspace showing: the new document
    // reaches the catalog and is then refused when it cannot be shown.
    window.app.state.workbench.workspace = Workspace::Design;
    assert!(window.create("Review B").is_err());
    let content = &window.app.state.model.workspace.content;
    assert_eq!(content.visualization_documents.len(), 1);
    assert!(content.visualization_documents_modified.is_empty());
    assert!(!content.visualization_documents_dirty);

    window.app.state.workbench.activate(Workspace::Results);
    let dataset = window.app.state.model.simulation.retained.runs[0]
        .label
        .clone();
    assert_eq!(
        tab_marks(&window.frames(Vec::new())),
        [(dataset, false), ("Review A".to_owned(), false)]
    );
}
