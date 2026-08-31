// An 8-bit ripple-carry adder built structurally from full adders.
//
// Bought for depth rather than breadth: the carry chain is eight instances
// deep, so a front end that resolves module instantiation, port association,
// and parameterised generate loops incorrectly produces a wrong sum rather
// than a compile error. The carry-out and the sum are both primary outputs, so
// the failure is visible at a port at every bit position.
//
// `generate`/`genvar` are exercised deliberately: elaboration-time unrolling is
// a distinct front-end mechanism from ordinary instantiation and has no other
// coverage in this corpus.

module full_adder (a, b, cin, sum, cout);
  input  a, b, cin;
  output sum, cout;

  assign sum  = a ^ b ^ cin;
  assign cout = (a & b) | (cin & (a ^ b));
endmodule

module ripple_adder (a, b, cin, sum, cout);
  parameter WIDTH = 8;

  input  [WIDTH-1:0] a;
  input  [WIDTH-1:0] b;
  input              cin;
  output [WIDTH-1:0] sum;
  output             cout;

  wire [WIDTH:0] carry;

  assign carry[0] = cin;
  assign cout     = carry[WIDTH];

  genvar i;
  generate
    for (i = 0; i < WIDTH; i = i + 1) begin : bit_slice
      full_adder stage (
        .a    (a[i]),
        .b    (b[i]),
        .cin  (carry[i]),
        .sum  (sum[i]),
        .cout (carry[i+1])
      );
    end
  endgenerate
endmodule
