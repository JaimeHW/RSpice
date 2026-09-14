//! Source-level task execution must agree across portable and native builds.

use rspice_veriloga_runtime::{AnalogTaskArgument, AnalogTaskInvocation, AnalogTaskKind};
mod support;
use support::DeviceFixture;

fn fixture(body: &str) -> DeviceFixture {
    DeviceFixture::compile(&format!(
        r#"`include "disciplines.vams"
module tasks(p,n);
inout p,n; electrical p,n;
integer i;
analog begin {body} I(p,n) <+ V(p,n); end
endmodule"#
    ))
}

fn levels(calls: &[AnalogTaskInvocation]) -> Vec<i64> {
    calls
        .iter()
        .map(|call| {
            assert_eq!(call.kind, AnalogTaskKind::Finish);
            let [AnalogTaskArgument::Integer(value)] = &*call.arguments else {
                panic!("integer diagnostic level");
            };
            *value
        })
        .collect()
}

#[test]
fn task_argument_functions_keep_authored_names_and_accepted_call_order() {
    let fixture = DeviceFixture::compile(include_str!("fixtures/task_argument_functions.va"));
    let mut device = fixture.device("FUNCTION_TASKS", &[1, 0]);
    for voltage in [1.0, 0.5] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![voltage + 0.25]);
        fixture.observe(&mut device);
        device.try_compute_jacobian().unwrap();
        fixture.observe(&mut device);
        assert_eq!(device.variable("__fn1_identity__x"), Some(0.25));
        assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    }
    device.try_advance_state().unwrap();
    let calls = device.drain_accepted_analog_tasks().collect::<Vec<_>>();
    assert_eq!(levels(&calls), vec![0, 1, 2]);
    assert_eq!(calls[0].site, calls[1].site);
    assert_ne!(calls[0].site, calls[2].site);
}

#[test]
fn task_only_models_keep_identical_calls_on_every_loop_trip() {
    let fixture = DeviceFixture::compile(
        "module task_only(p,n); inout p,n; electrical p,n; integer i;
         parameter integer repetitions=3;
         analog for(i=0;i<repetitions;i=i+1) begin $finish(1); $finish(1); end endmodule",
    );
    let mut device = fixture.device("TASK_ONLY", &[1, 0]);
    assert!(device.try_evaluate().unwrap().is_empty());
    fixture.observe(&mut device);
    device.try_compute_jacobian().unwrap();
    device.try_advance_state().unwrap();
    let calls = device.drain_accepted_analog_tasks().collect::<Vec<_>>();
    assert_eq!(levels(&calls), vec![1; 6]);
    assert_ne!(calls[0].site, calls[1].site);
    assert!(
        calls
            .chunks_exact(2)
            .all(|pair| { pair[0].site == calls[0].site && pair[1].site == calls[1].site })
    );
}

#[test]
fn task_guards_skip_singular_arguments_and_replace_failed_candidates() {
    let fixture = fixture("$finish(1); if(V(p,n)!=0) $finish(1/V(p,n));");
    let mut device = fixture.device("GUARDED_TASK", &[1, 0]);
    for (voltage, expected) in [(0.0, vec![1]), (1.0, vec![1, 1])] {
        device.update_voltages(&[voltage]);
        device.try_evaluate().unwrap();
        device.try_advance_state().unwrap();
        assert_eq!(
            levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
            expected
        );
    }
    device.update_voltages(&[0.01]);
    assert!(device.try_evaluate().is_err());
    assert!(device.try_advance_state().is_err());
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    device.update_voltages(&[0.0]);
    device.try_evaluate().unwrap();
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![1]
    );
}

#[test]
fn canonical_task_arguments_preserve_loop_calls_source_values_and_rollback() {
    let fixture = DeviceFixture::compile(include_str!("fixtures/canonical_task_arguments.va"));
    let mut device = fixture.device("LOOP_TASKS", &[1, 0]);
    device.try_set_analysis_step(true, false).unwrap();
    for voltage in [-0.5_f64, 0.75] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![voltage]);
        fixture.observe(&mut device);
        device.try_compute_jacobian().unwrap();
        fixture.observe(&mut device);
        assert_eq!(device.variable("before"), Some(0.0));
        assert_eq!(device.variable("after"), Some(1.0));
        assert!((device.variable("reported").unwrap() - voltage.exp()).abs() < 1e-12);
        assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    }
    device.try_advance_state().unwrap();
    let calls = device.drain_accepted_analog_tasks().collect::<Vec<_>>();
    assert_eq!(levels(&calls), vec![2, 0, 1, 2]);
    assert!(calls[1..].iter().all(|call| call.site == calls[1].site));
    assert_ne!(calls[0].site, calls[1].site);
    device.try_set_analysis_step(false, false).unwrap();
    let accepted = device.checkpoint_state().unwrap();
    for voltage in [-1.0, 0.25] {
        device.validate_checkpoint_state(&accepted).unwrap();
        device.apply_validated_checkpoint_state(&accepted);
        device.try_set_analysis_step(false, false).unwrap();
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![2.0 * voltage]);
        fixture.observe(&mut device);
        assert_eq!(device.variable("count"), Some(1.0));
        device.try_advance_state().unwrap();
        assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    }
}

