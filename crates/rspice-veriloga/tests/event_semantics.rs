use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

fn evaluate(device: &mut VerilogADevice, time: f64, voltage: f64) -> f64 {
    device.set_analysis_type(2);
    device.set_time(time);
    device.set_timestep(time.max(0.0));
    device.update_voltages(&[voltage]);
    device.try_evaluate().expect("event evaluation succeeds")[0]
}

#[test]
fn cross_is_newton_idempotent_and_does_not_treat_tolerance_as_a_crossing() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module tolerant_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 1, 0.0, 0.1, 1.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    assert_eq!(
        evaluate(&mut device, 1.0, -0.05).to_bits(),
        0.0_f64.to_bits(),
        "entering the expression tolerance on the same side is not a crossing"
    );
    assert_eq!(
        evaluate(&mut device, 1.0, -0.05).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 2.0, 1.0).to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        evaluate(&mut device, 2.0, 1.0).to_bits(),
        1.0_f64.to_bits(),
        "repeated Newton evaluations must preserve the crossing event"
    );
}

#[test]
fn cross_enable_disables_events_without_freezing_history() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module disabled_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 0, 0.0, 0.0, 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn above_fires_initially_and_only_on_subsequent_rising_events() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module stateful_above(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ above(V(p, n) - 1.5, 0.0, 0.0, 1.0);
endmodule
"#,
    );

    let mut initially_positive = model.device("A1", &[1, 0]);
    assert_eq!(
        evaluate(&mut initially_positive, 0.0, 2.0).to_bits(),
        1.0_f64.to_bits(),
        "above must trigger during initialization when already positive"
    );

    let mut crossing = model.device("A2", &[1, 0]);
    assert_eq!(
        evaluate(&mut crossing, 0.0, 1.0).to_bits(),
        0.0_f64.to_bits()
    );
    crossing.advance_state();
    assert_eq!(
        evaluate(&mut crossing, 1.0, 2.0).to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        evaluate(&mut crossing, 1.0, 2.0).to_bits(),
        1.0_f64.to_bits()
    );
    crossing.advance_state();
    assert_eq!(
        evaluate(&mut crossing, 2.0, 2.5).to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn above_fires_on_same_time_dc_sweep_crossings() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module dc_sweep_above(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ above(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(0);
    device.set_time(0.0);

    device.update_voltages(&[-1.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
    device.advance_state();

    device.update_voltages(&[1.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    device.advance_state();

    device.update_voltages(&[2.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
}

#[test]
fn full_event_control_argument_lists_compile() {
    DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module event_argument_lists(p, n);
    inout p, n;
    electrical p, n;
    real enabled, state;
    analog begin
        enabled = 1.0;
        @(cross(V(p, n), 1, 1.0e-12, 1.0e-6, enabled)) state = 1.0;
        @(above(V(p, n), 1.0e-12, 1.0e-6, enabled)) state = 2.0;
        @(cross(V(p, n), 1, , , enabled)) state = 3.0;
        @(above(V(p, n), , , enabled)) state = 4.0;
        I(p, n) <+ state;
    end
endmodule
"#,
    );
}

#[test]
fn null_event_optional_slots_use_runtime_defaults() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module null_event_arguments(p, n);
    inout p, n;
    electrical p, n;
    real enabled;
    analog begin
        enabled = 1.0;
        I(p, n) <+ cross(V(p, n), 1, , , enabled)
                   + above(V(p, n), , , enabled);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 1.0, 1.0),
        2.0,
        "null tolerances must select defaults without disabling either rising event"
    );
}

#[test]
fn cross_event_direction_is_evaluated_at_runtime() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module dynamic_cross_direction(p, n);
    inout p, n;
    electrical p, n;
    real direction, count;
    analog begin
        direction = V(p, n) >= 0.0 ? 1.0 : -1.0;
        @(cross(V(p, n), direction)) count = count + 1.0;
        I(p, n) <+ count;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 1.0);
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 2.0, -1.0),
        2.0,
        "a falling crossing must fire after the runtime direction changes to -1"
    );
}

#[test]
fn event_controlled_variables_are_newton_transactional_and_resume_exactly() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module transactional_event_counter(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(cross(V(p, n), +1)) count = count + 1.0;
        I(p, n) <+ count;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();

    assert_eq!(evaluate(&mut device, 1.0, 1.0), 1.0);
    assert_eq!(
        evaluate(&mut device, 1.0, 1.0),
        1.0,
        "a repeated Newton pass must replay from accepted procedural state"
    );
    assert_eq!(
        evaluate(&mut device, 1.0, -0.5),
        0.0,
        "a retreating Newton pass must discard the rejected crossing update"
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 2.0, 1.0), 1.0);
    device.advance_state();
    let checkpoint = device.checkpoint_state().unwrap();

    let mut restored = model.device("A1", &[1, 0]);
    restored.validate_checkpoint_state(&checkpoint).unwrap();
    restored.apply_validated_checkpoint_state(&checkpoint);
    assert_eq!(evaluate(&mut restored, 3.0, -1.0), 1.0);
    restored.advance_state();
    assert_eq!(evaluate(&mut restored, 4.0, 1.0), 2.0);
    assert_eq!(evaluate(&mut restored, 4.0, 1.0), 2.0);
}

#[test]
fn checkpoint_before_acceptance_excludes_step_event_variable_candidates() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module transactional_initial_step(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("tran")) count = count + 1.0;
        I(p, n) <+ count;
    end
