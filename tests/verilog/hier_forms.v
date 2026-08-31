// The port-connection forms, and the explicit event-list separators.
//
// The corpus's other hierarchical case connects its instances by name through
// a generate loop. Two things it therefore does not reach are here:
//
// * **ordered port connections** (IEEE 1364-2005 section 12.3.5) -- the same
//   child instantiated once positionally and once by name, with the named
//   connections deliberately written in a different order from the
//   declaration, so a front end that quietly treats named as positional wires
//   the cell backwards and both halves of the adder come out wrong
// * **both separators of an event expression** (section 9.7.4) -- `or` and
//   `,` are synonyms there, and each is used once
//
// The two level-sensitive processes name every signal they read, so their
// answers are what an implicit list would give. What they buy is the grammar,
// not a behavioural distinction, and they read the buffered nets rather than
// the ports so that their first wake-up follows a net change rather than
// racing the vector that caused it.

module hier_forms_cell (a, b, sum, carry);
  input  a, b;
  output sum, carry;

  wire   both;

  and g0 (both, a, b);
  xor g1 (sum, a, b);
  buf g2 (carry, both);
endmodule

module hier_forms (x, y, s0, c0, s1, c1, anded, ored);
  input      [1:0] x;
  input      [1:0] y;
  output           s0;
  output           c0;
  output           s1;
  output           c1;
  output reg [1:0] anded;
  output reg [1:0] ored;

  wire x0, x1, y0, y1;

  buf b0 (x0, x[0]);
  buf b1 (x1, x[1]);
  buf b2 (y0, y[0]);
  buf b3 (y1, y[1]);

  hier_forms_cell u_ordered (x0, y0, s0, c0);
  hier_forms_cell u_named (.carry(c1), .b(y1), .sum(s1), .a(x1));

  always @(x0 or x1 or y0 or y1) anded = {x1 & y1, x0 & y0};
  always @(x0, x1, y0, y1)       ored  = {x1 | y1, x0 | y0};
endmodule
