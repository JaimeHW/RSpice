//! Console rendering and diagnostic-projection contract tests.

use super::*;
use crate::diagnostics::LogSource;
use crate::services::drc::{DrcResult, DrcSeverity};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
use crate::workbench::AppState;
use crate::workbench::state::WorkbenchState;

#[test]
fn console_empty_hints_match_mockup_spacing_and_type_scale() {
    assert_eq!(EMPTY_HINT_PADDING_X, 12);
    assert_eq!(EMPTY_HINT_PADDING_Y, 20);
    assert_eq!(CONSOLE_FONT_SIZE, 12.0);
    assert_eq!(CONSOLE_FONT_SIZE, tokens::FS_1);
}

#[test]
fn severity_tones_preserve_warning_info_and_diagnostic_meaning() {
    assert_eq!(log_tone(LogSeverity::Error), SemanticTone::Error);
    assert_eq!(log_tone(LogSeverity::Warning), SemanticTone::Warning);
    assert_eq!(log_tone(LogSeverity::Info), SemanticTone::Info);
    assert_eq!(log_tone(LogSeverity::Debug), SemanticTone::Debug);
    assert_eq!(log_tone(LogSeverity::Trace), SemanticTone::Trace);

    assert_eq!(drc_tone(DrcSeverity::Critical), SemanticTone::Error);
    assert_eq!(drc_tone(DrcSeverity::Error), SemanticTone::Error);
    assert_eq!(drc_tone(DrcSeverity::Warning), SemanticTone::Warning);
    assert_eq!(drc_tone(DrcSeverity::Info), SemanticTone::Info);
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
    app.state.project_lifecycle.project_open = true;
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

    let interactive = console_clear_action(ConsolePage::Interactive, true);
    assert!(interactive.enabled);
    assert_eq!(interactive.label, "Clear interactive command history");
    assert!(!console_clear_action(ConsolePage::Interactive, false).enabled);

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
    app.state.script_console.history.push(ConsoleHistoryItem {
        command: "help".to_owned(),
        output: Default::default(),
    });

    app.state.workbench.console_page = ConsolePage::Interactive;
    assert!(
        console_clear_action(
            ConsolePage::Interactive,
            Command::ClearConsole.is_enabled(&app)
        )
        .enabled,
        "the painted control and the command must agree on what is clearable"
    );
    Command::ClearConsole.execute(&mut app);
    assert!(app.state.script_console.history.is_empty());
    assert!(
        !app.state.log_buffer.is_empty(),
        "clearing the interactive page must not reach output the user cannot see"
    );

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
    state.script_console.input_buffer = "pending command".to_owned();
    state.script_console.history.push(ConsoleHistoryItem {
        command: "help".to_owned(),
        output: Default::default(),
    });
    state.dialogs.drc_results = Some(DrcResult::new());

    let measurement = rspice_core::MeasureResult::success("gain", 42.0);
    let analysis =
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![measurement]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.simulation.runs.push(run);

    state.clear_primary_log();
    state.script_console.history.clear();

    assert!(state.log_buffer.is_empty());
    assert!(state.script_console.history.is_empty());
    assert_eq!(state.script_console.input_buffer, "pending command");
    assert!(state.dialogs.drc_results.is_some());
    assert_eq!(state.simulation.runs.len(), 1);
    assert_eq!(state.simulation.runs[0].analyses[0].measurements.len(), 1);
}

#[test]
fn interactive_console_uses_the_governed_command_dispatcher() {
    let mut app = RSpiceApp::test_instance();
    let navigator_was_visible = app.state.workbench.navigator_visible;
    app.state.script_console.input_buffer = "command toggle-navigator".to_owned();

    assert!(submit_interactive_command(&mut app));
    assert!(app.state.script_console.input_buffer.is_empty());
    assert_eq!(app.state.script_console.history.len(), 1);
    assert_eq!(
        app.state.script_console.history[0].command,
        "command toggle-navigator"
    );
    assert!(app.state.script_console.history[0].output.success);
    assert_ne!(
        app.state.workbench.navigator_visible, navigator_was_visible,
        "the typed command must execute through the real workbench dispatcher"
    );

    app.state.script_console.input_buffer = "   ".to_owned();
    assert!(!submit_interactive_command(&mut app));
    assert_eq!(app.state.script_console.history.len(), 1);
}

