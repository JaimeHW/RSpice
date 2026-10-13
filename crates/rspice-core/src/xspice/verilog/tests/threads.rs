//! Tasks, functions and procedural threads end to end (design P2 §4.1, §4.2).
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`).

use super::*;

/// Static tasks share one copy of their variables across concurrent enables
/// (IEEE 1364-2005 section 10.2.3); outputs and inouts copy back when the task
/// ends; a module-level loop variable keeps the task's last write; automatic
/// tasks and functions recurse; a static function's local keeps its value
/// between calls; a call is as wide and as signed as its result; a function
/// drives a continuous assignment.
#[test]
fn tasks_and_functions_run_as_icarus_runs_them() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg [7:0] a, b;
  reg [3:0] q;
  integer i, n;
  real rr;
  wire [7:0] w;
  reg [7:0] in;
  task t(input [7:0] v, output [7:0] o);
    reg [7:0] x;
    begin x = v; #10 o = x; end
  endtask
  task bump(inout [7:0] v);
    v = v + 1;
  endtask
  task twice(inout [7:0] v);
    begin bump(v); #1 bump(v); end
  endtask
  task send(input [3:0] value);
    for (i = 3; i >= 0; i = i - 1) begin
      #2 q[i] = value[i];
    end
  endtask
  task automatic rec(input integer k, output integer res);
    integer sub;
    begin
      if (k <= 1) res = 1;
      else begin #1 rec(k - 1, sub); res = k * sub; end
    end
  endtask
  function automatic integer fact(input integer k);
    if (k <= 1) fact = 1; else fact = k * fact(k - 1);
  endfunction
  function [7:0] counter(input [7:0] step);
    reg [7:0] total;
    begin
      if (step == 0) total = 0;
      else total = total + step;
      counter = total;
    end
  endfunction
  function signed [3:0] neg(input [3:0] v);
    neg = -v;
  endfunction
  function real half(input integer v);
    half = v / 2.0;
  endfunction
  function [7:0] add3(input [7:0] p, input [7:0] r, input [7:0] s);
    add3 = p + r + s;
  endfunction
  initial t(8'd1, a);
  initial #1 t(8'd2, b);
  assign w = add3(in, 8'd1, 8'd2);
  initial begin
    #12;
    $display("a=%0d b=%0d t=%0t", a, b, $time);
    a = 8'd10;
    twice(a);
    $display("twice=%0d t=%0t", a, $time);
    q = 4'b0000;
    send(4'b1011);
    $display("q=%b i=%0d t=%0t", q, i, $time);
    rec(4, n);
    $display("rec=%0d fact=%0d t=%0t", n, fact(5), $time);
    a = counter(0); a = counter(3); a = counter(4);
    $display("counter=%0d", a);
    $display("neg=%0d wide=%0d", neg(4'd3), neg(4'd3) + 8'd0);
    rr = half(7);
    $display("half=%f", rr);
    in = 8'd5;
    #1 $display("w=%0d", w);
    $display("nested=%0d", add3(add3(1, 2, 3), fact(3), counter(1)));
    $finish(0);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "a=2 b=2 t=12\n\
         twice=12 t=13\n\
         q=1011 i=-1 t=21\n\
         rec=24 fact=120 t=24\n\
         counter=7\n\
         neg=-3 wide=13\n\
         half=3.500000\n\
         w=8\n\
         nested=20\n"
    );
}

/// `-> event` wakes every `@event` waiter at once (section 9.7.3); `wait`
/// goes on at once when its condition is true and otherwise waits for it,
/// with `x` not true (section 9.7.6); `time` and `realtime` variables hold
/// the clock (section 3.9).
#[test]
fn events_and_wait_run_as_icarus_runs_them() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  event go, done;
  event kick;
  reg [3:0] count;
  reg ready;
  integer hits;
  time stamp;
  realtime rstamp;
  initial begin
    count = 0; hits = 0;
    #5 -> go;
    #5;
    #5 -> kick;
    @done $display("done t=%0t count=%0d hits=%0d stamp=%0t r=%0.1f", $time, count, hits, stamp, rstamp);
    $finish(0);
  end
  always @go begin
    count = count + 1;
    stamp = $time;
    rstamp = $realtime;
    $display("go t=%0t", $time);
  end
  initial begin
    wait (ready) $display("ready t=%0t", $time);
    wait (ready);
    $display("again t=%0t", $time);
    wait (count == 2) $display("two");
  end
  initial begin
    @(kick) hits = hits + 1;
    -> go;
    #1 -> done;
  end
  initial begin
    ready = 1'bx;
    #1 ready = 0;
    #9 ready = 1;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "go t=5\n\
         ready t=10\n\
         again t=10\n\
         go t=15\n\
         two\n\
         done t=16 count=2 hits=1 stamp=15 r=15.0\n"
    );
}

/// `fork ... join` (section 9.8.2): every branch starts at the fork, the
/// parent resumes once the last branch has ended, forks nest, a static task
/// forks and joins before it copies its output back, an empty fork joins at
/// once, a loop forks again into reused threads, a branch can wait on an
/// event, and a branch's `$finish` ends the run at its time step.
///
/// Icarus starts zero-delay branches last first (`vvp` pushes each forked
/// thread onto the front of the active queue), so it prints `second n=0`
/// before `first`. Design P2 §4.2 starts them in source order, which is the
/// one line here that differs from what Icarus printed.
#[test]
fn fork_and_join_run_as_icarus_runs_them() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg [7:0] a, b, c;
  integer n, k;
  reg clk;
  task par(input [7:0] x, output [7:0] y);
    reg [7:0] t;
    begin
      fork
        #2 t = x + 1;
        #1 y = x;
      join
      y = y + t;
    end
  endtask
  initial begin
    a = 0; b = 0; c = 0; n = 0;
    fork
      #3 begin a = 1; $display("a t=%0t", $time); end
      begin #1 b = 2; $display("b t=%0t", $time); #4 b = 3; end
      $display("c0 t=%0t a=%0d", $time, a);
      c = 7;
    join
    $display("join t=%0t a=%0d b=%0d c=%0d", $time, a, b, c);
    fork : named
      begin $display("first"); n = n + 1; end
      begin $display("second n=%0d", n); n = n + 10; end
      fork
        #2 $display("inner1 t=%0t", $time);
        #1 $display("inner2 t=%0t", $time);
      join
    join
    $display("named t=%0t n=%0d", $time, n);
    par(8'd4, c);
    $display("par c=%0d t=%0t", c, $time);
    fork join
    $display("empty t=%0t", $time);
    for (k = 0; k < 3; k = k + 1)
      fork
        #1 n = n + 1;
        #2 n = n + 1;
      join
    $display("loop n=%0d t=%0t", n, $time);
    clk = 0;
    #1 clk = 1;
    #1 clk = 0;
    #1 clk = 1;
    #5;
    fork
      #1 $display("finish t=%0t", $time);
      #1 $finish(0);
      #3 $display("never");
    join
    $display("never either");
  end
  always @(posedge clk) begin
    fork
      $display("edge t=%0t", $time);
      @(negedge clk) $display("neg t=%0t", $time);
    join
    $display("edge done t=%0t", $time);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "c0 t=0 a=0\n\
         b t=1\n\
         a t=3\n\
         join t=5 a=1 b=3 c=7\n\
         first\n\
         second n=1\n\
         inner2 t=6\n\
         inner1 t=7\n\
         named t=7 n=11\n\
         par c=9 t=9\n\
         empty t=9\n\
         loop n=17 t=15\n\
         edge t=16\n\
         neg t=17\n\
         edge done t=17\n\
         edge t=18\n\
         finish t=24\n"
    );
}

/// A task enable whose body reaches no timing control runs inline, in the
/// same uninterrupted execution as its caller: no process woken by the same
/// edge runs between the statement before the enable and the one after it.
/// A task that would wait, but is told not to, runs inline too.
///
/// Every writer adds 1, enables a task that adds 1, and adds 1 again, so a
/// reader woken by the same edge sees a multiple of 3 whichever of the two
/// runs first. Icarus does yield at every enable (`vvp` runs a task as a
/// forked thread and joins it), and printed `seen=4`, `seenw=1` and
/// `seenu=4`: section 11.4.2 lets a simulator suspend there, and RSpice
/// does not. The totals are what Icarus printed.
#[test]
fn a_task_that_does_not_wait_runs_inline() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg clk;
  integer v, seen_v, w, seen_w, u, seen_u;
  task bump; v = v + 1; endtask
  task bumpw; begin w = w + 1; end endtask
  task inner; u = u + 1; endtask
  task maybe(input d);
    begin
      if (d) @(negedge clk);
      inner;
    end
  endtask
  always @(posedge clk) begin v = v + 1; bump; v = v + 1; end
  always @(posedge clk) seen_v = v;
  always @(posedge clk) seen_w = w;
  always @(posedge clk) begin w = w + 1; bumpw; w = w + 1; end
  always @(posedge clk) begin u = u + 1; maybe(1'b0); u = u + 1; end
  always @(posedge clk) seen_u = u;
  initial begin
    clk = 0; v = 0; w = 0; u = 0;
    repeat (3) begin
      #5 clk = 1;
      #1 $display("%0d %0d %0d %0d %0d %0d", v, seen_v, w, seen_w, u, seen_u);
      #4 clk = 0;
    end
    $finish(0);
  end
endmodule
"#,
        &[],
    );
    let lines: Vec<Vec<i64>> = stdout(&report)
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect();
    assert_eq!(lines.len(), 3, "{lines:?}");
    for (edge, values) in lines.iter().enumerate() {
        let total = 3 * (edge as i64 + 1);
        assert_eq!([values[0], values[2], values[4]], [total; 3], "{lines:?}");
        for seen in [values[1], values[3], values[5]] {
            assert!(
                seen == total || seen == total - 3,
                "edge {edge}: a reader saw {seen} inside a writer's enable: {lines:?}"
            );
        }
    }
}

/// `disable` (section 10.3, design P2 §4.1): a named block disabled from
/// inside it is left at once (a loop break); one disabled from another
/// process while it waits is left where it waits; a task disabled while it
/// waits ends, and its enable goes on; a task that disables itself returns,
/// copying its output; disabling a named fork ends every branch and the
/// parent goes on after the join; a branch disabling the block around its
/// fork ends its siblings and the parent leaves the block; every thread
/// running a static task ends; a nonblocking update already scheduled still
/// lands; an `always` body disabled while it waits restarts at its event
/// control.
#[test]
fn disable_runs_as_icarus_runs_it() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg [7:0] a, b, c, r, q, n;
  reg clk;
  integer i, hits;
  task wait_t; begin #10 a = 9; end endtask
  task s(output [7:0] o); begin o = 8'd9; disable s; o = 8'd11; end endtask
  task shared; begin #4 b = b + 1; end endtask
  initial begin
    a = 0; b = 0; c = 0; r = 0; q = 0; n = 0; hits = 0;
    i = 0;
    begin : loop
      forever begin
        i = i + 1;
        if (i == 3) disable loop;
        c = c + 1;
      end
    end
    $display("loop i=%0d c=%0d", i, c);
    begin : blk
      #10 a = 1;
      #10 a = 2;
    end
    $display("blk a=%0d t=%0t", a, $time);
    wait_t;
    $display("task a=%0d t=%0t", a, $time);
    s(r);
    $display("self r=%0d", r);
    fork : f
      #2 n = n + 1;
      #3 disable f;
      #7 n = n + 10;
    join
    $display("fork n=%0d t=%0t", n, $time);
    begin : around
      fork
        #1 disable around;
        #5 n = n + 100;
      join
      n = n + 1000;
    end
    $display("around n=%0d t=%0t", n, $time);
    fork
      shared;
      #1 shared;
      #2 disable shared;
    join
    $display("shared b=%0d t=%0t", b, $time);
    begin : nb
      q <= #3 8'd5;
      #1 disable nb;
    end
    $display("nb q=%0d t=%0t", q, $time);
    #5 $display("nb later q=%0d t=%0t", q, $time);
    clk = 0;
    #1 clk = 1;
    #2 disable body;
    #1 clk = 0;
    #1 clk = 1;
    #1 clk = 0;
    #5 $display("always hits=%0d t=%0t", hits, $time);
    $finish(0);
  end
  initial begin
    #15 disable blk;
    #5 disable wait_t;
  end
  always @(posedge clk) begin : body
    hits = hits + 1;
    @(negedge clk) hits = hits + 10;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "loop i=3 c=2\n\
         blk a=1 t=15\n\
         task a=1 t=20\n\
         self r=9\n\
         fork n=1 t=23\n\
         around n=1 t=24\n\
         shared b=0 t=26\n\
         nb q=0 t=27\n\
         nb later q=5 t=32\n\
         always hits=12 t=43\n"
    );
}

/// A task disabled from outside copies nothing back, output or inout
/// (design P2 §4.1, where section 10.3 leaves the result unspecified).
/// Icarus does copy them and printed `r=5 s=4 t=5`.
#[test]
fn a_task_disabled_from_outside_copies_nothing_back() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg [7:0] r, s;
  task t(output [7:0] o); begin o = 8'd5; #10 o = 8'd7; end endtask
  task u(inout [7:0] v); begin v = v + 1; #10 v = v + 1; end endtask
  initial begin
    r = 0; s = 3;
    fork
      t(r);
      u(s);
      #5 begin disable t; disable u; end
    join
    $display("r=%0d s=%0d t=%0t", r, s, $time);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "r=0 s=3 t=5\n");
}

/// A parameter, a range and an initializer call functions of their own
/// module (IEEE 1364-2005 section 10.4.5): static and automatic, recursive,
/// through a named block and another function, real-valued, reading a
/// parameter and an instance override. The system task in `mask` is ignored
/// (rule d), as Icarus ignores it.
#[test]
fn constant_functions_run_as_icarus_runs_them() {
    let report = bench(
        r#"module child(w);
  output [31:0] w;
  parameter N = 5;
  function integer clog2(input integer v);
    integer k;
    begin
      k = v - 1;
      for (clog2 = 0; k > 0; clog2 = clog2 + 1) k = k >> 1;
    end
  endfunction
  localparam B = clog2(N);
  reg [B-1:0] r;
  assign w = B;
  initial begin r = ~0; #1 $display("child N=%0d B=%0d r=%b", N, B, r); end
endmodule

module top;
  parameter W = 4;
  function [7:0] mask(input integer n);
    begin mask = {W{1'b1}} << n; $display("inside mask"); end
  endfunction
  function automatic [15:0] fact(input [15:0] n);
    fact = (n > 1) ? fact(n - 1) * n : n;
  endfunction
  function integer twice(input integer a);
    begin : named twice = 2 * add1(a) - 2; end
  endfunction
  function integer add1(input integer a);
    add1 = a + 1;
  endfunction
  function real half(input real x);
    half = x / 2.0;
  endfunction
  localparam M = mask(1);
  localparam F5 = fact(5);
  localparam T = twice(21);
  localparam real H = half(5.0);
  localparam G = fact(3) + twice(add1(1));
  wire [31:0] w1, w2;
  child #(.N(17)) c1(w1);
  child c2(w2);
  reg [7:0] init = mask(2);
  initial begin
    #2 $display("M=%b F5=%0d T=%0d H=%f G=%0d init=%b w1=%0d w2=%0d", M, F5, T, H, G, init, w1, w2);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "child N=17 B=5 r=11111\n\
         child N=5 B=3 r=111\n\
         M=00011110 F5=120 T=42 H=2.500000 G=10 init=00111100 w1=5 w2=3\n"
    );
}

/// `j.set(v)` enables the task `set` of the instance `j`, with that
/// instance's storage and scope (IEEE 1364-2005 section 12.5).
#[test]
fn a_downward_task_name_enables_the_instance_task() {
    let report = bench(
        r#"`timescale 1ns/1ns
module jobs(out);
  output out;
  reg out;
  task set(input val);
    begin #1 out = val; $display("%m set %b at %0t", val, $time); end
  endtask
endmodule

module top;
  wire w1, w2;
  jobs j(w1);
  jobs k(w2);
  initial begin
    j.set(1'b1);
    k.set(1'b0);
    #1 $display("w1=%b w2=%b t=%0t", w1, w2, $time);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "top.j.set set 1 at 1\n\
         top.k.set set 0 at 2\n\
         w1=1 w2=0 t=3\n"
    );
}

/// A task name that reaches another root module is refused by what it is,
/// never by the symbol the parser stood in for it.
#[test]
fn a_cross_module_task_name_is_refused_in_source_terms() {
    let error = super::run_digital_testbench(
        &[(
            "tb.v".into(),
            "module main;
  initial other.passed;
endmodule
module other;
  task passed; $display(\"PASSED\"); endtask
endmodule
"
            .into(),
        )],
        &super::HdlTestbenchOptions::default(),
    )
    .expect_err("a cross-module task name is not supported yet");
    let text = error.to_string();
    assert!(
        text.contains("an upward or cross-module task name is not supported yet"),
        "{text}"
    );
    assert!(!text.contains("rspice_reference"), "{text}");
}
