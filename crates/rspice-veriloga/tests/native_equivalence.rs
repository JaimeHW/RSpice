//! Native/interpreter equivalence pins.
//!
//! The Cranelift JIT must never approximate: for every model it accepts,
//! the hybrid plan (native chunks + interpreted steps) must produce the
//! same stamps as the pure bytecode interpreter. These tests run the same
//! device both ways and compare matrix and RHS entries.
#![cfg(feature = "native")]

mod support;

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
}

fn collect_stamps(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> (HashMap<(usize, usize), f64>, HashMap<usize, f64>) {
    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    let mut rhs: HashMap<usize, f64> = HashMap::new();
    device.stamp(
        voltages,
        |row, col, value| *matrix.entry((row, col)).or_insert(0.0) += value,
        |node, value| *rhs.entry(node).or_insert(0.0) += value,
    );
    (matrix, rhs)
}

/// Stamp the model with the JIT active and with the interpreter forced,
/// and require near-bit agreement on every matrix and RHS entry
fn assert_stamps_match(model: rspice_veriloga::CompiledModel, nodes: &[usize], voltages: &[f64]) {
    let mut native = VerilogADevice::new("N1", model.clone(), nodes);
    let mut interp = VerilogADevice::new("I1", model, nodes);
    interp.force_interpreter();
    assert!(
        native.is_using_native(),
        "model must JIT for this pin to mean anything"
    );

    let (nm, nr) = collect_stamps(&mut native, voltages);
    let (im, ir) = collect_stamps(&mut interp, voltages);

    assert_eq!(nm.len(), im.len(), "matrix entry sets differ");
    for (key, ival) in &im {
        let nval = nm.get(key).copied().unwrap_or(f64::NAN);
        let tol = 1e-12 * ival.abs().max(1e-30);
        assert!(
            (nval - ival).abs() <= tol,
            "matrix {key:?}: native {nval:.17e} vs interpreter {ival:.17e}"
        );
    }
    assert_eq!(nr.len(), ir.len(), "rhs entry sets differ");
    for (key, ival) in &ir {
        let nval = nr.get(key).copied().unwrap_or(f64::NAN);
        let tol = 1e-12 * ival.abs().max(1e-30);
        assert!(
            (nval - ival).abs() <= tol,
            "rhs {key}: native {nval:.17e} vs interpreter {ival:.17e}"
        );
    }
}

#[test]
fn loop_model_chunks_and_matches_interpreter() {
    // Runtime loop + dynamic array indexing + $param_given + mod: the
    // chunked plan interleaves native chunks with interpreted steps
    let model = compile(
        r#"
`include "disciplines.vams"
module hybrid(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 5 from [1:8];
    parameter real scale = 1.0;
    real w[1:8];
    integer i;
    real total, gmod;
    analog begin
        // mod + param_given exercise the newly lowered instructions
        gmod = (12 % 5) * 1.0e-4;
        if ($param_given(scale))
            gmod = gmod * scale;
        for (i = 1; i <= nseg; i = i + 1)
            w[i] = 0.001 * i * V(p, n);
        total = 0.0;
        for (i = 1; i <= nseg; i = i + 1)
            total = total + w[i];
        I(p, n) <+ total + gmod * V(p, n);
    end
endmodule
"#,
    );

    let device = VerilogADevice::new("C1", model.clone(), &[1, 0]);
    assert!(
        device.native_chunk_count() > 0,
        "loop-bearing model must still produce native chunks"
    );
    drop(device);

    assert_stamps_match(model, &[1, 0], &[0.7]);
}

#[test]
fn voltage_dependent_chain_matches_interpreter() {
    let model = compile(
        r#"
`include "disciplines.vams"
module sq(p, n);
    inout p, n;
    electrical p, n;
    real gm;
    analog begin
        gm = 2.0 * V(p, n);
        I(p, n) <+ gm * V(p, n) + limexp(V(p, n)) * 1.0e-12;
    end
endmodule
"#,
    );
    assert_stamps_match(model, &[1, 0], &[0.6]);
}

fn bsim4_model() -> Option<rspice_veriloga::CompiledModel> {
    let Some(path) = support::optional_bsim4_va_path(env!("CARGO_MANIFEST_DIR")) else {
        eprintln!("bsim4.va not present; skipping");
        return None;
    };
    Some(
        VerilogACompiler::default()
            .compile_file(path.as_path())
            .expect("bsim4 compiles"),
    )
}

#[test]
fn bsim4_stamps_match_interpreter() {
    let Some(model) = bsim4_model() else { return };
    // d g s b on nodes 1..3 with b grounded; vgs=1.0, vds=1.2
    assert_stamps_match(model, &[2, 1, 3, 0], &[1.2, 1.0, 0.0]);
}

/// Manual throughput measurement (run with --ignored --nocapture in
/// release): stamps/second for the hybrid JIT vs the pure interpreter
#[test]
#[ignore]
fn bsim4_stamp_throughput() {
    let Some(model) = bsim4_model() else { return };

    let time_stamps = |device: &mut VerilogADevice, evals: u32| -> f64 {
        let mut matrix_sink = 0.0;
        let mut rhs_sink = 0.0;
        let start = std::time::Instant::now();
        for i in 0..evals {
            let vd = 1.2 + (i % 7) as f64 * 1e-6;
            device.stamp(
                &[vd, 1.0, 0.0],
                |_, _, v| matrix_sink += v,
                |_, v| rhs_sink += v,
            );
        }
        let elapsed = start.elapsed().as_secs_f64();
        assert!((matrix_sink + rhs_sink).is_finite());
        elapsed / evals as f64
    };

    let time_evaluate = |device: &mut VerilogADevice, evals: u32| -> f64 {
        device.update_voltages(&[1.2, 1.0, 0.0]);
        let mut sink = 0.0;
        let start = std::time::Instant::now();
        for _ in 0..evals {
            sink += device.evaluate().iter().sum::<f64>();
        }
        let elapsed = start.elapsed().as_secs_f64();
        assert!(sink.is_finite());
        elapsed / evals as f64
    };

    use rspice_veriloga::codegen::AssignmentStep;
    fn count_instructions(steps: &[AssignmentStep]) -> usize {
        steps
            .iter()
            .map(|s| match s {
                AssignmentStep::Assign(a) => a.program.instructions.len(),
                AssignmentStep::AssignIndexed { index, value, .. } => {
                    index.instructions.len() + value.instructions.len()
                }
                AssignmentStep::Loop { condition, body } => {
                    condition.instructions.len() + count_instructions(body)
                }
            })
            .sum()
    }
    let assign_instrs = count_instructions(&model.assignment_steps);
    let value_instrs: usize = model
        .stamp_programs
        .iter()
        .map(|p| p.value_program.instructions.len())
        .sum();
    let jac_instrs: usize = model
        .stamp_programs
        .iter()
        .flat_map(|p| p.jacobian_programs.iter())
        .map(|j| j.program.instructions.len())
        .sum();
    println!(
        "instructions: assignments {assign_instrs}, values {value_instrs}, jacobians {jac_instrs}"
    );

    let mut native = VerilogADevice::new("N1", model.clone(), &[2, 1, 3, 0]);
    assert!(native.is_using_native());
    println!("plan: {:?}", native.native_plan_stats());
    let mut interp = VerilogADevice::new("I1", model, &[2, 1, 3, 0]);
    interp.force_interpreter();

    let native_e = time_evaluate(&mut native, 200);
    let interp_e = time_evaluate(&mut interp, 200);
    println!(
        "BSIM4 evaluate(): native {:.3} ms, interpreter {:.3} ms ({:.1}x)",
        native_e * 1e3,
        interp_e * 1e3,
        interp_e / native_e
    );

    let native_s = time_stamps(&mut native, 200);
    let interp_s = time_stamps(&mut interp, 200);
    println!(
        "BSIM4 stamp(): native {:.3} ms/eval, interpreter {:.3} ms/eval ({:.1}x)",
        native_s * 1e3,
        interp_s * 1e3,
        interp_s / native_s
    );
}
