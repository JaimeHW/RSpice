// C10 c880 O3 template, variant c880-vlog: Icarus runs net/c880.v on the primary
// inputs and strobe the RSpice vlog run converted. The wrapper instance xdut
// carries the converter endpoints' names, so the dump names the outputs
// xdut.<port>__event_0 as RSpice does.
// rspice-o3-sources: ../../net/c880.v
// rspice-o3-inputs: xdut.g44__event_0 xdut.g57__event_0 xdut.g35__event_0 xdut.g21__event_0 xdut.g18__event_0 xdut.g23__event_0 xdut.g13__event_0 xdut.g53__event_0 xdut.g41__event_0 xdut.g37__event_0 xdut.g2__event_0 xdut.g31__event_0 xdut.g9__event_0 xdut.g17__event_0 xdut.g27__event_0 xdut.g60__event_0 xdut.g40__event_0 xdut.g14__event_0 xdut.g47__event_0 xdut.g36__event_0 xdut.g46__event_0 xdut.g19__event_0 xdut.g58__event_0 xdut.g8__event_0 xdut.g3__event_0 xdut.g26__event_0 xdut.g59__event_0 xdut.g54__event_0 xdut.g16__event_0 xdut.g11__event_0 xdut.g22__event_0 xdut.g32__event_0 xdut.g50__event_0 xdut.g30__event_0 xdut.g10__event_0 xdut.g45__event_0 xdut.g6__event_0 xdut.g1__event_0 xdut.g48__event_0 xdut.g28__event_0 xdut.g55__event_0 xdut.g42__event_0 xdut.g51__event_0 xdut.g25__event_0 xdut.g4__event_0 xdut.g15__event_0 xdut.g12__event_0 xdut.g29__event_0 xdut.g33__event_0 xdut.g7__event_0 xdut.g49__event_0 xdut.g24__event_0 xdut.g39__event_0 xdut.g56__event_0 xdut.g52__event_0 xdut.g34__event_0 xdut.g38__event_0 xdut.g20__event_0 xdut.g5__event_0 xdut.g43__event_0 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module c880_events( g44__event_0, g57__event_0, g35__event_0, g21__event_0, g18__event_0,
    g23__event_0, g13__event_0, g53__event_0, g41__event_0, g37__event_0, g2__event_0, g31__event_0,
    g9__event_0, g17__event_0, g27__event_0, g60__event_0, g40__event_0, g14__event_0, g47__event_0,
    g36__event_0, g46__event_0, g19__event_0, g58__event_0, g8__event_0, g3__event_0, g26__event_0,
    g59__event_0, g54__event_0, g16__event_0, g11__event_0, g22__event_0, g32__event_0,
    g50__event_0, g30__event_0, g10__event_0, g45__event_0, g6__event_0, g1__event_0, g48__event_0,
    g28__event_0, g55__event_0, g42__event_0, g51__event_0, g25__event_0, g4__event_0, g15__event_0,
    g12__event_0, g29__event_0, g33__event_0, g7__event_0, g49__event_0, g24__event_0, g39__event_0,
    g56__event_0, g52__event_0, g34__event_0, g38__event_0, g20__event_0, g5__event_0, g43__event_0,
    g855__event_0, g856__event_0, g857__event_0, g858__event_0, g859__event_0, g860__event_0,
    g861__event_0, g862__event_0, g863__event_0, g864__event_0, g865__event_0, g866__event_0,
    g867__event_0, g868__event_0, g869__event_0, g870__event_0, g871__event_0, g872__event_0,
    g873__event_0, g874__event_0, g875__event_0, g876__event_0, g877__event_0, g878__event_0,
    g879__event_0, g880__event_0);
  input g44__event_0, g57__event_0, g35__event_0, g21__event_0, g18__event_0, g23__event_0,
    g13__event_0, g53__event_0, g41__event_0, g37__event_0, g2__event_0, g31__event_0, g9__event_0,
    g17__event_0, g27__event_0, g60__event_0, g40__event_0, g14__event_0, g47__event_0,
    g36__event_0, g46__event_0, g19__event_0, g58__event_0, g8__event_0, g3__event_0, g26__event_0,
    g59__event_0, g54__event_0, g16__event_0, g11__event_0, g22__event_0, g32__event_0,
    g50__event_0, g30__event_0, g10__event_0, g45__event_0, g6__event_0, g1__event_0, g48__event_0,
    g28__event_0, g55__event_0, g42__event_0, g51__event_0, g25__event_0, g4__event_0, g15__event_0,
    g12__event_0, g29__event_0, g33__event_0, g7__event_0, g49__event_0, g24__event_0, g39__event_0,
    g56__event_0, g52__event_0, g34__event_0, g38__event_0, g20__event_0, g5__event_0, g43__event_0;
  output g855__event_0, g856__event_0, g857__event_0, g858__event_0, g859__event_0, g860__event_0,
    g861__event_0, g862__event_0, g863__event_0, g864__event_0, g865__event_0, g866__event_0,
    g867__event_0, g868__event_0, g869__event_0, g870__event_0, g871__event_0, g872__event_0,
    g873__event_0, g874__event_0, g875__event_0, g876__event_0, g877__event_0, g878__event_0,
    g879__event_0, g880__event_0;
  c880 dut( g44__event_0, g57__event_0, g35__event_0, g21__event_0, g18__event_0, g23__event_0,
    g13__event_0, g53__event_0, g41__event_0, g37__event_0, g2__event_0, g31__event_0, g9__event_0,
    g17__event_0, g27__event_0, g60__event_0, g40__event_0, g14__event_0, g47__event_0,
    g36__event_0, g46__event_0, g19__event_0, g58__event_0, g8__event_0, g3__event_0, g26__event_0,
    g59__event_0, g54__event_0, g16__event_0, g11__event_0, g22__event_0, g32__event_0,
    g50__event_0, g30__event_0, g10__event_0, g45__event_0, g6__event_0, g1__event_0, g48__event_0,
    g28__event_0, g55__event_0, g42__event_0, g51__event_0, g25__event_0, g4__event_0, g15__event_0,
    g12__event_0, g29__event_0, g33__event_0, g7__event_0, g49__event_0, g24__event_0, g39__event_0,
    g56__event_0, g52__event_0, g34__event_0, g38__event_0, g20__event_0, g5__event_0, g43__event_0,
    g855__event_0, g856__event_0, g857__event_0, g858__event_0, g859__event_0, g860__event_0,
    g861__event_0, g862__event_0, g863__event_0, g864__event_0, g865__event_0, g866__event_0,
    g867__event_0, g868__event_0, g869__event_0, g870__event_0, g871__event_0, g872__event_0,
    g873__event_0, g874__event_0, g875__event_0, g876__event_0, g877__event_0, g878__event_0,
    g879__event_0, g880__event_0);
