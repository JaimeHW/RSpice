// C10 c6288 O3 template, variant c6288-vlog: Icarus runs net/c6288.v on the primary
// inputs and strobe the RSpice vlog run converted. The wrapper instance xdut
// carries the converter endpoints' names, so the dump names the outputs
// xdut.<port>__event_0 as RSpice does.
// rspice-o3-sources: ../../net/c6288.v
// rspice-o3-inputs: xdut.g21__event_0 xdut.g18__event_0 xdut.g30__event_0 xdut.g5__event_0 xdut.g14__event_0 xdut.g26__event_0 xdut.g15__event_0 xdut.g7__event_0 xdut.g17__event_0 xdut.g27__event_0 xdut.g22__event_0 xdut.g4__event_0 xdut.g23__event_0 xdut.g13__event_0 xdut.g19__event_0 xdut.g6__event_0 xdut.g1__event_0 xdut.g28__event_0 xdut.g32__event_0 xdut.g12__event_0 xdut.g24__event_0 xdut.g9__event_0 xdut.g3__event_0 xdut.g10__event_0 xdut.g31__event_0 xdut.g11__event_0 xdut.g29__event_0 xdut.g20__event_0 xdut.g2__event_0 xdut.g8__event_0 xdut.g16__event_0 xdut.g25__event_0 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module c6288_events( g21__event_0, g18__event_0, g30__event_0, g5__event_0, g14__event_0,
    g26__event_0, g15__event_0, g7__event_0, g17__event_0, g27__event_0, g22__event_0, g4__event_0,
    g23__event_0, g13__event_0, g19__event_0, g6__event_0, g1__event_0, g28__event_0, g32__event_0,
    g12__event_0, g24__event_0, g9__event_0, g3__event_0, g10__event_0, g31__event_0, g11__event_0,
    g29__event_0, g20__event_0, g2__event_0, g8__event_0, g16__event_0, g25__event_0,
    g6257__event_0, g6258__event_0, g6259__event_0, g6260__event_0, g6261__event_0, g6262__event_0,
    g6263__event_0, g6264__event_0, g6265__event_0, g6266__event_0, g6267__event_0, g6268__event_0,
    g6269__event_0, g6270__event_0, g6271__event_0, g6272__event_0, g6273__event_0, g6274__event_0,
    g6275__event_0, g6276__event_0, g6277__event_0, g6278__event_0, g6279__event_0, g6280__event_0,
    g6281__event_0, g6282__event_0, g6283__event_0, g6284__event_0, g6285__event_0, g6286__event_0,
    g6287__event_0, g6288__event_0);
  input g21__event_0, g18__event_0, g30__event_0, g5__event_0, g14__event_0, g26__event_0,
    g15__event_0, g7__event_0, g17__event_0, g27__event_0, g22__event_0, g4__event_0, g23__event_0,
    g13__event_0, g19__event_0, g6__event_0, g1__event_0, g28__event_0, g32__event_0, g12__event_0,
    g24__event_0, g9__event_0, g3__event_0, g10__event_0, g31__event_0, g11__event_0, g29__event_0,
    g20__event_0, g2__event_0, g8__event_0, g16__event_0, g25__event_0;
  output g6257__event_0, g6258__event_0, g6259__event_0, g6260__event_0, g6261__event_0,
    g6262__event_0, g6263__event_0, g6264__event_0, g6265__event_0, g6266__event_0, g6267__event_0,
    g6268__event_0, g6269__event_0, g6270__event_0, g6271__event_0, g6272__event_0, g6273__event_0,
    g6274__event_0, g6275__event_0, g6276__event_0, g6277__event_0, g6278__event_0, g6279__event_0,
    g6280__event_0, g6281__event_0, g6282__event_0, g6283__event_0, g6284__event_0, g6285__event_0,
    g6286__event_0, g6287__event_0, g6288__event_0;
  c6288 dut( g21__event_0, g18__event_0, g30__event_0, g5__event_0, g14__event_0, g26__event_0,
    g15__event_0, g7__event_0, g17__event_0, g27__event_0, g22__event_0, g4__event_0, g23__event_0,
    g13__event_0, g19__event_0, g6__event_0, g1__event_0, g28__event_0, g32__event_0, g12__event_0,
    g24__event_0, g9__event_0, g3__event_0, g10__event_0, g31__event_0, g11__event_0, g29__event_0,
    g20__event_0, g2__event_0, g8__event_0, g16__event_0, g25__event_0, g6257__event_0,
    g6258__event_0, g6259__event_0, g6260__event_0, g6261__event_0, g6262__event_0, g6263__event_0,
    g6264__event_0, g6265__event_0, g6266__event_0, g6267__event_0, g6268__event_0, g6269__event_0,
    g6270__event_0, g6271__event_0, g6272__event_0, g6273__event_0, g6274__event_0, g6275__event_0,
    g6276__event_0, g6277__event_0, g6278__event_0, g6279__event_0, g6280__event_0, g6281__event_0,
    g6282__event_0, g6283__event_0, g6284__event_0, g6285__event_0, g6286__event_0, g6287__event_0,
    g6288__event_0);
