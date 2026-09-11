//! $bound_step / $discontinuity lowering: hidden per-evaluation
//! variables drive the engine's stepper. The bound resets to +inf and
//! takes the min of every active call; the discontinuity flag resets to
//! zero and reports rising edges across accepted steps.

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile_device(instance: &str, source: &str) -> VerilogADevice {
    compile_selected_device(instance, source, None)
}

fn compile_selected_device(instance: &str, source: &str, module: Option<&str>) -> VerilogADevice {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let report = compiler
        .compile_runtime(source, module)
        .expect("compile timestep-control model");
    VerilogADevice::try_new_with_canonical_ir(instance, report.model, &report.canonical_ir, &[1, 0])
        .expect("construct timestep-control device from paired runtime artifacts")
}

fn stamp_once(device: &mut VerilogADevice, voltages: &[f64]) {
    device.stamp(voltages, |_, _, _| {}, |_, _| {});
}

#[test]
fn switch_branch_discontinuity_follows_acceptance_and_checkpoint_restore() {
    let report = VerilogACompiler::default()
        .compile_runtime(
            "module switched(p,n); inout p,n; electrical p,n;
        analog if(V(p,n)>0) V(p,n)<+2*I(p,n); else I(p,n)<+3*V(p,n); endmodule",
            None,
        )
        .unwrap();
    let kind_slot = report.model.switch_branch_variables[0];
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        "SWITCH",
        report.model,
        &report.canonical_ir,
        &[1, 0],
    )
    .unwrap();
    device.set_internal_node_indices(&[2]);
    stamp_once(&mut device, &[-1.0, 0.0]);
    assert!(!device.discontinuity_rising());
    device.advance_state();
    for _ in 0..3 {
        stamp_once(&mut device, &[1.0, 0.0]);
        assert!(
            device.discontinuity_rising(),
            "Newton retries must retain the hint"
        );
    }
    stamp_once(&mut device, &[-1.0, 0.0]);
    assert!(
        !device.discontinuity_rising(),
        "an unaccepted mode must not replace accepted state"
    );
    stamp_once(&mut device, &[1.0, 0.0]);
    device.advance_state();
    assert!(!device.discontinuity_rising());
    let checkpoint = device.checkpoint_state().unwrap();
    stamp_once(&mut device, &[-1.0, 0.0]);
    assert!(
        device.discontinuity_rising(),
        "consecutive mode changes each require a hint"
    );
    device.advance_state();
    device.validate_checkpoint_state(&checkpoint).unwrap();
    device.apply_validated_checkpoint_state(&checkpoint);
    stamp_once(&mut device, &[1.0, 0.0]);
    assert!(!device.discontinuity_rising());
    let mut invalid = checkpoint.clone();
    invalid.accepted.variables[kind_slot] = 2.0;
    assert!(
        device
            .validate_checkpoint_state(&invalid)
            .unwrap_err()
            .to_string()
            .contains("switch-branch")
    );
}

