//! Console rendering and diagnostic-projection contract tests.

use super::*;
use crate::diagnostics::LogSource;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
use crate::workbench::AppState;
use crate::workbench::state::WorkbenchState;
use rspice_design::drc::{DrcResult, DrcSeverity};

#[test]
fn console_empty_hints_match_mockup_spacing_and_type_scale() {
    assert_eq!(CONSOLE_FONT_SIZE, 12.0);
    assert_eq!(CONSOLE_FONT_SIZE, tokens::FS_1);
}

/// A page with nothing to list is inset by the body padding and nothing
/// else: its line stands at the panel's own left edge, on the row the first
/// row would have taken. The hint used to carry a 12 × 20 inset of its own,
/// which floated it inside a panel every other page kept tight.
#[test]
fn an_empty_page_starts_where_its_first_row_would() {
    fn scan(shape: &egui::epaint::Shape, found: &mut Option<egui::Pos2>) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                if found.is_none_or(|best: egui::Pos2| text.pos.y < best.y) {
                    *found = Some(text.pos);
                }
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    scan(shape, found);
                }
            }
            _ => {}
        }
    }

    for page in ConsolePage::ALL {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.console_visible = true;
        app.state.workbench.console_page = page;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let size = egui::Vec2::new(900.0, 240.0);
        let mut run = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ui| {
                    let layout = crate::workbench::layout::LayoutSpec::resolve(
                        1280.0,
                        760.0,
                        &app.state.workbench,
                    );
                    super::show(ui, &mut app, layout);
                },
            )
        };
        // Sizing settles on the second pass; the third is the one measured.
        let _ = run();
        let _ = run();
        let output = run();
        let mut found = None;
        for clipped in &output.shapes {
            // The tab strip paints above the body; only body ink is in scope.
            if clipped.shape.visual_bounding_rect().top() < CONSOLE_HEADER_HEIGHT {
                continue;
            }
            scan(&clipped.shape, &mut found);
        }
        let hint = found.unwrap_or_else(|| panic!("{page:?} painted no empty line"));
        assert_eq!(
            hint.x, CONSOLE_BODY_PADDING_X,
            "{page:?} insets its empty line past the body padding"
        );
        let body_top = CONSOLE_HEADER_HEIGHT + CONSOLE_BODY_PADDING_TOP;
        assert!(
            (body_top..body_top + CONSOLE_ROW_MIN_HEIGHT).contains(&hint.y),
            "{page:?} drops its empty line to {} instead of the first row at {body_top}",
            hint.y
        );
    }
}

/// Only a warning or an error earns a mark, and the mark is the line's only
/// colour: notes, debug and trace lines are told apart by neutral ink alone.
#[test]
fn only_warnings_and_errors_earn_a_problem_mark() {
    assert_eq!(log_problem(LogSeverity::Error), Some(Problem::Failure));
    assert_eq!(log_problem(LogSeverity::Warning), Some(Problem::Warning));
    for quiet in [LogSeverity::Info, LogSeverity::Debug, LogSeverity::Trace] {
        assert_eq!(log_problem(quiet), None);
    }

    assert_eq!(drc_problem(DrcSeverity::Critical), Some(Problem::Failure));
    assert_eq!(drc_problem(DrcSeverity::Error), Some(Problem::Failure));
    assert_eq!(drc_problem(DrcSeverity::Warning), Some(Problem::Warning));
    assert_eq!(drc_problem(DrcSeverity::Info), None);

    let palette = Tokens::default().color;
    let neutral = [palette.text, palette.text_dim, palette.text_faint];
    for severity in [
        LogSeverity::Error,
        LogSeverity::Warning,
        LogSeverity::Info,
        LogSeverity::Debug,
        LogSeverity::Trace,
    ] {
        assert!(
            neutral.contains(&log_message_color(&Tokens::default(), severity)),
            "{severity:?} messages must be painted in neutral ink"
        );
    }
}

