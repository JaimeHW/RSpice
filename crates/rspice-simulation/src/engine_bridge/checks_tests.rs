//! A run's checks through the headless route are the core run's, field for
//! field: the `.ASSERT` reports of an operating point, a DC sweep and a
//! transient, and an HDL `$error` with its file, line and time.

use super::*;
use rspice_core::NoAbort;
use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::config::{HdlOutputCollector, HdlStream};
use rspice_simulation_contract::config::{DcSweepConfig, OpConfig, TransientAnalysisConfig};

fn parse(deck: &str) -> rspice_core::Netlist {
    rspice_core::Netlist::parse(deck).expect("the deck parses")
}

fn core_engine(netlist: &rspice_core::Netlist) -> rspice_core::Engine {
    rspice_core::Engine::default().resolved_for_netlist(netlist)
}

fn checks(result: &SimulationResult) -> &crate::results::SimulationChecks {
    result.checks().expect("the result carries its checks")
}

/// The checks as the app keeps them: the conversion moves them whole.
fn retained(result: SimulationResult, analysis: AnalysisType) -> RunChecks {
    crate::result_conversion::convert(result, analysis, "checks", || 0.0).checks
}

use rspice_results::analysis_result::RunChecks;
use rspice_results::analysis_type::AnalysisType;

/// RC, τ = 1 µs, a 1 V step with a 1 ps edge: `V(out)` crosses 0.5 V at
/// τ·ln 2 and stays above it to the stop time.
const RC: &str = "* rc\nV1 in 0 PWL(0 0 1p 1)\nR1 in out 1k\nC1 out 0 1n\n\
                  .options reltol=1e-6\n.tran 10n 2u\n\
                  .ASSERT a EXPR='V(out)' MAX=0.5 LEVEL=ERROR MESSAGE='settled too high'\n.end\n";

#[test]
fn a_transient_assertion_report_is_the_core_runs() {
    let netlist = parse(RC);
    let result = EngineBridge::new()
        .run(
            &AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: 2e-6,
                step_time: 10e-9,
                ..TransientAnalysisConfig::default()
            }),
            RC,
        )
        .expect("the transient runs");
    let max_step = rspice_core::execution::resolve_dialect_transient_maximum_step(
        rspice_core::SpiceDialect::BestAvailable,
        10e-9,
        2e-6,
        None,
        None,
    )
    .unwrap();
    let core = core_engine(&netlist)
        .run_tran_with_abort(&netlist, 2e-6, max_step, &NoAbort)
        .expect("the core transient runs");
    let expected = rspice_core::analysis::evaluate_tran_assertions(&netlist, &core, &NoAbort)
        .expect("the assertions evaluate");
    assert_eq!(checks(&result).assertions, expected);

    let [report] = expected.as_slice() else {
        panic!("one report: {expected:?}");
    };
    let [violation] = report.violations.as_slice() else {
        panic!("one violation: {report:?}");
    };
    let crossing = 1e-6 * std::f64::consts::LN_2;
    let start = violation.start.expect("a transient violation starts");
    assert!((start - crossing).abs() < 0.05e-9, "{start:e}");
    assert_eq!(violation.end, None, "open at the stop time");
    assert_eq!(report.message.as_deref(), Some("settled too high"));
    assert_eq!(report.origin.line, 7);

    assert_eq!(
        retained(result, AnalysisType::Transient).assertions,
        expected
    );
}

/// A level-1 NMOS at Vds = 1 V: the model form expands to both instances, and
/// each reports the operating point's vds as its extreme.
const NMOS: &str = "* nmos\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nm W=1u L=1u\nM2 d g 0 0 nm W=2u L=1u\n\
                    .model nm nmos level=1 vto=0.5 kp=1e-4\n.op\n\
                    .ASSERT sat MOD=nm PARAM=vds MAX=0.5\n.end\n";

#[test]
fn an_operating_point_assertion_report_is_the_core_runs() {
    let netlist = parse(NMOS);
    let result = EngineBridge::new()
        .run(&AnalysisConfig::DcOp(OpConfig::default()), NMOS)
        .expect("the operating point solves");
    let (op, devices) = core_engine(&netlist)
        .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
        .expect("the core operating point solves");
    let expected = rspice_core::analysis::evaluate_op_assertions(&netlist, &op, &devices);
    assert_eq!(checks(&result).assertions, expected);
    assert_eq!(expected.len(), 2, "{expected:?}");
    for report in &expected {
        assert_eq!(
            report.violations,
            [AssertViolation {
                start: None,
                end: None,
                extreme: Some(1.0),
                at: None,
            }],
            "{report:?}"
        );
    }
    assert_eq!(retained(result, AnalysisType::DcOp).assertions, expected);
}

