// C10 c1355 O3 template, variant c1355-vlog: Icarus runs net/c1355.v on the primary
// inputs and strobe the RSpice vlog run converted. The wrapper instance xdut
// carries the converter endpoints' names, so the dump names the outputs
// xdut.<port>__event_0 as RSpice does.
// rspice-o3-sources: ../../net/c1355.v
// rspice-o3-inputs: xdut.g21__event_0 xdut.g18__event_0 xdut.g31__event_0 xdut.g17__event_0 xdut.g27__event_0 xdut.g11__event_0 xdut.g22__event_0 xdut.g32__event_0 xdut.g30__event_0 xdut.g4__event_0 xdut.g12__event_0 xdut.g29__event_0 xdut.g24__event_0 xdut.g39__event_0 xdut.g38__event_0 xdut.g20__event_0 xdut.g5__event_0 xdut.g35__event_0 xdut.g23__event_0 xdut.g13__event_0 xdut.g2__event_0 xdut.g37__event_0 xdut.g41__event_0 xdut.g9__event_0 xdut.g40__event_0 xdut.g14__event_0 xdut.g36__event_0 xdut.g19__event_0 xdut.g8__event_0 xdut.g3__event_0 xdut.g26__event_0 xdut.g16__event_0 xdut.g10__event_0 xdut.g6__event_0 xdut.g1__event_0 xdut.g28__event_0 xdut.g25__event_0 xdut.g15__event_0 xdut.g33__event_0 xdut.g7__event_0 xdut.g34__event_0 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module c1355_events( g21__event_0, g18__event_0, g31__event_0, g17__event_0, g27__event_0,
    g11__event_0, g22__event_0, g32__event_0, g30__event_0, g4__event_0, g12__event_0, g29__event_0,
    g24__event_0, g39__event_0, g38__event_0, g20__event_0, g5__event_0, g35__event_0, g23__event_0,
    g13__event_0, g2__event_0, g37__event_0, g41__event_0, g9__event_0, g40__event_0, g14__event_0,
    g36__event_0, g19__event_0, g8__event_0, g3__event_0, g26__event_0, g16__event_0, g10__event_0,
    g6__event_0, g1__event_0, g28__event_0, g25__event_0, g15__event_0, g33__event_0, g7__event_0,
    g34__event_0, g1324__event_0, g1325__event_0, g1326__event_0, g1327__event_0, g1328__event_0,
    g1329__event_0, g1330__event_0, g1331__event_0, g1332__event_0, g1333__event_0, g1334__event_0,
    g1335__event_0, g1336__event_0, g1337__event_0, g1338__event_0, g1339__event_0, g1340__event_0,
    g1341__event_0, g1342__event_0, g1343__event_0, g1344__event_0, g1345__event_0, g1346__event_0,
    g1347__event_0, g1348__event_0, g1349__event_0, g1350__event_0, g1351__event_0, g1352__event_0,
    g1353__event_0, g1354__event_0, g1355__event_0);
  input g21__event_0, g18__event_0, g31__event_0, g17__event_0, g27__event_0, g11__event_0,
    g22__event_0, g32__event_0, g30__event_0, g4__event_0, g12__event_0, g29__event_0, g24__event_0,
    g39__event_0, g38__event_0, g20__event_0, g5__event_0, g35__event_0, g23__event_0, g13__event_0,
    g2__event_0, g37__event_0, g41__event_0, g9__event_0, g40__event_0, g14__event_0, g36__event_0,
    g19__event_0, g8__event_0, g3__event_0, g26__event_0, g16__event_0, g10__event_0, g6__event_0,
    g1__event_0, g28__event_0, g25__event_0, g15__event_0, g33__event_0, g7__event_0, g34__event_0;
  output g1324__event_0, g1325__event_0, g1326__event_0, g1327__event_0, g1328__event_0,
    g1329__event_0, g1330__event_0, g1331__event_0, g1332__event_0, g1333__event_0, g1334__event_0,
    g1335__event_0, g1336__event_0, g1337__event_0, g1338__event_0, g1339__event_0, g1340__event_0,
    g1341__event_0, g1342__event_0, g1343__event_0, g1344__event_0, g1345__event_0, g1346__event_0,
    g1347__event_0, g1348__event_0, g1349__event_0, g1350__event_0, g1351__event_0, g1352__event_0,
    g1353__event_0, g1354__event_0, g1355__event_0;
  c1355 dut( g21__event_0, g18__event_0, g31__event_0, g17__event_0, g27__event_0, g11__event_0,
    g22__event_0, g32__event_0, g30__event_0, g4__event_0, g12__event_0, g29__event_0, g24__event_0,
    g39__event_0, g38__event_0, g20__event_0, g5__event_0, g35__event_0, g23__event_0, g13__event_0,
    g2__event_0, g37__event_0, g41__event_0, g9__event_0, g40__event_0, g14__event_0, g36__event_0,
    g19__event_0, g8__event_0, g3__event_0, g26__event_0, g16__event_0, g10__event_0, g6__event_0,
    g1__event_0, g28__event_0, g25__event_0, g15__event_0, g33__event_0, g7__event_0, g34__event_0,
    g1324__event_0, g1325__event_0, g1326__event_0, g1327__event_0, g1328__event_0, g1329__event_0,
    g1330__event_0, g1331__event_0, g1332__event_0, g1333__event_0, g1334__event_0, g1335__event_0,
    g1336__event_0, g1337__event_0, g1338__event_0, g1339__event_0, g1340__event_0, g1341__event_0,
    g1342__event_0, g1343__event_0, g1344__event_0, g1345__event_0, g1346__event_0, g1347__event_0,
    g1348__event_0, g1349__event_0, g1350__event_0, g1351__event_0, g1352__event_0, g1353__event_0,
    g1354__event_0, g1355__event_0);
