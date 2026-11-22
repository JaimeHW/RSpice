//! Console rendering and diagnostic-projection contract tests.

use egui::Vec2;

use crate::workbench::AppState;
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::ShellCommand;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, Point, SimulationRun, SimulationState,
};
use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticOwner;
use rspice_app_state::workbench::app_state::diagnostic_store::listing::listing;
use rspice_app_workbench::ui::tokens::Tokens;
use rspice_app_workbench::ui::widgets::Problem;
use rspice_app_workbench::workbench::docks::console::problems::active_problem_count;
use rspice_app_workbench::workbench::docks::console::*;
use rspice_app_workbench::workbench::layout::LayoutSpec;
use rspice_app_workbench::workbench::state::workbench_session::{ConsolePage, WorkbenchSession};
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
        app.state.open_project_session();
        match owner {
            DiagnosticOwner::DesignRules => {
                // A finding about an object the drawing carries, so the row's
                // route is the real one the canvas resolves.
                let resistor = app.state.model.schematic.add_component(
                    rspice_app_state::state::ComponentType::Resistor,
                    Point::new(40, 40),
                );
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
                use rspice_app_state::state::{
                    Cell, CellViewRef, ComponentType, Library, PortDirection, SchematicState,
                    SymbolDocument, SymbolPin, View, ViewType,
                };
                use rspice_app_state::workbench::workspace::Workspace;

                // A symbol whose pin contract genuinely fails: its cell's
                // schematic declares a port that the symbol never places.
                let mut library = Library::new("work");
                let mut amp = Cell::new("amp");
                amp.add_view(View::new("schematic", ViewType::Schematic));
                amp.add_view(View::new("symbol", ViewType::Symbol));
                library.add_cell(amp);
                app.state.model.library_manager.add_library(library);

                let mut schematic = SchematicState::default();
                let port = schematic.add_component(ComponentType::Port, Point::new(0, 0));
                schematic
                    .document_mut_for_test()
                    .components
                    .iter_mut()
                    .find(|component| component.id == port)
                    .expect("the fixture just placed this port")
                    .value = "IN".to_owned();
                app.state.model.workspace.insert_schematic_editor(
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
                app.execute_command_key(DesignCommand::CheckCellview.key());
            }
            DiagnosticOwner::Dispatch => {
                // The real command, on an empty design: a schematic with no
                // components cannot be dispatched, so the pass blocks.
                app.state.model.schematic = rspice_app_state::state::SchematicState::default();
                app.execute_command_key(SimulateCommand::Preflight.key());
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
                app.state.model.workspace.content.design_rules =
                    serde_json::from_str(r#"{"schema_version":2}"#)
                        .expect("a stored deck always reads");
                app.state.publish_rule_deck_issues();
            }
            DiagnosticOwner::Lettering => {
                // A part's value in a code point no typeface has, once the
                // faces are known not to have it; the frame's refresh reads
                // the drawing.
                let unassigned = settled_missing('\u{38b}');
                let resistor = app.state.model.schematic.add_component(
                    rspice_app_state::state::ComponentType::Resistor,
                    Point::new(40, 40),
                );
                if let Some(part) = app
                    .state
                    .model
                    .schematic
                    .document_mut_for_test()
                    .components
                    .iter_mut()
                    .find(|part| part.id == resistor)
                {
                    part.value = format!("10 k{unassigned}");
                }
                let _ = app
                    .state
                    .refresh_live_checks(rspice_app_state::time_compat::Instant::now());
            }
            DiagnosticOwner::Import => {
                // A drawing placing a symbol no library has: it comes across
                // as a box with no pins, which the import says.
                let folder = std::env::temp_dir().join(format!(
                    "rspice-import-report-{}-{}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("system time after epoch")
                        .as_nanos()
                ));
                std::fs::create_dir_all(&folder).expect("drawing folder");
                let file = folder.join("unknown.asc");
                std::fs::write(
                    &file,
                    "Version 4\nSHEET 1 880 680\nSYMBOL nosuchpart 32 80 R0\nSYMATTR InstName U1\n",
                )
                .expect("schematic");
                crate::workbench::workflows::foreign_schematic_workflow::import_foreign_schematic_file(
                    &mut app,
                    &file,
                    rspice_design::foreign_schematic::ForeignTool::Ltspice,
                )
                .expect("the drawing imports");
                std::fs::remove_dir_all(&folder).ok();
            }
        }

        let batch = app
            .state
            .model
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
        // A deck that is not applied does not stop a run, a square in place
        // of a character does not either, and neither does a symbol pin that
        // needs attention, so each is a warning, not an error; it still has
        // to count.
        let counted = match owner {
            DiagnosticOwner::RuleDeck
            | DiagnosticOwner::Lettering
            | DiagnosticOwner::SymbolPins
            | DiagnosticOwner::Import => listed.counts().warnings,
            _ => listed.counts().errors,
        };
        assert!(
            counted > 0,
            "{owner:?} reported a failure that counts as nothing: {:?}",
            listed.counts()
        );
    }
}

