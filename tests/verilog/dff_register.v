// A 4-bit register: `always @(posedge clk)` with an asynchronous active-high
// reset and a synchronous load enable.
//
// This is the smallest design that requires a real event scheduler. The three
// mechanisms it forces are edge detection on a net that the harness toggles
// from outside the module, an asynchronous reset in the same sensitivity list
// as the clock (so the block must run on either edge and decide which fired),
// and non-blocking assignment to a vector.
//
// `q_delayed` is a second register clocked from the same edge and fed by `q`.
// Because both assignments are non-blocking, `q_delayed` must observe the
// *previous* value of `q`, never the one being written on this same edge. A
// scheduler that collapses the two updates into one pass produces
// `q_delayed == q` and the divergence shows up at a port.

module dff_register (clk, rst, en, d, q, q_delayed);
  parameter WIDTH = 4;

  input                  clk;
  input                  rst;
  input                  en;
  input      [WIDTH-1:0] d;
  output reg [WIDTH-1:0] q;
  output reg [WIDTH-1:0] q_delayed;

  always @(posedge clk or posedge rst) begin
    if (rst) begin
      q         <= {WIDTH{1'b0}};
      q_delayed <= {WIDTH{1'b0}};
    end else begin
      q_delayed <= q;
      if (en) begin
        q <= d;
      end
    end
  end
endmodule
