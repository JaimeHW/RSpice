// Two drivers on one net, and what the resolution table says the net reads.
//
// The corpus's other four-state case, `xz_propagation`, is about how x and z
// travel through operators. This one is about where they come from: a net with
// more than one continuous driver, which IEEE 1364-2005 section 6.1 resolves
// bit by bit through table 5-1.
//
// Four situations, and the design is arranged so a stimulus can reach all of
// them:
//
// * one driver enabled -- the net reads that driver
// * neither enabled -- both drive z, and the net is z
// * both enabled and agreeing -- the net reads the agreed value
// * both enabled and disagreeing -- the net is x, per bit, so a word where the
//   two operands agree on some bits and not others reads as a mixture
//
// `seen` is a third net reading the resolved one, so the answer has to survive
// being consumed as well as being observed. Icarus only: Verilator is a
// two-state simulator by design and cannot arbitrate a z.

module bus_forms (ena, enb, a, b, bus, seen);
  input        ena;
  input        enb;
  input  [3:0] a;
  input  [3:0] b;
  output [3:0] bus;
  output [3:0] seen;

  assign bus  = ena ? a : 4'bzzzz;
  assign bus  = enb ? b : 4'bzzzz;
  assign seen = bus;
endmodule
