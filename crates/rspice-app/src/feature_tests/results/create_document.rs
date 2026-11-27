//! Creating a result document through a whole application: what the New
//! result document form opens on and offers, and a commit the project owns.

use super::browser_rows;
use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_results::workbench::documents::result_document::create_document::*;
use rspice_app_state::results::visualization_document::ResultDocumentTracking;
use rspice_app_state::state::AnalysisResult;
use rspice_app_state::state::AnalysisResultFamilyMetadata;
use rspice_app_state::state::AnalysisResultPayload;
use rspice_app_state::state::AnalysisType;
use rspice_app_state::state::DigitalEventPointEvidence;
use rspice_app_state::state::DigitalEventTraceEvidence;
use rspice_app_state::state::RetainedTrace;
use rspice_app_state::state::SimulationRun;
use rspice_app_state::state::SimulationRunLifecycle;
use rspice_app_state::state::SoaEvaluationEvidence;
use rspice_app_state::state::SoaParameterEvidence;
use rspice_app_state::state::SoaRuleVerdictEvidence;
use rspice_app_state::state::SoaViolationEvidence;
use rspice_app_state::state::SoaViolationSeverityEvidence;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;
use rspice_results::document_creation::ResultDocumentLayout;
use rspice_results_ui::create_document::CreateResultDocumentDialogState;

fn retained_run_with(analysis_type: AnalysisType) -> SimulationRun {
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses
        .push(analysis_of(11, analysis_type, "analysis"));
    run
}

/// An analysis of `analysis_type` holding V(in) and V(out) over three
/// points, labelled `label`.
fn analysis_of(id: u64, analysis_type: AnalysisType, label: &str) -> AnalysisResult {
    let (names, x) = if analysis_type.is_bode_response() {
        (["|V(in)|", "|V(out)|"], vec![1.0, 2.0, 3.0])
    } else if analysis_type.is_raw_frequency_curve() {
        (["V(in)", "V(out)"], vec![1.0, 2.0, 3.0])
    } else {
        (["V(in)", "V(out)"], vec![0.0, 0.5, 1.0])
    };
    AnalysisResult::new(id, analysis_type, label).with_waveforms(
        names
            .iter()
            .map(|name| RetainedTrace::new(*name, x.clone(), vec![0.0, 1.0, 0.0], "#55aaff"))
            .collect(),
    )
}

/// The form over the analysis at `analysis` of `run`, drawn on `viewer` in
/// `layout`, plotting `signals`, under a name the reader typed.
fn draft(
    name: &str,
    run: &SimulationRun,
    analysis: usize,
    viewer: &str,
    layout: ResultDocumentLayout,
    signals: &[&str],
) -> CreateResultDocumentDialogState {
    CreateResultDocumentDialogState {
        open: true,
        name: name.to_owned(),
        name_touched: true,
        dataset_id: Some(run.dataset_id),
        analysis: Some(analysis),
        viewer_id: viewer.to_owned(),
        layout,
        signals: signals.iter().map(|signal| (*signal).to_owned()).collect(),
        validation_error: None,
    }
}

/// A finished run holding a transient and, second, an AC analysis.
fn transient_and_ac_run() -> SimulationRun {
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses
        .push(analysis_of(11, AnalysisType::Transient, "Step response"));
    run.analyses
        .push(analysis_of(12, AnalysisType::Ac, "Loop gain"));
    run
}

#[test]
fn the_sheets_offered_are_the_ones_that_draw_the_chosen_analysis() {
    let mut app = RSpiceApp::test_instance();
    let run = transient_and_ac_run();
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    let sheets = |analysis: usize| -> Vec<&'static str> {
        let form = draft(
            "x",
            &run,
            analysis,
            "",
            ResultDocumentLayout::SinglePane,
            &[],
        );
        choices(app.state.results_reads(), &form)
            .viewers
            .into_iter()
            .map(|(id, _)| id)
            .collect()
    };

    let transient = sheets(0);
    assert!(transient.contains(&"viewer-waveform"), "{transient:?}");
    assert!(!transient.contains(&"viewer-bode"), "{transient:?}");
    let ac = sheets(1);
    assert!(ac.contains(&"viewer-bode"), "{ac:?}");
    assert!(!ac.contains(&"viewer-waveform"), "{ac:?}");
}

/// A PRBS-7 bit stream of `bits` bits, a nanosecond each, whose edges take
/// the first fifth of a bit, sampled twenty times a bit.
fn bit_stream(bits: usize) -> (Vec<f64>, Vec<f64>) {
    const BIT: f64 = 1e-9;
    const SAMPLES: usize = 20;
    let mut register: u8 = 0x7f;
    let levels: Vec<f64> = (0..bits)
        .map(|_| {
            let bit = ((register >> 6) ^ (register >> 5)) & 1;
            register = ((register << 1) | bit) & 0x7f;
            f64::from(bit)
        })
        .collect();
    let mut time = Vec::with_capacity(bits * SAMPLES);
    let mut value = Vec::with_capacity(bits * SAMPLES);
    for (index, &level) in levels.iter().enumerate() {
        let from = levels[index.saturating_sub(1)];
        for sample in 0..SAMPLES {
            let along = sample as f64 / SAMPLES as f64;
            time.push((index as f64 + along) * BIT);
            value.push(from + (level - from) * (along / 0.2).min(1.0));
        }
    }
    (time, value)
}

