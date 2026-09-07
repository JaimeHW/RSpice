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
