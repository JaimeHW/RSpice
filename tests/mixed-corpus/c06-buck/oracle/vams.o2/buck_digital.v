// C6 O2: the digital half of net/buck_ctrl_ams.vams for d_cosim (RSpice-authored).
// The logic of ../vdig.o2/buck_digital.v with the vams sample instant: the
// analog block samples V(fb) on a 2 us timer at each period start (tick 0)
// and the process reads that sample at tick 1, so this half latches the
// 10-bit code the oracle deck quantizes from V(fb) (round(V·512), clipped to
// 0..1023) at tick 0 (the previous period's tick 64) and uses it at tick 1.
// The first period's sample is taken at t = 0, where V(fb) = 0: code 0. The
// over-voltage flag is left out: it sets above 2.2 V and V(fb) peaks at
// 1.995 V on the load step's recovery, so it never changes the duty.
`timescale 1ps/1ps
module buck_digital(code, hs, ls);
  input [9:0] code;
  output hs, ls;
  reg hs, ls;
  parameter integer TICK = 31250;
  parameter integer RAMP = 16;
  parameter integer TARGET = 922;
  integer ref, c, s, acc, duty, d, dfr, carry;
  initial begin
    hs = 1'b0;
    ls = 1'b0;
    ref = 0;
    c = 0;
    s = 0;
    acc = 0;
    duty = 0;
    d = 1;
    dfr = 0;
    carry = 0;
  end
  always begin
    #(TICK);
    c = s;
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
    s = code;
  end
endmodule