/// The eye is offered only for a transient it folds, as its sheet folds one
/// before it draws: a bit stream, never a step response, whose eye would
/// only say it has no usable source; and the eye chosen is the sheet made.
#[test]
fn the_eye_is_offered_only_for_a_transient_it_folds() {
    let mut app = RSpiceApp::test_instance();
    let (time, data) = bit_stream(300);
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(11, AnalysisType::Transient, "Link").with_waveforms(vec![
            RetainedTrace::new("V(data)", time, data, String::new()),
        ]),
    );
    run.analyses
        .push(analysis_of(12, AnalysisType::Transient, "Step response"));
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    let eye = |analysis: usize| {
        draft(
            "Eye review",
            &run,
            analysis,
            "eye-viewer",
            ResultDocumentLayout::SinglePane,
            &[],
        )
    };
    let offered = |analysis: usize| {
        choices(app.state.results_reads(), &eye(analysis))
            .viewers
            .iter()
            .any(|(id, _)| *id == "eye-viewer")
    };

    assert!(offered(0), "the bit stream's eye is offered");
    assert!(!offered(1), "the step response's eye is not");
    let bit_stream_eye = eye(0);
    let resolved = resolve_draft(app.state.results_reads(), &bit_stream_eye)
        .expect("the bit stream's eye resolves");
    assert_eq!(resolved.viewer.id, "eye-viewer");
    assert!(
        resolve_draft(app.state.results_reads(), &eye(1)).is_err(),
        "the step response's eye is refused"
    );
}

/// The form opens on what Results shows: the run, the analysis the data
/// browser picked (not the run's first), the sheet on the canvas, and the
/// signals plotted, named for the run and the analysis.
#[test]
fn the_form_opens_on_the_run_analysis_and_sheet_results_shows() {
    let mut app = RSpiceApp::test_instance();
    let older = retained_run_with(AnalysisType::Transient);
    let shown = transient_and_ac_run();
    app.state.model.simulation.retained.runs = vec![older, shown.clone()].into();
    assert!(app.state.model.simulation.select_run_analysis(1, 1));
    app.state.results.results.session.viewer = ResultViewer::Nyquist;

    open(&mut app.state.results_parts());

    let form = &app.state.results.create_result_document;
    assert!(form.open);
    assert_eq!(form.dataset_id, Some(shown.dataset_id));
    assert_eq!(form.analysis, Some(1));
    assert_eq!(form.viewer_id, "viewer-bode");
    assert_eq!(form.signals, ["|V(in)|", "|V(out)|"]);
    assert_eq!(form.name, format!("{} \u{2014} Loop gain", shown.name()));
    assert!(!form.name_touched);
}

/// A second document over the same analysis is offered the same name with
/// the first free number after it, never a bare counter.
#[test]
fn the_default_name_says_what_it_shows_and_is_never_taken() {
    let mut app = RSpiceApp::test_instance();
    let run = transient_and_ac_run();
    let base = format!("{} \u{2014} Step response", run.name());
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run_analysis(0, 0));

    open(&mut app.state.results_parts());
    assert_eq!(app.state.results.create_result_document.name, base);
    commit(&mut app.state.results_parts()).expect("the first document commits");
    open(&mut app.state.results_parts());

    assert_eq!(
        app.state.results.create_result_document.name,
        format!("{base} 2")
    );
    commit(&mut app.state.results_parts()).expect("the offered name is free");
}

/// Choosing another analysis moves the sheet to one that draws it, ticks
/// its plotted signals, and renames an untouched name; a typed name stays.
#[test]
fn choosing_another_analysis_follows_with_sheet_signals_and_an_untouched_name() {
    let mut app = RSpiceApp::test_instance();
    let run = transient_and_ac_run();
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    assert!(app.state.model.simulation.select_run_analysis(0, 0));
    open(&mut app.state.results_parts());
    let before = app.state.results.create_result_document.clone();
    assert_eq!(before.viewer_id, "viewer-waveform");

    let mut form = before.clone();
    form.analysis = Some(1);
    reconcile(app.state.results_reads(), &before, &mut form);
    assert_eq!(form.viewer_id, "viewer-bode");
    assert_eq!(form.signals, ["|V(in)|", "|V(out)|"]);
    assert_eq!(form.name, format!("{} \u{2014} Loop gain", run.name()));

    let mut typed = before.clone();
    typed.name = "Loop gain review".to_owned();
    typed.name_touched = true;
    let held = typed.clone();
    typed.analysis = Some(1);
    reconcile(app.state.results_reads(), &held, &mut typed);
    assert_eq!(typed.name, "Loop gain review");
}

