//! Hierarchical references to discrete-domain signals and events, downward
//! and upward (IEEE 1364-2005 sections 12.5 and 12.6).
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`), an RSpice-authored probe.

use super::*;

/// A downward read, write and event wait two levels down, and an upward read
/// by the scope's top-level module name.
#[test]
fn hierarchical_names_read_write_and_wait_on_another_scope() {
    let report = bench(
        r#"`timescale 1ns/1ns
module leaf(d, q);
  input d; output q; reg q; reg [3:0] count; event done;
  initial count = 0;
  always @(d) begin q = ~d; count = count + 1; -> done; end
  initial #14 $display("%0t %m sees top.flag=%b", $time, top.flag);
endmodule
module mid(d, q);
  input d; output q; wire q;
  leaf l(.d(d), .q(q));
endmodule
module top;
  reg d, flag; wire q;
  mid m(.d(d), .q(q));
  initial begin
    flag = 1'b1; #1 d = 0;
    #5 d = 1;
    #5 d = 0;
    #1 $display("%0t count=%0d q=%b leafq=%b", $time, m.l.count, q, m.l.q);
    m.l.count = 4'd9;
    #1 $display("%0t count=%0d", $time, m.l.count);
  end
  always @(m.l.done) $display("%0t done d=%b", $time, m.l.d);
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "1 done d=0\n\
         6 done d=1\n\
         11 done d=0\n\
         12 count=3 q=1 leafq=1\n\
         13 count=9\n\
         14 top.m.l sees top.flag=1\n"
    );
}

/// A continuous assignment and a function call reaching down, and an event
/// control reaching up.
#[test]
fn a_continuous_assignment_and_a_function_reach_down_and_an_event_up() {
    let report = bench(
        r#"`timescale 1ns/1ns
module leaf;
  reg [7:0] r;
  function [7:0] twice(input [7:0] v); twice = v * 2; endfunction
  initial r = 8'd5;
  always @(top.go) $display("%0t %m woke on top.go, r=%0d", $time, r);
endmodule
module top;
  event go;
  wire [7:0] w = u.r + 8'd1;
  leaf u();
  initial begin
    #1 $display("%0t w=%0d twice=%0d", $time, w, u.twice(8'd21));
    #1 -> go;
    #1 u.r = 8'd40;
    #1 $display("%0t w=%0d", $time, w);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "1 w=6 twice=42\n\
         2 top.u woke on top.go, r=5\n\
         4 w=41\n"
    );
}

#[test]
fn a_scope_named_through_an_escaped_module_name_is_the_child() {
    // `\a.u ` is named `a.u`; inside it, `u` is the child instance, not the
    // module with the prefix taken off. Icarus prints the child's time scale.
    let printed = both_orders(
        "`timescale 1us/1ns
         module top;
           \\a.u  m();
         endmodule
         `timescale 10us/1ns
         module \\a.u ;
           child u();
           initial begin
             $printtimescale(u);
             $printtimescale;
           end
         endmodule
         `timescale 1ns/1ps
         module child;
           reg x;
         endmodule\n",
    );
    assert_eq!(
        printed,
        "Time scale of (top.m.u) is 1ns / 1ps\nTime scale of (top.m) is 10us / 1ns\n"
    );
}

#[test]
fn a_reference_into_a_named_block_reaches_only_what_the_block_declares() {
    // Icarus rejects the name the block does not declare (`Could not find
    // variable ``m.b.y'' in ``m''`) and runs the one it does.
    let undeclared = super::run_digital_testbench(
        &[(
            "tb.v".into(),
            "module m;
  initial begin : b
    reg x;
    x = 0;
  end
  initial #1 m.b.y = 1;
endmodule
"
            .into(),
        )],
        &super::HdlTestbenchOptions::default(),
    )
    .expect_err("an undeclared name in a named block does not run")
    .to_string();
    assert!(
        undeclared.contains("`y` is not declared in named block `b` of module `m`"),
        "{undeclared}"
    );
    assert!(!undeclared.contains("Unsupported"), "{undeclared}");
    let report = bench(
        "module m;
  initial begin : b
    reg x;
    x = 0;
  end
  initial #1 begin m.b.x = 1; $display(\"x=%b\", m.b.x); end
endmodule
",
        &[],
    );
    assert_eq!(stdout(&report), "x=1\n");
}

