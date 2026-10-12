// C10 c499 O3 template, variant c499-xs: Icarus runs net/c499.v (the vlog rendering)
// on the primary inputs and strobe the RSpice xs run converted; the outputs are
// dumped under the xs digital net names.
// rspice-o3-sources: ../../net/c499.v
// rspice-o3-inputs: d_g21 d_g18 d_g31 d_g17 d_g27 d_g11 d_g22 d_g32 d_g30 d_g4 d_g12 d_g29 d_g24 d_g39 d_g38 d_g20 d_g5 d_g35 d_g23 d_g13 d_g2 d_g37 d_g41 d_g9 d_g40 d_g14 d_g36 d_g19 d_g8 d_g3 d_g26 d_g16 d_g10 d_g6 d_g1 d_g28 d_g25 d_g15 d_g33 d_g7 d_g34 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module tb;
  reg d_g21, d_g18, d_g31, d_g17, d_g27, d_g11, d_g22, d_g32, d_g30, d_g4, d_g12, d_g29, d_g24,
    d_g39, d_g38, d_g20, d_g5, d_g35, d_g23, d_g13, d_g2, d_g37, d_g41, d_g9, d_g40, d_g14, d_g36,
    d_g19, d_g8, d_g3, d_g26, d_g16, d_g10, d_g6, d_g1, d_g28, d_g25, d_g15, d_g33, d_g7, d_g34,
    d_stb;
  wire d_g468, d_g469, d_g470, d_g471, d_g472, d_g473, d_g474, d_g475, d_g476, d_g477, d_g478,
    d_g479, d_g480, d_g481, d_g482, d_g483, d_g484, d_g485, d_g486, d_g487, d_g488, d_g489, d_g490,
    d_g491, d_g492, d_g493, d_g494, d_g495, d_g496, d_g497, d_g498, d_g499;
  c499 dut( d_g21, d_g18, d_g31, d_g17, d_g27, d_g11, d_g22, d_g32, d_g30, d_g4, d_g12, d_g29,
    d_g24, d_g39, d_g38, d_g20, d_g5, d_g35, d_g23, d_g13, d_g2, d_g37, d_g41, d_g9, d_g40, d_g14,
    d_g36, d_g19, d_g8, d_g3, d_g26, d_g16, d_g10, d_g6, d_g1, d_g28, d_g25, d_g15, d_g33, d_g7,
    d_g34, d_g468, d_g469, d_g470, d_g471, d_g472, d_g473, d_g474, d_g475, d_g476, d_g477, d_g478,
    d_g479, d_g480, d_g481, d_g482, d_g483, d_g484, d_g485, d_g486, d_g487, d_g488, d_g489, d_g490,
    d_g491, d_g492, d_g493, d_g494, d_g495, d_g496, d_g497, d_g498, d_g499);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
