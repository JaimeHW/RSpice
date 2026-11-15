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
