//! A run's checks reach its retained results and the project file.

use super::*;
use crate::simulation::controller::test_execution;

/// RC, τ = 1 µs: `V(out)` crosses 0.5 V at τ·ln 2 and stays above it.
const RC: &str = "* rc checks\nV1 in 0 PWL(0 0 1p 1)\nR1 in out 1k\nC1 out 0 1n\n\
                  .options reltol=1e-6\n.tran 10n 2u\n\
                  .ASSERT a EXPR='V(out)' MAX=0.5 LEVEL=ERROR\n.end\n";

/// The assertion report a run through the simulator publishes is kept with
/// its transient result, and saving and reopening the project restores it
/// exactly.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_runs_assertion_report_is_retained_saved_and_restored() {
    let export_io = MockExportWorkflowIo::default();
    let mut state = RunTestHost::default();
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some(RC.to_owned());
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the fixture deck prepares");
    state.model.simulation.execution.trigger_simulation = true;
    controller.update(&mut state, &export_io);
    let run_sequence = controller.current_run_id.expect("the batch has a run");
    while controller.has_active_batch() {
        test_execution::wait_until_finished_unpolled(&controller.runner);
        controller.poll_completion(&mut state, &export_io);
    }

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Transient)
        .expect("the transient is retained");
    let checks = analysis.checks.clone();
    let [report] = checks.assertions.as_slice() else {
        panic!("one assertion report: {checks:?}");
    };
    let [violation] = report.violations.as_slice() else {
        panic!("one violation: {report:?}");
    };
    let start = violation.start.expect("a transient violation starts");
    assert!(
        (start - 1e-6 * std::f64::consts::LN_2).abs() < 0.05e-9,
        "{start:e}"
    );
    assert_eq!(violation.end, None, "open at the stop time");

    let persisted = crate::io::capture_simulation_results(&state.model.simulation);
    let json = serde_json::to_string(&persisted).expect("the results serialize");
    let restored: rspice_formats::project_results::ProjectSimulationResults =
        serde_json::from_str(&json).expect("the results deserialize");
    restored.validate().expect("the results validate");
    let restored = crate::io::simulation_state_from_results(restored).expect("the results restore");
    let reopened = restored
        .retained
        .runs
        .iter()
        .flat_map(|run| run.analyses.iter())
        .find(|analysis| analysis.analysis_type == AnalysisType::Transient)
        .expect("the transient is restored");
    assert_eq!(reopened.checks, checks);
}

/// An HDL call is routed to its line where a project Verilog-A bundle holds
/// its file, as its Problems row and its Console line both open it, and only
/// there: a file no bundle holds, or one two cell views each hold, is stated
/// without a jump rather than opened on a guess.
#[test]
fn an_hdl_call_routes_to_the_one_project_bundle_that_holds_its_file() {
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticRoute;
    let mut state = RunTestHost::default();
    let bundle = |library: &str| {
        crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
                library, "tb", "veriloga",
            )),
            crate::state::ProjectSourceLanguage::VerilogA,
            "tb/tb.va",
            "module tb; endmodule\n",
            Vec::<crate::state::ProjectSourceFile>::new(),
            Vec::<crate::state::ProjectSourceDependency>::new(),
        )
        .expect("a valid Verilog-A bundle")
    };
    let first = bundle("lib_a");
    let id = first.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(first)
        .expect("attach the cell view's source");
    let run = state.model.simulation.start_run().id;
    assert_eq!(
        state.model.hdl_call_route(run, "tb/tb.va", 9),
        Some(DiagnosticRoute::CodeDocument {
            bundle: id,
            logical_path: "tb/tb.va".into(),
            line: Some(9),
            column: None,
        })
    );
    assert_eq!(state.model.hdl_call_route(run, "elsewhere.va", 9), None);
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle("lib_b"))
        .expect("attach a second cell view's source");
    assert_eq!(
        state.model.hdl_call_route(run, "tb/tb.va", 9),
        None,
        "two cell views hold the file"
    );
}