/// Every console row is painted in neutral ink. The one colour a log, problem
/// or task row may carry is its leading mark, and a pass, a note or a
/// lifecycle word carries none.
#[test]
fn console_rows_paint_no_coloured_text() {
    fn text_colors(shape: &egui::epaint::Shape, colors: &mut Vec<egui::Color32>) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                colors.push(text.fallback_color);
                colors.extend(text.galley.job.sections.iter().map(|s| s.format.color));
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    text_colors(shape, colors);
                }
            }
            _ => {}
        }
    }

    let mut state = AppState::default();
    state.log_buffer.clear();
    state.log_buffer.log(
        LogSeverity::Error,
        LogSource::Simulation,
        "solve failed",
        None,
    );
    state.log_buffer.log(
        LogSeverity::Warning,
        LogSource::Simulation,
        "gmin stepping",
        None,
    );
    state.log_buffer.log(
        LogSeverity::Info,
        LogSource::Simulation,
        "run complete",
        None,
    );

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| console(ui, &mut state));
    });
    let mut colors = Vec::new();
    for clipped in &output.shapes {
        text_colors(&clipped.shape, &mut colors);
    }
    let palette = Tokens::default().color;
    assert!(!colors.is_empty(), "the console painted no text");
    for color in colors {
        assert!(
            [palette.text, palette.text_dim, palette.text_faint].contains(&color)
                || color == egui::Color32::PLACEHOLDER,
            "console text painted in {color:?}, not a neutral ink"
        );
    }
}

#[test]
fn netlist_problem_badge_is_owned_only_by_the_canonical_collection() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    app.state.ui.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    app.state
        .log_buffer
        .warning(LogSource::Simulation, "unrelated retained log warning");
    app.state.dialogs.drc_results = Some(DrcResult::new());
    app.state.ui.netlist.diagnostics = std::sync::Arc::new(
        crate::workbench::documents::netlist_document::NetlistDiagnosticCollection::try_new(
            vec![
                crate::workbench::documents::netlist_document::Diagnostic::error(
                    "bad netlist card",
                ),
            ],
            "",
        )
        .unwrap(),
    );

    assert_eq!(active_problem_count(&app.state), 1);
}

#[test]
fn netlist_first_results_keep_the_canonical_problem_badge() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let provenance = crate::state::AnalysisResultProvenance::new_with_source_domain(
        crate::state::AnalysisResultSourceDomain::ManualDeck,
        crate::product::AnalysisInstanceId::new(),
        crate::product::ObjectRevision::INITIAL,
        crate::product::ContentDigest::from_bytes([0x52; 32]),
        Vec::new(),
    )
    .unwrap();
    app.state.simulation.start_run().add_analysis(
        crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "manual transient",
        )
        .with_provenance(provenance),
    );
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    app.state
        .log_buffer
        .warning(LogSource::Simulation, "unrelated retained log warning");
    app.state.dialogs.drc_results = Some(DrcResult::new());

    assert!(!app.state.is_netlist_first_without_schematic());
    assert!(app.state.active_result_uses_manual_deck());
    assert!(netlist_diagnostics_own_problems(&app.state));
    assert_eq!(active_problem_count(&app.state), 0);
}

#[test]
fn clear_affordance_is_truthful_for_every_console_page() {
    let console = console_clear_action(ConsolePage::Console, true);
    assert!(console.enabled);
    assert_eq!(console.label, "Clear console output");
    assert!(!console_clear_action(ConsolePage::Console, false).enabled);

    for page in [
        ConsolePage::Problems,
        ConsolePage::Measurements,
        ConsolePage::TaskLog,
    ] {
        let action = console_clear_action(page, true);
        assert!(!action.enabled);
        assert_ne!(action.label, "Clear console output");
    }
}

#[test]
fn the_clear_control_clears_the_visible_page_through_its_command() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .log_buffer
        .warning(LogSource::Simulation, "visible console warning");

    app.state.workbench.console_page = ConsolePage::Problems;
    assert!(!Command::ClearConsole.is_enabled(&app));
    Command::ClearConsole.execute(&mut app);
    assert!(!app.state.log_buffer.is_empty());

    app.state.workbench.console_page = ConsolePage::Console;
    assert!(Command::ClearConsole.is_enabled(&app));
    Command::ClearConsole.execute(&mut app);
    assert!(app.state.log_buffer.is_empty());
    assert!(!Command::ClearConsole.is_enabled(&app));
}

#[test]
fn clearing_console_preserves_diagnostics_measurements_and_run_history() {
    let mut state = AppState::default();
    state
        .log_buffer
        .warning(LogSource::Simulation, "visible console warning");
    state.dialogs.drc_results = Some(DrcResult::new());

    let measurement = rspice_core::MeasureResult::success("gain", 42.0);
    let analysis =
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![measurement]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.simulation.retained.runs.push(run);

    state.clear_primary_log();

    assert!(state.log_buffer.is_empty());
    assert!(state.dialogs.drc_results.is_some());
    assert_eq!(state.simulation.retained.runs.len(), 1);
    assert_eq!(
        state.simulation.retained.runs[0].analyses[0]
            .measurements
            .len(),
        1
    );
}