const SWEEP: &str = "* dc\nV1 in 0 0\nR1 in 0 1k\n.dc V1 0 1 0.25\n\
                     .ASSERT lim EXPR='V(in)' MAX=0.3\n.end\n";

#[test]
fn a_dc_sweep_assertion_report_is_the_core_runs() {
    let netlist = parse(SWEEP);
    let result = EngineBridge::new()
        .run(
            &AnalysisConfig::DcSweep(DcSweepConfig {
                source: "V1".to_owned(),
                start: 0.0,
                stop: 1.0,
                step: 0.25,
                ..Default::default()
            }),
            SWEEP,
        )
        .expect("the sweep runs");
    let sweep = core_engine(&netlist)
        .run_dc_sweep(&netlist, "v1", 0.0, 1.0, 0.25)
        .expect("the core sweep runs");
    let expected = rspice_core::analysis::evaluate_dc_assertions(&netlist, &sweep);
    assert_eq!(checks(&result).assertions, expected);
    let [report]: &[AssertionReport] = expected.as_slice() else {
        panic!("one report: {expected:?}");
    };
    assert_eq!(
        report.violations,
        [AssertViolation {
            start: Some(0.5),
            end: Some(1.0),
            extreme: Some(1.0),
            at: Some(1.0),
        }]
    );
    assert_eq!(retained(result, AnalysisType::DcSweep).assertions, expected);
}

/// A design that reports an `$error` on its second rising clock edge, at
/// 15 ns: the headless result keeps the record with its file, line and time,
/// exactly as the core run delivered it.
#[test]
fn an_hdl_error_keeps_its_file_line_and_time() {
    let directory = std::env::temp_dir().join(format!(
        "rspice_simulation_hdl_checks_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("create the model directory");
    let model = directory.join("checker.va");
    std::fs::write(
        &model,
        "`timescale 1ns/1ns\n\
         module checker(p, n);\n\
         \x20 inout p, n;\n\
         \x20 electrical p, n;\n\
         \x20 reg clk;\n\
         \x20 integer count;\n\
         \x20 initial begin clk = 0; count = 0; end\n\
         \x20 always #5 clk = ~clk;\n\
         \x20 always @(posedge clk) begin\n\
         \x20   count = count + 1;\n\
         \x20   if (count == 2) $error(\"edge %0d is late\", count);\n\
         \x20 end\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n",
    )
    .expect("write the model");
    let deck = format!(
        "* hdl checks\nV1 a 0 1\nR1 a p 1k\nx1 p 0 checker\n.va \"{}\" checker\n.tran 1n 32n\n.end\n",
        model.display().to_string().replace('\\', "/")
    );
    let netlist = parse(&deck);
    let result = EngineBridge::new().run(
        &AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 32e-9,
            step_time: 1e-9,
            ..TransientAnalysisConfig::default()
        }),
        &deck,
    );
    let collector = std::sync::Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    let max_step = rspice_core::execution::resolve_dialect_transient_maximum_step(
        rspice_core::SpiceDialect::BestAvailable,
        1e-9,
        32e-9,
        None,
        None,
    )
    .unwrap();
    let core = rspice_core::Engine::new(config)
        .resolved_for_netlist(&netlist)
        .run_tran_with_abort(&netlist, 32e-9, max_step, &NoAbort);
    let _ = std::fs::remove_dir_all(&directory);
    let result = result.expect("the transient runs");
    core.expect("the core transient runs");
    let expected = collector
        .take()
        .into_iter()
        .filter(|record| matches!(record.stream, HdlStream::Diagnostic { .. }))
        .collect::<Vec<_>>();
    assert_eq!(checks(&result).hdl, expected);

    let [record] = expected.as_slice() else {
        panic!("one severity record: {expected:?}");
    };
    assert_eq!(record.text.trim_end(), "edge 2 is late");
    assert!(record.origin.file.ends_with("checker.va"), "{record:?}");
    assert_eq!(record.origin.line, 11);
    assert!((record.seconds - 15e-9).abs() < 1e-24, "{record:?}");

    let retained = retained(result, AnalysisType::Transient);
    let [diagnostic] = retained.hdl.as_slice() else {
        panic!("one retained diagnostic: {retained:?}");
    };
    assert_eq!(diagnostic.origin, record.origin);
    assert_eq!(diagnostic.point, record.point);
    assert_eq!(diagnostic.text, record.text);
}