endmodule
"#,
    );
    let count_index = model
        .variable_names
        .iter()
        .position(|name| name == "count")
        .expect("counter variable is present");
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    device.set_analysis_step(true, false);
    device.update_voltages(&[0.0]);

    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    assert_eq!(
        device.checkpoint_state().unwrap().accepted.variables[count_index],
        0.0,
        "a pre-accept checkpoint must retain the prior accepted counter"
    );

    device.advance_state();
    assert_eq!(
        device.checkpoint_state().unwrap().accepted.variables[count_index],
        1.0
    );
}

#[test]
fn step_events_follow_phase_and_analysis_filters() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module step_events(p, n);
    inout p, n;
    electrical p, n;
    real event_value;
    analog begin
        event_value = 0.0;
        @(initial_step("tran", "vendor_extension")) event_value = event_value + 1.0;
        @(final_step("tran")) event_value = event_value + 2.0;
        I(p, n) <+ event_value;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[0.0]);

    device.set_analysis_type(2);
    device.set_analysis_step(false, false);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        0.0_f64.to_bits()
    );

    device.set_analysis_step(true, false);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        1.0_f64.to_bits()
    );

    device.set_analysis_step(false, true);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        2.0_f64.to_bits()
    );

    device.set_analysis_step(true, true);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        3.0_f64.to_bits()
    );

    device.set_analysis_type(0);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn step_events_reject_non_string_analysis_filters() {
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let error = compiler
        .compile(
            r#"
`include "disciplines.vams"
module invalid_step_event(p, n);
    inout p, n;
    electrical p, n;
    analog @(initial_step("tran", 1.0)) I(p, n) <+ 1.0;
endmodule
"#,
        )
        .expect_err("non-string step-event filter must be rejected");
    assert!(error.to_string().contains("string literals"), "{error}");
}

/// `@(posedge expr)` has no digital solver behind it, so it compiles to the
/// analog `cross` operator — a continuous zero-crossing detector on the value
/// of the operand, which is not edge detection.
///
/// That reading is the conventional Verilog-A one and is kept, but it changes
/// what the source means, so it is said out loud on the compile report rather
/// than applied in silence.
#[test]
fn edge_events_on_continuous_signals_report_their_cross_interpretation() {
    let report = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(
            r#"
`include "disciplines.vams"
module edge_on_continuous(p, n);
    inout p, n;
    electrical p, n;
    real hits;
    analog begin
        hits = 0.0;
        @(posedge V(p, n)) hits = 1.0;
        @(negedge V(p, n)) hits = 2.0;
        I(p, n) <+ hits * V(p, n);
    end
endmodule
"#,
            Some("edge_on_continuous"),
        )
        .expect("an edge event on a continuous signal still compiles");

    let edge_diagnostics: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "VA-SEM-EDGE-EVENT-AS-CROSS")
        .collect();
    assert_eq!(
        edge_diagnostics.len(),
        2,
        "both edge events must be reported: {:?}",
        report.diagnostics
    );
    assert!(edge_diagnostics[0].message.contains("posedge"));
    assert!(edge_diagnostics[0].message.contains("cross"));
    assert!(edge_diagnostics[1].message.contains("negedge"));
    for diagnostic in edge_diagnostics {
        assert!(
            diagnostic.span.is_some(),
            "an edge event is pinned to its source position"
        );
    }
}

/// On a discrete-discipline signal the `cross` reading is simply wrong: a
/// digital net carries no continuous value to cross zero, so the event would
/// never fire. Refuse it instead of stamping a detector that does nothing.
///
/// The construct itself is legal — Verilog-AMS LRM 2.4 section 7.3.4's
/// `analog_event_expression` lists `posedge expression` beside the analog event
/// functions, and its worked example declares the operand a `wire`; section
/// 7.3.6.2 runs the guarded statement "at the time corresponding to a real
/// promotion of the digital time". So the message names the seam that is
/// missing rather than the construct, and cites the clauses it is short of.
#[test]
fn edge_events_on_discrete_signals_are_refused() {
    for keyword in ["posedge", "negedge"] {
        let source = format!(
            r#"
`include "disciplines.vams"
discipline digital_wire
    domain discrete;
enddiscipline
module edge_on_discrete(p, n);
    inout p, n;
    electrical p, n;
    digital_wire clk;
    real hits;
    analog begin
        hits = 0.0;
        @({keyword} clk) hits = 1.0;
        I(p, n) <+ hits * V(p, n);
    end
endmodule
"#
        );
        let error = rspice_veriloga::VerilogACompiler::default()
            .compile_module(&source, Some("edge_on_discrete"))
            .expect_err("a digital edge event must not compile to a cross detector");
        let message = error.to_string();
        assert!(
            message.contains(&format!("`{keyword} clk`")),
            "expected the refused construct to be named, got {message:?}"
        );
        assert!(
            message.contains("discrete-discipline"),
            "expected the discipline domain to be named, got {message:?}"
        );
    }
}

/// The built-in `logic` discipline is discrete too, and reaches the same
/// refusal without any user discipline declaration.
#[test]
fn edge_events_on_the_builtin_logic_discipline_are_refused() {
    let error = rspice_veriloga::VerilogACompiler::default()
        .compile_module(
            r#"
`include "disciplines.vams"
module edge_on_logic(p, n);
    inout p, n;
    electrical p, n;
    logic clk;
    real hits;
    analog begin
        hits = 0.0;
        @(posedge clk) hits = 1.0;
        I(p, n) <+ hits * V(p, n);
    end
endmodule
"#,
            Some("edge_on_logic"),
        )
        .expect_err("a digital edge event must not compile to a cross detector");
    assert!(error.to_string().contains("`posedge clk`"), "{error}");
}
