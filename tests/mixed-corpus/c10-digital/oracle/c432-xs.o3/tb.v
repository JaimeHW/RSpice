// C10 c432 O3 template, variant c432-xs: Icarus runs net/c432.v (the vlog rendering)
// on the primary inputs and strobe the RSpice xs run converted; the outputs are
// dumped under the xs digital net names.
// rspice-o3-sources: ../../net/c432.v
// rspice-o3-inputs: d_g35 d_g21 d_g18 d_g23 d_g13 d_g2 d_g31 d_g9 d_g17 d_g27 d_g14 d_g36 d_g19 d_g8 d_g3 d_g26 d_g16 d_g11 d_g22 d_g32 d_g30 d_g10 d_g6 d_g1 d_g28 d_g25 d_g4 d_g15 d_g12 d_g29 d_g33 d_g7 d_g24 d_g34 d_g20 d_g5 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module tb;
  reg d_g35, d_g21, d_g18, d_g23, d_g13, d_g2, d_g31, d_g9, d_g17, d_g27, d_g14, d_g36, d_g19, d_g8,
    d_g3, d_g26, d_g16, d_g11, d_g22, d_g32, d_g30, d_g10, d_g6, d_g1, d_g28, d_g25, d_g4, d_g15,
    d_g12, d_g29, d_g33, d_g7, d_g24, d_g34, d_g20, d_g5, d_stb;
  wire d_g426, d_g427, d_g428, d_g429, d_g430, d_g431, d_g432;
  c432 dut( d_g35, d_g21, d_g18, d_g23, d_g13, d_g2, d_g31, d_g9, d_g17, d_g27, d_g14, d_g36, d_g19,
    d_g8, d_g3, d_g26, d_g16, d_g11, d_g22, d_g32, d_g30, d_g10, d_g6, d_g1, d_g28, d_g25, d_g4,
    d_g15, d_g12, d_g29, d_g33, d_g7, d_g24, d_g34, d_g20, d_g5, d_g426, d_g427, d_g428, d_g429,
    d_g430, d_g431, d_g432);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
