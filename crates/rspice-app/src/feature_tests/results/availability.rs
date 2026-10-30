//! Results sheets drawn by a whole application: every sheet paints,
//! meshes and fits its window through the workbench's own dispatch, and the
//! evidence tables cost their viewport, not their dataset.

use rspice_results_ui::session::PaneAxis;
use rspice_results_ui::soa::SoaRuleSelection;

use crate::state::{
    ActiveViewer, AnalysisResult, AnalysisResultPayload, AnalysisType, RetainedTrace,
    SensitivityResultMode, SensitivityResultRow, SimulationRun, SpecializedViewerCacheProvenance,
};
use crate::ui::tokens::Tokens;
use crate::workbench::documents::result_document::test_fixtures::{
    events_analysis, network_matrix_analysis, optimization_analysis, soa_analysis,
};
use crate::workbench::documents::result_document::{
    ActivePaneFacts, AnalysisPresentationKey, WavePanePresentationKey, active_axis_is_pinned,
    active_axis_range, active_incomplete_evidence_reason, right_panel, set_active_axis_range,
    sheet_purpose, show, show_persistent_pane_viewer, smith, viewer_availability, waves,
};
use crate::workbench::{AppState, RSpiceApp, ResultViewer};

/// The two gates a Results sheet is looked at through: the small window the
/// display scaling of a real workstation produces, and a wide one.
const SHEET_FIT_GATES: [(f32, f32); 2] = [(1_024.0, 640.0), (1_680.0, 1_020.0)];

/// A resonant second-order response, swept densely enough to decimate.
///
/// Four points was not a sweep: it sat under every reduction threshold, so
/// the Bode, Nyquist and Smith fixtures exercised only the raw-stroke
/// path and never the one a real AC run takes. The resonance also makes
/// the complex locus genuinely non-monotone in its real part, so the
/// precondition assertion in `decimate_minmax` fires if either locus
/// sheet ever loses its `parametric` declaration.
pub(super) const AC_FIXTURE_POINTS: usize = 8_192;

/// Run one sheet plus its inspector panel through a real egui frame and
/// assert every mesh vertex is finite. A value that escapes an axis
/// mapping does not panic — it becomes a vertex at infinity and
/// degenerates the whole draw call.
///
/// Painted through `show_persistent_pane_viewer`, the same dispatch the
/// workbench itself paints through, so no sheet can pass here by a route
/// the product never takes.
fn draw_sheet_and_tessellate(app: &mut RSpiceApp, viewer: ResultViewer) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    // Twice: the first frame builds the caches, the second is the path a
    // reader actually spends their time on.
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::Panel::right("inspector")
                .show(ctx, |ui| right_panel(ui, &mut app.state.results_parts()));
            egui::CentralPanel::default().show(ctx, |ui| {
                super::with_surface_parts(app, |state, run_controller, selected_specification| {
                    show_persistent_pane_viewer(
                        ui,
                        state,
                        run_controller,
                        selected_specification,
                        viewer,
                        None,
                    )
                })
            });
        });
        for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
            let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                continue;
            };
            assert!(
                mesh.vertices
                    .iter()
                    .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                "{viewer:?} put a non-finite vertex in the mesh",
            );
        }
    }
}

/// The Results workspace at one window size, as its controls published
/// themselves. Two passes: the first settles the layout the second is
/// measured on.
fn sheet_route_nodes(
    mut app: RSpiceApp,
    width: f32,
    height: f32,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, height),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::Panel::right("inspector")
                    .show(ctx, |ui| right_panel(ui, &mut app.state.results_parts()));
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        super::with_surface_parts(
                            &mut app,
                            |state, run_controller, selected_specification| {
                                show(ui, state, run_controller, selected_specification)
                            },
                        )
                    });
            },
        );
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes
}

