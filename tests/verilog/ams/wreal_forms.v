// Verilog-AMS LRM 2.4 section 3.7's real net, in the forms a real-number model
// is built out of.
//
// One design rather than several so that the interactions are covered too: a
// real net feeds an algebraic block, a comparison on it drives four-state bits,
// and a process woken by the value-change event of section 3.7 counts how often
// its input moved.
//
// Nothing here converts between the two value domains. Section 3.7 makes that
// conversion an explicit `$realtobits`/`$bitstoreal`, and every bridge in this
// design is the one it leaves open: a four-state value used as a *condition*
// choosing between two reals.
module wreal_forms(vin, code, vout, vscaled, over, moves);
  input wreal vin;
  input [3:0] code;
  output wreal vout;
  output wreal vscaled;
  output over;
  output [3:0] moves;

  wire [3:0] code;
  reg over;
  reg [3:0] moves;

  // Table 4-2's arithmetic on real operands, in a continuous assignment: the
  // driver evaluates from the start of simulation (IEEE 1364-2005 section 6.1)
  // and again on every change of an operand.
  wreal step3, step2, step1, step0;
  assign step3 = code[3] ? 8.0 : 0.0;
  assign step2 = code[2] ? 4.0 : 0.0;
  assign step1 = code[1] ? 2.0 : 0.0;
  assign step0 = code[0] ? 1.0 : 0.0;
  assign vout = (step3 + step2) + (step1 + step0);

  // A real input mapped algebraically, and the conditional operator table 4-2
  // makes legal in a real expression, saturating the result.
  wreal raw;
  assign raw = vin * 0.5 + 0.25;
  assign vscaled = (raw > 2.0) ? 2.0 : raw;

  // A comparison on a real yields section 5.4.2 rule (g)'s one unsigned bit,
  // which drives an ordinary `reg`.
  always @(vscaled) if (vscaled >= 1.0) over = 1'b1; else over = 1'b0;

  // Section 3.7's event is a change of value. `moves` counts how often `vin`
  // moved, which is what distinguishes a wakeup rule from a sampling one.
  initial moves = 4'b0000;
  always @(vin) moves = moves + 4'b0001;
endmodule