endmodule
module tb;
  reg xdut_g21__event_0, xdut_g18__event_0, xdut_g30__event_0, xdut_g5__event_0, xdut_g14__event_0,
    xdut_g26__event_0, xdut_g15__event_0, xdut_g7__event_0, xdut_g17__event_0, xdut_g27__event_0,
    xdut_g22__event_0, xdut_g4__event_0, xdut_g23__event_0, xdut_g13__event_0, xdut_g19__event_0,
    xdut_g6__event_0, xdut_g1__event_0, xdut_g28__event_0, xdut_g32__event_0, xdut_g12__event_0,
    xdut_g24__event_0, xdut_g9__event_0, xdut_g3__event_0, xdut_g10__event_0, xdut_g31__event_0,
    xdut_g11__event_0, xdut_g29__event_0, xdut_g20__event_0, xdut_g2__event_0, xdut_g8__event_0,
    xdut_g16__event_0, xdut_g25__event_0, d_stb;
  wire o_g6257, o_g6258, o_g6259, o_g6260, o_g6261, o_g6262, o_g6263, o_g6264, o_g6265, o_g6266,
    o_g6267, o_g6268, o_g6269, o_g6270, o_g6271, o_g6272, o_g6273, o_g6274, o_g6275, o_g6276,
    o_g6277, o_g6278, o_g6279, o_g6280, o_g6281, o_g6282, o_g6283, o_g6284, o_g6285, o_g6286,
    o_g6287, o_g6288;
  c6288_events xdut( xdut_g21__event_0, xdut_g18__event_0, xdut_g30__event_0, xdut_g5__event_0,
    xdut_g14__event_0, xdut_g26__event_0, xdut_g15__event_0, xdut_g7__event_0, xdut_g17__event_0,
    xdut_g27__event_0, xdut_g22__event_0, xdut_g4__event_0, xdut_g23__event_0, xdut_g13__event_0,
    xdut_g19__event_0, xdut_g6__event_0, xdut_g1__event_0, xdut_g28__event_0, xdut_g32__event_0,
    xdut_g12__event_0, xdut_g24__event_0, xdut_g9__event_0, xdut_g3__event_0, xdut_g10__event_0,
    xdut_g31__event_0, xdut_g11__event_0, xdut_g29__event_0, xdut_g20__event_0, xdut_g2__event_0,
    xdut_g8__event_0, xdut_g16__event_0, xdut_g25__event_0, o_g6257, o_g6258, o_g6259, o_g6260,
    o_g6261, o_g6262, o_g6263, o_g6264, o_g6265, o_g6266, o_g6267, o_g6268, o_g6269, o_g6270,
    o_g6271, o_g6272, o_g6273, o_g6274, o_g6275, o_g6276, o_g6277, o_g6278, o_g6279, o_g6280,
    o_g6281, o_g6282, o_g6283, o_g6284, o_g6285, o_g6286, o_g6287, o_g6288);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(2, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
