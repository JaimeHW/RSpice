// C10 c880 O3 template, variant c880-xs: Icarus runs net/c880.v (the vlog rendering)
// on the primary inputs and strobe the RSpice xs run converted; the outputs are
// dumped under the xs digital net names.
// rspice-o3-sources: ../../net/c880.v
// rspice-o3-inputs: d_g44 d_g57 d_g35 d_g21 d_g18 d_g23 d_g13 d_g53 d_g41 d_g37 d_g2 d_g31 d_g9 d_g17 d_g27 d_g60 d_g40 d_g14 d_g47 d_g36 d_g46 d_g19 d_g58 d_g8 d_g3 d_g26 d_g59 d_g54 d_g16 d_g11 d_g22 d_g32 d_g50 d_g30 d_g10 d_g45 d_g6 d_g1 d_g48 d_g28 d_g55 d_g42 d_g51 d_g25 d_g4 d_g15 d_g12 d_g29 d_g33 d_g7 d_g49 d_g24 d_g39 d_g56 d_g52 d_g34 d_g38 d_g20 d_g5 d_g43 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module tb;
  reg d_g44, d_g57, d_g35, d_g21, d_g18, d_g23, d_g13, d_g53, d_g41, d_g37, d_g2, d_g31, d_g9,
    d_g17, d_g27, d_g60, d_g40, d_g14, d_g47, d_g36, d_g46, d_g19, d_g58, d_g8, d_g3, d_g26, d_g59,
    d_g54, d_g16, d_g11, d_g22, d_g32, d_g50, d_g30, d_g10, d_g45, d_g6, d_g1, d_g48, d_g28, d_g55,
    d_g42, d_g51, d_g25, d_g4, d_g15, d_g12, d_g29, d_g33, d_g7, d_g49, d_g24, d_g39, d_g56, d_g52,
    d_g34, d_g38, d_g20, d_g5, d_g43, d_stb;
  wire d_g855, d_g856, d_g857, d_g858, d_g859, d_g860, d_g861, d_g862, d_g863, d_g864, d_g865,
    d_g866, d_g867, d_g868, d_g869, d_g870, d_g871, d_g872, d_g873, d_g874, d_g875, d_g876, d_g877,
    d_g878, d_g879, d_g880;
  c880 dut( d_g44, d_g57, d_g35, d_g21, d_g18, d_g23, d_g13, d_g53, d_g41, d_g37, d_g2, d_g31, d_g9,
    d_g17, d_g27, d_g60, d_g40, d_g14, d_g47, d_g36, d_g46, d_g19, d_g58, d_g8, d_g3, d_g26, d_g59,
    d_g54, d_g16, d_g11, d_g22, d_g32, d_g50, d_g30, d_g10, d_g45, d_g6, d_g1, d_g48, d_g28, d_g55,
    d_g42, d_g51, d_g25, d_g4, d_g15, d_g12, d_g29, d_g33, d_g7, d_g49, d_g24, d_g39, d_g56, d_g52,
    d_g34, d_g38, d_g20, d_g5, d_g43, d_g855, d_g856, d_g857, d_g858, d_g859, d_g860, d_g861,
    d_g862, d_g863, d_g864, d_g865, d_g866, d_g867, d_g868, d_g869, d_g870, d_g871, d_g872, d_g873,
    d_g874, d_g875, d_g876, d_g877, d_g878, d_g879, d_g880);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
