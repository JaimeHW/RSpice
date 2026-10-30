//! The result inspector's cross-probe into Design, and its route to the
//! run's details, observed through a whole application.

use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;
use crate::workbench::ResultViewer;
use crate::workbench::docks::inspector::results::*;
use crate::workbench::docks::inspector::run_details;
use crate::workbench::docks::inspector::show;
use crate::workbench::state::Workspace;
use egui::Color32;
use egui::Pos2;
use egui::Rect;
use egui::Vec2;
use rspice_results_ui::session::PaneAxis;

#[test]
fn historical_result_revision_cannot_cross_probe_current_geometry() {
    let mut app = result_app_with_current_out_map(false);
    let stale_revision = app
        .state
        .model
        .workspace
        .content
        .project
        .revision()
        .next()
        .expect("next project revision");
    let run = app
        .state
        .model
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
        .with_waveforms(vec![crate::state::RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "#ffbd2e",
        )]),
    );
    app.state.model.simulation.retained.runs[0] = stale_run;
    app.state.results.results.session.selected_trace = Some(
        rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
            &app.state.model.simulation.retained.runs[0],
            0,
            0,
        )
        .expect("selected historical trace"),
    );

    let error = cross_probe(&mut app, "V(out)").expect_err("historical result must fail closed");

    assert!(error.contains("different project revision"));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
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

    let run_id = app
        .state
        .model
        .simulation
        .active_run()
        .expect("retained run")
        .id;
    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        run_id,
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            run_details::show(
                ui.ctx(),
                &mut app.state.results.run_details,
                &app.state.model,
            );
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

    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        u64::MAX,
    );
    assert_eq!(
        app.state
            .results
            .run_details
            .as_ref()
            .map(|details| details.run_id),
        Some(run_id),
        "a run no longer retained opens nothing"
    );
}

pub(crate) fn result_app_with_current_out_map(split: bool) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.split_with_results = split;
    app.state.workbench.activate(Workspace::Results);
    let project_revision = app.state.model.workspace.content.project.revision();
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
        .model
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
            .with_waveforms(vec![crate::state::RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#ffbd2e",
            )]),
        );
    app.state.results.results.session.selected_trace = Some(
        rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
            app.state
                .model
                .simulation
                .active_run()
                .expect("retained run"),
            0,
            0,
        )
        .expect("selected retained trace"),
    );
    let a = crate::state::Point::new(0, 0);
    let b = crate::state::Point::new(40, 0);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::new(91, vec![a, b]));
    app.state.model.simulation.source.cross_probe.update(
        app.state.model.workspace.content.active_view.clone(),
        std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
        std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
        std::collections::HashMap::new(),
        app.state.model.schematic.topology_version(),
    );
    app
}

/// Cross-probe `signal` as the inspector's button does, then answer what the
/// gesture raised, as the shell does once the inspector returns: Design
/// opens there.
pub(crate) fn cross_probe(app: &mut RSpiceApp, signal: &str) -> Result<String, String> {
    let outcome = cross_probe_trace_to_design(&mut app.state.results_parts(), signal);
    app.after_contribution();
    outcome
}

/// A finished 1 ms transient on the Waves sheet, the way James had it open:
/// `V(out)` swinging inside 4…6 V with its pane active, the trace selected,
/// and the solver's convergence evidence retained with no forced points.
pub(crate) fn waves_details_app() -> RSpiceApp {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, TransientConvergenceEvidence};
    let quality = rspice_core::diagnostics::ConvergenceQuality::default();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    let time: Vec<f64> = (0..=200).map(|step| f64::from(step) * 5.0e-6).collect();
    let volts: Vec<f64> = time
        .iter()
        .map(|t| 5.0 + 0.9 * (std::f64::consts::TAU * 2.0e3 * t).sin())
        .collect();
    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)", time.clone(), volts, "#ffbd2e"),
        ]);
    analysis.convergence = Some(std::sync::Arc::new(
        TransientConvergenceEvidence::capture(quality, &time, &rspice_core::abort_signal::NoAbort)
            .expect("convergence evidence"),
    ));
    app.state
        .model
        .simulation
        .start_run()
        .add_analysis(analysis);
    app.state.model.simulation.view.active_analysis_idx = Some(0);
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("retained run");
    let analysis_key = crate::workbench::documents::result_document::AnalysisPresentationKey::new(
        run.dataset_id,
        &run.analyses[0],
    );
    let selected = rspice_results_ui::selection::SelectedResultTrace::from_run_indices(run, 0, 0)
        .expect("selected retained trace");
    app.state.results.results.session.selected_trace = Some(selected);
    app.state.results.results.session.viewer = ResultViewer::Waves;
    app.state.results.results.session.active_wave_pane = Some(
        crate::workbench::documents::result_document::WavePanePresentationKey {
            analysis: analysis_key,
            unit: "V".to_owned(),
        },
    );
    // Y held at the 4…6 V the field showed James as `4.0000000 … 6.000`.
    crate::workbench::documents::result_document::set_active_axis_range(
        &Tokens::default(),
        &mut app.state.results_parts(),
        PaneAxis::Y,
        Some((4.0, 6.0)),
    );
    app
}

