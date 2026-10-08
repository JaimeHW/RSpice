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
//! # Execution
//!
//! These shipped signatures select the existing XSPICE ADC, DAC and bidirectional
//! bridge implementations. The compiler proves delegation by comparing the full
//! token sequence of the declaration, including parameters. Comments and spacing
//! may differ; a familiar name alone never replaces an authored body.
//!
//! Other declarations execute through the ordinary mixed runtime with their own
//! parameters, hierarchy and transactional instance state. Discrete processes can
//! sample analog nodes and variables and subscribe to cross, above, timer and
//! event-assigned scalar variables. Other analog event forms require further
//! language support.
//!
//! The shipped signatures expose `vsup` and derive levels from that parameter.
//! An authored body can declare its own threshold and loading parameters. Clause
//! 7.6's two-port shape still applies to connection declarations.

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

/// Conservative delegation proof: comments and whitespace may differ, but
/// every declaration token, parameter default and body token must match the
/// shipped signature. A familiar module name alone grants no delegation.
pub(crate) fn equivalent_declarations(
    source: &str,
    file: &crate::ast::SourceFile,
) -> std::collections::BTreeSet<String> {
    let tokens = |source: &str| {
        crate::lexer::Lexer::new(source, crate::source::SourceId::new(0))
            .collect_tokens()
            .ok()
            .map(|tokens| {
                tokens
                    .into_iter()
                    .map(|token| (token.kind, token.text))
                    .collect::<Vec<_>>()
            })
    };
    file.items
        .iter()
        .filter_map(|item| {
            let crate::ast::Item::ConnectModule(module) = item else {
                return None;
            };
            let (_, reference) = BUILTIN_CONNECT_MODULES
                .iter()
                .find(|(name, _)| module.name == *name)?;
            let declaration = source.get(module.span.start as usize..module.span.end as usize)?;
            (tokens(declaration)? == tokens(reference)?).then(|| module.name.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests;
