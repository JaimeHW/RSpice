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

/// `fork` hands the kernel one start state per branch, in source order, and
/// the state that resumes at the join (section 9.8.2, design P2 §4.2). A
/// branch runs in the forking task's frame and ends at its own end; the join
/// keeps the process frame below the task.
#[test]
fn a_fork_hands_the_kernel_its_branches_in_source_order_and_its_join() {
    let mut harness = Harness::from_source(
        "module top;\n\
         \x20 reg [7:0] a, b, c;\n\
         \x20 task t; fork a = 1; b = 2; join endtask\n\
         \x20 initial begin a = 0; b = 0; t; c = a + b; end\n\
         endmodule\n",
    );
    let DigitalProcessOutcome::Forked(fork) = harness.run() else {
        panic!("expected the process to fork");
    };
    let [first, second] = fork.branches() else {
        panic!("one start state per branch");
    };
    assert_eq!(
        fork.join().callers().len(),
        1,
        "the process frame waits below"
    );
    for branch in [first, second] {
        let routine = branch.routine().expect("a branch runs in the task's frame");
        assert_eq!(harness.plan.routines[routine.index()].name, "t");
        assert!(
            branch.callers().is_empty(),
            "a branch's end ends its thread"
        );
    }
    expect_finished(harness.resume(0, first));
    assert_eq!(
        (harness.get("a"), harness.get("b")),
        ("00000001".into(), "00000000".into())
    );
    expect_finished(harness.resume(0, second));
    assert_eq!(harness.get("b"), "00000010");
    expect_finished(harness.resume(0, fork.join()));
    assert_eq!(harness.get("c"), "00000011");
}

/// A `disable` of a named block another process waits in names the block's
/// scope. The waiting thread unwinds to where it stands and, resumed, leaves
/// the block at once, skipping the rest of it; the disabling thread is not
/// inside the block and goes on (section 10.3, design P2 §4.1).
#[test]
fn a_disabled_block_is_left_from_where_its_thread_waits() {
    use rspice_veriloga::canonical_ir::cfg::DigitalDisableTarget;
    use rspice_veriloga::canonical_ir::digital_eval::DigitalUnwind;
    let mut harness = Harness::from_source(
        "module top;\n\
         \x20 reg [7:0] a, b;\n\
         \x20 initial begin begin : blk a = 1; #10 a = 2; end b = 3; end\n\
         \x20 initial #5 disable blk;\n\
         endmodule\n",
    );
    let waiting = expect_suspended(harness.start(0));
    let delay = expect_suspended(harness.start(1));
    let DigitalProcessOutcome::Disabling(disable) = harness.resume(1, delay.resume_state()) else {
        panic!("expected the second process to disable");
    };
    let DigitalDisableTarget::Block(scope) = disable.target() else {
        panic!("`blk` is a named block");
    };
    assert_eq!(harness.plan.scopes[scope.index()].path, "top.blk");
    assert_eq!(
        disable
            .resume_state()
            .unwind(&harness.plan, disable.target(), true),
        DigitalUnwind::Untouched
    );
    let DigitalUnwind::Resume(unwound) =
        waiting
            .resume_state()
            .unwind(&harness.plan, disable.target(), false)
    else {
        panic!("the waiting process is inside the block");
    };
    assert_eq!(unwound.unwinding(), Some(scope));
    expect_finished(harness.resume(0, &unwound));
    assert_eq!(
        (harness.get("a"), harness.get("b")),
        ("00000001".into(), "00000011".into())
    );
    expect_finished(harness.resume(1, disable.resume_state()));
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

/// A parameter and a range call functions of their module at elaboration
/// (section 10.4.5): a static one with a loop and an automatic recursive one
/// that reads the first one's result. A constant function that reads a
/// signal is refused for that, as Icarus refuses it.
#[test]
fn constant_functions_fold_parameters_and_ranges() {
    let mut harness = Harness::from_source(
        "module top;\n\
         \x20 parameter N = 37;\n\
         \x20 function integer clog2(input integer v);\n\
         \x20   integer k;\n\
         \x20   begin k = v - 1; for (clog2 = 0; k > 0; clog2 = clog2 + 1) k = k >> 1; end\n\
         \x20 endfunction\n\
         \x20 function automatic integer fact(input integer n);\n\
         \x20   fact = (n > 1) ? n * fact(n - 1) : 1;\n\
         \x20 endfunction\n\
         \x20 localparam B = clog2(N);\n\
         \x20 localparam F = fact(B);\n\
         \x20 reg [B-1:0] r;\n\
         \x20 integer f;\n\
         \x20 initial begin r = ~0; f = F; end\n\
         endmodule\n",
    );
    expect_finished(harness.run());
    assert_eq!(harness.get("r"), "111111");
    assert_eq!(harness.get("f"), format!("{:032b}", 720));

    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(
            "module top;\n\
             \x20 reg [3:0] s;\n\
             \x20 function integer peek(input integer v);\n\
             \x20   peek = v + s;\n\
             \x20 endfunction\n\
             \x20 localparam P = peek(1);\n\
             \x20 integer q;\n\
             \x20 initial q = P;\n\
             endmodule\n",
        )
        .expect_err("a constant function may not read a signal")
        .to_string();
    assert!(error.contains("parameter `P`"), "{error}");
    assert!(!error.contains("Internal error"), "{error}");
}