/// `character`, once the faces are known not to have it. On the desktop a
/// worker with no faces to read is asked about it and answers on a frame; a
/// browser reads no installed faces, so there it is known at once.
fn settled_missing(character: char) -> char {
    use rspice_ui_kit::fonts::coverage::{Coverage, coverage};
    #[cfg(not(target_arch = "wasm32"))]
    {
        use rspice_ui_kit::fonts::system::{self, HanVariant, Source, SystemFonts};

        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        system::attach_with(
            &ctx,
            SystemFonts::spawn(Source::files(Vec::new()), HanVariant::Simplified.order()),
            Vec::new(),
        );
        rspice_ui_kit::fonts::coverage::ask([character]);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while coverage(character) == Coverage::Unsettled {
            assert!(
                std::time::Instant::now() < deadline,
                "the faces are asked about {character:?}"
            );
            let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    assert_eq!(coverage(character), Coverage::Missing);
    character
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
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel;

    let mut state: AppState =
        rspice_app_runs::simulation::controller::test_execution::run_manual_batch_session(
            "BSIM4 clamp\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L=1u W=10u\n\
         .model nch nmos level=54 version=4.8 pbs=0.05\n.op\n.end\n",
        );
    state.open_project_session();

    let batch = state
        .model
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
    use rspice_app_workbench::workbench::state::console_views::{
        ConsoleChannel, ConsoleReveal, ConsoleSelection,
    };

    rspice_app_state::diagnostics::engine_log::studio_logger_for_tests();
    let mut app = RSpiceApp::test_instance();
    app.state = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_session(
        "BSIM4 clamp\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L=1u W=10u\n\
         .model nch nmos level=54 version=4.8 pbs=0.05 pbsws=0.05\n.op\n.end\n",
    );
    app.state.open_project_session();
    let run_sequence = app.state.model.simulation.retained.runs[0].id;

    for parameter in ["pbs", "pbsws"] {
        let place = app
            .state
            .model
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
            .model
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

        app.state.model.diagnostics.request_route(place.route());
        crate::workbench::diagnostic_routes::apply_pending(&egui::Context::default(), &mut app);
        let view = &app.state.workbench.console_view;
        let key = place.line as u64;
        assert_eq!(view.channel, ConsoleChannel::Run(run_sequence));
        assert_eq!(view.selection, Some(ConsoleSelection::single(key)));
        assert_eq!(view.reveal, Some(ConsoleReveal::Line(key)));
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
    app.state.open_project_session();
    app.state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Netlist);
    app.state
        .model
        .log_buffer
        .warning(LogSource::Run, "unrelated retained log warning");
    app.state.code.netlist.diagnostics = std::sync::Arc::new(
        rspice_app_state::state::netlist_diagnostics::NetlistDiagnosticCollection::try_new(
            vec![
                rspice_app_state::state::netlist_diagnostics::Diagnostic::error("bad netlist card"),
            ],
            "",
        )
        .unwrap(),
    );

    assert_eq!(active_problem_count(&app.state, 0).total(), 1);

    // The same count wherever the reader is standing.
    app.state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
    assert_eq!(active_problem_count(&app.state, 1).total(), 1);
}

#[test]
fn clear_affordance_is_truthful() {
    let clear = console_clear_action(true);
    assert!(clear.enabled);
    assert_eq!(clear.label, "Clear the session log");
    let empty = console_clear_action(false);
    assert!(!empty.enabled);
    assert_eq!(empty.label, "The session log is already empty");
}

/// The tab strip holds the pages and what acts on the dock itself, the same
/// on every page. Clear is the Console page's own control, on its toolbar: it
/// used to sit in the strip on every page, disabled on the three that have
/// nothing a reader may clear.
#[test]
fn the_strip_holds_only_the_docks_own_actions_and_clear_is_the_consoles() {
    for page in ConsolePage::ALL {
        let mut app = busy_console();
        app.state.workbench.console_page = page;
        let nodes = labelled_bounds(&console_dock_output(&mut app, 900.0));
        let mut strip: Vec<&str> = nodes
            .iter()
            .filter(|(role, _, bounds)| {
                *role == egui::accesskit::Role::Button && bounds.bottom() <= CONSOLE_HEADER_HEIGHT
            })
            .map(|(_, label, _)| label.as_str())
            .collect();
        strip.sort_unstable();
        assert_eq!(
            strip,
            ["Collapse console", "Maximize console"],
            "{page:?}'s strip"
        );
        let clear = nodes
            .iter()
            .any(|(_, label, _)| label == "Clear the session log");
        assert_eq!(clear, page == ConsolePage::Console, "{page:?} offers Clear");
    }
}

#[test]
fn the_clear_control_clears_the_visible_page_through_its_command() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .log_buffer
        .warning(LogSource::Run, "visible console warning");

    app.state.workbench.console_page = ConsolePage::Problems;
    assert!(!ShellCommand::ClearConsole.is_enabled(&app));
    app.execute_command_key(ShellCommand::ClearConsole.key());
    assert!(!app.state.model.log_buffer.is_empty());

    app.state.workbench.console_page = ConsolePage::Console;
    assert!(ShellCommand::ClearConsole.is_enabled(&app));
    app.execute_command_key(ShellCommand::ClearConsole.key());
    assert!(app.state.model.log_buffer.is_empty());
    assert!(!ShellCommand::ClearConsole.is_enabled(&app));
}

/// A run's log is a record. Its channel offers no Clear, and the command
/// clears nothing from it, whatever the session's log holds.
#[test]
fn a_run_channel_has_no_clear_and_the_command_leaves_the_session_alone() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .log_buffer
        .warning(LogSource::Run, "visible console warning");
    app.state
        .model
        .simulation
        .retained
        .runs
        .push(SimulationRun::new(3));
    app.state.workbench.console_page = ConsolePage::Console;
    app.state
        .workbench
        .console_view
        .open(rspice_app_workbench::workbench::state::console_views::ConsoleChannel::Run(3));

    let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchSession::default());
    assert!(!clear_is_offered(
        desktop,
        app.state.workbench.console_view.channel
    ));
    assert!(!ShellCommand::ClearConsole.is_enabled(&app));
    app.execute_command_key(ShellCommand::ClearConsole.key());
    assert!(!app.state.model.log_buffer.is_empty());
}