/// Paint one sheet and count the primitives it actually emits.
fn drawn_shape_count(app: &mut RSpiceApp, viewer: ResultViewer) -> usize {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    let mut shapes = 0;
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                super::with_surface_parts(app, |state, run_controller, selected_specification| {
                    show_persistent_pane_viewer(
                        ui,
                        state,
                        run_controller,
                        selected_specification,
                        viewer,
                        None,
                    )
                })
            });
        });
        shapes = output.shapes.len();
    }
    shapes
}

fn operating_point_app(devices: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
        rspice_core::circuit::DeviceOpReport {
            entries: (0..devices)
                .map(|index| rspice_core::circuit::DeviceOpEntry {
                    // Spread across scopes so the group headers and the
                    // per-scope column sets are exercised too.
                    name: format!("X{}.M{index}", index % 8),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: Vec::new(),
                })
                .collect(),
        },
    );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app
}

fn sensitivity_app(parameters: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS").with_result_payload(
        AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: (0..parameters)
                .map(|index| SensitivityResultRow {
                    // Zero-padded: the payload requires strictly sorted
                    // parameter names, and "p10" sorts before "p2".
                    parameter: format!("p{index:06}"),
                    raw: (index as f64 * 1.0e-3).into(),
                    normalized: ((index as f64 * 1.0e-3).tanh()).into(),
                })
                .collect(),
        },
    );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app
}

fn manifest_app(analyses: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(1);
    for index in 0..analyses {
        run.add_analysis(AnalysisResult::new(
            index as u64 + 1,
            AnalysisType::Transient,
            format!("TRAN {index}"),
        ));
    }
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app
}

fn state_with_analysis(analysis: AnalysisResult) -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    state
}

fn active_analysis_key(state: &AppState) -> AnalysisPresentationKey {
    let run = state
        .model
        .simulation
        .active_run()
        .expect("active retained run");
    AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
}

/// Draw each newly reachable sheet through a real frame and tessellate it.
///
/// Availability only proves the tab lights up. This proves the sheet
/// behind it lays out, paints and meshes — the failure mode these viewers
/// were most exposed to, having never been compiled at all.
#[test]
fn every_newly_reachable_sheet_draws_and_meshes() {
    for (analysis, viewer) in [
        (soa_analysis(), ResultViewer::Soa),
        (optimization_analysis(), ResultViewer::Optimization),
        (events_analysis(), ResultViewer::Events),
    ] {
        let mut state = state_with_analysis(analysis);
        state.results.results.session.viewer = viewer;
        if viewer == ResultViewer::Soa {
            // The stress trace is the sheet's only plot; a card that never
            // opens is a card this test never covers.
            state.results.results.session.soa_stress_trace_open = true;
            state.results.results.session.selected_soa_rule = Some(SoaRuleSelection {
                analysis: active_analysis_key(&state),
                device_id: "M1".to_owned(),
                parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
            });
        }
        let mut app = RSpiceApp::test_instance();
        app.state = state;
        draw_sheet_and_tessellate(&mut app, viewer);
    }
}

/// Every sheet, painted on evidence that sheet declares itself able to
/// draw.
///
/// The fixture has to satisfy the sheet's own `viewer_availability`
/// contract, so this cannot pass by painting an empty state and calling it
/// coverage. Fourteen of the twenty-two sheets had never been through a
/// real frame in any test; their module tests derive and format values
/// without ever painting them, and a value that escapes an axis mapping
/// does not fail an assert — it becomes a vertex at infinity.
#[test]
fn every_available_sheet_draws_the_evidence_it_claims() {
    for viewer in ResultViewer::all() {
        let mut app = app_showing(viewer);
        assert!(
            viewer_availability(app.state.results_reads(), viewer).available,
            "the {viewer:?} fixture does not satisfy the sheet's own contract: {}",
            viewer_availability(app.state.results_reads(), viewer).reason
        );
        draw_sheet_and_tessellate(&mut app, viewer);
    }
}

