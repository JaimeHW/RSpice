//! Native JIT contract tests.
//!
//! Native mode is full JIT or error. These tests intentionally exercise the
//! foundation backend before broad canonical-IR codegen exists: construction
//! must return a native JIT error, not create a device that runs the VM.
#![cfg(feature = "native")]

use rspice_veriloga::codegen::Instruction;
use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::native::{compile_native, compile_native_with_canonical_ir};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::HashMap;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("Verilog-A source must compile")
}

fn dependent_default_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_dependent_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 2.0;
    parameter real derived = base * 3.0;
    analog I(p, n) <+ V(p, n) * derived;
endmodule
"#,
    )
}

fn dependent_default_chain_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_dependent_default_chain(p, n);
    inout p, n;
    electrical p, n;
    parameter real a = 2.0;
    parameter real b = a * 2.0;
    parameter real c = b + 1.0;
    analog I(p, n) <+ V(p, n) * c;
endmodule
"#,
    )
}

fn dependent_default_clamp_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_dependent_default_clamp(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 4.0;
    parameter real limited = base * 3.0 from [1.0:10.0];
    analog I(p, n) <+ V(p, n) * limited;
endmodule
"#,
    )
}

fn dependent_default_param_given_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_dependent_default_param_given(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 2.0;
    parameter real scale = $param_given(base) ? 5.0 : 3.0;
    parameter real derived = $param_given(scale) ? scale : scale + 2.0;
    analog I(p, n) <+ V(p, n) * derived;
endmodule
"#,
    )
}

fn dependent_default_binary_math_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_dependent_default_binary_math(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 3.0;
    parameter real squared = pow(base, 2.0);
    parameter real angle = atan2(squared, squared);
    parameter real gain = squared + angle;
    analog I(p, n) <+ V(p, n) * gain;
endmodule
"#,
    )
}

fn simple_resistor_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module rnative(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    )
}

fn multi_stamp_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_multi_stamp(p, n);
    inout p, n;
    electrical p, n;
    parameter real g1 = 0.25;
    parameter real g2 = 0.75;
    analog begin
        I(p, n) <+ g1 * V(p, n);
        I(p, n) <+ g2 * V(p, n);
    end
endmodule
"#,
    )
}

fn potential_branch_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    )
}

fn assignment_fed_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module assign_native(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = 0.25;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    )
}

fn chained_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module assign_chain_native(p, n);
    inout p, n;
    electrical p, n;
    real a, b;
    analog begin
        a = 0.25;
        b = a * 2.0;
        I(p, n) <+ b * V(p, n);
    end
endmodule
"#,
    )
}

fn scalar_context_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_context_scalar(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = (($temperature - 300.0) * 1.0e-3) + (2.0 * $abstime) + (3.0 * $mfactor);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn analysis_guard_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_analysis_guard(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = analysis("dc") ? 1.0 : 10.0;
        gain = gain + (analysis("tran") ? 2.0 : 0.0);
        gain = gain + (analysis("static") ? 4.0 : 0.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn above_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_above_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gate;
    analog begin
        gate = above(V(p, n), 1.5);
        I(p, n) <+ gate;
    end
endmodule
"#,
    )
}

fn timer_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_timer_assignment(p, n);
    inout p, n;
    electrical p, n;
    real tick;
    analog begin
        tick = timer(1.0, 0.5);
        I(p, n) <+ tick;
    end
endmodule
"#,
    )
}

fn transition_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_transition_assignment(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = transition(V(p, n) > 0.5, 0.2, 0.4, 0.4);
        I(p, n) <+ y;
    end
endmodule
"#,
    )
}

fn slew_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_slew_assignment(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = slew(V(p, n), 2.0, 2.0);
        I(p, n) <+ y;
    end
endmodule
"#,
    )
}

fn absdelay_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_absdelay_assignment(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = absdelay(V(p, n), 0.5);
        I(p, n) <+ y;
    end
endmodule
"#,
    )
}

fn cross_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_cross_assignment(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = cross(V(p, n), 1);
        I(p, n) <+ y;
    end
endmodule
"#,
    )
}

fn thermal_voltage_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_thermal_voltage(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ $vt * V(p, n);
endmodule
"#,
    )
}

fn sqrt_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_sqrt(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sqrt(V(p, n) * V(p, n) + 9.0);
endmodule
"#,
    )
}

fn abs_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_abs_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = abs($temperature - 320.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn comparison_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_comparison_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = (($temperature > 310.0) * 1.0)
             + (($temperature < 310.0) * 2.0)
             + (($temperature >= 315.0) * 4.0)
             + (($temperature <= 315.0) * 8.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn equality_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_equality_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = (($temperature == 315.0) * 1.0)
             + (($temperature != 315.0) * 2.0)
             + (($temperature == 314.0) * 4.0)
             + (($temperature != 314.0) * 8.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn logical_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_logical_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = ((($temperature > 300.0) && ($temperature < 320.0)) * 1.0)
             + ((($temperature < 300.0) || ($temperature > 314.0)) * 2.0)
             + ((!($temperature < 300.0)) * 4.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn logical_truthiness_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_logical_truthiness(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = (($abstime && 1.0) * 1.0)
             + (($abstime || 0.0) * 2.0)
             + ((!$abstime) * 4.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn ifelse_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_ifelse_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = $abstime ? 2.0 : 3.0;
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn minmax_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_minmax_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = min($temperature, 320.0) + max($temperature - 300.0, 0.0);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn exp_limexp_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_exp_limexp_assignment(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = exp($abstime) + limexp(($temperature - 300.0) * 0.1);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn transcendental_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_transcendental_assignment(p, n);
    inout p, n;
    electrical p, n;
    real x;
    real gain;
    analog begin
        x = $abstime + 0.25;
        gain = log(x + 2.0)
             + log10(x + 10.0)
             + sin(x)
             + cos(x)
             + tan(x * 0.1)
             + sinh(x * 0.1)
             + cosh(x * 0.1)
             + tanh(x)
             + asin(x * 0.1)
             + acos(x * 0.1)
             + atan(x);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn binary_math_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_binary_math_assignment(p, n);
    inout p, n;
    electrical p, n;
    real x;
    real gain;
    analog begin
        x = $abstime + 0.25;
        gain = pow(x + 2.0, 2.0)
             + ((x + 1.0) ** 3.0)
             + atan2(x, x + 0.5);
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn rounding_mod_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_rounding_mod_assignment(p, n);
    inout p, n;
    electrical p, n;
    real x;
    real gain;
    analog begin
        x = V(p, n) + 0.25;
        gain = floor(x)
             + ceil(x / 2.0)
             + (x % 2.0);
        I(p, n) <+ gain;
    end
endmodule
"#,
    )
}

fn integer_bit_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_integer_bit_assignment(p, n);
    inout p, n;
    electrical p, n;
    real x;
    real gain;
    analog begin
        x = V(p, n) + 8.75;
        gain = (x << 1.0)
             + ((-x) >> 1.0)
             + (x & 6.0)
             + (x | 3.0)
             + (x ^ 5.0);
        I(p, n) <+ gain;
    end
endmodule
"#,
    )
}

fn table_model_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_table_assignment(p, n);
    inout p, n;
    electrical p, n;
    real x;
    real gain;
    analog begin
        x = V(p, n);
        gain = $table_model(x, 0.0, 0.0, 1.0, 2.0, 2.0, 8.0);
        I(p, n) <+ gain;
    end
endmodule
"#,
    )
}

fn limit_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_limit_assignment(p, n);
    inout p, n;
    electrical p, n;
    real x;
    real explicit_limit;
    real default_limit;
    analog begin
        x = V(p, n);
        explicit_limit = $limit(x, 0.5);
        default_limit = $limit(x);
        I(p, n) <+ explicit_limit + default_limit;
    end
endmodule
"#,
    )
}

fn flag_context_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_context_flags(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    parameter real rknob = 2.0 from (0:inf);
    real gain;
    analog begin
        gain = (2.0 * $param_given(rknob)) + (3.0 * $port_connected(opt));
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}

fn runtime_loop_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module loop_native(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 2 from [1:4];
    integer i;
    real total;
    analog begin
        total = 0.0;
        for (i = 0; i < nseg; i = i + 1)
            total = total + V(p, n);
        I(p, n) <+ total;
    end
endmodule
"#,
    )
}

fn runtime_loop_limit_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module loop_limit_native(p, n);
    inout p, n;
    electrical p, n;
    parameter integer keep_running = 1;
    integer i;
    analog begin
        i = 0;
        while (keep_running)
            i = i + 1;
        I(p, n) <+ i * V(p, n);
    end
endmodule
"#,
    )
}

fn runtime_loop_truthiness_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module loop_truth_native(p, n);
    inout p, n;
    electrical p, n;
    parameter real gate = -0.0;
    integer i;
    analog begin
        i = 0;
        while (gate)
            i = i + 1;
        I(p, n) <+ i * V(p, n);
    end
endmodule
"#,
    )
}

fn indexed_assignment_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module indexed_native(p, n);
    inout p, n;
    electrical p, n;
    parameter integer idx = 1;
    real w[1:2];
    analog begin
        w[1] = 1.0;
        w[2] = 3.0;
        w[idx] = 5.0;
        I(p, n) <+ (w[1] + 10.0 * w[2]) * V(p, n);
    end
endmodule
"#,
    )
}

fn dynamic_array_read_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_dyn_array_read(p, n);
    inout p, n;
    electrical p, n;
    parameter integer sel = 2;
    real w[1:3];
    analog begin
        w[1] = 2.0;
        w[2] = 4.0;
        w[3] = 8.0;
        I(p, n) <+ w[sel] * V(p, n);
    end
endmodule
"#,
    )
}

