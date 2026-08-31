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
    let root = usize::from(mir.equations[0].expression.id);
    let unsupported = HirExprKind::StringLiteral {
        value: "unsupported-native-expression".into(),
    };
    hir.expressions[root].kind = unsupported.clone();
    mir.expressions[root].kind = unsupported;
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