#[test]
fn event_task_arguments_capture_higher_order_source_values() {
    let fixture = DeviceFixture::compile(
        "module task_derivative_argument(p,n); inout p,n; electrical p,n;
         real count=0,reported;
         analog begin reported=ddx(ddx(ddx(exp(V(p,n)),V(p,n)),V(p,n)),V(p,n));
         @(initial_step) begin count=count+1; $finish(reported); end
         I(p,n)<+count*V(p,n); end endmodule",
    );
    let mut device = fixture.device("DERIVATIVE_ARGUMENT", &[1, 0]);
    #[cfg(feature = "native")]
    assert!(device.is_using_native());
    device.try_set_analysis_step(true, false).unwrap();
    for voltage in [-0.5_f64, 0.75] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![voltage]);
        fixture.observe(&mut device);
        assert!((device.variable("reported").unwrap() - voltage.exp()).abs() < 1e-12);
        assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    }
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![2]
    );
}

#[test]
fn event_higher_order_readback_keeps_task_capture_and_acceptance_order() {
    let fixture = DeviceFixture::compile(
        "module event_task_derivative(p,n); inout p,n; electrical p,n;
         real count=0,reported;
         analog begin @(initial_step) begin count=count+1; $finish(count); end
         reported=ddx(ddx(ddx(exp(V(p,n)),V(p,n)),V(p,n)),V(p,n));
         I(p,n)<+count*V(p,n); end endmodule",
    );
    let mut device = fixture.device("EVENT_DERIVATIVE_TASK", &[1, 0]);
    device.try_set_analysis_step(true, false).unwrap();
    for voltage in [-0.5_f64, 0.75] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![voltage]);
        fixture.observe(&mut device);
        device.try_compute_jacobian().unwrap();
        fixture.observe(&mut device);
        assert!((device.variable("reported").unwrap() - voltage.exp()).abs() < 1e-12);
        assert_eq!(device.variable("count"), Some(1.0));
        assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    }
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![1]
    );
    device.try_set_analysis_step(false, false).unwrap();
    device.update_voltages(&[-0.25]);
    assert_eq!(device.try_evaluate().unwrap(), vec![-0.25]);
    fixture.observe(&mut device);
    assert!((device.variable("reported").unwrap() - (-0.25_f64).exp()).abs() < 1e-12);
    device.try_advance_state().unwrap();
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
}

#[test]
fn event_readback_and_tasks_publish_one_accepted_source_ordered_effect() {
    let fixture = DeviceFixture::compile(
        "module event_tasks(p,n); inout p,n; electrical p,n; real count=0;
         analog begin @(initial_step) begin count=count+1; $finish(count); end
         I(p,n)<+count*V(p,n); end endmodule",
    );
    let mut device = fixture.device("EVENT_TASK", &[1, 0]);
    device.try_set_analysis_step(true, false).unwrap();
    for voltage in [-0.5, 0.75] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![voltage]);
        fixture.observe(&mut device);
        device.try_compute_jacobian().unwrap();
        fixture.observe(&mut device);
        assert_eq!(device.variable("count"), Some(1.0));
        assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    }
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![1]
    );
    device.try_set_analysis_step(false, false).unwrap();
    device.try_evaluate().unwrap();
    fixture.observe(&mut device);
    device.try_advance_state().unwrap();
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
}

#[test]
fn finish_publishes_only_the_last_candidate_and_captures_each_argument() {
    let fixture = fixture("i=0; while(i < 3) begin $finish(i); i=i+1; end");
    let mut device = fixture.device("X", &[1, 0]);
    device.try_set_time(1.0).unwrap();
    device.update_voltages(&[2.0]);
    device.try_evaluate().unwrap();
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    device.try_set_time(2.0).unwrap();
    device.try_evaluate().unwrap();
    device.try_advance_state().unwrap();
    let calls: Vec<_> = device.drain_accepted_analog_tasks().collect();
    assert_eq!(levels(&calls), vec![0, 1, 2]);
    assert!(calls.iter().all(|call| call.time == 2.0));
    assert!(calls.iter().all(|call| call.site == calls[0].site));
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
}

#[test]
fn inactive_calls_do_not_evaluate_invalid_arguments() {
    let fixture = fixture("if(V(p,n)>0) $finish(99);");
    let mut device = fixture.device("X", &[1, 0]);
    device.update_voltages(&[-1.0]);
    device.try_evaluate().unwrap();
    device.try_advance_state().unwrap();
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
    device.update_voltages(&[1.0]);
    assert!(device.try_evaluate().is_err());
    assert!(device.try_advance_state().is_err());
}

