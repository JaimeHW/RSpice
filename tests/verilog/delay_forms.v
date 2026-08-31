// The two procedural delays, against an assignment with neither.
//
// `prompt` is written the moment the edge arrives. The other two are written
// six and four time units later by the two spellings IEEE 1364-2005 gives for
// a delay inside a process:
//
// * `lagged <= #6 d;` is section 9.2.2's intra-assignment delay. The
//   right-hand side is evaluated when the statement executes -- at the edge --
//   and only the update waits.
// * `#4 held <= d;` is section 9.7.7's delay control as a statement. The
//   process itself suspends, so the right-hand side is evaluated four units
//   *after* the edge, and everything the process would have done in between
//   does not happen.
//
// Both delays are longer than the gap from the edge to the sample, so both
// outputs trail `prompt` by one observation and neither lands on a sampling
// instant. That is the whole reason the numbers are 6 and 4 rather than
// something rounder: with the corpus's timing an update at 8 would race the
// observation it is supposed to be seen by.
//
// The reset path has no delay on it, so the first observations are of a
// defined state rather than of a register that has never been written.

module delay_forms (clk, rst, d, prompt, lagged, held);
  input            clk;
  input            rst;
  input      [3:0] d;
  output reg [3:0] prompt;
  output reg [3:0] lagged;
  output reg [3:0] held;

  always @(posedge clk) begin
    if (rst) begin
      prompt <= 4'd0;
      lagged <= 4'd0;
    end else begin
      prompt <= d;
      lagged <= #6 d;
    end
  end

  always @(posedge clk) begin
    if (rst) held <= 4'd0;
    else begin
      #4 held <= d;
    end
  end
endmodule
