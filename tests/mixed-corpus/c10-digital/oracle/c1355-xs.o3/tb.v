// C10 c1355 O3 template, variant c1355-xs: Icarus runs net/c1355.v (the vlog rendering)
// on the primary inputs and strobe the RSpice xs run converted; the outputs are
// dumped under the xs digital net names.
// rspice-o3-sources: ../../net/c1355.v
// rspice-o3-inputs: d_g21 d_g18 d_g31 d_g17 d_g27 d_g11 d_g22 d_g32 d_g30 d_g4 d_g12 d_g29 d_g24 d_g39 d_g38 d_g20 d_g5 d_g35 d_g23 d_g13 d_g2 d_g37 d_g41 d_g9 d_g40 d_g14 d_g36 d_g19 d_g8 d_g3 d_g26 d_g16 d_g10 d_g6 d_g1 d_g28 d_g25 d_g15 d_g33 d_g7 d_g34 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module tb;
  reg d_g21, d_g18, d_g31, d_g17, d_g27, d_g11, d_g22, d_g32, d_g30, d_g4, d_g12, d_g29, d_g24,
    d_g39, d_g38, d_g20, d_g5, d_g35, d_g23, d_g13, d_g2, d_g37, d_g41, d_g9, d_g40, d_g14, d_g36,
    d_g19, d_g8, d_g3, d_g26, d_g16, d_g10, d_g6, d_g1, d_g28, d_g25, d_g15, d_g33, d_g7, d_g34,
    d_stb;
  wire d_g1324, d_g1325, d_g1326, d_g1327, d_g1328, d_g1329, d_g1330, d_g1331, d_g1332, d_g1333,
    d_g1334, d_g1335, d_g1336, d_g1337, d_g1338, d_g1339, d_g1340, d_g1341, d_g1342, d_g1343,
    d_g1344, d_g1345, d_g1346, d_g1347, d_g1348, d_g1349, d_g1350, d_g1351, d_g1352, d_g1353,
    d_g1354, d_g1355;
  c1355 dut( d_g21, d_g18, d_g31, d_g17, d_g27, d_g11, d_g22, d_g32, d_g30, d_g4, d_g12, d_g29,
    d_g24, d_g39, d_g38, d_g20, d_g5, d_g35, d_g23, d_g13, d_g2, d_g37, d_g41, d_g9, d_g40, d_g14,
    d_g36, d_g19, d_g8, d_g3, d_g26, d_g16, d_g10, d_g6, d_g1, d_g28, d_g25, d_g15, d_g33, d_g7,
    d_g34, d_g1324, d_g1325, d_g1326, d_g1327, d_g1328, d_g1329, d_g1330, d_g1331, d_g1332, d_g1333,
    d_g1334, d_g1335, d_g1336, d_g1337, d_g1338, d_g1339, d_g1340, d_g1341, d_g1342, d_g1343,
    d_g1344, d_g1345, d_g1346, d_g1347, d_g1348, d_g1349, d_g1350, d_g1351, d_g1352, d_g1353,
    d_g1354, d_g1355);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