fn static_condition_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_static_condition(p, n);
    inout p, n;
    electrical p, n;
    parameter real enabled = 1.0;
    real guard;
    analog begin
        guard = enabled;
        if (guard)
            I(p, n) <+ V(p, n) * 2.0;
    end
endmodule
"#,
    )
}

fn static_condition_branch_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_static_branch(p, n);
    inout p, n;
    electrical p, n;
    parameter integer shorted = 0;
    analog begin
        if (shorted > 0)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
    )
}

fn static_condition_mfactor_branch_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_static_mfactor_branch(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if ($mfactor > 1.5)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
    )
}

fn internal_node_divider_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r = 1.0 from (0:inf);
    analog begin
        I(p, mid) <+ (V(p) - V(mid)) / r;
        I(mid, n) <+ V(mid, n) / r;
    end
endmodule
"#,
    )
}

fn assert_native_hard_fail_message(msg: &str) {
    assert!(
        msg.contains("native JIT"),
        "error must identify native JIT failure, got: {msg}"
    );
    assert!(
        msg.contains("no interpreter fallback"),
        "error must state the hard-fail contract, got: {msg}"
    );
}

fn stamp_device(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> (HashMap<(usize, usize), f64>, HashMap<usize, f64>) {
    let mut matrix = HashMap::new();
    let mut rhs = HashMap::new();

    device.stamp(
        voltages,
        |row, col, value| {
            *matrix.entry((row, col)).or_insert(0.0) += value;
        },
        |row, value| {
            *rhs.entry(row).or_insert(0.0) += value;
        },
    );

    (matrix, rhs)
}

fn stamp_reactive_device(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> HashMap<(usize, usize), f64> {
    let mut matrix = HashMap::new();

    device.stamp_reactive(voltages, |row, col, value| {
        *matrix.entry((row, col)).or_insert(0.0) += value;
    });

    matrix
}

fn noise_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module noisy(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) * 1.0e-3
                 + white_noise(1.0e-18, "thermal")
                 + flicker_noise(2.0e-18, 1.0, "flicker");
    end
endmodule
"#,
    )
}

fn reactive_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module capjit(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog begin
        I(p, n) <+ ddt(c * V(p, n));
    end
endmodule
"#,
    )
}

fn reactive_current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module reactive_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n);
        I(p, n) <+ ddt(I(p, n) * V(p, n));
    end
endmodule
"#,
    )
}

fn idt_current_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_idt_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 0.5);
endmodule
"#,
    )
}

fn idtmod_current_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_idtmod_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idtmod(1.0, 0.0, 1.0);
endmodule
"#,
    )
}

fn laplace_current_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_laplace_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), {1.0}, {1.0, 1.0});
endmodule
"#,
    )
}

fn zi_current_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_zi_current(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), {0.25}, {1.0, -0.75}, 1.0e-6);
        I(p, n) <+ y;
    end
endmodule
"#,
    )
}

fn current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n);
        I(p, n) <+ I(p, n) * 0.1;
    end
endmodule
"#,
    )
}

fn unavailable_current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_missing_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ I(p, n);
endmodule
"#,
    )
}

fn nonfinite_prior_current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_nonfinite_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) / 0.0;
        I(p, n) <+ I(p, n);
    end
