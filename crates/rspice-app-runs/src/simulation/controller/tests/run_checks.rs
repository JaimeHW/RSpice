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
