// Signed expression semantics, IEEE 1364-2005 section 5.4.2.
//
// The corpus's other cases ask what an operator computes and how wide it
// computes it. This one asks what the bits *mean*, which section 5.4.2 answers
// from the declarations and which changes the answer rather than the width: the
// same four bits are 15 under a `reg` and -1 under a `reg signed`, and every
// operator that has to widen, compare or divide them gives two different
// results depending on which.
//
// Three inputs, all four bits, and the whole design turns on the difference
// between them: `a` and `b` are `signed`, `u` is not, and no other declaration
// differs. Every output below is a rule of the clause:
//
// * `ext` -- section 5.4.1's extension under rule (j). `a` is signed, so it
//   sign-extends into the eight-bit context the target gives it: `4'b1111`
//   becomes `8'b11111111`, not the `8'b00001111` an unsigned operand gives.
//   Rule (a) is in here too -- `ext` itself is not declared `signed`, and the
//   extension is decided by the right-hand side alone.
// * `zext` -- the control for it. Same assignment, unsigned operand, zero fill.
// * `cmp` -- section 4.1.6 with rule (j): both operands signed, so the
//   comparison is signed and `-1 < 1` holds.
// * `mix` -- the same comparison with one unsigned operand, which makes the
//   whole comparison unsigned. `a` is still declared `signed` and is read as a
//   magnitude anyway, so `a < u` is `15 < 1` and false where `cmp` was true.
// * `quot`, `rem` -- section 4.1.5. Division truncates toward zero and the
//   modulus takes the sign of its first operand, so `-7 / 2` is -3 and
//   `-7 % 2` is -1. These are the only arithmetic operators whose bits differ:
//   `+`, `-` and `*` are one operation on two's complement at a common width,
//   which is why the difference `sum` shows is in its operands' extension
//   rather than in the addition.
// * `asr`, `lsr` -- section 4.1.12. `>>>` fills the vacated positions with the
//   sign bit of a signed expression; `>>` fills with zero whatever the operand
//   was declared. `4'b1001 >>> 1` is `4'b1100` and `4'b1001 >> 1` is `4'b0100`.
// * `sum` -- both operands signed, so both sign-extend into eight bits before
//   the addition: `-7 + 2` is `8'b11111011`.
// * `poison` -- rule (j) from the other side. One unsigned operand makes the
//   whole expression unsigned, so the *signed* `a` is zero-extended into it and
//   `a + u` is `9 + 3` rather than `-7 + 3`. A front end that extended each
//   operand by its own declaration instead of by the expression's gets every
//   other output here right and this one wrong.
// * `part` -- rule (e). A part-select is unsigned regardless of its operand,
//   even one selecting the whole vector, so `b[3:0]` is unsigned, the sum is
//   unsigned, and `a` is zero-extended again. This is how a signed expression
//   stops being one with no unsigned declaration anywhere in it.
//
// Two-state throughout, and every `b` is non-zero so that no division is the
// unknown section 4.1.5 gives division by zero. Both oracles can arbitrate.

module signed_forms (a, b, u, ext, zext, cmp, mix, quot, rem, asr, lsr, sum,
                     poison, part);
  input signed [3:0] a;
  input signed [3:0] b;
  input        [3:0] u;
  output       [7:0] ext;
  output       [7:0] zext;
  output             cmp;
  output             mix;
  output       [3:0] quot;
  output       [3:0] rem;
  output       [3:0] asr;
  output       [3:0] lsr;
  output       [7:0] sum;
  output       [7:0] poison;
  output       [7:0] part;

  assign ext    = a;
  assign zext   = u;
  assign cmp    = a < b;
  assign mix    = a < u;
  assign quot   = a / b;
  assign rem    = a % b;
  assign asr    = a >>> 1;
  assign lsr    = a >> 1;
  assign sum    = a + b;
  assign poison = a + u;
  assign part   = a + b[3:0];
endmodule
