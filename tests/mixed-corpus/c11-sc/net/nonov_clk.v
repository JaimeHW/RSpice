// C11: two-phase non-overlapping clock at fs = 1 MHz (RSpice-authored).
// ph1 is high over [20, 480] ns and ph2 over [520, 980] ns of every 1 us
// period; both start low. The outputs reach the switch controls through the
// built-in D/A converters at vcc.
`timescale 1ns/1ps
module nonov_clk(ph1, ph2);
  output ph1, ph2;
  reg ph1, ph2;
  initial begin
    ph1 = 1'b0;
    ph2 = 1'b0;
  end
  always begin
    #20 ph1 = 1'b1;
    #460 ph1 = 1'b0;
    #40 ph2 = 1'b1;
    #460 ph2 = 1'b0;
    #20;
  end
endmodule
