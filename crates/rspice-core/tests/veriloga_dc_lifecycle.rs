//! End-to-end DC lifecycle pins for runtime-compiled Verilog-A devices.
#![cfg(feature = "veriloga")]

use rspice_core::engine::DcSweepRange;
use rspice_core::{Engine, Netlist, NoAbort};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn write_model(name: &str, source: &str) -> PathBuf {
    let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rspice_dc_lifecycle_{name}_{}_{sequence}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn node_voltage(result: &rspice_core::solver::SimulationResult, name: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} is absent from {:?}", result.node_names));
    result.node_voltages[index]
}

const REBUILT_LIFECYCLE_MODEL: &str = r#"
`include "disciplines.vams"
module va_dc_rebuild_lifecycle(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0;
    real count;
    analog begin
        @(initial_step("dc")) count = count + 1.0;
        @(final_step("dc")) count = count + 10.0;
        V(p, n) <+ count + 0.0 * gain;
    end
endmodule
"#;

#[test]
fn analog_initial_is_not_replayed_by_dc_newton_or_final_step_evaluations() {
    let model = write_model(
        "analog_initial",
        r#"module va_initial_once(p,n);
inout p,n; electrical p,n;
integer launches;
analog initial launches=launches+1;
analog V(p,n)<+launches;
endmodule"#,
    );
    let deck = format!(
        "* pre-simulation lifecycle\nX1 out 0 va_initial_once\n.va \"{}\" va_initial_once\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    for _ in 0..2 {
        let result = engine
            .run_dc_op(&netlist)
            .expect("initialized DC source converges");
        assert_eq!(node_voltage(&result, "out"), 1.0);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn initialization_finish_returns_no_solution_and_does_not_cancel_or_poison_the_engine() {
    use rspice_core::{AbortSignal, AtomicAbort, ModelFinishPoint, SimulationOutcome};
    let model = write_model(
        "finish_initialization",
        r#"
module finish_initialization(p,n);
inout p,n; electrical p,n;
analog initial $finish(0);
analog I(p,n)<+V(p,n);
endmodule"#,
    );
    let deck = format!(
        "* initialization finish\nV1 out 0 2\nX1 out 0 finish_initialization\n.va \"{}\" finish_initialization\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    let abort = AtomicAbort::new();
    for _ in 0..2 {
        let outcome = engine
            .run_with_outcome(&abort, |engine, signal| {
                engine.run_dc_op_with_abort(&netlist, signal)
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: None,
            finish,
        } = outcome
        else {
            panic!("initialization must not manufacture an operating point: {outcome:?}");
        };
        assert_eq!(finish.model, "finish_initialization");
        assert_eq!(finish.instance, "X1");
        assert_eq!(finish.point, ModelFinishPoint::Initialization);
        assert_eq!(finish.diagnostic_level, 0);
        assert!(!abort.is_aborted());
    }
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine
                .run_tran_with_abort(&netlist, 1e-6, 1e-7, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine.run_ac_with_abort(
                &netlist,
                &[1e3, 1e4],
                signal
            ))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine.run_noise_with_abort(
                &netlist,
                1,
                &[1e3, 1e4],
                300.15,
                signal
            ))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine
                .run_dc_sweep_with_report_and_abort(&netlist, "V1", 0.0, 2.0, 1.0, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(!abort.is_aborted());
    let plain = Netlist::parse("* independent run\nV1 out 0 3\nR1 out 0 1k\n.end\n").unwrap();
    let SimulationOutcome::Completed(result) = engine
        .run_with_outcome(&abort, |engine, signal| {
            engine.run_dc_op_with_abort(&plain, signal)
        })
        .unwrap()
    else {
        panic!("a later run inherited another run's finish state");
    };
    assert_eq!(node_voltage(&result, "out"), 3.0);
    let _ = std::fs::remove_file(model);
}

#[test]
fn accepted_operating_point_finish_keeps_its_solution_and_final_step() {
    use rspice_core::{ModelFinishPoint, SimulationOutcome};
    let model = write_model(
        "finish_dc",
        r#"
module finish_dc(p,n);
inout p,n; electrical p,n;
integer final_count;
analog begin
  @(final_step("dc")) begin final_count=final_count+1; $finish(1); end
  V(p,n)<+final_count;
end
endmodule"#,
    );
    let deck = format!(
        "* accepted finish\nX1 out 0 finish_dc\n.va \"{}\" finish_dc\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let outcome = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_dc_op_with_abort(&netlist, signal)
        })
        .unwrap();
    let SimulationOutcome::Finished {
        result: Some(result),
        finish,
    } = outcome
    else {
        panic!("an accepted finish must retain its operating point: {outcome:?}");
    };
    assert_eq!(finish.point, ModelFinishPoint::OperatingPoint);
    assert_eq!(finish.diagnostic_level, 1);
    assert_eq!(node_voltage(&result, "out"), 1.0);
    let _ = std::fs::remove_file(model);
}

#[test]
fn a_portless_model_can_finish_before_empty_analysis_shortcuts() {
    use rspice_core::SimulationOutcome;
    let model = write_model(
        "finish_empty",
        "module finish_empty; analog initial $finish(0); endmodule",
    );
    let deck = format!(
        "* empty analog finish\nX1 finish_empty\n.va \"{}\" finish_empty\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    assert!(matches!(
        engine
            .run_with_outcome(&NoAbort, |engine, signal| engine
                .run_dc_op_with_abort(&netlist, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&NoAbort, |engine, signal| engine
                .run_tran_with_abort(&netlist, 1e-6, 1e-7, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&NoAbort, |engine, signal| engine.run_ac_with_abort(
                &netlist,
                &[1e3],
                signal
            ))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    let _ = std::fs::remove_file(model);
}

fn assert_rebuilt_lifecycle_values(
    points: &[(f64, rspice_core::solver::SimulationResult)],
    expected: &[f64],
) {
    assert_eq!(points.len(), expected.len());
    for (index, ((coordinate, result), expected)) in points.iter().zip(expected).enumerate() {
        let actual = node_voltage(result, "out");
        assert!(
            (actual - expected).abs() < 1.0e-12,
            "unexpected rebuilt lifecycle value at point {index} ({coordinate}): actual={actual}, expected={expected}"
        );
    }
}

#[test]
fn forced_initial_conditions_expose_ic_identity_and_boundaries() {
    let model = write_model(
        "forced_ic_identity",
        r#"
`include "disciplines.vams"
module va_forced_ic_identity(anchor, out);
    inout anchor, out;
    electrical anchor, out;
    real level;
    analog begin
        level = 0.0;
        if (analysis("ic")) level = level + 1.0;
        if (analysis("dc")) level = level + 100.0;
        @(initial_step("ic")) level = level + 2.0;
        @(final_step("ic")) level = level + 4.0;
        V(out) <+ level;
    end