/// A task's own variable read from another module, a generate block's
/// functions called by their block's name, a named block's variable read
/// through the module's name, and an instance named as a scope argument.
#[test]
fn references_reach_task_items_generate_routines_and_instance_scopes() {
    let report = bench(
        r#"`timescale 1ns/1ns
module leaf;
  reg [3:0] hits;
  task bump;
    integer calls;
    begin calls = calls + 1; hits = hits + 1; end
  endtask
  initial begin bump.calls = 0; hits = 0; end
endmodule
module top;
  genvar i;
  generate for (i = 0; i < 2; i = i + 1) begin : g
    function [7:0] scaled(input [7:0] v); scaled = v * (i + 2); endfunction
    reg [7:0] r;
  end endgenerate
  leaf u();
  initial begin : outer
    reg [7:0] seen;
    #1 u.bump; u.bump; seen = 8'd7;
    $display("%0t calls=%0d hits=%0d", $time, u.bump.calls, u.hits);
    $display("%0t g0=%0d g1=%0d", $time, g[0].scaled(8'd5), g[1].scaled(8'd5));
    g[1].r = top.outer.seen + 1;
    $display("%0t r=%0d", $time, g[1].r);
    $printtimescale(top.u);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "1 calls=2 hits=2\n\
         1 g0=10 g1=15\n\
         1 r=8\n\
         Time scale of (top.u) is 1ns / 1ns\n"
    );
}

/// Names that start at another top-level module read, write and wait on its
/// signals, a continuous assignment's net and an instance's variable among
/// them, and name an instance in it as a scope argument.
#[test]
fn names_into_another_top_level_module_reach_its_signals_and_scopes() {
    let report = bench(
        r#"`timescale 1ns/1ns
module a;
  integer n;
  wire [7:0] w;
  reg [7:0] r;
  assign w = r + 1;
  initial begin n = 5; r = 8'd3; #2 $display("a: n=%0d r=%0d", n, r); end
  sub u();
endmodule
module sub;
  reg [3:0] deep;
  initial deep = 4'd9;
endmodule
module b;
  initial begin
    #1 $display("b: a.n=%0d a.w=%0d a.u.deep=%0d", a.n, a.w, a.u.deep);
    a.n = a.n + 10; a.r = 8'd40; a.u.deep = 4'd2;
    $printtimescale(a.u);
    #2 $display("b: %0d %0d %0d", a.n, a.w, a.u.deep);
  end
  always @(a.r) $display("%0t b sees a.r=%0d", $time, a.r);
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 b sees a.r=3\n\
         b: a.n=5 a.w=4 a.u.deep=9\n\
         Time scale of (a.u) is 1ns / 1ns\n\
         1 b sees a.r=40\n\
         a: n=15 r=40\n\
         b: 15 41 2\n"
    );
}

