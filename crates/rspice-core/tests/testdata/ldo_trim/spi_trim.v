// C5 serially loaded trim register (RSpice-authored; corpus design C5).
// While csn is low each sclk rise shifts sdi in; csn's rise latches the word.
`timescale 1ns/1ps
module spi_trim(sclk, sdi, csn, tr3, tr2, tr1, tr0);
  input sclk, sdi, csn;
  output tr3, tr2, tr1, tr0;
  reg tr3, tr2, tr1, tr0;
  reg [3:0] sh;
  initial begin sh = 4'b0000; tr3 = 1'b0; tr2 = 1'b0; tr1 = 1'b0; tr0 = 1'b0; end
  always @(posedge sclk) if (!csn) sh <= {sh[2:0], sdi};
  always @(posedge csn) {tr3, tr2, tr1, tr0} <= sh;
endmodule