/// The document's panes plot the signals ticked in the form and no other;
/// with none ticked the pane plots nothing until the reader adds a signal.
#[test]
fn the_document_plots_the_signals_ticked_in_the_form() {
    let mut app = RSpiceApp::test_instance();
    let run = transient_and_ac_run();
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    let plotted = |app: &mut RSpiceApp, name: &str, ticked: &[&str]| {
        app.state.results.create_result_document = draft(
            name,
            &run,
            0,
            "viewer-waveform",
            ResultDocumentLayout::SinglePane,
            ticked,
        );
        let document_id = commit(&mut app.state.results_parts()).expect("document commits");
        app.state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project owns it")
            .traces()
            .iter()
            .map(|trace| trace.label.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(plotted(&mut app, "Inputs", &["V(in)"]), ["V(in)"]);
    assert_eq!(plotted(&mut app, "Empty", &[]), Vec::<String>::new());
}

/// Several panes spread the ticked signals by unit, as a run opens with a
/// strip per unit: the voltages fill the first pane and the currents the
/// next, in the order the page lays them out, and in four panes the two
/// past the units start empty: top left, top right, then the bottom row.
#[test]
fn panes_spread_the_ticked_signals_by_unit() {
    let mut app = RSpiceApp::test_instance();
    let signals = ["V(in)", "I(R1)", "V(out)", "I(C1)"];
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(11, AnalysisType::Transient, "Step response").with_waveforms(
            signals
                .iter()
                .map(|name| {
                    RetainedTrace::new(
                        *name,
                        vec![0.0, 0.5, 1.0],
                        vec![0.0, 1.0, 0.0],
                        String::new(),
                    )
                })
                .collect(),
        ),
    );
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    let mut panes = |layout: ResultDocumentLayout| {
        app.state.results.create_result_document =
            draft(layout.label(), &run, 0, "viewer-waveform", layout, &signals);
        let document_id = commit(&mut app.state.results_parts()).expect("document commits");
        let document = app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project owns it");
        let mut ordered = document.panes().iter().collect::<Vec<_>>();
        ordered.sort_by_key(|pane| pane.order);
        ordered
            .into_iter()
            .map(|pane| {
                document
                    .traces()
                    .iter()
                    .filter(|trace| trace.pane_id == pane.id)
                    .map(|trace| trace.label.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(
        panes(ResultDocumentLayout::TwoLinkedPanes),
        [vec!["V(in)", "V(out)"], vec!["I(R1)", "I(C1)"]]
    );
    assert_eq!(
        panes(ResultDocumentLayout::EngineeringGrid),
        [
            vec!["V(in)", "V(out)"],
            vec!["I(R1)", "I(C1)"],
            Vec::new(),
            Vec::new()
        ]
    );
}

#[test]
fn commit_is_project_owned_dirty_and_stably_selected() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    app.state.results.create_result_document = draft(
        "Transient review",
        &run,
        0,
        "viewer-waveform",
        ResultDocumentLayout::TwoLinkedPanes,
        &["V(out)"],
    );

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
    // One unit ticked: the first pane plots it and the second starts empty.
    assert_eq!(document.traces().len(), 1);
    assert!(document.traces().iter().all(|trace| {
        trace.pane_id == document.panes()[0].id
            && trace.label == "V(out)"
            && &*trace.coordinate_key == "x"
            && &*trace.signal_key == "y"
            && trace.row_predicates.len() == 3
    }));
    assert_eq!(document.tracking(), ResultDocumentTracking::pinned());
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&WorkspaceDocumentId::VisualizationDocument(document_id))
    );
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::Results
    );
    assert_eq!(
        app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
}

#[test]
fn release_target_table_commits_from_typed_operating_point_without_waveforms() {
    let mut app = RSpiceApp::test_instance();
    let mut analysis = AnalysisResult::new(11, AnalysisType::DcOp, "Operating point");
    analysis.dc_op = Some(rspice_app_state::state::DcOpResult {
        node_voltages: vec![rspice_app_state::state::OperatingPointValue {
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
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    app.state.results.create_result_document = draft(
        "Typed OP datasheet",
        &run,
        0,
        "viewer-table",
        ResultDocumentLayout::SinglePane,
        &[],
    );

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
                ]
                .into(),
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
            hdl_sources: Vec::new(),
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
                rspice_app_state::results::safety::soa_stress_waveform_name(
                    "M1",
                    rspice_app_state::results::safety::SoAParameter::Vds,
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
    // A `.STEP` held in one analysis: one value per point.
    let sweep = AnalysisResult::new(24, AnalysisType::Parametric, "STEP")
        .with_waveforms(vec![RetainedTrace::new(
            "V(out)",
            vec![1e3, 2e3, 4e3],
            vec![0.5, 1.0 / 3.0, 0.2],
            "#00aaff",
        )])
        .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
            target: "PARAM r".to_owned(),
            sweep_values: vec![1e3, 2e3, 4e3],
            failed_points: 0,
            member_measurements: Vec::new(),
        });

    for (analysis, viewer_id) in [
        (events, "viewer-digital-events"),
        (soa, "viewer-soa"),
        (optimization, "viewer-optimization"),
        (sweep, "viewer-sweep"),
    ] {
        let mut app = RSpiceApp::test_instance();
        let mut run = SimulationRun::new(31);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(analysis);
        let dataset_id = run.dataset_id;
        app.state.model.simulation.retained.runs = vec![run.clone()].into();
        app.state.results.create_result_document = draft(
            &format!("{viewer_id} review"),
            &run,
            0,
            viewer_id,
            ResultDocumentLayout::SinglePane,
            &[],
        );

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
        // The document reads the run it was made from; it holds no copy.
        let source = &document.datasets()[0];
        assert_eq!(source.binding().dataset_id, dataset_id);
        assert!(source.rows().is_empty());
        assert!(
            source
                .run_source()
                .is_some_and(|run| run.analyses().len() == 1)
        );
    }
}

#[test]
fn duplicate_name_revalidation_leaves_project_unchanged() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    app.state.model.simulation.retained.runs = vec![run.clone()].into();
    let draft = draft(
        "Transient review",
        &run,
        0,
        "viewer-waveform",
        ResultDocumentLayout::SinglePane,
        &["V(out)"],
    );
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
    dataset_id: rspice_app_state::product::DatasetId,
}

#[cfg(not(target_arch = "wasm32"))]
impl ResultsWindow {
    fn new() -> Self {
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let mut run = retained_run_with(AnalysisType::Transient);
        // A project saves a run only once its provenance is classified.
        run.restore_provenance(
            rspice_app_state::state::SimulationRunProvenance::LegacyUnattributed,
        )
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

    /// One frame carrying `events`.
    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        self.time += 0.1;
        let (app, frame) = (&mut self.app, &mut self.frame);
        self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::pos2(0.0, 0.0),
                    egui::vec2(1440.0, 900.0),
                )),
                time: Some(self.time),
                focused: true,
                events,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, frame),
        )
    }

    /// Three frames, the first carrying `events`; the last one's nodes.
    fn frames(
        &mut self,
        mut events: Vec<egui::Event>,
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let mut nodes = Vec::new();
        for _ in 0..3 {
            nodes = self
                .frame(std::mem::take(&mut events))
                .platform_output
                .accesskit_update
                .map(|update| update.nodes)
                .unwrap_or_default();
        }
        nodes
    }

    /// What egui wrote, over the last of three frames, of a widget id two
    /// widgets used.
    fn id_clashes(&mut self) -> Vec<String> {
        let mut clashes = Vec::new();
        for _ in 0..3 {
            let output = self.frame(Vec::new());
            let mut texts = Vec::new();
            for clipped in &output.shapes {
                painted_texts(&clipped.shape, &mut texts);
            }
            clashes = texts
                .into_iter()
                .filter(|text| text.contains("use of widget ID"))
                .collect();
        }
        clashes
    }

    /// Frames until no pane is still preparing what it derives from the
    /// transient, which the spectrum and the eye work out off the frame; the
    /// last one's nodes.
    fn settled_frames(&mut self) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let preparing = |nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]| {
            nodes.iter().any(|(_, node)| {
                node.label()
                    .is_some_and(|label| label.starts_with("Preparing derived data"))
            })
        };
        let mut nodes = self.frames(Vec::new());
        for _ in 0..1_000 {
            if !preparing(&nodes) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
            nodes = self.frames(Vec::new());
        }
        nodes
    }

    /// Create a two-pane waveform document named `name` over the run, the
    /// way the New result document form commits it.
    fn create(
        &mut self,
        name: &str,
    ) -> Result<rspice_app_state::product::ResultDocumentId, CreateResultDocumentError> {
        self.app.state.results.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: name.to_owned(),
            name_touched: true,
            dataset_id: Some(self.dataset_id),
            analysis: Some(0),
            viewer_id: "viewer-waveform".to_owned(),
            layout: ResultDocumentLayout::TwoLinkedPanes,
            signals: vec!["V(out)".to_owned()],
            validation_error: None,
        };
        commit(&mut self.app.state.results_parts())
    }

    /// Save the whole project to a fresh file, as Save all does. Each save
    /// takes a folder of its own: tests that start together can read the
    /// same clock, and two saves into one folder find each other's lease.
    fn save(&mut self) {
        use rspice_app_services::workbench::lifecycle::project_lifecycle::{
            DestinationAuthority, SaveScope, save_native,
        };
        let folder = std::env::temp_dir().join(format!(
            "rspice-result-document-marks-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
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
        rspice_app_services::workbench::lifecycle::project_lifecycle::has_unsaved_changes(
            &self.app.state,
        )
    }

    /// Click `node`, as AccessKit's default action does; the last frame's
    /// nodes.
    fn click(
        &mut self,
        node: egui::accesskit::NodeId,
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        self.frames(vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: node,
                data: None,
            },
        )]);
        self.frames(Vec::new())
    }

    /// Press and release `button` at `at`, as a mouse does.
    fn press(&mut self, at: egui::Pos2, button: egui::PointerButton) {
        let event = |pressed| egui::Event::PointerButton {
            pos: at,
            button,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frames(vec![egui::Event::PointerMoved(at), event(true)]);
        self.frames(vec![event(false)]);
    }

    /// Choose the button labelled `label`: a menu's item.
    fn choose(&mut self, label: &str) {
        let nodes = self.frames(Vec::new());
        let (button, _) = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .unwrap_or_else(|| panic!("{label} is offered"));
        self.click(*button);
    }

    /// The label the data browser gives the window's run.
    fn run_label(&self) -> String {
        self.app.state.model.simulation.retained.runs[0]
            .label
            .clone()
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

/// Each saved document's row in the data browser, top to bottom: its name,
/// and whether it is marked unsaved.
#[cfg(not(target_arch = "wasm32"))]
fn row_marks(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]) -> Vec<(String, bool)> {
    browser_rows::documents(nodes)
        .into_iter()
        .map(|row| (row.label, row.unsaved))
        .collect()
}

/// What the Results workspace draws, through whole frames of the
/// application, once a result document is created: the stage draws the
/// document, the data browser lists it and its row alone reads as shown, and
/// the toolbar's document control agrees with the stage in the same frame.
/// Picking the run's analysis in the browser keeps the saved document and
/// puts the run's own viewer back, and the toolbar's control goes with it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_created_document_is_drawn_and_its_row_reads_as_shown() {
    use egui::accesskit::Role;

    // What only each surface draws: the stage's bar offers to create a
    // document over a dataset's viewer and to add a pane over a document, and
    // the toolbar's tracking control exists only for a document. No plan
    // produced this run, so the control reads Historical.
    const DATASET_STAGE: &str = "New result document…";
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
    assert_eq!(
        browser_rows::shown(&nodes),
        ["Transient review"],
        "the created document's row is not the one that reads as shown"
    );

    let run = window.run_label();
    let nodes = window.click(browser_rows::analysis_under(&nodes, &run).id);

    assert!(
        window
            .app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .is_some(),
        "picking the run deleted the saved document"
    );
    assert!(
        drawn(&nodes, Role::Button, DATASET_STAGE),
        "the stage does not draw the run's viewer once the run is picked"
    );
    assert!(!drawn(&nodes, Role::Button, DOCUMENT_STAGE));
    assert!(
        !drawn(&nodes, Role::Button, DOCUMENT_TOOLBAR),
        "the toolbar keeps the document's control over the run"
    );
    assert_eq!(
        browser_rows::document_names(&nodes),
        ["Transient review"],
        "the document is still listed"
    );
    assert!(!browser_rows::documents(&nodes)[0].shown);
}