endmodule
"#,
    );
    let deck = format!(
        "* forced-IC physical analysis identity\n\
         R1 anchor 0 1g\n\
         X1 anchor out va_forced_ic_identity\n\
         .ic V(anchor)=1\n\
         .va \"{}\" va_forced_ic_identity\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse forced-IC lifecycle deck");
    let ordinary = Engine::default()
        .run_dc_op(&netlist)
        .expect("ordinary DC operating point runs");
    assert!((node_voltage(&ordinary, "out") - 100.0).abs() < 1.0e-12);

    let (forced, _) = Engine::default()
        .run_dc_op_forced_ic_with_report_and_abort(&netlist, &NoAbort)
        .expect("forced-IC operating point runs");
    assert!((node_voltage(&forced, "anchor") - 1.0).abs() < 1.0e-12);
    assert!((node_voltage(&forced, "out") - 7.0).abs() < 1.0e-12);

    let _ = std::fs::remove_file(model);
}

#[test]
fn dc_operating_point_exposes_initial_and_final_step_together() {
    let model = write_model(
        "single_point",
        r#"
`include "disciplines.vams"
module va_dc_single_point(p, n);
    inout p, n;
    electrical p, n;
    real level;
    analog begin
        level = 0.0;
        @(initial_step("dc")) level = level + 1.0;
        @(final_step("dc")) level = level + 2.0;
        V(p, n) <+ level;
    end
endmodule
"#,
    );
    let deck = format!(
        "* one-point DC lifecycle\n\
         X1 out 0 va_dc_single_point\n\
         .va \"{}\" va_dc_single_point\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let result = Engine::default().run_dc_op(&netlist).expect("DC OP runs");
    assert!((node_voltage(&result, "out") - 3.0).abs() < 1.0e-12);

    let _ = std::fs::remove_file(model);
}

#[test]
fn initial_and_final_step_state_is_committed_at_the_public_sweep_endpoints() {
    let model = write_model(
        "sweep_initial",
        r#"
`include "disciplines.vams"
module va_dc_sweep_initial(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("dc")) count = count + 1.0;
        @(final_step("dc")) count = count + 10.0;
        V(p, n) <+ count;
    end
endmodule
"#,
    );
    let deck = format!(
        "* persistent DC sweep lifecycle\n\
         VSW sense 0 0\n\
         X1 out 0 va_dc_sweep_initial\n\
         .va \"{}\" va_dc_sweep_initial\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "VSW", -1.0, 2.0, 1.0)
        .expect("DC source sweep runs");
    assert_eq!(points.len(), 4);
    for (point_index, (coordinate, result)) in points.into_iter().enumerate() {
        let expected = if point_index == 3 { 11.0 } else { 1.0 };
        assert!(
            (node_voltage(&result, "out") - expected).abs() < 1.0e-12,
            "unexpected lifecycle value at sweep coordinate {coordinate}: expected {expected}"
        );
    }

    let _ = std::fs::remove_file(model);
}

#[test]
fn above_crossing_state_is_committed_between_source_sweep_points() {
    let model = write_model(
        "sweep_above",
        r#"
`include "disciplines.vams"
module va_dc_sweep_above(sense, out);
    input sense;
    output out;
    electrical sense, out;
    real latched;
    analog begin
        @(above(V(sense))) latched = 1.0;
        V(out) <+ latched;
    end
endmodule
"#,
    );
    let deck = format!(
        "* same-time DC above lifecycle\n\
         VSW sense 0 0\n\
         X1 sense out va_dc_sweep_above\n\
         .va \"{}\" va_dc_sweep_above\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "VSW", -1.0, 3.0, 2.0)
        .expect("DC source sweep runs");
    let observed = points
        .iter()
        .map(|(_, result)| node_voltage(result, "out"))
        .collect::<Vec<_>>();
    assert_eq!(observed.len(), 3);
    assert!(observed[0].abs() < 1.0e-12, "{observed:?}");
    assert!((observed[1] - 1.0).abs() < 1.0e-12, "{observed:?}");
    assert!((observed[2] - 1.0).abs() < 1.0e-12, "{observed:?}");

    let _ = std::fs::remove_file(model);
}

#[test]
fn two_rebuilt_temperature_circuits_continue_one_accepted_lifecycle() {
    let model = write_model("rebuilt_temp", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* rebuilt TEMP sweep lifecycle\n\
         X1 out 0 va_dc_rebuild_lifecycle\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse rebuilt TEMP lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "TEMP", 25.0, 26.0, 1.0)
        .expect("rebuilt TEMP sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilt_global_parameter_sweep_continues_accepted_veriloga_state() {
    let model = write_model("rebuilt_global_param", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* rebuilt global parameter lifecycle\n\
         .param GAIN=1\n\
         X1 out 0 va_dc_rebuild_lifecycle gain={{GAIN}}\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse rebuilt parameter lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "gain", 1.0, 3.0, 1.0)
        .expect("rebuilt global parameter sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilt_device_parameter_sweep_continues_accepted_veriloga_state() {
    let model = write_model("rebuilt_device_param", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* rebuilt device parameter lifecycle\n\
         RLOAD sense 0 1k\n\
         X1 out 0 va_dc_rebuild_lifecycle\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse rebuilt device lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "rload:r", 1.0e3, 3.0e3, 1.0e3)
        .expect("rebuilt device parameter sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

#[test]
fn nested_rebuilt_sweep_uses_flattened_public_point_boundaries() {
    let model = write_model("rebuilt_nested", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* nested rebuilt lifecycle\n\
         VSW sense 0 0\n\
         X1 out 0 va_dc_rebuild_lifecycle\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse nested rebuilt lifecycle deck");
    let outer = rspice_core::netlist::DcSecondSweep::linear("TEMP".to_string(), 25.0, 26.0, 1.0);
    let points = Engine::default()
        .run_dc_sweep2_with_abort(
            &netlist,
            "VSW",
            DcSweepRange {
                start: 0.0,
                stop: 1.0,
                step: 1.0,
            },
            Some(&outer),
            &NoAbort,
        )
        .expect("nested rebuilt sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 1.0, 1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

/// A hoisted inner-`if` snapshot does not leak across Newton evaluations.
///
/// The frontend snapshots every non-trivial `if` condition into an
/// unconditional `__guardN` assignment; inside an untaken block the snapshot
/// reads whatever the block did not write, which on the bytecode route is the
/// previous evaluation's value of `t` (3.0, so `t > 0.1` reads true from the
/// second evaluation on) and on the JIT route is zero. Its only consumer is
/// ANDed with the block's own condition, so `g` stays 1.0 on both routes and
/// the operating point is the same whichever route this build selects:
/// `--features veriloga` evaluates the bytecode, `--features veriloga-native`
/// the x64 JIT. The finite oracle in `rspice-veriloga` pins the snapshot
/// itself; this pins that the engine never sees it.
#[test]
fn a_hoisted_condition_snapshot_does_not_leak_across_evaluations() {
    let model = write_model(
        "condition_snapshot",
        r#"
`include "disciplines.vams"
module va_condition_snapshot(p, n);
    inout p, n;
    electrical p, n;
    parameter real vth = 0.5;
    real g, t;
    analog begin
        g = 1.0;
        if (V(p, n) > vth) begin
            t = V(p, n) - vth;
            if (t > 0.1) g = 2.0;
        end
        t = 3.0;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );
    // V(in) = 0.3 keeps V(p, n) below vth on every iteration, so the block is
    // never taken and the device is a 1 S conductance: V(out) = 0.3 / (1 + 1e-3).
    let deck = format!(
        "* hoisted condition snapshot across evaluations\n\
         V1 in 0 0.3\n\
         X1 in out va_condition_snapshot\n\
         R1 out 0 1k\n\
         .va \"{}\" va_condition_snapshot\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse snapshot deck");
    let result = Engine::default().run_dc_op(&netlist).expect("DC OP runs");
    let expected = 0.3 / (1.0 + 1.0e-3);
    let actual = node_voltage(&result, "out");
    assert!(
        (actual - expected).abs() < 1.0e-9,
        "V(out) = {actual}, expected {expected}: g must stay 1.0 on the second evaluation on"
    );

    let _ = std::fs::remove_file(model);
}
