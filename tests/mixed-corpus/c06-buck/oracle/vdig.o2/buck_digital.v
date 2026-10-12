// C6 O2: the digital half of net/buck_ctrl.vams for d_cosim (RSpice-authored).
// Identical logic; the sample at tick 1 reads the 10-bit code the oracle deck
// quantizes from V(fb) (round(V·512), clipped to 0..1023) instead of V(fb).
`timescale 1ps/1ps
module buck_digital(code, hs, ls);
  input [9:0] code;
  output hs, ls;
  reg hs, ls;
  parameter integer TICK = 31250;
  parameter integer RAMP = 16;
  parameter integer TARGET = 922;
  integer ref, c, acc, duty, d, dfr, carry;
  initial begin
    hs = 1'b0;
    ls = 1'b0;
    ref = 0;
    c = 0;
    acc = 0;
    duty = 0;
    d = 1;
    dfr = 0;
    carry = 0;
  end
  always begin
    #(TICK);
    c = code;
    ref = ref + RAMP;
    if (ref > TARGET) ref = TARGET;
    acc = acc + ref - c;
    if (acc < 0) acc = 0;
    if (acc > 65535) acc = 65535;
    duty = acc / 64;
    dfr = dfr + duty % 16;
    carry = 0;
    if (dfr >= 16) begin
      dfr = dfr - 16;
      carry = 1;
    end
    d = duty / 16 + carry;
    if (d < 1) d = 1;
    if (d > 58) d = 58;
    #(TICK) hs = 1'b1;
    #(d * TICK) hs = 1'b0;
    #(2 * TICK) ls = 1'b1;
    #((60 - d) * TICK) ls = 1'b0;
  end
endmodule
