//! Console rendering and diagnostic-projection contract tests.

use super::*;
use crate::diagnostics::LogSource;
use crate::state::{AnalysisResult, AnalysisType, Point, SimulationRun};
use crate::workbench::AppState;
use crate::workbench::app_state::diagnostic_store::DiagnosticOwner;
use crate::workbench::app_state::diagnostic_store::listing::listing;
use crate::workbench::state::WorkbenchState;
use rspice_design::drc::DrcResult;

/// Every producer the store holds evidence for puts a real finding on the
/// page, with a code, a subject and somewhere to go.
///
/// The owners are the store's own list of them rather than a list written
/// here, and the match is exhaustive: a new producer cannot be added to
/// [`DiagnosticOwner`] without this test driving its failure path. Before the
/// canonical store, a producer "reported" a failure by writing a console
/// line, which meant a user-visible failure could ship with nothing on this
/// page at all — and several did.
#[test]
fn every_producer_puts_its_failures_on_the_page_as_canonical_records() {
    use rspice_design::drc::{DrcLocation, DrcViolation, DrcViolationType};

    for owner in DiagnosticOwner::ALL {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        match owner {
            DiagnosticOwner::DesignRules => {
                // A finding about an object the drawing carries, so the row's
                // route is the real one the canvas resolves.
                let resistor = app
                    .state
                    .schematic
                    .add_component(crate::state::ComponentType::Resistor, Point::new(40, 40));
                app.state.sync_active_schematic_to_workspace();
                let mut result = DrcResult::new();
                result.add_violation(DrcViolation::new(
                    1,
                    DrcViolationType::MissingParameter,
                    "R1 has no resistance",
                    DrcLocation::Component {
                        id: resistor,
                        name: "R1".to_owned(),
                    },
                ));
                result.completed = true;
                app.state
                    .publish_active_design_check_result(result)
                    .expect("the checks publish against the live design");
            }
            DiagnosticOwner::SymbolPins => {
                use crate::state::{
                    Cell, CellViewRef, ComponentType, Library, PortDirection, SchematicState,
                    SymbolDocument, SymbolPin, View, ViewType,
                };
                use crate::workbench::state::Workspace;

                // A symbol whose pin contract genuinely fails: its cell's
                // schematic declares a port that the symbol never places.
                let mut library = Library::new("work");
                let mut amp = Cell::new("amp");
                amp.add_view(View::new("schematic", ViewType::Schematic));
                amp.add_view(View::new("symbol", ViewType::Symbol));
                library.add_cell(amp);
                app.state.library_manager.add_library(library);

                let mut schematic = SchematicState::default();
                let port = schematic.add_component(ComponentType::Port, Point::new(0, 0));
                schematic
                    .document_mut_for_test()
                    .components
                    .iter_mut()
                    .find(|component| component.id == port)
                    .expect("the fixture just placed this port")
                    .value = "IN".to_owned();
                app.state.workspace.insert_schematic_editor(
                    CellViewRef::new("work", "amp", "schematic").key(),
                    schematic,
                );
                app.state
                    .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
                app.state
                    .store_active_symbol_document(&SymbolDocument {
                        pins: vec![SymbolPin::new("IN", PortDirection::In, None)],
                        ..SymbolDocument::default()
                    })
                    .expect("the fixture stores the symbol it is about to check");
                app.state.workbench.activate(Workspace::Design);

                // The real command the symbol editor runs, which routes to the
                // pin checks because a symbol view is active.
                Command::CheckCellview.execute(&mut app);
            }
            DiagnosticOwner::Dispatch => {
                // The real command, on an empty design: a schematic with no
                // components cannot be dispatched, so the pass blocks.
                app.state.schematic = crate::state::SchematicState::default();
                Command::PreflightChecks.execute(&mut app);
            }
            DiagnosticOwner::Run => {
                use rspice_results::convergence_attribution::{
                    ConvergenceAttribution, ConvergenceFailureClass, ConvergenceSite,
                    ConvergenceSiteKind,
                };

                let message = "Analysis failed: no DC path at vbias";
                let attribution = ConvergenceAttribution {
                    class: ConvergenceFailureClass::NoDcPathToGround,
                    sites: vec![ConvergenceSite {
                        name: "vbias".to_owned(),
                        kind: ConvergenceSiteKind::Node,
                        residual: None,
                    }],
                    elided_sites: 0,
                    failure_message: "no DC path at vbias".to_owned(),
                };
                app.state
                    .publish_run_failure("Transient", message, Some(&attribution), 1);
            }
            DiagnosticOwner::RuleDeck => {
                // A deck written under a schema this build does not read: none
                // of it is applied, which the team has to know before it signs
                // off under it.
                app.state.workspace.content.design_rules =
                    serde_json::from_str(r#"{"schema_version":2}"#)
                        .expect("a stored deck always reads");
                app.state.publish_rule_deck_issues();
            }
        }

        let batch = app
            .state
            .diagnostics
            .batches()
            .find(|batch| batch.inputs.owner() == owner)
            .unwrap_or_else(|| {
                panic!("{owner:?} reported a failure and published no canonical records")
            })
            .clone();
        assert!(
            !batch.records.is_empty(),
            "{owner:?} reported a failure and published an empty batch"
        );
        for record in &batch.records {
            assert!(
                !record.canonical.code.is_empty(),
                "{owner:?} published a finding with no rule code: {}",
                record.message
            );
            assert!(
                !record.subject.object.is_empty(),
                "{owner:?} published a finding that says nothing about what it is about: {}",
                record.message
            );
        }
        assert!(
            batch.records.iter().any(|record| record.route.is_some()),
            "{owner:?} published findings with nowhere to go at all"
        );

        // And every one of them reaches the page, where a reader looks.
        let listed = listing(&app.state);
        let listed_messages: Vec<&str> = (0..listed.len())
            .filter_map(|index| listed.row(index))
            .map(|row| row.message)
            .collect();
        for record in &batch.records {
            assert!(
                listed_messages.contains(&record.message.as_str()),
                "{owner:?} published {:?}, which the page does not list: {listed_messages:?}",
                record.message
            );
        }
        // A deck that is not applied does not stop a run, so it is a warning,
        // not an error; it still has to count.
        let counted = match owner {
            DiagnosticOwner::RuleDeck => listed.counts().warnings,
            _ => listed.counts().errors,
        };
        assert!(
            counted > 0,
            "{owner:?} reported a failure that counts as nothing: {:?}",
            listed.counts()
        );
    }
}

/// A run that completes with an engine warning puts it on the page, as the
/// run's failures are put there: a record with a code, a subject and
/// somewhere to go, beside the Console's prose rather than instead of it.
///
/// Driven through the real run: the engine clamps the card's `pbs` while it
/// binds `M1`, the runner hands the notice over with the result, and the
/// controller's completion path publishes it. Before this the clamp reached
/// the Console as one line and the page said nothing.
#[test]
fn a_completed_runs_engine_warnings_reach_the_page_as_canonical_records() {
    use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;

    let mut state = crate::simulation::controller::test_execution::run_manual_batch_session(
        "BSIM4 clamp\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L=1u W=10u\n\
         .model nch nmos level=54 version=4.8 pbs=0.05\n.op\n.end\n",
    );
    state.project_lifecycle.authority.open_session();

    let batch = state
        .diagnostics
        .batches()
        .find(|batch| batch.inputs.owner() == DiagnosticOwner::Run)
        .expect("the run published its warnings")
        .clone();
    let clamp = batch
        .records
        .iter()
        .find(|record| record.canonical.code.as_ref() == "MODEL-PARAMETER-CLAMPED")
        .unwrap_or_else(|| panic!("the pbs clamp is a record: {:#?}", batch.records));
    assert_eq!(clamp.level, DiagnosticLevel::Warning);
    assert!(
        clamp.subject.object.eq_ignore_ascii_case("model nch"),
        "the subject is the card, as the engine names it: {}",
        clamp.subject.object
    );
    assert!(
        clamp.message.contains("pbs = 50m") && clamp.message.contains("uses 100m"),
        "the record states the parameter, its value and the value used: {}",
        clamp.message
    );
    assert!(
        clamp.details.contains("M1"),
        "the record names the element that bound the card: {}",
        clamp.details
    );
    assert!(
        clamp.route.is_some(),
        "a manual deck's warning is repaired in the deck"
    );

    let listed = listing(&state);
    assert!(
        (0..listed.len())
            .filter_map(|index| listed.row(index))
            .any(|row| row.message == clamp.message),
        "the page lists the record"
    );
    assert!(
        listed.counts().warnings > 0,
        "a clamp counts as a warning: {:?}",
        listed.counts()
    );
    assert_eq!(
        listed.counts().errors,
        0,
        "a run that completed counts no error"
    );
}

/// Each clamp the engine raised in a real run names the line of that run's
/// log where the engine said so, two clamps in a row each their own, and
/// following one opens the run's log with that line selected and in view.
///
/// The engine site logs the clamp's warning, learns which line the run's sink
/// numbered it, and raises its notice with that number; the controller turns
/// it into the line's place in the run's log. No text is matched anywhere on
/// the way.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_clamp_opens_its_runs_log_on_the_line_the_engine_reported_it_on() {
    use crate::workbench::state::{ConsoleChannel, ConsoleReveal, ConsoleSelection};

    crate::diagnostics::engine_log::studio_logger_for_tests();
    let mut app = RSpiceApp::test_instance();
    app.state = crate::simulation::controller::test_execution::run_manual_batch_session(
        "BSIM4 clamp\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L=1u W=10u\n\
         .model nch nmos level=54 version=4.8 pbs=0.05 pbsws=0.05\n.op\n.end\n",
    );
    app.state.project_lifecycle.authority.open_session();
    let run_sequence = app.state.simulation.retained.runs[0].id;

    for parameter in ["pbs", "pbsws"] {
        let place = app
            .state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter())
            .find(|record| {
                record.canonical.code.as_ref() == "MODEL-PARAMETER-CLAMPED"
                    && record.message.starts_with(&format!("BSIM4 {parameter} = "))
            })
            .map(|clamp| {
                clamp
                    .log_place
                    .unwrap_or_else(|| panic!("the clamp names its line: {clamp:#?}"))
            })
            .unwrap_or_else(|| panic!("the {parameter} clamp is a record"));
        assert_eq!(place.run_sequence, run_sequence);
        let line = &app
            .state
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("the run is retained")
            .log
            .lines()[place.line];
        assert_eq!(line.source, LogSource::Engine);
        assert_eq!(
            line.message,
            format!("BSIM4: given {parameter} is less than 0.1; set to 0.1")
        );

        app.state.diagnostics.request_route(place.route());
        crate::workbench::diagnostic_routes::apply_pending(&egui::Context::default(), &mut app);
        let view = &app.state.workbench.console_view;
        let key = place.line as u64;
        assert_eq!(view.channel, ConsoleChannel::Run(run_sequence));
        assert_eq!(view.selection, Some(ConsoleSelection::single(key)));
        assert_eq!(view.reveal, Some(ConsoleReveal::Line(key)));
    }
}