/// The analysis rows of the strips the frame draws, top to bottom and left
/// to right: one per pane of a waveform document.
#[cfg(not(target_arch = "wasm32"))]
fn analysis_rows(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]) -> Vec<egui::Pos2> {
    let mut rows = nodes
        .iter()
        .filter(|(_, node)| node.label() == Some("TR"))
        .filter_map(|(_, node)| node.bounds())
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.y0.total_cmp(&b.y0).then(a.x0.total_cmp(&b.x0)));
    rows.into_iter()
        .map(|bounds| {
            egui::pos2(
                ((bounds.x0 + bounds.x1) * 0.5) as f32,
                ((bounds.y0 + bounds.y1) * 0.5) as f32,
            )
        })
        .collect()
}

/// A press in a document's inactive pane makes it the active pane, and the
/// menu on a pane's analysis row moves the pane and removes it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_press_activates_a_pane_and_its_analysis_row_moves_and_removes_it() {
    use egui::PointerButton;
    use rspice_app_results::workbench::documents::result_document::persistent_document::ordered_page_panes;

    let mut window = ResultsWindow::new();
    let document_id = window.create("Transient review").expect("document commits");
    let panes = |window: &ResultsWindow| {
        let content = &window.app.state.model.workspace.content;
        let document = content
            .visualization_document(document_id)
            .expect("the saved document");
        ordered_page_panes(document.panes(), &document.pages()[0])
            .into_iter()
            .map(|pane| pane.id)
            .collect::<Vec<_>>()
    };
    let active = |window: &ResultsWindow| {
        let session = &window.app.state.results.results.session;
        session.persistent_document_pane(document_id)
    };
    let [first, second] = panes(&window)[..] else {
        panic!("the document has two panes");
    };
    assert_eq!(active(&window), Some(first));

    let rows = analysis_rows(&window.frames(Vec::new()));
    assert_eq!(rows.len(), 2, "each pane draws its analysis row");
    window.press(rows[1], PointerButton::Primary);
    assert_eq!(
        active(&window),
        Some(second),
        "a press in the second pane did not make it active"
    );

    let rows = analysis_rows(&window.frames(Vec::new()));
    window.press(rows[1], PointerButton::Secondary);
    window.choose("Move pane earlier");
    assert_eq!(panes(&window), [second, first]);

    let rows = analysis_rows(&window.frames(Vec::new()));
    window.press(rows[0], PointerButton::Secondary);
    window.choose("Remove pane");
    assert_eq!(panes(&window), [first]);
}

