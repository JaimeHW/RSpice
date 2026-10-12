// C4 8-bit successive-approximation register (RSpice-authored; corpus design C4).
// One conversion is ten clocks. Clock 0 (start high, idle): samp falls and the
// MSB trial is set. Clocks 1..8 decide b7..b0 from cmp (1 keeps the trial bit)
// and set the next trial bit. Clock 9 raises eoc and samp; the bits hold the
// code until the next conversion's clock 0.
`timescale 1ns/1ps
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
