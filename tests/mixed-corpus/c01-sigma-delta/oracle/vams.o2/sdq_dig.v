// C1 vams O2: the digital half of net/sdq.vams for ngspice d_cosim (Icarus).
// The comparator is the deck's BComp + adc_bridge; this is the flip-flop.
`timescale 1ns/1ps
module sdq_dig(cmp, clk, dq, dqb);
  input cmp, clk; output dq, dqb; reg dq, dqb;
  initial begin dq = 1'b0; dqb = 1'b1; end            // d_dff ic=0
  always @(posedge clk) begin dq <= #1 cmp; dqb <= #1 !cmp; end
endmodule