/// An edit to one of two saved documents puts the unsaved mark on that
/// document's row in the data browser and not the other's, and saving the
/// project clears the mark.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_edit_marks_only_its_own_documents_row_until_the_project_is_saved() {
    let mut window = ResultsWindow::new();
    let first = window.create("Review A").expect("A commits");
    window.create("Review B").expect("B commits");
    window.save();
    assert_eq!(
        row_marks(&window.frames(Vec::new())),
        [
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
                rspice_app_state::results::visualization_document::DocumentEdit::Rename {
                    entity: rspice_app_state::results::visualization_document::EntityRef::Page(
                        page,
                    ),
                    value: "Settling".to_owned(),
                },
            ],
        )
        .expect("A takes the edit");
    assert!(window.project_unsaved());
    assert_eq!(
        row_marks(&window.frames(Vec::new())),
        [
            ("Review A".to_owned(), true),
            ("Review B".to_owned(), false),
        ],
        "only the edited document's row is marked"
    );

    window.save();
    assert!(!window.project_unsaved());
    assert_eq!(
        row_marks(&window.frames(Vec::new())),
        [
            ("Review A".to_owned(), false),
            ("Review B".to_owned(), false),
        ],
        "saving leaves a mark behind"
    );
}

/// Press the calculator's Plot on `expression`.
#[cfg(not(target_arch = "wasm32"))]
fn plot(state: &mut crate::workbench::app_state::AppState, expression: &str) {
    state.results.calculator_panel.expression = expression.to_owned();
    let mut panel = std::mem::take(&mut state.results.calculator_panel);
    rspice_app_results::workbench::results::calculator::plot_expression(
        &mut panel,
        &mut state.results_parts(),
    );
    state.results.calculator_panel = panel;
}

/// An expression the calculator plots is kept by the view in front. Over
/// the run the browser picked, the run's working view keeps it: no
/// document's row is marked and the project reads unsaved. Over a saved
/// document that document keeps it, and its row alone is marked.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_plotted_expression_is_kept_by_the_view_in_front() {
    let mut window = ResultsWindow::new();
    let document_id = window.create("Review A").expect("A commits");
    let run = window.run_label();
    let nodes = window.frames(Vec::new());
    window.click(browser_rows::analysis_under(&nodes, &run).id);
    assert!(
        window
            .app
            .state
            .results
            .results
            .session
            .persistent_pane_context
            .is_none(),
        "the picked run shows its own view"
    );
    window.save();
    assert!(!window.project_unsaved());

    plot(&mut window.app.state, "V(out)*2");
    assert_eq!(window.app.state.prompt_text(), Some("Plotted V(out)*2"));
    assert!(window.project_unsaved(), "the plotted trace is not unsaved");
    assert_eq!(
        row_marks(&window.frames(Vec::new())),
        [("Review A".to_owned(), false)],
        "an expression on the run's own view marked a document's row"
    );

    window.save();
    let nodes = window.frames(Vec::new());
    window.click(browser_rows::documents(&nodes)[0].id);
    window.save();
    window
        .app
        .state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    window.frames(Vec::new());
    plot(&mut window.app.state, "V(out)*3");
    assert_eq!(window.app.state.prompt_text(), Some("Plotted V(out)*3"));
    assert!(
        window
            .app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the saved document")
            .expression_traces()
            .iter()
            .any(|trace| trace.text == "V(out)*3"),
        "the document in front keeps the expression"
    );
    assert_eq!(
        row_marks(&window.frames(Vec::new())),
        [("Review A".to_owned(), true)],
        "only the document in front is marked"
    );
}

/// A create refused after it reached the catalog is rolled back whole: the
/// document is gone and no row carries a mark for it.
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
    assert_eq!(
        row_marks(&window.frames(Vec::new())),
        [("Review A".to_owned(), false)]
    );
}

