//! End-to-end Verilog-A device regression pins.
//!
//! Compiles small Verilog-A models through the full netlist -> engine path
//! and checks DC and transient results against closed-form solutions. These
//! pin the companion-form stamping (G into both KCL rows, Ieq on the RHS)
//! and the backward-Euler ddt() state pipeline.
#![cfg(feature = "veriloga")]

use rspice_core::engine::TransientCheckpoint;
#[cfg(not(feature = "veriloga-native"))]
use rspice_core::register_precompiled_veriloga_model;
#[cfg(feature = "veriloga-native")]
use rspice_core::register_precompiled_veriloga_runtime_with_dependencies;
use rspice_core::{Engine, Netlist, SimulationConfig};
#[cfg(feature = "veriloga-native")]
use rspice_veriloga::canonical_ir::{CanonicalIrArtifact, HirExprKind};
use rspice_veriloga::codegen::{BytecodeProgram, Instruction};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

fn write_model(name: &str, source: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_test_{}_{}.va", name, std::process::id()));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

/// Netlist-safe path text (the deck parser treats backslashes as escapes)
fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

#[cfg(all(
    feature = "veriloga-native",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn canonical_artifact_with_unsupported_root(
    compiler: &VerilogACompiler,
    source: &str,
) -> CanonicalIrArtifact {
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let metadata = artifact.metadata.clone();
    let mut hir = artifact.hir.clone();
    let mut mir = artifact.mir.clone();
    let hir_root = usize::from(hir.contributions[0].expression.id);
    let mir_root = usize::from(mir.equations[0].expression.id);
    let unsupported = HirExprKind::StringLiteral {
        value: "unsupported-native-expression".into(),
    };
    hir.expressions[hir_root].kind = unsupported.clone();
    // Native lowering rebuilds the CFG from the structured analog body.
    // Poison that authoritative expression as well as the equation views.
    let rspice_veriloga::canonical_ir::hir::HirRegion::Contribution(contribution) =
        &mut hir.body[0]
    else {
        panic!("fixture has one structured contribution");
    };
    hir.expressions[usize::from(contribution.expression.id)].kind = unsupported.clone();
    contribution.expression.kind = "string".into();
    mir.expressions[mir_root].kind = unsupported;
    hir.contributions[0].expression.kind = "string".into();
    mir.equations[0].expression.kind = "string".into();
    CanonicalIrArtifact::from_parts(metadata, hir, mir)
        .expect("synthetic canonical artifact has refreshed digests")
}

fn node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("node {want} not found in {names:?}"));
    &voltages[idx]
}

