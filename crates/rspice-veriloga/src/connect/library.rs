//! The built-in connect module library.
//!
//! # What the standard supplies, and what it does not
//!
//! Verilog-AMS LRM 2.4 clause 7 supplies a *mechanism* and no library. Section
//! 7.5 makes a connect module "a module"; section 7.6 fixes its shape — two
//! ports, one continuous and one discrete, in one of Table 7-2's three
//! direction combinations — and section 7.7's `connectrules` block says which
//! one bridges which discipline pair. Nowhere does clause 7 name a module a
//! compliant implementation shall provide, or fix a parameter's name or
//! default. The `a2d`, `d2a` and `bidir` of clause 7's figures are
//! illustrations of the mechanism, written without behavioural bodies, and
//! section 7.8.3.1's Figure 7-9 uses them the same way.
//!
//! So these three are **RSpice's**, not the standard's. They are named for the
//! figures because that is what a deck author will write in a `connectrules`
//! block.
//!
//! # Why they are signatures and carry no behavioural body
//!
//! Because this compiler cannot analyze one, and shipping source it refuses
//! would be shipping a comment with quotation marks around it. Both refusals
//! are the front end's own, both are by name, and both are pinned in
//! [`tests`] so that the day either lifts, the pin fails and this decision is
//! revisited rather than forgotten:
//!
//! * an analog-sensing discrete process — `always @(above(V(a) - vhi))`, which
//!   is how every published `a2d` is written — is refused three ways at once:
//!   "call to `above` inside a discrete-domain expression is not supported
//!   yet", "a branch access reads a continuous-domain signal and has no
//!   meaning in a discrete-domain expression", and "sensitivity-list term
//!   names no signal, so nothing can trigger it";
//! * a `d2a` written the other way — a discrete process setting a `real` that
//!   an `analog` block contributes through `transition` — is refused because
//!   "the continuous body and the discrete processes would both own the
//!   variable and they advance on different clocks, which this compiler does
//!   not resolve yet".
//!
//! That second message is the whole mixed-signal boundary problem stated at
//! the language level, and it is the same wall the engine hits: the host that
//! could run a connect module's discrete half
//! (`rspice_core`'s `xspice::verilog::MixedSignalHost`) refuses any trial time
//! off its integer-nanosecond grid, and an LTE-controlled transient does not
//! land there.
//!
//! # Where the behaviour is instead
//!
//! In the XSPICE bridge code models the engine already ships, which each of
//! these three delegates to. The delegation is name-for-name and the map is
//! written on each constant below; `rspice-core` pins the other half of it.
//! Delegating rather than transcribing is deliberate — two independent
//! transcriptions of one bridge semantics is how the estate acquired two
//! answers to the same question once already.
//!
//! # Supply sensitivity
//!
//! Section 7.7.3 lets a `connect` statement pass parameters to the connect
//! module it names, so a supply-sensitive connect module is a module with a
//! supply *parameter*: `connect a2d #(.vsup(1.8));`. That is what `vsup` is on
//! all three, and every level the delegation stamps is derived from it, so
//! overriding it moves all of them together.
//!
//! Two things are refused, each for its own reason.
//!
//! * **A connect module that reads a supply *net*.** Section 7.6 gives a
//!   connect module exactly two ports, one per domain, and
//!   [`super::ConnectError::ConnectModulePortCount`] enforces it, so there is
//!   no third port a supply rail could arrive on.
//! * **Thresholds settable independently of the supply** — a `vlo`/`vhi` pair
//!   defaulting to `vsup / 2.0`. A parameter whose default names another
//!   parameter does not fold in this compiler:
//!   [`crate::semantic::AnalyzedParameter::default`] is `None` for it and only
//!   the unevaluated `default_expr` survives, so the delegation would have to
//!   grow an evaluator for [`crate::ast::Expression`] on the engine side —
//!   a second implementation of something this crate already owns, which is
//!   the shape of drift this library exists to avoid. The supply is therefore
//!   the one level knob this wave, and the derivation lives in exactly one
//!   place: the engine's delegation, where the auto-bridge's identical
//!   derivation from `vcc` already lives.