#[test]
fn measurement_table_uses_project_specification_for_verdict_and_margin() {
    let measurement = rspice_core::MeasureResult::success("gain", 9.5);
    let spec = crate::state::SpecEntry {
        measurement: "GAIN".to_owned(),
        expression: String::new(),
        min: Some(9.0),
        max: Some(10.0),
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    let row = measurement_table_row(&measurement, "AC", Some(&spec));
    assert_eq!(row.status, "PASS");
    assert!(!row.failed, "a pass is neutral");
    assert_eq!(row.specification, "9.000000 … 10.000000");
    assert_eq!(row.margin, "+500.000000 m");
    assert_eq!(row.worst_point, "AC");

    let failed = rspice_core::MeasureResult::success("gain", 10.25);
    let row = measurement_table_row(&failed, "AC", Some(&spec));
    assert_eq!(row.status, "FAIL");
    assert!(row.failed);
    assert_eq!(row.margin, "-250.000000 m");
}

#[test]
fn measurement_table_presents_failvalue_against_the_raw_value() {
    let passing = rspice_core::MeasureResult {
        name: "peak_at".to_owned(),
        value: Some(20.0),
        raw_value: Some(3.0),
        error: None,
        passed: true,
        expected: None,
        tolerance: None,
        failure_limit: Some(4.0),
        failure_limit_exceeded: false,
        event_axis: Some(20.0),
        units: None,
    };
    let row = measurement_table_row(&passing, "TRAN", None);
    assert_eq!(row.status, "PASS");
    assert_eq!(
        row.value, "20.000000",
        "the published axis remains the value"
    );
    assert_eq!(row.specification, "|raw| < 4.000000");
    assert_eq!(
        row.margin, "+1.000000",
        "the margin uses raw=3, not value=20"
    );

    let failing_at_limit = rspice_core::MeasureResult {
        raw_value: Some(-4.0),
        passed: false,
        error: Some("FAILVALUE reached".to_owned()),
        failure_limit_exceeded: true,
        ..passing
    };
    let row = measurement_table_row(&failing_at_limit, "TRAN", None);
    assert_eq!(row.status, "FAIL");
    assert!(row.failed);
    assert_eq!(
        row.margin, "-0.000000",
        "inclusive FAILVALUE equality must retain a failing sign"
    );
}

#[test]
fn measurement_table_combines_every_contract_and_reports_the_limiting_margin() {
    let spec = crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(9.0),
        max: Some(11.0),
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    let passing = rspice_core::MeasureResult {
        name: "gain".to_owned(),
        value: Some(10.0),
        raw_value: Some(1.0),
        error: None,
        passed: true,
        expected: Some(10.0),
        tolerance: Some(2.0),
        failure_limit: Some(5.0),
        failure_limit_exceeded: false,
        event_axis: None,
        units: None,
    };
    let row = measurement_table_row(&passing, "AC", Some(&spec));
    assert_eq!(row.status, "PASS");
    assert_eq!(
        row.margin, "+1.000000",
        "the project upper bound limits clearance"
    );
    assert!(row.specification.contains("PROJECT 9.000000 … 11.000000"));
    assert!(row.specification.contains("GOAL 10.000000 ± 2.000000"));
    assert!(row.specification.contains("FAILVALUE |raw| < 5.000000"));

    let authored_failure = rspice_core::MeasureResult {
        raw_value: Some(6.0),
        error: Some("FAILVALUE reached".to_owned()),
        passed: false,
        failure_limit_exceeded: true,
        ..passing.clone()
    };
    let row = measurement_table_row(&authored_failure, "AC", Some(&spec));
    assert_eq!(row.status, "FAIL");
    assert_eq!(
        row.margin, "-1.000000",
        "a passing project bound must not hide the authored failure margin"
    );

    let project_failure = rspice_core::MeasureResult {
        value: Some(12.0),
        raw_value: Some(1.0),
        expected: Some(12.0),
        ..passing
    };
    let row = measurement_table_row(&project_failure, "AC", Some(&spec));
    assert_eq!(row.status, "FAIL");
    assert_eq!(row.margin, "-1.000000");
}

/// Lifecycle words are states, not severities: only a failed run is marked.
#[test]
fn task_lifecycle_copy_distinguishes_cancelled_failed_and_interrupted() {
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Aborted),
        ("CANCELLED", None)
    );
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Failed),
        ("FAIL", Some(Problem::Failure))
    );
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Interrupted),
        ("INTERRUPTED", None)
    );
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Completed),
        ("DONE", None)
    );
}

