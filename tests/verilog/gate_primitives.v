// Every wave-1 combinational construct the front end must accept, in one
// module: all eight Verilog gate primitives driven from a shared input cone,
// plus the continuous-assignment spellings of the same operators.
//
// The point of pairing them is that `and g (y, a, b)` and `assign y = a & b`
// must produce identical values for identical inputs. A front end that
// implements gate primitives and operators through different paths -- which is
// the normal way to build one -- has two chances to disagree, and this module
// makes the disagreement observable at a port rather than internal.
//
// Deliberately free of `$display`, initial blocks, and delays.

module gate_primitives (a, b, c,
                        g_and, g_or, g_nand, g_nor, g_xor, g_xnor, g_buf, g_not,
                        o_and, o_or, o_nand, o_nor, o_xor, o_xnor, o_buf, o_not,
                        wide, mux, reduce);
  input  a, b, c;

  // Primitive-instantiated forms.
  output g_and, g_or, g_nand, g_nor, g_xor, g_xnor, g_buf, g_not;
  // Operator forms of the same eight functions.
  output o_and, o_or, o_nand, o_nor, o_xor, o_xnor, o_buf, o_not;
  // Vector, conditional, and reduction forms.
  output [3:0] wide;
  output       mux;
  output       reduce;

  and  u_and  (g_and,  a, b);
  or   u_or   (g_or,   a, b);
  nand u_nand (g_nand, a, b);
  nor  u_nor  (g_nor,  a, b);
  xor  u_xor  (g_xor,  a, b);
  xnor u_xnor (g_xnor, a, b);
  buf  u_buf  (g_buf,  a);
  not  u_not  (g_not,  a);

  assign o_and  =  (a & b);
  assign o_or   =  (a | b);
  assign o_nand = ~(a & b);
  assign o_nor  = ~(a | b);
  assign o_xor  =  (a ^ b);
  assign o_xnor =  (a ~^ b);
  assign o_buf  =   a;
  assign o_not  =  ~a;

  // Concatenation and a vector operator, so bit ordering is observable.
  assign wide   = {a, b, c, 1'b1} ^ 4'b1010;
  // Conditional operator with a non-constant selector.
  assign mux    = c ? a : b;
  // Reduction operator over a concatenation.
  assign reduce = ^{a, b, c};
endmodule
