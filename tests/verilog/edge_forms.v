// Both clock edges, and a register before its first write.
//
// Every other clocked case in this corpus is triggered on `posedge` alone.
// `negedge` is a separate term in the sensitivity grammar and a separate
// classification in IEEE 1364-2005 table 5-2, and this is the only case that
// takes one.
//
// The falling-edge process reads `rise`, never a primary input. That is
// deliberate: the harness applies each vector at a multiple of the step, which
// is exactly when the clock falls, so a falling-edge process reading a driven
// input would be racing the stimulus. Reading a register the rising edge wrote
// half a period earlier has no such race, and the two processes then form a
// half-cycle pipeline: `fall` trails `rise` by one observation.
//
// Before the first falling edge `fall` has never been written, so it reads x
// -- the initial value section 4.2 gives a `reg`. That is also true if a
// simulator classifies the clock's arrival at 0 as a falling edge and runs the
// process then, because `rise` is x at that instant too, so the two readings
// agree and the case does not depend on which one a simulator does. Icarus
// only, for the x: Verilator is two-state by design.

module edge_forms (clk, d, rise, fall);
  input            clk;
  input      [3:0] d;
  output reg [3:0] rise;
  output reg [3:0] fall;

  always @(posedge clk) rise <= d;
  always @(negedge clk) fall <= rise;
endmodule