/// The page reads records. It must not read the session log back in, which is
/// what made it a severity filter over everything that had ever happened.
///
/// Every file the page is made of, not only its entry: the table, the filter
/// strip, the arrangement and the detail region are all the page, and a split
/// that moved the body out from under this guard would have retired it without
/// anyone deciding to.
#[test]
fn the_problems_page_never_names_the_log_buffer() {
    for (file, source) in [
        ("problems.rs", include_str!("problems.rs")),
        ("problems/arrange.rs", include_str!("problems/arrange.rs")),
        ("problems/detail.rs", include_str!("problems/detail.rs")),
        ("problems/grid.rs", include_str!("problems/grid.rs")),
        ("problems/strip.rs", include_str!("problems/strip.rs")),
    ] {
        let production = crate::source_guard::production_source(source);
        for forbidden in ["log_buffer", "LogEntry", "LogSeverity", "LogSource"] {
            assert!(
                !production.contains(forbidden),
                "{file} names `{forbidden}`; a condition is not an event"
            );
        }
    }
}

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
            // An empty field's galley has no extent, so the band filter below
            // cannot place it; it is placed by where it is drawn instead.
            egui::epaint::Shape::Text(text)
                if text.galley.job.text.is_empty() || text.pos.y < CONSOLE_HEADER_HEIGHT => {}
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
    state
        .log_buffer
        .log(LogSeverity::Error, LogSource::Run, "solve failed");
    state
        .log_buffer
        .log(LogSeverity::Warning, LogSource::Run, "gmin stepping");
    state
        .log_buffer
        .log(LogSeverity::Info, LogSource::Run, "run complete");

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| transcript::console(ui, &mut state));
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