/// Run the whole inspector at `width` the way the raster harness does — three
/// passes, the last one kept — and return what it painted and published.
pub(crate) fn run_inspector(
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

#[test]
fn full_results_cross_probe_navigates_to_design_after_exact_resolution() {
    let mut app = result_app_with_current_out_map(false);

    assert_eq!(cross_probe(&mut app, "V(out)"), Ok("OUT".to_owned()));

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(!app.state.workbench.split_with_results);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&91)
    );
    assert!(
        app.state
            .model
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

    assert_eq!(cross_probe(&mut app, "V(out)"), Ok("OUT".to_owned()));

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(
        app.state.workbench.results_split_visible(
            app.state.platform.project_lifecycle.is_open(),
            app.state
                .model
                .simulation
                .retained
                .has_retained_result_dataset(),
        )
    );
}

#[test]
fn stale_cross_probe_map_fails_without_leaving_results() {
    let mut app = result_app_with_current_out_map(true);
    app.state.model.schematic.add_wire(vec![
        crate::state::Point::new(80, 0),
        crate::state::Point::new(120, 0),
    ]);

    let error = cross_probe(&mut app, "V(out)").expect_err("topology mismatch must fail closed");

    assert!(error.contains("changed since this result"));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .model
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
    let view = app.state.model.workspace.content.active_view.clone();
    let view_type = app.state.model.workspace.content.active_view_type();
    app.state
        .model
        .workspace
        .open_as_root(view.clone(), view_type);

    let error = cross_probe(&mut app, "V(x1.out)")
        .expect_err("a trace read inside X1 has no conductor on the root sheet");
    assert_eq!(error, "Descend to /x1 to cross-probe it.");
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .model
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
    app.state.model.workspace.open_as_root(parent, view_type);
    app.state
        .model
        .workspace
        .descend_into("X1".to_owned(), view, view_type);

    let error = cross_probe(&mut app, "V(out)")
        .expect_err("a root-scoped trace does not name a conductor inside X1");
    assert_eq!(error, "Ascend to / to cross-probe it.");
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .model
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
    let view = app.state.model.workspace.content.active_view.clone();
    let view_type = app.state.model.workspace.content.active_view_type();
    let parent = crate::state::CellViewRef::new(&view.library, "tb", &view.view);
    app.state.model.workspace.open_as_root(parent, view_type);
    app.state
        .model
        .workspace
        .descend_into("amp".to_owned(), view, view_type);

    let net = cross_probe(&mut app, "V(xamp.out)").expect("the written scope is this occurrence");

    assert!(net.eq_ignore_ascii_case("out"), "{net}");
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
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
    let inset = crate::workbench::design_system::PANEL_INSET;
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

/// A deferred saved output is materialized from the inspector's run section
/// while the manifest sheet is showing, the one sheet whose panel the
/// inspector used to hide along with the only Materialize the product had.
#[test]
fn materialize_in_the_run_section_evaluates_a_deferred_output_on_the_manifest_sheet() {
    use egui::accesskit::{Action, ActionRequest, Role, TreeId};

    let mut app = deferred_output_app();
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let pass = |app: &mut RSpiceApp, events: Vec<egui::Event>| {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::ZERO,
                    Vec2::new(INSPECTOR_DEFAULT_WIDTH, 2400.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ui, |ui| show(ui, app));
            },
        );
        output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default()
    };
    let mut nodes = Vec::new();
    for _ in 0..3 {
        nodes = pass(&mut app, Vec::new());
    }
    let row_value = |nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)]| {
        nodes
            .iter()
            .find(|(_, node)| node.label() == Some("output_voltage") && node.value().is_some())
            .and_then(|(_, node)| node.value().map(str::to_owned))
    };
    assert_eq!(row_value(&nodes).as_deref(), Some("Deferred"));
    let materialize = nodes
        .iter()
        .find(|(_, node)| {
            node.role() == Role::Button && node.label() == Some("Materialize output_voltage")
        })
        .map(|(id, _)| *id)
        .expect("the deferred output's row offers Materialize on the manifest sheet");

    pass(
        &mut app,
        vec![egui::Event::AccessKitActionRequest(ActionRequest {
            action: Action::Click,
            target_tree: TreeId::ROOT,
            target_node: materialize,
            data: None,
        })],
    );
    let nodes = pass(&mut app, Vec::new());

    assert_eq!(
        app.state.results.results.session.viewer,
        ResultViewer::Manifest
    );
    let analysis = &app.state.model.simulation.retained.runs[0].analyses[0];
    assert!(
        matches!(
            &analysis.saved_output_receipts[0].status,
            crate::state::SavedOutputMaterializationStatus::Materialized {
                waveform_name,
                sample_count: 2,
            } if waveform_name == "output_voltage"
        ),
        "{:?}",
        analysis.saved_output_receipts[0].status
    );
    let materialized = analysis
        .waveforms
        .iter()
        .find(|wave| wave.name == "output_voltage")
        .expect("the materialized output is retained beside its source");
    assert_eq!(materialized.x[..], [0.0, 1.0]);
    assert_eq!(materialized.y[..], [0.0, 2.0]);
    assert_eq!(row_value(&nodes).as_deref(), Some("2 pts"));
    assert!(
        !nodes
            .iter()
            .any(|(_, node)| node.label() == Some("Materialize output_voltage")),
        "a materialized output offers Materialize again"
    );
}

