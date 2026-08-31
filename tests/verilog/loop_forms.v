// The three procedural loops, in one clocked process.
//
// `for`, `while` and `repeat` each compute one output, so a front end that
// implements one and not another produces a wrong number rather than a
// compile error, and the wrong number says which loop it was.
//
// * `ones`    -- a `for` loop over eight iterations, counting set bits by
//                testing the low bit and shifting right
// * `shifted` -- a `while` loop with its own counter, shifting left three
//                places
// * `doubled` -- a `repeat (2)` of a self-addition, which is a multiply by
//                four
//
// The loop bodies write process-local variables with blocking assignments and
// the results reach the ports with non-blocking ones, which is the ordinary
// division of labour inside a clocked block: the locals are scratch and must
// settle within the edge, and the ports are state and must not.
//
// Bit selects are constant throughout. A variable bit select -- `d[index]` --
// is the natural way to write the popcount and is not what this case does,
// because the front end does not implement one yet.

module loop_forms (clk, rst, d, ones, shifted, doubled);
  input            clk;
  input            rst;
  input      [7:0] d;
  output reg [3:0] ones;
  output reg [7:0] shifted;
  output reg [7:0] doubled;

  always @(posedge clk) begin : counters
    reg [3:0] index;
    reg [3:0] total;
    reg [7:0] work;

    if (rst) begin
      ones    <= 4'd0;
      shifted <= 8'd0;
      doubled <= 8'd0;
    end else begin
      total = 4'd0;
      work  = d;
      for (index = 4'd0; index < 4'd8; index = index + 4'd1) begin
        if (work[0]) total = total + 4'd1;
        work = work >> 1;
      end
      ones <= total;

      index = 4'd0;
      work  = d;
      while (index < 4'd3) begin
        work  = work << 1;
        index = index + 4'd1;
      end
      shifted <= work;

      work = d;
      repeat (2) work = work + work;
      doubled <= work;
    end
  end
endmodule