/// Corpus design C12, variant fail, run as a manual deck through the prepared
/// route the app takes: the testbench flags codes 1 to 15 with `$error` at
/// line 40, 99 us into each 100 us slot, and ends on `$finish`. The Problems
/// page lists exactly those fifteen records, each with the scope, line, time
/// and text the CLI prints for the same deck; the run completes with its
/// results kept and its checks fail. The deck's `.hdl` file is a file on
/// disk no editor of the app holds, so no row offers a jump.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn corpus_c12_fail_files_one_row_per_testbench_error_as_the_cli_prints_them() {
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel;
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c12-ldo-tb/deck/fail.tran.cir");
    let deck = std::fs::read_to_string(&path).expect("read the corpus deck");
    let export_io = MockExportWorkflowIo::default();
    let mut state = RunTestHost::default();
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some(deck);
    state.model.workspace.content.netlist_source_path = Some(path);
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the corpus deck prepares");
    state.model.simulation.execution.trigger_simulation = true;
    controller.update(&mut state, &export_io);
    let run_sequence = controller.current_run_id.expect("the batch has a run");
    while controller.has_active_batch() {
        test_execution::wait_until_finished_unpolled(&controller.runner);
        controller.poll_completion(&mut state, &export_io);
    }

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    assert_eq!(
        run.lifecycle,
        crate::state::SimulationRunLifecycle::Completed
    );
    assert!(run.analyses.iter().all(|analysis| analysis.success));
    assert_eq!(
        run.check_verdict(),
        Some(rspice_results::analysis_result::CheckVerdict::Failed)
    );
    let rows = state.model.diagnostics.published_run_records(run_sequence);
    let checks: Vec<_> = rows
        .iter()
        .filter(|record| record.canonical.code.starts_with("HDL-"))
        .collect();
    assert_eq!(checks.len(), 15, "one row per $error: {rows:#?}");
    for (code, record) in (1..=15).zip(&checks) {
        assert_eq!(record.canonical.code.as_ref(), "HDL-ERROR");
        assert_eq!(record.level, DiagnosticLevel::Error);
        assert_eq!(record.subject.object, "xtb");
        let line = record.subject.source_line.as_deref().unwrap_or_default();
        assert!(line.ends_with("tb_ldo_fail.vams:40"), "{line}");
        let time = format!(
            "{}s",
            crate::quantity::engineering::format_engineering_display_with(
                f64::from(100 * code + 99) * 1e-6,
                crate::quantity::engineering::EngineeringPrecision::Adaptive,
            )
        );
        assert!(
            record
                .message
                .starts_with(&format!("$error at t = {time}: code {code}: step ")),
            "{}",
            record.message
        );
        assert!(
            record.message.ends_with(" V is outside [5 mV, 7 mV]"),
            "{}",
            record.message
        );
        assert_eq!(
            record.route, None,
            "a manual deck's .hdl file has no editor"
        );
    }
}

/// A vector file whose expected outputs run past the transient's stop time:
/// the evaluator counts the two late samples in one warning, and the run
/// files it as one Warning row in the evaluator's words, at the first late
/// row of the file, and warns rather than passes.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn samples_after_the_stop_time_are_one_warning_row_and_warn_the_run() {
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel;
    let folder = std::env::temp_dir().join(format!("rspice-after-stop-{}", std::process::id()));
    std::fs::create_dir_all(&folder).expect("make the scratch folder");
    std::fs::write(
        folder.join("late.vec"),
        "radix 1 1\nvname a y\nio i o\ntunit ns\nvih 1.8\nvil 0\ntrise 1\n\
         0 0 0\n10 1 X\n15 1 1\n25 1 1\n30 1 0\n",
    )
    .expect("write the vector file");
    let path = folder.join("late.cir");
    let export_io = MockExportWorkflowIo::default();
    let mut state = RunTestHost::default();
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some(
        "* vec late\nr1 a y 1\nr2 y 0 1meg\n.vec 'late.vec'\n.tran 0.1n 20n\n.end\n".to_owned(),
    );
    state.model.workspace.content.netlist_source_path = Some(path);
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the deck prepares");
    state.model.simulation.execution.trigger_simulation = true;
    controller.update(&mut state, &export_io);
    let run_sequence = controller.current_run_id.expect("the batch has a run");
    while controller.has_active_batch() {
        test_execution::wait_until_finished_unpolled(&controller.runner);
        controller.poll_completion(&mut state, &export_io);
    }

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    assert_eq!(
        run.lifecycle,
        crate::state::SimulationRunLifecycle::Completed
    );
    assert_eq!(
        run.check_verdict(),
        Some(rspice_results::analysis_result::CheckVerdict::Warned)
    );
    let rows: Vec<_> = state
        .model
        .diagnostics
        .published_run_records(run_sequence)
        .into_iter()
        .filter(|record| record.canonical.code.starts_with("ASSERT-"))
        .collect();
    assert_eq!(rows.len(), 1, "{rows:#?}");
    let row = rows[0];
    assert_eq!(row.canonical.code.as_ref(), "ASSERT-AFTER-STOP");
    assert_eq!(row.level, DiagnosticLevel::Warning);
    assert!(
        row.message
            .starts_with("2 expected-output samples after the transient's stop time 2e-8 s"),
        "{}",
        row.message
    );
    let line = row.subject.source_line.as_deref().unwrap_or_default();
    assert!(line.ends_with("late.vec:11"), "{line}");
    assert_eq!(row.route, None, "the vector file is not the deck");
}