#[test]
fn clearing_console_preserves_diagnostics_measurements_and_run_history() {
    let mut state = AppState::default();
    state
        .model
        .log_buffer
        .warning(LogSource::Run, "visible console warning");
    state.model.design_checks.drc_results = Some(DrcResult::new());

    let measurement = rspice_core::MeasureResult::success("gain", 42.0);
    let analysis =
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_measurements(vec![measurement]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.model.simulation.retained.runs.push(run);

    state.clear_primary_log();

    assert!(state.model.log_buffer.is_empty());
    assert!(state.model.design_checks.drc_results.is_some());
    assert_eq!(state.model.simulation.retained.runs.len(), 1);
    assert_eq!(
        state.model.simulation.retained.runs[0].analyses[0]
            .measurements
            .len(),
        1
    );
}

/// The plan's limit as Verify's sheet judged a value against it.
fn sheet_verdict(
    minimum: Option<f64>,
    maximum: Option<f64>,
    unit: &str,
    signed_margin: f64,
) -> ProjectVerdict {
    use rspice_results::specification::SpecificationComparison;
    let comparison = match (minimum, maximum) {
        (Some(minimum), Some(maximum)) => SpecificationComparison::Range { minimum, maximum },
        (Some(limit), None) => SpecificationComparison::Minimum { limit },
        (None, Some(limit)) => SpecificationComparison::Maximum { limit },
        (None, None) => SpecificationComparison::Tracked,
    };
    ProjectVerdict {
        comparison,
        unit: unit.to_owned(),
        holds: signed_margin >= 0.0,
        signed_margin: Some(signed_margin),
    }
}

/// The table states the sheet's verdict and margin, not one of its own: a
/// value inside the bare limit that the sheet failed (a guard band narrowed
/// it) reads as the sheet reads it, in the sheet's unit and spelling.
#[test]
fn measurement_table_states_the_sheets_verdict_and_margin() {
    let measurement = rspice_core::MeasureResult::success("gain", 9.5);
    let passing = sheet_verdict(Some(9.0), Some(10.0), "dB", 0.5);
    let row = measurement_table_row(&measurement, "AC", Some(&passing));
    assert_eq!(row.status, "PASS");
    assert!(!row.failed, "a pass is neutral");
    assert_eq!(row.value, "9.5 dB");
    assert_eq!(row.specification, "9 dB … 10 dB");
    assert_eq!(row.margin, "+0.5 dB");
    assert_eq!(row.at, "AC");

    let guarded = sheet_verdict(Some(9.0), Some(10.0), "dB", -0.25);
    let row = measurement_table_row(&measurement, "AC", Some(&guarded));
    assert_eq!(row.status, "FAIL");
    assert!(row.failed);
    assert_eq!(row.margin, "\u{2212}0.25 dB");
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
    assert_eq!(row.value, "20", "the published axis remains the value");
    assert_eq!(row.specification, "|raw| < 4");
    assert_eq!(row.margin, "+1", "the margin uses raw=3, not value=20");

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
        row.margin, "\u{2212}0",
        "inclusive FAILVALUE equality must retain a failing sign"
    );
}

