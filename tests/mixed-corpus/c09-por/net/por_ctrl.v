// C9 power-on reset controller (RSpice-authored; corpus design C9).
// por_n sets on a rise of hi and clears on a fall of lo_n. An internal 1 MHz
// clock (rising at 0.5 us + k us) is exported on clk. rst_n rises on the 17th
// clock rise after por_n (the next clock plus 16 us) and clears on the first
// clock rise that sees por_n low. tie_hi is 1 from its initial block.
`timescale 1ns/1ps
module por_ctrl(hi, lo_n, por_n, rst_n, tie_hi, clk);
  input hi, lo_n;
  output por_n, rst_n, tie_hi, clk;
  reg por_n, rst_n, tie_hi, clk;
  reg [4:0] cnt;
  initial begin
    por_n = 1'b0; rst_n = 1'b0; tie_hi = 1'b1; clk = 1'b0; cnt = 5'd0;
  end
  always #500 clk = ~clk;
  always @(posedge hi) por_n = 1'b1;
  always @(negedge lo_n) por_n = 1'b0;
  always @(posedge clk) begin
    if (!por_n) begin
      cnt <= 5'd0;
      rst_n <= 1'b0;
    end else if (cnt != 5'd16) begin
      cnt <= cnt + 5'd1;
    end else begin
      rst_n <= 1'b1;
    end
  end
endmodule