#[test]
fn idtmod_checkpoint_retains_the_integral_when_the_modulus_changes() {
    let model = write_model(
        "circular_checkpoint",
        r#"
module circular_checkpoint(p, n);
    inout p, n; electrical p, n;
    analog V(p,n) <+ idtmod(0.0, 5.0, $abstime < 1.0e-6 ? 2.0 : 3.0);
endmodule
"#,
    );
    let deck = format!(
        "* circular history across a checkpoint\nX1 out 0 circular_checkpoint\n.va \"{}\" circular_checkpoint\n.end\n",
        deck_path(&model),
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    let continuous = engine.run_tran(&netlist, 1.5e-6, 1.0e-8).unwrap();
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 0.75e-6, 1.0e-8)
        .unwrap();
    let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 1.5e-6, 1.0e-8)
        .unwrap();
    assert_eq!(
        node_series(&first.node_names, &first.voltages, "out")
            .last()
            .unwrap()
            .to_bits(),
        node_series(&resumed.node_names, &resumed.voltages, "out")[0].to_bits(),
    );
    for result in [&continuous, &resumed] {
        let output = node_series(&result.node_names, &result.voltages, "out");
        for (&time, &voltage) in result.time.iter().zip(output) {
            let expected = if time < 1.0e-6 { 1.0 } else { 2.0 };
            assert!(
                (voltage - expected).abs() < 1.0e-12,
                "t={time}, V={voltage}"
            );
        }
        assert!((output.last().unwrap() - 2.0).abs() < 1.0e-12);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn model_nodeset_cannot_replace_a_failed_transient_equilibrium() {
    let model = write_model(
        "nodeset_without_equilibrium",
        r#"
module nodeset_without_equilibrium(out);
    inout out; electrical out;
    analog begin
        if (analysis("nodeset")) V(out)<+1;
        else I(out)<+V(out)*V(out)+1;
        I(out)<+ddt(V(out));
    end
endmodule
"#,
    );
    let deck = format!(
        "* no real equilibrium exists\nX1 out nodeset_without_equilibrium\n.va \"{}\" nodeset_without_equilibrium\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    // The temporary model-defined nodeset is solvable. The subsequent
    // equilibrium requires V(out)^2 + 1 = 0, which has no real solution.
    // A capacitor can still advance from the invalid seed, hiding a startup
    // failure behind a plausible-looking transient waveform.
    let error = Engine::default()
        .run_tran(&netlist, 1e-5, 1e-6)
        .expect_err("an unconverged nodeset must not become t=0");
    match error {
        rspice_core::SimulationError::ConvergenceFailed(_) => {}
        rspice_core::SimulationError::Circuit(detail) => {
            // Exhausted startup recovery retains both failures in one
            // diagnostic. It must still identify the failed DC equilibrium.
            assert!(
                detail
                    .starts_with("Transient startup failed: primary DC error: Convergence failed")
                    && detail.contains("; linearized fallback error:"),
                "unexpected startup refusal: {detail}"
            );
        }
        other => panic!("unexpected startup refusal: {other}"),
    }

    // Explicit initial conditions still provide a valid non-equilibrium
    // startup, either as hard t=0 clamps or with the operating point skipped.
    let netlist = Netlist::parse(&deck.replace(".end\n", ".ic V(out)=1\n.end\n")).unwrap();
    for startup in [
        rspice_core::engine::TransientStartupMode::OperatingPoint,
        rspice_core::engine::TransientStartupMode::Uic,
    ] {
        let result = Engine::default()
            .run_tran_with_startup_mode(&netlist, 1e-5, 1e-6, startup)
            .unwrap();
        let output = node_series(&result.node_names, &result.voltages, "out");
        assert!((output[0] - 1.0).abs() < 1e-10, "{startup:?}");
        for (&time, &voltage) in result.time.iter().zip(output) {
            let expected = (std::f64::consts::FRAC_PI_4 - time).tan();
            assert!(
                (voltage - expected).abs() < 1e-6,
                "{startup:?}, t={time}: {voltage}"
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

/// DC voltage divider: native 1k on top, Verilog-A 2k resistor on the
/// bottom. v(out) = 1 V * 2/(1+2) = 2/3 V.
#[test]
fn veriloga_resistor_divider_dc() {
    let model = write_model(
        "res",
        r#"
`include "disciplines.vams"
module va_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );

    let deck = format!(
        "* veriloga divider\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XR2 out 0 va_res r=2k\n\
         .va \"{}\" va_res\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = 2.0 / 3.0;
    assert!(
        (v_final - expected).abs() < 1e-6,
        "divider with Verilog-A resistor: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_transient_step_events_mark_exact_endpoints() {
    let model = write_model(
        "step_events",
        r#"
`include "disciplines.vams"
module va_step_events(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = 1.0e-6;
        @(initial_step("tran")) g = 1.0e-3;
        @(final_step("tran")) g = 2.0e-3;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* transient lifecycle events\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         X1 out 0 va_step_events\n\
         .va \"{}\" va_step_events\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-6, 1.0e-6)
        .expect("transient run");
    let out = node_series(&result.node_names, &result.voltages, "out");

    assert!((out[0] - 0.5).abs() < 1.0e-9, "initial point: {out:?}");
    assert!(
        (out[out.len() - 1] - 1.0 / 3.0).abs() < 1.0e-9,
        "final point: {out:?}"
    );
    assert!(
        out[1..out.len() - 1]
            .iter()
            .all(|voltage| (*voltage - 1.0 / 1.001).abs() < 1.0e-9),
        "interior points must not retain a lifecycle flag: {out:?}"
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn transient_operating_point_has_ic_and_static_queries_without_reinitialization() {
    let model = write_model(
        "analysis_phase",
        r#"module analysis_phase(p,n);
inout p,n; electrical p,n; real saved, initial_g;
analog initial saved=analysis("static") && analysis("ic") && analysis("tran") ? 1e-3 : 9e-3;
analog begin
    @(initial_step("tran")) initial_g=analysis("static") && analysis("ic") ? 1e-3 : 9e-3;
    @(initial_step("ic")) initial_g=20e-3;
    I(p,n)<+(saved+initial_g+(analysis("static") ? 1e-3 : 0))*V(p,n);
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!("* transient analysis phases\nV1 in 0 1\nR1 in out 1k\nX1 out 0 analysis_phase\n.va \"{}\" analysis_phase\n.end\n", deck_path(&model))).unwrap();
    let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
    let out = node_series(&result.node_names, &result.voltages, "out");
    assert!((out[0] - 0.25).abs() < 1e-9, "operating point: {out:?}");
    for value in &out[1..] {
        assert!(
            (*value - 1.0 / 3.0).abs() < 1e-9,
            "transient phase or initialization lifetime: {out:?}"
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_cross_refines_the_candidate_before_accepting_event_state() {
    let model = write_model(
        "cross_refinement",
        r#"
`include "disciplines.vams"
module va_cross_refinement(input_node, output_node);
    input input_node;
    output output_node;
    electrical input_node, output_node;
    real latched;
    analog begin
        @(cross(V(input_node), +1, 1.0e-12, 1.0e-6)) latched = 1.0;
        V(output_node) <+ latched;
    end
endmodule
"#,
    );

    let deck = format!(
        "* Verilog-A cross root refinement\n\
         V1 input 0 PWL(0 -1 1u 1)\n\
         X1 input output va_cross_refinement\n\
         .va \"{}\" va_cross_refinement\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse cross-refinement deck");
    let assert_event = |result: &rspice_core::engine::TransientResult, mode: &str| {
        let input = node_series(&result.node_names, &result.voltages, "input");
        let output = node_series(&result.node_names, &result.voltages, "output");
        let event_index = output
            .iter()
            .position(|value| *value > 0.5)
            .unwrap_or_else(|| panic!("{mode}: cross event must latch the output"));
        let event_time = result.time[event_index];
        let analytic_root = 0.5e-6;

        assert!(
            event_time >= analytic_root,
            "{mode}: cross event must not be accepted before the root: {event_time:.16e}"
        );
        assert!(
            event_time - analytic_root <= 1.0e-12,
            "{mode}: cross event missed time_tol: root={analytic_root:.16e}, event={event_time:.16e}"
        );
        assert!(
            input[event_index].abs() <= 1.0e-6,
            "{mode}: cross event missed expr_tol: input={:.16e} at t={event_time:.16e}",
            input[event_index]
        );
        assert!(
            output[..event_index]
                .iter()
                .all(|value| value.abs() < 1.0e-12),
            "{mode}: event-controlled state changed before the accepted root: {output:?}"
        );
        event_index
    };

    let adaptive = Engine::default()
        .run_tran(&netlist, 1.0e-6, 8.0e-7)
        .expect("adaptive cross-refinement transient run");
    assert_event(&adaptive, "adaptive");

    let locked = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(1.0e-6),
        locked_time_grid: Some(Arc::new(vec![0.0, 1.0e-6])),
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 1.0e-6, 1.0e-6)
    .expect("locked-grid cross-refinement transient run");
    let locked_event_index = assert_event(&locked, "locked grid");
    assert_eq!(
        locked.time.last().copied(),
        Some(1.0e-6),
        "an interior Verilog-A root must not consume the locked-grid endpoint"
    );
    assert!(
        locked.time.windows(2).all(|times| times[0] < times[1]),
        "root refinement and the locked-grid continuation must make strict progress: {:?}",
        locked.time
    );
    assert!(
        locked_event_index + 1 < locked.time.len(),
        "locked-grid integration must continue after restarting at the Verilog-A root: {:?}",
        locked.time
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_zi_commits_t0_and_lands_on_sample_lattice() {
    let model = write_model(
        "zi_lifecycle",
        r#"
`include "disciplines.vams"
module va_zi_lifecycle(p, n);
    inout p, n;
    electrical p, n;
    real sampled;
    analog begin
        sampled = zi_nd(1.0, '{1.0}, '{1.0}, 1.0e-6, 0.0);
        V(p, n) <+ sampled;
    end
endmodule
"#,
    );

    let deck = format!(
        "* Zi production transient lifecycle\n\
         X1 out 0 va_zi_lifecycle\n\
         .va \"{}\" va_zi_lifecycle\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 2.5e-6, 2.0e-6)
        .expect("Zi transient must accept t=0 before advancing");
    let out = node_series(&result.node_names, &result.voltages, "out");
    assert!(
        out.iter().all(|value| (*value - 1.0).abs() < 1.0e-12),
        "unity Zi source must hold one from the accepted t=0 sample: {out:?}"
    );
    for edge in [1.0e-6, 2.0e-6] {
        assert!(
            result
                .time
                .iter()
                .any(|time| (*time - edge).abs() <= f64::EPSILON * edge.max(1.0)),
            "sample edge {edge:.3e} missing from accepted grid: {:?}",
            result.time
        );
    }

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_slew_lands_on_the_exact_accepted_catch_up_corner() {
    let model = write_model(
        "slew_corner",
        r#"
`include "disciplines.vams"
module va_slew_corner(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ slew($abstime > 0.0 ? 1.0 : 0.0, 1.0e6, -1.0e6);
endmodule
"#,
    );
    let deck = format!(
        "* Slew catch-up scheduling\n\
         X1 out 0 va_slew_corner\n\
         .va \"{}\" va_slew_corner\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse slew-corner deck");
    let result = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(0.4e-6),
        locked_time_grid: Some(Arc::new(vec![0.0, 0.4e-6, 2.0e-6])),
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 2.0e-6, 2.0e-6)
    .expect("slew transient lands on its accepted catch-up corner");
    let out = node_series(&result.node_names, &result.voltages, "out");

    let ramp_index = result
        .time
        .iter()
        .position(|time| (*time - 0.4e-6).abs() <= 1.0e-18)
        .expect("locked ramp point");
    assert!((out[ramp_index] - 0.4).abs() < 1.0e-12, "{out:?}");

    let corner_index = result
        .time
        .iter()
        .position(|time| (*time - 1.0e-6).abs() <= 2.0e-18)
        .unwrap_or_else(|| panic!("accepted slew corner is missing: {:?}", result.time));
    assert!((out[corner_index] - 1.0).abs() < 1.0e-12, "{out:?}");
    assert_eq!(result.time.last().copied(), Some(2.0e-6));
    assert!(result.time.windows(2).all(|times| times[0] < times[1]));

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_transition_lands_on_exact_leading_and_trailing_corners() {
    let model = write_model(
        "transition_corners",
        r#"
`include "disciplines.vams"
module va_transition_corners(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ transition(
        $abstime > 0.0 ? 1.0 : 0.0,
        0.3e-6,
        0.4e-6,
        0.6e-6
    );
endmodule
"#,
    );
    let deck = format!(
        "* Transition exact-corner scheduling\n\
         X1 out 0 va_transition_corners\n\
         .va \"{}\" va_transition_corners\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse transition-corner deck");
    let result = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(0.2e-6),
        locked_time_grid: Some(Arc::new(vec![0.0, 0.2e-6, 2.0e-6])),
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 2.0e-6, 2.0e-6)
    .expect("transition transient lands on both accepted corners");
    let out = node_series(&result.node_names, &result.voltages, "out");

    let leading_time = 0.5e-6;
    let leading_index = result
        .time
        .iter()
        .position(|time| (*time - leading_time).abs() <= 2.0e-18)
        .unwrap_or_else(|| panic!("transition leading corner is missing: {:?}", result.time));
    assert_eq!(out[leading_index].to_bits(), 0.0_f64.to_bits());

    let trailing_time = 0.9e-6;
    let trailing_index = result
        .time
        .iter()
        .position(|time| (*time - trailing_time).abs() <= 2.0e-18)
        .unwrap_or_else(|| panic!("transition trailing corner is missing: {:?}", result.time));
    assert!((out[trailing_index] - 1.0).abs() < 1.0e-12, "{out:?}");
    assert_eq!(result.time.last().copied(), Some(2.0e-6));
    assert!(result.time.windows(2).all(|times| times[0] < times[1]));

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_transition_cancels_later_queued_events_on_a_locked_grid() {
    let model = write_model(
        "transition_queue_cancel",
        r#"
`include "disciplines.vams"
module va_transition_queue_cancel(p, n);
    inout p, n;
    electrical p, n;
    real target;
    analog begin
        target = $abstime < 0.2e-6 ? 0.0
               : $abstime < 0.4e-6 ? 1.0
               : $abstime < 0.6e-6 ? 2.0
               : 3.0;
        V(p, n) <+ transition(
            target,
            $abstime < 0.6e-6 ? 1.0e-6 : 0.5e-6,
            0.4e-6,
            0.4e-6
        );
    end
endmodule
"#,
    );
    let deck = format!(
        "* Transition queue cancellation\n\
         X1 out 0 va_transition_queue_cancel\n\
         .va \"{}\" va_transition_queue_cancel\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse transition queue-cancel deck");
    let result = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(0.2e-6),
        locked_time_grid: Some(Arc::new(vec![0.0, 0.2e-6, 0.4e-6, 0.6e-6, 2.0e-6])),
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 2.0e-6, 2.0e-6)
    .expect("transition queue cancellation transient succeeds");
    let out = node_series(&result.node_names, &result.voltages, "out");

    let leading_index = result
        .time
        .iter()
        .position(|time| (*time - 1.1e-6).abs() <= 2.0e-18)
        .unwrap_or_else(|| panic!("replacement leading corner is missing: {:?}", result.time));
    assert_eq!(out[leading_index].to_bits(), 0.0_f64.to_bits());
    let trailing_index = result
        .time
        .iter()
        .position(|time| (*time - 1.5e-6).abs() <= 2.0e-18)
        .unwrap_or_else(|| panic!("replacement trailing corner is missing: {:?}", result.time));
    assert!((out[trailing_index] - 3.0).abs() < 1.0e-12, "{out:?}");

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_transition_interruption_preserves_lrm_slopes_on_a_locked_grid() {
    let model = write_model(
        "transition_interrupt",
        r#"
`include "disciplines.vams"
module va_transition_interrupt(p, n);
    inout p, n;
    electrical p, n;
    real target;
    analog begin
        target = $abstime < 0.2e-6 ? 0.0
               : $abstime < 0.4e-6 ? 1.0
               : $abstime < 0.5e-6 ? 2.0
               : -1.0;
        V(p, n) <+ transition(target, 0.0, 0.4e-6, 0.6e-6);
    end
endmodule
"#,
    );
    let deck = format!(
        "* Transition LRM interruption slopes\n\
         X1 out 0 va_transition_interrupt\n\
         .va \"{}\" va_transition_interrupt\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse transition interruption deck");
    let result = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(0.1e-6),
        locked_time_grid: Some(Arc::new(vec![0.0, 0.2e-6, 0.4e-6, 0.5e-6, 2.0e-6])),
        ..SimulationConfig::default()
    })
    .run_tran(&netlist, 2.0e-6, 2.0e-6)
    .expect("transition interruption transient succeeds");
    let out = node_series(&result.node_names, &result.voltages, "out");

    for (time, expected) in [(0.4e-6, 0.5), (0.5e-6, 1.0), (0.9e-6, -1.0)] {
        let index = result
            .time
            .iter()
            .position(|actual| (*actual - time).abs() <= 2.0e-18)
            .unwrap_or_else(|| panic!("interruption point {time} is missing: {:?}", result.time));
        assert!(
            (out[index] - expected).abs() < 1.0e-12,
            "time={time} expected={expected} actual={} trace={out:?}",
            out[index]
        );
    }

    let _ = std::fs::remove_file(model);
}

#[test]
fn transient_finish_retains_the_accepted_endpoint_and_solves_final_step() {
    use rspice_core::{ModelFinishPoint, NoAbort, SimulationOutcome};
    for mixed in [false, true] {
        let source = r#"
module transient_finish(out);
    inout out; electrical out;
    real count;
    analog begin
        @(initial_step("tran")) count=1;
        @(final_step("tran")) begin count=count+10; $finish(2); end
        @(timer(2e-6)) if (count<10) $finish(1);
        V(out)<+count;
    end
endmodule
"#;
        let source = if mixed {
            source.replace(
                "real count;",
                "real count; reg marker; initial marker=1'b1;",
            )
        } else {
            source.to_owned()
        };
        let model = write_model(&format!("transient_finish_{mixed}"), &source);
        let deck = format!(
            "* finish at an accepted timer event\nX1 out transient_finish\n.va \"{}\" transient_finish\n.end\n",
            deck_path(&model)
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let outcome = Engine::default()
            .run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_tran_with_abort(&netlist, 10e-6, 0.2e-6, signal)
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: Some(result),
            finish,
        } = outcome
        else {
            panic!("accepted transient finish must return its partial waveform");
        };
        assert_eq!(finish.point, ModelFinishPoint::Transient { time: 2e-6 });
        assert_eq!(finish.diagnostic_level, 1);
        assert_eq!(result.time.last(), Some(&2e-6));
        let output = node_series(&result.node_names, &result.voltages, "out");
        assert_eq!(output.last(), Some(&11.0), "mixed={mixed}: {output:?}");
        assert!(output[..output.len() - 1].iter().all(|value| *value == 1.0));
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn transient_finish_at_origin_preserves_startup_constraints() {
    use rspice_core::{ModelFinishPoint, NoAbort, SimulationOutcome};
    for mixed in [false, true] {
        for startup in ["", ".ic V(out)=3\n", ".ic V(out)=3\n.tran 0.2u 10u UIC\n"] {
            let source = format!(
                r#"
module origin_finish(out);
    inout out; electrical out;
    real count;
    {}
    analog begin
        @(initial_step("tran")) begin count=count+1; $finish(1); end
        @(final_step("tran")) begin count=count+10; $finish(2); end
        I(out)<+V(out)-count;
    end
endmodule
"#,
                if mixed {
                    "reg marker; initial marker=1'b1;"
                } else {
                    ""
                }
            );
            let model = write_model(&format!("origin_finish_{mixed}"), &source);
            let netlist = Netlist::parse(&format!(
                "* finish at the accepted origin\nX1 out origin_finish\n.va \"{}\" origin_finish\n{startup}.end\n",
                deck_path(&model)
            )).unwrap();
            let outcome = Engine::default()
                .run_with_outcome(&NoAbort, |engine, signal| {
                    engine.run_tran_with_abort(&netlist, 10e-6, 0.2e-6, signal)
                })
                .unwrap();
            let SimulationOutcome::Finished {
                result: Some(result),
                finish,
            } = outcome
            else {
                panic!("origin finish must retain its accepted point");
            };
            assert_eq!(finish.point, ModelFinishPoint::Transient { time: 0.0 });
            assert_eq!(finish.diagnostic_level, 1);
            assert_eq!(result.time, [0.0]);
            let expected = if startup.is_empty() { 11.0 } else { 3.0 };
            let output = node_series(&result.node_names, &result.voltages, "out");
            assert!(
                (output[0] - expected).abs() < 1e-10,
                "mixed={mixed} startup={startup:?}: {output:?}"
            );
            let _ = std::fs::remove_file(model);
        }
    }
}

#[test]
fn transient_finish_refines_crossing_before_acceptance() {
    use rspice_core::{ModelFinishPoint, NoAbort, SimulationOutcome};
    for mixed in [false, true] {
        let source = format!(
            r#"
module crossing_finish(out, sense);
    inout out, sense; electrical out, sense;
    real count;
    {}
    analog begin
        @(initial_step("tran")) count=count+1;
        @(final_step("tran")) count=count+10;
        @(cross(V(sense)-0.5, 1)) if (count<10) $finish(1);
        V(out)<+count;
    end
endmodule
"#,
            if mixed {
                "reg marker; initial marker=1'b1;"
            } else {
                ""
            }
        );
        let model = write_model(&format!("crossing_finish_{mixed}"), &source);
        let netlist = Netlist::parse(&format!(
            "* refine a finish event\nV1 sense 0 PWL(0 0 1u 1)\nX1 out sense crossing_finish\n.va \"{}\" crossing_finish\n.end\n",
            deck_path(&model)
        )).unwrap();
        let outcome = Engine::default()
            .run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_tran_with_abort(&netlist, 2e-6, 0.3e-6, signal)
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: Some(result),
            finish,
        } = outcome
        else {
            panic!("crossing finish must retain its accepted point");
        };
        let ModelFinishPoint::Transient { time } = finish.point else {
            panic!("wrong finish point")
        };
        assert!((time - 0.5e-6).abs() < 1e-15, "mixed={mixed}: {time}");
        assert_eq!(result.time.last(), Some(&time));
        let output = node_series(&result.node_names, &result.voltages, "out");
        assert_eq!(output.last(), Some(&11.0));
        assert!(output[..output.len() - 1].iter().all(|value| *value == 1.0));
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn mixed_analog_timestep_controls_and_uic_lifecycle_match_the_runtime_device() {
    for uic in [false, true] {
        let mut reference: Option<rspice_core::engine::TransientResult> = None;
        for mixed in [false, true] {
            let source = format!(
                r#"
module timestep_controls(out);
    inout out; electrical out;
    real count;
    {}
    analog begin
        @(initial_step("tran")) count=count+1;
        @(final_step("tran")) count=count+10;
        $bound_step(0.1e-6);
        if ($abstime>=0.25e-6) $discontinuity(0);
        I(out)<+V(out)-count;
    end
endmodule
"#,
                if mixed {
                    "reg marker; initial marker=1'b1;"
                } else {
                    ""
                }
            );
            let model = write_model(&format!("timestep_controls_{mixed}_{uic}"), &source);
            let netlist = Netlist::parse(&format!(
                "* timestep controls\nX1 out timestep_controls\n.va \"{}\" timestep_controls\n.tran 0.5u 1u {}\n.end\n",
                deck_path(&model), if uic { "UIC" } else { "" }
            )).unwrap();
            let result = Engine::default().run_tran(&netlist, 1e-6, 0.5e-6).unwrap();
            assert!(
                result
                    .step_sizes
                    .iter()
                    .all(|dt| *dt <= 0.1e-6 * (1.0 + 1e-12))
            );
            let output = node_series(&result.node_names, &result.voltages, "out");
            assert!((output[0] - if uic { 0.0 } else { 1.0 }).abs() < 1e-10);
            assert!((output.last().unwrap() - 11.0).abs() < 1e-10);
            assert!(
                output[1..output.len() - 1]
                    .iter()
                    .all(|value| (*value - 1.0).abs() < 1e-10)
            );
            if let Some(reference) = &reference {
                assert_eq!(result.time, reference.time);
                assert_eq!(result.step_sizes, reference.step_sizes);
            } else {
                reference = Some(result);
            }
            let _ = std::fs::remove_file(model);
        }
    }
}

#[test]
fn transient_finish_keeps_checkpoint_state_and_stops_future_captures() {
    use rspice_core::engine::TransientStartupMode;
    use rspice_core::{NoAbort, SimulationOutcome};
    let model = write_model(
        "checkpoint_finish",
        r#"
module checkpoint_finish(out);
    inout out; electrical out;
    real count;
    analog begin
        @(initial_step("tran")) count=1;
        @(timer(2e-6)) $finish(1);
        @(final_step("tran")) count=count+10;
        V(out)<+count;
    end
endmodule
"#,
    );
    let netlist = Netlist::parse(&format!(
        "* finish checkpoint state\nX1 out checkpoint_finish\n.va \"{}\" checkpoint_finish\n.end\n",
        deck_path(&model)
    ))
    .unwrap();
    let engine = Engine::default();
    let final_capture = engine
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_checkpointed_with_abort(&netlist, 10e-6, 0.2e-6, signal)
        })
        .unwrap();
    let SimulationOutcome::Finished {
        result: Some((result, checkpoint)),
        ..
    } = final_capture
    else {
        panic!("finish must retain the final checkpoint");
    };
    assert_eq!(checkpoint.time, 2e-6);
    assert_eq!(
        TransientCheckpoint::from_text(&checkpoint.to_text())
            .unwrap()
            .time,
        2e-6
    );
    let out = node_series(&result.node_names, &result.voltages, "out");
    assert_eq!(out.last(), Some(&11.0));
    let scheduled = engine
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &netlist,
                10e-6,
                0.2e-6,
                TransientStartupMode::OperatingPoint,
                &[1e-6, 3e-6],
                signal,
            )
        })
        .unwrap();
    let SimulationOutcome::Finished {
        result: Some((result, checkpoints)),
        ..
    } = scheduled
    else {
        panic!("finish must retain earlier scheduled checkpoints");
    };
    assert_eq!(result.time.last(), Some(&2e-6));
    assert_eq!(checkpoints.len(), 1);
    let _ = std::fs::remove_file(model);
}

#[test]
fn portless_transient_models_execute_time_events_and_finish() {
    use rspice_core::engine::SpiceDialect;
    use rspice_core::{ModelFinishPoint, NoAbort, SimulationOutcome};
    for mixed in [false, true] {
        for (event, endpoint) in [
            ("timer(2e-6)", 2e-6),
            ("initial_step(\"tran\")", 0.0),
            ("final_step(\"tran\")", 10e-6),
        ] {
            let source = format!(
                r#"
module portless_finish;
    {}
    analog @({event}) $finish(1);
endmodule
"#,
                if mixed {
                    "reg marker; initial marker=1'b1;"
                } else {
                    ""
                }
            );
            let model = write_model(&format!("portless_finish_{mixed}"), &source);
            let netlist = Netlist::parse(&format!(
            "* a model does not need terminals to execute\nX1 portless_finish\n.va \"{}\" portless_finish\n.end\n",
            deck_path(&model)
        )).unwrap();
            for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
                let engine = Engine::new(SimulationConfig {
                    spice_dialect: dialect,
                    ..SimulationConfig::default()
                });
                let outcome = engine
                    .run_with_outcome(&NoAbort, |engine, signal| {
                        engine.run_tran_with_abort(&netlist, 10e-6, 0.2e-6, signal)
                    })
                    .unwrap();
                let SimulationOutcome::Finished {
                    result: Some(result),
                    finish,
                } = outcome
                else {
                    panic!("portless model must finish at {event}");
                };
                assert_eq!(finish.point, ModelFinishPoint::Transient { time: endpoint });
                assert_eq!(result.time.last(), Some(&endpoint));
                assert!(result.node_names.is_empty());
                assert!(result.voltages.is_empty());
                if !mixed {
                    let (_, checkpoint) = engine
                        .run_tran_checkpointed(&netlist, 10e-6, 0.2e-6)
                        .unwrap();
                    assert_eq!(
                        TransientCheckpoint::from_text(&checkpoint.to_text())
                            .unwrap()
                            .time,
                        endpoint
                    );
                }
            }
            let _ = std::fs::remove_file(model);
        }
    }
}

#[test]
fn transient_finish_retains_authored_fft_windows_and_partial_waveforms() {
    use rspice_core::{NoAbort, SimulationOutcome};
    let model = write_model(
        "fft_finish",
        r#"
module fft_finish(out);
    inout out; electrical out;
    analog begin
        @(timer(2e-6)) $finish(1);
        V(out)<+1;
    end
endmodule
"#,
    );
    let netlist = Netlist::parse(&format!(
        "* partial FFT history\nX1 out fft_finish\n.va \"{}\" fft_finish\n.fft V(out) NP=8 STOP=10u WINDOW=RECT FORMAT=UNORM\n.fft V(out) NP=8 STOP=1u WINDOW=RECT FORMAT=UNORM\n.fft V(out) NP=8 WINDOW=RECT FORMAT=UNORM\n.end\n",
        deck_path(&model)
    )).unwrap();
    let outcome = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, 10e-6, 0.2e-6, signal)
        })
        .expect("an unavailable FFT must not discard the accepted waveform");
    let SimulationOutcome::Finished {
        result: Some(result),
        ..
    } = outcome
    else {
        panic!("finish must retain its partial waveform");
    };
    assert_eq!(result.time.last(), Some(&2e-6));
    assert_eq!(result.fft_results.len(), 3);
    assert_eq!(
        Some(result.fft_results[0].stop_time),
        netlist.fft_analyses[0].stop
    );
    assert_eq!(
        Some(result.fft_results[1].stop_time),
        netlist.fft_analyses[1].stop
    );
    assert_eq!(result.fft_results[2].stop_time, 10e-6);
    for (index, fft) in result.fft_results.iter().enumerate() {
        fft.validate_status().unwrap();
        if index == 1 {
            assert!(fft.status.is_complete());
            assert!((fft.bins[0].real - 1.0).abs() < 1e-12);
            assert!(fft.bins[1..].iter().all(|bin| bin.magnitude < 1e-12));
        } else {
            assert_eq!(
                fft.status,
                rspice_core::engine::TransientFftStatus::IncompleteHistory {
                    available_start: 0.0,
                    available_stop: 2e-6
                }
            );
            assert!(fft.bins.is_empty());
            assert!(fft.metrics.is_none());
        }
    }
    let compressed = Engine::default()
        .run_tran_compressed(
            &netlist,
            10e-6,
            0.2e-6,
            rspice_core::engine::CompressionConfig::default(),
        )
        .expect("partial FFT status survives waveform compression");
    assert_eq!(compressed.post_results.fft, result.fft_results);
    let expanded = compressed.try_into_transient().unwrap();
    assert_eq!(expanded.time.last(), result.time.last());
    assert_eq!(expanded.fft_results, result.fft_results);
    let _ = std::fs::remove_file(model);
}

#[test]
fn transient_finish_fft_uses_the_last_required_sample_not_the_exclusive_stop() {
    for (name, event, endpoint, complete) in [
        ("fft_origin_finish", "initial_step", 0.0, false),
        ("fft_last_sample_finish", "timer(7.0)", 7.0, true),
    ] {
        let model = write_model(
            name,
            &format!(
                "module {name}(out);\ninout out; electrical out;\nanalog begin\n@({event}) $finish(1);\nV(out)<+1;\nend\nendmodule\n"
            ),
        );
        let netlist = Netlist::parse(&format!(
            "* stop-exclusive FFT boundary\nX1 out {name}\n.va \"{}\" {name}\n.fft V(out) NP=8 WINDOW=RECT FORMAT=UNORM\n.end\n",
            deck_path(&model)
        )).unwrap();
        let result = Engine::default().run_tran(&netlist, 8.0, 1.0).unwrap();
        assert_eq!(result.time.last(), Some(&endpoint));
        let fft = &result.fft_results[0];
        assert_eq!(fft.start_time, 0.0);
        assert_eq!(fft.stop_time, 8.0);
        assert_eq!(fft.sample_interval, 1.0);
        assert_eq!(fft.status.is_complete(), complete);
        fft.validate_status().unwrap();
        if complete {
            assert!((fft.bins[0].real - 1.0).abs() < 1e-12);
            assert!(fft.bins[1..].iter().all(|bin| bin.magnitude < 1e-12));
        } else {
            assert_eq!(
                fft.status,
                rspice_core::engine::TransientFftStatus::IncompleteHistory {
                    available_start: 0.0,
                    available_stop: 0.0,
                }
            );
        }
        let compressed = Engine::default()
            .run_tran_compressed(
                &netlist,
                8.0,
                1.0,
                rspice_core::engine::CompressionConfig::default(),
            )
            .unwrap();
        assert_eq!(compressed.post_results.fft, result.fft_results);
        assert_eq!(
            compressed.try_into_transient().unwrap().time.last(),
            Some(&endpoint)
        );
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn checkpoint_resume_does_not_repeat_model_nodeset_startup() {
    let model = write_model(
        "resume_nodeset",
        r#"
module resume_nodeset(out);
    inout out; electrical out;
    real started;
    analog begin
        @(timer(1e-6)) started=1;
        if ((analysis("nodeset") || $abstime<1e-6) && started>0)
            I(out)<+sqrt(-1-abs(V(out)));
        else I(out)<+V(out)-(1+started);
    end
endmodule
"#,
    );
    let deck = format!(
        "* accepted event state must not enter startup again\nX1 out resume_nodeset\n.va \"{}\" resume_nodeset\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    let continuous = engine.run_tran(&netlist, 4e-6, 0.2e-6).unwrap();
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 2e-6, 0.2e-6)
        .unwrap();
    assert!(checkpoint.to_text().contains("linearized_startup 0\n"));
    let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 4e-6, 0.2e-6)
        .expect("accepted model state must resume without a nodeset solve");
    let expected = node_series(&continuous.node_names, &continuous.voltages, "out");
    let actual = node_series(&resumed.node_names, &resumed.voltages, "out");
    assert_eq!(
        actual.last().unwrap().to_bits(),
        expected.last().unwrap().to_bits()
    );
    assert!(actual.iter().all(|value| (*value - 2.0).abs() < 1e-10));
    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_zi_iir_checkpoint_resume_is_bit_identical_on_and_between_edges() {
    let model = write_model(
        "zi_checkpoint",
        r#"
`include "disciplines.vams"
module va_zi_checkpoint(p, n);
    inout p, n;
    electrical p, n;
    real sampled;
    analog begin
        sampled = zi_nd(1.0, '{0.5, 0.25}, '{1.0, -0.5}, 1.0e-6, 0.0);
        V(p, n) <+ sampled;
    end
endmodule
"#,
    );
    let deck = format!(
        "* Zi checkpoint/resume\n\
         X1 out 0 va_zi_checkpoint\n\
         .va \"{}\" va_zi_checkpoint\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse Zi checkpoint deck");
    let engine = Engine::default();
    let continuous = engine
        .run_tran(&netlist, 3.5e-6, 0.2e-6)
        .expect("continuous Zi reference run");
    let expected = node_series(&continuous.node_names, &continuous.voltages, "out")
        .last()
        .copied()
        .expect("continuous endpoint");

    for checkpoint_time in [1.0e-6, 1.3e-6] {
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, checkpoint_time, 0.2e-6)
            .expect("Zi checkpoint segment solves");
        let serialized = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("runtime Verilog-A state survives portable text");
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &serialized, 3.5e-6, 0.2e-6)
            .expect("Zi checkpoint resumes");
        let actual = node_series(&resumed.node_names, &resumed.voltages, "out")
            .last()
            .copied()
            .expect("resumed endpoint");
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "Zi IIR endpoint differs after checkpoint at {checkpoint_time:.3e}"
        );
    }

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1.3e-6, 0.2e-6)
        .expect("missing-state refusal fixture captures");
    let text = checkpoint.to_text();
    let (prefix, _) = text
        .split_once("runtime_veriloga_state_available ")
        .expect("current checkpoint contains runtime Verilog-A provenance");
    let missing_state =
        format!("{prefix}runtime_veriloga_state_available 0\nruntime_veriloga_states 0\n");
    let legacy = TransientCheckpoint::from_text(&missing_state)
        .expect("state-absent checkpoint remains parseable for a precise refusal");
    let error = engine
        .run_tran_resume(&netlist, &legacy, 3.5e-6, 0.2e-6)
        .expect_err("legacy checkpoint must not invent runtime operator history");
    assert!(
        error
            .to_string()
            .contains("runtime-compiled Verilog-A accepted state"),
        "unexpected legacy refusal: {error}"
    );

    let _ = std::fs::remove_file(model);
}

/// Optional trailing terminals must remain observable to Verilog-A through
/// `$port_connected`; omitting `opt` below selects the weak conductance path.
#[test]
fn veriloga_optional_trailing_terminal_is_marked_unconnected() {
    let model = write_model(
        "optg",
        r#"
`include "disciplines.vams"
module va_optional_g(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    analog I(p, n) <+ ($port_connected(opt) ? 1e-3 : 1e-6) * V(p, n);
endmodule
"#,
    );

    let deck = format!(
        "* veriloga optional terminal\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XG1 out 0 va_optional_g\n\
         .va \"{}\" va_optional_g\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = 1.0e6 / (1.0e3 + 1.0e6);
    assert!(
        (v_final - expected).abs() < 1e-6,
        "omitted optional terminal should select weak path: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_runtime_stamp_errors_are_simulation_errors_not_panics() {
    let model = write_model(
        "runtime_oob",
        r#"
`include "disciplines.vams"

module va_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* veriloga runtime diagnostic\n\
         V1 in 0 1.0\n\
         XBAD in 0 va_runtime_oob\n\
         .va \"{}\" va_runtime_oob\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| Engine::default().run_dc_op(&netlist));

    let _ = std::fs::remove_file(model);

    let result = result.expect("Verilog-A runtime stamp errors must not panic");
    let err = result.expect_err("runtime stamp error must be reported to the caller");
    let text = err.to_string();
    assert!(
        text.contains("Verilog-A") && (text.contains("Array index 5") || text.contains("[1:4]")),
        "diagnostic should identify the Verilog-A array bounds error, got: {text}"
    );
}

#[test]
fn veriloga_dependent_parameter_default_errors_are_simulation_errors_not_zeroed() {
    let source = r#"
`include "disciplines.vams"
module va_bad_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real w = 1.0 from (0:inf);
    parameter real r = 10.0 / w from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;
    let model_path = write_model("bad_default", source);
    let mut compiled = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("model compiles before cache corruption");
    #[cfg(feature = "veriloga-native")]
    let canonical_ir = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(source)
        .expect("canonical IR compiles before cache corruption");
    compiled.parameters[1].default_program = Some(BytecodeProgram {
        instructions: vec![Instruction::PushParam(99)],
    });
    #[cfg(not(feature = "veriloga-native"))]
    register_precompiled_veriloga_model(&model_path, compiled)
        .expect("register corrupted precompiled model");
    #[cfg(feature = "veriloga-native")]
    register_precompiled_veriloga_runtime_with_dependencies(
        &model_path,
        std::slice::from_ref(&model_path),
        compiled,
        canonical_ir,
    )
    .expect("register corrupted precompiled runtime artifact");

    let deck = format!(
        "* veriloga dependent default diagnostic\n\
         V1 in 0 DC 1\n\
         XBAD in 0 va_bad_default\n\
         .va \"{}\" va_bad_default\n\
         .end\n",
        deck_path(&model_path)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| Engine::default().run_dc_op(&netlist));

    let _ = std::fs::remove_file(model_path);

    let result = result.expect("dependent default runtime errors must not panic");
    #[cfg(feature = "veriloga-native")]
    {
        let op = result.expect("native canonical defaults must ignore stale bytecode defaults");
        assert_eq!(op.branch_currents.len(), 1);
        assert!(
            (op.branch_currents[0] + 0.1).abs() < 1.0e-12,
            "canonical r = 10/w default should set source current to -0.1 A, got {:?}",
            op.branch_currents
        );
    }
    #[cfg(not(feature = "veriloga-native"))]
    {
        let err = result.expect_err("dependent default runtime error must be reported");
        let text = err.to_string();
        assert!(
            text.contains("Verilog-A") && text.contains("parameter"),
            "diagnostic should identify the Verilog-A parameter default failure, got: {text}"
        );
    }
}

#[test]
#[cfg(all(
    feature = "veriloga-native",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn veriloga_native_builder_uses_canonical_ir_without_bytecode_fallback() {
    let source = r#"
`include "disciplines.vams"
module va_canonical_required(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
"#;
    let model = write_model("canonical_required", source);
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let compiled = compiler.compile(source).expect("compile bytecode model");
    let canonical_ir = canonical_artifact_with_unsupported_root(&compiler, source);
    register_precompiled_veriloga_runtime_with_dependencies(
        &model,
        std::slice::from_ref(&model),
        compiled,
        canonical_ir,
    )
    .expect("register unsupported canonical sentinel");

    let deck = format!(
        "* native canonical IR path diagnostic\n\
         V1 in 0 DC 1\n\
         X1 in 0 va_canonical_required\n\
         .VERILOGA \"{}\" va_canonical_required\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let err = Engine::default()
        .build_circuit(&netlist)
        .expect_err("native builder must use canonical IR instead of bytecode-native fallback");
    let text = err.to_string();

    let _ = std::fs::remove_file(model);

    assert!(
        text.contains("native JIT")
            && text.contains("expression kind string")
            && text.contains("no interpreter fallback"),
        "diagnostic should prove the canonical native path hard-failed, got: {text}"
    );
}

#[test]
#[cfg(all(
    feature = "veriloga-native",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn veriloga_native_builder_runs_assignment_fed_canonical_ir_without_bytecode_fallback() {
    let model = write_model(
        "canonical_assignment_fed",
        r#"
`include "disciplines.vams"
module va_canonical_assignment_fed(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = 1.0e-3;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* native canonical IR path assignment-fed variable\n\
         V1 in 0 DC 1\n\
         R1 in out 1k\n\
         X1 out 0 va_canonical_assignment_fed\n\
         .VERILOGA \"{}\" va_canonical_assignment_fed\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_dc_op(&netlist)
        .expect("native builder must use canonical IR and solve assignment-fed model");

    let _ = std::fs::remove_file(model);

    let out_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap_or_else(|| panic!("node out not found in {:?}", result.node_names));
    let vout = result.node_voltages[out_idx];
    let expected = 0.5;
    assert!(
        (vout - expected).abs() < 1.0e-9,
        "canonical native assignment-fed conductance divider: got {vout}, want {expected}"
    );
}

/// RC charging: native 1k resistor, Verilog-A 1uF capacitor (ddt-based).
/// v(out) follows 1 - exp(-t/tau) with tau = 1 ms.
#[test]
fn veriloga_capacitor_rc_charge_matches_analytic() {
    let model = write_model(
        "cap",
        r#"
`include "disciplines.vams"
module va_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-6 from (0:inf);
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
    );

    let deck = format!(
        "* veriloga RC charge\n\
         V1 in 0 PULSE(0 1 0 1u 1u 1 2)\n\
         R1 in out 1k\n\
         XC1 out 0 va_cap c=1u\n\
         .va \"{}\" va_cap\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 3e-3, 5e-6)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let tau = 1e-3;

    // Compare against the analytic charging curve away from the pulse edge.
    let mut checked = 0usize;
    for (i, &t) in result.time.iter().enumerate() {
        if t < 5.0 * 5e-6 {
            continue; // skip the source ramp
        }
        let expected = 1.0 - (-(t - 1e-6) / tau).exp();
        let got = out[i];
        assert!(
            (got - expected).abs() < 0.02,
            "RC charge at t={t}: got {got}, want {expected}"
        );
        checked += 1;
    }
    assert!(
        checked > 50,
        "expected many compared samples, got {checked}"
    );

    // Near-final value should be close to 1 V
    let v_final = *out.last().expect("samples");
    assert!(
        (v_final - 1.0).abs() < 0.06,
        "RC settles to the source voltage, got {v_final}"
    );

    let _ = std::fs::remove_file(model);
}

/// A Verilog-A voltage contribution drives a node through a branch-current
/// unknown: V(p,n) <+ level must force v(out) = level.
#[test]
fn veriloga_voltage_source_drives_node() {
    let model = write_model(
        "vsrc",
        r#"
`include "disciplines.vams"
module va_vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real level = 1.0;
    analog V(p, n) <+ level;
endmodule
"#,
    );

    let deck = format!(
        "* veriloga voltage source\n\
         XV1 out 0 va_vsrc level=2.5\n\
         R1 out 0 1k\n\
         .va \"{}\" va_vsrc\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    assert!(
        (v_final - 2.5).abs() < 1e-9,
        "Verilog-A voltage source must pin the node, got {v_final}"
    );

    let _ = std::fs::remove_file(model);
}

/// An impedance-form resistor (V <+ I*r, the BSIM4 substrate-network
/// pattern) divides correctly against a native resistor.
#[test]
fn veriloga_impedance_resistor_divider() {
    let model = write_model(
        "zres",
        r#"
`include "disciplines.vams"
module va_zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    );

    // 1 V through native 1k on top, impedance-form 2k on the bottom:
    // v(out) = 2/3 V
    let deck = format!(
        "* veriloga impedance divider\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XZ1 out 0 va_zres r=2k\n\
         .va \"{}\" va_zres\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = 2.0 / 3.0;
    assert!(
        (v_final - expected).abs() < 1e-6,
        "impedance-form resistor divider: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}

/// Runtime (parameter-bounded) loops evaluate correctly through the
/// engine: conductance accumulated over nf iterations.
#[test]
fn veriloga_runtime_loop_conductance() {
    let model = write_model(
        "nfres",
        r#"
`include "disciplines.vams"
module va_nfres(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 1 from [1:inf);
    integer i;
    real g;
    analog begin
        g = 0.0;
        for (i = 0; i < nf; i = i + 1)
            g = g + 1.0e-3;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );

    // nf=4 fingers of 1mS each = 4mS = 250 ohm against 1k:
    // v(out) = 1 * 250/(1000+250) = 0.2 V
    let deck = format!(
        "* veriloga runtime loop\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XN1 out 0 va_nfres nf=4\n\
         .va \"{}\" va_nfres\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    assert!(
        (v_final - 0.2).abs() < 1e-9,
        "nf=4 runtime loop conductance: got {v_final}, want 0.2"
    );

    let _ = std::fs::remove_file(model);
}

/// Nonlinear Verilog-A conductance in a feedback divider converges via
/// Newton with the companion stamps: I = g*V^2 against a series resistor.
#[test]
fn veriloga_square_law_converges() {
    let model = write_model(
        "sql",
        r#"
`include "disciplines.vams"
module va_sql(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0 from (0:inf);
    real vd;
    analog begin
        vd = V(p, n);
        I(p, n) <+ g * vd * vd;
    end
endmodule
"#,
    );

    // 1 V source, 1k resistor, square-law device to ground:
    // KCL at out: (1 - v)/1000 = 1e-3 * v^2  =>  v^2 + v - 1 = 0
    // v = (sqrt(5) - 1)/2 ~= 0.61803
    let deck = format!(
        "* veriloga square law\n\
         V1 in 0 1.0\n\
         R1 in out 1k\n\
         XQ1 out 0 va_sql g=1m\n\
         .va \"{}\" va_sql\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 1e-4, 1e-5)
        .expect("transient run");

    let out = node_series(&result.node_names, &result.voltages, "out");
    let v_final = *out.last().expect("samples");
    let expected = (5.0_f64.sqrt() - 1.0) / 2.0;
    assert!(
        (v_final - expected).abs() < 1e-4,
        "square-law operating point: got {v_final}, want {expected}"
    );

    let _ = std::fs::remove_file(model);
}