/// An RC low-pass whose resistor runs 1 % per kelvin from 1 kΩ at 27 °C,
/// over five temperatures: its corner sits at 1 kHz at 27 °C and moves with
/// R(T) = 1k·(1 + 0.01·(T − 27)).
const TEMPERATURE_DECK: &str = "* rc low-pass over temperature\nV1 in 0 AC 1\n\
R1 in out 1k TC1=0.01\nC1 out 0 159.155n\n.step temp list -40 0 27 85 125\n\
.ac dec 10 10 100k\n.end\n";

/// The curves a frame's legend lists, by the name each chip toggles.
#[cfg(not(target_arch = "wasm32"))]
fn legend_curves(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
) -> std::collections::BTreeSet<String> {
    nodes
        .iter()
        .filter_map(|(_, node)| {
            node.label()?
                .strip_prefix("Toggle ")?
                .strip_suffix(" visibility")
                .map(str::to_owned)
        })
        .collect()
}

/// The traces a frame's plots draw, as each plot's own label counts them.
#[cfg(not(target_arch = "wasm32"))]
fn drawn_traces(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]) -> usize {
    nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::GraphicsDocument)
        .filter_map(|(_, node)| {
            let (head, _) = node.label()?.split_once(" visible trace")?;
            head.rsplit(' ').next()?.parse::<usize>().ok()
        })
        .sum()
}

/// A Bode document made from a five-temperature AC run draws the five
/// curves the Bode sheet draws, under the same member labels, as ViVA's
/// Plot Across Corners draws every corner: the document reads the run's
/// family through the sheet's own projection. Each member's curve is the
/// netlist's own low-pass at that temperature.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_bode_document_of_a_temperature_run_draws_every_member() {
    use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;
    let mut run = run_manual_batch(TEMPERATURE_DECK);
    // Only the output's magnitude is shown, so the legend lists each of
    // its members as a chip of its own.
    for analysis in &mut run.analyses {
        for trace in &mut analysis.waveforms {
            trace.visible = trace.data.name == "|V(OUT)|";
        }
    }
    let temperatures = [-40.0, 0.0, 27.0, 85.0, 125.0];
    assert_eq!(run.analyses.len(), temperatures.len());
    for (analysis, temperature) in run.analyses.iter().zip(temperatures) {
        let resistance = 1e3 * (1.0 + 0.01 * (temperature - 27.0));
        let magnitude = analysis
            .waveforms
            .iter()
            .map(|waveform| -> &rspice_results::waveform::RetainedWaveform { waveform.as_ref() })
            .find(|waveform| waveform.name == "|V(OUT)|")
            .expect("each member keeps its output's magnitude");
        for sample in 0..magnitude.y.len() {
            let frequency = magnitude.x.sample(sample).expect("a frequency");
            let gain = magnitude.y.sample(sample).expect("a gain");
            let pole = std::f64::consts::TAU * frequency * resistance * 159.155e-9;
            let expected = 1.0 / (1.0 + pole * pole).sqrt();
            assert!(
                (gain - expected).abs() < 1e-6,
                "{temperature} °C at {frequency} Hz: {gain} against {expected}"
            );
        }
    }
    let dataset_id = run.dataset_id;

    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    app.state.workbench.activate(Workspace::Results);
    app.state.results.results.session.viewer =
        rspice_app_results::workbench::documents::result_document::ResultViewer::Bode;
    let mut window = ResultsWindow {
        ctx,
        app,
        frame: eframe::Frame::_new_kittest(),
        time: 0.0,
        dataset_id,
    };
    let output_curves = |nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]| {
        legend_curves(nodes)
            .into_iter()
            .filter(|name| name.starts_with("|V(OUT)|"))
            .collect::<Vec<_>>()
    };
    let nodes = window.frames(Vec::new());
    assert_eq!(
        drawn_traces(&nodes),
        temperatures.len(),
        "the Bode sheet draws every temperature"
    );
    // The legend lists as many members as its row holds and folds the rest
    // into a count; each chip it lists names one member.
    let sheet = output_curves(&nodes);
    let members: Vec<String> = ["−40", "0", "27", "85", "125"]
        .iter()
        .map(|temperature| format!("|V(OUT)| · temp={temperature} °C"))
        .collect();
    assert!(
        !sheet.is_empty() && sheet.iter().all(|chip| members.contains(chip)),
        "the Bode sheet labels each curve by its member: {sheet:?}"
    );

    window.app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Bode over temperature".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        analysis: Some(0),
        viewer_id: "viewer-bode".to_owned(),
        layout: ResultDocumentLayout::SinglePane,
        signals: vec!["|V(OUT)|".to_owned()],
        validation_error: None,
    };
    commit(&mut window.app.state.results_parts()).expect("the document commits");
    let nodes = window.frames(Vec::new());
    assert!(
        drawn(
            &nodes,
            egui::accesskit::Role::Button,
            "Add a pane to this result document"
        ),
        "the stage draws the created document"
    );
    assert_eq!(
        drawn_traces(&nodes),
        temperatures.len(),
        "the document draws every temperature"
    );
    assert_eq!(
        output_curves(&nodes),
        sheet,
        "the document draws the members the sheet drew"
    );
}

/// A name another document has is refused in the form's footer, and Create
/// is disabled: Enter makes nothing and the form stays open.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_taken_name_is_refused_in_the_footer_and_create_is_disabled() {
    use egui::accesskit::Role;
    use egui::{Event, Key, Modifiers};

    let mut window = ResultsWindow::new();
    window
        .create("Transient review")
        .expect("the first document commits");
    open(&mut window.app.state.results_parts());
    {
        let form = &mut window.app.state.results.create_result_document;
        form.name = "transient review".to_owned();
        form.name_touched = true;
    }
    let nodes = window.frames(Vec::new());

    let refusal = "A document named \u{201c}transient review\u{201d} already exists.";
    assert!(
        nodes.iter().any(|(_, node)| {
            [node.label(), node.description(), node.value()].contains(&Some(refusal))
        }),
        "the form states why it refuses"
    );
    let create = nodes
        .iter()
        .find(|(_, node)| node.role() == Role::Button && node.label() == Some("Create"))
        .expect("the form draws Create");
    assert!(create.1.is_disabled(), "Create is disabled");

    let documents = |window: &ResultsWindow| {
        window
            .app
            .state
            .model
            .workspace
            .content
            .visualization_documents
            .len()
    };
    let before = documents(&window);
    window.frames(vec![Event::Key {
        key: Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]);
    assert_eq!(documents(&window), before, "Enter makes nothing");
    assert!(window.app.state.results.create_result_document.open);
}