/// The badge counts records, and only records.
///
/// It used to count log entries too, and to swap between the netlist summary
/// and the schematic result depending on which workspace was open — so the
/// same session reported a different number of problems from the Netlist page
/// than from the Design page, and a retained log warning counted as one of
/// them forever.
#[test]
fn the_problem_badge_counts_records_and_not_log_entries() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    app.state.ui.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    app.state
        .log_buffer
        .warning(LogSource::Run, "unrelated retained log warning");
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

    assert_eq!(active_problem_count(&app.state, 0).total(), 1);

    // The same count wherever the reader is standing.
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    assert_eq!(active_problem_count(&app.state, 1).total(), 1);
}

#[test]
fn clear_affordance_is_truthful_for_every_console_page() {
    let console = console_clear_action(ConsolePage::Console, true);
    assert!(console.enabled);
    assert_eq!(console.label, "Clear the session log");
    assert!(!console_clear_action(ConsolePage::Console, false).enabled);

    for page in [
        ConsolePage::Problems,
        ConsolePage::Measurements,
        ConsolePage::TaskLog,
    ] {
        let action = console_clear_action(page, true);
        assert!(!action.enabled);
        assert_ne!(action.label, "Clear the session log");
    }
}

#[test]
fn the_clear_control_clears_the_visible_page_through_its_command() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .log_buffer
        .warning(LogSource::Run, "visible console warning");

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

