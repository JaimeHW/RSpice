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
//! These shipped signatures select the engine's built-in connect-rule
//! converters. The compiler proves delegation by comparing the full token
//! sequence of the declaration, including parameters. Comments and spacing may
//! differ; a familiar name alone never replaces an authored body.
//!
//! Other declarations execute through the ordinary mixed runtime with their own
//! parameters, hierarchy and transactional instance state. Discrete processes can
//! sample analog nodes and variables and subscribe to cross, above, timer and
//! event-assigned scalar variables. Other analog event forms require further
//! language support.
//!
//! An authored body can declare its own threshold and loading parameters. Clause
//! 7.6's two-port shape still applies to connection declarations.
//!
//! # Two signatures per module
//!
//! [`A2D`], [`D2A`] and [`BIDIR`] declare the converters' whole parameter set.
//! The earlier, shorter signatures ([`LEGACY_BUILTIN_CONNECT_MODULES`]: a
//! supply and the edge times only) still delegate, so a deck carrying a
//! verbatim copy of one keeps working. A legacy copy accepts only the
//! parameters it declares, and its declared edge-time defaults are not
//! applied: they never were, and the converter's own defaults stand in for
//! them, exactly as before.
//!
//! The defaults that name another parameter (`vthi = vsup / 2`, `rx =
//! min(rlo, rhi)`, ...) do not fold here; the engine reads them as the
//! declarations state them, and a supply bound live to a net makes the
//! supply-relative ones track it.

/// Which shipped signature a token-equivalent declaration matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltinSignature {
    /// The full parameter set: [`A2D`], [`D2A`], [`BIDIR`].
    Full,
    /// The earlier signature: a supply and the edge times only.
    Legacy,
}

/// `a2d` — Table 7-2 row 1, continuous `input` and discrete `output`.
///
/// A hysteresis comparator: the output becomes `1` at `vthi` and `0` at
/// `vtlo`, holds between them, and reads `x` once the input has stayed
/// between them for `txdel`. Both thresholds default to half the supply, so
/// the band is empty and the module is a plain comparator.
pub const A2D: &str = "\
connectmodule a2d(a, d);
    input a;
    output d;
    electrical a;
    logic d;

    parameter real vsup = 3.3;
    parameter real vsupmin = 0;
    parameter real vthi = vsup / 2;
    parameter real vtlo = vsup / 2;
    parameter real txdel = inf;
    parameter real tdrise = 0;
    parameter real tdfall = 0;
endmodule
";

/// `d2a` — Table 7-2 row 2, discrete `input` and continuous `output`.
///
/// Each driver is a Thevenin source whose level and resistance follow its
/// state and strength: `vlo` behind `rlo` for a strong 0, `vhi` behind `rhi`
/// for a strong 1, `vx` behind `rx` for `x`, and `vz` behind `rz` for `z`,
/// with `rsup`, `rpull` and `rweak` for the other drive strengths. A change
/// takes the full `trise` or `tfall`, `tdelay` after the event.
///
/// The `x` level is deliberately not an error: a four-state net is `x` before
/// anything drives it, so refusing `x` would refuse every design at time zero.
pub const D2A: &str = "\
connectmodule d2a(d, a);
    input d;
    output a;
    logic d;
    electrical a;

    parameter real vsup = 3.3;
    parameter real vsupmin = 0;
    parameter real vlo = 0;
    parameter real vhi = vsup;
    parameter real vx = (vlo + vhi) / 2;
    parameter real vz = vx;
    parameter real rlo = 20;
    parameter real rhi = 20;
    parameter real rx = min(rlo, rhi);
    parameter real rz = inf;
    parameter real rsup = min(rlo, rhi) / 10;
    parameter real rpull = 10e3;
    parameter real rweak = 100e3;
    parameter real trise = 0;
    parameter real tfall = 0;
    parameter real tdelay = 0;
endmodule
";

/// `bidir` — Table 7-2 row 3, both ports `inout`.
///
/// Section 7.6's third example: a module of this kind "can bridge any mixed
/// port", which is why a bidirectional rule is ranked *below* a unidirectional
/// one that also fits rather than tying with it — see [`super::ConnectRuleTable::select`].
///
/// The [`D2A`] driven by the port's own drivers and the [`A2D`] reading the
/// same node for the port's receivers, both at once: `tdrise`/`tdfall` are the
/// receiving half's delays and `tdelay` the driving half's.
pub const BIDIR: &str = "\
connectmodule bidir(a, d);
    inout a;
    inout d;
    electrical a;
    logic d;

    parameter real vsup = 3.3;
    parameter real vsupmin = 0;
    parameter real vthi = vsup / 2;
    parameter real vtlo = vsup / 2;
    parameter real txdel = inf;
    parameter real tdrise = 0;
    parameter real tdfall = 0;
    parameter real vlo = 0;
    parameter real vhi = vsup;
    parameter real vx = (vlo + vhi) / 2;
    parameter real vz = vx;
    parameter real rlo = 20;
    parameter real rhi = 20;
    parameter real rx = min(rlo, rhi);
    parameter real rz = inf;
    parameter real rsup = min(rlo, rhi) / 10;
    parameter real rpull = 10e3;
    parameter real rweak = 100e3;
    parameter real trise = 0;
    parameter real tfall = 0;
    parameter real tdelay = 0;
endmodule
";

/// The earlier `a2d` signature, still delegating.
const LEGACY_A2D: &str = "\
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

/// The earlier `d2a` signature, still delegating.
const LEGACY_D2A: &str = "\
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

/// The earlier `bidir` signature, still delegating.
const LEGACY_BIDIR: &str = "\
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

/// The earlier signatures of the built-in connect modules, in declaration
/// order. A token-equivalent copy of one delegates with its own parameters.
pub const LEGACY_BUILTIN_CONNECT_MODULES: [(&str, &str); 3] = [
    ("a2d", LEGACY_A2D),
    ("d2a", LEGACY_D2A),
    ("bidir", LEGACY_BIDIR),
];

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
/// every declaration token, parameter default and body token must match a
/// shipped signature, current or legacy. A familiar module name alone grants
/// no delegation.
pub(crate) fn equivalent_declarations(
    source: &str,
    file: &crate::ast::SourceFile,
) -> std::collections::BTreeMap<String, BuiltinSignature> {
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
            let declaration = tokens(
                source.get(module.span.start as usize..module.span.end as usize)?,
            )?;
            [
                (BuiltinSignature::Full, &BUILTIN_CONNECT_MODULES),
                (BuiltinSignature::Legacy, &LEGACY_BUILTIN_CONNECT_MODULES),
            ]
            .into_iter()
            .find_map(|(signature, modules)| {
                let (_, reference) = modules.iter().find(|(name, _)| module.name == *name)?;
                (tokens(reference)? == declaration).then(|| (module.name.to_string(), signature))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests;