/// A finished transient on the manifest sheet whose analysis saved one output
/// on demand: `output_voltage`, the recipe `V(out)` over the retained `out`.
fn deferred_output_app() -> RSpiceApp {
    use crate::state::{
        AnalysisResult, AnalysisType, RetainedTrace, SavedOutputMaterializationStatus,
        SavedOutputReceipt, SimulationRun, SimulationRunLifecycle, SimulationRunProvenance,
    };
    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("out", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
        ]);
    analysis.saved_output_receipts.push(SavedOutputReceipt {
        source_bindings: Some(crate::state::SavedOutputSourceBindings {
            axis: crate::state::SavedOutputAxis::Waveform {
                name: "out".to_owned(),
            },
            references: [(
                "v(out)".to_owned(),
                crate::state::SavedOutputBoundSource::Waveform {
                    name: "out".to_owned(),
                },
            )]
            .into(),
        }),
        output_id: crate::product::SavedOutputId::new(),
        output_revision: crate::product::ObjectRevision::INITIAL,
        analysis_id: crate::product::AnalysisInstanceId::new(),
        contract_digest: crate::product::ContentDigest::from_bytes([0x7a; 32]),
        name: "output_voltage".to_owned(),
        source_expression: "V(out)".to_owned(),
        complex_policy: crate::state::ComplexExpressionPolicy::LegacyMagnitude,
        output_kind: crate::state::SavedOutputKind::RawVoltageOrCurrent,
        save_policy: crate::state::SavedOutputPolicy::OnDemandFromRetainedState,
        stored_precision: crate::state::SavedOutputPrecision::FullSourcePrecision,
        streaming: crate::state::SavedOutputStreaming::StoreOnly,
        display_intent: crate::state::SavedOutputDisplayIntent::Plot,
        status: SavedOutputMaterializationStatus::Deferred,
    });
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy fixture is explicitly classified");

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    app.state.model.simulation.view.active_analysis_idx = Some(0);
    app.state.results.results.session.viewer = ResultViewer::Manifest;
    app
}

/// The inspector's default width on a wide window and its floor in every
/// window class (`workbench::layout::inspector_width_bounds`).
pub(crate) const INSPECTOR_DEFAULT_WIDTH: f32 = 312.0;
const INSPECTOR_MIN_WIDTH: f32 = 278.0;

/// One run of text the inspector painted: where its layout box landed, the
/// rectangle it was clipped to, and whether anything of it is visible.
#[derive(Debug)]
struct PaintedText {
    text: String,
    rect: Rect,
    clip: Rect,
}
