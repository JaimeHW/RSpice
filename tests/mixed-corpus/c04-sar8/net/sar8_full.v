// C4 variant full (RSpice-authored; corpus design C4): the testbench and the
// SAR register in one source. tb_sar makes the 10 MHz clock (rising at
// 50 ns + k 100 ns) and holds start high, so the register converts back to
// back (conversion k from k us + 50 ns, eoc at k us + 950 ns); at every eoc it
// prints the conversion index and the code, and it finishes after NCONV
// conversions. sar8 is net/sar8.v unchanged.
`timescale 1ns/1ps
module tb_sar(clk, start, eoc, b7, b6, b5, b4, b3, b2, b1, b0);
  output clk, start;
  input eoc, b7, b6, b5, b4, b3, b2, b1, b0;
  reg clk, start;
  parameter integer NCONV = 32;
  integer n;
  initial begin clk = 1'b0; start = 1'b1; n = 0; end
  always #50 clk = ~clk;
  always @(posedge eoc) begin
    $display("%0d %0d", n, {b7, b6, b5, b4, b3, b2, b1, b0});
    n = n + 1;
    if (n == NCONV) $finish;
  end
endmodule

module sar8(clk, start, cmp, samp, eoc, b7, b6, b5, b4, b3, b2, b1, b0);
  input clk, start, cmp;
  output samp, eoc, b7, b6, b5, b4, b3, b2, b1, b0;
  reg samp, eoc, b7, b6, b5, b4, b3, b2, b1, b0;
  reg [3:0] step;
  initial begin
    samp = 1'b1; eoc = 1'b0; step = 4'd0;
    b7 = 1'b0; b6 = 1'b0; b5 = 1'b0; b4 = 1'b0;
    b3 = 1'b0; b2 = 1'b0; b1 = 1'b0; b0 = 1'b0;
  end
  always @(posedge clk) begin
    case (step)
      4'd0: if (start) begin
        samp <= 1'b0; eoc <= 1'b0;
        b7 <= 1'b1; b6 <= 1'b0; b5 <= 1'b0; b4 <= 1'b0;
        b3 <= 1'b0; b2 <= 1'b0; b1 <= 1'b0; b0 <= 1'b0;
        step <= 4'd8;
      end
      4'd8: begin b7 <= cmp; b6 <= 1'b1; step <= 4'd7; end
      4'd7: begin b6 <= cmp; b5 <= 1'b1; step <= 4'd6; end
      4'd6: begin b5 <= cmp; b4 <= 1'b1; step <= 4'd5; end
      4'd5: begin b4 <= cmp; b3 <= 1'b1; step <= 4'd4; end
      4'd4: begin b3 <= cmp; b2 <= 1'b1; step <= 4'd3; end
      4'd3: begin b2 <= cmp; b1 <= 1'b1; step <= 4'd2; end
      4'd2: begin b1 <= cmp; b0 <= 1'b1; step <= 4'd1; end
      4'd1: begin b0 <= cmp; step <= 4'd9; end
      4'd9: begin eoc <= 1'b1; samp <= 1'b1; step <= 4'd0; end
      default: step <= 4'd0;
    endcase
  end
endmodule
