// Every assignment target the front end accepts, on one edge.
//
// A name, a part select, a concatenation, and a bit select as a right-hand
// side, all writing at the same clock edge:
//
// * `swapped` is written twice in one block, through two disjoint part
//   selects. Both must land, and neither may disturb the other's half.
// * `{hi, lo}` is a concatenation as an assignment target (IEEE 1364-2005
//   section 9.2): the right-hand side is split across two registers by width,
//   most significant first.
// * `bit0` is written from a bit select of the input.
// * `tagged` is a concatenation on the *right*, and one whose second member is
//   a sized literal. A concatenation's width is the sum of its members'
//   declared widths (IEEE 1364-2005 section 4.1.14), so the `1'b1` shifts the
//   part select by exactly one place; a member whose width came from anywhere
//   else would shift it by a different one and lose the whole word.
//
// Together they cross-check each other: `swapped` is the two halves of `d`
// exchanged, `hi` and `lo` are those same halves kept in place, so
// `swapped` must read as `{lo, hi}` on every vector and nothing else can make
// all three agree.
//
// The reset writes the concatenation target as well, so the clear path
// exercises the same splitting the data path does.

module lvalue_forms (clk, rst, d, swapped, hi, lo, bit0, tagged);
  input            clk;
  input            rst;
  input      [7:0] d;
  output reg [7:0] swapped;
  output reg [3:0] hi;
  output reg [3:0] lo;
  output reg       bit0;
  output reg [7:0] tagged;

  always @(posedge clk) begin
    if (rst) begin
      swapped  <= 8'd0;
      {hi, lo} <= 8'd0;
      bit0     <= 1'b0;
      tagged   <= 8'd0;
    end else begin
      swapped[7:4] <= d[3:0];
      swapped[3:0] <= d[7:4];
      {hi, lo}     <= d;
      bit0         <= d[0];
      tagged       <= {d[6:0], 1'b1};
    end
  end
endmodule
