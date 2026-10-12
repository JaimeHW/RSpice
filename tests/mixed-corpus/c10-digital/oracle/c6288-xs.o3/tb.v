// C10 c6288 O3 template, variant c6288-xs: Icarus runs net/c6288.v (the vlog rendering)
// on the primary inputs and strobe the RSpice xs run converted; the outputs are
// dumped under the xs digital net names.
// rspice-o3-sources: ../../net/c6288.v
// rspice-o3-inputs: d_g21 d_g18 d_g30 d_g5 d_g14 d_g26 d_g15 d_g7 d_g17 d_g27 d_g22 d_g4 d_g23 d_g13 d_g19 d_g6 d_g1 d_g28 d_g32 d_g12 d_g24 d_g9 d_g3 d_g10 d_g31 d_g11 d_g29 d_g20 d_g2 d_g8 d_g16 d_g25 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module tb;
  reg d_g21, d_g18, d_g30, d_g5, d_g14, d_g26, d_g15, d_g7, d_g17, d_g27, d_g22, d_g4, d_g23, d_g13,
    d_g19, d_g6, d_g1, d_g28, d_g32, d_g12, d_g24, d_g9, d_g3, d_g10, d_g31, d_g11, d_g29, d_g20,
    d_g2, d_g8, d_g16, d_g25, d_stb;
  wire d_g6257, d_g6258, d_g6259, d_g6260, d_g6261, d_g6262, d_g6263, d_g6264, d_g6265, d_g6266,
    d_g6267, d_g6268, d_g6269, d_g6270, d_g6271, d_g6272, d_g6273, d_g6274, d_g6275, d_g6276,
    d_g6277, d_g6278, d_g6279, d_g6280, d_g6281, d_g6282, d_g6283, d_g6284, d_g6285, d_g6286,
    d_g6287, d_g6288;
  c6288 dut( d_g21, d_g18, d_g30, d_g5, d_g14, d_g26, d_g15, d_g7, d_g17, d_g27, d_g22, d_g4, d_g23,
    d_g13, d_g19, d_g6, d_g1, d_g28, d_g32, d_g12, d_g24, d_g9, d_g3, d_g10, d_g31, d_g11, d_g29,
    d_g20, d_g2, d_g8, d_g16, d_g25, d_g6257, d_g6258, d_g6259, d_g6260, d_g6261, d_g6262, d_g6263,
    d_g6264, d_g6265, d_g6266, d_g6267, d_g6268, d_g6269, d_g6270, d_g6271, d_g6272, d_g6273,
    d_g6274, d_g6275, d_g6276, d_g6277, d_g6278, d_g6279, d_g6280, d_g6281, d_g6282, d_g6283,
    d_g6284, d_g6285, d_g6286, d_g6287, d_g6288);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