/// Calls into another top-level module run its own tasks and functions, in
/// its scope and storage (IEEE 1364-2005 sections 10.2.2, 10.4.3 and 12.5):
/// a static task's arguments and side effects, a function reading that
/// module's variable, automatic routines with concurrent activations, the
/// tasks of an instance and a generate block in it, and a call from an
/// instance of this root.
#[test]
fn calls_into_another_top_level_module_run_its_routines_in_its_scope() {
    let report = bench(
        r#"`timescale 1ns/1ns
module lib;
  integer calls;
  reg [7:0] last;
  initial calls = 0;
  task bump(input [7:0] by, output [7:0] total);
    begin
      calls = calls + 1;
      last = by;
      #2 total = by + calls;
      $display("%0t %m by=%0d calls=%0d", $time, by, calls);
    end
  endtask
  function [7:0] twice(input [7:0] v);
    twice = 2 * v + calls;
  endfunction
  function automatic integer fact(input integer n);
    fact = n <= 1 ? 1 : n * fact(n - 1);
  endfunction
  task automatic hold(input integer n, output integer seen);
    begin
      #n seen = n;
      $display("%0t %m n=%0d", $time, n);
    end
  endtask
  sub u();
  generate if (1) begin : gb
    task gt(output [3:0] o);
      o = 4'd9;
    endtask
  end endgenerate
endmodule
module sub;
  reg [3:0] state;
  initial state = 4'd1;
  task step;
    state = state + 1;
  endtask
endmodule
module user;
  reg [7:0] t;
  integer s1, s2;
  initial begin
    lib.bump(8'd5, t);
    $display("%0t t=%0d calls=%0d last=%0d twice=%0d fact=%0d", $time, t, lib.calls,
             lib.last, lib.twice(8'd3), lib.fact(5));
    fork
      lib.hold(3, s1);
      lib.hold(1, s2);
    join
    $display("%0t s1=%0d s2=%0d", $time, s1, s2);
    lib.u.step;
    lib.u.step;
    lib.gb.gt(t);
    $display("%0t state=%0d gt=%0d", $time, lib.u.state, t);
  end
  probe p();
endmodule
module probe;
  initial #1 $display("%0t probe twice=%0d", $time, lib.twice(8'd10));
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "1 probe twice=21\n\
         2 lib.bump by=5 calls=1\n\
         2 t=6 calls=1 last=5 twice=7 fact=120\n\
         3 lib.hold n=1\n\
         5 lib.hold n=3\n\
         5 s1=3 s2=1\n\
         5 state=3 gt=9\n"
    );
}

/// A top-level module that declares only routines is still a scope the
/// others call into, here from a task an `always` enables, which suspends
/// through the call.
#[test]
fn a_top_level_module_of_routines_alone_is_called_into() {
    let report = bench(
        r#"`timescale 1ns/100ps
module top;
  integer n;
  task step; begin z.wait1(n); n = n + 1; end endtask
  initial n = 0;
  always step;
  initial #4.5 begin $display("%0t n=%0d %0d", $time, n, z.f(n)); $finish; end
endmodule
module z;
  task automatic wait1(input integer k); #1 $display("%t %m k=%0d", $realtime, k); endtask
  function integer f(input integer v); f = v * 10; endfunction
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "                  10 z.wait1 k=0\n\
         \x20                 20 z.wait1 k=1\n\
         \x20                 30 z.wait1 k=2\n\
         \x20                 40 z.wait1 k=3\n\
         50 n=4 40\n"
    );
}

/// `disable` of a hierarchical name is refused in source terms, never by the
/// symbol the parser stood in for it.
#[test]
fn a_hierarchical_disable_is_refused_in_source_terms() {
    let error = super::run_digital_testbench(
        &[(
            "tb.v".into(),
            "module top;
  initial begin
    fork
      lib.t;
      #3 disable lib.t;
    join
  end
endmodule
module lib;
  task t; #5; endtask
endmodule
"
            .into(),
        )],
        &super::HdlTestbenchOptions::default(),
    )
    .expect_err("a hierarchical disable is not supported yet");
    let text = error.to_string();
    assert!(
        text.contains("`disable` of a hierarchical name (IEEE 1364-2005 section 10.3)"),
        "{text}"
    );
    assert!(!text.contains("rspice_reference"), "{text}");
}

/// Alone, a module that calls into another top-level module has nothing to
/// link the call to, so it is refused rather than run with a routine that
/// does nothing.
#[test]
fn a_module_compiled_alone_refuses_a_call_into_another_top_level_module() {
    let Err(error) = super::CompiledDigitalDesign::compile(
        "module lib;
  task t; $display(\"t\"); endtask
endmodule
module user;
  initial lib.t;
endmodule
",
        Some("user"),
    ) else {
        panic!("the call has no routine to run");
    };
    let error = error.to_string();
    assert!(
        error.contains("`lib.t`") && error.contains("only a discrete testbench links"),
        "{error}"
    );
}

/// An instance that declares nothing is still a scope, and another top-level
/// module is named by its name alone.
#[test]
fn scope_arguments_name_empty_instances_and_other_top_level_modules() {
    let report = bench(
        "`timescale 1us/1ns
module top;
  initial begin
    $printtimescale(mid);
    $printtimescale(mid.leaf);
    $printtimescale(spare);
    $printtimescale(spare.inner);
    $display(\"%m done\");
  end
  shell mid();
endmodule
`timescale 10ns/1ns
module shell;
  hollow leaf();
endmodule
`timescale 1ns/1ps
module hollow;
endmodule
`timescale 1ms/1us
module spare;
  hollow inner();
endmodule
",
        &[],
    );
    assert_eq!(
        stdout(&report),
        "Time scale of (top.mid) is 10ns / 1ns\n\
         Time scale of (top.mid.leaf) is 1ns / 1ps\n\
         Time scale of (spare) is 1ms / 1us\n\
         Time scale of (spare.inner) is 1ns / 1ps\n\
         top done\n"
    );
}

/// What the standard rules out is a source error naming its clause. Icarus
/// rejects the constant function too; it prints a time scale for a variable,
/// which IEEE 1364-2005 section 17.4.2 does not give one.
#[test]
fn a_constant_function_or_a_time_scale_of_a_variable_is_a_source_error() {
    let run = |source: &str| {
        super::run_digital_testbench(
            &[("tb.v".into(), source.into())],
            &super::HdlTestbenchOptions::default(),
        )
        .expect_err("the source is in error")
        .to_string()
    };
    let constant = run("module m1;
  function integer f1(input integer i); f1 = i + 1; endfunction
endmodule
module test;
  function integer f2(input integer i); f2 = m1.f1(i); endfunction
  localparam p = f2(1);
  initial $display(p);
endmodule
");
    assert!(
        constant.contains("constant function `f2` contains a hierarchical reference")
            && constant.contains("section 10.4.5"),
        "{constant}"
    );
    let variable = run("module top;
  reg r;
  initial $printtimescale(top.r);
endmodule
");
    assert!(variable.contains("section 17.4.2"), "{variable}");
    assert!(!variable.contains("Unsupported"), "{variable}");
}