const BOUNDED: &str = r#"
`include "disciplines.vams"
module bounded(p, n);
    inout p, n;
    electrical p, n;
    parameter real cap = 1.0e-9;
    analog begin
        $bound_step(1.0e-6);
        if (V(p, n) > 0.5)
            $bound_step(cap);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn bound_step_takes_the_min_of_active_calls() {
    let mut device = compile_device("B1", BOUNDED);

    // Guard inactive: only the unconditional 1 us bound applies
    stamp_once(&mut device, &[0.2]);
    assert_eq!(device.transient_bound_step(), Some(1.0e-6));

    // Guard active: min(1u, 1n) = 1 ns
    stamp_once(&mut device, &[0.8]);
    assert_eq!(device.transient_bound_step(), Some(1.0e-9));

    // And it resets per evaluation rather than latching
    stamp_once(&mut device, &[0.2]);
    assert_eq!(device.transient_bound_step(), Some(1.0e-6));
}

#[test]
fn control_tasks_inside_runtime_loops_reset_once_per_evaluation() {
    let mut device = compile_device(
        "LOOP",
        r#"
module loop_controls(p,n);
    inout p,n; electrical p,n;
    parameter integer passes=3;
    integer i;
    analog begin
        for (i=1; i<=passes; i=i+1) begin
            $bound_step(i*1e-9);
            if (i==1) $discontinuity(0);
        end
        I(p,n) <+ V(p,n)*1e-3;
    end
endmodule
"#,
    );
    stamp_once(&mut device, &[0.5]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1e-9));
    assert!(device.discontinuity_pending());
    device.advance_state();

    assert!(device.try_set_parameter("passes", 0.0).unwrap());
    stamp_once(&mut device, &[0.5]);
    assert_eq!(device.try_transient_bound_step().unwrap(), None);
    assert!(!device.discontinuity_pending());
    device.advance_state();

    assert!(device.try_set_parameter("passes", 2.0).unwrap());
    stamp_once(&mut device, &[0.5]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(1e-9));
    assert!(device.discontinuity_rising());
}

const UNBOUNDED: &str = r#"
`include "disciplines.vams"
module plain(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0e-3;
endmodule
"#;

#[test]
fn models_without_bound_step_report_none() {
    let mut device = compile_device("P1", UNBOUNDED);
    stamp_once(&mut device, &[1.0]);
    assert_eq!(device.transient_bound_step(), None);
    assert!(!device.discontinuity_pending());
}

const PARAMETERIZED_BOUND: &str = r#"
`include "disciplines.vams"
module requested_bound(p, n);
    inout p, n;
    electrical p, n;
    parameter real limit = 0.0;
    analog begin
        $bound_step(limit);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn zero_bound_requests_the_solver_minimum_and_invalid_bounds_fail_closed() {
    let mut device = compile_device("BZERO", PARAMETERIZED_BOUND);
    stamp_once(&mut device, &[0.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(0.0));

    assert!(device.try_set_parameter("limit", -1.0).unwrap());
    stamp_once(&mut device, &[0.0]);
    let error = device
        .try_transient_bound_step()
        .expect_err("negative $bound_step requests must fail closed");
    assert!(error.to_string().contains("$bound_step"), "{error}");
}

const DISCONTINUOUS: &str = r#"
`include "disciplines.vams"
module disco(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (V(p, n) > 1.0)
            $discontinuity(0);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#;

#[test]
fn discontinuity_reports_rising_edges_only() {
    let mut device = compile_device("D1", DISCONTINUOUS);

    // Below threshold: quiet
    stamp_once(&mut device, &[0.5]);
    assert!(!device.discontinuity_pending());
    assert!(!device.discontinuity_rising());
    device.advance_state();

    // Crossing: pending and rising
    stamp_once(&mut device, &[1.5]);
    assert!(device.discontinuity_pending());
    assert!(device.discontinuity_rising());
    device.advance_state();

    // Still above threshold: pending but no longer rising (a level-true
    // region must not pin tiny steps forever)
    stamp_once(&mut device, &[1.6]);
    assert!(device.discontinuity_pending());
    assert!(!device.discontinuity_rising());
    device.advance_state();

    // Dropping back re-arms the edge detector
    stamp_once(&mut device, &[0.4]);
    assert!(!device.discontinuity_pending());
    device.advance_state();
    stamp_once(&mut device, &[1.2]);
    assert!(device.discontinuity_rising());
}

#[test]
fn child_control_tasks_reach_the_solver_after_hierarchy_flattening() {
    let source = r#"
module leaf(p,n);
 inout p,n; electrical p,n;
 parameter real bound=1e-9;
 analog begin
  if(V(p,n)>0.0) begin $bound_step(bound); $discontinuity(0); end
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
module nested(p,n);
 inout p,n; electrical p,n;
 leaf #(.bound(2e-9)) inner(p,n);
endmodule
module top(p,n);
 inout p,n; electrical p,n;
 nested a(p,n);
 leaf #(.bound(1e-9)) b(p,n);
 analog $bound_step(3e-9);
endmodule
"#;
    let mut device = compile_selected_device("hierarchy", source, Some("top"));
    stamp_once(&mut device, &[1.0]);
    assert_eq!(
        device.try_transient_bound_step().unwrap(),
        Some(1e-9),
        "{:?}",
        device.variables().collect::<Vec<_>>()
    );
    assert!(device.discontinuity_pending());
    device.advance_state();
    stamp_once(&mut device, &[-1.0]);
    assert_eq!(device.try_transient_bound_step().unwrap(), Some(3e-9));
    assert!(!device.discontinuity_pending());
}

#[test]
fn negative_discontinuity_requests_newton_iteration_without_a_time_event() {
    let source = r#"
module limiting(p,n);
 inout p,n; electrical p,n;
 analog begin
  if(V(p,n)>0.5) $discontinuity(-1);
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
"#;
    let mut device = compile_device("LIMITING", source);
    stamp_once(&mut device, &[1.0]);
    assert!(
        !device.limiter_converged(),
        "an active -1 hint must prevent Newton convergence"
    );
    assert!(
        !device.discontinuity_pending(),
        "a Newton hint must not become a transient restart"
    );
    stamp_once(&mut device, &[0.0]);
    assert!(
        device.limiter_converged(),
        "the hint must reset on the next evaluation"
    );
    assert!(!device.discontinuity_pending());
}

#[test]
fn discontinuity_rejects_nonconstant_and_invalid_degrees() {
    for degree in ["V(p,n)", "$abstime", "-2", "0.5", "1.0/0.0"] {
        let source = format!(
            "module invalid(p,n); inout p,n; electrical p,n; analog begin $discontinuity({degree}); I(p,n)<+V(p,n); end endmodule"
        );
        let error = VerilogACompiler::default()
            .compile(&source)
            .expect_err("invalid discontinuity degree must be rejected");
        assert!(
            error.to_string().contains("$discontinuity"),
            "{degree}: {error}"
        );
    }
}

#[test]
fn discontinuity_parameter_degrees_and_overlapping_hints_are_checked() {
    let source = r#"
module hints(p,n);
 inout p,n; electrical p,n;
 parameter real degree=0.0;
 analog begin
  $discontinuity(degree);
  if(V(p,n)>0.5) $discontinuity(-1);
  I(p,n)<+V(p,n);
 end
endmodule
"#;
    let mut device = compile_device("HINTS", source);
    stamp_once(&mut device, &[1.0]);
    assert!(device.discontinuity_pending());
    assert!(!device.limiter_converged());
    device.try_set_parameter("degree", -1.0).unwrap();
    stamp_once(&mut device, &[0.0]);
    assert!(!device.discontinuity_pending());
    assert!(!device.limiter_converged());
    device.try_set_parameter("degree", 3.0).unwrap();
    stamp_once(&mut device, &[0.0]);
    assert!(device.discontinuity_pending());
    assert!(device.limiter_converged());
    device.try_set_parameter("degree", 0.5).unwrap();
    let mut matrix_entries = 0;
    let mut rhs_entries = 0;
    let error = device
        .try_stamp(
            &[0.0],
            |_, _, _| matrix_entries += 1,
            |_, _| rhs_entries += 1,
        )
        .expect_err("invalid overridden degree must fail evaluation");
    assert_eq!(
        (matrix_entries, rhs_entries),
        (0, 0),
        "a failed evaluation must not publish matrix entries"
    );
    assert!(error.to_string().contains("$discontinuity"), "{error}");
    let error = device
        .try_evaluate()
        .expect_err("a finite contribution must not hide an invalid degree");
    assert!(error.to_string().contains("$discontinuity"), "{error}");
}

#[test]
fn discontinuity_in_children_preserves_transient_and_newton_hints() {
    let source = r#"
module leaf(p,n);
 inout p,n; electrical p,n;
 parameter real degree=0;
 analog begin
  if(V(p,n)>0.0) $discontinuity(degree);
  I(p,n)<+V(p,n)*1e-3;
 end
endmodule
module top(p,n);
 inout p,n; electrical p,n;
 leaf #(.degree(2)) transient_hint(p,n);
 leaf #(.degree(-1)) newton_hint(p,n);
endmodule
"#;
    let mut device = compile_selected_device("hierarchical hints", source, Some("top"));
    for voltage in [1.0, -1.0, 1.0] {
        stamp_once(&mut device, &[voltage]);
        assert_eq!(device.discontinuity_rising(), voltage > 0.0);
        assert_eq!(device.limiter_converged(), voltage < 0.0);
        device.advance_state();
    }
}

#[test]
fn discontinuity_accepts_omitted_and_parameter_expression_degrees() {
    for degree in ["", "(abs(degree))", "(degree+1)", "(degree>0 ? degree : 1)"] {
        let source = format!(
            "module constants(p,n); inout p,n; electrical p,n; parameter real degree=1; analog begin $discontinuity{degree}; I(p,n)<+V(p,n); end endmodule"
        );
        let mut device = compile_device("constants", &source);
        stamp_once(&mut device, &[0.0]);
        assert!(device.discontinuity_pending());
        assert!(device.limiter_converged());
    }
}

#[test]
fn discontinuity_checkpoints_reject_old_semantics_and_invalid_flags() {
    let mut device = compile_device("CHECKPOINT", DISCONTINUOUS);
    stamp_once(&mut device, &[1.5]);
    device.advance_state();
    let accepted = device.checkpoint_state().unwrap();
    let mut old = accepted.clone();
    old.state_version = 8;
    assert!(
        device
            .validate_checkpoint_state(&old)
            .unwrap_err()
            .to_string()
            .contains("version")
    );
    let slot = device
        .variables()
        .position(|(name, _)| name == "$discontinuity")
        .unwrap();
    let mut invalid = accepted.clone();
    invalid.accepted.variables[slot] = 4.0;
    assert!(
        device
            .validate_checkpoint_state(&invalid)
            .unwrap_err()
            .to_string()
            .contains("$discontinuity")
    );
    assert_eq!(device.checkpoint_state().unwrap(), accepted);
}

#[cfg(feature = "native")]
#[test]
fn custom_limit_function_identifiers_do_not_select_builtin_names() {
    let source = r#"
module custom_name(p,n);
 inout p,n; electrical p,n;
 analog function real pnjlim;
  input proposed,previous,increment;
  real proposed,previous,increment;
  pnjlim=min(proposed,previous+increment);
 endfunction
 analog I(p,n)<+$limit(V(p,n),pnjlim,0.25);
endmodule
"#;
    let mut device = compile_device("NAME", source);
    device.update_voltages(&[0.0]);
    assert_eq!(device.try_evaluate().unwrap(), [0.0]);
    device.update_voltages(&[1.0]);
    assert_eq!(device.try_evaluate().unwrap(), [0.25]);
}

#[cfg(feature = "native")]
#[test]
fn custom_limit_callbacks_coerce_integer_formals_and_return_values() {
    for (signature, body, expected) in [
        (
            "real proposed; input integer previous,increment",
            "min(proposed,previous+0.25*increment)",
            [0.0, 0.5, 1.5, 1.75],
        ),
        (
            "real proposed,previous,increment",
            "proposed",
            [0.0, 2.0, 2.0, 2.0],
        ),
    ] {
        let return_type = if body == "proposed" {
            "integer"
        } else {
            "real"
        };
        let source = format!(
            r#"
module numeric_limit(p,n);
 inout p,n; electrical p,n;
 analog function {return_type} numeric;
  input {signature};
  numeric={body};
 endfunction
 analog I(p,n)<+$limit(V(p,n),numeric,1.6);
endmodule
"#
        );
        let mut device = compile_device("NUMERIC", &source);
        for (voltage, expected) in [0.0, 1.75, 1.75, 1.75].into_iter().zip(expected) {
            device.update_voltages(&[voltage]);
            assert_eq!(device.try_evaluate().unwrap(), [expected]);
        }
    }
}

#[test]
fn unknown_limit_string_selectors_use_the_default_algorithm() {
    let source = r#"
module fallback_limit(p,n);
 inout p,n; electrical p,n;
 analog I(p,n)<+$limit(V(p,n),"unavailable_vendor_algorithm",0.1);
endmodule
"#;
    let plain = source.replace(",\"unavailable_vendor_algorithm\",0.1", "");
    let mut fallback = compile_device("FALLBACK", source);
    let mut reference = compile_device("DEFAULT", &plain);
    for voltage in [0.0, 2.0, 2.0, -2.0, -2.0] {
        fallback.update_voltages(&[voltage]);
        reference.update_voltages(&[voltage]);
        assert_eq!(
            fallback.try_evaluate().unwrap(),
            reference.try_evaluate().unwrap()
        );
        assert_eq!(fallback.limiter_converged(), reference.limiter_converged());
    }
}

#[test]
fn default_limit_preserves_probe_history_and_reports_clipping() {
    use rspice_veriloga::vm::VerilogAEvaluationMode as EvaluationMode;

    let mut device = compile_device(
        "DEFAULT",
        "module bounded(p,n); inout p,n; electrical p,n;
         analog I(p,n)<+$limit(V(p,n),0.25); endmodule",
    );
    device.update_voltages(&[0.0]);
    assert_eq!(
        device
            .try_evaluate_with_mode(EvaluationMode::NewtonLimited)
            .unwrap(),
        vec![0.0]
    );
    device.update_voltages(&[1.0]);
    assert_eq!(
        device
            .try_evaluate_with_mode(EvaluationMode::NewtonLimited)
            .unwrap(),
        vec![0.25]
    );
    assert!(!device.limiter_converged());
    let mut reference = device.clone();
    for mode in [EvaluationMode::StaticProbe, EvaluationMode::SmallSignal] {
        device.update_voltages(&[2.0]);
        assert_eq!(device.try_evaluate_with_mode(mode).unwrap(), vec![2.0]);
        assert!(!device.limiter_converged());
    }
    device.update_voltages(&[1.0]);
    for expected in [0.5, 0.75, 1.0] {
        assert_eq!(
            device
                .try_evaluate_with_mode(EvaluationMode::NewtonLimited)
                .unwrap(),
            vec![expected]
        );
        assert_eq!(
            reference
                .try_evaluate_with_mode(EvaluationMode::NewtonLimited)
                .unwrap(),
            vec![expected]
        );
        assert_eq!(device.limiter_converged(), expected == 1.0);
    }
}

#[test]
fn limiter_values_and_jacobians_share_previous_newton_history() {
    use rspice_veriloga::vm::VerilogAEvaluationMode as Mode;
    for body in [
        "limited=$limit(V(p,n),0.25); I(p,n)<+limited*limited;",
        "limited=$limit(V(p,n),0.25)*$limit(V(p,n),0.25); I(p,n)<+limited;",
        "I(p,n)<+pow($limit(V(p,n),0.25),2);",
        #[cfg(feature = "native")]
        "I(p,n)<+pow($limit(V(p,n),clip),2);",
        #[cfg(feature = "native")]
        "I(p,n)<+pow($limit(V(n,p),\"clip\",\"typed\",-1.0),2);",
    ] {
        let source = format!(
            "module bounded(p,n); inout p,n; electrical p,n; real limited;
             analog function real clip; input real proposed,previous;
              clip=min(proposed,previous+0.25); endfunction
             analog begin {body} end endmodule"
        );
        for stamp in [false, true] {
            let mut device = compile_device("HISTORY", &source);
            device.update_voltages(&[0.0]);
            assert_eq!(
                device.try_evaluate_with_mode(Mode::NewtonLimited).unwrap(),
                [0.0]
            );
            for limited in [0.25_f64, 0.5, 0.75, 1.0] {
                device.update_voltages(&[1.0]);
                if stamp {
                    let mut matrix = 0.0;
                    let mut rhs = 0.0;
                    device
                        .try_stamp_with_mode(
                            &[1.0],
                            |_, _, value| matrix += value,
                            |_, value| rhs += value,
                            Mode::NewtonLimited,
                        )
                        .unwrap();
                    assert_eq!(matrix, 2.0 * limited, "stamped derivative: {body}");
                    assert_eq!(rhs, limited * limited, "stamped companion: {body}");
                } else {
                    assert_eq!(
                        device.try_evaluate_with_mode(Mode::NewtonLimited).unwrap(),
                        [limited * limited],
                        "value: {body}"
                    );
                }
                for _ in 0..2 {
                    let derivative: f64 = device
                        .try_compute_jacobian()
                        .unwrap()
                        .iter()
                        .filter(|entry| {
                            matches!(entry.row, rspice_veriloga::codegen::StampIndex::Terminal(0))
                                && matches!(
                                    entry.col,
                                    rspice_veriloga::codegen::StampIndex::Terminal(0)
                                )
                        })
                        .map(|entry| entry.value)
                        .sum();
                    assert_eq!(derivative, 2.0 * limited, "queried derivative: {body}");
                }
                assert_eq!(device.limiter_converged(), limited == 1.0);
            }
        }
    }
}

#[test]
fn limiter_checkpoint_restores_the_previous_newton_value() {
    use rspice_veriloga::vm::VerilogAEvaluationMode as Mode;
    let source = "module bounded(p,n); inout p,n; electrical p,n;
        analog I(p,n)<+$limit(V(p,n),0.25); endmodule";
    let mut original = compile_device("RESUME", source);
    for (proposed, expected) in [(2.0, 2.0), (4.0, 2.25)] {
        original.update_voltages(&[proposed]);
        assert_eq!(
            original
                .try_evaluate_with_mode(Mode::NewtonLimited)
                .unwrap(),
            [expected]
        );
    }
    // Probing must preserve both history and its classification for saving.
    assert_eq!(
        original.try_evaluate_with_mode(Mode::StaticProbe).unwrap(),
        [4.0]
    );
    let saved = original.checkpoint_state().unwrap();
    let decoded = rspice_veriloga::device::VerilogADeviceCheckpoint::from_words(
        saved.instance_name.clone(),
        saved.model_name.clone(),
        saved.source_digest.clone(),
        saved.shape_identity.clone(),
        &saved.to_words(),
    )
    .unwrap();
    let mut resumed = compile_device("RESUME", source);
    resumed.validate_checkpoint_state(&decoded).unwrap();
    resumed.apply_validated_checkpoint_state(&decoded);
    for expected in [2.5, 2.75, 3.0] {
        for device in [&mut original, &mut resumed] {
            device.update_voltages(&[4.0]);
            assert_eq!(
                device.try_evaluate_with_mode(Mode::NewtonLimited).unwrap(),
                [expected]
            );
        }
    }
}

#[test]
fn default_limit_recovers_after_an_invalid_step_without_changing_history() {
    use rspice_veriloga::vm::VerilogAEvaluationMode as Mode;
    let mut device = compile_device(
        "STEP",
        "module bounded(p,n); inout p,n; electrical p,n;
        parameter real maxstep=0.25; analog I(p,n)<+$limit(V(p,n),maxstep); endmodule",
    );
    device.update_voltages(&[0.0]);
    assert_eq!(
        device.try_evaluate_with_mode(Mode::NewtonLimited).unwrap(),
        vec![0.0]
    );
    device.update_voltages(&[1.0]);
    assert!(device.try_set_parameter("maxstep", -1.0).unwrap());
    assert!(device.try_evaluate_with_mode(Mode::NewtonLimited).is_err());
    assert_eq!(
        device.try_evaluate_with_mode(Mode::StaticProbe).unwrap(),
        vec![1.0]
    );
    assert!(device.try_set_parameter("maxstep", 0.25).unwrap());
    assert_eq!(
        device.try_evaluate_with_mode(Mode::NewtonLimited).unwrap(),
        vec![0.25]
    );
}

#[test]
fn default_limit_recommendations_preserve_argument_effects_and_guards() {
    let source = r#"
module fallback_effect(p,n);
 inout p,n; electrical p,n;
 real observed,value;
 analog function real touch;
  input real input_value;
  output real observed;
  begin observed=7; touch=input_value; end
 endfunction
 analog begin
  observed=0;
  value=V(p,n)>0?$limit(V(p,n),"unavailable",touch(2,observed)):0;
  I(p,n)<+observed;
 end
endmodule
"#;
    let mut device = compile_device("EFFECT", source);
    for (voltage, expected) in [(-1.0, 0.0), (1.0, 7.0), (-1.0, 0.0)] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), [expected]);
    }
}

