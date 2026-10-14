// C4 variant split (RSpice-authored; corpus design C4): the testbench of
// net/sar8_full.v in a file of its own. It has no `timescale: the deck reads
// net/sar8.v first, and the HDL sources of a deck are one compilation unit, so
// tb_sar inherits sar8.v's 1ns/1ps (IEEE 1364-2005 section 19.8) and the run
// warns that it does.
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