endmodule
module tb;
  reg xdut_g44__event_0, xdut_g57__event_0, xdut_g35__event_0, xdut_g21__event_0, xdut_g18__event_0,
    xdut_g23__event_0, xdut_g13__event_0, xdut_g53__event_0, xdut_g41__event_0, xdut_g37__event_0,
    xdut_g2__event_0, xdut_g31__event_0, xdut_g9__event_0, xdut_g17__event_0, xdut_g27__event_0,
    xdut_g60__event_0, xdut_g40__event_0, xdut_g14__event_0, xdut_g47__event_0, xdut_g36__event_0,
    xdut_g46__event_0, xdut_g19__event_0, xdut_g58__event_0, xdut_g8__event_0, xdut_g3__event_0,
    xdut_g26__event_0, xdut_g59__event_0, xdut_g54__event_0, xdut_g16__event_0, xdut_g11__event_0,
    xdut_g22__event_0, xdut_g32__event_0, xdut_g50__event_0, xdut_g30__event_0, xdut_g10__event_0,
    xdut_g45__event_0, xdut_g6__event_0, xdut_g1__event_0, xdut_g48__event_0, xdut_g28__event_0,
    xdut_g55__event_0, xdut_g42__event_0, xdut_g51__event_0, xdut_g25__event_0, xdut_g4__event_0,
    xdut_g15__event_0, xdut_g12__event_0, xdut_g29__event_0, xdut_g33__event_0, xdut_g7__event_0,
    xdut_g49__event_0, xdut_g24__event_0, xdut_g39__event_0, xdut_g56__event_0, xdut_g52__event_0,
    xdut_g34__event_0, xdut_g38__event_0, xdut_g20__event_0, xdut_g5__event_0, xdut_g43__event_0,
    d_stb;
  wire o_g855, o_g856, o_g857, o_g858, o_g859, o_g860, o_g861, o_g862, o_g863, o_g864, o_g865,
    o_g866, o_g867, o_g868, o_g869, o_g870, o_g871, o_g872, o_g873, o_g874, o_g875, o_g876, o_g877,
    o_g878, o_g879, o_g880;
  c880_events xdut( xdut_g44__event_0, xdut_g57__event_0, xdut_g35__event_0, xdut_g21__event_0,
    xdut_g18__event_0, xdut_g23__event_0, xdut_g13__event_0, xdut_g53__event_0, xdut_g41__event_0,
    xdut_g37__event_0, xdut_g2__event_0, xdut_g31__event_0, xdut_g9__event_0, xdut_g17__event_0,
    xdut_g27__event_0, xdut_g60__event_0, xdut_g40__event_0, xdut_g14__event_0, xdut_g47__event_0,
    xdut_g36__event_0, xdut_g46__event_0, xdut_g19__event_0, xdut_g58__event_0, xdut_g8__event_0,
    xdut_g3__event_0, xdut_g26__event_0, xdut_g59__event_0, xdut_g54__event_0, xdut_g16__event_0,
    xdut_g11__event_0, xdut_g22__event_0, xdut_g32__event_0, xdut_g50__event_0, xdut_g30__event_0,
    xdut_g10__event_0, xdut_g45__event_0, xdut_g6__event_0, xdut_g1__event_0, xdut_g48__event_0,
    xdut_g28__event_0, xdut_g55__event_0, xdut_g42__event_0, xdut_g51__event_0, xdut_g25__event_0,
    xdut_g4__event_0, xdut_g15__event_0, xdut_g12__event_0, xdut_g29__event_0, xdut_g33__event_0,
    xdut_g7__event_0, xdut_g49__event_0, xdut_g24__event_0, xdut_g39__event_0, xdut_g56__event_0,
    xdut_g52__event_0, xdut_g34__event_0, xdut_g38__event_0, xdut_g20__event_0, xdut_g5__event_0,
    xdut_g43__event_0, o_g855, o_g856, o_g857, o_g858, o_g859, o_g860, o_g861, o_g862, o_g863,
    o_g864, o_g865, o_g866, o_g867, o_g868, o_g869, o_g870, o_g871, o_g872, o_g873, o_g874, o_g875,
    o_g876, o_g877, o_g878, o_g879, o_g880);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(2, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
