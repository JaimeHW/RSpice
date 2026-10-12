// C10 c1908 O3 template, variant c1908-xs: Icarus runs net/c1908.v (the vlog rendering)
// on the primary inputs and strobe the RSpice xs run converted; the outputs are
// dumped under the xs digital net names.
// rspice-o3-sources: ../../net/c1908.v
// rspice-o3-inputs: d_g21 d_g18 d_g32 d_g30 d_g12 d_g24 d_g5 d_g9 d_g14 d_g3 d_g26 d_g10 d_g15 d_g7 d_g31 d_g17 d_g27 d_g11 d_g22 d_g4 d_g29 d_g20 d_g23 d_g13 d_g2 d_g19 d_g8 d_g16 d_g6 d_g1 d_g28 d_g25 d_g33 d_stb
// rspice-o3-timescale: 1ps
`timescale 1ps/1ps
module tb;
  reg d_g21, d_g18, d_g32, d_g30, d_g12, d_g24, d_g5, d_g9, d_g14, d_g3, d_g26, d_g10, d_g15, d_g7,
    d_g31, d_g17, d_g27, d_g11, d_g22, d_g4, d_g29, d_g20, d_g23, d_g13, d_g2, d_g19, d_g8, d_g16,
    d_g6, d_g1, d_g28, d_g25, d_g33, d_stb;
  wire d_g1884, d_g1885, d_g1886, d_g1887, d_g1888, d_g1889, d_g1890, d_g1891, d_g1892, d_g1893,
    d_g1894, d_g1895, d_g1896, d_g1897, d_g1898, d_g1899, d_g1900, d_g1901, d_g1902, d_g1903,
    d_g1904, d_g1905, d_g1906, d_g1907, d_g1908;
  c1908 dut( d_g21, d_g18, d_g32, d_g30, d_g12, d_g24, d_g5, d_g9, d_g14, d_g3, d_g26, d_g10, d_g15,
    d_g7, d_g31, d_g17, d_g27, d_g11, d_g22, d_g4, d_g29, d_g20, d_g23, d_g13, d_g2, d_g19, d_g8,
    d_g16, d_g6, d_g1, d_g28, d_g25, d_g33, d_g1884, d_g1885, d_g1886, d_g1887, d_g1888, d_g1889,
    d_g1890, d_g1891, d_g1892, d_g1893, d_g1894, d_g1895, d_g1896, d_g1897, d_g1898, d_g1899,
    d_g1900, d_g1901, d_g1902, d_g1903, d_g1904, d_g1905, d_g1906, d_g1907, d_g1908);
  initial begin
    $dumpfile("o3.vcd");
    $dumpvars(1, tb);
    #650000 $finish;
  end
// RSPICE-O3-STIMULUS
endmodule