#[test]
fn jacobian_and_variable_observation_preserve_the_candidate_without_replaying_tasks() {
    let fixture = fixture("$finish(1);");
    let mut device = fixture.device("X", &[1, 0]);
    device.update_voltages(&[2.0]);
    device.try_evaluate().unwrap();
    fixture.observe(&mut device);
    device.try_compute_jacobian().unwrap();
    fixture.observe(&mut device);
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![1]
    );
}

#[test]
fn tasks_in_functions_and_transitive_wrappers_are_preserved() {
    let fixture = DeviceFixture::compile(
        r#"`include "disciplines.vams"
module tasks(p,n);
inout p,n; electrical p,n;
analog function real inner;
input level; real level;
begin $finish(level); inner=level; end
endfunction
analog function real outer;
input level; real level;
begin outer=inner(level); end
endfunction
analog I(p,n) <+ outer(2.0);
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![2.0]);
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![2]
    );
}

#[test]
fn a_failed_task_cannot_publish_an_earlier_call() {
    let fixture = fixture("$finish(1); $finish(99);");
    let mut device = fixture.device("X", &[1, 0]);
    assert!(device.try_evaluate().is_err());
    assert!(device.try_advance_state().is_err());
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
}

#[test]
fn conditional_expression_executes_only_the_selected_function_call() {
    let fixture = DeviceFixture::compile(
        r#"`include "disciplines.vams"
module tasks(p,n);
inout p,n; electrical p,n;
analog function real call_finish;
input level; real level;
begin $finish(level); call_finish=level; end
endfunction
analog I(p,n) <+ (V(p,n)>0 ? call_finish(1) : call_finish(2));
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    for (voltage, expected) in [(1.0, 1), (-1.0, 2)] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![expected as f64]);
        device.try_advance_state().unwrap();
        assert_eq!(
            levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
            vec![expected]
        );
    }
}

#[test]
fn a_branch_cannot_change_the_guard_of_its_own_else_arm() {
    let fixture = fixture("i=1; if(i) begin i=0; $finish(1); end else $finish(2);");
    let mut device = fixture.device("X", &[1, 0]);
    device.try_evaluate().unwrap();
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![1]
    );
}

#[test]
fn a_numerical_failure_after_a_task_invalidates_its_candidate() {
    let fixture = fixture("$finish; I(p,n) <+ 1.0 / V(p,n);");
    let mut device = fixture.device("X", &[1, 0]);
    device.update_voltages(&[0.0]);
    assert!(device.try_evaluate().is_err());
    assert!(device.try_advance_state().is_err());
    assert_eq!(device.drain_accepted_analog_tasks().count(), 0);
}

#[test]
fn conditional_output_arguments_update_the_caller_only_in_the_selected_arm() {
    let fixture = DeviceFixture::compile(
        r#"`include "disciplines.vams"
module tasks(p,n);
inout p,n; electrical p,n;
real y,z;
analog function real set_pair;
output yout; input xin; real yout,xin;
begin yout=xin+2.0; set_pair=yout+1.0; end
endfunction
analog begin
y=10.0;
z=V(p,n)>0 ? set_pair(y,3.0) : 0.0;
I(p,n) <+ y+z;
end
endmodule"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    for (voltage, expected) in [(1.0, 11.0), (-1.0, 10.0)] {
        device.update_voltages(&[voltage]);
        assert_eq!(device.try_evaluate().unwrap(), vec![expected]);
    }
}

#[test]
fn noise_probes_preserve_pending_calls_without_executing_them_again() {
    let fixture = fixture("$finish(1); I(p,n) <+ white_noise(1.0,\"n\");");
    let mut device = fixture.device("X", &[1, 0]);
    device.try_set_analysis_type(3).unwrap();
    device.update_voltages(&[2.0]);
    device.try_evaluate().unwrap();
    let noise = device
        .try_noise_processes_at_frequency(&[2.0], 1.0)
        .unwrap();
    assert_eq!(noise.len(), 1);
    device.try_advance_state().unwrap();
    assert_eq!(
        levels(&device.drain_accepted_analog_tasks().collect::<Vec<_>>()),
        vec![1]
    );
}

#[test]
fn canonical_validation_rejects_task_metadata_that_disagrees_with_execution() {
    use rspice_veriloga::canonical_ir::hir::HirRegion;
    let fixture = fixture("$finish(1);");
    let mut artifact = fixture.canonical_ir;
    let task = artifact
        .hir
        .body
        .iter_mut()
        .find_map(|region| {
            if let HirRegion::Task(task) = region {
                Some(task)
            } else {
                None
            }
        })
        .unwrap();
    task.initialization = !task.initialization;
    let diagnostics = artifact
        .hir
        .validate()
        .expect_err("mismatched task metadata must be rejected");
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("analog task region disagrees"))
    );
}