/// A run's log is a record. Its channel offers no Clear, and the command
/// clears nothing from it, whatever the session's log holds.
#[test]
fn a_run_channel_has_no_clear_and_the_command_leaves_the_session_alone() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .log_buffer
        .warning(LogSource::Run, "visible console warning");
    app.state
        .simulation
        .retained
        .runs
        .push(SimulationRun::new(3));
    app.state.workbench.console_page = ConsolePage::Console;
    app.state
        .workbench
        .console_view
        .open(crate::workbench::state::ConsoleChannel::Run(3));

    let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
    assert!(!clear_is_offered(
        desktop,
        ConsolePage::Console,
        app.state.workbench.console_view.channel
    ));
    assert!(!Command::ClearConsole.is_enabled(&app));
    Command::ClearConsole.execute(&mut app);
    assert!(!app.state.log_buffer.is_empty());
}

#[test]
fn clearing_console_preserves_diagnostics_measurements_and_run_history() {
    let mut state = AppState::default();
    state
        .log_buffer
        .warning(LogSource::Run, "visible console warning");
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
    assert_eq!(CONSOLE_COLUMN_GAP, 9.0);
    assert_eq!(CONSOLE_ROW_MIN_HEIGHT, 16.0);
    assert_eq!(CONSOLE_FONT_SIZE, 12.0);
    assert_eq!(CONSOLE_FONT_SIZE, tokens::FS_1);
}

/// The task log's status column is as wide as its widest status, measured in
/// its own font. A 58 px constant clipped "INTERRUPTED".
#[test]
fn the_task_status_column_fits_every_status() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let column = widest_status_width(ui);
            for status in ["INTERRUPTED", "CANCELLING", "CANCELLED", "RUNNING"] {
                let width = task_text_width(ui, status, FontWeight::Regular);
                assert!(
                    width <= column,
                    "{status} is {width} px in a {column} px column"
                );
            }
        });
    });
}

