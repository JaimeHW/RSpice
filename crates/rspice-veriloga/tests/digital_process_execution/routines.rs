//! Tasks and functions executed by the interpreter (IEEE 1364-2005 section 10,
//! design P2 §4.1). Expected values are what Icarus Verilog 13.0 computes for
//! the same source.

use super::*;
use rspice_veriloga::canonical_ir::digital_eval::{MAX_CALL_DEPTH, start_with_limit};

/// A task that suspends keeps its caller's frame: the suspension resumes
/// inside the task, and its outputs copy back only when it ends (section
/// 10.2.2). A static task's variables are signals; an automatic one's live in
/// the activation's frame.
#[test]
fn a_task_suspends_inside_its_body_and_copies_its_outputs_back_at_the_end() {
    let mut harness = Harness::from_source(
        "module top;\n\
         \x20 reg [7:0] a, b;\n\
         \x20 task t(input [7:0] v, output [7:0] o);\n\
         \x20   reg [7:0] x;\n\
         \x20   begin x = v + 1; o = x; #5 o = x + 1; end\n\
         \x20 endtask\n\
         \x20 task automatic u(input [7:0] v, output [7:0] o);\n\
         \x20   begin #1 o = v * 2; end\n\
         \x20 endtask\n\
         \x20 initial begin a = 0; t(8'd3, a); u(a, b); end\n\
         endmodule\n",
    );
    let suspension = expect_suspended(harness.run());
    assert!(matches!(suspension.wait(), DigitalWaitRequest::Delay(_)));
    let state = suspension.resume_state();
    let routine = state.routine().expect("suspended inside a task");
    assert_eq!(harness.plan.routines[routine.index()].name, "t");
    assert_eq!(state.callers().len(), 1, "the process frame waits below it");
    assert_eq!(
        harness.get("t.x"),
        "00000100",
        "a static variable is a signal"
    );
    assert_eq!(harness.get("a"), "00000000", "nothing copied back yet");

    let suspension = expect_suspended(harness.resume(0, state));
    let state = suspension.resume_state();
    let routine = state.routine().expect("suspended inside the second task");
    assert!(harness.plan.routines[routine.index()].automatic);
    assert_eq!(state.slots().len(), 2, "v and o live in the frame");
    assert_eq!(harness.get("a"), "00000101", "t's output copied back");

    expect_finished(harness.resume(0, state));
    assert_eq!(harness.get("b"), "00001010");
}

fn checkpoint_round_trip(
    plan: &CanonicalDigitalPlan,
    state: &DigitalResumeState,
    edit: impl FnOnce(&mut serde_json::Value),
) -> DigitalResumeState {
    use rspice_veriloga::canonical_ir::digital_eval::checkpoint::{
        DigitalCheckpointLimits, DigitalCheckpointReader, DigitalResumeCheckpoint,
    };
    let mut image = serde_json::to_value(DigitalResumeCheckpoint::capture(state)).unwrap();
    edit(&mut image);
    let image: DigitalResumeCheckpoint = serde_json::from_value(image).unwrap();
    DigitalCheckpointReader::new(plan, DigitalCheckpointLimits::default())
        .unwrap()
        .restore_resume(&image)
        .unwrap()
}

/// A continuation suspended two task frames deep, with an automatic frame's
/// slots, survives a checkpoint; a version 1 image of a process frame reads
/// as a frame with no routine.
#[test]
fn a_checkpoint_keeps_the_task_frames_of_a_suspended_process() {
    let mut harness = Harness::from_source(
        "module top;\n\
         \x20 integer r;\n\
         \x20 task automatic inner(input integer v, output integer o);\n\
         \x20   integer w;\n\
         \x20   begin w = v + 1; #2 o = w * 10; end\n\
         \x20 endtask\n\
         \x20 task outer(output integer o);\n\
         \x20   integer t;\n\
         \x20   begin inner(4, t); #1 o = t + 1; end\n\
         \x20 endtask\n\
         \x20 initial begin #1 outer(r); end\n\
         endmodule\n",
    );
    let first = expect_suspended(harness.run());
    let restored = checkpoint_round_trip(&harness.plan, first.resume_state(), |image| {
        image["header"]["version"] = serde_json::json!(1);
    });
    assert_eq!(&restored, first.resume_state(), "a version 1 process frame");
    let inside = expect_suspended(harness.resume(0, &restored));
    assert_eq!(inside.resume_state().callers().len(), 2);
    let restored = checkpoint_round_trip(&harness.plan, inside.resume_state(), |_| {});
    assert_eq!(&restored, inside.resume_state());
    let next = expect_suspended(harness.resume(0, &restored));
    expect_finished(harness.resume(0, next.resume_state()));
    assert_eq!(harness.get("r"), format!("{:032b}", 51));
}

/// Automatic functions recurse (section 10.4.1); a static function's local
/// keeps its value between calls; recursion without end is the named error
/// rather than a host stack overflow.
#[test]
fn functions_recurse_keep_static_state_and_bound_their_depth() {
    let mut harness = Harness::from_source(
        "module top;\n\
         \x20 integer f, c;\n\
         \x20 function automatic integer fact(input integer k);\n\
         \x20   if (k <= 1) fact = 1; else fact = k * fact(k - 1);\n\
         \x20 endfunction\n\
         \x20 function integer count(input integer step);\n\
         \x20   integer total;\n\
         \x20   begin if (step == 0) total = 0; else total = total + step; count = total; end\n\
         \x20 endfunction\n\
         \x20 initial begin f = fact(10); c = count(0); c = count(2); c = count(5); end\n\
         endmodule\n",
    );
    expect_finished(harness.run());
    assert_eq!(harness.get("f"), format!("{:032b}", 3_628_800));
    assert_eq!(harness.get("c"), format!("{:032b}", 7));

    let harness = Harness::from_source(
        "module top;\n\
         \x20 integer f;\n\
         \x20 function automatic integer down(input integer k);\n\
         \x20   down = down(k + 1);\n\
         \x20 endfunction\n\
         \x20 initial f = down(0);\n\
         endmodule\n",
    );
    let mut store = harness.store;
    let error = start_with_limit(
        &harness.plan,
        &harness.plan.processes[0],
        &mut store,
        4 * MAX_CALL_DEPTH,
    )
    .expect_err("unbounded recursion is refused");
    assert_eq!(error, DigitalEvalError::CallDepth);
}