/// Every sheet, painted with no retained dataset at all.
///
/// That is the workspace's state before the first run and the one a user
/// meets first, and it is where a plot divides by a zero-width axis span or
/// reads the first of an empty series. Fourteen of the twenty-two sheets had
/// never been through a real frame in any test — their module tests derive
/// and format values without ever painting them.
#[test]
fn every_sheet_paints_an_empty_dataset() {
    for viewer in ResultViewer::all() {
        let mut app = RSpiceApp::test_instance();
        draw_sheet_and_tessellate(&mut app, viewer);
        assert_eq!(
            app.state.results.results.session.viewer, viewer,
            "{viewer:?} did not stay selected across its own frame"
        );
    }
}

/// Every control of the four sheets that own a domain bar is inside the window
/// it is drawn in, at both gates.
///
/// Measured through AccessKit rather than through painted shapes, for the
/// reason the Studio's own gate states: a decorative shape may legitimately be
/// clipped and a control may not. A control past the right edge is a control
/// behind a horizontal scroll, which is the thing these sheets' single bar
/// exists to avoid; one past the bottom is a register the reader cannot reach
/// without scrolling a document well that does not scroll.
///
/// The Events sheet joined the list when it gained a bar of its own: its bus
/// radix, and the per-row disclosure that opens a word to its member rows,
/// are controls that a reader has to be able to reach.
#[test]
fn the_catalog_sheets_fit_their_window_at_both_gates() {
    // Sub-pixel: a control resting exactly on the edge is inside it.
    const TOLERANCE: f64 = 0.5;

    let mut offenders = Vec::new();
    let mut measured = 0_usize;
    // Counted separately: a disclosure that names the bus it opens is the one
    // control here whose absence would look exactly like a sheet that simply
    // declared no bus.
    let mut buses_measured = 0_usize;
    for viewer in [
        ResultViewer::Polar,
        ResultViewer::Scatter,
        ResultViewer::BoxViolin,
        ResultViewer::Events,
    ] {
        for (width, height) in SHEET_FIT_GATES {
            let mut app = app_showing(viewer);
            app.state.results.results.session.viewer = viewer;
            for (_, node) in sheet_route_nodes(app, width, height) {
                if !matches!(
                    node.role(),
                    egui::accesskit::Role::Button
                        | egui::accesskit::Role::CheckBox
                        | egui::accesskit::Role::ComboBox
                        | egui::accesskit::Role::Link
                        | egui::accesskit::Role::RadioButton
                        | egui::accesskit::Role::TextInput
                ) {
                    continue;
                }
                let Some(bounds) = node.bounds() else {
                    continue;
                };
                measured += 1;
                if node
                    .label()
                    .is_some_and(|label| label.starts_with("Bits of "))
                {
                    buses_measured += 1;
                }
                if bounds.x1 > f64::from(width) + TOLERANCE
                    || bounds.x0 < -TOLERANCE
                    || bounds.y1 > f64::from(height) + TOLERANCE
                    || bounds.y0 < -TOLERANCE
                {
                    offenders.push(format!(
                        "{width:.0}x{height:.0} {viewer:?}: {:?} {:?} spans x {:.1}..{:.1}, y {:.1}..{:.1}",
                        node.role(),
                        node.label().unwrap_or_default(),
                        bounds.x0,
                        bounds.x1,
                        bounds.y0,
                        bounds.y1
                    ));
                }
            }
        }
    }
    offenders.sort();
    offenders.dedup();
    assert!(
        offenders.is_empty(),
        "controls drawn outside the window they belong to:
{}",
        offenders.join(
            "
"
        )
    );
    assert!(
        measured >= 4 * SHEET_FIT_GATES.len(),
        "the gate measured {measured} controls, so it is not looking at the sheets' bars"
    );
    assert!(
        buses_measured >= 2 * SHEET_FIT_GATES.len(),
        "the gate measured {buses_measured} bus disclosures, so the Events fixture's two \
         declarations are not reaching the sheet"
    );
}