#[test]
fn console_tab_lane_reserves_every_visible_trailing_action() {
    use crate::workbench::state::ConsoleChannel;

    let session = ConsoleChannel::Session;
    let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
    for page in ConsolePage::ALL {
        let clear = clear_is_offered(desktop, page, session);
        assert_eq!(console_trailing_actions_width(desktop, clear, false), 90.0);
    }
    // A run's log has no Clear, and the lane gives its width back to the tabs.
    let run = clear_is_offered(desktop, ConsolePage::Console, ConsoleChannel::Run(1));
    assert_eq!(console_trailing_actions_width(desktop, run, false), 60.0);

    let phone = LayoutSpec::resolve_with_pointer(390.0, 844.0, true, &WorkbenchState::default());
    for page in ConsolePage::ALL {
        let clear = clear_is_offered(phone, page, session);
        assert_eq!(console_trailing_actions_width(phone, clear, true), 47.0);
    }

    let tablet = LayoutSpec::resolve_with_pointer(1_024.0, 768.0, true, &WorkbenchState::default());
    for page in ConsolePage::ALL {
        let clear = clear_is_offered(tablet, page, session);
        assert_eq!(console_trailing_actions_width(tablet, clear, true), 94.0);
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

/// Each channel says why it shows no rows, in terms of that channel: a
/// session with nothing in it, a run that wrote nothing, a find that matches
/// nothing. None of them promises lines that will never come.
#[test]
fn each_channel_says_why_it_shows_no_rows() {
    use crate::workbench::state::ConsoleChannel;

    let mut state = AppState::default();
    state.log_buffer.clear();
    let rendered = painted_console_body(|ui| transcript::console(ui, &mut state));
    assert!(rendered.contains("No studio activity yet."), "{rendered}");
    assert!(!rendered.contains("will appear"), "{rendered}");

    state.simulation.retained.runs.push(SimulationRun::new(4));
    state.workbench.console_view.open(ConsoleChannel::Run(4));
    let rendered = painted_console_body(|ui| transcript::console(ui, &mut state));
    assert!(rendered.contains("This run wrote no log."), "{rendered}");

    state.workbench.console_view.open(ConsoleChannel::Session);
    state
        .log_buffer
        .log(LogSeverity::Info, LogSource::Studio, "Project opened");
    state.workbench.console_view.find = "netlist".to_owned();
    let rendered = painted_console_body(|ui| transcript::console(ui, &mut state));
    assert!(
        rendered.contains("No line contains 'netlist'."),
        "{rendered}"
    );
}

/// The dock's header with the Console page in front, at a dock `width` wide,
/// after enough passes for sizing to settle, with the accessibility tree.
fn console_dock_output(app: &mut RSpiceApp, width: f32) -> egui::FullOutput {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let layout = LayoutSpec::resolve(1_280.0, 760.0, &app.state.workbench);
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(width, 320.0),
                )),
                ..Default::default()
            },
            |ui| super::show(ui, app, layout),
        ));
    }
    output.expect("three passes ran")
}

/// Every published node's role, label and bounds.
fn labelled_bounds(output: &egui::FullOutput) -> Vec<(egui::accesskit::Role, String, egui::Rect)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("accesskit is on")
        .nodes
        .iter()
        .filter_map(|(_, node)| {
            let label = node.label()?.to_owned();
            let bounds = node.bounds()?;
            Some((
                node.role(),
                label,
                egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                ),
            ))
        })
        .collect()
}

/// A session with a dozen runs and lines of every severity: the widest the
/// channel selector and the counts get in an ordinary day.
fn busy_console() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.console_visible = true;
    app.state.workbench.console_page = ConsolePage::Console;
    for sequence in 1..=12 {
        app.state
            .simulation
            .retained
            .runs
            .insert(0, SimulationRun::new(sequence));
    }
    app.state.log_buffer.clear();
    for index in 0..240 {
        app.state.log_buffer.log(
            LogSeverity::Info,
            LogSource::Studio,
            format!("event {index}"),
        );
    }
    for _ in 0..12 {
        app.state
            .log_buffer
            .log(LogSeverity::Warning, LogSource::Netlist, "gmin stepping");
    }
    for _ in 0..3 {
        app.state
            .log_buffer
            .log(LogSeverity::Error, LogSource::Studio, "Export failed");
    }
    app
}

/// The page's controls share the header band with the tabs and the dock's
/// actions: one row, beside the tabs and short of the actions, at a 900 px
/// dock.
#[test]
fn the_console_toolbar_fits_one_row_of_a_900_px_dock() {
    let mut app = busy_console();
    let nodes = labelled_bounds(&console_dock_output(&mut app, 900.0));
    let find = |label: &str| {
        nodes
            .iter()
            .find(|(_, candidate, _)| candidate.starts_with(label))
            .map(|(_, _, bounds)| *bounds)
            .unwrap_or_else(|| panic!("no {label:?} control: {nodes:?}"))
    };
    let tabs_end = nodes
        .iter()
        .filter(|(role, ..)| *role == egui::accesskit::Role::Tab)
        .map(|(_, _, bounds)| bounds.right())
        .fold(0.0, f32::max);
    assert!(tabs_end > 0.0, "no tab was published: {nodes:?}");
    let clear = find("Clear the session log");
    let controls = [
        find("Console channel"),
        find("Errors, 3"),
        find("Warnings, 12"),
        find("Info, 240"),
        find("Find in the log"),
    ];
    for control in controls {
        assert!(
            control.top() >= 0.0 && control.bottom() <= CONSOLE_HEADER_HEIGHT,
            "{control:?} leaves the {CONSOLE_HEADER_HEIGHT} px band"
        );
        assert!(control.left() >= tabs_end, "{control:?} overlaps the tabs");
    }
    let field = find("Find in the log");
    assert!(
        field.right() <= clear.left(),
        "the find field {field:?} runs under Clear {clear:?}"
    );
    assert!(field.width() >= 60.0, "the find field is cut to {field:?}");
}

