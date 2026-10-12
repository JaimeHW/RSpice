// C10 16-bit Fibonacci LFSR (RSpice-authored; corpus design C10): taps 16, 14,
// 13, 11 (x^16 + x^14 + x^13 + x^11 + 1, shifting right), seed 16'hACE1, an
// internal 10 MHz clock rising at 50 ns + k 100 ns and exported on clk. The
// state drives q15..q0, MSB first.
`timescale 1ns/1ps
module lfsr16(clk, q15, q14, q13, q12, q11, q10, q9, q8, q7, q6, q5, q4, q3, q2, q1, q0);
  output clk, q15, q14, q13, q12, q11, q10, q9, q8, q7, q6, q5, q4, q3, q2, q1, q0;
  reg clk;
  reg [15:0] s;
  assign {q15, q14, q13, q12, q11, q10, q9, q8, q7, q6, q5, q4, q3, q2, q1, q0} = s;
  initial begin clk = 1'b0; s = 16'hACE1; end
  always #50 clk = ~clk;
  always @(posedge clk) s <= {s[0] ^ s[2] ^ s[3] ^ s[5], s[15:1]};
endmodule