/// The evidence tables must cost their viewport, not their dataset.
///
/// `save_device_op` on a real block emits one row per device, a swept
/// design ranks one sensitivity row per parameter, and a save-all run
/// retains one manifest row per task. All three laid out, sensed and
/// painted every retained row on every frame, so the sheet's cost grew
/// without bound while the window stayed the same size. A row count is
/// not the assertion — the drawn primitive count is.
#[test]
fn the_evidence_tables_cost_their_viewport_not_their_dataset() {
    for (viewer, small, large) in [
        (
            ResultViewer::Op,
            drawn_shape_count(&mut operating_point_app(64), ResultViewer::Op),
            drawn_shape_count(&mut operating_point_app(20_000), ResultViewer::Op),
        ),
        (
            ResultViewer::Contribution,
            drawn_shape_count(&mut sensitivity_app(64), ResultViewer::Contribution),
            drawn_shape_count(&mut sensitivity_app(20_000), ResultViewer::Contribution),
        ),
        (
            ResultViewer::Manifest,
            drawn_shape_count(&mut manifest_app(64), ResultViewer::Manifest),
            drawn_shape_count(&mut manifest_app(4_000), ResultViewer::Manifest),
        ),
    ] {
        assert!(
            large < small * 2,
            "{viewer:?} drew {large} primitives for the large dataset against {small} \
             for the small one — it is still laying out rows it cannot show"
        );
    }
}

/// A run that stopped early must not draw like one that finished.
///
/// The waveform sheets deliberately still plot it — where a transient
/// stopped converging is what the plot is for — but for a long time they
/// plotted it with no mark at all, while every typed-evidence sheet
/// refused outright. A curve that ends early is indistinguishable from a
/// sweep specified to end there.
#[test]
fn a_run_that_did_not_complete_says_so_on_the_sheet_that_draws_it() {
    let mut app = app_showing(ResultViewer::Waves);
    assert_eq!(
        active_incomplete_evidence_reason(app.state.results_reads()),
        None,
        "a converged fixture must not claim a caution"
    );
    let clean_purpose = sheet_purpose(app.state.results_reads());

    app.state.model.simulation.retained.runs[0].analyses[0].success = false;

    // Still drawable: refusing the partial samples would take away the
    // one view that answers "where did it go wrong?".
    assert!(
        viewer_availability(app.state.results_reads(), ResultViewer::Waves).available,
        "partial transient samples must stay drawable"
    );
    let purpose = sheet_purpose(app.state.results_reads());
    assert_ne!(purpose, clean_purpose);
    assert!(
        purpose.contains("did not complete"),
        "the sheet bar has to say it in words: {purpose}"
    );

    // And the strip that draws it carries the mark, so a stack whose
    // other analyses converged cannot pass the failed one off as clean.
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| waves::show(ui, &mut app.state.results_parts()));
        },
    );
    let labels: Vec<String> = output
        .platform_output
        .accesskit_update
        .expect("the waveform stack publishes an accessibility tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect();
    assert!(
        labels
            .iter()
            .any(|label| label.contains("stop where it failed")),
        "no pane published the incomplete-evidence reason: {labels:?}"
    );
}

/// An explicit interval has to land in the store the sheet reads.
///
/// The waveform stack keys its viewports by analysis and every other
/// sheet by plot ordinal. A previous fit command wrote to the ordinal
/// store for all of them, so Fit was a silent no-op on the four sheets
/// people actually use. The same trap is waiting for anything that
/// writes a viewport, so this asserts through the reader, not the map.
#[test]
fn an_explicit_interval_lands_in_the_store_its_own_sheet_reads() {
    let tokens = Tokens::default();
    let mut app = app_showing(ResultViewer::Waves);
    let analysis = active_analysis_key(&app.state);
    app.state.results.results.session.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: "V".to_owned(),
    });

    assert!(set_active_axis_range(
        &tokens,
        &mut app.state.results_parts(),
        PaneAxis::X,
        Some((1.0e-6, 4.0e-6))
    ));
    assert_eq!(
        app.state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, 0)
            .x,
        Some((1.0e-6, 4.0e-6)),
        "the waveform stack must read its own analysis-keyed viewport"
    );
    assert!(active_axis_is_pinned(
        app.state.results_reads(),
        PaneAxis::X
    ));

    set_active_axis_range(&tokens, &mut app.state.results_parts(), PaneAxis::X, None);
    assert!(!active_axis_is_pinned(
        app.state.results_reads(),
        PaneAxis::X
    ));

    let mut app = app_showing(ResultViewer::Smith);
    app.state.results.results.session.viewer = ResultViewer::Smith;
    assert!(set_active_axis_range(
        &tokens,
        &mut app.state.results_parts(),
        PaneAxis::Y,
        Some((-0.5, 0.5))
    ));
    assert_eq!(
        app.state
            .results
            .results
            .session
            .plot_view(ResultViewer::Smith, 0)
            .y,
        Some((-0.5, 0.5)),
        "a single-canvas sheet keeps its viewport under the plot ordinal"
    );
    assert!(active_axis_is_pinned(
        app.state.results_reads(),
        PaneAxis::Y
    ));
}

