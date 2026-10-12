// C10 c1908 O3 template, variant c1908-vlog: Icarus runs net/c1908.v on the primary
// inputs and strobe the RSpice vlog run converted. The wrapper instance xdut
// carries the converter endpoints' names, so the dump names the outputs
// xdut.<port>__event_0 as RSpice does.
// rspice-o3-sources: ../../net/c1908.v
// rspice-o3-inputs: xdut.g21__event_0 xdut.g18__event_0 xdut.g32__event_0 xdut.g30__event_0 xdut.g12__event_0 xdut.g24__event_0 xdut.g5__event_0 xdut.g9__event_0 xdut.g14__event_0 xdut.g3__event_0 xdut.g26__event_0 xdut.g10__event_0 xdut.g15__event_0 xdut.g7__event_0 xdut.g31__event_0 xdut.g17__event_0 xdut.g27__event_0 xdut.g11__event_0 xdut.g22__event_0 xdut.g4__event_0 xdut.g29__event_0 xdut.g20__event_0 xdut.g23__event_0 xdut.g13__event_0 xdut.g2__event_0 xdut.g19__event_0 xdut.g8__event_0 xdut.g16__event_0 xdut.g6__event_0 xdut.g1__event_0 xdut.g28__event_0 xdut.g25__event_0 xdut.g33__event_0 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module c1908_events( g21__event_0, g18__event_0, g32__event_0, g30__event_0, g12__event_0,
    g24__event_0, g5__event_0, g9__event_0, g14__event_0, g3__event_0, g26__event_0, g10__event_0,
    g15__event_0, g7__event_0, g31__event_0, g17__event_0, g27__event_0, g11__event_0, g22__event_0,
    g4__event_0, g29__event_0, g20__event_0, g23__event_0, g13__event_0, g2__event_0, g19__event_0,
    g8__event_0, g16__event_0, g6__event_0, g1__event_0, g28__event_0, g25__event_0, g33__event_0,
    g1884__event_0, g1885__event_0, g1886__event_0, g1887__event_0, g1888__event_0, g1889__event_0,
    g1890__event_0, g1891__event_0, g1892__event_0, g1893__event_0, g1894__event_0, g1895__event_0,
    g1896__event_0, g1897__event_0, g1898__event_0, g1899__event_0, g1900__event_0, g1901__event_0,
    g1902__event_0, g1903__event_0, g1904__event_0, g1905__event_0, g1906__event_0, g1907__event_0,
    g1908__event_0);
  input g21__event_0, g18__event_0, g32__event_0, g30__event_0, g12__event_0, g24__event_0,
    g5__event_0, g9__event_0, g14__event_0, g3__event_0, g26__event_0, g10__event_0, g15__event_0,
    g7__event_0, g31__event_0, g17__event_0, g27__event_0, g11__event_0, g22__event_0, g4__event_0,
    g29__event_0, g20__event_0, g23__event_0, g13__event_0, g2__event_0, g19__event_0, g8__event_0,
    g16__event_0, g6__event_0, g1__event_0, g28__event_0, g25__event_0, g33__event_0;
  output g1884__event_0, g1885__event_0, g1886__event_0, g1887__event_0, g1888__event_0,
    g1889__event_0, g1890__event_0, g1891__event_0, g1892__event_0, g1893__event_0, g1894__event_0,
    g1895__event_0, g1896__event_0, g1897__event_0, g1898__event_0, g1899__event_0, g1900__event_0,
    g1901__event_0, g1902__event_0, g1903__event_0, g1904__event_0, g1905__event_0, g1906__event_0,
    g1907__event_0, g1908__event_0;
  c1908 dut( g21__event_0, g18__event_0, g32__event_0, g30__event_0, g12__event_0, g24__event_0,
    g5__event_0, g9__event_0, g14__event_0, g3__event_0, g26__event_0, g10__event_0, g15__event_0,
    g7__event_0, g31__event_0, g17__event_0, g27__event_0, g11__event_0, g22__event_0, g4__event_0,
    g29__event_0, g20__event_0, g23__event_0, g13__event_0, g2__event_0, g19__event_0, g8__event_0,
    g16__event_0, g6__event_0, g1__event_0, g28__event_0, g25__event_0, g33__event_0,
    g1884__event_0, g1885__event_0, g1886__event_0, g1887__event_0, g1888__event_0, g1889__event_0,
    g1890__event_0, g1891__event_0, g1892__event_0, g1893__event_0, g1894__event_0, g1895__event_0,
    g1896__event_0, g1897__event_0, g1898__event_0, g1899__event_0, g1900__event_0, g1901__event_0,
    g1902__event_0, g1903__event_0, g1904__event_0, g1905__event_0, g1906__event_0, g1907__event_0,
    g1908__event_0);
endmodule
module tb;
  reg xdut_g21__event_0, xdut_g18__event_0, xdut_g32__event_0, xdut_g30__event_0, xdut_g12__event_0,
    xdut_g24__event_0, xdut_g5__event_0, xdut_g9__event_0, xdut_g14__event_0, xdut_g3__event_0,
    xdut_g26__event_0, xdut_g10__event_0, xdut_g15__event_0, xdut_g7__event_0, xdut_g31__event_0,
    xdut_g17__event_0, xdut_g27__event_0, xdut_g11__event_0, xdut_g22__event_0, xdut_g4__event_0,
    xdut_g29__event_0, xdut_g20__event_0, xdut_g23__event_0, xdut_g13__event_0, xdut_g2__event_0,
    xdut_g19__event_0, xdut_g8__event_0, xdut_g16__event_0, xdut_g6__event_0, xdut_g1__event_0,
    xdut_g28__event_0, xdut_g25__event_0, xdut_g33__event_0, d_stb;
  wire o_g1884, o_g1885, o_g1886, o_g1887, o_g1888, o_g1889, o_g1890, o_g1891, o_g1892, o_g1893,
    o_g1894, o_g1895, o_g1896, o_g1897, o_g1898, o_g1899, o_g1900, o_g1901, o_g1902, o_g1903,
    o_g1904, o_g1905, o_g1906, o_g1907, o_g1908;
  c1908_events xdut( xdut_g21__event_0, xdut_g18__event_0, xdut_g32__event_0, xdut_g30__event_0,
    xdut_g12__event_0, xdut_g24__event_0, xdut_g5__event_0, xdut_g9__event_0, xdut_g14__event_0,
    xdut_g3__event_0, xdut_g26__event_0, xdut_g10__event_0, xdut_g15__event_0, xdut_g7__event_0,
    xdut_g31__event_0, xdut_g17__event_0, xdut_g27__event_0, xdut_g11__event_0, xdut_g22__event_0,
    xdut_g4__event_0, xdut_g29__event_0, xdut_g20__event_0, xdut_g23__event_0, xdut_g13__event_0,
    xdut_g2__event_0, xdut_g19__event_0, xdut_g8__event_0, xdut_g16__event_0, xdut_g6__event_0,
    xdut_g1__event_0, xdut_g28__event_0, xdut_g25__event_0, xdut_g33__event_0, o_g1884, o_g1885,
    o_g1886, o_g1887, o_g1888, o_g1889, o_g1890, o_g1891, o_g1892, o_g1893, o_g1894, o_g1895,
    o_g1896, o_g1897, o_g1898, o_g1899, o_g1900, o_g1901, o_g1902, o_g1903, o_g1904, o_g1905,
    o_g1906, o_g1907, o_g1908);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(2, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
