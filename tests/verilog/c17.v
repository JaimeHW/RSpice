// ISCAS85 c17 -- the smallest of the 1985 combinational benchmark circuits.
//
// Six 2-input NAND gates, five primary inputs, two primary outputs. The
// structure is fixed by the published benchmark and is reproduced identically
// wherever the circuit appears; see this corpus's RSPICE-VENDORING.md for the
// provenance argument.
//
// Structural gate instantiation only: no behavioural statements, no `$display`,
// no initial blocks. Everything observable about this module travels through
// its output ports, which is what the digital oracle harness compares.

module c17 (N1, N2, N3, N6, N7, N22, N23);
  input  N1, N2, N3, N6, N7;
  output N22, N23;

  wire N10, N11, N16, N19;

  nand g10 (N10, N1,  N3);
  nand g11 (N11, N3,  N6);
  nand g16 (N16, N2,  N11);
  nand g19 (N19, N11, N7);
  nand g22 (N22, N10, N16);
  nand g23 (N23, N16, N19);
endmodule