#[test]
fn interactive_console_rejects_unknown_private_and_unavailable_commands() {
    let mut app = RSpiceApp::test_instance();

    let unknown = execute_interactive_command("command no-such-command", &mut app);
    assert!(!unknown.success);
    assert!(unknown.message.contains("Unknown command ID"));

    let fuzzy = execute_interactive_command("command Toggle Navigator", &mut app);
    assert!(!fuzzy.success);
    assert!(
        fuzzy
            .message
            .contains("requires exactly one canonical stable ID")
    );

    let private = execute_interactive_command("command console-clear", &mut app);
    assert!(!private.success);
    assert!(private.message.contains("private to application chrome"));

    let unavailable = execute_interactive_command("command stop-run", &mut app);
    assert!(!unavailable.success);
    assert!(unavailable.message.contains("unavailable"));

    let catalog = execute_interactive_command("commands navigator", &mut app);
    assert!(catalog.success);
    assert!(
        catalog
            .data
            .as_deref()
            .is_some_and(|data| data.contains("toggle-navigator"))
    );

    let hidden = unavailable_command_output("future-context-command", CommandAvailability::Hidden)
        .expect("hidden commands are rejected");
    assert!(!hidden.success);
    assert!(hidden.message.contains("hidden in the current context"));
}

#[test]
fn interactive_console_reads_exact_retained_measurement_and_plan_state() {
    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC")
        .with_measurements(vec![rspice_core::MeasureResult::success("gain_dc", 42.0)]);
    let mut run = SimulationRun::new(7);
    run.add_analysis(analysis);
    app.state.simulation.runs.push(run);

    let measurement =
        execute_interactive_command("project.results[\"Run 7\"].measure(\"gain_dc\")", &mut app);
    assert!(measurement.success);
    assert!(measurement.message.contains("immutable Run 7"));
    assert!(
        measurement
            .data
            .as_deref()
            .is_some_and(|data| data.contains("42.000000"))
    );

    let plan = execute_interactive_command("plan.analyses.enabled", &mut app);
    assert!(plan.success);
    assert!(
        plan.data
            .as_deref()
            .is_some_and(|data| data.contains("tran"))
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
    assert_eq!(row.tone, SemanticTone::Success);
    assert_eq!(row.specification, "9.000000 … 10.000000");
    assert_eq!(row.margin, "+500.000000 m");
    assert_eq!(row.worst_point, "AC");

    let failed = rspice_core::MeasureResult::success("gain", 10.25);
    let row = measurement_table_row(&failed, "AC", Some(&spec));
    assert_eq!(row.status, "FAIL");
    assert_eq!(row.tone, SemanticTone::Error);
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
    assert_eq!(row.tone, SemanticTone::Error);
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

#[test]
fn task_lifecycle_copy_distinguishes_cancelled_failed_and_interrupted() {
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Aborted),
        ("CANCELLED", SemanticTone::Warning)
    );
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Failed),
        ("FAIL", SemanticTone::Error)
    );
    assert_eq!(
        run_lifecycle_presentation(crate::state::SimulationRunLifecycle::Interrupted),
        ("INTERRUPTED", SemanticTone::Warning)
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
    simulation.runs.push(retained);

    assert_eq!(active_measurement_count(&simulation), 0);
    simulation.active_run_idx = Some(0);
    assert_eq!(active_measurement_count(&simulation), 3);
    simulation.active_run_idx = Some(9);
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

/// Render the Console page and collect the text it painted.
fn painted_console(state: &mut AppState) -> String {
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
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        },
    );
    let mut rendered = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rendered);
    }
    rendered
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

    let rendered = painted_console(&mut app.state);
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

    let rendered = painted_console(&mut app.state);
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