/// The editor has to open on the interval the reader can see, including
/// on a sheet that is fitting its data rather than pinned.
#[test]
fn the_axis_editor_opens_on_what_the_sheet_actually_drew() {
    let mut app = app_showing(ResultViewer::Smith);
    app.state.results.results.session.viewer = ResultViewer::Smith;
    draw_sheet_and_tessellate(&mut app, ResultViewer::Smith);

    let facts = ActivePaneFacts {
        unit: None,
        x_unit: None,
        analysis: None,
        scale: None,
        limit_mask: "none bound",
        x_extent: None,
        y_extent: None,
    };
    let x = active_axis_range(app.state.results_reads(), &facts, PaneAxis::X)
        .expect("the Smith sheet recorded the interval it drew");
    assert!(
        x.0 < x.1 && x.0.is_finite() && x.1.is_finite(),
        "recorded a degenerate interval: {x:?}"
    );
    assert!(
        !active_axis_is_pinned(app.state.results_reads(), PaneAxis::X),
        "a fitted sheet must not report itself pinned just because it drew"
    );
}

/// Every sheet, drawn at the touch composition the tablet build uses.
///
/// The shell has raised its own rows to a 44 px target for a long time
/// while this workspace kept fixed workstation heights, so its chips and
/// icon buttons grew past the bands holding them. A frame is what proves
/// the composition survives, not the constants alone.
#[test]
fn every_sheet_draws_at_a_touch_composition() {
    for viewer in ResultViewer::all() {
        let mut app = app_showing(viewer);
        app.state.results.results.session.viewer = viewer;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        crate::ui::Theme::default()
            .apply_responsive_metrics_with_target(&ctx, Some(crate::ui::tokens::TOUCH_TARGET));
        assert!(
            Tokens::get(&ctx).metrics.is_touch(),
            "the fixture did not reach a touch composition"
        );
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_024.0, 1_366.0),
            )),
            ..Default::default()
        };
        for _ in 0..2 {
            let output = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    super::with_surface_parts(
                        &mut app,
                        |state, run_controller, selected_specification| {
                            show_persistent_pane_viewer(
                                ui,
                                state,
                                run_controller,
                                selected_specification,
                                viewer,
                                None,
                            )
                        },
                    );
                });
            });
            for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
                let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                    continue;
                };
                assert!(
                    mesh.vertices
                        .iter()
                        .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                    "{viewer:?} put a non-finite vertex in the mesh at a touch composition",
                );
            }
        }
    }
}

