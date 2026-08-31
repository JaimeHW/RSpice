// Non-blocking versus blocking assignment, arranged so the answer is *defined*
// rather than a race.
//
// This distinction is the single most common way a hand-written event
// scheduler is wrong, so it earns a dedicated case. But the textbook
// demonstration of it -- one `always` block writing a variable another reads on
// the same edge -- is explicitly unspecified by the LRM, and two conforming
// simulators may legitimately disagree. Such a design is useless as an oracle
// case: a disagreement would indict the harness, not the simulator.
//
// Every block below therefore reads only `din` and its own variables, so no
// cross-block ordering is involved and the LRM fixes the result exactly.
//
//   * `nb2` -- three non-blocking assignments in one block form a shift
//     register. All three right-hand sides are evaluated against the values
//     held *before* the edge, so `din` takes three clocks to reach `nb2`.
//
//   * `bl2` -- the same three assignments written blocking execute in source
//     order within the single edge, so `din` reaches `bl2` on the first clock.
//     `bl2` leading `nb2` by two clocks is the whole contrast.
//
//   * `swap_x`/`swap_y` -- the pair exchanges values every clock, which only
//     non-blocking assignment can express. A scheduler that applies updates
//     eagerly assigns both variables the same value on the first edge and they
//     never diverge again.
//
// No `$display`, no delays: the module is observed entirely through its ports.

module nba_ordering (clk, rst, din, nb2, bl2, swap_x, swap_y);
  input            clk;
  input            rst;
  input      [3:0] din;

  output reg [3:0] nb2;
  output reg [3:0] bl2;
  output reg [3:0] swap_x;
  output reg [3:0] swap_y;

  reg [3:0] nb0, nb1;
  reg [3:0] bl0, bl1;

  // Non-blocking: right-hand sides sample the pre-edge state, so this is a
  // three-deep shift register regardless of the order the statements appear in.
  always @(posedge clk) begin
    if (rst) begin
      nb0 <= 4'b0;
      nb1 <= 4'b0;
      nb2 <= 4'b0;
    end else begin
      nb0 <= din;
      nb1 <= nb0;
      nb2 <= nb1;
    end
  end

  // Blocking: statements execute in source order inside the edge, so `din`
  // propagates all the way to `bl2` within a single clock.
  always @(posedge clk) begin
    if (rst) begin
      bl0 = 4'b0;
      bl1 = 4'b0;
      bl2 = 4'b0;
    end else begin
      bl0 = din;
      bl1 = bl0;
      bl2 = bl1;
    end
  end

  // A true exchange. Both right-hand sides read the pre-edge values, so the
  // pair alternates; with eager updates they would collapse to one value.
  always @(posedge clk) begin
    if (rst) begin
      swap_x <= 4'b0101;
      swap_y <= 4'b1010;
    end else begin
      swap_x <= swap_y;
      swap_y <= swap_x;
    end
  end
endmodule