#[test]
fn measurement_badge_counts_only_the_active_immutable_dataset() {
    let mut simulation = SimulationState::default();
    let first = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![
        rspice_core::MeasureResult::success("gain", 42.0),
        rspice_core::MeasureResult::success("phase", 60.0),
    ]);
    let second = AnalysisResult::new(2, AnalysisType::Transient, "TRAN")
        .with_measurements(vec![rspice_core::MeasureResult::success("rise", 1.0e-9)]);
    let mut retained = SimulationRun::new(1);
    retained.add_analysis(first);
    retained.add_analysis(second);
    simulation.retained.runs.push(retained);

    assert_eq!(active_measurement_count(&simulation), 0);
    simulation.view.active_run_idx = Some(0);
    assert_eq!(active_measurement_count(&simulation), 3);
    simulation.view.active_run_idx = Some(9);
    assert_eq!(active_measurement_count(&simulation), 0);
}

#[test]
fn fractional_engine_progress_is_rendered_as_a_percentage() {
    assert_eq!(simulation_progress_percent(0.0), 0);
    assert_eq!(simulation_progress_percent(0.375), 38);
    assert_eq!(simulation_progress_percent(1.0), 100);
}

#[test]
fn console_chrome_and_grid_geometry_match_the_mockup() {
    assert_eq!(CONSOLE_HEADER_HEIGHT, 31.0);
    assert_eq!(CONSOLE_TAB_HEIGHT, 26.0);
    assert_eq!(CONSOLE_TOUCH_HEADER_HEIGHT, 44.0);
    assert_eq!(CONSOLE_ACTION_SIZE, 27.0);
    assert_eq!(CONSOLE_ACTION_MARGIN_RIGHT, 3.0);
    assert_eq!(CONSOLE_BODY_PADDING_TOP, 7.0);
    assert_eq!(CONSOLE_BODY_PADDING_BOTTOM, 7.0);
    assert_eq!(CONSOLE_BODY_PADDING_X, 10.0);
    assert_eq!(CONSOLE_TIME_WIDTH, 58.0);
    assert_eq!(CONSOLE_SOURCE_WIDTH, 62.0);
    assert_eq!(CONSOLE_COLUMN_GAP, 9.0);
    assert_eq!(CONSOLE_ROW_MIN_HEIGHT, 16.0);
    assert_eq!(CONSOLE_FONT_SIZE, 12.0);
    assert_eq!(CONSOLE_FONT_SIZE, tokens::FS_1);
}

#[test]
fn console_tab_lane_reserves_every_visible_trailing_action() {
    let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
    for page in ConsolePage::ALL {
        assert_eq!(console_trailing_actions_width(desktop, page, false), 90.0);
    }

    let phone = LayoutSpec::resolve_with_pointer(390.0, 844.0, true, &WorkbenchState::default());
    for page in ConsolePage::ALL {
        assert_eq!(console_trailing_actions_width(phone, page, true), 47.0);
    }

    let tablet = LayoutSpec::resolve_with_pointer(1_024.0, 768.0, true, &WorkbenchState::default());
    for page in ConsolePage::ALL {
        assert_eq!(console_trailing_actions_width(tablet, page, true), 94.0);
    }
}

/// Collect text from the same rendering path used by the console and its header.
fn painted_console_body(mut render: impl FnMut(&mut egui::Ui)) -> String {
    fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                rendered.push_str(&text.galley.job.text);
                rendered.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, rendered);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| render(ui));
        },
    );
    let mut rendered = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rendered);
    }
    rendered
}

#[test]
fn run_outcome_header_uses_selected_history_lifecycle() {
    use crate::state::SimulationRunLifecycle as Lifecycle;

    let mut observed = Vec::new();
    let mut expected = Vec::new();
    for (lifecycle, label) in [
        (Lifecycle::Completed, "completed"),
        (Lifecycle::Failed, "failed"),
        (Lifecycle::Aborted, "cancelled"),
        (Lifecycle::Interrupted, "interrupted"),
        (Lifecycle::LegacyUnknown, "legacy status unknown"),
    ] {
        let mut app = RSpiceApp::test_instance();
        let historical = app.state.simulation.start_run();
        historical.label = "Historical operating point".to_owned();
        historical.success = lifecycle == Lifecycle::Completed;
        historical.restore_lifecycle(lifecycle, 1.25).unwrap();
        app.state.simulation.start_run().mark_running().unwrap();
        app.state.simulation.retained.runs[0]
            .finish_lifecycle(Lifecycle::Completed)
            .unwrap();
        app.state.simulation.view.active_run_idx = Some(1);
        observed.push(painted_console_body(|ui| console_context(ui, &app)));
        expected.push(format!(
            "Selected result · Historical operating point · {label} · 1.25 s elapsed\n"
        ));
    }
    assert_eq!(observed, expected);
}