/// Renders of the page for review: the session with every kind of line, a
/// run's channel ending in its withheld-lines row, and a find with no match.
/// `RSPICE_REVIEW_DIR` says where they go.
#[test]
#[ignore = "writes review renders for a person to look at"]
fn console_page_review_renders() {
    use crate::diagnostics::LogAnchor;
    use crate::workbench::state::ConsoleChannel;

    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let mut app = busy_console();
    app.state.log_buffer.clear();
    let lines: [(LogSeverity, LogSource, String); 5] = [
        (
            LogSeverity::Info,
            LogSource::Studio,
            "Project opened: amplifier".to_owned(),
        ),
        (
            LogSeverity::Warning,
            LogSource::Netlist,
            "Model nch has no binning range for W=0.4u; the nearest bin is used".to_owned(),
        ),
        (
            LogSeverity::Error,
            LogSource::Studio,
            format!(
                "Could not read C:/projects/{}/netlist.cir",
                "deep/".repeat(55)
            ),
        ),
        (
            LogSeverity::Info,
            LogSource::Run,
            "Run 12 started: Transient, AC".to_owned(),
        ),
        (
            LogSeverity::Error,
            LogSource::Run,
            "Run 12 failed after 0.42 s: timestep too small".to_owned(),
        ),
    ];
    for (severity, source, message) in lines {
        let anchor =
            (source == LogSource::Run).then_some(LogAnchor::ResultRun { run_sequence: 12 });
        app.state
            .log_buffer
            .log_anchored(severity, source, message, anchor);
    }
    let selected = app.state.log_buffer.entries().nth(1).map(|entry| entry.id);
    app.state.workbench.console_view.selection =
        selected.map(crate::workbench::state::ConsoleSelection::single);
    let layout = LayoutSpec::resolve(1_280.0, 760.0, &app.state.workbench);
    let size = Vec2::new(900.0, 320.0);
    let render = |name: &str, app: &mut RSpiceApp| {
        let canvas = crate::ui::raster::render(size, |ui, _| super::show(ui, app, layout));
        let path = directory.join(format!("console-{name}.png"));
        std::fs::write(&path, canvas.png(size.y as usize)).expect("the review render is written");
    };
    render("session", &mut app);

    let run = app
        .state
        .simulation
        .retained
        .run_by_sequence_mut(12)
        .expect("the fixture holds run 12");
    for (severity, message) in [
        (LogSeverity::Info, "Run 12 \u{b7} RSpice \u{b7} amplifier"),
        (
            LogSeverity::Warning,
            "timestep too small at 1.204 us; halving",
        ),
        (
            LogSeverity::Info,
            "Transient: 0.42 s, 1,204 steps accepted, 37 rejected",
        ),
    ] {
        run.log.push(rspice_app_types::run_log::RunLogLine {
            unix_ms: 1_760_000_000_000,
            severity,
            source: LogSource::Run,
            message: message.to_owned(),
        });
    }
    run.log.absorb_engine_seal(5_000, 0);
    run.log.seal(1_760_000_000_000);
    app.state
        .workbench
        .console_view
        .open(ConsoleChannel::Run(12));
    render("run", &mut app);

    app.state
        .workbench
        .console_view
        .open(ConsoleChannel::Session);
    app.state.workbench.console_view.find = "vdd_core".to_owned();
    render("find-empty", &mut app);
}

/// A ring that has dropped its oldest entries says so in its first row, so a
/// truncated session never reads as the whole of it.
#[test]
fn a_session_that_dropped_entries_says_so_first() {
    let mut state = AppState::default();
    state.log_buffer = crate::diagnostics::LogBuffer::new(3);
    for index in 0..5 {
        state.log_buffer.log(
            LogSeverity::Info,
            LogSource::Studio,
            format!("event {index}"),
        );
    }
    let rendered = painted_console_body(|ui| transcript::console(ui, &mut state));
    let first = rendered.lines().next().unwrap_or_default();
    assert_eq!(
        first, "Earlier entries were removed (the Console keeps the latest 3)",
        "{rendered}"
    );
    assert!(!rendered.contains("event 1"), "{rendered}");
    assert!(rendered.contains("event 4"), "{rendered}");
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