/// A transient and an AC analysis of one RC low-pass, every signal plotted.
const LOW_PASS_DECK: &str = "* rc low-pass\nV1 in 0 PULSE(0 1 0 1u 1u 50u 100u) AC 1\n\
R1 in out 1k\nC1 out 0 10n\n.tran 1u 200u\n.ac dec 10 10 1meg\n.end\n";
/// An operating point of a divider.
const DIVIDER_DECK: &str = "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n";
/// A DC sweep of the same divider.
const SWEPT_DIVIDER_DECK: &str =
    "* swept divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.dc V1 0 1 0.1\n.end\n";

/// What one pane-full of a document shows once it is drawn: its plots'
/// traces for a sheet of signals, else its table's rows. The inspector's
/// measurement rows are not the document's.
#[cfg(not(target_arch = "wasm32"))]
fn shown_items(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]) -> usize {
    drawn_traces(nodes)
        + nodes
            .iter()
            .filter(|(_, node)| {
                node.role() == egui::accesskit::Role::Row
                    && !node
                        .label()
                        .is_some_and(|label| label.starts_with("Measurement: "))
            })
            .count()
}

/// That a document in `panes` panes, plotting `plotted`, drew the run's
/// data: an item at least in every pane it plots into. Signals spread by
/// unit leave the panes past the units empty; a sheet that plots no
/// signals of its own fills every pane.
#[cfg(not(target_arch = "wasm32"))]
fn assert_drawn(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    panes: usize,
    plotted: &[(usize, String, bool)],
    combination: &str,
) {
    let mut filled: Vec<usize> = plotted
        .iter()
        .filter(|(_, _, visible)| *visible)
        .map(|(pane, _, _)| *pane)
        .collect();
    filled.dedup();
    let filled = if filled.is_empty() {
        panes
    } else {
        filled.len()
    };
    let shown = shown_items(nodes);
    assert!(
        shown >= filled,
        "{combination}: {shown} items drawn over {filled} of {panes} panes"
    );
}

/// The words `shape` paints, into `texts`.
#[cfg(not(target_arch = "wasm32"))]
fn painted_texts(shape: &egui::Shape, texts: &mut Vec<String>) {
    match shape {
        egui::Shape::Text(text) => texts.push(text.galley.text().to_owned()),
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                painted_texts(shape, texts);
            }
        }
        _ => {}
    }
}

/// Two panes of one page that each draw the same analysis in two strips
/// keep their strips' widgets apart: egui finds no widget id used twice.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn two_panes_drawing_one_analysis_in_two_strips_each_use_no_widget_id_twice() {
    use rspice_app_state::results::visualization_document::DocumentEdit;
    let mut window = ResultsWindow::new();
    let signals = ["V(in)", "I(R1)", "V(out)", "I(C1)"];
    let mut run = SimulationRun::new(8);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(11, AnalysisType::Transient, "Step response").with_waveforms(
            signals
                .iter()
                .map(|name| {
                    RetainedTrace::new(
                        *name,
                        vec![0.0, 0.5, 1.0],
                        vec![0.0, 1.0, 0.0],
                        String::new(),
                    )
                })
                .collect(),
        ),
    );
    window.dataset_id = run.dataset_id;
    window.app.state.model.simulation.retained.runs = vec![run.clone()].into();
    assert!(window.app.state.model.simulation.select_run(0));
    window.app.state.results.create_result_document = draft(
        "Both units twice",
        &run,
        0,
        "viewer-waveform",
        ResultDocumentLayout::TwoLinkedPanes,
        &signals,
    );
    let document_id = commit(&mut window.app.state.results_parts()).expect("the document commits");
    // The spread gives each pane one unit; each pane takes the other's too.
    let content = &mut window.app.state.model.workspace.content;
    let document = content
        .visualization_document(document_id)
        .expect("the project owns it");
    let mut panes = document.panes().iter().collect::<Vec<_>>();
    panes.sort_by_key(|pane| pane.order);
    let edits = vec![
        DocumentEdit::SetSignalsPlotted {
            pane_id: panes[0].id,
            signals: vec!["I(R1)".to_owned(), "I(C1)".to_owned()],
            plotted: true,
        },
        DocumentEdit::SetSignalsPlotted {
            pane_id: panes[1].id,
            signals: vec!["V(in)".to_owned(), "V(out)".to_owned()],
            plotted: true,
        },
    ];
    let revision = document.revision();
    content
        .transact_visualization_document(document_id, revision, edits)
        .expect("each pane plots both units");

    let clashes = window.id_clashes();
    assert!(clashes.is_empty(), "{clashes:?}");
}

