// The three case statements, and the level-sensitive process they sit in.
//
// `case`, `casez` and `casex` differ only in what a label's bits mean, so the
// same selector is decoded three ways in one block and the three answers are
// separate outputs. A front end that treats `casez` as `case` gets `wildz`
// wrong and leaves `exact` right, which says exactly which mechanism broke.
//
// Everything else here is chosen to be covered nowhere else in this corpus:
// an implicit sensitivity list (`always @*`, section 9.7.5), a named block
// with a variable declared inside it (section 9.8.1), blocking assignment as
// the way a combinational process builds its result, a multi-label case item,
// and a `default` arm.
//
// `hit` is cleared at the top of the block and set only by one arm, which is
// the pattern that makes a combinational `case` free of inferred state. A
// process that ran only on `sel` rather than on everything it reads would
// leave `hit` stale when `a` or `b` changed.

module case_forms (sel, a, b, exact, wildz, wildx, hit);
  input      [3:0] sel;
  input      [7:0] a;
  input      [7:0] b;
  output reg [7:0] exact;
  output reg [7:0] wildz;
  output reg [7:0] wildx;
  output reg       hit;

  always @* begin : decode
    reg [7:0] mixed;

    mixed = a ^ b;
    hit   = 1'b0;

    case (sel)
      4'd0, 4'd1: exact = a;
      4'd2:       exact = b;
      4'd3:       begin
                    exact = mixed;
                    hit   = 1'b1;
                  end
      default:    exact = 8'h00;
    endcase

    casez (sel)
      4'b1???: wildz = a;
      4'b01??: wildz = b;
      4'b001?: wildz = mixed;
      default: wildz = 8'hFF;
    endcase

    casex (sel)
      4'b??11: wildx = a;
      4'b??0?: wildx = b;
      default: wildx = 8'h5A;
    endcase
  end
endmodule
