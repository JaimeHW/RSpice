// Real state: a real value that is still there on the next clock edge.
//
// `wreal_forms` covers every real-net form whose output is a closed form of its
// inputs at one instant. This covers the other half of a real-number model, and
// the half that could not be written at all until the discrete domain had a
// real *variable*: state.
//
// One design rather than four, so the interactions are covered too. Each of the
// four storage mechanisms is exercised, and three of them hold the same
// quantity by different means so a disagreement localises:
//
//   * `state`   -- a module-level `real` (IEEE 1364-2005 section 3.9), written
//                  by a nonblocking assignment, which is what makes the
//                  recurrence read the previous sample's value rather than this
//                  one's;
//   * `acc`     -- a process-local `real`, accumulating across a suspension, so
//                  its value lives in the process's resume state and nowhere
//                  else;
//   * `pattern` -- the same state parked in a 64-bit `reg` through Verilog-AMS
//                  LRM 2.4 section 3.7's own `$realtobits`/`$bitstoreal` bridge
//                  and brought back, which is exact or the case fails;
//   * `K`       -- a `parameter real`, fixed at elaboration by section 12.2 and
//                  folded into the recurrence rather than read at runtime.
//
// The sample clock is a stimulus column rather than a harness convenience: the
// reference model sees exactly the edges the design sees, because both read the
// same vectors.
module real_state(clk, vin, vout, vsum, vround);
  parameter real K = 0.25;

  input clk;
  input wreal vin;
  // The filter state, published through a driver on a real net.
  output wreal vout;
  // A real *variable* port: section 12.3.4's variable port form with section
  // 3.9's `real` as the type, which is the only real port a process may write.
  output real vsum;
  // The state after a round trip through its own bit pattern.
  output real vround;

  wire clk;
  real state;
  reg [63:0] pattern;

  // The canonical discrete-time single-pole low-pass. `<=` is load-bearing:
  // the right-hand side is evaluated in the active region and the update lands
  // in the nonblocking region (section 11), so `state` on the right is the
  // previous sample's.
  always @(posedge clk) state <= state + (vin - state) * K;

  // A process-local real that survives the suspension. The declaration is
  // outside the `forever`, so it is entered once and every later edge is a
  // resumption into the loop -- section 9.8.1 re-enters a declarative region
  // only when control enters it again.
  always begin : accumulate
    real acc;
    forever begin
      @(posedge clk);
      acc = acc + vin;
      vsum = acc;
    end
  end

  // Section 3.7's bridge, both ways, in one process so the pattern is written
  // before it is read and no unknown bit can reach the conversion. Blocking, so
  // `state` here is the value before this edge's nonblocking update lands --
  // `vround` therefore trails `vout` by exactly one sample, which the reference
  // model states rather than tolerates.
  always @(posedge clk) begin
    pattern = $realtobits(state);
    vround = $bitstoreal(pattern);
  end

  assign vout = state;
endmodule