#[test]
fn limiter_stamps_linearize_at_the_limited_value_once_per_iteration() {
    use rspice_veriloga::vm::VerilogAEvaluationMode as Mode;
    for (expression, derivative, rhs) in [("limited", 1.0, 0.0), ("limited*limited", 0.5, 0.0625)] {
        let source = format!(
            "module bounded(p,n); inout p,n; electrical p,n; real limited;
            analog begin limited=$limit(V(p,n),0.25); I(p,n)<+{expression}; end endmodule"
        );
        let mut device = compile_device("STAMP", &source);
        device
            .try_stamp_with_mode(&[0.0], |_, _, _| {}, |_, _| {}, Mode::NewtonLimited)
            .unwrap();
        let mut matrix = 0.0;
        let mut actual_rhs = 0.0;
        device
            .try_stamp_with_mode(
                &[1.0],
                |_, _, value| matrix += value,
                |_, value| actual_rhs += value,
                Mode::NewtonLimited,
            )
            .unwrap();
        assert_eq!(
            (matrix, actual_rhs),
            (derivative, rhs),
            "{expression}; {:?}",
            device.variables().collect::<Vec<_>>()
        );
        device.update_voltages(&[1.0]);
        let next = device.try_evaluate_with_mode(Mode::NewtonLimited).unwrap();
        assert_eq!(
            next,
            vec![if expression == "limited" { 0.5 } else { 0.25 }],
            "stamping must advance the limiter only once"
        );
    }
}

