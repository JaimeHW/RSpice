// C10 c432 O3 template, variant c432-vlog: Icarus runs net/c432.v on the primary
// inputs and strobe the RSpice vlog run converted. The wrapper instance xdut
// carries the converter endpoints' names, so the dump names the outputs
// xdut.<port>__event_0 as RSpice does.
// rspice-o3-sources: ../../net/c432.v
// rspice-o3-inputs: xdut.g35__event_0 xdut.g21__event_0 xdut.g18__event_0 xdut.g23__event_0 xdut.g13__event_0 xdut.g2__event_0 xdut.g31__event_0 xdut.g9__event_0 xdut.g17__event_0 xdut.g27__event_0 xdut.g14__event_0 xdut.g36__event_0 xdut.g19__event_0 xdut.g8__event_0 xdut.g3__event_0 xdut.g26__event_0 xdut.g16__event_0 xdut.g11__event_0 xdut.g22__event_0 xdut.g32__event_0 xdut.g30__event_0 xdut.g10__event_0 xdut.g6__event_0 xdut.g1__event_0 xdut.g28__event_0 xdut.g25__event_0 xdut.g4__event_0 xdut.g15__event_0 xdut.g12__event_0 xdut.g29__event_0 xdut.g33__event_0 xdut.g7__event_0 xdut.g24__event_0 xdut.g34__event_0 xdut.g20__event_0 xdut.g5__event_0 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module c432_events( g35__event_0, g21__event_0, g18__event_0, g23__event_0, g13__event_0,
    g2__event_0, g31__event_0, g9__event_0, g17__event_0, g27__event_0, g14__event_0, g36__event_0,
    g19__event_0, g8__event_0, g3__event_0, g26__event_0, g16__event_0, g11__event_0, g22__event_0,
    g32__event_0, g30__event_0, g10__event_0, g6__event_0, g1__event_0, g28__event_0, g25__event_0,
    g4__event_0, g15__event_0, g12__event_0, g29__event_0, g33__event_0, g7__event_0, g24__event_0,
    g34__event_0, g20__event_0, g5__event_0, g426__event_0, g427__event_0, g428__event_0,
    g429__event_0, g430__event_0, g431__event_0, g432__event_0);
  input g35__event_0, g21__event_0, g18__event_0, g23__event_0, g13__event_0, g2__event_0,
    g31__event_0, g9__event_0, g17__event_0, g27__event_0, g14__event_0, g36__event_0, g19__event_0,
    g8__event_0, g3__event_0, g26__event_0, g16__event_0, g11__event_0, g22__event_0, g32__event_0,
    g30__event_0, g10__event_0, g6__event_0, g1__event_0, g28__event_0, g25__event_0, g4__event_0,
    g15__event_0, g12__event_0, g29__event_0, g33__event_0, g7__event_0, g24__event_0, g34__event_0,
    g20__event_0, g5__event_0;
  output g426__event_0, g427__event_0, g428__event_0, g429__event_0, g430__event_0, g431__event_0,
    g432__event_0;
  c432 dut( g35__event_0, g21__event_0, g18__event_0, g23__event_0, g13__event_0, g2__event_0,
    g31__event_0, g9__event_0, g17__event_0, g27__event_0, g14__event_0, g36__event_0, g19__event_0,
    g8__event_0, g3__event_0, g26__event_0, g16__event_0, g11__event_0, g22__event_0, g32__event_0,
    g30__event_0, g10__event_0, g6__event_0, g1__event_0, g28__event_0, g25__event_0, g4__event_0,
    g15__event_0, g12__event_0, g29__event_0, g33__event_0, g7__event_0, g24__event_0, g34__event_0,
    g20__event_0, g5__event_0, g426__event_0, g427__event_0, g428__event_0, g429__event_0,
    g430__event_0, g431__event_0, g432__event_0);
endmodule
module tb;
  reg xdut_g35__event_0, xdut_g21__event_0, xdut_g18__event_0, xdut_g23__event_0, xdut_g13__event_0,
    xdut_g2__event_0, xdut_g31__event_0, xdut_g9__event_0, xdut_g17__event_0, xdut_g27__event_0,
    xdut_g14__event_0, xdut_g36__event_0, xdut_g19__event_0, xdut_g8__event_0, xdut_g3__event_0,
    xdut_g26__event_0, xdut_g16__event_0, xdut_g11__event_0, xdut_g22__event_0, xdut_g32__event_0,
    xdut_g30__event_0, xdut_g10__event_0, xdut_g6__event_0, xdut_g1__event_0, xdut_g28__event_0,
    xdut_g25__event_0, xdut_g4__event_0, xdut_g15__event_0, xdut_g12__event_0, xdut_g29__event_0,
    xdut_g33__event_0, xdut_g7__event_0, xdut_g24__event_0, xdut_g34__event_0, xdut_g20__event_0,
    xdut_g5__event_0, d_stb;
  wire o_g426, o_g427, o_g428, o_g429, o_g430, o_g431, o_g432;
  c432_events xdut( xdut_g35__event_0, xdut_g21__event_0, xdut_g18__event_0, xdut_g23__event_0,
    xdut_g13__event_0, xdut_g2__event_0, xdut_g31__event_0, xdut_g9__event_0, xdut_g17__event_0,
    xdut_g27__event_0, xdut_g14__event_0, xdut_g36__event_0, xdut_g19__event_0, xdut_g8__event_0,
    xdut_g3__event_0, xdut_g26__event_0, xdut_g16__event_0, xdut_g11__event_0, xdut_g22__event_0,
    xdut_g32__event_0, xdut_g30__event_0, xdut_g10__event_0, xdut_g6__event_0, xdut_g1__event_0,
    xdut_g28__event_0, xdut_g25__event_0, xdut_g4__event_0, xdut_g15__event_0, xdut_g12__event_0,
    xdut_g29__event_0, xdut_g33__event_0, xdut_g7__event_0, xdut_g24__event_0, xdut_g34__event_0,
    xdut_g20__event_0, xdut_g5__event_0, o_g426, o_g427, o_g428, o_g429, o_g430, o_g431, o_g432);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(2, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