#[test]
fn measurement_table_combines_every_contract_and_reports_the_limiting_margin() {
    let spec = sheet_verdict(Some(9.0), Some(11.0), "dB", 1.0);
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
        row.margin, "+1 dB",
        "the project upper bound limits clearance"
    );
    assert_eq!(
        row.specification,
        "PROJECT 9 dB … 11 dB · GOAL 10 dB ± 2 dB · FAILVALUE |raw| < 5"
    );

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
        row.margin, "\u{2212}1",
        "a passing project bound must not hide the authored failure margin"
    );

    let project_failure = rspice_core::MeasureResult {
        value: Some(12.0),
        raw_value: Some(1.0),
        expected: Some(12.0),
        ..passing
    };
    let spec = sheet_verdict(Some(9.0), Some(11.0), "dB", -1.0);
    let row = measurement_table_row(&project_failure, "AC", Some(&spec));
    assert_eq!(row.status, "FAIL");
    assert_eq!(row.margin, "\u{2212}1 dB");
}

/// Lifecycle words are states, not severities: only a failed run is marked.
#[test]
fn task_lifecycle_copy_distinguishes_cancelled_failed_and_interrupted() {
    assert_eq!(
        run_lifecycle_presentation(rspice_app_state::state::SimulationRunLifecycle::Aborted),
        ("CANCELLED", None)
    );
    assert_eq!(
        run_lifecycle_presentation(rspice_app_state::state::SimulationRunLifecycle::Failed),
        ("FAIL", Some(Problem::Failure))
    );
    assert_eq!(
        run_lifecycle_presentation(rspice_app_state::state::SimulationRunLifecycle::Interrupted),
        ("INTERRUPTED", None)
    );
    assert_eq!(
        run_lifecycle_presentation(rspice_app_state::state::SimulationRunLifecycle::Completed),
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

/// Where `label` is painted, as the pointer aims at it.
fn label_center(shapes: &[egui::epaint::ClippedShape], label: &str) -> Option<egui::Pos2> {
    shapes.iter().find_map(|clipped| match &clipped.shape {
        egui::epaint::Shape::Text(text) if text.galley.job.text == label => {
            Some(text.pos + text.galley.rect.center().to_vec2())
        }
        _ => None,
    })
}

/// The primary button going down or up at `at`.
fn press(at: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

/// A press on a console tab opens its page and leaves keyboard focus where it
/// was. Taking focus on a press drew the keyboard focus ring all the way
/// around the tab the pointer chose, over its underline. The keys still move
/// focus along the strip, and only they draw the ring.
#[test]
fn a_pressed_console_tab_opens_its_page_without_taking_keyboard_focus() {
    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let mut frame = |events: Vec<egui::Event>| {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 200.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let problems = active_problem_count(&app.state, ui.ctx().cumulative_pass_nr());
                    console_tabs(ui, &mut app, problems, CONSOLE_HEADER_HEIGHT, true);
                });
            },
        );
        (output.shapes, app.state.workbench.console_page)
    };

    let (shapes, page) = frame(Vec::new());
    assert_eq!(page, ConsolePage::Console);
    let at = label_center(&shapes, ConsolePage::Problems.label()).expect("the Problems tab");
    frame(vec![egui::Event::PointerMoved(at), press(at, true)]);
    frame(vec![press(at, false)]);
    let (_, page) = frame(Vec::new());
    assert_eq!(page, ConsolePage::Problems);
    assert_eq!(ctx.memory(|memory| memory.focused()), None);

    // Tab reaches the open page's tab, and the arrow keys carry focus on.
    frame(vec![key(egui::Key::Tab)]);
    frame(Vec::new());
    assert!(ctx.memory(|memory| memory.focused()).is_some());
    frame(vec![key(egui::Key::ArrowRight)]);
    let (_, page) = frame(Vec::new());
    assert_eq!(page, ConsolePage::Measurements);
    assert!(ctx.memory(|memory| memory.focused()).is_some());
}

