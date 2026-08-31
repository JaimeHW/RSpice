// The spellings of a gate instantiation that nothing else in this corpus
// uses.
//
// Every gate elsewhere in the corpus is a named two-input instance on its own
// statement. IEEE 1364-2005 section 7.1 allows three other forms, and each one
// is a separate path through the parser:
//
// * an **unnamed** instance -- `and (wide, a, b, c, d);`
// * **several instances on one statement**, separated by commas
// * an **n-input** gate, here four inputs rather than two
// * a **multi-output** `buf` and `not`, where section 7.4 makes every terminal
//   but the last an output driven from that last one
//
// Exhaustive over its four inputs, so there is nothing left to sample.

module gate_forms (a, b, c, d, wide, pair0, pair1, fanned0, fanned1, inverted0, inverted1);
  input  a, b, c, d;
  output wide, pair0, pair1, fanned0, fanned1, inverted0, inverted1;

  and  (wide, a, b, c, d);
  nand n0 (pair0, a, b), n1 (pair1, c, d);
  buf  (fanned0, fanned1, a);
  not  (inverted0, inverted1, b);
endmodule