/// A design's `$finish`, `$stop` and `$fatal` each end the run under their own
/// name: the checks carry the task, its file and line, the time and the
/// `$fatal` message, and only the `$fatal` fails the run.
#[test]
fn the_task_that_ended_a_run_is_named_in_its_checks() {
    use rspice_core::{ModelFinishPoint, ModelFinishTask};
    use rspice_results::analysis_result::CheckVerdict;
    let directory = std::env::temp_dir().join(format!(
        "rspice_simulation_hdl_finish_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("create the model directory");
    for (call, task, message, verdict) in [
        (
            "$finish;",
            ModelFinishTask::Finish,
            None,
            CheckVerdict::Passed,
        ),
        ("$stop;", ModelFinishTask::Stop, None, CheckVerdict::Passed),
        (
            "$fatal(1, \"code %0d\", 7);",
            ModelFinishTask::Fatal,
            Some("code 7"),
            CheckVerdict::Failed,
        ),
    ] {
        let model = directory.join(format!("ender_{}.va", task.name().trim_start_matches('$')));
        std::fs::write(
            &model,
            format!(
                "`timescale 1ns/1ns\n\
                 module ender(p, n);\n\
                 \x20 inout p, n;\n\
                 \x20 electrical p, n;\n\
                 \x20 initial begin\n\
                 \x20   #5 {call}\n\
                 \x20 end\n\
                 \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
                 endmodule\n"
            ),
        )
        .expect("write the model");
        let deck = format!(
            "* hdl finish\nV1 a 0 1\nR1 a p 1k\nx1 p 0 ender\n.va \"{}\" ender\n.tran 1n 20n\n.end\n",
            model.display().to_string().replace('\\', "/")
        );
        let result = EngineBridge::new()
            .run(
                &AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: 20e-9,
                    step_time: 1e-9,
                    ..TransientAnalysisConfig::default()
                }),
                &deck,
            )
            .unwrap_or_else(|error| panic!("{call}: the transient runs: {error}"));
        let finish = checks(&result)
            .finish
            .clone()
            .unwrap_or_else(|| panic!("{call}: the checks name the task that ended the run"));
        assert_eq!(finish.task, task, "{call}");
        assert_eq!(finish.instance, "x1", "{call}");
        assert_eq!(finish.model, "ender", "{call}");
        let source = finish.source.clone().expect("the call's line");
        assert!(source.file.ends_with(".va"), "{call}: {source:?}");
        assert_eq!(source.line, 6, "{call}");
        let ModelFinishPoint::Transient { time } = finish.point else {
            panic!("{call}: a transient finish: {finish:?}");
        };
        assert!((time - 5e-9).abs() < 1e-18, "{call}: {time:e}");
        assert_eq!(finish.message.as_deref(), message, "{call}");
        assert!(finish.to_string().contains(task.name()), "{call}: {finish}");
        let retained = retained(result, AnalysisType::Transient);
        assert_eq!(retained.finish, Some(finish), "{call}");
        assert_eq!(retained.verdict(), verdict, "{call}");
    }
    let _ = std::fs::remove_dir_all(&directory);
}

/// A run that reaches its stop time names no finish, and a clean run passes.
#[test]
fn a_run_that_reaches_its_end_names_no_finish() {
    let result = EngineBridge::new()
        .run(
            &AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: 2e-6,
                step_time: 10e-9,
                ..TransientAnalysisConfig::default()
            }),
            "* rc\nV1 in 0 PWL(0 0 1p 1)\nR1 in out 1k\nC1 out 0 1n\n.tran 10n 2u\n.end\n",
        )
        .expect("the transient runs");
    assert_eq!(checks(&result).finish, None);
    assert_eq!(
        retained(result, AnalysisType::Transient).verdict(),
        rspice_results::analysis_result::CheckVerdict::Passed
    );
}