/// A dataset each sheet reports itself able to draw. Built from retained
/// evidence and, where a sheet reads a derived cache, through the same
/// derivation the controller runs after a completed analysis — never by
/// asserting a capability the data does not support.
pub(super) fn app_showing(viewer: ResultViewer) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    // The production viewer classifies retained data outside an open
    // project as `no-project`; this fixture represents an open Results
    // workspace and must establish that authority explicitly.
    app.state.open_project_session();
    let analysis = match viewer {
        ResultViewer::Waves
        | ResultViewer::Table
        | ResultViewer::Manifest
        | ResultViewer::Eye
        | ResultViewer::Fft => transient_analysis(),
        ResultViewer::DcSweep => AnalysisResult::new(1, AnalysisType::DcSweep, "DC")
            .with_waveforms(vec![RetainedTrace::new(
                "V(out)",
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 0.4, 0.9, 1.2],
                "#00aaff",
            )]),
        ResultViewer::Bode | ResultViewer::Nyquist => ac_analysis(),
        ResultViewer::Smith | ResultViewer::Polar => sparameter_analysis(),
        ResultViewer::NetworkMatrix => network_matrix_analysis(false),
        ResultViewer::Scatter | ResultViewer::BoxViolin => {
            // The distribution sheets need a bounded measurement to normalize
            // against, and the box's default grouping draws nothing without
            // one.
            app.state
                .model
                .workspace
                .content
                .specs
                .push(crate::state::SpecEntry {
                    measurement: "gain_dc".to_owned(),
                    expression: String::new(),
                    min: Some(39.5),
                    max: None,
                    unit: "dB".to_owned(),
                    scope: crate::state::SpecPointScope::AllPoints,
                });
            monte_carlo_population_analysis()
        }
        ResultViewer::HarmonicBalance => {
            AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
                RetainedTrace::new(
                    "V(out)",
                    vec![0.0, 1.0e9, 2.0e9],
                    vec![1.0, 0.1, 0.01],
                    "#00aaff",
                )
                .with_complex_components(
                    "V(out)",
                    vec![1.0, 0.1, 0.01],
                    vec![0.0, 0.02, 0.001],
                ),
            ])
        }
        ResultViewer::PhaseNoise => AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
            .with_waveforms(vec![RetainedTrace::new(
                "phase_noise",
                vec![1.0e3, 1.0e4, 1.0e5],
                vec![-80.0, -100.0, -120.0],
                "#00aaff",
            )]),
        ResultViewer::Hist => monte_carlo_analysis(),
        ResultViewer::Op => AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
            rspice_core::circuit::DeviceOpReport {
                entries: vec![rspice_core::circuit::DeviceOpEntry {
                    name: "M1".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: Vec::new(),
                }],
            },
        ),
        ResultViewer::NoiseContrib => AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_waveforms(vec![RetainedTrace::new(
                "inoise",
                vec![1.0, 10.0, 100.0],
                vec![1.0e-9, 2.0e-9, 4.0e-9],
                "#00aaff",
            )]),
        ResultViewer::Contribution => AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![
                    SensitivityResultRow {
                        parameter: "r1".to_owned(),
                        raw: (0.25).into(),
                        normalized: (0.5).into(),
                    },
                    SensitivityResultRow {
                        parameter: "r2".to_owned(),
                        raw: (-0.125).into(),
                        normalized: (-0.25).into(),
                    },
                ],
            }),
        ResultViewer::TransferFunction => transfer_function_analysis(),
        ResultViewer::Specs => {
            app.state
                .model
                .workspace
                .content
                .specs
                .push(crate::state::SpecEntry {
                    measurement: "V(out)".to_owned(),
                    expression: String::new(),
                    min: Some(-2.0),
                    max: Some(5.0),
                    unit: "V".to_owned(),
                    scope: crate::state::SpecPointScope::AllPoints,
                });
            transient_analysis()
        }
        ResultViewer::PoleZero => AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
            .with_result_payload(AnalysisResultPayload::PoleZero {
                poles: vec![crate::state::ComplexResultValue {
                    real: -1.0e3,
                    imaginary: 2.0e3,
                }],
                zeros: vec![crate::state::ComplexResultValue {
                    real: -5.0e3,
                    imaginary: 0.0,
                }],
                pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                gain: Some(10.0),
            }),
        ResultViewer::Events => events_analysis(),
        ResultViewer::Soa => soa_analysis(),
        ResultViewer::Optimization => optimization_analysis(),
    };

    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    run.lifecycle = crate::state::SimulationRunLifecycle::Completed;
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));

    // Three sheets read a cache the controller fills after a run rather
    // than the retained analysis itself. Derive it exactly as the
    // controller's post-processing does, so the fixture cannot claim a
    // capability the data would not actually produce.
    match viewer {
        ResultViewer::Nyquist => derive_nyquist(&mut app),
        ResultViewer::Smith => {
            assert!(smith::synchronize_active_analysis(
                &mut app.state.results_parts()
            ));
        }
        _ => {}
    }

    if viewer == ResultViewer::Soa {
        // The stress trace is the sheet's only plot; a card that never
        // opens is a card this test never covers.
        app.state.results.results.session.soa_stress_trace_open = true;
        app.state.results.results.session.selected_soa_rule = Some(SoaRuleSelection {
            analysis: active_analysis_key(&app.state),
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
        });
    }
    app
}