endmodule
module tb;
  reg xdut_g21__event_0, xdut_g18__event_0, xdut_g31__event_0, xdut_g17__event_0, xdut_g27__event_0,
    xdut_g11__event_0, xdut_g22__event_0, xdut_g32__event_0, xdut_g30__event_0, xdut_g4__event_0,
    xdut_g12__event_0, xdut_g29__event_0, xdut_g24__event_0, xdut_g39__event_0, xdut_g38__event_0,
    xdut_g20__event_0, xdut_g5__event_0, xdut_g35__event_0, xdut_g23__event_0, xdut_g13__event_0,
    xdut_g2__event_0, xdut_g37__event_0, xdut_g41__event_0, xdut_g9__event_0, xdut_g40__event_0,
    xdut_g14__event_0, xdut_g36__event_0, xdut_g19__event_0, xdut_g8__event_0, xdut_g3__event_0,
    xdut_g26__event_0, xdut_g16__event_0, xdut_g10__event_0, xdut_g6__event_0, xdut_g1__event_0,
    xdut_g28__event_0, xdut_g25__event_0, xdut_g15__event_0, xdut_g33__event_0, xdut_g7__event_0,
    xdut_g34__event_0, d_stb;
  wire o_g1324, o_g1325, o_g1326, o_g1327, o_g1328, o_g1329, o_g1330, o_g1331, o_g1332, o_g1333,
    o_g1334, o_g1335, o_g1336, o_g1337, o_g1338, o_g1339, o_g1340, o_g1341, o_g1342, o_g1343,
    o_g1344, o_g1345, o_g1346, o_g1347, o_g1348, o_g1349, o_g1350, o_g1351, o_g1352, o_g1353,
    o_g1354, o_g1355;
  c1355_events xdut( xdut_g21__event_0, xdut_g18__event_0, xdut_g31__event_0, xdut_g17__event_0,
    xdut_g27__event_0, xdut_g11__event_0, xdut_g22__event_0, xdut_g32__event_0, xdut_g30__event_0,
    xdut_g4__event_0, xdut_g12__event_0, xdut_g29__event_0, xdut_g24__event_0, xdut_g39__event_0,
    xdut_g38__event_0, xdut_g20__event_0, xdut_g5__event_0, xdut_g35__event_0, xdut_g23__event_0,
    xdut_g13__event_0, xdut_g2__event_0, xdut_g37__event_0, xdut_g41__event_0, xdut_g9__event_0,
    xdut_g40__event_0, xdut_g14__event_0, xdut_g36__event_0, xdut_g19__event_0, xdut_g8__event_0,
    xdut_g3__event_0, xdut_g26__event_0, xdut_g16__event_0, xdut_g10__event_0, xdut_g6__event_0,
    xdut_g1__event_0, xdut_g28__event_0, xdut_g25__event_0, xdut_g15__event_0, xdut_g33__event_0,
    xdut_g7__event_0, xdut_g34__event_0, o_g1324, o_g1325, o_g1326, o_g1327, o_g1328, o_g1329,
    o_g1330, o_g1331, o_g1332, o_g1333, o_g1334, o_g1335, o_g1336, o_g1337, o_g1338, o_g1339,
    o_g1340, o_g1341, o_g1342, o_g1343, o_g1344, o_g1345, o_g1346, o_g1347, o_g1348, o_g1349,
    o_g1350, o_g1351, o_g1352, o_g1353, o_g1354, o_g1355);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(2, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