#[test]
fn limiter_rhs_correction_follows_the_executed_dataflow() {
    let cases = [
        ("I(p,n)<+$limit(V(p,n),0.25);", 1.0, 0.0),
        ("x=$limit(V(p,n),0.25); I(p,n)<+x*x;", 0.5, 0.0625),
        (
            "x=$limit(V(p,n),0.25); I(p,n)<+x*x; x=2*x; I(p,n)<+x*x;",
            2.5,
            0.3125,
        ),
        (
            "x=$limit(V(p,n),0.25); x=x*x; for(i=0;i<2;i=i+1) a[i]=x; I(p,n)<+a[0]+2*a[1];",
            1.5,
            0.1875,
        ),
        (
            "x=$limit(V(p,n),0.25); for(i=0;i<passes;i=i+1) a[i]=x*x; I(p,n)<+a[0]+2*a[1];",
            1.5,
            0.1875,
        ),
        (
            "x=pow($limit(V(p,n),0.25),3); I(p,n)<+ddx(x,V(p,n));",
            1.5,
            0.1875,
        ),
        (
            "x=$limit($limit(V(p,n),0.25),0.125); I(p,n)<+x*x;",
            0.25,
            0.015625,
        ),
        (
            "I(p,n)<+$limit(V(p,n),0.25)+$limit(2*V(p,n),0.125);",
            3.0,
            0.0,
        ),
        ("I(p,n)<+$limit(V(p,n)*V(p,n),0.25);", 2.0, 1.0),
        (
            "x=$limit(V(p,n),0.25); I(p,n)<+(V(p,n)>0 ? x*x : 0);",
            0.5,
            0.0625,
        ),
    ];
    for (body, expected_g, expected_rhs) in cases {
        let source = format!(
            "module correction(p,n); inout p,n; electrical p,n; parameter integer passes=2; real x,a[0:1]; integer i; analog begin {body} end endmodule"
        );
        let mut device = compile_device("CORRECTION", &source);
        device.try_stamp(&[0.0], |_, _, _| {}, |_, _| {}).unwrap();
        let mut g = 0.0;
        let mut rhs = 0.0;
        device
            .try_stamp(&[1.0], |_, _, v| g += v, |_, v| rhs += v)
            .unwrap();
        assert_eq!(
            (g, rhs),
            (expected_g, expected_rhs),
            "{body}; {:?}",
            device.variables().collect::<Vec<_>>()
        );
    }
}

