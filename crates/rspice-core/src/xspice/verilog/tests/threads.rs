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