endmodule
"#,
    )
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_compile_accepts_simple_resistor_subset() {
    let model = simple_resistor_model();

    let native = compile_native(&model).expect("x64 native JIT must compile simple resistor");

    assert_eq!(native.native_stamp_count(), model.stamp_programs.len());
    assert_eq!(
        native.plan_stats().jacobian_entry_points,
        model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum::<usize>()
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_compile_with_canonical_ir_accepts_simple_resistor_stamp_path() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_contract_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");

    let native = compile_native_with_canonical_ir(&model, &artifact)
        .expect("x64 native JIT must compile canonical stamp value path");

    assert_eq!(native.native_stamp_count(), model.stamp_programs.len());
    assert_eq!(native.plan_stats().stamp_value_entry_points, 1);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_dependent_parameter_defaults_without_fallback() {
    let model = dependent_default_model();
    assert!(
        model
            .parameters
            .iter()
            .any(|parameter| parameter.default_program.is_some()),
        "fixture must contain a dependent parameter default program"
    );

    let mut default_base = VerilogADevice::try_new("DEPDEFAULT1", model.clone(), &[1, 0])
        .expect("dependent default model uses native JIT");
    assert!(default_base.is_using_native());
    default_base.update_voltages(&[2.0]);
    let currents = default_base
        .try_evaluate()
        .expect("native dependent default evaluates from constant base");
    assert!(
        (currents[0] - 12.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut overridden_base = VerilogADevice::try_new("DEPDEFAULT2", model.clone(), &[1, 0])
        .expect("dependent default model uses native JIT");
    assert!(overridden_base.set_parameter("base", 4.0));
    overridden_base
        .try_resolve_parameter_defaults()
        .expect("native dependent default refresh succeeds");
    overridden_base.update_voltages(&[2.0]);
    let currents = overridden_base
        .try_evaluate()
        .expect("native dependent default re-evaluates after base override");
    assert!(
        (currents[0] - 24.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut overridden_derived = VerilogADevice::try_new("DEPDEFAULT3", model, &[1, 0])
        .expect("dependent default model uses native JIT");
    assert!(overridden_derived.set_parameter("base", 4.0));
    assert!(overridden_derived.set_parameter("derived", 9.0));
    overridden_derived
        .try_resolve_parameter_defaults()
        .expect("native dependent default refresh skips explicit derived parameter");
    overridden_derived.update_voltages(&[2.0]);
    let currents = overridden_derived
        .try_evaluate()
        .expect("native dependent default skips param-given target");
    assert!(
        (currents[0] - 18.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_dependent_parameter_defaults_resolve_in_declaration_order() {
    let model = dependent_default_chain_model();
    assert_eq!(
        model
            .parameters
            .iter()
            .filter(|parameter| parameter.default_program.is_some())
            .count(),
        2,
        "fixture must contain a dependent default chain"
    );

    let mut device = VerilogADevice::try_new("DEPCHAIN1", model, &[1, 0])
        .expect("dependent default chain model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[2.0]);
    let currents = device
        .try_evaluate()
        .expect("native dependent default chain evaluates");

    assert!(
        (currents[0] - 10.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_dependent_parameter_defaults_apply_declared_bounds() {
    let model = dependent_default_clamp_model();
    assert!(
        model
            .parameters
            .iter()
            .any(|parameter| parameter.default_program.is_some() && parameter.max == Some(10.0)),
        "fixture must contain a bounded dependent default"
    );

    let mut device = VerilogADevice::try_new("DEPCLAMP1", model, &[1, 0])
        .expect("bounded dependent default model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[2.0]);
    let currents = device
        .try_evaluate()
        .expect("native dependent default clamp evaluates");

    assert!(
        (currents[0] - 20.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_dependent_parameter_defaults_preserve_param_given_semantics() {
    let model = dependent_default_param_given_model();
    assert!(
        model
            .parameters
            .iter()
            .filter(|parameter| parameter.default_program.is_some())
            .count()
            >= 2,
        "fixture must contain param-given dependent defaults"
    );

    let mut defaults = VerilogADevice::try_new("DEPGIVEN1", model.clone(), &[1, 0])
        .expect("param-given dependent default model uses native JIT");
    defaults.update_voltages(&[2.0]);
    let currents = defaults
        .try_evaluate()
        .expect("native default defaults preserve param_given false");
    assert!(
        (currents[0] - 10.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut base_given = VerilogADevice::try_new("DEPGIVEN2", model.clone(), &[1, 0])
        .expect("param-given dependent default model uses native JIT");
    assert!(base_given.set_parameter("base", 4.0));
    base_given
        .try_resolve_parameter_defaults()
        .expect("native param-given default refresh succeeds");
    base_given.update_voltages(&[2.0]);
    let currents = base_given
        .try_evaluate()
        .expect("native default observes explicit base");
    assert!(
        (currents[0] - 14.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut scale_given = VerilogADevice::try_new("DEPGIVEN3", model, &[1, 0])
        .expect("param-given dependent default model uses native JIT");
    assert!(scale_given.set_parameter("scale", 11.0));
    scale_given
        .try_resolve_parameter_defaults()
        .expect("native param-given default refresh skips explicit scale");
    scale_given.update_voltages(&[2.0]);
    let currents = scale_given
        .try_evaluate()
        .expect("native default observes explicit scale without marking computed defaults given");
    assert!(
        (currents[0] - 22.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_dependent_parameter_defaults_support_binary_math_functions() {
    let model = dependent_default_binary_math_model();
    assert!(
        model
            .parameters
            .iter()
            .filter(|parameter| parameter.default_program.is_some())
            .count()
            >= 3,
        "fixture must contain binary-math dependent defaults"
    );

    let mut device = VerilogADevice::try_new("DEPBINMATH1", model, &[1, 0])
        .expect("binary-math dependent default model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[2.0]);
    let currents = device
        .try_evaluate()
        .expect("native binary-math dependent defaults evaluate");

    assert!(
        (currents[0] - (18.0 + std::f64::consts::FRAC_PI_2)).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_model_image_publishes_multiple_stamp_and_jacobian_entries() {
    let model = multi_stamp_model();
    assert_eq!(model.stamp_programs.len(), 2);

    let native = compile_native(&model).expect("multi-stamp model compiles native");

    assert_eq!(native.native_stamp_count(), 2);
    assert_eq!(native.plan_stats().stamp_value_entry_points, 2);
    assert_eq!(
        native.plan_stats().jacobian_entry_points,
        model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum::<usize>()
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_multiple_flow_contributions_from_one_image() {
    let model = multi_stamp_model();
    let mut device =
        VerilogADevice::try_new("MS1", model, &[1, 0]).expect("multi-stamp model uses native JIT");

    let currents = {
        device.update_voltages(&[4.0]);
        device.try_evaluate().expect("native multi-stamp evaluate")
    };
    assert_eq!(currents.len(), 2);
    assert!((currents[0] - 1.0).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 3.0).abs() < 1e-12, "currents: {currents:?}");

    let jacobians = device
        .try_compute_jacobian()
        .expect("native multi-stamp jacobian evaluate");
    let jacobian_order = jacobians
        .iter()
        .map(|entry| (entry.program_idx, entry.jacobian_idx, entry.value))
        .collect::<Vec<_>>();
    let expected = [
        (0, 0, 0.25),
        (0, 1, -0.25),
        (0, 2, -0.25),
        (0, 3, 0.25),
        (1, 0, 0.75),
        (1, 1, -0.75),
        (1, 2, -0.75),
        (1, 3, 0.75),
    ];
    assert_eq!(
        jacobian_order.len(),
        expected.len(),
        "jacobians: {jacobians:?}"
    );
    for (actual, expected) in jacobian_order.iter().zip(expected) {
        assert_eq!(actual.0, expected.0, "jacobians: {jacobians:?}");
        assert_eq!(actual.1, expected.1, "jacobians: {jacobians:?}");
        assert!(
            (actual.2 - expected.2).abs() < 1e-12,
            "jacobians: {jacobians:?}"
        );
    }

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);
    assert!((matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_potential_branch_unknowns() {
    let model = potential_branch_model();
    assert_eq!(model.branch_sources.len(), 1);

    let mut device =
        VerilogADevice::try_new("Z1", model, &[1, 2]).expect("potential branch uses native JIT");
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = stamp_device(&mut device, &[1.0, 0.5, 1.0e-3]);

    assert!((matrix.get(&(0, 2)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 2)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 2)).copied().unwrap_or_default() + 2000.0).abs() < 1e-9);
    assert!(
        rhs.get(&2).copied().unwrap_or_default().abs() < 1e-12,
        "rhs: {rhs:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_potential_branch_unknowns_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    assert_eq!(model.branch_sources.len(), 1);
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("ZCANON1", model, &artifact, &[1, 2])
            .expect("potential branch uses canonical native JIT path");
    assert!(device.is_using_native());
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = stamp_device(&mut device, &[1.0, 0.5, 1.0e-3]);

    assert!((matrix.get(&(0, 2)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 2)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 2)).copied().unwrap_or_default() + 2000.0).abs() < 1e-9);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_named_branch_voltage_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_named_vres(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) probe;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(probe) / r;
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("NVCANON1", model, &artifact, &[1, 0])
            .expect("named branch voltage uses canonical native JIT path");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_named_branch_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_named_zres(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) probe;
    parameter real r = 2000.0 from (0:inf);
    analog V(probe) <+ I(probe) * r;
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    assert_eq!(model.branch_sources.len(), 1);
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("NZCANON1", model, &artifact, &[1, 2])
            .expect("named branch current uses canonical native JIT path");
    assert!(device.is_using_native());
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = stamp_device(&mut device, &[1.0, 0.5, 1.0e-3]);

    assert!((matrix.get(&(0, 2)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 2)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 2)).copied().unwrap_or_default() + 2000.0).abs() < 1e-9);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_simple_resistor_without_interpreter_fallback() {
    let model = simple_resistor_model();
    let mut device =
        VerilogADevice::try_new("RN1", model, &[1, 0]).expect("simple resistor uses native JIT");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_simple_resistor_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_device_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("RCANON1", model, &artifact, &[1, 0])
            .expect("simple resistor uses canonical native JIT path");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_ddt_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_ddt_current(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CDTCANON1", model, &artifact, &[1, 0])
            .expect("canonical ddt current uses native JIT path");
    assert!(device.is_using_native());

    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("canonical ddt DC evaluation succeeds")[0]
            .to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    device.set_analysis_type(2);
    device.set_timestep(0.25);
    device.update_voltages(&[3.0]);
    let transient = device
        .try_evaluate()
        .expect("canonical ddt transient evaluation succeeds")[0];
    assert!(
        (transient - 4.0e-12).abs() < 1.0e-24,
        "transient ddt current: {transient}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_idt_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_idt_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 0.5);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CIDTCANON1", model, &artifact, &[1, 0])
            .expect("canonical idt current uses native JIT path");
    assert!(device.is_using_native());

    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("canonical idt DC evaluation succeeds")[0]
            .to_bits(),
        0.5_f64.to_bits()
    );
    device.advance_state();

    device.set_analysis_type(2);
    device.set_timestep(0.25);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("canonical idt transient evaluation succeeds")[0]
            .to_bits(),
        1.0_f64.to_bits()
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_idtmod_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_idtmod_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idtmod(1.0, 0.0, 1.0);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CIMODCANON1", model, &artifact, &[1, 0])
            .expect("canonical idtmod current uses native JIT path");
    assert!(device.is_using_native());

    device.update_voltages(&[0.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("canonical idtmod DC evaluation succeeds")[0]
            .to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    device.set_analysis_type(2);
    device.set_timestep(0.25);
    let mut phases = Vec::new();
    for _ in 0..6 {
        device.update_voltages(&[0.0]);
        phases.push(
            device
                .try_evaluate()
                .expect("canonical idtmod transient evaluation succeeds")[0],
        );
        device.advance_state();
    }

    let expected = [0.25, 0.5, 0.75, 0.0, 0.25, 0.5];
    assert_eq!(phases.len(), expected.len());
    for (index, (got, want)) in phases.iter().zip(expected).enumerate() {
        assert!(
            (got - want).abs() < 1.0e-12,
            "phase {index}: got {got}, want {want}, all {phases:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_assignment_fed_variable_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_assignment_fed(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = 0.25;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("ACANON1", model, &artifact, &[1, 0])
            .expect("assignment-fed stamp uses canonical native JIT path");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[8.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.25).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_dynamic_array_variable_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_array_fed(p, n);
    inout p, n;
    electrical p, n;
    real x[0:1];
    integer i;
    analog begin
        i = 0;
        x[i] = 0.5;
        I(p, n) <+ x[i] * V(p, n);
    end
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("ARRCANON1", model, &artifact, &[1, 0])
            .expect("dynamic array stamp uses canonical native JIT path");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[8.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_stamps_simulator_context_reads_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_context_stamp(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    parameter real rknob = 2.0 from (0:inf);
    analog I(p, n) <+ (
          (($temperature - 300.0) * 1.0e-3)
        + (100.0 * $vt)
        + (2.0 * $abstime)
        + (3.0 * $mfactor)
        + (analysis("tran") ? 5.0 : 7.0)
        + (10.0 * $param_given(rknob))
        + (11.0 * $port_connected(opt))
    ) * V(p, n);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CTXCANON1", model, &artifact, &[1, 0, 0])
            .expect("context reads use canonical native JIT path");
    assert!(device.is_using_native());
    assert!(device.set_parameter("rknob", 2.0));
    device.set_analysis_type(2);
    device.set_temperature(310.0);
    device.set_time(2.0);
    device.set_multiplicity(3.0);

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);
    let local_conductance = 0.01 + (100.0 * thermal_voltage(310.0)) + 4.0 + 9.0 + 5.0 + 10.0 + 11.0;
    let expected = local_conductance * 3.0;

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - expected).abs() < 1e-12,
        "matrix: {matrix:?}, expected: {expected}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_transition_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_transition_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n) > 0.5, 0.2, 0.4, 0.4);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CTRNACANON1", model, &artifact, &[1, 0])
            .expect("canonical transition current uses native JIT path");
    assert!(device.is_using_native());
    device.set_analysis_type(2);

    for (time, expected) in [(1.0, 0.0), (1.4, 0.5), (1.6, 1.0)] {
        device.set_time(time);
        device.update_voltages(&[1.0]);
        let currents = device
            .try_evaluate()
            .expect("canonical transition evaluation succeeds");
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_slew_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_slew_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 2.0, 2.0);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CSLEWCANON1", model, &artifact, &[1, 0])
            .expect("canonical slew current uses native JIT path");
    assert!(device.is_using_native());
    device.set_analysis_type(2);
    device.update_voltages(&[10.0]);

    for (time, expected) in [(0.0, 0.0), (0.5, 1.0), (1.0, 2.0)] {
        device.set_time(time);
        let currents = device
            .try_evaluate()
            .expect("canonical slew evaluation succeeds");
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_absdelay_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_absdelay_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 0.5);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CDELAYCANON1", model, &artifact, &[1, 0])
            .expect("canonical absdelay current uses native JIT path");
    assert!(device.is_using_native());

    device.set_analysis_type(0);
    device.update_voltages(&[7.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("canonical absdelay DC evaluation succeeds")[0]
            .to_bits(),
        7.0_f64.to_bits()
    );

    device.set_analysis_type(2);
    for (time, voltage, expected) in [(0.0, 0.0, 0.0), (0.5, 1.0, 0.0), (1.0, 3.0, 1.0)] {
        device.set_time(time);
        device.update_voltages(&[voltage]);
        let currents = device
            .try_evaluate()
            .expect("canonical absdelay transient evaluation succeeds");
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }

    device.set_time(1.25);
    device.update_voltages(&[5.0]);
    let currents = device
        .try_evaluate()
        .expect("canonical absdelay interpolation succeeds");
    assert!((currents[0] - 2.0).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_laplace_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_laplace_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), {1.0}, {1.0, 1.0});
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    assert_eq!(
        model.laplace_filters.len(),
        1,
        "fixture must contain one compiled Laplace filter"
    );
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CLAPCANON1", model, &artifact, &[1, 0])
            .expect("canonical Laplace current uses native JIT path");
    assert!(device.is_using_native());

    device.update_voltages(&[4.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("canonical Laplace DC evaluation succeeds")[0]
            .to_bits(),
        4.0_f64.to_bits()
    );

    device.advance_state();
    device.set_analysis_type(2);
    device.set_timestep(0.5);
    device.update_voltages(&[4.0]);
    let currents = device
        .try_evaluate()
        .expect("canonical Laplace transient evaluation succeeds");
    assert!(
        (currents[0] - (4.0 / 3.0)).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_zi_current_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_zi_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), {0.25}, {1.0, -0.75}, 1.0e-6);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    assert_eq!(
        model.zi_filters.len(),
        1,
        "fixture must contain one compiled zi filter"
    );
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("CZICANON1", model, &artifact, &[1, 0])
            .expect("canonical zi current uses native JIT path");
    assert!(device.is_using_native());

    device.set_analysis_type(0);
    device.update_voltages(&[2.0]);
    let currents = device
        .try_evaluate()
        .expect("canonical zi DC evaluation succeeds");
    assert!(
        (currents[0] - 2.0).abs() < 1.0e-12,
        "DC currents: {currents:?}"
    );

    device.set_analysis_type(2);
    device.set_timestep(0.5e-6);

    device.set_time(0.0);
    device.update_voltages(&[1.0]);
    let first = device
        .try_evaluate()
        .expect("canonical zi first sample succeeds")[0];
    let repeated = device
        .try_evaluate()
        .expect("canonical zi repeated sample succeeds")[0];
    assert_eq!(first.to_bits(), repeated.to_bits());
    assert!((first - 0.25).abs() < 1.0e-12, "first sample: {first}");
    device.advance_state();

    device.set_time(0.5e-6);
    device.update_voltages(&[1.0]);
    let held = device
        .try_evaluate()
        .expect("canonical zi hold evaluation succeeds")[0];
    assert!((held - 0.25).abs() < 1.0e-12, "held output: {held}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_canonical_ir_cache_key_does_not_reuse_bytecode_native_image() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_cache_guard(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ hypot(V(p, n), 2.0);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = std::sync::Arc::new(compiler.compile(source).expect("compile bytecode model"));
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");

    let bytecode_device =
        VerilogADevice::try_new("BYTECACHE1", std::sync::Arc::clone(&model), &[1, 0])
            .expect("bytecode-native path compiles and populates cache");
    assert!(bytecode_device.is_using_native());

    let error = VerilogADevice::try_new_with_canonical_ir(
        "CANONCACHE1",
        std::sync::Arc::clone(&model),
        &artifact,
        &[1, 0],
    )
    .expect_err("canonical-native path must not reuse cached bytecode-native image");

    assert!(
        error.to_string().contains("intrinsic function 'hypot'"),
        "{error}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_assignment_pass() {
    let model = assignment_fed_model();
    let mut device = VerilogADevice::try_new("AN1", model, &[1, 0])
        .expect("assignment-fed model uses native JIT");
    assert!(device.is_using_native());

    let (matrix, rhs) = stamp_device(&mut device, &[8.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.25).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_assignments_in_source_order() {
    let model = chained_assignment_model();
    assert!(
        model.assignment_steps.len() >= 2,
        "fixture must contain multiple scalar assignment steps"
    );
    let mut device = VerilogADevice::try_new("ACHAIN1", model, &[1, 0])
        .expect("assignment-chain model uses native JIT");
    assert!(device.is_using_native());
    assert_eq!(device.native_plan_stats().assignment_entry_points, 1);

    let (matrix, rhs) = stamp_device(&mut device, &[8.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.5).abs() < 1e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_simulator_context_reads() {
    let model = scalar_context_model();
    let mut device = VerilogADevice::try_new("CTX1", model, &[1, 0])
        .expect("context scalar model uses native JIT");
    device.set_temperature(310.0);
    device.set_time(2.0);
    device.set_multiplicity(3.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native context scalar evaluation succeeds");

    assert!(
        (currents[0] - 52.04).abs() < 1e-12,
        "currents: {currents:?}"
    );
    assert!((device.variable("gain").unwrap() - 13.01).abs() < 1e-12);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_analysis_guards_without_fallback() {
    let model = analysis_guard_model();
    let mut device =
        VerilogADevice::try_new("ANG1", model, &[1, 0]).expect("analysis model uses native JIT");
    assert!(device.is_using_native());

    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native dc analysis guard succeeds")[0]
            .to_bits(),
        10.0_f64.to_bits()
    );

    device.set_analysis_type(2);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native transient analysis guard succeeds")[0]
            .to_bits(),
        24.0_f64.to_bits()
    );

    device.set_analysis_type(4);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native static analysis guard succeeds")[0]
            .to_bits(),
        28.0_f64.to_bits()
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_above_assignments_without_fallback() {
    let model = above_assignment_model();
    let mut device =
        VerilogADevice::try_new("ABV1", model, &[1, 0]).expect("above model uses native JIT");
    assert!(device.is_using_native());

    device.update_voltages(&[1.0]);
    let currents = device
        .try_evaluate()
        .expect("native below-threshold above evaluation succeeds");
    assert_eq!(device.variable("gate"), Some(0.0));
    assert_eq!(currents[0].to_bits(), 0.0_f64.to_bits());

    device.update_voltages(&[2.0]);
    let currents = device
        .try_evaluate()
        .expect("native above-threshold above evaluation succeeds");
    assert_eq!(device.variable("gate"), Some(1.0));
    assert_eq!(currents[0].to_bits(), 1.0_f64.to_bits());
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_timer_assignments_without_fallback() {
    let model = timer_assignment_model();
    let mut device =
        VerilogADevice::try_new("TMR1", model, &[1, 0]).expect("timer model uses native JIT");
    assert!(device.is_using_native());
    device.set_analysis_type(2);
    device.set_timestep(0.01);

    for (time, expected) in [(0.75, 0.0), (1.0, 1.0), (1.25, 0.0), (1.5, 1.0)] {
        device.set_time(time);
        let currents = device
            .try_evaluate()
            .expect("native timer evaluation succeeds");
        assert_eq!(device.variable("tick"), Some(expected), "time: {time}");
        assert_eq!(currents[0].to_bits(), expected.to_bits(), "time: {time}");
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_transition_assignments_without_fallback() {
    let model = transition_assignment_model();
    let mut device =
        VerilogADevice::try_new("TRN1", model, &[1, 0]).expect("transition model uses native JIT");
    assert!(device.is_using_native());
    device.set_analysis_type(2);

    for (time, expected) in [(1.0, 0.0), (1.4, 0.5), (1.6, 1.0)] {
        device.set_time(time);
        device.update_voltages(&[1.0]);
        let currents = device
            .try_evaluate()
            .expect("native transition evaluation succeeds");
        assert!(
            (device.variable("y").unwrap() - expected).abs() < 1e-12,
            "time: {time}"
        );
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_slew_assignments_without_fallback() {
    let model = slew_assignment_model();
    let mut device =
        VerilogADevice::try_new("SLW1", model, &[1, 0]).expect("slew model uses native JIT");
    assert!(device.is_using_native());
    device.set_analysis_type(2);
    device.update_voltages(&[10.0]);

    for (time, expected) in [(0.0, 0.0), (0.5, 1.0), (1.0, 2.0)] {
        device.set_time(time);
        let currents = device
            .try_evaluate()
            .expect("native slew evaluation succeeds");
        assert!(
            (device.variable("y").unwrap() - expected).abs() < 1e-12,
            "time: {time}"
        );
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_absdelay_assignments_without_fallback() {
    let model = absdelay_assignment_model();
    let mut device =
        VerilogADevice::try_new("DLY1", model, &[1, 0]).expect("absdelay model uses native JIT");
    assert!(device.is_using_native());

    device.set_analysis_type(0);
    device.update_voltages(&[7.0]);
    let dc_currents = device
        .try_evaluate()
        .expect("native absdelay DC evaluation succeeds");
    assert_eq!(device.variable("y"), Some(7.0));
    assert_eq!(dc_currents[0], 7.0);

    device.set_analysis_type(2);
    for (time, voltage, expected) in [(0.0, 0.0, 0.0), (0.5, 1.0, 0.0), (1.0, 3.0, 1.0)] {
        device.set_time(time);
        device.update_voltages(&[voltage]);
        let currents = device
            .try_evaluate()
            .expect("native absdelay transient evaluation succeeds");
        assert!(
            (device.variable("y").unwrap() - expected).abs() < 1e-12,
            "time: {time}"
        );
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }

    device.set_time(1.25);
    device.update_voltages(&[5.0]);
    let currents = device
        .try_evaluate()
        .expect("native absdelay interpolation succeeds");
    assert!((device.variable("y").unwrap() - 2.0).abs() < 1e-12);
    assert!((currents[0] - 2.0).abs() < 1e-12);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_cross_assignments_without_fallback() {
    let model = cross_assignment_model();
    let mut device =
        VerilogADevice::try_new("XNG1", model, &[1, 0]).expect("cross model uses native JIT");
    assert!(device.is_using_native());

    device.set_analysis_type(0);
    device.update_voltages(&[7.0]);
    let dc_currents = device
        .try_evaluate()
        .expect("native cross DC evaluation succeeds");
    assert_eq!(device.variable("y"), Some(0.0));
    assert_eq!(dc_currents[0], 0.0);

    device.set_analysis_type(2);
    for (time, voltage, expected) in [(0.0, -1.0, 0.0), (0.5, 1.0, 1.0), (1.0, 2.0, 0.0)] {
        device.set_time(time);
        device.update_voltages(&[voltage]);
        let currents = device
            .try_evaluate()
            .expect("native cross transient evaluation succeeds");
        assert!(
            (device.variable("y").unwrap() - expected).abs() < 1e-12,
            "time: {time}"
        );
        assert!(
            (currents[0] - expected).abs() < 1e-12,
            "time: {time}, currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_thermal_voltage_context_read() {
    let model = thermal_voltage_model();
    let mut device = VerilogADevice::try_new("VT1", model, &[1, 0])
        .expect("thermal voltage model uses native JIT");
    assert!(device.is_using_native());
    device.set_temperature(315.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native thermal-voltage evaluation succeeds");

    assert!(
        (currents[0] - (thermal_voltage(315.0) * 4.0)).abs() < 1e-15,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_sqrt_expression() {
    let model = sqrt_model();
    let mut device =
        VerilogADevice::try_new("SQ1", model, &[1, 0]).expect("sqrt model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native sqrt expression evaluation succeeds");

    assert!((currents[0] - 5.0).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_abs_assignment() {
    let model = abs_assignment_model();
    let mut device = VerilogADevice::try_new("ABS1", model, &[1, 0])
        .expect("abs assignment model uses native JIT");
    assert!(device.is_using_native());
    device.set_temperature(315.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native abs assignment evaluation succeeds");

    assert!((currents[0] - 20.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(device.variable("gain"), Some(5.0));
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_ordered_comparison_assignments() {
    let model = comparison_assignment_model();
    let mut device = VerilogADevice::try_new("CMP1", model, &[1, 0])
        .expect("comparison assignment model uses native JIT");
    assert!(device.is_using_native());
    device.set_temperature(315.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native comparison assignment evaluation succeeds");

    assert!((currents[0] - 52.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(device.variable("gain"), Some(13.0));
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_equality_assignments() {
    let model = equality_assignment_model();
    let mut device = VerilogADevice::try_new("EQ1", model, &[1, 0])
        .expect("equality assignment model uses native JIT");
    assert!(device.is_using_native());
    device.set_temperature(315.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native equality assignment evaluation succeeds");

    assert!((currents[0] - 36.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(device.variable("gain"), Some(9.0));
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_logical_assignments() {
    let model = logical_assignment_model();
    let mut device = VerilogADevice::try_new("LOG1", model, &[1, 0])
        .expect("logical assignment model uses native JIT");
    assert!(device.is_using_native());
    device.set_temperature(315.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native logical assignment evaluation succeeds");

    assert!((currents[0] - 28.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(device.variable("gain"), Some(7.0));
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_preserves_logical_truthiness_boundaries() {
    let model = logical_truthiness_model();
    let cases = [
        ("within-epsilon", 0.5e-15, 4.0),
        ("at-epsilon", 1.0e-15, 0.0),
        ("outside-epsilon", 2.0e-15, 3.0),
        ("unordered", f64::NAN, 0.0),
    ];

    for (name, time, expected_gain) in cases {
        let mut device = VerilogADevice::try_new("LOGT1", model.clone(), &[1, 0])
            .expect("logical truthiness model uses native JIT");
        assert!(device.is_using_native());
        device.set_time(time);
        device.update_voltages(&[2.0]);

        let currents = device
            .try_evaluate()
            .expect("native logical truthiness evaluation succeeds");

        assert_eq!(device.variable("gain"), Some(expected_gain), "{name}");
        assert!(
            (currents[0] - (expected_gain * 2.0)).abs() < 1e-12,
            "{name}: currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_ifelse_assignments() {
    let model = ifelse_assignment_model();
    let cases = [
        ("within-epsilon", 0.5e-15, 3.0),
        ("at-epsilon", 1.0e-15, 3.0),
        ("outside-epsilon", 2.0e-15, 2.0),
        ("unordered", f64::NAN, 3.0),
    ];

    for (name, time, expected_gain) in cases {
        let mut device = VerilogADevice::try_new("IFELSE1", model.clone(), &[1, 0])
            .expect("ifelse assignment model uses native JIT");
        assert!(device.is_using_native());
        device.set_time(time);
        device.update_voltages(&[4.0]);

        let currents = device
            .try_evaluate()
            .expect("native ifelse assignment evaluation succeeds");

        assert_eq!(device.variable("gain"), Some(expected_gain), "{name}");
        assert!(
            (currents[0] - (expected_gain * 4.0)).abs() < 1e-12,
            "{name}: currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_minmax_assignments() {
    let model = minmax_assignment_model();
    let cases = [
        ("low-temperature", 295.0, 295.0),
        ("nominal-temperature", 315.0, 330.0),
        ("high-temperature", 335.0, 355.0),
    ];

    for (name, temperature, expected_gain) in cases {
        let mut device = VerilogADevice::try_new("MINMAX1", model.clone(), &[1, 0])
            .expect("min/max assignment model uses native JIT");
        assert!(device.is_using_native());
        device.set_temperature(temperature);
        device.update_voltages(&[2.0]);

        let currents = device
            .try_evaluate()
            .expect("native min/max assignment evaluation succeeds");

        assert_eq!(device.variable("gain"), Some(expected_gain), "{name}");
        assert!(
            (currents[0] - (expected_gain * 2.0)).abs() < 1e-12,
            "{name}: currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_exp_limexp_assignments() {
    let model = exp_limexp_assignment_model();
    let cases = [
        ("nominal", 0.5, 310.0),
        ("linear-limited", 0.25, 750.0),
        ("negative-limited", 0.125, -200.0),
    ];

    for (name, time, temperature) in cases {
        let mut device = VerilogADevice::try_new("EXPLIM1", model.clone(), &[1, 0])
            .expect("exp/limexp assignment model uses native JIT");
        assert!(device.is_using_native());
        device.set_time(time);
        device.set_temperature(temperature);
        device.update_voltages(&[2.0]);

        let expected_gain = time.exp() + limexp((temperature - 300.0) * 0.1);
        let currents = device
            .try_evaluate()
            .expect("native exp/limexp assignment evaluation succeeds");

        assert_eq!(device.variable("gain"), Some(expected_gain), "{name}");
        assert!(
            (currents[0] - (expected_gain * 2.0)).abs() < 1e-12,
            "{name}: currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_transcendental_assignments() {
    let model = transcendental_assignment_model();
    let cases = [("dc-ish", 0.0), ("mid", 0.5), ("large", 1.0)];

    for (name, time) in cases {
        let mut device = VerilogADevice::try_new("TRIG1", model.clone(), &[1, 0])
            .expect("transcendental assignment model uses native JIT");
        assert!(device.is_using_native());
        device.set_time(time);
        device.update_voltages(&[1.5]);

        let x = time + 0.25;
        let expected_gain = (x + 2.0).ln()
            + (x + 10.0).log10()
            + x.sin()
            + x.cos()
            + (x * 0.1).tan()
            + (x * 0.1).sinh()
            + (x * 0.1).cosh()
            + x.tanh()
            + (x * 0.1).asin()
            + (x * 0.1).acos()
            + x.atan();
        let currents = device
            .try_evaluate()
            .expect("native transcendental assignment evaluation succeeds");

        assert_eq!(device.variable("x"), Some(x), "{name}");
        assert_eq!(device.variable("gain"), Some(expected_gain), "{name}");
        assert!(
            (currents[0] - (expected_gain * 1.5)).abs() < 1e-12,
            "{name}: currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_binary_math_assignments() {
    let model = binary_math_assignment_model();
    let cases = [("dc-ish", 0.0), ("mid", 0.5), ("large", 1.0)];

    for (name, time) in cases {
        let mut device = VerilogADevice::try_new("BINMATH1", model.clone(), &[1, 0])
            .expect("binary math assignment model uses native JIT");
        assert!(device.is_using_native());
        device.set_time(time);
        device.update_voltages(&[1.5]);

        let x = time + 0.25;
        let expected_gain = (x + 2.0).powf(2.0) + (x + 1.0).powf(3.0) + x.atan2(x + 0.5);
        let currents = device
            .try_evaluate()
            .expect("native binary math assignment evaluation succeeds");

        assert_eq!(device.variable("x"), Some(x), "{name}");
        assert!(
            (device.variable("gain").unwrap() - expected_gain).abs() < 1e-12,
            "{name}: gain={:?}, expected={expected_gain}",
            device.variable("gain")
        );
        assert!(
            (currents[0] - (expected_gain * 1.5)).abs() < 1e-12,
            "{name}: currents: {currents:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_rounding_and_mod_assignments_without_fallback() {
    let model = rounding_mod_assignment_model();
    let mut device = VerilogADevice::try_new("RNDMOD1", model, &[1, 0])
        .expect("rounding/mod assignment model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[5.0]);

    let x: f64 = 5.25;
    let expected_gain = x.floor() + (x / 2.0).ceil() + (x % 2.0);
    let currents = device
        .try_evaluate()
        .expect("native rounding/mod assignment evaluation succeeds");

    assert_eq!(device.variable("x"), Some(x));
    assert_eq!(device.variable("gain"), Some(expected_gain));
    assert!((currents[0] - expected_gain).abs() < 1e-12);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_integer_bit_assignments_without_fallback() {
    let model = integer_bit_assignment_model();
    let mut device = VerilogADevice::try_new("BITOPS1", model, &[1, 0])
        .expect("integer bit assignment model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[4.0]);

    let x: f64 = 12.75;
    let expected_gain = (((x as i64) << 1_i64) as f64)
        + (((-x as i64) >> 1_i64) as f64)
        + (((x as i64) & 6_i64) as f64)
        + (((x as i64) | 3_i64) as f64)
        + (((x as i64) ^ 5_i64) as f64);
    let currents = device
        .try_evaluate()
        .expect("native integer bit assignment evaluation succeeds");

    assert_eq!(device.variable("x"), Some(x));
    assert_eq!(device.variable("gain"), Some(expected_gain));
    assert!((currents[0] - expected_gain).abs() < 1e-12);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_table_model_lookup_and_derivative_without_fallback() {
    let model = table_model_assignment_model();
    let mut device = VerilogADevice::try_new("TABLE1", model, &[1, 0])
        .expect("table model assignment uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[1.5]);

    let currents = device
        .try_evaluate()
        .expect("native table model evaluation succeeds");

    assert_eq!(device.variable("x"), Some(1.5));
    assert_eq!(device.variable("gain"), Some(5.0));
    assert_eq!(currents.len(), 1);
    assert!((currents[0] - 5.0).abs() < 1e-12, "currents: {currents:?}");

    let jacobians = device
        .try_compute_jacobian()
        .expect("native table model jacobian succeeds");
    let jacobian_order = jacobians
        .iter()
        .map(|entry| (entry.program_idx, entry.jacobian_idx, entry.value))
        .collect::<Vec<_>>();
    let expected = [(0, 0, 6.0), (0, 1, -6.0), (0, 2, -6.0), (0, 3, 6.0)];
    assert_eq!(
        jacobian_order.len(),
        expected.len(),
        "jacobians: {jacobians:?}"
    );
    for (actual, expected) in jacobian_order.iter().zip(expected) {
        assert_eq!(actual.0, expected.0, "jacobians: {jacobians:?}");
        assert_eq!(actual.1, expected.1, "jacobians: {jacobians:?}");
        assert!(
            (actual.2 - expected.2).abs() < 1e-12,
            "jacobians: {jacobians:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_limit_assignments_without_fallback() {
    let model = limit_assignment_model();
    let mut device = VerilogADevice::try_new("LIM1", model, &[1, 0])
        .expect("limit assignment model uses native JIT");
    assert!(device.is_using_native());

    device.update_voltages(&[0.0]);
    let currents = device
        .try_evaluate()
        .expect("native first limit evaluation succeeds");
    assert_eq!(device.variable("x"), Some(0.0));
    assert_eq!(device.variable("explicit_limit"), Some(0.0));
    assert_eq!(device.variable("default_limit"), Some(0.0));
    assert_eq!(currents[0].to_bits(), 0.0_f64.to_bits());

    device.update_voltages(&[10.0]);
    let currents = device
        .try_evaluate()
        .expect("native initialized upward limit evaluation succeeds");
    assert_eq!(device.variable("x"), Some(10.0));
    assert_eq!(device.variable("explicit_limit"), Some(0.5));
    assert_eq!(device.variable("default_limit"), Some(0.7));
    assert!((currents[0] - 1.2).abs() < 1e-12, "currents: {currents:?}");

    let currents = device
        .try_evaluate()
        .expect("native repeated limit evaluation succeeds");
    assert_eq!(device.variable("explicit_limit"), Some(1.0));
    assert_eq!(device.variable("default_limit"), Some(1.4));
    assert!((currents[0] - 2.4).abs() < 1e-12, "currents: {currents:?}");

    device.update_voltages(&[-10.0]);
    let currents = device
        .try_evaluate()
        .expect("native downward limit evaluation succeeds");
    assert_eq!(device.variable("explicit_limit"), Some(0.5));
    assert!(
        (device.variable("default_limit").unwrap() - 0.7).abs() < 1e-12,
        "default_limit={:?}",
        device.variable("default_limit")
    );
    assert!((currents[0] - 1.2).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_param_given_and_port_connected_reads() {
    let model = flag_context_model();

    let mut omitted = VerilogADevice::try_new("FLG1", model.clone(), &[1, 0])
        .expect("flag model uses native JIT");
    omitted.update_voltages(&[2.0]);
    assert_eq!(omitted.try_evaluate().unwrap()[0], 0.0);
    assert_eq!(omitted.variable("gain"), Some(0.0));

    let mut param_only = VerilogADevice::try_new("FLG2", model.clone(), &[1, 0])
        .expect("flag model uses native JIT");
    assert!(param_only.set_parameter("rknob", 2.0));
    param_only.update_voltages(&[2.0]);
    let currents = param_only.try_evaluate().unwrap();
    assert!((currents[0] - 4.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(param_only.variable("gain"), Some(2.0));

    let mut port_only = VerilogADevice::try_new("FLG3", model.clone(), &[1, 0, 0])
        .expect("flag model uses native JIT");
    port_only.update_voltages(&[2.0]);
    let currents = port_only.try_evaluate().unwrap();
    assert!((currents[0] - 6.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(port_only.variable("gain"), Some(3.0));

    let mut connected =
        VerilogADevice::try_new("FLG4", model, &[1, 0, 0]).expect("flag model uses native JIT");
    assert!(connected.set_parameter("rknob", 2.0));
    connected.update_voltages(&[2.0]);

    let currents = connected.try_evaluate().unwrap();
    assert!((currents[0] - 10.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(connected.variable("gain"), Some(5.0));
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_evaluates_internal_node_voltage_contributions() {
    let model = internal_node_divider_model();
    assert_eq!(model.internal_nodes, 1);
    assert_eq!(model.stamp_programs.len(), 2);

    let mut device =
        VerilogADevice::try_new("DIV1", model, &[1, 0]).expect("divider uses native JIT");
    assert!(device.is_using_native());
    device.set_internal_node_indices(&[2]);
    device.update_all_voltages(&[2.0, 0.5]);

    let currents = device
        .try_evaluate()
        .expect("native internal-node evaluation succeeds");

    assert!((currents[0] - 1.5).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 0.5).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_internal_node_jacobians() {
    let model = internal_node_divider_model();
    let mut device =
        VerilogADevice::try_new("DIV2", model, &[1, 0]).expect("divider uses native JIT");
    device.set_internal_node_indices(&[2]);

    let (matrix, rhs) = stamp_device(&mut device, &[2.0, 0.5]);

    assert!((matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(0, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 0)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 1)).copied().unwrap_or_default() - 2.0).abs() < 1e-12);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_runtime_loop_assignments_without_fallback() {
    let model = runtime_loop_model();
    assert!(
        model
            .assignment_steps
            .iter()
            .any(|step| matches!(step, rspice_veriloga::codegen::AssignmentStep::Loop { .. })),
        "fixture must contain a runtime assignment loop"
    );

    let mut default_segments = VerilogADevice::try_new("LOOP1", model.clone(), &[1, 0])
        .expect("runtime loop model uses native JIT");
    assert!(default_segments.is_using_native());
    default_segments.update_voltages(&[3.0]);
    let currents = default_segments
        .try_evaluate()
        .expect("native runtime loop evaluates default segment count");
    assert!(
        (currents[0] - 6.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut four_segments = VerilogADevice::try_new("LOOP2", model, &[1, 0])
        .expect("runtime loop model uses native JIT");
    assert!(four_segments.set_parameter("nseg", 4.0));
    four_segments
        .try_resolve_parameter_defaults()
        .expect("runtime loop parameter refresh succeeds");
    four_segments.update_voltages(&[3.0]);
    let currents = four_segments
        .try_evaluate()
        .expect("native runtime loop evaluates updated segment count");
    assert!(
        (currents[0] - 12.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_runtime_loop_iteration_limit_hard_fails_without_fallback() {
    let model = runtime_loop_limit_model();
    assert!(
        model
            .assignment_steps
            .iter()
            .any(|step| matches!(step, rspice_veriloga::codegen::AssignmentStep::Loop { .. })),
        "fixture must contain a runtime assignment loop"
    );

    let mut device =
        VerilogADevice::try_new("LOOPLIMIT1", model, &[1, 0]).expect("loop model uses native JIT");
    device.update_voltages(&[1.0]);
    let err = device
        .try_evaluate()
        .expect_err("native runtime loop limit must hard-fail");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("runtime loop iteration limit exceeded"),
        "error must preserve runtime loop limit diagnostic, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_runtime_loop_condition_uses_exact_zero_truthiness() {
    let model = runtime_loop_truthiness_model();
    assert!(
        model
            .assignment_steps
            .iter()
            .any(|step| matches!(step, rspice_veriloga::codegen::AssignmentStep::Loop { .. })),
        "fixture must contain a runtime assignment loop"
    );

    let mut negative_zero = VerilogADevice::try_new("LOOPTRUTH1", model.clone(), &[1, 0])
        .expect("loop truthiness model uses native JIT");
    assert!(negative_zero.is_using_native());
    negative_zero.update_voltages(&[1.0]);
    let currents = negative_zero
        .try_evaluate()
        .expect("native loop exits on -0.0 condition");
    assert_eq!(currents[0].to_bits(), 0.0_f64.to_bits());

    let mut positive_zero = VerilogADevice::try_new("LOOPTRUTH2", model.clone(), &[1, 0])
        .expect("loop truthiness model uses native JIT");
    assert!(positive_zero.set_parameter("gate", 0.0));
    positive_zero
        .try_resolve_parameter_defaults()
        .expect("loop truthiness parameter refresh succeeds");
    positive_zero.update_voltages(&[1.0]);
    let currents = positive_zero
        .try_evaluate()
        .expect("native loop exits on +0.0 condition");
    assert_eq!(currents[0].to_bits(), 0.0_f64.to_bits());

    let mut nan = VerilogADevice::try_new("LOOPTRUTH3", model, &[1, 0])
        .expect("loop truthiness model uses native JIT");
    assert!(nan.set_parameter("gate", f64::NAN));
    nan.try_resolve_parameter_defaults()
        .expect("loop truthiness parameter refresh succeeds");
    nan.update_voltages(&[1.0]);
    let err = nan
        .try_evaluate()
        .expect_err("NaN loop condition must stay active until the native loop limit");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("runtime loop iteration limit exceeded"),
        "error must preserve runtime loop limit diagnostic, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_indexed_assignments_without_fallback() {
    let model = indexed_assignment_model();
    assert!(
        model.assignment_steps.iter().any(|step| matches!(
            step,
            rspice_veriloga::codegen::AssignmentStep::AssignIndexed { .. }
        )),
        "fixture must contain an indexed assignment"
    );

    let mut first = VerilogADevice::try_new("IDXASSIGN1", model.clone(), &[1, 0])
        .expect("indexed assignment model uses native JIT");
    assert!(first.is_using_native());
    first.update_voltages(&[1.0]);
    let currents = first
        .try_evaluate()
        .expect("native indexed assignment to first element succeeds");
    assert!(
        (currents[0] - 35.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut second = VerilogADevice::try_new("IDXASSIGN2", model.clone(), &[1, 0])
        .expect("indexed assignment model uses native JIT");
    assert!(second.set_parameter("idx", 2.0));
    second.update_voltages(&[1.0]);
    let currents = second
        .try_evaluate()
        .expect("native indexed assignment to second element succeeds");
    assert!(
        (currents[0] - 51.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut out_of_range = VerilogADevice::try_new("IDXASSIGN3", model, &[1, 0])
        .expect("indexed assignment model uses native JIT");
    assert!(out_of_range.set_parameter("idx", 3.0));
    out_of_range.update_voltages(&[1.0]);
    let err = out_of_range
        .try_evaluate()
        .expect_err("indexed assignment outside bounds must hard-fail");
    let msg = err.to_string();
    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("array index 3 outside declared bounds [1:2]"),
        "error must preserve indexed assignment bounds diagnostic, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_static_conditions_without_fallback() {
    let model = static_condition_model();
    assert!(
        model
            .stamp_programs
            .iter()
            .any(|program| program.static_condition.is_some()),
        "fixture must contain a static condition program"
    );
    assert!(
        !model.assignment_steps.is_empty(),
        "fixture must route the condition through native assignment state"
    );

    let mut enabled = VerilogADevice::try_new("STATIC1", model.clone(), &[1, 0])
        .expect("static-condition model uses native JIT");
    assert!(enabled.is_using_native());
    enabled.update_voltages(&[2.0]);
    let currents = enabled
        .try_evaluate()
        .expect("native static condition evaluates enabled stamp");
    assert!(
        (currents[0] - 4.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
    let (matrix, rhs) = stamp_device(&mut enabled, &[2.0]);
    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 2.0).abs() < 1.0e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1.0e-12,
        "rhs: {rhs:?}"
    );

    let mut disabled = VerilogADevice::try_new("STATIC2", model, &[1, 0])
        .expect("static-condition model uses native JIT");
    assert!(disabled.set_parameter("enabled", 0.0));
    disabled
        .try_resolve_parameter_defaults()
        .expect("native static condition refreshes after parameter update");
    disabled.update_voltages(&[2.0]);
    let currents = disabled
        .try_evaluate()
        .expect("native static condition skips disabled stamp");
    assert!(
        currents[0].abs() < 1.0e-12,
        "disabled currents: {currents:?}"
    );
    let (matrix, rhs) = stamp_device(&mut disabled, &[2.0]);
    assert!(matrix.is_empty(), "disabled matrix: {matrix:?}");
    assert!(rhs.is_empty(), "disabled rhs: {rhs:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_static_conditions_control_potential_branch_activation() {
    let model = static_condition_branch_model();
    assert!(
        model
            .stamp_programs
            .iter()
            .any(|program| program.static_condition.is_some()),
        "fixture must contain a static condition program"
    );
    assert_eq!(
        model.branch_sources.len(),
        1,
        "fixture must allocate one branch-current unknown"
    );

    let mut disabled = VerilogADevice::try_new("STATICBR1", model.clone(), &[1, 2])
        .expect("static branch model uses native JIT");
    disabled.set_branch_current_indices(&[3]);
    let (matrix, rhs) = stamp_device(&mut disabled, &[1.0, 0.0, 0.0]);
    assert!(
        (matrix.get(&(2, 2)).copied().unwrap_or_default() - 1.0).abs() < 1.0e-12,
        "disabled matrix: {matrix:?}"
    );
    assert!(
        !matrix.contains_key(&(0, 2)),
        "disabled branch must stay open: {matrix:?}"
    );
    assert!(rhs.is_empty(), "disabled rhs: {rhs:?}");

    let mut enabled = VerilogADevice::try_new("STATICBR2", model, &[1, 2])
        .expect("static branch model uses native JIT");
    enabled.set_branch_current_indices(&[3]);
    assert!(enabled.set_parameter("shorted", 1.0));
    enabled
        .try_resolve_parameter_defaults()
        .expect("native static branch refreshes after parameter update");
    let (matrix, rhs) = stamp_device(&mut enabled, &[1.0, 0.0, 0.0]);
    assert!(
        (matrix.get(&(0, 2)).copied().unwrap_or_default() - 1.0).abs() < 1.0e-12,
        "enabled matrix: {matrix:?}"
    );
    assert!(
        (matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1.0e-12,
        "enabled matrix: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1.0e-12,
        "enabled rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_static_condition_rejects_dynamic_guard_bytecode_without_fallback() {
    let mut model = static_condition_model();
    for program in &mut model.stamp_programs {
        if program.static_condition.is_some() {
            program.static_condition.as_mut().unwrap().instructions =
                vec![Instruction::PushVoltage(0, 1)];
        }
    }

    let err = compile_native(&model).expect_err("dynamic static-condition bytecode must hard-fail");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("StaticCondition"),
        "error must identify static-condition entry, got: {msg}"
    );
    assert!(
        msg.contains("PushVoltage"),
        "error must identify rejected dynamic op, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_static_conditions_refresh_after_mfactor_update() {
    let model = static_condition_mfactor_branch_model();
    assert!(
        model
            .stamp_programs
            .iter()
            .any(|program| program.static_condition.is_some()),
        "fixture must contain a static condition program"
    );

    let mut device = VerilogADevice::try_new("STATICM1", model, &[1, 2])
        .expect("mfactor static branch model uses native JIT");
    device.set_branch_current_indices(&[3]);
    let (matrix, _) = stamp_device(&mut device, &[1.0, 0.0, 0.0]);
    assert!(
        !matrix.contains_key(&(0, 2)),
        "default mfactor should leave branch open: {matrix:?}"
    );

    device.set_multiplicity(2.0);
    let (matrix, rhs) = stamp_device(&mut device, &[1.0, 0.0, 0.0]);
    assert!(
        (matrix.get(&(0, 2)).copied().unwrap_or_default() - 2.0).abs() < 1.0e-12,
        "mfactor-updated branch must activate and scale KCL coupling: {matrix:?}"
    );
    assert!(
        (matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1.0e-12,
        "mfactor-updated branch row must activate: {matrix:?}"
    );
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1.0e-12,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_dynamic_array_reads_without_fallback() {
    let model = dynamic_array_read_model();
    assert!(
        model
            .assignment_steps
            .iter()
            .all(|step| matches!(step, rspice_veriloga::codegen::AssignmentStep::Assign(_))),
        "fixture must use direct assignments only"
    );
    assert!(
        model.stamp_programs.iter().any(|program| {
            program
                .value_program
                .instructions
                .iter()
                .any(|instruction| {
                    matches!(
                        instruction,
                        rspice_veriloga::codegen::Instruction::PushVariableDyn { .. }
                    )
                })
        }),
        "fixture must contain a dynamic array read in a stamp expression"
    );

    let mut default_sel = VerilogADevice::try_new("DYNARR1", model.clone(), &[1, 0])
        .expect("dynamic array read model uses native JIT");
    assert!(default_sel.is_using_native());
    default_sel.update_voltages(&[2.0]);
    let currents = default_sel
        .try_evaluate()
        .expect("native dynamic array read succeeds");
    assert!(
        (currents[0] - 8.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut third = VerilogADevice::try_new("DYNARR2", model.clone(), &[1, 0])
        .expect("dynamic array read model uses native JIT");
    assert!(third.set_parameter("sel", 3.0));
    third.update_voltages(&[2.0]);
    let currents = third
        .try_evaluate()
        .expect("native dynamic array read with rounded index succeeds");
    assert!(
        (currents[0] - 16.0).abs() < 1.0e-12,
        "currents: {currents:?}"
    );

    let mut out_of_range = VerilogADevice::try_new("DYNARR3", model, &[1, 0])
        .expect("dynamic array read model uses native JIT");
    assert!(out_of_range.set_parameter("sel", 4.0));
    out_of_range.update_voltages(&[2.0]);
    let err = out_of_range
        .try_evaluate()
        .expect_err("dynamic array index outside bounds must hard-fail");
    let msg = err.to_string();
    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("array index 4 outside declared bounds [1:3]"),
        "error must preserve array bounds diagnostic, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_large_signal_noise_terms_as_zero_without_fallback() {
    let model = noise_model();
    assert!(
        model.noise_sources.len() >= 2,
        "fixture must contain compiled white and flicker noise sources"
    );

    let mut device =
        VerilogADevice::try_new("NOISE1", model, &[1, 0]).expect("noise model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native large-signal noise evaluation succeeds");
    assert_eq!(currents.len(), 1);
    assert!(
        (currents[0] - 4.0e-3).abs() < 1e-15,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_with_canonical_ir_executes_large_signal_noise_terms_as_zero_without_fallback() {
    let source = r#"
`include "disciplines.vams"
module native_canonical_noisy(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) * 1.0e-3
                 + white_noise(1.0e-18, "thermal")
                 + flicker_noise(2.0e-18, 1.0, "flicker");
    end
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    assert!(
        model.noise_sources.len() >= 2,
        "fixture must contain compiled white and flicker noise sources"
    );
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");

    let mut device =
        VerilogADevice::try_new_with_canonical_ir("NOCANON1", model, &artifact, &[1, 0])
            .expect("noise model uses canonical native JIT path");
    assert!(device.is_using_native());
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("canonical native large-signal noise evaluation succeeds");
    assert_eq!(currents.len(), 1);
    assert!(
        (currents[0] - 4.0e-3).abs() < 1e-15,
        "currents: {currents:?}"
    );
}

#[test]
fn native_noise_analysis_rejects_noise_sources_without_fallback() {
    let model = noise_model();
    assert!(
        model.noise_sources.len() >= 2,
        "fixture must contain compiled white and flicker noise sources"
    );

    let mut device =
        VerilogADevice::try_new("NOISE2", model, &[1, 0]).expect("noise model uses native JIT");
    let err = device
        .try_noise_sources(&[4.0])
        .expect_err("native noise PSD evaluation must hard-fail until implemented");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("NoiseSources"),
        "error must name unsupported native noise analysis coverage, got: {msg}"
    );
}

#[test]
fn native_compile_accepts_reactive_ddt_jacobians_without_fallback() {
    let model = reactive_model();
    assert!(
        model
            .stamp_programs
            .iter()
            .any(|stamp| !stamp.reactive_jacobians.is_empty()),
        "fixture must contain compiled reactive Jacobians"
    );

    let native = compile_native(&model).expect("native JIT must compile ddt reactive coverage");

    assert!(
        native.plan_stats().reactive_jacobian_entry_points > 0,
        "reactive Jacobian entry points must be published"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_reactive_ddt_capacitance_without_fallback() {
    let model = reactive_model();
    let mut device =
        VerilogADevice::try_new("C1", model, &[1, 0]).expect("capacitor model uses native JIT");
    assert!(device.is_using_native());

    let matrix = stamp_reactive_device(&mut device, &[2.0]);

    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0e-12).abs() < 1e-24,
        "matrix: {matrix:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_compile_rejects_reactive_current_probes_without_fallback() {
    let model = reactive_current_probe_model();

    let err = compile_native(&model).expect_err("reactive current probes must not compile native");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("PushCurrent terminal pair 0,1 unavailable"),
        "error must name unavailable reactive current dependency, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_idt_current_and_jacobian_without_fallback() {
    let model = idt_current_model();
    let mut device =
        VerilogADevice::try_new("IDT1", model, &[1, 0]).expect("idt model uses native JIT");
    assert!(device.is_using_native());

    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native idt DC evaluation succeeds")[0]
            .to_bits(),
        0.5_f64.to_bits()
    );
    device.advance_state();

    device.set_analysis_type(2);
    device.set_timestep(0.25);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native idt transient evaluation succeeds")[0]
            .to_bits(),
        1.0_f64.to_bits()
    );

    let (matrix, rhs) = stamp_device(&mut device, &[2.0]);
    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 0.25).abs() < 1e-15,
        "matrix: {matrix:?}"
    );
    assert!(
        (rhs.get(&0).copied().unwrap_or_default() + 0.5).abs() < 1e-15,
        "rhs: {rhs:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_idtmod_current_without_fallback() {
    let model = idtmod_current_model();
    let mut device =
        VerilogADevice::try_new("IMOD1", model, &[1, 0]).expect("idtmod model uses native JIT");
    assert!(device.is_using_native());

    device.update_voltages(&[0.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native idtmod DC evaluation succeeds")[0]
            .to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    device.set_analysis_type(2);
    device.set_timestep(0.25);
    let mut phases = Vec::new();
    for _ in 0..6 {
        device.update_voltages(&[0.0]);
        phases.push(
            device
                .try_evaluate()
                .expect("native idtmod transient evaluation succeeds")[0],
        );
        device.advance_state();
    }

    let expected = [0.25, 0.5, 0.75, 0.0, 0.25, 0.5];
    assert_eq!(phases.len(), expected.len());
    for (index, (got, want)) in phases.iter().zip(expected).enumerate() {
        assert!(
            (got - want).abs() < 1.0e-12,
            "phase {index}: got {got}, want {want}, all {phases:?}"
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_laplace_current_without_fallback() {
    let model = laplace_current_model();
    assert_eq!(
        model.laplace_filters.len(),
        1,
        "fixture must contain one compiled Laplace filter"
    );

    let mut device =
        VerilogADevice::try_new("LAP1", model, &[1, 0]).expect("laplace model uses native JIT");
    assert!(device.is_using_native());

    device.update_voltages(&[4.0]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("native Laplace DC evaluation succeeds")[0]
            .to_bits(),
        4.0_f64.to_bits()
    );

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);
    assert!(
        (matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1.0e-12,
        "matrix: {matrix:?}"
    );
    assert!(
        rhs.get(&0).copied().unwrap_or_default().abs() < 1.0e-12,
        "rhs: {rhs:?}"
    );

    device.advance_state();
    device.set_analysis_type(2);
    device.set_timestep(0.5);
    device.update_voltages(&[4.0]);
    let currents = device
        .try_evaluate()
        .expect("native Laplace transient evaluation succeeds");
    assert!(
        (currents[0] - (4.0 / 3.0)).abs() < 1.0e-12,
        "currents: {currents:?}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_zi_current_without_fallback() {
    let model = zi_current_model();
    assert_eq!(
        model.zi_filters.len(),
        1,
        "fixture must contain one compiled zi filter"
    );

    let mut device =
        VerilogADevice::try_new("ZI1", model, &[1, 0]).expect("zi model uses native JIT");
    assert!(device.is_using_native());

    device.set_analysis_type(0);
    device.update_voltages(&[2.0]);
    let currents = device
        .try_evaluate()
        .expect("native zi DC evaluation succeeds");
    assert!(
        (currents[0] - 2.0).abs() < 1.0e-12,
        "DC currents: {currents:?}"
    );
    assert_eq!(device.variable("y"), Some(2.0));

    device.set_analysis_type(2);
    device.set_timestep(0.5e-6);

    device.set_time(0.0);
    device.update_voltages(&[1.0]);
    let first = device
        .try_evaluate()
        .expect("native zi first sample succeeds")[0];
    let repeated = device
        .try_evaluate()
        .expect("native zi repeated sample succeeds")[0];
    assert_eq!(first.to_bits(), repeated.to_bits());
    assert!((first - 0.25).abs() < 1.0e-12, "first sample: {first}");
    device.advance_state();

    device.set_time(0.5e-6);
    device.update_voltages(&[1.0]);
    let held = device
        .try_evaluate()
        .expect("native zi hold evaluation succeeds")[0];
    assert!((held - 0.25).abs() < 1.0e-12, "held output: {held}");
    device.advance_state();

    device.set_time(1.0e-6);
    device.update_voltages(&[1.0]);
    let next = device
        .try_evaluate()
        .expect("native zi second sample succeeds")[0];
    assert!((next - 0.4375).abs() < 1.0e-12, "second sample: {next}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_terminal_pair_current_probes_in_source_order() {
    let model = current_probe_model();
    let mut device = VerilogADevice::try_new("CP1", model, &[1, 0])
        .expect("current probe model uses native JIT");
    assert!(device.is_using_native());
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native current-probe evaluation succeeds");

    assert_eq!(currents.len(), 2);
    assert!((currents[0] - 4.0).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 0.4).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_compile_rejects_unavailable_terminal_pair_current_probes_without_fallback() {
    let model = unavailable_current_probe_model();

    let err = compile_native(&model).expect_err("missing current probe source must not compile");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("PushCurrent terminal pair 0,1 unavailable"),
        "error must name unavailable current pair, got: {msg}"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_evaluate_rejects_nonfinite_terminal_pair_current_probes_without_fallback() {
    let model = nonfinite_prior_current_probe_model();
    let mut device = VerilogADevice::try_new("CPINF1", model, &[1, 0])
        .expect("structurally available current probe compiles native");
    device.update_voltages(&[4.0]);

    let err = device
        .try_evaluate()
        .expect_err("non-finite prior terminal-pair current must be a runtime error");
    let msg = err.to_string();

    assert!(
        msg.contains("missing terminal-pair current slot"),
        "error must match interpreter current-probe semantics, got: {msg}"
    );
}

#[test]
fn native_compile_failure_is_not_interpreter_fallback() {
    let model = unavailable_current_probe_model();

    let err = VerilogADevice::try_new("H1", model, &[1, 0])
        .expect_err("native mode must fail until a complete native image exists");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
}

#[test]
fn native_new_panics_instead_of_falling_back() {
    let model = unavailable_current_probe_model();

    let panic = std::panic::catch_unwind(|| {
        let _ = VerilogADevice::new("H2", model, &[1, 0]);
    })
    .expect_err("unchecked constructor must panic on native JIT failure");

    let msg = panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&'static str>().copied())
        .unwrap_or("<non-string panic>");
    assert_native_hard_fail_message(msg);
}

fn thermal_voltage(temperature: f64) -> f64 {
    const K_BOLTZMANN: f64 = 1.380649e-23;
    const Q_ELECTRON: f64 = 1.602176634e-19;

    K_BOLTZMANN * temperature / Q_ELECTRON
}

fn limexp(value: f64) -> f64 {
    const LIMIT: f64 = 40.0;
    if value > LIMIT {
        let exp_limit = LIMIT.exp();
        exp_limit * (1.0 + value - LIMIT)
    } else if value < -LIMIT {
        (-LIMIT).exp()
    } else {
        value.exp()
    }
}