#[test]
fn limiter_rhs_preserves_branch_signs_multiplicity_and_probe_modes() {
    use rspice_veriloga::vm::VerilogAEvaluationMode as Mode;
    for (contribution, row, limited_g, limited_rhs, physical_g, physical_rhs) in [
        ("I(p,n)<+x*x;", 0, 0.5, 0.0625, 2.0, 1.0),
        ("V(p,n)<+x*x;", 1, 0.5, -0.0625, -1.0, -1.0),
        ("V(p,n): V(p,n)==-x*x;", 1, 1.5, 0.0625, 3.0, 1.0),
    ] {
        for m in [1.0, 3.0] {
            let source = format!(
                "module orientation(p,n); inout p,n; electrical p,n; real x; analog begin x=$limit(V(p,n),0.25); {contribution} end endmodule"
            );
            let mut device = compile_device("ORIENTATION", &source);
            device.set_multiplicity(m);
            if row == 1 {
                device.set_branch_current_indices(&[2]);
            }
            device
                .try_stamp(&[0.0, 0.0], |_, _, _| {}, |_, _| {})
                .unwrap();
            for mode in [Mode::StaticProbe, Mode::SmallSignal, Mode::NewtonLimited] {
                let mut g = 0.0;
                let mut rhs = 0.0;
                device
                    .try_stamp_with_mode(
                        &[1.0, 0.0],
                        |r, c, v| {
                            if r == row && c == 0 {
                                g += v;
                            }
                        },
                        |r, v| {
                            if r == row {
                                rhs += v;
                            }
                        },
                        mode,
                    )
                    .unwrap();
                let expected = if mode == Mode::NewtonLimited {
                    (limited_g, limited_rhs)
                } else {
                    (physical_g, physical_rhs)
                };
                let scale = if row == 0 { m } else { 1.0 };
                assert_eq!(
                    (g, rhs),
                    (expected.0 * scale, expected.1 * scale),
                    "{contribution}; m={m}; {mode:?}"
                );
            }
        }
    }
}

#[test]
fn limiter_rhs_preserves_transient_companion_coefficients() {
    for expression in ["ddt(x*x)", "x*ddt(x)", "idt(x*x,0)"] {
        let source = format!(
            "module dynamic_correction(p,n); inout p,n; electrical p,n; real x; analog begin x=$limit(V(p,n),0.25); I(p,n)<+{expression}; end endmodule"
        );
        let mut device = compile_device("DYNAMIC", &source);
        device.set_analysis_type(2);
        device.set_timestep(1.0);
        device.try_stamp(&[0.0], |_, _, _| {}, |_, _| {}).unwrap();
        device.advance_state();
        let mut g = 0.0;
        let mut rhs = 0.0;
        device
            .try_stamp(&[1.0], |_, _, v| g += v, |_, v| rhs += v)
            .unwrap();
        assert_eq!((g, rhs), (0.5, 0.0625), "{expression}");
    }
}