/// `a2d` — Table 7-2 row 1, continuous `input` and discrete `output`.
///
/// Delegates to the `adc_bridge` code model:
///
/// | `a2d` | `adc_bridge` |
/// |---|---|
/// | `vsup / 2` | `in_low` |
/// | `vsup / 2` | `in_high` |
/// | `tdrise` | `rise_delay` |
/// | `tdfall` | `fall_delay` |
///
/// `adc_bridge` reads a voltage at or below `in_low` as `0`, at or above
/// `in_high` as `1`, and anything between as `x` — a band, not a hysteresis.
/// Both thresholds are half the supply, so the band is empty and the module is
/// a plain comparator: the same collapse the engine's own auto-bridge makes
/// when it sets `in_low = in_high = vcc/2`.
pub const A2D: &str = "\
connectmodule a2d(a, d);
    input a;
    output d;
    electrical a;
    logic d;

    parameter real vsup = 3.3;
    parameter real tdrise = 1e-9;
    parameter real tdfall = 1e-9;
endmodule
";

/// `d2a` — Table 7-2 row 2, discrete `input` and continuous `output`.
///
/// Delegates to the `dac_bridge` code model:
///
/// | `d2a` | `dac_bridge` |
/// |---|---|
/// | `0` | `out_low` |
/// | `vsup` | `out_high` |
/// | `trise` | `t_rise` |
/// | `tfall` | `t_fall` |
///
/// `out_undef` is deliberately *not* stamped. `dac_bridge` sets it to the
/// midpoint of `out_low` and `out_high` exactly when those two are given and
/// it is not, so leaving it out is how the midpoint is obtained — delegated
/// rather than restated, so the two can never disagree about what half is.
///
/// That midpoint is what `x` and `z` drive, and it is deliberately not an
/// error: a four-state net is `x` before anything drives it, so refusing `x`
/// would refuse every design at time zero.
pub const D2A: &str = "\
connectmodule d2a(d, a);
    input d;
    output a;
    logic d;
    electrical a;

    parameter real vsup = 3.3;
    parameter real trise = 1e-9;
    parameter real tfall = 1e-9;
endmodule
";

/// `bidir` — Table 7-2 row 3, both ports `inout`.
///
/// Section 7.6's third example: a module of this kind "can bridge any mixed
/// port", which is why a bidirectional rule is ranked *below* a unidirectional
/// one that also fits rather than tying with it — see [`super::ConnectRuleTable::select`].
///
/// Delegates to the `bidi_bridge` code model, whose two directions take the
/// two threshold pairs:
///
/// | `bidir` | `bidi_bridge` |
/// |---|---|
/// | `vsup / 2` | `in_low` |
/// | `vsup / 2` | `in_high` |
/// | `vsup` | `out_high` |
/// | `trise` | `t_rise` |
/// | `tfall` | `t_fall` |
pub const BIDIR: &str = "\
connectmodule bidir(a, d);
    inout a;
    inout d;
    electrical a;
    logic d;

    parameter real vsup = 3.3;
    parameter real trise = 1e-9;
    parameter real tfall = 1e-9;
endmodule
";

/// The `connectrules` block that selects the three above for the one
/// discipline pair a SPICE deck can present: `electrical` on the matrix side,
/// `logic` on the event side.
///
/// All three rules are `merged`, section 7.8.3's default, which is what makes
/// several discrete ports on one node share a single bridge instance — the
/// shape the engine's auto-bridge already has, one bridge per node.
pub const BUILTIN_CONNECT_RULES: &str = "\
connectrules rspice_builtin;
    connect a2d;
    connect d2a;
    connect bidir;
endconnectrules
";

/// Every built-in connect module, in declaration order.
pub const BUILTIN_CONNECT_MODULES: [(&str, &str); 3] =
    [("a2d", A2D), ("d2a", D2A), ("bidir", BIDIR)];

/// The whole library as one source file: the three modules followed by the
/// `connectrules` block that selects them.
///
/// One file rather than three, because section 7.7.1 requires a `connect`
/// statement to name a *declared* connect module and
/// [`super::build_connect_rule_table`] reads one [`crate::ast::SourceFile`].
pub fn builtin_connect_library_source() -> String {
    let mut source = String::new();
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(BUILTIN_CONNECT_RULES);
    source
}

#[cfg(test)]
mod tests;