/// The source map of a run's traced HDL signals reaches the headless result
/// beside the histories it names, entry for entry the core run's.
#[test]
fn a_transient_carries_the_source_map_of_its_hdl_traces() {
    let directory = std::env::temp_dir().join(format!(
        "rspice_simulation_hdl_sources_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("create the model directory");
    let path = directory.join("gen.v");
    std::fs::write(
        &path,
        "`timescale 1ns/1ns\n\
         module gen(q);\n  output q;\n  reg q;\n\
         \x20 initial begin q = 0; #3 q = 1; #4 q = 0; end\nendmodule\n",
    )
    .expect("write the module");
    let deck = format!(
        "* hdl sources\n.hdl \"{}\"\nx1 q gen\nrq q 0 1k\n.tran 1n 20n\n.end\n",
        path.display().to_string().replace('\\', "/")
    );
    let netlist = parse(&deck);
    let result = EngineBridge::new().run(
        &AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 20e-9,
            step_time: 1e-9,
            ..TransientAnalysisConfig::default()
        }),
        &deck,
    );
    let core = core_engine(&netlist).run_tran(&netlist, 20e-9, 1e-9);
    let _ = std::fs::remove_dir_all(&directory);
    let SimulationResult::Transient { events, .. } = result.expect("the transient runs") else {
        panic!("a transient result");
    };
    let core = core.expect("the core transient runs");
    let expected = core
        .hdl_sources
        .iter()
        .map(rspice_results::events::HdlSourceEvidence::from)
        .collect::<Vec<_>>();
    assert!(
        expected.iter().any(|source| source.trace == "Q"
            && source
                .declaration
                .as_ref()
                .is_some_and(|line| line.line > 0)),
        "{expected:?}"
    );
    assert_eq!(events.hdl_sources, expected);
}

/// A testbench that ends on `$fatal` keeps the task that ended it and the
/// source map of its traces through the browser worker's contract, as JSON,
/// and both reach the retained result: the finish in its checks, the map in
/// its event payload, entry for entry what the native run retains.
#[test]
fn a_testbench_keeps_its_finish_and_source_map_through_the_worker_into_its_result() {
    use crate::runner::worker_contract::WorkerSimulationResult;
    use rspice_core::ModelFinishTask;
    use rspice_results::analysis_payload::AnalysisResultPayload;
    use rspice_results::analysis_result::CheckVerdict;

    let directory = std::env::temp_dir().join(format!(
        "rspice_simulation_worker_fatal_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("create the module directory");
    let path = directory.join("gen.v");
    std::fs::write(
        &path,
        "`timescale 1ns/1ns\n\
         module gen(q);\n  output q;\n  reg q;\n\
         \x20 initial begin q = 0; #3 q = 1; #4 $fatal(1, \"stuck at %0d\", q); end\nendmodule\n",
    )
    .expect("write the module");
    let deck = format!(
        "* hdl fatal through the worker\n.hdl \"{}\"\nx1 q gen\nrq q 0 1k\n.tran 1n 20n\n.end\n",
        path.display().to_string().replace('\\', "/")
    );
    let native = EngineBridge::new()
        .run(
            &AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: 20e-9,
                step_time: 1e-9,
                ..TransientAnalysisConfig::default()
            }),
            &deck,
        )
        .expect("the transient runs");
    let _ = std::fs::remove_dir_all(&directory);

    let wire = serde_json::to_string(
        &WorkerSimulationResult::try_from(native.clone()).expect("the worker carries a transient"),
    )
    .expect("the worker result serializes");
    let worker = SimulationResult::from(
        serde_json::from_str::<WorkerSimulationResult>(&wire).expect("the worker result reads"),
    );

    let finish = checks(&native)
        .finish
        .clone()
        .expect("the $fatal ended the run");
    assert_eq!(finish.task, ModelFinishTask::Fatal);
    assert_eq!(finish.message.as_deref(), Some("stuck at 1"));
    assert_eq!(checks(&worker).finish.as_ref(), Some(&finish));
    let (
        SimulationResult::Transient {
            events: native_events,
            ..
        },
        SimulationResult::Transient {
            events: worker_events,
            ..
        },
    ) = (&native, &worker)
    else {
        panic!("transient results");
    };
    assert!(
        native_events
            .hdl_sources
            .iter()
            .any(|source| source.trace == "Q" && source.declaration.is_some()),
        "{:?}",
        native_events.hdl_sources
    );
    assert_eq!(worker_events.hdl_sources, native_events.hdl_sources);

    let retained = |result: SimulationResult| {
        crate::result_conversion::convert(result, AnalysisType::Transient, "Transient", || 0.0)
    };
    let source_map =
        |result: &rspice_results::analysis_result::AnalysisResult| match &result.result_payload {
            Some(AnalysisResultPayload::TransientEvents { hdl_sources, .. }) => hdl_sources.clone(),
            other => panic!("an event payload: {other:?}"),
        };
    let native = retained(native);
    let worker = retained(worker);
    assert_eq!(native.checks.finish.as_ref(), Some(&finish));
    assert_eq!(worker.checks, native.checks);
    assert_eq!(worker.checks.verdict(), CheckVerdict::Failed);
    assert!(!source_map(&native).is_empty(), "the map is retained");
    assert_eq!(source_map(&worker), source_map(&native));
}