/// A press on a tab opens its page when the frame between the press and the
/// release laid out a different number of widgets before the strip, as the
/// frame a run finishes does. The tabs took their ids from that count: a
/// press in that frame was released on a tab with another id, and the page
/// never opened.
#[test]
fn a_console_tab_press_survives_widgets_coming_and_going_before_the_strip() {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let mut frame = |before: usize, events: Vec<egui::Event>| {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 200.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // What the frame laid out before the strip.
                    ui.skip_ahead_auto_ids(before);
                    let problems = active_problem_count(&app.state, ui.ctx().cumulative_pass_nr());
                    console_tabs(ui, &mut app, problems, CONSOLE_HEADER_HEIGHT, true);
                });
            },
        );
        (output.shapes, app.state.workbench.console_page)
    };

    let (shapes, page) = frame(1, Vec::new());
    assert_eq!(page, ConsolePage::Console);
    let at = label_center(&shapes, ConsolePage::TaskLog.label()).expect("the Task log tab");
    frame(1, vec![egui::Event::PointerMoved(at), press(at, true)]);
    frame(0, vec![press(at, false)]);
    let (_, page) = frame(0, Vec::new());
    assert_eq!(
        page,
        ConsolePage::TaskLog,
        "the press on Task log opened its page"
    );
}