pub(super) fn transient_analysis() -> AnalysisResult {
    let time: Vec<f64> = (0..64).map(|index| index as f64 * 1.0e-9).collect();
    let values: Vec<f64> = time
        .iter()
        .map(|t| (t * 1.0e9 * std::f64::consts::TAU / 8.0).sin())
        .collect();
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![RetainedTrace::new("V(out)", time, values, "#00aaff")])
}

pub(super) fn ac_analysis() -> AnalysisResult {
    let (mut frequency, mut real, mut imaginary) = (
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
    );
    let (mut magnitude, mut phase) = (
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
    );
    // H(jω) = 1 / (1 − (ω/ω0)² + j·ω/(Q·ω0)), log-swept through resonance.
    let (natural, quality) = (1.0e5_f64, 6.0_f64);
    for index in 0..AC_FIXTURE_POINTS {
        let decade = 3.0 + 4.0 * index as f64 / (AC_FIXTURE_POINTS - 1) as f64;
        let f = 10.0_f64.powf(decade);
        let ratio = f / natural;
        let (denominator_real, denominator_imaginary) = (1.0 - ratio * ratio, ratio / quality);
        let square =
            denominator_real * denominator_real + denominator_imaginary * denominator_imaginary;
        let (re, im) = (denominator_real / square, -denominator_imaginary / square);
        frequency.push(f);
        real.push(re);
        imaginary.push(im);
        magnitude.push(re.hypot(im));
        phase.push(im.atan2(re).to_degrees());
    }
    AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
        RetainedTrace::new("|V(out)|", frequency.clone(), magnitude, "#00aaff")
            .with_complex_components("V(out)", real, imaginary),
        RetainedTrace::new("phase(V(out))", frequency, phase, "#ffaa00"),
    ])
}

pub(super) fn sparameter_analysis() -> AnalysisResult {
    let mut analysis = ac_analysis();
    analysis.analysis_type = AnalysisType::SParameter;
    analysis.label = "SP".to_owned();
    analysis.waveforms.truncate(1);
    analysis.waveforms[0].name = "|S11|".to_owned();
    analysis.waveforms[0]
        .complex
        .as_mut()
        .expect("AC fixture complex components")
        .source_name = "S11".to_owned();
    analysis.with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
        noise_reference_temperature_kelvin: None,
        reference_impedances_ohm: vec![75.0, 100.0],
    })
}

