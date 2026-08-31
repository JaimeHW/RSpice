// Context-determined expression width, IEEE 1364-2005 section 5.4.1.
//
// The corpus's other cases ask what an operator computes. This one asks how
// wide it computes it, which section 5.4.1 answers with the size of the
// largest expression *including the assignment's left-hand side* -- and which
// is not the same question, because an operator run at the wrong width gives a
// wrong number rather than a narrow one.
//
// Every assignment here has a target wider than its operands, since that is
// the only arrangement in which the two readings of the rule differ. With the
// target no wider, computing at the operand width and computing at the context
// width agree on every bit, and the case would pass under either.
//
// The eight outputs, and the row of table 5-22 each one is:
//
// * `prod` -- `*` is context-determined on both sides, so `4'hF * 4'hF` in an
//   eight-bit target is 225 and not the 1 that multiplying at four bits and
//   widening the product afterwards leaves.
// * `sum5` -- one bit of target beyond the operands is enough: the carry out
//   of a four-bit sum is a real bit of a five-bit addition.
// * `shifted` -- a shift's result is the size of its *left* operand and that
//   operand is context-determined, so `a << 5` in an eight-bit target keeps
//   the bits a four-bit shift would have pushed out. The count is
//   self-determined and takes no part.
// * `mixed` -- the other side of the rule. Every operand of a concatenation is
//   self-determined, so `b + c` inside `{a, b + c}` is four bits and wraps,
//   however wide the target is. A front end that pushes the context through a
//   concatenation gets every other output here right and this one wrong.
// * `cmp` -- a comparison's operands size to each other and to nothing else.
//   `{a, b}` is eight bits and `c` is four, so `c` is zero-extended to eight
//   and the comparison is true only when `a` is zero as well; a comparison
//   made at four bits would ignore `a` entirely.
// * `inverted` -- `~` is context-determined with a result the size of its
//   operand, so an eight-bit target zero-extends the one bit `a == b` yields
//   and inverts all eight. Inverting first and widening afterwards gives
//   `8'b00000000` for every vector, which no vector here is.
// * `{cout, s}` -- the carry-out idiom. Section 5.4.1 puts the whole
//   concatenation target in the context, which is what makes `cout` the fifth
//   bit of the sum rather than a net that can never be 1.
//
// Two-state throughout: every input is driven 0 or 1 and every output is
// driven by exactly one continuous assignment, so no four-state rule is
// involved and both oracles can arbitrate.

module width_forms (a, b, c, prod, sum5, shifted, mixed, cmp, inverted, cout, s);
  input      [3:0] a;
  input      [3:0] b;
  input      [3:0] c;
  output     [7:0] prod;
  output     [4:0] sum5;
  output     [7:0] shifted;
  output     [7:0] mixed;
  output           cmp;
  output     [7:0] inverted;
  output           cout;
  output     [3:0] s;

  assign prod     = a * b;
  assign sum5     = a + b;
  assign shifted  = a << 5;
  assign mixed    = {a, b + c};
  assign cmp      = {a, b} == c;
  assign inverted = ~(a == b);
  assign {cout, s} = a + b;
endmodule