/// A collapsed dock shows its tabs and no page, so no tab is marked as the
/// open one, and a press on any tab opens the dock on that page. A press used
/// to change the page the strip marked and leave the dock shut.
#[test]
fn a_press_on_a_collapsed_docks_tab_opens_it_on_that_page() {
    let mut app = busy_console();
    app.state.workbench.console_visible = false;
    app.state.workbench.console_page = ConsolePage::Console;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    // Each frame answers with what it drew, the page the dock is on and
    // whether it shows that page.
    let mut frame = |events: Vec<egui::Event>| {
        let layout = LayoutSpec::resolve(1_280.0, 760.0, &app.state.workbench);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, CONSOLE_HEADER_HEIGHT),
                )),
                events,
                ..Default::default()
            },
            |ui| rspice_app_workbench::workbench::docks::console::show(ui, &mut app, layout),
        );
        let open = LayoutSpec::resolve(1_280.0, 760.0, &app.state.workbench).show_console_body;
        (output, app.state.workbench.console_page, open)
    };
    // Sizing settles on the second pass.
    let _ = frame(Vec::new());
    let (output, _, _) = frame(Vec::new());
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("accesskit is on");
    assert!(
        update
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Tab)
            .all(|(_, node)| !node.is_selected().unwrap_or(false)),
        "a collapsed dock marks a tab as open"
    );
    let problems = labelled_bounds(&output)
        .into_iter()
        .find(|(role, label, _)| {
            *role == egui::accesskit::Role::Tab && label.starts_with("Problems")
        })
        .map(|(_, _, bounds)| bounds.center())
        .expect("the Problems tab");
    let button = |pressed| egui::Event::PointerButton {
        pos: problems,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    frame(vec![egui::Event::PointerMoved(problems), button(true)]);
    let (_, page, open) = frame(vec![button(false)]);
    assert_eq!(page, ConsolePage::Problems);
    assert!(open, "the press left the dock collapsed");
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
    use rspice_app_workbench::workbench::state::console_views::ConsoleChannel;

    let mut state = AppState::default();
    state.model.log_buffer.clear();
    let rendered = painted_console_body(|ui| transcript::console(ui, &mut state));
    assert!(rendered.contains("Nothing logged yet."), "{rendered}");
    assert!(!rendered.contains("will appear"), "{rendered}");

    state
        .model
        .simulation
        .retained
        .runs
        .push(SimulationRun::new(4));
    state.workbench.console_view.open(ConsoleChannel::Run(4));
    let rendered = painted_console_body(|ui| transcript::console(ui, &mut state));
    assert!(rendered.contains("This run wrote no log."), "{rendered}");

    state.workbench.console_view.open(ConsoleChannel::Session);
    state
        .model
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
            |ui| rspice_app_workbench::workbench::docks::console::show(ui, app, layout),
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
            .model
            .simulation
            .retained
            .runs
            .insert(0, SimulationRun::new(sequence));
    }
    app.state.model.log_buffer.clear();
    for index in 0..240 {
        app.state.model.log_buffer.log(
            LogSeverity::Info,
            LogSource::Studio,
            format!("event {index}"),
        );
    }
    for _ in 0..12 {
        app.state
            .model
            .log_buffer
            .log(LogSeverity::Warning, LogSource::Netlist, "gmin stepping");
    }
    for _ in 0..3 {
        app.state
            .model
            .log_buffer
            .log(LogSeverity::Error, LogSource::Studio, "Export failed");
    }
    app
}

/// A ring that has dropped its oldest entries says so in its first row, so a
/// truncated session never reads as the whole of it.
#[test]
fn a_session_that_dropped_entries_says_so_first() {
    let mut state = AppState::default();
    state.model.log_buffer = rspice_app_state::diagnostics::LogBuffer::new(3);
    for index in 0..5 {
        state.model.log_buffer.log(
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
    let mut spec = sheet_verdict(Some(200.0), Some(300.0), "mV", 50.0);
    let row = measurement_table_row(&measured, "TRAN", Some(&spec));
    assert_eq!(row.status, "PASS");
    assert_eq!(row.value, "250 mV", "the value in the sheet's unit");
    assert_eq!(
        row.specification,
        "PROJECT 200 mV … 300 mV · GOAL 250 mV ± 10 mV · FAILVALUE |raw| < 500 mV"
    );
    // The project's room is 50 in the sheet's millivolts; the goal's is 0.01
    // in the measurement's own volts, which reads 10 mV, never 0.01 mV.
    assert_eq!(row.margin, "PROJECT +50 mV; GOAL +10 mV; FAILVALUE +250 mV");
    spec.unit = "mA".into();
    let row = measurement_table_row(&measured, "TRAN", Some(&spec));
    assert_eq!(row.status, "ERROR");
    assert!(row.margin.contains("incompatible"));
}