/// A signal ticked in the data browser while a document is shown is plotted
/// in the document's active pane, though the pane did not list it, and
/// unticked comes off it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_signal_ticked_in_the_browser_is_plotted_in_the_documents_active_pane() {
    use rspice_app_results::workbench::docks::navigator::results_actions::set_result_quantities_ticked;
    use rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    use rspice_results_ui::selection::ResultBrowserSelectionKey;
    let mut window = ResultsWindow::new();
    let run = window.app.state.model.simulation.retained.runs[0].clone();
    window.app.state.results.create_result_document = draft(
        "Inputs",
        &run,
        0,
        "viewer-waveform",
        ResultDocumentLayout::SinglePane,
        &["V(in)"],
    );
    let document_id = commit(&mut window.app.state.results_parts()).expect("the document commits");
    window.frames(Vec::new());
    let output = ResultBrowserSelectionKey::Waveform(SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    ));
    let plotted = |window: &ResultsWindow| {
        plotted_traces(&window.app.state, document_id)
            .into_iter()
            .filter(|(_, _, visible)| *visible)
            .map(|(_, label, _)| label)
            .collect::<Vec<_>>()
    };

    set_result_quantities_ticked(&mut window.app.state.results_parts(), [&output], true);
    window.frames(Vec::new());
    assert_eq!(
        plotted(&window),
        ["V(in)", "V(out)"],
        "ticked, it is plotted"
    );
    set_result_quantities_ticked(&mut window.app.state.results_parts(), [&output], false);
    window.frames(Vec::new());
    assert_eq!(plotted(&window), ["V(in)"], "unticked, it comes off");
}

/// The traces each pane of `document_id` plots, by pane and label.
#[cfg(not(target_arch = "wasm32"))]
fn plotted_traces(
    state: &crate::workbench::app_state::AppState,
    document_id: rspice_app_state::product::ResultDocumentId,
) -> Vec<(usize, String, bool)> {
    let document = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project owns the document");
    document
        .traces()
        .iter()
        .map(|trace| {
            let pane = document
                .panes()
                .iter()
                .position(|pane| pane.id == trace.pane_id)
                .expect("a trace sits in a pane of its document");
            (pane, trace.label.clone(), trace.visible)
        })
        .collect()
}

/// Every sheet the form offers for every analysis of a solved run, in every
/// layout, makes a document Results draws with the run's data in each pane,
/// and the project brings each one back, drawn, after a save and a reopen.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_offered_combination_makes_a_drawn_document_that_a_reopen_brings_back() {
    use rspice_app_runs::simulation::controller::test_execution::{
        plot_every_signal, run_manual_batch,
    };
    for deck in [LOW_PASS_DECK, DIVIDER_DECK, SWEPT_DIVIDER_DECK] {
        let mut run = plot_every_signal(run_manual_batch(deck));
        // A project saves a run only once its provenance is classified; a
        // run the deck classified already keeps its own.
        let _ = run.restore_provenance(
            rspice_app_state::state::SimulationRunProvenance::LegacyUnattributed,
        );
        let dataset_id = run.dataset_id;
        let mut window = ResultsWindow::new();
        window.dataset_id = dataset_id;
        window.app.state.model.simulation.retained.runs = vec![run.clone()].into();
        assert!(window.app.state.model.simulation.select_run(0));
        window.frames(Vec::new());

        let mut made = Vec::new();
        for (index, analysis) in run.analyses.iter().enumerate() {
            assert!(
                window
                    .app
                    .state
                    .model
                    .simulation
                    .select_run_analysis(0, index)
            );
            open(&mut window.app.state.results_parts());
            let opened = window.app.state.results.create_result_document.clone();
            let sheets = choices(window.app.state.results_reads(), &opened).viewers;
            assert!(
                !sheets.is_empty(),
                "{deck}: a sheet draws {}",
                analysis.label
            );
            for (sheet, _) in sheets {
                for layout in ResultDocumentLayout::ALL {
                    let mut form = opened.clone();
                    form.viewer_id = sheet.to_owned();
                    reconcile(window.app.state.results_reads(), &opened, &mut form);
                    form.layout = layout;
                    form.name = format!("{} {sheet} {}", analysis.label, layout.label());
                    form.name_touched = true;
                    let combination = form.name.clone();
                    window.app.state.results.create_result_document = form;
                    let document_id = commit(&mut window.app.state.results_parts())
                        .unwrap_or_else(|error| panic!("{combination}: {error}"));
                    let nodes = window.settled_frames();
                    let plotted = plotted_traces(&window.app.state, document_id);
                    assert_drawn(&nodes, layout.pane_count(), &plotted, &combination);
                    made.push((document_id, combination, layout.pane_count(), plotted));
                }
            }
        }

        let root = std::env::temp_dir().join(format!(
            "rspice-new-result-document-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("a scratch folder");
        let path = root.join("documents.rspiceproj");
        assert!(
            window.app.state.save_project_to_path(&path),
            "the project saves"
        );
        // The app that made them closes first, so the reopen reads the
        // project as its own and not read-only beside an open copy.
        drop(window);
        let mut reopened = ResultsWindow::new();
        reopened.app.state.model.simulation.retained.runs = Vec::new().into();
        assert!(
            reopened.app.state.load_project_from_path(&path),
            "the project reopens"
        );
        reopened.app.state.workbench.activate(Workspace::Results);
        for (document_id, combination, panes, plotted) in made {
            let document = reopened
                .app
                .state
                .model
                .workspace
                .content
                .visualization_document(document_id)
                .unwrap_or_else(|| panic!("{combination}: the reopened project keeps it"));
            assert_eq!(document.title(), combination);
            assert_eq!(document.panes().len(), panes, "{combination}");
            // Shown as choosing its row in the data browser shows it.
            assert!(
                rspice_app_workbench::workbench::chrome::document_bar::activate_document_by_id(
                    &mut reopened.app.state,
                    &WorkspaceDocumentId::VisualizationDocument(document_id),
                ),
                "{combination}: the reopened document shows"
            );
            assert_eq!(
                plotted_traces(&reopened.app.state, document_id),
                plotted,
                "{combination}: the reopened document plots what it plotted"
            );
            let nodes = reopened.settled_frames();
            assert_drawn(&nodes, panes, &plotted, &combination);
        }
        let _ = std::fs::remove_dir_all(&root);
    }
}
