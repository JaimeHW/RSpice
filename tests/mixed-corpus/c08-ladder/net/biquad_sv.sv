// C8 rung (d): the wreal biquad of biquad_wr.vams written with a
// SystemVerilog real-number-modelling nettype (RSpice-authored). The
// SV-RNM subset arrives at P5; today the source is refused.
`timescale 1ns/1ps
package c8_rnm;
  nettype real rnet;
endpackage

module biquad_sv(x, y);
  import c8_rnm::*;
  input rnet x;
  output rnet y;
  real x1, x2, y0, y1, y2;
  assign y = y0;
  initial begin
    x1 = 0.0;
    x2 = 0.0;
    y0 = 0.0;
    y1 = 0.0;
    y2 = 0.0;
    #500;
    forever begin
      y0 = 0.0009440841143955489 * (x + 2.0 * x1 + x2)
         + 1.9112262303409135 * y1 - 0.9150025667984957 * y2;
      x2 = x1;
      x1 = x;
      y2 = y1;
      y1 = y0;
      #1000;
    end
  end
endmodule