/// A Monte Carlo that retained both halves of its evidence: the sampled
/// variable and what every trial measured, indexed the same way.
///
/// The histogram fixture beside this one deliberately retains neither — it
/// exercises the path where a distribution comes from the derived histogram
/// cache — so the correlation and distribution sheets need their own.
pub(super) fn monte_carlo_population_analysis() -> AnalysisResult {
    use crate::state::{
        FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
        MonteCarloVariableMetadata,
    };

    const TRIALS: usize = 101;
    let samples: Vec<f64> = (0..TRIALS)
        .map(|index| (index as f64 - 50.0) / 50.0)
        .collect();
    let members = samples
        .iter()
        .enumerate()
        .map(|(index, sample)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial {
                    index,
                    seed: 0x73a4 + index as u64,
                },
                vec![
                    FamilyMeasurementEvidence {
                        unit: None,
                        name: "gain_dc".to_owned(),
                        value: Some(40.0 + 2.0 * sample),
                        passed: true,
                        error: None,
                    },
                    FamilyMeasurementEvidence {
                        unit: None,
                        name: "vos".to_owned(),
                        value: Some(60.0e-6 * sample),
                        passed: true,
                        error: None,
                    },
                ],
            )
        })
        .collect::<Vec<_>>();
    let mean = samples.iter().sum::<f64>() / TRIALS as f64;
    let variance = samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (TRIALS - 1) as f64;
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
        crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 0x73a4,
            runs_requested: TRIALS,
            runs_completed: TRIALS,
            failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableMetadata {
                mean_confidence: None,
                name: "XBRIDGE.dR".to_owned(),
                mean,
                std_dev: variance.sqrt(),
                min: -1.0,
                max: 1.0,
                samples,
            }],
            member_measurements: members,
        },
    )
}

pub(super) fn monte_carlo_analysis() -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC")
        .with_waveforms(vec![RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0, 2.0, 3.0],
            vec![0.95, 1.02, 0.98, 1.05],
            "#00aaff",
        )])
        .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 7,
            runs_requested: 4,
            runs_completed: 4,
            failures: 0,
            all_converged: true,
            member_measurements: Vec::new(),
            variables: vec![crate::state::MonteCarloVariableMetadata {
                mean_confidence: None,
                name: "V(out)".to_owned(),
                samples: vec![0.95, 1.02, 0.98, 1.05],
                mean: 1.0,
                std_dev: (0.0058_f64 / 3.0).sqrt(),
                min: 0.95,
                max: 1.05,
            }],
        })
}

pub(super) fn transfer_function_analysis() -> AnalysisResult {
    use crate::state::{
        TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
        TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
    };
    AnalysisResult::new(1, AnalysisType::Tf, "XF").with_result_payload(
        AnalysisResultPayload::TransferFunction {
            input_source: "vin".to_owned(),
            output_expression: "V(out)".to_owned(),
            input_quantity: TransferFunctionQuantityEvidence::Voltage,
            output_quantity: TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TransferFunctionNormalizationEvidence::None,
            accuracy: TransferFunctionAccuracyEvidence::Balanced,
            gain: Some(TransferFunctionScalarEvidence::Finite(-12.5)),
            input_resistance: Some(TransferFunctionScalarEvidence::Finite(1.0e6)),
            output_resistance: Some(TransferFunctionScalarEvidence::Finite(50.0)),
            nominal_input: None,
            nominal_output: None,
        },
    )
}

pub(super) fn in_flight_cache_provenance(app: &RSpiceApp) -> SpecializedViewerCacheProvenance {
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("the fixture retained one run");
    let dataset_id = run.dataset_id;
    let analysis = app
        .state
        .model
        .simulation
        .active_analysis()
        .expect("the fixture selected one analysis");
    SpecializedViewerCacheProvenance::for_analysis(dataset_id, analysis)
}

pub(super) fn ac_complex_trace(app: &RSpiceApp) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let analysis = app
        .state
        .model
        .simulation
        .active_analysis()
        .expect("the fixture selected one analysis");
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.complex.is_some())
        .expect("the AC fixture retains one complex trace");
    let complex = waveform
        .complex
        .as_ref()
        .expect("the trace was selected for its complex components");
    (
        waveform.x.to_vec(),
        complex.real.to_vec(),
        complex.imag.to_vec(),
    )
}

pub(super) fn derive_nyquist(app: &mut RSpiceApp) {
    let (frequency, real, imaginary) = ac_complex_trace(app);
    let provenance = in_flight_cache_provenance(app);
    app.state.results.analysis.nyquist_state.load_data(
        rspice_results::nyquist::NyquistData::from_arrays("V(out)", &frequency, &real, &imaginary),
    );
    app.state
        .results
        .analysis
        .bind_specialized_viewer_cache(ActiveViewer::Nyquist, provenance);
}