/// A narrowed console shows only the producer's entries, says how much of
/// the log that is, and offers the one control that puts the rest back.
#[test]
fn a_producer_filter_narrows_the_console_and_states_what_it_hid() {
    let mut app = RSpiceApp::test_instance();
    app.state.log_buffer.clear();
    app.state.log_buffer.log(
        crate::diagnostics::LogSeverity::Info,
        LogSource::Simulation,
        "  gain = 1.234000e1",
        None,
    );
    app.state.log_buffer.log(
        crate::diagnostics::LogSeverity::Info,
        LogSource::Simulation,
        "Transient: 512 points, 3 waveforms",
        None,
    );
    app.state.workbench.console_producer_filter =
        Some(crate::workbench::state::ConsoleProducerFilter::new(
            "dataset/7/analysis/3/artifact/gain",
            "gain",
        ));

    let rendered = painted_console_body(|ui| console(ui, &mut app.state));
    assert!(
        rendered.contains("PRODUCER · gain · 1 of 2 entries"),
        "the strip must state the producer and how much of the log it keeps:\n{rendered}"
    );
    assert!(rendered.contains("gain = 1.234000e1"), "{rendered}");
    assert!(
        !rendered.contains("Transient: 512 points"),
        "an entry that is not this producer's must be filtered out:\n{rendered}"
    );
    assert!(rendered.contains("Show all entries"), "{rendered}");
    assert!(
        !app.state
            .workbench
            .console_producer_filter
            .as_ref()
            .expect("the filter survives the frame")
            .scroll_to_newest,
        "the one-shot scroll request is consumed by the frame that honours it"
    );
}

/// Nothing matching is a fact about the log, and the console says which
/// fact rather than looking like an empty session.
#[test]
fn an_unmatched_producer_says_why_the_console_looks_empty() {
    let mut app = RSpiceApp::test_instance();
    app.state.log_buffer.clear();
    app.state.log_buffer.log(
        crate::diagnostics::LogSeverity::Info,
        LogSource::Simulation,
        "Transient: 512 points, 3 waveforms",
        None,
    );
    app.state.workbench.console_producer_filter =
        Some(crate::workbench::state::ConsoleProducerFilter::new(
            "dataset/7/analysis/3/quantity/V(out)",
            "V(out)",
        ));

    let rendered = painted_console_body(|ui| console(ui, &mut app.state));
    assert!(
        rendered.contains("No console entry names V(out)"),
        "{rendered}"
    );
    assert!(
        rendered.contains("carry no producer tag"),
        "the empty state names the reason, not just the absence:\n{rendered}"
    );
    assert!(
        !rendered.contains("not yet"),
        "the empty state states a present limitation, never a future promise:\n{rendered}"
    );
}

#[test]
fn measurement_units_keep_console_project_and_authored_contracts_distinct() {
    use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};
    let mut measured = rspice_core::MeasureResult::success("level", 0.25);
    measured.units = Some(MeasurementUnits {
        value: MeasurementUnit::Known("V".into()),
        raw_value: MeasurementUnit::Known("V".into()),
        axis: MeasurementUnit::Known("s".into()),
    });
    measured.expected = Some(0.25);
    measured.tolerance = Some(0.01);
    measured.failure_limit = Some(0.5);
    let mut spec = crate::state::SpecEntry {
        measurement: "level".into(),
        expression: String::new(),
        min: Some(200.0),
        max: Some(300.0),
        unit: "mV".into(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    let row = measurement_table_row(&measured, "TRAN", Some(&spec));
    assert_eq!(row.status, "PASS");
    assert!(
        row.value.starts_with("250") && row.value.ends_with(" mV"),
        "{}",
        row.value
    );
    assert!(
        row.margin.contains("PROJECT +50") && row.margin.contains("mV"),
        "{}",
        row.margin
    );
    assert!(
        row.margin.contains("GOAL") && row.margin.contains(" V"),
        "{}",
        row.margin
    );
    spec.unit = "mA".into();
    let row = measurement_table_row(&measured, "TRAN", Some(&spec));
    assert_eq!(row.status, "ERROR");
    assert!(row.margin.contains("incompatible"));
}
