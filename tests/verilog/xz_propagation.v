// Four-state behaviour: how X and Z travel, and where they stop.
//
// **This case is Icarus-only, and that is a property of Verilator rather than
// of the case.** Verilator is a two-state simulator: it resolves X to a
// concrete 0/1 at elaboration or randomly, per `--x-assign`/`--x-initial`, and
// models `z` only for the narrow tristate patterns it can infer. Asking the two
// oracles to agree here would be asking Verilator to be something it does not
// claim to be, so the manifest lists only `iverilog` for this entry. Every
// other entry in the corpus is deliberately two-state-clean so both oracles can
// be held to it.
//
// The behaviours under test, all observable at ports:
//
//   * `and_x` -- X is *not* contagious through a controlling value. `1'b0 & x`
//     is 0, while `1'b1 & x` is x. A front end that propagates X
//     unconditionally gets the first one wrong; one that ignores X gets the
//     second wrong.
//   * `or_x`  -- the dual, with 1 as the controlling value.
//   * `bus`   -- a tristate net with two conditional drivers: both off gives z,
//     one on gives that driver's value, both on with opposite values gives x.
//     That last case is bus contention and must resolve to x, not to either
//     driver.
//   * `eq_case`/`eq_log` -- `===` compares X and Z as ordinary values and always
//     yields a defined 0 or 1; `==` yields x when either operand contains one.
//     Confusing the two is why X leaks into control flow.
//   * `sel`   -- `casez` treats z (spelled `?`) in a case *item* as a wildcard,
//     matching the selector regardless. This is pattern matching, not value
//     comparison.

module xz_propagation (a, drive_hi, drive_lo, sel_in,
                       and_x, or_x, bus, eq_case, eq_log, sel);
  input        a;
  input        drive_hi;
  input        drive_lo;
  input  [3:0] sel_in;

  output       and_x;
  output       or_x;
  output       bus;
  output       eq_case;
  output       eq_log;
  output reg [1:0] sel;

  // A deliberately unknown value, introduced without an initial block so the
  // module stays purely structural/continuous.
  wire unknown = 1'bx;

  // 0 is controlling for AND, so `and_x` is 0 when a==0 and x when a==1.
  assign and_x = a & unknown;
  // 1 is controlling for OR, so `or_x` is 1 when a==1 and x when a==0.
  assign or_x  = a | unknown;

  // Two conditional drivers on one net: z when neither drives, the driven
  // value when exactly one does, and x when both drive opposite values.
  assign bus = drive_hi ? 1'b1 : 1'bz;
  assign bus = drive_lo ? 1'b0 : 1'bz;

  // `===` is defined for every operand including x; `==` is not.
  assign eq_case = (unknown === 1'bx);
  assign eq_log  = (unknown ==  1'bx);

  // `casez` wildcards on z in the case item, so 4'b1zzz matches any selector
  // whose top bit is 1.
  always @(*) begin
    casez (sel_in)
      4'b1???: sel = 2'b11;
      4'b01??: sel = 2'b10;
      4'b001?: sel = 2'b01;
      default: sel = 2'b00;
    endcase
  end
endmodule
